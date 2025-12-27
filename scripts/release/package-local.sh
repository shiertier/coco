#!/usr/bin/env bash
set -euo pipefail

version="${1:?version required}"
target="${2:?target required}"
target_dir="${3:?target dir required}"

bin_name="coco-local"
bin_path="${target_dir}/${bin_name}"
dist_dir="${target_dir}/dist"
archive="coco-local-${version}-${target}.tar.gz"

mkdir -p "${dist_dir}"
tar -C "${target_dir}" -czf "${dist_dir}/${archive}" "${bin_name}"

if [[ -n "${GITHUB_OUTPUT:-}" ]]; then
  echo "PACKAGE=${dist_dir}/${archive}" >> "${GITHUB_OUTPUT}"
fi
