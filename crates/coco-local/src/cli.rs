//! Command-line interface definitions for coco-local.

use std::path::PathBuf;

use clap::{Args, Parser, Subcommand, ValueEnum};

use crate::service::LocalServiceConfig;
use coco_protocol::VectorMetric;

/// CoCo local command-line interface.
#[derive(Debug, Clone, Parser)]
#[command(name = "coco", version, about = "CoCo local service")]
pub struct Cli {
    /// Command to execute.
    #[command(subcommand)]
    pub command: Option<Command>,
}

impl Cli {
    /// Returns the explicit command or the default start command.
    pub fn command_or_default(&self) -> Command {
        self.command
            .clone()
            .unwrap_or(Command::Start(StartArgs::default()))
    }
}

/// Available CLI commands.
#[derive(Debug, Clone, Subcommand)]
pub enum Command {
    /// Start the local service.
    Start(StartArgs),
    /// Import a file or directory.
    Import(ImportArgs),
    /// Show service status.
    Status(StatusArgs),
    /// Prune archived versions.
    Prune(PruneArgs),
    /// Check for updates.
    Update(UpdateArgs),
    /// Prepare local resources.
    Setup(SetupArgs),
    /// Manage Tree-sitter grammars.
    Grammar(GrammarArgs),
    /// Manage indexing configs.
    Config(ConfigArgs),
    /// Run experiments.
    Experiment(ExperimentArgs),
}

/// Arguments for starting the service.
#[derive(Debug, Clone, Default, Args)]
pub struct StartArgs {
    /// Host to bind.
    #[arg(long)]
    pub host: Option<String>,
    /// Port to bind.
    #[arg(long)]
    pub port: Option<u16>,
    /// Run without the TUI.
    #[arg(long)]
    pub headless: bool,
    /// Local ONNX model path.
    #[arg(long)]
    pub model_path: Option<PathBuf>,
    /// Model download URL override.
    #[arg(long)]
    pub model_url: Option<String>,
}

impl StartArgs {
    /// Applies CLI overrides to the service configuration.
    pub fn apply_overrides(&self, config: &mut LocalServiceConfig) {
        if let Some(host) = &self.host {
            config.host = host.clone();
        }
        if let Some(port) = self.port {
            config.port = port;
        }
        if let Some(path) = &self.model_path {
            config.model_path = Some(path.clone());
        }
        if let Some(url) = &self.model_url {
            config.model_url = Some(url.clone());
        }
    }
}

/// Arguments for importing content.
#[derive(Debug, Clone, Args)]
pub struct ImportArgs {
    /// File or directory to import.
    pub path: PathBuf,
    /// Recursively import a directory.
    #[arg(long)]
    pub recursive: bool,
    /// Override project id.
    #[arg(long)]
    pub project_id: Option<String>,
    /// Project root path for registration.
    #[arg(long)]
    pub project_path: Option<PathBuf>,
    /// Project name override.
    #[arg(long)]
    pub project_name: Option<String>,
    /// Service host.
    #[arg(long)]
    pub host: Option<String>,
    /// Service port.
    #[arg(long)]
    pub port: Option<u16>,
}

/// Arguments for status checks.
#[derive(Debug, Clone, Args)]
pub struct StatusArgs {
    /// Service host.
    #[arg(long)]
    pub host: Option<String>,
    /// Service port.
    #[arg(long)]
    pub port: Option<u16>,
}

/// Arguments for pruning archived versions.
#[derive(Debug, Clone, Args)]
pub struct PruneArgs {
    /// Project identifier to prune.
    pub project_id: String,
    /// Keep the most recent N archived versions.
    #[arg(long)]
    pub keep: Option<usize>,
    /// Service host.
    #[arg(long)]
    pub host: Option<String>,
    /// Service port.
    #[arg(long)]
    pub port: Option<u16>,
}

/// Arguments for update checks.
#[derive(Debug, Clone, Args)]
pub struct UpdateArgs {
    /// Install the latest version if available.
    #[arg(long)]
    pub install: bool,
    /// GitHub repository in owner/name form.
    #[arg(long)]
    pub repo: Option<String>,
}

/// Arguments for setup tasks.
#[derive(Debug, Clone, Args)]
pub struct SetupArgs {
    /// Local ONNX model file path to install.
    #[arg(long)]
    pub model_path: PathBuf,
}

/// Grammar subcommands.
#[derive(Debug, Clone, Args)]
pub struct GrammarArgs {
    /// Grammar operation.
    #[command(subcommand)]
    pub command: GrammarCommand,
}

/// Grammar management operations.
#[derive(Debug, Clone, Subcommand)]
pub enum GrammarCommand {
    /// List installed grammars.
    List,
    /// Install a grammar.
    Install(GrammarInstallArgs),
    /// Update installed grammars.
    Update(GrammarUpdateArgs),
}

/// Config subcommands.
#[derive(Debug, Clone, Args)]
pub struct ConfigArgs {
    /// Config operation.
    #[command(subcommand)]
    pub command: ConfigCommand,
}

/// Experiment subcommands.
#[derive(Debug, Clone, Args)]
pub struct ExperimentArgs {
    /// Experiment operation.
    #[command(subcommand)]
    pub command: ExperimentCommand,
}

/// Experiment operations.
#[derive(Debug, Clone, Subcommand)]
pub enum ExperimentCommand {
    /// Run an experiment spec.
    Run(ExperimentRunArgs),
    /// Compare experiment results.
    Compare(ExperimentCompareArgs),
}

/// Experiment run arguments.
#[derive(Debug, Clone, Args)]
pub struct ExperimentRunArgs {
    /// Experiment spec YAML path.
    pub spec: PathBuf,
    /// Output JSON path.
    #[arg(long)]
    pub output: Option<PathBuf>,
    /// Service host.
    #[arg(long)]
    pub host: Option<String>,
    /// Service port.
    #[arg(long)]
    pub port: Option<u16>,
}

/// Experiment compare arguments.
#[derive(Debug, Clone, Args)]
pub struct ExperimentCompareArgs {
    /// Results JSON files.
    pub results: Vec<PathBuf>,
    /// Filter by indexing config id.
    #[arg(long)]
    pub indexing_config_id: Option<String>,
    /// Filter by query config id.
    #[arg(long)]
    pub query_config_id: Option<String>,
    /// Filter by retrieval mode.
    #[arg(long, value_enum)]
    pub retrieval_mode: Option<CliRetrievalMode>,
}

/// CLI retrieval mode enum.
#[derive(Debug, Clone, Copy, ValueEnum)]
pub enum CliRetrievalMode {
    /// Vector-only retrieval.
    Vector,
    /// Full-text retrieval.
    Fts,
    /// Hybrid retrieval.
    Hybrid,
}

/// Config management operations.
#[derive(Debug, Clone, Subcommand)]
pub enum ConfigCommand {
    /// List indexing configs.
    List(ConfigListArgs),
    /// Create or update an indexing config.
    Set(ConfigSetArgs),
    /// Activate an indexing config for a project.
    Use(ConfigUseArgs),
}

/// List config arguments.
#[derive(Debug, Clone, Args)]
pub struct ConfigListArgs {
    /// Service host.
    #[arg(long)]
    pub host: Option<String>,
    /// Service port.
    #[arg(long)]
    pub port: Option<u16>,
    /// Project id to show the active config.
    #[arg(long)]
    pub project_id: Option<String>,
}

/// Config set arguments.
#[derive(Debug, Clone, Args)]
pub struct ConfigSetArgs {
    /// Service host.
    #[arg(long)]
    pub host: Option<String>,
    /// Service port.
    #[arg(long)]
    pub port: Option<u16>,
    /// Load config JSON from file.
    #[arg(long)]
    pub file: Option<PathBuf>,
    /// Config identifier.
    #[arg(long)]
    pub config_id: Option<String>,
    /// Chunking strategy name.
    #[arg(long)]
    pub chunking_strategy: Option<String>,
    /// Chunk size.
    #[arg(long)]
    pub chunk_size: Option<u32>,
    /// Chunk overlap size.
    #[arg(long)]
    pub chunk_overlap: Option<u32>,
    /// Embedding model name.
    #[arg(long)]
    pub embedding_model: Option<String>,
    /// Embedding dimensions.
    #[arg(long)]
    pub embedding_dimensions: Option<u32>,
    /// Vector similarity metric.
    #[arg(long, value_enum)]
    pub vector_metric: Option<CliVectorMetric>,
    /// Index params JSON file.
    #[arg(long)]
    pub index_params: Option<PathBuf>,
}

/// Config activation arguments.
#[derive(Debug, Clone, Args)]
pub struct ConfigUseArgs {
    /// Service host.
    #[arg(long)]
    pub host: Option<String>,
    /// Service port.
    #[arg(long)]
    pub port: Option<u16>,
    /// Project identifier.
    #[arg(long)]
    pub project_id: String,
    /// Config identifier to activate.
    #[arg(long)]
    pub config_id: String,
}

/// CLI vector metric enum.
#[derive(Debug, Clone, Copy, ValueEnum)]
pub enum CliVectorMetric {
    /// Cosine similarity.
    Cosine,
    /// Dot product similarity.
    Dot,
    /// L2 distance.
    L2,
}

impl From<CliVectorMetric> for VectorMetric {
    fn from(value: CliVectorMetric) -> Self {
        match value {
            CliVectorMetric::Cosine => VectorMetric::Cosine,
            CliVectorMetric::Dot => VectorMetric::Dot,
            CliVectorMetric::L2 => VectorMetric::L2,
        }
    }
}

/// Grammar install arguments.
#[derive(Debug, Clone, Args)]
pub struct GrammarInstallArgs {
    /// Grammar language key.
    pub lang: String,
    /// Override grammar URL.
    #[arg(long)]
    pub url: Option<String>,
    /// Override grammar SHA-256 checksum.
    #[arg(long)]
    pub sha256: Option<String>,
    /// Base URL for grammar downloads.
    #[arg(long)]
    pub base_url: Option<String>,
}

/// Grammar update arguments.
#[derive(Debug, Clone, Args)]
pub struct GrammarUpdateArgs {
    /// Base URL for grammar downloads.
    #[arg(long)]
    pub base_url: Option<String>,
}
