use std::sync::Arc;

use axum::extract::{Path as AxumPath, State};
use axum::Json;

use coco_core::build_search_intent;
use coco_protocol::{CocoError, StorageBackend};

use super::super::live::{maybe_apply_live_grep, refresh_results_from_fs, RefreshSummary};
use super::super::query::{
    apply_retrieval_config, ensure_embedding_dimensions, fill_query_embedding,
    load_indexing_config, resolve_import_config_id, resolve_query_config_id,
    validate_local_search_intent,
};
use super::super::state::ServiceState;
use super::super::types::{
    ApiResult, ChunkSummary, DocumentResponse, ImportRequest, ImportResponse, QueryRequest,
    QueryResponse, response_envelope,
};
use crate::ingest::{IngestRequest, Ingestor};

pub(crate) async fn import_document(
    State(state): State<Arc<ServiceState>>,
    Json(payload): Json<ImportRequest>,
) -> ApiResult<Json<coco_protocol::ResponseEnvelope<ImportResponse>>> {
    if payload.project_id.trim().is_empty() {
        return Err(CocoError::user("project_id must not be empty").into());
    }
    if payload.indexing_config.is_some() {
        return Err(CocoError::user("indexing_config must be registered before import").into());
    }

    let config_id = resolve_import_config_id(
        &state.meta,
        &payload.project_id,
        payload.indexing_config_id.as_deref(),
    )
    .await?;

    let path_hint = payload.path.clone();
    let ingestor = ingestor_from_state(&state);
    let result = ingestor
        .ingest_request(IngestRequest {
            project_id: payload.project_id,
            indexing_config_id: config_id,
            version_id: None,
            document_id: payload.document_id,
            content: payload.content,
            content_hash: None,
            file_type: payload.file_type,
            title: payload.title,
            path: payload.path,
            chunking: payload.chunking,
        })
        .await?;
    state.metrics.record_import(path_hint.as_deref());

    Ok(Json(response_envelope(ImportResponse {
        document_id: result.document_id,
        chunks: result.chunks.into_iter().map(ChunkSummary::from).collect(),
    })))
}

pub(crate) async fn query_documents(
    State(state): State<Arc<ServiceState>>,
    Json(payload): Json<QueryRequest>,
) -> ApiResult<Json<coco_protocol::ResponseEnvelope<QueryResponse>>> {
    if payload.indexing_config.is_some() {
        return Err(CocoError::user("indexing_config must be registered before query").into());
    }

    let project_id = payload
        .project_id
        .as_deref()
        .ok_or_else(|| CocoError::user("project_id must not be empty"))?;
    if project_id.trim().is_empty() {
        return Err(CocoError::user("project_id must not be empty").into());
    }

    let mut intent = payload.intent;
    if let Some(retrieval) = payload.retrieval_config {
        apply_retrieval_config(&mut intent, retrieval)?;
    }

    let requested_config_id = payload
        .indexing_config_id
        .as_deref()
        .or(intent.indexing_config_id.as_deref());
    let selected_config_id =
        resolve_query_config_id(&state.meta, Some(project_id), requested_config_id).await?;
    let config = load_indexing_config(&state.meta, &selected_config_id).await?;
    ensure_embedding_dimensions(&config, state.embedder.as_deref())?;

    let version_id = state.meta.ensure_active_version_id(project_id).await?;
    intent.indexing_config_id = Some(selected_config_id.clone());
    fill_query_embedding(&mut intent, state.embedder.as_deref()).await?;
    let intent = build_search_intent(intent)?;
    validate_local_search_intent(&intent, &config)?;

    let backend = state
        .vector
        .backend_for_config(
            &selected_config_id,
            &version_id,
            config.vector_metric,
            config.index_params.as_ref(),
        )
        .await?;
    let desired = intent.top_k().get() as usize;
    let query_text = intent.query_text().map(str::to_string);
    let results = backend.search_similar(intent).await?;
    let refresh = if state.live_retrieval_enabled {
        refresh_results_from_fs(
            &state.meta,
            results,
            state.live_retrieval_window_bytes,
        )
        .await
    } else {
        RefreshSummary::new(results)
    };
    let results = maybe_apply_live_grep(
        &state.meta,
        Some(project_id),
        query_text.as_deref(),
        refresh,
        desired,
        &state.live_grep,
    )
    .await;
    state.metrics.record_query();
    Ok(Json(response_envelope(QueryResponse { results })))
}

pub(crate) async fn get_document(
    State(state): State<Arc<ServiceState>>,
    AxumPath(doc_id): AxumPath<String>,
) -> ApiResult<Json<coco_protocol::ResponseEnvelope<DocumentResponse>>> {
    let Some(document) = state.meta.get_document(&doc_id).await? else {
        return Err(CocoError::user("document not found").into());
    };
    let chunks = state.meta.list_chunks(&doc_id).await?;
    Ok(Json(response_envelope(DocumentResponse {
        document: super::super::types::DocumentSummary::from(document),
        chunks: chunks.into_iter().map(ChunkSummary::from).collect(),
    })))
}

fn ingestor_from_state(state: &ServiceState) -> Ingestor {
    Ingestor::new(state.meta.clone(), state.vector.clone(), state.embedder.clone())
}
