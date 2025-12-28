use std::sync::Arc;

use coco_protocol::{CocoError, CocoErrorKind, CocoResult};

use super::config::LocalServiceConfig;
use crate::embedder::{LocalEmbedder, ModelPool, OrtEmbedder};

pub(crate) async fn prepare_embedder(
    config: &LocalServiceConfig,
) -> CocoResult<Option<Arc<LocalEmbedder>>> {
    if embedder_mode_is_stub() {
        let stub = LocalEmbedder::stub(config.model_name.clone(), config.model_dimensions)?;
        return Ok(Some(Arc::new(stub)));
    }
    if let Some(path) = config.model_path.clone() {
        let embedder =
            OrtEmbedder::from_file(path, config.model_name.clone(), config.model_dimensions)?;
        return Ok(Some(Arc::new(LocalEmbedder::Ort(Arc::new(embedder)))));
    }

    let spec = config.model_spec();
    let model_url_override = config.model_url.clone();
    let embedder = tokio::task::spawn_blocking(move || {
        let pool = ModelPool::global()?;
        pool.get_or_load(&spec, model_url_override.as_deref())
    })
    .await
    .map_err(|err| CocoError::system(format!("model preload task failed: {err}")))?
    .map_err(model_guidance)?;

    Ok(Some(Arc::new(LocalEmbedder::Ort(embedder))))
}

fn model_guidance(err: CocoError) -> CocoError {
    match err.kind() {
        CocoErrorKind::Network | CocoErrorKind::User => CocoError::user(format!(
            "model preparation failed: {err}. Set COCO_MODEL_PATH to a local ONNX file or COCO_MODEL_URL to a mirror"
        )),
        _ => CocoError::system(format!("model preparation failed: {err}")),
    }
}

fn embedder_mode_is_stub() -> bool {
    match std::env::var("COCO_EMBEDDER_MODE") {
        Ok(value) => matches!(value.to_ascii_lowercase().as_str(), "stub" | "mock"),
        Err(_) => false,
    }
}
