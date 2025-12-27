# 扩展指南

本指南覆盖不会破坏物理隔离的安全扩展点。

## 新增解析器或切分器（core）

- 在 `crates/coco-core` 实现解析或切分逻辑。
- 保持 crate 纯净：不引入 DB、HTTP、异步运行时。
- 优先选择减少 clone 与借用复杂度的数据结构。

## 新增本地存储后端

- 实现满足 `StorageBackend` 的具体类型。
- 在 `crates/coco-local` 以静态分发接入。
- 除非不可避免，避免 `Box<dyn ...>` 与运行时选择。

## 新增服务端存储后端

- 仅在 `private/coco-server` 中编写服务端专用代码。
- 使用 Postgres/pgvector 或其他网络型向量数据库。
- 不要在服务端 crate 中引入嵌入式数据库。

## 新增 embedding 提供方（服务端）

- 在 `private/coco-server` 中实现具体 embedding 客户端。
- 使用 `COCO_{PROVIDER}_API_KEY` 与 `COCO_{PROVIDER}_BASE_URL` 配置。
- 明确重试与超时，并覆盖失败路径测试。

## 新增本地 embedder

- 在 `crates/coco-local` 中实现具体 embedder。
- 保持与 `EmbeddingModel` 兼容并使用静态分发。
- 如需外部二进制，请通过显式配置进行开关控制。
