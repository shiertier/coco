use std::collections::HashSet;
use std::num::NonZeroU32;

use coco_protocol::{
    CocoError, CocoResult, Filter, FilterOp, FilterValue, HnswParams, INDEXING_PLAN_DEFAULT_STEPS,
    INDEXING_PLAN_VERSION, IndexingConfig, IndexingPlan, IvfPqParams, MAX_CONFIG_ID_LEN,
    QUERY_PLAN_DEFAULT_STEPS, QUERY_PLAN_VERSION, QueryPlan, RerankerConfig, RetrievalConfig,
    SearchIntent, SearchIntentInput, SearchQuery, SearchQueryInput, ValidationContext,
    VectorBackendKind, VectorIndexParams, VectorMetric,
};

/// Builds a validated search intent from wire input.
pub fn build_search_intent(input: SearchIntentInput) -> CocoResult<SearchIntent> {
    let SearchIntentInput {
        query,
        indexing_config_id,
        top_k,
        hybrid_alpha,
        filters,
        reranker,
    } = input;
    let query = match query {
        SearchQueryInput::Vector(vector) => {
            let (_, embedding) = vector.into_parts();
            let embedding = embedding
                .ok_or_else(|| validation_error("query_embedding required for vector search"))?;
            SearchQuery::Vector { embedding }
        }
        SearchQueryInput::Fts(fts) => {
            let text = fts
                .into_text()
                .ok_or_else(|| validation_error("query_text required for fts search"))?;
            SearchQuery::Fts { text }
        }
        SearchQueryInput::Hybrid(hybrid) => {
            let (text, embedding) = hybrid.into_parts();
            let text =
                text.ok_or_else(|| validation_error("query_text required for hybrid search"))?;
            let embedding = embedding
                .ok_or_else(|| validation_error("query_embedding required for hybrid search"))?;
            SearchQuery::Hybrid { text, embedding }
        }
    };
    build_search_intent_from_parts(
        query,
        indexing_config_id,
        top_k,
        hybrid_alpha.into(),
        filters,
        reranker,
    )
}

/// Builds a validated search intent from already-parsed parts.
pub fn build_search_intent_from_parts(
    query: SearchQuery,
    indexing_config_id: Option<String>,
    top_k: NonZeroU32,
    hybrid_alpha: f32,
    filters: Vec<Filter>,
    reranker: Option<RerankerConfig>,
) -> CocoResult<SearchIntent> {
    validate_search_query(&query)?;
    validate_hybrid_alpha(hybrid_alpha)?;
    validate_reranker(reranker.as_ref(), top_k)?;
    if let Some(config_id) = indexing_config_id.as_deref() {
        ensure_normalized_config_id(config_id)?;
    }
    for filter in &filters {
        validate_filter_value(filter)?;
    }
    Ok(SearchIntent::new_unchecked(
        query,
        indexing_config_id,
        top_k,
        hybrid_alpha,
        filters,
        reranker,
    ))
}

/// Normalizes and validates config identifiers.
pub fn normalize_config_id(config_id: &str) -> CocoResult<String> {
    let trimmed = config_id.trim();
    if trimmed.is_empty() {
        return Err(validation_error("config_id must not be empty"));
    }
    if trimmed.len() > MAX_CONFIG_ID_LEN {
        return Err(validation_error("config_id exceeds max length"));
    }
    for (index, ch) in trimmed.chars().enumerate() {
        if index == 0 {
            if !(ch.is_ascii_lowercase() || ch.is_ascii_digit()) {
                return Err(validation_error(
                    "config_id must start with a lowercase letter or digit",
                ));
            }
            continue;
        }
        if !(ch.is_ascii_lowercase() || ch.is_ascii_digit() || ch == '-' || ch == '_') {
            return Err(validation_error(
                "config_id must use lowercase letters, digits, '-' or '_'",
            ));
        }
    }
    Ok(trimmed.to_string())
}

/// Validates a search intent against protocol invariants.
pub fn validate_search_intent(
    intent: &SearchIntent,
    context: &ValidationContext,
) -> CocoResult<()> {
    validate_search_query(intent.query())?;
    validate_hybrid_alpha(intent.hybrid_alpha())?;
    validate_reranker(intent.reranker(), intent.top_k())?;
    if let (Some(expected), Some(embedding)) =
        (context.embedding_dimensions, intent.query_embedding())
    {
        if embedding.len() != expected {
            return Err(validation_error("query_embedding has wrong dimension"));
        }
    }
    if let Some(config_id) = intent.indexing_config_id() {
        ensure_normalized_config_id(config_id)?;
    } else if let Some(active_config_id) = context.active_config_id.as_deref() {
        ensure_normalized_config_id(active_config_id)?;
    }
    for filter in intent.filters() {
        validate_filter_value(filter)?;
    }
    if let Some(allowed) = context.allowed_filter_fields.as_ref() {
        for filter in intent.filters() {
            if !allowed.iter().any(|field| field == &filter.field) {
                return Err(validation_error("filter field not allowed"));
            }
        }
    }
    if let Some(allowed) = context.allowed_filter_ops.as_ref() {
        for filter in intent.filters() {
            if !allowed.contains(&filter.op) {
                return Err(validation_error("filter operator not allowed"));
            }
        }
    }
    Ok(())
}

/// Validates indexing configuration inputs.
pub fn validate_indexing_config(
    config: &IndexingConfig,
    context: &ValidationContext,
) -> CocoResult<()> {
    ensure_normalized_config_id(&config.config_id)?;
    if let Some(params) = config.index_params.as_ref() {
        validate_index_params(params, context.expected_vector_backend)?;
    }
    if let Some(expected) = context.expected_vector_backend {
        if let Some(selected) = config.vector_backend.as_ref() {
            if selected.kind != expected {
                return Err(validation_error(
                    "vector_backend does not match expected backend",
                ));
            }
        }
    }
    match config.vector_metric {
        VectorMetric::Cosine | VectorMetric::Dot | VectorMetric::L2 => {}
    }
    Ok(())
}

/// Validates retrieval configuration inputs.
pub fn validate_retrieval_config(
    config: &RetrievalConfig,
    context: &ValidationContext,
) -> CocoResult<()> {
    if config.top_k == 0 {
        return Err(validation_error("top_k must be greater than zero"));
    }
    if !(0.0..=1.0).contains(&config.hybrid_alpha) {
        return Err(validation_error("hybrid_alpha must be between 0 and 1"));
    }
    if let Some(reranker) = config.reranker.as_ref() {
        if reranker.rerank_top_n == 0 {
            return Err(validation_error("rerank_top_n must be greater than zero"));
        }
        if reranker.rerank_top_n > config.top_k {
            return Err(validation_error("rerank_top_n must be <= top_k"));
        }
    }
    if let Some(expected) = context.expected_vector_backend {
        if let Some(selected) = config.vector_backend.as_ref() {
            if selected.kind != expected {
                return Err(validation_error(
                    "vector_backend does not match expected backend",
                ));
            }
        }
    }
    Ok(())
}

/// Validates the indexing plan schema and required steps.
pub fn validate_indexing_plan(plan: &IndexingPlan) -> CocoResult<()> {
    if plan.version != INDEXING_PLAN_VERSION {
        return Err(validation_error("unsupported indexing plan version"));
    }
    validate_plan_steps("indexing plan", &plan.steps, &INDEXING_PLAN_DEFAULT_STEPS)
}

/// Validates the query plan schema and required steps.
pub fn validate_query_plan(plan: &QueryPlan) -> CocoResult<()> {
    if plan.version != QUERY_PLAN_VERSION {
        return Err(validation_error("unsupported query plan version"));
    }
    validate_plan_steps("query plan", &plan.steps, &QUERY_PLAN_DEFAULT_STEPS)
}

fn validate_plan_steps(label: &str, steps: &[String], required: &[&str]) -> CocoResult<()> {
    if steps.is_empty() {
        return Err(validation_error(&format!(
            "{label} steps must not be empty"
        )));
    }
    let mut seen = HashSet::with_capacity(steps.len());
    for step in steps {
        if step.trim().is_empty() {
            return Err(validation_error(&format!("{label} step must not be empty")));
        }
        if step.trim() != step {
            return Err(validation_error(&format!(
                "{label} step must be normalized"
            )));
        }
        if !seen.insert(step.as_str()) {
            return Err(validation_error(&format!("{label} steps must be unique")));
        }
    }
    for required_step in required {
        if !seen.contains(required_step) {
            return Err(validation_error(&format!(
                "{label} missing required step {required_step}"
            )));
        }
    }
    Ok(())
}

fn ensure_normalized_config_id(config_id: &str) -> CocoResult<()> {
    let normalized = normalize_config_id(config_id)?;
    if normalized != config_id {
        return Err(validation_error("config_id must be normalized"));
    }
    Ok(())
}

fn validate_index_params(
    params: &VectorIndexParams,
    backend: Option<VectorBackendKind>,
) -> CocoResult<()> {
    let mut kinds = 0;
    if let Some(hnsw) = params.hnsw.as_ref() {
        kinds += 1;
        validate_hnsw(hnsw)?;
    }
    if let Some(ivf_pq) = params.ivf_pq.as_ref() {
        kinds += 1;
        validate_ivf_pq(ivf_pq)?;
    }
    if kinds == 0 {
        return Err(validation_error("index_params must specify an index kind"));
    }
    if kinds > 1 {
        return Err(validation_error(
            "index_params must specify a single index kind",
        ));
    }
    if let Some(backend) = backend {
        match backend {
            VectorBackendKind::PgVector | VectorBackendKind::Qdrant => {
                if params.ivf_pq.is_some() {
                    return Err(validation_error(
                        "ivf_pq params not supported for this backend",
                    ));
                }
            }
        }
    }
    Ok(())
}

fn validate_hnsw(params: &HnswParams) -> CocoResult<()> {
    validate_positive_u32(params.m, "hnsw.m")?;
    validate_positive_u32(params.ef_construction, "hnsw.ef_construction")?;
    Ok(())
}

fn validate_ivf_pq(params: &IvfPqParams) -> CocoResult<()> {
    validate_positive_u32(params.num_partitions, "ivf_pq.num_partitions")?;
    validate_positive_u32(params.num_sub_vectors, "ivf_pq.num_sub_vectors")?;
    validate_positive_u32(params.sample_rate, "ivf_pq.sample_rate")?;
    validate_positive_u32(params.max_iterations, "ivf_pq.max_iterations")?;
    Ok(())
}

fn validate_positive_u32(value: Option<u32>, label: &str) -> CocoResult<()> {
    if let Some(0) = value {
        return Err(validation_error(&format!(
            "{label} must be greater than zero"
        )));
    }
    Ok(())
}

fn validate_filter_value(filter: &Filter) -> CocoResult<()> {
    match (&filter.op, &filter.value) {
        (FilterOp::Contains, FilterValue::String(_)) => Ok(()),
        (FilterOp::Contains, _) => {
            Err(validation_error("filter value must be string for contains"))
        }
        (FilterOp::In, FilterValue::List(values)) => {
            if values.is_empty() {
                Err(validation_error(
                    "filter value must be non-empty list for in",
                ))
            } else {
                Ok(())
            }
        }
        (FilterOp::In, _) => Err(validation_error("filter value must be list for in")),
        (_, FilterValue::List(_)) => Err(validation_error(
            "filter value must be scalar for this operator",
        )),
        _ => Ok(()),
    }
}

fn validate_search_query(query: &SearchQuery) -> CocoResult<()> {
    match query {
        SearchQuery::Vector { embedding } => {
            ensure_non_empty_embedding(embedding)?;
        }
        SearchQuery::Fts { text } => {
            ensure_non_empty_trimmed(text, "query_text")?;
        }
        SearchQuery::Hybrid { text, embedding } => {
            ensure_non_empty_trimmed(text, "query_text")?;
            ensure_non_empty_embedding(embedding)?;
        }
    }
    Ok(())
}

fn validate_hybrid_alpha(hybrid_alpha: f32) -> CocoResult<()> {
    if !(0.0..=1.0).contains(&hybrid_alpha) {
        return Err(validation_error("hybrid_alpha must be between 0 and 1"));
    }
    Ok(())
}

fn validate_reranker(reranker: Option<&RerankerConfig>, top_k: NonZeroU32) -> CocoResult<()> {
    if let Some(reranker) = reranker {
        if reranker.rerank_top_n == 0 {
            return Err(validation_error("rerank_top_n must be greater than zero"));
        }
        if reranker.rerank_top_n > top_k.get() {
            return Err(validation_error("rerank_top_n must be <= top_k"));
        }
    }
    Ok(())
}

fn ensure_non_empty_embedding(value: &[f32]) -> CocoResult<()> {
    if value.is_empty() {
        return Err(validation_error("query_embedding must not be empty"));
    }
    Ok(())
}

fn ensure_non_empty_trimmed(value: &str, label: &str) -> CocoResult<()> {
    if value.is_empty() {
        return Err(validation_error(&format!("{label} must not be empty")));
    }
    if value.trim() != value {
        return Err(validation_error(&format!("{label} must be normalized")));
    }
    Ok(())
}

fn validation_error(message: &str) -> CocoError {
    CocoError::user(format!("validation error: {message}"))
}
