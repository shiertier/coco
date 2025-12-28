//! Server-side components for CoCo.

/// HTTP embedding client for server mode.
pub mod embedder;
/// Optional queue adapters for ingest jobs.
pub mod queue;
/// HTTP API service for server mode.
pub mod service;
/// Storage adapters for server mode.
pub mod storage;
