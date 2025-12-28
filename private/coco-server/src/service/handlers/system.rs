use axum::Json;
use axum::extract::{Query, State};

use coco_core::{normalize_config_id, validate_indexing_config};
use coco_protocol::{CocoError, ValidationContext};

use super::super::constants::{WORKER_STALE_SECS, WORKER_STATUS_ID};
use super::super::indexing::{
    default_indexing_config, ensure_vector_backend, indexing_config_from_record,
};
use super::super::openapi::openapi_document;
use super::super::state::AppState;
use super::super::status::{queue_status, vector_backend_status};
use super::super::types::{
    ActivateConfigRequest, ActivateConfigResponse, ApiResult, HealthResponse,
    IndexingConfigResponse, ListConfigsQuery, ListConfigsResponse, PruneRequest, PruneResponse,
    RegisterProjectRequest, RegisterProjectResponse, UpsertConfigRequest, WorkerStatusResponse,
};
use super::super::utils::generate_id;
use crate::storage::meta::{
    DEFAULT_CONFIG_ID, NewIndexingConfig, NewOrganization, NewProject, ProjectUpdate,
};

#[utoipa::path(
    get,
    path = "/v1/sys/health",
    tag = "System",
    responses((status = 200, description = "Service health", body = HealthResponse))
)]
pub(crate) async fn health(State(state): State<AppState>) -> Json<HealthResponse> {
    let worker = match state.meta.get_worker_status(WORKER_STATUS_ID).await {
        Ok(Some(record)) => {
            let now = chrono::Utc::now();
            let age = now.signed_duration_since(record.updated_at).num_seconds();
            let status = if age <= WORKER_STALE_SECS {
                "ok"
            } else {
                "stale"
            };
            WorkerStatusResponse {
                status: status.to_string(),
                version: record.version,
                updated_at: Some(record.updated_at.to_rfc3339()),
            }
        }
        Ok(None) => WorkerStatusResponse {
            status: "missing".to_string(),
            version: "unknown".to_string(),
            updated_at: None,
        },
        Err(err) => {
            tracing::warn!("failed to read worker status: {err}");
            WorkerStatusResponse {
                status: "unknown".to_string(),
                version: "unknown".to_string(),
                updated_at: None,
            }
        }
    };
    let vector_backend = vector_backend_status(&state).await;
    let queue = queue_status(&state).await;
    Json(HealthResponse {
        status: "ok",
        service: "coco",
        version: env!("CARGO_PKG_VERSION"),
        worker,
        queue,
        vector_backend,
    })
}

#[utoipa::path(
    get,
    path = "/v1/sys/openapi",
    tag = "System",
    responses((
        status = 200,
        description = "OpenAPI document",
        body = String,
        content_type = "application/json"
    ))
)]
pub(crate) async fn openapi_json() -> Json<utoipa::openapi::OpenApi> {
    Json(openapi_document())
}

#[utoipa::path(
    post,
    path = "/v1/sys/register",
    tag = "System",
    request_body = RegisterProjectRequest,
    responses(
        (status = 200, description = "Project registered", body = RegisterProjectResponse),
        (status = 400, description = "Invalid request", body = coco_protocol::ErrorResponse),
        (status = 401, description = "Unauthorized", body = coco_protocol::ErrorResponse)
    )
)]
pub(crate) async fn register_project(
    State(state): State<AppState>,
    Json(payload): Json<RegisterProjectRequest>,
) -> ApiResult<Json<RegisterProjectResponse>> {
    if payload.org_id.trim().is_empty() {
        return Err(CocoError::user("org_id must not be empty").into());
    }
    if payload.user_id.trim().is_empty() {
        return Err(CocoError::user("user_id must not be empty").into());
    }
    if payload.name.trim().is_empty() {
        return Err(CocoError::user("project name must not be empty").into());
    }
    if payload.source_ref.trim().is_empty() {
        return Err(CocoError::user("source_ref must not be empty").into());
    }
    if source_ref_contains_path(&payload.source_ref) {
        return Err(CocoError::user(
            "source_ref must not contain path separators or drive prefixes",
        )
        .into());
    }
    let _ = payload.platform.as_deref();

    let org_id = payload.org_id.clone();
    let user_id = payload.user_id.clone();
    if state.meta.get_organization(&org_id).await?.is_none() {
        let _ = state
            .meta
            .create_organization(NewOrganization {
                id: org_id.clone(),
                name: payload.org_name.clone().unwrap_or_else(|| org_id.clone()),
                created_at: chrono::Utc::now(),
                max_documents: state.org_max_documents,
                max_chunks: state.org_max_chunks,
                max_storage_bytes: state.org_max_storage_bytes,
                max_embeddings_per_day: state.org_max_embeddings_per_day,
            })
            .await?;
    }

    state
        .meta
        .ensure_default_indexing_config(default_indexing_config(&state, &org_id))
        .await?;
    if state
        .meta
        .get_indexing_config(&org_id, DEFAULT_CONFIG_ID)
        .await?
        .is_none()
    {
        return Err(CocoError::user("default indexing config missing").into());
    }

    let project_id = payload.project_id.clone().unwrap_or_else(|| {
        let seed = format!("{org_id}:{user_id}:{}", payload.source_ref);
        generate_id("proj", seed.as_bytes())
    });
    if let Some(existing) = state
        .meta
        .get_project(&org_id, &user_id, &project_id)
        .await?
    {
        return Ok(Json(RegisterProjectResponse::from(existing)));
    }

    let version = state
        .meta
        .create_project_version(&org_id, &user_id, &project_id, DEFAULT_CONFIG_ID)
        .await?;
    let record = state
        .meta
        .create_project(NewProject {
            id: project_id.clone(),
            org_id: org_id.clone(),
            user_id: user_id.clone(),
            name: payload.name,
            created_at: chrono::Utc::now(),
            active_version_id: Some(version.id.clone()),
            active_config_id: DEFAULT_CONFIG_ID.to_string(),
        })
        .await?;

    state
        .meta
        .activate_project_version(&org_id, &user_id, &project_id, &version.id)
        .await?;

    Ok(Json(RegisterProjectResponse::from(record)))
}

#[utoipa::path(
    post,
    path = "/v1/sys/prune",
    tag = "System",
    request_body = PruneRequest,
    responses(
        (status = 200, description = "Prune completed", body = PruneResponse),
        (status = 400, description = "Invalid request", body = coco_protocol::ErrorResponse),
        (status = 401, description = "Unauthorized", body = coco_protocol::ErrorResponse)
    )
)]
pub(crate) async fn prune_project(
    State(state): State<AppState>,
    Json(payload): Json<PruneRequest>,
) -> ApiResult<Json<PruneResponse>> {
    if payload.org_id.trim().is_empty() || payload.project_id.trim().is_empty() {
        return Err(CocoError::user("org_id and project_id must not be empty").into());
    }
    if payload.user_id.trim().is_empty() {
        return Err(CocoError::user("user_id must not be empty").into());
    }
    let keep = payload.keep.unwrap_or(3);
    let removed = state
        .meta
        .gc_project_versions(&payload.org_id, &payload.user_id, &payload.project_id, keep)
        .await?;
    Ok(Json(PruneResponse { removed }))
}

#[utoipa::path(
    get,
    path = "/v1/sys/configs",
    tag = "System",
    params(
        ("org_id" = String, Query, description = "Organization identifier"),
        ("user_id" = Option<String>, Query, description = "User identifier"),
        ("project_id" = Option<String>, Query, description = "Project identifier")
    ),
    responses(
        (status = 200, description = "Indexing configs", body = ListConfigsResponse),
        (status = 400, description = "Invalid request", body = coco_protocol::ErrorResponse),
        (status = 401, description = "Unauthorized", body = coco_protocol::ErrorResponse)
    )
)]
pub(crate) async fn list_configs(
    State(state): State<AppState>,
    Query(query): Query<ListConfigsQuery>,
) -> ApiResult<Json<ListConfigsResponse>> {
    let org_id = query.org_id.trim();
    if org_id.is_empty() {
        return Err(CocoError::user("org_id must not be empty").into());
    }
    let active_config_id = if let Some(project_id) = query.project_id.as_deref() {
        if project_id.trim().is_empty() {
            return Err(CocoError::user("project_id must not be empty").into());
        }
        let Some(user_id) = query.user_id.as_deref() else {
            return Err(CocoError::user("user_id must be set when project_id is provided").into());
        };
        if user_id.trim().is_empty() {
            return Err(CocoError::user("user_id must not be empty").into());
        }
        let project = state
            .meta
            .get_project(org_id, user_id, project_id)
            .await?
            .ok_or_else(|| CocoError::user("project not found"))?;
        Some(project.active_config_id)
    } else {
        None
    };

    let configs = state
        .meta
        .list_indexing_configs(org_id)
        .await?
        .into_iter()
        .map(indexing_config_from_record)
        .collect();

    Ok(Json(ListConfigsResponse {
        configs,
        active_config_id,
    }))
}

#[utoipa::path(
    post,
    path = "/v1/sys/configs",
    tag = "System",
    request_body = UpsertConfigRequest,
    responses(
        (status = 200, description = "Indexing config", body = IndexingConfigResponse),
        (status = 400, description = "Invalid request", body = coco_protocol::ErrorResponse),
        (status = 401, description = "Unauthorized", body = coco_protocol::ErrorResponse)
    )
)]
pub(crate) async fn upsert_config(
    State(state): State<AppState>,
    Json(payload): Json<UpsertConfigRequest>,
) -> ApiResult<Json<IndexingConfigResponse>> {
    let org_id = payload.org_id.trim();
    if org_id.is_empty() {
        return Err(CocoError::user("org_id must not be empty").into());
    }

    let mut config = payload.config;
    let normalized = normalize_config_id(&config.config_id)?;
    if normalized != config.config_id {
        return Err(CocoError::user("config_id must be normalized").into());
    }
    if normalized == DEFAULT_CONFIG_ID {
        return Err(CocoError::user("default indexing config cannot be modified").into());
    }
    config.config_id = normalized;

    ensure_vector_backend(&mut config, state.vector_backend_kind)?;
    let context = ValidationContext {
        expected_vector_backend: Some(state.vector_backend_kind),
        ..Default::default()
    };
    validate_indexing_config(&config, &context)?;

    let new_config = NewIndexingConfig {
        org_id: org_id.to_string(),
        config_id: config.config_id.clone(),
        chunking: config.chunking,
        embedding: config.embedding,
        vector_backend: config.vector_backend,
        vector_metric: config.vector_metric,
        index_params: config.index_params,
        created_at: chrono::Utc::now(),
    };

    let record = if state
        .meta
        .get_indexing_config(org_id, &new_config.config_id)
        .await?
        .is_some()
    {
        state.meta.update_indexing_config(new_config).await?
    } else {
        state.meta.create_indexing_config(new_config).await?
    };

    Ok(Json(IndexingConfigResponse {
        config: indexing_config_from_record(record),
    }))
}

#[utoipa::path(
    post,
    path = "/v1/sys/configs/activate",
    tag = "System",
    request_body = ActivateConfigRequest,
    responses(
        (status = 200, description = "Activated config", body = ActivateConfigResponse),
        (status = 400, description = "Invalid request", body = coco_protocol::ErrorResponse),
        (status = 401, description = "Unauthorized", body = coco_protocol::ErrorResponse)
    )
)]
pub(crate) async fn activate_config(
    State(state): State<AppState>,
    Json(payload): Json<ActivateConfigRequest>,
) -> ApiResult<Json<ActivateConfigResponse>> {
    let org_id = payload.org_id.trim();
    if org_id.is_empty() {
        return Err(CocoError::user("org_id must not be empty").into());
    }
    let user_id = payload.user_id.trim();
    if user_id.is_empty() {
        return Err(CocoError::user("user_id must not be empty").into());
    }
    if payload.project_id.trim().is_empty() {
        return Err(CocoError::user("project_id must not be empty").into());
    }
    let normalized = normalize_config_id(&payload.config_id)?;
    if normalized != payload.config_id {
        return Err(CocoError::user("config_id must be normalized").into());
    }

    let update = state
        .meta
        .update_project(
            org_id,
            user_id,
            &payload.project_id,
            ProjectUpdate {
                active_config_id: Some(normalized.clone()),
                ..Default::default()
            },
        )
        .await?;
    if update.is_none() {
        return Err(CocoError::user("project not found").into());
    }

    Ok(Json(ActivateConfigResponse {
        active_config_id: normalized,
    }))
}

fn source_ref_contains_path(source_ref: &str) -> bool {
    if source_ref.contains('/') || source_ref.contains('\\') {
        return true;
    }
    let mut chars = source_ref.chars();
    let Some(first) = chars.next() else {
        return false;
    };
    let Some(second) = chars.next() else {
        return false;
    };
    first.is_ascii_alphabetic() && second == ':'
}

#[cfg(test)]
mod tests;
