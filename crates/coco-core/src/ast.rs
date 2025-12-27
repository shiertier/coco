//! Generic AST traversal and query helpers for Tree-sitter.

use coco_protocol::{CocoError, CocoResult};
use tree_sitter::{Language, Node, Query, QueryCursor, Tree, TreeCursor};

/// Parsed AST with owned source buffer.
#[derive(Debug, Clone)]
pub struct AstDocument {
    tree: Tree,
    source: Vec<u8>,
}

impl AstDocument {
    /// Creates a new AST document from a tree and the original source text.
    pub fn new(tree: Tree, source: impl Into<Vec<u8>>) -> Self {
        Self {
            tree,
            source: source.into(),
        }
    }

    /// Returns the root node wrapper.
    pub fn root(&self) -> AstNode<'_> {
        AstNode {
            node: self.tree.root_node(),
            source: &self.source,
        }
    }

    /// Returns the source buffer.
    pub fn source(&self) -> &[u8] {
        &self.source
    }

    /// Returns the underlying tree.
    pub fn tree(&self) -> &Tree {
        &self.tree
    }
}

/// Lightweight AST node wrapper with source access.
#[derive(Clone, Copy)]
pub struct AstNode<'a> {
    node: Node<'a>,
    source: &'a [u8],
}

impl<'a> AstNode<'a> {
    /// Returns the node kind.
    pub fn kind(&self) -> &str {
        self.node.kind()
    }

    /// Returns true if the node is named.
    pub fn is_named(&self) -> bool {
        self.node.is_named()
    }

    /// Returns the node byte range.
    pub fn byte_range(&self) -> (usize, usize) {
        let range = self.node.byte_range();
        (range.start, range.end)
    }

    /// Returns the node text as UTF-8.
    pub fn text(&self) -> CocoResult<&'a str> {
        self.node
            .utf8_text(self.source)
            .map_err(|err| CocoError::compute(format!("invalid utf8 in node text: {err}")))
    }

    /// Returns an iterator over child nodes.
    pub fn children(self) -> AstChildren<'a> {
        let cursor = self.node.walk();
        AstChildren {
            source: self.source,
            cursor,
            started: false,
        }
    }
}

impl std::fmt::Debug for AstNode<'_> {
    fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        formatter
            .debug_struct("AstNode")
            .field("kind", &self.node.kind())
            .field("byte_range", &self.node.byte_range())
            .finish()
    }
}

/// Iterator over child nodes.
pub struct AstChildren<'a> {
    source: &'a [u8],
    cursor: TreeCursor<'a>,
    started: bool,
}

impl<'a> Iterator for AstChildren<'a> {
    type Item = AstNode<'a>;

    fn next(&mut self) -> Option<Self::Item> {
        if !self.started {
            self.started = true;
            if !self.cursor.goto_first_child() {
                return None;
            }
            return Some(AstNode {
                node: self.cursor.node(),
                source: self.source,
            });
        }

        if self.cursor.goto_next_sibling() {
            return Some(AstNode {
                node: self.cursor.node(),
                source: self.source,
            });
        }
        None
    }
}

/// Query wrapper for extracting captures.
#[derive(Debug)]
pub struct AstQuery {
    query: Query,
    capture_names: Vec<String>,
}

impl AstQuery {
    /// Compiles a new query for the given language.
    pub fn new(language: Language, source: &str) -> CocoResult<Self> {
        let query = Query::new(language, source).map_err(|err| {
            CocoError::user(format!(
                "invalid tree-sitter query at {}:{}: {}",
                err.row, err.column, err.message
            ))
        })?;
        let capture_names = (0..query.capture_names().len())
            .map(|idx| query.capture_names()[idx].to_string())
            .collect();
        Ok(Self {
            query,
            capture_names,
        })
    }

    /// Runs the query against the given node and returns all captures.
    pub fn captures<'a>(
        &'a self,
        node: Node<'a>,
        source: &'a [u8],
    ) -> CocoResult<Vec<AstCapture>> {
        let mut cursor = QueryCursor::new();
        let mut captures = Vec::new();
        for (matched, capture_index) in cursor.captures(&self.query, node, |_| source) {
            if let Some(capture) = matched.captures.get(capture_index) {
                let name = self
                    .capture_names
                    .get(capture.index as usize)
                    .cloned()
                    .unwrap_or_else(|| capture.index.to_string());
                let range = capture.node.byte_range();
                let text = capture.node.utf8_text(source).map_err(|err| {
                    CocoError::compute(format!(
                        "invalid utf8 in capture {name} at {}..{}: {err}",
                        range.start, range.end
                    ))
                })?;
                captures.push(AstCapture {
                    name,
                    start_byte: range.start,
                    end_byte: range.end,
                    text: text.to_string(),
                });
            }
        }
        Ok(captures)
    }

    /// Returns capture names in declaration order.
    pub fn capture_names(&self) -> &[String] {
        &self.capture_names
    }
}

/// A single query capture with source offsets and text.
#[derive(Debug, Clone)]
pub struct AstCapture {
    /// Capture name from the query.
    pub name: String,
    /// Start byte offset in the source.
    pub start_byte: usize,
    /// End byte offset in the source.
    pub end_byte: usize,
    /// Captured text.
    pub text: String,
}
