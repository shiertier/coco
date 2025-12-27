# 安装

CoCo 以 Rust workspace 形式发布，包含本地与服务端的独立二进制。

## 源码构建（本地模式）

```bash
cargo build -p coco-local --release --features local-storage
```

如果希望无需手动设置 `PROTOC`，可以使用一键构建脚本：

```bash
./scripts/build-local.sh
```

二进制为 `target/release/coco-local`。可运行：

```bash
./target/release/coco-local start
```

开发环境可用 Cargo：

```bash
cargo run -p coco-local --features local-storage -- start
```

如果已有 ONNX 模型文件，将其安装到本地缓存：

```bash
cargo run -p coco-local -- setup --model-path /path/to/model.onnx
```

## 源码构建（服务端模式）

服务端二进制位于 `private/`，需要带 pgvector 的 Postgres 数据库。

```bash
cargo build -p coco-server --release
cargo build -p coco-worker --release
```

## Docker（服务端模式）

提供了 API、worker 与数据库的 docker compose 文件：

```bash
docker compose up -d
```

如果你发布或拉取 `coco-api` 别名，请在运行 Docker Compose 前设置
`COCO_API_IMAGE=coco-api`。

如需服务端构建 embedding，请在环境中设置 `COCO_OPENAI_API_KEY`。

如果要在 worker 镜像中包含 ONNX Runtime，请用以下命令重新构建：

```bash
docker build -f Dockerfile.worker --build-arg WITH_ORT=1 -t coco-worker:ort .
```

## Helm（服务端模式）

Helm chart 与示例 values 位于 `deploy/helm/coco`：

```bash
helm install coco ./deploy/helm/coco -f ./deploy/helm/coco/values.yaml
```

## 发布产物

若使用打包发布版本，自动更新命令需要仓库引用：

```bash
export COCO_RELEASE_REPO=owner/name
coco-local update --install
```
