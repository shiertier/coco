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
if ! command -v ps >/dev/null 2>&1; then
  echo "ps is required for perf tests" >&2
  exit 1
fi

lines="${COCO_PERF_LINES:-100000}"
embedder_mode="${COCO_PERF_EMBEDDER_MODE:-stub}"
sample_interval="${COCO_PERF_SAMPLE_INTERVAL:-0.1}"
settle_seconds="${COCO_PERF_IDLE_SECONDS:-0.5}"

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

sleep "${settle_seconds}"

read_rss_kb() {
  ps -o rss= -p "${service_pid}" 2>/dev/null | tr -d ' '
}

idle_kb="$(read_rss_kb)"
if [[ -z "${idle_kb}" ]]; then
  echo "failed to read service memory" >&2
  exit 1
fi

max_kb="${idle_kb}"

cargo run -p coco-local --features local-storage -- import "${project_root}" --recursive >"${tmp_root}/import.log" 2>&1 &
import_pid=$!

while kill -0 "${import_pid}" >/dev/null 2>&1; do
  current_kb="$(read_rss_kb || true)"
  if [[ -n "${current_kb}" ]]; then
    if (( current_kb > max_kb )); then
      max_kb="${current_kb}"
    fi
  fi
  sleep "${sample_interval}"
done
wait "${import_pid}"

current_kb="$(read_rss_kb || true)"
if [[ -n "${current_kb}" ]]; then
  if (( current_kb > max_kb )); then
    max_kb="${current_kb}"
  fi
fi

python3 - <<'PY' "${lines}" "${idle_kb}" "${max_kb}"
import sys

lines = int(sys.argv[1])
idle_kb = int(sys.argv[2])
peak_kb = int(sys.argv[3])

print(f"lines={lines}")
print(f"idle_mb={idle_kb / 1024:.2f}")
print(f"peak_mb={peak_kb / 1024:.2f}")
PY
