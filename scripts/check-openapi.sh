#!/usr/bin/env bash
set -euo pipefail

repo_root="$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd)"

pushd "${repo_root}" >/dev/null

COCO_OFFLINE="${COCO_OFFLINE:-}" bash scripts/generate-openapi.sh
python scripts/generate-ts-sdk.py
python scripts/generate-py-sdk.py

git diff --exit-code -- openapi.json sdk sdk_py

popd >/dev/null
