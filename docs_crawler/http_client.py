from __future__ import annotations

from dataclasses import dataclass
from typing import Dict, Optional
from urllib.error import HTTPError, URLError
from urllib.request import Request, urlopen


@dataclass(frozen=True)
class FetchResult:
    url: str
    status: int
    headers: Dict[str, str]
    body: Optional[bytes]
    error: Optional[str] = None


def fetch_bytes(
    url: str,
    *,
    timeout_seconds: int,
    user_agent: str,
    accept: str = "text/markdown, text/plain;q=0.9, */*;q=0.8",
    if_none_match: Optional[str] = None,
    if_modified_since: Optional[str] = None,
    extra_headers: Optional[Dict[str, str]] = None,
) -> FetchResult:
    headers = {"User-Agent": user_agent, "Accept": accept}
    if extra_headers:
        headers.update(extra_headers)
    if if_none_match:
        headers["If-None-Match"] = if_none_match
    if if_modified_since:
        headers["If-Modified-Since"] = if_modified_since

    request = Request(url, headers=headers, method="GET")
    try:
        with urlopen(request, timeout=timeout_seconds) as response:
            return FetchResult(
                url=url,
                status=getattr(response, "status", 200),
                headers=dict(response.headers),
                body=response.read(),
            )
    except HTTPError as exc:
        if exc.code == 304:
            return FetchResult(url=url, status=304, headers=dict(exc.headers), body=None)
        return FetchResult(url=url, status=exc.code, headers=dict(exc.headers), body=None, error=str(exc))
    except URLError as exc:
        return FetchResult(url=url, status=0, headers={}, body=None, error=str(exc))


def fetch_text(
    url: str,
    *,
    timeout_seconds: int,
    user_agent: str,
) -> str:
    result = fetch_bytes(url, timeout_seconds=timeout_seconds, user_agent=user_agent)
    if result.status != 200 or result.body is None:
        raise RuntimeError(f"Failed to fetch {url}: status={result.status} error={result.error}")
    return result.body.decode("utf-8", errors="replace")
