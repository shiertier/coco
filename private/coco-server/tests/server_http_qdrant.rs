mod support;

use qdrant_client::qdrant::{vectors_config, Distance};
use qdrant_client::Qdrant;
use serde_json::Value;
use sha2::{Digest, Sha256};
use support::{
    build_ingest_payload, build_query_payload, contains_chunk, make_embedding, query_with_config,
    unique_id, upsert_config, TestServer,
};

struct QdrantEnv {
    url: String,
    api_key: Option<String>,
    collection_prefix: String,
}

fn qdrant_env() -> Option<QdrantEnv> {
    let url = std::env::var("COCO_VECTOR_DB_URL").ok()?;
    let collection_prefix = std::env::var("COCO_VECTOR_DB_COLLECTION_PREFIX").ok()?;
    let api_key = std::env::var("COCO_VECTOR_DB_API_KEY").ok();
    Some(QdrantEnv {
        url,
        api_key,
        collection_prefix,
    })
}

fn is_qdrant_backend(server: &TestServer) -> Result<bool, String> {
    let kind = server.vector_backend_kind()?;
    Ok(kind == "qdrant")
}

fn join_name(prefix: &str, parts: &[&str]) -> String {
    let mut out = String::new();
    let prefix = prefix.trim_end_matches('_');
    if !prefix.is_empty() {
        out.push_str(prefix);
    }
    for part in parts {
        if !out.is_empty() {
            out.push('_');
        }
        out.push_str(part);
    }
    out
}

fn safe_collection_name(base: String) -> String {
    const MAX_COLLECTION_NAME_LEN: usize = 63;
    if base.len() <= MAX_COLLECTION_NAME_LEN {
        return base;
    }
    let mut hasher = Sha256::new();
    hasher.update(base.as_bytes());
    let digest = hasher.finalize();
    let suffix = hex8(&digest[..4]);
    let max_prefix_len = MAX_COLLECTION_NAME_LEN.saturating_sub(suffix.len() + 1);
    let trimmed = &base[..max_prefix_len];
    format!("{trimmed}_{suffix}")
}

fn hex8(bytes: &[u8]) -> String {
    let mut output = String::with_capacity(bytes.len() * 2);
    for byte in bytes {
        output.push(hex_digit(byte >> 4));
        output.push(hex_digit(byte & 0x0f));
    }
    output
}

fn hex_digit(value: u8) -> char {
    const HEX: &[u8; 16] = b"0123456789abcdef";
    HEX[value as usize] as char
}

fn collection_name(
    prefix: &str,
    org_id: &str,
    user_id: &str,
    project_id: &str,
    version_id: &str,
    config_id: &str,
) -> String {
    let base = join_name(
        prefix,
        &[org_id, user_id, project_id, version_id, config_id],
    );
    safe_collection_name(base)
}

fn collection_distance(env: &QdrantEnv, name: &str) -> Result<i32, String> {
    let mut builder = Qdrant::from_url(&env.url);
    if let Some(api_key) = env.api_key.as_ref() {
        builder = builder.api_key(api_key.clone());
    }
    let client = builder.build().map_err(|err| err.to_string())?;
    let runtime = tokio::runtime::Builder::new_current_thread()
        .enable_all()
        .build()
        .map_err(|err| err.to_string())?;
    let info = runtime.block_on(async {
        client
            .collection_info(name)
            .await
            .map_err(|err| err.to_string())
    })?;
    let Some(info) = info.result else {
        return Err("qdrant collection info missing".to_string());
    };
    let Some(config) = info.config else {
        return Err("qdrant collection config missing".to_string());
    };
    let Some(params) = config.params else {
        return Err("qdrant collection params missing".to_string());
    };
    let Some(vectors_config) = params.vectors_config else {
        return Err("qdrant vectors config missing".to_string());
    };
    let params = match vectors_config.config {
        Some(vectors_config::Config::Params(params)) => params,
        Some(vectors_config::Config::ParamsMap(_)) => {
            return Err("qdrant named vectors are not supported".to_string());
        }
        None => return Err("qdrant vectors config missing".to_string()),
    };
    Ok(params.distance)
}

#[test]
fn qdrant_query_returns_meta() -> Result<(), String> {
    let Some(server) = TestServer::from_env() else {
        eprintln!("skipping: COCO_TEST_SERVER_URL not set");
        return Ok(());
    };
    if !is_qdrant_backend(&server)? {
        eprintln!("skipping: vector backend is not qdrant");
        return Ok(());
    }
    let project = server.register_project("Qdrant Meta")?;

    let ingest =
        build_ingest_payload("doc-qdrant-meta", "chunk-qdrant-meta", make_embedding(1.0), None);
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

    let response = server
        .api_post("/v1/docs/query", &project)
        .json(&build_query_payload(make_embedding(1.0)))
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
    let hit = results
        .iter()
        .find(|item| {
            item.get("chunk")
                .and_then(|chunk| chunk.get("id"))
                .and_then(Value::as_str)
                == Some("chunk-qdrant-meta")
        })
        .ok_or_else(|| "expected chunk not found".to_string())?;
    let meta = hit
        .get("meta")
        .and_then(Value::as_object)
        .ok_or_else(|| "missing hit meta".to_string())?;
    let score = meta
        .get("score")
        .and_then(Value::as_f64)
        .ok_or_else(|| "missing hit score".to_string())?;
    if !(0.0..=1.0).contains(&score) {
        return Err(format!("score out of range: {score}"));
    }
    if !meta.contains_key("quality") || !meta.contains_key("verified") {
        return Err("missing quality/verified in hit meta".to_string());
    }
    Ok(())
}

#[test]
fn qdrant_tenant_isolation() -> Result<(), String> {
    let Some(server) = TestServer::from_env() else {
        eprintln!("skipping: COCO_TEST_SERVER_URL not set");
        return Ok(());
    };
    if !is_qdrant_backend(&server)? {
        eprintln!("skipping: vector backend is not qdrant");
        return Ok(());
    }

    let project_a = server.register_project("Qdrant Tenant A")?;
    let project_b = server.register_project("Qdrant Tenant B")?;

    let ingest_a =
        build_ingest_payload("doc-qa", "chunk-qa", make_embedding(1.0), None);
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

    let ingest_b =
        build_ingest_payload("doc-qb", "chunk-qb", make_embedding(2.0), None);
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

    let response_a = server
        .api_post("/v1/docs/query", &project_a)
        .json(&build_query_payload(make_embedding(1.0)))
        .send()
        .map_err(|err| err.to_string())?;
    let parsed_a = TestServer::parse_json(response_a)?;
    if !contains_chunk(&parsed_a, "chunk-qa") || contains_chunk(&parsed_a, "chunk-qb") {
        return Err("tenant A isolation violated".to_string());
    }

    let response_b = server
        .api_post("/v1/docs/query", &project_b)
        .json(&build_query_payload(make_embedding(2.0)))
        .send()
        .map_err(|err| err.to_string())?;
    let parsed_b = TestServer::parse_json(response_b)?;
    if !contains_chunk(&parsed_b, "chunk-qb") || contains_chunk(&parsed_b, "chunk-qa") {
        return Err("tenant B isolation violated".to_string());
    }
    Ok(())
}

#[test]
fn qdrant_config_isolation() -> Result<(), String> {
    let Some(server) = TestServer::from_env() else {
        eprintln!("skipping: COCO_TEST_SERVER_URL not set");
        return Ok(());
    };
    if !is_qdrant_backend(&server)? {
        eprintln!("skipping: vector backend is not qdrant");
        return Ok(());
    }
    let project = server.register_project("Qdrant Config Isolation")?;
    let config_a = unique_id("qcfg-a");
    let config_b = unique_id("qcfg-b");
    upsert_config(&server, &project, &config_a, "l2")?;
    upsert_config(&server, &project, &config_b, "l2")?;

    let ingest_a =
        build_ingest_payload("doc-qcfg-a", "chunk-qcfg-a", make_embedding(1.0), Some(&config_a));
    let response_a = server
        .api_post("/v1/docs/import", &project)
        .json(&ingest_a)
        .send()
        .map_err(|err| err.to_string())?;
    let parsed_a = TestServer::parse_json(response_a)?;
    let job_id_a = parsed_a
        .get("job_id")
        .and_then(Value::as_str)
        .ok_or_else(|| "missing job_id for config A".to_string())?;
    server.wait_for_job(&project, job_id_a)?;

    let ingest_b =
        build_ingest_payload("doc-qcfg-b", "chunk-qcfg-b", make_embedding(2.0), Some(&config_b));
    let response_b = server
        .api_post("/v1/docs/import", &project)
        .json(&ingest_b)
        .send()
        .map_err(|err| err.to_string())?;
    let parsed_b = TestServer::parse_json(response_b)?;
    let job_id_b = parsed_b
        .get("job_id")
        .and_then(Value::as_str)
        .ok_or_else(|| "missing job_id for config B".to_string())?;
    server.wait_for_job(&project, job_id_b)?;

    let response_a = query_with_config(&server, &project, &config_a, make_embedding(1.0))?;
    if !contains_chunk(&response_a, "chunk-qcfg-a") || contains_chunk(&response_a, "chunk-qcfg-b")
    {
        return Err("config A isolation violated".to_string());
    }

    let response_b = query_with_config(&server, &project, &config_b, make_embedding(2.0))?;
    if !contains_chunk(&response_b, "chunk-qcfg-b") || contains_chunk(&response_b, "chunk-qcfg-a")
    {
        return Err("config B isolation violated".to_string());
    }
    Ok(())
}

#[test]
fn qdrant_vector_metric_mapping() -> Result<(), String> {
    let Some(server) = TestServer::from_env() else {
        eprintln!("skipping: COCO_TEST_SERVER_URL not set");
        return Ok(());
    };
    if !is_qdrant_backend(&server)? {
        eprintln!("skipping: vector backend is not qdrant");
        return Ok(());
    }
    let Some(env) = qdrant_env() else {
        eprintln!("skipping: qdrant env not configured");
        return Ok(());
    };
    let project = server.register_project("Qdrant Metric Mapping")?;
    let config_id = unique_id("qmetric");
    upsert_config(&server, &project, &config_id, "cosine")?;

    let ingest =
        build_ingest_payload("doc-qmetric", "chunk-qmetric", make_embedding(1.0), Some(&config_id));
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

    let version_id = project
        .active_version_id
        .as_deref()
        .ok_or_else(|| "missing active_version_id".to_string())?;
    let collection = collection_name(
        &env.collection_prefix,
        &project.org_id,
        &project.user_id,
        &project.project_id,
        version_id,
        &config_id,
    );
    let distance = collection_distance(&env, &collection)?;
    if distance != Distance::Cosine as i32 {
        return Err(format!(
            "expected cosine distance, got {distance} for {collection}"
        ));
    }
    Ok(())
}
