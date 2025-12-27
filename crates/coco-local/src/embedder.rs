//! Local ONNX embedding model wrapper.

use std::collections::HashMap;
use std::fs::File;
use std::io::{Read, Write};
use std::path::{Path, PathBuf};
use std::sync::atomic::{AtomicBool, AtomicU64, Ordering};
use std::sync::{Arc, Mutex, OnceLock};

use coco_protocol::{CocoError, CocoResult, EmbeddingModel};
use ort::execution_providers::cpu::CPUExecutionProvider;
use ort::session::{Input, Output, Session};
use ort::tensor::TensorElementType;
use ort::value::{DynTensor, Tensor, ValueType};
use reqwest::blocking::Client;
use sha2::{Digest, Sha256};

/// ONNX Runtime backed embedding model for local mode.
#[derive(Debug)]
pub struct OrtEmbedder {
    session: Mutex<Session>,
    model_name: String,
    dimensions: usize,
    io: ModelIoSpec,
}

/// Supported embedding implementations for local mode.
#[derive(Debug, Clone)]
pub enum LocalEmbedder {
    /// ONNX Runtime embedder.
    Ort(Arc<OrtEmbedder>),
    /// Deterministic stub embedder for testing/offline flows.
    Stub(Arc<StubEmbedder>),
}

impl LocalEmbedder {
    /// Builds a stub embedder with deterministic outputs.
    pub fn stub(model_name: impl Into<String>, dimensions: usize) -> CocoResult<Self> {
        Ok(Self::Stub(Arc::new(StubEmbedder::new(
            model_name,
            dimensions,
        )?)))
    }
}

impl EmbeddingModel for LocalEmbedder {
    fn embed(&self, texts: &[&str]) -> CocoResult<Vec<Vec<f32>>> {
        match self {
            LocalEmbedder::Ort(embedder) => embedder.embed(texts),
            LocalEmbedder::Stub(embedder) => embedder.embed(texts),
        }
    }

    fn dimensions(&self) -> usize {
        match self {
            LocalEmbedder::Ort(embedder) => embedder.dimensions(),
            LocalEmbedder::Stub(embedder) => embedder.dimensions(),
        }
    }

    fn model_name(&self) -> &str {
        match self {
            LocalEmbedder::Ort(embedder) => embedder.model_name(),
            LocalEmbedder::Stub(embedder) => embedder.model_name(),
        }
    }
}

/// Deterministic stub embedding model for tests and offline checks.
#[derive(Debug)]
pub struct StubEmbedder {
    model_name: String,
    dimensions: usize,
}

impl StubEmbedder {
    /// Creates a new stub embedder.
    pub fn new(model_name: impl Into<String>, dimensions: usize) -> CocoResult<Self> {
        if dimensions == 0 {
            return Err(CocoError::user("embedding dimensions must be greater than zero"));
        }
        Ok(Self {
            model_name: model_name.into(),
            dimensions,
        })
    }

    fn embed_one(&self, text: &str) -> Vec<f32> {
        let mut output = vec![0.0_f32; self.dimensions];
        if !output.is_empty() {
            let value = (text.len() % 1000) as f32 / 1000.0;
            output[0] = value;
        }
        output
    }
}

impl EmbeddingModel for StubEmbedder {
    fn embed(&self, texts: &[&str]) -> CocoResult<Vec<Vec<f32>>> {
        Ok(texts.iter().map(|text| self.embed_one(text)).collect())
    }

    fn dimensions(&self) -> usize {
        self.dimensions
    }

    fn model_name(&self) -> &str {
        self.model_name.as_str()
    }
}

impl OrtEmbedder {
    /// Loads an ONNX model from a local file path.
    pub fn from_file<P: AsRef<Path>>(
        model_path: P,
        model_name: impl Into<String>,
        dimensions: usize,
    ) -> CocoResult<Self> {
        let config = OrtSessionConfig::from_env()?;
        Self::from_file_with_config(model_path, model_name, dimensions, config)
    }

    /// Loads an ONNX model with the provided session configuration.
    pub fn from_file_with_config<P: AsRef<Path>>(
        model_path: P,
        model_name: impl Into<String>,
        dimensions: usize,
        config: OrtSessionConfig,
    ) -> CocoResult<Self> {
        if dimensions == 0 {
            return Err(CocoError::user("embedding dimensions must be greater than zero"));
        }

        let path = model_path.as_ref();
        if !path.exists() {
            return Err(CocoError::user("embedding model file does not exist"));
        }

        let session = build_session(path, config)?;
        let io = ModelIoSpec::from_session(&session)?;

        Ok(Self {
            session: Mutex::new(session),
            model_name: model_name.into(),
            dimensions,
            io,
        })
    }
}

impl EmbeddingModel for OrtEmbedder {
    fn embed(&self, texts: &[&str]) -> CocoResult<Vec<Vec<f32>>> {
        if texts.is_empty() {
            return Ok(Vec::new());
        }

        let mut session = self
            .session
            .lock()
            .map_err(|_| CocoError::system("embedding session lock poisoned"))?;

        let (outputs, attention_mask) = match &self.io.input {
            InputMode::Text { name } => {
                let texts_owned: Vec<String> = texts.iter().map(|text| (*text).to_owned()).collect();
                let tensor = Tensor::from_string_array((
                    vec![texts_owned.len() as i64],
                    texts_owned.as_slice(),
                ))
                .map_err(CocoError::compute)?;
                let outputs = session
                    .run(vec![(name.clone(), tensor.upcast())])
                    .map_err(CocoError::compute)?;
                (outputs, None)
            }
            InputMode::TokenIds {
                input_ids,
                attention_mask,
                token_type_ids,
            } => {
                let batch = tokenize_batch(texts);
                let shape = vec![batch.batch_size as i64, batch.max_len as i64];
                let input_ids_tensor =
                    build_token_tensor(input_ids.element_type, &shape, batch.input_ids)?;
                let mut inputs = vec![(input_ids.name.clone(), input_ids_tensor)];

                let mask_snapshot = batch.attention_mask.clone();
                if let Some(mask_input) = attention_mask {
                    let mask_tensor =
                        build_token_tensor(mask_input.element_type, &shape, batch.attention_mask)?;
                    inputs.push((mask_input.name.clone(), mask_tensor));
                }
                if let Some(token_input) = token_type_ids {
                    let token_types = vec![0_i64; batch.max_len * batch.batch_size];
                    let token_tensor =
                        build_token_tensor(token_input.element_type, &shape, token_types)?;
                    inputs.push((token_input.name.clone(), token_tensor));
                }

                let outputs = session.run(inputs).map_err(CocoError::compute)?;
                (outputs, Some(mask_snapshot))
            }
        };

        let output = outputs
            .get(self.io.output.as_str())
            .or_else(|| if outputs.len() > 0 { Some(&outputs[0]) } else { None })
            .ok_or_else(|| CocoError::compute("embedding model produced no outputs"))?;
        let (shape, values) = output.try_extract_tensor::<f32>().map_err(CocoError::compute)?;
        extract_embeddings(shape, values, self.dimensions, attention_mask.as_deref())
    }

    fn dimensions(&self) -> usize {
        self.dimensions
    }

    fn model_name(&self) -> &str {
        self.model_name.as_str()
    }
}

/// ORT execution backend selection.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum OrtBackend {
    /// CPU execution provider.
    Cpu,
    /// CUDA execution provider.
    Cuda,
}

/// Session configuration for ONNX Runtime.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct OrtSessionConfig {
    /// Desired execution backend.
    pub backend: OrtBackend,
    /// Whether to enable the CPU arena allocator.
    pub cpu_arena: bool,
}

impl OrtSessionConfig {
    /// Builds a session config from environment variables.
    pub fn from_env() -> CocoResult<Self> {
        let backend = match std::env::var("COCO_ORT_BACKEND") {
            Ok(value) => parse_backend(&value)?,
            Err(_) => OrtBackend::Cpu,
        };

        Ok(Self {
            backend,
            cpu_arena: true,
        })
    }
}

/// Manages local ONNX model files.
#[derive(Debug, Clone)]
pub struct ModelStore {
    root: PathBuf,
}

impl ModelStore {
    /// Opens a model store at the given root path or the default location.
    pub fn open(root: Option<PathBuf>) -> CocoResult<Self> {
        let root = match root {
            Some(path) => path,
            None => default_model_root()?,
        };

        if !root.exists() {
            std::fs::create_dir_all(&root)
                .map_err(|err| CocoError::system(format!("failed to create model dir: {err}")))?;
        }

        Ok(Self { root })
    }

    /// Returns the full path for a model filename inside the store.
    pub fn model_path(&self, file_name: &str) -> PathBuf {
        self.root.join(file_name)
    }

    /// Ensures the model exists and returns its path.
    pub fn ensure_exists(&self, file_name: &str) -> CocoResult<PathBuf> {
        let path = self.model_path(file_name);
        if path.exists() {
            return Ok(path);
        }

        Err(CocoError::user("model file missing in local store"))
    }

    /// Ensures a model is present locally, downloading it if needed.
    pub fn ensure_model(
        &self,
        spec: &ModelSpec,
        override_url: Option<&str>,
        progress: Option<&DownloadProgress>,
    ) -> CocoResult<PathBuf> {
        let path = self.model_path(&spec.file_name);
        if path.exists() {
            if let Some(expected) = spec.sha256.as_deref() {
                verify_checksum(&path, expected)?;
            }
            return Ok(path);
        }

        let url = override_url
            .or(spec.url.as_deref())
            .ok_or_else(|| CocoError::user("model download url not provided"))?;

        let temp_path = path.with_extension("download");
        let download_result = download_file(url, &temp_path, progress);
        if let Some(progress) = progress {
            progress.mark_done();
        }
        download_result?;

        if let Some(expected) = spec.sha256.as_deref() {
            verify_checksum(&temp_path, expected)?;
        }

        std::fs::rename(&temp_path, &path).map_err(|err| {
            CocoError::system(format!("failed to finalize model download: {err}"))
        })?;

        Ok(path)
    }
}

/// Tracks download progress for model setup.
#[derive(Debug)]
pub struct DownloadProgress {
    downloaded: AtomicU64,
    total: AtomicU64,
    done: AtomicBool,
}

impl DownloadProgress {
    /// Creates a new progress tracker.
    pub fn new() -> Self {
        Self {
            downloaded: AtomicU64::new(0),
            total: AtomicU64::new(0),
            done: AtomicBool::new(false),
        }
    }

    /// Updates progress metrics.
    pub fn update(&self, downloaded: u64, total: Option<u64>) {
        self.downloaded.store(downloaded, Ordering::Relaxed);
        if let Some(total) = total {
            self.total.store(total, Ordering::Relaxed);
        }
    }

    /// Marks the download as completed.
    pub fn mark_done(&self) {
        self.done.store(true, Ordering::Relaxed);
    }

    /// Returns the current progress snapshot.
    pub fn snapshot(&self) -> (u64, Option<u64>, bool) {
        let downloaded = self.downloaded.load(Ordering::Relaxed);
        let total = self.total.load(Ordering::Relaxed);
        let done = self.done.load(Ordering::Relaxed);
        let total = if total == 0 { None } else { Some(total) };
        (downloaded, total, done)
    }
}

impl Default for DownloadProgress {
    fn default() -> Self {
        Self::new()
    }
}

fn default_model_root() -> CocoResult<PathBuf> {
    let home = std::env::var("HOME")
        .or_else(|_| std::env::var("USERPROFILE"))
        .map_err(|_| CocoError::system("failed to resolve home directory"))?;

    Ok(PathBuf::from(home).join(".coco").join("models"))
}

/// Global model pool for sharing ONNX sessions.
#[derive(Debug)]
pub struct ModelPool {
    store: ModelStore,
    models: Mutex<HashMap<String, ModelEntry>>,
}

impl ModelPool {
    /// Creates a new model pool with a given store.
    pub fn new(store: ModelStore) -> Self {
        Self {
            store,
            models: Mutex::new(HashMap::new()),
        }
    }

    /// Returns the global model pool instance.
    pub fn global() -> CocoResult<&'static ModelPool> {
        static GLOBAL: OnceLock<CocoResult<ModelPool>> = OnceLock::new();
        let stored = GLOBAL.get_or_init(|| ModelStore::open(None).map(ModelPool::new));
        match stored {
            Ok(pool) => Ok(pool),
            Err(err) => Err(err.clone()),
        }
    }

    /// Retrieves a model from the pool or loads it from disk.
    pub fn get_or_load(
        &self,
        spec: &ModelSpec,
        override_url: Option<&str>,
    ) -> CocoResult<Arc<OrtEmbedder>> {
        let key = spec.file_name.clone();
        let mut guard = self
            .models
            .lock()
            .map_err(|_| CocoError::system("model pool lock poisoned"))?;

        if let Some(entry) = guard.get(&key) {
            return Ok(Arc::clone(&entry.embedder));
        }

        let path = self.store.ensure_model(spec, override_url, None)?;
        let size = std::fs::metadata(&path)
            .map_err(|err| CocoError::system(format!("failed to read model metadata: {err}")))?
            .len();
        let embedder = OrtEmbedder::from_file(&path, spec.name.clone(), spec.dimensions)?;
        let shared = Arc::new(embedder);
        guard.insert(
            key,
            ModelEntry {
                embedder: Arc::clone(&shared),
                bytes: size,
            },
        );
        Ok(shared)
    }

    /// Returns aggregated model pool statistics.
    pub fn stats(&self) -> CocoResult<ModelPoolStats> {
        let guard = self
            .models
            .lock()
            .map_err(|_| CocoError::system("model pool lock poisoned"))?;

        let total_bytes = guard.values().map(|entry| entry.bytes).sum();
        Ok(ModelPoolStats {
            loaded_models: guard.len(),
            total_bytes,
        })
    }
}

#[derive(Debug, Clone)]
struct ModelEntry {
    embedder: Arc<OrtEmbedder>,
    bytes: u64,
}

/// Aggregated statistics for the model pool.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct ModelPoolStats {
    /// Number of loaded models.
    pub loaded_models: usize,
    /// Total size of loaded model files in bytes.
    pub total_bytes: u64,
}

/// Model download specification.
#[derive(Debug, Clone)]
pub struct ModelSpec {
    /// Human-readable model name.
    pub name: String,
    /// Embedding dimension count.
    pub dimensions: usize,
    /// Model file name inside the local store.
    pub file_name: String,
    /// Expected SHA-256 checksum (hex).
    pub sha256: Option<String>,
    /// Download URL for the model.
    pub url: Option<String>,
}

struct TokenizedBatch {
    input_ids: Vec<i64>,
    attention_mask: Vec<i64>,
    batch_size: usize,
    max_len: usize,
}

fn tokenize_batch(texts: &[&str]) -> TokenizedBatch {
    const MAX_TOKENS: usize = 256;
    const PAD_ID: i64 = 0;
    const CLS_ID: i64 = 101;
    const SEP_ID: i64 = 102;
    const TOKEN_OFFSET: i64 = 200;
    const TOKEN_BUCKETS: i64 = 1000;

    let mut samples = Vec::with_capacity(texts.len());
    let mut max_len = 0usize;

    for text in texts {
        let mut ids = Vec::new();
        ids.push(CLS_ID);
        for token in text.split_whitespace().take(MAX_TOKENS.saturating_sub(2)) {
            let hashed = token_hash(token) % TOKEN_BUCKETS;
            ids.push(TOKEN_OFFSET + hashed);
        }
        ids.push(SEP_ID);
        max_len = max_len.max(ids.len());
        samples.push(ids);
    }

    let batch_size = samples.len();
    let mut input_ids = Vec::with_capacity(batch_size * max_len);
    let mut attention_mask = Vec::with_capacity(batch_size * max_len);
    for ids in samples {
        for id in &ids {
            input_ids.push(*id);
            attention_mask.push(1);
        }
        if ids.len() < max_len {
            let pad = max_len - ids.len();
            input_ids.extend(std::iter::repeat_n(PAD_ID, pad));
            attention_mask.extend(std::iter::repeat_n(0, pad));
        }
    }

    TokenizedBatch {
        input_ids,
        attention_mask,
        batch_size,
        max_len,
    }
}

fn parse_backend(value: &str) -> CocoResult<OrtBackend> {
    match value.to_ascii_lowercase().as_str() {
        "" | "auto" | "cpu" => Ok(OrtBackend::Cpu),
        "cuda" => Ok(OrtBackend::Cuda),
        other => Err(CocoError::user(format!(
            "unsupported COCO_ORT_BACKEND value: {other}"
        ))),
    }
}

fn build_session(path: &Path, config: OrtSessionConfig) -> CocoResult<Session> {
    let mut builder = Session::builder().map_err(CocoError::compute)?;
    match config.backend {
        OrtBackend::Cpu => {
            let ep = CPUExecutionProvider::default()
                .with_arena_allocator(config.cpu_arena)
                .build();
            builder = builder
                .with_execution_providers([ep])
                .map_err(CocoError::compute)?;
        }
        OrtBackend::Cuda => {
            return Err(CocoError::user(
                "CUDA backend requires ort built with the cuda feature",
            ));
        }
    }

    builder.commit_from_file(path).map_err(CocoError::compute)
}

fn download_file(
    url: &str,
    dest: &Path,
    progress: Option<&DownloadProgress>,
) -> CocoResult<()> {
    let client = Client::builder()
        .build()
        .map_err(|err| CocoError::network(format!("failed to build http client: {err}")))?;

    let mut response = client
        .get(url)
        .send()
        .map_err(|err| CocoError::network(format!("download request failed: {err}")))?;

    if !response.status().is_success() {
        return Err(CocoError::network(format!(
            "download failed with status {}",
            response.status()
        )));
    }

    let total = response.content_length();
    if let Some(progress) = progress {
        progress.update(0, total);
    }

    let mut file = File::create(dest)
        .map_err(|err| CocoError::system(format!("failed to create model file: {err}")))?;

    let mut buffer = [0u8; 16 * 1024];
    let mut downloaded = 0u64;
    loop {
        let read = response
            .read(&mut buffer)
            .map_err(|err| CocoError::network(format!("download read failed: {err}")))?;
        if read == 0 {
            break;
        }
        file.write_all(&buffer[..read]).map_err(|err| {
            CocoError::system(format!("failed to write model file: {err}"))
        })?;
        downloaded = downloaded.saturating_add(read as u64);
        if let Some(progress) = progress {
            progress.update(downloaded, total);
        }
    }

    Ok(())
}

fn verify_checksum(path: &Path, expected: &str) -> CocoResult<()> {
    let actual = file_sha256(path)?;
    if actual != expected {
        return Err(CocoError::user("model checksum mismatch"));
    }
    Ok(())
}

fn file_sha256(path: &Path) -> CocoResult<String> {
    let mut file = File::open(path)
        .map_err(|err| CocoError::system(format!("failed to open model file: {err}")))?;
    let mut hasher = Sha256::new();
    let mut buffer = [0u8; 16 * 1024];
    loop {
        let read = file
            .read(&mut buffer)
            .map_err(|err| CocoError::system(format!("failed to read model file: {err}")))?;
        if read == 0 {
            break;
        }
        hasher.update(&buffer[..read]);
    }
    Ok(bytes_to_hex(&hasher.finalize()))
}

fn bytes_to_hex(bytes: &[u8]) -> String {
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

#[derive(Debug, Clone)]
struct ModelIoSpec {
    input: InputMode,
    output: String,
}

#[derive(Debug, Clone)]
enum InputMode {
    Text {
        name: String,
    },
    TokenIds {
        input_ids: TokenInput,
        attention_mask: Option<TokenInput>,
        token_type_ids: Option<TokenInput>,
    },
}

#[derive(Debug, Clone)]
struct TokenInput {
    name: String,
    element_type: TokenElementType,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum TokenElementType {
    I64,
    I32,
}

impl ModelIoSpec {
    fn from_session(session: &Session) -> CocoResult<Self> {
        if session.inputs.is_empty() || session.outputs.is_empty() {
            return Err(CocoError::compute(
                "embedding model session has no inputs or outputs",
            ));
        }

        let output = select_output_name(&session.outputs)?;

        let string_inputs: Vec<&Input> = session
            .inputs
            .iter()
            .filter(|input| matches_tensor_type(&input.input_type, TensorElementType::String))
            .collect();
        if !string_inputs.is_empty() {
            if session.inputs.len() != 1 {
                return Err(CocoError::user(
                    "string-input embedding model must define a single input",
                ));
            }
            return Ok(Self {
                input: InputMode::Text {
                    name: string_inputs[0].name.clone(),
                },
                output,
            });
        }

        let input_ids = find_input(&session.inputs, &["input_ids", "input"])
            .or_else(|| session.inputs.first())
            .ok_or_else(|| CocoError::compute("embedding model input_ids missing"))?;
        let attention_mask = find_input(&session.inputs, &["attention_mask", "mask"])
            .or_else(|| pick_by_index(&session.inputs, 1, &[input_ids.name.as_str()]));
        let token_type_ids = find_input(
            &session.inputs,
            &["token_type_ids", "token_type", "segment_ids"],
        )
        .or_else(|| pick_by_index(&session.inputs, 2, &[input_ids.name.as_str()]));

        Ok(Self {
            input: InputMode::TokenIds {
                input_ids: TokenInput::from_input(input_ids)?,
                attention_mask: attention_mask
                    .map(TokenInput::from_input)
                    .transpose()?,
                token_type_ids: token_type_ids
                    .map(TokenInput::from_input)
                    .transpose()?,
            },
            output,
        })
    }
}

impl TokenInput {
    fn from_input(input: &Input) -> CocoResult<Self> {
        let element_type = TokenElementType::from_value_type(&input.input_type)?;
        Ok(Self {
            name: input.name.clone(),
            element_type,
        })
    }
}

impl TokenElementType {
    fn from_value_type(value_type: &ValueType) -> CocoResult<Self> {
        match value_type {
            ValueType::Tensor { ty, .. } => match ty {
                TensorElementType::Int64 => Ok(Self::I64),
                TensorElementType::Int32 => Ok(Self::I32),
                _ => Err(CocoError::user("unsupported embedding input type")),
            },
            _ => Err(CocoError::user("unsupported embedding input type")),
        }
    }
}

fn build_token_tensor(
    element_type: TokenElementType,
    shape: &[i64],
    data: Vec<i64>,
) -> CocoResult<DynTensor> {
    match element_type {
        TokenElementType::I64 => Tensor::from_array((shape.to_vec(), data))
            .map_err(CocoError::compute)
            .map(Tensor::upcast),
        TokenElementType::I32 => {
            let mut values = Vec::with_capacity(data.len());
            for value in data {
                let cast = i32::try_from(value).map_err(|_| {
                    CocoError::user("token id out of range for int32 input")
                })?;
                values.push(cast);
            }
            Tensor::from_array((shape.to_vec(), values))
                .map_err(CocoError::compute)
                .map(Tensor::upcast)
        }
    }
}

fn select_output_name(outputs: &[Output]) -> CocoResult<String> {
    let preferred = [
        "sentence_embedding",
        "sentence_embeddings",
        "embedding",
        "embeddings",
        "pooler_output",
        "pooled_output",
        "output_0",
        "output",
    ];
    for candidate in preferred {
        if let Some(output) = outputs.iter().find(|out| matches_name(&out.name, candidate)) {
            return Ok(output.name.clone());
        }
    }
    outputs
        .first()
        .map(|output| output.name.clone())
        .ok_or_else(|| CocoError::compute("embedding model has no outputs"))
}

fn matches_name(name: &str, candidate: &str) -> bool {
    normalize_name(name) == candidate
}

fn normalize_name(name: &str) -> String {
    name.split(':')
        .next()
        .unwrap_or(name)
        .to_ascii_lowercase()
}

fn find_input<'a>(inputs: &'a [Input], names: &[&str]) -> Option<&'a Input> {
    for name in names {
        if let Some(input) = inputs.iter().find(|input| matches_name(&input.name, name)) {
            return Some(input);
        }
    }
    None
}

fn pick_by_index<'a>(
    inputs: &'a [Input],
    index: usize,
    excluded: &[&str],
) -> Option<&'a Input> {
    inputs.get(index).and_then(|input| {
        if excluded.contains(&input.name.as_str()) {
            None
        } else {
            Some(input)
        }
    })
}

fn matches_tensor_type(value_type: &ValueType, expected: TensorElementType) -> bool {
    matches!(
        value_type,
        ValueType::Tensor { ty, .. } if *ty == expected
    )
}

fn extract_embeddings(
    shape: &[i64],
    values: &[f32],
    expected_dim: usize,
    attention_mask: Option<&[i64]>,
) -> CocoResult<Vec<Vec<f32>>> {
    if shape.len() < 2 {
        return Err(CocoError::compute("embedding output has invalid rank"));
    }

    let batch = dim_to_usize(shape[0], "batch")?;
    if shape.len() == 2 {
        let dim = dim_to_usize(shape[1], "embedding")?;
        if dim != expected_dim {
            return Err(CocoError::compute("embedding dimension mismatch"));
        }
        let expected = batch
            .checked_mul(dim)
            .ok_or_else(|| CocoError::compute("embedding output size overflow"))?;
        if values.len() < expected {
            return Err(CocoError::compute("embedding output truncated"));
        }
        let mut outputs = Vec::with_capacity(batch);
        for idx in 0..batch {
            let start = idx * dim;
            outputs.push(values[start..start + dim].to_vec());
        }
        return Ok(outputs);
    }

    if shape.len() == 3 {
        let seq_len = dim_to_usize(shape[1], "sequence")?;
        let dim = dim_to_usize(shape[2], "embedding")?;
        if dim != expected_dim {
            return Err(CocoError::compute("embedding dimension mismatch"));
        }
        let expected = batch
            .checked_mul(seq_len)
            .and_then(|v| v.checked_mul(dim))
            .ok_or_else(|| CocoError::compute("embedding output size overflow"))?;
        if values.len() < expected {
            return Err(CocoError::compute("embedding output truncated"));
        }

        let mask = attention_mask.filter(|mask| mask.len() == batch * seq_len);
        let mut outputs = Vec::with_capacity(batch);
        for b in 0..batch {
            let mut pooled = vec![0.0; dim];
            let mut count = 0usize;
            for t in 0..seq_len {
                let mask_ok = mask
                    .map(|mask| mask[b * seq_len + t] != 0)
                    .unwrap_or(true);
                if !mask_ok {
                    continue;
                }
                count += 1;
                let offset = (b * seq_len + t) * dim;
                for d in 0..dim {
                    pooled[d] += values[offset + d];
                }
            }
            let denom = if count == 0 { 1.0 } else { count as f32 };
            for value in &mut pooled {
                *value /= denom;
            }
            outputs.push(pooled);
        }
        return Ok(outputs);
    }

    Err(CocoError::compute("embedding output rank not supported"))
}

fn dim_to_usize(value: i64, label: &str) -> CocoResult<usize> {
    if value <= 0 {
        return Err(CocoError::compute(format!(
            "embedding output {label} dimension is invalid"
        )));
    }
    Ok(value as usize)
}

fn token_hash(token: &str) -> i64 {
    let mut hash = 0u64;
    for byte in token.as_bytes() {
        hash = hash.wrapping_mul(31).wrapping_add(u64::from(*byte));
    }
    (hash & 0x7fff_ffff) as i64
}
