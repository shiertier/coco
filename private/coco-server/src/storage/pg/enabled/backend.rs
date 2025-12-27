use std::collections::{HashMap, HashSet};
use std::time::Duration;

use coco_core::normalize_config_id;
use coco_protocol::{CocoError, CocoResult, VectorIndexParams, VectorMetric};
use sea_orm::{
    ConnectOptions, ConnectionTrait, Database, DatabaseBackend, DatabaseConnection, Statement,
};

use super::helpers::{
    map_storage_err, push_value, validate_tenant, DEFAULT_CONNECT_TIMEOUT_SECS,
    DEFAULT_MAX_CONNECTIONS, DEFAULT_MIN_CONNECTIONS, DEFAULT_RRF_K, TABLE_PROJECTS,
};
use super::index::{
    build_fts_index_sql, build_vector_index_sql, fts_index_name, index_kind_from_params,
    vector_index_name,
};
use super::helpers::{
    chunk_from_row, COL_CONFIG_ID, COL_CONTENT, COL_DOC_ID, COL_END_LINE, COL_ID, COL_ORG_ID,
    COL_PROJECT_ID, COL_QUALITY_SCORE, COL_START_LINE, COL_USER_ID, COL_VERIFIED, COL_VERSION_ID,
    TABLE_CHUNKS,
};

/// Distance metric for pgvector.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum PgVectorMetric {
    /// L2 distance.
    L2,
    /// Cosine distance.
    Cosine,
    /// Negative inner product.
    Dot,
}

impl PgVectorMetric {
    pub(super) fn operator(self) -> &'static str {
        match self {
            PgVectorMetric::L2 => "<->",
            PgVectorMetric::Cosine => "<=>",
            PgVectorMetric::Dot => "<#>",
        }
    }

    fn index_ops(self) -> &'static str {
        match self {
            PgVectorMetric::L2 => "vector_l2_ops",
            PgVectorMetric::Cosine => "vector_cosine_ops",
            PgVectorMetric::Dot => "vector_ip_ops",
        }
    }
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

/// Tenant scoping for server storage.
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

/// PostgreSQL backend configuration.
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

impl PgBackendConfig {
    /// Creates a configuration with default settings.
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
            ensure_pgvector: true,
        }
    }
}

/// PostgreSQL storage backend.
#[derive(Debug, Clone)]
pub struct PgBackend {
    pub(super) db: DatabaseConnection,
    pub(super) tenant: TenantContext,
    pub(super) metric: PgVectorMetric,
    pub(super) rrf_k: u32,
}

impl PgBackend {
    /// Connects to PostgreSQL and initializes pgvector if requested.
    pub async fn connect(config: PgBackendConfig) -> CocoResult<Self> {
        validate_tenant(&config.tenant)?;

        let mut options = ConnectOptions::new(config.database_url);
        options.max_connections(config.max_connections);
        options.min_connections(config.min_connections);
        options.connect_timeout(config.connect_timeout);

        let db = Database::connect(options).await.map_err(map_storage_err)?;
        let backend = Self {
            db,
            tenant: config.tenant,
            metric: config.metric,
            rrf_k: config.rrf_k,
        };
        if config.ensure_pgvector {
            backend.ensure_pgvector().await?;
        }
        Ok(backend)
    }

    /// Builds a backend from an existing connection.
    pub fn from_connection(
        db: DatabaseConnection,
        tenant: TenantContext,
        metric: PgVectorMetric,
        rrf_k: u32,
    ) -> CocoResult<Self> {
        validate_tenant(&tenant)?;
        Ok(Self {
            db,
            tenant,
            metric,
            rrf_k,
        })
    }

    /// Returns a backend scoped to a specific version id.
    pub fn with_version(&self, version_id: Option<String>) -> Self {
        let mut tenant = self.tenant.clone();
        tenant.version_id = version_id;
        Self {
            db: self.db.clone(),
            tenant,
            metric: self.metric,
            rrf_k: self.rrf_k,
        }
    }

    /// Returns a backend scoped to a specific config id.
    pub fn with_config(&self, config_id: Option<String>) -> Self {
        let mut tenant = self.tenant.clone();
        tenant.config_id = config_id;
        Self {
            db: self.db.clone(),
            tenant,
            metric: self.metric,
            rrf_k: self.rrf_k,
        }
    }

    /// Returns the underlying database connection.
    pub fn connection(&self) -> &DatabaseConnection {
        &self.db
    }

    /// Returns the configured RRF constant.
    pub fn rrf_k(&self) -> u32 {
        self.rrf_k
    }

    /// Returns the pgvector extension version if available.
    pub async fn pgvector_version(&self) -> CocoResult<Option<String>> {
        let stmt = Statement::from_string(
            DatabaseBackend::Postgres,
            "SELECT extversion FROM pg_extension WHERE extname = 'vector'".to_string(),
        );
        let row = self.db.query_one(stmt).await.map_err(map_storage_err)?;
        let Some(row) = row else {
            return Ok(None);
        };
        row.try_get("", "extversion")
            .map_err(map_storage_err)
            .map(Some)
    }

    /// Ensures the pgvector extension exists.
    pub async fn ensure_pgvector(&self) -> CocoResult<()> {
        let stmt = Statement::from_string(
            DatabaseBackend::Postgres,
            "CREATE EXTENSION IF NOT EXISTS vector".to_string(),
        );
        self.db.execute(stmt).await.map_err(map_storage_err)?;
        Ok(())
    }

    /// Ensures vector and FTS indexes are present for a configuration.
    pub async fn ensure_indexes(
        &self,
        config_id: Option<&str>,
        params: Option<&VectorIndexParams>,
    ) -> CocoResult<()> {
        let index_kind = index_kind_from_params(params);
        let vector_index = vector_index_name(&index_kind, config_id);
        let fts_index = fts_index_name(config_id);
        let vector_sql = build_vector_index_sql(
            &vector_index,
            self.metric.index_ops(),
            &index_kind,
            config_id,
        );
        let fts_sql = build_fts_index_sql(&fts_index, config_id);
        self.db
            .execute(Statement::from_string(DatabaseBackend::Postgres, vector_sql))
            .await
            .map_err(map_storage_err)?;
        self.db
            .execute(Statement::from_string(DatabaseBackend::Postgres, fts_sql))
            .await
            .map_err(map_storage_err)?;
        Ok(())
    }

    pub(super) async fn resolve_version_id(&self) -> CocoResult<String> {
        if let Some(version_id) = &self.tenant.version_id {
            if version_id.trim().is_empty() {
                return Err(CocoError::user("version_id must not be empty"));
            }
            return Ok(version_id.to_string());
        }

        let mut values = Vec::new();
        let org_placeholder = push_value(&mut values, self.tenant.org_id.clone());
        let user_placeholder = push_value(&mut values, self.tenant.user_id.clone());
        let project_placeholder = push_value(&mut values, self.tenant.project_id.clone());
        let sql = format!(
            "SELECT active_version_id FROM {TABLE_PROJECTS} \
             WHERE {COL_ORG_ID} = {org_placeholder} \
             AND {COL_USER_ID} = {user_placeholder} \
             AND {COL_ID} = {project_placeholder}"
        );
        let stmt = Statement::from_sql_and_values(DatabaseBackend::Postgres, sql, values);
        let row = self.db.query_one(stmt).await.map_err(map_storage_err)?;
        let Some(row) = row else {
            return Err(CocoError::user("project not found for tenant"));
        };
        let active: Option<String> = row
            .try_get("", "active_version_id")
            .map_err(map_storage_err)?;
        active.ok_or_else(|| CocoError::user("active_version_id is not set"))
    }

    pub(super) async fn resolve_config_id(&self, requested: Option<&str>) -> CocoResult<String> {
        if let Some(config_id) = requested {
            let normalized = normalize_config_id(config_id)?;
            if normalized != config_id {
                return Err(CocoError::user("config_id must be normalized"));
            }
            return Ok(config_id.to_string());
        }
        if let Some(config_id) = &self.tenant.config_id {
            let normalized = normalize_config_id(config_id)?;
            if normalized != *config_id {
                return Err(CocoError::user("config_id must be normalized"));
            }
            return Ok(config_id.clone());
        }

        let mut values = Vec::new();
        let org_placeholder = push_value(&mut values, self.tenant.org_id.clone());
        let user_placeholder = push_value(&mut values, self.tenant.user_id.clone());
        let project_placeholder = push_value(&mut values, self.tenant.project_id.clone());
        let sql = format!(
            "SELECT active_config_id FROM {TABLE_PROJECTS} \
             WHERE {COL_ORG_ID} = {org_placeholder} \
             AND {COL_USER_ID} = {user_placeholder} \
             AND {COL_ID} = {project_placeholder}"
        );
        let stmt = Statement::from_sql_and_values(DatabaseBackend::Postgres, sql, values);
        let row = self.db.query_one(stmt).await.map_err(map_storage_err)?;
        let Some(row) = row else {
            return Err(CocoError::user("project not found for tenant"));
        };
        let active: Option<String> = row
            .try_get("", "active_config_id")
            .map_err(map_storage_err)?;
        active.ok_or_else(|| CocoError::user("active_config_id is not set"))
    }

    pub(super) fn vector_operator(&self) -> &'static str {
        self.metric.operator()
    }

    /// Filters chunk ids to those matching the current version and config scope.
    pub async fn filter_chunk_ids(&self, ids: &[String]) -> CocoResult<HashSet<String>> {
        if ids.is_empty() {
            return Ok(HashSet::new());
        }
        validate_tenant(&self.tenant)?;
        let version_id = self.resolve_version_id().await?;
        let config_id = self.resolve_config_id(None).await?;

        let mut values = Vec::new();
        let org_ph = push_value(&mut values, self.tenant.org_id.clone());
        let user_ph = push_value(&mut values, self.tenant.user_id.clone());
        let project_ph = push_value(&mut values, self.tenant.project_id.clone());
        let version_ph = push_value(&mut values, version_id);
        let config_ph = push_value(&mut values, config_id);

        let mut id_placeholders = Vec::with_capacity(ids.len());
        for id in ids {
            id_placeholders.push(push_value(&mut values, id.clone()));
        }
        let id_clause = id_placeholders.join(", ");
        let sql = format!(
            "SELECT {COL_ID} FROM {TABLE_CHUNKS} \
             WHERE {COL_ORG_ID} = {org_ph} \
             AND {COL_USER_ID} = {user_ph} \
             AND {COL_PROJECT_ID} = {project_ph} \
             AND {COL_VERSION_ID} = {version_ph} \
             AND {COL_CONFIG_ID} = {config_ph} \
             AND {COL_ID} IN ({id_clause})"
        );
        let rows = self
            .db
            .query_all(Statement::from_sql_and_values(
                DatabaseBackend::Postgres,
                sql,
                values,
            ))
            .await
            .map_err(map_storage_err)?;
        let mut allowed = HashSet::with_capacity(rows.len());
        for row in rows {
            let chunk_id: String = row.try_get("", COL_ID).map_err(map_storage_err)?;
            allowed.insert(chunk_id);
        }
        Ok(allowed)
    }

    /// Fetches chunks in batch by id for the current tenant scope.
    pub async fn get_chunks_by_ids(&self, ids: &[String]) -> CocoResult<HashMap<String, coco_protocol::Chunk>> {
        if ids.is_empty() {
            return Ok(HashMap::new());
        }
        validate_tenant(&self.tenant)?;
        let version_id = self.resolve_version_id().await?;
        let config_id = self.resolve_config_id(None).await?;

        let mut values = Vec::new();
        let org_ph = push_value(&mut values, self.tenant.org_id.clone());
        let user_ph = push_value(&mut values, self.tenant.user_id.clone());
        let project_ph = push_value(&mut values, self.tenant.project_id.clone());
        let version_ph = push_value(&mut values, version_id);
        let config_ph = push_value(&mut values, config_id);

        let mut id_placeholders = Vec::with_capacity(ids.len());
        for id in ids {
            id_placeholders.push(push_value(&mut values, id.clone()));
        }
        let id_clause = id_placeholders.join(", ");
        let sql = format!(
            "SELECT {COL_ID}, {COL_DOC_ID}, {COL_CONTENT}, \
             {COL_START_LINE}, {COL_END_LINE}, {COL_QUALITY_SCORE}, {COL_VERIFIED} \
             FROM {TABLE_CHUNKS} \
             WHERE {COL_ORG_ID} = {org_ph} \
             AND {COL_USER_ID} = {user_ph} \
             AND {COL_PROJECT_ID} = {project_ph} \
             AND {COL_VERSION_ID} = {version_ph} \
             AND {COL_CONFIG_ID} = {config_ph} \
             AND {COL_ID} IN ({id_clause})"
        );
        let rows = self
            .db
            .query_all(Statement::from_sql_and_values(
                DatabaseBackend::Postgres,
                sql,
                values,
            ))
            .await
            .map_err(map_storage_err)?;
        let mut chunks = HashMap::with_capacity(rows.len());
        for row in rows {
            let chunk = chunk_from_row(&row)?;
            chunks.insert(chunk.id.to_string(), chunk);
        }
        Ok(chunks)
    }
}
