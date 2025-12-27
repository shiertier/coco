use coco_core::normalize_config_id;
use coco_protocol::{
    CocoError, CocoResult, SearchHit, SearchIntent, StorageBackend, VectorRecord, VectorStore,
};
use qdrant_client::qdrant::{
    Condition, DeletePointsBuilder, GetPointsBuilder, PointStruct, UpsertPointsBuilder,
    Filter as QdrantFilter,
};

use super::executor::QdrantExecutor;
use super::store::QdrantStore;
use super::util::{
    map_qdrant_err, PAYLOAD_CONFIG_ID, PAYLOAD_DOC_ID, PAYLOAD_ORG_ID, PAYLOAD_PROJECT_ID,
    PAYLOAD_USER_ID, PAYLOAD_VERSION_ID,
};

impl VectorStore for QdrantStore {
    fn upsert_vectors(
        &self,
        records: &[VectorRecord],
    ) -> impl std::future::Future<Output = CocoResult<()>> + Send {
        let store = self.clone();
        async move {
            if records.is_empty() {
                return Ok(());
            }
            let mut config_id = None;
            let mut points = Vec::with_capacity(records.len());
            let mut vector_len = None;
            for record in records {
                let record_config = record
                    .metadata
                    .config_id
                    .as_deref()
                    .ok_or_else(|| CocoError::user("config_id required for vector record"))?;
                let normalized = normalize_config_id(record_config)?;
                if normalized != record_config {
                    return Err(CocoError::user("config_id must be normalized"));
                }
                if let Some(existing) = config_id.as_deref() {
                    if existing != record_config {
                        return Err(CocoError::user("config_id mismatch in vector batch"));
                    }
                } else {
                    config_id = Some(record_config.to_string());
                }
                let len = record.embedding.len();
                if let Some(existing) = vector_len {
                    if existing != len {
                        return Err(CocoError::user(
                            "embedding dimensions mismatch in vector batch",
                        ));
                    }
                } else {
                    vector_len = Some(len);
                }
            }
            let config_id = config_id.ok_or_else(|| CocoError::user("config_id required"))?;
            if config_id != store.config_id() {
                return Err(CocoError::user(
                    "config_id does not match qdrant backend",
                ));
            }
            let vector_len = vector_len.unwrap_or(0);
            let collection = store.collection_name(&config_id, store.version_id())?;
            store.ensure_collection(&collection, vector_len).await?;
            if store.use_collection_alias() {
                let alias = store.collection_alias(&config_id)?;
                store.ensure_alias(&collection, &alias).await?;
            }
            for record in records {
                let point_id = store.point_id_for(&config_id, &record.chunk_id);
                let payload = store.payload_for(&config_id, record);
                points.push(PointStruct::new(
                    point_id,
                    record.embedding.clone(),
                    payload,
                ));
            }
            store
                .client()
                .upsert_points(UpsertPointsBuilder::new(collection, points))
                .await
                .map_err(|err| map_qdrant_err("qdrant upsert failed", err))?;
            Ok(())
        }
    }

    fn search_vectors(
        &self,
        intent: SearchIntent,
    ) -> impl std::future::Future<Output = CocoResult<Vec<SearchHit>>> + Send {
        let executor = QdrantExecutor::new(self);
        async move { executor.search(intent).await }
    }

    fn delete_vectors_by_doc(
        &self,
        doc_id: coco_protocol::DocumentId,
    ) -> impl std::future::Future<Output = CocoResult<()>> + Send {
        let store = self.clone();
        async move {
            let collection = store.collection_name(store.config_id(), store.version_id())?;
            let exists = store
                .client()
                .collection_exists(&collection)
                .await
                .map_err(|err| map_qdrant_err("qdrant collection check failed", err))?;
            if !exists {
                return Ok(());
            }
            let filter = QdrantFilter::must(vec![
                Condition::matches(PAYLOAD_ORG_ID, store.org_id().to_string()),
                Condition::matches(PAYLOAD_USER_ID, store.user_id().to_string()),
                Condition::matches(PAYLOAD_PROJECT_ID, store.project_id().to_string()),
                Condition::matches(PAYLOAD_VERSION_ID, store.version_id().to_string()),
                Condition::matches(PAYLOAD_CONFIG_ID, store.config_id().to_string()),
                Condition::matches(PAYLOAD_DOC_ID, doc_id.to_string()),
            ]);
            store
                .client()
                .delete_points(DeletePointsBuilder::new(collection).points(filter))
                .await
                .map_err(|err| map_qdrant_err("qdrant delete failed", err))?;
            Ok(())
        }
    }

    fn get_vector(
        &self,
        chunk_id: coco_protocol::ChunkId,
    ) -> impl std::future::Future<Output = CocoResult<Option<VectorRecord>>> + Send {
        let store = self.clone();
        async move {
            let collection = store.collection_name(store.config_id(), store.version_id())?;
            let exists = store
                .client()
                .collection_exists(&collection)
                .await
                .map_err(|err| map_qdrant_err("qdrant collection check failed", err))?;
            if !exists {
                return Ok(None);
            }
            let point_id = store.point_id_for(store.config_id(), &chunk_id);
            let ids = vec![point_id.into()];
            let response = store
                .client()
                .get_points(
                    GetPointsBuilder::new(collection, ids)
                        .with_payload(true)
                        .with_vectors(true),
                )
                .await
                .map_err(|err| map_qdrant_err("qdrant get points failed", err))?;
            let point = response.result.into_iter().next();
            let Some(point) = point else {
                return Ok(None);
            };
            let Some(vectors) = point.vectors else {
                return Ok(None);
            };
            let Some(vector) = vectors.get_vector() else {
                return Ok(None);
            };
            let embedding = match vector {
                qdrant_client::qdrant::vector_output::Vector::Dense(dense) => dense.data,
                _ => {
                    return Err(CocoError::storage(
                        "qdrant vector output format not supported",
                    ))
                }
            };
            let chunk = store
                .pg()
                .with_config(Some(store.config_id().to_string()))
                .get_chunk(chunk_id.clone())
                .await?;
            let Some(chunk) = chunk else {
                return Ok(None);
            };
            Ok(Some(VectorRecord {
                chunk_id,
                embedding,
                metadata: coco_protocol::VectorMetadata {
                    config_id: Some(store.config_id().to_string()),
                    doc_id: chunk.doc_id.clone(),
                    content: chunk.content.clone(),
                    span: chunk.span,
                },
            }))
        }
    }
}
