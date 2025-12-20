# 絵文字エンコーダー

プレーンテキストを楽しく表現豊かな絵文字メッセージに変換します。

---

> このプロンプトを私たちの開発者[Console](/dashboard)にコピーして、ご自身で試してみてください！

|        | Content |
| --- | --- |
| System | あなたのタスクは、提供されたプレーンテキストメッセージを受け取り、同じ意味と意図を伝える表現豊かで絵文字が豊富なメッセージに変換することです。適切な場所で重要な単語やフレーズを関連する絵文字に置き換えて、視覚的な興味と感情を追加してください。絵文字を創造的に使用しますが、メッセージが明確で理解しやすいままであることを確認してください。核となるメッセージを変更したり、新しい情報を追加したりしないでください。 |
| User   | All the world's a stage, and all the men and women merely players. They have their exits and their entrances; And one man in his time plays many parts. |

## 出力例

All the 🌍's a 🎭, and all the 👨 and 👩 merely 🎭🎬. They have their 🚪🚶‍♂️ and their 🚶‍♀️🚪; And one 👨 in his ⌛ plays many 🎭.

---

## APIリクエスト

<Tabs>
<Tab title="Python">
```python
import anthropic

client = anthropic.Anthropic(  # defaults to os.environ.get("ANTHROPIC_API_KEY")
    api_key="my_api_key",
)
message = client.messages.create(
    model="claude-sonnet-4-5",
    max_tokens=1000,
    temperature=0,
    system="あなたのタスクは、提供されたプレーンテキストメッセージを受け取り、同じ意味と意図を伝える表現豊かで絵文字が豊富なメッセージに変換することです。適切な場所で重要な単語やフレーズを関連する絵文字に置き換えて、視覚的な興味と感情を追加してください。絵文字を創造的に使用しますが、メッセージが明確で理解しやすいままであることを確認してください。核となるメッセージを変更したり、新しい情報を追加したりしないでください。",
    messages=[
        {
            "role": "user",
            "content": [
                {
                    "type": "text",
                    "text": "All the world's a stage, and all the men and women merely players. They have their exits and their entrances; And one man in his time plays many parts.",
                }
            ],
        }
    ],
)
print(message.content)


````
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
  system: "あなたのタスクは、提供されたプレーンテキストメッセージを受け取り、同じ意味と意図を伝える表現豊かで絵文字が豊富なメッセージに変換することです。適切な場所で重要な単語やフレーズを関連する絵文字に置き換えて、視覚的な興味と感情を追加してください。絵文字を創造的に使用しますが、メッセージが明確で理解しやすいままであることを確認してください。核となるメッセージを変更したり、新しい情報を追加したりしないでください。",
  messages: [
    {
      "role": "user",
      "content": [
        {
          "type": "text",
          "text": "All the world's a stage, and all the men and women merely players. They have their exits and their entrances; And one man in his time plays many parts."
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
    system="あなたのタスクは、提供されたプレーンテキストメッセージを受け取り、同じ意味と意図を伝える表現豊かで絵文字が豊富なメッセージに変換することです。適切な場所で重要な単語やフレーズを関連する絵文字に置き換えて、視覚的な興味と感情を追加してください。絵文字を創造的に使用しますが、メッセージが明確で理解しやすいままであることを確認してください。核となるメッセージを変更したり、新しい情報を追加したりしないでください。",
    messages=[
        {
            "role": "user",
            "content": [
                {
                    "type": "text",
                    "text": "All the world's a stage, and all the men and women merely players. They have their exits and their entrances; And one man in his time plays many parts."
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
  system: "あなたのタスクは、提供されたプレーンテキストメッセージを受け取り、同じ意味と意図を伝える表現豊かで絵文字が豊富なメッセージに変換することです。適切な場所で重要な単語やフレーズを関連する絵文字に置き換えて、視覚的な興味と感情を追加してください。絵文字を創造的に使用しますが、メッセージが明確で理解しやすいままであることを確認してください。核となるメッセージを変更したり、新しい情報を追加したりしないでください。",
  messages: [
    {
      "role": "user",
      "content": [
        {
          "type": "text",
          "text": "All the world's a stage, and all the men and women merely players. They have their exits and their entrances; And one man in his time plays many parts."
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
    system="あなたのタスクは、提供されたプレーンテキストメッセージを受け取り、同じ意味と意図を伝える表現豊かで絵文字が豊富なメッセージに変換することです。適切な場所で重要な単語やフレーズを関連する絵文字に置き換えて、視覚的な興味と感情を追加してください。絵文字を創造的に使用しますが、メッセージが明確で理解しやすいままであることを確認してください。核となるメッセージを変更したり、新しい情報を追加したりしないでください。",
    messages=[
        {
            "role": "user",
            "content": [
                {
                    "type": "text",
                    "text": "All the world's a stage, and all the men and women merely players. They have their exits and their entrances; And one man in his time plays many parts."
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
  system: "あなたのタスクは、提供されたプレーンテキストメッセージを受け取り、同じ意味と意図を伝える表現豊かで絵文字が豊富なメッセージに変換することです。適切な場所で重要な単語やフレーズを関連する絵文字に置き換えて、視覚的な興味と感情を追加してください。絵文字を創造的に使用しますが、メッセージが明確で理解しやすいままであることを確認してください。核となるメッセージを変更したり、新しい情報を追加したりしないでください。",
  messages: [
    {
      "role": "user",
      "content": [
        {
          "type": "text",
          "text": "All the world's a stage, and all the men and women merely players. They have their exits and their entrances; And one man in his time plays many parts."
        }
      ]
    }
  ]
});
console.log(msg);

```

</Tab>
</Tabs>