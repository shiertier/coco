//! Local WASM grammar store and loader.

use std::collections::HashMap;
use std::fs::File;
use std::io::{Read, Write};
use std::path::{Path, PathBuf};

use coco_core::wasm::{WasmGrammar, WasmRuntimeConfig};
use coco_protocol::{CocoError, CocoResult};
use reqwest::blocking::Client;
use sha2::{Digest, Sha256};

/// Grammar download specification.
#[derive(Debug, Clone)]
pub struct GrammarSpec {
    /// Human-readable language name.
    pub name: String,
    /// Grammar file name inside the local store.
    pub file_name: String,
    /// Expected SHA-256 checksum (hex).
    pub sha256: Option<String>,
    /// Download URL for the grammar.
    pub url: Option<String>,
    /// Optional export function name for the grammar.
    pub language_export: Option<String>,
}

/// Manages local Tree-sitter WASM grammar files.
#[derive(Debug, Clone)]
pub struct GrammarStore {
    root: PathBuf,
}

impl GrammarStore {
    /// Opens a grammar store at the given root path or the default location.
    pub fn open(root: Option<PathBuf>) -> CocoResult<Self> {
        let root = match root {
            Some(path) => path,
            None => default_grammar_root()?,
        };

        if !root.exists() {
            std::fs::create_dir_all(&root).map_err(|err| {
                CocoError::system(format!("failed to create grammar dir: {err}"))
            })?;
        }

        Ok(Self { root })
    }

    /// Returns the full path for a grammar filename inside the store.
    pub fn grammar_path(&self, file_name: &str) -> PathBuf {
        self.root.join(file_name)
    }

    /// Ensures the grammar exists and returns its path.
    pub fn ensure_exists(&self, file_name: &str) -> CocoResult<PathBuf> {
        let path = self.grammar_path(file_name);
        if path.exists() {
            return Ok(path);
        }
        Err(CocoError::user("grammar file missing in local store"))
    }

    /// Ensures a grammar is present locally, downloading it if needed.
    pub fn ensure_grammar(
        &self,
        spec: &GrammarSpec,
        override_url: Option<&str>,
    ) -> CocoResult<PathBuf> {
        let path = self.grammar_path(&spec.file_name);
        if path.exists() {
            if let Some(expected) = spec.sha256.as_deref() {
                verify_checksum(&path, expected)?;
            }
            return Ok(path);
        }

        let url = override_url
            .or(spec.url.as_deref())
            .ok_or_else(|| CocoError::user("grammar download url not provided"))?;
        let temp_path = path.with_extension("download");
        download_file(url, &temp_path)?;

        if let Some(expected) = spec.sha256.as_deref() {
            verify_checksum(&temp_path, expected)?;
        }

        std::fs::rename(&temp_path, &path).map_err(|err| {
            CocoError::system(format!("failed to finalize grammar download: {err}"))
        })?;

        Ok(path)
    }

    /// Loads a grammar from disk and instantiates it with wasmtime.
    pub fn load(
        &self,
        spec: &GrammarSpec,
        override_url: Option<&str>,
        runtime: Option<WasmRuntimeConfig>,
    ) -> CocoResult<WasmGrammar> {
        let path = self.ensure_grammar(spec, override_url)?;
        let bytes = std::fs::read(&path)
            .map_err(|err| CocoError::system(format!("failed to read grammar: {err}")))?;
        let mut runtime = runtime.unwrap_or_default();
        if runtime.language_export.is_none() {
            runtime.language_export = spec.language_export.clone();
        }
        WasmGrammar::from_bytes(&bytes, runtime)
    }

    /// Lists installed grammar files.
    pub fn list_installed(&self) -> CocoResult<Vec<String>> {
        let mut items = Vec::new();
        if !self.root.exists() {
            return Ok(items);
        }
        let entries = std::fs::read_dir(&self.root).map_err(|err| {
            CocoError::system(format!("failed to read grammar dir: {err}"))
        })?;
        for entry in entries {
            let entry = entry.map_err(CocoError::system)?;
            let path = entry.path();
            if path
                .extension()
                .and_then(|ext| ext.to_str())
                .map(|ext| ext.eq_ignore_ascii_case("wasm"))
                != Some(true)
            {
                continue;
            }
            if let Some(name) = path.file_name().and_then(|name| name.to_str()) {
                items.push(name.to_string());
            }
        }
        items.sort();
        Ok(items)
    }
}

/// Registry for mapping file extensions to grammar specs.
#[derive(Debug, Default, Clone)]
pub struct GrammarCatalog {
    languages: HashMap<String, GrammarSpec>,
    extensions: HashMap<String, String>,
}

impl GrammarCatalog {
    /// Creates an empty catalog.
    pub fn new() -> Self {
        Self::default()
    }

    /// Registers a grammar spec for a language key.
    pub fn register_language(&mut self, key: impl Into<String>, spec: GrammarSpec) {
        self.languages.insert(key.into(), spec);
    }

    /// Registers a file extension mapping.
    pub fn register_extension(&mut self, extension: impl Into<String>, language: impl Into<String>) {
        let extension = normalize_extension(&extension.into());
        self.extensions.insert(extension, language.into());
    }

    /// Returns a grammar spec by language key.
    pub fn spec_for_language(&self, key: &str) -> Option<&GrammarSpec> {
        self.languages.get(key)
    }

    /// Returns a grammar spec by file extension.
    pub fn spec_for_extension(&self, extension: &str) -> Option<&GrammarSpec> {
        let extension = normalize_extension(extension);
        self.extensions
            .get(extension.as_str())
            .and_then(|key| self.languages.get(key))
    }
}

fn normalize_extension(extension: &str) -> String {
    extension.trim_start_matches('.').to_ascii_lowercase()
}

fn default_grammar_root() -> CocoResult<PathBuf> {
    let home = std::env::var("HOME")
        .or_else(|_| std::env::var("USERPROFILE"))
        .map_err(|_| CocoError::system("failed to resolve home directory"))?;
    Ok(PathBuf::from(home).join(".coco").join("grammars"))
}

fn download_file(url: &str, dest: &Path) -> CocoResult<()> {
    let client = Client::builder()
        .build()
        .map_err(|err| CocoError::network(format!("failed to build http client: {err}")))?;

    let mut response = client
        .get(url)
        .send()
        .map_err(|err| CocoError::network(format!("download request failed: {err}")))?;

    if !response.status().is_success() {
        return Err(CocoError::network(format!(
            "download failed with status {}",
            response.status()
        )));
    }

    let mut file = File::create(dest)
        .map_err(|err| CocoError::system(format!("failed to create grammar file: {err}")))?;

    let mut buffer = [0u8; 16 * 1024];
    loop {
        let read = response
            .read(&mut buffer)
            .map_err(|err| CocoError::network(format!("download read failed: {err}")))?;
        if read == 0 {
            break;
        }
        file.write_all(&buffer[..read]).map_err(|err| {
            CocoError::system(format!("failed to write grammar file: {err}"))
        })?;
    }

    Ok(())
}

fn verify_checksum(path: &Path, expected: &str) -> CocoResult<()> {
    let actual = file_sha256(path)?;
    if actual != expected {
        return Err(CocoError::user("grammar checksum mismatch"));
    }
    Ok(())
}

fn file_sha256(path: &Path) -> CocoResult<String> {
    let mut file = File::open(path)
        .map_err(|err| CocoError::system(format!("failed to open grammar file: {err}")))?;
    let mut hasher = Sha256::new();
    let mut buffer = [0u8; 16 * 1024];
    loop {
        let read = file
            .read(&mut buffer)
            .map_err(|err| CocoError::system(format!("failed to read grammar file: {err}")))?;
        if read == 0 {
            break;
        }
        hasher.update(&buffer[..read]);
    }
    Ok(bytes_to_hex(&hasher.finalize()))
}

fn bytes_to_hex(bytes: &[u8]) -> String {
    let mut output = String::with_capacity(bytes.len() * 2);
    for byte in bytes {
        output.push(hex_digit(byte >> 4));
        output.push(hex_digit(byte & 0x0f));
    }
    output
}

fn hex_digit(value: u8) -> char {
    const HEX: &[u8; 16] = b"0123456789abcdef";
    HEX[value as usize] as char
}
