# 架构概览

CoCo v3.0 遵循物理隔离与 No-DI（静态分发）原则。

## Workspace 布局

- `crates/coco-protocol`: 仅 DTO、trait 与错误类型
- `crates/coco-core`: 解析、切分、文本工具（无 I/O）
- `crates/coco-local`: 本地服务，使用 SQLite、LanceDB、ONNX Runtime
- `private/coco-server`: 服务端 API，使用 Postgres + pgvector
- `private/coco-worker`: 异步导入 Worker

## 本地模式

- 单个 Rust 服务暴露注册、导入、查询的 HTTP API。
- SQLite 存元数据；LanceDB 存向量。
- 文件系统 watcher 批量处理变更并周期性重新扫描。
- Embedding 使用 ONNX Runtime，或 `COCO_EMBEDDER_MODE=stub` 以离线运行。

## 服务端模式

- API 与 worker 是独立的二进制。
- Postgres + pgvector 存储元数据与向量。
- 多租户请求头（`x-coco-org-id`、`x-coco-project-id`）保证隔离。
- Embedding 通过外部服务生成（默认 OpenAI）。

## 设计约束

- 禁止使用 `#[cfg(feature = "server")]` 混合代码；必须物理隔离。
- 核心 crate 避免 DB 与 HTTP 依赖。
- 必须静态分发；避免运行时 `Box<dyn ...>`。
- 除非有底层需求并充分论证，禁止 `unsafe`。
