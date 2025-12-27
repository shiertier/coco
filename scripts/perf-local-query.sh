#!/usr/bin/env bash
set -euo pipefail

if ! command -v curl >/dev/null 2>&1; then
  echo "curl is required for perf tests" >&2
  exit 1
fi
if ! command -v python3 >/dev/null 2>&1; then
  echo "python3 is required for perf tests" >&2
  exit 1
fi

lines="${COCO_PERF_LINES:-100000}"
queries="${COCO_PERF_QUERIES:-200}"
concurrency="${COCO_PERF_CONCURRENCY:-1}"
warmup="${COCO_PERF_WARMUP:-10}"
query_mode="${COCO_PERF_QUERY_MODE:-vector}"
embedder_mode="${COCO_PERF_EMBEDDER_MODE:-stub}"

tmp_root="$(mktemp -d 2>/dev/null || mktemp -d -t coco-local-perf)"
cleanup() {
  if [[ -n "${service_pid:-}" ]]; then
    kill "${service_pid}" >/dev/null 2>&1 || true
    wait "${service_pid}" >/dev/null 2>&1 || true
  fi
  rm -rf "${tmp_root}"
}
trap cleanup EXIT

project_root="${tmp_root}/project"
mkdir -p "${project_root}"
python3 - <<'PY' "${project_root}" "${lines}"
import pathlib
import sys

root = pathlib.Path(sys.argv[1])
lines = int(sys.argv[2])
target = root / "main.rs"
with target.open("w", encoding="utf-8") as handle:
    for i in range(lines):
        handle.write(f"// line {i}\n")
PY

port="$(python3 - <<'PY'
import socket
sock = socket.socket()
sock.bind(("127.0.0.1", 0))
port = sock.getsockname()[1]
sock.close()
print(port)
PY
)"

export HOME="${tmp_root}/home"
export COCO_HOST="127.0.0.1"
export COCO_PORT="${port}"
export COCO_META_DB="${tmp_root}/meta.db"
export COCO_VECTOR_DIR="${tmp_root}/vectors"
export COCO_WATCH_ENABLED="0"

if [[ "${embedder_mode}" == "stub" ]]; then
  export COCO_EMBEDDER_MODE="stub"
else
  unset COCO_EMBEDDER_MODE
fi

cargo run -p coco-local --features local-storage -- start --headless >"${tmp_root}/service.log" 2>&1 &
service_pid=$!

ready=false
for _ in {1..40}; do
  if curl -fsS "http://127.0.0.1:${port}/v1/sys/health" >/dev/null; then
    ready=true
    break
  fi
  sleep 0.25
done
if [[ "${ready}" != "true" ]]; then
  echo "local service did not become ready" >&2
  exit 1
fi

cargo run -p coco-local --features local-storage -- import "${project_root}" --recursive >/dev/null

python3 - <<'PY' "${port}" "${query_mode}" "${queries}" "${concurrency}" "${warmup}"
import concurrent.futures
import json
import sys
import time
import urllib.request

port = int(sys.argv[1])
query_mode = sys.argv[2]
queries = int(sys.argv[3])
concurrency = int(sys.argv[4])
warmup = int(sys.argv[5])

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
    request = urllib.request.Request(url, data=body, headers=headers, method="POST")
    with urllib.request.urlopen(request, timeout=30) as response:
        raw = response.read()
    data = json.loads(raw)
    if not data.get("results"):
        raise RuntimeError("query returned no results")
    end = time.perf_counter()
    return end - start


for _ in range(warmup):
    run_one()

latencies = []
if concurrency <= 1:
    for _ in range(queries):
        latencies.append(run_one())
else:
    with concurrent.futures.ThreadPoolExecutor(max_workers=concurrency) as executor:
        futures = [executor.submit(run_one) for _ in range(queries)]
        for future in futures:
            latencies.append(future.result())

latencies.sort()
if not latencies:
    raise RuntimeError("no query samples recorded")


def percentile(p: float) -> float:
    if len(latencies) == 1:
        return latencies[0]
    index = int((len(latencies) - 1) * p)
    return latencies[index]

p50 = percentile(0.50)
p95 = percentile(0.95)
p99 = percentile(0.99)
avg = sum(latencies) / len(latencies)

print(f"queries={len(latencies)}")
print(f"concurrency={concurrency}")
print(f"p50_ms={p50 * 1000:.2f}")
print(f"p95_ms={p95 * 1000:.2f}")
print(f"p99_ms={p99 * 1000:.2f}")
print(f"avg_ms={avg * 1000:.2f}")
PY
