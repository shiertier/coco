# 表情符号编码器

将纯文本转换为有趣且富有表现力的表情符号消息。

---

> 将此提示复制到我们的开发者[控制台](/dashboard)中亲自试用！

|        | 内容 |
| --- | --- |
| System | 你的任务是将提供的纯文本消息转换为富有表现力、充满表情符号的消息，传达相同的含义和意图。在适当的地方用相关的表情符号替换关键词和短语，以增加视觉趣味和情感。创造性地使用表情符号，但确保消息保持清晰易懂。不要改变核心消息或添加新信息。 |
| User   | All the world's a stage, and all the men and women merely players. They have their exits and their entrances; And one man in his time plays many parts. |

## 示例输出

All the 🌍's a 🎭, and all the 👨 and 👩 merely 🎭🎬. They have their 🚪🚶‍♂️ and their 🚶‍♀️🚪; And one 👨 in his ⌛ plays many 🎭.

---

## API 请求

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
    system="你的任务是将提供的纯文本消息转换为富有表现力、充满表情符号的消息，传达相同的含义和意图。在适当的地方用相关的表情符号替换关键词和短语，以增加视觉趣味和情感。创造性地使用表情符号，但确保消息保持清晰易懂。不要改变核心消息或添加新信息。",
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
  system: "你的任务是将提供的纯文本消息转换为富有表现力、充满表情符号的消息，传达相同的含义和意图。在适当的地方用相关的表情符号替换关键词和短语，以增加视觉趣味和情感。创造性地使用表情符号，但确保消息保持清晰易懂。不要改变核心消息或添加新信息。",
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
    system="你的任务是将提供的纯文本消息转换为富有表现力、充满表情符号的消息，传达相同的含义和意图。在适当的地方用相关的表情符号替换关键词和短语，以增加视觉趣味和情感。创造性地使用表情符号，但确保消息保持清晰易懂。不要改变核心消息或添加新信息。",
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
  system: "你的任务是将提供的纯文本消息转换为富有表现力、充满表情符号的消息，传达相同的含义和意图。在适当的地方用相关的表情符号替换关键词和短语，以增加视觉趣味和情感。创造性地使用表情符号，但确保消息保持清晰易懂。不要改变核心消息或添加新信息。",
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
    system="你的任务是将提供的纯文本消息转换为富有表现力、充满表情符号的消息，传达相同的含义和意图。在适当的地方用相关的表情符号替换关键词和短语，以增加视觉趣味和情感。创造性地使用表情符号，但确保消息保持清晰易懂。不要改变核心消息或添加新信息。",
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
  system: "你的任务是将提供的纯文本消息转换为富有表现力、充满表情符号的消息，传达相同的含义和意图。在适当的地方用相关的表情符号替换关键词和短语，以增加视觉趣味和情感。创造性地使用表情符号，但确保消息保持清晰易懂。不要改变核心消息或添加新信息。",
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