use std::time::Duration;

use reqwest::blocking::{Client, RequestBuilder, Response};
use serde::Deserialize;
use serde_json::Value;
use uuid::Uuid;

const DEFAULT_ADMIN_KEY: &str = "admin";
const DEFAULT_API_KEY: &str = "api";

#[derive(Clone, Debug)]
pub struct TestServer {
    base_url: String,
    admin_key: String,
    api_key: String,
    client: Client,
}

#[derive(Clone, Debug)]
pub struct TestProject {
    pub org_id: String,
    pub user_id: String,
    pub project_id: String,
    pub active_config_id: String,
    pub active_version_id: Option<String>,
}

pub const EMBEDDING_DIM: usize = 1536;

#[derive(Debug, Deserialize)]
struct RegisterProjectResponse {
    project_id: String,
    org_id: String,
    active_config_id: String,
    active_version_id: Option<String>,
}

impl TestServer {
    pub fn from_env() -> Option<Self> {
        let base_url = std::env::var("COCO_TEST_SERVER_URL").ok()?;
        let admin_key =
            std::env::var("COCO_TEST_ADMIN_KEY").unwrap_or_else(|_| DEFAULT_ADMIN_KEY.to_string());
        let api_key =
            std::env::var("COCO_TEST_API_KEY").unwrap_or_else(|_| DEFAULT_API_KEY.to_string());
        let client = Client::builder()
            .timeout(Duration::from_secs(20))
            .build()
            .ok()?;
        Some(Self {
            base_url,
            admin_key,
            api_key,
            client,
        })
    }

    pub fn base_url(&self) -> &str {
        &self.base_url
    }

    pub fn admin_post(&self, path: &str) -> RequestBuilder {
        self.client
            .post(self.url(path))
            .bearer_auth(&self.admin_key)
    }

    pub fn admin_get(&self, path: &str) -> RequestBuilder {
        self.client
            .get(self.url(path))
            .bearer_auth(&self.admin_key)
    }

    pub fn api_post(&self, path: &str, project: &TestProject) -> RequestBuilder {
        touch_project_fields(project);
        self.client
            .post(self.url(path))
            .bearer_auth(&self.api_key)
            .header("x-coco-org-id", &project.org_id)
            .header("x-coco-user-id", &project.user_id)
            .header("x-coco-project-id", &project.project_id)
    }

    pub fn api_get(&self, path: &str, project: &TestProject) -> RequestBuilder {
        touch_project_fields(project);
        self.client
            .get(self.url(path))
            .bearer_auth(&self.api_key)
            .header("x-coco-org-id", &project.org_id)
            .header("x-coco-user-id", &project.user_id)
            .header("x-coco-project-id", &project.project_id)
    }

    pub fn api_post_without_project(&self, path: &str) -> RequestBuilder {
        self.client.post(self.url(path)).bearer_auth(&self.api_key)
    }

    pub fn register_project(&self, name: &str) -> Result<TestProject, String> {
        let org_id = unique_id("org");
        let user_id = unique_id("user");
        let payload = serde_json::json!({
            "org_id": org_id,
            "user_id": user_id,
            "name": name,
            "source_ref": "src:test",
            "platform": "test",
        });
        let response = self
            .admin_post("/v1/sys/register")
            .json(&payload)
            .send()
            .map_err(|err| err.to_string())?;
        let status = response.status();
        let body = response.text().map_err(|err| err.to_string())?;
        if !status.is_success() {
            return Err(format!("register failed: {status} {body}"));
        }
        let parsed: RegisterProjectResponse =
            serde_json::from_str(&body).map_err(|err| err.to_string())?;
        Ok(TestProject {
            org_id: parsed.org_id,
            user_id,
            project_id: parsed.project_id,
            active_config_id: parsed.active_config_id,
            active_version_id: parsed.active_version_id,
        })
    }

    pub fn parse_json(response: Response) -> Result<Value, String> {
        let body = response.text().map_err(|err| err.to_string())?;
        serde_json::from_str(&body).map_err(|err| err.to_string())
    }

    pub fn vector_backend_kind(&self) -> Result<String, String> {
        let response = self
            .admin_get("/v1/sys/health")
            .send()
            .map_err(|err| err.to_string())?;
        if !response.status().is_success() {
            return Err(format!("health check failed: {}", response.status()));
        }
        let parsed = Self::parse_json(response)?;
        parsed
            .get("vector_backend")
            .and_then(|backend| backend.get("kind"))
            .and_then(Value::as_str)
            .map(|value| value.to_string())
            .ok_or_else(|| "health response missing vector_backend.kind".to_string())
    }

    pub fn wait_for_job(
        &self,
        project: &TestProject,
        job_id: &str,
    ) -> Result<(), String> {
        let path = format!("/v1/jobs/{job_id}");
        for _ in 0..60 {
            let response = self
                .api_get(&path, project)
                .send()
                .map_err(|err| err.to_string())?;
            let status = response.status();
            let body = response.text().map_err(|err| err.to_string())?;
            if !status.is_success() {
                return Err(format!("job status error: {status} {body}"));
            }
            let parsed: Value = serde_json::from_str(&body).map_err(|err| err.to_string())?;
            match parsed.get("status").and_then(Value::as_str) {
                Some("completed") => return Ok(()),
                Some("failed") => {
                    let error = parsed
                        .get("error")
                        .and_then(Value::as_str)
                        .unwrap_or("unknown");
                    return Err(format!("job failed: {error}"));
                }
                _ => {}
            }
            std::thread::sleep(Duration::from_millis(250));
        }
        Err("job did not complete in time".to_string())
    }

    fn url(&self, path: &str) -> String {
        format!("{}{}", self.base_url.trim_end_matches('/'), path)
    }
}

fn touch_project_fields(project: &TestProject) {
    let _ = project.active_config_id.as_str();
    let _ = project.active_version_id.as_deref();
}

pub fn unique_id(prefix: &str) -> String {
    format!("{prefix}-{}", Uuid::now_v7())
}

pub fn make_embedding(value: f32) -> Vec<f32> {
    let mut embedding = vec![0.0_f32; EMBEDDING_DIM];
    if let Some(first) = embedding.first_mut() {
        *first = value;
    }
    embedding
}

pub fn build_ingest_payload(
    doc_id: &str,
    chunk_id: &str,
    embedding: Vec<f32>,
    config_id: Option<&str>,
) -> Value {
    let mut payload = serde_json::json!({
        "activate": true,
        "documents": [{
            "doc_id": doc_id,
            "source_ref": "src:test",
            "title": "Doc",
            "chunks": [{
                "chunk_id": chunk_id,
                "content": "hello world",
                "embedding": embedding,
                "start": 0,
                "end": 11
            }]
        }]
    });
    if let Some(config_id) = config_id {
        payload["indexing_config_id"] = Value::String(config_id.to_string());
    }
    payload
}

pub fn build_query_payload(embedding: Vec<f32>) -> Value {
    serde_json::json!({
        "intent": {
            "query_text": null,
            "query_embedding": embedding,
            "retrieval_mode": "vector",
            "top_k": 5,
            "hybrid_alpha": 0.5,
            "filters": [],
            "reranker": null
        }
    })
}

pub fn upsert_config(
    server: &TestServer,
    project: &TestProject,
    config_id: &str,
    vector_metric: &str,
) -> Result<(), String> {
    let payload = serde_json::json!({
        "org_id": project.org_id,
        "config": {
            "config_id": config_id,
            "chunking": {
                "strategy_name": "fixed_token",
                "chunk_size": 64,
                "chunk_overlap": 0
            },
            "embedding": {
                "model_name": "test",
                "dimensions": 1536
            },
            "vector_metric": vector_metric,
            "index_params": null,
            "vector_backend": null
        }
    });
    let response = server
        .admin_post("/v1/sys/configs")
        .json(&payload)
        .send()
        .map_err(|err| err.to_string())?;
    if !response.status().is_success() {
        return Err(format!("config upsert failed: {}", response.status()));
    }
    Ok(())
}

pub fn activate_config(
    server: &TestServer,
    project: &TestProject,
    config_id: &str,
) -> Result<(), String> {
    let payload = serde_json::json!({
        "org_id": project.org_id,
        "user_id": project.user_id,
        "project_id": project.project_id,
        "config_id": config_id
    });
    let response = server
        .admin_post("/v1/sys/configs/activate")
        .json(&payload)
        .send()
        .map_err(|err| err.to_string())?;
    if !response.status().is_success() {
        return Err(format!("config activate failed: {}", response.status()));
    }
    Ok(())
}

pub fn query_with_config(
    server: &TestServer,
    project: &TestProject,
    config_id: &str,
    embedding: Vec<f32>,
) -> Result<Value, String> {
    let mut payload = build_query_payload(embedding);
    payload["indexing_config_id"] = Value::String(config_id.to_string());
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

pub fn contains_chunk(parsed: &Value, chunk_id: &str) -> bool {
    parsed
        .get("data")
        .and_then(|data| data.get("results"))
        .and_then(Value::as_array)
        .map(|results| {
            results.iter().any(|item| {
                item.get("chunk")
                    .and_then(|chunk| chunk.get("id"))
                    .and_then(Value::as_str)
                    == Some(chunk_id)
            })
        })
        .unwrap_or(false)
}

#[test]
fn support_helpers_smoke() {
    let client = Client::builder()
        .timeout(Duration::from_millis(200))
        .build()
        .expect("test client");
    let server = TestServer {
        base_url: "http://localhost".to_string(),
        admin_key: DEFAULT_ADMIN_KEY.to_string(),
        api_key: DEFAULT_API_KEY.to_string(),
        client,
    };
    let project = TestProject {
        org_id: "org".to_string(),
        user_id: "user".to_string(),
        project_id: "project".to_string(),
        active_config_id: "default".to_string(),
        active_version_id: Some("v1".to_string()),
    };

    let _ = server.base_url();
    let _ = server.admin_post("/v1/sys/register");
    let _ = server.admin_get("/v1/sys/health");
    let _ = server.api_post("/v1/docs/query", &project);
    let _ = server.api_get("/v1/docs/query", &project);
    let _ = server.api_post_without_project("/v1/memo/query");

    let _ = project.active_config_id.as_str();
    let _ = project.active_version_id.as_deref();

    let _ = unique_id("test");
    let embedding = make_embedding(0.0);
    assert_eq!(embedding.len(), EMBEDDING_DIM);

    let ingest = build_ingest_payload("doc", "chunk", embedding.clone(), Some("default"));
    let _ = ingest.get("documents").and_then(Value::as_array);

    let query = build_query_payload(embedding);
    let _ = query.get("intent").and_then(|intent| intent.get("retrieval_mode"));

    let parsed = serde_json::json!({
        "data": { "results": [ { "chunk": { "id": "chunk" } } ] }
    });
    assert!(contains_chunk(&parsed, "chunk"));

    let _ = upsert_config as fn(&TestServer, &TestProject, &str, &str) -> Result<(), String>;
    let _ = activate_config as fn(&TestServer, &TestProject, &str) -> Result<(), String>;
    let _ =
        query_with_config as fn(&TestServer, &TestProject, &str, Vec<f32>) -> Result<Value, String>;
    let _ = TestServer::vector_backend_kind as fn(&TestServer) -> Result<String, String>;
    let _ = TestServer::wait_for_job as fn(&TestServer, &TestProject, &str) -> Result<(), String>;
}
