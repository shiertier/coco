use coco_protocol::FilterOp;

pub(crate) const DEFAULT_HOST: &str = "127.0.0.1";
pub(crate) const DEFAULT_PORT: u16 = 3456;
pub(crate) const DEFAULT_RATE_LIMIT_PER_MIN: u32 = 120;
pub(crate) const DEFAULT_RATE_LIMIT_BURST: u32 = 60;
pub(crate) const ACCESS_LOG_SCHEMA_VERSION: u32 = 1;
pub(crate) const DEFAULT_QUEUE_MODE: &str = "postgres";
pub(crate) const DEFAULT_REDIS_QUEUE: &str = "coco:ingest";
pub(crate) const DEFAULT_CHUNK_SIZE: u32 = 256;
pub(crate) const DEFAULT_CHUNK_OVERLAP: u32 = 32;
pub(crate) const DEFAULT_CHUNK_STRATEGY: &str = "fixed_token";
pub(crate) const SERVER_FILTER_FIELDS: [&str; 2] = ["doc_id", "chunk_id"];
pub(crate) const SERVER_FILTER_OPS: [FilterOp; 2] = [FilterOp::Eq, FilterOp::Contains];
pub(crate) const WORKER_STATUS_ID: &str = "worker";
pub(crate) const WORKER_STALE_SECS: i64 = 90;
pub(crate) const WORKER_IPC_TIMEOUT_SECS: u64 = 3;
