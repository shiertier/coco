#!/usr/bin/env python3
import concurrent.futures
import json
import os
import shutil
import subprocess
import sys
import time
from pathlib import Path
from typing import Any, Optional
from urllib import request


def pick_compose() -> list[str]:
    if shutil.which("docker"):
        result = subprocess.run(
            ["docker", "compose", "version"],
            check=False,
            capture_output=True,
            text=True,
        )
        if result.returncode == 0:
            return ["docker", "compose"]
    if shutil.which("docker-compose"):
        return ["docker-compose"]
    raise SystemExit("docker compose is required for perf tests")


def http_json(
    url: str, method: str = "GET", payload: Optional[dict[str, Any]] = None, headers: Optional[dict[str, str]] = None
) -> Any:
    body = None
    final_headers = {"content-type": "application/json"}
    if headers:
        final_headers.update(headers)
    if payload is not None:
        body = json.dumps(payload).encode("utf-8")
    req = request.Request(url, data=body, headers=final_headers, method=method)
    with request.urlopen(req, timeout=30) as resp:
        raw = resp.read()
    if not raw:
        return None
    return json.loads(raw)


def wait_http_ok(url: str, attempts: int, delay: float) -> bool:
    for _ in range(attempts):
        try:
            with request.urlopen(url, timeout=2) as resp:
                if resp.status == 200:
                    return True
        except Exception:
            pass
        time.sleep(delay)
    return False


def percentile(values: list[float], p: float) -> float:
    if len(values) == 1:
        return values[0]
    index = int((len(values) - 1) * p)
    return values[index]


def main() -> int:
    use_docker = os.environ.get("COCO_PERF_USE_DOCKER", "").lower() in ("1", "true", "yes")
    server_url = os.environ.get("COCO_PERF_SERVER_URL", "http://127.0.0.1:3456").rstrip("/")
    qdrant_url = os.environ.get("COCO_PERF_QDRANT_URL", "http://127.0.0.1:6333").rstrip("/")
    docs = int(os.environ.get("COCO_PERF_DOCS", "1000"))
    batch_size = int(os.environ.get("COCO_PERF_BATCH_SIZE", "50"))
    queries = int(os.environ.get("COCO_PERF_QUERIES", "200"))
    concurrency = int(os.environ.get("COCO_PERF_CONCURRENCY", "4"))
    warmup = int(os.environ.get("COCO_PERF_WARMUP", "10"))

    repo_root = Path(__file__).resolve().parents[1]
    compose_cmd = pick_compose() if use_docker else None

    env = os.environ.copy()
    if use_docker:
        env["COCO_VECTOR_BACKEND"] = "qdrant"

    def compose(args: list[str], check: bool = True) -> subprocess.CompletedProcess:
        if not compose_cmd:
            raise RuntimeError("compose requested without docker")
        return subprocess.run(
            [*compose_cmd, "-f", "docker-compose.yml", *args],
            cwd=repo_root,
            env=env,
            check=check,
            capture_output=True,
            text=True,
        )

    try:
        if use_docker:
            compose(["--profile", "qdrant", "up", "-d", "db", "qdrant", "coco-server", "coco-worker"])

            ready = False
            for _ in range(40):
                result = compose(["exec", "-T", "db", "pg_isready", "-U", "coco", "-d", "coco"], check=False)
                if result.returncode == 0:
                    ready = True
                    break
                time.sleep(1)
            if not ready:
                raise SystemExit("database did not become ready")

        if not wait_http_ok(f"{qdrant_url}/collections", 40, 0.25):
            raise SystemExit("qdrant did not become ready")

        if not wait_http_ok(f"{server_url}/v1/sys/health", 40, 0.25):
            raise SystemExit("server did not become ready")

        health = http_json(f"{server_url}/v1/sys/health")
        backend = health.get("vector_backend", {}) if isinstance(health, dict) else {}
        if backend.get("kind") != "qdrant":
            raise SystemExit(f"expected vector backend qdrant, got {backend.get('kind')}")

        org_id = f"org-perf-{int(time.time())}"
        register_payload = {
            "org_id": org_id,
            "name": "Server Qdrant Perf",
            "source_ref": "src:perf-qdrant",
            "platform": "docker" if use_docker else "local",
        }
        register = http_json(
            f"{server_url}/v1/sys/register",
            method="POST",
            payload=register_payload,
            headers={"Authorization": "Bearer admin"},
        )
        project_id = register.get("project_id")
        if not project_id:
            raise SystemExit("register did not return project_id")

        def ingest_batch(start_index: int, count: int) -> None:
            embedding = [0.0] * 1536
            embedding[0] = 1.0
            documents = []
            for idx in range(start_index, start_index + count):
                documents.append(
                    {
                        "doc_id": f"doc-{idx}",
                        "source_ref": "src:perf-qdrant",
                        "title": f"Doc {idx}",
                        "chunks": [
                            {
                                "chunk_id": f"chunk-{idx}",
                                "content": "hello world",
                                "embedding": embedding,
                                "start": 0,
                                "end": 11,
                            }
                        ],
                    }
                )
            payload = {"activate": True, "documents": documents}
            response = http_json(
                f"{server_url}/v1/ingest/batch",
                method="POST",
                payload=payload,
                headers={
                    "Authorization": "Bearer api",
                    "x-coco-org-id": org_id,
                    "x-coco-project-id": project_id,
                },
            )
            job_id = response.get("job_id")
            if not job_id:
                raise SystemExit("ingest did not return job_id")
            for _ in range(60):
                status_payload = http_json(
                    f"{server_url}/v1/jobs/{job_id}",
                    headers={"Authorization": "Bearer api"},
                )
                status = status_payload.get("status")
                if status == "completed":
                    return
                if status == "failed":
                    raise SystemExit("ingest job failed")
                time.sleep(1)
            raise SystemExit("ingest job did not complete in time")

        ingested = 0
        while ingested < docs:
            remaining = docs - ingested
            count = remaining if remaining < batch_size else batch_size
            ingest_batch(ingested, count)
            ingested += count

        url = f"{server_url}/v1/docs/query"
        embedding = [0.0] * 1536
        embedding[0] = 1.0
        payload = {
            "intent": {
                "query_text": None,
                "query_embedding": embedding,
                "retrieval_mode": "vector",
                "top_k": 5,
                "hybrid_alpha": 0.5,
                "filters": [],
                "reranker": None,
            }
        }
        body = json.dumps(payload).encode("utf-8")
        headers = {
            "content-type": "application/json",
            "Authorization": "Bearer api",
            "x-coco-org-id": org_id,
            "x-coco-project-id": project_id,
        }

        def run_one() -> float:
            start = time.perf_counter()
            req = request.Request(url, data=body, headers=headers, method="POST")
            with request.urlopen(req, timeout=30) as resp:
                raw = resp.read()
            data = json.loads(raw)
            meta = data.get("meta")
            payload = data.get("data", {})
            results = payload.get("results", [])
            if not isinstance(meta, dict) or "status" not in meta:
                raise RuntimeError("missing response meta.status")
            if not isinstance(results, list) or not results:
                raise RuntimeError("query returned no results")
            end = time.perf_counter()
            return end - start

        for _ in range(warmup):
            run_one()

        latencies: list[float] = []
        start_wall = time.perf_counter()
        if concurrency <= 1:
            for _ in range(queries):
                latencies.append(run_one())
        else:
            with concurrent.futures.ThreadPoolExecutor(
                max_workers=concurrency
            ) as executor:
                futures = [executor.submit(run_one) for _ in range(queries)]
                for future in futures:
                    latencies.append(future.result())
        end_wall = time.perf_counter()

        latencies.sort()
        if not latencies:
            raise RuntimeError("no query samples recorded")

        total_wall = end_wall - start_wall
        qps = len(latencies) / total_wall if total_wall > 0 else 0.0
        p50 = percentile(latencies, 0.50)
        p95 = percentile(latencies, 0.95)
        p99 = percentile(latencies, 0.99)
        avg = sum(latencies) / len(latencies)

        print(f"docs={docs}")
        print(f"queries={len(latencies)}")
        print(f"concurrency={concurrency}")
        print(f"p50_ms={p50 * 1000:.2f}")
        print(f"p95_ms={p95 * 1000:.2f}")
        print(f"p99_ms={p99 * 1000:.2f}")
        print(f"avg_ms={avg * 1000:.2f}")
        print(f"qps={qps:.2f}")
    finally:
        if use_docker and compose_cmd:
            subprocess.run(
                [*compose_cmd, "-f", "docker-compose.yml", "--profile", "qdrant", "down", "-v"],
                cwd=repo_root,
                env=env,
                check=False,
                capture_output=True,
                text=True,
            )
    return 0


if __name__ == "__main__":
    sys.exit(main())
