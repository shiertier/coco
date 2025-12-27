# 초등학교 2학년 수준 단순화기

복잡한 텍스트를 어린 학습자들이 이해하기 쉽게 만들어 줍니다.

---

> 이 프롬프트를 우리 개발자 [Console](/dashboard)에 복사해서 직접 사용해 보세요!

|        | 내용 |
| --- | --- |
| System | 당신의 임무는 제공된 텍스트를 3-5학년의 어린 학습자들이 읽고 이해하기 쉬운 방식으로 다시 작성하는 것입니다. 고급 어휘를 단순화하고, 긴 문장을 나누고, 어려운 개념을 평이한 언어로 설명하며, 정보를 명확하고 흥미로운 방식으로 제시하세요. 짧게 다시 작성된 텍스트는 연령에 적합한 방식으로 원본 텍스트의 핵심 아이디어를 전달해야 합니다. |
| User   | 미토콘드리아는 종종 세포의 발전소라고 불리며, 세포 호흡 과정을 통해 아데노신 삼인산(ATP) 생산에 중요한 역할을 하는 필수적인 세포 소기관입니다. ATP는 세포의 주요 에너지 화폐로, 다양한 세포 기능과 생화학적 반응이 일어날 수 있게 해줍니다. |

## 예시 출력

미토콘드리아는 세포 안에 있는 작은 부분으로 작은 발전소처럼 작동합니다. 이들은 ATP라고 불리는 특별한 분자를 만드는데, 이것은 세포의 에너지 돈과 같습니다. ATP는 세포가 중요한 일을 하고 원활하게 작동하도록 도와주는데, 마치 돈이 사람들이 필요한 것을 사는 데 도움을 주는 것과 같습니다. 미토콘드리아는 매우 중요한데, 이것들이 없다면 세포는 제대로 기능할 에너지를 갖지 못할 것이기 때문입니다.

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
  temperature=0,
  system="당신의 임무는 제공된 텍스트를 3-5학년의 어린 학습자들이 읽고 이해하기 쉬운 방식으로 다시 작성하는 것입니다. 고급 어휘를 단순화하고, 긴 문장을 나누고, 어려운 개념을 평이한 언어로 설명하며, 정보를 명확하고 흥미로운 방식으로 제시하세요. 짧게 다시 작성된 텍스트는 연령에 적합한 방식으로 원본 텍스트의 핵심 아이디어를 전달해야 합니다.",
  messages=[
    {
      "role": "user",
      "content": [
        {
          "type": "text",
          "text": "미토콘드리아는 종종 세포의 발전소라고 불리며, 세포 호흡 과정을 통해 아데노신 삼인산(ATP) 생산에 중요한 역할을 하는 필수적인 세포 소기관입니다. ATP는 세포의 주요 에너지 화폐로, 다양한 세포 기능과 생화학적 반응이 일어날 수 있게 해줍니다."
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
  system: "당신의 임무는 제공된 텍스트를 3-5학년의 어린 학습자들이 읽고 이해하기 쉬운 방식으로 다시 작성하는 것입니다. 고급 어휘를 단순화하고, 긴 문장을 나누고, 어려운 개념을 평이한 언어로 설명하며, 정보를 명확하고 흥미로운 방식으로 제시하세요. 짧게 다시 작성된 텍스트는 연령에 적합한 방식으로 원본 텍스트의 핵심 아이디어를 전달해야 합니다.",
  messages: [
    {
      "role": "user",
      "content": [
        {
          "type": "text",
          "text": "미토콘드리아는 종종 세포의 발전소라고 불리며, 세포 호흡 과정을 통해 아데노신 삼인산(ATP) 생산에 중요한 역할을 하는 필수적인 세포 소기관입니다. ATP는 세포의 주요 에너지 화폐로, 다양한 세포 기능과 생화학적 반응이 일어날 수 있게 해줍니다."
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
    system="당신의 임무는 제공된 텍스트를 3-5학년의 어린 학습자들이 읽고 이해하기 쉬운 방식으로 다시 작성하는 것입니다. 고급 어휘를 단순화하고, 긴 문장을 나누고, 어려운 개념을 평이한 언어로 설명하며, 정보를 명확하고 흥미로운 방식으로 제시하세요. 짧게 다시 작성된 텍스트는 연령에 적합한 방식으로 원본 텍스트의 핵심 아이디어를 전달해야 합니다.",
    messages=[
        {
            "role": "user",
            "content": [
                {
                    "type": "text",
                    "text": "미토콘드리아는 종종 세포의 발전소라고 불리며, 세포 호흡 과정을 통해 아데노신 삼인산(ATP) 생산에 중요한 역할을 하는 필수적인 세포 소기관입니다. ATP는 세포의 주요 에너지 화폐로, 다양한 세포 기능과 생화학적 반응이 일어날 수 있게 해줍니다."
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
  system: "당신의 임무는 제공된 텍스트를 3-5학년의 어린 학습자들이 읽고 이해하기 쉬운 방식으로 다시 작성하는 것입니다. 고급 어휘를 단순화하고, 긴 문장을 나누고, 어려운 개념을 평이한 언어로 설명하며, 정보를 명확하고 흥미로운 방식으로 제시하세요. 짧게 다시 작성된 텍스트는 연령에 적합한 방식으로 원본 텍스트의 핵심 아이디어를 전달해야 합니다.",
  messages: [
    {
      "role": "user",
      "content": [
        {
          "type": "text",
          "text": "미토콘드리아는 종종 세포의 발전소라고 불리며, 세포 호흡 과정을 통해 아데노신 삼인산(ATP) 생산에 중요한 역할을 하는 필수적인 세포 소기관입니다. ATP는 세포의 주요 에너지 화폐로, 다양한 세포 기능과 생화학적 반응이 일어날 수 있게 해줍니다."
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
system="당신의 임무는 제공된 텍스트를 3-5학년의 어린 학습자들이 읽고 이해하기 쉬운 방식으로 다시 작성하는 것입니다. 고급 어휘를 단순화하고, 긴 문장을 나누고, 어려운 개념을 평이한 언어로 설명하며, 정보를 명확하고 흥미로운 방식으로 제시하세요. 짧게 다시 작성된 텍스트는 연령에 적합한 방식으로 원본 텍스트의 핵심 아이디어를 전달해야 합니다.",
messages=[
{
"role": "user",
"content": [
{
"type": "text",
"text": "미토콘드리아는 종종 세포의 발전소라고 불리며, 세포 호흡 과정을 통해 아데노신 삼인산(ATP) 생산에 중요한 역할을 하는 필수적인 세포 소기관입니다. ATP는 세포의 주요 에너지 화폐로, 다양한 세포 기능과 생화학적 반응이 일어날 수 있게 해줍니다."
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
  system: "당신의 임무는 제공된 텍스트를 3-5학년의 어린 학습자들이 읽고 이해하기 쉬운 방식으로 다시 작성하는 것입니다. 고급 어휘를 단순화하고, 긴 문장을 나누고, 어려운 개념을 평이한 언어로 설명하며, 정보를 명확하고 흥미로운 방식으로 제시하세요. 짧게 다시 작성된 텍스트는 연령에 적합한 방식으로 원본 텍스트의 핵심 아이디어를 전달해야 합니다.",
  messages: [
    {
      "role": "user",
      "content": [
        {
          "type": "text",
          "text": "미토콘드리아는 종종 세포의 발전소라고 불리며, 세포 호흡 과정을 통해 아데노신 삼인산(ATP) 생산에 중요한 역할을 하는 필수적인 세포 소기관입니다. ATP는 세포의 주요 에너지 화폐로, 다양한 세포 기능과 생화학적 반응이 일어날 수 있게 해줍니다."
        }
      ]
    }
  ]
});
console.log(msg);

````

</Tab>
</Tabs>