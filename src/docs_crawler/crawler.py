from __future__ import annotations

import logging
from concurrent.futures import ThreadPoolExecutor, as_completed
from pathlib import Path
from typing import Dict, Iterable, List, Optional, Set

from .discovery import DiscoveredDoc, discover_documents
from .http_client import FetchResult, fetch_bytes
from .models import DocumentRef, Manifest, ManifestDocument, SourceConfig
from .storage import delete_document, load_manifest, save_manifest, write_document
from .util import (
    CrawlStats,
    collapse_index_path,
    ensure_within_dir,
    iso_now,
    normalize_url,
    safe_relative_path,
    sha256_bytes,
    split_language_from_path,
)

logger = logging.getLogger(__name__)


class Crawler:
    def __init__(
        self,
        *,
        allowed_extensions: Iterable[str],
        user_agent: str,
        timeout_seconds: int,
        max_workers: int,
    ) -> None:
        self.allowed_extensions = list(allowed_extensions)
        self.user_agent = user_agent
        self.timeout_seconds = timeout_seconds
        self.max_workers = max_workers

    def crawl_source(
        self,
        source: SourceConfig,
        *,
        prune: bool = False,
        dry_run: bool = False,
    ) -> CrawlStats:
        output_root = source.output_dir
        logger.info("crawl start source=%s output=%s", source.id, output_root)
        if dry_run:
            logger.info("dry run enabled source=%s", source.id)
        if prune:
            logger.info("prune enabled source=%s", source.id)
        allowed_extensions = (
            source.allowed_extensions
            if source.allowed_extensions is not None
            else self.allowed_extensions
        )
        manifest_path = output_root / ".manifest.json"
        manifest = load_manifest(manifest_path)
        if manifest is None:
            manifest = Manifest(
                version=1,
                source_id=source.id,
                generated_at=iso_now(),
                entrypoints=[ep.__dict__ for ep in source.entrypoints],
                documents={},
            )
        self._normalize_manifest_paths(manifest, output_root)

        discovered_urls: Set[str] = set()
        discovered_language_hints: Dict[str, Set[str]] = {}
        discovered_in_order: List[DiscoveredDoc] = []
        for entrypoint in source.entrypoints:
            logger.info("discover start source=%s type=%s", source.id, entrypoint.type)
            cache_dir: Optional[Path] = None
            if entrypoint.type == "github_tree":
                cache_dir = (
                    Path(entrypoint.git_cache_dir)
                    if entrypoint.git_cache_dir
                    else source.output_dir / ".git-cache"
                )
            discovered_batch = discover_documents(
                entrypoint,
                allowed_extensions=allowed_extensions,
                timeout_seconds=self.timeout_seconds,
                user_agent=self.user_agent,
                default_language=source.default_language,
                cache_dir=cache_dir,
            )
            before = len(discovered_urls)
            for discovered in discovered_batch:
                url = normalize_url(discovered.url)
                if url not in discovered_urls:
                    discovered_urls.add(url)
                    discovered_in_order.append(
                        DiscoveredDoc(
                            url=url,
                            language_hint=discovered.language_hint,
                            relative_path=discovered.relative_path,
                        )
                    )
                if discovered.language_hint:
                    discovered_language_hints.setdefault(url, set()).add(discovered.language_hint)
            added = len(discovered_urls) - before
            logger.info("discover done source=%s added=%d total=%d", source.id, added, len(discovered_urls))

        docs = self._build_document_refs(source, discovered_in_order, discovered_language_hints)
        stats = CrawlStats(discovered=len(docs))

        results: Dict[str, FetchResult] = {}
        total_docs = len(docs)
        logger.info("fetch start source=%s total=%d workers=%d", source.id, total_docs, self.max_workers)
        with ThreadPoolExecutor(max_workers=self.max_workers) as executor:
            futures = {}
            for doc in docs:
                previous = manifest.documents.get(doc.url)
                if_none_match = None
                if_modified_since = None
                if previous is not None:
                    previous_path = output_root / previous.local_path if previous.local_path else None
                    if previous_path is not None:
                        previous_path = ensure_within_dir(output_root, previous_path)
                    target_path = ensure_within_dir(output_root, doc.local_path)
                    cache_ok = (previous_path is not None and previous_path.exists()) or target_path.exists()
                    if cache_ok:
                        if_none_match = previous.etag
                        if_modified_since = previous.last_modified

                futures[
                    executor.submit(
                        fetch_bytes,
                        doc.url,
                        timeout_seconds=self.timeout_seconds,
                        user_agent=self.user_agent,
                        if_none_match=if_none_match,
                        if_modified_since=if_modified_since,
                    )
                ] = doc

            completed = 0
            progress_every = self._progress_interval(total_docs)
            for future in as_completed(futures):
                doc = futures[future]
                try:
                    results[doc.url] = future.result()
                except Exception as exc:  # pragma: no cover
                    results[doc.url] = FetchResult(
                        url=doc.url, status=0, headers={}, body=None, error=f"Unhandled error: {exc}"
                    )
                completed += 1
                if completed % progress_every == 0 or completed == total_docs:
                    logger.info("fetch progress source=%s %d/%d", source.id, completed, total_docs)

        now = iso_now()
        for doc in docs:
            result = results.get(doc.url)
            if result is None:
                continue

            previous = manifest.documents.get(doc.url)
            if result.status == 304 and previous is not None:
                target_path = ensure_within_dir(output_root, doc.local_path)
                previous_path = output_root / previous.local_path if previous.local_path else None
                if previous_path is not None:
                    previous_path = ensure_within_dir(output_root, previous_path)

                if not target_path.exists() and previous_path is not None and previous_path.exists():
                    if not dry_run:
                        target_path.parent.mkdir(parents=True, exist_ok=True)
                        previous_path.replace(target_path)
                    previous.local_path = str(target_path.relative_to(output_root).as_posix())
                    previous.checked_at = now
                    previous.status = "moved"
                    stats = CrawlStats(
                        discovered=stats.discovered,
                        fetched=stats.fetched,
                        updated=stats.updated + 1,
                        unchanged=stats.unchanged,
                        errors=stats.errors,
                        pruned=stats.pruned,
                    )
                    continue

                if not target_path.exists():
                    refreshed = fetch_bytes(
                        doc.url,
                        timeout_seconds=self.timeout_seconds,
                        user_agent=self.user_agent,
                    )
                    result = refreshed
                else:
                    previous.checked_at = now
                    previous.status = "not_modified"
                    stats = CrawlStats(
                        discovered=stats.discovered,
                        fetched=stats.fetched,
                        updated=stats.updated,
                        unchanged=stats.unchanged + 1,
                        errors=stats.errors,
                        pruned=stats.pruned,
                    )
                    continue

                previous.checked_at = now
                previous.status = "refetched"

            if result.status != 200 or result.body is None:
                logger.warning(
                    "fetch error source=%s url=%s status=%s error=%s",
                    source.id,
                    doc.url,
                    result.status,
                    result.error or "",
                )
                entry = previous or ManifestDocument(
                    url=doc.url,
                    local_path=str(doc.local_path.relative_to(output_root).as_posix()),
                    language=doc.language,
                    sha256="",
                    bytes=0,
                )
                entry.checked_at = now
                entry.status = "error"
                entry.error = result.error or f"status={result.status}"
                manifest.documents[doc.url] = entry
                stats = CrawlStats(
                    discovered=stats.discovered,
                    fetched=stats.fetched,
                    updated=stats.updated,
                    unchanged=stats.unchanged,
                    errors=stats.errors + 1,
                    pruned=stats.pruned,
                )
                continue

            body = result.body
            sha = sha256_bytes(body)
            local_path = ensure_within_dir(output_root, doc.local_path)
            relative_local_path = str(local_path.relative_to(output_root).as_posix())

            path_changed = previous is not None and bool(previous.local_path) and previous.local_path != relative_local_path
            changed = path_changed or previous is None or previous.sha256 != sha or not local_path.exists()
            if changed and not dry_run:
                if path_changed and previous is not None and previous.local_path:
                    previous_path = ensure_within_dir(output_root, output_root / previous.local_path)
                    if previous_path.exists() and previous_path != local_path:
                        delete_document(previous_path)
                write_document(local_path, body)

            entry = ManifestDocument(
                url=doc.url,
                local_path=relative_local_path,
                language=doc.language,
                sha256=sha,
                bytes=len(body),
                etag=result.headers.get("ETag") or result.headers.get("Etag"),
                last_modified=result.headers.get("Last-Modified"),
                fetched_at=now,
                checked_at=now,
                status="updated" if changed else "unchanged",
                error=None,
            )
            manifest.documents[doc.url] = entry

            stats = CrawlStats(
                discovered=stats.discovered,
                fetched=stats.fetched + 1,
                updated=stats.updated + (1 if changed else 0),
                unchanged=stats.unchanged + (0 if changed else 1),
                errors=stats.errors,
                pruned=stats.pruned,
            )

        if prune:
            to_remove = set(manifest.documents.keys()) - discovered_urls
            for url in to_remove:
                doc = manifest.documents.get(url)
                if doc is None:
                    continue
                if not dry_run:
                    delete_document(ensure_within_dir(output_root, output_root / doc.local_path))
                manifest.documents.pop(url, None)
            stats = CrawlStats(
                discovered=stats.discovered,
                fetched=stats.fetched,
                updated=stats.updated,
                unchanged=stats.unchanged,
                errors=stats.errors,
                pruned=stats.pruned + len(to_remove),
            )
            logger.info("prune done source=%s removed=%d", source.id, len(to_remove))

        manifest.generated_at = iso_now()
        manifest.entrypoints = [ep.__dict__ for ep in source.entrypoints]
        manifest.documents_order = [doc.url for doc in docs]
        if not dry_run:
            save_manifest(manifest_path, manifest)
            if source.id == "react-aria":
                self._write_rac_note(output_root)

        return stats

    def _build_document_refs(
        self,
        source: SourceConfig,
        discovered: List[DiscoveredDoc],
        url_language_hints: Dict[str, Set[str]],
    ) -> List[DocumentRef]:
        docs: List[DocumentRef] = []
        for doc in discovered:
            rel = doc.relative_path if doc.relative_path is not None else safe_relative_path(doc.url)
            hints = url_language_hints.get(doc.url, set())
            language_hint = next(iter(hints)) if len(hints) == 1 else None
            language, rel_without_lang = split_language_from_path(
                rel, source.default_language, language_hint=language_hint
            )
            rel_without_lang = collapse_index_path(rel_without_lang)
            if rel_without_lang.suffix == "":
                rel_without_lang = rel_without_lang.with_suffix(".html")

            local_path = source.output_dir / language / rel_without_lang
            docs.append(
                DocumentRef(
                    url=doc.url,
                    language=language,
                    relative_path=rel_without_lang,
                    local_path=local_path,
                )
            )
        return docs

    @staticmethod
    def _progress_interval(total: int) -> int:
        if total <= 0:
            return 1
        return min(50, max(1, total // 10))

    @staticmethod
    def _normalize_manifest_paths(manifest: Manifest, output_root: Path) -> None:
        for doc in manifest.documents.values():
            if not doc.local_path:
                continue
            local_path = Path(doc.local_path)
            if local_path.is_absolute():
                try:
                    doc.local_path = str(local_path.relative_to(output_root.resolve()).as_posix())
                except ValueError:
                    continue
                continue
            try:
                doc.local_path = str(local_path.relative_to(output_root).as_posix())
            except ValueError:
                doc.local_path = str(local_path.as_posix())

    @staticmethod
    def _write_rac_note(output_root: Path) -> None:
        note_path = output_root.parent / "rac.md"
        note = (
            "# React Aria Components note\n\n"
            "React Aria docs include React Aria Components (RAC) content.\n"
            "Source: https://react-aria.adobe.com/llms.txt\n\n"
            "Search for \"react-aria-components\" or \"React Aria Components\" to find RAC docs.\n"
        )
        note_path.write_text(note, encoding="utf-8")
