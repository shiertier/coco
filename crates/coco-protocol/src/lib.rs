//! Protocol types and error contracts shared across CoCo crates.

use std::fmt;
use std::future::Future;
use std::num::NonZeroU32;
use std::ops::Deref;

use serde::{Deserialize, Serialize};
use utoipa::ToSchema;

// Shared impls for ID newtypes.
macro_rules! impl_id_type {
    ($name:ident) => {
        impl fmt::Display for $name {
            fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
                self.0.fmt(f)
            }
        }

        impl Deref for $name {
            type Target = str;

            fn deref(&self) -> &Self::Target {
                &self.0
            }
        }

        impl AsRef<str> for $name {
            fn as_ref(&self) -> &str {
                &self.0
            }
        }

        impl From<String> for $name {
            fn from(value: String) -> Self {
                Self(value)
            }
        }

        impl From<$name> for String {
            fn from(value: $name) -> Self {
                value.0
            }
        }
    };
}

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

impl_id_type!(ProjectId);

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

impl_id_type!(DocumentId);

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

impl_id_type!(ChunkId);

/// Maximum allowed length for config identifiers.
pub const MAX_CONFIG_ID_LEN: usize = 63;

/// Metadata attached to a document.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, ToSchema)]
#[serde(tag = "origin", rename_all = "snake_case")]
pub enum DocumentMeta {
    /// Metadata for locally sourced documents.
    Local {
        /// Human-readable title of the document.
        title: Option<String>,
        /// Local path (Local Mode only).
        path: Option<String>,
        /// Creation timestamp in ISO-8601 or epoch string form.
        created_at: Option<String>,
        /// Update timestamp in ISO-8601 or epoch string form.
        updated_at: Option<String>,
        /// Optional quality score for the document.
        quality_score: Option<f32>,
        /// Whether the document has been verified.
        verified: Option<bool>,
    },
    /// Metadata for server-sourced documents.
    Server {
        /// Human-readable title of the document.
        title: Option<String>,
        /// Logical source reference (Server Mode only).
        source_ref: Option<String>,
        /// Creation timestamp in ISO-8601 or epoch string form.
        created_at: Option<String>,
        /// Update timestamp in ISO-8601 or epoch string form.
        updated_at: Option<String>,
        /// Optional quality score for the document.
        quality_score: Option<f32>,
        /// Whether the document has been verified.
        verified: Option<bool>,
    },
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
    /// Metadata for the document.
    pub metadata: DocumentMeta,
}

/// A span inside a document, expressed as byte offsets.
#[derive(Debug, Clone, Copy, Serialize, PartialEq, Eq, ToSchema)]
pub struct TextSpan {
    /// Inclusive start byte offset.
    start: usize,
    /// Exclusive end byte offset.
    end: usize,
}

impl TextSpan {
    pub fn new(start: usize, end: usize) -> CocoResult<Self> {
        if start > end {
            return Err(validation_error("text span start must be <= end"));
        }
        Ok(Self { start, end })
    }

    pub fn start(&self) -> usize {
        self.start
    }

    pub fn end(&self) -> usize {
        self.end
    }

    pub fn len(&self) -> usize {
        self.end - self.start
    }

    pub fn is_empty(&self) -> bool {
        self.start == self.end
    }
}

impl<'de> Deserialize<'de> for TextSpan {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        #[derive(Deserialize)]
        struct TextSpanDef {
            start: usize,
            end: usize,
        }

        let def = TextSpanDef::deserialize(deserializer)?;
        TextSpan::new(def.start, def.end).map_err(serde::de::Error::custom)
    }
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

/// Validated filter field identifier.
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, ToSchema)]
#[serde(transparent)]
#[schema(value_type = String)]
pub struct FilterField(String);

impl FilterField {
    pub fn new(value: impl Into<String>) -> CocoResult<Self> {
        let value = value.into();
        let trimmed = value.trim();
        if trimmed.is_empty() {
            return Err(validation_error("filter field must not be empty"));
        }
        if trimmed != value.as_str() {
            return Err(validation_error("filter field must be normalized"));
        }
        Ok(Self(value))
    }

    pub fn as_str(&self) -> &str {
        &self.0
    }
}

impl fmt::Display for FilterField {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        self.0.fmt(f)
    }
}

impl Deref for FilterField {
    type Target = str;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl AsRef<str> for FilterField {
    fn as_ref(&self) -> &str {
        &self.0
    }
}

impl TryFrom<String> for FilterField {
    type Error = CocoError;

    fn try_from(value: String) -> CocoResult<Self> {
        FilterField::new(value)
    }
}

impl From<FilterField> for String {
    fn from(value: FilterField) -> Self {
        value.0
    }
}

impl<'de> Deserialize<'de> for FilterField {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        let value = String::deserialize(deserializer)?;
        FilterField::new(value).map_err(serde::de::Error::custom)
    }
}

/// A field-level filter constraint.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, ToSchema)]
pub struct Filter {
    /// Field name to filter on.
    pub field: FilterField,
    /// Comparison operator.
    pub op: FilterOp,
    /// Typed filter value.
    pub value: FilterValue,
}

/// Hybrid weight for vector vs. keyword scoring.
#[derive(Debug, Clone, Copy, Serialize, PartialEq, ToSchema)]
#[serde(transparent)]
#[schema(value_type = f32)]
pub struct HybridAlpha(f32);

impl HybridAlpha {
    pub fn new(value: f32) -> CocoResult<Self> {
        if !(0.0..=1.0).contains(&value) {
            return Err(validation_error("hybrid_alpha must be between 0 and 1"));
        }
        Ok(Self(value))
    }

    pub fn get(self) -> f32 {
        self.0
    }
}

impl TryFrom<f32> for HybridAlpha {
    type Error = CocoError;

    fn try_from(value: f32) -> CocoResult<Self> {
        HybridAlpha::new(value)
    }
}

impl From<HybridAlpha> for f32 {
    fn from(value: HybridAlpha) -> Self {
        value.0
    }
}

impl<'de> Deserialize<'de> for HybridAlpha {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        let value = f32::deserialize(deserializer)?;
        HybridAlpha::new(value).map_err(serde::de::Error::custom)
    }
}

/// Query payload accepted over the wire.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, ToSchema)]
#[serde(tag = "retrieval_mode", rename_all = "snake_case")]
pub enum SearchQueryInput {
    /// Vector similarity search with embedding or text input.
    Vector(VectorQueryInput),
    /// Full-text search with query text.
    Fts(FtsQueryInput),
    /// Hybrid search with query text and optional embedding.
    Hybrid(HybridQueryInput),
}

impl SearchQueryInput {
    pub fn vector(query_text: Option<String>, query_embedding: Option<Vec<f32>>) -> CocoResult<Self> {
        Ok(Self::Vector(VectorQueryInput::new(query_text, query_embedding)?))
    }

    pub fn fts(query_text: impl Into<String>) -> CocoResult<Self> {
        Ok(Self::Fts(FtsQueryInput::new(query_text.into())?))
    }

    pub fn hybrid(query_text: impl Into<String>, query_embedding: Option<Vec<f32>>) -> CocoResult<Self> {
        Ok(Self::Hybrid(HybridQueryInput::new(query_text.into(), query_embedding)?))
    }

    pub fn retrieval_mode(&self) -> RetrievalMode {
        match self {
            SearchQueryInput::Vector(_) => RetrievalMode::Vector,
            SearchQueryInput::Fts(_) => RetrievalMode::Fts,
            SearchQueryInput::Hybrid(_) => RetrievalMode::Hybrid,
        }
    }

    pub fn query_text(&self) -> Option<&str> {
        match self {
            SearchQueryInput::Vector(query) => query.query_text(),
            SearchQueryInput::Fts(query) => query.query_text(),
            SearchQueryInput::Hybrid(query) => query.query_text(),
        }
    }

    pub fn query_embedding(&self) -> Option<&[f32]> {
        match self {
            SearchQueryInput::Vector(query) => query.query_embedding(),
            SearchQueryInput::Fts(_) => None,
            SearchQueryInput::Hybrid(query) => query.query_embedding(),
        }
    }

    pub fn set_query_embedding(&mut self, embedding: Vec<f32>) -> CocoResult<()> {
        match self {
            SearchQueryInput::Vector(query) => query.set_query_embedding(embedding),
            SearchQueryInput::Hybrid(query) => query.set_query_embedding(embedding),
            SearchQueryInput::Fts(_) => Err(validation_error(
                "query_embedding is not supported for fts search",
            )),
        }
    }

    pub fn into_retrieval_mode(self, mode: RetrievalMode) -> CocoResult<Self> {
        let (query_text, query_embedding) = match self {
            SearchQueryInput::Vector(query) => (query.query_text, query.query_embedding),
            SearchQueryInput::Fts(query) => (query.query_text, None),
            SearchQueryInput::Hybrid(query) => (query.query_text, query.query_embedding),
        };
        match mode {
            RetrievalMode::Vector => SearchQueryInput::vector(query_text, query_embedding),
            RetrievalMode::Fts => {
                let query_text = query_text.ok_or_else(|| {
                    validation_error("query_text required for fts search")
                })?;
                SearchQueryInput::fts(query_text)
            }
            RetrievalMode::Hybrid => {
                let query_text = query_text.ok_or_else(|| {
                    validation_error("query_text required for hybrid search")
                })?;
                SearchQueryInput::hybrid(query_text, query_embedding)
            }
        }
    }
}

/// Vector query input, providing text or embedding.
#[derive(Debug, Clone, Serialize, PartialEq, ToSchema)]
pub struct VectorQueryInput {
    /// Optional text used to build embeddings.
    #[serde(skip_serializing_if = "Option::is_none")]
    query_text: Option<String>,
    /// Optional query embedding for vector search.
    #[serde(skip_serializing_if = "Option::is_none")]
    query_embedding: Option<Vec<f32>>,
}

impl VectorQueryInput {
    pub fn new(query_text: Option<String>, query_embedding: Option<Vec<f32>>) -> CocoResult<Self> {
        let query_text = query_text
            .map(normalize_query_text)
            .transpose()?;
        if let Some(embedding) = query_embedding.as_ref() {
            ensure_non_empty_embedding(embedding)?;
        }
        if query_text.is_none() && query_embedding.is_none() {
            return Err(validation_error(
                "query_text or query_embedding required for vector search",
            ));
        }
        Ok(Self {
            query_text,
            query_embedding,
        })
    }

    pub fn query_text(&self) -> Option<&str> {
        self.query_text.as_deref()
    }

    pub fn query_embedding(&self) -> Option<&[f32]> {
        self.query_embedding.as_deref()
    }

    pub fn set_query_embedding(&mut self, embedding: Vec<f32>) -> CocoResult<()> {
        ensure_non_empty_embedding(&embedding)?;
        self.query_embedding = Some(embedding);
        Ok(())
    }

    pub fn into_parts(self) -> (Option<String>, Option<Vec<f32>>) {
        (self.query_text, self.query_embedding)
    }
}

impl<'de> Deserialize<'de> for VectorQueryInput {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        #[derive(Deserialize)]
        struct VectorQueryInputDef {
            query_text: Option<String>,
            query_embedding: Option<Vec<f32>>,
        }

        let def = VectorQueryInputDef::deserialize(deserializer)?;
        Ok(Self {
            query_text: def.query_text,
            query_embedding: def.query_embedding,
        })
    }
}

/// Full-text query input.
#[derive(Debug, Clone, Serialize, PartialEq, ToSchema)]
pub struct FtsQueryInput {
    /// Query text for full-text search.
    #[serde(skip_serializing_if = "Option::is_none")]
    query_text: Option<String>,
}

impl FtsQueryInput {
    pub fn new(query_text: String) -> CocoResult<Self> {
        let query_text = normalize_query_text(query_text)?;
        Ok(Self {
            query_text: Some(query_text),
        })
    }

    pub fn query_text(&self) -> Option<&str> {
        self.query_text.as_deref()
    }

    pub fn into_text(self) -> Option<String> {
        self.query_text
    }
}

impl<'de> Deserialize<'de> for FtsQueryInput {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        #[derive(Deserialize)]
        struct FtsQueryInputDef {
            query_text: Option<String>,
        }

        let def = FtsQueryInputDef::deserialize(deserializer)?;
        Ok(Self {
            query_text: def.query_text,
        })
    }
}

/// Hybrid query input.
#[derive(Debug, Clone, Serialize, PartialEq, ToSchema)]
pub struct HybridQueryInput {
    /// Query text for hybrid search.
    #[serde(skip_serializing_if = "Option::is_none")]
    query_text: Option<String>,
    /// Optional query embedding for hybrid search.
    #[serde(skip_serializing_if = "Option::is_none")]
    query_embedding: Option<Vec<f32>>,
}

impl HybridQueryInput {
    pub fn new(query_text: String, query_embedding: Option<Vec<f32>>) -> CocoResult<Self> {
        let query_text = normalize_query_text(query_text)?;
        if let Some(embedding) = query_embedding.as_ref() {
            ensure_non_empty_embedding(embedding)?;
        }
        Ok(Self {
            query_text: Some(query_text),
            query_embedding,
        })
    }

    pub fn query_text(&self) -> Option<&str> {
        self.query_text.as_deref()
    }

    pub fn query_embedding(&self) -> Option<&[f32]> {
        self.query_embedding.as_deref()
    }

    pub fn set_query_embedding(&mut self, embedding: Vec<f32>) -> CocoResult<()> {
        ensure_non_empty_embedding(&embedding)?;
        self.query_embedding = Some(embedding);
        Ok(())
    }

    pub fn into_parts(self) -> (Option<String>, Option<Vec<f32>>) {
        (self.query_text, self.query_embedding)
    }
}

impl<'de> Deserialize<'de> for HybridQueryInput {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        #[derive(Deserialize)]
        struct HybridQueryInputDef {
            query_text: Option<String>,
            query_embedding: Option<Vec<f32>>,
        }

        let def = HybridQueryInputDef::deserialize(deserializer)?;
        Ok(Self {
            query_text: def.query_text,
            query_embedding: def.query_embedding,
        })
    }
}

/// Search intent payload accepted over the wire.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, ToSchema)]
pub struct SearchIntentInput {
    /// Query payload for the request.
    #[serde(flatten)]
    pub query: SearchQueryInput,
    /// Optional indexing configuration selection (defaults to the project's default config).
    pub indexing_config_id: Option<String>,
    /// Number of candidates to return.
    #[schema(value_type = u32)]
    pub top_k: NonZeroU32,
    /// Hybrid weight for vector vs. keyword scoring.
    pub hybrid_alpha: HybridAlpha,
    /// Optional filter list.
    #[serde(default)]
    pub filters: Vec<Filter>,
    /// Optional reranker configuration.
    #[serde(default)]
    pub reranker: Option<RerankerConfig>,
}

impl SearchIntentInput {
    pub fn new(
        query: SearchQueryInput,
        indexing_config_id: Option<String>,
        top_k: u32,
        hybrid_alpha: f32,
        filters: Vec<Filter>,
        reranker: Option<RerankerConfig>,
    ) -> CocoResult<Self> {
        let top_k = NonZeroU32::new(top_k)
            .ok_or_else(|| validation_error("top_k must be greater than zero"))?;
        let hybrid_alpha = HybridAlpha::new(hybrid_alpha)?;
        Ok(Self {
            query,
            indexing_config_id,
            top_k,
            hybrid_alpha,
            filters,
            reranker,
        })
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

    pub fn set_query_embedding(&mut self, embedding: Vec<f32>) -> CocoResult<()> {
        self.query.set_query_embedding(embedding)
    }

    pub fn set_retrieval_mode(&mut self, mode: RetrievalMode) -> CocoResult<()> {
        self.query = self.query.clone().into_retrieval_mode(mode)?;
        Ok(())
    }

    pub fn set_top_k(&mut self, top_k: u32) -> CocoResult<()> {
        self.top_k = NonZeroU32::new(top_k)
            .ok_or_else(|| validation_error("top_k must be greater than zero"))?;
        Ok(())
    }

    pub fn set_hybrid_alpha(&mut self, hybrid_alpha: f32) -> CocoResult<()> {
        self.hybrid_alpha = HybridAlpha::new(hybrid_alpha)?;
        Ok(())
    }
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

    pub fn as_parts(&self) -> (Option<&str>, Option<&[f32]>) {
        match self {
            SearchQuery::Vector { embedding } => (None, Some(embedding.as_slice())),
            SearchQuery::Fts { text } => (Some(text.as_str()), None),
            SearchQuery::Hybrid { text, embedding } => (Some(text.as_str()), Some(embedding.as_slice())),
        }
    }
}

/// Search intent for internal use.
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
    /// Constructs a search intent without validation.
    pub fn new_unchecked(
        query: SearchQuery,
        indexing_config_id: Option<String>,
        top_k: NonZeroU32,
        hybrid_alpha: f32,
        filters: Vec<Filter>,
        reranker: Option<RerankerConfig>,
    ) -> Self {
        Self {
            query,
            indexing_config_id,
            top_k,
            hybrid_alpha,
            filters,
            reranker,
        }
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
    pub allowed_filter_fields: Option<Vec<FilterField>>,
    /// Allowed filter operators, if validation should enforce a whitelist.
    pub allowed_filter_ops: Option<Vec<FilterOp>>,
    /// Active config identifier for validating implicit config selection.
    pub active_config_id: Option<String>,
}

fn normalize_query_text(value: String) -> CocoResult<String> {
    let trimmed = value.trim();
    if trimmed.is_empty() {
        return Err(validation_error("query_text must not be empty"));
    }
    if trimmed != value {
        return Err(validation_error("query_text must be normalized"));
    }
    Ok(value)
}

fn ensure_non_empty_embedding(value: &[f32]) -> CocoResult<()> {
    if value.is_empty() {
        return Err(validation_error("query_embedding must not be empty"));
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
        let intent = SearchIntentInput::new(
            SearchQueryInput::hybrid("hello", Some(vec![0.5, -1.0])).expect("query"),
            Some("default".to_string()),
            8,
            0.7,
            vec![Filter {
                field: FilterField::new("path").expect("filter field"),
                op: FilterOp::Contains,
                value: FilterValue::String("src/".to_string()),
            }],
            Some(RerankerConfig {
                model_name: "bge".to_string(),
                rerank_top_n: 5,
            }),
        )
        .expect("intent");

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
        let meta = DocumentMeta::Local {
            title: Some("Title".to_string()),
            path: Some("/tmp/file.md".to_string()),
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
                span: TextSpan::new(0, 5).expect("span"),
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
}
