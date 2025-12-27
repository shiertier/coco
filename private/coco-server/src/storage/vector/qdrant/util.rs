use std::collections::HashMap;

use coco_protocol::CocoError;
use qdrant_client::qdrant::value::Kind as QdrantValueKind;
use qdrant_client::qdrant::Value;
use qdrant_client::QdrantError;
use sha2::{Digest, Sha256};
use tracing::warn;

const MAX_COLLECTION_NAME_LEN: usize = 63;

pub(super) const PAYLOAD_ORG_ID: &str = "org_id";
pub(super) const PAYLOAD_USER_ID: &str = "user_id";
pub(super) const PAYLOAD_PROJECT_ID: &str = "project_id";
pub(super) const PAYLOAD_VERSION_ID: &str = "version_id";
pub(super) const PAYLOAD_CONFIG_ID: &str = "config_id";
pub(super) const PAYLOAD_DOC_ID: &str = "doc_id";
pub(super) const PAYLOAD_CHUNK_ID: &str = "chunk_id";
pub(super) const PAYLOAD_ALIAS_ACTIVE: &str = "active";

pub(super) fn map_qdrant_err(context: &str, err: QdrantError) -> CocoError {
    warn!("{context}: {err}");
    CocoError::storage(context)
}

pub(super) fn join_name(prefix: &str, parts: &[&str]) -> String {
    let mut out = String::new();
    let prefix = prefix.trim_end_matches('_');
    if !prefix.is_empty() {
        out.push_str(prefix);
    }
    for part in parts {
        if !out.is_empty() {
            out.push('_');
        }
        out.push_str(part);
    }
    out
}

pub(super) fn safe_collection_name(base: String) -> String {
    if base.len() <= MAX_COLLECTION_NAME_LEN {
        return base;
    }
    let mut hasher = Sha256::new();
    hasher.update(base.as_bytes());
    let digest = hasher.finalize();
    let suffix = hex8(&digest[..4]);
    let max_prefix_len = MAX_COLLECTION_NAME_LEN.saturating_sub(suffix.len() + 1);
    let trimmed = &base[..max_prefix_len];
    format!("{trimmed}_{suffix}")
}

pub(super) fn payload_string<'a>(payload: &'a HashMap<String, Value>, key: &str) -> Option<&'a str> {
    let value = payload.get(key)?;
    match value.kind.as_ref()? {
        QdrantValueKind::StringValue(value) => Some(value.as_str()),
        _ => None,
    }
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
