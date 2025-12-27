//! Filesystem watcher and sync loop for local mode.

use std::collections::{HashMap, HashSet};
use std::path::{Path, PathBuf};
use std::pin::Pin;
use std::sync::Arc;
use std::time::Duration;

use notify::{Event, EventKind, RecommendedWatcher, RecursiveMode, Watcher};
use tokio::sync::{mpsc, RwLock};
use tokio::time::Sleep;
use tracing::{info, warn};

use coco_protocol::{CocoError, CocoResult};

use crate::fs::{collect_files, path_to_string, should_ignore_path};
use crate::ids::sha256_hex;
use crate::ingest::{
    file_type_for_path, normalize_path, title_for_path, IngestRequest, Ingestor,
};
use crate::metrics::LocalMetrics;
use crate::storage::meta::ProjectRecord;

const DEFAULT_DEBOUNCE_MS: u64 = 300;
const DEFAULT_RESCAN_SECS: u64 = 300;
const DEFAULT_WATCH_ENABLED: bool = true;
/// Watch configuration for local filesystem monitoring.
#[derive(Debug, Clone)]
pub struct WatchConfig {
    /// Whether filesystem watch is enabled.
    pub enabled: bool,
    /// Debounce window for file change events.
    pub debounce: Duration,
    /// Interval for periodic rescan.
    pub rescan_interval: Duration,
}

impl WatchConfig {
    /// Loads watch configuration from environment variables.
    pub fn from_env() -> CocoResult<Self> {
        let enabled = env_bool("COCO_WATCH_ENABLED", DEFAULT_WATCH_ENABLED)?;
        let debounce_ms = env_u64("COCO_WATCH_DEBOUNCE_MS", DEFAULT_DEBOUNCE_MS)?;
        let rescan_secs = env_u64("COCO_WATCH_RESCAN_SECS", DEFAULT_RESCAN_SECS)?;
        Ok(Self {
            enabled,
            debounce: Duration::from_millis(debounce_ms),
            rescan_interval: Duration::from_secs(rescan_secs),
        })
    }
}

/// Handle for sending updates to the watch loop.
#[derive(Debug, Clone)]
pub struct WatchSender {
    tx: mpsc::UnboundedSender<WatchCommand>,
}

impl WatchSender {
    /// Registers a new project path for watching.
    pub fn watch_project(&self, project: ProjectRecord) -> CocoResult<()> {
        self.tx
            .send(WatchCommand::AddProject(project))
            .map_err(|_| CocoError::system("watch loop is not running"))
    }
}

/// Starts the watch service and returns a sender for updates.
pub async fn start_watch_service(
    ingestor: Ingestor,
    config: WatchConfig,
    metrics: Arc<LocalMetrics>,
) -> CocoResult<Option<WatchSender>> {
    if !config.enabled {
        return Ok(None);
    }

    let projects = ingestor.meta().list_projects().await?;
    let (command_tx, command_rx) = mpsc::unbounded_channel();
    let (event_tx, event_rx) = mpsc::unbounded_channel();

    let mut watcher = notify::recommended_watcher(move |res| {
        let _ = event_tx.send(res);
    })
    .map_err(|err| CocoError::system(format!("failed to initialize watcher: {err}")))?;

    let mut watch_projects = Vec::new();
    for project in projects {
        if let Some(watch_project) = WatchProject::from_record(&project)? {
            watch_projects.push(watch_project);
        }
    }

    for project in &watch_projects {
        if let Err(err) = watcher.watch(&project.root, RecursiveMode::Recursive) {
            warn!("failed to watch {}: {err}", project.root.display());
        } else {
            info!("watching {}", project.root.display());
        }
    }

    let projects_state = Arc::new(RwLock::new(watch_projects));
    let sender = WatchSender { tx: command_tx };

    tokio::spawn(watch_loop(
        ingestor,
        watcher,
        projects_state,
        config,
        metrics,
        event_rx,
        command_rx,
    ));

    Ok(Some(sender))
}

#[derive(Debug)]
enum WatchCommand {
    AddProject(ProjectRecord),
}

#[derive(Debug, Clone)]
struct WatchProject {
    id: String,
    root: PathBuf,
    active_config_id: String,
}

impl WatchProject {
    fn from_record(record: &ProjectRecord) -> CocoResult<Option<Self>> {
        if record.path.trim().is_empty() {
            return Ok(None);
        }
        let path = normalize_path(Path::new(&record.path))?;
        Ok(Some(Self {
            id: record.id.clone(),
            root: path,
            active_config_id: record.active_config_id.clone(),
        }))
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum PendingAction {
    Upsert,
    Remove,
}

async fn watch_loop(
    ingestor: Ingestor,
    mut watcher: RecommendedWatcher,
    projects: Arc<RwLock<Vec<WatchProject>>>,
    config: WatchConfig,
    metrics: Arc<LocalMetrics>,
    mut event_rx: mpsc::UnboundedReceiver<notify::Result<Event>>,
    mut command_rx: mpsc::UnboundedReceiver<WatchCommand>,
) {
    let mut pending: HashMap<PathBuf, PendingAction> = HashMap::new();
    let mut debounce_sleep: Option<Pin<Box<Sleep>>> = None;
    let mut rescan = tokio::time::interval(config.rescan_interval);

    loop {
        tokio::select! {
            Some(command) = command_rx.recv() => {
                handle_command(command, &mut watcher, &projects).await;
            }
            Some(event) = event_rx.recv() => {
                if let Ok(event) = event {
                    queue_event(event, &mut pending);
                    metrics.set_pending_events(pending.len());
                    if !pending.is_empty() {
                        debounce_sleep = Some(Box::pin(tokio::time::sleep(config.debounce)));
                    }
                }
            }
            _ = rescan.tick() => {
                let ingestor = ingestor.clone();
                let projects = Arc::clone(&projects);
                let config = config.clone();
                let metrics = Arc::clone(&metrics);
                tokio::spawn(async move {
                    if let Err(err) = rescan_projects(&ingestor, &projects, &config, &metrics).await {
                        warn!("rescan failed: {err}");
                    }
                });
            }
            _ = debounce_sleep.as_mut().unwrap(), if debounce_sleep.is_some() => {
                debounce_sleep = None;
                if let Err(err) = process_pending(&ingestor, &projects, &metrics, &mut pending).await {
                    warn!("processing pending events failed: {err}");
                }
                metrics.set_pending_events(pending.len());
            }
            else => break,
        }
    }
}

async fn handle_command(
    command: WatchCommand,
    watcher: &mut RecommendedWatcher,
    projects: &Arc<RwLock<Vec<WatchProject>>>,
) {
    match command {
        WatchCommand::AddProject(project) => {
            let Some(watch_project) = (match WatchProject::from_record(&project) {
                Ok(value) => value,
                Err(err) => {
                    warn!("invalid project {}: {err}", project.id);
                    return;
                }
            }) else {
                return;
            };
            {
                let guard = projects.read().await;
                if guard.iter().any(|item| item.root == watch_project.root) {
                    return;
                }
            }
            if let Err(err) = watcher.watch(&watch_project.root, RecursiveMode::Recursive) {
                warn!("failed to watch {}: {err}", watch_project.root.display());
                return;
            }
            info!("watching {}", watch_project.root.display());
            projects.write().await.push(watch_project);
        }
    }
}

fn queue_event(event: Event, pending: &mut HashMap<PathBuf, PendingAction>) {
    let action = match event.kind {
        EventKind::Remove(_) => PendingAction::Remove,
        _ => PendingAction::Upsert,
    };
    for path in event.paths {
        if should_ignore_path(&path) {
            continue;
        }
        let effective = if action == PendingAction::Remove {
            PendingAction::Remove
        } else if path.exists() {
            PendingAction::Upsert
        } else {
            PendingAction::Remove
        };
        if effective == PendingAction::Upsert && !path.is_file() {
            continue;
        }
        merge_pending(pending, path, effective);
    }
}

fn merge_pending(
    pending: &mut HashMap<PathBuf, PendingAction>,
    path: PathBuf,
    action: PendingAction,
) {
    match pending.get_mut(&path) {
        Some(existing) => {
            if *existing == PendingAction::Remove && action == PendingAction::Upsert {
                *existing = PendingAction::Upsert;
            } else if *existing == PendingAction::Upsert && action == PendingAction::Remove {
                *existing = PendingAction::Remove;
            }
        }
        None => {
            pending.insert(path, action);
        }
    }
}

async fn process_pending(
    ingestor: &Ingestor,
    projects: &Arc<RwLock<Vec<WatchProject>>>,
    metrics: &LocalMetrics,
    pending: &mut HashMap<PathBuf, PendingAction>,
) -> CocoResult<()> {
    let batch: Vec<(PathBuf, PendingAction)> = pending.drain().collect();
    if batch.is_empty() {
        return Ok(());
    }
    let projects_guard = projects.read().await;
    for (path, action) in batch {
        let Some(project) = find_project_for_path(&projects_guard, &path) else {
            continue;
        };
        match action {
            PendingAction::Upsert => {
                if let Err(err) = upsert_path(ingestor, metrics, project, &path).await {
                    warn!("failed to ingest {}: {err}", path.display());
                }
            }
            PendingAction::Remove => {
                if let Err(err) = remove_path(ingestor, project, &path).await {
                    warn!("failed to delete {}: {err}", path.display());
                }
            }
        }
    }
    Ok(())
}

async fn upsert_path(
    ingestor: &Ingestor,
    metrics: &LocalMetrics,
    project: &WatchProject,
    path: &Path,
) -> CocoResult<()> {
    let normalized = normalize_path(path)?;
    if should_ignore_path(&normalized) {
        return Ok(());
    }
    let path_str = path_to_string(&normalized)?;
    let content = match tokio::fs::read_to_string(&normalized).await {
        Ok(content) => content,
        Err(err) => {
            return Err(CocoError::system(format!(
                "failed to read file {}: {err}",
                normalized.display()
            )))
        }
    };
    let content_hash = sha256_hex(content.as_bytes());
    let existing = ingestor
        .document_by_path(&project.id, &path_str)
        .await?;
    if let Some(existing) = &existing {
        if existing.content_hash == content_hash {
            return Ok(());
        }
    }

    let file_type = file_type_for_path(&normalized);
    let title = title_for_path(&normalized);
    let document_id = existing.as_ref().map(|doc| doc.id.clone());

    let path_record = path_str.clone();
    ingestor
        .ingest_request(IngestRequest {
            project_id: project.id.clone(),
            indexing_config_id: project.active_config_id.clone(),
            version_id: None,
            document_id,
            content,
            content_hash: Some(content_hash),
            file_type,
            title,
            path: Some(path_str),
            chunking: None,
        })
        .await?;
    metrics.record_import(Some(&path_record));
    Ok(())
}

async fn remove_path(
    ingestor: &Ingestor,
    project: &WatchProject,
    path: &Path,
) -> CocoResult<()> {
    let normalized = normalize_path(path)?;
    let path_str = path_to_string(&normalized)?;
    ingestor.delete_by_path(&project.id, &path_str).await?;
    Ok(())
}

async fn rescan_projects(
    ingestor: &Ingestor,
    projects: &Arc<RwLock<Vec<WatchProject>>>,
    config: &WatchConfig,
    metrics: &LocalMetrics,
) -> CocoResult<()> {
    let projects = projects.read().await.clone();
    for project in projects {
        rescan_project(ingestor, metrics, &project, config).await?;
    }
    Ok(())
}

async fn rescan_project(
    ingestor: &Ingestor,
    metrics: &LocalMetrics,
    project: &WatchProject,
    _config: &WatchConfig,
) -> CocoResult<()> {
    let files = collect_files(&project.root, true)?;
    let mut file_set: HashSet<String> = HashSet::new();
    for path in &files {
        let path_str = path_to_string(path)?;
        file_set.insert(path_str);
    }

    for path in files {
        if let Err(err) = upsert_path(ingestor, metrics, project, &path).await {
            warn!("rescan ingest failed for {}: {err}", path.display());
        }
    }

    let version_id = ingestor
        .meta()
        .ensure_active_version_id(&project.id)
        .await?;
    let docs = ingestor
        .meta()
        .list_documents(&project.id, &version_id)
        .await?;
    for doc in docs {
        if doc.path.is_empty() {
            continue;
        }
        if !file_set.contains(&doc.path) {
            ingestor.delete_by_path(&project.id, &doc.path).await?;
        }
    }

    Ok(())
}

fn find_project_for_path<'a>(
    projects: &'a [WatchProject],
    path: &Path,
) -> Option<&'a WatchProject> {
    let mut best: Option<&WatchProject> = None;
    let mut best_depth = 0usize;
    for project in projects {
        if path.starts_with(&project.root) {
            let depth = project.root.components().count();
            if depth >= best_depth {
                best = Some(project);
                best_depth = depth;
            }
        }
    }
    best
}


fn env_u64(key: &str, default: u64) -> CocoResult<u64> {
    match std::env::var(key) {
        Ok(value) => value
            .parse::<u64>()
            .map_err(|_| CocoError::user(format!("{key} must be a valid number"))),
        Err(_) => Ok(default),
    }
}

fn env_bool(key: &str, default: bool) -> CocoResult<bool> {
    match std::env::var(key) {
        Ok(value) => match value.to_ascii_lowercase().as_str() {
            "1" | "true" | "yes" => Ok(true),
            "0" | "false" | "no" => Ok(false),
            _ => Err(CocoError::user(format!(
                "{key} must be a boolean value"
            ))),
        },
        Err(_) => Ok(default),
    }
}
