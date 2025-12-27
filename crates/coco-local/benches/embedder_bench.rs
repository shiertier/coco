use criterion::{black_box, criterion_group, criterion_main, Criterion, Throughput};
use coco_local::embedder::LocalEmbedder;
use coco_protocol::EmbeddingModel;

fn bench_stub_embedder(c: &mut Criterion) {
    let embedder = LocalEmbedder::stub("stub", 384).expect("stub embedder");
    let texts: Vec<String> = (0..256)
        .map(|idx| format!("sample text {idx} for embedding benchmarks"))
        .collect();
    let refs: Vec<&str> = texts.iter().map(|text| text.as_str()).collect();

    let mut group = c.benchmark_group("embedder");
    group.throughput(Throughput::Elements(refs.len() as u64));
    group.bench_function("stub_embed", |b| {
        b.iter(|| {
            let output = embedder.embed(black_box(&refs)).expect("embed");
            black_box(output);
        })
    });
    group.finish();
}

criterion_group!(benches, bench_stub_embedder);
criterion_main!(benches);
