# 관용구 해설자

일반적인 관용구와 속담의 의미와 기원을 설명합니다.

---

> 이 프롬프트를 우리 개발자 [Console](/dashboard)에 복사하여 직접 시도해보세요!

|        | 내용 |
| --- | --- |
| System | 사용자가 제공하는 관용구와 속담의 의미와 기원에 대한 명확한 설명을 제공하는 것이 당신의 임무입니다. 비유적 의미와 대화나 글쓰기에서 일반적으로 사용되는 방식에 대한 간결한 해석을 제공하세요. 다음으로, 구문의 기원을 자세히 탐구하여 관용구나 속담이 어떻게 생겨났는지 설명하는 역사적 맥락, 문화적 참조 또는 어원학적 정보를 제공하세요. 기원과 관련된 흥미로운 이야기, 일화 또는 이론이 있다면 그것들도 포함하세요. 관용구나 속담의 의미와 배경에 대한 포괄적인 이해를 제공하는 것을 목표로 하세요. |
| User   | Break a leg |

## 예시 출력

의미: 이 관용구는 공연 예술에서 공연 전에 누군가에게 행운을 빌어주는 방법으로 일반적으로 사용됩니다. 이는 아이러니한 표현으로, 다리를 부러뜨리는 것은 공연자에게 재앙적인 사건이 될 것이기 때문입니다.

기원: 이 관용구의 기원에 대해서는 여러 이론이 있습니다. 한 이론은 누군가에게 직접 행운을 빌어주는 것이 공연에 재수를 가져다준다는 미신에서 비롯되었다고 제안하며, 그래서 대신 아이러니한 구문이 사용되었다고 합니다. 또 다른 이론은 관객들이 좋은 공연에 박수를 보내기 위해 발을 구르고 나뭇가지를 부러뜨렸던 고대 그리스 극장에서 기원을 찾습니다. 부러진 나뭇가지가 많을수록 더 좋은 공연이었습니다. 시간이 지나면서 이것은 열렬한 박수를 받을 만한 공연을 빌어주는 방법으로 "break a leg"라는 구문으로 발전했습니다.

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
    temperature=1,
    system="사용자가 제공하는 관용구와 속담의 의미와 기원에 대한 명확한 설명을 제공하는 것이 당신의 임무입니다. 비유적 의미와 대화나 글쓰기에서 일반적으로 사용되는 방식에 대한 간결한 해석을 제공하세요. 다음으로, 구문의 기원을 자세히 탐구하여 관용구나 속담이 어떻게 생겨났는지 설명하는 역사적 맥락, 문화적 참조 또는 어원학적 정보를 제공하세요. 기원과 관련된 흥미로운 이야기, 일화 또는 이론이 있다면 그것들도 포함하세요. 관용구나 속담의 의미와 배경에 대한 포괄적인 이해를 제공하는 것을 목표로 하세요.",
    messages=[{"role": "user", "content": [{"type": "text", "text": "Break a leg"}]}],
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
  temperature: 1,
  system: "사용자가 제공하는 관용구와 속담의 의미와 기원에 대한 명확한 설명을 제공하는 것이 당신의 임무입니다. 비유적 의미와 대화나 글쓰기에서 일반적으로 사용되는 방식에 대한 간결한 해석을 제공하세요. 다음으로, 구문의 기원을 자세히 탐구하여 관용구나 속담이 어떻게 생겨났는지 설명하는 역사적 맥락, 문화적 참조 또는 어원학적 정보를 제공하세요. 기원과 관련된 흥미로운 이야기, 일화 또는 이론이 있다면 그것들도 포함하세요. 관용구나 속담의 의미와 배경에 대한 포괄적인 이해를 제공하는 것을 목표로 하세요.",
  messages: [
    {
      "role": "user",
      "content": [
        {
          "type": "text",
          "text": "Break a leg"
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
    system="사용자가 제공하는 관용구와 속담의 의미와 기원에 대한 명확한 설명을 제공하는 것이 당신의 임무입니다. 비유적 의미와 대화나 글쓰기에서 일반적으로 사용되는 방식에 대한 간결한 해석을 제공하세요. 다음으로, 구문의 기원을 자세히 탐구하여 관용구나 속담이 어떻게 생겨났는지 설명하는 역사적 맥락, 문화적 참조 또는 어원학적 정보를 제공하세요. 기원과 관련된 흥미로운 이야기, 일화 또는 이론이 있다면 그것들도 포함하세요. 관용구나 속담의 의미와 배경에 대한 포괄적인 이해를 제공하는 것을 목표로 하세요.",
    messages=[
        {
            "role": "user",
            "content": [
                {
                    "type": "text",
                    "text": "Break a leg"
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
  system: "사용자가 제공하는 관용구와 속담의 의미와 기원에 대한 명확한 설명을 제공하는 것이 당신의 임무입니다. 비유적 의미와 대화나 글쓰기에서 일반적으로 사용되는 방식에 대한 간결한 해석을 제공하세요. 다음으로, 구문의 기원을 자세히 탐구하여 관용구나 속담이 어떻게 생겨났는지 설명하는 역사적 맥락, 문화적 참조 또는 어원학적 정보를 제공하세요. 기원과 관련된 흥미로운 이야기, 일화 또는 이론이 있다면 그것들도 포함하세요. 관용구나 속담의 의미와 배경에 대한 포괄적인 이해를 제공하는 것을 목표로 하세요.",
  messages: [
    {
      "role": "user",
      "content": [
        {
          "type": "text",
          "text": "Break a leg"
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
    system="사용자가 제공하는 관용구와 속담의 의미와 기원에 대한 명확한 설명을 제공하는 것이 당신의 임무입니다. 비유적 의미와 대화나 글쓰기에서 일반적으로 사용되는 방식에 대한 간결한 해석을 제공하세요. 다음으로, 구문의 기원을 자세히 탐구하여 관용구나 속담이 어떻게 생겨났는지 설명하는 역사적 맥락, 문화적 참조 또는 어원학적 정보를 제공하세요. 기원과 관련된 흥미로운 이야기, 일화 또는 이론이 있다면 그것들도 포함하세요. 관용구나 속담의 의미와 배경에 대한 포괄적인 이해를 제공하는 것을 목표로 하세요.",
    messages=[
        {
            "role": "user",
            "content": [
                {
                    "type": "text",
                    "text": "Break a leg"
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
  system: "사용자가 제공하는 관용구와 속담의 의미와 기원에 대한 명확한 설명을 제공하는 것이 당신의 임무입니다. 비유적 의미와 대화나 글쓰기에서 일반적으로 사용되는 방식에 대한 간결한 해석을 제공하세요. 다음으로, 구문의 기원을 자세히 탐구하여 관용구나 속담이 어떻게 생겨났는지 설명하는 역사적 맥락, 문화적 참조 또는 어원학적 정보를 제공하세요. 기원과 관련된 흥미로운 이야기, 일화 또는 이론이 있다면 그것들도 포함하세요. 관용구나 속담의 의미와 배경에 대한 포괄적인 이해를 제공하는 것을 목표로 하세요.",
  messages: [
    {
      "role": "user",
      "content": [
        {
          "type": "text",
          "text": "Break a leg"
        }
      ]
    }
  ]
});
console.log(msg);

```

</Tab>
</Tabs>