# CoCo TypeScript SDK

This SDK is generated from `openapi.json`.

## Regenerate

```
COCO_OFFLINE=1 bash scripts/generate-openapi.sh
python scripts/generate-ts-sdk.py
```

## Install

```
npm install coco-sdk
```

## Usage

Query endpoints return `ResponseEnvelope` objects. Use `data` for payloads and `meta` for status.

```ts
import { CocoClient } from "coco-sdk";

const client = new CocoClient({
  baseUrl: "http://127.0.0.1:3456",
  apiKey: "YOUR_API_KEY",
});

const headers = {
  "x-coco-org-id": "org-123",
  "x-coco-project-id": "proj-456",
};

const result = await client.queryDocuments(
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
  },
  headers
);

console.log(result.data.results.length);
console.log(result.meta.status);
```
