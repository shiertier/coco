use criterion::{black_box, criterion_group, criterion_main, BenchmarkId, Criterion, Throughput};
use coco_core::chunking::FixedTokenSplitter;
use coco_core::code::{CodeParser, CodeSplitter};
use coco_core::markdown::{MarkdownParser, MarkdownSplitter};
use coco_core::text::{sanitize_text, token_windows, truncate_tokens};
use coco_protocol::{Chunker, ChunkingStrategy, DocumentParser, FileType, ParsedDocument};

fn markdown_sample() -> String {
    let mut content = String::from("# Title\n\nIntro paragraph.\n\n");
    for idx in 0..50 {
        content.push_str(&format!("## Section {idx}\n\n"));
        content.push_str("Some text for benchmarking.\n\n");
        content.push_str("```rust\nfn example() { println!(\"hi\"); }\n```\n\n");
    }
    content
}

fn code_sample() -> String {
    let mut content = String::from("use std::fmt;\n\n");
    for idx in 0..200 {
        content.push_str(&format!(
            "pub fn func_{idx}() {{ println!(\"{idx}\"); }}\n\n"
        ));
    }
    content
}

fn bench_markdown(c: &mut Criterion) {
    let content = markdown_sample();
    let parser = MarkdownParser::new();
    let splitter = MarkdownSplitter::new();
    let config = ChunkingStrategy {
        strategy_name: "markdown_header".to_string(),
        chunk_size: 128,
        chunk_overlap: 0,
    };

    c.bench_function("markdown_parse", |b| {
        b.iter(|| {
            let doc = parser
                .parse(black_box(&content), FileType::Markdown)
                .expect("parse markdown");
            black_box(doc);
        })
    });

    let doc = ParsedDocument {
        content: content.clone(),
        file_type: FileType::Markdown,
    };
    c.bench_function("markdown_chunk", |b| {
        b.iter(|| {
            let spans = splitter.chunk(black_box(&doc), black_box(&config)).unwrap();
            black_box(spans);
        })
    });
}

fn bench_code(c: &mut Criterion) {
    let content = code_sample();
    let parser = CodeParser::new();
    let splitter = CodeSplitter::new();
    let config = ChunkingStrategy {
        strategy_name: "code".to_string(),
        chunk_size: 256,
        chunk_overlap: 32,
    };

    c.bench_function("code_parse", |b| {
        b.iter(|| {
            let doc = parser
                .parse(black_box(&content), FileType::Rust)
                .expect("parse code");
            black_box(doc);
        })
    });

    let doc = ParsedDocument {
        content: content.clone(),
        file_type: FileType::Rust,
    };
    c.bench_function("code_chunk", |b| {
        b.iter(|| {
            let spans = splitter.chunk(black_box(&doc), black_box(&config)).unwrap();
            black_box(spans);
        })
    });
}

fn bench_text(c: &mut Criterion) {
    let content = "alpha  beta\tgamma\n\ndelta epsilon\n".repeat(200);
    c.bench_function("sanitize_text", |b| {
        b.iter(|| black_box(sanitize_text(black_box(&content))))
    });

    let mut group = c.benchmark_group("token_windows");
    group.throughput(Throughput::Bytes(content.len() as u64));
    group.bench_with_input(
        BenchmarkId::from_parameter("window_64"),
        &content,
        |b, input| {
            b.iter(|| {
                let spans = token_windows(black_box(input), 64, 16).unwrap();
                black_box(spans);
            })
        },
    );
    group.finish();

    c.bench_function("truncate_tokens", |b| {
        b.iter(|| black_box(truncate_tokens(black_box(&content), 128)))
    });
}

fn bench_fixed_token(c: &mut Criterion) {
    let content = "one two three four five six seven eight nine ten ".repeat(200);
    let doc = ParsedDocument {
        content: content.clone(),
        file_type: FileType::PlainText,
    };
    let splitter = FixedTokenSplitter::new();
    let config = ChunkingStrategy {
        strategy_name: "fixed_token".to_string(),
        chunk_size: 64,
        chunk_overlap: 16,
    };

    c.bench_function("fixed_token_split", |b| {
        b.iter(|| {
            let spans = splitter.chunk(black_box(&doc), black_box(&config)).unwrap();
            black_box(spans);
        })
    });
}

criterion_group!(
    benches,
    bench_markdown,
    bench_code,
    bench_text,
    bench_fixed_token
);
criterion_main!(benches);
