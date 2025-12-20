# 错误

---

## HTTP 错误

我们的 API 遵循可预测的 HTTP 错误代码格式：

* 400 - `invalid_request_error`：您的请求格式或内容存在问题。我们也可能对下面未列出的其他 4XX 状态代码使用此错误类型。
* 401 - `authentication_error`：您的 API 密钥存在问题。
* 403 - `permission_error`：您的 API 密钥没有使用指定资源的权限。
* 404 - `not_found_error`：未找到请求的资源。
* 413 - `request_too_large`：请求超过了允许的最大字节数。标准 API 端点的最大请求大小为 32 MB。
* 429 - `rate_limit_error`：您的账户已达到速率限制。
* 500 - `api_error`：Anthropic 系统内部发生了意外错误。
* 529 - `overloaded_error`：API 暂时过载。

  <Warning>
  当 API 在所有用户中遇到高流量时，可能会出现 529 错误。
  
  在极少数情况下，如果您的组织使用量急剧增加，您可能会由于 API 上的加速限制而看到 429 错误。为避免达到加速限制，请逐步增加流量并保持一致的使用模式。
  </Warning>

当通过 SSE 接收[流式](/docs/zh-CN/build-with-claude/streaming)响应时，可能在返回 200 响应后发生错误，在这种情况下错误处理不会遵循这些标准机制。

## 请求大小限制

API 强制执行请求大小限制以确保最佳性能：

| 端点类型 | 最大请求大小 |
|:---|:---|
| Messages API | 32 MB |
| Token Counting API | 32 MB |
| [Batch API](/docs/zh-CN/build-with-claude/batch-processing) | 256 MB |
| [Files API](/docs/zh-CN/build-with-claude/files) | 500 MB |

如果您超过这些限制，您将收到 413 `request_too_large` 错误。该错误在请求到达我们的 API 服务器之前由 Cloudflare 返回。

## 错误形状

错误始终以 JSON 格式返回，具有顶级 `error` 对象，该对象始终包含 `type` 和 `message` 值。响应还包含一个 `request_id` 字段，以便更容易跟踪和调试。例如：

```json JSON
{
  "type": "error",
  "error": {
    "type": "not_found_error",
    "message": "The requested resource could not be found."
  },
  "request_id": "req_011CSHoEeqs5C35K2UUqR7Fy"
}
```

根据我们的[版本控制](/docs/zh-CN/api/versioning)政策，我们可能会扩展这些对象内的值，并且 `type` 值可能会随时间增长。

## 请求 ID

每个 API 响应都包含一个唯一的 `request-id` 标头。此标头包含诸如 `req_018EeWyXxfu5pfWkrYcMdjWG` 之类的值。当就特定请求联系支持时，请包含此 ID 以帮助我们快速解决您的问题。

我们的官方 SDK 将此值作为顶级响应对象上的属性提供，包含 `request-id` 标头的值：

<CodeGroup>
  ```python Python
  import anthropic

  client = anthropic.Anthropic()

  message = client.messages.create(
      model="claude-sonnet-4-5",
      max_tokens=1024,
      messages=[
          {"role": "user", "content": "Hello, Claude"}
      ]
  )
  print(f"Request ID: {message._request_id}")
  ```

  ```typescript TypeScript
  import Anthropic from '@anthropic-ai/sdk';

  const client = new Anthropic();

  const message = await client.messages.create({
    model: 'claude-sonnet-4-5',
    max_tokens: 1024,
    messages: [
      {"role": "user", "content": "Hello, Claude"}
    ]
  });
  console.log('Request ID:', message._request_id);
  ```
</CodeGroup>

## 长请求

<Warning>
 我们强烈建议对长时间运行的请求使用[流式 Messages API](/docs/zh-CN/build-with-claude/streaming)或[Message Batches API](/docs/zh-CN/api/creating-message-batches)，特别是那些超过 10 分钟的请求。
</Warning>

我们不建议在不使用我们的[流式 Messages API](/docs/zh-CN/build-with-claude/streaming)或[Message Batches API](/docs/zh-CN/api/creating-message-batches)的情况下设置大的 `max_tokens` 值：

- 某些网络可能会在可变时间段后断开空闲连接，这可能导致请求失败或超时而不从 Anthropic 接收响应。
- 网络的可靠性各不相同；我们的[Message Batches API](/docs/zh-CN/api/creating-message-batches)可以通过允许您轮询结果而不是要求不间断的网络连接来帮助您管理网络问题的风险。

如果您正在构建直接 API 集成，您应该知道设置 [TCP socket keep-alive](https://tldp.org/HOWTO/TCP-Keepalive-HOWTO/programming.html) 可以减少某些网络上空闲连接超时的影响。

我们的 [SDK](/docs/zh-CN/api/client-sdks) 将验证您的非流式 Messages API 请求预计不会超过 10 分钟超时，并且还将为 TCP keep-alive 设置套接字选项。