# Beta 头部

使用 Claude API 的 beta 头部的文档

---

Beta 头部允许您在实验性功能和新模型能力成为标准 API 的一部分之前访问它们。

这些功能可能会发生变化，并可能在未来版本中被修改或删除。

<Info>
Beta 头部通常与[客户端 SDK 中的 beta 命名空间](/docs/zh-CN/api/client-sdks#beta-namespace-in-client-sdks)结合使用
</Info>

## 如何使用 beta 头部

要访问 beta 功能，请在您的 API 请求中包含 `anthropic-beta` 头部：

```http
POST /v1/messages
Content-Type: application/json
X-API-Key: YOUR_API_KEY
anthropic-beta: BETA_FEATURE_NAME
```

使用 SDK 时，您可以在请求选项中指定 beta 头部：

<CodeGroup>

```python Python
from anthropic import Anthropic

client = Anthropic()

response = client.beta.messages.create(
    model="claude-sonnet-4-5",
    max_tokens=1024,
    messages=[
        {"role": "user", "content": "Hello, Claude"}
    ],
    betas=["beta-feature-name"]
)
```

```typescript TypeScript
import Anthropic from '@anthropic-ai/sdk';

const anthropic = new Anthropic();

const msg = await anthropic.beta.messages.create({
  model: 'claude-sonnet-4-5',
  max_tokens: 1024,
  messages: [
    { role: 'user', content: 'Hello, Claude' }
  ],
  betas: ['beta-feature-name']
});
```

```bash cURL
curl https://api.anthropic.com/v1/messages \
  -H "x-api-key: $ANTHROPIC_API_KEY" \
  -H "anthropic-version: 2023-06-01" \
  -H "anthropic-beta: beta-feature-name" \
  -H "content-type: application/json" \
  -d '{
    "model": "claude-sonnet-4-5",
    "max_tokens": 1024,
    "messages": [
      {"role": "user", "content": "Hello, Claude"}
    ]
  }'
```

</CodeGroup>

<Warning>
Beta 功能是实验性的，可能会：
- 在没有通知的情况下发生破坏性变更
- 被弃用或删除
- 具有不同的速率限制或定价
- 在所有地区都不可用
</Warning>

### 多个 beta 功能

要在单个请求中使用多个 beta 功能，请在头部中包含所有功能名称，用逗号分隔：

```http
anthropic-beta: feature1,feature2,feature3
```

### 版本命名约定

Beta 功能名称通常遵循模式：`feature-name-YYYY-MM-DD`，其中日期表示 beta 版本发布的时间。始终使用文档中记录的确切 beta 功能名称。

## 错误处理

如果您使用无效或不可用的 beta 头部，您将收到错误响应：

```json
{
  "type": "error",
  "error": {
    "type": "invalid_request_error",
    "message": "Unsupported beta header: invalid-beta-name"
  }
}
```

## 获取帮助

有关 beta 功能的问题：

1. 查看特定功能的文档
2. 查看 [API 更新日志](/docs/zh-CN/api/versioning)以获取更新
3. 联系支持以获得生产使用方面的帮助

请记住，beta 功能是"按原样"提供的，可能没有与稳定 API 功能相同的 SLA 保证。