#!/usr/bin/env bash
set -euo pipefail

if ! command -v curl >/dev/null 2>&1; then
  echo "curl is required for E2E tests" >&2
  exit 1
fi
if ! command -v python3 >/dev/null 2>&1; then
  echo "python3 is required for E2E tests" >&2
  exit 1
fi

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

org_id="org-e2e-$(date +%s)"
register_payload="$(python3 - <<'PY' "${org_id}"
import json, sys
org_id = sys.argv[1]
payload = {
  "org_id": org_id,
  "name": "Server Qdrant E2E",
  "source_ref": "src:e2e-qdrant",
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

ingest_payload="$(python3 - <<'PY' "${org_id}" "${project_id}"
import json, sys
org_id, project_id = sys.argv[1], sys.argv[2]
embedding = [0.0] * 1536
embedding[0] = 1.0
payload = {
  "activate": True,
  "documents": [{
    "doc_id": "doc-e2e",
    "source_ref": "src:e2e-qdrant",
    "title": "Doc E2E",
    "chunks": [{
      "chunk_id": "chunk-e2e",
      "content": "hello world",
      "embedding": embedding,
      "start": 0,
      "end": 11
    }]
  }]
}
print(json.dumps(payload))
PY
)"
job_id="$(
  curl -fsS \
    -H "content-type: application/json" \
    -H "Authorization: Bearer api" \
    -H "x-coco-org-id: ${org_id}" \
    -H "x-coco-project-id: ${project_id}" \
    -d "${ingest_payload}" \
    "http://127.0.0.1:3456/v1/ingest/batch" \
  | python3 - <<'PY'
import json, sys
data = json.load(sys.stdin)
print(data["job_id"])
PY
)"

status=""
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
    break
  fi
  if [[ "${status}" == "failed" ]]; then
    echo "ingest job failed" >&2
    exit 1
  fi
  sleep 1
done
if [[ "${status}" != "completed" ]]; then
  echo "ingest job did not complete" >&2
  exit 1
fi

query_payload="$(python3 - <<'PY'
import json
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
print(json.dumps(payload))
PY
)"
result_count="$(
  curl -fsS \
    -H "content-type: application/json" \
    -H "Authorization: Bearer api" \
    -H "x-coco-org-id: ${org_id}" \
    -H "x-coco-project-id: ${project_id}" \
    -d "${query_payload}" \
    "http://127.0.0.1:3456/v1/docs/query" \
  | python3 - <<'PY'
import json, sys
data = json.load(sys.stdin)
meta = data.get("meta")
payload = data.get("data", {})
results = payload.get("results", [])
if not isinstance(meta, dict) or "status" not in meta:
    print("missing response meta.status", file=sys.stderr)
    sys.exit(1)
if not isinstance(results, list):
    print("missing response data.results", file=sys.stderr)
    sys.exit(1)
if results:
    first = results[0]
    if not isinstance(first, dict) or "meta" not in first or "chunk" not in first:
        print("missing SearchHit meta/chunk", file=sys.stderr)
        sys.exit(1)
print(len(results))
PY
)"
if [[ "${result_count}" -lt 1 ]]; then
  echo "no results returned from query" >&2
  exit 1
fi

echo "qdrant E2E ok"
