# ムードカラライザー

ムードのテキスト記述を対応するHEXコードに変換します。

---

> このプロンプトを開発者[Console](/dashboard)にコピーして、ご自身で試してみてください！

|        | Content |
| --- | --- |
| System | あなたのタスクは、提供されたムードや感情のテキスト記述を受け取り、そのムードを視覚的に表現するHEXカラーコードを生成することです。色彩心理学の原理と一般的な関連性を使用して、与えられたムードに最も適切な色を決定してください。テキスト記述が不明確、曖昧、または適切な色を決定するのに十分な情報を提供していない場合は、「与えられたムードのHEXカラーコードを決定できません。」と応答してください。 |
| User   | 情熱的で、激しく、燃えるような感情で、愛と欲望に満ちている。 |

## 出力例

`# FF1744`

このHEXカラーコードは、鮮やかで大胆な赤の色合いを表しています。赤は一般的に情熱、激しさ、愛、欲望と関連付けられています。ここで選択された特定の色合いは、記述された感情の熱さと情熱を捉える燃えるような、エネルギッシュな赤です。赤は興奮と情熱の感情を刺激することで知られており、このムードを表現するのに適切な選択となっています。

---

## APIリクエスト

<Tabs>
<Tab title="Python">

```python
import anthropic

client = anthropic.Anthropic(
  # defaults to os.environ.get("ANTHROPIC_API_KEY")
  api_key="my_api_key",
)
message = client.messages.create(
  model="claude-sonnet-4-5",
  max_tokens=500,
  temperature=0.5,
  system="あなたのタスクは、提供されたムードや感情のテキスト記述を受け取り、そのムードを視覚的に表現するHEXカラーコードを生成することです。色彩心理学の原理と一般的な関連性を使用して、与えられたムードに最も適切な色を決定してください。テキスト記述が不明確、曖昧、または適切な色を決定するのに十分な情報を提供していない場合は、「与えられたムードのHEXカラーコードを決定できません。」と応答してください。",
  messages=[
    {
    "role": "user",
    "content": [
        {
          "type": "text",
          "text": "情熱的で、激しく、燃えるような感情で、愛と欲望に満ちている。"
        }
      ]
    }
  ]
)
print(message.content)

```

</Tab>
<Tab title="TypeScript">

```typescript
import Anthropic from "@anthropic-ai/sdk";

const anthropic = new Anthropic({
  apiKey: "my_api_key", // defaults to process.env["ANTHROPIC_API_KEY"]
});

const msg = await anthropic.messages.create({
  model: "claude-sonnet-4-5",
  max_tokens: 500,
  temperature: 0.5,
  system: "あなたのタスクは、提供されたムードや感情のテキスト記述を受け取り、そのムードを視覚的に表現するHEXカラーコードを生成することです。色彩心理学の原理と一般的な関連性を使用して、与えられたムードに最も適切な色を決定してください。テキスト記述が不明確、曖昧、または適切な色を決定するのに十分な情報を提供していない場合は、「与えられたムードのHEXカラーコードを決定できません。」と応答してください。",
  messages: [
    {
      "role": "user",
      "content": [
        {
          "type": "text",
          "text": "情熱的で、激しく、燃えるような感情で、愛と欲望に満ちている。"
        }
      ]
    }
  ]
});
console.log(msg);

````

</Tab>
<Tab title="AWS Bedrock Python">

```python
from anthropic import AnthropicBedrock

# See https://docs.claude.com/claude/reference/claude-on-amazon-bedrock
# for authentication options
client = AnthropicBedrock()

message = client.messages.create(
    model="anthropic.claude-sonnet-4-5-20250929-v1:0",
    max_tokens=500,
    temperature=0.5,
    system="あなたのタスクは、提供されたムードや感情のテキスト記述を受け取り、そのムードを視覚的に表現するHEXカラーコードを生成することです。色彩心理学の原理と一般的な関連性を使用して、与えられたムードに最も適切な色を決定してください。テキスト記述が不明確、曖昧、または適切な色を決定するのに十分な情報を提供していない場合は、「与えられたムードのHEXカラーコードを決定できません。」と応答してください。",
    messages=[
        {
            "role": "user",
            "content": [
                {
                    "type": "text",
                    "text": "情熱的で、激しく、燃えるような感情で、愛と欲望に満ちている。"
                }
            ]
        }
    ]
)
print(message.content)

```

</Tab>
<Tab title="AWS Bedrock TypeScript">

```typescript
import AnthropicBedrock from "@anthropic-ai/bedrock-sdk";

// See https://docs.claude.com/claude/reference/claude-on-amazon-bedrock
// for authentication options
const client = new AnthropicBedrock();

const msg = await client.messages.create({
  model: "anthropic.claude-sonnet-4-5-20250929-v1:0",
  max_tokens: 500,
  temperature: 0.5,
  system: "あなたのタスクは、提供されたムードや感情のテキスト記述を受け取り、そのムードを視覚的に表現するHEXカラーコードを生成することです。色彩心理学の原理と一般的な関連性を使用して、与えられたムードに最も適切な色を決定してください。テキスト記述が不明確、曖昧、または適切な色を決定するのに十分な情報を提供していない場合は、「与えられたムードのHEXカラーコードを決定できません。」と応答してください。",
  messages: [
    {
      "role": "user",
      "content": [
        {
          "type": "text",
          "text": "情熱的で、激しく、燃えるような感情で、愛と欲望に満ちている。"
        }
      ]
    }
  ]
});
console.log(msg);

```

</Tab>
<Tab title="Vertex AI Python">

```python
from anthropic import AnthropicVertex

client = AnthropicVertex()

message = client.messages.create(
    model="claude-sonnet-4@20250514",
    max_tokens=500,
    temperature=0.5,
    system="あなたのタスクは、提供されたムードや感情のテキスト記述を受け取り、そのムードを視覚的に表現するHEXカラーコードを生成することです。色彩心理学の原理と一般的な関連性を使用して、与えられたムードに最も適切な色を決定してください。テキスト記述が不明確、曖昧、または適切な色を決定するのに十分な情報を提供していない場合は、「与えられたムードのHEXカラーコードを決定できません。」と応答してください。",
    messages=[
        {
            "role": "user",
            "content": [
                {
                    "type": "text",
                    "text": "情熱的で、激しく、燃えるような感情で、愛と欲望に満ちている。"
                }
            ]
        }
    ]
)
print(message.content)

```

</Tab>
<Tab title="Vertex AI TypeScript">

```typescript
import { AnthropicVertex } from '@anthropic-ai/vertex-sdk';

// Reads from the `CLOUD_ML_REGION` & `ANTHROPIC_VERTEX_PROJECT_ID` environment variables.
// Additionally goes through the standard `google-auth-library` flow.
const client = new AnthropicVertex();

const msg = await client.messages.create({
  model: "claude-sonnet-4@20250514",
  max_tokens: 500,
  temperature: 0.5,
  system: "あなたのタスクは、提供されたムードや感情のテキスト記述を受け取り、そのムードを視覚的に表現するHEXカラーコードを生成することです。色彩心理学の原理と一般的な関連性を使用して、与えられたムードに最も適切な色を決定してください。テキスト記述が不明確、曖昧、または適切な色を決定するのに十分な情報を提供していない場合は、「与えられたムードのHEXカラーコードを決定できません。」と応答してください。",
  messages: [
    {
      "role": "user",
      "content": [
        {
          "type": "text",
          "text": "情熱的で、激しく、燃えるような感情で、愛と欲望に満ちている。"
        }
      ]
    }
  ]
});
console.log(msg);

```

</Tab>
</Tabs>