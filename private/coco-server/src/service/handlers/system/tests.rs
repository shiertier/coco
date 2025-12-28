use std::sync::Arc;

use axum::Json;
use axum::extract::State;
use axum::http::{HeaderMap, HeaderValue};
use chrono::Utc;
use sea_orm::{ConnectionTrait, DatabaseBackend, Statement};
use uuid::Uuid;

use coco_protocol::{
    ChunkingStrategy, CocoErrorKind, EmbeddingConfig, HybridAlpha, SearchQueryInput,
    VectorBackendKind, VectorMetric,
};

use super::register_project;
use crate::service::config::{PgPoolConfig, QueueMode};
use crate::service::handlers::docs::query_documents;
use crate::service::limiter::RateLimiter;
use crate::service::state::AppState;
use crate::service::types::{PublicSearchIntent, QueryRequest, RegisterProjectRequest};
use crate::storage::meta::{
    DEFAULT_CONFIG_ID, NewIndexingConfig, NewOrganization, ServerMetaStore,
};

fn env_db_url() -> Option<String> {
    std::env::var("COCO_TEST_DB_URL")
        .ok()
        .filter(|value| !value.is_empty())
}

fn test_state(meta: ServerMetaStore, database_url: String, embedding_dimensions: u32) -> AppState {
    AppState {
        meta,
        database_url,
        admin_key: "admin".to_string(),
        api_key: "api".to_string(),
        limiter: Arc::new(RateLimiter::new(10, 10)),
        queue: None,
        queue_mode: QueueMode::Postgres,
        worker_addr: None,
        ingest_blob_dir: None,
        ingest_wasm_module_ref: None,
        query_pg_pool: PgPoolConfig::default(),
        embedder: None,
        embedding_dimensions,
        org_max_documents: None,
        org_max_chunks: None,
        org_max_storage_bytes: None,
        org_max_embeddings_per_day: None,
        vector_backend_kind: VectorBackendKind::PgVector,
        vector_db: None,
    }
}

fn make_embedding(value: f32) -> Vec<f32> {
    let mut embedding = vec![0.0_f32; 1536];
    if let Some(first) = embedding.first_mut() {
        *first = value;
    }
    embedding
}

#[tokio::test]
async fn register_fails_on_conflicting_default_config() -> Result<(), Box<dyn std::error::Error>> {
    let Some(database_url) = env_db_url() else {
        eprintln!("skipping: COCO_TEST_DB_URL not set");
        return Ok(());
    };
    let meta = ServerMetaStore::connect(&database_url).await?;
    let org_id = format!("org-{}", Uuid::now_v7());
    let user_id = format!("user-{}", Uuid::now_v7());
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
    meta.create_indexing_config(NewIndexingConfig {
        org_id: org_id.clone(),
        config_id: DEFAULT_CONFIG_ID.to_string(),
        chunking: ChunkingStrategy {
            strategy_name: "fixed_token".to_string(),
            chunk_size: 128,
            chunk_overlap: 0,
        },
        embedding: EmbeddingConfig {
            model_name: "test".to_string(),
            dimensions: Some(1536),
        },
        vector_backend: None,
        vector_metric: VectorMetric::L2,
        index_params: None,
        created_at: Utc::now(),
    })
    .await?;

    let state = test_state(meta, database_url, 1536);
    let payload = RegisterProjectRequest {
        org_id,
        user_id,
        org_name: None,
        project_id: None,
        name: "Test Project".to_string(),
        source_ref: "src:test".to_string(),
        platform: None,
    };
    let result = register_project(State(state), Json(payload)).await;
    let error = result.expect_err("expected register failure");
    assert_eq!(error.0.kind(), CocoErrorKind::User);
    Ok(())
}

#[tokio::test]
async fn query_fails_when_active_config_id_missing() -> Result<(), Box<dyn std::error::Error>> {
    let Some(database_url) = env_db_url() else {
        eprintln!("skipping: COCO_TEST_DB_URL not set");
        return Ok(());
    };
    let meta = ServerMetaStore::connect(&database_url).await?;
    let state = test_state(meta, database_url.clone(), 1536);
    let org_id = format!("org-{}", Uuid::now_v7());
    let user_id = format!("user-{}", Uuid::now_v7());
    let payload = RegisterProjectRequest {
        org_id: org_id.clone(),
        user_id: user_id.clone(),
        org_name: None,
        project_id: None,
        name: "Config Missing".to_string(),
        source_ref: "src:test".to_string(),
        platform: None,
    };
    let Json(response) = register_project(State(state.clone()), Json(payload)).await?;

    let update = format!(
        "UPDATE projects SET active_config_id = '' WHERE id = '{}'",
        response.project_id
    );
    state
        .meta
        .connection()
        .execute(Statement::from_string(DatabaseBackend::Postgres, update))
        .await?;

    let mut headers = HeaderMap::new();
    headers.insert("x-coco-org-id", HeaderValue::from_str(&org_id)?);
    headers.insert("x-coco-user-id", HeaderValue::from_str(&user_id)?);
    headers.insert(
        "x-coco-project-id",
        HeaderValue::from_str(&response.project_id)?,
    );
    let request = QueryRequest {
        intent: PublicSearchIntent {
            query: SearchQueryInput::vector(None, Some(make_embedding(1.0))).expect("query"),
            indexing_config_id: None,
            top_k: std::num::NonZeroU32::new(3).expect("top_k"),
            hybrid_alpha: HybridAlpha::new(0.2).expect("hybrid_alpha"),
            filters: Vec::new(),
            reranker: None,
        },
        indexing_config_id: None,
        retrieval_config: None,
        indexing_config: None,
    };
    let result = query_documents(State(state), headers, Json(request)).await;
    let error = result.expect_err("expected query failure");
    assert_eq!(error.0.kind(), CocoErrorKind::User);
    Ok(())
}
