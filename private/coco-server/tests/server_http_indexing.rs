mod support;

use serde_json::Value;
use support::{
    TestServer, activate_config, build_ingest_payload, contains_chunk, make_embedding,
    query_with_config, unique_id, upsert_config,
};

#[test]
fn config_id_isolation_and_activation() -> Result<(), String> {
    let Some(server) = TestServer::from_env() else {
        eprintln!("skipping: COCO_TEST_SERVER_URL not set");
        return Ok(());
    };
    let project = server.register_project("Indexing Isolation")?;
    let config_a = unique_id("alpha");
    let config_b = unique_id("beta");
    upsert_config(&server, &project, &config_a, "l2")?;
    upsert_config(&server, &project, &config_b, "l2")?;

    let ingest_a = build_ingest_payload("doc-a", "chunk-a", make_embedding(1.0), Some(&config_a));
    let response = server
        .api_post("/v1/docs/import", &project)
        .json(&ingest_a)
        .send()
        .map_err(|err| err.to_string())?;
    let parsed = TestServer::parse_json(response)?;
    let job_id = parsed
        .get("job_id")
        .and_then(Value::as_str)
        .ok_or_else(|| "missing job_id for config_a".to_string())?;
    server.wait_for_job(&project, job_id)?;

    let ingest_b = build_ingest_payload("doc-b", "chunk-b", make_embedding(2.0), Some(&config_b));
    let response = server
        .api_post("/v1/docs/import", &project)
        .json(&ingest_b)
        .send()
        .map_err(|err| err.to_string())?;
    let parsed = TestServer::parse_json(response)?;
    let job_id = parsed
        .get("job_id")
        .and_then(Value::as_str)
        .ok_or_else(|| "missing job_id for config_b".to_string())?;
    server.wait_for_job(&project, job_id)?;

    let result_a = query_with_config(&server, &project, &config_a, make_embedding(1.0))?;
    let status = result_a
        .get("meta")
        .and_then(|meta| meta.get("status"))
        .and_then(Value::as_str)
        .unwrap_or("missing");
    if status != "stale" {
        return Err(format!("expected stale status for config_a, got {status}"));
    }
    if !contains_chunk(&result_a, "chunk-a") || contains_chunk(&result_a, "chunk-b") {
        return Err("config_a results are not isolated".to_string());
    }

    let result_b = query_with_config(&server, &project, &config_b, make_embedding(2.0))?;
    let status = result_b
        .get("meta")
        .and_then(|meta| meta.get("status"))
        .and_then(Value::as_str)
        .unwrap_or("missing");
    if status != "stale" {
        return Err(format!("expected stale status for config_b, got {status}"));
    }
    if !contains_chunk(&result_b, "chunk-b") || contains_chunk(&result_b, "chunk-a") {
        return Err("config_b results are not isolated".to_string());
    }

    activate_config(&server, &project, &config_b)?;
    let refreshed = query_with_config(&server, &project, &config_b, make_embedding(2.0))?;
    let status = refreshed
        .get("meta")
        .and_then(|meta| meta.get("status"))
        .and_then(Value::as_str)
        .unwrap_or("missing");
    if status != "fresh" {
        return Err(format!(
            "expected fresh status after activation, got {status}"
        ));
    }

    Ok(())
}

#[test]
fn referenced_config_rejects_updates() -> Result<(), String> {
    let Some(server) = TestServer::from_env() else {
        eprintln!("skipping: COCO_TEST_SERVER_URL not set");
        return Ok(());
    };
    let project = server.register_project("Indexing Update Rejection")?;
    let config_id = unique_id("ref");
    upsert_config(&server, &project, &config_id, "l2")?;

    let ingest = build_ingest_payload(
        "doc-ref",
        "chunk-ref",
        make_embedding(3.0),
        Some(&config_id),
    );
    let response = server
        .api_post("/v1/docs/import", &project)
        .json(&ingest)
        .send()
        .map_err(|err| err.to_string())?;
    let parsed = TestServer::parse_json(response)?;
    let job_id = parsed
        .get("job_id")
        .and_then(Value::as_str)
        .ok_or_else(|| "missing job_id".to_string())?;
    server.wait_for_job(&project, job_id)?;

    let update_payload = serde_json::json!({
        "org_id": project.org_id,
        "config": {
            "config_id": config_id,
            "chunking": {
                "strategy_name": "fixed_token",
                "chunk_size": 32,
                "chunk_overlap": 0
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
        .json(&update_payload)
        .send()
        .map_err(|err| err.to_string())?;
    if !response.status().is_client_error() {
        return Err(format!(
            "expected update rejection, got {}",
            response.status()
        ));
    }

    Ok(())
}

#[test]
fn rejects_invalid_and_missing_configs() -> Result<(), String> {
    let Some(server) = TestServer::from_env() else {
        eprintln!("skipping: COCO_TEST_SERVER_URL not set");
        return Ok(());
    };
    let project = server.register_project("Indexing Missing Config")?;

    let ingest = build_ingest_payload(
        "doc-missing",
        "chunk-missing",
        make_embedding(4.0),
        Some("missing-config"),
    );
    let response = server
        .api_post("/v1/docs/import", &project)
        .json(&ingest)
        .send()
        .map_err(|err| err.to_string())?;
    if !response.status().is_client_error() {
        return Err(format!("expected 4xx, got {}", response.status()));
    }

    let default_payload = serde_json::json!({
        "org_id": project.org_id,
        "config": {
            "config_id": "default",
            "chunking": {
                "strategy_name": "fixed_token",
                "chunk_size": 64,
                "chunk_overlap": 0
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
        .json(&default_payload)
        .send()
        .map_err(|err| err.to_string())?;
    if !response.status().is_client_error() {
        return Err(format!(
            "expected default update rejection, got {}",
            response.status()
        ));
    }

    let response = server
        .admin_post("/v1/sys/configs/activate")
        .json(&serde_json::json!({
            "org_id": project.org_id,
            "user_id": project.user_id,
            "project_id": project.project_id,
            "config_id": "Default"
        }))
        .send()
        .map_err(|err| err.to_string())?;
    if !response.status().is_client_error() {
        return Err(format!(
            "expected invalid config id rejection, got {}",
            response.status()
        ));
    }

    Ok(())
}
