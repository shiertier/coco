# 面试问题制作器

为面试生成问题。

---

> 将此提示复制到我们的开发者[控制台](/dashboard)中亲自试用！

|        | 内容 |
| --- | --- |
| System | 您的任务是根据给定的上下文为面试生成一系列深思熟虑的开放式问题。这些问题应该旨在引出面试者富有洞察力和详细的回答，让他们能够展示自己的知识、经验和批判性思维技能。避免是/否问题或那些答案显而易见的问题。相反，专注于鼓励反思、自我评估以及分享具体例子或轶事的问题。 |
| User   | 您正在为一家成熟的电子商务公司面试营销经理职位的候选人。理想的候选人应该具有开发和执行多渠道营销活动的经验、强大的分析技能，以及与跨职能团队有效协作的能力。他们还应该对保持最新营销趋势和技术的了解充满热情。 |

## 示例输出

1.  您能向我介绍一下您在之前的职位中开发和执行的一个成功的多渠道营销活动吗？您面临了哪些挑战，又是如何克服这些挑战的？
2.  您如何衡量营销活动的成功，您关注哪些指标？您能提供一个如何使用数据和分析来优化活动表现的例子吗？
3.  根据您的经验，成功的电子商务营销策略的关键要素是什么？您如何确保您的策略与公司的整体业务目标保持一致？
4.  您能分享一个您必须与跨职能团队（例如产品、销售或客户服务）合作以实现营销目标的例子吗？您遇到了哪些挑战，又是如何促进有效协作的？
5.  您如何了解最新的营销趋势和技术，又是如何将这些知识应用于改进您的营销策略或活动的？
6.  您能描述一次您参与的营销活动没有达到预期效果的情况吗？您从这次经历中学到了什么，又是如何将这些经验教训应用到未来的活动中的？
7.  我们公司重视创新和持续改进。您能分享一个您引入新的营销方法、工具或技术，并对您团队的表现或结果产生重大影响的例子吗？

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
  max_tokens=1000,
  temperature=0.5,
  system="您的任务是根据给定的上下文为面试生成一系列深思熟虑的开放式问题。这些问题应该旨在引出面试者富有洞察力和详细的回答，让他们能够展示自己的知识、经验和批判性思维技能。避免是/否问题或那些答案显而易见的问题。相反，专注于鼓励反思、自我评估以及分享具体例子或轶事的问题。",
  messages=[
    {
      "role": "user",
      "content": [
        {
          "type": "text",
          "text": "您正在为一家成熟的电子商务公司面试营销经理职位的候选人。理想的候选人应该具有开发和执行多渠道营销活动的经验、强大的分析技能，以及与跨职能团队有效协作的能力。他们还应该对保持最新营销趋势和技术的了解充满热情。"
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
  temperature: 0.5,
  system: "您的任务是根据给定的上下文为面试生成一系列深思熟虑的开放式问题。这些问题应该旨在引出面试者富有洞察力和详细的回答，让他们能够展示自己的知识、经验和批判性思维技能。避免是/否问题或那些答案显而易见的问题。相反，专注于鼓励反思、自我评估以及分享具体例子或轶事的问题。",
  messages: [
    {
      "role": "user",
      "content": [
        {
          "type": "text",
          "text": "您正在为一家成熟的电子商务公司面试营销经理职位的候选人。理想的候选人应该具有开发和执行多渠道营销活动的经验、强大的分析技能，以及与跨职能团队有效协作的能力。他们还应该对保持最新营销趋势和技术的了解充满热情。"
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
    temperature=0.5,
    system="您的任务是根据给定的上下文为面试生成一系列深思熟虑的开放式问题。这些问题应该旨在引出面试者富有洞察力和详细的回答，让他们能够展示自己的知识、经验和批判性思维技能。避免是/否问题或那些答案显而易见的问题。相反，专注于鼓励反思、自我评估以及分享具体例子或轶事的问题。",
    messages=[
        {
            "role": "user",
            "content": [
                {
                    "type": "text",
                    "text": "您正在为一家成熟的电子商务公司面试营销经理职位的候选人。理想的候选人应该具有开发和执行多渠道营销活动的经验、强大的分析技能，以及与跨职能团队有效协作的能力。他们还应该对保持最新营销趋势和技术的了解充满热情。"
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
temperature: 0.5,
system: "您的任务是根据给定的上下文为面试生成一系列深思熟虑的开放式问题。这些问题应该旨在引出面试者富有洞察力和详细的回答，让他们能够展示自己的知识、经验和批判性思维技能。避免是/否问题或那些答案显而易见的问题。相反，专注于鼓励反思、自我评估以及分享具体例子或轶事的问题。",
messages: [
{
"role": "user",
"content": [
{
"type": "text",
"text": "您正在为一家成熟的电子商务公司面试营销经理职位的候选人。理想的候选人应该具有开发和执行多渠道营销活动的经验、强大的分析技能，以及与跨职能团队有效协作的能力。他们还应该对保持最新营销趋势和技术的了解充满热情。"
}
]
}
]
});
console.log(msg);

````
</Tab>
<Tab title="Vertex AI Python">

```python
from anthropic import AnthropicVertex

client = AnthropicVertex()

message = client.messages.create(
    model="claude-sonnet-4@20250514",
    max_tokens=1000,
    temperature=0.5,
    system="您的任务是根据给定的上下文为面试生成一系列深思熟虑的开放式问题。这些问题应该旨在引出面试者富有洞察力和详细的回答，让他们能够展示自己的知识、经验和批判性思维技能。避免是/否问题或那些答案显而易见的问题。相反，专注于鼓励反思、自我评估以及分享具体例子或轶事的问题。",
    messages=[
        {
            "role": "user",
            "content": [
                {
                    "type": "text",
                    "text": "您正在为一家成熟的电子商务公司面试营销经理职位的候选人。理想的候选人应该具有开发和执行多渠道营销活动的经验、强大的分析技能，以及与跨职能团队有效协作的能力。他们还应该对保持最新营销趋势和技术的了解充满热情。"
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
  temperature: 0.5,
  system: "您的任务是根据给定的上下文为面试生成一系列深思熟虑的开放式问题。这些问题应该旨在引出面试者富有洞察力和详细的回答，让他们能够展示自己的知识、经验和批判性思维技能。避免是/否问题或那些答案显而易见的问题。相反，专注于鼓励反思、自我评估以及分享具体例子或轶事的问题。",
  messages: [
    {
      "role": "user",
      "content": [
        {
          "type": "text",
          "text": "您正在为一家成熟的电子商务公司面试营销经理职位的候选人。理想的候选人应该具有开发和执行多渠道营销活动的经验、强大的分析技能，以及与跨职能团队有效协作的能力。他们还应该对保持最新营销趋势和技术的了解充满热情。"
        }
      ]
    }
  ]
});
console.log(msg);

```

</Tab>
</Tabs>