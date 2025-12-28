use std::net::SocketAddr;
use std::sync::Arc;

use axum::Router;
use axum::middleware;
use axum::routing::{get, post};
use axum_server::tls_rustls::RustlsConfig;
use tower_http::trace::TraceLayer;
use tracing::info;

use coco_protocol::{CocoError, CocoResult, EmbeddingModel};

use super::config::ServerConfig;
use super::embedder::load_embedder;
use super::handlers::{
    activate_config, get_job, health, import_documents, index_documents, ingest_batch, job_events,
    list_configs, openapi_json, prune_project, query_documents, query_memos, register_project,
    upsert_config,
};
use super::limiter::RateLimiter;
use super::middleware::{access_log, require_admin, require_api};
use super::state::AppState;
use super::worker::check_worker_connectivity;
use crate::queue::RedisQueue;
use crate::storage::meta::{DEFAULT_EMBEDDING_DIM, ServerMetaStore};

/// Starts the server HTTP service.
pub async fn run(config: ServerConfig) -> CocoResult<()> {
    let meta = ServerMetaStore::connect(&config.database_url).await?;
    let limiter = Arc::new(RateLimiter::new(
        config.rate_limit_per_min,
        config.rate_limit_burst,
    ));
    let embedder = load_embedder()?;
    let embedding_dimensions = match embedder.as_ref() {
        Some(embedder) => {
            let dimensions = u32::try_from(embedder.dimensions())
                .map_err(|_| CocoError::user("embedding dimensions out of range"))?;
            if dimensions != DEFAULT_EMBEDDING_DIM {
                return Err(CocoError::user(
                    "embedding dimensions must match server vector index",
                ));
            }
            dimensions
        }
        None => DEFAULT_EMBEDDING_DIM,
    };
    let queue = match config.queue_mode {
        super::config::QueueMode::Postgres => None,
        super::config::QueueMode::Redis => {
            let url = config
                .redis_url
                .as_ref()
                .ok_or_else(|| CocoError::user("COCO_REDIS_URL required for redis queue"))?;
            Some(RedisQueue::connect(url, config.redis_queue.clone()).await?)
        }
    };
    if let Some(worker_addr) = config.worker_addr.as_deref() {
        check_worker_connectivity(worker_addr).await?;
    }

    let state = AppState {
        meta,
        database_url: config.database_url.clone(),
        admin_key: config.admin_key.clone(),
        api_key: config.api_key.clone(),
        limiter,
        queue,
        queue_mode: config.queue_mode,
        worker_addr: config.worker_addr.clone(),
        ingest_blob_dir: config.ingest_blob_dir.clone(),
        ingest_wasm_module_ref: config.ingest_wasm_module_ref.clone(),
        query_pg_pool: config.query_pg_pool,
        embedder,
        embedding_dimensions,
        org_max_documents: config.org_max_documents,
        org_max_chunks: config.org_max_chunks,
        org_max_storage_bytes: config.org_max_storage_bytes,
        org_max_embeddings_per_day: config.org_max_embeddings_per_day,
        vector_backend_kind: config.vector_backend_kind,
        vector_db: config.vector_db.clone(),
    };

    let admin_routes = Router::new()
        .route("/v1/sys/register", post(register_project))
        .route("/v1/sys/prune", post(prune_project))
        .route("/v1/sys/configs", get(list_configs).post(upsert_config))
        .route("/v1/sys/configs/activate", post(activate_config))
        .route("/v1/sys/openapi", get(openapi_json))
        .route_layer(middleware::from_fn_with_state(state.clone(), require_admin));

    let api_routes = Router::new()
        .route("/v1/docs/query", post(query_documents))
        .route("/v1/docs/index", post(index_documents))
        .route("/v1/docs/import", post(import_documents))
        .route("/v1/ingest/batch", post(ingest_batch))
        .route("/v1/memo/query", post(query_memos))
        .route("/v1/jobs/:id", get(get_job))
        .route("/v1/jobs/:id/events", get(job_events))
        .route_layer(middleware::from_fn_with_state(state.clone(), require_api));

    let app = Router::new()
        .route("/v1/sys/health", get(health))
        .merge(admin_routes)
        .merge(api_routes)
        .with_state(state)
        .layer(middleware::from_fn(access_log))
        .layer(TraceLayer::new_for_http());

    let addr: SocketAddr = format!("{}:{}", config.host, config.port)
        .parse()
        .map_err(|_| CocoError::user("invalid host or port"))?;
    if let Some(tls) = config.tls {
        let tls_config = RustlsConfig::from_pem_file(&tls.cert_path, &tls.key_path)
            .await
            .map_err(|err| CocoError::system(format!("failed to load TLS cert/key: {err}")))?;
        info!("coco-server listening on https://{addr}");
        axum_server::bind_rustls(addr, tls_config)
            .serve(app.into_make_service())
            .await
            .map_err(|err| CocoError::system(format!("https server error: {err}")))?;
    } else {
        if config.tls_mode == super::config::TlsMode::Proxy {
            info!("coco-server listening on http://{addr} (tls terminated by proxy)");
        } else {
            info!("coco-server listening on http://{addr}");
        }
        let listener = tokio::net::TcpListener::bind(addr)
            .await
            .map_err(|err| CocoError::system(format!("failed to bind {addr}: {err}")))?;
        axum::serve(listener, app)
            .await
            .map_err(|err| CocoError::system(format!("http server error: {err}")))?;
    }
    Ok(())
}
