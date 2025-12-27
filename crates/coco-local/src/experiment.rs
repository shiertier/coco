//! Experiment specification types for local CLI workflows.

use std::path::Path;

use coco_core::{normalize_config_id, validate_indexing_config, validate_retrieval_config};
use coco_protocol::{
    CocoError, CocoResult, IndexingConfig, RetrievalConfig, RetrievalMode, ValidationContext,
};
use serde::{Deserialize, Serialize};

/// Experiment specification loaded from YAML.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ExperimentSpec {
    /// Human-readable experiment name.
    pub name: String,
    /// Project identifier to target.
    pub project_id: String,
    /// Optional dataset revision label.
    #[serde(default)]
    pub dataset_revision: Option<String>,
    /// Optional dataset commit hash.
    #[serde(default)]
    pub dataset_commit: Option<String>,
    /// Optional random seed for reproducibility.
    #[serde(default)]
    pub random_seed: Option<u64>,
    /// Optional concurrency setting.
    #[serde(default)]
    pub concurrency: Option<u32>,
    /// Indexing strategies to evaluate.
    pub indexing_strategies: Vec<IndexingConfig>,
    /// Query strategies to evaluate.
    pub query_strategies: Vec<QueryStrategy>,
    /// Evaluation cases to run.
    pub evaluation_set: Vec<EvaluationCase>,
    /// Metrics to compute during evaluation.
    #[serde(default)]
    pub metrics: Vec<MetricSpec>,
}

impl ExperimentSpec {
    /// Loads an experiment specification from a YAML file.
    pub fn from_path(path: &Path) -> CocoResult<Self> {
        let content = std::fs::read_to_string(path).map_err(|err| {
            CocoError::system(format!("failed to read experiment file: {err}"))
        })?;
        let spec = serde_yaml::from_str::<Self>(&content).map_err(|err| {
            CocoError::user(format!("invalid experiment yaml: {err}"))
        })?;
        spec.validate()?;
        Ok(spec)
    }

    /// Validates the experiment specification.
    pub fn validate(&self) -> CocoResult<()> {
        if self.name.trim().is_empty() {
            return Err(CocoError::user("experiment name must not be empty"));
        }
        if self.project_id.trim().is_empty() {
            return Err(CocoError::user("project_id must not be empty"));
        }
        if self.indexing_strategies.is_empty() {
            return Err(CocoError::user(
                "indexing_strategies must not be empty",
            ));
        }
        if self.query_strategies.is_empty() {
            return Err(CocoError::user(
                "query_strategies must not be empty",
            ));
        }
        if self.evaluation_set.is_empty() {
            return Err(CocoError::user("evaluation_set must not be empty"));
        }

        for config in &self.indexing_strategies {
            let normalized = normalize_config_id(&config.config_id)?;
            if normalized != config.config_id {
                return Err(CocoError::user(
                    "indexing_strategies config_id must be normalized",
                ));
            }
            if config.vector_backend.is_some() {
                return Err(CocoError::user(
                    "indexing_strategies vector_backend is not supported in local mode",
                ));
            }
            validate_indexing_config(config, &ValidationContext::default())?;
        }

        for strategy in &self.query_strategies {
            let normalized = normalize_config_id(&strategy.query_config_id)?;
            if normalized != strategy.query_config_id {
                return Err(CocoError::user(
                    "query_strategies query_config_id must be normalized",
                ));
            }
            if strategy.retrieval_config.vector_backend.is_some() {
                return Err(CocoError::user(
                    "query_strategies retrieval_config.vector_backend is not supported in local mode",
                ));
            }
            validate_retrieval_config(
                &strategy.retrieval_config,
                &ValidationContext::default(),
            )?;
        }

        for case in &self.evaluation_set {
            case.validate()?;
        }

        for metric in &self.metrics {
            metric.validate()?;
        }

        if let Some(concurrency) = self.concurrency {
            if concurrency == 0 {
                return Err(CocoError::user("concurrency must be greater than zero"));
            }
        }

        Ok(())
    }
}

/// Query strategy definition used in experiments.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct QueryStrategy {
    /// Stable identifier for the query strategy.
    pub query_config_id: String,
    /// Retrieval configuration for the query.
    pub retrieval_config: RetrievalConfig,
}

/// Evaluation case definition.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EvaluationCase {
    /// Stable identifier for the evaluation case.
    pub question_id: String,
    /// Query text to execute.
    pub query: String,
    /// Expected document identifiers.
    pub expected_doc_ids: Vec<String>,
}

impl EvaluationCase {
    fn validate(&self) -> CocoResult<()> {
        if self.question_id.trim().is_empty() {
            return Err(CocoError::user("question_id must not be empty"));
        }
        if self.query.trim().is_empty() {
            return Err(CocoError::user("query must not be empty"));
        }
        if self.expected_doc_ids.is_empty() {
            return Err(CocoError::user(
                "expected_doc_ids must not be empty",
            ));
        }
        Ok(())
    }
}

/// Metric definition for experiment evaluation.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MetricSpec {
    /// Metric kind identifier.
    pub kind: MetricKind,
    /// Optional K value for rank-based metrics.
    #[serde(default)]
    pub k: Option<u32>,
}

impl MetricSpec {
    fn validate(&self) -> CocoResult<()> {
        if matches!(self.kind, MetricKind::RecallAtK) {
            let k = self
                .k
                .ok_or_else(|| CocoError::user("metric recall_at_k requires k"))?;
            if k == 0 {
                return Err(CocoError::user("metric recall_at_k requires k > 0"));
            }
        }
        Ok(())
    }
}

/// Supported metric kinds for experiment evaluation.
#[derive(Debug, Clone, Copy, Serialize, Deserialize, Eq, PartialEq, Hash)]
#[serde(rename_all = "snake_case")]
pub enum MetricKind {
    /// Recall@K metric.
    RecallAtK,
    /// Mean reciprocal rank (MRR).
    Mrr,
    /// Hit rate metric.
    HitRate,
    /// Latency in milliseconds.
    LatencyMs,
}

/// Experiment results serialized to JSON.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ExperimentResultFile {
    /// Experiment name.
    pub name: String,
    /// Project identifier.
    pub project_id: String,
    /// Dataset revision label.
    #[serde(default)]
    pub dataset_revision: Option<String>,
    /// Dataset commit hash.
    #[serde(default)]
    pub dataset_commit: Option<String>,
    /// Random seed used for the run.
    pub random_seed: u64,
    /// Concurrency level used for the run.
    pub concurrency: u32,
    /// Timestamp for the run.
    pub run_at: String,
    /// Collected results.
    pub results: Vec<ExperimentResultEntry>,
}

/// Result entry per indexing/query strategy pair.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ExperimentResultEntry {
    /// Indexing configuration identifier.
    pub indexing_config_id: String,
    /// Query strategy identifier.
    pub query_config_id: String,
    /// Retrieval mode used for the query strategy.
    pub retrieval_mode: RetrievalMode,
    /// Optional version identifier (server-side).
    #[serde(default)]
    pub version_id: Option<String>,
    /// Metric values.
    pub metrics: Vec<MetricValue>,
}

/// Metric value output for experiment results.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MetricValue {
    /// Metric kind.
    pub kind: MetricKind,
    /// Optional K value for rank-based metrics.
    #[serde(default)]
    pub k: Option<u32>,
    /// Metric value.
    pub value: f32,
}
