# Installation

CoCo ships as a Rust workspace with separate local and server binaries.

## Build from source (local mode)

```bash
cargo build -p coco-local --release --features local-storage
```

The binary is `target/release/coco-local`. You can run:

```bash
./target/release/coco-local start
```

For development, you can use Cargo:

```bash
cargo run -p coco-local --features local-storage -- start
```

If you already have an ONNX model file, install it into the local cache:

```bash
cargo run -p coco-local -- setup --model-path /path/to/model.onnx
```

## Build from source (server mode)

Server binaries are in `private/` and require a Postgres database with pgvector.

```bash
cargo build -p coco-server --release
cargo build -p coco-worker --release
```

## Docker (server mode)

A docker compose file is provided for the API, worker, and database:

```bash
docker compose up -d
```

If you publish or pull the `coco-api` alias, set `COCO_API_IMAGE=coco-api`
before running Docker Compose.

Set `COCO_OPENAI_API_KEY` in your environment if you want the server to build embeddings.

To include ONNX Runtime in the worker image, rebuild with:

```bash
docker build -f Dockerfile.worker --build-arg WITH_ORT=1 -t coco-worker:ort .
```

## Helm (server mode)

Helm chart and example values live under `deploy/helm/coco`:

```bash
helm install coco ./deploy/helm/coco -f ./deploy/helm/coco/values.yaml
```

## Release artifacts

If you use a packaged release, the auto-update command needs a repo reference:

```bash
export COCO_RELEASE_REPO=owner/name
coco-local update --install
```
