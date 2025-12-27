# Experiments

Local mode ships an experiment runner to benchmark indexing and retrieval strategies
against a labeled evaluation set.

## Spec file (`experiment.yml`)

The experiment spec is a YAML file. Required fields are `name`, `project_id`,
`indexing_strategies`, `query_strategies`, and `evaluation_set`.

```yaml
name: "baseline"
project_id: "repo"
dataset_revision: "v1"
dataset_commit: "abc123"
random_seed: 42
concurrency: 4
indexing_strategies:
  - config_id: "default"
    chunking:
      strategy_name: "fixed_token"
      chunk_size: 256
      chunk_overlap: 32
    embedding:
      model_name: "default"
      dimensions: 384
    vector_metric: "cosine"
query_strategies:
  - query_config_id: "hybrid-k5"
    retrieval_config:
      retrieval_mode: "hybrid"
      top_k: 5
      hybrid_alpha: 0.5
      reranker: null
evaluation_set:
  - question_id: "q1"
    query: "how to run"
    expected_doc_ids: ["doc-1"]
metrics:
  - kind: "recall_at_k"
    k: 5
  - kind: "mrr"
  - kind: "hit_rate"
  - kind: "latency_ms"
```

Notes:

- `indexing_strategies[].config_id` and `query_strategies[].query_config_id` must be normalized.
- Local mode rejects `indexing_strategies[].vector_backend` and
  `query_strategies[].retrieval_config.vector_backend`.
- `hybrid_alpha` is required for all retrieval modes; `top_k` must be > 0.

## Run

```bash
coco experiment run ./experiment.yml
```

Optional flags:

- `--output ./results.json` (defaults to `experiment.results.json` next to the spec)
- `--host 127.0.0.1`
- `--port 3456`

The runner will:

1. Register/update indexing configs from `indexing_strategies`.
2. Reindex the project per config and activate it.
3. Run each query strategy against `evaluation_set`.
4. Emit `results.json`.

## Compare

```bash
coco experiment compare ./results-a.json ./results-b.json
```

Filters:

- `--indexing-config-id <id>`
- `--query-config-id <id>`
- `--retrieval-mode vector|fts|hybrid`

The compare command prints a table of mean metric values per strategy.

## Metrics

- `recall_at_k`: fraction of expected documents appearing in the top-K results.
- `mrr`: mean reciprocal rank of the first expected document.
- `hit_rate`: fraction of queries with at least one expected document returned.
- `latency_ms`: mean end-to-end query latency in milliseconds.

## Results file (`results.json`)

```json
{
  "name": "baseline",
  "project_id": "repo",
  "dataset_revision": "v1",
  "dataset_commit": "abc123",
  "random_seed": 42,
  "concurrency": 4,
  "run_at": "2024-01-01T00:00:00.000Z",
  "results": [
    {
      "indexing_config_id": "default",
      "query_config_id": "hybrid-k5",
      "retrieval_mode": "hybrid",
      "version_id": null,
      "metrics": [
        { "kind": "recall_at_k", "k": 5, "value": 0.8 },
        { "kind": "mrr", "k": null, "value": 0.6 },
        { "kind": "hit_rate", "k": null, "value": 0.9 },
        { "kind": "latency_ms", "k": null, "value": 12.3 }
      ]
    }
  ]
}
```

`query_config_id` identifies the retrieval strategy, while `retrieval_mode`
captures the concrete mode (`vector`, `fts`, or `hybrid`) from the
`retrieval_config` used during the run.
