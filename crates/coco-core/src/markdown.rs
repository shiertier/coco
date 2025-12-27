//! Markdown parsing and chunking primitives.

use coco_protocol::{
    Chunker, ChunkingStrategy, CocoError, CocoResult, DocumentParser, FileType, ParsedDocument,
    TextSpan,
};
use tree_sitter::Node;

/// Parses Markdown content using tree-sitter.
#[derive(Debug, Default)]
pub struct MarkdownParser;

const MARKDOWN_SUPPORTED_TYPES: &[FileType] = &[FileType::Markdown];

impl MarkdownParser {
    /// Creates a new Markdown parser.
    pub fn new() -> Self {
        Self
    }
}

impl DocumentParser for MarkdownParser {
    fn parse(&self, content: &str, file_type: FileType) -> CocoResult<ParsedDocument> {
        if file_type != FileType::Markdown {
            return Err(CocoError::user("unsupported file type for MarkdownParser"));
        }

        let mut parser = tree_sitter::Parser::new();
        let language = tree_sitter_markdown::language();
        parser
            .set_language(language)
            .map_err(|_| CocoError::compute("failed to load markdown grammar"))?;

        let tree = parser
            .parse(content, None)
            .ok_or_else(|| CocoError::compute("failed to parse markdown content"))?;

        if tree.root_node().has_error() {
            return Err(CocoError::compute("markdown parse error"));
        }

        Ok(ParsedDocument {
            content: content.to_string(),
            file_type,
        })
    }

    fn supported_types(&self) -> &'static [FileType] {
        MARKDOWN_SUPPORTED_TYPES
    }
}

/// Splits Markdown documents into header-based chunks.
#[derive(Debug, Default)]
pub struct MarkdownSplitter;

impl MarkdownSplitter {
    /// Creates a new Markdown splitter.
    pub fn new() -> Self {
        Self
    }
}

impl Chunker for MarkdownSplitter {
    fn chunk(&self, doc: &ParsedDocument, _config: &ChunkingStrategy) -> CocoResult<Vec<TextSpan>> {
        if doc.file_type != FileType::Markdown {
            return Err(CocoError::user("unsupported file type for MarkdownSplitter"));
        }

        let content = doc.content.as_str();
        if content.is_empty() {
            return Ok(Vec::new());
        }

        let mut parser = tree_sitter::Parser::new();
        let language = tree_sitter_markdown::language();
        parser
            .set_language(language)
            .map_err(|_| CocoError::compute("failed to load markdown grammar"))?;

        let tree = parser
            .parse(content, None)
            .ok_or_else(|| CocoError::compute("failed to parse markdown content"))?;

        let root = tree.root_node();
        if root.has_error() {
            return Err(CocoError::compute("markdown parse error"));
        }

        let mut heading_starts = Vec::new();
        collect_heading_starts(root, false, &mut heading_starts);

        heading_starts.sort_unstable();
        heading_starts.dedup();

        if heading_starts.is_empty() {
            return Ok(vec![TextSpan {
                start: 0,
                end: content.len(),
            }]);
        }

        let mut spans = Vec::new();
        let mut iter = heading_starts.iter().copied();
        let first = iter.next().unwrap();

        if first > 0 {
            push_span(content, 0, first, &mut spans)?;
        }

        let mut prev = first;
        for start in iter {
            push_span(content, prev, start, &mut spans)?;
            prev = start;
        }

        push_span(content, prev, content.len(), &mut spans)?;
        Ok(spans)
    }
}

fn push_span(
    content: &str,
    start: usize,
    end: usize,
    spans: &mut Vec<TextSpan>,
) -> CocoResult<()> {
    if start >= end {
        return Ok(());
    }

    let slice = content
        .get(start..end)
        .ok_or_else(|| CocoError::compute("invalid span boundaries from parser"))?;

    if slice.trim().is_empty() {
        return Ok(());
    }

    spans.push(TextSpan { start, end });
    Ok(())
}

fn heading_level(node: &Node) -> Option<u8> {
    match node.kind() {
        "atx_heading" => atx_heading_level(node),
        "setext_heading" => setext_heading_level(node),
        _ => None,
    }
}

fn collect_heading_starts(node: Node, excluded: bool, starts: &mut Vec<usize>) {
    let kind = node.kind();
    let excluded_here = excluded
        || matches!(
            kind,
            "fenced_code_block"
                | "indented_code_block"
                | "table"
                | "tight_list"
                | "loose_list"
                | "list_item"
        );

    if !excluded_here {
        if let Some(level) = heading_level(&node) {
            if level <= 2 {
                starts.push(node.start_byte());
            }
        }
    }

    let mut cursor = node.walk();
    for child in node.children(&mut cursor) {
        collect_heading_starts(child, excluded_here, starts);
    }
}

fn atx_heading_level(node: &Node) -> Option<u8> {
    for index in 0..node.child_count() {
        let child = node.child(index)?;
        let level = match child.kind() {
            "atx_h1_marker" => 1,
            "atx_h2_marker" => 2,
            "atx_h3_marker" => 3,
            "atx_h4_marker" => 4,
            "atx_h5_marker" => 5,
            "atx_h6_marker" => 6,
            _ => continue,
        };
        return Some(level);
    }
    None
}

fn setext_heading_level(node: &Node) -> Option<u8> {
    for index in 0..node.child_count() {
        let child = node.child(index)?;
        let level = match child.kind() {
            "setext_h1_underline" => 1,
            "setext_h2_underline" => 2,
            _ => continue,
        };
        return Some(level);
    }
    None
}

#[cfg(test)]
mod tests {
    use super::*;

    fn strategy() -> ChunkingStrategy {
        ChunkingStrategy {
            strategy_name: "markdown_header".to_string(),
            chunk_size: 128,
            chunk_overlap: 0,
        }
    }

    #[test]
    fn markdown_parser_rejects_non_markdown() {
        let parser = MarkdownParser::new();
        let result = parser.parse("fn main() {}", FileType::Rust);
        assert!(matches!(result, Err(CocoError::User { .. })));
    }

    #[test]
    fn markdown_parser_accepts_markdown() {
        let parser = MarkdownParser::new();
        let doc = parser
            .parse("# Title\n\nBody\n", FileType::Markdown)
            .expect("parse markdown");
        assert_eq!(doc.file_type, FileType::Markdown);
        assert!(doc.content.contains("Title"));
    }

    #[test]
    fn markdown_splitter_skips_code_block_headings() {
        let content = "# Title\n\nIntro.\n\n```rust\n# Not a heading\n```\n\n## Section\nBody\n";
        let parser = MarkdownParser::new();
        let doc = parser
            .parse(content, FileType::Markdown)
            .expect("parse markdown");
        let splitter = MarkdownSplitter::new();
        let spans = splitter.chunk(&doc, &strategy()).expect("split markdown");
        assert_eq!(spans.len(), 2);

        let first = &doc.content[spans[0].start..spans[0].end];
        let second = &doc.content[spans[1].start..spans[1].end];
        assert!(first.contains("# Title"));
        assert!(!first.contains("## Section"));
        assert!(second.contains("## Section"));
    }
}
