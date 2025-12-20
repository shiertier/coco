import json
import os
import re
import shutil
import subprocess
import tempfile
from dataclasses import dataclass
from pathlib import Path, PurePosixPath
from typing import Iterable, List, Optional, Set
from urllib.parse import quote, urlparse, urlunparse

from .http_client import fetch_bytes, fetch_text
from .models import EntryPointConfig
from .util import (
    join_url,
    normalize_url,
    split_language_from_path,
    url_has_allowed_extension,
)


ORDERED_LINK_RE = re.compile(r"\[[^\]]*\]\(([^)]+)\)|https?://[^\s)]+")
GITHUB_API_BASE_URL = "https://api.github.com"
GITHUB_RAW_BASE_URL = "https://raw.githubusercontent.com"


class DiscoveryError(RuntimeError):
    pass


@dataclass(frozen=True)
class DiscoveredDoc:
    url: str
    language_hint: Optional[str] = None
    relative_path: Optional[PurePosixPath] = None


def discover_documents(
    entrypoint: EntryPointConfig,
    *,
    allowed_extensions: Iterable[str],
    timeout_seconds: int,
    user_agent: str,
    default_language: str,
    cache_dir: Optional[os.PathLike] = None,
) -> List[DiscoveredDoc]:
    if entrypoint.type == "llms_txt":
        return _discover_llms_txt(
            entrypoint,
            allowed_extensions=allowed_extensions,
            timeout_seconds=timeout_seconds,
            user_agent=user_agent,
        )
    if entrypoint.type == "github_tree":
        return _discover_github_tree(
            entrypoint,
            allowed_extensions=allowed_extensions,
            timeout_seconds=timeout_seconds,
            user_agent=user_agent,
            default_language=default_language,
            cache_dir=cache_dir,
        )
    raise DiscoveryError(f"Unsupported entrypoint type: {entrypoint.type}")


def _discover_llms_txt(
    entrypoint: EntryPointConfig,
    *,
    allowed_extensions: Iterable[str],
    timeout_seconds: int,
    user_agent: str,
) -> List[DiscoveredDoc]:
    if not entrypoint.url:
        raise DiscoveryError("llms_txt entrypoint requires url")

    text = fetch_text(entrypoint.url, timeout_seconds=timeout_seconds, user_agent=user_agent)
    urls: Set[str] = set()
    ordered: List[DiscoveredDoc] = []
    base_language = entrypoint.language.strip() if entrypoint.language else None
    language_hint = base_language.lower() if base_language else None
    variant_languages = entrypoint.language_variants or None
    if variant_languages and not base_language:
        raise DiscoveryError("language_variants requires entrypoint.language")

    for line in text.splitlines():
        match = ORDERED_LINK_RE.search(line)
        if match is None:
            continue
        dest = match.group(1) if match.group(1) is not None else match.group(0)
        dest = dest.strip()
        if not dest or dest.startswith("#"):
            continue
        if match.group(1) is not None:
            dest = dest.split()[0].strip().strip("\"'")  # drop optional title
            if not dest or dest.startswith("#"):
                continue
            abs_url = join_url(entrypoint.url, dest)
        else:
            abs_url = normalize_url(dest)

        parsed = urlparse(abs_url)
        if parsed.scheme and parsed.scheme not in ("http", "https"):
            continue
        if not url_has_allowed_extension(abs_url, allowed_extensions):
            continue

        abs_url = normalize_url(abs_url)
        if variant_languages:
            if abs_url not in urls:
                urls.add(abs_url)
                ordered.append(DiscoveredDoc(url=abs_url, language_hint=language_hint))
            for variant in variant_languages:
                if not variant or not variant.strip():
                    continue
                variant_clean = variant.strip()
                if variant_clean.lower() == base_language.lower():
                    continue
                variant_url = _rewrite_language_segment(abs_url, base_language, variant_clean)
                if not variant_url:
                    continue
                variant_url = normalize_url(variant_url)
                if variant_url in urls:
                    continue
                urls.add(variant_url)
                ordered.append(DiscoveredDoc(url=variant_url, language_hint=variant_clean.lower()))
            continue

        if abs_url in urls:
            continue
        urls.add(abs_url)
        ordered.append(DiscoveredDoc(url=abs_url, language_hint=language_hint))

    return ordered


def _discover_github_tree(
    entrypoint: EntryPointConfig,
    *,
    allowed_extensions: Iterable[str],
    timeout_seconds: int,
    user_agent: str,
    default_language: str,
    cache_dir: Optional[os.PathLike],
) -> List[DiscoveredDoc]:
    if not entrypoint.repo or not entrypoint.path:
        raise DiscoveryError("github_tree entrypoint requires repo and path")

    repo = entrypoint.repo.strip().strip("/")
    ref = entrypoint.ref.strip() if entrypoint.ref else "main"
    base_path = entrypoint.path.strip().strip("/")
    api_base_url = (
        entrypoint.api_base_url.strip().rstrip("/") if entrypoint.api_base_url else GITHUB_API_BASE_URL
    )
    raw_base_url = (
        entrypoint.raw_base_url.strip().rstrip("/") if entrypoint.raw_base_url else GITHUB_RAW_BASE_URL
    )
    api_ref = quote(ref, safe="")
    api_url = f"{api_base_url}/repos/{repo}/git/trees/{api_ref}?recursive=1"

    try:
        payload = _fetch_json(api_url, timeout_seconds=timeout_seconds, user_agent=user_agent)
        if payload.get("truncated"):
            raise DiscoveryError(f"GitHub tree response truncated for {repo}@{ref}")

        tree = payload.get("tree")
        if not isinstance(tree, list):
            raise DiscoveryError("GitHub tree response missing tree list")
        tree_paths = [item.get("path") for item in tree if isinstance(item, dict) and item.get("type") == "blob"]
    except DiscoveryError as exc:
        try:
            tree_paths = _list_repo_paths_via_git(repo, ref, cache_dir=cache_dir)
        except DiscoveryError as git_exc:
            raise DiscoveryError(f"{exc}; git fallback failed: {git_exc}") from git_exc

    base_language = entrypoint.language.strip().lower() if entrypoint.language else default_language.strip().lower()
    allow_languages = {lang.strip().lower() for lang in entrypoint.languages or [] if lang.strip()}

    ordered: List[DiscoveredDoc] = []
    seen: Set[str] = set()
    for item_path in tree_paths:
        if not item_path or not isinstance(item_path, str):
            continue

        if base_path:
            if item_path == base_path:
                rel_path = PurePosixPath(PurePosixPath(base_path).name)
            elif item_path.startswith(f"{base_path}/"):
                rel_path = PurePosixPath(item_path[len(base_path) + 1 :])
            else:
                continue
        else:
            rel_path = PurePosixPath(item_path)

        raw_url = f"{raw_base_url}/{repo}/{ref}/{quote(item_path, safe='/')}"
        if not url_has_allowed_extension(raw_url, allowed_extensions):
            continue

        if allow_languages:
            language, _ = split_language_from_path(rel_path, base_language)
            if language.lower() not in allow_languages:
                continue

        if raw_url in seen:
            continue
        seen.add(raw_url)
        ordered.append(
            DiscoveredDoc(
                url=raw_url,
                language_hint=base_language,
                relative_path=rel_path,
            )
        )

    ordered.sort(key=lambda doc: doc.relative_path.as_posix() if doc.relative_path else doc.url)
    return ordered


def _list_repo_paths_via_git(
    repo: str,
    ref: str,
    *,
    cache_dir: Optional[os.PathLike] = None,
) -> List[str]:
    if shutil.which("git") is None:
        raise DiscoveryError("git is not available for github_tree fallback")

    remote_url = f"https://github.com/{repo}.git"
    env = dict(os.environ)
    env.setdefault("GIT_TERMINAL_PROMPT", "0")
    env.setdefault("GIT_HTTP_VERSION", "HTTP/1.1")
    token = os.environ.get("GITHUB_TOKEN") or os.environ.get("GH_TOKEN")
    fetch_cmd = ["git"]
    if token:
        fetch_cmd.extend(["-c", f"http.extraHeader=Authorization: Bearer {token}"])
    fetch_cmd.extend(["fetch", "--depth", "1", "origin", ref])

    if cache_dir is None:
        with tempfile.TemporaryDirectory(prefix="docs-crawler-git-") as tmpdir:
            _run_git(["git", "init", "-q"], cwd=tmpdir, env=env)
            _run_git(["git", "remote", "add", "origin", remote_url], cwd=tmpdir, env=env)
            _run_git(fetch_cmd, cwd=tmpdir, env=env)
            result = _run_git(
                ["git", "ls-tree", "-r", "--name-only", "FETCH_HEAD"],
                cwd=tmpdir,
                env=env,
                capture_output=True,
            )
        return [line.strip() for line in result.stdout.splitlines() if line.strip()]

    repo_dir = _prepare_git_cache_dir(cache_dir, repo, remote_url, env)
    _run_git(fetch_cmd, cwd=str(repo_dir), env=env)
    result = _run_git(
        ["git", "ls-tree", "-r", "--name-only", "FETCH_HEAD"],
        cwd=str(repo_dir),
        env=env,
        capture_output=True,
    )
    return [line.strip() for line in result.stdout.splitlines() if line.strip()]


def _prepare_git_cache_dir(
    cache_dir: os.PathLike,
    repo: str,
    remote_url: str,
    env: dict,
) -> Path:
    cache_root = Path(cache_dir).expanduser()
    parts = [part for part in repo.split("/") if part not in ("", ".", "..")]
    repo_dir = cache_root.joinpath(*parts)
    repo_dir.mkdir(parents=True, exist_ok=True)

    try:
        _run_git(["git", "rev-parse", "--is-inside-work-tree"], cwd=str(repo_dir), env=env)
    except DiscoveryError:
        _run_git(["git", "init", "-q"], cwd=str(repo_dir), env=env)

    try:
        result = _run_git(
            ["git", "remote", "get-url", "origin"],
            cwd=str(repo_dir),
            env=env,
            capture_output=True,
        )
        current_remote = result.stdout.strip()
        if current_remote and current_remote != remote_url:
            _run_git(["git", "remote", "set-url", "origin", remote_url], cwd=str(repo_dir), env=env)
    except DiscoveryError:
        _run_git(["git", "remote", "add", "origin", remote_url], cwd=str(repo_dir), env=env)

    return repo_dir


def _run_git(
    args: List[str],
    *,
    cwd: str,
    env: dict,
    capture_output: bool = False,
) -> subprocess.CompletedProcess:
    try:
        return subprocess.run(
            args,
            cwd=cwd,
            env=env,
            check=True,
            text=True,
            stdout=subprocess.PIPE if capture_output else subprocess.DEVNULL,
            stderr=subprocess.PIPE,
        )
    except FileNotFoundError as exc:
        raise DiscoveryError("git is not installed") from exc
    except subprocess.CalledProcessError as exc:
        detail = exc.stderr.strip() or "unknown git error"
        raise DiscoveryError(f"git command failed: {detail}") from exc


def _fetch_json(
    url: str,
    *,
    timeout_seconds: int,
    user_agent: str,
) -> dict:
    token = os.environ.get("GITHUB_TOKEN") or os.environ.get("GH_TOKEN")
    extra_headers = {"Authorization": f"Bearer {token}"} if token else None
    result = fetch_bytes(
        url,
        timeout_seconds=timeout_seconds,
        user_agent=user_agent,
        accept="application/vnd.github+json",
        extra_headers=extra_headers,
    )
    if result.status != 200 or result.body is None:
        raise DiscoveryError(f"Failed to fetch {url}: status={result.status} error={result.error}")
    try:
        return json.loads(result.body.decode("utf-8"))
    except json.JSONDecodeError as exc:  # pragma: no cover
        raise DiscoveryError(f"Invalid JSON from {url}") from exc


def _rewrite_language_segment(url: str, base_language: str, target_language: str) -> Optional[str]:
    parsed = urlparse(url)
    parts = parsed.path.split("/")
    replaced = False
    for idx, part in enumerate(parts):
        if part.lower() == base_language.lower():
            parts[idx] = target_language
            replaced = True
            break
    if not replaced:
        return None
    return urlunparse(parsed._replace(path="/".join(parts)))
