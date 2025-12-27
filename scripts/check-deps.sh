#!/usr/bin/env bash
set -euo pipefail

check_forbidden() {
  local manifest="$1"
  local pattern="$2"
  local label="$3"

  if rg -n "$pattern" "$manifest" >/dev/null; then
    echo "Dependency violation in $manifest ($label):"
    rg -n "$pattern" "$manifest"
    exit 1
  fi
}

check_forbidden "crates/coco-core/Cargo.toml" "(axum|hyper|lancedb|pgvector|reqwest|rusqlite|sea-orm|sqlx|tokio|tonic)" "core must stay pure"
check_forbidden "crates/coco-local/Cargo.toml" "(pgvector|postgres|sqlx-postgres|tokio-postgres)" "local must not depend on server storage"
check_forbidden "private/coco-server/Cargo.toml" "(lancedb|rusqlite|sqlx-sqlite)" "server must not depend on embedded storage"
check_forbidden "private/coco-worker/Cargo.toml" "(lancedb|rusqlite|sqlx-sqlite)" "worker must not depend on embedded storage"
