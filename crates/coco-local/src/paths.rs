//! Shared filesystem paths for local mode.

use std::path::PathBuf;

use coco_protocol::{CocoError, CocoResult};
use tracing::warn;

/// Returns the base data directory (default: ~/.coco).
pub fn data_dir() -> CocoResult<PathBuf> {
    let home = std::env::var("HOME")
        .or_else(|_| std::env::var("USERPROFILE"))
        .map_err(|_| CocoError::system("failed to resolve home directory"))?;
    Ok(PathBuf::from(home).join(".coco"))
}

/// Returns the service lock file path.
pub fn lock_path() -> CocoResult<PathBuf> {
    Ok(data_dir()?.join("service.lock"))
}

/// Returns the SQLite metadata database path.
pub fn meta_db_path() -> CocoResult<PathBuf> {
    let primary = std::env::var("COCO_META_DB")
        .ok()
        .filter(|value| !value.is_empty());
    let legacy = std::env::var("COCO_DB_URL")
        .ok()
        .filter(|value| !value.is_empty());
    let selected = match (primary, legacy) {
        (Some(primary), Some(legacy)) => {
            if primary != legacy {
                warn!("COCO_META_DB and COCO_DB_URL are both set; using COCO_META_DB");
            }
            Some(primary)
        }
        (Some(primary), None) => Some(primary),
        (None, Some(legacy)) => Some(legacy),
        (None, None) => None,
    };
    if let Some(value) = selected {
        if value.starts_with("sqlite:") {
            return sqlite_path_from_url(&value).ok_or_else(|| {
                CocoError::user("COCO_META_DB must be a sqlite file path or sqlite URL")
            });
        }
        return Ok(PathBuf::from(value));
    }
    Ok(data_dir()?.join("meta.db"))
}

fn sqlite_path_from_url(value: &str) -> Option<PathBuf> {
    let value = value.strip_prefix("sqlite:")?;
    let value = value.trim_start_matches("//");
    let path_part = value.split('?').next().unwrap_or("");
    if path_part.is_empty() {
        return None;
    }
    Some(PathBuf::from(path_part))
}
