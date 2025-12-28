#![cfg(feature = "local-storage")]

mod support;

use std::sync::Arc;
use std::time::Duration;

use chrono::Utc;
use coco_core::build_search_intent;
use coco_local::embedder::LocalEmbedder;
use coco_local::ingest::{file_type_for_path, title_for_path, IngestRequest, Ingestor};
use coco_local::storage::lance::LanceStore;
use coco_local::storage::meta::{
    LocalMetaStore, NewIndexingConfig, NewProject, DEFAULT_CONFIG_ID,
};
use coco_protocol::{
    EmbeddingModel, SearchHit, SearchIntentInput, SearchQueryInput, StorageBackend,
};
use serde::Deserialize;

use support::{
    default_chunking, default_indexing_config, env_lock, pick_port, spawn_service, temp_root,
    EnvSnapshot,
};

#[derive(Debug, Deserialize)]
struct Envelope<T> {
    meta: EnvelopeMeta,
    data: T,
}

#[derive(Debug, Deserialize)]
struct EnvelopeMeta {
    status: String,
}

#[derive(Debug, Deserialize)]
struct RegisterResponse {
    project_id: String,
    active_config_id: String,
}

#[derive(Debug, Deserialize)]
struct ImportResponse {
    document_id: String,
}

#[derive(Debug, Deserialize)]
struct QueryResponse {
    results: Vec<SearchHit>,
}

#[tokio::test]
async fn file_change_ingest_and_query() -> coco_protocol::CocoResult<()> {
    let root = temp_root("coco-local-service-test");
    let meta_db = root.join("meta.db");
    let vector_dir = root.join("vectors");
    let project_root = root.join("project");
    std::fs::create_dir_all(&project_root).expect("project dir");
    let file_path = project_root.join("readme.md");
    std::fs::write(&file_path, "# Title\n\nBody\n").expect("write file");

    let meta = LocalMetaStore::connect(&meta_db).await?;
    meta.ensure_default_indexing_config(default_indexing_config(3))
        .await?;
    let vector = LanceStore::new(vector_dir.clone(), 3);
    let embedder = Arc::new(LocalEmbedder::stub("stub", 3)?);
    let ingestor = Ingestor::new(
        meta.clone(),
        vector.clone(),
        Some(Arc::clone(&embedder)),
        default_chunking(),
    );

    let project_id = "proj-local".to_string();
    let project = meta
        .create_project(NewProject {
            id: project_id.clone(),
            name: "Local Project".to_string(),
            path: project_root.to_string_lossy().to_string(),
            created_at: Utc::now(),
            active_version_id: None,
            active_config_id: DEFAULT_CONFIG_ID.to_string(),
        })
        .await?;
    let version_id = project
        .active_version_id
        .clone()
        .expect("active version id");
    let default_config = meta
        .get_indexing_config(DEFAULT_CONFIG_ID)
        .await?
        .expect("default config");
    let backend = vector
        .backend_for_config(
            &default_config.config_id,
            &version_id,
            default_config.vector_metric,
            default_config.index_params.as_ref(),
        )
        .await?;

    let content = std::fs::read_to_string(&file_path).expect("read file");
    let path_string = file_path.to_string_lossy().to_string();
    let result = ingestor
        .ingest_request(IngestRequest {
            project_id: project_id.clone(),
            indexing_config_id: DEFAULT_CONFIG_ID.to_string(),
            version_id: None,
            document_id: None,
            content,
            content_hash: None,
            file_type: file_type_for_path(&file_path),
            title: title_for_path(&file_path),
            path: Some(path_string),
            chunking: None,
        })
        .await?;

    let first_chunk = result.chunks.first().expect("chunk");
    let query_text = first_chunk.content.as_str();
    let embedding = embedder.embed(&[query_text])?.remove(0);

    let intent = SearchIntentInput::new(
        SearchQueryInput::vector(None, Some(embedding)).expect("query"),
        None,
        5,
        0.5,
        Vec::new(),
        None,
    )
    .expect("intent");
    let intent = build_search_intent(intent)?;
    let results = backend.search_similar(intent).await?;
    assert!(results
        .iter()
        .any(|item| item.chunk.doc_id.as_str() == result.document_id));

    let _ = std::fs::remove_dir_all(&root);
    Ok(())
}

#[tokio::test]
async fn file_removal_prunes_document() -> coco_protocol::CocoResult<()> {
    let root = temp_root("coco-local-service-test");
    let meta_db = root.join("meta.db");
    let vector_dir = root.join("vectors");
    let project_root = root.join("project");
    std::fs::create_dir_all(&project_root).expect("project dir");
    let file_path = project_root.join("notes.md");
    std::fs::write(&file_path, "Initial content").expect("write file");

    let meta = LocalMetaStore::connect(&meta_db).await?;
    meta.ensure_default_indexing_config(default_indexing_config(3))
        .await?;
    let vector = LanceStore::new(vector_dir.clone(), 3);
    let embedder = Arc::new(LocalEmbedder::stub("stub", 3)?);
    let ingestor = Ingestor::new(
        meta.clone(),
        vector.clone(),
        Some(Arc::clone(&embedder)),
        default_chunking(),
    );

    let project_id = "proj-remove".to_string();
    let project = meta
        .create_project(NewProject {
            id: project_id.clone(),
            name: "Removal Project".to_string(),
            path: project_root.to_string_lossy().to_string(),
            created_at: Utc::now(),
            active_version_id: None,
            active_config_id: DEFAULT_CONFIG_ID.to_string(),
        })
        .await?;
    let version_id = project
        .active_version_id
        .clone()
        .expect("active version id");
    let default_config = meta
        .get_indexing_config(DEFAULT_CONFIG_ID)
        .await?
        .expect("default config");
    let backend = vector
        .backend_for_config(
            &default_config.config_id,
            &version_id,
            default_config.vector_metric,
            default_config.index_params.as_ref(),
        )
        .await?;

    let content = std::fs::read_to_string(&file_path).expect("read file");
    let path_string = file_path.to_string_lossy().to_string();
    let result = ingestor
        .ingest_request(IngestRequest {
            project_id: project_id.clone(),
            indexing_config_id: DEFAULT_CONFIG_ID.to_string(),
            version_id: None,
            document_id: None,
            content,
            content_hash: None,
            file_type: file_type_for_path(&file_path),
            title: title_for_path(&file_path),
            path: Some(path_string.clone()),
            chunking: None,
        })
        .await?;

    let existed = ingestor.delete_by_path(&project_id, &path_string).await?;
    assert!(existed);

    let missing = ingestor.meta().get_document(&result.document_id).await?;
    assert!(missing.is_none());
    let _ = backend.delete_by_doc(result.document_id.into()).await?;

    let _ = std::fs::remove_dir_all(&root);
    Ok(())
}

#[tokio::test]
async fn service_start_and_stop() -> coco_protocol::CocoResult<()> {
    let _guard = env_lock().lock().expect("env lock");
    let _snapshot = EnvSnapshot::capture(&["COCO_EMBEDDER_MODE", "COCO_WATCH_ENABLED"]);
    std::env::set_var("COCO_EMBEDDER_MODE", "stub");
    std::env::set_var("COCO_WATCH_ENABLED", "0");

    let root = temp_root("coco-local-service-test");
    let port = pick_port();
    let runner = spawn_service(root, port);
    let client = reqwest::Client::new();
    let healthy = runner.wait_for_health(&client).await;
    assert!(healthy, "service did not become healthy");

    runner.shutdown().await;
    Ok(())
}

#[tokio::test]
async fn register_import_query_uses_response_envelope() -> coco_protocol::CocoResult<()> {
    let _guard = env_lock().lock().expect("env lock");
    let _snapshot = EnvSnapshot::capture(&[
        "COCO_EMBEDDER_MODE",
        "COCO_WATCH_ENABLED",
        "COCO_LIVE_RETRIEVAL_ENABLED",
        "COCO_LIVE_GREP_ENABLED",
    ]);
    std::env::set_var("COCO_EMBEDDER_MODE", "stub");
    std::env::set_var("COCO_WATCH_ENABLED", "0");
    std::env::set_var("COCO_LIVE_RETRIEVAL_ENABLED", "0");
    std::env::set_var("COCO_LIVE_GREP_ENABLED", "0");

    let root = temp_root("coco-local-service-test");
    let port = pick_port();
    let runner = spawn_service(root, port);
    let client = reqwest::Client::new();
    let base_url = runner.base_url();
    let healthy = runner.wait_for_health(&client).await;
    assert!(healthy, "service did not become healthy");

    let register_body = serde_json::json!({
        "name": "Test Project",
        "path": root.join("project").to_string_lossy().to_string(),
    });
    let register_url = format!("{base_url}/v1/sys/register");
    let register_response = client
        .post(&register_url)
        .json(&register_body)
        .send()
        .await?;
    assert!(register_response.status().is_success());
    let register: Envelope<RegisterResponse> = register_response.json().await?;
    assert_eq!(register.meta.status, "fresh");
    assert_eq!(register.data.active_config_id, DEFAULT_CONFIG_ID);

    let import_body = serde_json::json!({
        "project_id": register.data.project_id,
        "content": "# Title\n\nHello world\n",
        "file_type": "markdown",
        "path": "README.md",
    });
    let import_url = format!("{base_url}/v1/docs/import");
    let import_response = client.post(&import_url).json(&import_body).send().await?;
    assert!(import_response.status().is_success());
    let import: Envelope<ImportResponse> = import_response.json().await?;
    assert_eq!(import.meta.status, "fresh");

    let query_body = serde_json::json!({
        "project_id": register.data.project_id,
        "intent": {
            "query_text": "Hello world",
            "retrieval_mode": "vector",
            "top_k": 5,
            "hybrid_alpha": 0.5,
            "filters": [],
            "reranker": null
        }
    });
    let query_url = format!("{base_url}/v1/docs/query");
    let query_response = client.post(&query_url).json(&query_body).send().await?;
    assert!(query_response.status().is_success());
    let query: Envelope<QueryResponse> = query_response.json().await?;
    assert_eq!(query.meta.status, "fresh");
    assert!(
        query
            .data
            .results
            .iter()
            .any(|hit| hit.chunk.doc_id.as_str() == import.data.document_id)
    );

    runner.shutdown().await;
    Ok(())
}
