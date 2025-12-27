//! Filesystem helpers for local mode.

use std::path::{Path, PathBuf};

use coco_protocol::{CocoError, CocoResult};

const IGNORED_DIRS: &[&str] = &[
    ".git",
    "node_modules",
    "target",
    ".coco",
    ".idea",
    ".vscode",
];

/// Returns true if the path should be ignored.
pub fn should_ignore_path(path: &Path) -> bool {
    path.components().any(|component| {
        component
            .as_os_str()
            .to_str()
            .map(|name| IGNORED_DIRS.contains(&name))
            .unwrap_or(false)
    })
}

/// Collects files under a root path, optionally recursively.
pub fn collect_files(root: &Path, recursive: bool) -> CocoResult<Vec<PathBuf>> {
    let mut output = Vec::new();
    if should_ignore_path(root) {
        return Ok(output);
    }
    if root.is_file() {
        output.push(root.to_path_buf());
        return Ok(output);
    }
    if !root.is_dir() {
        return Err(CocoError::user("path must be a file or directory"));
    }
    if recursive {
        collect_recursive(root, &mut output)?;
    } else {
        collect_shallow(root, &mut output)?;
    }
    Ok(output)
}

/// Converts a path to a UTF-8 string for storage.
pub fn path_to_string(path: &Path) -> CocoResult<String> {
    path.to_str()
        .map(|value| value.to_string())
        .ok_or_else(|| CocoError::user("path contains invalid characters"))
}

fn collect_shallow(root: &Path, output: &mut Vec<PathBuf>) -> CocoResult<()> {
    let entries = std::fs::read_dir(root).map_err(|err| {
        CocoError::system(format!("failed to read directory {}: {err}", root.display()))
    })?;
    for entry in entries {
        let entry = entry.map_err(CocoError::system)?;
        let path = entry.path();
        if should_ignore_path(&path) {
            continue;
        }
        if entry
            .metadata()
            .map_err(CocoError::system)?
            .is_file()
        {
            output.push(path);
        }
    }
    Ok(())
}

fn collect_recursive(root: &Path, output: &mut Vec<PathBuf>) -> CocoResult<()> {
    let entries = std::fs::read_dir(root).map_err(|err| {
        CocoError::system(format!("failed to read directory {}: {err}", root.display()))
    })?;
    for entry in entries {
        let entry = entry.map_err(CocoError::system)?;
        let path = entry.path();
        if should_ignore_path(&path) {
            continue;
        }
        let metadata = entry.metadata().map_err(CocoError::system)?;
        if metadata.is_dir() {
            collect_recursive(&path, output)?;
        } else if metadata.is_file() {
            output.push(path);
        }
    }
    Ok(())
}
