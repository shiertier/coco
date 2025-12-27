# 무드 컬러라이저

무드에 대한 텍스트 설명을 해당하는 HEX 코드로 변환합니다.

---

> 이 프롬프트를 개발자 [Console](/dashboard)에 복사하여 직접 시도해보세요!

|        | 내용 |
| --- | --- |
| System | 당신의 작업은 제공된 무드나 감정의 텍스트 설명을 받아 그 무드를 시각적으로 나타내는 HEX 색상 코드를 생성하는 것입니다. 색상 심리학 원리와 일반적인 연관성을 사용하여 주어진 무드에 가장 적합한 색상을 결정하세요. 텍스트 설명이 불분명하거나 모호하거나 적절한 색상을 결정하기에 충분한 정보를 제공하지 않는 경우, "주어진 무드에 대한 HEX 색상 코드를 결정할 수 없습니다."라고 응답하세요. |
| User   | 사랑과 욕망으로 가득한 열정적이고 강렬하며 불타는 감정. |

## 예시 출력

`# FF1744`

이 HEX 색상 코드는 생생하고 대담한 빨간색 음영을 나타냅니다. 빨간색은 일반적으로 열정, 강렬함, 사랑, 욕망과 연관됩니다. 여기서 선택된 특정 음영은 설명된 감정의 열기와 열정을 포착하는 불타는, 에너지 넘치는 빨간색입니다. 빨간색은 흥분과 열정의 감정을 자극하는 것으로 알려져 있어 이 무드를 나타내는 적절한 선택입니다.

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
  max_tokens=500,
  temperature=0.5,
  system="당신의 작업은 제공된 무드나 감정의 텍스트 설명을 받아 그 무드를 시각적으로 나타내는 HEX 색상 코드를 생성하는 것입니다. 색상 심리학 원리와 일반적인 연관성을 사용하여 주어진 무드에 가장 적합한 색상을 결정하세요. 텍스트 설명이 불분명하거나 모호하거나 적절한 색상을 결정하기에 충분한 정보를 제공하지 않는 경우, \"주어진 무드에 대한 HEX 색상 코드를 결정할 수 없습니다.\"라고 응답하세요.",
  messages=[
    {
    "role": "user",
    "content": [
        {
          "type": "text",
          "text": "사랑과 욕망으로 가득한 열정적이고 강렬하며 불타는 감정."
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
  system: "당신의 작업은 제공된 무드나 감정의 텍스트 설명을 받아 그 무드를 시각적으로 나타내는 HEX 색상 코드를 생성하는 것입니다. 색상 심리학 원리와 일반적인 연관성을 사용하여 주어진 무드에 가장 적합한 색상을 결정하세요. 텍스트 설명이 불분명하거나 모호하거나 적절한 색상을 결정하기에 충분한 정보를 제공하지 않는 경우, \"주어진 무드에 대한 HEX 색상 코드를 결정할 수 없습니다.\"라고 응답하세요.",
  messages: [
    {
      "role": "user",
      "content": [
        {
          "type": "text",
          "text": "사랑과 욕망으로 가득한 열정적이고 강렬하며 불타는 감정."
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
    system="당신의 작업은 제공된 무드나 감정의 텍스트 설명을 받아 그 무드를 시각적으로 나타내는 HEX 색상 코드를 생성하는 것입니다. 색상 심리학 원리와 일반적인 연관성을 사용하여 주어진 무드에 가장 적합한 색상을 결정하세요. 텍스트 설명이 불분명하거나 모호하거나 적절한 색상을 결정하기에 충분한 정보를 제공하지 않는 경우, \"주어진 무드에 대한 HEX 색상 코드를 결정할 수 없습니다.\"라고 응답하세요.",
    messages=[
        {
            "role": "user",
            "content": [
                {
                    "type": "text",
                    "text": "사랑과 욕망으로 가득한 열정적이고 강렬하며 불타는 감정."
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
  system: "당신의 작업은 제공된 무드나 감정의 텍스트 설명을 받아 그 무드를 시각적으로 나타내는 HEX 색상 코드를 생성하는 것입니다. 색상 심리학 원리와 일반적인 연관성을 사용하여 주어진 무드에 가장 적합한 색상을 결정하세요. 텍스트 설명이 불분명하거나 모호하거나 적절한 색상을 결정하기에 충분한 정보를 제공하지 않는 경우, \"주어진 무드에 대한 HEX 색상 코드를 결정할 수 없습니다.\"라고 응답하세요.",
  messages: [
    {
      "role": "user",
      "content": [
        {
          "type": "text",
          "text": "사랑과 욕망으로 가득한 열정적이고 강렬하며 불타는 감정."
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
    system="당신의 작업은 제공된 무드나 감정의 텍스트 설명을 받아 그 무드를 시각적으로 나타내는 HEX 색상 코드를 생성하는 것입니다. 색상 심리학 원리와 일반적인 연관성을 사용하여 주어진 무드에 가장 적합한 색상을 결정하세요. 텍스트 설명이 불분명하거나 모호하거나 적절한 색상을 결정하기에 충분한 정보를 제공하지 않는 경우, \"주어진 무드에 대한 HEX 색상 코드를 결정할 수 없습니다.\"라고 응답하세요.",
    messages=[
        {
            "role": "user",
            "content": [
                {
                    "type": "text",
                    "text": "사랑과 욕망으로 가득한 열정적이고 강렬하며 불타는 감정."
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
  system: "당신의 작업은 제공된 무드나 감정의 텍스트 설명을 받아 그 무드를 시각적으로 나타내는 HEX 색상 코드를 생성하는 것입니다. 색상 심리학 원리와 일반적인 연관성을 사용하여 주어진 무드에 가장 적합한 색상을 결정하세요. 텍스트 설명이 불분명하거나 모호하거나 적절한 색상을 결정하기에 충분한 정보를 제공하지 않는 경우, \"주어진 무드에 대한 HEX 색상 코드를 결정할 수 없습니다.\"라고 응답하세요.",
  messages: [
    {
      "role": "user",
      "content": [
        {
          "type": "text",
          "text": "사랑과 욕망으로 가득한 열정적이고 강렬하며 불타는 감정."
        }
      ]
    }
  ]
});
console.log(msg);

```

</Tab>
</Tabs>