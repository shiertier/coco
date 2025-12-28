//! PostgreSQL metadata store for server mode.

use chrono::{NaiveDate, Utc};
use coco_core::{normalize_config_id, validate_indexing_config};
use coco_protocol::{
    ChunkingStrategy, CocoError, CocoResult, EmbeddingConfig, IndexingConfig, ValidationContext,
    VectorBackendConfig, VectorIndexParams, VectorMetric,
};
use sea_orm::{
    ActiveModelTrait, ColumnTrait, ConnectionTrait, Database, DatabaseBackend, DatabaseConnection,
    DatabaseTransaction, EntityTrait, IntoActiveModel, PaginatorTrait, QueryFilter, QueryOrder,
    Set, Statement, TransactionTrait, Value,
    sea_query::{Expr, OnConflict},
};
use sea_orm_migration::MigratorTrait;

mod entities;
mod migration;

use entities::{
    chunks, documents, indexing_configs, ingest_jobs, organizations, project_versions, projects,
    worker_status,
};

const STATUS_ACTIVE: &str = "active";
const STATUS_READY: &str = "ready";
const STATUS_ARCHIVED: &str = "archived";
/// Default indexing configuration identifier.
pub const DEFAULT_CONFIG_ID: &str = "default";
/// Server-side fixed embedding dimension for pgvector.
pub const DEFAULT_EMBEDDING_DIM: u32 = 1536;
const DEFAULT_VECTOR_METRIC: VectorMetric = VectorMetric::L2;

/// Ingest job status: queued.
pub const JOB_STATUS_QUEUED: &str = "queued";
/// Ingest job status: running.
pub const JOB_STATUS_RUNNING: &str = "running";
/// Ingest job status: failed.
pub const JOB_STATUS_FAILED: &str = "failed";
/// Ingest job status: completed.
pub const JOB_STATUS_COMPLETED: &str = "completed";

/// Metadata store backed by PostgreSQL.
#[derive(Clone)]
pub struct ServerMetaStore {
    db: DatabaseConnection,
}

impl ServerMetaStore {
    /// Connects to PostgreSQL and runs migrations.
    pub async fn connect(url: &str) -> CocoResult<Self> {
        let db = Database::connect(url).await.map_err(map_storage_err)?;
        let store = Self { db };
        store.run_migrations().await?;
        Ok(store)
    }

    /// Returns the underlying database connection.
    pub fn connection(&self) -> &DatabaseConnection {
        &self.db
    }

    /// Begins a database transaction.
    pub async fn begin(&self) -> CocoResult<DatabaseTransaction> {
        self.db.begin().await.map_err(map_storage_err)
    }

    /// Creates a new indexing configuration record.
    pub async fn create_indexing_config(
        &self,
        config: NewIndexingConfig,
    ) -> CocoResult<IndexingConfigRecord> {
        let protocol_config = config.to_protocol();
        validate_indexing_config(&protocol_config, &ValidationContext::default())?;
        validate_indexing_config_constraints(&protocol_config)?;
        validate_indexing_config_constraints(&protocol_config)?;
        ensure_canonical_config_id(&config.config_id)?;

        let exists = indexing_configs::Entity::find()
            .filter(indexing_configs::Column::OrgId.eq(&config.org_id))
            .filter(indexing_configs::Column::ConfigId.eq(&config.config_id))
            .one(&self.db)
            .await
            .map_err(map_storage_err)?;
        if exists.is_some() {
            return Err(CocoError::user("indexing config already exists"));
        }

        let record = IndexingConfigRecord::from(config);
        let active_model = indexing_configs::ActiveModel {
            org_id: Set(record.org_id.clone()),
            config_id: Set(record.config_id.clone()),
            chunking: Set(serialize_json(&record.chunking, "chunking")?),
            embedding: Set(serialize_json(&record.embedding, "embedding")?),
            vector_backend: Set(optional_json(&record.vector_backend, "vector_backend")?),
            vector_metric: Set(vector_metric_to_string(record.vector_metric)),
            index_params: Set(optional_json(&record.index_params, "index_params")?),
            created_at: Set(record.created_at),
        };
        indexing_configs::Entity::insert(active_model)
            .exec(&self.db)
            .await
            .map_err(map_storage_err)?;
        Ok(record)
    }

    /// Ensures the default indexing configuration exists.
    pub async fn ensure_default_indexing_config(
        &self,
        config: NewIndexingConfig,
    ) -> CocoResult<IndexingConfigRecord> {
        if config.config_id != DEFAULT_CONFIG_ID {
            return Err(CocoError::user(
                "default indexing config must use config_id=default",
            ));
        }
        if let Some(existing) = self
            .get_indexing_config(&config.org_id, &config.config_id)
            .await?
        {
            let existing_protocol = existing.to_protocol();
            let candidate_protocol = config.to_protocol();
            if existing_protocol != candidate_protocol {
                return Err(CocoError::user(
                    "default indexing config already exists with different values",
                ));
            }
            return Ok(existing);
        }
        self.create_indexing_config(config).await
    }

    /// Fetches an indexing configuration by org + config id.
    pub async fn get_indexing_config(
        &self,
        org_id: &str,
        config_id: &str,
    ) -> CocoResult<Option<IndexingConfigRecord>> {
        ensure_canonical_config_id(config_id)?;
        let model = indexing_configs::Entity::find()
            .filter(indexing_configs::Column::OrgId.eq(org_id))
            .filter(indexing_configs::Column::ConfigId.eq(config_id))
            .one(&self.db)
            .await
            .map_err(map_storage_err)?;
        model.map(IndexingConfigRecord::try_from).transpose()
    }

    /// Lists indexing configurations for an organization.
    pub async fn list_indexing_configs(
        &self,
        org_id: &str,
    ) -> CocoResult<Vec<IndexingConfigRecord>> {
        let models = indexing_configs::Entity::find()
            .filter(indexing_configs::Column::OrgId.eq(org_id))
            .all(&self.db)
            .await
            .map_err(map_storage_err)?;
        models
            .into_iter()
            .map(IndexingConfigRecord::try_from)
            .collect()
    }

    /// Ensures an indexing configuration exists for an organization.
    pub async fn ensure_indexing_config_exists(
        &self,
        org_id: &str,
        config_id: &str,
    ) -> CocoResult<()> {
        ensure_indexing_config_exists_with(&self.db, org_id, config_id).await
    }

    /// Updates an existing indexing configuration if it has not been referenced.
    pub async fn update_indexing_config(
        &self,
        config: NewIndexingConfig,
    ) -> CocoResult<IndexingConfigRecord> {
        if config.config_id == DEFAULT_CONFIG_ID {
            return Err(CocoError::user("default indexing config cannot be updated"));
        }
        let protocol_config = config.to_protocol();
        validate_indexing_config(&protocol_config, &ValidationContext::default())?;

        let txn = self.db.begin().await.map_err(map_storage_err)?;
        let model = indexing_configs::Entity::find()
            .filter(indexing_configs::Column::OrgId.eq(&config.org_id))
            .filter(indexing_configs::Column::ConfigId.eq(&config.config_id))
            .one(&txn)
            .await
            .map_err(map_storage_err)?;
        let Some(model) = model else {
            txn.rollback().await.map_err(map_storage_err)?;
            return Err(CocoError::user("indexing config not found"));
        };

        let doc_refs = documents::Entity::find()
            .filter(documents::Column::OrgId.eq(&config.org_id))
            .filter(documents::Column::ConfigId.eq(&config.config_id))
            .count(&txn)
            .await
            .map_err(map_storage_err)?;
        let chunk_refs = chunks::Entity::find()
            .filter(chunks::Column::OrgId.eq(&config.org_id))
            .filter(chunks::Column::ConfigId.eq(&config.config_id))
            .count(&txn)
            .await
            .map_err(map_storage_err)?;
        let project_refs = projects::Entity::find()
            .filter(projects::Column::OrgId.eq(&config.org_id))
            .filter(projects::Column::ActiveConfigId.eq(&config.config_id))
            .count(&txn)
            .await
            .map_err(map_storage_err)?;
        let version_refs = project_versions::Entity::find()
            .filter(project_versions::Column::OrgId.eq(&config.org_id))
            .filter(project_versions::Column::ActiveConfigId.eq(&config.config_id))
            .count(&txn)
            .await
            .map_err(map_storage_err)?;
        if doc_refs > 0 || chunk_refs > 0 || project_refs > 0 || version_refs > 0 {
            txn.rollback().await.map_err(map_storage_err)?;
            return Err(CocoError::user(
                "indexing config is already referenced and cannot be updated",
            ));
        }

        let record = IndexingConfigRecord::from(config);
        let mut active_model: indexing_configs::ActiveModel = model.into();
        active_model.chunking = Set(serialize_json(&record.chunking, "chunking")?);
        active_model.embedding = Set(serialize_json(&record.embedding, "embedding")?);
        active_model.vector_backend = Set(optional_json(&record.vector_backend, "vector_backend")?);
        active_model.vector_metric = Set(vector_metric_to_string(record.vector_metric));
        active_model.index_params = Set(optional_json(&record.index_params, "index_params")?);
        active_model.created_at = Set(record.created_at);

        let updated = active_model.update(&txn).await.map_err(map_storage_err)?;
        txn.commit().await.map_err(map_storage_err)?;
        IndexingConfigRecord::try_from(updated)
    }

    /// Creates an organization record.
    pub async fn create_organization(
        &self,
        org: NewOrganization,
    ) -> CocoResult<OrganizationRecord> {
        let active_model = organizations::ActiveModel {
            id: Set(org.id.clone()),
            name: Set(org.name.clone()),
            created_at: Set(org.created_at),
            max_documents: Set(org.max_documents),
            max_chunks: Set(org.max_chunks),
            max_storage_bytes: Set(org.max_storage_bytes),
            max_embeddings_per_day: Set(org.max_embeddings_per_day),
        };
        organizations::Entity::insert(active_model)
            .exec(&self.db)
            .await
            .map_err(map_storage_err)?;
        Ok(OrganizationRecord::from(org))
    }

    /// Fetches an organization by id.
    pub async fn get_organization(&self, org_id: &str) -> CocoResult<Option<OrganizationRecord>> {
        let model = organizations::Entity::find_by_id(org_id)
            .one(&self.db)
            .await
            .map_err(map_storage_err)?;
        Ok(model.map(OrganizationRecord::from))
    }

    /// Fetches worker status by id.
    pub async fn get_worker_status(
        &self,
        worker_id: &str,
    ) -> CocoResult<Option<WorkerStatusRecord>> {
        let model = worker_status::Entity::find_by_id(worker_id)
            .one(&self.db)
            .await
            .map_err(map_storage_err)?;
        Ok(model.map(WorkerStatusRecord::from))
    }

    /// Upserts worker status by id.
    pub async fn upsert_worker_status(&self, worker_id: &str, version: &str) -> CocoResult<()> {
        let updated_at = now();
        let active_model = worker_status::ActiveModel {
            id: Set(worker_id.to_string()),
            version: Set(version.to_string()),
            updated_at: Set(updated_at),
        };
        worker_status::Entity::insert(active_model)
            .on_conflict(
                OnConflict::column(worker_status::Column::Id)
                    .update_columns([
                        worker_status::Column::Version,
                        worker_status::Column::UpdatedAt,
                    ])
                    .to_owned(),
            )
            .exec(&self.db)
            .await
            .map_err(map_storage_err)?;
        Ok(())
    }

    /// Creates a project record.
    pub async fn create_project(&self, project: NewProject) -> CocoResult<ProjectRecord> {
        self.ensure_indexing_config_exists(&project.org_id, &project.active_config_id)
            .await?;
        let active_model = projects::ActiveModel {
            id: Set(project.id.clone()),
            org_id: Set(project.org_id.clone()),
            user_id: Set(project.user_id.clone()),
            name: Set(project.name.clone()),
            created_at: Set(project.created_at),
            active_version_id: Set(project.active_version_id.clone()),
            active_config_id: Set(project.active_config_id.clone()),
        };
        projects::Entity::insert(active_model)
            .exec(&self.db)
            .await
            .map_err(map_storage_err)?;
        Ok(ProjectRecord::from(project))
    }

    /// Fetches a project by org and project id.
    pub async fn get_project(
        &self,
        org_id: &str,
        user_id: &str,
        project_id: &str,
    ) -> CocoResult<Option<ProjectRecord>> {
        let model = projects::Entity::find()
            .filter(projects::Column::OrgId.eq(org_id))
            .filter(projects::Column::UserId.eq(user_id))
            .filter(projects::Column::Id.eq(project_id))
            .one(&self.db)
            .await
            .map_err(map_storage_err)?;
        Ok(model.map(ProjectRecord::from))
    }

    /// Updates fields on an existing project.
    pub async fn update_project(
        &self,
        org_id: &str,
        user_id: &str,
        project_id: &str,
        update: ProjectUpdate,
    ) -> CocoResult<Option<ProjectRecord>> {
        let txn = self.db.begin().await.map_err(map_storage_err)?;
        let model = projects::Entity::find()
            .filter(projects::Column::OrgId.eq(org_id))
            .filter(projects::Column::UserId.eq(user_id))
            .filter(projects::Column::Id.eq(project_id))
            .one(&txn)
            .await
            .map_err(map_storage_err)?;
        let Some(model) = model else {
            txn.rollback().await.map_err(map_storage_err)?;
            return Ok(None);
        };
        let mut active_model: projects::ActiveModel = model.into();
        if let Some(name) = update.name {
            active_model.name = Set(name);
        }
        if let Some(active_version_id) = update.active_version_id {
            active_model.active_version_id = Set(active_version_id);
        }
        if let Some(active_config_id) = update.active_config_id {
            ensure_indexing_config_exists_with(&txn, org_id, &active_config_id).await?;
            active_model.active_config_id = Set(active_config_id);
        }
        let updated = active_model.update(&txn).await.map_err(map_storage_err)?;
        txn.commit().await.map_err(map_storage_err)?;
        Ok(Some(ProjectRecord::from(updated)))
    }

    /// Creates a document record.
    pub async fn create_document(&self, document: NewDocument) -> CocoResult<DocumentRecord> {
        self.ensure_indexing_config_exists(&document.org_id, &document.config_id)
            .await?;
        let active_model = documents::ActiveModel {
            id: Set(document.id.clone()),
            org_id: Set(document.org_id.clone()),
            user_id: Set(document.user_id.clone()),
            project_id: Set(document.project_id.clone()),
            config_id: Set(document.config_id.clone()),
            source_ref: Set(document.source_ref.clone()),
            title: Set(document.title.clone()),
            content_hash: Set(document.content_hash.clone()),
            indexed_at: Set(document.indexed_at),
            quality_score: Set(document.quality_score),
            verified: Set(document.verified),
        };
        documents::Entity::insert(active_model)
            .exec(&self.db)
            .await
            .map_err(map_storage_err)?;
        Ok(DocumentRecord::from(document))
    }

    /// Creates or updates a document record.
    pub async fn upsert_document(&self, document: NewDocument) -> CocoResult<DocumentRecord> {
        self.ensure_indexing_config_exists(&document.org_id, &document.config_id)
            .await?;
        let active_model = documents::ActiveModel {
            id: Set(document.id.clone()),
            org_id: Set(document.org_id.clone()),
            user_id: Set(document.user_id.clone()),
            project_id: Set(document.project_id.clone()),
            config_id: Set(document.config_id.clone()),
            source_ref: Set(document.source_ref.clone()),
            title: Set(document.title.clone()),
            content_hash: Set(document.content_hash.clone()),
            indexed_at: Set(document.indexed_at),
            quality_score: Set(document.quality_score),
            verified: Set(document.verified),
        };
        documents::Entity::insert(active_model)
            .on_conflict(
                OnConflict::column(documents::Column::Id)
                    .update_columns([
                        documents::Column::OrgId,
                        documents::Column::UserId,
                        documents::Column::ProjectId,
                        documents::Column::ConfigId,
                        documents::Column::SourceRef,
                        documents::Column::Title,
                        documents::Column::ContentHash,
                        documents::Column::IndexedAt,
                        documents::Column::QualityScore,
                        documents::Column::Verified,
                    ])
                    .to_owned(),
            )
            .exec(&self.db)
            .await
            .map_err(map_storage_err)?;
        Ok(DocumentRecord::from(document))
    }

    /// Creates a new project version and returns it.
    pub async fn create_project_version(
        &self,
        org_id: &str,
        user_id: &str,
        project_id: &str,
        config_id: &str,
    ) -> CocoResult<ProjectVersionRecord> {
        self.ensure_indexing_config_exists(org_id, config_id)
            .await?;
        let version_id = generate_version_id()?;
        let created_at = now();
        let active_model = project_versions::ActiveModel {
            id: Set(version_id.clone()),
            org_id: Set(org_id.to_string()),
            user_id: Set(user_id.to_string()),
            project_id: Set(project_id.to_string()),
            created_at: Set(created_at),
            status: Set(STATUS_READY.to_string()),
            active_config_id: Set(config_id.to_string()),
        };
        project_versions::Entity::insert(active_model)
            .exec(&self.db)
            .await
            .map_err(map_storage_err)?;
        Ok(ProjectVersionRecord {
            id: version_id,
            org_id: org_id.to_string(),
            user_id: user_id.to_string(),
            project_id: project_id.to_string(),
            created_at,
            status: STATUS_READY.to_string(),
            active_config_id: config_id.to_string(),
        })
    }

    /// Activates a project version and updates the project pointer in one transaction.
    pub async fn activate_project_version(
        &self,
        org_id: &str,
        user_id: &str,
        project_id: &str,
        version_id: &str,
    ) -> CocoResult<()> {
        let txn = self.db.begin().await.map_err(map_storage_err)?;
        let version = project_versions::Entity::find()
            .filter(project_versions::Column::OrgId.eq(org_id))
            .filter(project_versions::Column::UserId.eq(user_id))
            .filter(project_versions::Column::ProjectId.eq(project_id))
            .filter(project_versions::Column::Id.eq(version_id))
            .one(&txn)
            .await
            .map_err(map_storage_err)?;
        let Some(version) = version else {
            txn.rollback().await.map_err(map_storage_err)?;
            return Err(CocoError::user("project version not found"));
        };
        projects::Entity::update_many()
            .filter(projects::Column::OrgId.eq(org_id))
            .filter(projects::Column::UserId.eq(user_id))
            .filter(projects::Column::Id.eq(project_id))
            .col_expr(
                projects::Column::ActiveVersionId,
                Expr::value(version_id.to_string()),
            )
            .col_expr(
                projects::Column::ActiveConfigId,
                Expr::value(version.active_config_id.clone()),
            )
            .exec(&txn)
            .await
            .map_err(map_storage_err)?;

        project_versions::Entity::update_many()
            .filter(project_versions::Column::OrgId.eq(org_id))
            .filter(project_versions::Column::UserId.eq(user_id))
            .filter(project_versions::Column::ProjectId.eq(project_id))
            .col_expr(
                project_versions::Column::Status,
                Expr::value(STATUS_ARCHIVED.to_string()),
            )
            .exec(&txn)
            .await
            .map_err(map_storage_err)?;

        project_versions::Entity::update_many()
            .filter(project_versions::Column::OrgId.eq(org_id))
            .filter(project_versions::Column::UserId.eq(user_id))
            .filter(project_versions::Column::ProjectId.eq(project_id))
            .filter(project_versions::Column::Id.eq(version_id))
            .col_expr(
                project_versions::Column::Status,
                Expr::value(STATUS_ACTIVE.to_string()),
            )
            .exec(&txn)
            .await
            .map_err(map_storage_err)?;

        txn.commit().await.map_err(map_storage_err)?;
        Ok(())
    }

    /// Archives old versions and deletes their chunks.
    pub async fn gc_project_versions(
        &self,
        org_id: &str,
        user_id: &str,
        project_id: &str,
        keep: usize,
    ) -> CocoResult<usize> {
        if keep == 0 {
            return Err(CocoError::user("keep must be greater than zero"));
        }

        let versions = project_versions::Entity::find()
            .filter(project_versions::Column::OrgId.eq(org_id))
            .filter(project_versions::Column::UserId.eq(user_id))
            .filter(project_versions::Column::ProjectId.eq(project_id))
            .order_by_desc(project_versions::Column::CreatedAt)
            .all(&self.db)
            .await
            .map_err(map_storage_err)?;

        if versions.len() <= keep {
            return Ok(0);
        }

        let archive_ids: Vec<String> = versions
            .into_iter()
            .skip(keep)
            .map(|version| version.id)
            .collect();

        project_versions::Entity::update_many()
            .filter(project_versions::Column::OrgId.eq(org_id))
            .filter(project_versions::Column::UserId.eq(user_id))
            .filter(project_versions::Column::ProjectId.eq(project_id))
            .filter(project_versions::Column::Id.is_in(archive_ids.clone()))
            .col_expr(
                project_versions::Column::Status,
                Expr::value(STATUS_ARCHIVED.to_string()),
            )
            .exec(&self.db)
            .await
            .map_err(map_storage_err)?;

        let delete_result = chunks::Entity::delete_many()
            .filter(chunks::Column::OrgId.eq(org_id))
            .filter(chunks::Column::UserId.eq(user_id))
            .filter(chunks::Column::ProjectId.eq(project_id))
            .filter(chunks::Column::VersionId.is_in(archive_ids))
            .exec(&self.db)
            .await
            .map_err(map_storage_err)?;

        Ok(delete_result.rows_affected as usize)
    }

    /// Counts documents for an organization.
    pub async fn count_documents_by_org(&self, org_id: &str, user_id: &str) -> CocoResult<i64> {
        let count = documents::Entity::find()
            .filter(documents::Column::OrgId.eq(org_id))
            .filter(documents::Column::UserId.eq(user_id))
            .count(&self.db)
            .await
            .map_err(map_storage_err)?;
        i64::try_from(count).map_err(|_| CocoError::storage("document count exceeds i64 range"))
    }

    /// Counts chunks for an organization.
    pub async fn count_chunks_by_org(&self, org_id: &str, user_id: &str) -> CocoResult<i64> {
        let count = chunks::Entity::find()
            .filter(chunks::Column::OrgId.eq(org_id))
            .filter(chunks::Column::UserId.eq(user_id))
            .count(&self.db)
            .await
            .map_err(map_storage_err)?;
        i64::try_from(count).map_err(|_| CocoError::storage("chunk count exceeds i64 range"))
    }

    /// Sums chunk content sizes for an organization.
    pub async fn sum_chunk_bytes_by_org(&self, org_id: &str, user_id: &str) -> CocoResult<i64> {
        let sql = "SELECT COALESCE(SUM(LENGTH(content)), 0) AS total \
                   FROM chunks WHERE org_id = $1 AND user_id = $2";
        let stmt = Statement::from_sql_and_values(
            DatabaseBackend::Postgres,
            sql.to_string(),
            vec![
                Value::from(org_id.to_string()),
                Value::from(user_id.to_string()),
            ],
        );
        let row = self.db.query_one(stmt).await.map_err(map_storage_err)?;
        let Some(row) = row else {
            return Ok(0);
        };
        row.try_get("", "total").map_err(map_storage_err)
    }

    /// Returns embedding usage for a day.
    pub async fn get_embedding_calls_for_day(
        &self,
        org_id: &str,
        user_id: &str,
        day: NaiveDate,
    ) -> CocoResult<i64> {
        let sql = "SELECT embedding_calls FROM org_daily_usage \
                   WHERE org_id = $1 AND user_id = $2 AND day = $3";
        let stmt = Statement::from_sql_and_values(
            DatabaseBackend::Postgres,
            sql.to_string(),
            vec![
                Value::from(org_id.to_string()),
                Value::from(user_id.to_string()),
                Value::from(day),
            ],
        );
        let row = self.db.query_one(stmt).await.map_err(map_storage_err)?;
        let Some(row) = row else {
            return Ok(0);
        };
        row.try_get("", "embedding_calls").map_err(map_storage_err)
    }

    /// Increments embedding usage for a day.
    pub async fn increment_embedding_calls(
        &self,
        org_id: &str,
        user_id: &str,
        day: NaiveDate,
        add: i64,
    ) -> CocoResult<()> {
        if add <= 0 {
            return Ok(());
        }
        let sql = "INSERT INTO org_daily_usage \
                   (org_id, user_id, day, embedding_calls, updated_at) \
                   VALUES ($1, $2, $3, $4, $5) \
                   ON CONFLICT (org_id, user_id, day) DO UPDATE \
                   SET embedding_calls = org_daily_usage.embedding_calls + $3, \
                       updated_at = $5";
        let now = now();
        let stmt = Statement::from_sql_and_values(
            DatabaseBackend::Postgres,
            sql.to_string(),
            vec![
                Value::from(org_id.to_string()),
                Value::from(user_id.to_string()),
                Value::from(day),
                Value::from(add),
                Value::from(now),
            ],
        );
        self.db.execute(stmt).await.map_err(map_storage_err)?;
        Ok(())
    }

    /// Creates a new ingest job record.
    pub async fn create_ingest_job(&self, job: NewIngestJob) -> CocoResult<IngestJobRecord> {
        let active_model = ingest_jobs::ActiveModel {
            id: Set(job.id.clone()),
            org_id: Set(job.org_id.clone()),
            user_id: Set(job.user_id.clone()),
            project_id: Set(job.project_id.clone()),
            status: Set(JOB_STATUS_QUEUED.to_string()),
            payload: Set(job.payload.clone()),
            attempts: Set(0),
            error: Set(None),
            version_id: Set(None),
            created_at: Set(job.created_at),
            updated_at: Set(job.updated_at),
        };
        ingest_jobs::Entity::insert(active_model)
            .exec(&self.db)
            .await
            .map_err(map_storage_err)?;
        Ok(IngestJobRecord::from(job))
    }

    /// Fetches an ingest job by id.
    pub async fn get_ingest_job(&self, job_id: &str) -> CocoResult<Option<IngestJobRecord>> {
        let model = ingest_jobs::Entity::find_by_id(job_id)
            .one(&self.db)
            .await
            .map_err(map_storage_err)?;
        Ok(model.map(IngestJobRecord::from))
    }

    /// Claims a queued ingest job by id and marks it running.
    pub async fn claim_ingest_job_by_id(&self, job_id: &str) -> CocoResult<IngestJobRecord> {
        let model = ingest_jobs::Entity::find_by_id(job_id)
            .one(&self.db)
            .await
            .map_err(map_storage_err)?;
        let Some(model) = model else {
            return Err(CocoError::user("job not found"));
        };
        if model.status != JOB_STATUS_QUEUED {
            return Err(CocoError::user("job not queued"));
        }
        let attempts = model.attempts;
        let mut active = model.into_active_model();
        active.status = Set(JOB_STATUS_RUNNING.to_string());
        active.attempts = Set(attempts + 1);
        active.updated_at = Set(now());
        let updated = active.update(&self.db).await.map_err(map_storage_err)?;
        Ok(IngestJobRecord::from(updated))
    }

    /// Claims the next queued ingest job and marks it running.
    pub async fn claim_next_ingest_job(&self) -> CocoResult<Option<IngestJobRecord>> {
        let model = ingest_jobs::Entity::find()
            .filter(ingest_jobs::Column::Status.eq(JOB_STATUS_QUEUED))
            .order_by_asc(ingest_jobs::Column::CreatedAt)
            .one(&self.db)
            .await
            .map_err(map_storage_err)?;
        let Some(model) = model else {
            return Ok(None);
        };
        let attempts = model.attempts;
        let updated_at = now();
        let mut active = model.into_active_model();
        active.status = Set(JOB_STATUS_RUNNING.to_string());
        active.attempts = Set(attempts + 1);
        active.updated_at = Set(updated_at);
        let updated = active.update(&self.db).await.map_err(map_storage_err)?;
        Ok(Some(IngestJobRecord::from(updated)))
    }

    /// Updates ingest job status and optional fields.
    pub async fn update_ingest_job_status(
        &self,
        job_id: &str,
        status: &str,
        error: Option<String>,
        version_id: Option<String>,
    ) -> CocoResult<()> {
        let model = ingest_jobs::Entity::find_by_id(job_id)
            .one(&self.db)
            .await
            .map_err(map_storage_err)?;
        let Some(model) = model else {
            return Err(CocoError::user("job not found"));
        };
        let mut active = model.into_active_model();
        active.status = Set(status.to_string());
        active.error = Set(error);
        active.version_id = Set(version_id);
        active.updated_at = Set(now());
        active.update(&self.db).await.map_err(map_storage_err)?;
        Ok(())
    }

    async fn run_migrations(&self) -> CocoResult<()> {
        migration::Migrator::up(&self.db, None)
            .await
            .map_err(map_storage_err)
    }
}

/// Input data for creating an organization.
#[derive(Debug, Clone)]
pub struct NewOrganization {
    /// Organization identifier.
    pub id: String,
    /// Organization name.
    pub name: String,
    /// Creation timestamp.
    pub created_at: sea_orm::prelude::DateTimeUtc,
    /// Max documents allowed.
    pub max_documents: Option<i64>,
    /// Max chunks allowed.
    pub max_chunks: Option<i64>,
    /// Max storage bytes allowed.
    pub max_storage_bytes: Option<i64>,
    /// Max embedding calls per day.
    pub max_embeddings_per_day: Option<i64>,
}

/// Stored organization record.
#[derive(Debug, Clone)]
pub struct OrganizationRecord {
    /// Organization identifier.
    pub id: String,
    /// Organization name.
    pub name: String,
    /// Creation timestamp.
    pub created_at: sea_orm::prelude::DateTimeUtc,
    /// Max documents allowed.
    pub max_documents: Option<i64>,
    /// Max chunks allowed.
    pub max_chunks: Option<i64>,
    /// Max storage bytes allowed.
    pub max_storage_bytes: Option<i64>,
    /// Max embedding calls per day.
    pub max_embeddings_per_day: Option<i64>,
}

/// Input data for creating a project.
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
    pub created_at: sea_orm::prelude::DateTimeUtc,
    /// Active version identifier if known.
    pub active_version_id: Option<String>,
    /// Active indexing configuration identifier.
    pub active_config_id: String,
}

/// Stored project record.
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
    /// Creation timestamp.
    pub created_at: sea_orm::prelude::DateTimeUtc,
    /// Active version identifier.
    pub active_version_id: Option<String>,
    /// Active indexing configuration identifier.
    pub active_config_id: String,
}

/// Project fields that can be updated.
#[derive(Debug, Clone, Default)]
pub struct ProjectUpdate {
    /// Updated name, if present.
    pub name: Option<String>,
    /// Updated active version id (use `Some(None)` to clear).
    pub active_version_id: Option<Option<String>>,
    /// Updated active config id, if present.
    pub active_config_id: Option<String>,
}

/// Input data for creating an indexing configuration.
#[derive(Debug, Clone)]
pub struct NewIndexingConfig {
    /// Organization identifier.
    pub org_id: String,
    /// Indexing configuration identifier.
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
    pub created_at: sea_orm::prelude::DateTimeUtc,
}

impl NewIndexingConfig {
    fn to_protocol(&self) -> IndexingConfig {
        IndexingConfig {
            config_id: self.config_id.clone(),
            chunking: self.chunking.clone(),
            embedding: self.embedding.clone(),
            vector_backend: self.vector_backend.clone(),
            vector_metric: self.vector_metric,
            index_params: self.index_params.clone(),
        }
    }
}

/// Stored indexing configuration record.
#[derive(Debug, Clone)]
pub struct IndexingConfigRecord {
    /// Organization identifier.
    pub org_id: String,
    /// Indexing configuration identifier.
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
    pub created_at: sea_orm::prelude::DateTimeUtc,
}

impl IndexingConfigRecord {
    fn to_protocol(&self) -> IndexingConfig {
        IndexingConfig {
            config_id: self.config_id.clone(),
            chunking: self.chunking.clone(),
            embedding: self.embedding.clone(),
            vector_backend: self.vector_backend.clone(),
            vector_metric: self.vector_metric,
            index_params: self.index_params.clone(),
        }
    }
}

/// Input data for creating a document.
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
    /// Source reference for upstream pipeline.
    pub source_ref: String,
    /// Optional document title.
    pub title: Option<String>,
    /// Content hash for change detection.
    pub content_hash: String,
    /// Index timestamp.
    pub indexed_at: sea_orm::prelude::DateTimeUtc,
    /// Optional quality score.
    pub quality_score: Option<f32>,
    /// Whether the document has been verified.
    pub verified: bool,
}

/// Stored document record.
#[derive(Debug, Clone)]
pub struct DocumentRecord {
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
    /// Source reference for upstream pipeline.
    pub source_ref: String,
    /// Optional document title.
    pub title: Option<String>,
    /// Content hash for change detection.
    pub content_hash: String,
    /// Index timestamp.
    pub indexed_at: sea_orm::prelude::DateTimeUtc,
    /// Optional quality score.
    pub quality_score: Option<f32>,
    /// Whether the document has been verified.
    pub verified: bool,
}

/// Stored project version record.
#[derive(Debug, Clone)]
pub struct ProjectVersionRecord {
    /// Version identifier.
    pub id: String,
    /// Organization identifier.
    pub org_id: String,
    /// User identifier.
    pub user_id: String,
    /// Project identifier.
    pub project_id: String,
    /// Creation timestamp.
    pub created_at: sea_orm::prelude::DateTimeUtc,
    /// Version status.
    pub status: String,
    /// Indexing configuration identifier for this version.
    pub active_config_id: String,
}

/// Input data for creating an ingest job.
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
    /// JSON payload with batch data.
    pub payload: String,
    /// Creation timestamp.
    pub created_at: sea_orm::prelude::DateTimeUtc,
    /// Update timestamp.
    pub updated_at: sea_orm::prelude::DateTimeUtc,
}

/// Stored ingest job record.
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
    /// Job status.
    pub status: String,
    /// JSON payload with batch data.
    pub payload: String,
    /// Attempt counter.
    pub attempts: i32,
    /// Optional error details.
    pub error: Option<String>,
    /// Optional version identifier.
    pub version_id: Option<String>,
    /// Creation timestamp.
    pub created_at: sea_orm::prelude::DateTimeUtc,
    /// Update timestamp.
    pub updated_at: sea_orm::prelude::DateTimeUtc,
}

/// Stored worker status record.
#[derive(Debug, Clone)]
pub struct WorkerStatusRecord {
    /// Worker identifier.
    pub id: String,
    /// Worker version signature.
    pub version: String,
    /// Last heartbeat time.
    pub updated_at: sea_orm::prelude::DateTimeUtc,
}

impl From<NewOrganization> for OrganizationRecord {
    fn from(value: NewOrganization) -> Self {
        Self {
            id: value.id,
            name: value.name,
            created_at: value.created_at,
            max_documents: value.max_documents,
            max_chunks: value.max_chunks,
            max_storage_bytes: value.max_storage_bytes,
            max_embeddings_per_day: value.max_embeddings_per_day,
        }
    }
}

impl From<organizations::Model> for OrganizationRecord {
    fn from(value: organizations::Model) -> Self {
        Self {
            id: value.id,
            name: value.name,
            created_at: value.created_at,
            max_documents: value.max_documents,
            max_chunks: value.max_chunks,
            max_storage_bytes: value.max_storage_bytes,
            max_embeddings_per_day: value.max_embeddings_per_day,
        }
    }
}

impl From<NewProject> for ProjectRecord {
    fn from(value: NewProject) -> Self {
        Self {
            id: value.id,
            org_id: value.org_id,
            user_id: value.user_id,
            name: value.name,
            created_at: value.created_at,
            active_version_id: value.active_version_id,
            active_config_id: value.active_config_id,
        }
    }
}

impl From<NewIndexingConfig> for IndexingConfigRecord {
    fn from(value: NewIndexingConfig) -> Self {
        Self {
            org_id: value.org_id,
            config_id: value.config_id,
            chunking: value.chunking,
            embedding: value.embedding,
            vector_backend: value.vector_backend,
            vector_metric: value.vector_metric,
            index_params: value.index_params,
            created_at: value.created_at,
        }
    }
}

impl TryFrom<indexing_configs::Model> for IndexingConfigRecord {
    type Error = CocoError;

    fn try_from(value: indexing_configs::Model) -> CocoResult<Self> {
        Ok(Self {
            org_id: value.org_id,
            config_id: value.config_id,
            chunking: deserialize_json(&value.chunking, "chunking")?,
            embedding: deserialize_json(&value.embedding, "embedding")?,
            vector_backend: optional_deserialize_json(value.vector_backend, "vector_backend")?,
            vector_metric: vector_metric_from_string(&value.vector_metric)?,
            index_params: optional_deserialize_json(value.index_params, "index_params")?,
            created_at: value.created_at,
        })
    }
}

impl From<projects::Model> for ProjectRecord {
    fn from(value: projects::Model) -> Self {
        Self {
            id: value.id,
            org_id: value.org_id,
            user_id: value.user_id,
            name: value.name,
            created_at: value.created_at,
            active_version_id: value.active_version_id,
            active_config_id: value.active_config_id,
        }
    }
}

impl From<NewDocument> for DocumentRecord {
    fn from(value: NewDocument) -> Self {
        Self {
            id: value.id,
            org_id: value.org_id,
            user_id: value.user_id,
            project_id: value.project_id,
            config_id: value.config_id,
            source_ref: value.source_ref,
            title: value.title,
            content_hash: value.content_hash,
            indexed_at: value.indexed_at,
            quality_score: value.quality_score,
            verified: value.verified,
        }
    }
}

impl From<NewIngestJob> for IngestJobRecord {
    fn from(value: NewIngestJob) -> Self {
        Self {
            id: value.id,
            org_id: value.org_id,
            user_id: value.user_id,
            project_id: value.project_id,
            status: JOB_STATUS_QUEUED.to_string(),
            payload: value.payload,
            attempts: 0,
            error: None,
            version_id: None,
            created_at: value.created_at,
            updated_at: value.updated_at,
        }
    }
}

impl From<ingest_jobs::Model> for IngestJobRecord {
    fn from(value: ingest_jobs::Model) -> Self {
        Self {
            id: value.id,
            org_id: value.org_id,
            user_id: value.user_id,
            project_id: value.project_id,
            status: value.status,
            payload: value.payload,
            attempts: value.attempts,
            error: value.error,
            version_id: value.version_id,
            created_at: value.created_at,
            updated_at: value.updated_at,
        }
    }
}

impl From<worker_status::Model> for WorkerStatusRecord {
    fn from(value: worker_status::Model) -> Self {
        Self {
            id: value.id,
            version: value.version,
            updated_at: value.updated_at,
        }
    }
}

async fn ensure_indexing_config_exists_with<C: ConnectionTrait>(
    conn: &C,
    org_id: &str,
    config_id: &str,
) -> CocoResult<()> {
    ensure_canonical_config_id(config_id)?;
    let model = indexing_configs::Entity::find()
        .filter(indexing_configs::Column::OrgId.eq(org_id))
        .filter(indexing_configs::Column::ConfigId.eq(config_id))
        .one(conn)
        .await
        .map_err(map_storage_err)?;
    if model.is_none() {
        return Err(CocoError::user("indexing config not found"));
    }
    Ok(())
}

fn validate_indexing_config_constraints(config: &IndexingConfig) -> CocoResult<()> {
    let Some(dimensions) = config.embedding.dimensions else {
        return Err(CocoError::user(
            "embedding dimensions must be set for server indexing configs",
        ));
    };
    if dimensions != DEFAULT_EMBEDDING_DIM {
        return Err(CocoError::user(
            "embedding dimensions must match server vector index",
        ));
    }
    if config.vector_metric != DEFAULT_VECTOR_METRIC {
        return Err(CocoError::user(
            "vector_metric must match server vector index",
        ));
    }
    Ok(())
}

fn ensure_canonical_config_id(config_id: &str) -> CocoResult<()> {
    let normalized = normalize_config_id(config_id)?;
    if normalized != config_id {
        return Err(CocoError::user("config_id must be normalized"));
    }
    Ok(())
}

fn vector_metric_to_string(metric: VectorMetric) -> String {
    match metric {
        VectorMetric::Cosine => "cosine",
        VectorMetric::Dot => "dot",
        VectorMetric::L2 => "l2",
    }
    .to_string()
}

fn vector_metric_from_string(value: &str) -> CocoResult<VectorMetric> {
    match value {
        "cosine" => Ok(VectorMetric::Cosine),
        "dot" => Ok(VectorMetric::Dot),
        "l2" => Ok(VectorMetric::L2),
        _ => Err(CocoError::storage(format!(
            "invalid vector_metric value: {value}",
        ))),
    }
}

fn serialize_json<T: serde::Serialize>(value: &T, field: &str) -> CocoResult<String> {
    serde_json::to_string(value)
        .map_err(|err| CocoError::storage(format!("invalid {field} value: {err}")))
}

fn optional_json<T: serde::Serialize>(
    value: &Option<T>,
    field: &str,
) -> CocoResult<Option<String>> {
    value
        .as_ref()
        .map(|value| serialize_json(value, field))
        .transpose()
}

fn deserialize_json<T: serde::de::DeserializeOwned>(value: &str, field: &str) -> CocoResult<T> {
    serde_json::from_str(value)
        .map_err(|err| CocoError::storage(format!("invalid {field} value: {err}")))
}

fn optional_deserialize_json<T: serde::de::DeserializeOwned>(
    value: Option<String>,
    field: &str,
) -> CocoResult<Option<T>> {
    value
        .map(|value| deserialize_json(&value, field))
        .transpose()
}

fn generate_version_id() -> CocoResult<String> {
    let now = std::time::SystemTime::now()
        .duration_since(std::time::UNIX_EPOCH)
        .map_err(|_| CocoError::system("system clock before UNIX epoch"))?;
    Ok(format!("v{}", now.as_millis()))
}

fn now() -> sea_orm::prelude::DateTimeUtc {
    Utc::now()
}

fn map_storage_err<E: std::fmt::Display>(error: E) -> CocoError {
    CocoError::storage(error)
}
