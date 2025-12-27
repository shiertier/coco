# 二年级简化器

让复杂文本变得易于年轻学习者理解。

---

> 将此提示复制到我们的开发者[控制台](/dashboard)中亲自试用！

|        | 内容 |
| --- | --- |
| System | 您的任务是将提供的文本重写为3-5年级的年轻学习者易于阅读和理解的方式。简化高级词汇，分解长句子，用简单的语言解释困难的概念，并以清晰、引人入胜的方式呈现信息。简短的重写文本应该以适合年龄的方式传达原文的核心思想。 |
| User   | 线粒体，通常被称为细胞的动力源，是重要的细胞器，在通过细胞呼吸过程产生三磷酸腺苷（ATP）方面发挥着关键作用。ATP是细胞的主要能量货币，使各种细胞功能和生化反应得以发生。 |

## 示例输出

线粒体是细胞内的微小部分，就像小发电站一样工作。它们制造一种叫做ATP的特殊分子，这就像细胞的能量货币。ATP帮助细胞做重要的工作并保持其平稳运行，就像金钱帮助人们购买他们需要的东西一样。线粒体非常重要，因为没有它们，细胞就没有能量正常运作。

---

## API请求

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
  system="您的任务是将提供的文本重写为3-5年级的年轻学习者易于阅读和理解的方式。简化高级词汇，分解长句子，用简单的语言解释困难的概念，并以清晰、引人入胜的方式呈现信息。简短的重写文本应该以适合年龄的方式传达原文的核心思想。",
  messages=[
    {
      "role": "user",
      "content": [
        {
          "type": "text",
          "text": "线粒体，通常被称为细胞的动力源，是重要的细胞器，在通过细胞呼吸过程产生三磷酸腺苷（ATP）方面发挥着关键作用。ATP是细胞的主要能量货币，使各种细胞功能和生化反应得以发生。"
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
  system: "您的任务是将提供的文本重写为3-5年级的年轻学习者易于阅读和理解的方式。简化高级词汇，分解长句子，用简单的语言解释困难的概念，并以清晰、引人入胜的方式呈现信息。简短的重写文本应该以适合年龄的方式传达原文的核心思想。",
  messages: [
    {
      "role": "user",
      "content": [
        {
          "type": "text",
          "text": "线粒体，通常被称为细胞的动力源，是重要的细胞器，在通过细胞呼吸过程产生三磷酸腺苷（ATP）方面发挥着关键作用。ATP是细胞的主要能量货币，使各种细胞功能和生化反应得以发生。"
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
    system="您的任务是将提供的文本重写为3-5年级的年轻学习者易于阅读和理解的方式。简化高级词汇，分解长句子，用简单的语言解释困难的概念，并以清晰、引人入胜的方式呈现信息。简短的重写文本应该以适合年龄的方式传达原文的核心思想。",
    messages=[
        {
            "role": "user",
            "content": [
                {
                    "type": "text",
                    "text": "线粒体，通常被称为细胞的动力源，是重要的细胞器，在通过细胞呼吸过程产生三磷酸腺苷（ATP）方面发挥着关键作用。ATP是细胞的主要能量货币，使各种细胞功能和生化反应得以发生。"
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
  system: "您的任务是将提供的文本重写为3-5年级的年轻学习者易于阅读和理解的方式。简化高级词汇，分解长句子，用简单的语言解释困难的概念，并以清晰、引人入胜的方式呈现信息。简短的重写文本应该以适合年龄的方式传达原文的核心思想。",
  messages: [
    {
      "role": "user",
      "content": [
        {
          "type": "text",
          "text": "线粒体，通常被称为细胞的动力源，是重要的细胞器，在通过细胞呼吸过程产生三磷酸腺苷（ATP）方面发挥着关键作用。ATP是细胞的主要能量货币，使各种细胞功能和生化反应得以发生。"
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
system="您的任务是将提供的文本重写为3-5年级的年轻学习者易于阅读和理解的方式。简化高级词汇，分解长句子，用简单的语言解释困难的概念，并以清晰、引人入胜的方式呈现信息。简短的重写文本应该以适合年龄的方式传达原文的核心思想。",
messages=[
{
"role": "user",
"content": [
{
"type": "text",
"text": "线粒体，通常被称为细胞的动力源，是重要的细胞器，在通过细胞呼吸过程产生三磷酸腺苷（ATP）方面发挥着关键作用。ATP是细胞的主要能量货币，使各种细胞功能和生化反应得以发生。"
}
]
}
]
)
print(message.content)

````

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
  system: "您的任务是将提供的文本重写为3-5年级的年轻学习者易于阅读和理解的方式。简化高级词汇，分解长句子，用简单的语言解释困难的概念，并以清晰、引人入胜的方式呈现信息。简短的重写文本应该以适合年龄的方式传达原文的核心思想。",
  messages: [
    {
      "role": "user",
      "content": [
        {
          "type": "text",
          "text": "线粒体，通常被称为细胞的动力源，是重要的细胞器，在通过细胞呼吸过程产生三磷酸腺苷（ATP）方面发挥着关键作用。ATP是细胞的主要能量货币，使各种细胞功能和生化反应得以发生。"
        }
      ]
    }
  ]
});
console.log(msg);

````

</Tab>
</Tabs>