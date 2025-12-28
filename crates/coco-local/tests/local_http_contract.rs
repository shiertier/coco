#![cfg(feature = "local-storage")]

mod support;

use serde::Deserialize;

use support::{EnvSnapshot, env_lock, pick_port, spawn_service, temp_root};

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
struct ActivateResponse {
    active_config_id: String,
}

#[tokio::test]
async fn config_id_lifecycle_and_query_validation() -> coco_protocol::CocoResult<()> {
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

    let root = temp_root("coco-local-http");
    let port = pick_port();
    let runner = spawn_service(root, port);
    let client = reqwest::Client::new();
    let healthy = runner.wait_for_health(&client).await;
    assert!(healthy, "service did not become healthy");

    let base_url = runner.base_url();
    let register_body = serde_json::json!({
        "name": "Test Project",
        "path": "project",
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
    assert_eq!(register.data.active_config_id, "default");

    let invalid_config_body = serde_json::json!({
        "config": {
            "config_id": "Bad",
            "chunking": {
                "strategy_name": "fixed_token",
                "chunk_size": 8,
                "chunk_overlap": 0
            },
            "embedding": {
                "model_name": "stub",
                "dimensions": 3
            },
            "vector_metric": "l2",
            "index_params": null,
            "vector_backend": null
        }
    });
    let config_url = format!("{base_url}/v1/sys/configs");
    let invalid_response = client
        .post(&config_url)
        .json(&invalid_config_body)
        .send()
        .await?;
    assert_eq!(invalid_response.status(), reqwest::StatusCode::BAD_REQUEST);

    let config_body = serde_json::json!({
        "config": {
            "config_id": "alt-config",
            "chunking": {
                "strategy_name": "fixed_token",
                "chunk_size": 8,
                "chunk_overlap": 0
            },
            "embedding": {
                "model_name": "stub",
                "dimensions": 3
            },
            "vector_metric": "l2",
            "index_params": null,
            "vector_backend": null
        }
    });
    let config_response = client.post(&config_url).json(&config_body).send().await?;
    assert!(config_response.status().is_success());

    let missing_query_body = serde_json::json!({
        "project_id": register.data.project_id,
        "indexing_config_id": "missing-config",
        "intent": {
            "query_text": "hello",
            "retrieval_mode": "vector",
            "top_k": 5,
            "hybrid_alpha": 0.5,
            "filters": [],
            "reranker": null
        }
    });
    let query_url = format!("{base_url}/v1/docs/query");
    let missing_query_response = client
        .post(&query_url)
        .json(&missing_query_body)
        .send()
        .await?;
    assert_eq!(
        missing_query_response.status(),
        reqwest::StatusCode::BAD_REQUEST
    );

    let inactive_query_body = serde_json::json!({
        "project_id": register.data.project_id,
        "indexing_config_id": "alt-config",
        "intent": {
            "query_text": "hello",
            "retrieval_mode": "vector",
            "top_k": 5,
            "hybrid_alpha": 0.5,
            "filters": [],
            "reranker": null
        }
    });
    let inactive_query_response = client
        .post(&query_url)
        .json(&inactive_query_body)
        .send()
        .await?;
    assert_eq!(
        inactive_query_response.status(),
        reqwest::StatusCode::BAD_REQUEST
    );

    let activate_body = serde_json::json!({
        "project_id": register.data.project_id,
        "config_id": "alt-config",
    });
    let activate_url = format!("{base_url}/v1/sys/configs/activate");
    let activate_response = client
        .post(&activate_url)
        .json(&activate_body)
        .send()
        .await?;
    assert!(activate_response.status().is_success());
    let activate: Envelope<ActivateResponse> = activate_response.json().await?;
    assert_eq!(activate.data.active_config_id, "alt-config");

    let active_query_body = serde_json::json!({
        "project_id": register.data.project_id,
        "indexing_config_id": "alt-config",
        "intent": {
            "query_text": "hello",
            "retrieval_mode": "vector",
            "top_k": 5,
            "hybrid_alpha": 0.5,
            "filters": [],
            "reranker": null
        }
    });
    let active_query_response = client
        .post(&query_url)
        .json(&active_query_body)
        .send()
        .await?;
    assert!(active_query_response.status().is_success());

    let backend_query_body = serde_json::json!({
        "project_id": register.data.project_id,
        "indexing_config_id": "alt-config",
        "intent": {
            "query_text": "hello",
            "retrieval_mode": "vector",
            "top_k": 5,
            "hybrid_alpha": 0.5,
            "filters": [],
            "reranker": null
        },
        "retrieval_config": {
            "retrieval_mode": "vector",
            "top_k": 5,
            "hybrid_alpha": 0.5,
            "vector_backend": {
                "kind": "qdrant",
                "url": "http://localhost:6333"
            }
        }
    });
    let backend_query_response = client
        .post(&query_url)
        .json(&backend_query_body)
        .send()
        .await?;
    assert_eq!(
        backend_query_response.status(),
        reqwest::StatusCode::BAD_REQUEST
    );

    let filter_query_body = serde_json::json!({
        "project_id": register.data.project_id,
        "indexing_config_id": "alt-config",
        "intent": {
            "query_text": "hello",
            "retrieval_mode": "vector",
            "top_k": 5,
            "hybrid_alpha": 0.5,
            "filters": [
                {
                    "field": "content",
                    "op": "contains",
                    "value": "hello"
                }
            ],
            "reranker": null
        }
    });
    let filter_query_response = client
        .post(&query_url)
        .json(&filter_query_body)
        .send()
        .await?;
    assert!(filter_query_response.status().is_success());

    runner.shutdown().await;
    Ok(())
}
