# Vertex AI上のClaude

AnthropicのClaudeモデルは、[Vertex AI](https://cloud.google.com/vertex-ai)を通じて一般提供されています。

---

Claudeにアクセスするための Vertex APIは、[Messages API](/docs/ja/api/messages)とほぼ同じであり、同じすべてのオプションをサポートしていますが、2つの主な違いがあります：

* Vertexでは、`model`はリクエストボディで渡されません。代わりに、Google Cloudエンドポイント URLで指定されます。
* Vertexでは、`anthropic_version`はリクエストボディで渡され（ヘッダーではなく）、値`vertex-2023-10-16`に設定する必要があります。

Vertexは、Anthropicの公式[クライアント SDK](/docs/ja/api/client-sdks)によってもサポートされています。このガイドでは、PythonまたはTypeScriptのいずれかで、Vertex AI上のClaudeにリクエストを行うプロセスについて説明します。

このガイドでは、Vertex AIを使用できるGCPプロジェクトが既にあることを前提としています。セットアップに必要な情報と完全なウォークスルーについては、[Anthropicから Claude 3モデルを使用する](https://cloud.google.com/vertex-ai/generative-ai/docs/partner-models/use-claude)を参照してください。

## Vertex AIにアクセスするためのSDKをインストールする

まず、選択した言語用のAnthropicの[クライアント SDK](/docs/ja/api/client-sdks)をインストールします。

<CodeGroup>
  ```python Python
  pip install -U google-cloud-aiplatform "anthropic[vertex]"
  ```

  ```typescript TypeScript
  npm install @anthropic-ai/vertex-sdk
  ```
</CodeGroup>

## Vertex AIへのアクセス

### モデルの可用性

Anthropicモデルの可用性はリージョンによって異なることに注意してください。[Vertex AI Model Garden](https://cloud.google.com/model-garden)で「Claude」を検索するか、[Claude 3を使用する](https://cloud.google.com/vertex-ai/generative-ai/docs/partner-models/use-claude)にアクセスして、最新情報を確認してください。

#### APIモデルID

| モデル                          | Vertex AI APIモデルID |
| ------------------------------ | ------------------------ |
| Claude Sonnet 4.5              | claude-sonnet-4-5@20250929 |
| Claude Sonnet 4                | claude-sonnet-4@20250514 |
| Claude Sonnet 3.7 <Tooltip tooltipContent="2025年10月28日時点で非推奨。">⚠️</Tooltip> | claude-3-7-sonnet@20250219 |
| Claude Opus 4.5                | claude-opus-4-5@20251101 |
| Claude Opus 4.1                | claude-opus-4-1@20250805 |
| Claude Opus 4                  | claude-opus-4@20250514   |
| Claude Opus 3 <Tooltip tooltipContent="2025年6月30日時点で非推奨。">⚠️</Tooltip> | claude-3-opus@20240229   |
| Claude Haiku 4.5               | claude-haiku-4-5@20251001 |
| Claude Haiku 3.5 <Tooltip tooltipContent="2025年12月19日時点で非推奨。">⚠️</Tooltip> | claude-3-5-haiku@20241022 |
| Claude Haiku 3                 | claude-3-haiku@20240307  |

### リクエストの作成

リクエストを実行する前に、`gcloud auth application-default login`を実行してGCPで認証する必要がある場合があります。

以下の例は、Vertex AI上のClaudeからテキストを生成する方法を示しています：
<CodeGroup>

  ```python Python
  from anthropic import AnthropicVertex

  project_id = "MY_PROJECT_ID"
  region = "global"

  client = AnthropicVertex(project_id=project_id, region=region)

  message = client.messages.create(
      model="claude-sonnet-4-5@20250929",
      max_tokens=100,
      messages=[
          {
              "role": "user",
              "content": "Hey Claude!",
          }
      ],
  )
  print(message)
  ```

  ```typescript TypeScript
  import { AnthropicVertex } from '@anthropic-ai/vertex-sdk';

  const projectId = 'MY_PROJECT_ID';
  const region = 'global';

  // Goes through the standard `google-auth-library` flow.
  const client = new AnthropicVertex({
    projectId,
    region,
  });

  async function main() {
    const result = await client.messages.create({
      model: 'claude-sonnet-4-5@20250929',
      max_tokens: 100,
      messages: [
        {
          role: 'user',
          content: 'Hey Claude!',
        },
      ],
    });
    console.log(JSON.stringify(result, null, 2));
  }

  main();
  ```

  ```bash Shell
  MODEL_ID=claude-sonnet-4-5@20250929
  LOCATION=global
  PROJECT_ID=MY_PROJECT_ID

  curl \
  -X POST \
  -H "Authorization: Bearer $(gcloud auth print-access-token)" \
  -H "Content-Type: application/json" \
  https://$LOCATION-aiplatform.googleapis.com/v1/projects/${PROJECT_ID}/locations/${LOCATION}/publishers/anthropic/models/${MODEL_ID}:streamRawPredict -d \
  '{
    "anthropic_version": "vertex-2023-10-16",
    "messages": [{
      "role": "user",
      "content": "Hey Claude!"
    }],
    "max_tokens": 100,
  }'
  ```
</CodeGroup>

詳細については、[クライアント SDK](/docs/ja/api/client-sdks)と公式の[Vertex AI ドキュメント](https://cloud.google.com/vertex-ai/docs)を参照してください。

## アクティビティログ

Vertexは、[リクエスト-レスポンスログサービス](https://cloud.google.com/vertex-ai/generative-ai/docs/multimodal/request-response-logging)を提供しており、顧客はプロンプトと使用に関連する完了をログに記録できます。

Anthropicは、アクティビティを理解し、潜在的な悪用を調査するために、少なくとも30日間のローリングベースでアクティビティをログに記録することをお勧めします。

<Note>
このサービスを有効にしても、GoogleやAnthropicはコンテンツへのアクセスを取得しません。
</Note>

## 機能サポート
Vertexで現在サポートされているすべての機能は、[ここ](/docs/ja/api/overview)で確認できます。

## グローバルエンドポイント対地域エンドポイント

**Claude Sonnet 4.5およびすべての将来のモデル**から始まり、Google Vertex AIは2つのエンドポイントタイプを提供します：

- **グローバルエンドポイント**：最大可用性のための動的ルーティング
- **地域エンドポイント**：特定の地理的領域を通じた保証されたデータルーティング

地域エンドポイントには、グローバルエンドポイントよりも10%の価格プレミアムが含まれます。

<Note>
これはClaude Sonnet 4.5および将来のモデルにのみ適用されます。古いモデル（Claude Sonnet 4、Opus 4、およびそれ以前）は、既存の価格構造を維持しています。
</Note>

### 各オプションを使用する場合

**グローバルエンドポイント（推奨）：**
- 最大可用性とアップタイムを提供します
- リクエストを利用可能な容量があるリージョンに動的にルーティングします
- 価格プレミアムなし
- データレジデンシーが柔軟なアプリケーションに最適です
- 従量課金トラフィックのみをサポートします（プロビジョニングされたスループットには地域エンドポイントが必要です）

**地域エンドポイント：**
- トラフィックを特定の地理的領域を通じてルーティングします
- データレジデンシーとコンプライアンス要件に必要です
- 従量課金とプロビジョニングされたスループットの両方をサポートします
- 10%の価格プレミアムは、専用地域容量のインフラストラクチャコストを反映しています

### 実装

**グローバルエンドポイントを使用する（推奨）：**

クライアントを初期化するときに、`region`パラメータを`"global"`に設定します：

<CodeGroup>
```python Python
from anthropic import AnthropicVertex

project_id = "MY_PROJECT_ID"
region = "global"

client = AnthropicVertex(project_id=project_id, region=region)

message = client.messages.create(
    model="claude-sonnet-4-5@20250929",
    max_tokens=100,
    messages=[
        {
            "role": "user",
            "content": "Hey Claude!",
        }
    ],
)
print(message)
```

```typescript TypeScript
import { AnthropicVertex } from '@anthropic-ai/vertex-sdk';

const projectId = 'MY_PROJECT_ID';
const region = 'global';

const client = new AnthropicVertex({
  projectId,
  region,
});

const result = await client.messages.create({
  model: 'claude-sonnet-4-5@20250929',
  max_tokens: 100,
  messages: [
    {
      role: 'user',
      content: 'Hey Claude!',
    },
  ],
});
```
</CodeGroup>

**地域エンドポイントを使用する：**

`"us-east1"`や`"europe-west1"`などの特定のリージョンを指定します：

<CodeGroup>
```python Python
from anthropic import AnthropicVertex

project_id = "MY_PROJECT_ID"
region = "us-east1"  # Specify a specific region

client = AnthropicVertex(project_id=project_id, region=region)

message = client.messages.create(
    model="claude-sonnet-4-5@20250929",
    max_tokens=100,
    messages=[
        {
            "role": "user",
            "content": "Hey Claude!",
        }
    ],
)
print(message)
```

```typescript TypeScript
import { AnthropicVertex } from '@anthropic-ai/vertex-sdk';

const projectId = 'MY_PROJECT_ID';
const region = 'us-east1';  // Specify a specific region

const client = new AnthropicVertex({
  projectId,
  region,
});

const result = await client.messages.create({
  model: 'claude-sonnet-4-5@20250929',
  max_tokens: 100,
  messages: [
    {
      role: 'user',
      content: 'Hey Claude!',
    },
  ],
});
```
</CodeGroup>

### 追加リソース

- **Google Vertex AI価格：** [cloud.google.com/vertex-ai/generative-ai/pricing](https://cloud.google.com/vertex-ai/generative-ai/pricing)
- **Claudeモデルドキュメント：** [Vertex AI上のClaude](https://cloud.google.com/vertex-ai/generative-ai/docs/partner-models/claude)
- **Googleブログ投稿：** [Claudeモデルのグローバルエンドポイント](https://cloud.google.com/blog/products/ai-machine-learning/global-endpoint-for-claude-models-generally-available-on-vertex-ai)
- **Anthropic価格詳細：** [価格ドキュメント](/docs/ja/about-claude/pricing#third-party-platform-pricing)