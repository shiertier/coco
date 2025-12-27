#!/usr/bin/env python3
import concurrent.futures
import json
import os
import shutil
import socket
import subprocess
import sys
import tempfile
import time
from pathlib import Path
from typing import Optional
from urllib import request


def ensure_command(cmd: str) -> None:
    if shutil.which(cmd) is None:
        raise SystemExit(f"{cmd} is required for perf tests")


def write_project(root: Path, lines: int) -> None:
    root.mkdir(parents=True, exist_ok=True)
    target = root / "main.rs"
    with target.open("w", encoding="utf-8") as handle:
        for i in range(lines):
            handle.write(f"// line {i}\n")


def pick_port() -> int:
    sock = socket.socket()
    sock.bind(("127.0.0.1", 0))
    port = sock.getsockname()[1]
    sock.close()
    return port


def wait_ready(port: int) -> bool:
    url = f"http://127.0.0.1:{port}/v1/sys/health"
    for _ in range(40):
        try:
            with request.urlopen(url, timeout=1) as resp:
                if resp.status == 200:
                    return True
        except Exception:
            pass
        time.sleep(0.25)
    return False


def build_env(tmp_root: Path, port: int, embedder_mode: str) -> dict[str, str]:
    env = os.environ.copy()
    env["HOME"] = str(tmp_root / "home")
    env["COCO_HOST"] = "127.0.0.1"
    env["COCO_PORT"] = str(port)
    env["COCO_META_DB"] = str(tmp_root / "meta.db")
    env["COCO_VECTOR_DIR"] = str(tmp_root / "vectors")
    env["COCO_WATCH_ENABLED"] = "0"
    if embedder_mode == "stub":
        env["COCO_EMBEDDER_MODE"] = "stub"
    else:
        env.pop("COCO_EMBEDDER_MODE", None)
    return env


def stop_process(proc: subprocess.Popen) -> None:
    if proc.poll() is not None:
        return
    proc.terminate()
    try:
        proc.wait(timeout=5)
    except subprocess.TimeoutExpired:
        proc.kill()
        proc.wait(timeout=5)


def percentile(values: list[float], p: float) -> float:
    if len(values) == 1:
        return values[0]
    index = int((len(values) - 1) * p)
    return values[index]


def main() -> int:
    ensure_command("cargo")

    lines = int(os.environ.get("COCO_PERF_LINES", "100000"))
    queries = int(os.environ.get("COCO_PERF_QUERIES", "200"))
    concurrency = int(os.environ.get("COCO_PERF_CONCURRENCY", "1"))
    warmup = int(os.environ.get("COCO_PERF_WARMUP", "10"))
    query_mode = os.environ.get("COCO_PERF_QUERY_MODE", "vector")
    embedder_mode = os.environ.get("COCO_PERF_EMBEDDER_MODE", "stub")

    repo_root = Path(__file__).resolve().parents[1]
    tmp_root = Path(tempfile.mkdtemp(prefix="coco-local-perf-"))
    service_proc: Optional[subprocess.Popen] = None
    service_log = None
    try:
        project_root = tmp_root / "project"
        write_project(project_root, lines)

        port = pick_port()
        env = build_env(tmp_root, port, embedder_mode)
        (tmp_root / "home").mkdir(parents=True, exist_ok=True)

        service_log = (tmp_root / "service.log").open("wb")
        service_proc = subprocess.Popen(
            [
                "cargo",
                "run",
                "-p",
                "coco-local",
                "--features",
                "local-storage",
                "--",
                "start",
                "--headless",
            ],
            cwd=repo_root,
            env=env,
            stdout=service_log,
            stderr=subprocess.STDOUT,
        )

        if not wait_ready(port):
            raise SystemExit("local service did not become ready")

        subprocess.run(
            [
                "cargo",
                "run",
                "-p",
                "coco-local",
                "--features",
                "local-storage",
                "--",
                "import",
                str(project_root),
                "--recursive",
            ],
            cwd=repo_root,
            env=env,
            check=True,
        )

        url = f"http://127.0.0.1:{port}/v1/docs/query"
        payload = {
            "intent": {
                "query_text": "line 42",
                "retrieval_mode": query_mode,
                "top_k": 5,
                "hybrid_alpha": 0.5,
                "filters": [],
                "reranker": None,
            }
        }
        body = json.dumps(payload).encode("utf-8")
        headers = {"content-type": "application/json"}

        def run_one() -> float:
            start = time.perf_counter()
            req = request.Request(url, data=body, headers=headers, method="POST")
            with request.urlopen(req, timeout=30) as resp:
                raw = resp.read()
            data = json.loads(raw)
            if not data.get("results"):
                raise RuntimeError("query returned no results")
            end = time.perf_counter()
            return end - start

        for _ in range(warmup):
            run_one()

        latencies: list[float] = []
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

        latencies.sort()
        if not latencies:
            raise RuntimeError("no query samples recorded")

        p50 = percentile(latencies, 0.50)
        p95 = percentile(latencies, 0.95)
        p99 = percentile(latencies, 0.99)
        avg = sum(latencies) / len(latencies)

        print(f"queries={len(latencies)}")
        print(f"concurrency={concurrency}")
        print(f"p50_ms={p50 * 1000:.2f}")
        print(f"p95_ms={p95 * 1000:.2f}")
        print(f"p99_ms={p99 * 1000:.2f}")
        print(f"avg_ms={avg * 1000:.2f}")
    finally:
        if service_proc is not None:
            stop_process(service_proc)
        if service_log is not None:
            service_log.close()
        if tmp_root.exists():
            shutil.rmtree(tmp_root)
    return 0


if __name__ == "__main__":
    sys.exit(main())
