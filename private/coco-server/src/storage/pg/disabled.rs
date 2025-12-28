use std::collections::{HashMap, HashSet};
use std::time::Duration;

use coco_protocol::{
    Chunk, ChunkId, CocoError, CocoResult, DocumentId, SearchHit, SearchIntent, StorageBackend,
    VectorMetric, VectorRecord, VectorStore,
};

const DEFAULT_RRF_K: u32 = 60;
const DEFAULT_MAX_CONNECTIONS: u32 = 16;
const DEFAULT_MIN_CONNECTIONS: u32 = 1;
const DEFAULT_CONNECT_TIMEOUT_SECS: u64 = 10;

/// Placeholder when server-storage feature is disabled.
#[derive(Debug, Clone)]
pub struct PgBackend;

/// Placeholder executor when server-storage feature is disabled.
#[derive(Debug, Clone)]
pub struct PgExecutor;

/// Placeholder config when server-storage feature is disabled.
#[derive(Debug, Clone)]
pub struct PgBackendConfig {
    /// PostgreSQL connection URL.
    pub database_url: String,
    /// Tenant scoping.
    pub tenant: TenantContext,
    /// Vector distance metric.
    pub metric: PgVectorMetric,
    /// RRF constant for hybrid ranking.
    pub rrf_k: u32,
    /// Maximum pooled connections.
    pub max_connections: u32,
    /// Minimum pooled connections.
    pub min_connections: u32,
    /// Connect timeout.
    pub connect_timeout: Duration,
    /// Whether to attempt creating pgvector extension.
    pub ensure_pgvector: bool,
}

/// Placeholder metric when server-storage feature is disabled.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum PgVectorMetric {
    /// L2 distance.
    L2,
    /// Cosine distance.
    Cosine,
    /// Negative inner product.
    Dot,
}

impl From<VectorMetric> for PgVectorMetric {
    fn from(metric: VectorMetric) -> Self {
        match metric {
            VectorMetric::Cosine => PgVectorMetric::Cosine,
            VectorMetric::Dot => PgVectorMetric::Dot,
            VectorMetric::L2 => PgVectorMetric::L2,
        }
    }
}

/// Placeholder tenant context when server-storage feature is disabled.
#[derive(Debug, Clone)]
pub struct TenantContext {
    /// Organization identifier.
    pub org_id: String,
    /// User identifier.
    pub user_id: String,
    /// Project identifier.
    pub project_id: String,
    /// Optional project version identifier.
    pub version_id: Option<String>,
    /// Optional indexing configuration identifier.
    pub config_id: Option<String>,
}

impl PgBackend {
    /// Returns an error because server-storage is disabled.
    pub async fn connect(_: PgBackendConfig) -> CocoResult<Self> {
        Err(CocoError::storage(
            "server-storage feature disabled for PgBackend",
        ))
    }

    /// Returns the same backend because server-storage is disabled.
    pub fn with_version(&self, _: Option<String>) -> Self {
        Self
    }

    /// Returns the same backend because server-storage is disabled.
    pub fn with_config(&self, _: Option<String>) -> Self {
        Self
    }

    /// Returns a default RRF constant when server-storage is disabled.
    pub fn rrf_k(&self) -> u32 {
        DEFAULT_RRF_K
    }

    /// Returns no version when server-storage is disabled.
    pub async fn pgvector_version(&self) -> CocoResult<Option<String>> {
        Ok(None)
    }

    /// Returns an error because server-storage is disabled.
    pub async fn get_chunks_by_ids(&self, _ids: &[String]) -> CocoResult<HashMap<String, Chunk>> {
        Err(CocoError::storage(
            "server-storage feature disabled for PgBackend",
        ))
    }

    /// Returns an error because server-storage is disabled.
    pub async fn filter_chunk_ids(&self, _ids: &[String]) -> CocoResult<HashSet<String>> {
        Err(CocoError::storage(
            "server-storage feature disabled for PgBackend",
        ))
    }
}

impl PgExecutor {
    /// Returns a stub executor when server-storage is disabled.
    pub fn new(_: &PgBackend) -> Self {
        Self
    }

    /// Returns an error because server-storage is disabled.
    pub async fn search(
        &self,
        _intent: coco_protocol::SearchIntent,
    ) -> CocoResult<Vec<coco_protocol::SearchHit>> {
        Err(CocoError::storage(
            "server-storage feature disabled for PgBackend",
        ))
    }
}

impl PgBackendConfig {
    /// Creates a placeholder configuration when server-storage is disabled.
    pub fn new(database_url: String, org_id: String, user_id: String, project_id: String) -> Self {
        Self {
            database_url,
            tenant: TenantContext {
                org_id,
                user_id,
                project_id,
                version_id: None,
                config_id: None,
            },
            metric: PgVectorMetric::L2,
            rrf_k: DEFAULT_RRF_K,
            max_connections: DEFAULT_MAX_CONNECTIONS,
            min_connections: DEFAULT_MIN_CONNECTIONS,
            connect_timeout: Duration::from_secs(DEFAULT_CONNECT_TIMEOUT_SECS),
            ensure_pgvector: false,
        }
    }
}

impl StorageBackend for PgBackend {
    type UpsertChunksFuture<'a> = std::future::Ready<CocoResult<()>>;
    type SearchSimilarFuture<'a> = std::future::Ready<CocoResult<Vec<coco_protocol::SearchHit>>>;
    type DeleteByDocFuture<'a> = std::future::Ready<CocoResult<()>>;
    type GetChunkFuture<'a> = std::future::Ready<CocoResult<Option<Chunk>>>;

    fn upsert_chunks<'a>(&'a self, _chunks: &'a [Chunk]) -> Self::UpsertChunksFuture<'a> {
        std::future::ready(Err(CocoError::storage(
            "server-storage feature disabled for PgBackend",
        )))
    }

    fn search_similar<'a>(
        &'a self,
        _intent: coco_protocol::SearchIntent,
    ) -> Self::SearchSimilarFuture<'a> {
        std::future::ready(Err(CocoError::storage(
            "server-storage feature disabled for PgBackend",
        )))
    }

    fn delete_by_doc<'a>(
        &'a self,
        _doc_id: coco_protocol::DocumentId,
    ) -> Self::DeleteByDocFuture<'a> {
        std::future::ready(Err(CocoError::storage(
            "server-storage feature disabled for PgBackend",
        )))
    }

    fn get_chunk<'a>(&'a self, _chunk_id: coco_protocol::ChunkId) -> Self::GetChunkFuture<'a> {
        std::future::ready(Err(CocoError::storage(
            "server-storage feature disabled for PgBackend",
        )))
    }
}

impl VectorStore for PgBackend {
    type UpsertVectorsFuture<'a> = std::future::Ready<CocoResult<()>>;
    type SearchVectorsFuture<'a> = std::future::Ready<CocoResult<Vec<SearchHit>>>;
    type DeleteVectorsByDocFuture<'a> = std::future::Ready<CocoResult<()>>;
    type GetVectorFuture<'a> = std::future::Ready<CocoResult<Option<VectorRecord>>>;

    fn upsert_vectors<'a>(&'a self, _records: &'a [VectorRecord]) -> Self::UpsertVectorsFuture<'a> {
        std::future::ready(Err(CocoError::storage(
            "server-storage feature disabled for PgBackend",
        )))
    }

    fn search_vectors<'a>(&'a self, _intent: SearchIntent) -> Self::SearchVectorsFuture<'a> {
        std::future::ready(Err(CocoError::storage(
            "server-storage feature disabled for PgBackend",
        )))
    }

    fn delete_vectors_by_doc<'a>(
        &'a self,
        _doc_id: DocumentId,
    ) -> Self::DeleteVectorsByDocFuture<'a> {
        std::future::ready(Err(CocoError::storage(
            "server-storage feature disabled for PgBackend",
        )))
    }

    fn get_vector<'a>(&'a self, _chunk_id: ChunkId) -> Self::GetVectorFuture<'a> {
        std::future::ready(Err(CocoError::storage(
            "server-storage feature disabled for PgBackend",
        )))
    }
}
