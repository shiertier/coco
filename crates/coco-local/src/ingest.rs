//! Local ingestion pipeline shared by HTTP and filesystem watch.

use std::path::{Path, PathBuf};
use std::sync::Arc;

use chrono::Utc;
use coco_core::{
    chunking::FixedTokenSplitter, code::CodeParser, code::CodeSplitter, markdown::MarkdownParser,
    markdown::MarkdownSplitter,
};
use coco_protocol::{
    Chunk, Chunker, ChunkingStrategy, CocoError, CocoResult, DocumentId, DocumentParser,
    EmbeddingModel, FileType, StorageBackend, TextSpan,
};

use crate::embedder::LocalEmbedder;
use crate::ids::{generate_id, sha256_hex};
use crate::storage::lance::LanceStore;
use crate::storage::meta::{
    ChunkRecord, DocumentRecord, IndexingConfigRecord, LocalMetaStore, NewChunk, NewDocument,
};

/// Ingestion request payload for local mode.
#[derive(Debug, Clone)]
pub struct IngestRequest {
    /// Project identifier.
    pub project_id: String,
    /// Indexing configuration identifier.
    pub indexing_config_id: String,
    /// Optional project version identifier.
    pub version_id: Option<String>,
    /// Optional document identifier override.
    pub document_id: Option<String>,
    /// Raw document content.
    pub content: String,
    /// Optional precomputed content hash.
    pub content_hash: Option<String>,
    /// File type for parser selection.
    pub file_type: FileType,
    /// Optional title metadata.
    pub title: Option<String>,
    /// Optional absolute path for local documents.
    pub path: Option<String>,
    /// Optional chunking override.
    pub chunking: Option<ChunkingStrategy>,
}

/// Result of an ingestion operation.
#[derive(Debug, Clone)]
pub struct IngestResult {
    /// Document identifier.
    pub document_id: String,
    /// Stored chunk records.
    pub chunks: Vec<ChunkRecord>,
}

/// Local ingestion orchestrator.
#[derive(Clone)]
pub struct Ingestor {
    meta: LocalMetaStore,
    vector: LanceStore,
    embedder: Option<Arc<LocalEmbedder>>,
}

impl Ingestor {
    /// Creates a new ingestor from local resources.
    pub fn new(
        meta: LocalMetaStore,
        vector: LanceStore,
        embedder: Option<Arc<LocalEmbedder>>,
    ) -> Self {
        Self {
            meta,
            vector,
            embedder,
        }
    }

    /// Returns the metadata store handle.
    pub fn meta(&self) -> &LocalMetaStore {
        &self.meta
    }

    /// Ingests a document payload and returns stored chunk records.
    pub async fn ingest_request(&self, request: IngestRequest) -> CocoResult<IngestResult> {
        if request.project_id.trim().is_empty() {
            return Err(CocoError::user("project_id must not be empty"));
        }

        let project = self.meta.get_project(&request.project_id).await?;
        if project.is_none() {
            return Err(CocoError::user("project must be registered before import"));
        }

        let version_id = match request.version_id.as_deref() {
            Some(value) => {
                self.meta
                    .ensure_project_version_exists(&request.project_id, value)
                    .await?;
                value.to_string()
            }
            None => {
                self.meta
                    .ensure_active_version_id(&request.project_id)
                    .await?
            }
        };

        let indexing_config = load_indexing_config(&self.meta, &request.indexing_config_id).await?;
        ensure_embedding_dimensions(&indexing_config, self.embedder.as_deref())?;

        let content_hash = request
            .content_hash
            .unwrap_or_else(|| sha256_hex(request.content.as_bytes()));
        let doc_id = match request.document_id {
            Some(value) if !value.trim().is_empty() => value,
            _ => match request
                .path
                .as_deref()
                .map(str::trim)
                .filter(|value| !value.is_empty())
            {
                Some(path) => doc_id_for_path(&request.project_id, &version_id, Path::new(path))?,
                None => generate_id("doc", format!("{version_id}:{content_hash}").as_bytes()),
            },
        };
        let indexed_at = Utc::now();
        let chunking = request
            .chunking
            .unwrap_or_else(|| indexing_config.chunking.clone());
        let config_id = request.indexing_config_id;

        if let Some(existing) = self.meta.get_document(&doc_id).await? {
            let existing_config = load_indexing_config(&self.meta, &existing.config_id).await?;
            let backend = self
                .vector
                .backend_for_config(
                    &existing.config_id,
                    &existing.version_id,
                    existing_config.vector_metric,
                    existing_config.index_params.as_ref(),
                )
                .await?;
            self.meta.delete_chunks_by_doc(&doc_id).await?;
            self.meta.delete_document(&doc_id).await?;
            backend.delete_by_doc(doc_id.clone().into()).await?;
        }

        let spans = chunk_document(request.file_type, &request.content, &chunking)?;
        let chunks = build_chunks(&doc_id, &request.content, &spans, self.embedder.as_deref())?;

        let backend = self
            .vector
            .backend_for_config(
                &config_id,
                &version_id,
                indexing_config.vector_metric,
                indexing_config.index_params.as_ref(),
            )
            .await?;
        backend.upsert_chunks(&chunks).await?;

        let line_starts = line_starts(&request.content);
        let mut chunk_records = Vec::with_capacity(chunks.len());
        for chunk in &chunks {
            let (start_line, end_line) = span_line_range(&line_starts, chunk.span)?;
            chunk_records.push(NewChunk {
                id: chunk.id.to_string(),
                doc_id: doc_id.clone(),
                version_id: version_id.clone(),
                config_id: config_id.clone(),
                content: chunk.content.clone(),
                start_line,
                end_line,
            });
        }

        self.meta
            .create_document(NewDocument {
                id: doc_id.clone(),
                project_id: request.project_id.clone(),
                version_id: version_id.clone(),
                config_id: config_id.clone(),
                path: request.path.unwrap_or_default(),
                title: request.title.clone(),
                content_hash,
                indexed_at,
            })
            .await?;

        let stored_chunks = self.meta.create_chunks(chunk_records).await?;

        Ok(IngestResult {
            document_id: doc_id,
            chunks: stored_chunks,
        })
    }

    /// Finds a document by project and path.
    pub async fn document_by_path(
        &self,
        project_id: &str,
        path: &str,
    ) -> CocoResult<Option<DocumentRecord>> {
        let version_id = self.meta.ensure_active_version_id(project_id).await?;
        self.meta
            .get_document_by_path(project_id, &version_id, path)
            .await
    }

    /// Removes a document by project and path, returning whether one was deleted.
    pub async fn delete_by_path(&self, project_id: &str, path: &str) -> CocoResult<bool> {
        let version_id = self.meta.ensure_active_version_id(project_id).await?;
        if let Some(doc) = self
            .meta
            .get_document_by_path(project_id, &version_id, path)
            .await?
        {
            let config = load_indexing_config(&self.meta, &doc.config_id).await?;
            let backend = self
                .vector
                .backend_for_config(
                    &doc.config_id,
                    &doc.version_id,
                    config.vector_metric,
                    config.index_params.as_ref(),
                )
                .await?;
            self.meta.delete_chunks_by_doc(&doc.id).await?;
            self.meta.delete_document(&doc.id).await?;
            backend.delete_by_doc(doc.id.clone().into()).await?;
            return Ok(true);
        }
        Ok(false)
    }
}

/// Infers a file type from a filesystem path.
pub fn file_type_for_path(path: &Path) -> FileType {
    let extension = path
        .extension()
        .and_then(|ext| ext.to_str())
        .unwrap_or_default()
        .to_ascii_lowercase();
    match extension.as_str() {
        "md" | "markdown" => FileType::Markdown,
        "rs" => FileType::Rust,
        "py" => FileType::Python,
        "ts" | "tsx" => FileType::TypeScript,
        "go" => FileType::Go,
        "txt" => FileType::PlainText,
        _ => FileType::Other,
    }
}

/// Generates a stable document id for a project version and path.
pub fn doc_id_for_path(project_id: &str, version_id: &str, path: &Path) -> CocoResult<String> {
    let path_str = path_string(path)?;
    Ok(generate_id(
        "doc",
        format!("{project_id}:{version_id}:{path_str}").as_bytes(),
    ))
}

/// Returns a display title derived from a path.
pub fn title_for_path(path: &Path) -> Option<String> {
    path.file_name()
        .and_then(|name| name.to_str())
        .map(|name| name.to_string())
}

fn path_string(path: &Path) -> CocoResult<String> {
    path.to_str()
        .map(|value| value.to_string())
        .ok_or_else(|| CocoError::user("path contains invalid characters"))
}

fn chunk_document(
    file_type: FileType,
    content: &str,
    chunking: &ChunkingStrategy,
) -> CocoResult<Vec<TextSpan>> {
    match file_type {
        FileType::Markdown => {
            let parser = MarkdownParser::new();
            let splitter = MarkdownSplitter::new();
            let parsed = parser.parse(content, file_type)?;
            splitter.chunk(&parsed, chunking)
        }
        FileType::Rust | FileType::Python | FileType::TypeScript | FileType::Go => {
            let parser = CodeParser::new();
            let splitter = CodeSplitter::new();
            let parsed = parser.parse(content, file_type)?;
            splitter.chunk(&parsed, chunking)
        }
        _ => {
            let parsed = coco_protocol::ParsedDocument {
                content: content.to_string(),
                file_type,
            };
            let splitter = FixedTokenSplitter::new();
            splitter.chunk(&parsed, chunking)
        }
    }
}

fn build_chunks(
    doc_id: &str,
    content: &str,
    spans: &[TextSpan],
    embedder: Option<&LocalEmbedder>,
) -> CocoResult<Vec<Chunk>> {
    if spans.is_empty() {
        return Ok(Vec::new());
    }
    let embedder = embedder.ok_or_else(|| CocoError::user("embedding model not configured"))?;
    let mut chunk_texts = Vec::with_capacity(spans.len());
    for span in spans {
        let slice = content
            .get(span.start()..span.end())
            .ok_or_else(|| CocoError::compute("invalid chunk boundaries"))?;
        chunk_texts.push(slice.to_string());
    }

    let text_refs: Vec<&str> = chunk_texts.iter().map(|text| text.as_str()).collect();
    let embeddings = embedder.embed(&text_refs)?;

    if embeddings.len() != spans.len() {
        return Err(CocoError::compute("embedding output size mismatch"));
    }

    let mut chunks = Vec::with_capacity(spans.len());
    for (index, span) in spans.iter().enumerate() {
        let chunk_id = generate_id(
            "chunk",
            format!("{doc_id}:{}:{}", span.start(), span.end()).as_bytes(),
        );
        chunks.push(Chunk {
            id: chunk_id.into(),
            doc_id: DocumentId::new(doc_id),
            content: chunk_texts[index].clone(),
            embedding: Some(embeddings[index].clone()),
            span: *span,
            quality_score: None,
            verified: None,
        });
    }
    Ok(chunks)
}

async fn load_indexing_config(
    meta: &LocalMetaStore,
    config_id: &str,
) -> CocoResult<IndexingConfigRecord> {
    meta.get_indexing_config(config_id)
        .await?
        .ok_or_else(|| CocoError::user("indexing config not found"))
}

fn ensure_embedding_dimensions(
    config: &IndexingConfigRecord,
    embedder: Option<&LocalEmbedder>,
) -> CocoResult<()> {
    let Some(embedder) = embedder else {
        return Ok(());
    };
    let Some(expected) = config.embedding.dimensions else {
        return Ok(());
    };
    if embedder.dimensions() != expected as usize {
        return Err(CocoError::user(
            "embedding dimensions do not match indexing config",
        ));
    }
    Ok(())
}

fn line_starts(content: &str) -> Vec<usize> {
    let mut starts = vec![0];
    for (index, byte) in content.as_bytes().iter().enumerate() {
        if *byte == b'\n' {
            let next = index.saturating_add(1);
            if next < content.len() {
                starts.push(next);
            }
        }
    }
    starts
}

fn span_line_range(line_starts: &[usize], span: TextSpan) -> CocoResult<(i32, i32)> {
    if line_starts.is_empty() {
        return Ok((1, 1));
    }
    let start_line = line_for_offset(line_starts, span.start())?;
    let end_offset = span.end().saturating_sub(1);
    let end_line = line_for_offset(line_starts, end_offset)?;
    Ok((start_line, end_line))
}

fn line_for_offset(line_starts: &[usize], offset: usize) -> CocoResult<i32> {
    let mut low = 0usize;
    let mut high = line_starts.len();
    while low < high {
        let mid = (low + high) / 2;
        if line_starts[mid] <= offset {
            low = mid + 1;
        } else {
            high = mid;
        }
    }
    let line = low.max(1);
    i32::try_from(line).map_err(|_| CocoError::user("line number out of range"))
}

/// Normalizes a filesystem path for storage.
pub fn normalize_path(path: &Path) -> CocoResult<PathBuf> {
    match std::fs::canonicalize(path) {
        Ok(path) => Ok(path),
        Err(_) => Ok(path.to_path_buf()),
    }
}
