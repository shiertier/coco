use chrono::Utc;
use coco_protocol::{
    Chunk, ChunkId, ChunkingStrategy, DocumentId, EmbeddingConfig, RetrievalMode, SearchIntent,
    SearchIntentInput, StorageBackend, TextSpan, VectorMetric,
};
use coco_server::storage::meta::{
    NewDocument, NewIndexingConfig, NewOrganization, NewProject, ServerMetaStore,
    DEFAULT_CONFIG_ID,
};
use coco_server::storage::pg::{PgBackend, PgBackendConfig};

const EMBEDDING_DIM: usize = 1536;

fn env_value(key: &str) -> Option<String> {
    std::env::var(key).ok().filter(|value| !value.is_empty())
}

fn make_embedding(value: f32) -> Vec<f32> {
    let mut embedding = vec![0.0_f32; EMBEDDING_DIM];
    if !embedding.is_empty() {
        embedding[0] = value;
    }
    embedding
}

async fn ensure_default_config(
    meta: &ServerMetaStore,
    org_id: &str,
) -> coco_protocol::CocoResult<()> {
    meta.ensure_default_indexing_config(NewIndexingConfig {
        org_id: org_id.to_string(),
        config_id: DEFAULT_CONFIG_ID.to_string(),
        chunking: ChunkingStrategy {
            strategy_name: "fixed_token".to_string(),
            chunk_size: 256,
            chunk_overlap: 32,
        },
        embedding: EmbeddingConfig {
            model_name: "test".to_string(),
            dimensions: Some(EMBEDDING_DIM as u32),
        },
        vector_backend: None,
        vector_metric: VectorMetric::L2,
        index_params: None,
        created_at: Utc::now(),
    })
    .await?;
    Ok(())
}

#[tokio::test]
async fn ingest_and_query_roundtrip() -> coco_protocol::CocoResult<()> {
    let Some(database_url) = env_value("COCO_TEST_DB_URL") else {
        eprintln!("skipping: COCO_TEST_DB_URL not set");
        return Ok(());
    };

    let bootstrap = PgBackendConfig::new(
        database_url.clone(),
        "bootstrap".to_string(),
        "bootstrap".to_string(),
        "bootstrap".to_string(),
    );
    let _ = PgBackend::connect(bootstrap).await?;

    let meta = ServerMetaStore::connect(&database_url).await?;
    let org_id = format!("org-{}", Utc::now().timestamp_nanos_opt().unwrap_or(0));
    let user_id = format!("user-{}", Utc::now().timestamp_nanos_opt().unwrap_or(0));
    let project_id = format!("proj-{}", Utc::now().timestamp_nanos_opt().unwrap_or(0));
    meta.create_organization(NewOrganization {
        id: org_id.clone(),
        name: "Test Org".to_string(),
        created_at: Utc::now(),
        max_documents: None,
        max_chunks: None,
        max_storage_bytes: None,
        max_embeddings_per_day: None,
    })
    .await?;
    ensure_default_config(&meta, &org_id).await?;

    meta.create_project(NewProject {
        id: project_id.clone(),
        org_id: org_id.clone(),
        user_id: user_id.clone(),
        name: "Test Project".to_string(),
        created_at: Utc::now(),
        active_version_id: None,
        active_config_id: DEFAULT_CONFIG_ID.to_string(),
    })
    .await?;

    let version = meta
        .create_project_version(&org_id, &user_id, &project_id, DEFAULT_CONFIG_ID)
        .await?;

    let doc_id = "doc-test".to_string();
    meta.upsert_document(NewDocument {
        id: doc_id.clone(),
        org_id: org_id.clone(),
        user_id: user_id.clone(),
        project_id: project_id.clone(),
        config_id: DEFAULT_CONFIG_ID.to_string(),
        source_ref: "src:doc".to_string(),
        title: Some("Doc".to_string()),
        content_hash: "hash".to_string(),
        indexed_at: Utc::now(),
        quality_score: None,
        verified: false,
    })
    .await?;

    let config = PgBackendConfig::new(
        database_url,
        org_id.clone(),
        user_id.clone(),
        project_id.clone(),
    );
    let backend = PgBackend::connect(config).await?;
    backend
        .ensure_indexes(Some(DEFAULT_CONFIG_ID), None)
        .await?;
    let backend = backend.with_version(Some(version.id));

    let chunk = Chunk {
        id: ChunkId::new("chunk-1"),
        doc_id: doc_id.clone().into(),
        content: "hello".to_string(),
        embedding: Some(make_embedding(1.0)),
        span: TextSpan { start: 0, end: 5 },
        quality_score: None,
        verified: Some(false),
    };
    backend.upsert_chunks(std::slice::from_ref(&chunk)).await?;

    let intent = SearchIntentInput {
        query_text: None,
        query_embedding: Some(make_embedding(1.0)),
        retrieval_mode: RetrievalMode::Vector,
        indexing_config_id: None,
        top_k: 5,
        hybrid_alpha: 0.5,
        filters: Vec::new(),
        reranker: None,
    };
    let intent = SearchIntent::try_from(intent)?;
    let results = backend.search_similar(intent).await?;
    assert!(results.iter().any(|item| item.chunk.id == chunk.id));
    Ok(())
}

#[tokio::test]
async fn multi_tenant_isolation() -> coco_protocol::CocoResult<()> {
    let Some(database_url) = env_value("COCO_TEST_DB_URL") else {
        eprintln!("skipping: COCO_TEST_DB_URL not set");
        return Ok(());
    };

    let bootstrap = PgBackendConfig::new(
        database_url.clone(),
        "bootstrap".to_string(),
        "bootstrap".to_string(),
        "bootstrap".to_string(),
    );
    let _ = PgBackend::connect(bootstrap).await?;

    let meta = ServerMetaStore::connect(&database_url).await?;
    let org_a = format!("org-a-{}", Utc::now().timestamp_nanos_opt().unwrap_or(0));
    let org_b = format!("org-b-{}", Utc::now().timestamp_nanos_opt().unwrap_or(0));
    let user_a = format!("user-a-{}", Utc::now().timestamp_nanos_opt().unwrap_or(0));
    let user_b = format!("user-b-{}", Utc::now().timestamp_nanos_opt().unwrap_or(0));
    let project_a = format!("proj-a-{}", Utc::now().timestamp_nanos_opt().unwrap_or(0));
    let project_b = format!("proj-b-{}", Utc::now().timestamp_nanos_opt().unwrap_or(0));

    meta.create_organization(NewOrganization {
        id: org_a.clone(),
        name: "Org A".to_string(),
        created_at: Utc::now(),
        max_documents: None,
        max_chunks: None,
        max_storage_bytes: None,
        max_embeddings_per_day: None,
    })
    .await?;
    ensure_default_config(&meta, &org_a).await?;
    meta.create_organization(NewOrganization {
        id: org_b.clone(),
        name: "Org B".to_string(),
        created_at: Utc::now(),
        max_documents: None,
        max_chunks: None,
        max_storage_bytes: None,
        max_embeddings_per_day: None,
    })
    .await?;
    ensure_default_config(&meta, &org_b).await?;

    meta.create_project(NewProject {
        id: project_a.clone(),
        org_id: org_a.clone(),
        user_id: user_a.clone(),
        name: "Project A".to_string(),
        created_at: Utc::now(),
        active_version_id: None,
        active_config_id: DEFAULT_CONFIG_ID.to_string(),
    })
    .await?;
    meta.create_project(NewProject {
        id: project_b.clone(),
        org_id: org_b.clone(),
        user_id: user_b.clone(),
        name: "Project B".to_string(),
        created_at: Utc::now(),
        active_version_id: None,
        active_config_id: DEFAULT_CONFIG_ID.to_string(),
    })
    .await?;

    let version_a = meta
        .create_project_version(&org_a, &user_a, &project_a, DEFAULT_CONFIG_ID)
        .await?;
    meta.activate_project_version(&org_a, &user_a, &project_a, &version_a.id)
        .await?;
    let version_b = meta
        .create_project_version(&org_b, &user_b, &project_b, DEFAULT_CONFIG_ID)
        .await?;
    meta.activate_project_version(&org_b, &user_b, &project_b, &version_b.id)
        .await?;

    meta.upsert_document(NewDocument {
        id: "doc-a".to_string(),
        org_id: org_a.clone(),
        user_id: user_a.clone(),
        project_id: project_a.clone(),
        config_id: DEFAULT_CONFIG_ID.to_string(),
        source_ref: "src:a".to_string(),
        title: Some("Doc A".to_string()),
        content_hash: "hash-a".to_string(),
        indexed_at: Utc::now(),
        quality_score: None,
        verified: false,
    })
    .await?;
    meta.upsert_document(NewDocument {
        id: "doc-b".to_string(),
        org_id: org_b.clone(),
        user_id: user_b.clone(),
        project_id: project_b.clone(),
        config_id: DEFAULT_CONFIG_ID.to_string(),
        source_ref: "src:b".to_string(),
        title: Some("Doc B".to_string()),
        content_hash: "hash-b".to_string(),
        indexed_at: Utc::now(),
        quality_score: None,
        verified: false,
    })
    .await?;

    let backend_a = PgBackend::connect(PgBackendConfig::new(
        database_url.clone(),
        org_a.clone(),
        user_a.clone(),
        project_a.clone(),
    ))
    .await?;
    backend_a
        .ensure_indexes(Some(DEFAULT_CONFIG_ID), None)
        .await?;
    let backend_a = backend_a.with_version(Some(version_a.id));

    let backend_b = PgBackend::connect(PgBackendConfig::new(
        database_url,
        org_b.clone(),
        user_b.clone(),
        project_b.clone(),
    ))
    .await?;
    backend_b
        .ensure_indexes(Some(DEFAULT_CONFIG_ID), None)
        .await?;
    let backend_b = backend_b.with_version(Some(version_b.id));

    let chunk_a = Chunk {
        id: ChunkId::new("chunk-a"),
        doc_id: DocumentId::new("doc-a"),
        content: "alpha".to_string(),
        embedding: Some(make_embedding(1.0)),
        span: TextSpan { start: 0, end: 5 },
        quality_score: None,
        verified: Some(false),
    };
    let chunk_b = Chunk {
        id: ChunkId::new("chunk-b"),
        doc_id: DocumentId::new("doc-b"),
        content: "beta".to_string(),
        embedding: Some(make_embedding(2.0)),
        span: TextSpan { start: 0, end: 4 },
        quality_score: None,
        verified: Some(false),
    };
    backend_a.upsert_chunks(std::slice::from_ref(&chunk_a)).await?;
    backend_b.upsert_chunks(std::slice::from_ref(&chunk_b)).await?;

    let intent = SearchIntentInput {
        query_text: None,
        query_embedding: Some(make_embedding(1.0)),
        retrieval_mode: RetrievalMode::Vector,
        indexing_config_id: None,
        top_k: 5,
        hybrid_alpha: 0.5,
        filters: Vec::new(),
        reranker: None,
    };
    let intent = SearchIntent::try_from(intent)?;
    let results_a = backend_a.search_similar(intent).await?;
    assert!(results_a.iter().any(|item| item.chunk.id == chunk_a.id));
    assert!(!results_a.iter().any(|item| item.chunk.id == chunk_b.id));

    let intent = SearchIntentInput {
        query_text: None,
        query_embedding: Some(make_embedding(2.0)),
        retrieval_mode: RetrievalMode::Vector,
        indexing_config_id: None,
        top_k: 5,
        hybrid_alpha: 0.5,
        filters: Vec::new(),
        reranker: None,
    };
    let intent = SearchIntent::try_from(intent)?;
    let results_b = backend_b.search_similar(intent).await?;
    assert!(results_b.iter().any(|item| item.chunk.id == chunk_b.id));
    assert!(!results_b.iter().any(|item| item.chunk.id == chunk_a.id));

    Ok(())
}
