# CoCo Python SDK

This SDK is generated from `openapi.json` via `openapi-python-client`.

## Regenerate

```
COCO_OFFLINE=1 bash scripts/generate-openapi.sh
python3 scripts/generate-py-sdk.py
```

The generator is installed automatically using `pip` into `scripts/.cache`.
Python is required. Use `OPENAPI_PYTHON_CLIENT_VERSION` to pin the generator version.

Regeneration may change the public API surface; use the generated docs in this directory after running the script.

## Install

```
pip install coco-sdk
```

## Usage

Use the generated `api/` modules and the `Client` entrypoint; see the generated package for available endpoints.

```python
from coco_sdk import Client
from coco_sdk.api.docs import query_documents
from coco_sdk.models import PublicSearchIntent, QueryRequest, RetrievalMode

client = Client(
    base_url="http://127.0.0.1:3456",
    token="YOUR_API_KEY",
)

intent = PublicSearchIntent(
    query_text="how to start the service",
    query_embedding=None,
    retrieval_mode=RetrievalMode.VECTOR,
    top_k=10,
    hybrid_alpha=1.0,
    filters=[],
    reranker=None,
)
payload = QueryRequest(intent=intent)
response = query_documents.sync(
    client=client,
    body=payload,
    x_coco_org_id="org-123",
    x_coco_user_id="user-789",
    x_coco_project_id="proj-456",
)

print(response.data.results)
```
