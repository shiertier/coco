//! Local HTTP service for CoCo.

mod app;
mod config;
mod constants;
mod embedder;
mod handlers;
mod live;
mod middleware;
mod query;
mod state;
mod types;

pub use app::run;
pub use config::LocalServiceConfig;
