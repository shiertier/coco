//! Self-update helpers for coco-local.

use std::path::{Path, PathBuf};

use serde::Deserialize;
use tokio::fs;

use coco_protocol::{CocoError, CocoResult};

const GITHUB_API_BASE: &str = "https://api.github.com";

#[derive(Debug, Clone)]
pub struct UpdateInfo {
    pub current: String,
    pub latest: String,
    pub update_available: bool,
    pub asset: Option<ReleaseAssetInfo>,
}

#[derive(Debug, Clone)]
pub struct ReleaseAssetInfo {
    pub name: String,
    pub url: String,
}

#[derive(Debug)]
pub enum InstallOutcome {
    Replaced,
    Staged(PathBuf),
}

#[derive(Debug, Deserialize)]
struct Release {
    tag_name: String,
    assets: Vec<ReleaseAsset>,
}

#[derive(Debug, Deserialize)]
struct ReleaseAsset {
    name: String,
    browser_download_url: String,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
struct Version {
    major: u64,
    minor: u64,
    patch: u64,
}

pub async fn check_for_update(repo: &str) -> CocoResult<UpdateInfo> {
    let release = fetch_latest_release(repo).await?;
    let current = env!("CARGO_PKG_VERSION").to_string();
    let latest = release.tag_name.clone();
    let update_available = is_newer_version(&current, &latest);
    let asset = resolve_asset(&release);
    Ok(UpdateInfo {
        current,
        latest,
        update_available,
        asset,
    })
}

pub async fn download_and_install(info: &UpdateInfo) -> CocoResult<InstallOutcome> {
    if !info.update_available {
        return Err(CocoError::user("already on the latest version"));
    }
    let asset = info
        .asset
        .as_ref()
        .ok_or_else(|| CocoError::user("no release asset for this platform"))?;
    let temp_dir = prepare_temp_dir()?;
    let archive_path = temp_dir.join(&asset.name);
    download_asset(&asset.url, &archive_path).await?;
    extract_archive(&archive_path, &temp_dir)?;
    let extracted = find_extracted_binary(&temp_dir)?;
    install_binary(&extracted)
}

fn is_newer_version(current: &str, latest: &str) -> bool {
    match (parse_version(current), parse_version(latest)) {
        (Some(current), Some(latest)) => latest > current,
        _ => normalize_version(latest) != normalize_version(current),
    }
}

fn normalize_version(value: &str) -> String {
    value.trim_start_matches('v').to_string()
}

fn parse_version(value: &str) -> Option<Version> {
    let value = normalize_version(value);
    let base = value.split(['-', '+']).next().unwrap_or(&value);
    let mut parts = base.split('.');
    let major = parts.next()?.parse().ok()?;
    let minor = parts.next()?.parse().ok()?;
    let patch = parts.next()?.parse().ok()?;
    Some(Version {
        major,
        minor,
        patch,
    })
}

fn resolve_asset(release: &Release) -> Option<ReleaseAssetInfo> {
    let target = current_target()?;
    let archive_ext = if cfg!(windows) { "zip" } else { "tar.gz" };
    let name = format!("coco-local-{}-{target}.{archive_ext}", release.tag_name);
    release
        .assets
        .iter()
        .find(|asset| asset.name == name)
        .map(|asset| ReleaseAssetInfo {
            name: asset.name.clone(),
            url: asset.browser_download_url.clone(),
        })
}

fn current_target() -> Option<&'static str> {
    match (std::env::consts::OS, std::env::consts::ARCH) {
        ("linux", "x86_64") => Some("x86_64-unknown-linux-gnu"),
        ("macos", "x86_64") => Some("x86_64-apple-darwin"),
        ("macos", "aarch64") => Some("aarch64-apple-darwin"),
        ("windows", "x86_64") => Some("x86_64-pc-windows-msvc"),
        _ => None,
    }
}

async fn fetch_latest_release(repo: &str) -> CocoResult<Release> {
    let url = format!("{GITHUB_API_BASE}/repos/{repo}/releases/latest");
    let client = reqwest::Client::builder()
        .user_agent("coco-local-update")
        .build()
        .map_err(CocoError::system)?;
    let response = client
        .get(url)
        .send()
        .await
        .map_err(|err| CocoError::network(format!("request failed: {err}")))?;
    let status = response.status();
    let body = response
        .bytes()
        .await
        .map_err(|err| CocoError::network(format!("response read failed: {err}")))?;
    if !status.is_success() {
        let text = String::from_utf8_lossy(&body);
        return Err(CocoError::system(format!(
            "release lookup failed with status {}: {}",
            status.as_u16(),
            text
        )));
    }
    serde_json::from_slice(&body).map_err(CocoError::system)
}

async fn download_asset(url: &str, path: &Path) -> CocoResult<()> {
    let client = reqwest::Client::builder()
        .user_agent("coco-local-update")
        .build()
        .map_err(CocoError::system)?;
    let response = client
        .get(url)
        .send()
        .await
        .map_err(|err| CocoError::network(format!("download failed: {err}")))?;
    let status = response.status();
    let body = response
        .bytes()
        .await
        .map_err(|err| CocoError::network(format!("download read failed: {err}")))?;
    if !status.is_success() {
        let text = String::from_utf8_lossy(&body);
        return Err(CocoError::system(format!(
            "download failed with status {}: {}",
            status.as_u16(),
            text
        )));
    }
    fs::write(path, &body)
        .await
        .map_err(|err| CocoError::system(format!("write failed: {err}")))?;
    Ok(())
}

fn prepare_temp_dir() -> CocoResult<PathBuf> {
    let mut dir = std::env::temp_dir();
    let nanos = std::time::SystemTime::now()
        .duration_since(std::time::UNIX_EPOCH)
        .map_err(|err| CocoError::system(format!("clock error: {err}")))?
        .as_nanos();
    dir.push(format!("coco-update-{nanos}"));
    std::fs::create_dir_all(&dir)
        .map_err(|err| CocoError::system(format!("failed to create temp dir: {err}")))?;
    Ok(dir)
}

fn extract_archive(archive: &Path, dest: &Path) -> CocoResult<()> {
    let archive_name = archive
        .file_name()
        .and_then(|name| name.to_str())
        .unwrap_or_default();
    let tar_flag = if archive_name.ends_with(".tar.gz") {
        "-xzf"
    } else {
        "-xf"
    };
    let mut status = std::process::Command::new("tar");
    status.arg(tar_flag).arg(archive).arg("-C").arg(dest);
    let output = status
        .output()
        .map_err(|err| CocoError::system(format!("failed to run tar: {err}")))?;
    if output.status.success() {
        return Ok(());
    }
    if cfg!(windows) {
        return extract_archive_powershell(archive, dest);
    }
    Err(CocoError::system(format!(
        "archive extract failed: {}",
        String::from_utf8_lossy(&output.stderr)
    )))
}

fn extract_archive_powershell(archive: &Path, dest: &Path) -> CocoResult<()> {
    let command = format!(
        "Expand-Archive -LiteralPath '{}' -DestinationPath '{}' -Force",
        archive.display(),
        dest.display()
    );
    let output = std::process::Command::new("powershell")
        .arg("-NoProfile")
        .arg("-Command")
        .arg(command)
        .output()
        .map_err(|err| CocoError::system(format!("failed to run powershell: {err}")))?;
    if output.status.success() {
        Ok(())
    } else {
        Err(CocoError::system(format!(
            "archive extract failed: {}",
            String::from_utf8_lossy(&output.stderr)
        )))
    }
}

fn find_extracted_binary(dir: &Path) -> CocoResult<PathBuf> {
    let name = if cfg!(windows) {
        "coco-local.exe"
    } else {
        "coco-local"
    };
    let candidate = dir.join(name);
    if candidate.exists() {
        return Ok(candidate);
    }
    Err(CocoError::system(
        "extracted binary not found in update archive",
    ))
}

fn install_binary(new_binary: &Path) -> CocoResult<InstallOutcome> {
    let current = std::env::current_exe()
        .map_err(|err| CocoError::system(format!("failed to locate current binary: {err}")))?;
    if cfg!(windows) {
        let staged = current.with_extension("new.exe");
        std::fs::copy(new_binary, &staged)
            .map_err(|err| CocoError::system(format!("failed to stage update: {err}")))?;
        return Ok(InstallOutcome::Staged(staged));
    }

    let backup = current.with_extension("old");
    if backup.exists() {
        std::fs::remove_file(&backup)
            .map_err(|err| CocoError::system(format!("failed to remove backup: {err}")))?;
    }
    std::fs::rename(&current, &backup)
        .map_err(|err| CocoError::system(format!("failed to backup binary: {err}")))?;
    std::fs::rename(new_binary, &current)
        .map_err(|err| CocoError::system(format!("failed to install update: {err}")))?;
    Ok(InstallOutcome::Replaced)
}
