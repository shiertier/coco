mod support;

use serde::Deserialize;
use serde_json::Value;
use support::{unique_id, TestProject, TestServer};

const EMBEDDING_DIM: usize = 1536;

#[derive(Debug, Deserialize)]
struct HealthResponse {
    vector_backend: VectorBackendStatus,
}

#[derive(Debug, Deserialize)]
struct VectorBackendStatus {
    kind: String,
}

fn make_embedding(value: f32) -> Vec<f32> {
    let mut embedding = vec![0.0_f32; EMBEDDING_DIM];
    if let Some(first) = embedding.first_mut() {
        *first = value;
    }
    embedding
}

fn query_payload(embedding: Vec<f32>) -> Value {
    serde_json::json!({
        "intent": {
            "query_text": null,
            "query_embedding": embedding,
            "retrieval_mode": "vector",
            "top_k": 3,
            "hybrid_alpha": 0.2,
            "filters": [],
            "reranker": null
        }
    })
}

fn assert_error_response(response: reqwest::blocking::Response) -> Result<Value, String> {
    let status = response.status();
    if !status.is_client_error() {
        return Err(format!("expected 4xx, got {status}"));
    }
    let headers = response.headers().clone();
    if headers.contains_key("trace_id") || headers.contains_key("request_id") {
        return Err("response headers must not include trace_id/request_id".to_string());
    }
    let parsed = TestServer::parse_json(response)?;
    if parsed.get("kind").is_none() || parsed.get("message").is_none() {
        return Err("error response must include kind/message".to_string());
    }
    let message = parsed
        .get("message")
        .and_then(Value::as_str)
        .unwrap_or("");
    if message.trim().is_empty() {
        return Err("error message must not be empty".to_string());
    }
    if message.contains("system error") || message.contains("internal error") {
        return Err("error message must be public-safe".to_string());
    }
    Ok(parsed)
}

fn query_with_filter(
    server: &TestServer,
    project: &TestProject,
    field: &str,
    op: &str,
) -> Result<(), String> {
    let payload = serde_json::json!({
        "intent": {
            "query_text": null,
            "query_embedding": make_embedding(1.0),
            "retrieval_mode": "vector",
            "top_k": 3,
            "hybrid_alpha": 0.2,
            "filters": [{
                "field": field,
                "op": op,
                "value": "value"
            }],
            "reranker": null
        }
    });
    let response = server
        .api_post("/v1/docs/query", project)
        .json(&payload)
        .send()
        .map_err(|err| err.to_string())?;
    let _ = assert_error_response(response)?;
    Ok(())
}

fn query_with_config_id(
    server: &TestServer,
    project: &TestProject,
    config_id: &str,
) -> Result<Value, String> {
    let payload = serde_json::json!({
        "indexing_config_id": config_id,
        "intent": {
            "query_text": null,
            "query_embedding": make_embedding(1.0),
            "retrieval_mode": "vector",
            "top_k": 3,
            "hybrid_alpha": 0.2,
            "filters": [],
            "reranker": null
        }
    });
    let response = server
        .api_post("/v1/docs/query", project)
        .json(&payload)
        .send()
        .map_err(|err| err.to_string())?;
    if !response.status().is_success() {
        return Err(format!("query failed: {}", response.status()));
    }
    TestServer::parse_json(response)
}

#[test]
fn docs_query_returns_envelope() -> Result<(), String> {
    let Some(server) = TestServer::from_env() else {
        eprintln!("skipping: COCO_TEST_SERVER_URL not set");
        return Ok(());
    };
    let project = server.register_project("Contract Docs")?;
    let response = server
        .api_post("/v1/docs/query", &project)
        .json(&query_payload(make_embedding(1.0)))
        .send()
        .map_err(|err| err.to_string())?;
    if !response.status().is_success() {
        return Err(format!("query failed: {}", response.status()));
    }
    let headers = response.headers().clone();
    if headers.contains_key("trace_id") || headers.contains_key("request_id") {
        return Err("response headers must not include trace_id/request_id".to_string());
    }
    let parsed = TestServer::parse_json(response)?;
    if parsed.get("meta").is_none() || parsed.get("data").is_none() {
        return Err("expected ResponseEnvelope fields".to_string());
    }
    let status = parsed
        .get("meta")
        .and_then(|meta| meta.get("status"))
        .and_then(Value::as_str)
        .unwrap_or("missing");
    if status != "fresh" {
        return Err(format!("expected fresh status, got {status}"));
    }
    let body_str = serde_json::to_string(&parsed).map_err(|err| err.to_string())?;
    if body_str.contains("trace_id") || body_str.contains("request_id") {
        return Err("response body must not include trace_id/request_id".to_string());
    }
    Ok(())
}

#[test]
fn memo_query_contract() -> Result<(), String> {
    let Some(server) = TestServer::from_env() else {
        eprintln!("skipping: COCO_TEST_SERVER_URL not set");
        return Ok(());
    };
    let payload = serde_json::json!({
        "session_token": "session",
        "intent": {
            "query_text": null,
            "query_embedding": make_embedding(0.5),
            "retrieval_mode": "vector",
            "top_k": 3,
            "hybrid_alpha": 0.2,
            "filters": [],
            "reranker": null
        }
    });
    let response = server
        .api_post_without_project("/v1/memo/query")
        .json(&payload)
        .send()
        .map_err(|err| err.to_string())?;
    if !response.status().is_success() {
        return Err(format!("memo query failed: {}", response.status()));
    }
    let parsed = TestServer::parse_json(response)?;
    if parsed.get("meta").is_none() || parsed.get("data").is_none() {
        return Err("memo query must return ResponseEnvelope".to_string());
    }

    let empty_token = serde_json::json!({
        "session_token": "",
        "intent": {
            "query_text": null,
            "query_embedding": make_embedding(0.5),
            "retrieval_mode": "vector",
            "top_k": 3,
            "hybrid_alpha": 0.2,
            "filters": [],
            "reranker": null
        }
    });
    let response = server
        .api_post_without_project("/v1/memo/query")
        .json(&empty_token)
        .send()
        .map_err(|err| err.to_string())?;
    let _ = assert_error_response(response)?;

    let with_config = serde_json::json!({
        "session_token": "session",
        "intent": {
            "query_text": null,
            "query_embedding": make_embedding(0.5),
            "retrieval_mode": "vector",
            "top_k": 3,
            "hybrid_alpha": 0.2,
            "filters": [],
            "reranker": null,
            "indexing_config_id": "default"
        }
    });
    let response = server
        .api_post_without_project("/v1/memo/query")
        .json(&with_config)
        .send()
        .map_err(|err| err.to_string())?;
    let _ = assert_error_response(response)?;

    let with_retrieval = serde_json::json!({
        "session_token": "session",
        "intent": {
            "query_text": null,
            "query_embedding": make_embedding(0.5),
            "retrieval_mode": "vector",
            "top_k": 3,
            "hybrid_alpha": 0.2,
            "filters": [],
            "reranker": null
        },
        "retrieval_config": {
            "retrieval_mode": "vector",
            "top_k": 3,
            "hybrid_alpha": 0.2,
            "reranker": null,
            "vector_backend": {
                "kind": "pg_vector",
                "url": null,
                "api_key": null,
                "collection_prefix": null
            }
        }
    });
    let response = server
        .api_post_without_project("/v1/memo/query")
        .json(&with_retrieval)
        .send()
        .map_err(|err| err.to_string())?;
    let _ = assert_error_response(response)?;

    Ok(())
}

#[test]
fn config_id_lifecycle_and_stale() -> Result<(), String> {
    let Some(server) = TestServer::from_env() else {
        eprintln!("skipping: COCO_TEST_SERVER_URL not set");
        return Ok(());
    };
    let project = server.register_project("Config Lifecycle")?;
    let config_id = unique_id("fast");
    let config_payload = serde_json::json!({
        "org_id": project.org_id,
        "config": {
            "config_id": config_id,
            "chunking": {
                "strategy_name": "fixed_token",
                "chunk_size": 128,
                "chunk_overlap": 16
            },
            "embedding": {
                "model_name": "test",
                "dimensions": 1536
            },
            "vector_metric": "l2",
            "index_params": null,
            "vector_backend": null
        }
    });
    let response = server
        .admin_post("/v1/sys/configs")
        .json(&config_payload)
        .send()
        .map_err(|err| err.to_string())?;
    if !response.status().is_success() {
        return Err(format!("config upsert failed: {}", response.status()));
    }

    let stale = query_with_config_id(&server, &project, &config_id)?;
    let status = stale
        .get("meta")
        .and_then(|meta| meta.get("status"))
        .and_then(Value::as_str)
        .unwrap_or("missing");
    if status != "stale" {
        return Err(format!("expected stale status, got {status}"));
    }

    let activate = serde_json::json!({
        "org_id": project.org_id,
        "user_id": project.user_id,
        "project_id": project.project_id,
        "config_id": config_id
    });
    let response = server
        .admin_post("/v1/sys/configs/activate")
        .json(&activate)
        .send()
        .map_err(|err| err.to_string())?;
    if !response.status().is_success() {
        return Err(format!("config activate failed: {}", response.status()));
    }

    let fresh = query_with_config_id(&server, &project, &config_id)?;
    let status = fresh
        .get("meta")
        .and_then(|meta| meta.get("status"))
        .and_then(Value::as_str)
        .unwrap_or("missing");
    if status != "fresh" {
        return Err(format!("expected fresh status, got {status}"));
    }

    let response = server
        .api_post("/v1/docs/query", &project)
        .json(&serde_json::json!({
            "indexing_config_id": "unknown-config",
            "intent": {
                "query_text": null,
                "query_embedding": make_embedding(1.0),
                "retrieval_mode": "vector",
                "top_k": 3,
                "hybrid_alpha": 0.2,
                "filters": [],
                "reranker": null
            }
        }))
        .send()
        .map_err(|err| err.to_string())?;
    let _ = assert_error_response(response)?;
    Ok(())
}

#[test]
fn query_contract_rejections() -> Result<(), String> {
    let Some(server) = TestServer::from_env() else {
        eprintln!("skipping: COCO_TEST_SERVER_URL not set");
        return Ok(());
    };
    let project = server.register_project("Contract Rejections")?;

    let response = server
        .api_post("/v1/docs/query", &project)
        .json(&serde_json::json!({
            "indexing_config": {
                "config_id": "inline",
                "chunking": {
                    "strategy_name": "fixed_token",
                    "chunk_size": 128,
                    "chunk_overlap": 16
                },
                "embedding": {
                    "model_name": "test",
                    "dimensions": 1536
                },
                "vector_metric": "l2",
                "index_params": null,
                "vector_backend": null
            },
            "intent": {
                "query_text": null,
                "query_embedding": make_embedding(1.0),
                "retrieval_mode": "vector",
                "top_k": 3,
                "hybrid_alpha": 0.2,
                "filters": [],
                "reranker": null
            }
        }))
        .send()
        .map_err(|err| err.to_string())?;
    let _ = assert_error_response(response)?;

    let response = server
        .admin_get("/v1/sys/health")
        .send()
        .map_err(|err| err.to_string())?;
    if !response.status().is_success() {
        return Err(format!("health check failed: {}", response.status()));
    }
    let body = response.text().map_err(|err| err.to_string())?;
    let health: HealthResponse = serde_json::from_str(&body).map_err(|err| err.to_string())?;
    let mismatch_kind = match health.vector_backend.kind.as_str() {
        "pg_vector" => "qdrant",
        "qdrant" => "pg_vector",
        other => {
            return Err(format!("unknown vector backend kind: {other}"));
        }
    };
    let response = server
        .api_post("/v1/docs/query", &project)
        .json(&serde_json::json!({
            "intent": {
                "query_text": null,
                "query_embedding": null,
                "retrieval_mode": "fts",
                "top_k": 3,
                "hybrid_alpha": 0.2,
                "filters": [],
                "reranker": null
            }
        }))
        .send()
        .map_err(|err| err.to_string())?;
    let error = assert_error_response(response)?;
    let kind = error
        .get("kind")
        .and_then(Value::as_str)
        .unwrap_or("");
    if kind != "user" {
        return Err(format!("expected user error, got {kind}"));
    }
    let response = server
        .api_post("/v1/docs/query", &project)
        .json(&serde_json::json!({
            "intent": {
                "query_text": null,
                "query_embedding": make_embedding(1.0),
                "retrieval_mode": "vector",
                "top_k": 3,
                "hybrid_alpha": 0.2,
                "filters": [],
                "reranker": null
            },
            "retrieval_config": {
                "retrieval_mode": "vector",
                "top_k": 3,
                "hybrid_alpha": 0.2,
                "reranker": null,
                "vector_backend": {
                    "kind": mismatch_kind,
                    "url": null,
                    "api_key": null,
                    "collection_prefix": null
                }
            }
        }))
        .send()
        .map_err(|err| err.to_string())?;
    let _ = assert_error_response(response)?;
    let response = server
        .admin_post("/v1/sys/configs")
        .json(&serde_json::json!({
            "org_id": project.org_id,
            "config": {
                "config_id": unique_id("bad"),
                "chunking": {
                    "strategy_name": "fixed_token",
                    "chunk_size": 128,
                    "chunk_overlap": 16
                },
                "embedding": {
                    "model_name": "test",
                    "dimensions": 1536
                },
                "vector_metric": "l2",
                "index_params": null,
                "vector_backend": {
                    "kind": mismatch_kind,
                    "url": null,
                    "api_key": null,
                    "collection_prefix": null
                }
            }
        }))
        .send()
        .map_err(|err| err.to_string())?;
    let _ = assert_error_response(response)?;

    query_with_filter(&server, &project, "org_id", "eq")?;
    query_with_filter(&server, &project, "project_id", "eq")?;
    query_with_filter(&server, &project, "version_id", "eq")?;
    query_with_filter(&server, &project, "config_id", "eq")?;
    query_with_filter(&server, &project, "content", "contains")?;

    for op in ["neq", "gt", "gte", "lt", "lte", "in"] {
        query_with_filter(&server, &project, "doc_id", op)?;
    }

    let response = server
        .api_post("/v1/docs/query", &project)
        .json(&serde_json::json!({
            "indexing_config_id": "Default",
            "intent": {
                "query_text": null,
                "query_embedding": make_embedding(1.0),
                "retrieval_mode": "vector",
                "top_k": 3,
                "hybrid_alpha": 0.2,
                "filters": [],
                "reranker": null
            }
        }))
        .send()
        .map_err(|err| err.to_string())?;
    let _ = assert_error_response(response)?;

    let response = server
        .api_post("/v1/docs/query", &project)
        .json(&serde_json::json!({
            "indexing_config_id": " default",
            "intent": {
                "query_text": null,
                "query_embedding": make_embedding(1.0),
                "retrieval_mode": "vector",
                "top_k": 3,
                "hybrid_alpha": 0.2,
                "filters": [],
                "reranker": null
            }
        }))
        .send()
        .map_err(|err| err.to_string())?;
    let _ = assert_error_response(response)?;

    let response = server
        .api_post("/v1/docs/query", &project)
        .json(&serde_json::json!({
            "intent": {
                "query_text": null,
                "query_embedding": make_embedding(1.0),
                "retrieval_mode": "vector",
                "top_k": 3,
                "hybrid_alpha": 0.2,
                "filters": [],
                "reranker": null
            },
            "retrieval_config": {
                "retrieval_mode": "fts",
                "top_k": 3,
                "hybrid_alpha": 0.2,
                "reranker": null,
                "vector_backend": null
            }
        }))
        .send()
        .map_err(|err| err.to_string())?;
    let _ = assert_error_response(response)?;

    Ok(())
}
