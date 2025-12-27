//! Server-side components for CoCo.

/// HTTP embedding client for server mode.
pub mod embedder;
/// HTTP API service for server mode.
pub mod service;
/// Optional queue adapters for ingest jobs.
pub mod queue;
/// Storage adapters for server mode.
pub mod storage;
