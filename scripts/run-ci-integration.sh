#!/usr/bin/env bash
set -euo pipefail

compose="docker compose"
if ! ${compose} version >/dev/null 2>&1; then
  compose="docker-compose"
fi

${compose} -f docker-compose.yml up -d db

cleanup() {
  ${compose} -f docker-compose.yml down -v
}
trap cleanup EXIT

for _ in {1..30}; do
  if ${compose} -f docker-compose.yml exec -T db pg_isready -U coco -d coco >/dev/null 2>&1; then
    break
  fi
  sleep 1
done

export COCO_TEST_DB_URL="postgres://coco:coco@127.0.0.1:5432/coco"

cargo test -p coco-server --features server-storage --tests
cargo test -p coco-worker --tests
