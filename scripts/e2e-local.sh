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

tmp_root="$(mktemp -d 2>/dev/null || mktemp -d -t coco-local-e2e)"
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
cat > "${project_root}/readme.md" <<'EOF'
# Title

Local E2E content.
EOF

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

if [[ "${COCO_E2E_MODEL_DOWNLOAD:-0}" == "1" ]]; then
  unset COCO_EMBEDDER_MODE
else
  export COCO_EMBEDDER_MODE="stub"
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

register_payload="$(python3 - <<'PY' "${project_root}"
import json, sys
print(json.dumps({"name": "Local E2E", "path": sys.argv[1]}))
PY
)"
project_id="$(
  curl -fsS \
    -H "content-type: application/json" \
    -d "${register_payload}" \
    "http://127.0.0.1:${port}/v1/sys/register" \
  | python3 - <<'PY'
import json, sys
data = json.load(sys.stdin)
print(data["project_id"])
PY
)"

import_payload="$(python3 - <<'PY' "${project_id}" "${project_root}/readme.md"
import json, pathlib, sys
project_id = sys.argv[1]
path = pathlib.Path(sys.argv[2])
payload = {
  "project_id": project_id,
  "content": path.read_text(),
  "file_type": "markdown",
  "title": path.name,
  "path": str(path),
}
print(json.dumps(payload))
PY
)"
curl -fsS \
  -H "content-type: application/json" \
  -d "${import_payload}" \
  "http://127.0.0.1:${port}/v1/docs/import" \
  >/dev/null

query_payload="$(python3 - <<'PY'
import json
payload = {
  "intent": {
    "query_text": "Title",
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
    -d "${query_payload}" \
    "http://127.0.0.1:${port}/v1/docs/query" \
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

if [[ "${COCO_E2E_MODEL_DOWNLOAD:-0}" == "1" ]]; then
  if [[ ! -f "${HOME}/.coco/models/all-MiniLM-L6-v2.onnx" ]]; then
    echo "model download did not complete" >&2
    exit 1
  fi
fi

echo "local E2E ok"
