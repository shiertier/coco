# Contributing

This project follows Physical Separation and No-DI. Keep changes small and data-structure driven.

## Requirements

- Rust 1.85+ (workspace edition is 2024)
- `clippy` must pass with `-D warnings`
- No `unsafe` unless fully justified and reviewed
- No `#![allow(...)]` lint suppression without a strong reason

## Common commands

```bash
cargo check --workspace
cargo clippy --workspace -- -D warnings
cargo test --workspace
cargo test -p coco-core
```

## Integration tests (server/worker)

Integration tests require Postgres with pgvector. Set `COCO_TEST_DB_URL`:

```bash
export COCO_TEST_DB_URL=postgres://user:pass@localhost:5432/coco
scripts/run-ci-integration.sh
```

## E2E scripts

```bash
scripts/e2e-local.sh
scripts/e2e-server.sh
```

## Performance scripts

```bash
scripts/perf-local-import.sh
scripts/perf-local-query.sh
scripts/perf-local-memory.sh
```

## Coverage

```bash
scripts/check-coverage.sh
```
