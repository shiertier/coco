use coco_protocol::{Chunk, CocoError, CocoResult, TextSpan};
use sea_orm::{QueryResult, Value};

pub(super) const TABLE_CHUNKS: &str = "chunks";
pub(super) const TABLE_PROJECTS: &str = "projects";

pub(super) const COL_ID: &str = "id";
pub(super) const COL_ORG_ID: &str = "org_id";
pub(super) const COL_USER_ID: &str = "user_id";
pub(super) const COL_PROJECT_ID: &str = "project_id";
pub(super) const COL_DOC_ID: &str = "doc_id";
pub(super) const COL_VERSION_ID: &str = "version_id";
pub(super) const COL_CONFIG_ID: &str = "config_id";
pub(super) const COL_CONTENT: &str = "content";
pub(super) const COL_START_LINE: &str = "start_line";
pub(super) const COL_END_LINE: &str = "end_line";
pub(super) const COL_QUALITY_SCORE: &str = "quality_score";
pub(super) const COL_VERIFIED: &str = "verified";
pub(super) const COL_EMBEDDING: &str = "embedding";
pub(super) const COL_EMBEDDING_TEXT: &str = "embedding_text";
pub(super) const COL_DISTANCE: &str = "distance";
pub(super) const COL_SCORE: &str = "score";

pub(super) const DEFAULT_MAX_CONNECTIONS: u32 = 16;
pub(super) const DEFAULT_MIN_CONNECTIONS: u32 = 1;
pub(super) const DEFAULT_CONNECT_TIMEOUT_SECS: u64 = 10;
pub(super) const DEFAULT_RRF_K: u32 = 60;

pub(super) fn chunk_from_row(row: &QueryResult) -> CocoResult<Chunk> {
    let chunk_id: String = row.try_get("", COL_ID).map_err(map_storage_err)?;
    let doc_id: String = row.try_get("", COL_DOC_ID).map_err(map_storage_err)?;
    let content: String = row.try_get("", COL_CONTENT).map_err(map_storage_err)?;
    let start_line: i64 = row.try_get("", COL_START_LINE).map_err(map_storage_err)?;
    let end_line: i64 = row.try_get("", COL_END_LINE).map_err(map_storage_err)?;
    let quality_score: Option<f32> = row
        .try_get("", COL_QUALITY_SCORE)
        .map_err(map_storage_err)?;
    let verified: bool = row.try_get("", COL_VERIFIED).map_err(map_storage_err)?;
    let start = usize::try_from(start_line)
        .map_err(|_| CocoError::storage("start_line out of range for TextSpan"))?;
    let end = usize::try_from(end_line)
        .map_err(|_| CocoError::storage("end_line out of range for TextSpan"))?;
    let span = TextSpan::new(start, end)
        .map_err(|_| CocoError::storage("invalid span range for TextSpan"))?;
    Ok(Chunk {
        id: chunk_id.into(),
        doc_id: doc_id.into(),
        content,
        embedding: None,
        span,
        quality_score,
        verified: Some(verified),
    })
}

pub(super) fn parse_vector_text(value: &str) -> CocoResult<Vec<f32>> {
    let trimmed = value.trim();
    let trimmed = trimmed
        .strip_prefix('[')
        .and_then(|value| value.strip_suffix(']'))
        .unwrap_or(trimmed);
    if trimmed.is_empty() {
        return Ok(Vec::new());
    }
    let mut values = Vec::new();
    for part in trimmed.split(',') {
        let part = part.trim();
        if part.is_empty() {
            continue;
        }
        let number = part
            .parse::<f32>()
            .map_err(|_| CocoError::storage("invalid vector literal"))?;
        values.push(number);
    }
    Ok(values)
}

pub(super) fn vector_literal(values: &[f32]) -> String {
    let mut output = String::from("[");
    for (idx, value) in values.iter().enumerate() {
        if idx > 0 {
            output.push(',');
        }
        output.push_str(&value.to_string());
    }
    output.push(']');
    output
}

pub(super) fn validate_tenant(tenant: &super::backend::TenantContext) -> CocoResult<()> {
    if tenant.org_id.is_empty() {
        return Err(CocoError::user("org_id must not be empty"));
    }
    if tenant.user_id.is_empty() {
        return Err(CocoError::user("user_id must not be empty"));
    }
    if tenant.project_id.is_empty() {
        return Err(CocoError::user("project_id must not be empty"));
    }
    if let Some(config_id) = tenant.config_id.as_deref() {
        if config_id.trim().is_empty() {
            return Err(CocoError::user("config_id must not be empty"));
        }
    }
    Ok(())
}

pub(super) fn push_value(values: &mut Vec<Value>, value: impl Into<Value>) -> String {
    let index = values.len() + 1;
    values.push(value.into());
    format!("${index}")
}

pub(super) fn to_i64(value: usize) -> CocoResult<i64> {
    i64::try_from(value).map_err(|_| CocoError::user("value out of range for database integer"))
}

pub(super) fn map_storage_err<E: std::fmt::Display>(error: E) -> CocoError {
    CocoError::storage(error)
}
