//! Generic chunking strategies.

use coco_protocol::{Chunker, ChunkingStrategy, CocoError, CocoResult, ParsedDocument, TextSpan};

use crate::text::token_spans;

/// Splits content by whitespace token count with optional overlap.
#[derive(Debug, Default)]
pub struct FixedTokenSplitter;

impl FixedTokenSplitter {
    /// Creates a new fixed-token splitter.
    pub fn new() -> Self {
        Self
    }
}

impl Chunker for FixedTokenSplitter {
    fn chunk(&self, doc: &ParsedDocument, config: &ChunkingStrategy) -> CocoResult<Vec<TextSpan>> {
        let chunk_size = config.chunk_size as usize;
        if chunk_size == 0 {
            return Err(CocoError::user("chunk_size must be greater than zero"));
        }

        let overlap = config.chunk_overlap as usize;
        if overlap >= chunk_size {
            return Err(CocoError::user("chunk_overlap must be smaller than chunk_size"));
        }

        let tokens = token_spans(&doc.content);
        if tokens.is_empty() {
            return Ok(Vec::new());
        }

        let step = chunk_size - overlap;
        let mut spans = Vec::new();
        let mut index = 0usize;

        while index < tokens.len() {
            let end_index = (index + chunk_size).min(tokens.len());
            let start = tokens[index].start();
            let end = tokens[end_index - 1].end();
            spans.push(TextSpan::new(start, end)?);

            if end_index == tokens.len() {
                break;
            }

            index = index.saturating_add(step);
        }

        Ok(spans)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use coco_protocol::FileType;

    fn doc(content: &str) -> ParsedDocument {
        ParsedDocument {
            content: content.to_string(),
            file_type: FileType::PlainText,
        }
    }

    #[test]
    fn fixed_token_splitter_rejects_invalid_config() {
        let splitter = FixedTokenSplitter::new();
        let mut config = ChunkingStrategy {
            strategy_name: "fixed_token".to_string(),
            chunk_size: 0,
            chunk_overlap: 0,
        };
        let result = splitter.chunk(&doc("one two"), &config);
        assert!(matches!(result, Err(CocoError::User { .. })));

        config.chunk_size = 2;
        config.chunk_overlap = 2;
        let result = splitter.chunk(&doc("one two three"), &config);
        assert!(matches!(result, Err(CocoError::User { .. })));
    }

    #[test]
    fn fixed_token_splitter_respects_overlap() {
        let splitter = FixedTokenSplitter::new();
        let config = ChunkingStrategy {
            strategy_name: "fixed_token".to_string(),
            chunk_size: 2,
            chunk_overlap: 1,
        };
        let doc = doc("one two three four five");
        let spans = splitter.chunk(&doc, &config).expect("split tokens");
        let texts: Vec<&str> = spans
            .iter()
            .map(|span| &doc.content[span.start()..span.end()])
            .collect();
        assert_eq!(texts, vec!["one two", "two three", "three four", "four five"]);
    }
}
