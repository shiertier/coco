use std::sync::Arc;

use coco_protocol::CocoResult;

use crate::embedder::HttpEmbedder;

pub(crate) fn load_embedder() -> CocoResult<Option<Arc<HttpEmbedder>>> {
    let provider = std::env::var("COCO_EMBEDDING_PROVIDER")
        .ok()
        .filter(|value| !value.is_empty());
    let api_key_present = std::env::var("COCO_OPENAI_API_KEY")
        .ok()
        .filter(|value| !value.is_empty())
        .is_some();
    if provider.is_none() && !api_key_present {
        return Ok(None);
    }
    let embedder = HttpEmbedder::from_env()?;
    Ok(Some(Arc::new(embedder)))
}
