use crate::discovery::{DiscoveredDoc, discover_documents};
use crate::http_client::HttpClient;
use crate::models::{DocumentRef, Manifest, ManifestDocument, SourceConfig};
use crate::storage::{delete_document, load_manifest, save_manifest, write_document};
use crate::util::{
    CrawlStats, collapse_index_path, ensure_within_dir, iso_now, normalize_url, path_to_posix,
    safe_relative_path, sha256_bytes, split_language_from_path,
};
use log::{info, warn};
use rayon::prelude::*;
use std::collections::{HashMap, HashSet};
use std::fs;
use std::path::{Path, PathBuf};
use std::sync::atomic::{AtomicUsize, Ordering};

pub struct Crawler {
    allowed_extensions: Vec<String>,
    user_agent: String,
    timeout_seconds: u64,
    max_workers: usize,
}

impl Crawler {
    pub fn new(
        allowed_extensions: &[String],
        user_agent: &str,
        timeout_seconds: u64,
        max_workers: usize,
    ) -> Self {
        Self {
            allowed_extensions: allowed_extensions.to_vec(),
            user_agent: user_agent.to_string(),
            timeout_seconds,
            max_workers,
        }
    }

    pub fn crawl_source(
        &self,
        source: &SourceConfig,
        prune: bool,
        dry_run: bool,
    ) -> Result<CrawlStats, String> {
        let output_root = &source.output_dir;
        let output_root_abs = absolute_path(output_root);
        info!(
            "crawl start source={} output={}",
            source.id,
            output_root.display()
        );
        if dry_run {
            info!("dry run enabled source={}", source.id);
        }
        if prune {
            info!("prune enabled source={}", source.id);
        }

        let allowed_extensions = source
            .allowed_extensions
            .as_ref()
            .unwrap_or(&self.allowed_extensions)
            .to_vec();

        let manifest_path = output_root.join(".manifest.json");
        let mut manifest = load_manifest(&manifest_path)?.unwrap_or_else(|| Manifest {
            version: 1,
            source_id: source.id.clone(),
            generated_at: iso_now(),
            entrypoints: Vec::new(),
            documents: HashMap::new(),
            documents_order: Vec::new(),
        });

        normalize_manifest_paths(&mut manifest, output_root, &output_root_abs);

        let http = HttpClient::new(self.timeout_seconds, &self.user_agent)
            .map_err(|err| format!("Failed to init HTTP client: {err}"))?;

        let mut discovered_urls = HashSet::new();
        let mut discovered_language_hints: HashMap<String, HashSet<String>> = HashMap::new();
        let mut discovered_in_order: Vec<DiscoveredDoc> = Vec::new();

        for entrypoint in &source.entrypoints {
            info!(
                "discover start source={} type={}",
                source.id, entrypoint.kind
            );
            let cache_dir = if entrypoint.kind == "github_tree" {
                Some(
                    entrypoint
                        .git_cache_dir
                        .as_deref()
                        .map(PathBuf::from)
                        .unwrap_or_else(|| source.output_dir.join(".git-cache")),
                )
            } else {
                None
            };

            let discovered_batch = discover_documents(
                entrypoint,
                &allowed_extensions,
                &http,
                &source.default_language,
                cache_dir.as_deref(),
            )
            .map_err(|err| err.to_string())?;

            let before = discovered_urls.len();
            for discovered in discovered_batch {
                let url = normalize_url(&discovered.url);
                if discovered_urls.insert(url.clone()) {
                    discovered_in_order.push(DiscoveredDoc {
                        url: url.clone(),
                        language_hint: discovered.language_hint.clone(),
                        relative_path: discovered.relative_path.clone(),
                    });
                }
                if let Some(hint) = discovered.language_hint {
                    discovered_language_hints
                        .entry(url)
                        .or_default()
                        .insert(hint);
                }
            }
            let added = discovered_urls.len().saturating_sub(before);
            info!(
                "discover done source={} added={} total={}",
                source.id,
                added,
                discovered_urls.len()
            );
        }

        let docs = build_document_refs(source, &discovered_in_order, &discovered_language_hints);
        let mut stats = CrawlStats {
            discovered: docs.len(),
            ..CrawlStats::default()
        };

        let total_docs = docs.len();
        info!(
            "fetch start source={} total={} workers={}",
            source.id, total_docs, self.max_workers
        );

        let mut tasks = Vec::with_capacity(docs.len());
        for doc in &docs {
            let mut if_none_match = None;
            let mut if_modified_since = None;
            if let Some(previous) = manifest.documents.get(&doc.url) {
                let previous_path = if previous.local_path.is_empty() {
                    None
                } else {
                    Some(PathBuf::from(&previous.local_path))
                };
                let previous_path = previous_path
                    .map(|path| ensure_within_dir(output_root, &output_root.join(path)))
                    .transpose()?;
                let target_path = ensure_within_dir(output_root, &doc.local_path)?;
                let cache_ok = previous_path
                    .as_ref()
                    .map(|path| path.exists())
                    .unwrap_or(false)
                    || target_path.exists();
                if cache_ok {
                    if_none_match = previous.etag.clone();
                    if_modified_since = previous.last_modified.clone();
                }
            }
            tasks.push(FetchTask {
                doc: doc.clone(),
                if_none_match,
                if_modified_since,
            });
        }

        let progress_every = progress_interval(total_docs);
        let completed = AtomicUsize::new(0);
        let pool = rayon::ThreadPoolBuilder::new()
            .num_threads(self.max_workers)
            .build()
            .map_err(|err| err.to_string())?;

        let results: Vec<(String, crate::http_client::FetchResult)> = pool.install(|| {
            tasks
                .par_iter()
                .map(|task| {
                    let result = http.fetch_bytes(
                        &task.doc.url,
                        None,
                        task.if_none_match.as_deref(),
                        task.if_modified_since.as_deref(),
                        None,
                    );
                    let done = completed.fetch_add(1, Ordering::SeqCst) + 1;
                    if done % progress_every == 0 || done == total_docs {
                        info!(
                            "fetch progress source={} {}/{}",
                            source.id, done, total_docs
                        );
                    }
                    (task.doc.url.clone(), result)
                })
                .collect()
        });

        let mut results_map: HashMap<String, crate::http_client::FetchResult> =
            results.into_iter().collect();

        let now = iso_now();
        for doc in &docs {
            let Some(mut result) = results_map.remove(&doc.url) else {
                continue;
            };

            let mut previous = manifest.documents.remove(&doc.url);
            if result.status == 304 {
                if let Some(mut prev) = previous.take() {
                    let target_path = ensure_within_dir(output_root, &doc.local_path)?;
                    let previous_path = if prev.local_path.is_empty() {
                        None
                    } else {
                        Some(PathBuf::from(&prev.local_path))
                    };
                    let previous_path = previous_path
                        .map(|path| ensure_within_dir(output_root, &output_root.join(path)))
                        .transpose()?;

                    if !target_path.exists()
                        && previous_path
                            .as_ref()
                            .map(|path| path.exists())
                            .unwrap_or(false)
                    {
                        if !dry_run {
                            if let Some(parent) = target_path.parent() {
                                fs::create_dir_all(parent).map_err(|err| {
                                    format!("Failed to create dir {}: {err}", parent.display())
                                })?;
                            }
                            if let Some(previous_path) = previous_path.as_ref() {
                                fs::rename(previous_path, &target_path).map_err(|err| {
                                    format!(
                                        "Failed to move {} -> {}: {err}",
                                        previous_path.display(),
                                        target_path.display()
                                    )
                                })?;
                            }
                        }
                        let relative_target = target_path
                            .strip_prefix(&output_root_abs)
                            .unwrap_or(&target_path);
                        prev.local_path = path_to_posix(relative_target);
                        prev.checked_at = Some(now.clone());
                        prev.status = Some("moved".to_string());
                        manifest.documents.insert(doc.url.clone(), prev);
                        stats.updated += 1;
                        continue;
                    }

                    if !target_path.exists() {
                        let refreshed = http.fetch_bytes(&doc.url, None, None, None, None);
                        result = refreshed;
                        prev.checked_at = Some(now.clone());
                        prev.status = Some("refetched".to_string());
                        previous = Some(prev);
                    } else {
                        prev.checked_at = Some(now.clone());
                        prev.status = Some("not_modified".to_string());
                        manifest.documents.insert(doc.url.clone(), prev);
                        stats.unchanged += 1;
                        continue;
                    }
                }
            }

            let local_path = ensure_within_dir(output_root, &doc.local_path)?;
            let relative_local = local_path
                .strip_prefix(&output_root_abs)
                .unwrap_or(&local_path);
            let relative_local_path = path_to_posix(relative_local);

            if result.status != 200 || result.body.is_none() {
                warn!(
                    "fetch error source={} url={} status={} error={}",
                    source.id,
                    doc.url,
                    result.status,
                    result.error.clone().unwrap_or_default()
                );
                let mut entry = previous.unwrap_or_else(|| ManifestDocument {
                    url: doc.url.clone(),
                    local_path: relative_local_path.clone(),
                    language: doc.language.clone(),
                    sha256: String::new(),
                    bytes: 0,
                    ..ManifestDocument::default()
                });
                entry.checked_at = Some(now.clone());
                entry.status = Some("error".to_string());
                entry.error = Some(
                    result
                        .error
                        .unwrap_or_else(|| format!("status={}", result.status)),
                );
                manifest.documents.insert(doc.url.clone(), entry);
                stats.errors += 1;
                continue;
            }

            let body = result.body.take().unwrap_or_default();
            let sha = sha256_bytes(&body);

            let path_changed = previous
                .as_ref()
                .map(|doc| !doc.local_path.is_empty() && doc.local_path != relative_local_path)
                .unwrap_or(false);
            let changed = path_changed
                || previous
                    .as_ref()
                    .map(|doc| doc.sha256 != sha)
                    .unwrap_or(true)
                || !local_path.exists();

            if changed && !dry_run {
                if path_changed {
                    if let Some(prev) = previous.as_ref() {
                        if !prev.local_path.is_empty() {
                            let previous_path = ensure_within_dir(
                                output_root,
                                &output_root.join(PathBuf::from(&prev.local_path)),
                            )?;
                            if previous_path.exists() && previous_path != local_path {
                                delete_document(&previous_path)?;
                            }
                        }
                    }
                }
                write_document(&local_path, &body)?;
            }

            let entry = ManifestDocument {
                url: doc.url.clone(),
                local_path: relative_local_path,
                language: doc.language.clone(),
                sha256: sha,
                bytes: body.len(),
                etag: header_value(&result.headers, "etag"),
                last_modified: header_value(&result.headers, "last-modified"),
                fetched_at: Some(now.clone()),
                checked_at: Some(now.clone()),
                status: Some(if changed {
                    "updated".to_string()
                } else {
                    "unchanged".to_string()
                }),
                error: None,
            };
            manifest.documents.insert(doc.url.clone(), entry);

            stats.fetched += 1;
            if changed {
                stats.updated += 1;
            } else {
                stats.unchanged += 1;
            }
        }

        if prune {
            let to_remove: Vec<String> = manifest
                .documents
                .keys()
                .filter(|url| !discovered_urls.contains(*url))
                .cloned()
                .collect();
            for url in &to_remove {
                if let Some(doc) = manifest.documents.get(url) {
                    if !dry_run {
                        let path = ensure_within_dir(
                            output_root,
                            &output_root.join(PathBuf::from(&doc.local_path)),
                        )?;
                        delete_document(&path)?;
                    }
                }
                manifest.documents.remove(url);
            }
            stats.pruned += to_remove.len();
            info!(
                "prune done source={} removed={}",
                source.id,
                to_remove.len()
            );
        }

        manifest.generated_at = iso_now();
        manifest.entrypoints = source
            .entrypoints
            .iter()
            .map(|ep| serde_json::to_value(ep).unwrap_or(serde_json::Value::Null))
            .collect();
        manifest.documents_order = docs.iter().map(|doc| doc.url.clone()).collect();

        if !dry_run {
            save_manifest(&manifest_path, &manifest)?;
            if source.id == "react-aria" {
                write_rac_note(output_root)?;
            }
        }

        Ok(stats)
    }
}

#[derive(Clone)]
struct FetchTask {
    doc: DocumentRef,
    if_none_match: Option<String>,
    if_modified_since: Option<String>,
}

fn build_document_refs(
    source: &SourceConfig,
    discovered: &[DiscoveredDoc],
    url_language_hints: &HashMap<String, HashSet<String>>,
) -> Vec<DocumentRef> {
    let mut docs = Vec::with_capacity(discovered.len());
    for doc in discovered {
        let rel = doc
            .relative_path
            .clone()
            .unwrap_or_else(|| safe_relative_path(&doc.url));
        let hints = url_language_hints
            .get(&doc.url)
            .cloned()
            .unwrap_or_default();
        let language_hint = if hints.len() == 1 {
            hints.iter().next().cloned()
        } else {
            None
        };
        let (language, mut rel_without_lang) =
            split_language_from_path(&rel, &source.default_language, language_hint.as_deref());
        rel_without_lang = collapse_index_path(&rel_without_lang);
        if rel_without_lang.extension().is_none() {
            rel_without_lang.set_extension("html");
        }

        let local_path = source
            .output_dir
            .join(&language)
            .join(rel_without_lang.clone());
        docs.push(DocumentRef {
            url: doc.url.clone(),
            language,
            local_path,
        });
    }
    docs
}

fn progress_interval(total: usize) -> usize {
    if total == 0 {
        1
    } else {
        (total / 10).clamp(1, 50)
    }
}

fn normalize_manifest_paths(manifest: &mut Manifest, output_root: &Path, output_root_abs: &Path) {
    for doc in manifest.documents.values_mut() {
        if doc.local_path.is_empty() {
            continue;
        }
        let local_path = PathBuf::from(&doc.local_path);
        if local_path.is_absolute() {
            if let Ok(rel) = local_path.strip_prefix(output_root_abs) {
                doc.local_path = path_to_posix(rel);
            }
            continue;
        }
        if let Ok(rel) = local_path.strip_prefix(output_root) {
            doc.local_path = path_to_posix(rel);
        } else {
            doc.local_path = path_to_posix(&local_path);
        }
    }
}

fn header_value(headers: &HashMap<String, String>, key: &str) -> Option<String> {
    let lower = key.to_lowercase();
    if let Some(value) = headers.get(&lower) {
        return Some(value.clone());
    }
    headers
        .iter()
        .find(|(k, _)| k.eq_ignore_ascii_case(&lower))
        .map(|(_, v)| v.clone())
}

fn write_rac_note(output_root: &Path) -> Result<(), String> {
    let note_path = output_root
        .parent()
        .unwrap_or_else(|| Path::new("."))
        .join("rac.md");
    let note = concat!(
        "# React Aria Components note\n\n",
        "React Aria docs include React Aria Components (RAC) content.\n",
        "Source: https://react-aria.adobe.com/llms.txt\n\n",
        "Search for \"react-aria-components\" or \"React Aria Components\" to find RAC docs.\n"
    );
    fs::write(&note_path, note)
        .map_err(|err| format!("Failed to write {}: {err}", note_path.display()))?;
    Ok(())
}

fn absolute_path(path: &Path) -> PathBuf {
    if path.is_absolute() {
        path.to_path_buf()
    } else {
        std::env::current_dir()
            .unwrap_or_else(|_| PathBuf::from("."))
            .join(path)
    }
}
