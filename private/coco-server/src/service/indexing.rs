use coco_protocol::{
    ChunkingStrategy, CocoError, CocoResult, EmbeddingConfig, EmbeddingModel, IndexingConfig,
    VectorBackendConfig, VectorBackendKind, VectorMetric,
};

use super::constants::{DEFAULT_CHUNK_OVERLAP, DEFAULT_CHUNK_SIZE, DEFAULT_CHUNK_STRATEGY};
use super::state::AppState;
use crate::storage::meta::{DEFAULT_CONFIG_ID, IndexingConfigRecord, NewIndexingConfig};

pub(crate) fn default_indexing_config(state: &AppState, org_id: &str) -> NewIndexingConfig {
    let model_name = match state.embedder.as_ref() {
        Some(embedder) => embedder.model_name().to_string(),
        None => "unknown".to_string(),
    };
    let dimensions = Some(state.embedding_dimensions);
    NewIndexingConfig {
        org_id: org_id.to_string(),
        config_id: DEFAULT_CONFIG_ID.to_string(),
        chunking: ChunkingStrategy {
            strategy_name: DEFAULT_CHUNK_STRATEGY.to_string(),
            chunk_size: DEFAULT_CHUNK_SIZE,
            chunk_overlap: DEFAULT_CHUNK_OVERLAP,
        },
        embedding: EmbeddingConfig {
            model_name,
            dimensions,
        },
        vector_backend: Some(VectorBackendConfig {
            kind: state.vector_backend_kind,
            url: None,
            api_key: None,
            collection_prefix: None,
        }),
        vector_metric: VectorMetric::L2,
        index_params: None,
        created_at: chrono::Utc::now(),
    }
}

pub(crate) fn ensure_vector_backend(
    config: &mut IndexingConfig,
    expected: VectorBackendKind,
) -> CocoResult<()> {
    if let Some(backend) = config.vector_backend.as_ref() {
        if backend.kind != expected {
            return Err(CocoError::user(
                "vector_backend does not match server configuration",
            ));
        }
        if backend.url.is_some() || backend.api_key.is_some() || backend.collection_prefix.is_some()
        {
            return Err(CocoError::user(
                "vector_backend overrides are not supported",
            ));
        }
    }
    config.vector_backend = Some(VectorBackendConfig {
        kind: expected,
        url: None,
        api_key: None,
        collection_prefix: None,
    });
    Ok(())
}

pub(crate) fn validate_indexing_config_backend(
    config: &IndexingConfigRecord,
    expected: VectorBackendKind,
) -> CocoResult<()> {
    if let Some(backend) = config.vector_backend.as_ref() {
        if backend.kind != expected {
            return Err(CocoError::user(
                "vector_backend does not match server configuration",
            ));
        }
    }
    Ok(())
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
