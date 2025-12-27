# Context Core (CoCo) v0.0.1

CoCo is a semantic retrieval engine for code and knowledge bases. The repository
follows Physical Separation and No-DI (static dispatch) across local and server
products.

## Repository layout

- `crates/` (open source)
  - `coco-protocol`: DTOs, traits, error types
  - `coco-core`: parsing, chunking, text utilities (no I/O)
  - `coco-local`: local service (SQLite + LanceDB + ONNX Runtime)
- `private/` (closed source)
  - `coco-server`: API service (Postgres + pgvector)
  - `coco-worker`: ingest worker
- `docs/`: product documentation
- `scrapy_docs/`: scraped reference docs (docs-crawler output)

## Quick start (local mode)

```bash
export COCO_EMBEDDER_MODE=stub
cargo run -p coco-local --features local-storage -- start --headless
```

In another terminal:

```bash
cargo run -p coco-local --features local-storage -- import /path/to/project --recursive
```

Query:

```bash
curl -s \
  -H "content-type: application/json" \
  -d '{
    "intent": {
      "query_text": "init",
      "retrieval_mode": "vector",
      "top_k": 5,
      "hybrid_alpha": 0.5,
      "filters": [],
      "reranker": null
    }
  }' \
  http://127.0.0.1:3456/v1/docs/query
```

## Server mode (Docker)

```bash
docker compose up -d
```

Set `COCO_OPENAI_API_KEY` if you want the server to generate embeddings.

To use the `coco-api` image alias, set `COCO_API_IMAGE=coco-api` before running
Docker Compose.

## Documentation

See `docs/README.md`.

## Notes

`src/docs_crawler/` is a future reference tool and not part of the CoCo build.
docs-crawler writes to `scrapy_docs/` by default; set `COCO_SCRAPY_DOCS_DIR` to override
(user `scrapy` defaults to `~/.cache/coco/scrapy_docs`).
