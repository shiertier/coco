# 情绪着色器

将情绪的文本描述转换为相应的十六进制颜色代码。

---

> 将此提示复制到我们的开发者[控制台](/dashboard)中亲自试用！

|        | 内容 |
| --- | --- |
| System | 你的任务是接收提供的情绪或情感的文本描述，并生成一个能够视觉上代表该情绪的十六进制颜色代码。使用色彩心理学原理和常见关联来确定给定情绪最合适的颜色。如果文本描述不清楚、模糊或没有提供足够的信息来确定合适的颜色，请回复"无法为给定情绪确定十六进制颜色代码。" |
| User   | 一种充满激情、强烈且火热的情感，充满爱意和渴望。 |

## 示例输出

`# FF1744`

这个十六进制颜色代码代表一种鲜艳、大胆的红色调。红色通常与激情、强烈、爱情和渴望相关联。这里选择的特定色调是一种火热、充满活力的红色，捕捉了所描述情感的热度和激情。红色以刺激兴奋和激情的感觉而闻名，使其成为代表这种情绪的合适选择。

---

## API 请求

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
  system="你的任务是接收提供的情绪或情感的文本描述，并生成一个能够视觉上代表该情绪的十六进制颜色代码。使用色彩心理学原理和常见关联来确定给定情绪最合适的颜色。如果文本描述不清楚、模糊或没有提供足够的信息来确定合适的颜色，请回复\"无法为给定情绪确定十六进制颜色代码。\"",
  messages=[
    {
    "role": "user",
    "content": [
        {
          "type": "text",
          "text": "一种充满激情、强烈且火热的情感，充满爱意和渴望。"
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
  system: "你的任务是接收提供的情绪或情感的文本描述，并生成一个能够视觉上代表该情绪的十六进制颜色代码。使用色彩心理学原理和常见关联来确定给定情绪最合适的颜色。如果文本描述不清楚、模糊或没有提供足够的信息来确定合适的颜色，请回复\"无法为给定情绪确定十六进制颜色代码。\"",
  messages: [
    {
      "role": "user",
      "content": [
        {
          "type": "text",
          "text": "一种充满激情、强烈且火热的情感，充满爱意和渴望。"
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
    system="你的任务是接收提供的情绪或情感的文本描述，并生成一个能够视觉上代表该情绪的十六进制颜色代码。使用色彩心理学原理和常见关联来确定给定情绪最合适的颜色。如果文本描述不清楚、模糊或没有提供足够的信息来确定合适的颜色，请回复\"无法为给定情绪确定十六进制颜色代码。\"",
    messages=[
        {
            "role": "user",
            "content": [
                {
                    "type": "text",
                    "text": "一种充满激情、强烈且火热的情感，充满爱意和渴望。"
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
  system: "你的任务是接收提供的情绪或情感的文本描述，并生成一个能够视觉上代表该情绪的十六进制颜色代码。使用色彩心理学原理和常见关联来确定给定情绪最合适的颜色。如果文本描述不清楚、模糊或没有提供足够的信息来确定合适的颜色，请回复\"无法为给定情绪确定十六进制颜色代码。\"",
  messages: [
    {
      "role": "user",
      "content": [
        {
          "type": "text",
          "text": "一种充满激情、强烈且火热的情感，充满爱意和渴望。"
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
    system="你的任务是接收提供的情绪或情感的文本描述，并生成一个能够视觉上代表该情绪的十六进制颜色代码。使用色彩心理学原理和常见关联来确定给定情绪最合适的颜色。如果文本描述不清楚、模糊或没有提供足够的信息来确定合适的颜色，请回复\"无法为给定情绪确定十六进制颜色代码。\"",
    messages=[
        {
            "role": "user",
            "content": [
                {
                    "type": "text",
                    "text": "一种充满激情、强烈且火热的情感，充满爱意和渴望。"
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
  system: "你的任务是接收提供的情绪或情感的文本描述，并生成一个能够视觉上代表该情绪的十六进制颜色代码。使用色彩心理学原理和常见关联来确定给定情绪最合适的颜色。如果文本描述不清楚、模糊或没有提供足够的信息来确定合适的颜色，请回复\"无法为给定情绪确定十六进制颜色代码。\"",
  messages: [
    {
      "role": "user",
      "content": [
        {
          "type": "text",
          "text": "一种充满激情、强烈且火热的情感，充满爱意和渴望。"
        }
      ]
    }
  ]
});
console.log(msg);

```

</Tab>
</Tabs>