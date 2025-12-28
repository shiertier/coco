use std::path::{Path, PathBuf};

use coco_protocol::{ChunkingStrategy, CocoError, CocoResult};
use tracing::warn;

use super::constants::{
    DEFAULT_CHUNK_OVERLAP, DEFAULT_CHUNK_SIZE, DEFAULT_DIMENSIONS, DEFAULT_HOST,
    DEFAULT_LIVE_GREP_ENABLED, DEFAULT_LIVE_GREP_MAX_RESULTS, DEFAULT_LIVE_GREP_TIMEOUT_MS,
    DEFAULT_LIVE_RETRIEVAL_ENABLED, DEFAULT_LIVE_RETRIEVAL_WINDOW_BYTES, DEFAULT_MODEL_FILE,
    DEFAULT_MODEL_NAME, DEFAULT_MODEL_URL, DEFAULT_PORT,
};
use crate::embedder::ModelSpec;
use crate::paths;

/// Configuration for the local HTTP service.
#[derive(Debug, Clone)]
pub struct LocalServiceConfig {
    /// Host to bind the HTTP server.
    pub host: String,
    /// Port to bind the HTTP server.
    pub port: u16,
    /// SQLite metadata database path.
    pub meta_db_path: PathBuf,
    /// LanceDB storage directory.
    pub vector_path: PathBuf,
    /// Optional ONNX model path for embedding.
    pub model_path: Option<PathBuf>,
    /// Optional URL override for downloading models.
    pub model_url: Option<String>,
    /// Model identifier used in responses.
    pub model_name: String,
    /// Model file name stored in the local cache.
    pub model_file: String,
    /// Expected embedding dimensions.
    pub model_dimensions: usize,
    /// Default chunking strategy.
    pub chunking: ChunkingStrategy,
    /// Whether to refresh stale hits from the filesystem.
    pub live_retrieval_enabled: bool,
    /// Byte window radius for fuzzy anchor.
    pub live_retrieval_window_bytes: usize,
    /// Whether to enable live grep fallback.
    pub live_grep_enabled: bool,
    /// Maximum number of live grep results.
    pub live_grep_max_results: usize,
    /// Timeout for live grep in milliseconds.
    pub live_grep_timeout_ms: u64,
}

impl LocalServiceConfig {
    /// Loads configuration from environment variables.
    pub fn from_env() -> CocoResult<Self> {
        let host = env_optional("COCO_HOST").unwrap_or_else(|| DEFAULT_HOST.to_string());
        let port = env_u16("COCO_PORT", DEFAULT_PORT)?;
        warn_if_mode_set();
        let data_dir = paths::data_dir()?;
        let meta_db_path = resolve_meta_db_path(&data_dir)?;
        let vector_path = resolve_vector_path(&data_dir);
        let model_path = env_optional("COCO_MODEL_PATH").map(PathBuf::from);
        let model_url = env_optional("COCO_MODEL_URL");
        let model_name =
            env_optional("COCO_MODEL_NAME").unwrap_or_else(|| DEFAULT_MODEL_NAME.to_string());
        let model_file =
            env_optional("COCO_MODEL_FILE").unwrap_or_else(|| DEFAULT_MODEL_FILE.to_string());
        let model_dimensions = env_usize("COCO_MODEL_DIMENSIONS", DEFAULT_DIMENSIONS)?;
        let chunk_size = env_u32("COCO_CHUNK_SIZE", DEFAULT_CHUNK_SIZE)?;
        let chunk_overlap = env_u32("COCO_CHUNK_OVERLAP", DEFAULT_CHUNK_OVERLAP)?;
        let live_retrieval_enabled = env_bool(
            "COCO_LIVE_RETRIEVAL_ENABLED",
            DEFAULT_LIVE_RETRIEVAL_ENABLED,
        )?;
        let live_retrieval_window_bytes = env_usize(
            "COCO_LIVE_RETRIEVAL_WINDOW_BYTES",
            DEFAULT_LIVE_RETRIEVAL_WINDOW_BYTES,
        )?;
        let live_grep_enabled = env_bool("COCO_LIVE_GREP_ENABLED", DEFAULT_LIVE_GREP_ENABLED)?;
        let live_grep_max_results =
            env_usize("COCO_LIVE_GREP_MAX_RESULTS", DEFAULT_LIVE_GREP_MAX_RESULTS)?;
        let live_grep_timeout_ms =
            env_u64("COCO_LIVE_GREP_TIMEOUT_MS", DEFAULT_LIVE_GREP_TIMEOUT_MS)?;
        let chunking = ChunkingStrategy {
            strategy_name: "fixed_token".to_string(),
            chunk_size,
            chunk_overlap,
        };

        Ok(Self {
            host,
            port,
            meta_db_path,
            vector_path,
            model_path,
            model_url,
            model_name,
            model_file,
            model_dimensions,
            chunking,
            live_retrieval_enabled,
            live_retrieval_window_bytes,
            live_grep_enabled,
            live_grep_max_results,
            live_grep_timeout_ms,
        })
    }

    /// Builds the default model specification for downloads.
    pub fn model_spec(&self) -> ModelSpec {
        ModelSpec {
            name: self.model_name.clone(),
            dimensions: self.model_dimensions,
            file_name: self.model_file.clone(),
            sha256: None,
            url: Some(DEFAULT_MODEL_URL.to_string()),
        }
    }
}

fn resolve_meta_db_path(data_dir: &Path) -> CocoResult<PathBuf> {
    let primary = env_optional("COCO_META_DB");
    let legacy = env_optional("COCO_DB_URL");
    let selected = match (primary, legacy) {
        (Some(primary), Some(legacy)) => {
            if primary != legacy {
                warn!("COCO_META_DB and COCO_DB_URL are both set; using COCO_META_DB");
            }
            Some(primary)
        }
        (Some(primary), None) => Some(primary),
        (None, Some(legacy)) => Some(legacy),
        (None, None) => None,
    };
    match selected {
        Some(value) => parse_meta_db_value(&value),
        None => Ok(data_dir.join("meta.db")),
    }
}

fn parse_meta_db_value(value: &str) -> CocoResult<PathBuf> {
    if value.starts_with("sqlite:") {
        return sqlite_path_from_url(value).ok_or_else(|| {
            CocoError::user("COCO_META_DB must be a sqlite file path or sqlite URL")
        });
    }
    Ok(PathBuf::from(value))
}

fn sqlite_path_from_url(value: &str) -> Option<PathBuf> {
    let value = value.strip_prefix("sqlite:")?;
    let value = value.trim_start_matches("//");
    let path_part = value.split('?').next().unwrap_or("");
    if path_part.is_empty() {
        return None;
    }
    Some(PathBuf::from(path_part))
}

fn resolve_vector_path(data_dir: &Path) -> PathBuf {
    let primary = env_optional("COCO_VECTOR_DIR");
    let legacy = env_optional("COCO_LANCEDB_PATH");
    match (primary, legacy) {
        (Some(primary), Some(legacy)) => {
            if primary != legacy {
                warn!("COCO_VECTOR_DIR and COCO_LANCEDB_PATH are both set; using COCO_VECTOR_DIR");
            }
            PathBuf::from(primary)
        }
        (Some(primary), None) => PathBuf::from(primary),
        (None, Some(legacy)) => PathBuf::from(legacy),
        (None, None) => data_dir.join("vectors"),
    }
}

fn warn_if_mode_set() {
    let Some(mode) = env_optional("COCO_MODE") else {
        return;
    };
    if !mode.eq_ignore_ascii_case("local") {
        warn!("COCO_MODE is ignored by coco-local (expected 'local' for compatibility)");
    } else {
        warn!("COCO_MODE is ignored by coco-local (compatibility only)");
    }
}

fn env_optional(key: &str) -> Option<String> {
    std::env::var(key).ok().filter(|value| !value.is_empty())
}

fn env_u16(key: &str, default: u16) -> CocoResult<u16> {
    match env_optional(key) {
        Some(value) => value
            .parse::<u16>()
            .map_err(|_| CocoError::user(format!("{key} must be a valid number"))),
        None => Ok(default),
    }
}

fn env_u64(key: &str, default: u64) -> CocoResult<u64> {
    match env_optional(key) {
        Some(value) => value
            .parse::<u64>()
            .map_err(|_| CocoError::user(format!("{key} must be a valid number"))),
        None => Ok(default),
    }
}

fn env_u32(key: &str, default: u32) -> CocoResult<u32> {
    match env_optional(key) {
        Some(value) => value
            .parse::<u32>()
            .map_err(|_| CocoError::user(format!("{key} must be a valid number"))),
        None => Ok(default),
    }
}

fn env_usize(key: &str, default: usize) -> CocoResult<usize> {
    match env_optional(key) {
        Some(value) => value
            .parse::<usize>()
            .map_err(|_| CocoError::user(format!("{key} must be a valid number"))),
        None => Ok(default),
    }
}

fn env_bool(key: &str, default: bool) -> CocoResult<bool> {
    match env_optional(key) {
        Some(value) => match value.to_ascii_lowercase().as_str() {
            "true" | "1" | "yes" | "on" => Ok(true),
            "false" | "0" | "no" | "off" => Ok(false),
            _ => Err(CocoError::user(format!("{key} must be a valid bool"))),
        },
        None => Ok(default),
    }
}
