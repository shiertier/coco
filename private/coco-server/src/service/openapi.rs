use utoipa::openapi::security::{Http, HttpAuthScheme, SecurityRequirement, SecurityScheme};
use utoipa::{Modify, OpenApi};

use super::types::{
    ActivateConfigRequest, ActivateConfigResponse, HealthResponse, IndexRequest, IndexResponse,
    IndexingConfigResponse, IngestBatchRequest, IngestBatchResponse, IngestChunk, IngestDocument,
    JobStatusResponse, ListConfigsQuery, ListConfigsResponse, MemoQueryRequest,
    MemoQueryResponseEnvelope, PruneRequest, PruneResponse, PublicFilter, PublicFilterOp,
    PublicSearchIntent, QueryRequest, QueryResponse, QueryResponseEnvelope, QueueStatusResponse,
    RegisterProjectRequest, RegisterProjectResponse, UpsertConfigRequest, VectorBackendStatus,
    WorkerStatusResponse,
};

#[derive(OpenApi)]
#[openapi(
    paths(
        crate::service::handlers::system::health,
        crate::service::handlers::system::register_project,
        crate::service::handlers::system::prune_project,
        crate::service::handlers::system::list_configs,
        crate::service::handlers::system::upsert_config,
        crate::service::handlers::system::activate_config,
        crate::service::handlers::docs::query_documents,
        crate::service::handlers::docs::index_documents,
        crate::service::handlers::docs::import_documents,
        crate::service::handlers::docs::query_memos,
        crate::service::handlers::docs::ingest_batch,
        crate::service::handlers::jobs::get_job,
        crate::service::handlers::jobs::job_events,
        crate::service::handlers::system::openapi_json
    ),
    components(
        schemas(
            HealthResponse,
            WorkerStatusResponse,
            QueueStatusResponse,
            VectorBackendStatus,
            RegisterProjectRequest,
            RegisterProjectResponse,
            PruneRequest,
            PruneResponse,
            ListConfigsResponse,
            ListConfigsQuery,
            UpsertConfigRequest,
            IndexingConfigResponse,
            ActivateConfigRequest,
            ActivateConfigResponse,
            QueryRequest,
            QueryResponse,
            QueryResponseEnvelope,
            IndexRequest,
            IndexResponse,
            MemoQueryRequest,
            MemoQueryResponseEnvelope,
            IngestBatchRequest,
            IngestDocument,
            IngestChunk,
            IngestBatchResponse,
            JobStatusResponse,
            PublicSearchIntent,
            coco_protocol::SearchHit,
            coco_protocol::SearchHitMeta,
            coco_protocol::ResponseMeta,
            coco_protocol::ResponseStatus,
            coco_protocol::Chunk,
            coco_protocol::TextSpan,
            coco_protocol::RetrievalMode,
            PublicFilter,
            PublicFilterOp,
            coco_protocol::RerankerConfig,
            coco_protocol::ErrorResponse,
            coco_protocol::CocoErrorKind,
            coco_protocol::VectorBackendKind,
            coco_protocol::VectorIndexParams,
            coco_protocol::VectorMetric,
            coco_protocol::HnswParams,
            coco_protocol::IvfPqParams,
        )
    ),
    modifiers(&SecurityAddon),
    tags(
        (name = "System", description = "System and admin endpoints"),
        (name = "Docs", description = "Document ingestion and retrieval"),
        (name = "Memo", description = "User memo retrieval"),
        (name = "Jobs", description = "Ingest job status endpoints")
    )
)]
struct ApiDoc;

struct SecurityAddon;

impl Modify for SecurityAddon {
    fn modify(&self, openapi: &mut utoipa::openapi::OpenApi) {
        let components = openapi.components.get_or_insert_with(Default::default);
        components.add_security_scheme(
            "bearer_auth",
            SecurityScheme::Http(Http::new(HttpAuthScheme::Bearer)),
        );
        openapi.security = Some(vec![SecurityRequirement::new(
            "bearer_auth",
            Vec::<String>::new(),
        )]);
    }
}

/// Generates the OpenAPI document for the server API.
pub fn openapi_document() -> utoipa::openapi::OpenApi {
    ApiDoc::openapi()
}
