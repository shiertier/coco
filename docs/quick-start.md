# 快速开始

本指南启动 CoCo 本地模式，导入项目并执行查询。

## 前置条件

- Rust 工具链 1.85+（`rustup show`）
- Git 与可用的 Cargo 环境

## 1. 启动本地服务

在一个终端中：

```bash
export COCO_EMBEDDER_MODE=stub
cargo run -p coco-local --features local-storage -- start --headless
```

- `COCO_EMBEDDER_MODE=stub` 用于避免下载 ONNX 模型。
- 若要使用默认模型下载流程，请去掉该环境变量。

检查健康状态：

```bash
curl -s http://127.0.0.1:3456/v1/sys/health
```

## 2. 导入项目

在另一个终端中：

```bash
cargo run -p coco-local --features local-storage -- import /path/to/project --recursive
```

## 3. 查询

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

## 下一步

- 在 `configuration.md` 调整配置。
- 服务端模式接口请看 `api.md`。
