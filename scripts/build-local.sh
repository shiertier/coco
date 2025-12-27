#!/usr/bin/env bash
set -euo pipefail

crate="coco-local"
features="local-storage"

cargo fetch -p protoc-bin-vendored >/dev/null

proto_bin=""
if command -v rg >/dev/null 2>&1; then
  proto_bin="$(rg --files -g 'protoc' "${HOME}/.cargo/registry/src" | rg 'protoc-bin-vendored-.*/bin/protoc$' | head -n 1 || true)"
else
  proto_bin="$(find "${HOME}/.cargo/registry/src" -type f -name protoc -path '*protoc-bin-vendored*/bin/protoc' | head -n 1 || true)"
fi

if [[ -z "${proto_bin}" ]]; then
  echo "protoc not found; install protobuf or set PROTOC" >&2
  exit 1
fi

PROTOC="${proto_bin}" cargo build -p "${crate}" --release --features "${features}"
echo "built: target/release/${crate}"
