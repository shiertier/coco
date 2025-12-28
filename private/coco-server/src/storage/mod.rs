//! Server storage adapters for metadata and vectors.

/// PostgreSQL vector storage backend.
pub mod pg;
/// Vector backend selection and aggregation.
pub mod vector;

/// PostgreSQL metadata store.
#[cfg(feature = "server-storage")]
pub mod meta;

#[cfg(not(feature = "server-storage"))]
pub mod meta {
    use coco_protocol::{
        ChunkingStrategy, CocoError, CocoResult, EmbeddingConfig, VectorBackendConfig,
        VectorIndexParams, VectorMetric,
    };

    /// Ingest job status: queued.
    pub const JOB_STATUS_QUEUED: &str = "queued";
    /// Ingest job status: running.
    pub const JOB_STATUS_RUNNING: &str = "running";
    /// Ingest job status: failed.
    pub const JOB_STATUS_FAILED: &str = "failed";
    /// Ingest job status: completed.
    pub const JOB_STATUS_COMPLETED: &str = "completed";
    /// Default indexing configuration identifier.
    pub const DEFAULT_CONFIG_ID: &str = "default";
    /// Server-side fixed embedding dimension for pgvector.
    pub const DEFAULT_EMBEDDING_DIM: u32 = 1536;

    /// Placeholder metadata store when server-storage is disabled.
    #[derive(Debug, Clone)]
    pub struct ServerMetaStore;

    impl ServerMetaStore {
        /// Returns an error because server-storage is disabled.
        pub async fn connect(_: &str) -> CocoResult<Self> {
            Err(CocoError::storage(
                "server-storage feature disabled for ServerMetaStore",
            ))
        }

        /// Returns an error because server-storage is disabled.
        pub async fn create_indexing_config(
            &self,
            _: NewIndexingConfig,
        ) -> CocoResult<IndexingConfigRecord> {
            Err(CocoError::storage(
                "server-storage feature disabled for ServerMetaStore",
            ))
        }

        /// Returns an error because server-storage is disabled.
        pub async fn ensure_default_indexing_config(
            &self,
            _: NewIndexingConfig,
        ) -> CocoResult<IndexingConfigRecord> {
            Err(CocoError::storage(
                "server-storage feature disabled for ServerMetaStore",
            ))
        }

        /// Returns an error because server-storage is disabled.
        pub async fn get_indexing_config(
            &self,
            _: &str,
            _: &str,
        ) -> CocoResult<Option<IndexingConfigRecord>> {
            Err(CocoError::storage(
                "server-storage feature disabled for ServerMetaStore",
            ))
        }

        /// Returns an error because server-storage is disabled.
        pub async fn list_indexing_configs(
            &self,
            _: &str,
        ) -> CocoResult<Vec<IndexingConfigRecord>> {
            Err(CocoError::storage(
                "server-storage feature disabled for ServerMetaStore",
            ))
        }

        /// Returns an error because server-storage is disabled.
        pub async fn ensure_indexing_config_exists(&self, _: &str, _: &str) -> CocoResult<()> {
            Err(CocoError::storage(
                "server-storage feature disabled for ServerMetaStore",
            ))
        }

        /// Returns an error because server-storage is disabled.
        pub async fn update_indexing_config(
            &self,
            _: NewIndexingConfig,
        ) -> CocoResult<IndexingConfigRecord> {
            Err(CocoError::storage(
                "server-storage feature disabled for ServerMetaStore",
            ))
        }

        /// Returns an error because server-storage is disabled.
        pub async fn get_organization(&self, _: &str) -> CocoResult<Option<OrganizationRecord>> {
            Err(CocoError::storage(
                "server-storage feature disabled for ServerMetaStore",
            ))
        }

        /// Returns an error because server-storage is disabled.
        pub async fn get_project(
            &self,
            _: &str,
            _: &str,
            _: &str,
        ) -> CocoResult<Option<ProjectRecord>> {
            Err(CocoError::storage(
                "server-storage feature disabled for ServerMetaStore",
            ))
        }

        /// Returns an error because server-storage is disabled.
        pub async fn create_organization(
            &self,
            _: NewOrganization,
        ) -> CocoResult<OrganizationRecord> {
            Err(CocoError::storage(
                "server-storage feature disabled for ServerMetaStore",
            ))
        }

        /// Returns an error because server-storage is disabled.
        pub async fn create_project(&self, _: NewProject) -> CocoResult<ProjectRecord> {
            Err(CocoError::storage(
                "server-storage feature disabled for ServerMetaStore",
            ))
        }

        /// Returns an error because server-storage is disabled.
        pub async fn update_project(
            &self,
            _: &str,
            _: &str,
            _: &str,
            _: ProjectUpdate,
        ) -> CocoResult<Option<ProjectRecord>> {
            Err(CocoError::storage(
                "server-storage feature disabled for ServerMetaStore",
            ))
        }

        /// Returns an error because server-storage is disabled.
        pub async fn create_document(&self, _: NewDocument) -> CocoResult<DocumentRecord> {
            Err(CocoError::storage(
                "server-storage feature disabled for ServerMetaStore",
            ))
        }

        /// Returns an error because server-storage is disabled.
        pub async fn upsert_document(&self, _: NewDocument) -> CocoResult<DocumentRecord> {
            Err(CocoError::storage(
                "server-storage feature disabled for ServerMetaStore",
            ))
        }

        /// Returns an error because server-storage is disabled.
        pub async fn create_project_version(
            &self,
            _: &str,
            _: &str,
            _: &str,
            _: &str,
        ) -> CocoResult<ProjectVersionRecord> {
            Err(CocoError::storage(
                "server-storage feature disabled for ServerMetaStore",
            ))
        }

        /// Returns an error because server-storage is disabled.
        pub async fn activate_project_version(
            &self,
            _: &str,
            _: &str,
            _: &str,
            _: &str,
        ) -> CocoResult<()> {
            Err(CocoError::storage(
                "server-storage feature disabled for ServerMetaStore",
            ))
        }

        /// Returns an error because server-storage is disabled.
        pub async fn gc_project_versions(
            &self,
            _: &str,
            _: &str,
            _: &str,
            _: usize,
        ) -> CocoResult<usize> {
            Err(CocoError::storage(
                "server-storage feature disabled for ServerMetaStore",
            ))
        }

        /// Returns an error because server-storage is disabled.
        pub async fn count_documents_by_org(&self, _: &str, _: &str) -> CocoResult<i64> {
            Err(CocoError::storage(
                "server-storage feature disabled for ServerMetaStore",
            ))
        }

        /// Returns an error because server-storage is disabled.
        pub async fn count_chunks_by_org(&self, _: &str, _: &str) -> CocoResult<i64> {
            Err(CocoError::storage(
                "server-storage feature disabled for ServerMetaStore",
            ))
        }

        /// Returns an error because server-storage is disabled.
        pub async fn sum_chunk_bytes_by_org(&self, _: &str, _: &str) -> CocoResult<i64> {
            Err(CocoError::storage(
                "server-storage feature disabled for ServerMetaStore",
            ))
        }

        /// Returns an error because server-storage is disabled.
        pub async fn get_embedding_calls_for_day(
            &self,
            _: &str,
            _: &str,
            _: chrono::NaiveDate,
        ) -> CocoResult<i64> {
            Err(CocoError::storage(
                "server-storage feature disabled for ServerMetaStore",
            ))
        }

        /// Returns an error because server-storage is disabled.
        pub async fn increment_embedding_calls(
            &self,
            _: &str,
            _: &str,
            _: chrono::NaiveDate,
            _: i64,
        ) -> CocoResult<()> {
            Err(CocoError::storage(
                "server-storage feature disabled for ServerMetaStore",
            ))
        }

        /// Returns an error because server-storage is disabled.
        pub async fn get_worker_status(&self, _: &str) -> CocoResult<Option<WorkerStatusRecord>> {
            Err(CocoError::storage(
                "server-storage feature disabled for ServerMetaStore",
            ))
        }

        /// Returns an error because server-storage is disabled.
        pub async fn upsert_worker_status(&self, _: &str, _: &str) -> CocoResult<()> {
            Err(CocoError::storage(
                "server-storage feature disabled for ServerMetaStore",
            ))
        }

        /// Returns an error because server-storage is disabled.
        pub async fn create_ingest_job(&self, _: NewIngestJob) -> CocoResult<IngestJobRecord> {
            Err(CocoError::storage(
                "server-storage feature disabled for ServerMetaStore",
            ))
        }

        /// Returns an error because server-storage is disabled.
        pub async fn get_ingest_job(&self, _: &str) -> CocoResult<Option<IngestJobRecord>> {
            Err(CocoError::storage(
                "server-storage feature disabled for ServerMetaStore",
            ))
        }

        /// Returns an error because server-storage is disabled.
        pub async fn claim_ingest_job_by_id(&self, _: &str) -> CocoResult<IngestJobRecord> {
            Err(CocoError::storage(
                "server-storage feature disabled for ServerMetaStore",
            ))
        }

        /// Returns an error because server-storage is disabled.
        pub async fn claim_next_ingest_job(&self) -> CocoResult<Option<IngestJobRecord>> {
            Err(CocoError::storage(
                "server-storage feature disabled for ServerMetaStore",
            ))
        }

        /// Returns an error because server-storage is disabled.
        pub async fn update_ingest_job_status(
            &self,
            _: &str,
            _: &str,
            _: Option<String>,
            _: Option<String>,
        ) -> CocoResult<()> {
            Err(CocoError::storage(
                "server-storage feature disabled for ServerMetaStore",
            ))
        }
    }

    /// Placeholder organization record.
    #[derive(Debug, Clone)]
    pub struct OrganizationRecord {
        /// Organization identifier.
        pub id: String,
        /// Organization name.
        pub name: String,
        /// Max documents allowed.
        pub max_documents: Option<i64>,
        /// Max chunks allowed.
        pub max_chunks: Option<i64>,
        /// Max storage bytes allowed.
        pub max_storage_bytes: Option<i64>,
        /// Max embedding calls per day.
        pub max_embeddings_per_day: Option<i64>,
    }

    /// Placeholder project record.
    #[derive(Debug, Clone)]
    pub struct ProjectRecord {
        /// Project identifier.
        pub id: String,
        /// Organization identifier.
        pub org_id: String,
        /// User identifier.
        pub user_id: String,
        /// Project name.
        pub name: String,
        /// Active version identifier.
        pub active_version_id: Option<String>,
        /// Active indexing configuration identifier.
        pub active_config_id: String,
    }

    /// Placeholder project update payload.
    #[derive(Debug, Clone, Default)]
    pub struct ProjectUpdate {
        /// Updated name.
        pub name: Option<String>,
        /// Updated active version identifier.
        pub active_version_id: Option<Option<String>>,
        /// Updated active config identifier.
        pub active_config_id: Option<String>,
    }

    /// Placeholder indexing configuration record.
    #[derive(Debug, Clone)]
    pub struct IndexingConfigRecord {
        /// Organization identifier.
        pub org_id: String,
        /// Configuration identifier.
        pub config_id: String,
        /// Chunking strategy definition.
        pub chunking: ChunkingStrategy,
        /// Embedding model definition.
        pub embedding: EmbeddingConfig,
        /// Optional vector backend selection.
        pub vector_backend: Option<VectorBackendConfig>,
        /// Vector similarity metric.
        pub vector_metric: VectorMetric,
        /// Optional backend-specific index parameters.
        pub index_params: Option<VectorIndexParams>,
        /// Creation timestamp.
        pub created_at: chrono::DateTime<chrono::Utc>,
    }

    /// Placeholder document record.
    #[derive(Debug, Clone)]
    pub struct DocumentRecord {
        /// Document identifier.
        pub id: String,
        /// Indexing configuration identifier.
        pub config_id: String,
    }

    /// Placeholder worker status record.
    #[derive(Debug, Clone)]
    pub struct WorkerStatusRecord {
        /// Worker identifier.
        pub id: String,
        /// Worker version signature.
        pub version: String,
        /// Last heartbeat time.
        pub updated_at: chrono::DateTime<chrono::Utc>,
    }

    /// Placeholder project version record.
    #[derive(Debug, Clone)]
    pub struct ProjectVersionRecord {
        /// Version identifier.
        pub id: String,
        /// Indexing configuration identifier.
        pub active_config_id: String,
    }

    /// Placeholder ingest job record.
    #[derive(Debug, Clone)]
    pub struct IngestJobRecord {
        /// Job identifier.
        pub id: String,
        /// Organization identifier.
        pub org_id: String,
        /// User identifier.
        pub user_id: String,
        /// Project identifier.
        pub project_id: String,
        /// Status string.
        pub status: String,
        /// JSON payload.
        pub payload: String,
        /// Attempt counter.
        pub attempts: i32,
        /// Error details if any.
        pub error: Option<String>,
        /// Version identifier.
        pub version_id: Option<String>,
        /// Creation timestamp.
        pub created_at: chrono::DateTime<chrono::Utc>,
        /// Update timestamp.
        pub updated_at: chrono::DateTime<chrono::Utc>,
    }

    /// Placeholder new organization payload.
    #[derive(Debug, Clone)]
    pub struct NewOrganization {
        /// Organization identifier.
        pub id: String,
        /// Organization name.
        pub name: String,
        /// Creation timestamp.
        pub created_at: chrono::DateTime<chrono::Utc>,
        /// Max documents allowed.
        pub max_documents: Option<i64>,
        /// Max chunks allowed.
        pub max_chunks: Option<i64>,
        /// Max storage bytes allowed.
        pub max_storage_bytes: Option<i64>,
        /// Max embedding calls per day.
        pub max_embeddings_per_day: Option<i64>,
    }

    /// Placeholder new project payload.
    #[derive(Debug, Clone)]
    pub struct NewProject {
        /// Project identifier.
        pub id: String,
        /// Organization identifier.
        pub org_id: String,
        /// User identifier.
        pub user_id: String,
        /// Project name.
        pub name: String,
        /// Creation timestamp.
        pub created_at: chrono::DateTime<chrono::Utc>,
        /// Active version identifier.
        pub active_version_id: Option<String>,
        /// Active indexing configuration identifier.
        pub active_config_id: String,
    }

    /// Placeholder new indexing configuration payload.
    #[derive(Debug, Clone)]
    pub struct NewIndexingConfig {
        /// Organization identifier.
        pub org_id: String,
        /// Configuration identifier.
        pub config_id: String,
        /// Chunking strategy definition.
        pub chunking: ChunkingStrategy,
        /// Embedding model definition.
        pub embedding: EmbeddingConfig,
        /// Optional vector backend selection.
        pub vector_backend: Option<VectorBackendConfig>,
        /// Vector similarity metric.
        pub vector_metric: VectorMetric,
        /// Optional backend-specific index parameters.
        pub index_params: Option<VectorIndexParams>,
        /// Creation timestamp.
        pub created_at: chrono::DateTime<chrono::Utc>,
    }

    /// Placeholder new document payload.
    #[derive(Debug, Clone)]
    pub struct NewDocument {
        /// Document identifier.
        pub id: String,
        /// Organization identifier.
        pub org_id: String,
        /// User identifier.
        pub user_id: String,
        /// Project identifier.
        pub project_id: String,
        /// Indexing configuration identifier.
        pub config_id: String,
        /// Source reference.
        pub source_ref: String,
        /// Optional title.
        pub title: Option<String>,
        /// Content hash.
        pub content_hash: String,
        /// Indexed timestamp.
        pub indexed_at: chrono::DateTime<chrono::Utc>,
        /// Optional quality score.
        pub quality_score: Option<f32>,
        /// Whether the document has been verified.
        pub verified: bool,
    }

    /// Placeholder new ingest job payload.
    #[derive(Debug, Clone)]
    pub struct NewIngestJob {
        /// Job identifier.
        pub id: String,
        /// Organization identifier.
        pub org_id: String,
        /// User identifier.
        pub user_id: String,
        /// Project identifier.
        pub project_id: String,
        /// Payload.
        pub payload: String,
        /// Creation timestamp.
        pub created_at: chrono::DateTime<chrono::Utc>,
        /// Update timestamp.
        pub updated_at: chrono::DateTime<chrono::Utc>,
    }
}
