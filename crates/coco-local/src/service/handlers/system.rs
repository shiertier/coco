use std::sync::Arc;

use axum::extract::{Query, State};
use axum::Json;
use chrono::Utc;
use tracing::warn;

use coco_protocol::CocoError;

use super::super::constants::DEFAULT_PRUNE_KEEP;
use super::super::query::{ensure_active_config_id, indexing_config_from_record, normalize_and_check};
use super::super::state::ServiceState;
use super::super::types::{
    ActivateConfigRequest, ActivateConfigResponse, ApiResult, HealthResponse, IndexingConfigResponse,
    ListConfigsQuery, ListConfigsResponse, PruneRequest, PruneResponse, RegisterProjectRequest,
    RegisterProjectResponse, UpsertConfigRequest, response_envelope,
};
use crate::ids::generate_id;
use crate::storage::meta::{NewProject, ProjectUpdate, DEFAULT_CONFIG_ID};

pub(crate) async fn health() -> Json<coco_protocol::ResponseEnvelope<HealthResponse>> {
    Json(response_envelope(HealthResponse {
        status: "ok",
        service: "coco",
    }))
}

pub(crate) async fn list_configs(
    State(state): State<Arc<ServiceState>>,
    Query(query): Query<ListConfigsQuery>,
) -> ApiResult<Json<coco_protocol::ResponseEnvelope<ListConfigsResponse>>> {
    let active_config_id = if let Some(project_id) = query.project_id.as_deref() {
        if project_id.trim().is_empty() {
            return Err(CocoError::user("project_id must not be empty").into());
        }
        let project = state
            .meta
            .get_project(project_id)
            .await?
            .ok_or_else(|| CocoError::user("project not found"))?;
        Some(ensure_active_config_id(&project.active_config_id)?)
    } else {
        None
    };

    let configs = state
        .meta
        .list_indexing_configs()
        .await?
        .into_iter()
        .map(indexing_config_from_record)
        .collect();

    Ok(Json(response_envelope(ListConfigsResponse {
        configs,
        active_config_id,
    })))
}

pub(crate) async fn upsert_config(
    State(state): State<Arc<ServiceState>>,
    Json(payload): Json<UpsertConfigRequest>,
) -> ApiResult<Json<coco_protocol::ResponseEnvelope<IndexingConfigResponse>>> {
    let config = payload.config;
    let config_id = normalize_and_check(&config.config_id, "config_id")?;
    if config_id == DEFAULT_CONFIG_ID {
        return Err(CocoError::user("default indexing config cannot be modified").into());
    }
    if config.vector_backend.is_some() {
        return Err(CocoError::user("vector_backend is not supported in local mode").into());
    }

    let new_config = crate::storage::meta::NewIndexingConfig {
        config_id: config_id.clone(),
        chunking: config.chunking,
        embedding: config.embedding,
        vector_backend: config.vector_backend,
        vector_metric: config.vector_metric,
        index_params: config.index_params,
        created_at: Utc::now(),
    };

    let record = if state.meta.get_indexing_config(&config_id).await?.is_some() {
        state.meta.update_indexing_config(new_config).await?
    } else {
        state.meta.create_indexing_config(new_config).await?
    };
    state.vector.invalidate_config(&config_id).await?;

    Ok(Json(response_envelope(IndexingConfigResponse {
        config: indexing_config_from_record(record),
    })))
}

pub(crate) async fn activate_config(
    State(state): State<Arc<ServiceState>>,
    Json(payload): Json<ActivateConfigRequest>,
) -> ApiResult<Json<coco_protocol::ResponseEnvelope<ActivateConfigResponse>>> {
    if payload.project_id.trim().is_empty() {
        return Err(CocoError::user("project_id must not be empty").into());
    }
    let config_id = normalize_and_check(&payload.config_id, "config_id")?;
    state.meta.ensure_indexing_config_exists(&config_id).await?;

    let update = state
        .meta
        .update_project(
            &payload.project_id,
            ProjectUpdate {
                active_config_id: Some(config_id.clone()),
                ..Default::default()
            },
        )
        .await?;
    if update.is_none() {
        return Err(CocoError::user("project not found").into());
    }

    Ok(Json(response_envelope(ActivateConfigResponse {
        active_config_id: config_id,
    })))
}

pub(crate) async fn register_project(
    State(state): State<Arc<ServiceState>>,
    Json(payload): Json<RegisterProjectRequest>,
) -> ApiResult<Json<coco_protocol::ResponseEnvelope<RegisterProjectResponse>>> {
    let project_id = payload
        .id
        .unwrap_or_else(|| generate_id("proj", payload.path.as_bytes()));
    if project_id.trim().is_empty() {
        return Err(CocoError::user("project id must not be empty").into());
    }
    if payload.name.trim().is_empty() {
        return Err(CocoError::user("project name must not be empty").into());
    }
    if payload.path.trim().is_empty() {
        return Err(CocoError::user("project path must not be empty").into());
    }

    if let Some(existing) = state.meta.get_project(&project_id).await? {
        return Ok(Json(response_envelope(RegisterProjectResponse::from(existing))));
    }

    let record = state
        .meta
        .create_project(NewProject {
            id: project_id.clone(),
            name: payload.name,
            path: payload.path,
            created_at: Utc::now(),
            active_version_id: None,
            active_config_id: DEFAULT_CONFIG_ID.to_string(),
        })
        .await?;

    if let Some(watcher) = &state.watcher {
        if let Err(err) = watcher.watch_project(record.clone()) {
            warn!("failed to watch project {}: {err}", record.id);
        }
    }

    Ok(Json(response_envelope(RegisterProjectResponse::from(record))))
}

pub(crate) async fn prune_project(
    State(state): State<Arc<ServiceState>>,
    Json(payload): Json<PruneRequest>,
) -> ApiResult<Json<coco_protocol::ResponseEnvelope<PruneResponse>>> {
    if payload.project_id.trim().is_empty() {
        return Err(CocoError::user("project_id must not be empty").into());
    }
    let keep = payload.keep.unwrap_or(DEFAULT_PRUNE_KEEP);
    let versions = state
        .meta
        .gc_project_versions(&payload.project_id, keep)
        .await?;
    if !versions.is_empty() {
        let configs = state.meta.list_indexing_configs().await?;
        for version_id in versions {
            for config in &configs {
                let backend = state
                    .vector
                    .backend_for_config(
                        &config.config_id,
                        &version_id,
                        config.vector_metric,
                        config.index_params.as_ref(),
                    )
                    .await?;
                backend.delete_by_version().await?;
            }
        }
    }
    Ok(Json(response_envelope(PruneResponse { status: "ok" })))
}
