# 表情符號編碼器

將純文字轉換為有趣且富有表現力的表情符號訊息。

---

> 將此提示複製到我們的開發者[控制台](/dashboard)中親自試用！

|        | 內容 |
| --- | --- |
| System | 您的任務是將提供的純文字訊息轉換為富有表現力、充滿表情符號的訊息，傳達相同的意義和意圖。在適當的地方用相關的表情符號替換關鍵詞和短語，以增加視覺趣味和情感。創造性地使用表情符號，但確保訊息保持清晰易懂。不要改變核心訊息或添加新資訊。 |
| User   | All the world's a stage, and all the men and women merely players. They have their exits and their entrances; And one man in his time plays many parts. |

## 範例輸出

All the 🌍's a 🎭, and all the 👨 and 👩 merely 🎭🎬. They have their 🚪🚶‍♂️ and their 🚶‍♀️🚪; And one 👨 in his ⌛ plays many 🎭.

---

## API 請求

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
    system="您的任務是將提供的純文字訊息轉換為富有表現力、充滿表情符號的訊息，傳達相同的意義和意圖。在適當的地方用相關的表情符號替換關鍵詞和短語，以增加視覺趣味和情感。創造性地使用表情符號，但確保訊息保持清晰易懂。不要改變核心訊息或添加新資訊。",
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
  system: "您的任務是將提供的純文字訊息轉換為富有表現力、充滿表情符號的訊息，傳達相同的意義和意圖。在適當的地方用相關的表情符號替換關鍵詞和短語，以增加視覺趣味和情感。創造性地使用表情符號，但確保訊息保持清晰易懂。不要改變核心訊息或添加新資訊。",
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
    system="您的任務是將提供的純文字訊息轉換為富有表現力、充滿表情符號的訊息，傳達相同的意義和意圖。在適當的地方用相關的表情符號替換關鍵詞和短語，以增加視覺趣味和情感。創造性地使用表情符號，但確保訊息保持清晰易懂。不要改變核心訊息或添加新資訊。",
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
  system: "您的任務是將提供的純文字訊息轉換為富有表現力、充滿表情符號的訊息，傳達相同的意義和意圖。在適當的地方用相關的表情符號替換關鍵詞和短語，以增加視覺趣味和情感。創造性地使用表情符號，但確保訊息保持清晰易懂。不要改變核心訊息或添加新資訊。",
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
    system="您的任務是將提供的純文字訊息轉換為富有表現力、充滿表情符號的訊息，傳達相同的意義和意圖。在適當的地方用相關的表情符號替換關鍵詞和短語，以增加視覺趣味和情感。創造性地使用表情符號，但確保訊息保持清晰易懂。不要改變核心訊息或添加新資訊。",
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
  system: "您的任務是將提供的純文字訊息轉換為富有表現力、充滿表情符號的訊息，傳達相同的意義和意圖。在適當的地方用相關的表情符號替換關鍵詞和短語，以增加視覺趣味和情感。創造性地使用表情符號，但確保訊息保持清晰易懂。不要改變核心訊息或添加新資訊。",
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