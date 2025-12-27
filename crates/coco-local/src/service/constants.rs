pub(crate) const DEFAULT_HOST: &str = "127.0.0.1";
pub(crate) const DEFAULT_PORT: u16 = 3456;
pub(crate) const DEFAULT_DIMENSIONS: usize = 384;
pub(crate) const DEFAULT_CHUNK_SIZE: u32 = 256;
pub(crate) const DEFAULT_CHUNK_OVERLAP: u32 = 32;
pub(crate) const DEFAULT_MODEL_NAME: &str = "all-MiniLM-L6-v2";
pub(crate) const DEFAULT_MODEL_FILE: &str = "all-MiniLM-L6-v2.onnx";
pub(crate) const DEFAULT_MODEL_URL: &str =
    "https://huggingface.co/sentence-transformers/all-MiniLM-L6-v2/resolve/main/model.onnx";
pub(crate) const ACCESS_LOG_SCHEMA_VERSION: u32 = 1;
pub(crate) const DEFAULT_LIVE_RETRIEVAL_ENABLED: bool = true;
pub(crate) const DEFAULT_LIVE_RETRIEVAL_WINDOW_BYTES: usize = 2048;
pub(crate) const DEFAULT_LIVE_GREP_ENABLED: bool = false;
pub(crate) const DEFAULT_LIVE_GREP_MAX_RESULTS: usize = 10;
pub(crate) const DEFAULT_LIVE_GREP_TIMEOUT_MS: u64 = 200;
pub(crate) const DEFAULT_PRUNE_KEEP: usize = 3;
