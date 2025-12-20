# ベータヘッダー

Claude APIでベータヘッダーを使用するためのドキュメント

---

ベータヘッダーを使用すると、実験的な機能や新しいモデル機能が標準APIの一部になる前にアクセスできます。

これらの機能は変更される可能性があり、将来のリリースで修正または削除される場合があります。

<Info>
ベータヘッダーは、[クライアントSDKのベータ名前空間](/docs/ja/api/client-sdks#beta-namespace-in-client-sdks)と組み合わせて使用されることがよくあります
</Info>

## ベータヘッダーの使用方法

ベータ機能にアクセスするには、APIリクエストに`anthropic-beta`ヘッダーを含めます：

```http
POST /v1/messages
Content-Type: application/json
X-API-Key: YOUR_API_KEY
anthropic-beta: BETA_FEATURE_NAME
```

SDKを使用する場合、リクエストオプションでベータヘッダーを指定できます：

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
ベータ機能は実験的であり、以下の可能性があります：
- 予告なしに破壊的変更が行われる
- 非推奨または削除される
- 異なるレート制限や価格設定がある
- すべての地域で利用できない
</Warning>

### 複数のベータ機能

単一のリクエストで複数のベータ機能を使用するには、ヘッダーにすべての機能名をカンマで区切って含めます：

```http
anthropic-beta: feature1,feature2,feature3
```

### バージョン命名規則

ベータ機能名は通常、`feature-name-YYYY-MM-DD`のパターンに従います。ここで日付はベータバージョンがリリースされた時期を示します。常にドキュメントに記載されている正確なベータ機能名を使用してください。

## エラーハンドリング

無効または利用できないベータヘッダーを使用すると、エラーレスポンスが返されます：

```json
{
  "type": "error",
  "error": {
    "type": "invalid_request_error",
    "message": "Unsupported beta header: invalid-beta-name"
  }
}
```

## ヘルプの取得

ベータ機能に関する質問については：

1. 特定の機能のドキュメントを確認する
2. 更新情報について[APIチェンジログ](/docs/ja/api/versioning)を確認する
3. 本番環境での使用についてはサポートに連絡する

ベータ機能は「現状のまま」提供されており、安定したAPI機能と同じSLA保証がない場合があることを覚えておいてください。