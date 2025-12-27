use coco_protocol::{
    normalize_config_id, Chunk, CocoError, CocoResult, StorageBackend, VectorMetadata, VectorRecord,
    VectorStore,
};
use sea_orm::{ConnectionTrait, DatabaseBackend, Statement};

use super::backend::PgBackend;
use super::executor::PgExecutor;
use super::helpers::{
    map_storage_err, push_value, to_i64, validate_tenant, vector_literal, COL_CONFIG_ID,
    COL_CONTENT, COL_DOC_ID, COL_EMBEDDING, COL_EMBEDDING_TEXT, COL_END_LINE, COL_ID, COL_ORG_ID,
    COL_PROJECT_ID, COL_QUALITY_SCORE, COL_START_LINE, COL_USER_ID, COL_VERIFIED, COL_VERSION_ID,
    TABLE_CHUNKS,
};
use super::helpers::{chunk_from_row, parse_vector_text};

impl StorageBackend for PgBackend {
    fn upsert_chunks(
        &self,
        chunks: &[Chunk],
    ) -> impl std::future::Future<Output = CocoResult<()>> + Send {
        let db = self.db.clone();
        let tenant = self.tenant.clone();
        let backend = self.clone();
        async move {
            if chunks.is_empty() {
                return Ok(());
            }
            validate_tenant(&tenant)?;
            let version_id = backend.resolve_version_id().await?;
            let config_id = backend.resolve_config_id(None).await?;
            let mut values = Vec::new();
            let mut rows = Vec::with_capacity(chunks.len());

            for chunk in chunks {
                let embedding = chunk
                    .embedding
                    .as_ref()
                    .ok_or_else(|| CocoError::user("chunk embedding is required"))?;
                let embedding_literal = vector_literal(embedding);

                let id_ph = push_value(&mut values, chunk.id.to_string());
                let org_ph = push_value(&mut values, tenant.org_id.clone());
                let user_ph = push_value(&mut values, tenant.user_id.clone());
                let project_ph = push_value(&mut values, tenant.project_id.clone());
                let doc_ph = push_value(&mut values, chunk.doc_id.to_string());
                let version_ph = push_value(&mut values, version_id.clone());
                let config_ph = push_value(&mut values, config_id.clone());
                let content_ph = push_value(&mut values, chunk.content.clone());
                let start_line = to_i64(chunk.span.start)?;
                let end_line = to_i64(chunk.span.end)?;
                let start_ph = push_value(&mut values, start_line);
                let end_ph = push_value(&mut values, end_line);
                let quality_ph = push_value(&mut values, chunk.quality_score);
                let verified_ph = push_value(&mut values, chunk.verified.unwrap_or(false));
                let embed_ph = push_value(&mut values, embedding_literal);

                rows.push(format!(
                    "({id_ph}, {org_ph}, {user_ph}, {project_ph}, {doc_ph}, {version_ph}, \
                         {config_ph}, {content_ph}, {start_ph}, {end_ph}, {quality_ph}, \
                         {verified_ph}, {embed_ph}::vector)"
                ));
            }

            let sql = format!(
                "INSERT INTO {TABLE_CHUNKS} \
                     ({COL_ID}, {COL_ORG_ID}, {COL_USER_ID}, {COL_PROJECT_ID}, {COL_DOC_ID}, \
                     {COL_VERSION_ID}, {COL_CONFIG_ID}, {COL_CONTENT}, {COL_START_LINE}, \
                     {COL_END_LINE}, {COL_QUALITY_SCORE}, {COL_VERIFIED}, {COL_EMBEDDING}) \
                     VALUES {} \
                     ON CONFLICT ({COL_ID}) DO UPDATE SET \
                     {COL_USER_ID} = EXCLUDED.{COL_USER_ID}, \
                     {COL_DOC_ID} = EXCLUDED.{COL_DOC_ID}, \
                     {COL_VERSION_ID} = EXCLUDED.{COL_VERSION_ID}, \
                     {COL_CONFIG_ID} = EXCLUDED.{COL_CONFIG_ID}, \
                     {COL_CONTENT} = EXCLUDED.{COL_CONTENT}, \
                     {COL_START_LINE} = EXCLUDED.{COL_START_LINE}, \
                     {COL_END_LINE} = EXCLUDED.{COL_END_LINE}, \
                     {COL_QUALITY_SCORE} = EXCLUDED.{COL_QUALITY_SCORE}, \
                     {COL_VERIFIED} = EXCLUDED.{COL_VERIFIED}, \
                     {COL_EMBEDDING} = EXCLUDED.{COL_EMBEDDING}",
                rows.join(", ")
            );

            let stmt = Statement::from_sql_and_values(DatabaseBackend::Postgres, sql, values);
            db.execute(stmt).await.map_err(map_storage_err)?;
            Ok(())
        }
    }

    fn search_similar(
        &self,
        intent: coco_protocol::SearchIntent,
    ) -> impl std::future::Future<Output = CocoResult<Vec<coco_protocol::SearchHit>>> + Send {
        let executor = PgExecutor::new(self);
        async move { executor.search(intent).await }
    }

    fn delete_by_doc(
        &self,
        doc_id: coco_protocol::DocumentId,
    ) -> impl std::future::Future<Output = CocoResult<()>> + Send {
        let db = self.db.clone();
        let tenant = self.tenant.clone();
        let backend = self.clone();
        async move {
            validate_tenant(&tenant)?;
            let version_id = backend.resolve_version_id().await?;
            let config_id = backend.resolve_config_id(None).await?;
            let mut values = Vec::new();
            let org_ph = push_value(&mut values, tenant.org_id);
            let user_ph = push_value(&mut values, tenant.user_id);
            let project_ph = push_value(&mut values, tenant.project_id);
            let doc_ph = push_value(&mut values, doc_id.to_string());
            let version_ph = push_value(&mut values, version_id);
            let config_ph = push_value(&mut values, config_id);
            let sql = format!(
                "DELETE FROM {TABLE_CHUNKS} \
                 WHERE {COL_ORG_ID} = {org_ph} AND {COL_USER_ID} = {user_ph} \
                 AND {COL_PROJECT_ID} = {project_ph} \
                 AND {COL_DOC_ID} = {doc_ph} AND {COL_VERSION_ID} = {version_ph} \
                 AND {COL_CONFIG_ID} = {config_ph}"
            );
            db.execute(Statement::from_sql_and_values(
                DatabaseBackend::Postgres,
                sql,
                values,
            ))
            .await
            .map_err(map_storage_err)?;
            Ok(())
        }
    }

    fn get_chunk(
        &self,
        chunk_id: coco_protocol::ChunkId,
    ) -> impl std::future::Future<Output = CocoResult<Option<coco_protocol::Chunk>>> + Send {
        let db = self.db.clone();
        let tenant = self.tenant.clone();
        let backend = self.clone();
        async move {
            validate_tenant(&tenant)?;
            let version_id = backend.resolve_version_id().await?;
            let config_id = backend.resolve_config_id(None).await?;
            let mut values = Vec::new();
            let org_ph = push_value(&mut values, tenant.org_id);
            let user_ph = push_value(&mut values, tenant.user_id);
            let project_ph = push_value(&mut values, tenant.project_id);
            let chunk_ph = push_value(&mut values, chunk_id.to_string());
            let version_ph = push_value(&mut values, version_id);
            let config_ph = push_value(&mut values, config_id);
            let sql = format!(
                "SELECT {COL_ID}, {COL_DOC_ID}, {COL_CONTENT}, \
                 {COL_START_LINE}, {COL_END_LINE}, {COL_QUALITY_SCORE}, {COL_VERIFIED} \
                 FROM {TABLE_CHUNKS} \
                 WHERE {COL_ORG_ID} = {org_ph} AND {COL_USER_ID} = {user_ph} \
                 AND {COL_PROJECT_ID} = {project_ph} \
                 AND {COL_ID} = {chunk_ph} AND {COL_VERSION_ID} = {version_ph} \
                 AND {COL_CONFIG_ID} = {config_ph}"
            );
            let row = db
                .query_one(Statement::from_sql_and_values(
                    DatabaseBackend::Postgres,
                    sql,
                    values,
                ))
                .await
                .map_err(map_storage_err)?;
            let Some(row) = row else {
                return Ok(None);
            };
            let chunk = chunk_from_row(&row)?;
            Ok(Some(chunk))
        }
    }
}

impl VectorStore for PgBackend {
    fn upsert_vectors(
        &self,
        records: &[VectorRecord],
    ) -> impl std::future::Future<Output = CocoResult<()>> + Send {
        let backend = self.clone();
        async move {
            if records.is_empty() {
                return Ok(());
            }
            let mut resolved_config: Option<String> = None;
            let mut chunks = Vec::with_capacity(records.len());
            for record in records {
                let Some(config_id) = record.metadata.config_id.as_deref() else {
                    return Err(CocoError::user("config_id required for vector record"));
                };
                let normalized = normalize_config_id(config_id)?;
                if normalized != config_id {
                    return Err(CocoError::user("config_id must be normalized"));
                }
                if let Some(existing) = resolved_config.as_deref() {
                    if existing != config_id {
                        return Err(CocoError::user("config_id mismatch in vector batch"));
                    }
                } else {
                    resolved_config = Some(config_id.to_string());
                }
                chunks.push(Chunk {
                    id: record.chunk_id.clone(),
                    doc_id: record.metadata.doc_id.clone(),
                    content: record.metadata.content.clone(),
                    embedding: Some(record.embedding.clone()),
                    span: record.metadata.span,
                    quality_score: None,
                    verified: None,
                });
            }
            backend.upsert_chunks(&chunks).await
        }
    }

    fn search_vectors(
        &self,
        intent: coco_protocol::SearchIntent,
    ) -> impl std::future::Future<Output = CocoResult<Vec<coco_protocol::SearchHit>>> + Send {
        let executor = PgExecutor::new(self);
        async move { executor.search(intent).await }
    }

    fn delete_vectors_by_doc(
        &self,
        doc_id: coco_protocol::DocumentId,
    ) -> impl std::future::Future<Output = CocoResult<()>> + Send {
        self.delete_by_doc(doc_id)
    }

    fn get_vector(
        &self,
        chunk_id: coco_protocol::ChunkId,
    ) -> impl std::future::Future<Output = CocoResult<Option<VectorRecord>>> + Send {
        let db = self.db.clone();
        let tenant = self.tenant.clone();
        let backend = self.clone();
        async move {
            validate_tenant(&tenant)?;
            let version_id = backend.resolve_version_id().await?;
            let config_id = backend.resolve_config_id(None).await?;
            let mut values = Vec::new();
            let org_ph = push_value(&mut values, tenant.org_id);
            let user_ph = push_value(&mut values, tenant.user_id);
            let project_ph = push_value(&mut values, tenant.project_id);
            let chunk_ph = push_value(&mut values, chunk_id.to_string());
            let version_ph = push_value(&mut values, version_id);
            let config_ph = push_value(&mut values, config_id.clone());
            let sql = format!(
                "SELECT {COL_ID}, {COL_DOC_ID}, {COL_CONTENT}, {COL_START_LINE}, {COL_END_LINE}, \
                 {COL_QUALITY_SCORE}, {COL_VERIFIED}, {COL_EMBEDDING}::text AS {COL_EMBEDDING_TEXT} \
                 FROM {TABLE_CHUNKS} \
                 WHERE {COL_ORG_ID} = {org_ph} AND {COL_USER_ID} = {user_ph} \
                 AND {COL_PROJECT_ID} = {project_ph} \
                 AND {COL_ID} = {chunk_ph} AND {COL_VERSION_ID} = {version_ph} \
                 AND {COL_CONFIG_ID} = {config_ph}"
            );
            let row = db
                .query_one(Statement::from_sql_and_values(
                    DatabaseBackend::Postgres,
                    sql,
                    values,
                ))
                .await
                .map_err(map_storage_err)?;
            let Some(row) = row else {
                return Ok(None);
            };
            let embedding_text: Option<String> =
                row.try_get("", COL_EMBEDDING_TEXT).map_err(map_storage_err)?;
            let embedding_text =
                embedding_text.ok_or_else(|| CocoError::storage("embedding missing for vector"))?;
            let embedding = parse_vector_text(&embedding_text)?;
            let chunk = chunk_from_row(&row)?;
            Ok(Some(VectorRecord {
                chunk_id: chunk.id,
                embedding,
                metadata: VectorMetadata {
                    config_id: Some(config_id),
                    doc_id: chunk.doc_id,
                    content: chunk.content,
                    span: chunk.span,
                },
            }))
        }
    }
}
