mod config;
mod crawler;
mod discovery;
mod http_client;
mod models;
mod storage;
mod util;

use clap::{Parser, Subcommand};
use config::{get_source, load_config};
use crawler::Crawler;
use log::LevelFilter;
use std::path::PathBuf;

#[derive(Parser)]
#[command(name = "docs-crawler", about = "Crawl md/mdx docs for LLM reference")]
struct Cli {
    #[arg(long, default_value = "sources.json")]
    config: PathBuf,
    #[arg(long, default_value = "INFO")]
    log_level: String,
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    Crawl {
        #[arg(long)]
        source: Option<String>,
        #[arg(long)]
        prune: bool,
        #[arg(long)]
        dry_run: bool,
        #[arg(long)]
        max_workers: Option<usize>,
        #[arg(long)]
        timeout: Option<u64>,
    },
}

fn configure_logging(level_name: &str) -> Result<(), String> {
    let filter = match level_name.to_uppercase().as_str() {
        "TRACE" => LevelFilter::Trace,
        "DEBUG" => LevelFilter::Debug,
        "INFO" => LevelFilter::Info,
        "WARN" | "WARNING" => LevelFilter::Warn,
        "ERROR" => LevelFilter::Error,
        _ => return Err(format!("Invalid log level: {level_name}")),
    };

    env_logger::Builder::new()
        .filter_level(filter)
        .format(|buf, record| {
            use std::io::Write;
            writeln!(
                buf,
                "{} {} {}: {}",
                buf.timestamp(),
                record.level(),
                record.target(),
                record.args()
            )
        })
        .init();
    Ok(())
}

fn main() {
    let cli = Cli::parse();
    if let Err(err) = configure_logging(&cli.log_level) {
        eprintln!("{err}");
        std::process::exit(2);
    }

    let config = match load_config(&cli.config) {
        Ok(config) => config,
        Err(err) => {
            eprintln!("{err}");
            std::process::exit(2);
        }
    };

    match cli.command {
        Commands::Crawl {
            source,
            prune,
            dry_run,
            max_workers,
            timeout,
        } => {
            let mut sources = config.sources.clone();
            if let Some(source_id) = source.as_deref() {
                match get_source(&config, source_id) {
                    Some(src) => sources = vec![src.clone()],
                    None => {
                        eprintln!("Unknown source: {source_id}");
                        std::process::exit(2);
                    }
                }
            }

            let mut exit_code = 0;
            for src in sources {
                let defaults = &config.defaults;
                let crawler = Crawler::new(
                    &defaults.allowed_extensions,
                    &defaults.user_agent,
                    timeout.unwrap_or(defaults.timeout_seconds),
                    max_workers
                        .or(src.max_workers)
                        .unwrap_or(defaults.max_workers),
                );
                match crawler.crawl_source(&src, prune, dry_run) {
                    Ok(stats) => {
                        println!(
                            "[{}] discovered={} fetched={} updated={} unchanged={} errors={} pruned={}",
                            src.id,
                            stats.discovered,
                            stats.fetched,
                            stats.updated,
                            stats.unchanged,
                            stats.errors,
                            stats.pruned
                        );
                        if stats.errors > 0 {
                            exit_code = 1;
                        }
                    }
                    Err(err) => {
                        eprintln!("{}", err);
                        std::process::exit(1);
                    }
                }
            }
            std::process::exit(exit_code);
        }
    }
}
