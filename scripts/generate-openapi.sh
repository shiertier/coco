#!/usr/bin/env bash
set -euo pipefail

repo_root="$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd)"
output_path="${repo_root}/openapi.json"
offline_flag=""

if [[ "${COCO_OFFLINE:-}" == "1" ]]; then
  offline_flag="--offline"
fi

cargo run -p coco-server --bin openapi --features server-storage --locked ${offline_flag} > "${output_path}"
echo "Wrote ${output_path}"
