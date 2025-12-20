from __future__ import annotations

from dataclasses import dataclass, field
from pathlib import Path
from typing import Any, Dict, List, Optional


@dataclass(frozen=True)
class DefaultsConfig:
    allowed_extensions: List[str] = field(default_factory=lambda: [".md", ".mdx"])
    user_agent: str = "docs-crawler/0.1"
    timeout_seconds: int = 30
    max_workers: int = 8


@dataclass(frozen=True)
class EntryPointConfig:
    type: str
    url: Optional[str] = None
    language: Optional[str] = None
    repo: Optional[str] = None
    ref: Optional[str] = None
    path: Optional[str] = None
    languages: Optional[List[str]] = None
    api_base_url: Optional[str] = None
    raw_base_url: Optional[str] = None
    language_variants: Optional[List[str]] = None
    git_cache_dir: Optional[str] = None


@dataclass(frozen=True)
class SourceConfig:
    id: str
    description: Optional[str]
    output_dir: Path
    default_language: str
    entrypoints: List[EntryPointConfig]
    allowed_extensions: Optional[List[str]] = None
    max_workers: Optional[int] = None


@dataclass(frozen=True)
class CrawlerConfig:
    version: int
    defaults: DefaultsConfig
    sources: List[SourceConfig]


@dataclass(frozen=True)
class DocumentRef:
    url: str
    language: str
    relative_path: Path
    local_path: Path


@dataclass
class ManifestDocument:
    url: str
    local_path: str
    language: str
    sha256: str
    bytes: int
    etag: Optional[str] = None
    last_modified: Optional[str] = None
    fetched_at: Optional[str] = None
    checked_at: Optional[str] = None
    status: Optional[str] = None
    error: Optional[str] = None


@dataclass
class Manifest:
    version: int
    source_id: str
    generated_at: str
    entrypoints: List[Dict[str, Any]]
    documents: Dict[str, ManifestDocument]
    documents_order: List[str] = field(default_factory=list)
