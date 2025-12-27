use coco_protocol::VectorIndexParams;
use sha2::{Digest, Sha256};

use super::helpers::{COL_CONFIG_ID, COL_CONTENT, COL_EMBEDDING, TABLE_CHUNKS};

pub(super) enum IndexKind {
    IvfFlat,
    Hnsw {
        m: Option<u32>,
        ef_construction: Option<u32>,
    },
}

pub(super) fn index_kind_from_params(params: Option<&VectorIndexParams>) -> IndexKind {
    if let Some(hnsw) = params.and_then(|params| params.hnsw.as_ref()) {
        return IndexKind::Hnsw {
            m: hnsw.m,
            ef_construction: hnsw.ef_construction,
        };
    }
    IndexKind::IvfFlat
}

pub(super) fn vector_index_name(index_kind: &IndexKind, config_id: Option<&str>) -> String {
    if config_id.is_none() && matches!(index_kind, IndexKind::IvfFlat) {
        return "chunks_embedding_idx".to_string();
    }
    let suffix = match index_kind {
        IndexKind::IvfFlat => "embedding_ivfflat",
        IndexKind::Hnsw { .. } => "embedding_hnsw",
    };
    index_name(&format!("chunks_{suffix}"), config_id)
}

pub(super) fn fts_index_name(config_id: Option<&str>) -> String {
    if config_id.is_none() {
        return "chunks_content_fts_idx".to_string();
    }
    index_name("chunks_content_fts", config_id)
}

pub(super) fn build_vector_index_sql(
    index_name: &str,
    metric_ops: &str,
    index_kind: &IndexKind,
    config_id: Option<&str>,
) -> String {
    let where_clause = config_filter_clause(config_id);
    match index_kind {
        IndexKind::IvfFlat => format!(
            "CREATE INDEX IF NOT EXISTS {index_name} \
             ON {TABLE_CHUNKS} USING ivfflat ({COL_EMBEDDING} {metric_ops}){where_clause}"
        ),
        IndexKind::Hnsw { m, ef_construction } => {
            let options = hnsw_options_clause(*m, *ef_construction);
            format!(
                "CREATE INDEX IF NOT EXISTS {index_name} \
                 ON {TABLE_CHUNKS} USING hnsw ({COL_EMBEDDING} {metric_ops}){options}{where_clause}"
            )
        }
    }
}

pub(super) fn build_fts_index_sql(index_name: &str, config_id: Option<&str>) -> String {
    let where_clause = config_filter_clause(config_id);
    format!(
        "CREATE INDEX IF NOT EXISTS {index_name} \
         ON {TABLE_CHUNKS} USING GIN (to_tsvector('simple', {COL_CONTENT})){where_clause}"
    )
}

fn hnsw_options_clause(m: Option<u32>, ef_construction: Option<u32>) -> String {
    let mut options = Vec::new();
    if let Some(m) = m {
        options.push(format!("m={m}"));
    }
    if let Some(ef) = ef_construction {
        options.push(format!("ef_construction={ef}"));
    }
    if options.is_empty() {
        String::new()
    } else {
        format!(" WITH ({})", options.join(", "))
    }
}

fn config_filter_clause(config_id: Option<&str>) -> String {
    match config_id {
        Some(config_id) => format!(" WHERE {COL_CONFIG_ID} = '{config_id}'"),
        None => String::new(),
    }
}

fn index_name(prefix: &str, config_id: Option<&str>) -> String {
    let Some(config_id) = config_id else {
        return prefix.to_string();
    };
    let base = format!("{prefix}_{config_id}");
    if base.len() <= 63 {
        return base;
    }
    let mut hasher = Sha256::new();
    hasher.update(config_id.as_bytes());
    let digest = hasher.finalize();
    let suffix = hex8(&digest[..4]);
    let max_prefix_len = 63usize.saturating_sub(suffix.len() + 1);
    let trimmed_prefix = &base[..max_prefix_len];
    format!("{trimmed_prefix}_{suffix}")
}

fn hex8(bytes: &[u8]) -> String {
    let mut output = String::with_capacity(bytes.len() * 2);
    for byte in bytes {
        output.push(hex_digit(byte >> 4));
        output.push(hex_digit(byte & 0x0f));
    }
    output
}

fn hex_digit(value: u8) -> char {
    const HEX: &[u8; 16] = b"0123456789abcdef";
    HEX[value as usize] as char
}
