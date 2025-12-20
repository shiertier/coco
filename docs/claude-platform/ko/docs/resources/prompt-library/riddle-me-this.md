# 수수께끼를 내어보세요

수수께끼를 생성하고 사용자가 해답에 도달할 수 있도록 안내합니다.

---

> 이 프롬프트를 우리의 개발자 [Console](/dashboard)에 복사하여 직접 시도해보세요!

| 역할 | 내용 |
| ---- | --- |
| User | 영리한 수수께끼를 생성하고 사용자가 올바른 해답에 도달할 수 있도록 단계별 가이드를 제공하세요. 수수께끼는 도전적이지만 논리적 사고와 세심한 주의를 통해 해결 가능해야 합니다. 각 수수께끼를 제시한 후, 사용자를 점진적으로 답으로 이끄는 힌트나 질문들을 제공하세요. 힌트는 너무 명백하지 않으면서도 사용자의 사고 과정을 안내하기에 충분한 정보를 제공해야 합니다. 마지막으로 해답을 공개하고 주어진 힌트를 사용하여 수수께끼를 어떻게 해결할 수 있는지에 대한 간단한 설명을 제공하세요. |

## 예시 출력

나는 살아있지 않지만 자란다; 나는 폐가 없지만 공기가 필요하다; 나는 입이 없지만 물이 나를 죽인다. 나는 무엇인가?

힌트 1: 물질의 다양한 상태와 그것들이 어떻게 변할 수 있는지 생각해보세요.  
힌트 2: 언급된 세 가지 요소 간의 관계를 생각해보세요: 공기, 물, 그리고 신비한 물체.  
힌트 3: 답은 일상생활에서 발생하는 흔한 현상과 관련이 있습니다.

해답: 불.  
설명: 불은 살아있지 않지만 연료를 소비하면서 자라고 퍼집니다. 폐가 없지만 지속되기 위해 산소(공기)가 필요합니다. 입이 없지만 물은 종종 불을 끄는 데 사용되어 효과적으로 불을 "죽입니다".

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
  messages=[
    {
      "role": "user",
      "content": [
        {
          "type": "text",
          "text": "영리한 수수께끼를 생성하고 사용자가 올바른 해답에 도달할 수 있도록 단계별 가이드를 제공하세요. 수수께끼는 도전적이지만 논리적 사고와 세심한 주의를 통해 해결 가능해야 합니다. 각 수수께끼를 제시한 후, 사용자를 점진적으로 답으로 이끄는 힌트나 질문들을 제공하세요. 힌트는 너무 명백하지 않으면서도 사용자의 사고 과정을 안내하기에 충분한 정보를 제공해야 합니다. 마지막으로 해답을 공개하고 주어진 힌트를 사용하여 수수께끼를 어떻게 해결할 수 있는지에 대한 간단한 설명을 제공하세요."
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
  messages: [
    {
      "role": "user",
      "content": [
        {
          "type": "text",
          "text": "영리한 수수께끼를 생성하고 사용자가 올바른 해답에 도달할 수 있도록 단계별 가이드를 제공하세요. 수수께끼는 도전적이지만 논리적 사고와 세심한 주의를 통해 해결 가능해야 합니다. 각 수수께끼를 제시한 후, 사용자를 점진적으로 답으로 이끄는 힌트나 질문들을 제공하세요. 힌트는 너무 명백하지 않으면서도 사용자의 사고 과정을 안내하기에 충분한 정보를 제공해야 합니다. 마지막으로 해답을 공개하고 주어진 힌트를 사용하여 수수께끼를 어떻게 해결할 수 있는지에 대한 간단한 설명을 제공하세요."
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
    messages=[
        {
            "role": "user",
            "content": [
                {
                    "type": "text",
                    "text": "영리한 수수께끼를 생성하고 사용자가 올바른 해답에 도달할 수 있도록 단계별 가이드를 제공하세요. 수수께끼는 도전적이지만 논리적 사고와 세심한 주의를 통해 해결 가능해야 합니다. 각 수수께끼를 제시한 후, 사용자를 점진적으로 답으로 이끄는 힌트나 질문들을 제공하세요. 힌트는 너무 명백하지 않으면서도 사용자의 사고 과정을 안내하기에 충분한 정보를 제공해야 합니다. 마지막으로 해답을 공개하고 주어진 힌트를 사용하여 수수께끼를 어떻게 해결할 수 있는지에 대한 간단한 설명을 제공하세요."
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
  messages: [
    {
      "role": "user",
      "content": [
        {
          "type": "text",
          "text": "영리한 수수께끼를 생성하고 사용자가 올바른 해답에 도달할 수 있도록 단계별 가이드를 제공하세요. 수수께끼는 도전적이지만 논리적 사고와 세심한 주의를 통해 해결 가능해야 합니다. 각 수수께끼를 제시한 후, 사용자를 점진적으로 답으로 이끄는 힌트나 질문들을 제공하세요. 힌트는 너무 명백하지 않으면서도 사용자의 사고 과정을 안내하기에 충분한 정보를 제공해야 합니다. 마지막으로 해답을 공개하고 주어진 힌트를 사용하여 수수께끼를 어떻게 해결할 수 있는지에 대한 간단한 설명을 제공하세요."
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
    messages=[
        {
            "role": "user",
            "content": [
                {
                    "type": "text",
                    "text": "영리한 수수께끼를 생성하고 사용자가 올바른 해답에 도달할 수 있도록 단계별 가이드를 제공하세요. 수수께끼는 도전적이지만 논리적 사고와 세심한 주의를 통해 해결 가능해야 합니다. 각 수수께끼를 제시한 후, 사용자를 점진적으로 답으로 이끄는 힌트나 질문들을 제공하세요. 힌트는 너무 명백하지 않으면서도 사용자의 사고 과정을 안내하기에 충분한 정보를 제공해야 합니다. 마지막으로 해답을 공개하고 주어진 힌트를 사용하여 수수께끼를 어떻게 해결할 수 있는지에 대한 간단한 설명을 제공하세요."
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
  temperature: 0,
  messages: [
    {
      "role": "user",
      "content": [
        {
          "type": "text",
          "text": "영리한 수수께끼를 생성하고 사용자가 올바른 해답에 도달할 수 있도록 단계별 가이드를 제공하세요. 수수께끼는 도전적이지만 논리적 사고와 세심한 주의를 통해 해결 가능해야 합니다. 각 수수께끼를 제시한 후, 사용자를 점진적으로 답으로 이끄는 힌트나 질문들을 제공하세요. 힌트는 너무 명백하지 않으면서도 사용자의 사고 과정을 안내하기에 충분한 정보를 제공해야 합니다. 마지막으로 해답을 공개하고 주어진 힌트를 사용하여 수수께끼를 어떻게 해결할 수 있는지에 대한 간단한 설명을 제공하세요."
        }
      ]
    }
  ]
});
console.log(msg);

```

</Tab>
</Tabs>