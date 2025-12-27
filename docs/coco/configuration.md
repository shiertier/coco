# Configuration Reference

Environment variables configure local, server, and worker modes. Values are read at startup.

Legacy aliases:

- `COCO_DB_URL` (legacy alias for `COCO_META_DB`)
- `COCO_LANCEDB_PATH` (legacy alias for `COCO_VECTOR_DIR`)
- `COCO_MODE` (accepted for compatibility but ignored by mode-specific binaries)

## Local mode

Core service settings:

- `COCO_HOST` (default `127.0.0.1`)
- `COCO_PORT` (default `3456`)
- `COCO_META_DB` (default `$HOME/.coco/meta.db`)
- `COCO_VECTOR_DIR` (default `$HOME/.coco/vectors`)

Model settings:

- `COCO_MODEL_PATH` (path to a local ONNX model file)
- `COCO_MODEL_URL` (override download URL)
- `COCO_MODEL_NAME` (default `all-MiniLM-L6-v2`)
- `COCO_MODEL_FILE` (default `all-MiniLM-L6-v2.onnx`)
- `COCO_MODEL_DIMENSIONS` (default `384`)
- `COCO_EMBEDDER_MODE` (`stub` or `mock` to disable downloads)
- `COCO_ORT_BACKEND` (`cpu` or `cuda`)

Chunking settings:

- `COCO_CHUNK_SIZE` (default `256`)
- `COCO_CHUNK_OVERLAP` (default `32`)

Live retrieval settings:

- `COCO_LIVE_RETRIEVAL_ENABLED` (default `true`)
- `COCO_LIVE_RETRIEVAL_WINDOW_BYTES` (default `2048`)

Live grep settings:

- `COCO_LIVE_GREP_ENABLED` (default `false`)
- `COCO_LIVE_GREP_MAX_RESULTS` (default `10`)
- `COCO_LIVE_GREP_TIMEOUT_MS` (default `200`)

Watcher settings:

- `COCO_WATCH_ENABLED` (default `1`)
- `COCO_WATCH_DEBOUNCE_MS` (default `300`)
- `COCO_WATCH_RESCAN_SECS` (default `300`)

Update settings:

- `COCO_RELEASE_REPO` (GitHub repo in `owner/name` form)

## Server mode

Core service settings:

- `COCO_HOST` (default `127.0.0.1`)
- `COCO_PORT` (default `3456`)
- `COCO_META_DB` (required, Postgres + pgvector; `COCO_DB_URL` is a legacy alias)
- `COCO_ADMIN_KEY` (required)
- `COCO_API_KEY` (required)

TLS settings:

- `COCO_TLS_MODE` (`tls` or `proxy`, default `tls`)
- `COCO_TLS_CERT` (path to TLS certificate, optional)
- `COCO_TLS_KEY` (path to TLS private key, optional)

Queue settings:

- `COCO_QUEUE_MODE` (`postgres` or `redis`, default `postgres`)
- `COCO_REDIS_URL` (required when queue mode is `redis`)
- `COCO_REDIS_QUEUE` (default `coco:ingest`)

Rate limits:

- `COCO_RATE_LIMIT_PER_MIN`
- `COCO_RATE_LIMIT_BURST`

Organization limits:

- `COCO_ORG_MAX_DOCUMENTS`
- `COCO_ORG_MAX_CHUNKS`
- `COCO_ORG_MAX_STORAGE_BYTES`
- `COCO_ORG_MAX_EMBEDDINGS_PER_DAY`

Embedding provider:

- `COCO_EMBEDDING_PROVIDER` (default `openai`)
- `COCO_EMBEDDING_MODEL` (default `text-embedding-3-small`)
- `COCO_EMBEDDING_DIMENSIONS` (default `1536`)
- `COCO_EMBEDDING_BATCH_SIZE` (default `64`)
- `COCO_EMBEDDING_TIMEOUT_SECS` (default `30`)
- `COCO_EMBEDDING_MAX_RETRIES` (default `3`)
- `COCO_EMBEDDING_BACKOFF_MS` (default `500`)
- `COCO_EMBEDDING_BACKOFF_MAX_MS` (default `8000`)
- `COCO_OPENAI_API_KEY` (when provider is `openai`)
- `COCO_OPENAI_BASE_URL` (optional override for `openai`)
- `COCO_{PROVIDER}_API_KEY` and `COCO_{PROVIDER}_BASE_URL` for other providers

Worker IPC settings:

- `COCO_WORKER_ADDR` (optional gRPC/IPC address for API <-> Worker)

Ingest settings:

- `COCO_INGEST_BLOB_DIR` (directory for ingest blob payloads)
- `COCO_INGEST_WASM_MODULE_REF` (optional wasm rule module reference)

Query pool settings:

- `COCO_QUERY_PG_MAX_CONNECTIONS`
- `COCO_QUERY_PG_MIN_CONNECTIONS`
- `COCO_QUERY_PG_CONNECT_TIMEOUT_SECS`

VectorDB settings:

- `COCO_VECTOR_BACKEND` (`pgvector` or `qdrant`, default `pgvector`)
- `COCO_VECTOR_DB_URL` (optional, Qdrant endpoint URL)
- `COCO_VECTOR_DB_API_KEY` (optional, API key for Qdrant)
- `COCO_VECTOR_DB_COLLECTION_PREFIX` (required when using Qdrant)

## Worker mode

- `COCO_DB_URL` (required)
- `COCO_WORKER_POLL_MS` (default `500`)
- `COCO_WORKER_BATCH_SIZE` (default `256`)
- `COCO_WORKER_MAX_ATTEMPTS` (default `3`)
- `COCO_WORKER_RETRY_BACKOFF_MS` (default `500`)
- `COCO_WORKER_ADDR` (optional gRPC/IPC address for API <-> Worker)
- `COCO_INGEST_PG_MAX_CONNECTIONS`
- `COCO_INGEST_PG_MIN_CONNECTIONS`
- `COCO_INGEST_PG_CONNECT_TIMEOUT_SECS`
- `COCO_INGEST_BLOB_DIR` (optional ingest blob directory)
- `COCO_INGEST_WASM_DIR` (base directory for wasm rule modules)
- `COCO_INGEST_WASM_MAX_MEMORY_BYTES` (default set by worker)
- `COCO_INGEST_WASM_MAX_FUEL` (default set by worker)
- `COCO_INGEST_WASM_FAILURE_MODE` (`fail` or `skip`, default `fail`)
- `COCO_QUEUE_MODE` (`postgres` or `redis`, default `postgres`)
- `COCO_REDIS_URL` (required when queue mode is `redis`)
- `COCO_REDIS_QUEUE` (default `coco:ingest`)
- `COCO_REDIS_BLOCK_SECS` (default `2`)

## Request-level configuration

Query requests accept request-scoped configuration:

- `indexing_config_id` (selects a registered indexing config; must be normalized)
- `retrieval_config` (overrides `retrieval_mode`, `top_k`, `hybrid_alpha`, `reranker`)
  - `retrieval_config.vector_backend` is server-only and must match `COCO_VECTOR_BACKEND`
  - Local mode rejects `retrieval_config.vector_backend`

## Filters

Reserved fields are not accepted in public API filters:

- `org_id`, `project_id`, `version_id`, `config_id`

Allowed fields and operators:

- Local: `doc_id`, `chunk_id`, `content` with `eq` or `contains`
- Server: `doc_id`, `chunk_id` with `eq` or `contains`

Schema mapping:

- `doc_id` -> `documents.doc_id` / `chunks.doc_id`
- `chunk_id` -> `chunks.id`
- `content` -> `chunks.content` (local-only)

## Indexing configs

`config_id` rules:

- length 1..63 (aligned with Postgres identifier limits)
- lowercase letters, digits, `-`, `_`
- must start with a lowercase letter or digit
- trimming must not change the value

Defaults and lifecycle:

- `config_id=default` is reserved and cannot be updated
- project registration sets `active_config_id=default`
- registration fails if the default config is missing
- configs are switched by activation; referenced configs cannot be updated or deleted

Fields:

- `vector_metric` and `index_params` control vector index behavior
- Server requires `embedding.dimensions = 1536` and `vector_metric = l2`
- `vector_backend` is server-only; local mode rejects it

Registry responses include `config_id`, `chunking`, `embedding`, `vector_metric`,
`index_params`, `vector_backend`, and timestamps. List responses include
`active_config_id` when a project is specified.

## Testing

- `COCO_TEST_DB_URL` enables integration tests against a Postgres database.
