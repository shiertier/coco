# 이모지 인코더

평범한 텍스트를 재미있고 표현력 있는 이모지 메시지로 변환합니다.

---

> 이 프롬프트를 우리 개발자 [Console](/dashboard)에 복사해서 직접 시도해보세요!

|        | Content |
| --- | --- |
| System | 당신의 임무는 제공된 평문 메시지를 받아서 동일한 의미와 의도를 전달하는 표현력 있고 이모지가 풍부한 메시지로 변환하는 것입니다. 적절한 곳에서 핵심 단어와 구문을 관련 이모지로 대체하여 시각적 흥미와 감정을 추가하세요. 이모지를 창의적으로 사용하되 메시지가 명확하고 이해하기 쉽게 유지되도록 하세요. 핵심 메시지를 변경하거나 새로운 정보를 추가하지 마세요. |
| User   | All the world's a stage, and all the men and women merely players. They have their exits and their entrances; And one man in his time plays many parts. |

## 예시 출력

All the 🌍's a 🎭, and all the 👨 and 👩 merely 🎭🎬. They have their 🚪🚶‍♂️ and their 🚶‍♀️🚪; And one 👨 in his ⌛ plays many 🎭.

---

## API 요청

<Tabs>
<Tab title="Python">
```python
import anthropic

client = anthropic.Anthropic(  # defaults to os.environ.get("ANTHROPIC_API_KEY")
    api_key="my_api_key",
)
message = client.messages.create(
    model="claude-sonnet-4-5",
    max_tokens=1000,
    temperature=0,
    system="당신의 임무는 제공된 평문 메시지를 받아서 동일한 의미와 의도를 전달하는 표현력 있고 이모지가 풍부한 메시지로 변환하는 것입니다. 적절한 곳에서 핵심 단어와 구문을 관련 이모지로 대체하여 시각적 흥미와 감정을 추가하세요. 이모지를 창의적으로 사용하되 메시지가 명확하고 이해하기 쉽게 유지되도록 하세요. 핵심 메시지를 변경하거나 새로운 정보를 추가하지 마세요.",
    messages=[
        {
            "role": "user",
            "content": [
                {
                    "type": "text",
                    "text": "All the world's a stage, and all the men and women merely players. They have their exits and their entrances; And one man in his time plays many parts.",
                }
            ],
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
  max_tokens: 1000,
  temperature: 0,
  system: "당신의 임무는 제공된 평문 메시지를 받아서 동일한 의미와 의도를 전달하는 표현력 있고 이모지가 풍부한 메시지로 변환하는 것입니다. 적절한 곳에서 핵심 단어와 구문을 관련 이모지로 대체하여 시각적 흥미와 감정을 추가하세요. 이모지를 창의적으로 사용하되 메시지가 명확하고 이해하기 쉽게 유지되도록 하세요. 핵심 메시지를 변경하거나 새로운 정보를 추가하지 마세요.",
  messages: [
    {
      "role": "user",
      "content": [
        {
          "type": "text",
          "text": "All the world's a stage, and all the men and women merely players. They have their exits and their entrances; And one man in his time plays many parts."
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
    system="당신의 임무는 제공된 평문 메시지를 받아서 동일한 의미와 의도를 전달하는 표현력 있고 이모지가 풍부한 메시지로 변환하는 것입니다. 적절한 곳에서 핵심 단어와 구문을 관련 이모지로 대체하여 시각적 흥미와 감정을 추가하세요. 이모지를 창의적으로 사용하되 메시지가 명확하고 이해하기 쉽게 유지되도록 하세요. 핵심 메시지를 변경하거나 새로운 정보를 추가하지 마세요.",
    messages=[
        {
            "role": "user",
            "content": [
                {
                    "type": "text",
                    "text": "All the world's a stage, and all the men and women merely players. They have their exits and their entrances; And one man in his time plays many parts."
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
  system: "당신의 임무는 제공된 평문 메시지를 받아서 동일한 의미와 의도를 전달하는 표현력 있고 이모지가 풍부한 메시지로 변환하는 것입니다. 적절한 곳에서 핵심 단어와 구문을 관련 이모지로 대체하여 시각적 흥미와 감정을 추가하세요. 이모지를 창의적으로 사용하되 메시지가 명확하고 이해하기 쉽게 유지되도록 하세요. 핵심 메시지를 변경하거나 새로운 정보를 추가하지 마세요.",
  messages: [
    {
      "role": "user",
      "content": [
        {
          "type": "text",
          "text": "All the world's a stage, and all the men and women merely players. They have their exits and their entrances; And one man in his time plays many parts."
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
    system="당신의 임무는 제공된 평문 메시지를 받아서 동일한 의미와 의도를 전달하는 표현력 있고 이모지가 풍부한 메시지로 변환하는 것입니다. 적절한 곳에서 핵심 단어와 구문을 관련 이모지로 대체하여 시각적 흥미와 감정을 추가하세요. 이모지를 창의적으로 사용하되 메시지가 명확하고 이해하기 쉽게 유지되도록 하세요. 핵심 메시지를 변경하거나 새로운 정보를 추가하지 마세요.",
    messages=[
        {
            "role": "user",
            "content": [
                {
                    "type": "text",
                    "text": "All the world's a stage, and all the men and women merely players. They have their exits and their entrances; And one man in his time plays many parts."
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
  system: "당신의 임무는 제공된 평문 메시지를 받아서 동일한 의미와 의도를 전달하는 표현력 있고 이모지가 풍부한 메시지로 변환하는 것입니다. 적절한 곳에서 핵심 단어와 구문을 관련 이모지로 대체하여 시각적 흥미와 감정을 추가하세요. 이모지를 창의적으로 사용하되 메시지가 명확하고 이해하기 쉽게 유지되도록 하세요. 핵심 메시지를 변경하거나 새로운 정보를 추가하지 마세요.",
  messages: [
    {
      "role": "user",
      "content": [
        {
          "type": "text",
          "text": "All the world's a stage, and all the men and women merely players. They have their exits and their entrances; And one man in his time plays many parts."
        }
      ]
    }
  ]
});
console.log(msg);

```

</Tab>
</Tabs>