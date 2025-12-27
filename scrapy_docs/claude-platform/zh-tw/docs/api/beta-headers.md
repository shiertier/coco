# Beta 標頭

使用 Claude API 的 beta 標頭的文件

---

Beta 標頭允許您在實驗性功能和新模型功能成為標準 API 的一部分之前就能存取它們。

這些功能可能會發生變更，並可能在未來版本中被修改或移除。

<Info>
Beta 標頭通常與[客戶端 SDK 中的 beta 命名空間](/docs/zh-TW/api/client-sdks#beta-namespace-in-client-sdks)一起使用
</Info>

## 如何使用 beta 標頭

要存取 beta 功能，請在您的 API 請求中包含 `anthropic-beta` 標頭：

```http
POST /v1/messages
Content-Type: application/json
X-API-Key: YOUR_API_KEY
anthropic-beta: BETA_FEATURE_NAME
```

使用 SDK 時，您可以在請求選項中指定 beta 標頭：

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
Beta 功能是實驗性的，可能會：
- 在沒有通知的情況下發生重大變更
- 被棄用或移除
- 具有不同的速率限制或定價
- 不在所有地區提供
</Warning>

### 多個 beta 功能

要在單一請求中使用多個 beta 功能，請在標頭中包含所有功能名稱，並用逗號分隔：

```http
anthropic-beta: feature1,feature2,feature3
```

### 版本命名慣例

Beta 功能名稱通常遵循模式：`feature-name-YYYY-MM-DD`，其中日期表示 beta 版本發布的時間。請始終使用文件中記錄的確切 beta 功能名稱。

## 錯誤處理

如果您使用無效或不可用的 beta 標頭，您將收到錯誤回應：

```json
{
  "type": "error",
  "error": {
    "type": "invalid_request_error",
    "message": "Unsupported beta header: invalid-beta-name"
  }
}
```

## 獲得協助

有關 beta 功能的問題：

1. 檢查特定功能的文件
2. 查看 [API 變更日誌](/docs/zh-TW/api/versioning)以獲取更新
3. 聯繫支援以獲得生產使用方面的協助

請記住，beta 功能是「按現狀」提供的，可能沒有與穩定 API 功能相同的 SLA 保證。