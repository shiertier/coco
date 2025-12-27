#![cfg(feature = "local-storage")]

mod support;

use chrono::Utc;
use coco_local::storage::lance::LanceBackend;
use coco_local::storage::meta::{
    LocalMetaStore, NewChunk, NewDocument, NewIndexingConfig, NewProject, DEFAULT_CONFIG_ID,
};
use coco_protocol::{
    Chunk, ChunkingStrategy, EmbeddingConfig, RetrievalMode, SearchIntent, SearchIntentInput,
    StorageBackend, TextSpan, VectorMetric,
};
use support::temp_root;

#[tokio::test]
async fn register_import_query_roundtrip() -> coco_protocol::CocoResult<()> {
    let root = temp_root("coco-local-test");
    let meta_db = root.join("meta.db");
    let vector_dir = root.join("vectors");

    let meta = LocalMetaStore::connect(&meta_db).await?;
    meta.ensure_default_indexing_config(NewIndexingConfig {
        config_id: DEFAULT_CONFIG_ID.to_string(),
        chunking: ChunkingStrategy {
            strategy_name: "fixed_token".to_string(),
            chunk_size: 64,
            chunk_overlap: 0,
        },
        embedding: EmbeddingConfig {
            model_name: "stub".to_string(),
            dimensions: Some(3),
        },
        vector_backend: None,
        vector_metric: VectorMetric::L2,
        index_params: None,
        created_at: Utc::now(),
    })
    .await?;
    let project_id = "proj-test".to_string();
    let project = meta
        .create_project(NewProject {
            id: project_id.clone(),
            name: "Test Project".to_string(),
            path: "/tmp/project".to_string(),
            created_at: Utc::now(),
            active_version_id: None,
            active_config_id: DEFAULT_CONFIG_ID.to_string(),
        })
        .await?;
    let version_id = project
        .active_version_id
        .clone()
        .expect("active version id");

    let vector = LanceBackend::open(&vector_dir, 3, &version_id).await?;

    let doc_id = "doc-test".to_string();
    meta.create_document(NewDocument {
        id: doc_id.clone(),
        project_id: project_id.clone(),
        version_id: version_id.clone(),
        config_id: DEFAULT_CONFIG_ID.to_string(),
        path: "/tmp/project/readme.md".to_string(),
        title: Some("Readme".to_string()),
        content_hash: "hash".to_string(),
        indexed_at: Utc::now(),
    })
    .await?;

    meta.create_chunks(vec![NewChunk {
        id: "chunk-test".to_string(),
        doc_id: doc_id.clone(),
        version_id: version_id.clone(),
        config_id: DEFAULT_CONFIG_ID.to_string(),
        content: "hello".to_string(),
        start_line: 1,
        end_line: 1,
    }])
    .await?;

    let chunk = Chunk {
        id: "chunk-test".into(),
        doc_id: doc_id.clone().into(),
        content: "hello".to_string(),
        embedding: Some(vec![1.0, 0.0, 0.0]),
        span: TextSpan { start: 0, end: 5 },
        quality_score: None,
        verified: None,
    };
    vector.upsert_chunks(vec![chunk.clone()]).await?;

    let intent = SearchIntentInput {
        query_text: None,
        query_embedding: Some(vec![1.0, 0.0, 0.0]),
        retrieval_mode: RetrievalMode::Vector,
        indexing_config_id: None,
        top_k: 5,
        hybrid_alpha: 0.5,
        filters: Vec::new(),
        reranker: None,
    };
    let intent = SearchIntent::try_from(intent)?;
    let results = vector.search_similar(intent).await?;
    assert_eq!(results.len(), 1);
    assert_eq!(results[0].chunk.id, chunk.id);
    assert_eq!(results[0].chunk.doc_id.as_str(), doc_id);

    let _ = std::fs::remove_dir_all(&root);
    Ok(())
}
