use std::collections::{HashSet, VecDeque};
use std::path::{Path, PathBuf};
use std::process::Command;
use std::time::Duration;

use coco_protocol::{SearchHit, TextSpan};
use tracing::warn;

use crate::ids::generate_id;
use crate::ingest::doc_id_for_path;
use crate::storage::meta::LocalMetaStore;

use super::refresh::{non_empty_path, RefreshSummary};
use crate::service::state::LiveGrepConfig;

struct LiveGrepMatch {
    path: PathBuf,
    line_number: usize,
    line: String,
}

pub(crate) async fn maybe_apply_live_grep(
    meta: &LocalMetaStore,
    project_id: Option<&str>,
    query_text: Option<&str>,
    refresh: RefreshSummary,
    desired: usize,
    live_grep: &LiveGrepConfig,
) -> Vec<SearchHit> {
    let mut results = refresh.results;
    if !live_grep.enabled {
        return results;
    }
    if live_grep.max_results == 0 {
        return results;
    }
    if desired == 0 {
        return results;
    }
    if results.len() >= desired && refresh.anchor_failed.is_empty() {
        return results;
    }
    let query_text = query_text.unwrap_or("").trim();
    if query_text.is_empty() {
        return results;
    }
    let project_id = match project_id {
        Some(value) if !value.trim().is_empty() => value,
        _ => return results,
    };
    let project = match meta.get_project(project_id).await {
        Ok(Some(project)) => project,
        Ok(None) => return results,
        Err(err) => {
            warn!("failed to load project {}: {err}", project_id);
            return results;
        }
    };
    let root = match non_empty_path(&project.path) {
        Some(path) => path.to_path_buf(),
        None => return results,
    };
    let version_id = match meta.ensure_active_version_id(project_id).await {
        Ok(version_id) => version_id,
        Err(err) => {
            warn!("failed to resolve version for project {}: {err}", project_id);
            return results;
        }
    };

    let max_results = live_grep.max_results.max(1);
    let matches = run_live_grep(&root, query_text, max_results, live_grep.timeout).await;
    if matches.is_empty() {
        return results;
    }

    let mut existing = HashSet::new();
    for hit in &results {
        existing.insert(dedupe_key(&hit.chunk.doc_id, &hit.chunk.content));
    }

    let mut replacements = VecDeque::new();
    for entry in matches {
        let Some(hit) = build_live_grep_hit(project_id, &version_id, &entry) else {
            continue;
        };
        let key = dedupe_key(&hit.chunk.doc_id, &hit.chunk.content);
        if existing.insert(key) {
            replacements.push_back(hit);
        }
        if replacements.len() >= max_results {
            break;
        }
    }

    if replacements.is_empty() {
        return results;
    }

    if !refresh.anchor_failed.is_empty() {
        for index in &refresh.anchor_failed {
            let Some(replacement) = replacements.pop_front() else {
                break;
            };
            if *index < results.len() {
                results[*index] = replacement;
            }
        }
    }

    if results.len() < desired {
        let remaining = desired - results.len();
        for _ in 0..remaining {
            let Some(replacement) = replacements.pop_front() else {
                break;
            };
            results.push(replacement);
        }
    }

    results
}

fn build_live_grep_hit(
    project_id: &str,
    version_id: &str,
    entry: &LiveGrepMatch,
) -> Option<SearchHit> {
    let doc_id = doc_id_for_path(project_id, version_id, &entry.path).ok()?;
    let chunk_id = generate_id(
        "chunk",
        format!("{doc_id}:{}", entry.line_number).as_bytes(),
    );
    let content = entry.line.trim_end().to_string();
    let span = TextSpan {
        start: 0,
        end: content.len(),
    };
    Some(SearchHit {
        meta: coco_protocol::SearchHitMeta {
            score: 0.0,
            quality: None,
            verified: None,
        },
        chunk: coco_protocol::Chunk {
            id: chunk_id.into(),
            doc_id: doc_id.into(),
            content,
            embedding: None,
            span,
            quality_score: None,
            verified: None,
        },
    })
}

fn dedupe_key(doc_id: &str, content: &str) -> String {
    format!("{doc_id}:{content}")
}

async fn run_live_grep(
    root: &Path,
    query: &str,
    max_results: usize,
    timeout: Duration,
) -> Vec<LiveGrepMatch> {
    if max_results == 0 {
        return Vec::new();
    }
    if !root.exists() {
        return Vec::new();
    }
    if let Some(matches) = run_ripgrep(root, query, max_results, timeout).await {
        return matches;
    }
    run_grep(root, query, max_results, timeout).await
}

async fn run_ripgrep(
    root: &Path,
    query: &str,
    max_results: usize,
    timeout: Duration,
) -> Option<Vec<LiveGrepMatch>> {
    let root_path = root.to_path_buf();
    let root_for_cmd = root_path.clone();
    let query = query.to_string();
    let max_count = max_results.to_string();
    let output = match tokio::time::timeout(
        timeout,
        tokio::task::spawn_blocking(move || {
            let mut cmd = Command::new("rg");
            cmd.current_dir(&root_for_cmd)
                .arg("--json")
                .arg("--no-messages")
                .arg("--fixed-strings")
                .arg("--max-count")
                .arg(max_count)
                .arg(query)
                .arg(".");
            cmd.output()
        }),
    )
    .await
    {
        Ok(Ok(Ok(output))) => output,
        Ok(Ok(Err(err))) => {
            if err.kind() == std::io::ErrorKind::NotFound {
                return None;
            }
            warn!("rg failed: {err}");
            return Some(Vec::new());
        }
        Ok(Err(err)) => {
            warn!("rg join error: {err}");
            return Some(Vec::new());
        }
        Err(_) => {
            warn!("rg timed out");
            return Some(Vec::new());
        }
    };

    match output.status.code() {
        Some(0) => {}
        Some(1) => return Some(Vec::new()),
        _ => {
            warn!("rg returned non-zero status");
            return Some(Vec::new());
        }
    }

    let stdout = String::from_utf8_lossy(&output.stdout);
    let mut matches = Vec::new();
    for line in stdout.lines() {
        let Ok(value) = serde_json::from_str::<serde_json::Value>(line) else {
            continue;
        };
        if value.get("type").and_then(|value| value.as_str()) != Some("match") {
            continue;
        }
        let Some(data) = value.get("data") else {
            continue;
        };
        let path_text = data
            .get("path")
            .and_then(|path| path.get("text"))
            .and_then(|text| text.as_str());
        let line_number = data.get("line_number").and_then(|num| num.as_u64());
        let line_text = data
            .get("lines")
            .and_then(|lines| lines.get("text"))
            .and_then(|text| text.as_str());
        let (Some(path_text), Some(line_number), Some(line_text)) =
            (path_text, line_number, line_text)
        else {
            continue;
        };
        matches.push(LiveGrepMatch {
            path: root_path.join(path_text),
            line_number: line_number as usize,
            line: line_text.trim_end().to_string(),
        });
        if matches.len() >= max_results {
            break;
        }
    }
    Some(matches)
}

async fn run_grep(
    root: &Path,
    query: &str,
    max_results: usize,
    timeout: Duration,
) -> Vec<LiveGrepMatch> {
    let root_path = root.to_path_buf();
    let root_for_cmd = root_path.clone();
    let query = query.to_string();
    let max_count = max_results.to_string();
    let output = match tokio::time::timeout(
        timeout,
        tokio::task::spawn_blocking(move || {
            let mut cmd = Command::new("grep");
            cmd.current_dir(&root_for_cmd)
                .arg("-R")
                .arg("-n")
                .arg("-F")
                .arg("-m")
                .arg(max_count)
                .arg("--")
                .arg(query)
                .arg(".");
            cmd.output()
        }),
    )
    .await
    {
        Ok(Ok(Ok(output))) => output,
        Ok(Ok(Err(err))) => {
            warn!("grep failed: {err}");
            return Vec::new();
        }
        Ok(Err(err)) => {
            warn!("grep join error: {err}");
            return Vec::new();
        }
        Err(_) => {
            warn!("grep timed out");
            return Vec::new();
        }
    };
    if !output.status.success() {
        return Vec::new();
    }

    let stdout = String::from_utf8_lossy(&output.stdout);
    let mut matches = Vec::new();
    for line in stdout.lines() {
        let mut parts = line.splitn(3, ':');
        let Some(path_text) = parts.next() else {
            continue;
        };
        let Some(line_number) = parts.next() else {
            continue;
        };
        let line_text = parts.next().unwrap_or_default();
        let Ok(line_number) = line_number.parse::<usize>() else {
            continue;
        };
        matches.push(LiveGrepMatch {
            path: root_path.join(path_text),
            line_number,
            line: line_text.trim_end().to_string(),
        });
        if matches.len() >= max_results {
            break;
        }
    }
    matches
}
