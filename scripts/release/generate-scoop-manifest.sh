#!/usr/bin/env bash
set -euo pipefail

raw_version="${1:?version required}"
repo="${2:?github repo required (owner/name)}"
sha_windows="${3:?sha256 for windows required}"
output="${4:-packaging/scoop/coco-local.json}"

version="${raw_version#v}"
tag="v${version}"

mkdir -p "$(dirname "${output}")"

cat > "${output}" <<EOF
{
  "version": "${version}",
  "description": "Context Core local service",
  "homepage": "https://github.com/${repo}",
  "architecture": {
    "64bit": {
      "url": "https://github.com/${repo}/releases/download/${tag}/coco-local-${raw_version}-x86_64-pc-windows-msvc.zip",
      "hash": "${sha_windows}"
    }
  },
  "bin": "coco-local.exe",
  "checkver": {
    "github": "https://github.com/${repo}"
  },
  "autoupdate": {
    "architecture": {
      "64bit": {
        "url": "https://github.com/${repo}/releases/download/v\\$version/coco-local-v\\$version-x86_64-pc-windows-msvc.zip"
      }
    }
  }
}
EOF

echo "Wrote ${output}"
