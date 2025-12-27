use coco_protocol::{CocoError, CocoResult, RetrievalMode, SearchHit, SearchIntent};
use qdrant_client::qdrant::SearchPointsBuilder;

use super::store::QdrantStore;
use super::util::map_qdrant_err;

/// Qdrant query executor mapping SearchIntent to Qdrant search.
#[derive(Debug, Clone)]
pub struct QdrantExecutor {
    store: QdrantStore,
}

impl QdrantExecutor {
    /// Creates a new executor from an existing store.
    pub fn new(store: &QdrantStore) -> Self {
        Self {
            store: store.clone(),
        }
    }

    /// Executes the search intent using Qdrant.
    pub async fn search(&self, intent: SearchIntent) -> CocoResult<Vec<SearchHit>> {
        if matches!(intent.retrieval_mode(), RetrievalMode::Fts) {
            return Err(CocoError::user("fts search not supported for qdrant"));
        }
        let config_id = self.store.resolve_config_id(intent.indexing_config_id())?;
        let collection = self
            .store
            .collection_name(&config_id, self.store.version_id())?;
        let alias = self.store.collection_alias(&config_id)?;
        let use_alias = self.store.use_collection_alias();

        let exists = self
            .store
            .client()
            .collection_exists(&collection)
            .await
            .map_err(|err| map_qdrant_err("qdrant collection check failed", err))?;
        if !exists {
            return Ok(Vec::new());
        }
        if use_alias {
            self.store.ensure_alias(&collection, &alias).await?;
        }
        let embedding = intent
            .query_embedding()
            .ok_or_else(|| CocoError::user("query embedding required for qdrant search"))?
            .to_vec();
        let top_k = intent.top_k().get() as u64;
        let filter = self.store.build_filter(&config_id, intent.filters())?;
        let target_collection = if use_alias { alias } else { collection };
        let search = SearchPointsBuilder::new(target_collection, embedding, top_k)
            .with_payload(true)
            .filter(filter);
        let response = self
            .store
            .client()
            .search_points(search)
            .await
            .map_err(|err| map_qdrant_err("qdrant search failed", err))?;
        self.store.collect_search_hits(response.result, &config_id).await
    }
}
