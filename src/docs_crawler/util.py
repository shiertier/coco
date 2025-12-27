from __future__ import annotations

import hashlib
import re
from dataclasses import dataclass
from datetime import datetime, timezone
from pathlib import Path
from typing import Iterable, List, Optional, Tuple
from urllib.parse import urljoin, urlparse, urlunparse


LANG_SEGMENT_RE = re.compile(r"^[a-z]{2}(?:-[a-z0-9]{2,8})?$", re.IGNORECASE)


def iso_now() -> str:
    return datetime.now(timezone.utc).isoformat()


def normalize_url(url: str) -> str:
    parsed = urlparse(url)
    parsed = parsed._replace(fragment="")
    return urlunparse(parsed)


def url_has_allowed_extension(url: str, allowed_extensions: Iterable[str]) -> bool:
    if "*" in allowed_extensions:
        return True
    path = urlparse(url).path.lower()
    return any(path.endswith(ext.lower()) for ext in allowed_extensions)


def join_url(base: str, maybe_relative: str) -> str:
    return normalize_url(urljoin(base, maybe_relative))


def split_language_from_path(
    path: Path, default_language: str, language_hint: Optional[str] = None
) -> Tuple[str, Path]:
    parts = path.parts
    hint = language_hint.strip().lower() if language_hint and language_hint.strip() else None
    if parts and LANG_SEGMENT_RE.match(parts[0]):
        return parts[0].lower(), Path(*parts[1:]) if len(parts) > 1 else Path("index.md")
    if hint and len(parts) > 1 and parts[0].lower() == "docs" and LANG_SEGMENT_RE.match(parts[1]):
        if parts[1].lower() == hint:
            remainder = Path(parts[0], *parts[2:]) if len(parts) > 2 else Path(parts[0]) / "index.md"
            return parts[1].lower(), remainder
    if hint:
        return hint, path
    return default_language, path


def safe_relative_path(url: str) -> Path:
    raw_path = urlparse(url).path
    raw_path = raw_path.lstrip("/")
    if not raw_path:
        return Path("index.md")
    parts: List[str] = []
    for part in raw_path.split("/"):
        if part in ("", "."):
            continue
        if part == "..":
            continue
        parts.append(part)
    if not parts:
        return Path("index.md")
    return Path(*parts)


def collapse_index_path(path: Path) -> Path:
    name = path.name.lower()
    if name in ("index.md", "index.mdx") and path.parent != Path("."):
        return path.parent.with_suffix(path.suffix)
    return path


def sha256_bytes(data: bytes) -> str:
    return hashlib.sha256(data).hexdigest()


def ensure_within_dir(root: Path, path: Path) -> Path:
    root_resolved = root.resolve()
    path_resolved = path.resolve()
    try:
        path_resolved.relative_to(root_resolved)
    except ValueError as exc:
        raise ValueError(f"Refusing to write outside root: {path}") from exc
    return path


@dataclass(frozen=True)
class CrawlStats:
    discovered: int = 0
    fetched: int = 0
    updated: int = 0
    unchanged: int = 0
    errors: int = 0
    pruned: int = 0
