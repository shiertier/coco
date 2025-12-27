#[cfg(feature = "server-storage")]
use sea_orm::{ConnectionTrait, DatabaseBackend, Statement};

use coco_protocol::{VectorBackendKind};

use super::config::QueueMode;
use super::state::AppState;
use super::types::{QueueStatusResponse, VectorBackendStatus};
use crate::storage::pg::{PgBackend, PgBackendConfig};

pub(crate) async fn queue_status(state: &AppState) -> QueueStatusResponse {
    match state.queue_mode {
        QueueMode::Redis => {
            let Some(queue) = state.queue.as_ref() else {
                return QueueStatusResponse {
                    mode: "redis".to_string(),
                    status: "missing".to_string(),
                    detail: Some("redis queue not initialized".to_string()),
                };
            };
            match queue.ping().await {
                Ok(()) => QueueStatusResponse {
                    mode: "redis".to_string(),
                    status: "ok".to_string(),
                    detail: None,
                },
                Err(_) => QueueStatusResponse {
                    mode: "redis".to_string(),
                    status: "error".to_string(),
                    detail: Some("redis unreachable".to_string()),
                },
            }
        }
        QueueMode::Postgres => postgres_queue_status(state).await,
    }
}

#[cfg(feature = "server-storage")]
async fn postgres_queue_status(state: &AppState) -> QueueStatusResponse {
    let ping = state
        .meta
        .connection()
        .execute(Statement::from_string(
            DatabaseBackend::Postgres,
            "SELECT 1".to_string(),
        ))
        .await;
    match ping {
        Ok(_) => QueueStatusResponse {
            mode: "postgres".to_string(),
            status: "ok".to_string(),
            detail: None,
        },
        Err(_) => QueueStatusResponse {
            mode: "postgres".to_string(),
            status: "error".to_string(),
            detail: Some("database unreachable".to_string()),
        },
    }
}

#[cfg(not(feature = "server-storage"))]
async fn postgres_queue_status(_state: &AppState) -> QueueStatusResponse {
    QueueStatusResponse {
        mode: "postgres".to_string(),
        status: "disabled".to_string(),
        detail: Some("server-storage feature disabled".to_string()),
    }
}

pub(crate) async fn vector_backend_status(state: &AppState) -> VectorBackendStatus {
    match state.vector_backend_kind {
        VectorBackendKind::PgVector => {
            let mut config = PgBackendConfig::new(
                state.database_url.clone(),
                "health".to_string(),
                "health".to_string(),
                "health".to_string(),
            );
            state.query_pg_pool.apply(&mut config);
            match PgBackend::connect(config).await {
                Ok(backend) => match backend.pgvector_version().await {
                    Ok(version) => VectorBackendStatus {
                        kind: state.vector_backend_kind,
                        status: "ok".to_string(),
                        version,
                        detail: None,
                    },
                    Err(err) => VectorBackendStatus {
                        kind: state.vector_backend_kind,
                        status: "error".to_string(),
                        version: None,
                        detail: Some(format!("pgvector version check failed: {err}")),
                    },
                },
                Err(err) => VectorBackendStatus {
                    kind: state.vector_backend_kind,
                    status: "error".to_string(),
                    version: None,
                    detail: Some(format!("pgvector connect failed: {err}")),
                },
            }
        }
        VectorBackendKind::Qdrant => {
            let Some(vector_db) = state.vector_db.as_ref() else {
                return VectorBackendStatus {
                    kind: state.vector_backend_kind,
                    status: "missing_config".to_string(),
                    version: None,
                    detail: Some("qdrant config not provided".to_string()),
                };
            };
            let url = vector_db.url.as_deref().unwrap_or("unknown").to_string();
            let mut builder = qdrant_client::Qdrant::from_url(&url);
            if let Some(api_key) = vector_db.api_key.as_ref() {
                builder = builder.api_key(api_key.clone());
            }
            match builder.build() {
                Ok(client) => match client.health_check().await {
                    Ok(reply) => VectorBackendStatus {
                        kind: state.vector_backend_kind,
                        status: "ok".to_string(),
                        version: Some(reply.version),
                        detail: None,
                    },
                    Err(err) => VectorBackendStatus {
                        kind: state.vector_backend_kind,
                        status: "error".to_string(),
                        version: None,
                        detail: Some(format!("qdrant health check failed: {err}")),
                    },
                },
                Err(err) => VectorBackendStatus {
                    kind: state.vector_backend_kind,
                    status: "error".to_string(),
                    version: None,
                    detail: Some(format!("qdrant client build failed: {err}")),
                },
            }
        }
    }
}
