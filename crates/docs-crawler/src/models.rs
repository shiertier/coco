use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::path::PathBuf;

#[derive(Debug, Clone)]
pub struct DefaultsConfig {
    pub allowed_extensions: Vec<String>,
    pub user_agent: String,
    pub timeout_seconds: u64,
    pub max_workers: usize,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EntryPointConfig {
    #[serde(rename = "type")]
    pub kind: String,
    pub url: Option<String>,
    pub language: Option<String>,
    pub repo: Option<String>,
    #[serde(rename = "ref")]
    pub r#ref: Option<String>,
    pub path: Option<String>,
    pub languages: Option<Vec<String>>,
    pub api_base_url: Option<String>,
    pub raw_base_url: Option<String>,
    pub language_variants: Option<Vec<String>>,
    pub git_cache_dir: Option<String>,
}

#[derive(Debug, Clone)]
pub struct SourceConfig {
    pub id: String,
    pub output_dir: PathBuf,
    pub default_language: String,
    pub entrypoints: Vec<EntryPointConfig>,
    pub allowed_extensions: Option<Vec<String>>,
    pub max_workers: Option<usize>,
}

#[derive(Debug, Clone)]
pub struct CrawlerConfig {
    pub defaults: DefaultsConfig,
    pub sources: Vec<SourceConfig>,
}

#[derive(Debug, Clone)]
pub struct DocumentRef {
    pub url: String,
    pub language: String,
    pub local_path: PathBuf,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(default)]
pub struct ManifestDocument {
    pub url: String,
    pub local_path: String,
    pub language: String,
    pub sha256: String,
    pub bytes: usize,
    pub etag: Option<String>,
    pub last_modified: Option<String>,
    pub fetched_at: Option<String>,
    pub checked_at: Option<String>,
    pub status: Option<String>,
    pub error: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(default)]
pub struct Manifest {
    pub version: i64,
    pub source_id: String,
    pub generated_at: String,
    pub entrypoints: Vec<serde_json::Value>,
    pub documents: HashMap<String, ManifestDocument>,
    pub documents_order: Vec<String>,
}
