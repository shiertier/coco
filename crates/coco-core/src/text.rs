//! Text sanitization and hashing helpers.

use std::fmt::Write;

use coco_protocol::{CocoError, CocoResult, TextSpan};
use sha2::{Digest, Sha256};
use unicode_normalization::UnicodeNormalization;

/// Sanitizes text by removing control characters and normalizing whitespace.
pub fn sanitize_text(input: &str) -> String {
    let normalized: String = input.nfc().collect();
    let mut output = String::with_capacity(normalized.len());
    let mut last_space = false;
    let mut last_newline = false;

    for ch in normalized.chars() {
        let ch = if ch == '\r' { '\n' } else { ch };

        if ch.is_control() && ch != '\n' && ch != '\t' {
            continue;
        }

        if ch == '\n' {
            if !last_newline {
                output.push('\n');
            }
            last_newline = true;
            last_space = false;
            continue;
        }

        if ch.is_whitespace() {
            if !last_space {
                output.push(' ');
            }
            last_space = true;
            last_newline = false;
            continue;
        }

        output.push(ch);
        last_space = false;
        last_newline = false;
    }

    output
}

/// Computes a SHA-256 hex digest for the provided content.
pub fn content_hash(content: &str) -> String {
    let mut hasher = ContentHasher::new();
    hasher.update(content.as_bytes());
    hasher.finalize_hex()
}

/// Incremental SHA-256 hasher for streaming content.
#[derive(Debug, Default)]
pub struct ContentHasher {
    hasher: Sha256,
}

impl ContentHasher {
    /// Creates a new streaming SHA-256 hasher.
    pub fn new() -> Self {
        Self {
            hasher: Sha256::new(),
        }
    }

    /// Updates the hash state with the provided bytes.
    pub fn update(&mut self, bytes: &[u8]) {
        self.hasher.update(bytes);
    }

    /// Finalizes the hash and returns a lowercase hex digest.
    pub fn finalize_hex(self) -> String {
        let digest = self.hasher.finalize();
        bytes_to_hex(&digest)
    }
}

/// Truncates content to the given token count without splitting words.
pub fn truncate_tokens(content: &str, max_tokens: usize) -> String {
    if max_tokens == 0 {
        return String::new();
    }

    let tokens = token_spans(content);
    if tokens.len() <= max_tokens {
        return content.to_string();
    }

    let end = tokens[max_tokens - 1].end();
    content[..end].to_string()
}

/// Creates sliding windows of tokens with optional overlap.
pub fn token_windows(
    content: &str,
    window_size: usize,
    overlap: usize,
) -> CocoResult<Vec<TextSpan>> {
    if window_size == 0 {
        return Err(CocoError::user("window_size must be greater than zero"));
    }

    if overlap >= window_size {
        return Err(CocoError::user("overlap must be smaller than window_size"));
    }

    let tokens = token_spans(content);
    if tokens.is_empty() {
        return Ok(Vec::new());
    }

    let step = window_size - overlap;
    let mut spans = Vec::new();
    let mut index = 0usize;

    while index < tokens.len() {
        let end_index = (index + window_size).min(tokens.len());
        spans.push(TextSpan::new(
            tokens[index].start(),
            tokens[end_index - 1].end(),
        )?);

        if end_index == tokens.len() {
            break;
        }

        index = index.saturating_add(step);
    }

    Ok(spans)
}

pub(crate) fn token_spans(content: &str) -> Vec<TextSpan> {
    let mut spans = Vec::new();
    let mut in_token = false;
    let mut start = 0usize;

    for (index, ch) in content.char_indices() {
        if ch.is_whitespace() {
            if in_token {
                spans.push(TextSpan::new(start, index).expect("valid span"));
                in_token = false;
            }
        } else if !in_token {
            start = index;
            in_token = true;
        }
    }

    if in_token {
        spans.push(TextSpan::new(start, content.len()).expect("valid span"));
    }

    spans
}

fn bytes_to_hex(bytes: &[u8]) -> String {
    let mut output = String::with_capacity(bytes.len() * 2);
    for byte in bytes {
        let _ = write!(&mut output, "{byte:02x}");
    }
    output
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn sanitize_text_normalizes_whitespace() {
        let input = "a \t b\r\n\nc";
        let output = sanitize_text(input);
        assert_eq!(output, "a b\nc");
    }

    #[test]
    fn content_hash_matches_streaming_hasher() {
        let input = "hello world";
        let direct = content_hash(input);
        let mut hasher = ContentHasher::new();
        hasher.update(input.as_bytes());
        let streaming = hasher.finalize_hex();
        assert_eq!(direct, streaming);
    }

    #[test]
    fn truncate_tokens_respects_limit() {
        let input = "one two three";
        let output = truncate_tokens(input, 2);
        assert_eq!(output, "one two");
    }

    #[test]
    fn token_windows_respects_overlap() {
        let input = "one two three four";
        let windows = token_windows(input, 2, 1).expect("build windows");
        let texts: Vec<&str> = windows
            .iter()
            .map(|span| &input[span.start()..span.end()])
            .collect();
        assert_eq!(texts, vec!["one two", "two three", "three four"]);
    }

    #[test]
    fn token_windows_rejects_invalid_config() {
        let result = token_windows("one two", 2, 2);
        assert!(matches!(result, Err(CocoError::User { .. })));
    }
}
