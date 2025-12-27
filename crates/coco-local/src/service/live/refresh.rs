use std::collections::HashMap;
use std::path::Path;

use coco_protocol::{CocoError, CocoResult, DocumentId, SearchHit, TextSpan};
use tracing::warn;

use crate::storage::meta::{DocumentRecord, LocalMetaStore};

struct DocumentSnapshot {
    record: DocumentRecord,
    stale: bool,
    content: Option<String>,
}

pub(crate) struct RefreshSummary {
    pub(crate) results: Vec<SearchHit>,
    pub(crate) anchor_failed: Vec<usize>,
}

impl RefreshSummary {
    pub(crate) fn new(results: Vec<SearchHit>) -> Self {
        Self {
            results,
            anchor_failed: Vec::new(),
        }
    }
}

pub(crate) async fn refresh_results_from_fs(
    meta: &LocalMetaStore,
    results: Vec<SearchHit>,
    window_bytes: usize,
) -> RefreshSummary {
    let mut snapshots = HashMap::<DocumentId, DocumentSnapshot>::new();
    for hit in &results {
        if snapshots.contains_key(&hit.chunk.doc_id) {
            continue;
        }
        let record = match meta.get_document(&hit.chunk.doc_id).await {
            Ok(Some(record)) => record,
            Ok(None) => continue,
            Err(err) => {
                warn!("failed to load document {}: {err}", hit.chunk.doc_id);
                continue;
            }
        };
        let stale = match file_is_stale(&record).await {
            Ok(value) => value,
            Err(err) => {
                warn!("failed to stat {}: {err}", record.path);
                false
            }
        };
        snapshots.insert(
            hit.chunk.doc_id.clone(),
            DocumentSnapshot {
                record,
                stale,
                content: None,
            },
        );
    }

    let mut refreshed = Vec::with_capacity(results.len());
    let mut anchor_failed = Vec::new();
    for (index, hit) in results.into_iter().enumerate() {
        let Some(snapshot) = snapshots.get_mut(&hit.chunk.doc_id) else {
            refreshed.push(hit);
            continue;
        };
        if !snapshot.stale {
            refreshed.push(hit);
            continue;
        }
        let path_value = snapshot.record.path.clone();
        let Some(path) = non_empty_path(&path_value) else {
            refreshed.push(hit);
            continue;
        };
        let content = match load_document_content(snapshot, path).await {
            Some(content) => content,
            None => {
                refreshed.push(hit);
                continue;
            }
        };
        let (hit, anchored) = refresh_hit_from_content(hit, content, window_bytes);
        if !anchored {
            anchor_failed.push(index);
        }
        refreshed.push(hit);
    }
    RefreshSummary {
        results: refreshed,
        anchor_failed,
    }
}

async fn file_is_stale(record: &DocumentRecord) -> CocoResult<bool> {
    let Some(path) = non_empty_path(&record.path) else {
        return Ok(false);
    };
    let metadata = tokio::fs::metadata(path).await.map_err(|err| {
        CocoError::system(format!("failed to read metadata: {err}"))
    })?;
    let modified = metadata
        .modified()
        .map_err(|err| CocoError::system(format!("failed to read mtime: {err}")))?;
    let modified: chrono::DateTime<chrono::Utc> = modified.into();
    Ok(modified > record.indexed_at)
}

pub(crate) fn non_empty_path(path: &str) -> Option<&Path> {
    let trimmed = path.trim();
    if trimmed.is_empty() {
        return None;
    }
    Some(Path::new(trimmed))
}

async fn load_document_content<'a>(
    snapshot: &'a mut DocumentSnapshot,
    path: &Path,
) -> Option<&'a str> {
    if snapshot.content.is_none() {
        match tokio::fs::read_to_string(path).await {
            Ok(content) => snapshot.content = Some(content),
            Err(err) => {
                warn!("failed to read {}: {err}", path.display());
                return None;
            }
        }
    }
    snapshot.content.as_deref()
}

fn refresh_hit_from_content(
    mut hit: SearchHit,
    content: &str,
    window_bytes: usize,
) -> (SearchHit, bool) {
    let needle = hit.chunk.content.as_str();
    if needle.is_empty() {
        return (hit, true);
    }
    let original_span = hit.chunk.span;
    if let Some(span) = locate_span(content, needle, original_span, window_bytes) {
        if let Some(slice) = content.get(span.start()..span.end()) {
            hit.chunk.content = slice.to_string();
            hit.chunk.span = span;
            return (hit, true);
        }
    }
    (hit, false)
}

fn locate_span(
    content: &str,
    needle: &str,
    original: TextSpan,
    window_bytes: usize,
) -> Option<TextSpan> {
    let len = needle.len();
    if original.end() <= content.len() {
        if let Some(slice) = content.get(original.start()..original.end()) {
            if slice == needle {
                return Some(original);
            }
        }
    }
    let window = search_window(content.len(), original, window_bytes);
    if let Some(start) = find_best_match(content, needle, window, original.start()) {
        if let Some(end) = start.checked_add(len) {
            return TextSpan::new(start, end).ok();
        }
    }
    let snippet = prefix_snippet(needle, 64)?;
    let snippet_len = snippet.len();
    if let Some(start) = find_best_match(content, &snippet, window, original.start()) {
        if let Some(end) = start.checked_add(snippet_len) {
            return TextSpan::new(start, end).ok();
        }
    }
    None
}

fn search_window(content_len: usize, original: TextSpan, window_bytes: usize) -> (usize, usize) {
    let radius = window_bytes.max(1);
    let start = original.start().saturating_sub(radius);
    let end = original.end().saturating_add(radius).min(content_len);
    (start, end)
}

fn find_best_match(
    content: &str,
    needle: &str,
    window: (usize, usize),
    preferred_start: usize,
) -> Option<usize> {
    if needle.is_empty() {
        return None;
    }
    let mut best: Option<(usize, usize)> = None;
    let (window_start, window_end) = window;
    for (pos, _) in content.match_indices(needle) {
        if pos < window_start {
            continue;
        }
        if pos + needle.len() > window_end {
            continue;
        }
        let distance = preferred_start.abs_diff(pos);
        match best {
            Some((best_dist, _)) if distance >= best_dist => {}
            _ => best = Some((distance, pos)),
        }
    }
    best.map(|(_, pos)| pos)
}

fn prefix_snippet(value: &str, max_chars: usize) -> Option<String> {
    let trimmed = value.trim();
    if trimmed.is_empty() {
        return None;
    }
    Some(trimmed.chars().take(max_chars).collect())
}
