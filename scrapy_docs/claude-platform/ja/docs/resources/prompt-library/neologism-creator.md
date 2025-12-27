# 新語創造者

ユーザーが提供する概念やアイデアに基づいて新しい単語を発明し、その定義を提供します。

---

> このプロンプトを私たちの開発者[Console](/dashboard)にコピーして、ご自身で試してみてください！

|      | Content |
| ---- | --- |
| User | 無知や情報不足に見えることを避けるために、何かを理解しているふりをする行為を表す新しい単語を作ってもらえますか？ |

## 出力例

この概念を表現するために「fauxprehension（フォープリヘンション）」という新語はいかがでしょうか？

Fauxprehension (fəʊprɪˈhɛnʃ(ə)n) - 名詞  
 定義：無知、情報不足、または話についていけないように見えることを避けるために、トピック、概念、または状況の理解や把握を装う行為。知識や主題の把握を偽装することで、しばしば社会的または職業的な場面で自分のイメージや地位を維持するために用いられる。

語源：「fauxprehension」という単語は、偽物や偽りを意味するフランス語由来の「faux」と、何かを理解する行為や能力を意味する「comprehension」を組み合わせた造語です。

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
  max_tokens=1000,
  temperature=1,
  messages=[
    {
      "role": "user",
      "content": [
        {
          "type": "text",
          "text": "Can you help me create a new word for the act of pretending to understand something in order to avoid looking ignorant or uninformed?"
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
  max_tokens: 1000,
  temperature: 1,
  messages: [
    {
      "role": "user",
      "content": [
        {
          "type": "text",
          "text": "Can you help me create a new word for the act of pretending to understand something in order to avoid looking ignorant or uninformed?"
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
    max_tokens=1000,
    temperature=1,
    messages=[
        {
            "role": "user",
            "content": [
                {
                    "type": "text",
                    "text": "Can you help me create a new word for the act of pretending to understand something in order to avoid looking ignorant or uninformed?"
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
  max_tokens: 1000,
  temperature: 1,
  messages: [
    {
      "role": "user",
      "content": [
        {
          "type": "text",
          "text": "Can you help me create a new word for the act of pretending to understand something in order to avoid looking ignorant or uninformed?"
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
    max_tokens=1000,
    temperature=1,
    messages=[
        {
            "role": "user",
            "content": [
                {
                    "type": "text",
                    "text": "Can you help me create a new word for the act of pretending to understand something in order to avoid looking ignorant or uninformed?"
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
  max_tokens: 1000,
  temperature: 1,
  messages: [
    {
      "role": "user",
      "content": [
        {
          "type": "text",
          "text": "Can you help me create a new word for the act of pretending to understand something in order to avoid looking ignorant or uninformed?"
        }
      ]
    }
  ]
});
console.log(msg);

```

</Tab>
</Tabs>