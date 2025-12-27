use std::path::PathBuf;
use std::time::Duration;

use coco_protocol::{CocoError, CocoResult, VectorBackendConfig, VectorBackendKind};
use tracing::warn;

use super::constants::{
    DEFAULT_HOST, DEFAULT_PORT, DEFAULT_QUEUE_MODE, DEFAULT_RATE_LIMIT_BURST,
    DEFAULT_RATE_LIMIT_PER_MIN, DEFAULT_REDIS_QUEUE,
};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum QueueMode {
    Postgres,
    Redis,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum TlsMode {
    Tls,
    Proxy,
}

#[derive(Debug, Clone)]
pub struct TlsConfig {
    pub cert_path: String,
    pub key_path: String,
}

#[derive(Debug, Clone, Copy, Default)]
pub(crate) struct PgPoolConfig {
    pub(crate) max_connections: Option<u32>,
    pub(crate) min_connections: Option<u32>,
    pub(crate) connect_timeout: Option<Duration>,
}

impl PgPoolConfig {
    pub(crate) fn apply(&self, config: &mut crate::storage::pg::PgBackendConfig) {
        if let Some(max_connections) = self.max_connections {
            config.max_connections = max_connections;
        }
        if let Some(min_connections) = self.min_connections {
            config.min_connections = min_connections;
        }
        if let Some(connect_timeout) = self.connect_timeout {
            config.connect_timeout = connect_timeout;
        }
    }
}

/// Server configuration loaded from environment.
#[derive(Debug, Clone)]
pub struct ServerConfig {
    /// Host to bind.
    pub host: String,
    /// Port to bind.
    pub port: u16,
    /// TLS mode configuration.
    pub tls_mode: TlsMode,
    /// TLS certificate/key pair for HTTPS.
    pub tls: Option<TlsConfig>,
    /// PostgreSQL database URL.
    pub database_url: String,
    /// Admin API key.
    pub admin_key: String,
    /// Client API key.
    pub api_key: String,
    /// Requests per minute limit.
    pub rate_limit_per_min: u32,
    /// Rate limiter burst capacity.
    pub rate_limit_burst: u32,
    /// Default max documents per organization.
    pub org_max_documents: Option<i64>,
    /// Default max chunks per organization.
    pub org_max_chunks: Option<i64>,
    /// Default max storage bytes per organization.
    pub org_max_storage_bytes: Option<i64>,
    /// Default max embedding calls per day per organization.
    pub org_max_embeddings_per_day: Option<i64>,
    /// Queue mode for ingest jobs.
    pub queue_mode: QueueMode,
    /// Optional Redis URL when using Redis queue.
    pub redis_url: Option<String>,
    /// Redis list key for ingest job notifications.
    pub redis_queue: String,
    /// Optional worker gRPC address for IPC.
    pub worker_addr: Option<String>,
    /// Optional directory for ingest blob payloads.
    pub ingest_blob_dir: Option<PathBuf>,
    /// Optional wasm rule module reference for ingest jobs.
    pub ingest_wasm_module_ref: Option<String>,
    /// Query pool configuration for pgvector connections.
    pub(crate) query_pg_pool: PgPoolConfig,
    /// Selected vector backend kind.
    pub vector_backend_kind: VectorBackendKind,
    /// Optional vector database configuration.
    pub vector_db: Option<VectorBackendConfig>,
}

impl ServerConfig {
    /// Loads configuration from environment variables.
    pub fn from_env() -> CocoResult<Self> {
        let host = env_optional("COCO_HOST").unwrap_or_else(|| DEFAULT_HOST.to_string());
        let port = env_u16("COCO_PORT", DEFAULT_PORT)?;
        let tls_mode = tls_mode_from_env()?;
        let tls_cert = env_optional("COCO_TLS_CERT");
        let tls_key = env_optional("COCO_TLS_KEY");
        let tls = match (tls_cert, tls_key) {
            (Some(cert_path), Some(key_path)) => Some(TlsConfig {
                cert_path,
                key_path,
            }),
            (None, None) => None,
            _ => {
                return Err(CocoError::user(
                    "COCO_TLS_CERT and COCO_TLS_KEY must be set together",
                ))
            }
        };
        if tls.is_none() && tls_mode == TlsMode::Tls {
            return Err(CocoError::user(
                "TLS is required; set COCO_TLS_CERT/COCO_TLS_KEY or COCO_TLS_MODE=proxy",
            ));
        }
        let database_url = resolve_database_url()?;
        let admin_key = env_required("COCO_ADMIN_KEY")?;
        let api_key = env_required("COCO_API_KEY")?;
        let rate_limit_per_min =
            env_u32("COCO_RATE_LIMIT_PER_MIN", DEFAULT_RATE_LIMIT_PER_MIN)?;
        let rate_limit_burst = env_u32("COCO_RATE_LIMIT_BURST", DEFAULT_RATE_LIMIT_BURST)?;
        let org_max_documents = env_i64_optional("COCO_ORG_MAX_DOCUMENTS")?;
        let org_max_chunks = env_i64_optional("COCO_ORG_MAX_CHUNKS")?;
        let org_max_storage_bytes = env_i64_optional("COCO_ORG_MAX_STORAGE_BYTES")?;
        let org_max_embeddings_per_day =
            env_i64_optional("COCO_ORG_MAX_EMBEDDINGS_PER_DAY")?;
        warn_if_mode_set();
        let queue_mode = queue_mode_from_env()?;
        let redis_url = env_optional("COCO_REDIS_URL");
        if queue_mode == QueueMode::Redis && redis_url.is_none() {
            return Err(CocoError::user("COCO_REDIS_URL required for redis queue"));
        }
        let redis_queue =
            env_optional("COCO_REDIS_QUEUE").unwrap_or_else(|| DEFAULT_REDIS_QUEUE.to_string());
        let worker_addr = env_optional("COCO_WORKER_ADDR");
        let ingest_blob_dir = env_optional("COCO_INGEST_BLOB_DIR").map(PathBuf::from);
        let ingest_wasm_module_ref = env_optional("COCO_INGEST_WASM_MODULE_REF");
        let query_pg_pool = pg_pool_config_from_env("COCO_QUERY")?;
        let vector_backend_kind = vector_backend_kind_from_env()?;
        let vector_db = vector_backend_config_from_env(vector_backend_kind)?;
        Ok(Self {
            host,
            port,
            tls_mode,
            tls,
            database_url,
            admin_key,
            api_key,
            rate_limit_per_min,
            rate_limit_burst,
            org_max_documents,
            org_max_chunks,
            org_max_storage_bytes,
            org_max_embeddings_per_day,
            queue_mode,
            redis_url,
            redis_queue,
            worker_addr,
            ingest_blob_dir,
            ingest_wasm_module_ref,
            query_pg_pool,
            vector_backend_kind,
            vector_db,
        })
    }
}

fn queue_mode_from_env() -> CocoResult<QueueMode> {
    let raw = env_optional("COCO_QUEUE_MODE").unwrap_or_else(|| DEFAULT_QUEUE_MODE.to_string());
    match raw.to_ascii_lowercase().as_str() {
        "postgres" | "pg" => Ok(QueueMode::Postgres),
        "redis" => Ok(QueueMode::Redis),
        other => Err(CocoError::user(format!(
            "unsupported COCO_QUEUE_MODE value: {other}"
        ))),
    }
}

fn tls_mode_from_env() -> CocoResult<TlsMode> {
    let raw = env_optional("COCO_TLS_MODE").unwrap_or_else(|| "tls".to_string());
    match raw.to_ascii_lowercase().as_str() {
        "tls" => Ok(TlsMode::Tls),
        "proxy" => Ok(TlsMode::Proxy),
        other => Err(CocoError::user(format!(
            "unsupported COCO_TLS_MODE value: {other}"
        ))),
    }
}

fn vector_backend_kind_from_env() -> CocoResult<VectorBackendKind> {
    let raw = env_optional("COCO_VECTOR_BACKEND").unwrap_or_else(|| "pgvector".to_string());
    match raw.to_ascii_lowercase().as_str() {
        "pgvector" | "pg" => Ok(VectorBackendKind::PgVector),
        "qdrant" => Ok(VectorBackendKind::Qdrant),
        other => Err(CocoError::user(format!(
            "unsupported COCO_VECTOR_BACKEND value: {other}"
        ))),
    }
}

fn vector_backend_config_from_env(
    kind: VectorBackendKind,
) -> CocoResult<Option<VectorBackendConfig>> {
    if kind != VectorBackendKind::Qdrant {
        return Ok(None);
    }
    let url = env_optional("COCO_VECTOR_DB_URL")
        .ok_or_else(|| CocoError::user("COCO_VECTOR_DB_URL required for qdrant backend"))?;
    let api_key = env_optional("COCO_VECTOR_DB_API_KEY");
    let collection_prefix = env_optional("COCO_VECTOR_DB_COLLECTION_PREFIX").ok_or_else(|| {
        CocoError::user("COCO_VECTOR_DB_COLLECTION_PREFIX required for qdrant backend")
    })?;
    Ok(Some(VectorBackendConfig {
        kind,
        url: Some(url),
        api_key,
        collection_prefix: Some(collection_prefix),
    }))
}

fn env_optional(key: &str) -> Option<String> {
    std::env::var(key).ok().filter(|value| !value.is_empty())
}

fn env_required(key: &str) -> CocoResult<String> {
    env_optional(key).ok_or_else(|| CocoError::user(format!("{key} must be set")))
}

fn env_u16(key: &str, default: u16) -> CocoResult<u16> {
    match env_optional(key) {
        Some(value) => value
            .parse::<u16>()
            .map_err(|_| CocoError::user(format!("{key} must be a valid number"))),
        None => Ok(default),
    }
}

fn env_u32(key: &str, default: u32) -> CocoResult<u32> {
    match env_optional(key) {
        Some(value) => value
            .parse::<u32>()
            .map_err(|_| CocoError::user(format!("{key} must be a valid number"))),
        None => Ok(default),
    }
}

fn env_u32_optional(key: &str) -> CocoResult<Option<u32>> {
    match env_optional(key) {
        Some(value) => {
            let parsed = value
                .parse::<u32>()
                .map_err(|_| CocoError::user(format!("{key} must be a valid number")))?;
            if parsed == 0 {
                return Err(CocoError::user(format!("{key} must be greater than zero")));
            }
            Ok(Some(parsed))
        }
        None => Ok(None),
    }
}

fn env_u64_optional(key: &str) -> CocoResult<Option<u64>> {
    match env_optional(key) {
        Some(value) => {
            let parsed = value
                .parse::<u64>()
                .map_err(|_| CocoError::user(format!("{key} must be a valid number")))?;
            if parsed == 0 {
                return Err(CocoError::user(format!("{key} must be greater than zero")));
            }
            Ok(Some(parsed))
        }
        None => Ok(None),
    }
}

fn env_i64_optional(key: &str) -> CocoResult<Option<i64>> {
    match env_optional(key) {
        Some(value) => {
            let parsed = value
                .parse::<i64>()
                .map_err(|_| CocoError::user(format!("{key} must be a valid number")))?;
            if parsed <= 0 {
                return Err(CocoError::user(format!("{key} must be greater than zero")));
            }
            Ok(Some(parsed))
        }
        None => Ok(None),
    }
}

fn pg_pool_config_from_env(prefix: &str) -> CocoResult<PgPoolConfig> {
    let max_connections = env_u32_optional(&format!("{prefix}_PG_MAX_CONNECTIONS"))?;
    let min_connections = env_u32_optional(&format!("{prefix}_PG_MIN_CONNECTIONS"))?;
    if let (Some(min), Some(max)) = (min_connections, max_connections) {
        if min > max {
            return Err(CocoError::user(format!(
                "{prefix}_PG_MIN_CONNECTIONS must not exceed {prefix}_PG_MAX_CONNECTIONS"
            )));
        }
    }
    let connect_timeout = env_u64_optional(&format!("{prefix}_PG_CONNECT_TIMEOUT_SECS"))?
        .map(Duration::from_secs);
    Ok(PgPoolConfig {
        max_connections,
        min_connections,
        connect_timeout,
    })
}

fn resolve_database_url() -> CocoResult<String> {
    let primary = env_optional("COCO_META_DB");
    let legacy = env_optional("COCO_DB_URL");
    match (primary, legacy) {
        (Some(primary), Some(legacy)) => {
            if primary != legacy {
                warn!("COCO_META_DB and COCO_DB_URL are both set; using COCO_META_DB");
            }
            Ok(primary)
        }
        (Some(primary), None) => Ok(primary),
        (None, Some(legacy)) => Ok(legacy),
        (None, None) => Err(CocoError::user(
            "COCO_META_DB must be set (COCO_DB_URL is supported as a legacy alias)",
        )),
    }
}

fn warn_if_mode_set() {
    let Some(mode) = env_optional("COCO_MODE") else {
        return;
    };
    if !mode.eq_ignore_ascii_case("server") {
        warn!("COCO_MODE is ignored by coco-server (expected 'server' for compatibility)");
    } else {
        warn!("COCO_MODE is ignored by coco-server (compatibility only)");
    }
}
