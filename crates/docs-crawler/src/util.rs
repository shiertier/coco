use once_cell::sync::Lazy;
use regex::Regex;
use sha2::{Digest, Sha256};
use std::path::{Component, Path, PathBuf};
use url::Url;

pub static LANG_SEGMENT_RE: Lazy<Regex> =
    Lazy::new(|| Regex::new(r"(?i)^[a-z]{2}(?:-[a-z0-9]{2,8})?$").expect("valid language regex"));

pub fn iso_now() -> String {
    chrono::Utc::now().to_rfc3339()
}

pub fn normalize_url(url: &str) -> String {
    url.split('#').next().unwrap_or(url).to_string()
}

pub fn url_has_allowed_extension(url: &str, allowed_extensions: &[String]) -> bool {
    if allowed_extensions.iter().any(|ext| ext == "*") {
        return true;
    }
    let path = Url::parse(url)
        .ok()
        .map(|parsed| parsed.path().to_string())
        .unwrap_or_else(|| url.to_string());
    let path = path.to_lowercase();
    allowed_extensions
        .iter()
        .any(|ext| path.ends_with(&ext.to_lowercase()))
}

pub fn join_url(base: &str, maybe_relative: &str) -> Result<String, String> {
    let base_url = Url::parse(base).map_err(|err| format!("Invalid base url: {err}"))?;
    let joined = base_url
        .join(maybe_relative)
        .map_err(|err| format!("Failed to join url: {err}"))?;
    Ok(normalize_url(joined.as_str()))
}

pub fn split_language_from_path(
    path: &Path,
    default_language: &str,
    language_hint: Option<&str>,
) -> (String, PathBuf) {
    let parts: Vec<String> = path
        .components()
        .filter_map(|component| match component {
            Component::Normal(value) => Some(value.to_string_lossy().to_string()),
            _ => None,
        })
        .collect();

    let hint = language_hint
        .map(|value| value.trim().to_lowercase())
        .filter(|value| !value.is_empty());

    if let Some(first) = parts.first() {
        if LANG_SEGMENT_RE.is_match(first) {
            let lang = first.to_lowercase();
            let remainder = if parts.len() > 1 {
                let mut remainder = PathBuf::new();
                for part in parts.iter().skip(1) {
                    remainder.push(part);
                }
                remainder
            } else {
                PathBuf::from("index.md")
            };
            return (lang, remainder);
        }
    }

    if let Some(hint) = hint {
        if parts.len() > 1
            && parts[0].eq_ignore_ascii_case("docs")
            && LANG_SEGMENT_RE.is_match(&parts[1])
            && parts[1].eq_ignore_ascii_case(&hint)
        {
            let remainder = if parts.len() > 2 {
                let mut remainder = PathBuf::new();
                remainder.push(&parts[0]);
                for part in parts.iter().skip(2) {
                    remainder.push(part);
                }
                remainder
            } else {
                let mut remainder = PathBuf::new();
                remainder.push(&parts[0]);
                remainder.push("index.md");
                remainder
            };
            return (parts[1].to_lowercase(), remainder);
        }
        return (hint, path.to_path_buf());
    }

    (default_language.trim().to_lowercase(), path.to_path_buf())
}

pub fn safe_relative_path(url: &str) -> PathBuf {
    let raw_path = Url::parse(url)
        .ok()
        .map(|parsed| parsed.path().to_string())
        .unwrap_or_else(|| url.to_string());
    let raw_path = raw_path.trim_start_matches('/');
    if raw_path.is_empty() {
        return PathBuf::from("index.md");
    }
    let mut parts = Vec::new();
    for part in raw_path.split('/') {
        match part {
            "" | "." => continue,
            ".." => continue,
            value => parts.push(value.to_string()),
        }
    }
    if parts.is_empty() {
        PathBuf::from("index.md")
    } else {
        let mut path = PathBuf::new();
        for part in parts {
            path.push(part);
        }
        path
    }
}

pub fn collapse_index_path(path: &Path) -> PathBuf {
    let name = path
        .file_name()
        .map(|value| value.to_string_lossy().to_lowercase());
    if let Some(name) = name {
        if name == "index.md" || name == "index.mdx" {
            if let Some(parent) = path.parent() {
                if parent != Path::new(".") && parent != Path::new("") {
                    if let Some(ext) = path.extension() {
                        return parent.with_extension(ext);
                    }
                }
            }
        }
    }
    path.to_path_buf()
}

pub fn sha256_bytes(data: &[u8]) -> String {
    let mut hasher = Sha256::new();
    hasher.update(data);
    format!("{:x}", hasher.finalize())
}

pub fn ensure_within_dir(root: &Path, path: &Path) -> Result<PathBuf, String> {
    let root_abs = absolutize(root);
    let candidate = absolutize(path);
    let root_clean = clean_path(&root_abs);
    let candidate_clean = clean_path(&candidate);
    if !candidate_clean.starts_with(&root_clean) {
        return Err(format!(
            "Refusing to write outside root: {}",
            path.display()
        ));
    }
    Ok(candidate_clean)
}

fn absolutize(path: &Path) -> PathBuf {
    if path.is_absolute() {
        clean_path(path)
    } else {
        match std::env::current_dir() {
            Ok(cwd) => clean_path(&cwd.join(path)),
            Err(_) => clean_path(path),
        }
    }
}

fn clean_path(path: &Path) -> PathBuf {
    let mut result = PathBuf::new();
    for component in path.components() {
        match component {
            Component::Prefix(prefix) => result.push(prefix.as_os_str()),
            Component::RootDir => result.push(Path::new(std::path::MAIN_SEPARATOR_STR)),
            Component::CurDir => {}
            Component::ParentDir => {
                result.pop();
            }
            Component::Normal(value) => result.push(value),
        }
    }
    result
}

pub fn path_to_posix(path: &Path) -> String {
    path.to_string_lossy().replace('\\', "/")
}

#[derive(Debug, Default, Clone)]
pub struct CrawlStats {
    pub discovered: usize,
    pub fetched: usize,
    pub updated: usize,
    pub unchanged: usize,
    pub errors: usize,
    pub pruned: usize,
}
