use axum::Json;
use axum::http::StatusCode;
use axum::response::{IntoResponse, Response};
use coco_protocol::{
    CocoError, CocoErrorKind, CocoResult, ErrorResponse, FilterField, FilterValue, HybridAlpha,
    IndexingConfig, IndexingPlan, ResponseMeta, SearchHit, SearchIntentInput, SearchQueryInput,
    VectorBackendKind,
};
use tracing::warn;
use utoipa::ToSchema;

use super::worker_ipc;
use crate::storage::meta::{IngestJobRecord, ProjectRecord};

#[derive(Debug, serde::Serialize, ToSchema)]
pub(crate) struct HealthResponse {
    pub(crate) status: &'static str,
    pub(crate) service: &'static str,
    pub(crate) version: &'static str,
    pub(crate) worker: WorkerStatusResponse,
    pub(crate) queue: QueueStatusResponse,
    pub(crate) vector_backend: VectorBackendStatus,
}

#[derive(Debug, serde::Serialize, ToSchema)]
pub(crate) struct WorkerStatusResponse {
    pub(crate) status: String,
    pub(crate) version: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub(crate) updated_at: Option<String>,
}

#[derive(Debug, serde::Serialize, ToSchema)]
pub(crate) struct QueueStatusResponse {
    pub(crate) mode: String,
    pub(crate) status: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub(crate) detail: Option<String>,
}

#[derive(Debug, serde::Serialize, ToSchema)]
pub(crate) struct VectorBackendStatus {
    pub(crate) kind: VectorBackendKind,
    pub(crate) status: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub(crate) version: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub(crate) detail: Option<String>,
}

#[derive(Debug, serde::Deserialize, serde::Serialize, ToSchema)]
pub(crate) struct RegisterProjectRequest {
    pub(crate) org_id: String,
    pub(crate) user_id: String,
    #[serde(default)]
    pub(crate) org_name: Option<String>,
    #[serde(default)]
    pub(crate) project_id: Option<String>,
    pub(crate) name: String,
    pub(crate) source_ref: String,
    #[serde(default)]
    pub(crate) platform: Option<String>,
}

#[derive(Debug, serde::Serialize, ToSchema)]
pub(crate) struct RegisterProjectResponse {
    pub(crate) project_id: String,
    pub(crate) org_id: String,
    pub(crate) name: String,
    pub(crate) active_version_id: Option<String>,
    pub(crate) active_config_id: String,
}

impl From<ProjectRecord> for RegisterProjectResponse {
    fn from(value: ProjectRecord) -> Self {
        Self {
            project_id: value.id,
            org_id: value.org_id,
            name: value.name,
            active_version_id: value.active_version_id,
            active_config_id: value.active_config_id,
        }
    }
}

#[derive(Debug, serde::Deserialize, serde::Serialize, ToSchema)]
pub(crate) struct PruneRequest {
    pub(crate) org_id: String,
    pub(crate) user_id: String,
    pub(crate) project_id: String,
    pub(crate) keep: Option<usize>,
}

#[derive(Debug, serde::Serialize, ToSchema)]
pub(crate) struct PruneResponse {
    pub(crate) removed: usize,
}

#[derive(Debug, serde::Deserialize, ToSchema)]
pub(crate) struct ListConfigsQuery {
    pub(crate) org_id: String,
    #[serde(default)]
    pub(crate) user_id: Option<String>,
    #[serde(default)]
    pub(crate) project_id: Option<String>,
}

#[derive(Debug, serde::Serialize, ToSchema)]
pub(crate) struct ListConfigsResponse {
    pub(crate) configs: Vec<IndexingConfig>,
    pub(crate) active_config_id: Option<String>,
}

#[derive(Debug, serde::Deserialize, ToSchema)]
pub(crate) struct UpsertConfigRequest {
    pub(crate) org_id: String,
    pub(crate) config: IndexingConfig,
}

#[derive(Debug, serde::Serialize, ToSchema)]
pub(crate) struct IndexingConfigResponse {
    pub(crate) config: IndexingConfig,
}

#[derive(Debug, serde::Deserialize, ToSchema)]
pub(crate) struct ActivateConfigRequest {
    pub(crate) org_id: String,
    pub(crate) user_id: String,
    pub(crate) project_id: String,
    pub(crate) config_id: String,
}

#[derive(Debug, serde::Serialize, ToSchema)]
pub(crate) struct ActivateConfigResponse {
    pub(crate) active_config_id: String,
}

/// Filter operator documented for the public server API.
#[derive(Debug, Clone, Copy, serde::Deserialize, serde::Serialize, PartialEq, Eq, ToSchema)]
#[serde(rename_all = "snake_case")]
pub(crate) enum PublicFilterOp {
    /// Equal comparison.
    Eq,
    /// Substring containment.
    Contains,
}

/// Filter constraint accepted by the public server API.
#[derive(Debug, Clone, serde::Deserialize, serde::Serialize, PartialEq, Eq, ToSchema)]
pub(crate) struct PublicFilter {
    /// Field name to filter on (server allows `doc_id` and `chunk_id`).
    pub(crate) field: String,
    /// Comparison operator (server allows `eq` and `contains` only).
    #[schema(value_type = PublicFilterOp)]
    pub(crate) op: coco_protocol::FilterOp,
    /// String-encoded filter value.
    pub(crate) value: String,
}

impl TryFrom<PublicFilter> for coco_protocol::Filter {
    type Error = CocoError;

    fn try_from(value: PublicFilter) -> CocoResult<Self> {
        Ok(coco_protocol::Filter {
            field: FilterField::new(value.field)?,
            op: value.op,
            value: FilterValue::String(value.value),
        })
    }
}

/// Search intent describing how retrieval should run.
#[derive(Debug, Clone, serde::Deserialize, serde::Serialize, PartialEq, ToSchema)]
pub(crate) struct PublicSearchIntent {
    /// Query payload (controls required fields).
    #[serde(flatten)]
    pub(crate) query: SearchQueryInput,
    /// Optional indexing configuration selection (defaults to the project's default config).
    pub(crate) indexing_config_id: Option<String>,
    /// Number of candidates to return.
    #[schema(value_type = u32)]
    pub(crate) top_k: std::num::NonZeroU32,
    /// Hybrid weight for vector vs. keyword scoring.
    pub(crate) hybrid_alpha: HybridAlpha,
    /// Optional filter list (server allows `doc_id`/`chunk_id` with `eq`/`contains`).
    #[serde(default)]
    pub(crate) filters: Vec<PublicFilter>,
    /// Optional reranker configuration.
    pub(crate) reranker: Option<coco_protocol::RerankerConfig>,
}

impl TryFrom<PublicSearchIntent> for SearchIntentInput {
    type Error = CocoError;

    fn try_from(value: PublicSearchIntent) -> CocoResult<Self> {
        let filters = value
            .filters
            .into_iter()
            .map(TryInto::try_into)
            .collect::<Result<Vec<_>, _>>()?;
        Ok(SearchIntentInput {
            query: value.query,
            indexing_config_id: value.indexing_config_id,
            top_k: value.top_k,
            hybrid_alpha: value.hybrid_alpha,
            filters,
            reranker: value.reranker,
        })
    }
}

#[derive(Debug, serde::Deserialize, serde::Serialize, ToSchema)]
pub(crate) struct QueryRequest {
    pub(crate) intent: PublicSearchIntent,
    #[serde(default)]
    pub(crate) indexing_config_id: Option<String>,
    #[serde(default)]
    pub(crate) retrieval_config: Option<coco_protocol::RetrievalConfig>,
    #[serde(default)]
    pub(crate) indexing_config: Option<IndexingConfig>,
}

#[derive(Debug, serde::Serialize, ToSchema)]
pub(crate) struct QueryResponse {
    pub(crate) results: Vec<SearchHit>,
}

#[derive(Debug, serde::Serialize, ToSchema)]
pub(crate) struct QueryResponseEnvelope {
    pub(crate) meta: ResponseMeta,
    pub(crate) data: QueryResponse,
}

#[derive(Debug, serde::Deserialize, serde::Serialize, ToSchema)]
pub(crate) struct IndexRequest {
    #[serde(default)]
    pub(crate) indexing_config_id: Option<String>,
}

#[derive(Debug, serde::Serialize, ToSchema)]
pub(crate) struct IndexResponse {
    pub(crate) status: String,
}

#[derive(Debug, serde::Deserialize, serde::Serialize, ToSchema)]
pub(crate) struct MemoQueryRequest {
    pub(crate) session_token: String,
    pub(crate) intent: PublicSearchIntent,
    #[serde(default)]
    pub(crate) retrieval_config: Option<coco_protocol::RetrievalConfig>,
}

#[derive(Debug, serde::Serialize, ToSchema)]
pub(crate) struct MemoQueryResponseEnvelope {
    pub(crate) meta: ResponseMeta,
    pub(crate) data: QueryResponse,
}

#[derive(Debug, serde::Deserialize, serde::Serialize, ToSchema)]
pub(crate) struct IngestBatchRequest {
    #[serde(default = "default_true")]
    pub(crate) activate: bool,
    #[serde(default)]
    pub(crate) indexing_config_id: Option<String>,
    pub(crate) documents: Vec<IngestDocument>,
}

#[derive(Debug, serde::Deserialize, serde::Serialize)]
pub(crate) struct IngestJobPayload {
    pub(crate) api_version: String,
    #[serde(default)]
    pub(crate) request: Option<IngestBatchRequest>,
    #[serde(default)]
    pub(crate) blob_ref: Option<String>,
    #[serde(default)]
    pub(crate) plan: Option<IndexingPlan>,
    #[serde(default)]
    pub(crate) wasm_module_ref: Option<String>,
}

#[derive(Debug, serde::Deserialize, serde::Serialize, ToSchema)]
pub(crate) struct IngestDocument {
    pub(crate) doc_id: String,
    pub(crate) source_ref: String,
    #[serde(default)]
    pub(crate) title: Option<String>,
    #[serde(default)]
    pub(crate) content_hash: Option<String>,
    #[serde(default)]
    pub(crate) quality_score: Option<f32>,
    #[serde(default)]
    pub(crate) verified: Option<bool>,
    pub(crate) chunks: Vec<IngestChunk>,
}

#[derive(Debug, serde::Deserialize, serde::Serialize, ToSchema)]
pub(crate) struct IngestChunk {
    pub(crate) chunk_id: String,
    pub(crate) content: String,
    pub(crate) embedding: Vec<f32>,
    pub(crate) start: usize,
    pub(crate) end: usize,
    #[serde(default)]
    pub(crate) quality_score: Option<f32>,
    #[serde(default)]
    pub(crate) verified: Option<bool>,
}

impl IngestBatchRequest {
    pub(crate) fn to_ipc(&self) -> coco_protocol::CocoResult<worker_ipc::IngestBatchRequest> {
        let documents = self
            .documents
            .iter()
            .map(IngestDocument::to_ipc)
            .collect::<coco_protocol::CocoResult<Vec<_>>>()?;
        Ok(worker_ipc::IngestBatchRequest {
            activate: Some(self.activate),
            indexing_config_id: self.indexing_config_id.clone(),
            documents,
        })
    }
}

impl IngestDocument {
    fn to_ipc(&self) -> coco_protocol::CocoResult<worker_ipc::IngestDocument> {
        let chunks = self
            .chunks
            .iter()
            .map(IngestChunk::to_ipc)
            .collect::<coco_protocol::CocoResult<Vec<_>>>()?;
        Ok(worker_ipc::IngestDocument {
            doc_id: self.doc_id.clone(),
            source_ref: self.source_ref.clone(),
            title: self.title.clone(),
            content_hash: self.content_hash.clone(),
            quality_score: self.quality_score,
            verified: self.verified,
            chunks,
        })
    }
}

impl IngestChunk {
    fn to_ipc(&self) -> coco_protocol::CocoResult<worker_ipc::IngestChunk> {
        let start = u64::try_from(self.start)
            .map_err(|_| CocoError::user("chunk start is out of range"))?;
        let end =
            u64::try_from(self.end).map_err(|_| CocoError::user("chunk end is out of range"))?;
        Ok(worker_ipc::IngestChunk {
            chunk_id: self.chunk_id.clone(),
            content: self.content.clone(),
            embedding: self.embedding.clone(),
            start,
            end,
            quality_score: self.quality_score,
            verified: self.verified,
        })
    }
}

#[derive(Debug, serde::Serialize, ToSchema)]
pub(crate) struct IngestBatchResponse {
    pub(crate) job_id: String,
}

#[derive(Debug, serde::Serialize, ToSchema)]
pub(crate) struct JobStatusResponse {
    pub(crate) job_id: String,
    pub(crate) status: String,
    pub(crate) attempts: i32,
    pub(crate) version_id: Option<String>,
    pub(crate) error: Option<String>,
    pub(crate) created_at: String,
    pub(crate) updated_at: String,
}

impl From<IngestJobRecord> for JobStatusResponse {
    fn from(value: IngestJobRecord) -> Self {
        Self {
            job_id: value.id,
            status: value.status,
            attempts: value.attempts,
            version_id: value.version_id,
            error: value.error,
            created_at: value.created_at.to_rfc3339(),
            updated_at: value.updated_at.to_rfc3339(),
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

impl std::fmt::Display for ApiError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        std::fmt::Display::fmt(&self.0, f)
    }
}

impl std::error::Error for ApiError {}

impl IntoResponse for ApiError {
    fn into_response(self) -> Response {
        let error = self.0;
        let status = status_code_for_error(&error);
        let kind = error.kind();
        if kind != CocoErrorKind::User {
            warn!("internal error: {}", error);
        }
        let body = ErrorResponse {
            kind,
            message: error.into_public_message(),
        };
        (status, Json(body)).into_response()
    }
}

pub(crate) type ApiResult<T> = Result<T, ApiError>;

fn status_code_for_error(error: &CocoError) -> StatusCode {
    match error.kind() {
        CocoErrorKind::User => StatusCode::BAD_REQUEST,
        CocoErrorKind::Network => StatusCode::BAD_GATEWAY,
        CocoErrorKind::Storage => StatusCode::SERVICE_UNAVAILABLE,
        CocoErrorKind::System | CocoErrorKind::Compute => StatusCode::INTERNAL_SERVER_ERROR,
    }
}

fn default_true() -> bool {
    true
}
