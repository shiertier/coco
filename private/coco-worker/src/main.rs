use std::net::SocketAddr;
use std::path::{Path, PathBuf};
use std::sync::Arc;
use std::time::{Duration, Instant};

use chrono::Utc;
use coco_protocol::{
    normalize_config_id, validate_indexing_plan, Chunk, CocoError, CocoResult, IndexingPlan,
    StorageBackend, TextSpan, VectorMetric,
};
use coco_server::queue::RedisQueue;
use coco_server::storage::meta::{
    IngestJobRecord, NewDocument, ServerMetaStore, JOB_STATUS_COMPLETED, JOB_STATUS_FAILED,
    JOB_STATUS_QUEUED,
};
use coco_server::storage::pg::{PgBackend, PgBackendConfig, PgVectorMetric};
use serde::{Deserialize, Serialize};
use sha2::{Digest, Sha256};
use tokio::task::JoinError;
use tokio::time::sleep;
use tonic::{Request, Response, Status};
use tonic::transport::Server;
use tracing::{error, info, warn};
use wasmtime::{Config as WasmConfig, Engine, Linker, Module, Store, StoreLimitsBuilder, TypedFunc};

mod worker_ipc {
    tonic::include_proto!("coco.worker.v1");
}

const DEFAULT_POLL_INTERVAL_MS: u64 = 500;
const DEFAULT_BATCH_SIZE: usize = 256;
const DEFAULT_MAX_ATTEMPTS: i32 = 3;
const DEFAULT_RETRY_BACKOFF_MS: u64 = 500;
const DEFAULT_QUEUE_MODE: &str = "postgres";
const DEFAULT_REDIS_QUEUE: &str = "coco:ingest";
const DEFAULT_REDIS_BLOCK_SECS: u64 = 2;
const WORKER_STATUS_ID: &str = "worker";
const WORKER_HEARTBEAT_SECS: u64 = 30;
const DEFAULT_WASM_MAX_MEMORY_BYTES: u64 = 64 * 1024 * 1024;
const DEFAULT_WASM_MAX_FUEL: u64 = 5_000_000;
const DEFAULT_WASM_FAILURE_MODE: &str = "skip";

#[derive(Debug, Clone, Copy, Default)]
struct PgPoolConfig {
    max_connections: Option<u32>,
    min_connections: Option<u32>,
    connect_timeout: Option<Duration>,
}

impl PgPoolConfig {
    fn apply(&self, config: &mut PgBackendConfig) {
        if let Some(max_connections) = self.max_connections {
            config.max_connections = max_connections;
        }
        if let Some(min_connections) = self.min_connections {
            config.min_connections = min_connections;
        }
        if let Some(connect_timeout) = self.connect_timeout {
            config.connect_timeout = connect_timeout;
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum WasmFailureMode {
    Skip,
    Fail,
}

#[derive(Debug, Clone)]
struct WasmRuleConfig {
    module_dir: Option<PathBuf>,
    max_memory_bytes: u64,
    max_fuel: u64,
    failure_mode: WasmFailureMode,
}

impl Default for WasmRuleConfig {
    fn default() -> Self {
        Self {
            module_dir: None,
            max_memory_bytes: DEFAULT_WASM_MAX_MEMORY_BYTES,
            max_fuel: DEFAULT_WASM_MAX_FUEL,
            failure_mode: WasmFailureMode::Skip,
        }
    }
}

#[tokio::main]
async fn main() -> CocoResult<()> {
    tracing_subscriber::fmt::init();
    let config = WorkerConfig::from_env()?;
    run_worker(config).await
}

#[derive(Clone)]
struct WorkerState {
    config: WorkerConfig,
    meta: ServerMetaStore,
}

async fn run_worker(config: WorkerConfig) -> CocoResult<()> {
    let meta = ServerMetaStore::connect(&config.database_url).await?;
    meta.upsert_worker_status(WORKER_STATUS_ID, env!("CARGO_PKG_VERSION"))
        .await?;
    let redis_queue = if config.queue_mode == QueueMode::Redis {
        let url = config
            .redis_url
            .as_ref()
            .ok_or_else(|| CocoError::user("COCO_REDIS_URL required for redis queue"))?;
        Some(RedisQueue::connect(url, config.redis_queue.clone()).await?)
    } else {
        None
    };
    let state = Arc::new(WorkerState {
        config: config.clone(),
        meta: meta.clone(),
    });
    let queue_task = tokio::spawn(run_queue_loop(state.clone(), redis_queue));
    let result = match config.worker_addr.clone() {
        Some(addr) => {
            let ipc_task = tokio::spawn(run_ipc_server(state.clone(), addr));
            tokio::select! {
                result = queue_task => result.map_err(|err| join_error("queue", err))?,
                result = ipc_task => result.map_err(|err| join_error("ipc", err))?,
            }
        }
        None => queue_task
            .await
            .map_err(|err| join_error("queue", err))?,
    };
    result?;
    Ok(())
}

async fn run_queue_loop(
    state: Arc<WorkerState>,
    redis_queue: Option<RedisQueue>,
) -> CocoResult<()> {
    let mut last_heartbeat = Instant::now();
    loop {
        if last_heartbeat.elapsed() >= Duration::from_secs(WORKER_HEARTBEAT_SECS) {
            if let Err(err) = state
                .meta
                .upsert_worker_status(WORKER_STATUS_ID, env!("CARGO_PKG_VERSION"))
                .await
            {
                warn!("failed to update worker status: {err}");
            } else {
                last_heartbeat = Instant::now();
            }
        }
        if state.config.queue_mode == QueueMode::Redis {
            if let Some(queue) = &redis_queue {
                if let Err(err) = queue.pop(state.config.redis_block).await {
                    warn!("redis queue pop failed: {err}");
                    sleep(state.config.poll_interval).await;
                    continue;
                }
            }
        }

        match state.meta.claim_next_ingest_job().await? {
            Some(job) => {
                let job_id = job.id.clone();
                info!("claimed ingest job {}", job_id);
                if let Err(err) = process_job(&state.config, &state.meta, job.clone()).await {
                    error!("job {} failed: {err}", job_id);
                    handle_job_failure(&state.config, &state.meta, &job_id, job.attempts, err)
                        .await?;
                }
            }
            None => {
                sleep(state.config.poll_interval).await;
            }
        }
    }
}

async fn run_ipc_server(state: Arc<WorkerState>, addr: String) -> CocoResult<()> {
    let socket_addr: SocketAddr = addr
        .parse()
        .map_err(|_| CocoError::user("COCO_WORKER_ADDR must be host:port"))?;
    let service = WorkerIpcService { state };
    Server::builder()
        .add_service(worker_ipc::worker_service_server::WorkerServiceServer::new(
            service,
        ))
        .serve(socket_addr)
        .await
        .map_err(|err| CocoError::network(format!("worker IPC server failed: {err}")))?;
    Ok(())
}

#[derive(Clone)]
struct WorkerIpcService {
    state: Arc<WorkerState>,
}

#[tonic::async_trait]
impl worker_ipc::worker_service_server::WorkerService for WorkerIpcService {
    async fn ping(
        &self,
        _request: Request<worker_ipc::PingRequest>,
    ) -> Result<Response<worker_ipc::PingResponse>, Status> {
        Ok(Response::new(worker_ipc::PingResponse {
            version: env!("CARGO_PKG_VERSION").to_string(),
        }))
    }

    async fn submit_ingest(
        &self,
        request: Request<worker_ipc::SubmitIngestRequest>,
    ) -> Result<Response<worker_ipc::SubmitIngestResponse>, Status> {
        let worker_ipc::SubmitIngestRequest {
            job_id,
            org_id,
            project_id,
            api_version,
            request,
            blob_ref,
            user_id,
        } = request.into_inner();
        if job_id.trim().is_empty() {
            return Err(Status::invalid_argument("job_id must not be empty"));
        }
        if api_version.trim().is_empty() {
            return Err(Status::invalid_argument("api_version must not be empty"));
        }
        if user_id.trim().is_empty() {
            return Err(Status::invalid_argument("user_id must not be empty"));
        }
        let ingest_request = match request {
            Some(payload) => Some(IngestBatchPayload::try_from(payload).map_err(map_ipc_error)?),
            None => None,
        };
        let job = self
            .state
            .meta
            .claim_ingest_job_by_id(&job_id)
            .await
            .map_err(map_ipc_error)?;
        let mut job_payload: IngestJobPayload = serde_json::from_str(&job.payload)
            .map_err(|err| map_ipc_error(CocoError::system(err)))?;
        job_payload.api_version = api_version;
        if let Some(ingest_request) = ingest_request {
            job_payload.request = Some(ingest_request);
        }
        if blob_ref.is_some() {
            job_payload.blob_ref = blob_ref;
        }
        if job.org_id != org_id || job.project_id != project_id || job.user_id != user_id {
            return Err(Status::invalid_argument(
                "job identifiers do not match request",
            ));
        }
        let job_id = job.id.clone();
        let attempts = job.attempts;
        let context = IngestJobContext {
            job_id: job_id.clone(),
            org_id: job.org_id.clone(),
            user_id: job.user_id.clone(),
            project_id: job.project_id.clone(),
            payload: IngestJobPayload {
                api_version: job_payload.api_version,
                request: job_payload.request,
                blob_ref: job_payload.blob_ref,
                plan: job_payload.plan,
                wasm_module_ref: job_payload.wasm_module_ref,
            },
        };
        let state = Arc::clone(&self.state);
        tokio::spawn(async move {
            if let Err(err) = process_payload(&state.config, &state.meta, context).await {
                error!("job {} failed: {err}", job_id);
                if let Err(err) = handle_job_failure(
                    &state.config,
                    &state.meta,
                    &job_id,
                    attempts,
                    err,
                )
                .await
                {
                    warn!("failed to update job {}: {err}", job_id);
                }
            }
        });
        Ok(Response::new(worker_ipc::SubmitIngestResponse {
            job_id: job.id,
        }))
    }
}

fn join_error(task: &str, err: JoinError) -> CocoError {
    CocoError::system(format!("{task} task failed: {err}"))
}

fn map_ipc_error(err: CocoError) -> Status {
    match err.kind() {
        coco_protocol::CocoErrorKind::User => Status::invalid_argument(err.to_string()),
        coco_protocol::CocoErrorKind::Network | coco_protocol::CocoErrorKind::Storage => {
            Status::unavailable(err.to_string())
        }
        coco_protocol::CocoErrorKind::System | coco_protocol::CocoErrorKind::Compute => {
            Status::internal(err.to_string())
        }
    }
}

async fn process_job(
    config: &WorkerConfig,
    meta: &ServerMetaStore,
    job: IngestJobRecord,
) -> CocoResult<()> {
    let payload: IngestJobPayload =
        serde_json::from_str(&job.payload).map_err(CocoError::system)?;
    let context = IngestJobContext {
        job_id: job.id,
        org_id: job.org_id,
        user_id: job.user_id,
        project_id: job.project_id,
        payload,
    };
    process_payload(config, meta, context).await
}

async fn process_payload(
    config: &WorkerConfig,
    meta: &ServerMetaStore,
    context: IngestJobContext,
) -> CocoResult<()> {
    let project = meta
        .get_project(&context.org_id, &context.user_id, &context.project_id)
        .await?
        .ok_or_else(|| CocoError::user("project not found for ingest job"))?;
    ensure_version_compatible(&context.payload.api_version)?;
    let _plan = resolve_indexing_plan(&context.payload)?;
    let wasm_module_ref = context.payload.wasm_module_ref.clone();
    let (request, blob_path) =
        resolve_ingest_request(context.payload, config.ingest_blob_dir.as_deref()).await?;
    if let Some(module_ref) = wasm_module_ref.as_deref() {
        match apply_wasm_rule(&config.wasm_rule, module_ref, &request).await? {
            WasmDecision::Skip => {
                meta.update_ingest_job_status(&context.job_id, JOB_STATUS_COMPLETED, None, None)
                    .await?;
                if let Some(path) = blob_path {
                    if let Err(err) = tokio::fs::remove_file(&path).await {
                        warn!("failed to remove ingest blob {}: {err}", path.display());
                    }
                }
                return Ok(());
            }
            WasmDecision::Proceed => {}
        }
    }
    if request.documents.is_empty() {
        return Err(CocoError::user("documents must not be empty"));
    }

    let incoming_stats = IncomingStats::from_request(&request)?;
    enforce_org_quotas(meta, &context.org_id, &context.user_id, &incoming_stats).await?;

    let config_id = match request.indexing_config_id.as_deref() {
        Some(config_id) => {
            let normalized = normalize_config_id(config_id)?;
            if normalized != config_id {
                return Err(CocoError::user("indexing_config_id must be normalized"));
            }
            config_id.to_string()
        }
        None => project.active_config_id.clone(),
    };
    let indexing_config = meta
        .get_indexing_config(&context.org_id, &config_id)
        .await?
        .ok_or_else(|| CocoError::user("indexing config not found"))?;
    if indexing_config.vector_metric != VectorMetric::L2 {
        return Err(CocoError::user(
            "vector_metric must match server vector index",
        ));
    }
    let embedding_dimensions = indexing_config.embedding.dimensions.ok_or_else(|| {
        CocoError::user("embedding dimensions must be set for server indexing configs")
    })?;
    let embedding_dimensions = usize::try_from(embedding_dimensions)
        .map_err(|_| CocoError::user("embedding dimensions out of range"))?;
    let version = meta
        .create_project_version(
            &context.org_id,
            &context.user_id,
            &context.project_id,
            &config_id,
        )
        .await?;

    let mut config_backend = PgBackendConfig::new(
        config.database_url.clone(),
        context.org_id.clone(),
        context.user_id.clone(),
        context.project_id.clone(),
    );
    config.ingest_pg_pool.apply(&mut config_backend);
    config_backend.metric = PgVectorMetric::from(indexing_config.vector_metric);
    let backend = PgBackend::connect(config_backend).await?;
    backend
        .ensure_indexes(
            Some(&config_id),
            indexing_config.index_params.as_ref(),
        )
        .await?;
    let backend = backend
        .with_version(Some(version.id.clone()))
        .with_config(Some(config_id.clone()));

    let indexed_at = Utc::now();
    let mut chunks = Vec::new();
    for document in request.documents {
        let content_hash = document
            .content_hash
            .clone()
            .unwrap_or_else(|| hash_chunks(&document.chunks));
        meta.upsert_document(NewDocument {
            id: document.doc_id.clone(),
            org_id: context.org_id.clone(),
            user_id: context.user_id.clone(),
            project_id: context.project_id.clone(),
            config_id: config_id.clone(),
            source_ref: document.source_ref.clone(),
            title: document.title.clone(),
            content_hash,
            indexed_at,
            quality_score: document.quality_score,
            verified: document.verified.unwrap_or(false),
        })
        .await?;

        for chunk in document.chunks {
            if chunk.embedding.len() != embedding_dimensions {
                return Err(CocoError::user(
                    "embedding dimensions must match indexing config",
                ));
            }
            chunks.push(Chunk {
                id: chunk.chunk_id.into(),
                doc_id: document.doc_id.clone().into(),
                content: chunk.content,
                embedding: Some(chunk.embedding),
                span: TextSpan::new(chunk.start, chunk.end)?,
                quality_score: chunk.quality_score,
                verified: chunk.verified,
            });
        }
    }

    upsert_chunks_batched(&backend, chunks, config.batch_size).await?;

    if request.activate {
        meta.activate_project_version(
            &context.org_id,
            &context.user_id,
            &context.project_id,
            &version.id,
        )
        .await?;
    }

    if incoming_stats.total_chunks > 0 {
        meta.increment_embedding_calls(
            &context.org_id,
            &context.user_id,
            Utc::now().date_naive(),
            incoming_stats.total_chunks,
        )
        .await?;
    }

    meta.update_ingest_job_status(
        &context.job_id,
        JOB_STATUS_COMPLETED,
        None,
        Some(version.id.clone()),
    )
    .await?;
    if let Some(path) = blob_path {
        if let Err(err) = tokio::fs::remove_file(&path).await {
            warn!("failed to remove ingest blob {}: {err}", path.display());
        }
    }

    Ok(())
}

fn resolve_indexing_plan(payload: &IngestJobPayload) -> CocoResult<IndexingPlan> {
    let plan = payload.plan.clone().unwrap_or_default();
    validate_indexing_plan(&plan)?;
    Ok(plan)
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum WasmDecision {
    Skip,
    Proceed,
}

async fn apply_wasm_rule(
    config: &WasmRuleConfig,
    module_ref: &str,
    request: &IngestBatchPayload,
) -> CocoResult<WasmDecision> {
    let total_docs = i32::try_from(request.documents.len())
        .map_err(|_| CocoError::user("document count exceeds i32 range"))?;
    let mut total_chunks: i32 = 0;
    for document in &request.documents {
        let chunks = i32::try_from(document.chunks.len())
            .map_err(|_| CocoError::user("chunk count exceeds i32 range"))?;
        total_chunks = total_chunks.saturating_add(chunks);
    }
    let module_dir = config.module_dir.clone();
    let module_ref = module_ref.to_string();
    let max_memory_bytes = config.max_memory_bytes;
    let max_fuel = config.max_fuel;
    let handle = tokio::task::spawn_blocking(move || {
        let module_path = resolve_wasm_module_path(&module_ref, module_dir.as_deref())?;
        let bytes = std::fs::read(&module_path).map_err(CocoError::storage)?;

        let mut engine_config = WasmConfig::new();
        if max_fuel > 0 {
            engine_config.consume_fuel(true);
        }
        let engine = Engine::new(&engine_config).map_err(CocoError::compute)?;
        let module = Module::from_binary(&engine, &bytes).map_err(CocoError::compute)?;

        let max_memory = usize::try_from(max_memory_bytes)
            .map_err(|_| CocoError::user("wasm memory limit exceeds usize"))?;
        let limits = StoreLimitsBuilder::new().memory_size(max_memory).build();
        let mut store = Store::new(&engine, limits);
        store.limiter(|state| state);
        if max_fuel > 0 {
            store.set_fuel(max_fuel).map_err(CocoError::compute)?;
        }
        let linker = Linker::new(&engine);
        let instance = linker
            .instantiate(&mut store, &module)
            .map_err(CocoError::compute)?;

        let decision = match instance.get_typed_func::<(i32, i32), i32>(&mut store, "coco_rule")
        {
            Ok(func) => call_rule(&mut store, func, total_docs, total_chunks)?,
            Err(_) => {
                let func = instance
                    .get_typed_func::<(), i32>(&mut store, "coco_rule")
                    .map_err(CocoError::compute)?;
                func.call(&mut store, ()).map_err(CocoError::compute)?
            }
        };
        Ok::<_, CocoError>(if decision == 0 {
            WasmDecision::Skip
        } else {
            WasmDecision::Proceed
        })
    });

    match handle.await {
        Ok(result) => match result {
            Ok(decision) => Ok(decision),
            Err(err) => handle_wasm_error(err, config.failure_mode),
        },
        Err(err) => handle_wasm_error(
            CocoError::system(format!("wasm rule task failed: {err}")),
            config.failure_mode,
        ),
    }
}

fn call_rule(
    store: &mut Store<wasmtime::StoreLimits>,
    func: TypedFunc<(i32, i32), i32>,
    total_docs: i32,
    total_chunks: i32,
) -> CocoResult<i32> {
    func.call(store, (total_docs, total_chunks))
        .map_err(CocoError::compute)
}

fn handle_wasm_error(error: CocoError, mode: WasmFailureMode) -> CocoResult<WasmDecision> {
    match mode {
        WasmFailureMode::Skip => {
            warn!("wasm rule failed, skipping rule: {error}");
            Ok(WasmDecision::Proceed)
        }
        WasmFailureMode::Fail => Err(error),
    }
}

fn resolve_wasm_module_path(
    module_ref: &str,
    base_dir: Option<&Path>,
) -> CocoResult<PathBuf> {
    let base_dir = match base_dir {
        Some(dir) => Some(std::fs::canonicalize(dir).map_err(CocoError::storage)?),
        None => None,
    };
    let mut path = PathBuf::from(module_ref);
    if path.is_relative() {
        let base_dir = base_dir.as_ref().ok_or_else(|| {
            CocoError::user("COCO_INGEST_WASM_DIR must be set for relative wasm_module_ref")
        })?;
        path = base_dir.join(path);
    }
    let canonical_path = std::fs::canonicalize(&path).map_err(CocoError::storage)?;
    if let Some(base_dir) = base_dir {
        if !canonical_path.starts_with(&base_dir) {
            return Err(CocoError::user("wasm_module_ref is outside COCO_INGEST_WASM_DIR"));
        }
    }
    Ok(canonical_path)
}

async fn resolve_ingest_request(
    payload: IngestJobPayload,
    blob_dir: Option<&Path>,
) -> CocoResult<(IngestBatchPayload, Option<PathBuf>)> {
    if let Some(request) = payload.request {
        return Ok((request, None));
    }
    let blob_ref = payload
        .blob_ref
        .ok_or_else(|| CocoError::user("ingest payload missing"))?;
    let (request, path) = load_blob_payload(&blob_ref, blob_dir).await?;
    Ok((request, Some(path)))
}

async fn load_blob_payload(
    blob_ref: &str,
    blob_dir: Option<&Path>,
) -> CocoResult<(IngestBatchPayload, PathBuf)> {
    let blob_ref = blob_ref.to_string();
    let base_dir = blob_dir.map(PathBuf::from);
    tokio::task::spawn_blocking(move || {
        let base_dir = base_dir
            .ok_or_else(|| CocoError::user("COCO_INGEST_BLOB_DIR must be set to read blob_ref"))?;
        let mut path = PathBuf::from(&blob_ref);
        if !path.is_absolute() {
            path = base_dir.join(path);
        }
        let canonical_base = std::fs::canonicalize(&base_dir).map_err(CocoError::storage)?;
        let canonical_path = std::fs::canonicalize(&path).map_err(CocoError::storage)?;
        if !canonical_path.starts_with(&canonical_base) {
            return Err(CocoError::user("blob_ref is outside COCO_INGEST_BLOB_DIR"));
        }
        let file = std::fs::File::open(&canonical_path).map_err(CocoError::storage)?;
        let payload = serde_json::from_reader(file).map_err(CocoError::storage)?;
        Ok((payload, canonical_path))
    })
    .await
    .map_err(|err| CocoError::system(format!("blob load task failed: {err}")))?
}

#[derive(Debug, Clone)]
struct WorkerConfig {
    database_url: String,
    poll_interval: Duration,
    batch_size: usize,
    max_attempts: i32,
    retry_backoff: Duration,
    queue_mode: QueueMode,
    redis_url: Option<String>,
    redis_queue: String,
    redis_block: Duration,
    worker_addr: Option<String>,
    ingest_pg_pool: PgPoolConfig,
    ingest_blob_dir: Option<PathBuf>,
    wasm_rule: WasmRuleConfig,
}

impl WorkerConfig {
    fn from_env() -> CocoResult<Self> {
        let database_url = env_required("COCO_DB_URL")?;
        let poll_interval_ms = env_u64("COCO_WORKER_POLL_MS", DEFAULT_POLL_INTERVAL_MS)?;
        let batch_size = env_usize("COCO_WORKER_BATCH_SIZE", DEFAULT_BATCH_SIZE)?;
        if batch_size == 0 {
            return Err(CocoError::user(
                "COCO_WORKER_BATCH_SIZE must be greater than zero",
            ));
        }
        let max_attempts = env_i32("COCO_WORKER_MAX_ATTEMPTS", DEFAULT_MAX_ATTEMPTS)?;
        if max_attempts <= 0 {
            return Err(CocoError::user(
                "COCO_WORKER_MAX_ATTEMPTS must be greater than zero",
            ));
        }
        let retry_backoff_ms = env_u64("COCO_WORKER_RETRY_BACKOFF_MS", DEFAULT_RETRY_BACKOFF_MS)?;
        let queue_mode = queue_mode_from_env()?;
        let redis_url = env_optional("COCO_REDIS_URL");
        if queue_mode == QueueMode::Redis && redis_url.is_none() {
            return Err(CocoError::user("COCO_REDIS_URL required for redis queue"));
        }
        let redis_queue =
            env_optional("COCO_REDIS_QUEUE").unwrap_or_else(|| DEFAULT_REDIS_QUEUE.to_string());
        let redis_block_secs = env_u64("COCO_REDIS_BLOCK_SECS", DEFAULT_REDIS_BLOCK_SECS)?;
        let worker_addr = env_optional("COCO_WORKER_ADDR");
        let ingest_pg_pool = pg_pool_config_from_env("COCO_INGEST")?;
        let ingest_blob_dir = env_optional("COCO_INGEST_BLOB_DIR").map(PathBuf::from);
        let wasm_rule = wasm_rule_config_from_env()?;
        Ok(Self {
            database_url,
            poll_interval: Duration::from_millis(poll_interval_ms),
            batch_size,
            max_attempts,
            retry_backoff: Duration::from_millis(retry_backoff_ms),
            queue_mode,
            redis_url,
            redis_queue,
            redis_block: Duration::from_secs(redis_block_secs),
            worker_addr,
            ingest_pg_pool,
            ingest_blob_dir,
            wasm_rule,
        })
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum QueueMode {
    Postgres,
    Redis,
}

#[derive(Debug, Deserialize, Serialize)]
struct IngestJobPayload {
    api_version: String,
    #[serde(default)]
    request: Option<IngestBatchPayload>,
    #[serde(default)]
    blob_ref: Option<String>,
    #[serde(default)]
    plan: Option<IndexingPlan>,
    #[serde(default)]
    wasm_module_ref: Option<String>,
}

#[derive(Debug)]
struct IngestJobContext {
    job_id: String,
    org_id: String,
    user_id: String,
    project_id: String,
    payload: IngestJobPayload,
}

#[derive(Debug, Deserialize, Serialize)]
struct IngestBatchPayload {
    #[serde(default = "default_true")]
    activate: bool,
    #[serde(default)]
    indexing_config_id: Option<String>,
    documents: Vec<IngestDocument>,
}

#[derive(Debug, Deserialize, Serialize)]
struct IngestDocument {
    doc_id: String,
    source_ref: String,
    #[serde(default)]
    title: Option<String>,
    #[serde(default)]
    content_hash: Option<String>,
    #[serde(default)]
    quality_score: Option<f32>,
    #[serde(default)]
    verified: Option<bool>,
    chunks: Vec<IngestChunk>,
}

#[derive(Debug, Deserialize, Serialize)]
struct IngestChunk {
    chunk_id: String,
    content: String,
    embedding: Vec<f32>,
    start: usize,
    end: usize,
    #[serde(default)]
    quality_score: Option<f32>,
    #[serde(default)]
    verified: Option<bool>,
}

impl TryFrom<worker_ipc::IngestBatchRequest> for IngestBatchPayload {
    type Error = CocoError;

    fn try_from(value: worker_ipc::IngestBatchRequest) -> Result<Self, Self::Error> {
        let documents = value
            .documents
            .into_iter()
            .map(IngestDocument::try_from)
            .collect::<CocoResult<Vec<_>>>()?;
        Ok(Self {
            activate: value.activate.unwrap_or(true),
            indexing_config_id: value.indexing_config_id,
            documents,
        })
    }
}

impl TryFrom<worker_ipc::IngestDocument> for IngestDocument {
    type Error = CocoError;

    fn try_from(value: worker_ipc::IngestDocument) -> Result<Self, Self::Error> {
        let chunks = value
            .chunks
            .into_iter()
            .map(IngestChunk::try_from)
            .collect::<CocoResult<Vec<_>>>()?;
        Ok(Self {
            doc_id: value.doc_id,
            source_ref: value.source_ref,
            title: value.title,
            content_hash: value.content_hash,
            quality_score: value.quality_score,
            verified: value.verified,
            chunks,
        })
    }
}

impl TryFrom<worker_ipc::IngestChunk> for IngestChunk {
    type Error = CocoError;

    fn try_from(value: worker_ipc::IngestChunk) -> Result<Self, Self::Error> {
        let start = usize::try_from(value.start)
            .map_err(|_| CocoError::user("chunk start is out of range"))?;
        let end =
            usize::try_from(value.end).map_err(|_| CocoError::user("chunk end is out of range"))?;
        Ok(Self {
            chunk_id: value.chunk_id,
            content: value.content,
            embedding: value.embedding,
            start,
            end,
            quality_score: value.quality_score,
            verified: value.verified,
        })
    }
}

#[derive(Debug, Clone)]
struct IncomingStats {
    total_documents: i64,
    total_chunks: i64,
    total_bytes: i64,
}

impl IncomingStats {
    fn from_request(request: &IngestBatchPayload) -> CocoResult<Self> {
        let total_documents = i64::try_from(request.documents.len())
            .map_err(|_| CocoError::user("document count exceeds i64 range"))?;
        let mut total_chunks: i64 = 0;
        let mut total_bytes: i64 = 0;
        for document in &request.documents {
            let chunks_len = i64::try_from(document.chunks.len())
                .map_err(|_| CocoError::user("chunk count exceeds i64 range"))?;
            total_chunks = total_chunks.saturating_add(chunks_len);
            for chunk in &document.chunks {
                let bytes = i64::try_from(chunk.content.len())
                    .map_err(|_| CocoError::user("chunk size exceeds i64 range"))?;
                total_bytes = total_bytes.saturating_add(bytes);
            }
        }
        Ok(Self {
            total_documents,
            total_chunks,
            total_bytes,
        })
    }
}

fn hash_chunks(chunks: &[IngestChunk]) -> String {
    let mut hasher = Sha256::new();
    for chunk in chunks {
        hasher.update(chunk.content.as_bytes());
    }
    bytes_to_hex(&hasher.finalize())
}

fn bytes_to_hex(bytes: &[u8]) -> String {
    let mut output = String::with_capacity(bytes.len() * 2);
    for byte in bytes {
        output.push(hex_digit(byte >> 4));
        output.push(hex_digit(byte & 0x0f));
    }
    output
}

fn hex_digit(value: u8) -> char {
    const HEX: &[u8; 16] = b"0123456789abcdef";
    HEX[value as usize] as char
}

fn ensure_version_compatible(api_version: &str) -> CocoResult<()> {
    let worker_version = env!("CARGO_PKG_VERSION");
    let api_major = parse_major_version(api_version)?;
    let worker_major = parse_major_version(worker_version)?;
    if api_major != worker_major {
        return Err(CocoError::system(format!(
            "version mismatch: api={api_version} worker={worker_version}"
        )));
    }
    Ok(())
}

fn parse_major_version(version: &str) -> CocoResult<u64> {
    let major = version
        .split('.')
        .next()
        .ok_or_else(|| CocoError::system("invalid version format"))?;
    major
        .parse::<u64>()
        .map_err(|_| CocoError::system("invalid version format"))
}

async fn enforce_org_quotas(
    meta: &ServerMetaStore,
    org_id: &str,
    user_id: &str,
    incoming: &IncomingStats,
) -> CocoResult<()> {
    let org = meta
        .get_organization(org_id)
        .await?
        .ok_or_else(|| CocoError::user("organization not found"))?;
    let max_documents = normalize_limit(org.max_documents);
    let max_chunks = normalize_limit(org.max_chunks);
    let max_storage_bytes = normalize_limit(org.max_storage_bytes);
    let max_embeddings_per_day = normalize_limit(org.max_embeddings_per_day);

    if let Some(limit) = max_documents {
        let current = meta.count_documents_by_org(org_id, user_id).await?;
        if current.saturating_add(incoming.total_documents) > limit {
            return Err(CocoError::user("document quota exceeded"));
        }
    }

    if let Some(limit) = max_chunks {
        let current = meta.count_chunks_by_org(org_id, user_id).await?;
        if current.saturating_add(incoming.total_chunks) > limit {
            return Err(CocoError::user("chunk quota exceeded"));
        }
    }

    if let Some(limit) = max_storage_bytes {
        let current = meta.sum_chunk_bytes_by_org(org_id, user_id).await?;
        if current.saturating_add(incoming.total_bytes) > limit {
            return Err(CocoError::user("storage quota exceeded"));
        }
    }

    if let Some(limit) = max_embeddings_per_day {
        let today = Utc::now().date_naive();
        let current = meta
            .get_embedding_calls_for_day(org_id, user_id, today)
            .await?;
        if current.saturating_add(incoming.total_chunks) > limit {
            return Err(CocoError::user("embedding quota exceeded"));
        }
    }

    Ok(())
}

fn normalize_limit(limit: Option<i64>) -> Option<i64> {
    match limit {
        Some(value) if value > 0 => Some(value),
        _ => None,
    }
}

async fn handle_job_failure(
    config: &WorkerConfig,
    meta: &ServerMetaStore,
    job_id: &str,
    attempts: i32,
    err: CocoError,
) -> CocoResult<()> {
    let error = Some(format!("{err}"));
    if attempts < config.max_attempts {
        meta.update_ingest_job_status(job_id, JOB_STATUS_QUEUED, error, None)
            .await?;
        let delay = retry_delay(config.retry_backoff, attempts);
        sleep(delay).await;
        return Ok(());
    }
    meta.update_ingest_job_status(job_id, JOB_STATUS_FAILED, error, None)
        .await?;
    Ok(())
}

fn retry_delay(base: Duration, attempt: i32) -> Duration {
    let capped = attempt.clamp(1, 64) as u32;
    let exp = capped.saturating_sub(1).min(6);
    base.saturating_mul(1_u32 << exp)
}

fn queue_mode_from_env() -> CocoResult<QueueMode> {
    let raw = env_optional("COCO_QUEUE_MODE").unwrap_or_else(|| DEFAULT_QUEUE_MODE.to_string());
    match raw.to_ascii_lowercase().as_str() {
        "postgres" | "pg" => Ok(QueueMode::Postgres),
        "redis" => Ok(QueueMode::Redis),
        other => Err(CocoError::user(format!(
            "unsupported COCO_QUEUE_MODE value: {other}"
        ))),
    }
}

fn wasm_rule_config_from_env() -> CocoResult<WasmRuleConfig> {
    let module_dir = env_optional("COCO_INGEST_WASM_DIR").map(PathBuf::from);
    let max_memory_bytes = env_u64(
        "COCO_INGEST_WASM_MAX_MEMORY_BYTES",
        DEFAULT_WASM_MAX_MEMORY_BYTES,
    )?;
    if max_memory_bytes == 0 {
        return Err(CocoError::user(
            "COCO_INGEST_WASM_MAX_MEMORY_BYTES must be greater than zero",
        ));
    }
    let max_fuel = env_u64("COCO_INGEST_WASM_MAX_FUEL", DEFAULT_WASM_MAX_FUEL)?;
    let failure_mode = wasm_failure_mode_from_env()?;
    Ok(WasmRuleConfig {
        module_dir,
        max_memory_bytes,
        max_fuel,
        failure_mode,
    })
}

fn wasm_failure_mode_from_env() -> CocoResult<WasmFailureMode> {
    let raw = env_optional("COCO_INGEST_WASM_FAILURE_MODE")
        .unwrap_or_else(|| DEFAULT_WASM_FAILURE_MODE.to_string());
    match raw.to_ascii_lowercase().as_str() {
        "skip" => Ok(WasmFailureMode::Skip),
        "fail" => Ok(WasmFailureMode::Fail),
        other => Err(CocoError::user(format!(
            "unsupported COCO_INGEST_WASM_FAILURE_MODE value: {other}"
        ))),
    }
}

async fn upsert_chunks_batched(
    backend: &PgBackend,
    mut chunks: Vec<Chunk>,
    batch_size: usize,
) -> CocoResult<()> {
    if chunks.is_empty() {
        return Ok(());
    }
    while !chunks.is_empty() {
        let take = batch_size.min(chunks.len());
        let batch: Vec<Chunk> = chunks.drain(..take).collect();
        backend.upsert_chunks(&batch).await?;
    }
    Ok(())
}

fn env_optional(key: &str) -> Option<String> {
    std::env::var(key).ok().filter(|value| !value.is_empty())
}

fn env_required(key: &str) -> CocoResult<String> {
    env_optional(key).ok_or_else(|| CocoError::user(format!("{key} must be set")))
}

fn env_u64(key: &str, default: u64) -> CocoResult<u64> {
    match env_optional(key) {
        Some(value) => value
            .parse::<u64>()
            .map_err(|_| CocoError::user(format!("{key} must be a valid number"))),
        None => Ok(default),
    }
}

fn env_u32_optional(key: &str) -> CocoResult<Option<u32>> {
    match env_optional(key) {
        Some(value) => {
            let parsed = value
                .parse::<u32>()
                .map_err(|_| CocoError::user(format!("{key} must be a valid number")))?;
            if parsed == 0 {
                return Err(CocoError::user(format!("{key} must be greater than zero")));
            }
            Ok(Some(parsed))
        }
        None => Ok(None),
    }
}

fn env_u64_optional(key: &str) -> CocoResult<Option<u64>> {
    match env_optional(key) {
        Some(value) => {
            let parsed = value
                .parse::<u64>()
                .map_err(|_| CocoError::user(format!("{key} must be a valid number")))?;
            if parsed == 0 {
                return Err(CocoError::user(format!("{key} must be greater than zero")));
            }
            Ok(Some(parsed))
        }
        None => Ok(None),
    }
}

fn env_i32(key: &str, default: i32) -> CocoResult<i32> {
    match env_optional(key) {
        Some(value) => value
            .parse::<i32>()
            .map_err(|_| CocoError::user(format!("{key} must be a valid number"))),
        None => Ok(default),
    }
}

fn env_usize(key: &str, default: usize) -> CocoResult<usize> {
    match env_optional(key) {
        Some(value) => value
            .parse::<usize>()
            .map_err(|_| CocoError::user(format!("{key} must be a valid number"))),
        None => Ok(default),
    }
}

fn pg_pool_config_from_env(prefix: &str) -> CocoResult<PgPoolConfig> {
    let max_connections = env_u32_optional(&format!("{prefix}_PG_MAX_CONNECTIONS"))?;
    let min_connections = env_u32_optional(&format!("{prefix}_PG_MIN_CONNECTIONS"))?;
    if let (Some(min), Some(max)) = (min_connections, max_connections) {
        if min > max {
            return Err(CocoError::user(format!(
                "{prefix}_PG_MIN_CONNECTIONS must not exceed {prefix}_PG_MAX_CONNECTIONS"
            )));
        }
    }
    let connect_timeout = env_u64_optional(&format!("{prefix}_PG_CONNECT_TIMEOUT_SECS"))?
        .map(Duration::from_secs);
    Ok(PgPoolConfig {
        max_connections,
        min_connections,
        connect_timeout,
    })
}

fn default_true() -> bool {
    true
}

#[cfg(test)]
mod tests {
    use super::*;
    use coco_server::storage::meta::{
        NewIndexingConfig, NewIngestJob, NewOrganization, NewProject, DEFAULT_CONFIG_ID,
        JOB_STATUS_COMPLETED,
    };
    use coco_protocol::{ChunkingStrategy, EmbeddingConfig, VectorMetric};
    use tokio::time::Duration as TokioDuration;

    const EMBEDDING_DIM: usize = 1536;

    fn env_value(key: &str) -> Option<String> {
        std::env::var(key).ok().filter(|value| !value.is_empty())
    }

    fn make_embedding(value: f32) -> Vec<f32> {
        let mut embedding = vec![0.0_f32; EMBEDDING_DIM];
        if !embedding.is_empty() {
            embedding[0] = value;
        }
        embedding
    }

    async fn ensure_default_config(
        meta: &ServerMetaStore,
        org_id: &str,
    ) -> CocoResult<()> {
        meta.ensure_default_indexing_config(NewIndexingConfig {
            org_id: org_id.to_string(),
            config_id: DEFAULT_CONFIG_ID.to_string(),
            chunking: ChunkingStrategy {
                strategy_name: "fixed_token".to_string(),
                chunk_size: 256,
                chunk_overlap: 32,
            },
            embedding: EmbeddingConfig {
                model_name: "test".to_string(),
                dimensions: Some(EMBEDDING_DIM as u32),
            },
            vector_backend: None,
            vector_metric: VectorMetric::L2,
            index_params: None,
            created_at: Utc::now(),
        })
        .await?;
        Ok(())
    }

    #[tokio::test]
    async fn worker_processes_ingest_job() -> CocoResult<()> {
        let Some(database_url) = env_value("COCO_TEST_DB_URL") else {
            eprintln!("skipping: COCO_TEST_DB_URL not set");
            return Ok(());
        };

        let bootstrap = PgBackendConfig::new(
            database_url.clone(),
            "bootstrap".to_string(),
            "bootstrap".to_string(),
            "bootstrap".to_string(),
        );
        let _ = PgBackend::connect(bootstrap).await?;

        let meta = ServerMetaStore::connect(&database_url).await?;
        let org_id = format!("org-{}", Utc::now().timestamp_nanos_opt().unwrap_or(0));
        let project_id = format!("proj-{}", Utc::now().timestamp_nanos_opt().unwrap_or(0));
        let user_id = format!("user-{}", Utc::now().timestamp_nanos_opt().unwrap_or(0));

        meta.create_organization(NewOrganization {
            id: org_id.clone(),
            name: "Test Org".to_string(),
            created_at: Utc::now(),
            max_documents: None,
            max_chunks: None,
            max_storage_bytes: None,
            max_embeddings_per_day: None,
        })
        .await?;
        ensure_default_config(&meta, &org_id).await?;

        meta.create_project(NewProject {
            id: project_id.clone(),
            org_id: org_id.clone(),
            user_id: user_id.clone(),
            name: "Test Project".to_string(),
            created_at: Utc::now(),
            active_version_id: None,
            active_config_id: DEFAULT_CONFIG_ID.to_string(),
        })
        .await?;

        let payload = IngestJobPayload {
            api_version: env!("CARGO_PKG_VERSION").to_string(),
            request: Some(IngestBatchPayload {
                activate: true,
                indexing_config_id: None,
                documents: vec![IngestDocument {
                    doc_id: "doc-1".to_string(),
                    source_ref: "src:doc".to_string(),
                    title: Some("Doc".to_string()),
                    content_hash: Some("hash".to_string()),
                    quality_score: None,
                    verified: None,
                    chunks: vec![IngestChunk {
                        chunk_id: "chunk-1".to_string(),
                        content: "hello".to_string(),
                        embedding: make_embedding(1.0),
                        start: 0,
                        end: 5,
                        quality_score: None,
                        verified: None,
                    }],
                }],
            }),
            blob_ref: None,
            plan: Some(IndexingPlan::default()),
            wasm_module_ref: None,
        };
        let payload_json = serde_json::to_string(&payload).map_err(CocoError::system)?;
        let job_id = format!("job-{}", Utc::now().timestamp_nanos_opt().unwrap_or(0));
        let now = Utc::now();
        meta.create_ingest_job(NewIngestJob {
            id: job_id.clone(),
            org_id: org_id.clone(),
            user_id: user_id.clone(),
            project_id: project_id.clone(),
            payload: payload_json,
            created_at: now,
            updated_at: now,
        })
        .await?;

        let Some(job) = meta.claim_next_ingest_job().await? else {
            return Err(CocoError::system("expected queued job"));
        };

        let config = WorkerConfig {
            database_url,
            poll_interval: TokioDuration::from_millis(1),
            batch_size: 16,
            max_attempts: 1,
            retry_backoff: TokioDuration::from_millis(1),
            queue_mode: QueueMode::Postgres,
            redis_url: None,
            redis_queue: DEFAULT_REDIS_QUEUE.to_string(),
            redis_block: TokioDuration::from_secs(DEFAULT_REDIS_BLOCK_SECS),
            worker_addr: None,
            ingest_pg_pool: PgPoolConfig::default(),
            ingest_blob_dir: None,
            wasm_rule: WasmRuleConfig::default(),
        };

        process_job(&config, &meta, job).await?;

        let job = meta
            .get_ingest_job(&job_id)
            .await?
            .ok_or_else(|| CocoError::system("job not found after processing"))?;
        assert_eq!(job.status, JOB_STATUS_COMPLETED);
        assert!(job.version_id.is_some());

        Ok(())
    }
}
