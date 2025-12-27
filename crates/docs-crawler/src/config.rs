use crate::models::{CrawlerConfig, DefaultsConfig, EntryPointConfig, SourceConfig};
use serde::Deserialize;
use std::env;
use std::error::Error;
use std::ffi::OsStr;
use std::fmt::{self, Display};
use std::fs;
use std::path::{Component, Path, PathBuf};

#[derive(Debug)]
pub struct ConfigError(String);

impl Display for ConfigError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.0)
    }
}

impl Error for ConfigError {}

#[derive(Debug, Deserialize)]
struct ConfigRaw {
    version: i64,
    defaults: Option<DefaultsRaw>,
    sources: Vec<SourceRaw>,
}

#[derive(Debug, Deserialize)]
struct DefaultsRaw {
    allowed_extensions: Option<Vec<String>>,
    user_agent: Option<String>,
    timeout_seconds: Option<u64>,
    max_workers: Option<usize>,
}

#[derive(Debug, Deserialize)]
struct SourceRaw {
    id: String,
    output_dir: String,
    default_language: Option<String>,
    entrypoints: Vec<EntryPointRaw>,
    allowed_extensions: Option<Vec<String>>,
    max_workers: Option<usize>,
}

#[derive(Debug, Deserialize)]
struct EntryPointRaw {
    #[serde(rename = "type")]
    kind: String,
    url: Option<String>,
    language: Option<String>,
    repo: Option<String>,
    #[serde(rename = "ref")]
    r#ref: Option<String>,
    path: Option<String>,
    languages: Option<Vec<String>>,
    api_base_url: Option<String>,
    raw_base_url: Option<String>,
    language_variants: Option<Vec<String>>,
    git_cache_dir: Option<String>,
}

fn expand_tilde(path: &str) -> PathBuf {
    if path == "~" || path.starts_with("~/") {
        if let Ok(home) = env::var("HOME").or_else(|_| env::var("USERPROFILE")) {
            let suffix = path.strip_prefix("~/").unwrap_or("");
            return PathBuf::from(home).join(suffix);
        }
    }
    PathBuf::from(path)
}

fn scrapy_docs_root() -> Option<PathBuf> {
    if let Ok(value) = env::var("COCO_SCRAPY_DOCS_DIR") {
        let trimmed = value.trim();
        if !trimmed.is_empty() {
            return Some(expand_tilde(trimmed));
        }
    }
    let username = env::var("USER")
        .or_else(|_| env::var("USERNAME"))
        .ok();
    if username.as_deref() == Some("scrapy") {
        let home = env::var("HOME")
            .or_else(|_| env::var("USERPROFILE"))
            .ok()?;
        return Some(PathBuf::from(home).join(".cache").join("coco").join("scrapy_docs"));
    }
    None
}

fn strip_scrapy_docs_prefix(path: &Path) -> Option<PathBuf> {
    let mut components = path.components();
    let first = loop {
        match components.next() {
            Some(Component::CurDir) => continue,
            other => break other,
        }
    };
    match first {
        Some(Component::Normal(name)) if name == OsStr::new("scrapy_docs") => {
            let rest: PathBuf = components.collect();
            Some(rest)
        }
        _ => None,
    }
}

fn resolve_output_dir(output_dir: PathBuf) -> PathBuf {
    if output_dir.is_absolute() {
        return output_dir;
    }
    let Some(root) = scrapy_docs_root() else {
        return output_dir;
    };
    if let Some(stripped) = strip_scrapy_docs_prefix(&output_dir) {
        return root.join(stripped);
    }
    root.join(output_dir)
}

fn trim_required(field: &str, value: String) -> Result<String, ConfigError> {
    let trimmed = value.trim();
    if trimmed.is_empty() {
        return Err(ConfigError(format!("{} must be a non-empty string", field)));
    }
    Ok(trimmed.to_string())
}

fn trim_optional(value: Option<String>) -> Option<String> {
    value.and_then(|raw| {
        let trimmed = raw.trim();
        if trimmed.is_empty() {
            None
        } else {
            Some(trimmed.to_string())
        }
    })
}

fn normalize_list(field: &str, items: Option<Vec<String>>) -> Result<Option<Vec<String>>, ConfigError> {
    let Some(items) = items else {
        return Ok(None);
    };
    let mut trimmed = Vec::with_capacity(items.len());
    for item in items {
        let t = item.trim();
        if t.is_empty() {
            return Err(ConfigError(format!("{} must be a list of non-empty strings", field)));
        }
        trimmed.push(t.to_string());
    }
    Ok(Some(trimmed))
}

pub fn load_config(path: &Path) -> Result<CrawlerConfig, ConfigError> {
    let raw = fs::read_to_string(path)
        .map_err(|_| ConfigError(format!("Config file not found: {}", path.display())))?;
    let parsed: ConfigRaw = serde_json::from_str(&raw)
        .map_err(|_| ConfigError(format!("Invalid JSON in config file: {}", path.display())))?;

    if parsed.version != 1 {
        return Err(ConfigError(format!("Unsupported config version: {}", parsed.version)));
    }

    let defaults_raw = parsed.defaults.unwrap_or(DefaultsRaw {
        allowed_extensions: None,
        user_agent: None,
        timeout_seconds: None,
        max_workers: None,
    });

    let allowed_extensions = defaults_raw
        .allowed_extensions
        .unwrap_or_else(|| vec![".md".to_string(), ".mdx".to_string()]);
    if allowed_extensions.is_empty() {
        return Err(ConfigError("defaults.allowed_extensions must be a list of strings".to_string()));
    }

    let user_agent = defaults_raw
        .user_agent
        .unwrap_or_else(|| "docs-crawler/0.1".to_string());
    if user_agent.trim().is_empty() {
        return Err(ConfigError("defaults.user_agent must be a non-empty string".to_string()));
    }

    let timeout_seconds = defaults_raw.timeout_seconds.unwrap_or(30);
    if timeout_seconds == 0 {
        return Err(ConfigError("defaults.timeout_seconds must be a positive integer".to_string()));
    }

    let max_workers = defaults_raw.max_workers.unwrap_or(8);
    if max_workers == 0 {
        return Err(ConfigError("defaults.max_workers must be a positive integer".to_string()));
    }

    let defaults = DefaultsConfig {
        allowed_extensions,
        user_agent,
        timeout_seconds,
        max_workers,
    };

    let mut sources = Vec::with_capacity(parsed.sources.len());
    for (idx, src) in parsed.sources.into_iter().enumerate() {
        let source_id = trim_required(&format!("sources[{idx}].id"), src.id)?;
        let output_dir = trim_required(&format!("sources[{idx}].output_dir"), src.output_dir)?;
        let output_dir = resolve_output_dir(PathBuf::from(output_dir));
        let default_language = trim_required(
            &format!("sources[{idx}].default_language"),
            src.default_language.unwrap_or_else(|| "en".to_string()),
        )?;

        let allowed_extensions = normalize_list(&format!("sources[{idx}].allowed_extensions"), src.allowed_extensions)?;
        let max_workers = match src.max_workers {
            Some(0) => {
                return Err(ConfigError(format!(
                    "sources[{idx}].max_workers must be a positive integer"
                )))
            }
            other => other,
        };

        let mut entrypoints = Vec::with_capacity(src.entrypoints.len());
        for (eidx, ep) in src.entrypoints.into_iter().enumerate() {
            let kind = trim_required(&format!("sources[{idx}].entrypoints[{eidx}].type"), ep.kind)?;
            let url = trim_optional(ep.url);
            let language = trim_optional(ep.language);
            let repo = trim_optional(ep.repo);
            let r#ref = trim_optional(ep.r#ref);
            let path = trim_optional(ep.path);
            let languages = normalize_list(
                &format!("sources[{idx}].entrypoints[{eidx}].languages"),
                ep.languages,
            )?;
            let mut language_variants = normalize_list(
                &format!("sources[{idx}].entrypoints[{eidx}].language_variants"),
                ep.language_variants,
            )?;
            if let Some(variants) = language_variants.as_mut() {
                if variants.is_empty() {
                    language_variants = None;
                } else if language.is_none() {
                    return Err(ConfigError(format!(
                        "sources[{idx}].entrypoints[{eidx}].language must be set when language_variants is used"
                    )));
                }
            }

            if kind == "llms_txt" {
                if url.is_none() {
                    return Err(ConfigError(format!(
                        "sources[{idx}].entrypoints[{eidx}].url must be a non-empty string"
                    )));
                }
            } else if kind == "github_tree" {
                if repo.is_none() {
                    return Err(ConfigError(format!(
                        "sources[{idx}].entrypoints[{eidx}].repo must be a non-empty string"
                    )));
                }
                if path.is_none() {
                    return Err(ConfigError(format!(
                        "sources[{idx}].entrypoints[{eidx}].path must be a non-empty string"
                    )));
                }
            }

            entrypoints.push(EntryPointConfig {
                kind,
                url,
                language,
                repo,
                r#ref,
                path,
                languages,
                api_base_url: trim_optional(ep.api_base_url),
                raw_base_url: trim_optional(ep.raw_base_url),
                language_variants,
                git_cache_dir: trim_optional(ep.git_cache_dir),
            });
        }

        sources.push(SourceConfig {
            id: source_id,
            output_dir,
            default_language,
            entrypoints,
            allowed_extensions,
            max_workers,
        });
    }

    Ok(CrawlerConfig {
        defaults,
        sources,
    })
}

pub fn get_source<'a>(config: &'a CrawlerConfig, source_id: &str) -> Option<&'a SourceConfig> {
    config.sources.iter().find(|source| source.id == source_id)
}
