#!/usr/bin/env bash
set -euo pipefail

repo_root="$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd)"
cd "$repo_root"

interval_seconds=1800
cmd=(codex exec "tasks.md这两者完全对齐了吗，如果tasks没有对齐，未做的更正，已做完的应该增加新的todo来使代码更接近目标。如果没有问题的话无需更新，无需确认，直接开始修改tasks.md" --full-auto)

while true; do
  if ! "${cmd[@]}"; then
    printf 'codex exec failed at %s\n' "$(date '+%Y-%m-%d %H:%M:%S')"
  fi
  sleep "$interval_seconds"
done
