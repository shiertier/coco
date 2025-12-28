use std::num::NonZeroU32;

use coco_core::{
    build_search_intent, normalize_config_id, validate_indexing_config, validate_search_intent,
};
use coco_protocol::{
    ChunkId, ChunkingStrategy, CocoError, CocoErrorKind, DocumentId, EmbeddingConfig,
    ErrorResponse, Filter, FilterField, FilterOp, FilterValue, HnswParams, IndexingConfig,
    RerankerConfig, RetrievalMode, SearchIntent, SearchIntentInput, SearchQuery, SearchQueryInput,
    TextSpan, ValidationContext, VectorIndexParams, VectorMetadata, VectorMetric, VectorRecord,
};

fn sample_chunking() -> ChunkingStrategy {
    ChunkingStrategy {
        strategy_name: "fixed_token".to_string(),
        chunk_size: 256,
        chunk_overlap: 32,
    }
}

fn sample_embedding() -> EmbeddingConfig {
    EmbeddingConfig {
        model_name: "all-MiniLM-L6-v2".to_string(),
        dimensions: Some(384),
    }
}

fn sample_indexing_config(config_id: &str) -> IndexingConfig {
    IndexingConfig {
        config_id: config_id.to_string(),
        chunking: sample_chunking(),
        embedding: sample_embedding(),
        vector_metric: VectorMetric::Cosine,
        index_params: None,
        vector_backend: None,
    }
}

fn sample_intent(mode: RetrievalMode) -> SearchIntentInput {
    let query = match mode {
        RetrievalMode::Vector => {
            SearchQueryInput::vector(None, Some(vec![0.1, 0.2])).expect("query")
        }
        RetrievalMode::Fts => SearchQueryInput::fts("hello").expect("query"),
        RetrievalMode::Hybrid => {
            SearchQueryInput::hybrid("hello", Some(vec![0.1, 0.2])).expect("query")
        }
    };
    SearchIntentInput::new(query, None, 5, 0.5, Vec::new(), None).expect("intent")
}

#[test]
fn search_intent_indexing_config_id_defaults_to_none() {
    let json = serde_json::json!({
        "query_text": "hello",
        "retrieval_mode": "fts",
        "top_k": 3,
        "hybrid_alpha": 0.5,
        "filters": [],
        "reranker": null
    });
    let intent: SearchIntentInput = serde_json::from_value(json).expect("deserialize intent");
    assert!(intent.indexing_config_id.is_none());
}

#[test]
fn search_intent_serializes_indexing_config_id() {
    let intent = SearchIntentInput {
        indexing_config_id: Some("default".to_string()),
        ..sample_intent(RetrievalMode::Fts)
    };
    let value = serde_json::to_value(&intent).expect("serialize intent");
    assert_eq!(value["indexing_config_id"], "default");
}

#[test]
fn vector_record_serializes_config_id() {
    let record = VectorRecord {
        chunk_id: ChunkId::new("chunk-1"),
        embedding: vec![0.1, 0.2],
        metadata: VectorMetadata {
            config_id: Some("default".to_string()),
            doc_id: DocumentId::new("doc-1"),
            content: "hello".to_string(),
            span: TextSpan::new(0, 5).expect("span"),
        },
    };
    let value = serde_json::to_value(&record).expect("serialize record");
    assert_eq!(value["metadata"]["config_id"], "default");
    let decoded: VectorRecord = serde_json::from_value(value).expect("deserialize record");
    assert_eq!(decoded, record);
}

#[test]
fn validate_search_intent_rejects_missing_query_embedding_for_vector() {
    let intent = SearchIntentInput::new(
        SearchQueryInput::vector(Some("hello".to_string()), None).expect("query"),
        None,
        5,
        0.5,
        Vec::new(),
        None,
    )
    .expect("intent");
    assert!(build_search_intent(intent).is_err());
}

#[test]
fn validate_search_intent_rejects_empty_query_embedding() {
    assert!(SearchQueryInput::vector(None, Some(Vec::new())).is_err());
}

#[test]
fn validate_search_intent_rejects_missing_query_text_for_fts() {
    let json = serde_json::json!({
        "retrieval_mode": "fts",
        "top_k": 3,
        "hybrid_alpha": 0.5,
        "filters": [],
        "reranker": null
    });
    let decoded = serde_json::from_value::<SearchIntentInput>(json);
    assert!(decoded.is_err());
}

#[test]
fn validate_search_intent_rejects_missing_query_text_for_hybrid() {
    let json = serde_json::json!({
        "retrieval_mode": "hybrid",
        "query_embedding": [0.1, 0.2],
        "top_k": 3,
        "hybrid_alpha": 0.5,
        "filters": [],
        "reranker": null
    });
    let decoded = serde_json::from_value::<SearchIntentInput>(json);
    assert!(decoded.is_err());
}

#[test]
fn validate_search_intent_rejects_missing_query_embedding_for_hybrid() {
    let intent = SearchIntentInput::new(
        SearchQueryInput::hybrid("hello", None).expect("query"),
        None,
        5,
        0.5,
        Vec::new(),
        None,
    )
    .expect("intent");
    assert!(build_search_intent(intent).is_err());
}

#[test]
fn validate_search_intent_rejects_zero_top_k() {
    let query = SearchQueryInput::fts("hello").expect("query");
    assert!(SearchIntentInput::new(query, None, 0, 0.5, Vec::new(), None).is_err());
}

#[test]
fn validate_search_intent_rejects_out_of_range_alpha() {
    let query = SearchQueryInput::fts("hello").expect("query");
    assert!(SearchIntentInput::new(query, None, 5, 1.5, Vec::new(), None).is_err());
}

#[test]
fn validate_search_intent_rejects_blank_query_text() {
    assert!(SearchQueryInput::fts("   ").is_err());
}

#[test]
fn validate_search_intent_rejects_reranker_zero_or_oversized() {
    let mut intent = sample_intent(RetrievalMode::Fts);
    intent.reranker = Some(coco_protocol::RerankerConfig {
        model_name: "rerank".to_string(),
        rerank_top_n: 0,
    });
    assert!(build_search_intent(intent).is_err());

    let mut intent = sample_intent(RetrievalMode::Fts);
    intent.reranker = Some(coco_protocol::RerankerConfig {
        model_name: "rerank".to_string(),
        rerank_top_n: 999,
    });
    assert!(build_search_intent(intent).is_err());
}

#[test]
fn build_search_intent_rejects_invalid_indexing_config_id() {
    let mut intent = sample_intent(RetrievalMode::Fts);
    intent.indexing_config_id = Some(" Bad".to_string());
    assert!(build_search_intent(intent).is_err());
}

#[test]
fn validate_search_intent_rejects_invalid_indexing_config_id() {
    let intent = SearchIntent::new_unchecked(
        SearchQuery::Fts {
            text: "hello".to_string(),
        },
        Some(" Bad".to_string()),
        NonZeroU32::new(5).expect("top_k"),
        0.5,
        Vec::new(),
        None,
    );
    assert!(validate_search_intent(&intent, &ValidationContext::default()).is_err());
}

#[test]
fn validate_search_intent_rejects_invalid_hybrid_alpha() {
    let intent = SearchIntent::new_unchecked(
        SearchQuery::Fts {
            text: "hello".to_string(),
        },
        None,
        NonZeroU32::new(5).expect("top_k"),
        1.5,
        Vec::new(),
        None,
    );
    assert!(validate_search_intent(&intent, &ValidationContext::default()).is_err());
}

#[test]
fn validate_search_intent_rejects_invalid_reranker() {
    let intent = SearchIntent::new_unchecked(
        SearchQuery::Fts {
            text: "hello".to_string(),
        },
        None,
        NonZeroU32::new(5).expect("top_k"),
        0.5,
        Vec::new(),
        Some(RerankerConfig {
            model_name: "rerank".to_string(),
            rerank_top_n: 0,
        }),
    );
    assert!(validate_search_intent(&intent, &ValidationContext::default()).is_err());

    let intent = SearchIntent::new_unchecked(
        SearchQuery::Fts {
            text: "hello".to_string(),
        },
        None,
        NonZeroU32::new(3).expect("top_k"),
        0.5,
        Vec::new(),
        Some(RerankerConfig {
            model_name: "rerank".to_string(),
            rerank_top_n: 5,
        }),
    );
    assert!(validate_search_intent(&intent, &ValidationContext::default()).is_err());
}

#[test]
fn validate_search_intent_enforces_filter_allowlist() {
    let intent = SearchIntentInput {
        filters: vec![Filter {
            field: FilterField::new("path").expect("filter field"),
            op: FilterOp::Contains,
            value: FilterValue::String("src".to_string()),
        }],
        ..sample_intent(RetrievalMode::Fts)
    };
    let intent = build_search_intent(intent).expect("validated intent");
    let context = ValidationContext {
        allowed_filter_fields: Some(vec![FilterField::new("doc_id").expect("filter field")]),
        ..ValidationContext::default()
    };
    assert!(validate_search_intent(&intent, &context).is_err());
}

#[test]
fn build_search_intent_rejects_filter_value_mismatch() {
    let intent = SearchIntentInput {
        filters: vec![Filter {
            field: FilterField::new("path").expect("filter field"),
            op: FilterOp::Contains,
            value: FilterValue::Int(42),
        }],
        ..sample_intent(RetrievalMode::Fts)
    };
    assert!(build_search_intent(intent).is_err());

    let intent = SearchIntentInput {
        filters: vec![Filter {
            field: FilterField::new("path").expect("filter field"),
            op: FilterOp::In,
            value: FilterValue::Bool(true),
        }],
        ..sample_intent(RetrievalMode::Fts)
    };
    assert!(build_search_intent(intent).is_err());

    let intent = SearchIntentInput {
        filters: vec![Filter {
            field: FilterField::new("path").expect("filter field"),
            op: FilterOp::In,
            value: FilterValue::String("docs".to_string()),
        }],
        ..sample_intent(RetrievalMode::Fts)
    };
    assert!(build_search_intent(intent).is_err());
}

#[test]
fn validate_search_intent_rejects_filter_value_mismatch() {
    let intent = SearchIntent::new_unchecked(
        SearchQuery::Fts {
            text: "hello".to_string(),
        },
        None,
        NonZeroU32::new(5).expect("top_k"),
        0.5,
        vec![Filter {
            field: FilterField::new("path").expect("filter field"),
            op: FilterOp::Contains,
            value: FilterValue::Int(42),
        }],
        None,
    );
    assert!(validate_search_intent(&intent, &ValidationContext::default()).is_err());
}

#[test]
fn vector_index_params_serialize_and_validate() {
    let params = VectorIndexParams {
        hnsw: Some(HnswParams {
            m: Some(16),
            ef_construction: Some(128),
        }),
        ivf_pq: None,
    };
    let config = IndexingConfig {
        index_params: Some(params.clone()),
        ..sample_indexing_config("default")
    };
    let value = serde_json::to_value(&params).expect("serialize params");
    assert!(value.get("hnsw").is_some());
    assert!(validate_indexing_config(&config, &ValidationContext::default()).is_ok());
}

#[test]
fn vector_index_params_rejects_multiple_kinds() {
    let params = VectorIndexParams {
        hnsw: Some(HnswParams {
            m: Some(16),
            ef_construction: Some(128),
        }),
        ivf_pq: Some(Default::default()),
    };
    let config = IndexingConfig {
        index_params: Some(params),
        ..sample_indexing_config("default")
    };
    assert!(validate_indexing_config(&config, &ValidationContext::default()).is_err());
}

#[test]
fn vector_index_params_rejects_empty_params() {
    let params = VectorIndexParams {
        hnsw: None,
        ivf_pq: None,
    };
    let config = IndexingConfig {
        index_params: Some(params),
        ..sample_indexing_config("default")
    };
    assert!(validate_indexing_config(&config, &ValidationContext::default()).is_err());
}

#[test]
fn indexing_config_requires_vector_metric() {
    let json = serde_json::json!({
        "config_id": "default",
        "chunking": {
            "strategy_name": "fixed_token",
            "chunk_size": 256,
            "chunk_overlap": 32
        },
        "embedding": {
            "model_name": "all-MiniLM-L6-v2",
            "dimensions": 384
        }
    });
    let decoded = serde_json::from_value::<IndexingConfig>(json);
    assert!(decoded.is_err());
}

#[test]
fn config_id_validation_rejects_leading_separator() {
    assert!(normalize_config_id("-bad").is_err());
    assert!(normalize_config_id("_bad").is_err());
}

#[test]
fn config_id_validation_rejects_trim_changes() {
    let config = sample_indexing_config(" default");
    assert!(validate_indexing_config(&config, &ValidationContext::default()).is_err());
}

#[test]
fn config_id_length_enforced() {
    let ok = "a".repeat(63);
    let too_long = "a".repeat(64);
    assert!(normalize_config_id(&ok).is_ok());
    assert!(normalize_config_id(&too_long).is_err());
}

#[test]
fn text_span_rejects_inverted_range() {
    assert!(TextSpan::new(5, 3).is_err());
    let json = serde_json::json!({ "start": 5, "end": 3 });
    let decoded = serde_json::from_value::<TextSpan>(json);
    assert!(decoded.is_err());
}

#[test]
fn filter_field_rejects_blank() {
    assert!(FilterField::new(" ").is_err());
}

#[test]
fn error_response_serialization_is_stable() {
    let response = ErrorResponse::from(CocoError::user("bad input"));
    let value = serde_json::to_value(&response).expect("serialize error response");
    let object = value.as_object().expect("error response object");
    assert_eq!(object.len(), 2);
    assert_eq!(
        value["kind"],
        serde_json::to_value(CocoErrorKind::User).unwrap()
    );
    assert_eq!(value["message"], "bad input");
}
