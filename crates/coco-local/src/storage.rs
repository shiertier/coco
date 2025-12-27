//! Local storage adapters for metadata and vectors.

/// Local metadata store backed by SQLite.
#[path = "meta/mod.rs"]
pub mod meta;

/// LanceDB vector storage backend.
#[path = "lance.rs"]
pub mod lance;
