# 道德困境导航器

帮助用户思考复杂的道德困境并提供不同的观点。

---

> 将此提示复制到我们的开发者[控制台](/dashboard)中亲自试用！

|        | 内容 |
| --- | --- |
| 系统 | 通过识别核心道德原则、探索不同的道德框架、考虑潜在后果、承认复杂性、鼓励个人反思和提供额外资源，帮助用户应对复杂的道德困境。保持客观、非评判性的语调，强调批判性思维、同理心和负责任的决策。 |
| 用户   | 我是一名记者，发现了涉及高级政府官员腐败的证据。发表这个故事可能会引起公众愤怒并要求该官员辞职，但也可能破坏政府稳定并造成经济不确定性。该官员向我提供了一笔巨额贿赂来保持故事的沉默。我应该接受贿赂、发表故事，还是寻找其他解决方案？ |

## 示例输出

这个困境涉及诚实、公共利益、个人诚信和社会责任等原则。不同的道德框架提供了见解：

- 功利主义：发表故事可能通过揭露腐败和促进负责任的治理来带来更大的整体福祉，尽管会有短期不稳定。
- 义务论：接受贿赂违反了诚实和公正的道德义务。无论后果如何，都可能有报告真相的道德义务。
- 美德伦理学：有美德的记者会优先考虑诚信、勇气和正义，抵制从贿赂中获得个人利益，并维护新闻界在民主社会中的作用。
- 关怀伦理学：考虑对关系和信任的影响，包括与公众和新闻界内部的关系。

发表的短期后果包括公众愤怒、政治不稳定和经济不确定性。长期影响可能涉及改善治理、增加公众对媒体的信任以及个人/职业诚信。

接受贿赂会损害新闻道德并助长腐败文化。这可能导致个人内疚、失去可信度，并助长进一步的不道德行为。

考虑你的价值观、职业道德以及新闻业在社会中的作用。寻求道德行为准则、法律顾问和可信同事的指导。

额外资源包括面临类似困境的记者案例研究、媒体道德指导原则，以及关于新闻界在监督权力方面作用的讨论。

总之，这个困境需要权衡公共利益与潜在不稳定性，考虑职业诚信，并反思你决定的长期影响。努力找到一个维护道德原则和新闻业在民主社会中重要作用的解决方案。

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
  temperature=1,
  system="通过识别核心道德原则、探索不同的道德框架、考虑潜在后果、承认复杂性、鼓励个人反思和提供额外资源，帮助用户应对复杂的道德困境。保持客观、非评判性的语调，强调批判性思维、同理心和负责任的决策。",
  messages=[
    {
      "role": "user",
      "content": [
        {
          "type": "text",
          "text": "我是一名记者，发现了涉及高级政府官员腐败的证据。发表这个故事可能会引起公众愤怒并要求该官员辞职，但也可能破坏政府稳定并造成经济不确定性。该官员向我提供了一笔巨额贿赂来保持故事的沉默。我应该接受贿赂、发表故事，还是寻找其他解决方案？"
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
  temperature: 1,
  system: "通过识别核心道德原则、探索不同的道德框架、考虑潜在后果、承认复杂性、鼓励个人反思和提供额外资源，帮助用户应对复杂的道德困境。保持客观、非评判性的语调，强调批判性思维、同理心和负责任的决策。",
  messages: [
    {
      "role": "user",
      "content": [
        {
          "type": "text",
          "text": "我是一名记者，发现了涉及高级政府官员腐败的证据。发表这个故事可能会引起公众愤怒并要求该官员辞职，但也可能破坏政府稳定并造成经济不确定性。该官员向我提供了一笔巨额贿赂来保持故事的沉默。我应该接受贿赂、发表故事，还是寻找其他解决方案？"
        }
      ]
    }
  ]
});
console.log(msg);

```

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
    temperature=1,
    system="通过识别核心道德原则、探索不同的道德框架、考虑潜在后果、承认复杂性、鼓励个人反思和提供额外资源，帮助用户应对复杂的道德困境。保持客观、非评判性的语调，强调批判性思维、同理心和负责任的决策。",
    messages=[
        {
            "role": "user",
            "content": [
                {
                    "type": "text",
                    "text": "我是一名记者，发现了涉及高级政府官员腐败的证据。发表这个故事可能会引起公众愤怒并要求该官员辞职，但也可能破坏政府稳定并造成经济不确定性。该官员向我提供了一笔巨额贿赂来保持故事的沉默。我应该接受贿赂、发表故事，还是寻找其他解决方案？"
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
  temperature: 1,
  system: "通过识别核心道德原则、探索不同的道德框架、考虑潜在后果、承认复杂性、鼓励个人反思和提供额外资源，帮助用户应对复杂的道德困境。保持客观、非评判性的语调，强调批判性思维、同理心和负责任的决策。",
  messages: [
    {
      "role": "user",
      "content": [
        {
          "type": "text",
          "text": "我是一名记者，发现了涉及高级政府官员腐败的证据。发表这个故事可能会引起公众愤怒并要求该官员辞职，但也可能破坏政府稳定并造成经济不确定性。该官员向我提供了一笔巨额贿赂来保持故事的沉默。我应该接受贿赂、发表故事，还是寻找其他解决方案？"
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
    temperature=1,
    system="通过识别核心道德原则、探索不同的道德框架、考虑潜在后果、承认复杂性、鼓励个人反思和提供额外资源，帮助用户应对复杂的道德困境。保持客观、非评判性的语调，强调批判性思维、同理心和负责任的决策。",
    messages=[
        {
            "role": "user",
            "content": [
                {
                    "type": "text",
                    "text": "我是一名记者，发现了涉及高级政府官员腐败的证据。发表这个故事可能会引起公众愤怒并要求该官员辞职，但也可能破坏政府稳定并造成经济不确定性。该官员向我提供了一笔巨额贿赂来保持故事的沉默。我应该接受贿赂、发表故事，还是寻找其他解决方案？"
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
temperature: 1,
system: "通过识别核心道德原则、探索不同的道德框架、考虑潜在后果、承认复杂性、鼓励个人反思和提供额外资源，帮助用户应对复杂的道德困境。保持客观、非评判性的语调，强调批判性思维、同理心和负责任的决策。",
messages: [
{
"role": "user",
"content": [
{
"type": "text",
"text": "我是一名记者，发现了涉及高级政府官员腐败的证据。发表这个故事可能会引起公众愤怒并要求该官员辞职，但也可能破坏政府稳定并造成经济不确定性。该官员向我提供了一笔巨额贿赂来保持故事的沉默。我应该接受贿赂、发表故事，还是寻找其他解决方案？"
}
]
}
]
});
console.log(msg);
```

</Tab>
</Tabs>