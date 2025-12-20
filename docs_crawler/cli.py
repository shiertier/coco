import argparse
import logging
from pathlib import Path
from typing import List, Optional

from .config import ConfigError, get_source, load_config
from .crawler import Crawler


def _configure_logging(level_name: str) -> None:
    level = getattr(logging, level_name.upper(), None)
    if not isinstance(level, int):
        raise ValueError(f"Invalid log level: {level_name}")
    logging.basicConfig(
        level=level,
        format="%(asctime)s %(levelname)s %(name)s: %(message)s",
    )


def _build_parser() -> argparse.ArgumentParser:
    parser = argparse.ArgumentParser(prog="docs-crawler", description="Crawl md/mdx docs for LLM reference")
    parser.add_argument("--config", type=Path, default=Path("sources.json"), help="Config JSON file path")
    parser.add_argument("--log-level", default="INFO", help="Logging level (default: INFO)")

    sub = parser.add_subparsers(dest="command", required=True)

    crawl = sub.add_parser("crawl", help="Discover and fetch docs")
    crawl.add_argument("--source", help="Source id to crawl (default: all)")
    crawl.add_argument("--prune", action="store_true", help="Delete docs no longer discovered")
    crawl.add_argument("--dry-run", action="store_true", help="Print actions but do not write files")
    crawl.add_argument("--max-workers", type=int, help="Override max concurrent fetches")
    crawl.add_argument("--timeout", type=int, help="Override request timeout (seconds)")
    return parser


def main(argv: Optional[List[str]] = None) -> int:
    parser = _build_parser()
    args = parser.parse_args(argv)

    try:
        _configure_logging(args.log_level)
    except ValueError as exc:
        parser.error(str(exc))
        return 2

    try:
        config = load_config(args.config)
    except ConfigError as exc:
        parser.error(str(exc))
        return 2

    if args.command == "crawl":
        sources = config.sources
        if args.source:
            src = get_source(config, args.source)
            if src is None:
                parser.error(f"Unknown source: {args.source}")
                return 2
            sources = [src]

        exit_code = 0
        defaults = config.defaults
        for source in sources:
            crawler = Crawler(
                allowed_extensions=defaults.allowed_extensions,
                user_agent=defaults.user_agent,
                timeout_seconds=args.timeout or defaults.timeout_seconds,
                max_workers=args.max_workers or source.max_workers or defaults.max_workers,
            )
            stats = crawler.crawl_source(source, prune=args.prune, dry_run=args.dry_run)
            print(
                f"[{source.id}] discovered={stats.discovered} fetched={stats.fetched} "
                f"updated={stats.updated} unchanged={stats.unchanged} errors={stats.errors} pruned={stats.pruned}"
            )
            if stats.errors:
                exit_code = 1
        return exit_code

    parser.error(f"Unknown command: {args.command}")
    return 2
