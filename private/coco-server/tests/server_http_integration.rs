mod support;

use serde_json::Value;
use support::{build_ingest_payload, build_query_payload, make_embedding, TestProject, TestServer};

fn assert_query_contains_chunk(
    server: &TestServer,
    project: &TestProject,
    embedding: Vec<f32>,
    chunk_id: &str,
) -> Result<Value, String> {
    let response = server
        .api_post("/v1/docs/query", project)
        .json(&build_query_payload(embedding))
        .send()
        .map_err(|err| err.to_string())?;
    if !response.status().is_success() {
        return Err(format!("query failed: {}", response.status()));
    }
    let parsed = TestServer::parse_json(response)?;
    let results = parsed
        .get("data")
        .and_then(|data| data.get("results"))
        .and_then(Value::as_array)
        .ok_or_else(|| "missing results".to_string())?;
    let found = results.iter().any(|item| {
        item.get("chunk")
            .and_then(|chunk| chunk.get("id"))
            .and_then(Value::as_str)
            == Some(chunk_id)
    });
    if !found {
        return Err(format!("expected chunk {chunk_id} not found"));
    }
    Ok(parsed)
}

#[test]
fn ingest_and_query_roundtrip() -> Result<(), String> {
    let Some(server) = TestServer::from_env() else {
        eprintln!("skipping: COCO_TEST_SERVER_URL not set");
        return Ok(());
    };
    let project = server.register_project("Server Integration")?;
    if project.active_config_id != "default" {
        return Err(format!(
            "expected default config, got {}",
            project.active_config_id
        ));
    }
    if project.active_version_id.is_none() {
        return Err("expected active_version_id".to_string());
    }

    let ingest = build_ingest_payload("doc-integ", "chunk-integ", make_embedding(1.0), None);
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

    let response_json =
        assert_query_contains_chunk(&server, &project, make_embedding(1.0), "chunk-integ")?;
    let status = response_json
        .get("meta")
        .and_then(|meta| meta.get("status"))
        .and_then(Value::as_str)
        .unwrap_or("missing");
    if status != "fresh" {
        return Err(format!("expected fresh response, got {status}"));
    }
    let results = response_json
        .get("data")
        .and_then(|data| data.get("results"))
        .and_then(Value::as_array)
        .ok_or_else(|| "missing results".to_string())?;
    for item in results {
        if let Some(chunk) = item.get("chunk") {
            if chunk.get("path").is_some() || chunk.get("line").is_some() {
                return Err("server response must not include path/line".to_string());
            }
        }
    }
    Ok(())
}

#[test]
fn multi_tenant_isolation() -> Result<(), String> {
    let Some(server) = TestServer::from_env() else {
        eprintln!("skipping: COCO_TEST_SERVER_URL not set");
        return Ok(());
    };

    let project_a = server.register_project("Tenant A")?;
    let project_b = server.register_project("Tenant B")?;

    let ingest_a = build_ingest_payload("doc-a", "chunk-a", make_embedding(1.0), None);
    let response_a = server
        .api_post("/v1/docs/import", &project_a)
        .json(&ingest_a)
        .send()
        .map_err(|err| err.to_string())?;
    let parsed_a = TestServer::parse_json(response_a)?;
    let job_id_a = parsed_a
        .get("job_id")
        .and_then(Value::as_str)
        .ok_or_else(|| "missing job_id for tenant A".to_string())?;
    server.wait_for_job(&project_a, job_id_a)?;

    let ingest_b = build_ingest_payload("doc-b", "chunk-b", make_embedding(2.0), None);
    let response_b = server
        .api_post("/v1/docs/import", &project_b)
        .json(&ingest_b)
        .send()
        .map_err(|err| err.to_string())?;
    let parsed_b = TestServer::parse_json(response_b)?;
    let job_id_b = parsed_b
        .get("job_id")
        .and_then(Value::as_str)
        .ok_or_else(|| "missing job_id for tenant B".to_string())?;
    server.wait_for_job(&project_b, job_id_b)?;

    assert_query_contains_chunk(&server, &project_a, make_embedding(1.0), "chunk-a")?;
    let response_b = assert_query_contains_chunk(&server, &project_b, make_embedding(2.0), "chunk-b")?;
    let results_b = response_b
        .get("data")
        .and_then(|data| data.get("results"))
        .and_then(Value::as_array)
        .ok_or_else(|| "missing results".to_string())?;
    let leaked = results_b.iter().any(|item| {
        item.get("chunk")
            .and_then(|chunk| chunk.get("id"))
            .and_then(Value::as_str)
            == Some("chunk-a")
    });
    if leaked {
        return Err("tenant isolation violated".to_string());
    }
    Ok(())
}
