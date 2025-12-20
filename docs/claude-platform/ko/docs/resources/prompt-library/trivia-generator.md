# 퀴즈 생성기

다양한 주제에 대한 퀴즈 문제를 생성하고 필요할 때 힌트를 제공합니다.

---

> 이 프롬프트를 우리의 개발자 [Console](/dashboard)에 복사해서 직접 시도해보세요!

| 역할 | 내용 |
| ---- | --- |
| 사용자 | 다양한 주제에 대한 퀴즈 문제를 생성하고 사용자가 정답에 도달할 수 있도록 힌트를 제공하세요. 다양한 카테고리에서 선택하여 사용자의 지식이나 추론 능력을 테스트하는 문제를 만드세요. 사용자를 해답으로 안내하는 점점 더 구체적인 힌트 시리즈를 제공하세요. 문제가 도전적이고 힌트가 답을 너무 쉽게 알려주지 않으면서도 사용자에게 도움이 될 충분한 정보를 제공하도록 하세요. |

## 예시 출력

카테고리: 지리

문제:  
 프랑스와 스페인 모두와 국경을 접하는 세계 유일의 국가는 어디인가요?

힌트 1: 이 국가는 프랑스와 스페인 사이의 피레네 산맥에 위치해 있습니다.  
 힌트 2: 아름다운 산악 풍경, 스키 리조트, 면세 쇼핑으로 유명합니다.  
 힌트 3: 이 국가의 수도 이름에는 국가명 자체가 포함되어 있습니다.

답: 안도라

카테고리: 과학

문제:  
 식물이 햇빛, 물, 이산화탄소를 에너지와 산소로 변환하는 과정을 무엇이라고 하나요?

힌트 1: 이 과정은 음식과 산소를 모두 제공하므로 지구상의 생명에 필수적입니다.  
 힌트 2: 주로 식물의 잎에서 일어납니다.  
 힌트 3: 이 과정에는 햇빛을 흡수하는 녹색 색소인 엽록소가 관여합니다.

답: 광합성

카테고리: 문학

문제:  
 1813년에 처음 출간된 고전 소설 "오만과 편견"을 쓴 작가는 누구인가요?

힌트 1: 이 작가는 1775년부터 1817년까지 살았던 영국 여성입니다.  
 힌트 2: 영국 사회와 예의범절에 대한 재치 있고 통찰력 있는 묘사로 유명합니다.  
 힌트 3: 그녀의 다른 유명한 작품으로는 "이성과 감성"과 "엠마"가 있습니다.

답: 제인 오스틴

카테고리: 역사

문제:  
 동독과 서독을 나누었던 베를린 장벽이 철거되기 시작한 해는 언제인가요?

힌트 1: 이 사건은 20세기 후반에 일어났습니다.  
 힌트 2: 냉전 종료의 중요한 순간을 의미했습니다.  
 힌트 3: 이 해는 종종 유럽에서 "공산주의의 몰락"이라는 구절과 연관됩니다.

답: 1989년

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
  max_tokens=2000,
  temperature=0.5,
  messages=[
    {
    "role": "user",
    "content": [
        {
          "type": "text",
          "text": "다양한 주제에 대한 퀴즈 문제를 생성하고 사용자가 정답에 도달할 수 있도록 힌트를 제공하세요. 다양한 카테고리에서 선택하여 사용자의 지식이나 추론 능력을 테스트하는 문제를 만드세요. 사용자를 해답으로 안내하는 점점 더 구체적인 힌트 시리즈를 제공하세요. 문제가 도전적이고 힌트가 답을 너무 쉽게 알려주지 않으면서도 사용자에게 도움이 될 충분한 정보를 제공하도록 하세요."
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
          "text": "다양한 주제에 대한 퀴즈 문제를 생성하고 사용자가 정답에 도달할 수 있도록 힌트를 제공하세요. 다양한 카테고리에서 선택하여 사용자의 지식이나 추론 능력을 테스트하는 문제를 만드세요. 사용자를 해답으로 안내하는 점점 더 구체적인 힌트 시리즈를 제공하세요. 문제가 도전적이고 힌트가 답을 너무 쉽게 알려주지 않으면서도 사용자에게 도움이 될 충분한 정보를 제공하도록 하세요."
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
"text": "다양한 주제에 대한 퀴즈 문제를 생성하고 사용자가 정답에 도달할 수 있도록 힌트를 제공하세요. 다양한 카테고리에서 선택하여 사용자의 지식이나 추론 능력을 테스트하는 문제를 만드세요. 사용자를 해답으로 안내하는 점점 더 구체적인 힌트 시리즈를 제공하세요. 문제가 도전적이고 힌트가 답을 너무 쉽게 알려주지 않으면서도 사용자에게 도움이 될 충분한 정보를 제공하도록 하세요."
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
          "text": "다양한 주제에 대한 퀴즈 문제를 생성하고 사용자가 정답에 도달할 수 있도록 힌트를 제공하세요. 다양한 카테고리에서 선택하여 사용자의 지식이나 추론 능력을 테스트하는 문제를 만드세요. 사용자를 해답으로 안내하는 점점 더 구체적인 힌트 시리즈를 제공하세요. 문제가 도전적이고 힌트가 답을 너무 쉽게 알려주지 않으면서도 사용자에게 도움이 될 충분한 정보를 제공하도록 하세요."
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
                    "text": "다양한 주제에 대한 퀴즈 문제를 생성하고 사용자가 정답에 도달할 수 있도록 힌트를 제공하세요. 다양한 카테고리에서 선택하여 사용자의 지식이나 추론 능력을 테스트하는 문제를 만드세요. 사용자를 해답으로 안내하는 점점 더 구체적인 힌트 시리즈를 제공하세요. 문제가 도전적이고 힌트가 답을 너무 쉽게 알려주지 않으면서도 사용자에게 도움이 될 충분한 정보를 제공하도록 하세요."
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
          "text": "다양한 주제에 대한 퀴즈 문제를 생성하고 사용자가 정답에 도달할 수 있도록 힌트를 제공하세요. 다양한 카테고리에서 선택하여 사용자의 지식이나 추론 능력을 테스트하는 문제를 만드세요. 사용자를 해답으로 안내하는 점점 더 구체적인 힌트 시리즈를 제공하세요. 문제가 도전적이고 힌트가 답을 너무 쉽게 알려주지 않으면서도 사용자에게 도움이 될 충분한 정보를 제공하도록 하세요."
        }
      ]
    }
  ]
});
console.log(msg);

```

</Tab>
</Tabs>