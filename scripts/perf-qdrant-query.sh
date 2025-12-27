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

docs="${COCO_PERF_DOCS:-1000}"
batch_size="${COCO_PERF_BATCH_SIZE:-50}"
queries="${COCO_PERF_QUERIES:-200}"
concurrency="${COCO_PERF_CONCURRENCY:-4}"
warmup="${COCO_PERF_WARMUP:-10}"

compose="docker compose"
if ! ${compose} version >/dev/null 2>&1; then
  compose="docker-compose"
fi

cleanup() {
  ${compose} -f docker-compose.yml --profile qdrant down -v >/dev/null 2>&1 || true
}
trap cleanup EXIT

export COCO_VECTOR_BACKEND="qdrant"

${compose} -f docker-compose.yml --profile qdrant up -d db qdrant coco-server coco-worker

ready=false
for _ in {1..40}; do
  if ${compose} -f docker-compose.yml exec -T db pg_isready -U coco -d coco >/dev/null 2>&1; then
    ready=true
    break
  fi
  sleep 1
done
if [[ "${ready}" != "true" ]]; then
  echo "database did not become ready" >&2
  exit 1
fi

ready=false
for _ in {1..40}; do
  if curl -fsS "http://127.0.0.1:6333/collections" >/dev/null; then
    ready=true
    break
  fi
  sleep 0.25
done
if [[ "${ready}" != "true" ]]; then
  echo "qdrant did not become ready" >&2
  exit 1
fi

ready=false
for _ in {1..40}; do
  if curl -fsS "http://127.0.0.1:3456/v1/sys/health" >/dev/null; then
    ready=true
    break
  fi
  sleep 0.25
done
if [[ "${ready}" != "true" ]]; then
  echo "server did not become ready" >&2
  exit 1
fi

backend_kind="$(
  curl -fsS "http://127.0.0.1:3456/v1/sys/health" \
  | python3 - <<'PY'
import json, sys
data = json.load(sys.stdin)
backend = data.get("vector_backend", {})
print(backend.get("kind", ""))
PY
)"
if [[ "${backend_kind}" != "qdrant" ]]; then
  echo "expected vector backend qdrant, got ${backend_kind}" >&2
  exit 1
fi

org_id="org-perf-$(date +%s)"
register_payload="$(python3 - <<'PY' "${org_id}"
import json, sys
org_id = sys.argv[1]
payload = {
  "org_id": org_id,
  "name": "Server Qdrant Perf",
  "source_ref": "src:perf-qdrant",
  "platform": "docker",
}
print(json.dumps(payload))
PY
)"
project_id="$(
  curl -fsS \
    -H "content-type: application/json" \
    -H "Authorization: Bearer admin" \
    -d "${register_payload}" \
    "http://127.0.0.1:3456/v1/sys/register" \
  | python3 - <<'PY'
import json, sys
data = json.load(sys.stdin)
print(data["project_id"])
PY
)"

ingest_batch() {
  local start_index="$1"
  local count="$2"
  local payload
  payload="$(python3 - <<'PY' "${start_index}" "${count}"
import json, sys
start_index = int(sys.argv[1])
count = int(sys.argv[2])
embedding = [0.0] * 1536
embedding[0] = 1.0
documents = []
for idx in range(start_index, start_index + count):
    documents.append({
        "doc_id": f"doc-{idx}",
        "source_ref": "src:perf-qdrant",
        "title": f"Doc {idx}",
        "chunks": [{
            "chunk_id": f"chunk-{idx}",
            "content": "hello world",
            "embedding": embedding,
            "start": 0,
            "end": 11
        }]
    })
payload = {"activate": True, "documents": documents}
print(json.dumps(payload))
PY
)"
  local job_id
  job_id="$(
    curl -fsS \
      -H "content-type: application/json" \
      -H "Authorization: Bearer api" \
      -H "x-coco-org-id: ${org_id}" \
      -H "x-coco-project-id: ${project_id}" \
      -d "${payload}" \
      "http://127.0.0.1:3456/v1/ingest/batch" \
    | python3 - <<'PY'
import json, sys
data = json.load(sys.stdin)
print(data["job_id"])
PY
  )"
  for _ in {1..60}; do
    status="$(
      curl -fsS \
        -H "Authorization: Bearer api" \
        "http://127.0.0.1:3456/v1/jobs/${job_id}" \
      | python3 - <<'PY'
import json, sys
data = json.load(sys.stdin)
print(data["status"])
PY
    )"
    if [[ "${status}" == "completed" ]]; then
      return 0
    fi
    if [[ "${status}" == "failed" ]]; then
      echo "ingest job failed" >&2
      exit 1
    fi
    sleep 1
  done
  echo "ingest job did not complete in time" >&2
  exit 1
}

ingested=0
while [[ "${ingested}" -lt "${docs}" ]]; do
  remaining=$((docs - ingested))
  if [[ "${remaining}" -lt "${batch_size}" ]]; then
    ingest_batch "${ingested}" "${remaining}"
    ingested="${docs}"
  else
    ingest_batch "${ingested}" "${batch_size}"
    ingested=$((ingested + batch_size))
  fi
done

python3 - <<'PY' "${docs}" "${queries}" "${concurrency}" "${warmup}" "${org_id}" "${project_id}"
import concurrent.futures
import json
import sys
import time
import urllib.request

docs = int(sys.argv[1])
queries = int(sys.argv[2])
concurrency = int(sys.argv[3])
warmup = int(sys.argv[4])
org_id = sys.argv[5]
project_id = sys.argv[6]

url = "http://127.0.0.1:3456/v1/docs/query"
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
    request = urllib.request.Request(url, data=body, headers=headers, method="POST")
    with urllib.request.urlopen(request, timeout=30) as response:
        raw = response.read()
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

latencies = []
start_wall = time.perf_counter()
if concurrency <= 1:
    for _ in range(queries):
        latencies.append(run_one())
else:
    with concurrent.futures.ThreadPoolExecutor(max_workers=concurrency) as executor:
        futures = [executor.submit(run_one) for _ in range(queries)]
        for future in futures:
            latencies.append(future.result())
end_wall = time.perf_counter()

latencies.sort()
if not latencies:
    raise RuntimeError("no query samples recorded")

def percentile(p: float) -> float:
    if len(latencies) == 1:
        return latencies[0]
    index = int((len(latencies) - 1) * p)
    return latencies[index]

total_wall = end_wall - start_wall
qps = len(latencies) / total_wall if total_wall > 0 else 0.0
p50 = percentile(0.50)
p95 = percentile(0.95)
p99 = percentile(0.99)
avg = sum(latencies) / len(latencies)

print(f"docs={docs}")
print(f"queries={len(latencies)}")
print(f"concurrency={concurrency}")
print(f"p50_ms={p50 * 1000:.2f}")
print(f"p95_ms={p95 * 1000:.2f}")
print(f"p99_ms={p99 * 1000:.2f}")
print(f"avg_ms={avg * 1000:.2f}")
print(f"qps={qps:.2f}")
PY

echo "qdrant perf query ok"
