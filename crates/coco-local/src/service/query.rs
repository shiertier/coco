use coco_core::{normalize_config_id, validate_retrieval_config, validate_search_intent};
use coco_protocol::{
    CocoError, CocoResult, EmbeddingModel, FilterField, FilterOp, IndexingConfig, RetrievalConfig,
    SearchIntent, SearchIntentInput, ValidationContext,
};

use crate::embedder::LocalEmbedder;
use crate::storage::meta::{IndexingConfigRecord, LocalMetaStore, DEFAULT_CONFIG_ID};

pub(crate) async fn resolve_import_config_id(
    meta: &LocalMetaStore,
    project_id: &str,
    requested: Option<&str>,
) -> CocoResult<String> {
    let project = meta
        .get_project(project_id)
        .await?
        .ok_or_else(|| CocoError::user("project not found"))?;
    let active_config_id = ensure_active_config_id(&project.active_config_id)?;
    let config_id = match requested {
        Some(value) => normalize_and_check(value, "indexing_config_id")?,
        None => active_config_id,
    };
    meta.ensure_indexing_config_exists(&config_id).await?;
    Ok(config_id)
}

pub(crate) async fn resolve_query_config_id(
    meta: &LocalMetaStore,
    project_id: Option<&str>,
    requested: Option<&str>,
) -> CocoResult<String> {
    let active_config_id = if let Some(project_id) = project_id {
        if project_id.trim().is_empty() {
            return Err(CocoError::user("project_id must not be empty"));
        }
        let project = meta
            .get_project(project_id)
            .await?
            .ok_or_else(|| CocoError::user("project not found"))?;
        ensure_active_config_id(&project.active_config_id)?
    } else {
        DEFAULT_CONFIG_ID.to_string()
    };

    let config_id = match requested {
        Some(value) => normalize_and_check(value, "indexing_config_id")?,
        None => active_config_id.clone(),
    };
    if config_id != active_config_id {
        return Err(CocoError::user(
            "indexing_config_id does not match active config",
        ));
    }
    meta.ensure_indexing_config_exists(&config_id).await?;
    Ok(config_id)
}

pub(crate) async fn load_indexing_config(
    meta: &LocalMetaStore,
    config_id: &str,
) -> CocoResult<IndexingConfigRecord> {
    meta.get_indexing_config(config_id)
        .await?
        .ok_or_else(|| CocoError::user("indexing config not found"))
}

pub(crate) fn apply_retrieval_config(
    intent: &mut SearchIntentInput,
    retrieval: RetrievalConfig,
) -> CocoResult<()> {
    if retrieval.vector_backend.is_some() {
        return Err(CocoError::user(
            "retrieval_config.vector_backend is not supported in local mode",
        ));
    }
    validate_retrieval_config(&retrieval, &ValidationContext::default())?;
    intent.set_retrieval_mode(retrieval.retrieval_mode)?;
    intent.set_top_k(retrieval.top_k)?;
    intent.set_hybrid_alpha(retrieval.hybrid_alpha)?;
    intent.reranker = retrieval.reranker;
    Ok(())
}

pub(crate) fn validate_local_search_intent(
    intent: &SearchIntent,
    config: &IndexingConfigRecord,
) -> CocoResult<()> {
    let allowed_fields = vec![
        FilterField::new("doc_id")?,
        FilterField::new("chunk_id")?,
        FilterField::new("content")?,
    ];
    let context = ValidationContext {
        embedding_dimensions: config.embedding.dimensions.map(|dim| dim as usize),
        expected_vector_backend: None,
        allowed_filter_fields: Some(allowed_fields),
        allowed_filter_ops: Some(vec![FilterOp::Eq, FilterOp::Contains]),
        active_config_id: Some(config.config_id.clone()),
    };
    validate_search_intent(intent, &context)
}

pub(crate) fn ensure_embedding_dimensions(
    config: &IndexingConfigRecord,
    embedder: Option<&LocalEmbedder>,
) -> CocoResult<()> {
    let Some(embedder) = embedder else {
        return Ok(());
    };
    let Some(expected) = config.embedding.dimensions else {
        return Ok(());
    };
    if embedder.dimensions() != expected as usize {
        return Err(CocoError::user(
            "embedding dimensions do not match indexing config",
        ));
    }
    Ok(())
}

pub(crate) fn ensure_active_config_id(active_config_id: &str) -> CocoResult<String> {
    if active_config_id.trim().is_empty() {
        return Err(CocoError::user("active_config_id is not set"));
    }
    Ok(active_config_id.to_string())
}

pub(crate) fn normalize_and_check(value: &str, field: &str) -> CocoResult<String> {
    let normalized = normalize_config_id(value)?;
    if normalized != value {
        return Err(CocoError::user(format!("{field} must be normalized")));
    }
    Ok(normalized)
}

pub(crate) fn indexing_config_from_record(record: IndexingConfigRecord) -> IndexingConfig {
    IndexingConfig {
        config_id: record.config_id,
        chunking: record.chunking,
        embedding: record.embedding,
        vector_backend: record.vector_backend,
        vector_metric: record.vector_metric,
        index_params: record.index_params,
    }
}

pub(crate) async fn fill_query_embedding(
    intent: &mut SearchIntentInput,
    embedder: Option<&LocalEmbedder>,
) -> CocoResult<()> {
    match intent.retrieval_mode() {
        coco_protocol::RetrievalMode::Vector | coco_protocol::RetrievalMode::Hybrid => {
            if intent.query_embedding().is_some() {
                return Ok(());
            }
            let query_text = intent
                .query_text()
                .ok_or_else(|| CocoError::user("query_text required to build embedding"))?;
            let embedder =
                embedder.ok_or_else(|| CocoError::user("embedding model not configured"))?;
            let query_text = query_text.to_string();
            let embedder = embedder.clone();
            let embeddings = tokio::task::spawn_blocking(move || {
                let refs = [query_text.as_str()];
                embedder.embed(&refs)
            })
            .await
            .map_err(|err| CocoError::system(format!("embedding task failed: {err}")))?;
            let embedding = embeddings?
                .into_iter()
                .next()
                .ok_or_else(|| CocoError::compute("empty embedding output"))?;
            intent.set_query_embedding(embedding)?;
        }
        coco_protocol::RetrievalMode::Fts => {
            if intent.query_text().is_none() {
                return Err(CocoError::user("query_text required for fts search"));
            }
        }
    }
    Ok(())
}
