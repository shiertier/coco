//! HTTP embedding client for server mode.

use std::time::Duration;

use coco_protocol::{CocoError, CocoResult, EmbeddingModel};
use reqwest::StatusCode;
use reqwest::blocking::Client;
use serde::{Deserialize, Serialize};

const DEFAULT_OPENAI_URL: &str = "https://api.openai.com/v1/embeddings";
const DEFAULT_OPENAI_MODEL: &str = "text-embedding-3-small";
const DEFAULT_DIMENSIONS: usize = 1536;
const DEFAULT_BATCH_SIZE: usize = 64;
const DEFAULT_TIMEOUT_SECS: u64 = 30;
const DEFAULT_MAX_RETRIES: u32 = 3;
const DEFAULT_BACKOFF_MS: u64 = 500;
const DEFAULT_BACKOFF_MAX_MS: u64 = 8_000;

/// HTTP embedding provider settings.
#[derive(Debug, Clone)]
pub struct ProviderConfig {
    /// Provider name identifier (e.g., openai).
    pub name: String,
    /// API key used for authorization.
    pub api_key: String,
    /// Full embeddings endpoint URL.
    pub endpoint: String,
}

impl ProviderConfig {
    /// Loads provider settings from environment variables using the given prefix.
    pub fn from_env(prefix: &str, default_endpoint: Option<&str>) -> CocoResult<Self> {
        let api_key = env_required(&format!("{prefix}_API_KEY"))?;
        let endpoint = match env_optional(&format!("{prefix}_BASE_URL")) {
            Some(value) => value,
            None => default_endpoint
                .map(|value| value.to_string())
                .ok_or_else(|| CocoError::user("embedding base url must be provided"))?,
        };
        let name = prefix.trim_start_matches("COCO_").to_ascii_lowercase();
        Ok(Self {
            name,
            api_key,
            endpoint,
        })
    }
}

/// Configuration for the HTTP embedding client.
#[derive(Debug, Clone)]
pub struct HttpEmbedderConfig {
    /// Provider settings.
    pub provider: ProviderConfig,
    /// Model identifier for the embedding API.
    pub model: String,
    /// Expected embedding dimension.
    pub dimensions: usize,
    /// Maximum number of inputs per request.
    pub max_batch_size: usize,
    /// Request timeout.
    pub timeout: Duration,
    /// Maximum retry attempts for transient failures.
    pub max_retries: u32,
    /// Initial backoff delay.
    pub backoff_initial: Duration,
    /// Maximum backoff delay.
    pub backoff_max: Duration,
}

impl HttpEmbedderConfig {
    /// Loads an embedding configuration for the provider selected in env.
    pub fn from_env() -> CocoResult<Self> {
        let provider = env_optional("COCO_EMBEDDING_PROVIDER").unwrap_or_else(|| "openai".into());
        Self::from_env_for_provider(&provider)
    }

    /// Loads an embedding configuration for the given provider name.
    pub fn from_env_for_provider(provider: &str) -> CocoResult<Self> {
        let prefix = format!("COCO_{}", provider.to_ascii_uppercase());
        let default_endpoint = if provider.eq_ignore_ascii_case("openai") {
            Some(DEFAULT_OPENAI_URL)
        } else {
            None
        };
        let provider = ProviderConfig::from_env(&prefix, default_endpoint)?;
        let model = env_optional("COCO_EMBEDDING_MODEL")
            .unwrap_or_else(|| DEFAULT_OPENAI_MODEL.to_string());
        let dimensions = env_usize("COCO_EMBEDDING_DIMENSIONS", DEFAULT_DIMENSIONS)?;
        let max_batch_size = env_usize("COCO_EMBEDDING_BATCH_SIZE", DEFAULT_BATCH_SIZE)?;
        let timeout_secs = env_u64("COCO_EMBEDDING_TIMEOUT_SECS", DEFAULT_TIMEOUT_SECS)?;
        let max_retries = env_u32("COCO_EMBEDDING_MAX_RETRIES", DEFAULT_MAX_RETRIES)?;
        let backoff_ms = env_u64("COCO_EMBEDDING_BACKOFF_MS", DEFAULT_BACKOFF_MS)?;
        let backoff_max_ms = env_u64("COCO_EMBEDDING_BACKOFF_MAX_MS", DEFAULT_BACKOFF_MAX_MS)?;

        Ok(Self {
            provider,
            model,
            dimensions,
            max_batch_size,
            timeout: Duration::from_secs(timeout_secs),
            max_retries,
            backoff_initial: Duration::from_millis(backoff_ms),
            backoff_max: Duration::from_millis(backoff_max_ms),
        })
    }
}

/// HTTP embedding client backed by an external provider.
#[derive(Debug, Clone)]
pub struct HttpEmbedder {
    client: Client,
    config: HttpEmbedderConfig,
}

impl HttpEmbedder {
    /// Creates a new HTTP embedding client.
    pub fn new(config: HttpEmbedderConfig) -> CocoResult<Self> {
        if config.dimensions == 0 {
            return Err(CocoError::user(
                "embedding dimensions must be greater than zero",
            ));
        }
        if config.max_batch_size == 0 {
            return Err(CocoError::user(
                "embedding batch size must be greater than zero",
            ));
        }
        let client = Client::builder()
            .timeout(config.timeout)
            .user_agent("coco-http-embedder")
            .build()
            .map_err(CocoError::network)?;
        Ok(Self { client, config })
    }

    /// Builds a client using environment configuration.
    pub fn from_env() -> CocoResult<Self> {
        let config = HttpEmbedderConfig::from_env()?;
        Self::new(config)
    }
}

impl EmbeddingModel for HttpEmbedder {
    fn embed(&self, texts: &[&str]) -> CocoResult<Vec<Vec<f32>>> {
        if texts.is_empty() {
            return Ok(Vec::new());
        }
        let mut results = Vec::with_capacity(texts.len());
        for batch in texts.chunks(self.config.max_batch_size) {
            let request = EmbeddingRequest {
                model: self.config.model.as_str(),
                input: batch,
            };
            let response = self.request_embeddings(&request)?;
            let mut data = response.data;
            data.sort_by_key(|item| item.index);
            if data.len() != batch.len() {
                return Err(CocoError::compute("embedding response size mismatch"));
            }
            for item in data {
                if item.embedding.len() != self.config.dimensions {
                    return Err(CocoError::compute("embedding dimension mismatch"));
                }
                results.push(item.embedding);
            }
        }
        Ok(results)
    }

    fn dimensions(&self) -> usize {
        self.config.dimensions
    }

    fn model_name(&self) -> &str {
        self.config.model.as_str()
    }
}

impl HttpEmbedder {
    fn request_embeddings(&self, request: &EmbeddingRequest<'_>) -> CocoResult<EmbeddingResponse> {
        let body = serde_json::to_vec(request).map_err(CocoError::compute)?;
        let mut attempt = 0_u32;
        let mut backoff = self.config.backoff_initial;

        loop {
            attempt = attempt.saturating_add(1);
            let response = self
                .client
                .post(&self.config.provider.endpoint)
                .bearer_auth(&self.config.provider.api_key)
                .header("content-type", "application/json")
                .body(body.clone())
                .send()
                .map_err(CocoError::network)?;

            if response.status().is_success() {
                return parse_response(response);
            }

            if attempt > self.config.max_retries || !should_retry(response.status()) {
                let status = response.status();
                let detail = response.text().unwrap_or_default();
                return Err(CocoError::network(format!(
                    "embedding request failed ({status}): {detail}"
                )));
            }

            let wait = retry_after(&response).unwrap_or(backoff);
            std::thread::sleep(wait);
            backoff = std::cmp::min(backoff * 2, self.config.backoff_max);
        }
    }
}

#[derive(Serialize)]
struct EmbeddingRequest<'a> {
    model: &'a str,
    input: &'a [&'a str],
}

#[derive(Deserialize)]
struct EmbeddingResponse {
    data: Vec<EmbeddingData>,
}

#[derive(Deserialize)]
struct EmbeddingData {
    embedding: Vec<f32>,
    index: usize,
}

fn parse_response(response: reqwest::blocking::Response) -> CocoResult<EmbeddingResponse> {
    let bytes = response.bytes().map_err(CocoError::network)?;
    serde_json::from_slice(&bytes).map_err(CocoError::compute)
}

fn should_retry(status: StatusCode) -> bool {
    status == StatusCode::TOO_MANY_REQUESTS || status.is_server_error()
}

fn retry_after(response: &reqwest::blocking::Response) -> Option<Duration> {
    let value = response.headers().get("retry-after")?.to_str().ok()?;
    let secs = value.parse::<u64>().ok()?;
    Some(Duration::from_secs(secs))
}

fn env_required(key: &str) -> CocoResult<String> {
    env_optional(key).ok_or_else(|| CocoError::user(format!("{key} must be set")))
}

fn env_optional(key: &str) -> Option<String> {
    std::env::var(key).ok().filter(|value| !value.is_empty())
}

fn env_usize(key: &str, default: usize) -> CocoResult<usize> {
    match env_optional(key) {
        Some(value) => value
            .parse::<usize>()
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
