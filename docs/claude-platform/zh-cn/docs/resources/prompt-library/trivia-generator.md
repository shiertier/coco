# 问答题生成器

生成各种主题的问答题，并在需要时提供提示。

---

> 将此提示复制到我们的开发者[控制台](/dashboard)中亲自试用！

| 角色 | 内容 |
| ---- | --- |
| 用户 | 生成各种主题的问答题，并提供提示帮助用户得出正确答案。从多样化的类别中选择，创建测试用户知识或推理技能的问题。提供一系列逐渐具体的提示来引导用户找到解决方案。确保问题具有挑战性，提示提供足够的信息来帮助用户，但不会过于轻易地泄露答案。 |

## 示例输出

类别：地理

问题：  
世界上唯一一个同时与法国和西班牙接壤的国家是什么？

提示1：这个国家位于法国和西班牙之间的比利牛斯山脉中。  
提示2：它以其令人惊叹的山地景观、滑雪胜地和免税购物而闻名。  
提示3：该国的首都城市名称中包含了国家本身的名字。

答案：安道尔

类别：科学

问题：  
植物将阳光、水和二氧化碳转化为能量和氧气的过程叫什么？

提示1：这个过程对地球上的生命至关重要，因为它提供食物和氧气。  
提示2：它主要在植物的叶子中进行。  
提示3：这个过程涉及叶绿素，一种吸收阳光的绿色色素。

答案：光合作用

类别：文学

问题：  
谁写了1813年首次出版的经典小说《傲慢与偏见》？

提示1：作者是一位生活在1775年至1817年的英国女性。  
提示2：她以机智、深刻地描绘英国社会和礼仪而闻名。  
提示3：她的其他著名作品包括《理智与情感》和《爱玛》。

答案：简·奥斯汀

类别：历史

问题：  
分隔东西德的柏林墙是在哪一年开始被拆除的？

提示1：这个事件发生在20世纪后期。  
提示2：它标志着冷战结束的重要时刻。  
提示3：这一年通常与欧洲"共产主义垮台"这一短语相关联。

答案：1989年

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
  max_tokens=2000,
  temperature=0.5,
  messages=[
    {
    "role": "user",
    "content": [
        {
          "type": "text",
          "text": "生成各种主题的问答题，并提供提示帮助用户得出正确答案。从多样化的类别中选择，创建测试用户知识或推理技能的问题。提供一系列逐渐具体的提示来引导用户找到解决方案。确保问题具有挑战性，提示提供足够的信息来帮助用户，但不会过于轻易地泄露答案。"
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
  max_tokens: 2000,
  temperature: 0.5,
  messages: [
    {
      "role": "user",
      "content": [
        {
          "type": "text",
          "text": "生成各种主题的问答题，并提供提示帮助用户得出正确答案。从多样化的类别中选择，创建测试用户知识或推理技能的问题。提供一系列逐渐具体的提示来引导用户找到解决方案。确保问题具有挑战性，提示提供足够的信息来帮助用户，但不会过于轻易地泄露答案。"
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
temperature=0.5,
messages=[
{
"role": "user",
"content": [
{
"type": "text",
"text": "生成各种主题的问答题，并提供提示帮助用户得出正确答案。从多样化的类别中选择，创建测试用户知识或推理技能的问题。提供一系列逐渐具体的提示来引导用户找到解决方案。确保问题具有挑战性，提示提供足够的信息来帮助用户，但不会过于轻易地泄露答案。"
}
]
}
]
)
print(message.content)

````
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
  temperature: 0.5,
  messages: [
    {
      "role": "user",
      "content": [
        {
          "type": "text",
          "text": "生成各种主题的问答题，并提供提示帮助用户得出正确答案。从多样化的类别中选择，创建测试用户知识或推理技能的问题。提供一系列逐渐具体的提示来引导用户找到解决方案。确保问题具有挑战性，提示提供足够的信息来帮助用户，但不会过于轻易地泄露答案。"
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
    max_tokens=2000,
    temperature=0.5,
    messages=[
        {
            "role": "user",
            "content": [
                {
                    "type": "text",
                    "text": "生成各种主题的问答题，并提供提示帮助用户得出正确答案。从多样化的类别中选择，创建测试用户知识或推理技能的问题。提供一系列逐渐具体的提示来引导用户找到解决方案。确保问题具有挑战性，提示提供足够的信息来帮助用户，但不会过于轻易地泄露答案。"
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
  max_tokens: 2000,
  temperature: 0.5,
  messages: [
    {
      "role": "user",
      "content": [
        {
          "type": "text",
          "text": "生成各种主题的问答题，并提供提示帮助用户得出正确答案。从多样化的类别中选择，创建测试用户知识或推理技能的问题。提供一系列逐渐具体的提示来引导用户找到解决方案。确保问题具有挑战性，提示提供足够的信息来帮助用户，但不会过于轻易地泄露答案。"
        }
      ]
    }
  ]
});
console.log(msg);

```

</Tab>
</Tabs>