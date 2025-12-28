use crate::http_client::HttpClient;
use crate::models::EntryPointConfig;
use crate::util::{join_url, normalize_url, split_language_from_path, url_has_allowed_extension};
use once_cell::sync::Lazy;
use percent_encoding::AsciiSet;
use percent_encoding::CONTROLS;
use regex::Regex;
use serde::Deserialize;
use std::collections::{HashMap, HashSet};
use std::env;
use std::error::Error;
use std::fmt::{self, Display};
use std::fs;
use std::path::{Path, PathBuf};
use std::process::{Command, Stdio};
use tempfile::TempDir;
use url::Url;

static ORDERED_LINK_RE: Lazy<Regex> = Lazy::new(|| {
    Regex::new(r"\[[^\]]*\]\(([^)]+)\)|https?://[^\s)]+").expect("valid llms.txt regex")
});

const GITHUB_API_BASE_URL: &str = "https://api.github.com";
const GITHUB_RAW_BASE_URL: &str = "https://raw.githubusercontent.com";

#[derive(Debug)]
pub struct DiscoveryError(String);

impl Display for DiscoveryError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.0)
    }
}

impl Error for DiscoveryError {}

#[derive(Debug, Clone)]
pub struct DiscoveredDoc {
    pub url: String,
    pub language_hint: Option<String>,
    pub relative_path: Option<PathBuf>,
}

pub fn discover_documents(
    entrypoint: &EntryPointConfig,
    allowed_extensions: &[String],
    http: &HttpClient,
    default_language: &str,
    cache_dir: Option<&Path>,
) -> Result<Vec<DiscoveredDoc>, DiscoveryError> {
    match entrypoint.kind.as_str() {
        "llms_txt" => discover_llms_txt(entrypoint, allowed_extensions, http),
        "github_tree" => discover_github_tree(
            entrypoint,
            allowed_extensions,
            http,
            default_language,
            cache_dir,
        ),
        other => Err(DiscoveryError(format!(
            "Unsupported entrypoint type: {other}"
        ))),
    }
}

fn discover_llms_txt(
    entrypoint: &EntryPointConfig,
    allowed_extensions: &[String],
    http: &HttpClient,
) -> Result<Vec<DiscoveredDoc>, DiscoveryError> {
    let url = entrypoint
        .url
        .as_ref()
        .ok_or_else(|| DiscoveryError("llms_txt entrypoint requires url".to_string()))?;
    let text = http
        .fetch_text(url)
        .map_err(|err| DiscoveryError(err.to_string()))?;

    let base_language = entrypoint
        .language
        .as_ref()
        .map(|lang| lang.trim().to_string());
    let language_hint = base_language.as_ref().map(|lang| lang.to_lowercase());
    let variant_languages = entrypoint.language_variants.as_ref();
    if variant_languages.is_some() && base_language.is_none() {
        return Err(DiscoveryError(
            "language_variants requires entrypoint.language".to_string(),
        ));
    }

    let mut urls = HashSet::new();
    let mut ordered = Vec::new();

    for line in text.lines() {
        let Some(captures) = ORDERED_LINK_RE.captures(line) else {
            continue;
        };
        let dest = if let Some(group) = captures.get(1) {
            group.as_str()
        } else {
            captures.get(0).map(|m| m.as_str()).unwrap_or("")
        };
        let mut dest = dest.trim();
        if dest.is_empty() || dest.starts_with('#') {
            continue;
        }
        let abs_url = if captures.get(1).is_some() {
            dest = dest.split_whitespace().next().unwrap_or("");
            dest = dest.trim_matches(&['"', '\''][..]);
            if dest.is_empty() || dest.starts_with('#') {
                continue;
            }
            join_url(url, dest).map_err(DiscoveryError)?
        } else {
            normalize_url(dest)
        };

        let parsed = Url::parse(&abs_url);
        if let Ok(parsed) = parsed {
            if parsed.scheme() != "http" && parsed.scheme() != "https" {
                continue;
            }
        }

        if !url_has_allowed_extension(&abs_url, allowed_extensions) {
            continue;
        }

        let abs_url = normalize_url(&abs_url);
        if let Some(variants) = variant_languages {
            if urls.insert(abs_url.clone()) {
                ordered.push(DiscoveredDoc {
                    url: abs_url.clone(),
                    language_hint: language_hint.clone(),
                    relative_path: None,
                });
            }
            let base_language = base_language.as_ref().unwrap();
            for variant in variants {
                let variant_clean = variant.trim();
                if variant_clean.is_empty() {
                    continue;
                }
                if variant_clean.eq_ignore_ascii_case(base_language) {
                    continue;
                }
                let Some(variant_url) =
                    rewrite_language_segment(&abs_url, base_language, variant_clean)
                else {
                    continue;
                };
                let variant_url = normalize_url(&variant_url);
                if urls.insert(variant_url.clone()) {
                    ordered.push(DiscoveredDoc {
                        url: variant_url,
                        language_hint: Some(variant_clean.to_lowercase()),
                        relative_path: None,
                    });
                }
            }
            continue;
        }

        if urls.insert(abs_url.clone()) {
            ordered.push(DiscoveredDoc {
                url: abs_url,
                language_hint: language_hint.clone(),
                relative_path: None,
            });
        }
    }

    Ok(ordered)
}

fn discover_github_tree(
    entrypoint: &EntryPointConfig,
    allowed_extensions: &[String],
    http: &HttpClient,
    default_language: &str,
    cache_dir: Option<&Path>,
) -> Result<Vec<DiscoveredDoc>, DiscoveryError> {
    let repo = entrypoint
        .repo
        .as_ref()
        .ok_or_else(|| DiscoveryError("github_tree entrypoint requires repo".to_string()))?
        .trim()
        .trim_matches('/');
    let r#ref = entrypoint
        .r#ref
        .as_deref()
        .unwrap_or("main")
        .trim()
        .to_string();
    let base_path = entrypoint
        .path
        .as_ref()
        .ok_or_else(|| DiscoveryError("github_tree entrypoint requires path".to_string()))?
        .trim()
        .trim_matches('/')
        .to_string();
    let api_base_url = entrypoint
        .api_base_url
        .as_deref()
        .unwrap_or(GITHUB_API_BASE_URL)
        .trim()
        .trim_end_matches('/');
    let raw_base_url = entrypoint
        .raw_base_url
        .as_deref()
        .unwrap_or(GITHUB_RAW_BASE_URL)
        .trim()
        .trim_end_matches('/');

    let api_ref = percent_encode_component(&r#ref);
    let api_url = format!("{api_base_url}/repos/{repo}/git/trees/{api_ref}?recursive=1");

    let tree_paths = match fetch_github_tree(&api_url, http) {
        Ok(paths) => paths,
        Err(err) => list_repo_paths_via_git(repo, &r#ref, cache_dir)
            .map_err(|git_err| DiscoveryError(format!("{err}; git fallback failed: {git_err}")))?,
    };

    let base_language = entrypoint
        .language
        .as_deref()
        .unwrap_or(default_language)
        .trim()
        .to_lowercase();
    let allow_languages: HashSet<String> = entrypoint
        .languages
        .clone()
        .unwrap_or_default()
        .into_iter()
        .filter_map(|lang| {
            let trimmed = lang.trim().to_lowercase();
            if trimmed.is_empty() {
                None
            } else {
                Some(trimmed)
            }
        })
        .collect();

    let mut ordered = Vec::new();
    let mut seen = HashSet::new();

    for item_path in tree_paths {
        if item_path.is_empty() {
            continue;
        }

        let rel_path = if base_path.is_empty() {
            PathBuf::from(&item_path)
        } else if item_path == base_path {
            match Path::new(&base_path).file_name() {
                Some(name) => PathBuf::from(name),
                None => PathBuf::from(&base_path),
            }
        } else if item_path.starts_with(&format!("{base_path}/")) {
            PathBuf::from(&item_path[base_path.len() + 1..])
        } else {
            continue;
        };

        let raw_url =
            build_raw_url(raw_base_url, repo, &r#ref, &item_path).map_err(DiscoveryError)?;

        if !url_has_allowed_extension(&raw_url, allowed_extensions) {
            continue;
        }

        if !allow_languages.is_empty() {
            let (language, _) = split_language_from_path(&rel_path, &base_language, None);
            if !allow_languages.contains(&language) {
                continue;
            }
        }

        if seen.insert(raw_url.clone()) {
            ordered.push(DiscoveredDoc {
                url: raw_url,
                language_hint: Some(base_language.clone()),
                relative_path: Some(rel_path),
            });
        }
    }

    ordered.sort_by(|a, b| {
        let a_key = a
            .relative_path
            .as_ref()
            .map(|path| path.to_string_lossy().to_string())
            .unwrap_or_else(|| a.url.clone());
        let b_key = b
            .relative_path
            .as_ref()
            .map(|path| path.to_string_lossy().to_string())
            .unwrap_or_else(|| b.url.clone());
        a_key.cmp(&b_key)
    });

    Ok(ordered)
}

fn percent_encode_component(value: &str) -> String {
    const ENCODE_SET: &AsciiSet = &CONTROLS
        .add(b' ')
        .add(b'"')
        .add(b'#')
        .add(b'<')
        .add(b'>')
        .add(b'?')
        .add(b'`')
        .add(b'{')
        .add(b'}')
        .add(b'%')
        .add(b'+');
    percent_encoding::utf8_percent_encode(value, ENCODE_SET).to_string()
}

fn build_raw_url(
    raw_base_url: &str,
    repo: &str,
    r#ref: &str,
    item_path: &str,
) -> Result<String, String> {
    let mut base = Url::parse(&format!("{raw_base_url}/{repo}/{ref}/"))
        .map_err(|err| format!("Invalid raw base url: {err}"))?;
    {
        let mut segments = base
            .path_segments_mut()
            .map_err(|_| "Cannot set raw URL path".to_string())?;
        segments.pop_if_empty();
        for segment in item_path.split('/') {
            if segment.is_empty() {
                continue;
            }
            segments.push(segment);
        }
    }
    Ok(base.to_string())
}

#[derive(Debug, Deserialize)]
struct GitTreeResponse {
    truncated: Option<bool>,
    tree: Option<Vec<GitTreeItem>>,
}

#[derive(Debug, Deserialize)]
struct GitTreeItem {
    path: Option<String>,
    #[serde(rename = "type")]
    kind: Option<String>,
}

fn fetch_github_tree(api_url: &str, http: &HttpClient) -> Result<Vec<String>, DiscoveryError> {
    let mut headers = HashMap::new();
    if let Ok(token) = env::var("GITHUB_TOKEN").or_else(|_| env::var("GH_TOKEN")) {
        headers.insert("Authorization".to_string(), format!("Bearer {token}"));
    }

    let result = http.fetch_bytes(
        api_url,
        Some("application/vnd.github+json"),
        None,
        None,
        if headers.is_empty() {
            None
        } else {
            Some(&headers)
        },
    );
    if result.status != 200 || result.body.is_none() {
        return Err(DiscoveryError(format!(
            "Failed to fetch {}: status={} error={}",
            api_url,
            result.status,
            result.error.unwrap_or_default()
        )));
    }

    let response: GitTreeResponse = serde_json::from_slice(result.body.as_ref().unwrap())
        .map_err(|_| DiscoveryError(format!("Invalid JSON from {api_url}")))?;
    if response.truncated.unwrap_or(false) {
        return Err(DiscoveryError(format!(
            "GitHub tree response truncated for {api_url}"
        )));
    }
    let tree = response
        .tree
        .ok_or_else(|| DiscoveryError("GitHub tree response missing tree list".to_string()))?;

    let mut paths = Vec::new();
    for item in tree {
        if item.kind.as_deref() != Some("blob") {
            continue;
        }
        if let Some(path) = item.path {
            paths.push(path);
        }
    }
    Ok(paths)
}

fn list_repo_paths_via_git(
    repo: &str,
    r#ref: &str,
    cache_dir: Option<&Path>,
) -> Result<Vec<String>, DiscoveryError> {
    let remote_url = format!("https://github.com/{repo}.git");
    let mut envs = HashMap::new();
    envs.insert("GIT_TERMINAL_PROMPT".to_string(), "0".to_string());
    envs.insert("GIT_HTTP_VERSION".to_string(), "HTTP/1.1".to_string());

    let token = env::var("GITHUB_TOKEN")
        .or_else(|_| env::var("GH_TOKEN"))
        .ok();
    let mut fetch_args = Vec::new();
    if let Some(token) = token.as_deref() {
        fetch_args.push("-c".to_string());
        fetch_args.push(format!("http.extraHeader=Authorization: Bearer {token}"));
    }
    fetch_args.push("fetch".to_string());
    fetch_args.push("--depth".to_string());
    fetch_args.push("1".to_string());
    fetch_args.push("origin".to_string());
    fetch_args.push(r#ref.to_string());

    let output = if let Some(cache_dir) = cache_dir {
        let repo_dir = prepare_git_cache_dir(cache_dir, repo, &remote_url, &envs)?;
        run_git(&fetch_args, &repo_dir, &envs, false)?;
        run_git(
            &[
                "ls-tree".to_string(),
                "-r".to_string(),
                "--name-only".to_string(),
                "FETCH_HEAD".to_string(),
            ],
            &repo_dir,
            &envs,
            true,
        )?
    } else {
        let temp_root = TempDir::new()
            .map_err(|err| DiscoveryError(format!("Failed to create git temp dir: {err}")))?;
        let repo_dir = temp_root.path().join("repo");
        let mut clone_args = Vec::new();
        if let Some(token) = token.as_deref() {
            clone_args.push("-c".to_string());
            clone_args.push(format!("http.extraHeader=Authorization: Bearer {token}"));
        }
        clone_args.push("clone".to_string());
        clone_args.push("--depth".to_string());
        clone_args.push("1".to_string());
        clone_args.push("--branch".to_string());
        clone_args.push(r#ref.to_string());
        clone_args.push(remote_url.clone());
        clone_args.push(repo_dir.to_string_lossy().to_string());

        run_git(&clone_args, temp_root.path(), &envs, false)?;
        run_git(
            &[
                "ls-tree".to_string(),
                "-r".to_string(),
                "--name-only".to_string(),
                "HEAD".to_string(),
            ],
            &repo_dir,
            &envs,
            true,
        )?
    };

    Ok(output
        .lines()
        .map(|line| line.trim().to_string())
        .filter(|line| !line.is_empty())
        .collect())
}

fn prepare_git_cache_dir(
    cache_dir: &Path,
    repo: &str,
    remote_url: &str,
    envs: &HashMap<String, String>,
) -> Result<PathBuf, DiscoveryError> {
    let mut repo_dir = cache_dir.to_path_buf();
    for part in repo.split('/') {
        if part.is_empty() || part == "." || part == ".." {
            continue;
        }
        repo_dir.push(part);
    }
    fs::create_dir_all(&repo_dir)
        .map_err(|err| DiscoveryError(format!("Failed to create cache dir: {err}")))?;

    if run_git(
        &["rev-parse".to_string(), "--is-inside-work-tree".to_string()],
        &repo_dir,
        envs,
        false,
    )
    .is_err()
    {
        run_git(
            &["init".to_string(), "-q".to_string()],
            &repo_dir,
            envs,
            false,
        )?;
    }

    match run_git(
        &[
            "remote".to_string(),
            "get-url".to_string(),
            "origin".to_string(),
        ],
        &repo_dir,
        envs,
        true,
    ) {
        Ok(output) => {
            let current = output.trim();
            if !current.is_empty() && current != remote_url {
                run_git(
                    &[
                        "remote".to_string(),
                        "set-url".to_string(),
                        "origin".to_string(),
                        remote_url.to_string(),
                    ],
                    &repo_dir,
                    envs,
                    false,
                )?;
            }
        }
        Err(_) => {
            run_git(
                &[
                    "remote".to_string(),
                    "add".to_string(),
                    "origin".to_string(),
                    remote_url.to_string(),
                ],
                &repo_dir,
                envs,
                false,
            )?;
        }
    }

    Ok(repo_dir)
}

fn run_git(
    args: &[String],
    cwd: &Path,
    envs: &HashMap<String, String>,
    capture_output: bool,
) -> Result<String, DiscoveryError> {
    let mut command = Command::new("git");
    command.args(args).current_dir(cwd);
    for (key, value) in envs {
        command.env(key, value);
    }
    if capture_output {
        command.stdout(Stdio::piped()).stderr(Stdio::piped());
    } else {
        command.stdout(Stdio::null()).stderr(Stdio::piped());
    }

    let output = command
        .output()
        .map_err(|err| DiscoveryError(format!("git command failed to start: {err}")))?;
    if !output.status.success() {
        let detail = String::from_utf8_lossy(&output.stderr).trim().to_string();
        let detail = if detail.is_empty() {
            "unknown git error".to_string()
        } else {
            detail
        };
        return Err(DiscoveryError(format!("git command failed: {detail}")));
    }

    if capture_output {
        Ok(String::from_utf8_lossy(&output.stdout).to_string())
    } else {
        Ok(String::new())
    }
}

fn rewrite_language_segment(
    url: &str,
    base_language: &str,
    target_language: &str,
) -> Option<String> {
    let mut parsed = Url::parse(url).ok()?;
    let mut segments: Vec<String> = parsed
        .path_segments()
        .map(|segments| segments.map(|seg| seg.to_string()).collect())
        .unwrap_or_default();
    let mut replaced = false;
    for segment in segments.iter_mut() {
        if segment.eq_ignore_ascii_case(base_language) {
            *segment = target_language.to_string();
            replaced = true;
            break;
        }
    }
    if !replaced {
        return None;
    }
    let new_path = format!("/{}", segments.join("/"));
    parsed.set_path(&new_path);
    Some(parsed.to_string())
}
