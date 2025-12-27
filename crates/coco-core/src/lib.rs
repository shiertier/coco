//! Pure parsing, chunking, and text processing logic for CoCo.

/// Markdown parsing and chunking utilities.
pub mod markdown;

/// Generic chunking utilities.
pub mod chunking;

/// Text sanitization and hashing helpers.
pub mod text;

/// Source code parsing and chunking utilities.
pub mod code;

/// Tree-sitter WASM runtime helpers.
pub mod wasm;

/// Tree-sitter AST traversal and query helpers.
pub mod ast;

/// Protocol validation helpers kept out of the DTO crate.
pub mod protocol_validation;

pub use protocol_validation::{
    build_search_intent, build_search_intent_from_parts, normalize_config_id,
    validate_indexing_config, validate_indexing_plan, validate_query_plan,
    validate_retrieval_config, validate_search_intent,
};
