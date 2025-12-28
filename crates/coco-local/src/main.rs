use std::collections::{HashMap, HashSet};
use std::hash::{Hash, Hasher};
use std::path::{Path, PathBuf};
use std::sync::Arc;
use std::time::Instant;

use chrono::{SecondsFormat, Utc};
use clap::Parser;
use serde::{Deserialize, Serialize};

use coco_local::cli::{
    Cli, Command, ConfigCommand, ExperimentCommand, ExperimentCompareArgs, ExperimentRunArgs,
    GrammarCommand, GrammarInstallArgs, GrammarUpdateArgs, SetupArgs, UpdateArgs,
};
use coco_local::embedder::{DownloadProgress, ModelStore};
use coco_local::experiment::{
    EvaluationCase, ExperimentResultEntry, ExperimentResultFile, ExperimentSpec, MetricKind,
    MetricSpec, MetricValue, QueryStrategy,
};
use coco_local::fs::{collect_files, path_to_string};
use coco_local::ingest::{file_type_for_path, normalize_path, title_for_path};
use coco_local::metrics::LocalMetrics;
use coco_local::service::LocalServiceConfig;
use coco_local::storage::meta::DEFAULT_CONFIG_ID;
use coco_local::{grammar, paths, update};
use coco_core::{normalize_config_id, validate_indexing_config};
use coco_protocol::{
    ChunkingStrategy, CocoError, CocoErrorKind, CocoResult, EmbeddingConfig, ErrorResponse,
    FileType, IndexingConfig, ResponseEnvelope, RetrievalConfig, RetrievalMode, SearchHit,
    SearchIntentInput, SearchQueryInput, ValidationContext, VectorIndexParams, VectorMetric,
};
use tokio::sync::Semaphore;
use tokio::task::JoinSet;

#[tokio::main]
async fn main() {
    tracing_subscriber::fmt::init();
    let cli = Cli::parse();
    let command = cli.command_or_default();
    let result = match command {
        Command::Start(args) => run_start(args).await,
        Command::Import(args) => run_import(args).await,
        Command::Status(args) => run_status(args).await,
        Command::Prune(args) => run_prune(args).await,
        Command::Update(args) => run_update(args).await,
        Command::Setup(args) => run_setup(args),
        Command::Grammar(args) => run_grammar(args).await,
        Command::Config(args) => run_config(args).await,
        Command::Experiment(args) => run_experiment(args).await,
    };
    if let Err(err) = result {
        eprintln!("{err}");
        std::process::exit(1);
    }
}

async fn run_start(args: coco_local::cli::StartArgs) -> CocoResult<()> {
    let mut config = LocalServiceConfig::from_env()?;
    args.apply_overrides(&mut config);
    let (status, lock) =
        coco_local::lifecycle::ensure_single_instance(&config.host, config.port)?;
    if status == coco_local::lifecycle::InstanceStatus::Running {
        println!("coco-local already running on {}:{}", config.host, config.port);
        return Ok(());
    }
    let _lock_guard = lock;

    if !is_headless(args.headless) {
        prefetch_model_if_needed(&config)?;
    }

    let metrics = Arc::new(LocalMetrics::new());
    let service_metrics = Arc::clone(&metrics);
    let service_config = config.clone();
    let service_task =
        tokio::spawn(async move { coco_local::service::run(service_config, service_metrics).await });

    if is_headless(args.headless) {
        return await_service(service_task).await;
    }

    let host = config.host.clone();
    let port = config.port;
    let tui_metrics = Arc::clone(&metrics);
    let tui_task = tokio::task::spawn_blocking(move || {
        coco_local::tui::run_dashboard(tui_metrics, host, port)
    });

    let result = tokio::select! {
        result = service_task => {
            match result {
                Ok(Ok(())) => Ok(()),
                Ok(Err(err)) => Err(err),
                Err(err) => Err(CocoError::system(format!("service task failed: {err}"))),
            }
        }
        result = tui_task => {
            match result {
                Ok(Ok(())) => Ok(()),
                Ok(Err(err)) => Err(err),
                Err(err) => Err(CocoError::system(format!("tui task failed: {err}"))),
            }
        }
    };

    result
}

async fn await_service(
    service_task: tokio::task::JoinHandle<CocoResult<()>>,
) -> CocoResult<()> {
    match service_task.await {
        Ok(result) => result,
        Err(err) => Err(CocoError::system(format!("service task failed: {err}"))),
    }
}

fn run_setup(args: SetupArgs) -> CocoResult<()> {
    let config = LocalServiceConfig::from_env()?;
    let source = normalize_path(&args.model_path)?;
    if !source.is_file() {
        return Err(CocoError::user("model path must point to a file"));
    }
    let store = ModelStore::open(None)?;
    let dest = store.model_path(&config.model_file);
    std::fs::copy(&source, &dest).map_err(|err| {
        CocoError::system(format!(
            "failed to copy model to {}: {err}",
            dest.display()
        ))
    })?;
    println!("installed model at {}", dest.display());
    Ok(())
}

async fn run_import(args: coco_local::cli::ImportArgs) -> CocoResult<()> {
    let (host, port) = resolve_host_port(args.host.clone(), args.port)?;
    let base_url = format!("http://{}:{port}", host);
    let client = reqwest::Client::new();

    let project_id = resolve_project_id(&client, &base_url, &args).await?;
    let import_path = normalize_path(&args.path)?;
    let files = collect_files(&import_path, args.recursive)?;

    if files.is_empty() {
        return Err(CocoError::user("no files found to import"));
    }

    let mut imported = 0usize;
    for file in files {
        if let Err(err) = import_file(&client, &base_url, &project_id, &file, None).await {
            eprintln!("import failed for {}: {err}", file.display());
        } else {
            imported = imported.saturating_add(1);
        }
    }

    println!("imported {imported} file(s) into project {project_id}");
    Ok(())
}

async fn run_status(args: coco_local::cli::StatusArgs) -> CocoResult<()> {
    let (host, port) = resolve_host_port(args.host, args.port)?;
    let base_url = format!("http://{}:{port}", host);
    let client = reqwest::Client::new();

    let health = fetch_health(&client, &format!("{base_url}/v1/sys/health")).await;
    match health {
        Ok(health) => println!(
            "service: {} (status={})",
            health.service.unwrap_or_else(|| "unknown".to_string()),
            health.status
        ),
        Err(err) => println!("service: down ({err})"),
    }

    let meta_path = paths::meta_db_path()?;
    let store = coco_local::storage::meta::LocalMetaStore::connect(&meta_path).await?;
    let project_count = store.count_projects().await?;
    let document_count = store.count_documents().await?;
    let chunk_count = store.count_chunks().await?;

    println!("projects: {project_count}");
    println!("documents: {document_count}");
    println!("chunks: {chunk_count}");

    let projects = store.list_projects().await?;
    for project in projects {
        println!(
            "project {} | {} | {}",
            project.id, project.name, project.path
        );
    }

    Ok(())
}

async fn run_prune(args: coco_local::cli::PruneArgs) -> CocoResult<()> {
    if args.project_id.trim().is_empty() {
        return Err(CocoError::user("project_id must not be empty"));
    }
    let (host, port) = resolve_host_port(args.host, args.port)?;
    let base_url = format!("http://{}:{port}", host);
    let client = reqwest::Client::new();
    let request = PruneRequest {
        project_id: args.project_id,
        keep: args.keep,
    };
    let response = post_json::<PruneRequest, ResponseEnvelope<PruneResponse>>(
        &client,
        &format!("{base_url}/v1/sys/prune"),
        &request,
    )
    .await?;
    println!("status: {}", response.data.status);
    Ok(())
}

async fn run_update(args: UpdateArgs) -> CocoResult<()> {
    let repo = resolve_update_repo(args.repo)?;
    let info = update::check_for_update(&repo).await?;
    println!("current: {}", info.current);
    println!("latest: {}", info.latest);
    if !info.update_available {
        println!("status: up to date");
        return Ok(());
    }
    println!("status: update available");
    if let Some(asset) = &info.asset {
        println!("asset: {}", asset.name);
        println!("download: {}", asset.url);
    } else {
        println!("asset: unavailable for this platform");
    }
    if args.install {
        match update::download_and_install(&info).await? {
            update::InstallOutcome::Replaced => {
                println!("installed: updated in place");
            }
            update::InstallOutcome::Staged(path) => {
                println!("installed: staged update at {}", path.display());
                println!("note: replace the current binary to complete update");
            }
        }
    }
    Ok(())
}

async fn run_grammar(args: coco_local::cli::GrammarArgs) -> CocoResult<()> {
    let store = grammar::GrammarStore::open(None)?;
    match args.command {
        GrammarCommand::List => {
            let items = store.list_installed()?;
            if items.is_empty() {
                println!("no grammars installed");
            } else {
                for item in items {
                    println!("{item}");
                }
            }
        }
        GrammarCommand::Install(install) => install_grammar(&store, install)?,
        GrammarCommand::Update(update) => update_grammars(&store, update)?,
    }
    Ok(())
}

async fn run_config(args: coco_local::cli::ConfigArgs) -> CocoResult<()> {
    match args.command {
        ConfigCommand::List(args) => run_config_list(args).await,
        ConfigCommand::Set(args) => run_config_set(args).await,
        ConfigCommand::Use(args) => run_config_use(args).await,
    }
}

async fn run_experiment(args: coco_local::cli::ExperimentArgs) -> CocoResult<()> {
    match args.command {
        ExperimentCommand::Run(args) => run_experiment_run(args).await,
        ExperimentCommand::Compare(args) => run_experiment_compare(args),
    }
}

async fn run_experiment_run(args: ExperimentRunArgs) -> CocoResult<()> {
    let spec = ExperimentSpec::from_path(&args.spec)?;
    let (host, port) = resolve_host_port(args.host.clone(), args.port)?;
    let base_url = format!("http://{}:{port}", host);
    let client = reqwest::Client::new();

    for config in &spec.indexing_strategies {
        upsert_indexing_config(&client, &base_url, config).await?;
    }

    let local_config = LocalServiceConfig::from_env()?;
    let meta = coco_local::storage::meta::LocalMetaStore::connect(&local_config.meta_db_path)
        .await?;
    let project = meta
        .get_project(&spec.project_id)
        .await?
        .ok_or_else(|| CocoError::user("project not found"))?;
    let original_active_config_id = project.active_config_id.clone();
    let project_path = normalize_path(Path::new(&project.path))?;
    let files = collect_files(&project_path, true)?;
    if files.is_empty() {
        return Err(CocoError::user("no files found to index for experiment"));
    }

    let random_seed = spec.random_seed.unwrap_or(42);
    let concurrency = spec.concurrency.unwrap_or(1);
    let mut results = Vec::new();
    let run_result = async {
        for config in &spec.indexing_strategies {
            let indexed = reindex_project_for_config(
                &client,
                &base_url,
                &spec.project_id,
                &config.config_id,
                &files,
            )
            .await?;
            println!("indexed {indexed} files for {}", config.config_id);

            activate_indexing_config(
                &client,
                &base_url,
                &spec.project_id,
                &config.config_id,
            )
            .await?;

            for strategy in &spec.query_strategies {
                let metrics = metrics_for_strategy(&spec, strategy);
                let mut accumulators = vec![MetricAccumulator::default(); metrics.len()];
                let outcomes = run_experiment_cases(
                    &client,
                    &base_url,
                    &spec.project_id,
                    strategy,
                    &config.config_id,
                    &spec.evaluation_set,
                    concurrency,
                )
                .await?;
                for outcome in outcomes {
                    update_metric_accumulators(
                        &metrics,
                        &mut accumulators,
                        &outcome.case,
                        &outcome.doc_ids,
                        outcome.latency_ms,
                        strategy.retrieval_config.top_k,
                    );
                }

                let metric_values = metrics
                    .into_iter()
                    .zip(accumulators)
                    .map(|(metric, acc)| MetricValue {
                        kind: metric.kind,
                        k: metric.k,
                        value: acc.mean(),
                    })
                    .collect();
                results.push(ExperimentResultEntry {
                    indexing_config_id: config.config_id.clone(),
                    query_config_id: strategy.query_config_id.clone(),
                    retrieval_mode: strategy.retrieval_config.retrieval_mode,
                    version_id: None,
                    metrics: metric_values,
                });
            }
        }
        Ok::<(), CocoError>(())
    }
    .await;

    if let Err(err) = activate_indexing_config(
        &client,
        &base_url,
        &spec.project_id,
        &original_active_config_id,
    )
    .await
    {
        eprintln!("failed to restore active config: {err}");
    }

    run_result?;

    let dataset_commit = spec
        .dataset_commit
        .clone()
        .or_else(|| resolve_git_commit(&project_path));
    let run_at = Utc::now().to_rfc3339_opts(SecondsFormat::Millis, true);
    let output = ExperimentResultFile {
        name: spec.name,
        project_id: spec.project_id.clone(),
        dataset_revision: spec.dataset_revision,
        dataset_commit,
        random_seed,
        concurrency,
        run_at,
        results,
    };
    let output_path = resolve_experiment_output_path(&args.spec, args.output.as_deref());
    let json = serde_json::to_string_pretty(&output).map_err(CocoError::system)?;
    std::fs::write(&output_path, json)
        .map_err(|err| CocoError::system(format!("failed to write results: {err}")))?;
    println!("wrote results to {}", output_path.display());
    Ok(())
}

fn run_experiment_compare(args: ExperimentCompareArgs) -> CocoResult<()> {
    if args.results.is_empty() {
        return Err(CocoError::user("results file list must not be empty"));
    }
    let indexing_filter = match args.indexing_config_id.as_deref() {
        Some(value) => Some(ensure_normalized_config_id(value, "indexing_config_id")?),
        None => None,
    };
    let query_filter = match args.query_config_id.as_deref() {
        Some(value) => Some(ensure_normalized_config_id(value, "query_config_id")?),
        None => None,
    };
    let retrieval_filter = args.retrieval_mode.map(|mode| match mode {
        coco_local::cli::CliRetrievalMode::Vector => RetrievalMode::Vector,
        coco_local::cli::CliRetrievalMode::Fts => RetrievalMode::Fts,
        coco_local::cli::CliRetrievalMode::Hybrid => RetrievalMode::Hybrid,
    });

    let mut aggregations: HashMap<MetricKey, MetricAccumulator> = HashMap::new();
    for path in args.results {
        let file = load_result_file(&path)?;
        for entry in file.results {
            if let Some(filter) = indexing_filter.as_deref() {
                if entry.indexing_config_id != filter {
                    continue;
                }
            }
            if let Some(filter) = query_filter.as_deref() {
                if entry.query_config_id != filter {
                    continue;
                }
            }
            if let Some(filter) = retrieval_filter {
                if entry.retrieval_mode != filter {
                    continue;
                }
            }
            for metric in entry.metrics {
                let key = MetricKey {
                    indexing_config_id: entry.indexing_config_id.clone(),
                    query_config_id: entry.query_config_id.clone(),
                    retrieval_mode: entry.retrieval_mode,
                    metric_kind: metric.kind,
                    k: metric.k,
                };
                aggregations.entry(key).or_default().add(metric.value);
            }
        }
    }

    if aggregations.is_empty() {
        println!("no results matched filters");
        return Ok(());
    }

    let mut rows: Vec<(MetricKey, MetricAccumulator)> =
        aggregations.into_iter().collect();
    rows.sort_by(|(a, _), (b, _)| a.cmp(b));
    println!("indexing_config | query_config | retrieval_mode | metric | k | mean");
    for (key, acc) in rows {
        let k = key
            .k
            .map(|value| value.to_string())
            .unwrap_or_else(|| "-".to_string());
        println!(
            "{} | {} | {} | {} | {} | {:.4}",
            key.indexing_config_id,
            key.query_config_id,
            retrieval_mode_label(key.retrieval_mode),
            metric_kind_label(key.metric_kind),
            k,
            acc.mean()
        );
    }
    Ok(())
}

async fn run_config_list(args: coco_local::cli::ConfigListArgs) -> CocoResult<()> {
    let (host, port) = resolve_host_port(args.host, args.port)?;
    let base_url = format!("http://{}:{port}", host);
    let client = reqwest::Client::new();
    let mut url =
        reqwest::Url::parse(&format!("{base_url}/v1/sys/configs")).map_err(|err| {
            CocoError::system(format!("invalid config list url: {err}"))
        })?;
    if let Some(project_id) = args.project_id.as_deref() {
        if project_id.trim().is_empty() {
            return Err(CocoError::user("project_id must not be empty"));
        }
        url.query_pairs_mut().append_pair("project_id", project_id);
    }

    let response = get_json::<ResponseEnvelope<ConfigListResponse>>(&client, url.as_str()).await?;
    let active_config_id = response.data.active_config_id;
    let mut configs = response.data.configs;
    configs.sort_by(|a, b| a.config_id.cmp(&b.config_id));

    if let Some(active) = active_config_id.as_deref() {
        println!("active_config_id: {active}");
    }
    if configs.is_empty() {
        println!("no configs");
        return Ok(());
    }
    for config in configs {
        let marker = if active_config_id.as_deref() == Some(config.config_id.as_str()) {
            "*"
        } else {
            " "
        };
        let dims = config
            .embedding
            .dimensions
            .map(|value| value.to_string())
            .unwrap_or_else(|| "-".to_string());
        println!(
            "{marker} {} | model={} dims={} chunking={}({}/{}) metric={}",
            config.config_id,
            config.embedding.model_name,
            dims,
            config.chunking.strategy_name,
            config.chunking.chunk_size,
            config.chunking.chunk_overlap,
            vector_metric_label(config.vector_metric),
        );
    }
    Ok(())
}

async fn run_config_set(args: coco_local::cli::ConfigSetArgs) -> CocoResult<()> {
    let (host, port) = resolve_host_port(args.host.clone(), args.port)?;
    let base_url = format!("http://{}:{port}", host);
    let client = reqwest::Client::new();
    let config = build_indexing_config(args)?;

    if config.vector_backend.is_some() {
        return Err(CocoError::user(
            "vector_backend is not supported in local mode",
        ));
    }
    if config.config_id == DEFAULT_CONFIG_ID {
        return Err(CocoError::user(
            "default indexing config cannot be modified",
        ));
    }
    validate_indexing_config(&config, &ValidationContext::default())?;

    let request = ConfigUpsertRequest { config };
    let response = post_json::<ConfigUpsertRequest, ResponseEnvelope<ConfigUpsertResponse>>(
        &client,
        &format!("{base_url}/v1/sys/configs"),
        &request,
    )
    .await?;
    println!(
        "saved config {}",
        response.data.config.config_id
    );
    Ok(())
}

async fn run_config_use(args: coco_local::cli::ConfigUseArgs) -> CocoResult<()> {
    let (host, port) = resolve_host_port(args.host, args.port)?;
    let base_url = format!("http://{}:{port}", host);
    let client = reqwest::Client::new();
    if args.project_id.trim().is_empty() {
        return Err(CocoError::user("project_id must not be empty"));
    }
    let config_id = ensure_normalized_config_id(&args.config_id, "config_id")?;

    let request = ConfigActivateRequest {
        project_id: args.project_id,
        config_id,
    };
    let response = post_json::<ConfigActivateRequest, ResponseEnvelope<ConfigActivateResponse>>(
        &client,
        &format!("{base_url}/v1/sys/configs/activate"),
        &request,
    )
    .await?;
    println!("active_config_id: {}", response.data.active_config_id);
    Ok(())
}

fn install_grammar(store: &grammar::GrammarStore, args: GrammarInstallArgs) -> CocoResult<()> {
    let spec = grammar_spec_from_args(&args.lang, args.url, args.base_url, args.sha256)?;
    let path = store.ensure_grammar(&spec, None)?;
    println!("installed {}", path.display());
    Ok(())
}

fn update_grammars(store: &grammar::GrammarStore, args: GrammarUpdateArgs) -> CocoResult<()> {
    let base_url = resolve_base_url(args.base_url)?;
    let items = store.list_installed()?;
    if items.is_empty() {
        println!("no grammars installed");
        return Ok(());
    }
    for item in items {
        let lang = item.trim_end_matches(".wasm");
        let spec = grammar_spec_from_args(lang, None, Some(base_url.clone()), None)?;
        let path = store.grammar_path(&spec.file_name);
        if path.exists() {
            let _ = std::fs::remove_file(&path);
        }
        let path = store.ensure_grammar(&spec, None)?;
        println!("updated {}", path.display());
    }
    Ok(())
}

async fn import_file(
    client: &reqwest::Client,
    base_url: &str,
    project_id: &str,
    file: &std::path::Path,
    indexing_config_id: Option<&str>,
) -> CocoResult<()> {
    let normalized = normalize_path(file)?;
    let content = tokio::fs::read_to_string(&normalized)
        .await
        .map_err(|err| {
            CocoError::system(format!("failed to read file {}: {err}", normalized.display()))
        })?;
    let file_type = file_type_for_path(&normalized);
    let title = title_for_path(&normalized);
    let path = path_to_string(&normalized)?;

    let request = ImportRequest {
        project_id: project_id.to_string(),
        indexing_config_id: indexing_config_id.map(|value| value.to_string()),
        indexing_config: None,
        document_id: None,
        content,
        file_type,
        title,
        path: Some(path),
        chunking: None,
    };

    let response = post_json::<ImportRequest, ImportResponse>(
        client,
        &format!("{base_url}/v1/docs/import"),
        &request,
    )
    .await?;
    let _ = response.document_id;
    Ok(())
}

async fn resolve_project_id(
    client: &reqwest::Client,
    base_url: &str,
    args: &coco_local::cli::ImportArgs,
) -> CocoResult<String> {
    if let Some(project_id) = &args.project_id {
        return Ok(project_id.clone());
    }
    let project_path = match &args.project_path {
        Some(path) => normalize_path(path)?,
        None => {
            if args.path.is_dir() {
                normalize_path(&args.path)?
            } else {
                return Err(CocoError::user(
                    "project id or project path is required when importing a file",
                ));
            }
        }
    };
    let name = args
        .project_name
        .clone()
        .or_else(|| {
            project_path
                .file_name()
                .and_then(|name| name.to_str())
                .map(|name| name.to_string())
        })
        .unwrap_or_else(|| "project".to_string());
    let path = path_to_string(&project_path)?;
    let request = RegisterProjectRequest {
        id: None,
        name,
        path,
    };
    let response = post_json::<RegisterProjectRequest, RegisterProjectResponse>(
        client,
        &format!("{base_url}/v1/sys/register"),
        &request,
    )
    .await?;
    Ok(response.project_id)
}

fn resolve_host_port(
    host: Option<String>,
    port: Option<u16>,
) -> CocoResult<(String, u16)> {
    let mut config = LocalServiceConfig::from_env()?;
    if let Some(host) = host {
        config.host = host;
    }
    if let Some(port) = port {
        config.port = port;
    }
    Ok((config.host, config.port))
}

fn resolve_update_repo(repo: Option<String>) -> CocoResult<String> {
    if let Some(repo) = repo {
        return Ok(repo);
    }
    match std::env::var("COCO_RELEASE_REPO") {
        Ok(value) if !value.is_empty() => Ok(value),
        _ => Err(CocoError::user(
            "update repo required (use --repo or COCO_RELEASE_REPO)",
        )),
    }
}

async fn fetch_health(
    client: &reqwest::Client,
    url: &str,
) -> CocoResult<HealthResponse> {
    get_json::<HealthResponse>(client, url).await
}

fn grammar_spec_from_args(
    lang: &str,
    url: Option<String>,
    base_url: Option<String>,
    sha256: Option<String>,
) -> CocoResult<grammar::GrammarSpec> {
    let file_name = format!("{lang}.wasm");
    let resolved = match url {
        Some(url) => Some(url),
        None => base_url.map(|base| {
            let base = base.trim_end_matches('/');
            format!("{base}/{file_name}")
        }),
    };
    if resolved.is_none() {
        return Err(CocoError::user(
            "grammar url is required (use --url or --base-url)",
        ));
    }
    Ok(grammar::GrammarSpec {
        name: lang.to_string(),
        file_name,
        sha256,
        url: resolved,
        language_export: None,
    })
}

fn resolve_base_url(base_url: Option<String>) -> CocoResult<String> {
    if let Some(url) = base_url {
        return Ok(url);
    }
    match std::env::var("COCO_GRAMMAR_BASE_URL") {
        Ok(value) if !value.is_empty() => Ok(value),
        _ => Err(CocoError::user(
            "COCO_GRAMMAR_BASE_URL must be set to update grammars",
        )),
    }
}

async fn post_json<T, R>(client: &reqwest::Client, url: &str, payload: &T) -> CocoResult<R>
where
    T: Serialize + ?Sized,
    R: for<'de> Deserialize<'de>,
{
    let body = serde_json::to_vec(payload).map_err(CocoError::system)?;
    let response = client
        .post(url)
        .header("content-type", "application/json")
        .body(body)
        .send()
        .await
        .map_err(|err| CocoError::network(format!("request failed: {err}")))?;
    parse_response(response).await
}

async fn get_json<R>(client: &reqwest::Client, url: &str) -> CocoResult<R>
where
    R: for<'de> Deserialize<'de>,
{
    let response = client
        .get(url)
        .send()
        .await
        .map_err(|err| CocoError::network(format!("request failed: {err}")))?;
    parse_response(response).await
}

async fn parse_response<R>(response: reqwest::Response) -> CocoResult<R>
where
    R: for<'de> Deserialize<'de>,
{
    let status = response.status();
    let bytes = response
        .bytes()
        .await
        .map_err(|err| CocoError::network(format!("response read failed: {err}")))?;
    if !status.is_success() {
        if let Ok(error) = serde_json::from_slice::<ErrorResponse>(&bytes) {
            return Err(map_error_response(error));
        }
        let body = String::from_utf8_lossy(&bytes);
        return Err(CocoError::system(format!(
            "request failed with status {}: {}",
            status.as_u16(),
            body
        )));
    }
    serde_json::from_slice::<R>(&bytes).map_err(CocoError::system)
}

fn map_error_response(error: ErrorResponse) -> CocoError {
    match error.kind {
        CocoErrorKind::User => CocoError::user(error.message),
        CocoErrorKind::Network => CocoError::network(error.message),
        CocoErrorKind::Storage => CocoError::storage(error.message),
        CocoErrorKind::Compute => CocoError::compute(error.message),
        CocoErrorKind::System => CocoError::system(error.message),
    }
}

async fn upsert_indexing_config(
    client: &reqwest::Client,
    base_url: &str,
    config: &IndexingConfig,
) -> CocoResult<IndexingConfig> {
    let request = ConfigUpsertRequest {
        config: config.clone(),
    };
    let response = post_json::<ConfigUpsertRequest, ResponseEnvelope<ConfigUpsertResponse>>(
        client,
        &format!("{base_url}/v1/sys/configs"),
        &request,
    )
    .await?;
    Ok(response.data.config)
}

async fn activate_indexing_config(
    client: &reqwest::Client,
    base_url: &str,
    project_id: &str,
    config_id: &str,
) -> CocoResult<String> {
    let request = ConfigActivateRequest {
        project_id: project_id.to_string(),
        config_id: config_id.to_string(),
    };
    let response = post_json::<ConfigActivateRequest, ResponseEnvelope<ConfigActivateResponse>>(
        client,
        &format!("{base_url}/v1/sys/configs/activate"),
        &request,
    )
    .await?;
    Ok(response.data.active_config_id)
}

async fn reindex_project_for_config(
    client: &reqwest::Client,
    base_url: &str,
    project_id: &str,
    config_id: &str,
    files: &[PathBuf],
) -> CocoResult<usize> {
    let mut imported = 0usize;
    for file in files {
        import_file(client, base_url, project_id, file, Some(config_id)).await?;
        imported = imported.saturating_add(1);
    }
    Ok(imported)
}

async fn run_experiment_cases(
    client: &reqwest::Client,
    base_url: &str,
    project_id: &str,
    strategy: &QueryStrategy,
    config_id: &str,
    cases: &[EvaluationCase],
    concurrency: u32,
) -> CocoResult<Vec<CaseOutcome>> {
    let limit = usize::try_from(concurrency).unwrap_or(1).max(1);
    if limit == 1 {
        let mut outcomes = Vec::new();
        for case in cases {
            outcomes.push(
                run_experiment_case(client, base_url, project_id, strategy, config_id, case)
                    .await?,
            );
        }
        return Ok(outcomes);
    }

    let semaphore = Arc::new(Semaphore::new(limit));
    let mut join_set = JoinSet::new();
    for case in cases {
        let permit = semaphore.clone().acquire_owned().await.map_err(|_| {
            CocoError::system("failed to acquire experiment concurrency permit")
        })?;
        let client = client.clone();
        let base_url = base_url.to_string();
        let project_id = project_id.to_string();
        let strategy = strategy.clone();
        let config_id = config_id.to_string();
        let case = case.clone();
        join_set.spawn(async move {
            let _permit = permit;
            run_experiment_case(
                &client,
                &base_url,
                &project_id,
                &strategy,
                &config_id,
                &case,
            )
            .await
        });
    }

    let mut outcomes = Vec::new();
    while let Some(result) = join_set.join_next().await {
        let outcome = result.map_err(|err| {
            CocoError::system(format!("experiment task failed: {err}"))
        })??;
        outcomes.push(outcome);
    }
    Ok(outcomes)
}

async fn run_experiment_case(
    client: &reqwest::Client,
    base_url: &str,
    project_id: &str,
    strategy: &QueryStrategy,
    config_id: &str,
    case: &EvaluationCase,
) -> CocoResult<CaseOutcome> {
    let intent = build_experiment_intent(case, strategy, config_id);
    let request = QueryRequest {
        intent,
        project_id: Some(project_id.to_string()),
        indexing_config_id: None,
        retrieval_config: None,
        indexing_config: None,
    };
    let start = Instant::now();
    let response = post_json::<QueryRequest, ResponseEnvelope<QueryResponse>>(
        client,
        &format!("{base_url}/v1/docs/query"),
        &request,
    )
    .await?;
    let latency_ms = start.elapsed().as_secs_f32() * 1000.0;
    let doc_ids: Vec<String> = response
        .data
        .results
        .into_iter()
        .map(|hit| hit.chunk.doc_id.to_string())
        .collect();
    Ok(CaseOutcome {
        case: case.clone(),
        doc_ids,
        latency_ms,
    })
}

fn build_experiment_intent(
    case: &EvaluationCase,
    strategy: &QueryStrategy,
    config_id: &str,
) -> SearchIntentInput {
    let query = match strategy.retrieval_config.retrieval_mode {
        RetrievalMode::Vector => {
            SearchQueryInput::vector(Some(case.query.clone()), None).expect("query")
        }
        RetrievalMode::Fts => SearchQueryInput::fts(case.query.clone()).expect("query"),
        RetrievalMode::Hybrid => {
            SearchQueryInput::hybrid(case.query.clone(), None).expect("query")
        }
    };
    SearchIntentInput::new(
        query,
        Some(config_id.to_string()),
        strategy.retrieval_config.top_k,
        strategy.retrieval_config.hybrid_alpha,
        Vec::new(),
        strategy.retrieval_config.reranker.clone(),
    )
    .expect("intent")
}

fn metrics_for_strategy(spec: &ExperimentSpec, strategy: &QueryStrategy) -> Vec<MetricSpec> {
    if spec.metrics.is_empty() {
        return vec![
            MetricSpec {
                kind: MetricKind::RecallAtK,
                k: Some(strategy.retrieval_config.top_k),
            },
            MetricSpec {
                kind: MetricKind::Mrr,
                k: None,
            },
            MetricSpec {
                kind: MetricKind::HitRate,
                k: None,
            },
            MetricSpec {
                kind: MetricKind::LatencyMs,
                k: None,
            },
        ];
    }
    spec.metrics.clone()
}

fn update_metric_accumulators(
    metrics: &[MetricSpec],
    accumulators: &mut [MetricAccumulator],
    case: &EvaluationCase,
    doc_ids: &[String],
    latency_ms: f32,
    fallback_k: u32,
) {
    let expected: HashSet<&str> = case
        .expected_doc_ids
        .iter()
        .map(String::as_str)
        .collect();

    let mut first_rank = None;
    for (index, doc_id) in doc_ids.iter().enumerate() {
        if expected.contains(doc_id.as_str()) {
            first_rank = Some(index + 1);
            break;
        }
    }
    let hit_rate = if first_rank.is_some() { 1.0 } else { 0.0 };
    let mrr = first_rank
        .map(|rank| 1.0 / rank as f32)
        .unwrap_or(0.0);

    for (metric, accumulator) in metrics.iter().zip(accumulators.iter_mut()) {
        let value = match metric.kind {
            MetricKind::RecallAtK => {
                let k = metric.k.unwrap_or(fallback_k);
                let limit = k as usize;
                let mut seen = HashSet::new();
                for doc_id in doc_ids.iter().take(limit) {
                    let doc_id = doc_id.as_str();
                    if expected.contains(doc_id) {
                        seen.insert(doc_id);
                    }
                }
                if expected.is_empty() {
                    0.0
                } else {
                    seen.len() as f32 / expected.len() as f32
                }
            }
            MetricKind::Mrr => mrr,
            MetricKind::HitRate => hit_rate,
            MetricKind::LatencyMs => latency_ms,
        };
        accumulator.add(value);
    }
}

fn resolve_experiment_output_path(spec: &Path, output: Option<&Path>) -> PathBuf {
    match output {
        Some(path) => path.to_path_buf(),
        None => {
            let parent = spec.parent().unwrap_or_else(|| Path::new("."));
            parent.join("results.json")
        }
    }
}

fn load_result_file(path: &Path) -> CocoResult<ExperimentResultFile> {
    let content = std::fs::read_to_string(path).map_err(|err| {
        CocoError::system(format!("failed to read results file: {err}"))
    })?;
    serde_json::from_str::<ExperimentResultFile>(&content)
        .map_err(|err| CocoError::user(format!("invalid results json: {err}")))
}

fn metric_kind_label(kind: MetricKind) -> &'static str {
    match kind {
        MetricKind::RecallAtK => "recall_at_k",
        MetricKind::Mrr => "mrr",
        MetricKind::HitRate => "hit_rate",
        MetricKind::LatencyMs => "latency_ms",
    }
}

fn retrieval_mode_label(mode: RetrievalMode) -> &'static str {
    match mode {
        RetrievalMode::Vector => "vector",
        RetrievalMode::Fts => "fts",
        RetrievalMode::Hybrid => "hybrid",
    }
}

fn resolve_git_commit(project_root: &Path) -> Option<String> {
    let git_dir = resolve_git_dir(project_root)?;
    let head = std::fs::read_to_string(git_dir.join("HEAD")).ok()?;
    let head = head.trim();
    if let Some(reference) = head.strip_prefix("ref: ") {
        let ref_path = git_dir.join(reference);
        if let Ok(hash) = std::fs::read_to_string(&ref_path) {
            return Some(hash.trim().to_string());
        }
        let packed_refs = std::fs::read_to_string(git_dir.join("packed-refs")).ok()?;
        for line in packed_refs.lines() {
            let line = line.trim();
            if line.starts_with('#') || line.starts_with('^') || line.is_empty() {
                continue;
            }
            if let Some((hash, name)) = line.split_once(' ') {
                if name == reference {
                    return Some(hash.to_string());
                }
            }
        }
        None
    } else if head.len() >= 7 {
        Some(head.to_string())
    } else {
        None
    }
}

fn resolve_git_dir(project_root: &Path) -> Option<PathBuf> {
    let direct = project_root.join(".git");
    if direct.is_dir() {
        return Some(direct);
    }
    if direct.is_file() {
        let content = std::fs::read_to_string(&direct).ok()?;
        let content = content.trim();
        if let Some(path) = content.strip_prefix("gitdir: ") {
            return Some(project_root.join(path.trim()));
        }
    }
    None
}

#[derive(Debug, Clone, Copy, Default)]
struct MetricAccumulator {
    sum: f32,
    count: u32,
}

#[derive(Debug, Clone, Eq, PartialEq)]
struct MetricKey {
    indexing_config_id: String,
    query_config_id: String,
    retrieval_mode: RetrievalMode,
    metric_kind: MetricKind,
    k: Option<u32>,
}

#[derive(Debug, Clone)]
struct CaseOutcome {
    case: EvaluationCase,
    doc_ids: Vec<String>,
    latency_ms: f32,
}

impl Hash for MetricKey {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.indexing_config_id.hash(state);
        self.query_config_id.hash(state);
        retrieval_mode_label(self.retrieval_mode).hash(state);
        metric_kind_label(self.metric_kind).hash(state);
        self.k.hash(state);
    }
}

impl Ord for MetricKey {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        (
            &self.indexing_config_id,
            &self.query_config_id,
            retrieval_mode_label(self.retrieval_mode),
            metric_kind_label(self.metric_kind),
            self.k,
        )
            .cmp(&(
                &other.indexing_config_id,
                &other.query_config_id,
                retrieval_mode_label(other.retrieval_mode),
                metric_kind_label(other.metric_kind),
                other.k,
            ))
    }
}

impl PartialOrd for MetricKey {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

impl MetricAccumulator {
    fn add(&mut self, value: f32) {
        self.sum += value;
        self.count = self.count.saturating_add(1);
    }

    fn mean(self) -> f32 {
        if self.count == 0 {
            0.0
        } else {
            self.sum / self.count as f32
        }
    }
}

fn build_indexing_config(args: coco_local::cli::ConfigSetArgs) -> CocoResult<IndexingConfig> {
    let coco_local::cli::ConfigSetArgs {
        file,
        config_id,
        chunking_strategy,
        chunk_size,
        chunk_overlap,
        embedding_model,
        embedding_dimensions,
        vector_metric,
        index_params,
        host: _,
        port: _,
    } = args;

    if let Some(file) = file {
        if config_id.is_some()
            || chunking_strategy.is_some()
            || chunk_size.is_some()
            || chunk_overlap.is_some()
            || embedding_model.is_some()
            || embedding_dimensions.is_some()
            || vector_metric.is_some()
            || index_params.is_some()
        {
            return Err(CocoError::user(
                "config fields are not allowed when --file is provided",
            ));
        }
        let content = std::fs::read_to_string(&file).map_err(|err| {
            CocoError::system(format!("failed to read {}: {err}", file.display()))
        })?;
        let config = serde_json::from_str::<IndexingConfig>(&content).map_err(|err| {
            CocoError::user(format!("invalid config json: {err}"))
        })?;
        let _ = ensure_normalized_config_id(&config.config_id, "config_id")?;
        return Ok(config);
    }

    let config_id = config_id
        .ok_or_else(|| CocoError::user("config_id is required"))?;
    let config_id = ensure_normalized_config_id(&config_id, "config_id")?;
    let defaults = LocalServiceConfig::from_env()?;
    let chunking = ChunkingStrategy {
        strategy_name: chunking_strategy
            .unwrap_or_else(|| defaults.chunking.strategy_name.clone()),
        chunk_size: chunk_size.unwrap_or(defaults.chunking.chunk_size),
        chunk_overlap: chunk_overlap
            .unwrap_or(defaults.chunking.chunk_overlap),
    };
    let embedding = EmbeddingConfig {
        model_name: embedding_model
            .unwrap_or_else(|| defaults.model_name.clone()),
        dimensions: Some(embedding_dimensions.unwrap_or(
            defaults.model_dimensions as u32,
        )),
    };
    let vector_metric = vector_metric
        .map(VectorMetric::from)
        .unwrap_or(VectorMetric::L2);
    let index_params = match index_params {
        Some(path) => {
            let content = std::fs::read_to_string(&path).map_err(|err| {
                CocoError::system(format!(
                    "failed to read {}: {err}",
                    path.display()
                ))
            })?;
            Some(
                serde_json::from_str::<VectorIndexParams>(&content).map_err(|err| {
                    CocoError::user(format!("invalid index params json: {err}"))
                })?,
            )
        }
        None => None,
    };

    Ok(IndexingConfig {
        config_id,
        chunking,
        embedding,
        vector_metric,
        index_params,
        vector_backend: None,
    })
}

fn ensure_normalized_config_id(value: &str, field: &str) -> CocoResult<String> {
    let normalized = normalize_config_id(value)?;
    if normalized != value {
        return Err(CocoError::user(format!("{field} must be normalized")));
    }
    Ok(normalized)
}

fn vector_metric_label(metric: VectorMetric) -> &'static str {
    match metric {
        VectorMetric::Cosine => "cosine",
        VectorMetric::Dot => "dot",
        VectorMetric::L2 => "l2",
    }
}

fn prefetch_model_if_needed(config: &LocalServiceConfig) -> CocoResult<()> {
    if embedder_mode_is_stub() {
        return Ok(());
    }
    if let Some(path) = &config.model_path {
        if !path.exists() {
            return Err(CocoError::user("COCO_MODEL_PATH does not exist"));
        }
        return Ok(());
    }

    let store = ModelStore::open(None)?;
    let spec = config.model_spec();
    let path = store.model_path(&spec.file_name);
    if path.exists() {
        return Ok(());
    }

    let progress = Arc::new(DownloadProgress::new());
    let progress_for_download = Arc::clone(&progress);
    let progress_for_tui = Arc::clone(&progress);
    let model_name = spec.name.clone();
    let override_url = config.model_url.clone();
    let download_spec = spec.clone();

    let download_handle = std::thread::spawn(move || {
        let _guard = ProgressGuard {
            progress: Arc::clone(&progress_for_download),
        };
        store.ensure_model(
            &download_spec,
            override_url.as_deref(),
            Some(progress_for_download.as_ref()),
        )
    });

    let tui_result = coco_local::tui::run_model_download(progress_for_tui, model_name);
    tui_result?;
    download_handle
        .join()
        .map_err(|_| CocoError::system("model download thread panicked"))??;
    Ok(())
}

fn embedder_mode_is_stub() -> bool {
    match std::env::var("COCO_EMBEDDER_MODE") {
        Ok(value) => matches!(value.to_ascii_lowercase().as_str(), "stub" | "mock"),
        Err(_) => false,
    }
}

fn is_headless(cli_headless: bool) -> bool {
    cli_headless || env_flag("COCO_HEADLESS") || env_flag("CI")
}

fn env_flag(key: &str) -> bool {
    match std::env::var(key) {
        Ok(value) => matches!(value.to_ascii_lowercase().as_str(), "1" | "true" | "yes"),
        Err(_) => false,
    }
}

struct ProgressGuard {
    progress: Arc<DownloadProgress>,
}

impl Drop for ProgressGuard {
    fn drop(&mut self) {
        self.progress.mark_done();
    }
}

#[derive(Debug, Serialize)]
struct RegisterProjectRequest {
    #[serde(skip_serializing_if = "Option::is_none")]
    id: Option<String>,
    name: String,
    path: String,
}

#[derive(Debug, Deserialize)]
struct RegisterProjectResponse {
    project_id: String,
}

#[derive(Debug, Serialize)]
struct ImportRequest {
    project_id: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    indexing_config_id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    indexing_config: Option<IndexingConfig>,
    #[serde(skip_serializing_if = "Option::is_none")]
    document_id: Option<String>,
    content: String,
    file_type: FileType,
    #[serde(skip_serializing_if = "Option::is_none")]
    title: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    path: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    chunking: Option<ChunkingStrategy>,
}

#[derive(Debug, Deserialize)]
struct ImportResponse {
    document_id: String,
}

#[derive(Debug, Serialize)]
struct QueryRequest {
    intent: SearchIntentInput,
    #[serde(skip_serializing_if = "Option::is_none")]
    project_id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    indexing_config_id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    retrieval_config: Option<RetrievalConfig>,
    #[serde(skip_serializing_if = "Option::is_none")]
    indexing_config: Option<IndexingConfig>,
}

#[derive(Debug, Deserialize)]
struct QueryResponse {
    results: Vec<SearchHit>,
}

#[derive(Debug, Serialize)]
struct ConfigUpsertRequest {
    config: IndexingConfig,
}

#[derive(Debug, Deserialize)]
struct ConfigUpsertResponse {
    config: IndexingConfig,
}

#[derive(Debug, Deserialize)]
struct ConfigListResponse {
    configs: Vec<IndexingConfig>,
    active_config_id: Option<String>,
}

#[derive(Debug, Serialize)]
struct ConfigActivateRequest {
    project_id: String,
    config_id: String,
}

#[derive(Debug, Deserialize)]
struct ConfigActivateResponse {
    active_config_id: String,
}

#[derive(Debug, Serialize)]
struct PruneRequest {
    project_id: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    keep: Option<usize>,
}

#[derive(Debug, Deserialize)]
struct PruneResponse {
    status: String,
}

#[derive(Debug, Deserialize)]
struct HealthResponse {
    status: String,
    service: Option<String>,
}
