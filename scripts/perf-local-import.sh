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

elapsed="$(
  python3 - <<'PY' "${project_root}"
import subprocess
import sys
import time

project_root = sys.argv[1]
cmd = [
    "cargo", "run", "-p", "coco-local", "--features", "local-storage", "--",
    "import", project_root, "--recursive",
]
start = time.perf_counter()
subprocess.run(cmd, check=True)
end = time.perf_counter()
print(f"{end - start:.3f}")
PY
)"

python3 - <<'PY' "${lines}" "${elapsed}"
import sys

lines = int(sys.argv[1])
elapsed = float(sys.argv[2])
rate = lines / elapsed if elapsed > 0 else 0.0
print(f"lines={lines}")
print(f"elapsed_sec={elapsed:.3f}")
print(f"lines_per_sec={rate:.2f}")
PY
