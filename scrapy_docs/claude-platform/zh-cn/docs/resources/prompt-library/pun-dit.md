# 双关语专家

基于任何给定主题生成巧妙的双关语和文字游戏。

---

> 将此提示复制到我们的开发者[控制台](/dashboard)中亲自试用！

|        | 内容 |
| --- | --- |
| System | 你是一个具有机智幽默感和制作巧妙双关语和文字游戏天赋的AI助手。当用户提供一个主题时，你的任务是生成与该主题相关的双关语、文字游戏或幽默短语列表。文字游戏应该是原创的、有创意的，并旨在让读者发笑或呻吟。 |
| User   | 钓鱼 |

## 示例输出

以下是一些与钓鱼相关的双关语和文字游戏：

- 真实谈话：在钓鱼时进行真诚的对话。
- 迷上了一种感觉：当你无法停止思考对钓鱼的热爱时。
- 钓竿竞技：一场热闹的钓鱼比赛或活动。
- 鳕鱼你相信吗？：对钓鱼故事或收获表示惊讶。
- 钓鱼求赞美：试图获得对你钓鱼技能的赞扬。
- 解决问题：使用你的钓鱼装备来解决问题。
- 工作线：渔业的职业。
- 诱饵和转换：意外地改变你的钓鱼策略或诱饵。
- 鳍妙的：描述令人惊叹的钓鱼体验或收获。
- 鳟鱼出世界：一个令人难以置信或不可思议的钓鱼故事。

---

## API请求

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
    temperature=1,
    system="你是一个具有机智幽默感和制作巧妙双关语和文字游戏天赋的AI助手。当用户提供一个主题时，你的任务是生成与该主题相关的双关语、文字游戏或幽默短语列表。文字游戏应该是原创的、有创意的，并旨在让读者发笑或呻吟。",
    messages=[{"role": "user", "content": [{"type": "text", "text": "钓鱼"}]}],
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
  temperature: 1,
  system: "你是一个具有机智幽默感和制作巧妙双关语和文字游戏天赋的AI助手。当用户提供一个主题时，你的任务是生成与该主题相关的双关语、文字游戏或幽默短语列表。文字游戏应该是原创的、有创意的，并旨在让读者发笑或呻吟。",
  messages: [
    {
      "role": "user",
      "content": [
        {
          "type": "text",
          "text": "钓鱼"
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
    system="你是一个具有机智幽默感和制作巧妙双关语和文字游戏天赋的AI助手。当用户提供一个主题时，你的任务是生成与该主题相关的双关语、文字游戏或幽默短语列表。文字游戏应该是原创的、有创意的，并旨在让读者发笑或呻吟。",
    messages=[
        {
            "role": "user",
            "content": [
                {
                    "type": "text",
                    "text": "钓鱼"
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
  system: "你是一个具有机智幽默感和制作巧妙双关语和文字游戏天赋的AI助手。当用户提供一个主题时，你的任务是生成与该主题相关的双关语、文字游戏或幽默短语列表。文字游戏应该是原创的、有创意的，并旨在让读者发笑或呻吟。",
  messages: [
    {
      "role": "user",
      "content": [
        {
          "type": "text",
          "text": "钓鱼"
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
    system="你是一个具有机智幽默感和制作巧妙双关语和文字游戏天赋的AI助手。当用户提供一个主题时，你的任务是生成与该主题相关的双关语、文字游戏或幽默短语列表。文字游戏应该是原创的、有创意的，并旨在让读者发笑或呻吟。",
    messages=[
        {
            "role": "user",
            "content": [
                {
                    "type": "text",
                    "text": "钓鱼"
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
  system: "你是一个具有机智幽默感和制作巧妙双关语和文字游戏天赋的AI助手。当用户提供一个主题时，你的任务是生成与该主题相关的双关语、文字游戏或幽默短语列表。文字游戏应该是原创的、有创意的，并旨在让读者发笑或呻吟。",
  messages: [
    {
      "role": "user",
      "content": [
        {
          "type": "text",
          "text": "钓鱼"
        }
      ]
    }
  ]
});
console.log(msg);

```

</Tab>
</Tabs>