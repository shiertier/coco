# Extension Guide

This guide covers safe extension points without breaking Physical Separation.

## Add a new parser or chunker (core)

- Implement parsing or chunking logic in `crates/coco-core`.
- Keep the crate pure: no DB, no HTTP, no async runtimes.
- Favor data structures that minimize cloning and borrow complexity.

## Add a new local storage backend

- Implement a concrete type that satisfies `StorageBackend`.
- Wire it in `crates/coco-local` with static dispatch.
- Avoid `Box<dyn ...>` and runtime selection unless unavoidable.

## Add a new server storage backend

- Keep server-only code in `private/coco-server`.
- Use Postgres/pgvector or another network VDB.
- Do not introduce embedded databases into server crates.

## Add a new embedding provider (server)

- Implement a concrete embedding client in `private/coco-server`.
- Use `COCO_{PROVIDER}_API_KEY` and `COCO_{PROVIDER}_BASE_URL` for config.
- Keep retries and timeouts explicit and test failure paths.

## Add a new local embedder

- Implement a concrete embedder in `crates/coco-local`.
- Keep it compatible with `EmbeddingModel` and static dispatch.
- If it needs external binaries, gate it behind explicit config.
