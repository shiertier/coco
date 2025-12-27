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
