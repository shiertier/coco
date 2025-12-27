# Quick Start

This guide spins up CoCo local mode, imports a project, and runs a query.

## Prerequisites

- Rust toolchain 1.85+ (`rustup show`)
- Git and a working Cargo setup

## 1. Start the local service

In one terminal:

```bash
export COCO_EMBEDDER_MODE=stub
cargo run -p coco-local --features local-storage -- start --headless
```

- `COCO_EMBEDDER_MODE=stub` avoids downloading an ONNX model.
- Omit the env var to use the default model download flow.

Check health:

```bash
curl -s http://127.0.0.1:3456/v1/sys/health
```

## 2. Import a project

In another terminal:

```bash
cargo run -p coco-local --features local-storage -- import /path/to/project --recursive
```

## 3. Query

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

## Next steps

- Tune configuration in configuration.md.
- Use the API docs in api.md for server mode endpoints.
