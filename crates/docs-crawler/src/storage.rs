use crate::models::Manifest;
use std::fs;
use std::path::Path;

pub fn load_manifest(path: &Path) -> Result<Option<Manifest>, String> {
    if !path.exists() {
        return Ok(None);
    }
    let raw = fs::read_to_string(path)
        .map_err(|err| format!("Failed to read manifest {}: {err}", path.display()))?;
    let mut manifest: Manifest = serde_json::from_str(&raw)
        .map_err(|err| format!("Invalid manifest format {}: {err}", path.display()))?;

    for (url, doc) in manifest.documents.iter_mut() {
        if doc.url.is_empty() {
            doc.url = url.to_string();
        }
    }

    Ok(Some(manifest))
}

pub fn save_manifest(path: &Path, manifest: &Manifest) -> Result<(), String> {
    if let Some(parent) = path.parent() {
        fs::create_dir_all(parent)
            .map_err(|err| format!("Failed to create manifest directory: {err}"))?;
    }
    let payload = serde_json::to_string_pretty(manifest)
        .map_err(|err| format!("Failed to serialize manifest: {err}"))?;
    fs::write(path, format!("{}\n", payload))
        .map_err(|err| format!("Failed to write manifest {}: {err}", path.display()))?;
    Ok(())
}

pub fn write_document(path: &Path, content: &[u8]) -> Result<(), String> {
    if let Some(parent) = path.parent() {
        fs::create_dir_all(parent)
            .map_err(|err| format!("Failed to create document directory: {err}"))?;
    }
    fs::write(path, content)
        .map_err(|err| format!("Failed to write document {}: {err}", path.display()))?;
    Ok(())
}

pub fn delete_document(path: &Path) -> Result<(), String> {
    match fs::remove_file(path) {
        Ok(_) => Ok(()),
        Err(err) if err.kind() == std::io::ErrorKind::NotFound => Ok(()),
        Err(err) => Err(format!("Failed to delete {}: {err}", path.display())),
    }
}
