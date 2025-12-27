# Context Core (CoCo) v0.0.1

CoCo 是一个面向代码与知识库的语义检索引擎。本仓库遵循 Physical Separation 与 No-DI（静态分发）设计，在 local 与 server 产品之间严格隔离。

## 仓库结构

- `crates/`（开源）
  - `coco-protocol`: DTO、Trait 接口、错误类型
  - `coco-core`: 解析、分块、文本工具（无 I/O）
  - `coco-local`: 本地服务（SQLite + LanceDB + ONNX Runtime）
  - `docs-crawler`: 文档抓取与预处理工具
- `private/`（闭源）
  - `coco-server`: API 服务（Postgres + pgvector）
  - `coco-worker`: 异步 Worker
- `docs/`: 产品文档
- `scrapy_docs/`: docs-crawler 输出的抓取结果

## 快速开始（本地模式）

```bash
export COCO_EMBEDDER_MODE=stub
cargo run -p coco-local --features local-storage -- start --headless
```

另起一个终端：

```bash
cargo run -p coco-local --features local-storage -- import /path/to/project --recursive
```

查询：

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

## 开发者安装（本地）

一键构建（自动使用 vendored `protoc`）：

```bash
./scripts/release/build-local.sh
```

产物：`target/release/coco-local`。

手动构建：

```bash
cargo build -p coco-local --release --features local-storage
```

## 服务端模式（Docker）

```bash
docker compose up -d
```

如果需要服务端生成向量，设置 `COCO_OPENAI_API_KEY`。

如需使用 `coco-api` 镜像别名，在运行 Docker Compose 之前设置 `COCO_API_IMAGE=coco-api`。

## 文档

见 `docs/README.md`。

## 备注

docs-crawler 默认写入 `scrapy_docs/`；可通过 `COCO_SCRAPY_DOCS_DIR` 覆盖。
`scrapy` 用户默认目录为 `~/.cache/coco/scrapy_docs`。
