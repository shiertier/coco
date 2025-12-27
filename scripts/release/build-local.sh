#!/usr/bin/env bash
set -euo pipefail

crate="coco-local"
features="local-storage"

proto_bin=""
if command -v protoc >/dev/null 2>&1; then
  proto_bin="$(command -v protoc)"
fi

if [[ -z "${proto_bin}" ]]; then
  if [[ -d "${HOME}/.cargo/registry/src" ]]; then
    if command -v rg >/dev/null 2>&1; then
      proto_bin="$(rg --files -g 'protoc' "${HOME}/.cargo/registry/src" | rg 'protoc-bin-vendored-.*/bin/protoc$' | head -n 1 || true)"
    else
      proto_bin="$(find "${HOME}/.cargo/registry/src" -type f -name protoc -path '*protoc-bin-vendored*/bin/protoc' | head -n 1 || true)"
    fi
  fi
fi

if [[ -z "${proto_bin}" && -z "${CARGO_NET_OFFLINE:-}" ]]; then
  echo "protoc not found; fetching vendored protoc via cargo..." >&2
  cargo fetch >/dev/null 2>&1 || true
  if command -v rg >/dev/null 2>&1; then
    proto_bin="$(rg --files -g 'protoc' "${HOME}/.cargo/registry/src" | rg 'protoc-bin-vendored-.*/bin/protoc$' | head -n 1 || true)"
  else
    proto_bin="$(find "${HOME}/.cargo/registry/src" -type f -name protoc -path '*protoc-bin-vendored*/bin/protoc' | head -n 1 || true)"
  fi
fi

if [[ -z "${proto_bin}" ]]; then
  echo "protoc not found; install protobuf or set PROTOC" >&2
  exit 1
fi

PROTOC="${proto_bin}" cargo build -p "${crate}" --release --features "${features}"
echo "built: target/release/${crate}"
