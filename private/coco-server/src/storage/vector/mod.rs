//! Vector backend selection and hybrid search aggregation.

use std::num::NonZeroU32;

use coco_protocol::{
    CocoError, CocoResult, RetrievalMode, SearchHit, SearchIntent, SearchQuery,
    VectorBackendConfig, VectorBackendKind, VectorIndexParams, VectorMetric, VectorStore,
};
use tracing::info;

use super::pg::{PgBackend, PgBackendConfig, PgExecutor};

mod qdrant;
mod rrf;

#[cfg(test)]
mod tests;

/// Server vector backend configuration.
#[derive(Debug, Clone)]
pub struct ServerVectorBackendConfig {
    /// Selected vector backend kind.
    pub kind: VectorBackendKind,
    /// Postgres backend configuration for pgvector or FTS.
    pub pg: PgBackendConfig,
    /// Optional vector DB backend configuration.
    pub vector_db: Option<VectorBackendConfig>,
    /// Vector similarity metric for the backend.
    pub vector_metric: VectorMetric,
    /// Optional backend-specific index params.
    pub index_params: Option<VectorIndexParams>,
    /// Whether to use collection alias for active versions.
    pub use_collection_alias: bool,
}

impl ServerVectorBackendConfig {
    /// Builds a config with the selected backend and Postgres settings.
    pub fn new(kind: VectorBackendKind, pg: PgBackendConfig, vector_metric: VectorMetric) -> Self {
        Self {
            kind,
            pg,
            vector_db: None,
            vector_metric,
            index_params: None,
            use_collection_alias: false,
        }
    }
}

/// Server vector backend selection.
#[derive(Debug, Clone)]
pub enum ServerVectorBackend {
    /// Use pgvector for vector operations (and Postgres for FTS).
    PgVector(PgBackend),
    /// Use a vector database for vector operations and Postgres for FTS.
    Qdrant {
        vector: Box<QdrantStore>,
        fts: PgBackend,
    },
}

impl ServerVectorBackend {
    /// Connects to the selected backend and logs the backend version if available.
    pub async fn connect(config: ServerVectorBackendConfig) -> CocoResult<Self> {
        match config.kind {
            VectorBackendKind::PgVector => {
                let backend = PgBackend::connect(config.pg).await?;
                log_pgvector(&backend).await?;
                Ok(Self::PgVector(backend))
            }
            VectorBackendKind::Qdrant => {
                let vector_config = config
                    .vector_db
                    .ok_or_else(|| CocoError::user("qdrant backend configuration is required"))?;
                let tenant = config.pg.tenant.clone();
                let fts_backend = PgBackend::connect(config.pg).await?;
                let vector_backend = QdrantStore::connect(
                    vector_config,
                    tenant,
                    config.vector_metric,
                    config.index_params.clone(),
                    config.use_collection_alias,
                    fts_backend.clone(),
                )
                .await?;
                let endpoint = vector_backend
                    .config()
                    .url
                    .as_deref()
                    .unwrap_or("unknown");
                info!("vector backend: qdrant endpoint={endpoint} version=unknown");
                Ok(Self::Qdrant {
                    vector: Box::new(vector_backend),
                    fts: fts_backend,
                })
            }
        }
    }

    /// Returns the selected backend kind.
    pub fn kind(&self) -> VectorBackendKind {
        match self {
            Self::PgVector(_) => VectorBackendKind::PgVector,
            Self::Qdrant { .. } => VectorBackendKind::Qdrant,
        }
    }
}

impl VectorStore for ServerVectorBackend {
    fn upsert_vectors(
        &self,
        records: &[coco_protocol::VectorRecord],
    ) -> impl std::future::Future<Output = CocoResult<()>> + Send {
        let backend = self.clone();
        async move {
            match backend {
                Self::PgVector(pg) => pg.upsert_vectors(records).await,
                Self::Qdrant { vector, .. } => vector.upsert_vectors(records).await,
            }
        }
    }

    fn search_vectors(
        &self,
        intent: SearchIntent,
    ) -> impl std::future::Future<Output = CocoResult<Vec<coco_protocol::SearchHit>>> + Send {
        let backend = self.clone();
        async move {
            match backend {
                Self::PgVector(pg) => pg.search_vectors(intent).await,
                Self::Qdrant { vector, fts } => match intent.retrieval_mode() {
                    RetrievalMode::Vector => {
                        let vector_results = vector.search_vectors(intent).await?;
                        filter_vector_results(&fts, vector_results).await
                    }
                    RetrievalMode::Fts => PgExecutor::new(&fts).search(intent).await,
                    RetrievalMode::Hybrid => {
                        let top_k = intent.top_k().get();
                        let candidate_k = top_k.saturating_mul(2).max(1);
                        let candidate_top_k = NonZeroU32::new(candidate_k)
                            .ok_or_else(|| CocoError::user("top_k must be greater than zero"))?;
                        let filters = intent.filters().to_vec();
                        let config_id = intent.indexing_config_id().map(str::to_string);
                        let reranker = intent.reranker().cloned();
                        let hybrid_alpha = intent.hybrid_alpha();
                        let embedding = intent
                            .query_embedding()
                            .ok_or_else(|| {
                                CocoError::user("query embedding required for hybrid search")
                            })?
                            .to_vec();
                        let query_text = intent
                            .query_text()
                            .ok_or_else(|| {
                                CocoError::user("query text required for hybrid search")
                            })?
                            .to_string();

                        let vector_intent = SearchIntent::new(
                            SearchQuery::Vector { embedding },
                            config_id.clone(),
                            candidate_top_k,
                            hybrid_alpha,
                            filters.clone(),
                            reranker.clone(),
                        )?;
                        let vector_results = vector.search_vectors(vector_intent).await?;
                        let vector_results = filter_vector_results(&fts, vector_results).await?;

                        let fts_intent = SearchIntent::new(
                            SearchQuery::Fts { text: query_text },
                            config_id,
                            candidate_top_k,
                            hybrid_alpha,
                            filters,
                            reranker,
                        )?;
                        let fts_results = PgExecutor::new(&fts).search(fts_intent).await?;

                        Ok(rrf::merge_rrf(
                            vector_results,
                            fts_results,
                            fts.rrf_k(),
                            top_k as usize,
                        ))
                    }
                },
            }
        }
    }

    fn delete_vectors_by_doc(
        &self,
        doc_id: coco_protocol::DocumentId,
    ) -> impl std::future::Future<Output = CocoResult<()>> + Send {
        let backend = self.clone();
        async move {
            match backend {
                Self::PgVector(pg) => pg.delete_vectors_by_doc(doc_id).await,
                Self::Qdrant { vector, .. } => vector.delete_vectors_by_doc(doc_id).await,
            }
        }
    }

    fn get_vector(
        &self,
        chunk_id: coco_protocol::ChunkId,
    ) -> impl std::future::Future<Output = CocoResult<Option<coco_protocol::VectorRecord>>> + Send
    {
        let backend = self.clone();
        async move {
            match backend {
                Self::PgVector(pg) => pg.get_vector(chunk_id).await,
                Self::Qdrant { vector, .. } => vector.get_vector(chunk_id).await,
            }
        }
    }
}

async fn log_pgvector(backend: &PgBackend) -> CocoResult<()> {
    let version = backend.pgvector_version().await?.unwrap_or("unknown".to_string());
    info!("vector backend: pgvector version={version}");
    Ok(())
}

async fn filter_vector_results(
    fts_backend: &PgBackend,
    vector_results: Vec<SearchHit>,
) -> CocoResult<Vec<SearchHit>> {
    if vector_results.is_empty() {
        return Ok(vector_results);
    }
    let ids: Vec<String> = vector_results
        .iter()
        .map(|hit| hit.chunk.id.to_string())
        .collect();
    let allowed = fts_backend.filter_chunk_ids(&ids).await?;
    Ok(rrf::retain_allowed(vector_results, &allowed))
}

pub(crate) use qdrant::QdrantStore;
