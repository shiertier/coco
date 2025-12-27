//! Protocol types and error contracts shared across CoCo crates.

use std::collections::HashSet;
use std::fmt;
use std::future::Future;
use std::num::NonZeroU32;
use std::ops::Deref;

use serde::{Deserialize, Serialize};
use utoipa::ToSchema;

/// Stable identifier for a project.
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize, ToSchema)]
#[serde(transparent)]
#[schema(value_type = String)]
pub struct ProjectId(String);

impl ProjectId {
    pub fn new(value: impl Into<String>) -> Self {
        Self(value.into())
    }

    pub fn as_str(&self) -> &str {
        &self.0
    }
}

impl fmt::Display for ProjectId {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        self.0.fmt(f)
    }
}

impl Deref for ProjectId {
    type Target = str;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl AsRef<str> for ProjectId {
    fn as_ref(&self) -> &str {
        &self.0
    }
}

impl From<String> for ProjectId {
    fn from(value: String) -> Self {
        Self(value)
    }
}

impl From<ProjectId> for String {
    fn from(value: ProjectId) -> Self {
        value.0
    }
}

/// Stable identifier for a document.
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize, ToSchema)]
#[serde(transparent)]
#[schema(value_type = String)]
pub struct DocumentId(String);

impl DocumentId {
    pub fn new(value: impl Into<String>) -> Self {
        Self(value.into())
    }

    pub fn as_str(&self) -> &str {
        &self.0
    }
}

impl fmt::Display for DocumentId {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        self.0.fmt(f)
    }
}

impl Deref for DocumentId {
    type Target = str;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl AsRef<str> for DocumentId {
    fn as_ref(&self) -> &str {
        &self.0
    }
}

impl From<String> for DocumentId {
    fn from(value: String) -> Self {
        Self(value)
    }
}

impl From<DocumentId> for String {
    fn from(value: DocumentId) -> Self {
        value.0
    }
}

/// Stable identifier for a chunk.
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize, ToSchema)]
#[serde(transparent)]
#[schema(value_type = String)]
pub struct ChunkId(String);

impl ChunkId {
    pub fn new(value: impl Into<String>) -> Self {
        Self(value.into())
    }

    pub fn as_str(&self) -> &str {
        &self.0
    }
}

impl fmt::Display for ChunkId {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        self.0.fmt(f)
    }
}

impl Deref for ChunkId {
    type Target = str;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl AsRef<str> for ChunkId {
    fn as_ref(&self) -> &str {
        &self.0
    }
}

impl From<String> for ChunkId {
    fn from(value: String) -> Self {
        Self(value)
    }
}

impl From<ChunkId> for String {
    fn from(value: ChunkId) -> Self {
        value.0
    }
}

/// Maximum allowed length for config identifiers.
pub const MAX_CONFIG_ID_LEN: usize = 63;

/// Metadata attached to a document.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, ToSchema)]
pub struct DocumentMeta {
    /// Human-readable title of the document.
    pub title: Option<String>,
    /// Local path (Local Mode only).
    pub path: Option<String>,
    /// Logical source reference (Server Mode only).
    pub source_ref: Option<String>,
    /// Creation timestamp in ISO-8601 or epoch string form.
    pub created_at: Option<String>,
    /// Update timestamp in ISO-8601 or epoch string form.
    pub updated_at: Option<String>,
    /// Optional quality score for the document.
    pub quality_score: Option<f32>,
    /// Whether the document has been verified.
    pub verified: Option<bool>,
}

/// A raw document stored in the system.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, ToSchema)]
pub struct Document {
    /// Document identifier.
    #[schema(value_type = String)]
    pub id: DocumentId,
    /// Owning project identifier.
    #[schema(value_type = String)]
    pub project_id: ProjectId,
    /// Full document content.
    pub content: String,
    /// Optional metadata.
    pub metadata: DocumentMeta,
}

/// A span inside a document, expressed as byte offsets.
#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq, ToSchema)]
pub struct TextSpan {
    /// Inclusive start byte offset.
    pub start: usize,
    /// Exclusive end byte offset.
    pub end: usize,
}

/// A chunk extracted from a document.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, ToSchema)]
pub struct Chunk {
    /// Chunk identifier.
    #[schema(value_type = String)]
    pub id: ChunkId,
    /// Source document identifier.
    #[schema(value_type = String)]
    pub doc_id: DocumentId,
    /// Chunk content.
    pub content: String,
    /// Optional embedding vector.
    pub embedding: Option<Vec<f32>>,
    /// Location of the chunk in the source document.
    pub span: TextSpan,
    /// Optional quality score for the chunk.
    pub quality_score: Option<f32>,
    /// Whether the chunk has been verified.
    pub verified: Option<bool>,
}

/// Retrieval mode for query execution.
#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq, ToSchema)]
#[serde(rename_all = "snake_case")]
pub enum RetrievalMode {
    /// Vector similarity search.
    Vector,
    /// Full-text keyword search.
    Fts,
    /// Hybrid vector + keyword search.
    Hybrid,
}

/// Filter operator used in search conditions.
#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq, ToSchema)]
#[serde(rename_all = "snake_case")]
pub enum FilterOp {
    /// Equal comparison.
    Eq,
    /// Not-equal comparison.
    Neq,
    /// Greater-than comparison.
    Gt,
    /// Greater-than-or-equal comparison.
    Gte,
    /// Less-than comparison.
    Lt,
    /// Less-than-or-equal comparison.
    Lte,
    /// Substring containment.
    Contains,
    /// Membership in a list value.
    In,
}

/// Typed filter values used in search conditions.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, ToSchema)]
#[serde(untagged)]
pub enum FilterValue {
    /// String value filter.
    String(String),
    /// Integer value filter.
    Int(i64),
    /// Floating-point value filter.
    Float(f64),
    /// Boolean value filter.
    Bool(bool),
    /// List of scalar filter values.
    List(Vec<FilterValueScalar>),
}

/// Scalar values allowed inside list filters.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, ToSchema)]
#[serde(untagged)]
pub enum FilterValueScalar {
    /// String value filter.
    String(String),
    /// Integer value filter.
    Int(i64),
    /// Floating-point value filter.
    Float(f64),
    /// Boolean value filter.
    Bool(bool),
}

/// A field-level filter constraint.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, ToSchema)]
pub struct Filter {
    /// Field name to filter on.
    pub field: String,
    /// Comparison operator.
    pub op: FilterOp,
    /// Typed filter value.
    pub value: FilterValue,
}

/// Search intent payload accepted over the wire.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, ToSchema)]
pub struct SearchIntentInput {
    /// Optional user query text for keyword or hybrid search.
    pub query_text: Option<String>,
    /// Optional query embedding for vector search.
    pub query_embedding: Option<Vec<f32>>,
    /// Retrieval mode to apply.
    pub retrieval_mode: RetrievalMode,
    /// Optional indexing configuration selection (defaults to the project's default config).
    pub indexing_config_id: Option<String>,
    /// Number of candidates to return.
    pub top_k: u32,
    /// Hybrid weight for vector vs. keyword scoring.
    pub hybrid_alpha: f32,
    /// Optional filter list.
    pub filters: Vec<Filter>,
    /// Optional reranker configuration.
    pub reranker: Option<RerankerConfig>,
}

/// Validated search query variants.
#[derive(Debug, Clone, PartialEq)]
pub enum SearchQuery {
    /// Vector similarity search with an embedding.
    Vector { embedding: Vec<f32> },
    /// Full-text search with query text.
    Fts { text: String },
    /// Hybrid search with query text and embedding.
    Hybrid { text: String, embedding: Vec<f32> },
}

impl SearchQuery {
    pub fn retrieval_mode(&self) -> RetrievalMode {
        match self {
            SearchQuery::Vector { .. } => RetrievalMode::Vector,
            SearchQuery::Fts { .. } => RetrievalMode::Fts,
            SearchQuery::Hybrid { .. } => RetrievalMode::Hybrid,
        }
    }

    pub fn query_text(&self) -> Option<&str> {
        match self {
            SearchQuery::Vector { .. } => None,
            SearchQuery::Fts { text } | SearchQuery::Hybrid { text, .. } => Some(text.as_str()),
        }
    }

    pub fn query_embedding(&self) -> Option<&[f32]> {
        match self {
            SearchQuery::Vector { embedding } | SearchQuery::Hybrid { embedding, .. } => {
                Some(embedding.as_slice())
            }
            SearchQuery::Fts { .. } => None,
        }
    }

    fn into_parts(self) -> (Option<String>, Option<Vec<f32>>) {
        match self {
            SearchQuery::Vector { embedding } => (None, Some(embedding)),
            SearchQuery::Fts { text } => (Some(text), None),
            SearchQuery::Hybrid { text, embedding } => (Some(text), Some(embedding)),
        }
    }
}

/// Validated search intent for internal use.
#[derive(Debug, Clone, PartialEq)]
pub struct SearchIntent {
    query: SearchQuery,
    indexing_config_id: Option<String>,
    top_k: NonZeroU32,
    hybrid_alpha: f32,
    filters: Vec<Filter>,
    reranker: Option<RerankerConfig>,
}

impl SearchIntent {
    pub fn new(
        query: SearchQuery,
        indexing_config_id: Option<String>,
        top_k: NonZeroU32,
        hybrid_alpha: f32,
        filters: Vec<Filter>,
        reranker: Option<RerankerConfig>,
    ) -> CocoResult<Self> {
        if !(0.0..=1.0).contains(&hybrid_alpha) {
            return Err(validation_error("hybrid_alpha must be between 0 and 1"));
        }
        Ok(Self {
            query,
            indexing_config_id,
            top_k,
            hybrid_alpha,
            filters,
            reranker,
        })
    }

    pub fn query(&self) -> &SearchQuery {
        &self.query
    }

    pub fn query_text(&self) -> Option<&str> {
        self.query.query_text()
    }

    pub fn query_embedding(&self) -> Option<&[f32]> {
        self.query.query_embedding()
    }

    pub fn retrieval_mode(&self) -> RetrievalMode {
        self.query.retrieval_mode()
    }

    pub fn indexing_config_id(&self) -> Option<&str> {
        self.indexing_config_id.as_deref()
    }

    pub fn top_k(&self) -> NonZeroU32 {
        self.top_k
    }

    pub fn hybrid_alpha(&self) -> f32 {
        self.hybrid_alpha
    }

    pub fn filters(&self) -> &[Filter] {
        &self.filters
    }

    pub fn reranker(&self) -> Option<&RerankerConfig> {
        self.reranker.as_ref()
    }

    pub fn validate_context(&self, context: &ValidationContext) -> CocoResult<()> {
        if let Some(expected) = context.embedding_dimensions {
            if let Some(embedding) = self.query_embedding() {
                if embedding.len() != expected {
                    return Err(validation_error("query_embedding has wrong dimension"));
                }
            }
        }
        if let Some(config_id) = self.indexing_config_id() {
            ensure_normalized_config_id(config_id)?;
        } else if let Some(active_config_id) = context.active_config_id.as_deref() {
            ensure_normalized_config_id(active_config_id)?;
        }
        if let Some(allowed) = context.allowed_filter_fields.as_ref() {
            for filter in &self.filters {
                if !allowed.iter().any(|field| field == &filter.field) {
                    return Err(validation_error("filter field not allowed"));
                }
            }
        }
        if let Some(allowed) = context.allowed_filter_ops.as_ref() {
            for filter in &self.filters {
                if !allowed.contains(&filter.op) {
                    return Err(validation_error("filter operator not allowed"));
                }
            }
        }
        Ok(())
    }
}

impl TryFrom<SearchIntentInput> for SearchIntent {
    type Error = CocoError;

    fn try_from(input: SearchIntentInput) -> CocoResult<Self> {
        let SearchIntentInput {
            query_text,
            query_embedding,
            retrieval_mode,
            indexing_config_id,
            top_k,
            hybrid_alpha,
            filters,
            reranker,
        } = input;
        if top_k == 0 {
            return Err(validation_error("top_k must be greater than zero"));
        }
        if !(0.0..=1.0).contains(&hybrid_alpha) {
            return Err(validation_error("hybrid_alpha must be between 0 and 1"));
        }
        let query = match retrieval_mode {
            RetrievalMode::Vector => {
                let embedding = query_embedding.ok_or_else(|| {
                    validation_error("query_embedding required for vector search")
                })?;
                SearchQuery::Vector { embedding }
            }
            RetrievalMode::Fts => {
                let text = query_text
                    .filter(|value| !value.is_empty())
                    .ok_or_else(|| validation_error("query_text required for fts search"))?;
                SearchQuery::Fts { text }
            }
            RetrievalMode::Hybrid => {
                let text = query_text
                    .filter(|value| !value.is_empty())
                    .ok_or_else(|| validation_error("query_text required for hybrid search"))?;
                let embedding = query_embedding.ok_or_else(|| {
                    validation_error("query_embedding required for hybrid search")
                })?;
                SearchQuery::Hybrid { text, embedding }
            }
        };
        let top_k = NonZeroU32::new(top_k)
            .ok_or_else(|| validation_error("top_k must be greater than zero"))?;
        SearchIntent::new(
            query,
            indexing_config_id,
            top_k,
            hybrid_alpha,
            filters,
            reranker,
        )
    }
}

impl From<SearchIntent> for SearchIntentInput {
    fn from(intent: SearchIntent) -> Self {
        let SearchIntent {
            query,
            indexing_config_id,
            top_k,
            hybrid_alpha,
            filters,
            reranker,
        } = intent;
        let retrieval_mode = query.retrieval_mode();
        let (query_text, query_embedding) = query.into_parts();
        SearchIntentInput {
            query_text,
            query_embedding,
            retrieval_mode,
            indexing_config_id,
            top_k: top_k.get(),
            hybrid_alpha,
            filters,
            reranker,
        }
    }
}

/// Indexing plan schema version.
pub const INDEXING_PLAN_VERSION: u32 = 1;

/// Default steps executed for indexing.
pub const INDEXING_PLAN_DEFAULT_STEPS: [&str; 4] = ["parse", "chunk", "embed", "store"];

/// Query plan schema version.
pub const QUERY_PLAN_VERSION: u32 = 1;

/// Default steps executed for queries.
pub const QUERY_PLAN_DEFAULT_STEPS: [&str; 1] = ["retrieve"];

/// Execution plan for indexing jobs.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, ToSchema)]
pub struct IndexingPlan {
    /// Plan schema version.
    pub version: u32,
    /// Ordered execution steps (string identifiers).
    pub steps: Vec<String>,
}

impl Default for IndexingPlan {
    fn default() -> Self {
        Self {
            version: INDEXING_PLAN_VERSION,
            steps: Vec::from(INDEXING_PLAN_DEFAULT_STEPS.map(str::to_string)),
        }
    }
}

/// Execution plan for queries.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, ToSchema)]
pub struct QueryPlan {
    /// Plan schema version.
    pub version: u32,
    /// Ordered execution steps (string identifiers).
    pub steps: Vec<String>,
}

impl Default for QueryPlan {
    fn default() -> Self {
        Self {
            version: QUERY_PLAN_VERSION,
            steps: Vec::from(QUERY_PLAN_DEFAULT_STEPS.map(str::to_string)),
        }
    }
}

/// Indexing-time configuration for a project.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, ToSchema)]
pub struct IndexingConfig {
    /// Stable identifier of this indexing strategy.
    /// Note: a config_id must map to a compatible embedding dimension/metric.
    pub config_id: String,
    /// Chunking strategy definition.
    pub chunking: ChunkingStrategy,
    /// Embedding model definition.
    pub embedding: EmbeddingConfig,
    /// Vector similarity metric for the index.
    pub vector_metric: VectorMetric,
    /// Optional index parameter overrides per backend.
    pub index_params: Option<VectorIndexParams>,
    /// Optional vector backend selection.
    pub vector_backend: Option<VectorBackendConfig>,
}

/// Chunking strategy parameters.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, ToSchema)]
pub struct ChunkingStrategy {
    /// Strategy name (e.g., markdown_header, fixed_token).
    pub strategy_name: String,
    /// Target chunk size in tokens or characters.
    pub chunk_size: u32,
    /// Overlap size between adjacent chunks.
    pub chunk_overlap: u32,
}

/// Embedding model configuration.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, ToSchema)]
pub struct EmbeddingConfig {
    /// Model name used to generate embeddings.
    pub model_name: String,
    /// Embedding dimensions if known.
    pub dimensions: Option<u32>,
}

/// Supported vector similarity metrics.
#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq, ToSchema)]
#[serde(rename_all = "snake_case")]
pub enum VectorMetric {
    /// Cosine similarity.
    Cosine,
    /// Dot product similarity.
    Dot,
    /// Euclidean (L2) distance.
    L2,
}

/// HNSW index parameters.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, ToSchema, Default)]
pub struct HnswParams {
    /// Number of connections per node.
    pub m: Option<u32>,
    /// Construction ef parameter.
    pub ef_construction: Option<u32>,
}

/// IVF-PQ index parameters.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, ToSchema, Default)]
pub struct IvfPqParams {
    /// Number of IVF partitions.
    pub num_partitions: Option<u32>,
    /// Number of PQ sub-vectors.
    pub num_sub_vectors: Option<u32>,
    /// Sample rate for IVF training.
    pub sample_rate: Option<u32>,
    /// Max iterations for IVF training.
    pub max_iterations: Option<u32>,
}

/// Backend-specific index parameter overrides.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, ToSchema, Default)]
pub struct VectorIndexParams {
    /// Optional HNSW parameters.
    pub hnsw: Option<HnswParams>,
    /// Optional IVF-PQ parameters.
    pub ivf_pq: Option<IvfPqParams>,
}

/// Query-time retrieval configuration.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, ToSchema)]
pub struct RetrievalConfig {
    /// Retrieval mode to apply.
    pub retrieval_mode: RetrievalMode,
    /// Number of candidates to return.
    pub top_k: u32,
    /// Hybrid weight for vector vs. keyword scoring.
    pub hybrid_alpha: f32,
    /// Optional reranker configuration.
    pub reranker: Option<RerankerConfig>,
    /// Optional vector backend selection.
    pub vector_backend: Option<VectorBackendConfig>,
}

/// Supported vector backend types.
#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq, ToSchema)]
#[serde(rename_all = "snake_case")]
pub enum VectorBackendKind {
    /// Postgres pgvector backend.
    PgVector,
    /// Qdrant vector database backend.
    Qdrant,
}

/// Configuration for selecting a vector backend.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, ToSchema)]
pub struct VectorBackendConfig {
    /// Backend kind identifier.
    pub kind: VectorBackendKind,
    /// Optional endpoint or connection URL.
    pub url: Option<String>,
    /// Optional API key or token.
    pub api_key: Option<String>,
    /// Optional collection name prefix.
    pub collection_prefix: Option<String>,
}

/// Vector metadata stored alongside embeddings.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, ToSchema)]
pub struct VectorMetadata {
    /// Optional indexing configuration identifier used for storage partitioning/filtering.
    pub config_id: Option<String>,
    /// Source document identifier.
    #[schema(value_type = String)]
    pub doc_id: DocumentId,
    /// Chunk content.
    pub content: String,
    /// Location of the chunk in the source document.
    pub span: TextSpan,
}

/// Vector record persisted in vector stores.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, ToSchema)]
pub struct VectorRecord {
    /// Chunk identifier.
    #[schema(value_type = String)]
    pub chunk_id: ChunkId,
    /// Embedding vector.
    pub embedding: Vec<f32>,
    /// Metadata associated with the vector.
    pub metadata: VectorMetadata,
}

/// Response freshness indicator.
#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq, ToSchema)]
#[serde(rename_all = "snake_case")]
pub enum ResponseStatus {
    /// The response is computed from up-to-date data.
    Fresh,
    /// The response is computed from cached or stale data.
    Stale,
}

/// Response metadata for request-level status.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, ToSchema)]
pub struct ResponseMeta {
    /// Request freshness status.
    pub status: ResponseStatus,
}

/// Envelope for consistent response payloads.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct ResponseEnvelope<T> {
    /// Metadata describing the response.
    pub meta: ResponseMeta,
    /// Wrapped response data.
    pub data: T,
}

/// Metadata associated with a search hit.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, ToSchema)]
pub struct SearchHitMeta {
    /// Similarity or relevance score.
    pub score: f32,
    /// Optional quality indicator.
    pub quality: Option<f32>,
    /// Whether the result has been verified.
    pub verified: Option<bool>,
}

/// A search hit with metadata and chunk payload.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, ToSchema)]
pub struct SearchHit {
    /// Search scoring metadata.
    pub meta: SearchHitMeta,
    /// Retrieved chunk payload.
    pub chunk: Chunk,
}

/// Reranker configuration for post-retrieval scoring.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, ToSchema)]
pub struct RerankerConfig {
    /// Model name for reranking.
    pub model_name: String,
    /// Number of top candidates to rerank.
    pub rerank_top_n: u32,
}

/// High-level error categories for CoCo.
///
/// Adding new variants is a breaking change for the public API.
#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq, ToSchema)]
#[serde(rename_all = "snake_case")]
pub enum CocoErrorKind {
    /// System-level failures such as IO or configuration.
    System,
    /// User input or validation errors.
    User,
    /// Network or transport failures.
    Network,
    /// Storage layer failures.
    Storage,
    /// Compute or model execution failures.
    Compute,
}

/// Canonical error type for CoCo.
#[derive(Debug, thiserror::Error, Clone)]
pub enum CocoError {
    /// System-level failures such as IO or configuration.
    #[error("system error: {message}")]
    System {
        /// Human-readable error message.
        message: String,
    },
    /// User input or validation errors.
    #[error("user error: {message}")]
    User {
        /// Human-readable error message.
        message: String,
    },
    /// Network or transport failures.
    #[error("network error: {message}")]
    Network {
        /// Human-readable error message.
        message: String,
    },
    /// Storage layer failures.
    #[error("storage error: {message}")]
    Storage {
        /// Human-readable error message.
        message: String,
    },
    /// Compute or model execution failures.
    #[error("compute error: {message}")]
    Compute {
        /// Human-readable error message.
        message: String,
    },
}

impl CocoError {
    /// Creates a system error from a displayable source.
    pub fn system<E: std::fmt::Display>(error: E) -> Self {
        CocoError::System {
            message: error.to_string(),
        }
    }

    /// Creates a user error from a displayable source.
    pub fn user<E: std::fmt::Display>(error: E) -> Self {
        CocoError::User {
            message: error.to_string(),
        }
    }

    /// Creates a network error from a displayable source.
    pub fn network<E: std::fmt::Display>(error: E) -> Self {
        CocoError::Network {
            message: error.to_string(),
        }
    }

    /// Creates a storage error from a displayable source.
    pub fn storage<E: std::fmt::Display>(error: E) -> Self {
        CocoError::Storage {
            message: error.to_string(),
        }
    }

    /// Creates a compute error from a displayable source.
    pub fn compute<E: std::fmt::Display>(error: E) -> Self {
        CocoError::Compute {
            message: error.to_string(),
        }
    }

    /// Returns the coarse error kind for classification.
    pub fn kind(&self) -> CocoErrorKind {
        match self {
            CocoError::System { .. } => CocoErrorKind::System,
            CocoError::User { .. } => CocoErrorKind::User,
            CocoError::Network { .. } => CocoErrorKind::Network,
            CocoError::Storage { .. } => CocoErrorKind::Storage,
            CocoError::Compute { .. } => CocoErrorKind::Compute,
        }
    }

    /// Maps the error to an HTTP status code.
    pub fn http_status(&self) -> u16 {
        match self {
            CocoError::User { .. } => 400,
            CocoError::Network { .. } => 502,
            CocoError::Storage { .. } => 503,
            CocoError::System { .. } | CocoError::Compute { .. } => 500,
        }
    }

    /// Returns a stable, public-facing error message.
    pub fn public_message(&self) -> &str {
        match self {
            CocoError::User { message } => message.as_str(),
            _ => "internal error",
        }
    }

    /// Consumes the error and returns a public-facing error message.
    pub fn into_public_message(self) -> String {
        match self {
            CocoError::User { message } => message,
            _ => "internal error".to_string(),
        }
    }
}

impl From<std::io::Error> for CocoError {
    fn from(error: std::io::Error) -> Self {
        CocoError::system(error)
    }
}

/// Convenience result type for CoCo operations.
pub type CocoResult<T> = Result<T, CocoError>;

/// Serializable error response used by API layers.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, ToSchema)]
pub struct ErrorResponse {
    /// Error classification.
    pub kind: CocoErrorKind,
    /// Human-readable error message.
    pub message: String,
}

impl From<CocoError> for ErrorResponse {
    fn from(error: CocoError) -> Self {
        ErrorResponse {
            kind: error.kind(),
            message: error.into_public_message(),
        }
    }
}

/// Context for validating protocol inputs.
#[derive(Debug, Clone, Default)]
pub struct ValidationContext {
    /// Expected embedding dimensions, if known.
    pub embedding_dimensions: Option<usize>,
    /// Expected vector backend kind for validation.
    pub expected_vector_backend: Option<VectorBackendKind>,
    /// Allowed filter fields, if validation should enforce a whitelist.
    pub allowed_filter_fields: Option<Vec<String>>,
    /// Allowed filter operators, if validation should enforce a whitelist.
    pub allowed_filter_ops: Option<Vec<FilterOp>>,
    /// Active config identifier, if validation should enforce selection.
    pub active_config_id: Option<String>,
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
    intent.validate_context(context)
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
    validate_plan_steps(
        "indexing plan",
        &plan.steps,
        &INDEXING_PLAN_DEFAULT_STEPS,
    )
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
        return Err(validation_error(&format!("{label} steps must not be empty")));
    }
    let mut seen = HashSet::new();
    for step in steps {
        if step.trim().is_empty() {
            return Err(validation_error(&format!(
                "{label} step must not be empty"
            )));
        }
        if step.trim() != step {
            return Err(validation_error(&format!(
                "{label} step must be normalized"
            )));
        }
        if !seen.insert(step.as_str()) {
            return Err(validation_error(&format!(
                "{label} steps must be unique"
            )));
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
        return Err(validation_error("index_params must specify a single index kind"));
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
    if let Some(m) = params.m {
        if m == 0 {
            return Err(validation_error("hnsw.m must be greater than zero"));
        }
    }
    if let Some(ef_construction) = params.ef_construction {
        if ef_construction == 0 {
            return Err(validation_error(
                "hnsw.ef_construction must be greater than zero",
            ));
        }
    }
    Ok(())
}

fn validate_ivf_pq(params: &IvfPqParams) -> CocoResult<()> {
    if let Some(value) = params.num_partitions {
        if value == 0 {
            return Err(validation_error(
                "ivf_pq.num_partitions must be greater than zero",
            ));
        }
    }
    if let Some(value) = params.num_sub_vectors {
        if value == 0 {
            return Err(validation_error(
                "ivf_pq.num_sub_vectors must be greater than zero",
            ));
        }
    }
    if let Some(value) = params.sample_rate {
        if value == 0 {
            return Err(validation_error(
                "ivf_pq.sample_rate must be greater than zero",
            ));
        }
    }
    if let Some(value) = params.max_iterations {
        if value == 0 {
            return Err(validation_error(
                "ivf_pq.max_iterations must be greater than zero",
            ));
        }
    }
    Ok(())
}

fn validation_error(message: &str) -> CocoError {
    CocoError::user(format!("validation error: {message}"))
}

/// Supported file types for parsing.
#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq, ToSchema)]
#[serde(rename_all = "snake_case")]
pub enum FileType {
    /// Markdown documents.
    Markdown,
    /// Rust source files.
    Rust,
    /// Python source files.
    Python,
    /// TypeScript source files.
    TypeScript,
    /// Go source files.
    Go,
    /// Plain text files.
    PlainText,
    /// Fallback for unknown types.
    Other,
}

/// Parsed document output for downstream processing.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, ToSchema)]
pub struct ParsedDocument {
    /// Original content after parsing.
    pub content: String,
    /// File type used during parsing.
    pub file_type: FileType,
}

/// Storage backend contract for vector and keyword retrieval.
pub trait StorageBackend {
    /// Upserts a batch of chunks into storage.
    fn upsert_chunks(&self, chunks: &[Chunk]) -> impl Future<Output = CocoResult<()>> + Send;
    /// Searches for similar chunks given an intent.
    fn search_similar(
        &self,
        intent: SearchIntent,
    ) -> impl Future<Output = CocoResult<Vec<SearchHit>>> + Send;
    /// Deletes all chunks associated with a document.
    fn delete_by_doc(&self, doc_id: DocumentId) -> impl Future<Output = CocoResult<()>> + Send;
    /// Fetches a single chunk by id.
    fn get_chunk(
        &self,
        chunk_id: ChunkId,
    ) -> impl Future<Output = CocoResult<Option<Chunk>>> + Send;
}

/// Vector store contract for dedicated vector backends.
pub trait VectorStore {
    /// Upserts vector records into storage.
    fn upsert_vectors(
        &self,
        records: &[VectorRecord],
    ) -> impl Future<Output = CocoResult<()>> + Send;
    /// Searches for similar vectors given an intent.
    fn search_vectors(
        &self,
        intent: SearchIntent,
    ) -> impl Future<Output = CocoResult<Vec<SearchHit>>> + Send;
    /// Deletes vectors associated with a document.
    fn delete_vectors_by_doc(
        &self,
        doc_id: DocumentId,
    ) -> impl Future<Output = CocoResult<()>> + Send;
    /// Fetches a single vector record by chunk id.
    fn get_vector(
        &self,
        chunk_id: ChunkId,
    ) -> impl Future<Output = CocoResult<Option<VectorRecord>>> + Send;
}

/// Embedding model contract for vector generation.
pub trait EmbeddingModel {
    /// Computes embeddings for the provided input texts.
    fn embed(&self, texts: &[&str]) -> CocoResult<Vec<Vec<f32>>>;
    /// Returns the embedding dimension.
    fn dimensions(&self) -> usize;
    /// Returns the model identifier.
    fn model_name(&self) -> &str;
}

/// Document parser contract for syntactic analysis.
pub trait DocumentParser {
    /// Parses content into a structured representation.
    fn parse(&self, content: &str, file_type: FileType) -> CocoResult<ParsedDocument>;
    /// Returns the supported file types.
    fn supported_types(&self) -> &'static [FileType];
}

/// Chunker contract for splitting parsed documents.
pub trait Chunker {
    /// Produces text spans according to the chunking strategy.
    fn chunk(&self, doc: &ParsedDocument, config: &ChunkingStrategy) -> CocoResult<Vec<TextSpan>>;
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn search_intent_roundtrip() {
        let intent = SearchIntentInput {
            query_text: Some("hello".to_string()),
            query_embedding: Some(vec![0.5, -1.0]),
            retrieval_mode: RetrievalMode::Hybrid,
            indexing_config_id: Some("default".to_string()),
            top_k: 8,
            hybrid_alpha: 0.7,
            filters: vec![Filter {
                field: "path".to_string(),
                op: FilterOp::Contains,
                value: FilterValue::String("src/".to_string()),
            }],
            reranker: Some(RerankerConfig {
                model_name: "bge".to_string(),
                rerank_top_n: 5,
            }),
        };

        let json = serde_json::to_string(&intent).expect("serialize intent");
        let decoded: SearchIntentInput = serde_json::from_str(&json).expect("deserialize intent");
        let reserialized = serde_json::to_string(&decoded).expect("serialize intent");
        assert_eq!(reserialized, json);
    }

    #[test]
    fn filter_op_serialization_is_snake_case() {
        let json = serde_json::to_string(&FilterOp::Gte).expect("serialize filter op");
        assert_eq!(json, "\"gte\"");
    }

    #[test]
    fn retrieval_mode_serialization_is_snake_case() {
        let json = serde_json::to_string(&RetrievalMode::Fts).expect("serialize retrieval mode");
        assert_eq!(json, "\"fts\"");
    }

    #[test]
    fn error_response_uses_kind_and_message() {
        let error = CocoError::user("bad input");
        let response = ErrorResponse::from(error);
        assert_eq!(response.kind, CocoErrorKind::User);
        assert_eq!(response.message, "bad input");
    }

    #[test]
    fn document_meta_roundtrip() {
        let meta = DocumentMeta {
            title: Some("Title".to_string()),
            path: Some("/tmp/file.md".to_string()),
            source_ref: None,
            created_at: Some("2024-01-01T00:00:00Z".to_string()),
            updated_at: None,
            quality_score: Some(0.8),
            verified: Some(true),
        };
        let json = serde_json::to_string(&meta).expect("serialize meta");
        let decoded: DocumentMeta = serde_json::from_str(&json).expect("deserialize meta");
        assert_eq!(decoded, meta);
    }

    #[test]
    fn response_status_serialization_is_snake_case() {
        let json = serde_json::to_string(&ResponseStatus::Fresh).expect("serialize status");
        assert_eq!(json, "\"fresh\"");
    }

    #[test]
    fn response_envelope_serializes_search_hits_with_optional_meta() {
        #[derive(Serialize)]
        struct HitPayload {
            results: Vec<SearchHit>,
        }

        let hit = SearchHit {
            meta: SearchHitMeta {
                score: 0.42,
                quality: None,
                verified: None,
            },
            chunk: Chunk {
                id: ChunkId::new("chunk-1"),
                doc_id: DocumentId::new("doc-1"),
                content: "hello".to_string(),
                embedding: None,
                span: TextSpan { start: 0, end: 5 },
                quality_score: None,
                verified: None,
            },
        };
        let envelope = ResponseEnvelope {
            meta: ResponseMeta {
                status: ResponseStatus::Fresh,
            },
            data: HitPayload { results: vec![hit] },
        };
        let value = serde_json::to_value(&envelope).expect("serialize envelope");
        let meta = &value["data"]["results"][0]["meta"];
        assert_eq!(meta["quality"], serde_json::Value::Null);
        assert_eq!(meta["verified"], serde_json::Value::Null);
        let chunk = &value["data"]["results"][0]["chunk"];
        assert_eq!(chunk["quality_score"], serde_json::Value::Null);
        assert_eq!(chunk["verified"], serde_json::Value::Null);
    }

    #[test]
    fn vector_metric_serialization_is_snake_case() {
        let json = serde_json::to_string(&VectorMetric::L2).expect("serialize metric");
        assert_eq!(json, "\"l2\"");
    }

    #[test]
    fn normalize_config_id_rejects_invalid_values() {
        assert!(normalize_config_id("Bad").is_err());
        assert!(normalize_config_id("has space").is_err());
        assert!(normalize_config_id("").is_err());
    }

    #[test]
    fn validate_search_intent_checks_embedding_dimensions() {
        let intent = SearchIntentInput {
            query_text: None,
            query_embedding: Some(vec![0.1, 0.2]),
            retrieval_mode: RetrievalMode::Vector,
            indexing_config_id: Some("default".to_string()),
            top_k: 3,
            hybrid_alpha: 0.5,
            filters: Vec::new(),
            reranker: None,
        };
        let intent = SearchIntent::try_from(intent).expect("validated intent");
        let context = ValidationContext {
            embedding_dimensions: Some(3),
            expected_vector_backend: None,
            allowed_filter_fields: None,
            allowed_filter_ops: None,
            active_config_id: None,
        };
        assert!(validate_search_intent(&intent, &context).is_err());
    }

    #[test]
    fn validate_search_intent_rejects_filter_ops_outside_allowlist() {
        let intent = SearchIntentInput {
            query_text: Some("hello".to_string()),
            query_embedding: None,
            retrieval_mode: RetrievalMode::Fts,
            indexing_config_id: None,
            top_k: 3,
            hybrid_alpha: 0.5,
            filters: vec![Filter {
                field: "doc_id".to_string(),
                op: FilterOp::Gt,
                value: FilterValue::String("doc-1".to_string()),
            }],
            reranker: None,
        };
        let intent = SearchIntent::try_from(intent).expect("validated intent");
        let context = ValidationContext {
            embedding_dimensions: None,
            expected_vector_backend: None,
            allowed_filter_fields: Some(vec!["doc_id".to_string()]),
            allowed_filter_ops: Some(vec![FilterOp::Eq, FilterOp::Contains]),
            active_config_id: None,
        };
        assert!(validate_search_intent(&intent, &context).is_err());
    }
}
