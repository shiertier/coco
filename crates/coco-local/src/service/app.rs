use std::net::SocketAddr;
use std::sync::Arc;
use std::time::Duration;

use axum::Router;
use axum::middleware::from_fn;
use axum::routing::{get, post};
use chrono::Utc;
use tokio::net::TcpListener;
use tower_http::cors::CorsLayer;
use tracing::info;

use coco_protocol::{CocoError, CocoResult, EmbeddingConfig, VectorMetric};

use super::config::LocalServiceConfig;
use super::embedder::prepare_embedder;
use super::handlers::{
    activate_config, get_document, health, import_document, list_configs, prune_project,
    query_documents, register_project, upsert_config,
};
use super::middleware::access_log;
use super::state::{LiveGrepConfig, ServiceState};
use crate::ingest::Ingestor;
use crate::metrics::LocalMetrics;
use crate::storage::lance::LanceStore;
use crate::storage::meta::{DEFAULT_CONFIG_ID, LocalMetaStore, NewIndexingConfig};
use crate::watcher::{WatchConfig, start_watch_service};

/// Starts the local HTTP service.
pub async fn run(config: LocalServiceConfig, metrics: Arc<LocalMetrics>) -> CocoResult<()> {
    let meta = LocalMetaStore::connect(&config.meta_db_path).await?;
    meta.ensure_default_indexing_config(NewIndexingConfig {
        config_id: DEFAULT_CONFIG_ID.to_string(),
        chunking: config.chunking.clone(),
        embedding: EmbeddingConfig {
            model_name: config.model_name.clone(),
            dimensions: Some(config.model_dimensions as u32),
        },
        vector_backend: None,
        vector_metric: VectorMetric::L2,
        index_params: None,
        created_at: Utc::now(),
    })
    .await?;
    let vector = LanceStore::new(config.vector_path.clone(), config.model_dimensions);
    let default_config = meta
        .get_indexing_config(DEFAULT_CONFIG_ID)
        .await?
        .ok_or_else(|| CocoError::system("default indexing config missing"))?;
    let bootstrap_version = "bootstrap";
    vector
        .backend_for_config(
            &default_config.config_id,
            bootstrap_version,
            default_config.vector_metric,
            default_config.index_params.as_ref(),
        )
        .await?;
    let embedder = prepare_embedder(&config).await?;
    let ingestor = Ingestor::new(meta.clone(), vector.clone(), embedder.clone());
    let watcher =
        start_watch_service(ingestor, WatchConfig::from_env()?, Arc::clone(&metrics)).await?;

    let live_grep = LiveGrepConfig {
        enabled: config.live_grep_enabled,
        max_results: config.live_grep_max_results,
        timeout: Duration::from_millis(config.live_grep_timeout_ms),
    };

    let state = Arc::new(ServiceState {
        meta,
        vector,
        embedder,
        watcher,
        metrics,
        live_retrieval_enabled: config.live_retrieval_enabled,
        live_retrieval_window_bytes: config.live_retrieval_window_bytes,
        live_grep,
    });

    let app = Router::new()
        .route("/v1/sys/health", get(health))
        .route("/v1/sys/register", post(register_project))
        .route("/v1/sys/prune", post(prune_project))
        .route("/v1/sys/configs", get(list_configs).post(upsert_config))
        .route("/v1/sys/configs/activate", post(activate_config))
        .route("/v1/docs/import", post(import_document))
        .route("/v1/docs/query", post(query_documents))
        .route("/v1/docs/:id", get(get_document))
        .with_state(state)
        .layer(CorsLayer::permissive())
        .layer(from_fn(access_log));

    let addr: SocketAddr = format!("{}:{}", config.host, config.port)
        .parse()
        .map_err(|_| CocoError::user("invalid host or port"))?;
    info!("coco-local listening on {addr}");
    let listener = TcpListener::bind(addr)
        .await
        .map_err(|err| CocoError::system(format!("failed to bind: {err}")))?;
    axum::serve(listener, app)
        .await
        .map_err(|err| CocoError::system(format!("http server error: {err}")))?;
    Ok(())
}
