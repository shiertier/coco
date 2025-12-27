# API 参考

服务端 API 通过 OpenAPI 描述，并在 `/v1` 下暴露。

## OpenAPI

- 生成的规范: `openapi.json`
- 重新生成:

```bash
scripts/generate-openapi.sh
```

服务端也在以下路径提供规范：

```
GET /v1/sys/openapi
```

## 认证

服务端接口需要 Bearer token：

- 管理员路由: `Authorization: Bearer $COCO_ADMIN_KEY`
- API 路由: `Authorization: Bearer $COCO_API_KEY`（也接受管理员 key）

API 路由还需要多租户请求头：

- `x-coco-org-id`
- `x-coco-user-id`
- `x-coco-project-id`

本地模式不需要认证请求头。

## 约定

### 响应封装

所有响应返回 `ResponseEnvelope`，包含请求级 `meta` 与载荷 `data`。
`SearchHit` 的评分元数据位于 `data.results[].meta`，而不是 `meta`。

`ResponseMeta.status` 表示请求的新鲜度：

- `fresh`: 活动配置查询的默认值。
- `stale`: 仅在服务端查询中，调用方显式请求非活动 `indexing_config_id` 时设置。

Memo 查询始终返回 `fresh`。

### SearchHit 与 ResponseMeta

`ResponseMeta` 只表示请求级状态。所有评分字段（`score`、
`quality`、`verified`）都属于 `SearchHitMeta`。

### Config ID 规则

`config_id` 在使用前必须规范化：

- 1..63 个字符
- 小写字母、数字、`-`、`_`
- 必须以小写字母或数字开头
- 修剪空白不能改变值

无效示例：`"Bad"`、`"has space"`、`"-start"`、`" default"`、长度 `> 63`。

错误示例：

```json
{ "kind": "user", "message": "config_id must be normalized" }
```

```json
{ "kind": "user", "message": "validation error: config_id exceeds max length" }
```

```json
{
  "kind": "user",
  "message": "validation error: config_id must start with a lowercase letter or digit"
}
```

## 示例

### 响应封装（服务端，fresh）

示例查询响应（服务端）：

```json
{
  "meta": {
    "status": "fresh"
  },
  "data": {
    "results": [
      {
        "meta": {
          "score": 0.42,
          "quality": null,
          "verified": false
        },
        "chunk": {
          "id": "chunk-1",
          "doc_id": "doc-1",
          "content": "Hello CoCo",
          "embedding": null,
          "span": { "start": 0, "end": 10 },
          "quality_score": null,
          "verified": false
        }
      }
    ]
  }
}
```

本地模式在命中元数据与 chunk 载荷中都保持 `quality` 和 `verified` 为 `null`。

### 响应封装（服务端，stale）

当显式使用非活动 `indexing_config_id` 时，响应标记为 stale：

```json
{
  "meta": { "status": "stale" },
  "data": {
    "results": [
      {
        "meta": { "score": 0.42, "quality": null, "verified": false },
        "chunk": {
          "id": "chunk-1",
          "doc_id": "doc-1",
          "content": "Hello CoCo",
          "embedding": null,
          "span": { "start": 0, "end": 10 },
          "quality_score": null,
          "verified": false
        }
      }
    ]
  }
}
```

### 响应封装（memo 查询）

Memo 查询返回 `ResponseEnvelope` 且始终 `fresh`：

```json
{
  "meta": { "status": "fresh" },
  "data": { "results": [] }
}
```

### 响应封装（本地，quality 为 null）

本地模式保持 `quality`/`verified` 字段为 `null`：

```json
{
  "meta": { "status": "fresh" },
  "data": {
    "results": [
      {
        "meta": { "score": 0.42, "quality": null, "verified": null },
        "chunk": {
          "id": "chunk-1",
          "doc_id": "doc-1",
          "content": "Hello CoCo",
          "embedding": null,
          "span": { "start": 0, "end": 10 },
          "quality_score": null,
          "verified": null
        }
      }
    ]
  }
}
```

### 错误响应

错误**不会**被响应封装包裹；它们直接返回 `ErrorResponse`
（`kind` + `message`）：

```json
{
  "kind": "user",
  "message": "indexing_config_id must be normalized"
}
```

`ErrorResponse.message` 是稳定、对外公开且安全的英文字符串，
不包含内部错误细节。

## 破坏性变更

- 查询响应现在被 `ResponseEnvelope`（`meta` + `data`）包裹。客户端需要
  读取 `data.results` 而不是顶层 `results`。
