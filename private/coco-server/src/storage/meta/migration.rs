use sea_orm::{ConnectionTrait, DatabaseBackend, Statement};
use sea_orm_migration::prelude::*;

use super::DEFAULT_EMBEDDING_DIM;

const DEFAULT_CONFIG_ID: &str = "default";

pub(crate) struct Migrator;

#[async_trait::async_trait]
impl MigratorTrait for Migrator {
    fn migrations() -> Vec<Box<dyn MigrationTrait>> {
        vec![
            Box::new(CreateServerMetaTables),
            Box::new(AddQualityColumns),
            Box::new(AddWorkerStatusTable),
            Box::new(AddServerConfigIdColumn),
            Box::new(AddServerConfigIdColumns),
            Box::new(CreateServerIndexingConfigs),
            Box::new(AddUserIdColumns),
        ]
    }
}

#[derive(DeriveMigrationName)]
pub(crate) struct CreateServerMetaTables;

#[async_trait::async_trait]
impl MigrationTrait for CreateServerMetaTables {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .create_table(
                Table::create()
                    .table(Organizations::Table)
                    .if_not_exists()
                    .col(
                        ColumnDef::new(Organizations::Id)
                            .string()
                            .not_null()
                            .primary_key(),
                    )
                    .col(ColumnDef::new(Organizations::Name).string().not_null())
                    .col(ColumnDef::new(Organizations::CreatedAt).timestamp().not_null())
                    .col(ColumnDef::new(Organizations::MaxDocuments).big_integer())
                    .col(ColumnDef::new(Organizations::MaxChunks).big_integer())
                    .col(ColumnDef::new(Organizations::MaxStorageBytes).big_integer())
                    .col(ColumnDef::new(Organizations::MaxEmbeddingsPerDay).big_integer())
                    .to_owned(),
            )
            .await?;

        manager
            .create_table(
                Table::create()
                    .table(OrgDailyUsage::Table)
                    .if_not_exists()
                    .col(ColumnDef::new(OrgDailyUsage::OrgId).string().not_null())
                    .col(ColumnDef::new(OrgDailyUsage::UserId).string().not_null())
                    .col(ColumnDef::new(OrgDailyUsage::Day).date().not_null())
                    .col(
                        ColumnDef::new(OrgDailyUsage::EmbeddingCalls)
                            .big_integer()
                            .not_null(),
                    )
                    .col(ColumnDef::new(OrgDailyUsage::UpdatedAt).timestamp().not_null())
                    .primary_key(
                        Index::create()
                            .col(OrgDailyUsage::OrgId)
                            .col(OrgDailyUsage::UserId)
                            .col(OrgDailyUsage::Day),
                    )
                    .foreign_key(
                        ForeignKey::create()
                            .from(OrgDailyUsage::Table, OrgDailyUsage::OrgId)
                            .to(Organizations::Table, Organizations::Id)
                            .on_delete(ForeignKeyAction::Cascade),
                    )
                    .to_owned(),
            )
            .await?;

        manager
            .create_table(
                Table::create()
                    .table(Projects::Table)
                    .if_not_exists()
                    .col(
                        ColumnDef::new(Projects::Id)
                            .string()
                            .not_null()
                            .primary_key(),
                    )
                    .col(ColumnDef::new(Projects::OrgId).string().not_null())
                    .col(ColumnDef::new(Projects::UserId).string().not_null())
                    .col(ColumnDef::new(Projects::Name).string().not_null())
                    .col(ColumnDef::new(Projects::CreatedAt).timestamp().not_null())
                    .col(ColumnDef::new(Projects::ActiveVersionId).string())
                    .foreign_key(
                        ForeignKey::create()
                            .from(Projects::Table, Projects::OrgId)
                            .to(Organizations::Table, Organizations::Id)
                            .on_delete(ForeignKeyAction::Cascade),
                    )
                    .to_owned(),
            )
            .await?;

        manager
            .create_table(
                Table::create()
                    .table(ProjectVersions::Table)
                    .if_not_exists()
                    .col(
                        ColumnDef::new(ProjectVersions::Id)
                            .string()
                            .not_null()
                            .primary_key(),
                    )
                    .col(ColumnDef::new(ProjectVersions::OrgId).string().not_null())
                    .col(ColumnDef::new(ProjectVersions::UserId).string().not_null())
                    .col(ColumnDef::new(ProjectVersions::ProjectId).string().not_null())
                    .col(ColumnDef::new(ProjectVersions::CreatedAt).timestamp().not_null())
                    .col(ColumnDef::new(ProjectVersions::Status).string().not_null())
                    .foreign_key(
                        ForeignKey::create()
                            .from(ProjectVersions::Table, ProjectVersions::ProjectId)
                            .to(Projects::Table, Projects::Id)
                            .on_delete(ForeignKeyAction::Cascade),
                    )
                    .to_owned(),
            )
            .await?;

        manager
            .create_table(
                Table::create()
                    .table(Documents::Table)
                    .if_not_exists()
                    .col(
                        ColumnDef::new(Documents::Id)
                            .string()
                            .not_null()
                            .primary_key(),
                    )
                    .col(ColumnDef::new(Documents::OrgId).string().not_null())
                    .col(ColumnDef::new(Documents::UserId).string().not_null())
                    .col(ColumnDef::new(Documents::ProjectId).string().not_null())
                    .col(ColumnDef::new(Documents::SourceRef).string().not_null())
                    .col(ColumnDef::new(Documents::Title).string())
                    .col(ColumnDef::new(Documents::ContentHash).string().not_null())
                    .col(ColumnDef::new(Documents::IndexedAt).timestamp().not_null())
                    .col(ColumnDef::new(Documents::QualityScore).float())
                    .col(
                        ColumnDef::new(Documents::Verified)
                            .boolean()
                            .not_null()
                            .default(false),
                    )
                    .foreign_key(
                        ForeignKey::create()
                            .from(Documents::Table, Documents::ProjectId)
                            .to(Projects::Table, Projects::Id)
                            .on_delete(ForeignKeyAction::Cascade),
                    )
                    .to_owned(),
            )
            .await?;

        manager
            .create_table(
                Table::create()
                    .table(Chunks::Table)
                    .if_not_exists()
                    .col(
                        ColumnDef::new(Chunks::Id)
                            .string()
                            .not_null()
                            .primary_key(),
                    )
                    .col(ColumnDef::new(Chunks::OrgId).string().not_null())
                    .col(ColumnDef::new(Chunks::UserId).string().not_null())
                    .col(ColumnDef::new(Chunks::ProjectId).string().not_null())
                    .col(ColumnDef::new(Chunks::DocId).string().not_null())
                    .col(ColumnDef::new(Chunks::VersionId).string().not_null())
                    .col(ColumnDef::new(Chunks::Content).text().not_null())
                    .col(ColumnDef::new(Chunks::StartLine).integer().not_null())
                    .col(ColumnDef::new(Chunks::EndLine).integer().not_null())
                    .col(ColumnDef::new(Chunks::QualityScore).float())
                    .col(
                        ColumnDef::new(Chunks::Verified)
                            .boolean()
                            .not_null()
                            .default(false),
                    )
                    .col(
                        ColumnDef::new(Chunks::Embedding)
                            .custom(Alias::new(format!(
                                "vector({DEFAULT_EMBEDDING_DIM})"
                            )))
                            .null(),
                    )
                    .foreign_key(
                        ForeignKey::create()
                            .from(Chunks::Table, Chunks::DocId)
                            .to(Documents::Table, Documents::Id)
                            .on_delete(ForeignKeyAction::Cascade),
                    )
                    .foreign_key(
                        ForeignKey::create()
                            .from(Chunks::Table, Chunks::VersionId)
                            .to(ProjectVersions::Table, ProjectVersions::Id)
                            .on_delete(ForeignKeyAction::Cascade),
                    )
                    .to_owned(),
            )
            .await?;

        manager
            .create_table(
                Table::create()
                    .table(IngestJobs::Table)
                    .if_not_exists()
                    .col(
                        ColumnDef::new(IngestJobs::Id)
                            .string()
                            .not_null()
                            .primary_key(),
                    )
                    .col(ColumnDef::new(IngestJobs::OrgId).string().not_null())
                    .col(ColumnDef::new(IngestJobs::UserId).string().not_null())
                    .col(ColumnDef::new(IngestJobs::ProjectId).string().not_null())
                    .col(ColumnDef::new(IngestJobs::Status).string().not_null())
                    .col(ColumnDef::new(IngestJobs::Payload).text().not_null())
                    .col(ColumnDef::new(IngestJobs::Attempts).integer().not_null())
                    .col(ColumnDef::new(IngestJobs::Error).text())
                    .col(ColumnDef::new(IngestJobs::VersionId).string())
                    .col(ColumnDef::new(IngestJobs::CreatedAt).timestamp().not_null())
                    .col(ColumnDef::new(IngestJobs::UpdatedAt).timestamp().not_null())
                    .foreign_key(
                        ForeignKey::create()
                            .from(IngestJobs::Table, IngestJobs::OrgId)
                            .to(Organizations::Table, Organizations::Id)
                            .on_delete(ForeignKeyAction::Cascade),
                    )
                    .foreign_key(
                        ForeignKey::create()
                            .from(IngestJobs::Table, IngestJobs::ProjectId)
                            .to(Projects::Table, Projects::Id)
                            .on_delete(ForeignKeyAction::Cascade),
                    )
                    .to_owned(),
            )
            .await?;

        manager
            .create_index(
                Index::create()
                    .name("idx_projects_org")
                    .table(Projects::Table)
                    .col(Projects::OrgId)
                    .col(Projects::UserId)
                    .to_owned(),
            )
            .await?;

        manager
            .create_index(
                Index::create()
                    .name("idx_documents_project")
                    .table(Documents::Table)
                    .col(Documents::ProjectId)
                    .col(Documents::UserId)
                    .to_owned(),
            )
            .await?;

        manager
            .create_index(
                Index::create()
                    .name("idx_chunks_project_version")
                    .table(Chunks::Table)
                    .col(Chunks::ProjectId)
                    .col(Chunks::UserId)
                    .col(Chunks::VersionId)
                    .to_owned(),
            )
            .await?;

        manager
            .create_index(
                Index::create()
                    .name("idx_ingest_jobs_status_created")
                    .table(IngestJobs::Table)
                    .col(IngestJobs::Status)
                    .col(IngestJobs::CreatedAt)
                    .to_owned(),
            )
            .await?;

        enable_rls(manager, Projects::Table, Projects::OrgId).await?;
        enable_rls(manager, Documents::Table, Documents::OrgId).await?;
        enable_rls(manager, ProjectVersions::Table, ProjectVersions::OrgId).await?;
        enable_rls(manager, Chunks::Table, Chunks::OrgId).await?;
        enable_rls(manager, IngestJobs::Table, IngestJobs::OrgId).await?;
        enable_rls(manager, OrgDailyUsage::Table, OrgDailyUsage::OrgId).await?;

        Ok(())
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .drop_table(Table::drop().table(Chunks::Table).if_exists().to_owned())
            .await?;
        manager
            .drop_table(Table::drop().table(IngestJobs::Table).if_exists().to_owned())
            .await?;
        manager
            .drop_table(Table::drop().table(OrgDailyUsage::Table).if_exists().to_owned())
            .await?;
        manager
            .drop_table(Table::drop().table(Documents::Table).if_exists().to_owned())
            .await?;
        manager
            .drop_table(Table::drop().table(ProjectVersions::Table).if_exists().to_owned())
            .await?;
        manager
            .drop_table(Table::drop().table(Projects::Table).if_exists().to_owned())
            .await?;
        manager
            .drop_table(Table::drop().table(Organizations::Table).if_exists().to_owned())
            .await?;
        Ok(())
    }
}

#[derive(DeriveMigrationName)]
pub(crate) struct AddQualityColumns;

#[async_trait::async_trait]
impl MigrationTrait for AddQualityColumns {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        let mut doc_quality = ColumnDef::new(Documents::QualityScore);
        doc_quality.float();
        let mut doc_verified = ColumnDef::new(Documents::Verified);
        doc_verified.boolean().not_null().default(false);
        manager
            .alter_table(
                Table::alter()
                    .table(Documents::Table)
                    .add_column_if_not_exists(&mut doc_quality)
                    .add_column_if_not_exists(&mut doc_verified)
                    .to_owned(),
            )
            .await?;

        let mut chunk_quality = ColumnDef::new(Chunks::QualityScore);
        chunk_quality.float();
        let mut chunk_verified = ColumnDef::new(Chunks::Verified);
        chunk_verified.boolean().not_null().default(false);
        manager
            .alter_table(
                Table::alter()
                    .table(Chunks::Table)
                    .add_column_if_not_exists(&mut chunk_quality)
                    .add_column_if_not_exists(&mut chunk_verified)
                    .to_owned(),
            )
            .await?;

        Ok(())
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .alter_table(
                Table::alter()
                    .table(Chunks::Table)
                    .drop_column(Chunks::QualityScore)
                    .drop_column(Chunks::Verified)
                    .to_owned(),
            )
            .await?;
        manager
            .alter_table(
                Table::alter()
                    .table(Documents::Table)
                    .drop_column(Documents::QualityScore)
                    .drop_column(Documents::Verified)
                    .to_owned(),
            )
            .await?;
        Ok(())
    }
}

#[derive(DeriveMigrationName)]
pub(crate) struct AddWorkerStatusTable;

#[async_trait::async_trait]
impl MigrationTrait for AddWorkerStatusTable {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .create_table(
                Table::create()
                    .table(WorkerStatus::Table)
                    .if_not_exists()
                    .col(
                        ColumnDef::new(WorkerStatus::Id)
                            .string()
                            .not_null()
                            .primary_key(),
                    )
                    .col(ColumnDef::new(WorkerStatus::Version).string().not_null())
                    .col(ColumnDef::new(WorkerStatus::UpdatedAt).timestamp().not_null())
                    .to_owned(),
            )
            .await?;
        Ok(())
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .drop_table(
                Table::drop()
                    .table(WorkerStatus::Table)
                    .if_exists()
                    .to_owned(),
            )
            .await?;
        Ok(())
    }
}

#[derive(DeriveMigrationName)]
pub(crate) struct AddServerConfigIdColumn;

#[async_trait::async_trait]
impl MigrationTrait for AddServerConfigIdColumn {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        let mut column = ColumnDef::new(Projects::ActiveConfigId);
        column
            .string()
            .not_null()
            .default(DEFAULT_CONFIG_ID);
        manager
            .alter_table(
                Table::alter()
                    .table(Projects::Table)
                    .add_column_if_not_exists(&mut column)
                    .to_owned(),
            )
            .await?;

        let backfill = format!(
            "UPDATE {} SET {} = '{}' WHERE {} IS NULL",
            Projects::Table.to_string(),
            Projects::ActiveConfigId.to_string(),
            DEFAULT_CONFIG_ID,
            Projects::ActiveConfigId.to_string()
        );
        manager
            .get_connection()
            .execute(Statement::from_string(DatabaseBackend::Postgres, backfill))
            .await?;

        Ok(())
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .alter_table(
                Table::alter()
                    .table(Projects::Table)
                    .drop_column(Projects::ActiveConfigId)
                    .to_owned(),
            )
            .await?;
        Ok(())
    }
}

#[derive(DeriveMigrationName)]
pub(crate) struct AddServerConfigIdColumns;

#[async_trait::async_trait]
impl MigrationTrait for AddServerConfigIdColumns {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        let mut doc_column = ColumnDef::new(Documents::ConfigId);
        doc_column
            .string()
            .not_null()
            .default(DEFAULT_CONFIG_ID);
        manager
            .alter_table(
                Table::alter()
                    .table(Documents::Table)
                    .add_column_if_not_exists(&mut doc_column)
                    .to_owned(),
            )
            .await?;

        let mut chunk_column = ColumnDef::new(Chunks::ConfigId);
        chunk_column
            .string()
            .not_null()
            .default(DEFAULT_CONFIG_ID);
        manager
            .alter_table(
                Table::alter()
                    .table(Chunks::Table)
                    .add_column_if_not_exists(&mut chunk_column)
                    .to_owned(),
            )
            .await?;

        let mut version_column = ColumnDef::new(ProjectVersions::ActiveConfigId);
        version_column
            .string()
            .not_null()
            .default(DEFAULT_CONFIG_ID);
        manager
            .alter_table(
                Table::alter()
                    .table(ProjectVersions::Table)
                    .add_column_if_not_exists(&mut version_column)
                    .to_owned(),
            )
            .await?;

        let backfill = format!(
            "UPDATE {} SET {} = '{}' WHERE {} IS NULL",
            Documents::Table.to_string(),
            Documents::ConfigId.to_string(),
            DEFAULT_CONFIG_ID,
            Documents::ConfigId.to_string()
        );
        manager
            .get_connection()
            .execute(Statement::from_string(DatabaseBackend::Postgres, backfill))
            .await?;

        let backfill = format!(
            "UPDATE {} SET {} = '{}' WHERE {} IS NULL",
            Chunks::Table.to_string(),
            Chunks::ConfigId.to_string(),
            DEFAULT_CONFIG_ID,
            Chunks::ConfigId.to_string()
        );
        manager
            .get_connection()
            .execute(Statement::from_string(DatabaseBackend::Postgres, backfill))
            .await?;

        let backfill = format!(
            "UPDATE {} SET {} = '{}' WHERE {} IS NULL",
            ProjectVersions::Table.to_string(),
            ProjectVersions::ActiveConfigId.to_string(),
            DEFAULT_CONFIG_ID,
            ProjectVersions::ActiveConfigId.to_string()
        );
        manager
            .get_connection()
            .execute(Statement::from_string(DatabaseBackend::Postgres, backfill))
            .await?;

        manager
            .create_index(
                Index::create()
                    .name("idx_chunks_project_version_config")
                    .table(Chunks::Table)
                    .col(Chunks::OrgId)
                    .col(Chunks::ProjectId)
                    .col(Chunks::VersionId)
                    .col(Chunks::ConfigId)
                    .to_owned(),
            )
            .await?;
        manager
            .create_index(
                Index::create()
                    .name("uniq_chunks_org_project_version_config_id")
                    .table(Chunks::Table)
                    .col(Chunks::OrgId)
                    .col(Chunks::ProjectId)
                    .col(Chunks::VersionId)
                    .col(Chunks::ConfigId)
                    .col(Chunks::Id)
                    .unique()
                    .to_owned(),
            )
            .await?;

        Ok(())
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .drop_index(
                Index::drop()
                    .name("idx_chunks_project_version_config")
                    .table(Chunks::Table)
                    .to_owned(),
            )
            .await?;
        manager
            .drop_index(
                Index::drop()
                    .name("uniq_chunks_org_project_version_config_id")
                    .table(Chunks::Table)
                    .to_owned(),
            )
            .await?;
        manager
            .alter_table(
                Table::alter()
                    .table(ProjectVersions::Table)
                    .drop_column(ProjectVersions::ActiveConfigId)
                    .to_owned(),
            )
            .await?;
        manager
            .alter_table(
                Table::alter()
                    .table(Chunks::Table)
                    .drop_column(Chunks::ConfigId)
                    .to_owned(),
            )
            .await?;
        manager
            .alter_table(
                Table::alter()
                    .table(Documents::Table)
                    .drop_column(Documents::ConfigId)
                    .to_owned(),
            )
            .await?;
        Ok(())
    }
}

#[derive(DeriveMigrationName)]
pub(crate) struct CreateServerIndexingConfigs;

#[async_trait::async_trait]
impl MigrationTrait for CreateServerIndexingConfigs {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .create_table(
                Table::create()
                    .table(IndexingConfigs::Table)
                    .if_not_exists()
                    .col(
                        ColumnDef::new(IndexingConfigs::OrgId)
                            .string()
                            .not_null(),
                    )
                    .col(
                        ColumnDef::new(IndexingConfigs::ConfigId)
                            .string()
                            .not_null(),
                    )
                    .col(
                        ColumnDef::new(IndexingConfigs::Chunking)
                            .text()
                            .not_null(),
                    )
                    .col(
                        ColumnDef::new(IndexingConfigs::Embedding)
                            .text()
                            .not_null(),
                    )
                    .col(ColumnDef::new(IndexingConfigs::VectorBackend).text())
                    .col(
                        ColumnDef::new(IndexingConfigs::VectorMetric)
                            .string()
                            .not_null(),
                    )
                    .col(ColumnDef::new(IndexingConfigs::IndexParams).text())
                    .col(
                        ColumnDef::new(IndexingConfigs::CreatedAt)
                            .timestamp()
                            .not_null(),
                    )
                    .primary_key(
                        Index::create()
                            .col(IndexingConfigs::OrgId)
                            .col(IndexingConfigs::ConfigId),
                    )
                    .foreign_key(
                        ForeignKey::create()
                            .from(IndexingConfigs::Table, IndexingConfigs::OrgId)
                            .to(Organizations::Table, Organizations::Id)
                            .on_delete(ForeignKeyAction::Cascade),
                    )
                    .to_owned(),
            )
            .await?;

        enable_rls(manager, IndexingConfigs::Table, IndexingConfigs::OrgId).await?;
        Ok(())
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .drop_table(
                Table::drop()
                    .table(IndexingConfigs::Table)
                    .if_exists()
                    .to_owned(),
            )
            .await?;
        Ok(())
    }
}

#[derive(DeriveMigrationName)]
pub(crate) struct AddUserIdColumns;

#[async_trait::async_trait]
impl MigrationTrait for AddUserIdColumns {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        let default_user = "system";

        let mut project_user = ColumnDef::new(Projects::UserId);
        project_user.string().not_null().default(default_user);
        manager
            .alter_table(
                Table::alter()
                    .table(Projects::Table)
                    .add_column_if_not_exists(&mut project_user)
                    .to_owned(),
            )
            .await?;

        let mut version_user = ColumnDef::new(ProjectVersions::UserId);
        version_user.string().not_null().default(default_user);
        manager
            .alter_table(
                Table::alter()
                    .table(ProjectVersions::Table)
                    .add_column_if_not_exists(&mut version_user)
                    .to_owned(),
            )
            .await?;

        let mut document_user = ColumnDef::new(Documents::UserId);
        document_user.string().not_null().default(default_user);
        manager
            .alter_table(
                Table::alter()
                    .table(Documents::Table)
                    .add_column_if_not_exists(&mut document_user)
                    .to_owned(),
            )
            .await?;

        let mut chunk_user = ColumnDef::new(Chunks::UserId);
        chunk_user.string().not_null().default(default_user);
        manager
            .alter_table(
                Table::alter()
                    .table(Chunks::Table)
                    .add_column_if_not_exists(&mut chunk_user)
                    .to_owned(),
            )
            .await?;

        let mut job_user = ColumnDef::new(IngestJobs::UserId);
        job_user.string().not_null().default(default_user);
        manager
            .alter_table(
                Table::alter()
                    .table(IngestJobs::Table)
                    .add_column_if_not_exists(&mut job_user)
                    .to_owned(),
            )
            .await?;

        let mut usage_user = ColumnDef::new(OrgDailyUsage::UserId);
        usage_user.string().not_null().default(default_user);
        manager
            .alter_table(
                Table::alter()
                    .table(OrgDailyUsage::Table)
                    .add_column_if_not_exists(&mut usage_user)
                    .to_owned(),
            )
            .await?;

        let backend = manager.get_database_backend();
        let conn = manager.get_connection();
        let table_name = OrgDailyUsage::Table.to_string();
        let drop_pk = format!("ALTER TABLE {table_name} DROP CONSTRAINT IF EXISTS org_daily_usage_pkey");
        conn.execute(Statement::from_string(backend, drop_pk))
            .await?;
        let add_pk = format!(
            "ALTER TABLE {table_name} ADD PRIMARY KEY ({}, {}, {})",
            OrgDailyUsage::OrgId.to_string(),
            OrgDailyUsage::UserId.to_string(),
            OrgDailyUsage::Day.to_string()
        );
        conn.execute(Statement::from_string(backend, add_pk))
            .await?;

        Ok(())
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        let backend = manager.get_database_backend();
        let conn = manager.get_connection();
        let table_name = OrgDailyUsage::Table.to_string();
        let drop_pk = format!("ALTER TABLE {table_name} DROP CONSTRAINT IF EXISTS org_daily_usage_pkey");
        conn.execute(Statement::from_string(backend, drop_pk))
            .await?;
        let add_pk = format!(
            "ALTER TABLE {table_name} ADD PRIMARY KEY ({}, {})",
            OrgDailyUsage::OrgId.to_string(),
            OrgDailyUsage::Day.to_string()
        );
        conn.execute(Statement::from_string(backend, add_pk))
            .await?;

        manager
            .alter_table(
                Table::alter()
                    .table(OrgDailyUsage::Table)
                    .drop_column(OrgDailyUsage::UserId)
                    .to_owned(),
            )
            .await?;
        manager
            .alter_table(
                Table::alter()
                    .table(IngestJobs::Table)
                    .drop_column(IngestJobs::UserId)
                    .to_owned(),
            )
            .await?;
        manager
            .alter_table(
                Table::alter()
                    .table(Chunks::Table)
                    .drop_column(Chunks::UserId)
                    .to_owned(),
            )
            .await?;
        manager
            .alter_table(
                Table::alter()
                    .table(Documents::Table)
                    .drop_column(Documents::UserId)
                    .to_owned(),
            )
            .await?;
        manager
            .alter_table(
                Table::alter()
                    .table(ProjectVersions::Table)
                    .drop_column(ProjectVersions::UserId)
                    .to_owned(),
            )
            .await?;
        manager
            .alter_table(
                Table::alter()
                    .table(Projects::Table)
                    .drop_column(Projects::UserId)
                    .to_owned(),
            )
            .await?;

        Ok(())
    }
}

async fn enable_rls<T: Iden + Copy>(
    manager: &SchemaManager<'_>,
    table: T,
    org_column: T,
) -> Result<(), DbErr> {
    let backend = manager.get_database_backend();
    let table_name = table.to_string();
    let column_name = org_column.to_string();
    let enable_sql = format!("ALTER TABLE {table_name} ENABLE ROW LEVEL SECURITY");
    let policy_sql = format!(
        "CREATE POLICY coco_tenant_isolation ON {table_name} \
         USING ({column_name} = current_setting('coco.org_id', true))"
    );
    let conn = manager.get_connection();
    conn.execute(Statement::from_string(backend, enable_sql))
        .await?;
    conn.execute(Statement::from_string(backend, policy_sql))
        .await?;
    Ok(())
}

#[derive(DeriveIden, Copy, Clone)]
enum Organizations {
    Table,
    Id,
    Name,
    CreatedAt,
    MaxDocuments,
    MaxChunks,
    MaxStorageBytes,
    MaxEmbeddingsPerDay,
}

#[derive(DeriveIden, Copy, Clone)]
enum Projects {
    Table,
    Id,
    OrgId,
    UserId,
    Name,
    CreatedAt,
    ActiveVersionId,
    ActiveConfigId,
}

#[derive(DeriveIden, Copy, Clone)]
enum ProjectVersions {
    Table,
    Id,
    OrgId,
    UserId,
    ProjectId,
    CreatedAt,
    Status,
    ActiveConfigId,
}

#[derive(DeriveIden, Copy, Clone)]
enum Documents {
    Table,
    Id,
    OrgId,
    UserId,
    ProjectId,
    ConfigId,
    SourceRef,
    Title,
    ContentHash,
    IndexedAt,
    QualityScore,
    Verified,
}

#[derive(DeriveIden, Copy, Clone)]
enum Chunks {
    Table,
    Id,
    OrgId,
    UserId,
    ProjectId,
    DocId,
    VersionId,
    ConfigId,
    Content,
    StartLine,
    EndLine,
    QualityScore,
    Verified,
    Embedding,
}

#[derive(DeriveIden, Copy, Clone)]
enum IngestJobs {
    Table,
    Id,
    OrgId,
    UserId,
    ProjectId,
    Status,
    Payload,
    Attempts,
    Error,
    VersionId,
    CreatedAt,
    UpdatedAt,
}

#[derive(DeriveIden, Copy, Clone)]
enum OrgDailyUsage {
    Table,
    OrgId,
    UserId,
    Day,
    EmbeddingCalls,
    UpdatedAt,
}

#[derive(DeriveIden, Copy, Clone)]
enum IndexingConfigs {
    Table,
    OrgId,
    ConfigId,
    Chunking,
    Embedding,
    VectorBackend,
    VectorMetric,
    IndexParams,
    CreatedAt,
}

#[derive(DeriveIden, Copy, Clone)]
enum WorkerStatus {
    Table,
    Id,
    Version,
    UpdatedAt,
}
