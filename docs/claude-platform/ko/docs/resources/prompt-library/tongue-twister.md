# 발음하기 어려운 말

도전적인 발음하기 어려운 말을 만들어보세요.

---

> 이 프롬프트를 우리 개발자 [Console](/dashboard)에 복사해서 직접 시도해보세요!

| 역할 | 내용 |
| ---- | --- |
| 사용자 | 복잡하고 창의적인 발음하기 어려운 말을 생성하세요. 말하기 어려울 뿐만 아니라 흥미롭고 재미있으며 잠재적으로 유머러스한 발음하기 어려운 말을 만드는 것을 목표로 하세요. 발음하기 어려운 말의 난이도와 즐거움을 높이기 위해 말장난, 운율, 두운법을 포함하는 것을 고려하세요. |

## 예시 출력

발음하기 어려운 말 1:  
 영리한 뾰족쥐가 반짝이는 빛나는 신발을 씹어 뚫었다.

발음하기 어려운 말 2:  
 상쾌한 바람이 고귀한 혈통의 수다스럽고 지껄이는 개코원숭이 옆으로 불어갔다.

발음하기 어려운 말 3:  
 당황한 피리 부는 사람이 피리 연주를 연습하면서 절인 고추 한 펙을 골랐다.

발음하기 어려운 말 4:  
 여섯 개의 매끄럽고 윤기 나며 가느다란 썰매가 가파르고 눈 덮인 비탈을 빠르게 미끄러져 내려갔다.

---

## API 요청

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
  temperature=1,
  messages=[
    {
      "role": "user",
      "content": [
        {
          "type": "text",
          "text": "복잡하고 창의적인 발음하기 어려운 말을 생성하세요. 말하기 어려울 뿐만 아니라 흥미롭고 재미있으며 잠재적으로 유머러스한 발음하기 어려운 말을 만드는 것을 목표로 하세요. 발음하기 어려운 말의 난이도와 즐거움을 높이기 위해 말장난, 운율, 두운법을 포함하는 것을 고려하세요."
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
  temperature: 1,
  messages: [
    {
      "role": "user",
      "content": [
        {
          "type": "text",
          "text": "복잡하고 창의적인 발음하기 어려운 말을 생성하세요. 말하기 어려울 뿐만 아니라 흥미롭고 재미있으며 잠재적으로 유머러스한 발음하기 어려운 말을 만드는 것을 목표로 하세요. 발음하기 어려운 말의 난이도와 즐거움을 높이기 위해 말장난, 운율, 두운법을 포함하는 것을 고려하세요."
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
    messages=[
        {
            "role": "user",
            "content": [
                {
                    "type": "text",
                    "text": "복잡하고 창의적인 발음하기 어려운 말을 생성하세요. 말하기 어려울 뿐만 아니라 흥미롭고 재미있으며 잠재적으로 유머러스한 발음하기 어려운 말을 만드는 것을 목표로 하세요. 발음하기 어려운 말의 난이도와 즐거움을 높이기 위해 말장난, 운율, 두운법을 포함하는 것을 고려하세요."
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
  messages: [
    {
      "role": "user",
      "content": [
        {
          "type": "text",
          "text": "복잡하고 창의적인 발음하기 어려운 말을 생성하세요. 말하기 어려울 뿐만 아니라 흥미롭고 재미있으며 잠재적으로 유머러스한 발음하기 어려운 말을 만드는 것을 목표로 하세요. 발음하기 어려운 말의 난이도와 즐거움을 높이기 위해 말장난, 운율, 두운법을 포함하는 것을 고려하세요."
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
    messages=[
        {
            "role": "user",
            "content": [
                {
                    "type": "text",
                    "text": "복잡하고 창의적인 발음하기 어려운 말을 생성하세요. 말하기 어려울 뿐만 아니라 흥미롭고 재미있으며 잠재적으로 유머러스한 발음하기 어려운 말을 만드는 것을 목표로 하세요. 발음하기 어려운 말의 난이도와 즐거움을 높이기 위해 말장난, 운율, 두운법을 포함하는 것을 고려하세요."
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
  messages: [
    {
      "role": "user",
      "content": [
        {
          "type": "text",
          "text": "복잡하고 창의적인 발음하기 어려운 말을 생성하세요. 말하기 어려울 뿐만 아니라 흥미롭고 재미있으며 잠재적으로 유머러스한 발음하기 어려운 말을 만드는 것을 목표로 하세요. 발음하기 어려운 말의 난이도와 즐거움을 높이기 위해 말장난, 운율, 두운법을 포함하는 것을 고려하세요."
        }
      ]
    }
  ]
});
console.log(msg);

```

</Tab>
</Tabs>