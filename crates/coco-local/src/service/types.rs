use axum::http::StatusCode;
use axum::response::{IntoResponse, Response};
use axum::Json;
use coco_protocol::{
    ChunkingStrategy, CocoError, CocoErrorKind, ErrorResponse, FileType, IndexingConfig,
    ResponseEnvelope, ResponseMeta, ResponseStatus, RetrievalConfig, SearchHit, SearchIntentInput,
};
use tracing::warn;

use crate::storage::meta::{ChunkRecord, DocumentRecord, ProjectRecord};

#[derive(Debug, serde::Deserialize)]
pub(crate) struct RegisterProjectRequest {
    pub(crate) id: Option<String>,
    pub(crate) name: String,
    pub(crate) path: String,
}

#[derive(Debug, serde::Serialize)]
pub(crate) struct RegisterProjectResponse {
    pub(crate) project_id: String,
    pub(crate) name: String,
    pub(crate) path: String,
    pub(crate) created_at: String,
    pub(crate) active_config_id: String,
}

impl From<ProjectRecord> for RegisterProjectResponse {
    fn from(value: ProjectRecord) -> Self {
        Self {
            project_id: value.id,
            name: value.name,
            path: value.path,
            created_at: value.created_at.to_rfc3339(),
            active_config_id: value.active_config_id,
        }
    }
}

#[derive(Debug, serde::Serialize)]
pub(crate) struct HealthResponse {
    pub(crate) status: &'static str,
    pub(crate) service: &'static str,
}

#[derive(Debug, serde::Deserialize)]
pub(crate) struct ListConfigsQuery {
    #[serde(default)]
    pub(crate) project_id: Option<String>,
}

#[derive(Debug, serde::Serialize)]
pub(crate) struct ListConfigsResponse {
    pub(crate) configs: Vec<IndexingConfig>,
    pub(crate) active_config_id: Option<String>,
}

#[derive(Debug, serde::Deserialize)]
pub(crate) struct UpsertConfigRequest {
    pub(crate) config: IndexingConfig,
}

#[derive(Debug, serde::Serialize)]
pub(crate) struct IndexingConfigResponse {
    pub(crate) config: IndexingConfig,
}

#[derive(Debug, serde::Deserialize)]
pub(crate) struct ActivateConfigRequest {
    pub(crate) project_id: String,
    pub(crate) config_id: String,
}

#[derive(Debug, serde::Serialize)]
pub(crate) struct ActivateConfigResponse {
    pub(crate) active_config_id: String,
}

#[derive(Debug, serde::Deserialize)]
pub(crate) struct PruneRequest {
    pub(crate) project_id: String,
    #[serde(default)]
    pub(crate) keep: Option<usize>,
}

#[derive(Debug, serde::Serialize)]
pub(crate) struct PruneResponse {
    pub(crate) status: &'static str,
}

#[derive(Debug, serde::Deserialize)]
pub(crate) struct ImportRequest {
    pub(crate) project_id: String,
    #[serde(default)]
    pub(crate) indexing_config_id: Option<String>,
    #[serde(default)]
    pub(crate) indexing_config: Option<IndexingConfig>,
    pub(crate) document_id: Option<String>,
    pub(crate) content: String,
    pub(crate) file_type: FileType,
    pub(crate) title: Option<String>,
    pub(crate) path: Option<String>,
    pub(crate) chunking: Option<ChunkingStrategy>,
}

#[derive(Debug, serde::Serialize)]
pub(crate) struct ImportResponse {
    pub(crate) document_id: String,
    pub(crate) chunks: Vec<ChunkSummary>,
}

#[derive(Debug, serde::Deserialize)]
pub(crate) struct QueryRequest {
    pub(crate) intent: SearchIntentInput,
    #[serde(default)]
    pub(crate) project_id: Option<String>,
    #[serde(default)]
    pub(crate) indexing_config_id: Option<String>,
    #[serde(default)]
    pub(crate) retrieval_config: Option<RetrievalConfig>,
    #[serde(default)]
    pub(crate) indexing_config: Option<IndexingConfig>,
}

#[derive(Debug, serde::Serialize)]
pub(crate) struct QueryResponse {
    pub(crate) results: Vec<SearchHit>,
}

#[derive(Debug, serde::Serialize)]
pub(crate) struct DocumentResponse {
    pub(crate) document: DocumentSummary,
    pub(crate) chunks: Vec<ChunkSummary>,
}

#[derive(Debug, serde::Serialize)]
pub(crate) struct DocumentSummary {
    pub(crate) id: String,
    pub(crate) project_id: String,
    pub(crate) path: String,
    pub(crate) title: Option<String>,
    pub(crate) content_hash: String,
    pub(crate) indexed_at: String,
}

impl From<DocumentRecord> for DocumentSummary {
    fn from(value: DocumentRecord) -> Self {
        Self {
            id: value.id,
            project_id: value.project_id,
            path: value.path,
            title: value.title,
            content_hash: value.content_hash,
            indexed_at: value.indexed_at.to_rfc3339(),
        }
    }
}

#[derive(Debug, serde::Serialize)]
pub(crate) struct ChunkSummary {
    pub(crate) id: String,
    pub(crate) doc_id: String,
    pub(crate) content: String,
    pub(crate) start_line: i32,
    pub(crate) end_line: i32,
}

impl From<ChunkRecord> for ChunkSummary {
    fn from(value: ChunkRecord) -> Self {
        Self {
            id: value.id,
            doc_id: value.doc_id,
            content: value.content,
            start_line: value.start_line,
            end_line: value.end_line,
        }
    }
}

#[derive(Debug)]
pub(crate) struct ApiError(pub(crate) CocoError);

impl From<CocoError> for ApiError {
    fn from(value: CocoError) -> Self {
        Self(value)
    }
}

impl IntoResponse for ApiError {
    fn into_response(self) -> Response {
        let error = self.0;
        let status = StatusCode::from_u16(error.http_status())
            .unwrap_or(StatusCode::INTERNAL_SERVER_ERROR);
        let kind = error.kind();
        if kind != CocoErrorKind::User {
            warn!("internal error: {}", error);
        }
        let message = error.into_public_message();
        let body = ErrorResponse { kind, message };
        (status, Json(body)).into_response()
    }
}

pub(crate) type ApiResult<T> = Result<T, ApiError>;

pub(crate) fn response_envelope<T>(data: T) -> ResponseEnvelope<T> {
    ResponseEnvelope {
        meta: ResponseMeta {
            status: ResponseStatus::Fresh,
        },
        data,
    }
}
