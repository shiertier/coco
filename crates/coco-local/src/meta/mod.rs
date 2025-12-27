//! SQLite-backed metadata store for local mode.

use std::collections::HashSet;
use std::path::Path;

use chrono::{DateTime, Utc};
use coco_core::{normalize_config_id, validate_indexing_config};
use coco_protocol::{
    ChunkingStrategy, CocoError, CocoResult, EmbeddingConfig, IndexingConfig, ValidationContext,
    VectorBackendConfig, VectorIndexParams, VectorMetric,
};
use sea_orm::prelude::Expr;
use sea_orm::{
    ActiveModelTrait, ColumnTrait, ConnectionTrait, Database, DatabaseConnection,
    DatabaseTransaction, EntityTrait, PaginatorTrait, QueryFilter, QueryOrder, Set,
    TransactionTrait,
};
use sea_orm_migration::MigratorTrait;

mod entities;
mod migration;

use entities::{chunks, documents, indexing_configs, project_versions, projects};

type Timestamp = DateTime<Utc>;

/// Default indexing configuration identifier.
pub const DEFAULT_CONFIG_ID: &str = "default";
const STATUS_BUILDING: &str = "BUILDING";
const STATUS_ACTIVE: &str = "ACTIVE";
const STATUS_ARCHIVED: &str = "ARCHIVED";

/// Local metadata store backed by SQLite.
#[derive(Clone)]
pub struct LocalMetaStore {
    db: DatabaseConnection,
}

impl LocalMetaStore {
    /// Connects to a SQLite database at the given path and runs migrations.
    pub async fn connect(path: impl AsRef<Path>) -> CocoResult<Self> {
        let path = path.as_ref();
        if let Some(parent) = path.parent() {
            std::fs::create_dir_all(parent).map_err(CocoError::system)?;
        }
        let url = sqlite_url(path)?;
        Self::connect_url(&url).await
    }

    /// Connects to the provided database URL and runs migrations.
    pub async fn connect_url(url: &str) -> CocoResult<Self> {
        let db = Database::connect(url).await.map_err(map_storage_err)?;
        let store = Self { db };
        store.run_migrations().await?;
        Ok(store)
    }

    /// Creates an in-memory SQLite metadata store.
    pub async fn connect_in_memory() -> CocoResult<Self> {
        Self::connect_url("sqlite::memory:").await
    }

    /// Returns the underlying database connection.
    pub fn connection(&self) -> &DatabaseConnection {
        &self.db
    }

    /// Begins a new database transaction.
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
        if protocol_config.vector_backend.is_some() {
            return Err(CocoError::user(
                "vector_backend is not supported in local mode",
            ));
        }

        let exists = indexing_configs::Entity::find_by_id(&config.config_id)
            .one(&self.db)
            .await
            .map_err(map_storage_err)?;
        if exists.is_some() {
            return Err(CocoError::user("indexing config already exists"));
        }

        let record = IndexingConfigRecord::from(config);
        let active_model = indexing_configs::ActiveModel {
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
        if let Some(existing) = self.get_indexing_config(&config.config_id).await? {
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

    /// Fetches an indexing configuration by id.
    pub async fn get_indexing_config(
        &self,
        config_id: &str,
    ) -> CocoResult<Option<IndexingConfigRecord>> {
        ensure_canonical_config_id(config_id)?;
        let model = indexing_configs::Entity::find_by_id(config_id)
            .one(&self.db)
            .await
            .map_err(map_storage_err)?;
        model.map(IndexingConfigRecord::try_from).transpose()
    }

    /// Lists all indexing configurations.
    pub async fn list_indexing_configs(&self) -> CocoResult<Vec<IndexingConfigRecord>> {
        let models = indexing_configs::Entity::find()
            .all(&self.db)
            .await
            .map_err(map_storage_err)?;
        models
            .into_iter()
            .map(IndexingConfigRecord::try_from)
            .collect()
    }

    /// Ensures an indexing configuration exists.
    pub async fn ensure_indexing_config_exists(&self, config_id: &str) -> CocoResult<()> {
        ensure_indexing_config_exists_with(&self.db, config_id).await
    }

    /// Ensures a project version exists.
    pub async fn ensure_project_version_exists(
        &self,
        project_id: &str,
        version_id: &str,
    ) -> CocoResult<()> {
        ensure_project_version_exists_with(&self.db, project_id, version_id).await
    }

    /// Updates an existing indexing configuration if it has not been referenced.
    pub async fn update_indexing_config(
        &self,
        config: NewIndexingConfig,
    ) -> CocoResult<IndexingConfigRecord> {
        if config.config_id == DEFAULT_CONFIG_ID {
            return Err(CocoError::user(
                "default indexing config cannot be updated",
            ));
        }
        let protocol_config = config.to_protocol();
        validate_indexing_config(&protocol_config, &ValidationContext::default())?;
        if protocol_config.vector_backend.is_some() {
            return Err(CocoError::user(
                "vector_backend is not supported in local mode",
            ));
        }

        let txn = self.db.begin().await.map_err(map_storage_err)?;
        let model = indexing_configs::Entity::find_by_id(&config.config_id)
            .one(&txn)
            .await
            .map_err(map_storage_err)?;
        let Some(model) = model else {
            txn.rollback().await.map_err(map_storage_err)?;
            return Err(CocoError::user("indexing config not found"));
        };

        let doc_refs = documents::Entity::find()
            .filter(documents::Column::ConfigId.eq(&config.config_id))
            .count(&txn)
            .await
            .map_err(map_storage_err)?;
        let chunk_refs = chunks::Entity::find()
            .filter(chunks::Column::ConfigId.eq(&config.config_id))
            .count(&txn)
            .await
            .map_err(map_storage_err)?;
        let project_refs = projects::Entity::find()
            .filter(projects::Column::ActiveConfigId.eq(&config.config_id))
            .count(&txn)
            .await
            .map_err(map_storage_err)?;
        if doc_refs > 0 || chunk_refs > 0 || project_refs > 0 {
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

    /// Creates a new project record.
    pub async fn create_project(&self, project: NewProject) -> CocoResult<ProjectRecord> {
        self.ensure_indexing_config_exists(&project.active_config_id)
            .await?;
        let mut project = project;
        let active_version_id = match project.active_version_id.as_deref() {
            Some(value) if !value.trim().is_empty() => value.to_string(),
            _ => generate_version_id()?,
        };
        project.active_version_id = Some(active_version_id.clone());

        let txn = self.db.begin().await.map_err(map_storage_err)?;
        let active_model = projects::ActiveModel {
            id: Set(project.id.clone()),
            name: Set(project.name.clone()),
            path: Set(project.path.clone()),
            created_at: Set(project.created_at),
            active_version_id: Set(project.active_version_id.clone()),
            active_config_id: Set(project.active_config_id.clone()),
        };
        projects::Entity::insert(active_model)
            .exec(&txn)
            .await
            .map_err(map_storage_err)?;

        let version_model = project_versions::ActiveModel {
            id: Set(active_version_id),
            project_id: Set(project.id.clone()),
            status: Set(STATUS_ACTIVE.to_string()),
            created_at: Set(project.created_at),
            item_count: Set(0),
        };
        project_versions::Entity::insert(version_model)
            .exec(&txn)
            .await
            .map_err(map_storage_err)?;
        txn.commit().await.map_err(map_storage_err)?;

        Ok(ProjectRecord::from(project))
    }

    /// Fetches a project by id.
    pub async fn get_project(&self, project_id: &str) -> CocoResult<Option<ProjectRecord>> {
        let model = projects::Entity::find_by_id(project_id)
            .one(&self.db)
            .await
            .map_err(map_storage_err)?;
        Ok(model.map(ProjectRecord::from))
    }

    /// Lists all projects.
    pub async fn list_projects(&self) -> CocoResult<Vec<ProjectRecord>> {
        let models = projects::Entity::find()
            .all(&self.db)
            .await
            .map_err(map_storage_err)?;
        Ok(models.into_iter().map(ProjectRecord::from).collect())
    }

    /// Counts all projects.
    pub async fn count_projects(&self) -> CocoResult<u64> {
        projects::Entity::find()
            .count(&self.db)
            .await
            .map_err(map_storage_err)
    }

    /// Updates fields on an existing project.
    pub async fn update_project(
        &self,
        project_id: &str,
        update: ProjectUpdate,
    ) -> CocoResult<Option<ProjectRecord>> {
        let txn = self.db.begin().await.map_err(map_storage_err)?;
        let model = projects::Entity::find_by_id(project_id)
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
        if let Some(path) = update.path {
            active_model.path = Set(path);
        }
        if let Some(active_version_id) = update.active_version_id {
            if let Some(version_id) = active_version_id.as_deref() {
                ensure_project_version_exists_with(&txn, project_id, version_id).await?;
            }
            active_model.active_version_id = Set(active_version_id);
        }
        if let Some(active_config_id) = update.active_config_id {
            ensure_indexing_config_exists_with(&txn, &active_config_id).await?;
            active_model.active_config_id = Set(active_config_id);
        }
        let updated = active_model.update(&txn).await.map_err(map_storage_err)?;
        txn.commit().await.map_err(map_storage_err)?;
        Ok(Some(ProjectRecord::from(updated)))
    }

    /// Ensures the project has an active version id and returns it.
    pub async fn ensure_active_version_id(&self, project_id: &str) -> CocoResult<String> {
        let project = self
            .get_project(project_id)
            .await?
            .ok_or_else(|| CocoError::user("project not found"))?;
        if let Some(active) = project
            .active_version_id
            .as_deref()
            .filter(|value| !value.trim().is_empty())
        {
            let existing = project_versions::Entity::find_by_id(active)
                .filter(project_versions::Column::ProjectId.eq(project_id))
                .one(&self.db)
                .await
                .map_err(map_storage_err)?;
            if existing.is_none() {
                let created_at = project.created_at;
                let model = project_versions::ActiveModel {
                    id: Set(active.to_string()),
                    project_id: Set(project_id.to_string()),
                    status: Set(STATUS_ACTIVE.to_string()),
                    created_at: Set(created_at),
                    item_count: Set(0),
                };
                project_versions::Entity::insert(model)
                    .exec(&self.db)
                    .await
                    .map_err(map_storage_err)?;
            }
            return Ok(active.to_string());
        }

        let version_id = generate_version_id()?;
        let created_at = Utc::now();
        let txn = self.db.begin().await.map_err(map_storage_err)?;
        let version_model = project_versions::ActiveModel {
            id: Set(version_id.clone()),
            project_id: Set(project_id.to_string()),
            status: Set(STATUS_ACTIVE.to_string()),
            created_at: Set(created_at),
            item_count: Set(0),
        };
        project_versions::Entity::insert(version_model)
            .exec(&txn)
            .await
            .map_err(map_storage_err)?;

        projects::Entity::update_many()
            .filter(projects::Column::Id.eq(project_id))
            .col_expr(
                projects::Column::ActiveVersionId,
                Expr::value(version_id.clone()),
            )
            .exec(&txn)
            .await
            .map_err(map_storage_err)?;
        txn.commit().await.map_err(map_storage_err)?;

        Ok(version_id)
    }

    /// Creates a new project version in BUILDING status.
    pub async fn create_project_version(&self, project_id: &str) -> CocoResult<ProjectVersionRecord> {
        if project_id.trim().is_empty() {
            return Err(CocoError::user("project_id must not be empty"));
        }
        let exists = projects::Entity::find_by_id(project_id)
            .one(&self.db)
            .await
            .map_err(map_storage_err)?;
        if exists.is_none() {
            return Err(CocoError::user("project not found"));
        }
        let version_id = generate_version_id()?;
        let created_at = Utc::now();
        let model = project_versions::ActiveModel {
            id: Set(version_id.clone()),
            project_id: Set(project_id.to_string()),
            status: Set(STATUS_BUILDING.to_string()),
            created_at: Set(created_at),
            item_count: Set(0),
        };
        project_versions::Entity::insert(model)
            .exec(&self.db)
            .await
            .map_err(map_storage_err)?;
        Ok(ProjectVersionRecord {
            id: version_id,
            project_id: project_id.to_string(),
            status: STATUS_BUILDING.to_string(),
            created_at,
            item_count: 0,
        })
    }

    /// Activates a project version and archives all others.
    pub async fn activate_project_version(
        &self,
        project_id: &str,
        version_id: &str,
    ) -> CocoResult<()> {
        let txn = self.db.begin().await.map_err(map_storage_err)?;
        let version = project_versions::Entity::find_by_id(version_id)
            .filter(project_versions::Column::ProjectId.eq(project_id))
            .one(&txn)
            .await
            .map_err(map_storage_err)?;
        let Some(_version) = version else {
            txn.rollback().await.map_err(map_storage_err)?;
            return Err(CocoError::user("project version not found"));
        };

        let item_count = chunks::Entity::find()
            .filter(chunks::Column::VersionId.eq(version_id))
            .count(&txn)
            .await
            .map_err(map_storage_err)?;
        let item_count = i64::try_from(item_count)
            .map_err(|_| CocoError::storage("item_count exceeds i64 range"))?;

        projects::Entity::update_many()
            .filter(projects::Column::Id.eq(project_id))
            .col_expr(
                projects::Column::ActiveVersionId,
                Expr::value(version_id.to_string()),
            )
            .exec(&txn)
            .await
            .map_err(map_storage_err)?;

        project_versions::Entity::update_many()
            .filter(project_versions::Column::ProjectId.eq(project_id))
            .col_expr(
                project_versions::Column::Status,
                Expr::value(STATUS_ARCHIVED.to_string()),
            )
            .exec(&txn)
            .await
            .map_err(map_storage_err)?;

        project_versions::Entity::update_many()
            .filter(project_versions::Column::ProjectId.eq(project_id))
            .filter(project_versions::Column::Id.eq(version_id))
            .col_expr(
                project_versions::Column::Status,
                Expr::value(STATUS_ACTIVE.to_string()),
            )
            .col_expr(
                project_versions::Column::ItemCount,
                Expr::value(item_count),
            )
            .exec(&txn)
            .await
            .map_err(map_storage_err)?;

        txn.commit().await.map_err(map_storage_err)?;
        Ok(())
    }

    /// Archives older versions and returns the archived version ids.
    pub async fn gc_project_versions(
        &self,
        project_id: &str,
        keep: usize,
    ) -> CocoResult<Vec<String>> {
        if keep == 0 {
            return Err(CocoError::user("keep must be greater than zero"));
        }
        let active_version_id = self.ensure_active_version_id(project_id).await?;

        let versions = project_versions::Entity::find()
            .filter(project_versions::Column::ProjectId.eq(project_id))
            .order_by_desc(project_versions::Column::CreatedAt)
            .all(&self.db)
            .await
            .map_err(map_storage_err)?;

        let mut candidates = Vec::new();
        for version in versions {
            if version.id == active_version_id {
                continue;
            }
            if version.status == STATUS_BUILDING {
                continue;
            }
            candidates.push(version);
        }

        if candidates.len() <= keep {
            return Ok(Vec::new());
        }

        let archive_ids: Vec<String> = candidates
            .into_iter()
            .skip(keep)
            .map(|version| version.id)
            .collect();
        if archive_ids.is_empty() {
            return Ok(Vec::new());
        }

        project_versions::Entity::update_many()
            .filter(project_versions::Column::ProjectId.eq(project_id))
            .filter(project_versions::Column::Id.is_in(archive_ids.clone()))
            .col_expr(
                project_versions::Column::Status,
                Expr::value(STATUS_ARCHIVED.to_string()),
            )
            .exec(&self.db)
            .await
            .map_err(map_storage_err)?;

        chunks::Entity::delete_many()
            .filter(chunks::Column::VersionId.is_in(archive_ids.clone()))
            .exec(&self.db)
            .await
            .map_err(map_storage_err)?;
        documents::Entity::delete_many()
            .filter(documents::Column::VersionId.is_in(archive_ids.clone()))
            .exec(&self.db)
            .await
            .map_err(map_storage_err)?;

        Ok(archive_ids)
    }

    /// Deletes a project by id.
    pub async fn delete_project(&self, project_id: &str) -> CocoResult<()> {
        projects::Entity::delete_by_id(project_id)
            .exec(&self.db)
            .await
            .map_err(map_storage_err)?;
        Ok(())
    }

    /// Creates a new document record.
    pub async fn create_document(&self, document: NewDocument) -> CocoResult<DocumentRecord> {
        self.ensure_indexing_config_exists(&document.config_id)
            .await?;
        ensure_project_version_exists_with(&self.db, &document.project_id, &document.version_id)
            .await?;
        let active_model = documents::ActiveModel {
            id: Set(document.id.clone()),
            project_id: Set(document.project_id.clone()),
            version_id: Set(document.version_id.clone()),
            config_id: Set(document.config_id.clone()),
            path: Set(document.path.clone()),
            title: Set(document.title.clone()),
            content_hash: Set(document.content_hash.clone()),
            indexed_at: Set(document.indexed_at),
        };
        documents::Entity::insert(active_model)
            .exec(&self.db)
            .await
            .map_err(map_storage_err)?;
        Ok(DocumentRecord::from(document))
    }

    /// Fetches a document by id.
    pub async fn get_document(&self, document_id: &str) -> CocoResult<Option<DocumentRecord>> {
        let model = documents::Entity::find_by_id(document_id)
            .one(&self.db)
            .await
            .map_err(map_storage_err)?;
        Ok(model.map(DocumentRecord::from))
    }

    /// Fetches a document by project and path.
    pub async fn get_document_by_path(
        &self,
        project_id: &str,
        version_id: &str,
        path: &str,
    ) -> CocoResult<Option<DocumentRecord>> {
        ensure_project_version_exists_with(&self.db, project_id, version_id).await?;
        let model = documents::Entity::find()
            .filter(documents::Column::ProjectId.eq(project_id))
            .filter(documents::Column::VersionId.eq(version_id))
            .filter(documents::Column::Path.eq(path))
            .one(&self.db)
            .await
            .map_err(map_storage_err)?;
        Ok(model.map(DocumentRecord::from))
    }

    /// Lists documents under the given project.
    pub async fn list_documents(
        &self,
        project_id: &str,
        version_id: &str,
    ) -> CocoResult<Vec<DocumentRecord>> {
        ensure_project_version_exists_with(&self.db, project_id, version_id).await?;
        let models = documents::Entity::find()
            .filter(documents::Column::ProjectId.eq(project_id))
            .filter(documents::Column::VersionId.eq(version_id))
            .all(&self.db)
            .await
            .map_err(map_storage_err)?;
        Ok(models.into_iter().map(DocumentRecord::from).collect())
    }

    /// Counts all documents.
    pub async fn count_documents(&self) -> CocoResult<u64> {
        documents::Entity::find()
            .count(&self.db)
            .await
            .map_err(map_storage_err)
    }

    /// Updates fields on an existing document.
    pub async fn update_document(
        &self,
        document_id: &str,
        update: DocumentUpdate,
    ) -> CocoResult<Option<DocumentRecord>> {
        let model = documents::Entity::find_by_id(document_id)
            .one(&self.db)
            .await
            .map_err(map_storage_err)?;
        let Some(model) = model else {
            return Ok(None);
        };
        let mut active_model: documents::ActiveModel = model.into();
        if let Some(path) = update.path {
            active_model.path = Set(path);
        }
        if let Some(title) = update.title {
            active_model.title = Set(title);
        }
        if let Some(content_hash) = update.content_hash {
            active_model.content_hash = Set(content_hash);
        }
        if let Some(indexed_at) = update.indexed_at {
            active_model.indexed_at = Set(indexed_at);
        }
        let updated = active_model
            .update(&self.db)
            .await
            .map_err(map_storage_err)?;
        Ok(Some(DocumentRecord::from(updated)))
    }

    /// Deletes a document by id.
    pub async fn delete_document(&self, document_id: &str) -> CocoResult<()> {
        documents::Entity::delete_by_id(document_id)
            .exec(&self.db)
            .await
            .map_err(map_storage_err)?;
        Ok(())
    }

    /// Deletes all documents associated with a project.
    pub async fn delete_documents_by_project(&self, project_id: &str) -> CocoResult<()> {
        documents::Entity::delete_many()
            .filter(documents::Column::ProjectId.eq(project_id))
            .exec(&self.db)
            .await
            .map_err(map_storage_err)?;
        Ok(())
    }

    /// Creates chunk records in bulk.
    pub async fn create_chunks(&self, chunks: Vec<NewChunk>) -> CocoResult<Vec<ChunkRecord>> {
        if chunks.is_empty() {
            return Ok(Vec::new());
        }
        let mut seen = HashSet::new();
        for chunk in &chunks {
            if seen.insert(chunk.config_id.clone()) {
                self.ensure_indexing_config_exists(&chunk.config_id)
                    .await?;
            }
        }
        let mut models = Vec::with_capacity(chunks.len());
        let mut records = Vec::with_capacity(chunks.len());
        for chunk in chunks {
            let record = ChunkRecord::from(chunk);
            models.push(chunks::ActiveModel {
                id: Set(record.id.clone()),
                doc_id: Set(record.doc_id.clone()),
                version_id: Set(record.version_id.clone()),
                config_id: Set(record.config_id.clone()),
                content: Set(record.content.clone()),
                start_line: Set(record.start_line),
                end_line: Set(record.end_line),
            });
            records.push(record);
        }
        chunks::Entity::insert_many(models)
            .exec(&self.db)
            .await
            .map_err(map_storage_err)?;
        Ok(records)
    }

    /// Fetches a chunk by id.
    pub async fn get_chunk(&self, chunk_id: &str) -> CocoResult<Option<ChunkRecord>> {
        let model = chunks::Entity::find_by_id(chunk_id)
            .one(&self.db)
            .await
            .map_err(map_storage_err)?;
        Ok(model.map(ChunkRecord::from))
    }

    /// Lists chunks under the given document.
    pub async fn list_chunks(&self, document_id: &str) -> CocoResult<Vec<ChunkRecord>> {
        let models = chunks::Entity::find()
            .filter(chunks::Column::DocId.eq(document_id))
            .all(&self.db)
            .await
            .map_err(map_storage_err)?;
        Ok(models.into_iter().map(ChunkRecord::from).collect())
    }

    /// Counts all chunks.
    pub async fn count_chunks(&self) -> CocoResult<u64> {
        chunks::Entity::find()
            .count(&self.db)
            .await
            .map_err(map_storage_err)
    }

    /// Updates fields on an existing chunk.
    pub async fn update_chunk(
        &self,
        chunk_id: &str,
        update: ChunkUpdate,
    ) -> CocoResult<Option<ChunkRecord>> {
        let model = chunks::Entity::find_by_id(chunk_id)
            .one(&self.db)
            .await
            .map_err(map_storage_err)?;
        let Some(model) = model else {
            return Ok(None);
        };
        let mut active_model: chunks::ActiveModel = model.into();
        if let Some(content) = update.content {
            active_model.content = Set(content);
        }
        if let Some(start_line) = update.start_line {
            active_model.start_line = Set(start_line);
        }
        if let Some(end_line) = update.end_line {
            active_model.end_line = Set(end_line);
        }
        let updated = active_model
            .update(&self.db)
            .await
            .map_err(map_storage_err)?;
        Ok(Some(ChunkRecord::from(updated)))
    }

    /// Deletes a chunk by id.
    pub async fn delete_chunk(&self, chunk_id: &str) -> CocoResult<()> {
        chunks::Entity::delete_by_id(chunk_id)
            .exec(&self.db)
            .await
            .map_err(map_storage_err)?;
        Ok(())
    }

    /// Deletes all chunks associated with a document.
    pub async fn delete_chunks_by_doc(&self, document_id: &str) -> CocoResult<()> {
        chunks::Entity::delete_many()
            .filter(chunks::Column::DocId.eq(document_id))
            .exec(&self.db)
            .await
            .map_err(map_storage_err)?;
        Ok(())
    }

    async fn run_migrations(&self) -> CocoResult<()> {
        migration::Migrator::up(&self.db, None)
            .await
            .map_err(map_storage_err)
    }
}

/// Input data for creating a project.
#[derive(Debug, Clone)]
pub struct NewProject {
    /// Project identifier.
    pub id: String,
    /// Human-readable project name.
    pub name: String,
    /// Absolute path to the project root.
    pub path: String,
    /// Creation timestamp.
    pub created_at: Timestamp,
    /// Optional active version identifier.
    pub active_version_id: Option<String>,
    /// Active indexing configuration identifier.
    pub active_config_id: String,
}

/// Stored project record.
#[derive(Debug, Clone)]
pub struct ProjectRecord {
    /// Project identifier.
    pub id: String,
    /// Human-readable project name.
    pub name: String,
    /// Absolute path to the project root.
    pub path: String,
    /// Creation timestamp.
    pub created_at: Timestamp,
    /// Optional active version identifier.
    pub active_version_id: Option<String>,
    /// Active indexing configuration identifier.
    pub active_config_id: String,
}

/// Project fields that can be updated.
#[derive(Debug, Clone, Default)]
pub struct ProjectUpdate {
    /// Updated name, if present.
    pub name: Option<String>,
    /// Updated path, if present.
    pub path: Option<String>,
    /// Updated active version id (use `Some(None)` to clear).
    pub active_version_id: Option<Option<String>>,
    /// Updated active config id, if present.
    pub active_config_id: Option<String>,
}

/// Stored project version record.
#[derive(Debug, Clone)]
pub struct ProjectVersionRecord {
    /// Version identifier.
    pub id: String,
    /// Owning project identifier.
    pub project_id: String,
    /// Version status.
    pub status: String,
    /// Creation timestamp.
    pub created_at: Timestamp,
    /// Count of indexed items.
    pub item_count: i64,
}

/// Input data for creating an indexing configuration.
#[derive(Debug, Clone)]
pub struct NewIndexingConfig {
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
    pub created_at: Timestamp,
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
    pub created_at: Timestamp,
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
    /// Owning project identifier.
    pub project_id: String,
    /// Project version identifier.
    pub version_id: String,
    /// Indexing configuration identifier.
    pub config_id: String,
    /// Document path relative to the project root.
    pub path: String,
    /// Optional document title.
    pub title: Option<String>,
    /// Content hash for change detection.
    pub content_hash: String,
    /// Indexing timestamp.
    pub indexed_at: Timestamp,
}

/// Stored document record.
#[derive(Debug, Clone)]
pub struct DocumentRecord {
    /// Document identifier.
    pub id: String,
    /// Owning project identifier.
    pub project_id: String,
    /// Project version identifier.
    pub version_id: String,
    /// Indexing configuration identifier.
    pub config_id: String,
    /// Document path relative to the project root.
    pub path: String,
    /// Optional document title.
    pub title: Option<String>,
    /// Content hash for change detection.
    pub content_hash: String,
    /// Indexing timestamp.
    pub indexed_at: Timestamp,
}

/// Document fields that can be updated.
#[derive(Debug, Clone, Default)]
pub struct DocumentUpdate {
    /// Updated path, if present.
    pub path: Option<String>,
    /// Updated title (use `Some(None)` to clear).
    pub title: Option<Option<String>>,
    /// Updated content hash, if present.
    pub content_hash: Option<String>,
    /// Updated indexing timestamp, if present.
    pub indexed_at: Option<Timestamp>,
}

/// Input data for creating a chunk.
#[derive(Debug, Clone)]
pub struct NewChunk {
    /// Chunk identifier.
    pub id: String,
    /// Owning document identifier.
    pub doc_id: String,
    /// Project version identifier.
    pub version_id: String,
    /// Indexing configuration identifier.
    pub config_id: String,
    /// Chunk content.
    pub content: String,
    /// First line number of the chunk.
    pub start_line: i32,
    /// Last line number of the chunk.
    pub end_line: i32,
}

/// Stored chunk record.
#[derive(Debug, Clone)]
pub struct ChunkRecord {
    /// Chunk identifier.
    pub id: String,
    /// Owning document identifier.
    pub doc_id: String,
    /// Project version identifier.
    pub version_id: String,
    /// Indexing configuration identifier.
    pub config_id: String,
    /// Chunk content.
    pub content: String,
    /// First line number of the chunk.
    pub start_line: i32,
    /// Last line number of the chunk.
    pub end_line: i32,
}

/// Chunk fields that can be updated.
#[derive(Debug, Clone, Default)]
pub struct ChunkUpdate {
    /// Updated content, if present.
    pub content: Option<String>,
    /// Updated start line, if present.
    pub start_line: Option<i32>,
    /// Updated end line, if present.
    pub end_line: Option<i32>,
}

impl From<NewProject> for ProjectRecord {
    fn from(value: NewProject) -> Self {
        Self {
            id: value.id,
            name: value.name,
            path: value.path,
            created_at: value.created_at,
            active_version_id: value.active_version_id,
            active_config_id: value.active_config_id,
        }
    }
}

impl From<projects::Model> for ProjectRecord {
    fn from(value: projects::Model) -> Self {
        Self {
            id: value.id,
            name: value.name,
            path: value.path,
            created_at: value.created_at,
            active_version_id: value.active_version_id,
            active_config_id: value.active_config_id,
        }
    }
}

impl From<project_versions::Model> for ProjectVersionRecord {
    fn from(value: project_versions::Model) -> Self {
        Self {
            id: value.id,
            project_id: value.project_id,
            status: value.status,
            created_at: value.created_at,
            item_count: value.item_count,
        }
    }
}

impl From<NewIndexingConfig> for IndexingConfigRecord {
    fn from(value: NewIndexingConfig) -> Self {
        Self {
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

impl From<NewDocument> for DocumentRecord {
    fn from(value: NewDocument) -> Self {
        Self {
            id: value.id,
            project_id: value.project_id,
            version_id: value.version_id,
            config_id: value.config_id,
            path: value.path,
            title: value.title,
            content_hash: value.content_hash,
            indexed_at: value.indexed_at,
        }
    }
}

impl From<documents::Model> for DocumentRecord {
    fn from(value: documents::Model) -> Self {
        Self {
            id: value.id,
            project_id: value.project_id,
            version_id: value.version_id,
            config_id: value.config_id,
            path: value.path,
            title: value.title,
            content_hash: value.content_hash,
            indexed_at: value.indexed_at,
        }
    }
}

impl From<NewChunk> for ChunkRecord {
    fn from(value: NewChunk) -> Self {
        Self {
            id: value.id,
            doc_id: value.doc_id,
            version_id: value.version_id,
            config_id: value.config_id,
            content: value.content,
            start_line: value.start_line,
            end_line: value.end_line,
        }
    }
}

impl From<chunks::Model> for ChunkRecord {
    fn from(value: chunks::Model) -> Self {
        Self {
            id: value.id,
            doc_id: value.doc_id,
            version_id: value.version_id,
            config_id: value.config_id,
            content: value.content,
            start_line: value.start_line,
            end_line: value.end_line,
        }
    }
}

async fn ensure_indexing_config_exists_with<C: ConnectionTrait>(
    conn: &C,
    config_id: &str,
) -> CocoResult<()> {
    ensure_canonical_config_id(config_id)?;
    let model = indexing_configs::Entity::find_by_id(config_id)
        .one(conn)
        .await
        .map_err(map_storage_err)?;
    if model.is_none() {
        return Err(CocoError::user("indexing config not found"));
    }
    Ok(())
}

async fn ensure_project_version_exists_with<C: ConnectionTrait>(
    conn: &C,
    project_id: &str,
    version_id: &str,
) -> CocoResult<()> {
    if project_id.trim().is_empty() {
        return Err(CocoError::user("project_id must not be empty"));
    }
    if version_id.trim().is_empty() {
        return Err(CocoError::user("version_id must not be empty"));
    }
    let model = project_versions::Entity::find_by_id(version_id)
        .filter(project_versions::Column::ProjectId.eq(project_id))
        .one(conn)
        .await
        .map_err(map_storage_err)?;
    if model.is_none() {
        return Err(CocoError::user("project version not found"));
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

fn deserialize_json<T: serde::de::DeserializeOwned>(
    value: &str,
    field: &str,
) -> CocoResult<T> {
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
    Ok(format!("v{}", uuid::Uuid::now_v7()))
}

fn sqlite_url(path: &Path) -> CocoResult<String> {
    let path_str = path
        .to_str()
        .ok_or_else(|| CocoError::user("sqlite path must be valid UTF-8"))?;
    Ok(format!("sqlite://{}?mode=rwc", path_str))
}

fn map_storage_err<E: std::fmt::Display>(error: E) -> CocoError {
    CocoError::storage(error)
}

#[cfg(test)]
mod tests {
    use super::*;
    use sea_orm::Database;

    fn sample_config(config_id: &str) -> NewIndexingConfig {
        NewIndexingConfig {
            config_id: config_id.to_string(),
            chunking: ChunkingStrategy {
                strategy_name: "fixed_token".to_string(),
                chunk_size: 256,
                chunk_overlap: 32,
            },
            embedding: EmbeddingConfig {
                model_name: "all-MiniLM-L6-v2".to_string(),
                dimensions: Some(384),
            },
            vector_backend: None,
            vector_metric: VectorMetric::Cosine,
            index_params: None,
            created_at: Utc::now(),
        }
    }

    async fn store_without_migrations() -> LocalMetaStore {
        let db = Database::connect("sqlite::memory:")
            .await
            .expect("connect sqlite");
        LocalMetaStore { db }
    }

    #[tokio::test]
    async fn default_indexing_config_requires_default_id() {
        let store = store_without_migrations().await;
        let err = store
            .ensure_default_indexing_config(sample_config("custom"))
            .await
            .expect_err("default config must use config_id=default");
        assert!(err
            .public_message()
            .contains("config_id=default"));
    }

    #[tokio::test]
    async fn default_indexing_config_is_immutable() {
        let store = store_without_migrations().await;
        let err = store
            .update_indexing_config(sample_config(DEFAULT_CONFIG_ID))
            .await
            .expect_err("default config cannot be updated");
        assert!(err.public_message().contains("cannot be updated"));
    }
}
