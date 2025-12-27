#![cfg(feature = "local-storage")]

use std::path::PathBuf;
use std::sync::{Mutex, OnceLock};
use std::time::{Duration, SystemTime, UNIX_EPOCH};

use chrono::Utc;
use coco_local::metrics::LocalMetrics;
use coco_local::service::LocalServiceConfig;
use coco_local::storage::meta::{NewIndexingConfig, DEFAULT_CONFIG_ID};
use coco_protocol::{ChunkingStrategy, EmbeddingConfig, VectorMetric};

pub fn env_lock() -> &'static Mutex<()> {
    static LOCK: OnceLock<Mutex<()>> = OnceLock::new();
    LOCK.get_or_init(|| Mutex::new(()))
}

pub struct EnvSnapshot {
    values: Vec<(&'static str, Option<String>)>,
}

impl EnvSnapshot {
    pub fn capture(keys: &[&'static str]) -> Self {
        let values = keys
            .iter()
            .map(|key| (*key, std::env::var(*key).ok()))
            .collect();
        Self { values }
    }
}

impl Drop for EnvSnapshot {
    fn drop(&mut self) {
        for (key, value) in &self.values {
            if let Some(value) = value {
                std::env::set_var(key, value);
            } else {
                std::env::remove_var(key);
            }
        }
    }
}

pub fn temp_root(prefix: &str) -> PathBuf {
    let mut dir = std::env::temp_dir();
    let nanos = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .expect("system time")
        .as_nanos();
    dir.push(format!("{prefix}-{nanos}"));
    std::fs::create_dir_all(&dir).expect("create temp dir");
    dir
}

pub fn default_chunking() -> ChunkingStrategy {
    ChunkingStrategy {
        strategy_name: "fixed_token".to_string(),
        chunk_size: 8,
        chunk_overlap: 0,
    }
}

pub fn default_indexing_config(dimensions: u32) -> NewIndexingConfig {
    NewIndexingConfig {
        config_id: DEFAULT_CONFIG_ID.to_string(),
        chunking: default_chunking(),
        embedding: EmbeddingConfig {
            model_name: "stub".to_string(),
            dimensions: Some(dimensions),
        },
        vector_backend: None,
        vector_metric: VectorMetric::L2,
        index_params: None,
        created_at: Utc::now(),
    }
}

pub fn pick_port() -> u16 {
    let listener = std::net::TcpListener::bind("127.0.0.1:0").expect("bind port");
    listener.local_addr().expect("local addr").port()
}

pub struct ServiceRunner {
    pub root: PathBuf,
    pub port: u16,
    handle: tokio::task::JoinHandle<coco_protocol::CocoResult<()>>,
}

impl ServiceRunner {
    pub fn base_url(&self) -> String {
        format!("http://127.0.0.1:{}", self.port)
    }

    pub async fn wait_for_health(&self, client: &reqwest::Client) -> bool {
        let url = format!("{}/v1/sys/health", self.base_url());
        for _ in 0..10 {
            if let Ok(response) = client.get(&url).send().await {
                if response.status().is_success() {
                    return true;
                }
            }
            tokio::time::sleep(Duration::from_millis(50)).await;
        }
        false
    }

    pub async fn shutdown(self) {
        self.handle.abort();
        let _ = self.handle.await;
        let _ = std::fs::remove_dir_all(&self.root);
    }
}

pub fn spawn_service(root: PathBuf, port: u16) -> ServiceRunner {
    let config = LocalServiceConfig {
        host: "127.0.0.1".to_string(),
        port,
        meta_db_path: root.join("meta.db"),
        vector_path: root.join("vectors"),
        model_path: None,
        model_url: None,
        model_name: "stub".to_string(),
        model_file: "stub.onnx".to_string(),
        model_dimensions: 3,
        chunking: default_chunking(),
    };
    let metrics = std::sync::Arc::new(LocalMetrics::new());
    let handle = tokio::spawn(async move { coco_local::service::run(config, metrics).await });
    ServiceRunner { root, port, handle }
}
