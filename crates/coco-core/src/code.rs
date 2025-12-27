//! Source code parsing and chunking utilities.

use coco_protocol::{
    Chunker, ChunkingStrategy, CocoError, CocoResult, DocumentParser, FileType, ParsedDocument,
    TextSpan,
};

use crate::text::{token_spans, token_windows};

/// Parses source code using lightweight heuristics.
#[derive(Debug, Default)]
pub struct CodeParser;

impl CodeParser {
    /// Creates a new code parser.
    pub fn new() -> Self {
        Self
    }
}

impl DocumentParser for CodeParser {
    fn parse(&self, content: &str, file_type: FileType) -> CocoResult<ParsedDocument> {
        if !is_code_type(file_type) {
            return Err(CocoError::user("unsupported file type for CodeParser"));
        }

        Ok(ParsedDocument {
            content: content.to_string(),
            file_type,
        })
    }

    fn supported_types(&self) -> Vec<FileType> {
        vec![
            FileType::Rust,
            FileType::Python,
            FileType::TypeScript,
            FileType::Go,
        ]
    }
}

/// Splits source code into semantic chunks based on structural boundaries.
#[derive(Debug, Default)]
pub struct CodeSplitter;

impl CodeSplitter {
    /// Creates a new code splitter.
    pub fn new() -> Self {
        Self
    }
}

impl Chunker for CodeSplitter {
    fn chunk(&self, doc: &ParsedDocument, config: &ChunkingStrategy) -> CocoResult<Vec<TextSpan>> {
        if !is_code_type(doc.file_type) {
            return Err(CocoError::user("unsupported file type for CodeSplitter"));
        }

        let content = doc.content.as_str();
        if content.is_empty() {
            return Ok(Vec::new());
        }

        let chunk_size = config.chunk_size as usize;
        if chunk_size == 0 {
            return Err(CocoError::user("chunk_size must be greater than zero"));
        }

        let overlap = config.chunk_overlap as usize;
        if overlap >= chunk_size {
            return Err(CocoError::user("chunk_overlap must be smaller than chunk_size"));
        }

        let lines = collect_lines(content);
        let boundary_lines = boundary_line_indices(&lines, doc.file_type);

        if boundary_lines.is_empty() {
            return split_large_span(content, 0, content.len(), chunk_size, overlap);
        }

        let preamble_end = preamble_end_offset(&lines, doc.file_type);
        let include_preamble = preamble_end > 0 && preamble_end <= 8 * 1024;

        let mut spans = Vec::new();
        for (index, boundary_line) in boundary_lines.iter().enumerate() {
            let start_line = include_comment_block(&lines, *boundary_line, doc.file_type);
            let mut start = lines[start_line].start;
            if include_preamble && start > preamble_end {
                start = 0;
            }

            let end = if let Some(next_line) = boundary_lines.get(index + 1) {
                lines[*next_line].start
            } else {
                content.len()
            };

            let mut chunk_spans = split_large_span(content, start, end, chunk_size, overlap)?;
            spans.append(&mut chunk_spans);
        }

        Ok(spans)
    }
}

struct LineInfo<'a> {
    start: usize,
    end: usize,
    text: &'a str,
}

fn collect_lines(content: &str) -> Vec<LineInfo<'_>> {
    let mut lines = Vec::new();
    let mut start = 0usize;

    for line in content.split_inclusive('\n') {
        let end = start + line.len();
        let text = line.trim_end_matches('\n');
        lines.push(LineInfo { start, end, text });
        start = end;
    }

    if content.is_empty() || content.ends_with('\n') {
        return lines;
    }

    if let Some(last) = lines.last_mut() {
        last.end = content.len();
    }

    lines
}

fn boundary_line_indices(lines: &[LineInfo<'_>], file_type: FileType) -> Vec<usize> {
    let mut boundaries = Vec::new();
    for (index, line) in lines.iter().enumerate() {
        let trimmed = line.text.trim_start();
        if is_boundary(file_type, trimmed) {
            boundaries.push(index);
        }
    }
    boundaries
}

fn include_comment_block(lines: &[LineInfo<'_>], boundary_line: usize, file_type: FileType) -> usize {
    let mut start_line = boundary_line;
    while start_line > 0 {
        let previous = lines[start_line - 1].text.trim();
        if previous.is_empty() || is_comment_line(file_type, previous) {
            start_line -= 1;
            continue;
        }
        break;
    }
    start_line
}

fn preamble_end_offset(lines: &[LineInfo<'_>], file_type: FileType) -> usize {
    let mut end = 0usize;
    for line in lines {
        let trimmed = line.text.trim();
        if trimmed.is_empty() || is_comment_line(file_type, trimmed) {
            end = line.end;
            continue;
        }

        if is_preamble_line(file_type, trimmed) {
            end = line.end;
            continue;
        }

        break;
    }
    end
}

fn split_large_span(
    content: &str,
    start: usize,
    end: usize,
    chunk_size: usize,
    overlap: usize,
) -> CocoResult<Vec<TextSpan>> {
    if start >= end {
        return Ok(Vec::new());
    }

    let slice = content
        .get(start..end)
        .ok_or_else(|| CocoError::compute("invalid span boundaries in code splitter"))?;

    let tokens = token_spans(slice);
    if tokens.len() <= chunk_size {
        return Ok(vec![TextSpan { start, end }]);
    }

    let windows = token_windows(slice, chunk_size, overlap)?;
    let mut spans = Vec::with_capacity(windows.len());
    for window in windows {
        spans.push(TextSpan {
            start: start + window.start,
            end: start + window.end,
        });
    }

    Ok(spans)
}

fn is_code_type(file_type: FileType) -> bool {
    matches!(
        file_type,
        FileType::Rust | FileType::Python | FileType::TypeScript | FileType::Go
    )
}

fn is_boundary(file_type: FileType, line: &str) -> bool {
    if line.is_empty() {
        return false;
    }

    match file_type {
        FileType::Rust => is_rust_boundary(line),
        FileType::Python => is_python_boundary(line),
        FileType::TypeScript => is_typescript_boundary(line),
        FileType::Go => is_go_boundary(line),
        _ => false,
    }
}

fn is_comment_line(file_type: FileType, line: &str) -> bool {
    match file_type {
        FileType::Python => line.starts_with('#')
            || line.starts_with("\"\"\"")
            || line.starts_with("'''"),
        FileType::Rust | FileType::TypeScript | FileType::Go => {
            line.starts_with("//")
                || line.starts_with("/*")
                || line.starts_with('*')
                || line.starts_with("*/")
        }
        _ => false,
    }
}

fn is_preamble_line(file_type: FileType, line: &str) -> bool {
    match file_type {
        FileType::Rust => {
            line.starts_with("use ")
                || line.starts_with("pub use ")
                || line.starts_with("extern crate ")
                || line.starts_with("mod ")
        }
        FileType::Python => line.starts_with("import ") || line.starts_with("from "),
        FileType::TypeScript => {
            line.starts_with("import ")
                || line.starts_with("export ")
                || line.starts_with("type ")
                || line.starts_with("interface ")
        }
        FileType::Go => line.starts_with("package ") || line.starts_with("import "),
        _ => false,
    }
}

fn is_rust_boundary(line: &str) -> bool {
    let trimmed = strip_visibility(line);
    trimmed.starts_with("fn ")
        || trimmed.starts_with("struct ")
        || trimmed.starts_with("enum ")
        || trimmed.starts_with("trait ")
        || trimmed.starts_with("impl ")
        || trimmed.starts_with("mod ")
        || trimmed.starts_with("const ")
        || trimmed.starts_with("static ")
}

fn strip_visibility(line: &str) -> &str {
    if let Some(rest) = line.strip_prefix("pub ") {
        return rest;
    }
    if let Some(rest) = line.strip_prefix("pub(crate) ") {
        return rest;
    }
    if let Some(rest) = line.strip_prefix("pub(super) ") {
        return rest;
    }
    if let Some(rest) = line.strip_prefix("pub(in ") {
        if let Some(split) = rest.split_once(')') {
            return split.1.trim_start();
        }
    }
    line
}

fn is_python_boundary(line: &str) -> bool {
    line.starts_with("def ")
        || line.starts_with("async def ")
        || line.starts_with("class ")
}

fn is_typescript_boundary(line: &str) -> bool {
    line.starts_with("function ")
        || line.starts_with("class ")
        || line.starts_with("interface ")
        || line.starts_with("type ")
        || line.starts_with("enum ")
        || line.starts_with("export function ")
        || line.starts_with("export class ")
        || line.starts_with("export interface ")
        || line.starts_with("export type ")
        || line.starts_with("export enum ")
}

fn is_go_boundary(line: &str) -> bool {
    line.starts_with("func ")
        || line.starts_with("type ")
        || line.starts_with("var ")
        || line.starts_with("const ")
}
