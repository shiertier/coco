# CoCo Python SDK

This SDK is generated from `openapi.json` and uses the Python standard library.

## Regenerate

```
COCO_OFFLINE=1 bash scripts/generate-openapi.sh
python scripts/generate-py-sdk.py
```

## Install

```
pip install coco-sdk
```

## Usage

Query endpoints return response envelopes. Access payloads through `result["data"]` and metadata through `result["meta"]`.

```python
from coco_sdk import CocoClient

client = CocoClient(
    base_url="http://127.0.0.1:3456",
    api_key="YOUR_API_KEY",
)

headers = {
    "x-coco-org-id": "org-123",
    "x-coco-project-id": "proj-456",
}

result = client.queryDocuments(
    {
        "intent": {
            "query_text": "how to start the service",
            "query_embedding": None,
            "retrieval_mode": "vector",
            "top_k": 10,
            "hybrid_alpha": 1.0,
            "filters": [],
            "reranker": None,
        }
    },
    headers,
)

print(len(result["data"]["results"]))
print(result["meta"]["status"])
```
