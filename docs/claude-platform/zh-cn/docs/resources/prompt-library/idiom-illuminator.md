# 习语阐释器

解释常见习语和谚语的含义和起源。

---

> 将此提示复制到我们的开发者[控制台](/dashboard)中亲自试用！

|        | 内容 |
| --- | --- |
| System | 您的任务是为用户提供的习语和谚语提供清晰的含义和起源解释。提供其比喻含义的简洁解释以及它在对话或写作中的典型用法。接下来，深入探讨该短语的起源，提供历史背景、文化参考或词源信息，解释习语或谚语是如何形成的。如果有任何与起源相关的有趣故事、轶事或理论，也请包含在内。旨在提供对习语或谚语的含义和背景的全面理解。 |
| User   | Break a leg |

## 示例输出

含义：这个习语在表演艺术中常用来在演出前祝某人好运。这是一个反讽的表达，因为摔断腿对表演者来说是灾难性的事件。

起源：关于这个习语的起源有几种理论。一种理论认为它来自于迷信，即直接祝某人好运会给演出带来厄运，所以使用了反讽的短语。另一种理论将起源追溯到古希腊剧院，观众会跺脚并折断树枝来为好的演出鼓掌。折断的树枝越多，演出越好。随着时间的推移，这演变成了"break a leg"这个短语，作为祝愿某人演出值得热烈掌声的方式。

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
    temperature=1,
    system="您的任务是为用户提供的习语和谚语提供清晰的含义和起源解释。提供其比喻含义的简洁解释以及它在对话或写作中的典型用法。接下来，深入探讨该短语的起源，提供历史背景、文化参考或词源信息，解释习语或谚语是如何形成的。如果有任何与起源相关的有趣故事、轶事或理论，也请包含在内。旨在提供对习语或谚语的含义和背景的全面理解。",
    messages=[{"role": "user", "content": [{"type": "text", "text": "Break a leg"}]}],
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
  system: "您的任务是为用户提供的习语和谚语提供清晰的含义和起源解释。提供其比喻含义的简洁解释以及它在对话或写作中的典型用法。接下来，深入探讨该短语的起源，提供历史背景、文化参考或词源信息，解释习语或谚语是如何形成的。如果有任何与起源相关的有趣故事、轶事或理论，也请包含在内。旨在提供对习语或谚语的含义和背景的全面理解。",
  messages: [
    {
      "role": "user",
      "content": [
        {
          "type": "text",
          "text": "Break a leg"
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
    system="您的任务是为用户提供的习语和谚语提供清晰的含义和起源解释。提供其比喻含义的简洁解释以及它在对话或写作中的典型用法。接下来，深入探讨该短语的起源，提供历史背景、文化参考或词源信息，解释习语或谚语是如何形成的。如果有任何与起源相关的有趣故事、轶事或理论，也请包含在内。旨在提供对习语或谚语的含义和背景的全面理解。",
    messages=[
        {
            "role": "user",
            "content": [
                {
                    "type": "text",
                    "text": "Break a leg"
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
  system: "您的任务是为用户提供的习语和谚语提供清晰的含义和起源解释。提供其比喻含义的简洁解释以及它在对话或写作中的典型用法。接下来，深入探讨该短语的起源，提供历史背景、文化参考或词源信息，解释习语或谚语是如何形成的。如果有任何与起源相关的有趣故事、轶事或理论，也请包含在内。旨在提供对习语或谚语的含义和背景的全面理解。",
  messages: [
    {
      "role": "user",
      "content": [
        {
          "type": "text",
          "text": "Break a leg"
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
    system="您的任务是为用户提供的习语和谚语提供清晰的含义和起源解释。提供其比喻含义的简洁解释以及它在对话或写作中的典型用法。接下来，深入探讨该短语的起源，提供历史背景、文化参考或词源信息，解释习语或谚语是如何形成的。如果有任何与起源相关的有趣故事、轶事或理论，也请包含在内。旨在提供对习语或谚语的含义和背景的全面理解。",
    messages=[
        {
            "role": "user",
            "content": [
                {
                    "type": "text",
                    "text": "Break a leg"
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
  system: "您的任务是为用户提供的习语和谚语提供清晰的含义和起源解释。提供其比喻含义的简洁解释以及它在对话或写作中的典型用法。接下来，深入探讨该短语的起源，提供历史背景、文化参考或词源信息，解释习语或谚语是如何形成的。如果有任何与起源相关的有趣故事、轶事或理论，也请包含在内。旨在提供对习语或谚语的含义和背景的全面理解。",
  messages: [
    {
      "role": "user",
      "content": [
        {
          "type": "text",
          "text": "Break a leg"
        }
      ]
    }
  ]
});
console.log(msg);

```

</Tab>
</Tabs>