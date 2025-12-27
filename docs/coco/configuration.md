# 配置参考

环境变量用于配置本地、服务端和 Worker 模式，启动时读取。

历史别名：

- `COCO_DB_URL`（`COCO_META_DB` 的历史别名）
- `COCO_LANCEDB_PATH`（`COCO_VECTOR_DIR` 的历史别名）
- `COCO_MODE`（为兼容保留，但模式二进制会忽略）

## 本地模式

核心服务设置：

- `COCO_HOST`（默认 `127.0.0.1`）
- `COCO_PORT`（默认 `3456`）
- `COCO_META_DB`（默认 `$HOME/.coco/meta.db`）
- `COCO_VECTOR_DIR`（默认 `$HOME/.coco/vectors`）

模型设置：

- `COCO_MODEL_PATH`（本地 ONNX 模型文件路径）
- `COCO_MODEL_URL`（覆盖下载 URL）
- `COCO_MODEL_NAME`（默认 `all-MiniLM-L6-v2`）
- `COCO_MODEL_FILE`（默认 `all-MiniLM-L6-v2.onnx`）
- `COCO_MODEL_DIMENSIONS`（默认 `384`）
- `COCO_EMBEDDER_MODE`（`stub` 或 `mock` 以禁用下载）
- `COCO_ORT_BACKEND`（`cpu` 或 `cuda`）

切分设置：

- `COCO_CHUNK_SIZE`（默认 `256`）
- `COCO_CHUNK_OVERLAP`（默认 `32`）

实时检索设置：

- `COCO_LIVE_RETRIEVAL_ENABLED`（默认 `true`）
- `COCO_LIVE_RETRIEVAL_WINDOW_BYTES`（默认 `2048`）

实时 grep 设置：

- `COCO_LIVE_GREP_ENABLED`（默认 `false`）
- `COCO_LIVE_GREP_MAX_RESULTS`（默认 `10`）
- `COCO_LIVE_GREP_TIMEOUT_MS`（默认 `200`）

Watcher 设置：

- `COCO_WATCH_ENABLED`（默认 `1`）
- `COCO_WATCH_DEBOUNCE_MS`（默认 `300`）
- `COCO_WATCH_RESCAN_SECS`（默认 `300`）

更新设置：

- `COCO_RELEASE_REPO`（`owner/name` 形式的 GitHub 仓库）

## 服务端模式

核心服务设置：

- `COCO_HOST`（默认 `127.0.0.1`）
- `COCO_PORT`（默认 `3456`）
- `COCO_META_DB`（必填，Postgres + pgvector；`COCO_DB_URL` 为历史别名）
- `COCO_ADMIN_KEY`（必填）
- `COCO_API_KEY`（必填）

TLS 设置：

- `COCO_TLS_MODE`（`tls` 或 `proxy`，默认 `tls`）
- `COCO_TLS_CERT`（TLS 证书路径，可选）
- `COCO_TLS_KEY`（TLS 私钥路径，可选）

队列设置：

- `COCO_QUEUE_MODE`（`postgres` 或 `redis`，默认 `postgres`）
- `COCO_REDIS_URL`（队列模式为 `redis` 时必填）
- `COCO_REDIS_QUEUE`（默认 `coco:ingest`）

限流：

- `COCO_RATE_LIMIT_PER_MIN`
- `COCO_RATE_LIMIT_BURST`

组织限制：

- `COCO_ORG_MAX_DOCUMENTS`
- `COCO_ORG_MAX_CHUNKS`
- `COCO_ORG_MAX_STORAGE_BYTES`
- `COCO_ORG_MAX_EMBEDDINGS_PER_DAY`

Embedding 提供方：

- `COCO_EMBEDDING_PROVIDER`（默认 `openai`）
- `COCO_EMBEDDING_MODEL`（默认 `text-embedding-3-small`）
- `COCO_EMBEDDING_DIMENSIONS`（默认 `1536`）
- `COCO_EMBEDDING_BATCH_SIZE`（默认 `64`）
- `COCO_EMBEDDING_TIMEOUT_SECS`（默认 `30`）
- `COCO_EMBEDDING_MAX_RETRIES`（默认 `3`）
- `COCO_EMBEDDING_BACKOFF_MS`（默认 `500`）
- `COCO_EMBEDDING_BACKOFF_MAX_MS`（默认 `8000`）
- `COCO_OPENAI_API_KEY`（当提供方为 `openai` 时必填）
- `COCO_OPENAI_BASE_URL`（`openai` 的可选覆盖 URL）
- 其他提供方使用 `COCO_{PROVIDER}_API_KEY` 与 `COCO_{PROVIDER}_BASE_URL`

Worker IPC 设置：

- `COCO_WORKER_ADDR`（可选的 gRPC/IPC 地址，用于 API <-> Worker）

导入设置：

- `COCO_INGEST_BLOB_DIR`（导入 blob 载荷目录）
- `COCO_INGEST_WASM_MODULE_REF`（可选的 wasm 规则模块引用）

查询连接池设置：

- `COCO_QUERY_PG_MAX_CONNECTIONS`
- `COCO_QUERY_PG_MIN_CONNECTIONS`
- `COCO_QUERY_PG_CONNECT_TIMEOUT_SECS`

向量数据库设置：

- `COCO_VECTOR_BACKEND`（`pgvector` 或 `qdrant`，默认 `pgvector`）
- `COCO_VECTOR_DB_URL`（可选，Qdrant 端点 URL）
- `COCO_VECTOR_DB_API_KEY`（可选，Qdrant API key）
- `COCO_VECTOR_DB_COLLECTION_PREFIX`（使用 Qdrant 时必填）

## Worker 模式

- `COCO_DB_URL`（必填）
- `COCO_WORKER_POLL_MS`（默认 `500`）
- `COCO_WORKER_BATCH_SIZE`（默认 `256`）
- `COCO_WORKER_MAX_ATTEMPTS`（默认 `3`）
- `COCO_WORKER_RETRY_BACKOFF_MS`（默认 `500`）
- `COCO_WORKER_ADDR`（可选的 gRPC/IPC 地址，用于 API <-> Worker）
- `COCO_INGEST_PG_MAX_CONNECTIONS`
- `COCO_INGEST_PG_MIN_CONNECTIONS`
- `COCO_INGEST_PG_CONNECT_TIMEOUT_SECS`
- `COCO_INGEST_BLOB_DIR`（可选的导入 blob 目录）
- `COCO_INGEST_WASM_DIR`（wasm 规则模块基目录）
- `COCO_INGEST_WASM_MAX_MEMORY_BYTES`（默认由 worker 设置）
- `COCO_INGEST_WASM_MAX_FUEL`（默认由 worker 设置）
- `COCO_INGEST_WASM_FAILURE_MODE`（`fail` 或 `skip`，默认 `fail`）
- `COCO_QUEUE_MODE`（`postgres` 或 `redis`，默认 `postgres`）
- `COCO_REDIS_URL`（队列模式为 `redis` 时必填）
- `COCO_REDIS_QUEUE`（默认 `coco:ingest`）
- `COCO_REDIS_BLOCK_SECS`（默认 `2`）

## 请求级配置

查询请求支持请求级配置：

- `indexing_config_id`（选择已注册的索引配置；必须规范化）
- `retrieval_config`（覆盖 `retrieval_mode`、`top_k`、`hybrid_alpha`、`reranker`）
  - `retrieval_config.vector_backend` 仅服务端支持，且必须与 `COCO_VECTOR_BACKEND` 匹配
  - 本地模式拒绝 `retrieval_config.vector_backend`

## 过滤器

保留字段不允许出现在公开 API 的过滤条件中：

- `org_id`、`project_id`、`version_id`、`config_id`

允许的字段与操作符：

- 本地：`doc_id`、`chunk_id`、`content`，支持 `eq` 或 `contains`
- 服务端：`doc_id`、`chunk_id`，支持 `eq` 或 `contains`

Schema 映射：

- `doc_id` -> `documents.doc_id` / `chunks.doc_id`
- `chunk_id` -> `chunks.id`
- `content` -> `chunks.content`（仅本地）

## 索引配置

`config_id` 规则：

- 长度 1..63（与 Postgres 标识符限制一致）
- 小写字母、数字、`-`、`_`
- 必须以小写字母或数字开头
- 修剪空白不能改变值

默认值与生命周期：

- `config_id=default` 为保留值，不可更新
- 项目注册会设置 `active_config_id=default`
- 默认配置不存在时注册失败
- 通过激活切换配置；被引用的配置不可更新或删除

字段：

- `vector_metric` 与 `index_params` 控制向量索引行为
- 服务端要求 `embedding.dimensions = 1536` 且 `vector_metric = l2`
- `vector_backend` 仅服务端支持，本地模式拒绝

注册表响应包含 `config_id`、`chunking`、`embedding`、`vector_metric`、
`index_params`、`vector_backend` 与时间戳。列表响应在指定项目时包含
`active_config_id`。

## 测试
