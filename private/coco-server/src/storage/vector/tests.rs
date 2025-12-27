use super::rrf::merge_rrf;
use coco_protocol::{Chunk, ChunkId, DocumentId, SearchHit, SearchHitMeta, TextSpan};

fn make_hit(id: &str) -> SearchHit {
    SearchHit {
        meta: SearchHitMeta {
            score: 0.0,
            quality: None,
            verified: None,
        },
        chunk: Chunk {
            id: ChunkId::new(id),
            doc_id: DocumentId::new("doc"),
            content: "content".to_string(),
            embedding: None,
            span: TextSpan::new(0, 1).expect("span"),
            quality_score: None,
            verified: None,
        },
    }
}

#[test]
fn merge_rrf_sorts_by_descending_score() {
    let vector_results = vec![make_hit("a"), make_hit("b"), make_hit("c")];
    let merged = merge_rrf(vector_results, Vec::new(), 60, 3);
    assert_eq!(merged.len(), 3);
    assert_eq!(merged[0].chunk.id.as_str(), "a");
    assert_eq!(merged[1].chunk.id.as_str(), "b");
    assert_eq!(merged[2].chunk.id.as_str(), "c");
    assert!(merged[0].meta.score > merged[1].meta.score);
    assert!(merged[1].meta.score > merged[2].meta.score);
}
