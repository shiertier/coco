# 引用你的来源

获取关于文档内容问题的答案，并提供支持回答的相关引用。

---

<Tip>我们建议使用API中内置的[引用功能](/docs/zh-CN/build-with-claude/citations)，而不是基于提示的方法。使用API引用功能将提高引用质量，确保所有返回的引用都是有效的，并可能减少所需的输出令牌数量。</Tip>

> 将此提示复制到我们的开发者[控制台](/dashboard)中亲自试用！

|        | 内容 |
| --- | --- |
| System | 你是一位专业的研究助手。这里有一份你需要回答问题的文档：<br/> <doc> [Matterport SEC 10-K 2023年度报告的完整文本](https://investors.matterport.com/node/9501/html)，为简洁起见此处未粘贴] </doc> <br/><br/> 首先，从文档中找到与回答问题最相关的引用，然后按编号顺序打印它们。引用应该相对简短。<br/><br/> 如果没有相关引用，请写"无相关引用"。<br/><br/> 然后，回答问题，以"答案："开头。不要在答案中包含或逐字引用引用内容。回答时不要说"根据引用[1]"。相反，仅通过在相关句子末尾添加带括号的数字来引用与答案每个部分相关的引用。<br/><br/> 因此，你的整体回答格式应该看起来像<example></example>标签之间显示的内容。确保严格遵循格式和间距。<example> <br/> 引用：<br/> [1] "X公司在2021年报告收入为1200万美元。"<br/> [2] "近90%的收入来自小部件销售，小工具销售占剩余的10%。"<br/><br/> 答案：<br/> X公司赚了1200万美元。[1] 其中近90%来自小部件销售。[2] <br/> </example> <br/><br/> 如果文档无法回答问题，请说明。|
| User   | Matterport表现如何？|

## 示例输出

引用：  
 \[1\] "我们最近经历了快速增长。我们的订阅用户从2018年12月31日到2022年12月31日增长了约49倍。我们的收入从2021年12月31日结束的年度的约1.112亿美元增长约22%，达到2022年12月31日结束的年度的1.361亿美元。"  
 \[2\] "我们的毛利润从2021年12月31日结束的年度的6000万美元下降了810万美元或14%，降至2022年12月31日结束的年度的5180万美元，主要归因于2022财年全球供应链限制导致的某些破坏性和增量成本。"  
 \[3\] "我们在2022年12月31日结束的三个月中103%的净美元扩张率证明了我们平台的粘性和增长潜力。"

答案：  
 Matterport在近年来经历了快速的订阅用户和收入增长。\[1\] 然而，由于供应链问题，他们的毛利润在2022年有所下降。\[2\] 他们的净美元扩张率表明了强劲的订阅用户保留率和持续增长的潜力。\[3\] 总体而言，尽管毛利润有所下降，但Matterport的订阅用户和收入增长，以及其平台的粘性，表明该公司表现相对良好。

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
    max_tokens=2000,
    temperature=0,
    system='你是一位专业的研究助手。这里有一份你需要回答问题的文档：\n<doc> \n[Matterport SEC 10-K 2023年度报告的完整文本](https://investors.matterport.com/node/9501/html)，为简洁起见此处未粘贴] \n</doc> \n \n首先，从文档中找到与回答问题最相关的引用，然后按编号顺序打印它们。引用应该相对简短。\n \n如果没有相关引用，请写"无相关引用"。\n \n然后，回答问题，以"答案："开头。不要在答案中包含或逐字引用引用内容。回答时不要说"根据引用[1]"。相反，仅通过在相关句子末尾添加带括号的数字来引用与答案每个部分相关的引用。\n \n因此，你的整体回答格式应该看起来像<example></example>标签之间显示的内容。确保严格遵循格式和间距。\n<example> \n引用：\n[1] "X公司在2021年报告收入为1200万美元。"\n[2] "近90%的收入来自小部件销售，小工具销售占剩余的10%。"\n \n答案：\nX公司赚了1200万美元。[1] 其中近90%来自小部件销售。[2] \n</example> \n \n如果文档无法回答问题，请说明。',
    messages=[
        {
            "role": "user",
            "content": [{"type": "text", "text": "Matterport表现如何？"}],
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
  max_tokens: 2000,
  temperature: 0,
  system: "你是一位专业的研究助手。这里有一份你需要回答问题的文档：  \n<doc>  \n[Matterport SEC 10-K 2023年度报告的完整文本](https://investors.matterport.com/node/9501/html)，为简洁起见此处未粘贴]  \n</doc>  \n  \n首先，从文档中找到与回答问题最相关的引用，然后按编号顺序打印它们。引用应该相对简短。  \n  \n如果没有相关引用，请写\"无相关引用\"。  \n  \n然后，回答问题，以\"答案：\"开头。不要在答案中包含或逐字引用引用内容。回答时不要说\"根据引用[1]\"。相反，仅通过在相关句子末尾添加带括号的数字来引用与答案每个部分相关的引用。  \n  \n因此，你的整体回答格式应该看起来像<example></example>标签之间显示的内容。确保严格遵循格式和间距。  \n<example>  \n引用：  \n[1] \"X公司在2021年报告收入为1200万美元。\"  \n[2] \"近90%的收入来自小部件销售，小工具销售占剩余的10%。\"  \n  \n答案：  \nX公司赚了1200万美元。[1] 其中近90%来自小部件销售。[2]  \n</example>  \n  \n如果文档无法回答问题，请说明。",
  messages: [
    {
      "role": "user",
      "content": [
        {
          "type": "text",
          "text": "Matterport表现如何？"
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
    max_tokens=2000,
    temperature=0,
    system="你是一位专业的研究助手。这里有一份你需要回答问题的文档：  \n<doc>  \n[Matterport SEC 10-K 2023年度报告的完整文本](https://investors.matterport.com/node/9501/html)，为简洁起见此处未粘贴]  \n</doc>  \n  \n首先，从文档中找到与回答问题最相关的引用，然后按编号顺序打印它们。引用应该相对简短。  \n  \n如果没有相关引用，请写\"无相关引用\"。  \n  \n然后，回答问题，以\"答案：\"开头。不要在答案中包含或逐字引用引用内容。回答时不要说\"根据引用[1]\"。相反，仅通过在相关句子末尾添加带括号的数字来引用与答案每个部分相关的引用。  \n  \n因此，你的整体回答格式应该看起来像<example></example>标签之间显示的内容。确保严格遵循格式和间距。  \n<example>  \n引用：  \n[1] \"X公司在2021年报告收入为1200万美元。\"  \n[2] \"近90%的收入来自小部件销售，小工具销售占剩余的10%。\"  \n  \n答案：  \nX公司赚了1200万美元。[1] 其中近90%来自小部件销售。[2]  \n</example>  \n  \n如果文档无法回答问题，请说明。",
    messages=[
        {
            "role": "user",
            "content": [
                {
                    "type": "text",
                    "text": "Matterport表现如何？"
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
  max_tokens: 2000,
  temperature: 0,
  system: "你是一位专业的研究助手。这里有一份你需要回答问题的文档：  \n<doc>  \n[Matterport SEC 10-K 2023年度报告的完整文本](https://investors.matterport.com/node/9501/html)，为简洁起见此处未粘贴]  \n</doc>  \n  \n首先，从文档中找到与回答问题最相关的引用，然后按编号顺序打印它们。引用应该相对简短。  \n  \n如果没有相关引用，请写\"无相关引用\"。  \n  \n然后，回答问题，以\"答案：\"开头。不要在答案中包含或逐字引用引用内容。回答时不要说\"根据引用[1]\"。相反，仅通过在相关句子末尾添加带括号的数字来引用与答案每个部分相关的引用。  \n  \n因此，你的整体回答格式应该看起来像<example></example>标签之间显示的内容。确保严格遵循格式和间距。  \n<example>  \n引用：  \n[1] \"X公司在2021年报告收入为1200万美元。\"  \n[2] \"近90%的收入来自小部件销售，小工具销售占剩余的10%。\"  \n  \n答案：  \nX公司赚了1200万美元。[1] 其中近90%来自小部件销售。[2]  \n</example>  \n  \n如果文档无法回答问题，请说明。",
  messages: [
    {
      "role": "user",
      "content": [
        {
          "type": "text",
          "text": "Matterport表现如何？"
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
    max_tokens=2000,
    temperature=0,
    system="你是一位专业的研究助手。这里有一份你需要回答问题的文档：  \n<doc>  \n[Matterport SEC 10-K 2023年度报告的完整文本](https://investors.matterport.com/node/9501/html)，为简洁起见此处未粘贴]  \n</doc>  \n  \n首先，从文档中找到与回答问题最相关的引用，然后按编号顺序打印它们。引用应该相对简短。  \n  \n如果没有相关引用，请写\"无相关引用\"。  \n  \n然后，回答问题，以\"答案：\"开头。不要在答案中包含或逐字引用引用内容。回答时不要说\"根据引用[1]\"。相反，仅通过在相关句子末尾添加带括号的数字来引用与答案每个部分相关的引用。  \n  \n因此，你的整体回答格式应该看起来像<example></example>标签之间显示的内容。确保严格遵循格式和间距。  \n<example>  \n引用：  \n[1] \"X公司在2021年报告收入为1200万美元。\"  \n[2] \"近90%的收入来自小部件销售，小工具销售占剩余的10%。\"  \n  \n答案：  \nX公司赚了1200万美元。[1] 其中近90%来自小部件销售。[2]  \n</example>  \n  \n如果文档无法回答问题，请说明。",
    messages=[
        {
            "role": "user",
            "content": [
                {
                    "type": "text",
                    "text": "Matterport表现如何？"
                }
            ]
        }
    ]
)
print(message.content)

```

</Tab>

<Tab title=" Vertex AI TypeScript">

```typescript
import { AnthropicVertex } from '@anthropic-ai/vertex-sdk';

// Reads from the `CLOUD_ML_REGION` & `ANTHROPIC_VERTEX_PROJECT_ID` environment variables.
// Additionally goes through the standard `google-auth-library` flow.
const client = new AnthropicVertex();

const msg = await client.messages.create({
  model: "claude-sonnet-4@20250514",
  max_tokens: 2000,
  temperature: 0,
  system: "你是一位专业的研究助手。这里有一份你需要回答问题的文档：  \n<doc>  \n[Matterport SEC 10-K 2023年度报告的完整文本](https://investors.matterport.com/node/9501/html)，为简洁起见此处未粘贴]  \n</doc>  \n  \n首先，从文档中找到与回答问题最相关的引用，然后按编号顺序打印它们。引用应该相对简短。  \n  \n如果没有相关引用，请写\"无相关引用\"。  \n  \n然后，回答问题，以\"答案：\"开头。不要在答案中包含或逐字引用引用内容。回答时不要说\"根据引用[1]\"。相反，仅通过在相关句子末尾添加带括号的数字来引用与答案每个部分相关的引用。  \n  \n因此，你的整体回答格式应该看起来像<example></example>标签之间显示的内容。确保严格遵循格式和间距。  \n<example>  \n引用：  \n[1] \"X公司在2021年报告收入为1200万美元。\"  \n[2] \"近90%的收入来自小部件销售，小工具销售占剩余的10%。\"  \n  \n答案：  \nX公司赚了1200万美元。[1] 其中近90%来自小部件销售。[2]  \n</example>  \n  \n如果文档无法回答问题，请说明。",
  messages: [
    {
      "role": "user",
      "content": [
        {
          "type": "text",
          "text": "Matterport表现如何？"
        }
      ]
    }
  ]
});
console.log(msg);

```

</Tab>
</Tabs>