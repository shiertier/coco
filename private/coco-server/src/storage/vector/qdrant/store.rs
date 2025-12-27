use std::collections::{HashMap, HashSet};

use coco_protocol::{
    normalize_config_id, CocoError, CocoResult, Filter, FilterOp, SearchHit, SearchHitMeta,
    VectorBackendConfig, VectorIndexParams, VectorMetric, VectorRecord,
};
use qdrant_client::qdrant::{
    Condition, CreateAliasBuilder, CreateCollectionBuilder, Distance, Filter as QdrantFilter,
    HnswConfigDiff, Value,
};
use qdrant_client::Qdrant;
use sha2::{Digest, Sha256};
use tracing::warn;

use super::util::{
    join_name, map_qdrant_err, payload_string, safe_collection_name, PAYLOAD_ALIAS_ACTIVE,
    PAYLOAD_CHUNK_ID, PAYLOAD_CONFIG_ID, PAYLOAD_DOC_ID, PAYLOAD_ORG_ID, PAYLOAD_PROJECT_ID,
    PAYLOAD_USER_ID, PAYLOAD_VERSION_ID,
};
use crate::storage::pg::PgBackend;
use crate::storage::vector::rrf::normalize_scores;

/// Qdrant vector backend for server mode.
#[derive(Clone)]
pub struct QdrantStore {
    client: Qdrant,
    config: VectorBackendConfig,
    org_id: String,
    user_id: String,
    project_id: String,
    version_id: String,
    config_id: String,
    vector_metric: VectorMetric,
    index_params: Option<VectorIndexParams>,
    use_collection_alias: bool,
    pg: PgBackend,
}

impl std::fmt::Debug for QdrantStore {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("QdrantStore")
            .field("endpoint", &self.config.url)
            .field("collection_prefix", &self.config.collection_prefix)
            .field("org_id", &self.org_id)
            .field("user_id", &self.user_id)
            .field("project_id", &self.project_id)
            .field("version_id", &self.version_id)
            .field("config_id", &self.config_id)
            .field("vector_metric", &self.vector_metric)
            .field("use_collection_alias", &self.use_collection_alias)
            .finish()
    }
}

impl QdrantStore {
    /// Connects to Qdrant and prepares the backend context.
    pub async fn connect(
        config: VectorBackendConfig,
        tenant: crate::storage::pg::TenantContext,
        vector_metric: VectorMetric,
        index_params: Option<VectorIndexParams>,
        use_collection_alias: bool,
        pg: PgBackend,
    ) -> CocoResult<Self> {
        let url = config
            .url
            .as_deref()
            .ok_or_else(|| CocoError::user("qdrant url is required"))?;
        let mut builder = Qdrant::from_url(url);
        if let Some(api_key) = config.api_key.as_ref() {
            builder = builder.api_key(api_key.clone());
        }
        let client =
            builder.build().map_err(|err| CocoError::storage(format!("qdrant error: {err}")))?;
        let version_id = tenant
            .version_id
            .ok_or_else(|| CocoError::user("version_id must be set for qdrant backend"))?;
        let config_id = tenant
            .config_id
            .ok_or_else(|| CocoError::user("config_id must be set for qdrant backend"))?;
        let normalized = normalize_config_id(&config_id)?;
        if normalized != config_id {
            return Err(CocoError::user("config_id must be normalized"));
        }
        Ok(Self {
            client,
            config,
            org_id: tenant.org_id,
            user_id: tenant.user_id,
            project_id: tenant.project_id,
            version_id,
            config_id,
            vector_metric,
            index_params,
            use_collection_alias,
            pg,
        })
    }

    /// Returns the stored configuration.
    pub fn config(&self) -> &VectorBackendConfig {
        &self.config
    }

    pub(super) fn client(&self) -> &Qdrant {
        &self.client
    }

    pub(super) fn version_id(&self) -> &str {
        &self.version_id
    }

    pub(super) fn config_id(&self) -> &str {
        &self.config_id
    }

    pub(super) fn org_id(&self) -> &str {
        &self.org_id
    }

    pub(super) fn user_id(&self) -> &str {
        &self.user_id
    }

    pub(super) fn project_id(&self) -> &str {
        &self.project_id
    }

    pub(super) fn use_collection_alias(&self) -> bool {
        self.use_collection_alias
    }

    pub(super) fn pg(&self) -> &PgBackend {
        &self.pg
    }

    pub(super) fn resolve_config_id(&self, requested: Option<&str>) -> CocoResult<String> {
        match requested {
            Some(config_id) => {
                let normalized = normalize_config_id(config_id)?;
                if normalized != config_id {
                    return Err(CocoError::user("config_id must be normalized"));
                }
                if config_id != self.config_id {
                    return Err(CocoError::user(
                        "indexing_config_id does not match backend config",
                    ));
                }
                Ok(config_id.to_string())
            }
            None => Ok(self.config_id.clone()),
        }
    }

    fn collection_prefix(&self) -> CocoResult<&str> {
        self.config
            .collection_prefix
            .as_deref()
            .ok_or_else(|| CocoError::user("qdrant collection prefix is required"))
    }

    pub(super) fn collection_name(&self, config_id: &str, version_id: &str) -> CocoResult<String> {
        let prefix = self.collection_prefix()?;
        let base = join_name(
            prefix,
            &[
                &self.org_id,
                &self.user_id,
                &self.project_id,
                version_id,
                config_id,
            ],
        );
        Ok(safe_collection_name(base))
    }

    pub(super) fn collection_alias(&self, config_id: &str) -> CocoResult<String> {
        let prefix = self.collection_prefix()?;
        let base = join_name(
            prefix,
            &[
                &self.org_id,
                &self.user_id,
                &self.project_id,
                config_id,
                PAYLOAD_ALIAS_ACTIVE,
            ],
        );
        Ok(safe_collection_name(base))
    }

    pub(super) fn point_id_for(&self, config_id: &str, chunk_id: &str) -> u64 {
        let mut hasher = Sha256::new();
        hasher.update(self.org_id.as_bytes());
        hasher.update(b"|");
        hasher.update(self.user_id.as_bytes());
        hasher.update(b"|");
        hasher.update(self.project_id.as_bytes());
        hasher.update(b"|");
        hasher.update(self.version_id.as_bytes());
        hasher.update(b"|");
        hasher.update(config_id.as_bytes());
        hasher.update(b"|");
        hasher.update(chunk_id.as_bytes());
        let digest = hasher.finalize();
        let mut bytes = [0u8; 8];
        bytes.copy_from_slice(&digest[..8]);
        u64::from_be_bytes(bytes)
    }

    pub(super) fn payload_for(&self, config_id: &str, record: &VectorRecord) -> HashMap<String, Value> {
        let mut payload = HashMap::with_capacity(7);
        payload.insert(PAYLOAD_ORG_ID.to_string(), self.org_id.clone().into());
        payload.insert(PAYLOAD_USER_ID.to_string(), self.user_id.clone().into());
        payload.insert(
            PAYLOAD_PROJECT_ID.to_string(),
            self.project_id.clone().into(),
        );
        payload.insert(
            PAYLOAD_VERSION_ID.to_string(),
            self.version_id.clone().into(),
        );
        payload.insert(PAYLOAD_CONFIG_ID.to_string(), config_id.to_string().into());
        payload.insert(
            PAYLOAD_DOC_ID.to_string(),
            record.metadata.doc_id.to_string().into(),
        );
        payload.insert(
            PAYLOAD_CHUNK_ID.to_string(),
            record.chunk_id.to_string().into(),
        );
        payload
    }

    fn distance_for_metric(&self) -> Distance {
        match self.vector_metric {
            VectorMetric::Cosine => Distance::Cosine,
            VectorMetric::Dot => Distance::Dot,
            VectorMetric::L2 => Distance::Euclid,
        }
    }

    fn score_from_qdrant(&self, score: f32) -> f32 {
        match self.vector_metric {
            VectorMetric::L2 => 1.0 / (1.0 + score.max(0.0)),
            VectorMetric::Cosine | VectorMetric::Dot => score,
        }
    }

    fn hnsw_config(&self) -> Option<HnswConfigDiff> {
        let params = self.index_params.as_ref()?.hnsw.as_ref()?;
        Some(HnswConfigDiff {
            m: params.m.map(u64::from),
            ef_construct: params.ef_construction.map(u64::from),
            full_scan_threshold: None,
            max_indexing_threads: None,
            on_disk: None,
            payload_m: None,
            inline_storage: None,
        })
    }

    pub(super) async fn ensure_collection(&self, collection: &str, vector_len: usize) -> CocoResult<()> {
        if vector_len == 0 {
            return Err(CocoError::user(
                "embedding dimensions must be greater than zero",
            ));
        }
        let exists = self
            .client
            .collection_exists(collection)
            .await
            .map_err(|err| map_qdrant_err("qdrant collection check failed", err))?;
        if exists {
            self.validate_collection(collection, vector_len).await?;
            return Ok(());
        }
        let mut builder = CreateCollectionBuilder::new(collection)
            .vectors_config(qdrant_client::qdrant::VectorParamsBuilder::new(
                vector_len as u64,
                self.distance_for_metric(),
            ));
        if let Some(hnsw) = self.hnsw_config() {
            builder = builder.hnsw_config(hnsw);
        }
        self.client
            .create_collection(builder)
            .await
            .map_err(|err| map_qdrant_err("qdrant create collection failed", err))?;
        Ok(())
    }

    pub(super) async fn ensure_alias(&self, collection: &str, alias: &str) -> CocoResult<()> {
        if !self.use_collection_alias {
            return Ok(());
        }
        if let Err(err) = self
            .client
            .create_alias(CreateAliasBuilder::new(collection, alias))
            .await
        {
            warn!("qdrant create alias failed: {err}");
            if let Err(delete_err) = self.client.delete_alias(alias).await {
                warn!("qdrant delete alias failed: {delete_err}");
            }
            self.client
                .create_alias(CreateAliasBuilder::new(collection, alias))
                .await
                .map_err(|err| map_qdrant_err("qdrant create alias failed", err))?;
        }
        Ok(())
    }

    async fn validate_collection(&self, collection: &str, vector_len: usize) -> CocoResult<()> {
        let info = self
            .client
            .collection_info(collection)
            .await
            .map_err(|err| map_qdrant_err("qdrant collection info failed", err))?;
        let Some(info) = info.result else {
            return Err(CocoError::storage("qdrant collection info missing"));
        };
        let Some(config) = info.config else {
            return Err(CocoError::storage("qdrant collection config missing"));
        };
        let Some(params) = config.params else {
            return Err(CocoError::storage("qdrant collection params missing"));
        };
        let Some(vectors_config) = params.vectors_config else {
            return Err(CocoError::storage("qdrant vectors config missing"));
        };
        let params = match vectors_config.config {
            Some(qdrant_client::qdrant::vectors_config::Config::Params(params)) => params,
            Some(qdrant_client::qdrant::vectors_config::Config::ParamsMap(_)) => {
                return Err(CocoError::storage("qdrant named vectors are not supported"));
            }
            None => return Err(CocoError::storage("qdrant vectors config missing")),
        };
        if params.size != vector_len as u64 {
            return Err(CocoError::user(
                "qdrant collection vector dimensions mismatch",
            ));
        }
        let expected = self.distance_for_metric() as i32;
        if params.distance != expected {
            return Err(CocoError::user(
                "qdrant collection vector metric mismatch",
            ));
        }
        Ok(())
    }

    pub(super) fn build_filter(
        &self,
        config_id: &str,
        filters: &[Filter],
    ) -> CocoResult<QdrantFilter> {
        let mut conditions = Vec::with_capacity(filters.len() + 5);
        conditions.push(Condition::matches(PAYLOAD_ORG_ID, self.org_id.clone()));
        conditions.push(Condition::matches(PAYLOAD_USER_ID, self.user_id.clone()));
        conditions.push(Condition::matches(
            PAYLOAD_PROJECT_ID,
            self.project_id.clone(),
        ));
        conditions.push(Condition::matches(
            PAYLOAD_VERSION_ID,
            self.version_id.clone(),
        ));
        conditions.push(Condition::matches(PAYLOAD_CONFIG_ID, config_id.to_string()));
        for filter in filters {
            conditions.push(self.filter_condition(filter)?);
        }
        Ok(QdrantFilter::must(conditions))
    }

    fn filter_condition(&self, filter: &Filter) -> CocoResult<Condition> {
        let field = match filter.field.as_str() {
            "doc_id" => PAYLOAD_DOC_ID,
            "chunk_id" => PAYLOAD_CHUNK_ID,
            _ => return Err(CocoError::user("unsupported filter field")),
        };
        let value = match &filter.value {
            coco_protocol::FilterValue::String(value) => value,
            _ => return Err(CocoError::user("qdrant filters require string values")),
        };
        match filter.op {
            FilterOp::Eq => Ok(Condition::matches(field, value.clone())),
            FilterOp::Contains => Ok(Condition::matches_text(field, value.clone())),
            _ => Err(CocoError::user("unsupported filter op for qdrant")),
        }
    }

    pub(super) async fn collect_search_hits(
        &self,
        points: Vec<qdrant_client::qdrant::ScoredPoint>,
        config_id: &str,
    ) -> CocoResult<Vec<SearchHit>> {
        let pg = self.pg.with_config(Some(config_id.to_string()));
        let mut ids = Vec::new();
        let mut seen = HashSet::new();
        for point in &points {
            let Some(chunk_id) = payload_string(&point.payload, PAYLOAD_CHUNK_ID) else {
                continue;
            };
            if seen.insert(chunk_id.to_string()) {
                ids.push(chunk_id.to_string());
            }
        }
        let mut chunks = pg.get_chunks_by_ids(&ids).await?;
        let mut results = Vec::with_capacity(points.len());
        for point in points {
            let Some(chunk_id) = payload_string(&point.payload, PAYLOAD_CHUNK_ID) else {
                continue;
            };
            let Some(chunk) = chunks.remove(chunk_id) else {
                continue;
            };
            let score = self.score_from_qdrant(point.score);
            results.push(SearchHit {
                meta: SearchHitMeta {
                    score,
                    quality: chunk.quality_score,
                    verified: chunk.verified,
                },
                chunk,
            });
        }
        Ok(normalize_scores(results))
    }

}
