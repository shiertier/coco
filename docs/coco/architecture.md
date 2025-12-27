# Architecture Overview

CoCo v3.0 follows Physical Separation and No-DI (static dispatch) principles.

## Workspace layout

- `crates/coco-protocol`: DTOs, traits, and error types only
- `crates/coco-core`: parsing, chunking, text utilities (no I/O)
- `crates/coco-local`: local service with SQLite, LanceDB, and ONNX Runtime
- `private/coco-server`: server API with Postgres + pgvector
- `private/coco-worker`: async ingest worker

## Local mode

- Single Rust service exposes HTTP APIs for register, import, and query.
- SQLite stores metadata; LanceDB stores vectors.
- Filesystem watcher batches changes and rescans periodically.
- Embeddings use ONNX Runtime, or `COCO_EMBEDDER_MODE=stub` for offline flows.

## Server mode

- API and worker are separate binaries.
- Postgres + pgvector store metadata and vectors.
- Multi-tenant headers (`x-coco-org-id`, `x-coco-project-id`) enforce isolation.
- Embeddings are generated via external providers (default OpenAI).

## Design constraints

- No `#[cfg(feature = "server")]` mixing; code stays physically separated.
- Core crates avoid DB and HTTP dependencies.
- Static dispatch is required; avoid `Box<dyn ...>` at runtime.
- `unsafe` is forbidden unless justified by a low-level requirement.
