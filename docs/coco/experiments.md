# 实验

本地模式自带实验运行器，用于基准测试不同索引与检索策略，
针对带标注的评估集进行对比。

## 规格文件（`experiment.yml`）

实验规格是一个 YAML 文件。必填字段为 `name`、`project_id`、
`indexing_strategies`、`query_strategies` 与 `evaluation_set`。

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

注意事项：

- `indexing_strategies[].config_id` 与 `query_strategies[].query_config_id` 必须规范化。
- 本地模式拒绝 `indexing_strategies[].vector_backend` 和
  `query_strategies[].retrieval_config.vector_backend`。
- `hybrid_alpha` 对所有检索模式都是必填；`top_k` 必须 > 0。

## 运行

```bash
coco experiment run ./experiment.yml
```

可选参数：

- `--output ./results.json`（默认输出到与规格同目录的 `experiment.results.json`）
- `--host 127.0.0.1`
- `--port 3456`

运行器将：

1. 注册/更新来自 `indexing_strategies` 的索引配置。
2. 按配置重建索引并激活。
3. 对 `evaluation_set` 运行每个查询策略。
4. 输出 `results.json`。

## 对比

```bash
coco experiment compare ./results-a.json ./results-b.json
```

过滤项：

- `--indexing-config-id <id>`
- `--query-config-id <id>`
- `--retrieval-mode vector|fts|hybrid`

对比命令输出每个策略的平均指标表。

## 指标

- `recall_at_k`：期望文档出现在前 K 结果中的比例。
- `mrr`：首个期望文档的平均倒数排名。
- `hit_rate`：至少命中一个期望文档的查询比例。
- `latency_ms`：端到端查询延迟的平均值（毫秒）。

## 结果文件（`results.json`）

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

`query_config_id` 标识检索策略，而 `retrieval_mode`
记录该次运行实际使用的模式（`vector`、`fts` 或 `hybrid`），来自
运行时的 `retrieval_config`。
