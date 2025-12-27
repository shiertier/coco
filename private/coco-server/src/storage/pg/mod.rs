//! PostgreSQL + pgvector storage backend for server mode.

#[cfg(feature = "server-storage")]
mod enabled;
#[cfg(not(feature = "server-storage"))]
mod disabled;

#[cfg(feature = "server-storage")]
pub use enabled::{PgBackend, PgBackendConfig, PgExecutor, PgVectorMetric, TenantContext};
#[cfg(not(feature = "server-storage"))]
pub use disabled::{PgBackend, PgBackendConfig, PgExecutor, PgVectorMetric, TenantContext};
