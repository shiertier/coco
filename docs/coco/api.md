# API Reference

The server API is documented via OpenAPI and exposed under `/v1`.

## OpenAPI

- Generated spec: `openapi.json`
- Regenerate:

```bash
scripts/generate-openapi.sh
```

Server also exposes the spec at:

```
GET /v1/sys/openapi
```

## Authentication

Server endpoints require bearer tokens:

- Admin routes: `Authorization: Bearer $COCO_ADMIN_KEY`
- API routes: `Authorization: Bearer $COCO_API_KEY` (admin key also accepted)

Multi-tenant headers are required for API routes:

- `x-coco-org-id`
- `x-coco-user-id`
- `x-coco-project-id`

Local mode does not require auth headers.

## Conventions

### Response envelope

All responses return a `ResponseEnvelope` with a request-level `meta` and a payload `data`.
`SearchHit` scoring metadata lives under `data.results[].meta`, not under `meta`.

`ResponseMeta.status` indicates request freshness:

- `fresh`: default for active config queries.
- `stale`: only set on server queries when the caller explicitly requests a
  non-active `indexing_config_id`.

Memo queries always return `fresh`.

### SearchHit vs ResponseMeta

`ResponseMeta` is request-level status only. All scoring fields (`score`,
`quality`, `verified`) belong to `SearchHitMeta`.

### Config ID rules

`config_id` must be normalized before use:

- 1..63 chars
- lowercase letters, digits, `-`, `_`
- must start with lowercase letter or digit
- trimming must not change the value

Invalid examples: `"Bad"`, `"has space"`, `"-start"`, `" default"`, length `> 63`.

Error examples:

```json
{ "kind": "user", "message": "config_id must be normalized" }
```

```json
{ "kind": "user", "message": "validation error: config_id exceeds max length" }
```

```json
{
  "kind": "user",
  "message": "validation error: config_id must start with a lowercase letter or digit"
}
```

## Examples

### Response envelope (server, fresh)

Example query response (server):

```json
{
  "meta": {
    "status": "fresh"
  },
  "data": {
    "results": [
      {
        "meta": {
          "score": 0.42,
          "quality": null,
          "verified": false
        },
        "chunk": {
          "id": "chunk-1",
          "doc_id": "doc-1",
          "content": "Hello CoCo",
          "embedding": null,
          "span": { "start": 0, "end": 10 },
          "quality_score": null,
          "verified": false
        }
      }
    ]
  }
}
```

Local mode leaves `quality` and `verified` as `null` in both the hit metadata and chunk payload.

### Response envelope (server, stale)

When an explicit non-active `indexing_config_id` is used, the response is marked stale:

```json
{
  "meta": { "status": "stale" },
  "data": {
    "results": [
      {
        "meta": { "score": 0.42, "quality": null, "verified": false },
        "chunk": {
          "id": "chunk-1",
          "doc_id": "doc-1",
          "content": "Hello CoCo",
          "embedding": null,
          "span": { "start": 0, "end": 10 },
          "quality_score": null,
          "verified": false
        }
      }
    ]
  }
}
```

### Response envelope (memo query)

Memo queries return a `ResponseEnvelope` and always `fresh`:

```json
{
  "meta": { "status": "fresh" },
  "data": { "results": [] }
}
```

### Response envelope (local, quality null)

Local mode keeps `quality`/`verified` fields as `null`:

```json
{
  "meta": { "status": "fresh" },
  "data": {
    "results": [
      {
        "meta": { "score": 0.42, "quality": null, "verified": null },
        "chunk": {
          "id": "chunk-1",
          "doc_id": "doc-1",
          "content": "Hello CoCo",
          "embedding": null,
          "span": { "start": 0, "end": 10 },
          "quality_score": null,
          "verified": null
        }
      }
    ]
  }
}
```

### Error responses

Errors are **not** wrapped in a response envelope; they return `ErrorResponse`
directly with `kind` + `message`:

```json
{
  "kind": "user",
  "message": "indexing_config_id must be normalized"
}
```

`ErrorResponse.message` is a stable, public-safe English string and does not
include internal error details.

## Breaking changes

- Query responses are now wrapped in `ResponseEnvelope` (`meta` + `data`). Update clients to
  read `data.results` instead of top-level `results`.
- Access log entries include `schema_version`; any change to its value is a breaking change
  for log consumers.

## Versioning policy

- The public API stays under `/v1` until a breaking change requires a `/v2` migration.
- SDK major versions track breaking API changes and `schema_version` bumps.

## Register a project (server)

Register responses include:

- `project_id`: canonical project UUID (server + local, used in `x-coco-project-id`).
- `active_config_id`: the active indexing config (defaults to `default`).
- `active_version_id`: server-only ingest version (nullable).
- Server responses also include `org_id` (used in `x-coco-org-id`).
- Local mode also returns `path` and `created_at`.

Server requests require `org_id` and `user_id`; `project_id` is optional.

`source_ref` must be a logical identifier (not a filesystem path). Examples that are rejected:

- `/Users/alice/repo`
- `C:\repo\docs`

```bash
curl -s \
  -H "authorization: Bearer ${COCO_ADMIN_KEY}" \
  -H "content-type: application/json" \
  -d '{"org_id":"acme","user_id":"user-1","project_id":"repo","name":"Repo","source_ref":"git:repo"}' \
  http://127.0.0.1:3456/v1/sys/register
```

## Indexing configs (server + local)

Indexing configs must be registered before import/query. Inline `indexing_config`
payloads are rejected; use `/v1/sys/configs` + `indexing_config_id`.

### Endpoints

- `GET /v1/sys/configs` (server: admin auth; local: no auth)
  - Optional `project_id` query param (local) or `project_id` + `org_id` (server)
  - Response includes `active_config_id` when `project_id` is provided
- `POST /v1/sys/configs` (server: admin auth; local: no auth)
- `POST /v1/sys/configs/activate` (server: admin auth; local: no auth)

### Config example

```json
{
  "config": {
    "config_id": "fast-v1",
    "chunking": { "strategy": "fixed_token", "size": 256, "overlap": 32 },
    "embedding": { "model": "default", "dimensions": 1536 },
    "vector_metric": "cosine",
    "index_params": { "hnsw_m": 16, "hnsw_ef_construction": 100 }
  }
}
```

### Activate example

```json
{ "project_id": "proj-123", "config_id": "fast-v1" }
```

## Filters (Local vs Server)

Filter operators are restricted to `eq` and `contains` in both modes.

| Mode  | Allowed fields                  | Allowed ops        |
|-------|---------------------------------|--------------------|
| Local | `doc_id`, `chunk_id`, `content` | `eq`, `contains`   |
| Server| `doc_id`, `chunk_id`            | `eq`, `contains`   |

Other operators (`neq`, `gt`, `lt`, `in`, ...) are rejected in public API input.

## Memo query (server)

`POST /v1/memo/query` requires:

- `Authorization: Bearer $COCO_API_KEY`
- non-empty `session_token`
- no org/user/project headers (ignored if present)

Not supported:

- `indexing_config_id`
- `retrieval_config.vector_backend`

Memo responses are always `meta.status = fresh`.

## Errors

### Error kinds

`CocoErrorKind` values:

- `system`: IO/config/runtime issues
- `user`: validation/input errors
- `network`: upstream/network failures
- `storage`: DB/vector backend failures
- `compute`: model/execution failures

### Error kind to HTTP status

| kind     | status |
|----------|--------|
| user     | 400    |
| network  | 502    |
| storage  | 503    |
| system   | 500    |
| compute  | 500    |

### Trace IDs

`trace_id` is emitted **only** in access logs. It is not returned in response
bodies or headers. `request_id` is not used.

`trace_id` uses UUIDv7 (sortable UUID) format.

## Access log schema

Schema versioning is strict; a bump to `schema_version` indicates a breaking change.

For `schema_version = 1`, fields are:

- `schema_version` (number)
- `ts` (RFC-3339 millis, UTC)
- `trace_id` (UUIDv7)
- `method` (HTTP method)
- `path` (request path)
- `status` (HTTP status code)
- `latency_ms` (integer milliseconds)

### Batch ingest (server)

Embeddings must match the configured dimension (default 1536). The example uses a shortened
vector for readability; replace it with a full-length embedding in real requests.

```bash
curl -s \
  -H "authorization: Bearer ${COCO_API_KEY}" \
  -H "x-coco-org-id: acme" \
  -H "x-coco-user-id: user-1" \
  -H "x-coco-project-id: repo" \
  -H "content-type: application/json" \
  -d '{
    "activate": true,
    "documents": [
      {
        "doc_id": "doc-1",
        "source_ref": "git:repo#readme.md",
        "title": "README",
        "content_hash": "abc123",
        "chunks": [
          {
            "chunk_id": "chunk-1",
            "content": "Hello CoCo",
            "embedding": [0.0, 0.0, 0.0],
            "start": 0,
            "end": 10
          }
        ]
      }
    ]
  }' \
  http://127.0.0.1:3456/v1/ingest/batch
```

### Query (server)

```bash
curl -s \
  -H "authorization: Bearer ${COCO_API_KEY}" \
  -H "x-coco-org-id: acme" \
  -H "x-coco-user-id: user-1" \
  -H "x-coco-project-id: repo" \
  -H "content-type: application/json" \
  -d '{
    "indexing_config_id": "fast-v1",
    "intent": {
      "query_text": "readme",
      "retrieval_mode": "vector",
      "top_k": 5,
      "hybrid_alpha": 0.5,
      "filters": [],
      "reranker": null
    }
  }' \
  http://127.0.0.1:3456/v1/docs/query
```

### Query (local)

```bash
curl -s \
  -H "content-type: application/json" \
  -d '{
    "intent": {
      "query_text": "readme",
      "retrieval_mode": "vector",
      "top_k": 5,
      "hybrid_alpha": 0.5,
      "filters": [],
      "reranker": null
    }
  }' \
  http://127.0.0.1:3456/v1/docs/query
```

Local mode rejects explicit non-active configs:

```json
{
  "kind": "user",
  "message": "indexing_config_id does not match active config"
}
```
