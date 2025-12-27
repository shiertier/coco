use std::fs;

use axum::http::HeaderMap;
use axum::response::Response;
use axum::{extract::State, Json};

use coco_protocol::{
    validate_search_intent, CocoError, FilterField, IndexingPlan, ResponseMeta, ResponseStatus,
    SearchIntent, SearchIntentInput, ValidationContext, VectorStore,
};

use super::super::constants::{SERVER_FILTER_FIELDS, SERVER_FILTER_OPS};
use super::super::indexing::validate_indexing_config_backend;
use super::super::ingest::{build_ipc_request, write_ingest_blob};
use super::super::middleware::org_user_project_from_headers;
use super::super::query::{
    apply_retrieval_config, extract_indexing_config_id, extract_version_id_filter,
    fill_query_embedding, project_for_request,
};
use super::super::state::AppState;
use super::super::stream::stream_results;
use super::super::types::{
    ApiResult, IndexRequest, IndexResponse, IngestBatchRequest, IngestBatchResponse,
    MemoQueryRequest, MemoQueryResponseEnvelope, QueryRequest, QueryResponse,
};
use super::super::vector_backend::{build_vector_backend, VectorBackendRequest};
use super::super::worker::submit_ingest_ipc;
use super::super::utils::generate_id;
use crate::storage::meta::NewIngestJob;

#[utoipa::path(
    post,
    path = "/v1/docs/query",
    tag = "Docs",
    params(
        ("x-coco-org-id" = String, Header, description = "Organization identifier"),
        ("x-coco-user-id" = String, Header, description = "User identifier"),
        ("x-coco-project-id" = String, Header, description = "Project identifier")
    ),
    request_body = QueryRequest,
    responses(
        (status = 200, description = "Query results", body = QueryResponseEnvelope),
        (status = 400, description = "Invalid request", body = coco_protocol::ErrorResponse),
        (status = 401, description = "Unauthorized", body = coco_protocol::ErrorResponse),
        (status = 429, description = "Rate limited", body = coco_protocol::ErrorResponse)
    )
)]
pub(crate) async fn query_documents(
    State(state): State<AppState>,
    headers: HeaderMap,
    Json(payload): Json<QueryRequest>,
) -> ApiResult<Response> {
    let (org_id, user_id, project_id) = org_user_project_from_headers(&headers)?;
    if payload.indexing_config.is_some() {
        return Err(CocoError::user("indexing_config must be registered before query").into());
    }
    let mut intent: SearchIntentInput = payload.intent.try_into()?;
    if let Some(retrieval) = payload.retrieval_config {
        apply_retrieval_config(&mut intent, retrieval, state.vector_backend_kind)?;
    }
    let requested_version_id = extract_version_id_filter(&mut intent)?;

    let project = project_for_request(&state, &org_id, &user_id, &project_id).await?;
    let active_version_id = project
        .active_version_id
        .ok_or_else(|| CocoError::user("active_version_id is not set"))?;
    let active_config_id = project.active_config_id.clone();
    let requested_config_id = match payload.indexing_config_id.as_deref() {
        Some(config_id) => {
            let normalized = coco_protocol::normalize_config_id(config_id)?;
            if normalized != config_id {
                return Err(CocoError::user("indexing_config_id must be normalized").into());
            }
            Some(config_id.to_string())
        }
        None => extract_indexing_config_id(&intent)?,
    };
    let active_config = state
        .meta
        .get_indexing_config(&org_id, &active_config_id)
        .await?
        .ok_or_else(|| CocoError::user("active indexing config not found"))?;
    let selected_config = if let Some(config_id) = requested_config_id.as_deref() {
        intent.indexing_config_id = Some(config_id.to_string());
        if config_id == active_config_id {
            active_config.clone()
        } else {
            state
                .meta
                .get_indexing_config(&org_id, config_id)
                .await?
                .ok_or_else(|| CocoError::user("indexing config not found"))?
        }
    } else {
        intent.indexing_config_id = Some(active_config_id.clone());
        active_config.clone()
    };
    let embedding_dimensions = selected_config
        .embedding
        .dimensions
        .ok_or_else(|| CocoError::user("embedding dimensions must be set for indexing config"))?;
    let embedding_dimensions = usize::try_from(embedding_dimensions)
        .map_err(|_| CocoError::user("embedding dimensions out of range"))?;
    validate_indexing_config_backend(&selected_config, state.vector_backend_kind)?;

    fill_query_embedding(&mut intent, state.embedder.as_deref()).await?;
    let intent = SearchIntent::try_from(intent)?;
    let context = ValidationContext {
        embedding_dimensions: Some(embedding_dimensions),
        expected_vector_backend: None,
        allowed_filter_fields: Some(
            SERVER_FILTER_FIELDS
                .iter()
                .map(|field| FilterField::new(*field))
                .collect::<Result<Vec<_>, _>>()?,
        ),
        allowed_filter_ops: Some(SERVER_FILTER_OPS.to_vec()),
        active_config_id: Some(active_config_id.clone()),
    };
    validate_search_intent(&intent, &context)?;
    let is_stale = requested_version_id
        .as_deref()
        .is_some_and(|requested| requested != active_version_id)
        || requested_config_id
            .as_deref()
            .is_some_and(|requested| requested != active_config_id);
    let status = if is_stale {
        ResponseStatus::Stale
    } else {
        ResponseStatus::Fresh
    };

    let selected_version_id = requested_version_id
        .clone()
        .unwrap_or_else(|| active_version_id.clone());
    let selected_config_id = selected_config.config_id.clone();

    let backend = build_vector_backend(
        &state,
        VectorBackendRequest {
            org_id: org_id.clone(),
            user_id: user_id.clone(),
            project_id: project_id.clone(),
            version_id: selected_version_id,
            config_id: selected_config_id,
            vector_metric: selected_config.vector_metric,
            index_params: selected_config.index_params.clone(),
            use_collection_alias: !is_stale,
        },
    )
    .await?;
    let results = backend.search_vectors(intent).await?;
    let response = stream_results(results, status)?;
    Ok(response)
}

#[utoipa::path(
    post,
    path = "/v1/docs/index",
    tag = "Docs",
    params(
        ("x-coco-org-id" = String, Header, description = "Organization identifier"),
        ("x-coco-user-id" = String, Header, description = "User identifier"),
        ("x-coco-project-id" = String, Header, description = "Project identifier")
    ),
    request_body = IndexRequest,
    responses(
        (status = 200, description = "Reindex accepted", body = IndexResponse),
        (status = 400, description = "Invalid request", body = coco_protocol::ErrorResponse),
        (status = 401, description = "Unauthorized", body = coco_protocol::ErrorResponse),
        (status = 429, description = "Rate limited", body = coco_protocol::ErrorResponse)
    )
)]
pub(crate) async fn index_documents(
    State(state): State<AppState>,
    headers: HeaderMap,
    Json(payload): Json<IndexRequest>,
) -> ApiResult<Json<IndexResponse>> {
    let (org_id, user_id, project_id) = org_user_project_from_headers(&headers)?;
    let project = project_for_request(&state, &org_id, &user_id, &project_id).await?;
    let config_id = match payload.indexing_config_id.as_deref() {
        Some(config_id) => {
            let normalized = coco_protocol::normalize_config_id(config_id)?;
            if normalized != config_id {
                return Err(CocoError::user("indexing_config_id must be normalized").into());
            }
            config_id.to_string()
        }
        None => project.active_config_id,
    };
    let config = state
        .meta
        .get_indexing_config(&org_id, &config_id)
        .await?
        .ok_or_else(|| CocoError::user("indexing config not found"))?;
    validate_indexing_config_backend(&config, state.vector_backend_kind)?;
    Ok(Json(IndexResponse {
        status: "accepted".to_string(),
    }))
}

#[utoipa::path(
    post,
    path = "/v1/memo/query",
    tag = "Memo",
    request_body = MemoQueryRequest,
    responses(
        (status = 200, description = "Memo query results", body = MemoQueryResponseEnvelope),
        (status = 400, description = "Invalid request", body = coco_protocol::ErrorResponse),
        (status = 401, description = "Unauthorized", body = coco_protocol::ErrorResponse),
        (status = 429, description = "Rate limited", body = coco_protocol::ErrorResponse)
    )
)]
pub(crate) async fn query_memos(
    State(state): State<AppState>,
    Json(payload): Json<MemoQueryRequest>,
) -> ApiResult<Json<MemoQueryResponseEnvelope>> {
    if payload.session_token.trim().is_empty() {
        return Err(CocoError::user("session_token must not be empty").into());
    }
    if payload.intent.indexing_config_id.is_some() {
        return Err(CocoError::user("indexing_config_id is not supported for memo query").into());
    }
    let mut intent: SearchIntentInput = payload.intent.try_into()?;
    if let Some(retrieval) = payload.retrieval_config {
        if retrieval.vector_backend.is_some() {
            return Err(CocoError::user(
                "retrieval_config.vector_backend is not supported for memo query",
            )
            .into());
        }
        apply_retrieval_config(&mut intent, retrieval, state.vector_backend_kind)?;
    }
    let intent = SearchIntent::try_from(intent)?;
    let context = ValidationContext {
        embedding_dimensions: None,
        expected_vector_backend: None,
        allowed_filter_fields: Some(
            SERVER_FILTER_FIELDS
                .iter()
                .map(|field| FilterField::new(*field))
                .collect::<Result<Vec<_>, _>>()?,
        ),
        allowed_filter_ops: Some(SERVER_FILTER_OPS.to_vec()),
        active_config_id: None,
    };
    validate_search_intent(&intent, &context)?;
    Ok(Json(MemoQueryResponseEnvelope {
        meta: ResponseMeta {
            status: ResponseStatus::Fresh,
        },
        data: QueryResponse { results: Vec::new() },
    }))
}

#[utoipa::path(
    post,
    path = "/v1/ingest/batch",
    tag = "Docs",
    params(
        ("x-coco-org-id" = String, Header, description = "Organization identifier"),
        ("x-coco-user-id" = String, Header, description = "User identifier"),
        ("x-coco-project-id" = String, Header, description = "Project identifier")
    ),
    request_body = IngestBatchRequest,
    responses(
        (status = 200, description = "Batch accepted", body = IngestBatchResponse),
        (status = 400, description = "Invalid request", body = coco_protocol::ErrorResponse),
        (status = 401, description = "Unauthorized", body = coco_protocol::ErrorResponse),
        (status = 429, description = "Rate limited", body = coco_protocol::ErrorResponse)
    )
)]
pub(crate) async fn ingest_batch(
    State(state): State<AppState>,
    headers: HeaderMap,
    Json(payload): Json<IngestBatchRequest>,
) -> ApiResult<Json<IngestBatchResponse>> {
    let (org_id, user_id, project_id) = org_user_project_from_headers(&headers)?;
    let mut payload = payload;
    if payload.documents.is_empty() {
        return Err(CocoError::user("documents must not be empty").into());
    }

    let project = project_for_request(&state, &org_id, &user_id, &project_id).await?;
    let config_id = match payload.indexing_config_id.as_deref() {
        Some(config_id) => {
            let normalized = coco_protocol::normalize_config_id(config_id)?;
            if normalized != config_id {
                return Err(CocoError::user("indexing_config_id must be normalized").into());
            }
            config_id.to_string()
        }
        None => project.active_config_id,
    };
    let config = state
        .meta
        .get_indexing_config(&org_id, &config_id)
        .await?
        .ok_or_else(|| CocoError::user("indexing config not found"))?;
    validate_indexing_config_backend(&config, state.vector_backend_kind)?;
    payload.indexing_config_id = Some(config_id);

    let created_at = chrono::Utc::now();
    let blob_ref = match state.ingest_blob_dir.as_ref() {
        Some(dir) => Some(write_ingest_blob(dir, &payload)?),
        None => None,
    };
    let job_payload = super::super::types::IngestJobPayload {
        api_version: env!("CARGO_PKG_VERSION").to_string(),
        request: if blob_ref.is_some() { None } else { Some(payload) },
        blob_ref,
        plan: Some(IndexingPlan::default()),
        wasm_module_ref: state.ingest_wasm_module_ref.clone(),
    };
    let payload_json = serde_json::to_string(&job_payload).map_err(CocoError::system)?;
    let job_id = generate_id(
        "job",
        format!(
            "{}:{}:{}:{}:{}",
            org_id,
            user_id,
            project_id,
            created_at.to_rfc3339(),
            payload_json.len()
        )
        .as_bytes(),
    );
    let ipc_request = match state.worker_addr.as_deref() {
        Some(_) => Some(build_ipc_request(
            &job_id,
            &org_id,
            &user_id,
            &project_id,
            &job_payload,
        )?),
        None => None,
    };

    if let Err(err) = state
        .meta
        .create_ingest_job(NewIngestJob {
            id: job_id.clone(),
            org_id: org_id.clone(),
            user_id: user_id.clone(),
            project_id: project_id.clone(),
            payload: payload_json,
            created_at,
            updated_at: created_at,
        })
        .await
    {
        if let Some(blob_ref) = job_payload.blob_ref.as_deref() {
            let _ = fs::remove_file(blob_ref);
        }
        return Err(err.into());
    }

    if let Some(queue) = &state.queue {
        if let Err(err) = queue.enqueue(&job_id).await {
            tracing::warn!("failed to enqueue job {}: {err}", job_id);
        }
    }
    if let (Some(worker_addr), Some(ipc_request)) =
        (state.worker_addr.as_deref(), ipc_request)
    {
        if let Err(err) = submit_ingest_ipc(worker_addr, ipc_request).await {
            tracing::warn!("failed to dispatch ingest job {} to IPC: {err}", job_id);
        }
    }

    Ok(Json(IngestBatchResponse { job_id }))
}

#[utoipa::path(
    post,
    path = "/v1/docs/import",
    tag = "Docs",
    params(
        ("x-coco-org-id" = String, Header, description = "Organization identifier"),
        ("x-coco-user-id" = String, Header, description = "User identifier"),
        ("x-coco-project-id" = String, Header, description = "Project identifier")
    ),
    request_body = IngestBatchRequest,
    responses(
        (status = 200, description = "Batch accepted", body = IngestBatchResponse),
        (status = 400, description = "Invalid request", body = coco_protocol::ErrorResponse),
        (status = 401, description = "Unauthorized", body = coco_protocol::ErrorResponse),
        (status = 429, description = "Rate limited", body = coco_protocol::ErrorResponse)
    )
)]
pub(crate) async fn import_documents(
    State(state): State<AppState>,
    headers: HeaderMap,
    Json(payload): Json<IngestBatchRequest>,
) -> ApiResult<Json<IngestBatchResponse>> {
    ingest_batch(State(state), headers, Json(payload)).await
}
