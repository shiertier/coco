from __future__ import annotations

import json
from pathlib import Path
from typing import Any, Dict, List, Optional

from .models import CrawlerConfig, DefaultsConfig, EntryPointConfig, SourceConfig


class ConfigError(ValueError):
    pass


def _require(obj: Dict[str, Any], key: str, expected_type: Any) -> Any:
    if key not in obj:
        raise ConfigError(f"Missing required key: {key}")
    value = obj[key]
    if not isinstance(value, expected_type):
        raise ConfigError(f"Invalid type for {key}: expected {expected_type}, got {type(value)}")
    return value


def _optional(obj: Dict[str, Any], key: str, expected_type: Any) -> Any:
    if key not in obj:
        return None
    value = obj[key]
    if not isinstance(value, expected_type):
        raise ConfigError(f"Invalid type for {key}: expected {expected_type}, got {type(value)}")
    return value


def load_config(path: Path) -> CrawlerConfig:
    try:
        raw = json.loads(path.read_text(encoding="utf-8"))
    except FileNotFoundError as exc:
        raise ConfigError(f"Config file not found: {path}") from exc
    except json.JSONDecodeError as exc:
        raise ConfigError(f"Invalid JSON in config file: {path}") from exc

    if not isinstance(raw, dict):
        raise ConfigError("Config root must be an object")

    version = _require(raw, "version", int)
    if version != 1:
        raise ConfigError(f"Unsupported config version: {version}")

    defaults_raw = raw.get("defaults", {})
    if defaults_raw is None:
        defaults_raw = {}
    if not isinstance(defaults_raw, dict):
        raise ConfigError("defaults must be an object")

    allowed_extensions = defaults_raw.get("allowed_extensions", [".md", ".mdx"])
    if not isinstance(allowed_extensions, list) or not all(
        isinstance(ext, str) for ext in allowed_extensions
    ):
        raise ConfigError("defaults.allowed_extensions must be a list of strings")

    user_agent = defaults_raw.get("user_agent", "docs-crawler/0.1")
    if not isinstance(user_agent, str) or not user_agent.strip():
        raise ConfigError("defaults.user_agent must be a non-empty string")

    timeout_seconds = defaults_raw.get("timeout_seconds", 30)
    if not isinstance(timeout_seconds, int) or timeout_seconds <= 0:
        raise ConfigError("defaults.timeout_seconds must be a positive integer")

    max_workers = defaults_raw.get("max_workers", 8)
    if not isinstance(max_workers, int) or max_workers <= 0:
        raise ConfigError("defaults.max_workers must be a positive integer")

    defaults = DefaultsConfig(
        allowed_extensions=allowed_extensions,
        user_agent=user_agent,
        timeout_seconds=timeout_seconds,
        max_workers=max_workers,
    )

    sources_raw = _require(raw, "sources", list)
    sources: List[SourceConfig] = []
    for idx, src in enumerate(sources_raw):
        if not isinstance(src, dict):
            raise ConfigError(f"sources[{idx}] must be an object")

        source_id = _require(src, "id", str).strip()
        if not source_id:
            raise ConfigError(f"sources[{idx}].id must be a non-empty string")

        description = _optional(src, "description", str)
        output_dir_raw = _require(src, "output_dir", str)
        output_dir = Path(output_dir_raw)

        default_language = src.get("default_language", "en")
        if not isinstance(default_language, str) or not default_language.strip():
            raise ConfigError(f"sources[{idx}].default_language must be a non-empty string")

        source_allowed_extensions = _optional(src, "allowed_extensions", list)
        if source_allowed_extensions is not None and not all(
            isinstance(ext, str) for ext in source_allowed_extensions
        ):
            raise ConfigError(f"sources[{idx}].allowed_extensions must be a list of strings")

        source_max_workers = _optional(src, "max_workers", int)
        if source_max_workers is not None and source_max_workers <= 0:
            raise ConfigError(f"sources[{idx}].max_workers must be a positive integer")

        entrypoints_raw = _require(src, "entrypoints", list)
        entrypoints: List[EntryPointConfig] = []
        for eidx, ep in enumerate(entrypoints_raw):
            if not isinstance(ep, dict):
                raise ConfigError(f"sources[{idx}].entrypoints[{eidx}] must be an object")
            ep_type = _require(ep, "type", str)
            ep_url = _optional(ep, "url", str)
            ep_language = _optional(ep, "language", str)
            ep_repo = _optional(ep, "repo", str)
            ep_ref = _optional(ep, "ref", str)
            ep_path = _optional(ep, "path", str)
            ep_languages = _optional(ep, "languages", list)
            ep_variants = _optional(ep, "language_variants", list)
            ep_api_base_url = _optional(ep, "api_base_url", str)
            ep_raw_base_url = _optional(ep, "raw_base_url", str)
            ep_git_cache_dir = _optional(ep, "git_cache_dir", str)
            if ep_languages is not None and not all(isinstance(lang, str) and lang.strip() for lang in ep_languages):
                raise ConfigError(
                    f"sources[{idx}].entrypoints[{eidx}].languages must be a list of strings"
                )
            if ep_variants is not None:
                if not all(isinstance(lang, str) and lang.strip() for lang in ep_variants):
                    raise ConfigError(
                        f"sources[{idx}].entrypoints[{eidx}].language_variants must be a list of strings"
                    )
                ep_variants = [lang.strip() for lang in ep_variants if lang.strip()]
                if not ep_variants:
                    ep_variants = None
                elif not ep_language or not ep_language.strip():
                    raise ConfigError(
                        f"sources[{idx}].entrypoints[{eidx}].language must be set when language_variants is used"
                    )
            if ep_type == "llms_txt":
                if ep_url is None or not ep_url.strip():
                    raise ConfigError(f"sources[{idx}].entrypoints[{eidx}].url must be a non-empty string")
            elif ep_type == "github_tree":
                if ep_repo is None or not ep_repo.strip():
                    raise ConfigError(f"sources[{idx}].entrypoints[{eidx}].repo must be a non-empty string")
                if ep_path is None or not ep_path.strip():
                    raise ConfigError(f"sources[{idx}].entrypoints[{eidx}].path must be a non-empty string")

            entrypoints.append(
                EntryPointConfig(
                    type=ep_type,
                    url=ep_url,
                    language=ep_language,
                    repo=ep_repo,
                    ref=ep_ref,
                    path=ep_path,
                    languages=[lang.strip() for lang in ep_languages] if ep_languages else None,
                    api_base_url=ep_api_base_url,
                    raw_base_url=ep_raw_base_url,
                    language_variants=ep_variants,
                    git_cache_dir=ep_git_cache_dir,
                )
            )

        sources.append(
            SourceConfig(
                id=source_id,
                description=description,
                output_dir=output_dir,
                default_language=default_language,
                entrypoints=entrypoints,
                allowed_extensions=source_allowed_extensions,
                max_workers=source_max_workers,
            )
        )

    return CrawlerConfig(version=version, defaults=defaults, sources=sources)


def get_source(config: CrawlerConfig, source_id: str) -> Optional[SourceConfig]:
    for source in config.sources:
        if source.id == source_id:
            return source
    return None
