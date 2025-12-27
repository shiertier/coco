use coco_protocol::{
    validate_retrieval_config, CocoError, CocoResult, EmbeddingModel, RetrievalConfig,
    SearchIntentInput, ValidationContext, VectorBackendKind,
};

use super::state::AppState;
use crate::embedder::HttpEmbedder;

pub(crate) async fn project_for_request(
    state: &AppState,
    org_id: &str,
    user_id: &str,
    project_id: &str,
) -> CocoResult<crate::storage::meta::ProjectRecord> {
    let Some(project) = state.meta.get_project(org_id, user_id, project_id).await? else {
        return Err(CocoError::user("project not found"));
    };
    Ok(project)
}

pub(crate) fn extract_version_id_filter(
    intent: &mut SearchIntentInput,
) -> CocoResult<Option<String>> {
    let mut retained = Vec::with_capacity(intent.filters.len());
    for filter in intent.filters.drain(..) {
        if filter.field.as_str() == "version_id" {
            return Err(CocoError::user(
                "version_id filter is not supported for server queries",
            ));
        }
        retained.push(filter);
    }
    intent.filters = retained;
    Ok(None)
}

pub(crate) fn extract_indexing_config_id(
    intent: &SearchIntentInput,
) -> CocoResult<Option<String>> {
    let Some(config_id) = intent.indexing_config_id.as_deref() else {
        return Ok(None);
    };
    let normalized = coco_protocol::normalize_config_id(config_id)?;
    if normalized != config_id {
        return Err(CocoError::user(
            "indexing_config_id must be normalized",
        ));
    }
    Ok(Some(config_id.to_string()))
}

pub(crate) async fn fill_query_embedding(
    intent: &mut SearchIntentInput,
    embedder: Option<&HttpEmbedder>,
) -> CocoResult<()> {
    match intent.retrieval_mode {
        coco_protocol::RetrievalMode::Vector | coco_protocol::RetrievalMode::Hybrid => {
            if intent.query_embedding.is_some() {
                return Ok(());
            }
            let query_text = intent
                .query_text
                .as_deref()
                .ok_or_else(|| CocoError::user("query_text required to build embedding"))?;
            let embedder =
                embedder.ok_or_else(|| CocoError::user("embedding model not configured"))?;
            let query_owned = query_text.to_string();
            let embedder = embedder.clone();
            let embedding = tokio::task::spawn_blocking(move || {
                embedder
                    .embed(&[query_owned.as_str()])?
                    .into_iter()
                    .next()
                    .ok_or_else(|| CocoError::compute("empty embedding output"))
            })
            .await
            .map_err(|err| CocoError::system(format!("embedding task failed: {err}")))??;
            intent.query_embedding = Some(embedding);
        }
        coco_protocol::RetrievalMode::Fts => {
            if intent.query_text.is_none() {
                return Err(CocoError::user("query_text required for fts search"));
            }
        }
    }
    Ok(())
}

pub(crate) fn apply_retrieval_config(
    intent: &mut SearchIntentInput,
    retrieval: RetrievalConfig,
    expected_backend: VectorBackendKind,
) -> CocoResult<()> {
    let context = ValidationContext {
        expected_vector_backend: Some(expected_backend),
        ..Default::default()
    };
    validate_retrieval_config(&retrieval, &context)?;
    intent.retrieval_mode = retrieval.retrieval_mode;
    intent.top_k = retrieval.top_k;
    intent.hybrid_alpha = retrieval.hybrid_alpha;
    intent.reranker = retrieval.reranker;
    Ok(())
}
