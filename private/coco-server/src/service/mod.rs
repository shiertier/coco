//! Server HTTP API for CoCo.

mod app;
mod config;
mod constants;
mod embedder;
mod handlers;
mod indexing;
mod ingest;
mod limiter;
mod middleware;
mod openapi;
mod query;
mod state;
mod status;
mod stream;
mod types;
mod utils;
mod vector_backend;
mod worker;
pub(crate) mod worker_ipc;

pub use app::run;
pub use config::ServerConfig;
pub use openapi::openapi_document;
