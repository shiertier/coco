use sea_orm_migration::prelude::*;
use sea_orm_migration::sea_orm::{ConnectionTrait, DatabaseBackend, Statement, Value};

const DEFAULT_CONFIG_ID: &str = "default";

pub(crate) struct Migrator;

#[async_trait::async_trait]
impl MigratorTrait for Migrator {
    fn migrations() -> Vec<Box<dyn MigrationTrait>> {
        vec![
            Box::new(CreateLocalMetaTables),
            Box::new(AddLocalConfigIdColumns),
            Box::new(CreateLocalIndexingConfigs),
            Box::new(AddLocalVersioning),
        ]
    }
}

#[derive(DeriveMigrationName)]
pub(crate) struct CreateLocalMetaTables;

#[async_trait::async_trait]
impl MigrationTrait for CreateLocalMetaTables {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
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
                    .col(ColumnDef::new(Projects::Name).string().not_null())
                    .col(ColumnDef::new(Projects::Path).string().not_null())
                    .col(ColumnDef::new(Projects::CreatedAt).timestamp().not_null())
                    .col(ColumnDef::new(Projects::ActiveVersionId).string())
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
                    .col(ColumnDef::new(Documents::ProjectId).string().not_null())
                    .col(ColumnDef::new(Documents::Path).string().not_null())
                    .col(ColumnDef::new(Documents::Title).string())
                    .col(ColumnDef::new(Documents::ContentHash).string().not_null())
                    .col(ColumnDef::new(Documents::IndexedAt).timestamp().not_null())
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
                    .col(ColumnDef::new(Chunks::Id).string().not_null().primary_key())
                    .col(ColumnDef::new(Chunks::DocId).string().not_null())
                    .col(ColumnDef::new(Chunks::Content).text().not_null())
                    .col(ColumnDef::new(Chunks::StartLine).integer().not_null())
                    .col(ColumnDef::new(Chunks::EndLine).integer().not_null())
                    .foreign_key(
                        ForeignKey::create()
                            .from(Chunks::Table, Chunks::DocId)
                            .to(Documents::Table, Documents::Id)
                            .on_delete(ForeignKeyAction::Cascade),
                    )
                    .to_owned(),
            )
            .await?;

        Ok(())
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .drop_table(Table::drop().table(Chunks::Table).if_exists().to_owned())
            .await?;
        manager
            .drop_table(Table::drop().table(Documents::Table).if_exists().to_owned())
            .await?;
        manager
            .drop_table(Table::drop().table(Projects::Table).if_exists().to_owned())
            .await?;
        Ok(())
    }
}

#[derive(DeriveMigrationName)]
pub(crate) struct AddLocalConfigIdColumns;

#[async_trait::async_trait]
impl MigrationTrait for AddLocalConfigIdColumns {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .alter_table(
                Table::alter()
                    .table(Projects::Table)
                    .add_column(
                        ColumnDef::new(Projects::ActiveConfigId)
                            .string()
                            .not_null()
                            .default(DEFAULT_CONFIG_ID),
                    )
                    .to_owned(),
            )
            .await?;
        manager
            .alter_table(
                Table::alter()
                    .table(Documents::Table)
                    .add_column(
                        ColumnDef::new(Documents::ConfigId)
                            .string()
                            .not_null()
                            .default(DEFAULT_CONFIG_ID),
                    )
                    .to_owned(),
            )
            .await?;
        manager
            .alter_table(
                Table::alter()
                    .table(Chunks::Table)
                    .add_column(
                        ColumnDef::new(Chunks::ConfigId)
                            .string()
                            .not_null()
                            .default(DEFAULT_CONFIG_ID),
                    )
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
            .execute(Statement::from_string(DatabaseBackend::Sqlite, backfill))
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
            .execute(Statement::from_string(DatabaseBackend::Sqlite, backfill))
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
            .execute(Statement::from_string(DatabaseBackend::Sqlite, backfill))
            .await?;

        Ok(())
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
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
pub(crate) struct CreateLocalIndexingConfigs;

#[async_trait::async_trait]
impl MigrationTrait for CreateLocalIndexingConfigs {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .create_table(
                Table::create()
                    .table(IndexingConfigs::Table)
                    .if_not_exists()
                    .col(
                        ColumnDef::new(IndexingConfigs::ConfigId)
                            .string()
                            .not_null()
                            .primary_key(),
                    )
                    .col(ColumnDef::new(IndexingConfigs::Chunking).text().not_null())
                    .col(ColumnDef::new(IndexingConfigs::Embedding).text().not_null())
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
                    .to_owned(),
            )
            .await?;
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
pub(crate) struct AddLocalVersioning;

#[async_trait::async_trait]
impl MigrationTrait for AddLocalVersioning {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
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
                    .col(
                        ColumnDef::new(ProjectVersions::ProjectId)
                            .string()
                            .not_null(),
                    )
                    .col(ColumnDef::new(ProjectVersions::Status).string().not_null())
                    .col(
                        ColumnDef::new(ProjectVersions::CreatedAt)
                            .timestamp()
                            .not_null(),
                    )
                    .col(
                        ColumnDef::new(ProjectVersions::ItemCount)
                            .big_integer()
                            .not_null()
                            .default(0),
                    )
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
            .alter_table(
                Table::alter()
                    .table(Documents::Table)
                    .add_column(
                        ColumnDef::new(Documents::VersionId)
                            .string()
                            .not_null()
                            .default(""),
                    )
                    .to_owned(),
            )
            .await?;
        manager
            .alter_table(
                Table::alter()
                    .table(Chunks::Table)
                    .add_column(
                        ColumnDef::new(Chunks::VersionId)
                            .string()
                            .not_null()
                            .default(""),
                    )
                    .to_owned(),
            )
            .await?;

        backfill_versions(manager).await?;
        Ok(())
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .alter_table(
                Table::alter()
                    .table(Chunks::Table)
                    .drop_column(Chunks::VersionId)
                    .to_owned(),
            )
            .await?;
        manager
            .alter_table(
                Table::alter()
                    .table(Documents::Table)
                    .drop_column(Documents::VersionId)
                    .to_owned(),
            )
            .await?;
        manager
            .drop_table(
                Table::drop()
                    .table(ProjectVersions::Table)
                    .if_exists()
                    .to_owned(),
            )
            .await?;
        Ok(())
    }
}

async fn backfill_versions(manager: &SchemaManager<'_>) -> Result<(), DbErr> {
    let conn = manager.get_connection();
    let rows = conn
        .query_all(Statement::from_string(
            DatabaseBackend::Sqlite,
            "SELECT id, active_version_id FROM projects".to_string(),
        ))
        .await?;
    let now = chrono::Utc::now();
    for row in rows {
        let project_id: String = row.try_get("", "id")?;
        let active_version: Option<String> = row.try_get("", "active_version_id")?;
        let version_id = active_version.unwrap_or_else(generate_version_id);
        let status = "ACTIVE".to_string();

        let count_stmt = Statement::from_sql_and_values(
            DatabaseBackend::Sqlite,
            "SELECT COUNT(*) AS total FROM chunks \
             WHERE doc_id IN (SELECT id FROM documents WHERE project_id = ?)"
                .to_string(),
            vec![Value::from(project_id.clone())],
        );
        let count_row = conn.query_one(count_stmt).await?;
        let item_count: i64 = count_row
            .and_then(|row| row.try_get("", "total").ok())
            .unwrap_or(0);

        let insert = Statement::from_sql_and_values(
            DatabaseBackend::Sqlite,
            "INSERT OR IGNORE INTO project_versions \
             (id, project_id, status, created_at, item_count) VALUES (?, ?, ?, ?, ?)"
                .to_string(),
            vec![
                Value::from(version_id.clone()),
                Value::from(project_id.clone()),
                Value::from(status),
                Value::from(now),
                Value::from(item_count),
            ],
        );
        conn.execute(insert).await?;

        let update_project = Statement::from_sql_and_values(
            DatabaseBackend::Sqlite,
            "UPDATE projects SET active_version_id = ? WHERE id = ? AND active_version_id IS NULL"
                .to_string(),
            vec![
                Value::from(version_id.clone()),
                Value::from(project_id.clone()),
            ],
        );
        conn.execute(update_project).await?;

        let update_documents = Statement::from_sql_and_values(
            DatabaseBackend::Sqlite,
            "UPDATE documents SET version_id = ? WHERE project_id = ?".to_string(),
            vec![
                Value::from(version_id.clone()),
                Value::from(project_id.clone()),
            ],
        );
        conn.execute(update_documents).await?;

        let update_chunks = Statement::from_sql_and_values(
            DatabaseBackend::Sqlite,
            "UPDATE chunks SET version_id = ? \
             WHERE doc_id IN (SELECT id FROM documents WHERE project_id = ?)"
                .to_string(),
            vec![
                Value::from(version_id.clone()),
                Value::from(project_id.clone()),
            ],
        );
        conn.execute(update_chunks).await?;
    }
    Ok(())
}

fn generate_version_id() -> String {
    format!("v{}", uuid::Uuid::now_v7())
}

#[derive(DeriveIden)]
enum Projects {
    Table,
    Id,
    Name,
    Path,
    CreatedAt,
    ActiveVersionId,
    ActiveConfigId,
}

#[derive(DeriveIden)]
enum Documents {
    Table,
    Id,
    ProjectId,
    VersionId,
    ConfigId,
    Path,
    Title,
    ContentHash,
    IndexedAt,
}

#[derive(DeriveIden)]
enum Chunks {
    Table,
    Id,
    DocId,
    VersionId,
    ConfigId,
    Content,
    StartLine,
    EndLine,
}

#[derive(DeriveIden)]
enum ProjectVersions {
    Table,
    Id,
    ProjectId,
    Status,
    CreatedAt,
    ItemCount,
}

#[derive(DeriveIden)]
enum IndexingConfigs {
    Table,
    ConfigId,
    Chunking,
    Embedding,
    VectorBackend,
    VectorMetric,
    IndexParams,
    CreatedAt,
}
