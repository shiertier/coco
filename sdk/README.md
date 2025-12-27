# CoCo TypeScript SDK

This SDK is generated from `openapi.json` via `openapi-typescript-codegen`.

## Regenerate

```
COCO_OFFLINE=1 bash scripts/generate-openapi.sh
python3 scripts/generate-ts-sdk.py
```

The generator is downloaded automatically using `npm` into `scripts/.cache`.
Node.js and npm are required. Use `OPENAPI_TS_CODEGEN_VERSION` to pin the generator version.

Regeneration may change the public API surface; use the generated docs in this directory after running the script.

## Install

```
npm install coco-sdk
```

## Usage

Service class names are derived from OpenAPI tags; use the generated `services/` exports to call endpoints.

```ts
import { DocsService, OpenAPI } from "coco-sdk";

OpenAPI.BASE = "http://127.0.0.1:3456";
OpenAPI.TOKEN = "YOUR_API_KEY";

const result = await DocsService.queryDocuments(
  "org-123",
  "user-789",
  "proj-456",
  {
    intent: {
      query_text: "how to start the service",
      query_embedding: null,
      retrieval_mode: "vector",
      top_k: 10,
      hybrid_alpha: 1.0,
      filters: [],
      reranker: null,
    },
  }
);

console.log(result.data.results.length);
console.log(result.meta.status);
```
