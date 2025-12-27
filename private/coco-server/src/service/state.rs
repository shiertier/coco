use std::path::PathBuf;
use std::sync::Arc;

use coco_protocol::{VectorBackendConfig, VectorBackendKind};

use super::config::QueueMode;
use super::limiter::RateLimiter;
use crate::embedder::HttpEmbedder;
use crate::queue::RedisQueue;
use crate::storage::meta::ServerMetaStore;

#[derive(Clone)]
pub(crate) struct AppState {
    pub(crate) meta: ServerMetaStore,
    pub(crate) database_url: String,
    pub(crate) admin_key: String,
    pub(crate) api_key: String,
    pub(crate) limiter: Arc<RateLimiter>,
    pub(crate) queue: Option<RedisQueue>,
    pub(crate) queue_mode: QueueMode,
    pub(crate) worker_addr: Option<String>,
    pub(crate) ingest_blob_dir: Option<PathBuf>,
    pub(crate) ingest_wasm_module_ref: Option<String>,
    pub(crate) query_pg_pool: super::config::PgPoolConfig,
    pub(crate) embedder: Option<Arc<HttpEmbedder>>,
    pub(crate) embedding_dimensions: u32,
    pub(crate) org_max_documents: Option<i64>,
    pub(crate) org_max_chunks: Option<i64>,
    pub(crate) org_max_storage_bytes: Option<i64>,
    pub(crate) org_max_embeddings_per_day: Option<i64>,
    pub(crate) vector_backend_kind: VectorBackendKind,
    pub(crate) vector_db: Option<VectorBackendConfig>,
}
