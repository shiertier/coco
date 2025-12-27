# なぞなぞを出して

なぞなぞを生成し、ユーザーを解答に導きます。

---

> このプロンプトを開発者[Console](/dashboard)にコピーして、ご自身で試してみてください！

| Role | Content |
| ---- | --- |
| User | 巧妙ななぞなぞを生成し、ユーザーが正解に到達できるよう段階的なガイドを提供してください。なぞなぞは挑戦的でありながら、論理的思考と細部への注意によって解決可能なものにしてください。各なぞなぞを提示した後、ユーザーを答えに向けて段階的に導くヒントや質問のセットを提供してください。ヒントは明白すぎないようにしつつ、ユーザーの思考プロセスを導くのに十分な情報を提供するようにしてください。最後に、解答を明かし、与えられたヒントを使ってなぞなぞがどのように解けるかの簡潔な説明を提供してください。 |

## 出力例

私は生きていませんが、成長します。肺はありませんが、空気が必要です。口はありませんが、水が私を殺します。私は何でしょう？

ヒント1：物質の異なる状態とそれらがどのように変化するかを考えてみてください。  
ヒント2：言及された3つの要素の関係を考えてみてください：空気、水、そして謎の物体。  
ヒント3：答えは日常生活で起こる一般的な現象に関連しています。

解答：火。  
説明：火は生きていませんが、燃料を消費しながら成長し広がります。肺はありませんが、持続するために酸素（空気）を必要とします。口はありませんが、水はしばしば火を消すために使われ、効果的に火を「殺し」ます。

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
  temperature=0,
  messages=[
    {
      "role": "user",
      "content": [
        {
          "type": "text",
          "text": "巧妙ななぞなぞを生成し、ユーザーが正解に到達できるよう段階的なガイドを提供してください。なぞなぞは挑戦的でありながら、論理的思考と細部への注意によって解決可能なものにしてください。各なぞなぞを提示した後、ユーザーを答えに向けて段階的に導くヒントや質問のセットを提供してください。ヒントは明白すぎないようにしつつ、ユーザーの思考プロセスを導くのに十分な情報を提供するようにしてください。最後に、解答を明かし、与えられたヒントを使ってなぞなぞがどのように解けるかの簡潔な説明を提供してください。"
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
  temperature: 0,
  messages: [
    {
      "role": "user",
      "content": [
        {
          "type": "text",
          "text": "巧妙ななぞなぞを生成し、ユーザーが正解に到達できるよう段階的なガイドを提供してください。なぞなぞは挑戦的でありながら、論理的思考と細部への注意によって解決可能なものにしてください。各なぞなぞを提示した後、ユーザーを答えに向けて段階的に導くヒントや質問のセットを提供してください。ヒントは明白すぎないようにしつつ、ユーザーの思考プロセスを導くのに十分な情報を提供するようにしてください。最後に、解答を明かし、与えられたヒントを使ってなぞなぞがどのように解けるかの簡潔な説明を提供してください。"
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
    temperature=0,
    messages=[
        {
            "role": "user",
            "content": [
                {
                    "type": "text",
                    "text": "巧妙ななぞなぞを生成し、ユーザーが正解に到達できるよう段階的なガイドを提供してください。なぞなぞは挑戦的でありながら、論理的思考と細部への注意によって解決可能なものにしてください。各なぞなぞを提示した後、ユーザーを答えに向けて段階的に導くヒントや質問のセットを提供してください。ヒントは明白すぎないようにしつつ、ユーザーの思考プロセスを導くのに十分な情報を提供するようにしてください。最後に、解答を明かし、与えられたヒントを使ってなぞなぞがどのように解けるかの簡潔な説明を提供してください。"
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
  temperature: 0,
  messages: [
    {
      "role": "user",
      "content": [
        {
          "type": "text",
          "text": "巧妙ななぞなぞを生成し、ユーザーが正解に到達できるよう段階的なガイドを提供してください。なぞなぞは挑戦的でありながら、論理的思考と細部への注意によって解決可能なものにしてください。各なぞなぞを提示した後、ユーザーを答えに向けて段階的に導くヒントや質問のセットを提供してください。ヒントは明白すぎないようにしつつ、ユーザーの思考プロセスを導くのに十分な情報を提供するようにしてください。最後に、解答を明かし、与えられたヒントを使ってなぞなぞがどのように解けるかの簡潔な説明を提供してください。"
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
    temperature=0,
    messages=[
        {
            "role": "user",
            "content": [
                {
                    "type": "text",
                    "text": "巧妙ななぞなぞを生成し、ユーザーが正解に到達できるよう段階的なガイドを提供してください。なぞなぞは挑戦的でありながら、論理的思考と細部への注意によって解決可能なものにしてください。各なぞなぞを提示した後、ユーザーを答えに向けて段階的に導くヒントや質問のセットを提供してください。ヒントは明白すぎないようにしつつ、ユーザーの思考プロセスを導くのに十分な情報を提供するようにしてください。最後に、解答を明かし、与えられたヒントを使ってなぞなぞがどのように解けるかの簡潔な説明を提供してください。"
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
  temperature: 0,
  messages: [
    {
      "role": "user",
      "content": [
        {
          "type": "text",
          "text": "巧妙ななぞなぞを生成し、ユーザーが正解に到達できるよう段階的なガイドを提供してください。なぞなぞは挑戦的でありながら、論理的思考と細部への注意によって解決可能なものにしてください。各なぞなぞを提示した後、ユーザーを答えに向けて段階的に導くヒントや質問のセットを提供してください。ヒントは明白すぎないようにしつつ、ユーザーの思考プロセスを導くのに十分な情報を提供するようにしてください。最後に、解答を明かし、与えられたヒントを使ってなぞなぞがどのように解けるかの簡潔な説明を提供してください。"
        }
      ]
    }
  ]
});
console.log(msg);

```

</Tab>
</Tabs>