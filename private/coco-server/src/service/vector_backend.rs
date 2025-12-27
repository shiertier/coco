use coco_protocol::{CocoResult, VectorMetric, VectorIndexParams};

use super::state::AppState;
use crate::storage::pg::{PgBackendConfig, PgVectorMetric};
use crate::storage::vector::{ServerVectorBackend, ServerVectorBackendConfig};

#[derive(Debug, Clone)]
pub(crate) struct VectorBackendRequest {
    pub(crate) org_id: String,
    pub(crate) user_id: String,
    pub(crate) project_id: String,
    pub(crate) version_id: String,
    pub(crate) config_id: String,
    pub(crate) vector_metric: VectorMetric,
    pub(crate) index_params: Option<VectorIndexParams>,
    pub(crate) use_collection_alias: bool,
}

pub(crate) async fn build_vector_backend(
    state: &AppState,
    request: VectorBackendRequest,
) -> CocoResult<ServerVectorBackend> {
    let mut pg = PgBackendConfig::new(
        state.database_url.clone(),
        request.org_id.clone(),
        request.user_id.clone(),
        request.project_id.clone(),
    );
    state.query_pg_pool.apply(&mut pg);
    pg.tenant.version_id = Some(request.version_id);
    pg.tenant.config_id = Some(request.config_id);
    pg.metric = PgVectorMetric::from(request.vector_metric);
    let config = ServerVectorBackendConfig {
        kind: state.vector_backend_kind,
        pg,
        vector_db: state.vector_db.clone(),
        vector_metric: request.vector_metric,
        index_params: request.index_params,
        use_collection_alias: request.use_collection_alias,
    };
    ServerVectorBackend::connect(config).await
}
