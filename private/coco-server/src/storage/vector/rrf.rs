use std::collections::{HashMap, HashSet};

use coco_protocol::SearchHit;

pub(crate) fn merge_rrf(
    vector_results: Vec<SearchHit>,
    fts_results: Vec<SearchHit>,
    rrf_k: u32,
    top_k: usize,
) -> Vec<SearchHit> {
    let mut merged: HashMap<String, SearchHit> = HashMap::new();
    let k = rrf_k as f32;

    for (index, result) in vector_results.into_iter().enumerate() {
        let rank = (index + 1) as f32;
        let score = 1.0 / (k + rank);
        merged
            .entry(result.chunk.id.to_string())
            .and_modify(|existing| existing.meta.score += score)
            .or_insert(SearchHit {
                meta: coco_protocol::SearchHitMeta {
                    score,
                    quality: result.meta.quality,
                    verified: result.meta.verified,
                },
                chunk: result.chunk,
            });
    }

    for (index, result) in fts_results.into_iter().enumerate() {
        let rank = (index + 1) as f32;
        let score = 1.0 / (k + rank);
        merged
            .entry(result.chunk.id.to_string())
            .and_modify(|existing| existing.meta.score += score)
            .or_insert(SearchHit {
                meta: coco_protocol::SearchHitMeta {
                    score,
                    quality: result.meta.quality,
                    verified: result.meta.verified,
                },
                chunk: result.chunk,
            });
    }

    let mut results: Vec<SearchHit> = merged.into_values().collect();
    results.sort_by(|a, b| b.meta.score.total_cmp(&a.meta.score));
    results.truncate(top_k);
    normalize_scores(results)
}

pub(crate) fn retain_allowed(results: Vec<SearchHit>, allowed: &HashSet<String>) -> Vec<SearchHit> {
    results
        .into_iter()
        .filter(|hit| allowed.contains(hit.chunk.id.as_str()))
        .collect()
}

pub(crate) fn normalize_scores(mut results: Vec<SearchHit>) -> Vec<SearchHit> {
    if results.is_empty() {
        return results;
    }
    let mut min_score = f32::INFINITY;
    let mut max_score = f32::NEG_INFINITY;
    for hit in &results {
        min_score = min_score.min(hit.meta.score);
        max_score = max_score.max(hit.meta.score);
    }
    let needs_normalization = min_score < 0.0 || max_score > 1.0;
    if needs_normalization {
        let range = (max_score - min_score).max(f32::EPSILON);
        for hit in &mut results {
            let normalized = (hit.meta.score - min_score) / range;
            hit.meta.score = normalized.clamp(0.0, 1.0);
        }
    } else {
        for hit in &mut results {
            hit.meta.score = hit.meta.score.clamp(0.0, 1.0);
        }
    }
    results
}
