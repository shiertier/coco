use coco_protocol::{CocoError, CocoResult, RetrievalMode, SearchHit, SearchIntent};
use sea_orm::{ConnectionTrait, DatabaseBackend, Statement};

use super::backend::PgBackend;
use super::helpers::{map_storage_err, validate_tenant, COL_DISTANCE, COL_SCORE};
use super::queries::{
    build_fts_query, build_hybrid_query, build_vector_query, collect_results, HybridQueryArgs,
    ScoreMode,
};

/// Query executor that maps SearchIntent into Postgres queries.
#[derive(Debug, Clone)]
pub struct PgExecutor {
    backend: PgBackend,
}

impl PgExecutor {
    /// Creates a new executor from an existing backend.
    pub fn new(backend: &PgBackend) -> Self {
        Self {
            backend: backend.clone(),
        }
    }

    /// Executes the search intent using pgvector/FTS queries.
    pub async fn search(&self, intent: SearchIntent) -> CocoResult<Vec<SearchHit>> {
        let db = self.backend.connection().clone();
        let tenant = self.backend.tenant.clone();
        let backend = self.backend.clone();
        validate_tenant(&tenant)?;
        let top_k = intent.top_k().get() as usize;
        let version_id = backend.resolve_version_id().await?;
        let config_id = backend
            .resolve_config_id(intent.indexing_config_id())
            .await?;
        match intent.retrieval_mode() {
            RetrievalMode::Vector => {
                let embedding = intent
                    .query_embedding()
                    .ok_or_else(|| CocoError::user("query embedding required for vector search"))?;
                let (sql, values) = build_vector_query(
                    embedding,
                    intent.filters(),
                    &tenant,
                    &version_id,
                    &config_id,
                    top_k,
                    backend.vector_operator(),
                )?;
                let rows = db
                    .query_all_raw(Statement::from_sql_and_values(
                        DatabaseBackend::Postgres,
                        sql,
                        values,
                    ))
                    .await
                    .map_err(map_storage_err)?;
                collect_results(rows, COL_DISTANCE, ScoreMode::Distance)
            }
            RetrievalMode::Fts => {
                let query_text = intent
                    .query_text()
                    .ok_or_else(|| CocoError::user("query text required for fts search"))?;
                let (sql, values) = build_fts_query(
                    query_text,
                    intent.filters(),
                    &tenant,
                    &version_id,
                    &config_id,
                    top_k,
                )?;
                let rows = db
                    .query_all_raw(Statement::from_sql_and_values(
                        DatabaseBackend::Postgres,
                        sql,
                        values,
                    ))
                    .await
                    .map_err(map_storage_err)?;
                collect_results(rows, COL_SCORE, ScoreMode::Score)
            }
            RetrievalMode::Hybrid => {
                let embedding = intent
                    .query_embedding()
                    .ok_or_else(|| CocoError::user("query embedding required for hybrid search"))?;
                let query_text = intent
                    .query_text()
                    .ok_or_else(|| CocoError::user("query text required for hybrid search"))?;
                let (sql, values) = build_hybrid_query(HybridQueryArgs {
                    embedding,
                    query_text,
                    filters: intent.filters(),
                    tenant: &tenant,
                    version_id: &version_id,
                    config_id: &config_id,
                    top_k,
                    rrf_k: backend.rrf_k(),
                    operator: backend.vector_operator(),
                })?;
                let rows = db
                    .query_all_raw(Statement::from_sql_and_values(
                        DatabaseBackend::Postgres,
                        sql,
                        values,
                    ))
                    .await
                    .map_err(map_storage_err)?;
                collect_results(rows, COL_SCORE, ScoreMode::Score)
            }
        }
    }
}
