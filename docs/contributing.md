# 贡献指南

本项目遵循物理隔离与 No-DI。改动保持小而以数据结构驱动。

## 要求

- Rust 1.85+（workspace edition 为 2024）
- `clippy` 必须通过并带 `-D warnings`
- 除非充分论证并评审，否则禁止 `unsafe`
- 不允许无充分理由的 `#![allow(...)]` 以压制 lint

## 常用命令

```bash
cargo check --workspace
cargo clippy --workspace -- -D warnings
cargo test --workspace
cargo test -p coco-core
```

## 集成测试（server/worker）

集成测试需要带 pgvector 的 Postgres。设置 `COCO_TEST_DB_URL`：

```bash
export COCO_TEST_DB_URL=postgres://user:pass@localhost:5432/coco
scripts/run-ci-integration.sh
```

## E2E 脚本

```bash
scripts/e2e-local.sh
scripts/e2e-server.sh
```

## 性能脚本

```bash
scripts/perf-local-import.py
scripts/perf-local-query.py
scripts/perf-local-memory.py
scripts/perf-qdrant-query.py
```

`scripts/perf-qdrant-query.py` defaults to existing services at `COCO_PERF_SERVER_URL` and `COCO_PERF_QDRANT_URL`. Set `COCO_PERF_USE_DOCKER=1` to spin up Docker services.

## 覆盖率

```bash
scripts/check-coverage.sh
```
