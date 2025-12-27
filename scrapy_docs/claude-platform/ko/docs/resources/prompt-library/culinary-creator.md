# 요리 창작자

사용자의 사용 가능한 재료와 식단 선호도를 바탕으로 레시피 아이디어를 제안합니다.

---

> 이 프롬프트를 우리 개발자 [Console](/dashboard)에 복사하여 직접 시도해보세요!

|        | 내용 |
| --- | --- |
| System | 당신의 임무는 사용자가 입력한 사용 가능한 재료와 식단 선호도를 바탕으로 개인화된 레시피 아이디어를 생성하는 것입니다. 이 정보를 사용하여 주어진 재료로 만들 수 있으면서 사용자의 식단 요구사항(언급된 경우)을 수용하는 다양하고 창의적이며 맛있는 레시피를 제안하세요. 각 레시피에 대해 간단한 설명, 필요한 재료 목록, 그리고 간단한 조리법을 제공하세요. 레시피가 따라하기 쉽고, 영양가 있으며, 최소한의 추가 재료나 장비로 준비할 수 있도록 하세요. |
| User   | 사용 가능한 재료: 닭가슴살, 브로콜리, 당근, 양파, 마늘, 올리브오일, 쌀, 글루텐프리 간장, 꿀 <br/> 식단 선호도: 글루텐프리 |

## 예시 출력

제안 레시피:

허니 갈릭 치킨 볶음

설명: 부드러운 닭가슴살, 아삭한 채소, 그리고 달콤하고 짭짤한 허니 갈릭 소스가 특징인 빠르고 풍미 있는 볶음 요리입니다.  
재료:

- 닭가슴살 2개, 한 입 크기로 자른 것
- 브로콜리 꽃송이 2컵
- 당근 슬라이스 1컵
- 양파 1개, 다진 것
- 마늘 3쪽, 다진 것
- 올리브오일 2큰술
- 간장 1/4컵 (글루텐프리)
- 꿀 2큰술

조리법:

1.  큰 팬이나 웍에 올리브오일을 중강불로 가열합니다.
2.  다진 양파와 다진 마늘을 넣고 향이 날 때까지 약 2분간 볶습니다.
3.  닭고기 조각을 넣고 더 이상 분홍색이 아닐 때까지 약 5-7분간 조리합니다.
4.  브로콜리와 당근을 넣고 채소가 아삭하게 익을 때까지 약 5분간 볶습니다.
5.  작은 그릇에 간장과 꿀을 함께 휘젓습니다.
6.  소스를 닭고기와 채소 위에 부어 고르게 코팅되도록 저어줍니다.
7.  소스가 약간 걸쭉해질 때까지 추가로 2-3분간 조리합니다.
8.  찐 쌀과 함께 뜨겁게 서빙합니다.

치킨 앤 베지 라이스 볼

설명: 양념한 닭고기, 구운 채소, 그리고 간장을 뿌린 든든하고 영양가 있는 라이스 볼입니다.

재료:

- 닭가슴살 2개, 한 입 크기로 자른 것
- 브로콜리 꽃송이 2컵
- 당근 슬라이스 1컵
- 양파 1개, 다진 것
- 마늘 2쪽, 다진 것
- 올리브오일 2큰술
- 간장 1/4컵 (글루텐프리)
- 조리된 쌀 2컵

조리법:

1.  오븐을 400°F (200°C)로 예열합니다.
2.  큰 그릇에 닭고기 조각, 브로콜리, 당근, 양파, 마늘을 넣습니다.
3.  올리브오일과 간장의 절반을 뿌리고 고르게 코팅되도록 버무립니다.
4.  닭고기와 채소를 베이킹 시트에 펼치고 예열된 오븐에서 닭고기가 완전히 익고 채소가 부드러워질 때까지 20-25분간 굽습니다.
5.  조리된 쌀을 서빙 볼에 나누어 담습니다.
6.  각 볼 위에 구운 닭고기와 채소를 올립니다.
7.  각 볼 위에 남은 간장을 뿌립니다.
8.  뜨겁게 서빙하고 즐기세요!

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
  system="당신의 임무는 사용자가 입력한 사용 가능한 재료와 식단 선호도를 바탕으로 개인화된 레시피 아이디어를 생성하는 것입니다. 이 정보를 사용하여 주어진 재료로 만들 수 있으면서 사용자의 식단 요구사항(언급된 경우)을 수용하는 다양하고 창의적이며 맛있는 레시피를 제안하세요. 각 레시피에 대해 간단한 설명, 필요한 재료 목록, 그리고 간단한 조리법을 제공하세요. 레시피가 따라하기 쉽고, 영양가 있으며, 최소한의 추가 재료나 장비로 준비할 수 있도록 하세요.",
  messages=[
    {
    "role": "user",
    "content": [
        {
          "type": "text",
          "text": "사용 가능한 재료: 닭가슴살, 브로콜리, 당근, 양파, 마늘, 올리브오일, 쌀, 글루텐프리 간장, 꿀 \n식단 선호도: 글루텐프리"
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
  system: "당신의 임무는 사용자가 입력한 사용 가능한 재료와 식단 선호도를 바탕으로 개인화된 레시피 아이디어를 생성하는 것입니다. 이 정보를 사용하여 주어진 재료로 만들 수 있으면서 사용자의 식단 요구사항(언급된 경우)을 수용하는 다양하고 창의적이며 맛있는 레시피를 제안하세요. 각 레시피에 대해 간단한 설명, 필요한 재료 목록, 그리고 간단한 조리법을 제공하세요. 레시피가 따라하기 쉽고, 영양가 있으며, 최소한의 추가 재료나 장비로 준비할 수 있도록 하세요.",
  messages: [
    {
      "role": "user",
      "content": [
        {
          "type": "text",
          "text": "사용 가능한 재료: 닭가슴살, 브로콜리, 당근, 양파, 마늘, 올리브오일, 쌀, 글루텐프리 간장, 꿀  \n식단 선호도: 글루텐프리"
        }
      ]
    }
  ]
});
console.log(msg);

````

</Tab>
<Tab title="AWS Bedrock Python">

```
from anthropic import AnthropicBedrock

# See https://docs.claude.com/claude/reference/claude-on-amazon-bedrock
# for authentication options
client = AnthropicBedrock()

message = client.messages.create(
    model="anthropic.claude-sonnet-4-5-20250929-v1:0",
    max_tokens=2000,
    temperature=0.5,
    system="당신의 임무는 사용자가 입력한 사용 가능한 재료와 식단 선호도를 바탕으로 개인화된 레시피 아이디어를 생성하는 것입니다. 이 정보를 사용하여 주어진 재료로 만들 수 있으면서 사용자의 식단 요구사항(언급된 경우)을 수용하는 다양하고 창의적이며 맛있는 레시피를 제안하세요. 각 레시피에 대해 간단한 설명, 필요한 재료 목록, 그리고 간단한 조리법을 제공하세요. 레시피가 따라하기 쉽고, 영양가 있으며, 최소한의 추가 재료나 장비로 준비할 수 있도록 하세요.",
    messages=[
        {
            "role": "user",
            "content": [
                {
                    "type": "text",
                    "text": "사용 가능한 재료: 닭가슴살, 브로콜리, 당근, 양파, 마늘, 올리브오일, 쌀, 글루텐프리 간장, 꿀  \n식단 선호도: 글루텐프리"
                }
            ]
        }
    ]
)
print(message.content)

```

</Tab>
<Tab title="AWS Bedrock TypeScript">

```
import AnthropicBedrock from "@anthropic-ai/bedrock-sdk";

// See https://docs.claude.com/claude/reference/claude-on-amazon-bedrock
// for authentication options
const client = new AnthropicBedrock();

const msg = await client.messages.create({
  model: "anthropic.claude-sonnet-4-5-20250929-v1:0",
  max_tokens: 2000,
  temperature: 0.5,
  system: "당신의 임무는 사용자가 입력한 사용 가능한 재료와 식단 선호도를 바탕으로 개인화된 레시피 아이디어를 생성하는 것입니다. 이 정보를 사용하여 주어진 재료로 만들 수 있으면서 사용자의 식단 요구사항(언급된 경우)을 수용하는 다양하고 창의적이며 맛있는 레시피를 제안하세요. 각 레시피에 대해 간단한 설명, 필요한 재료 목록, 그리고 간단한 조리법을 제공하세요. 레시피가 따라하기 쉽고, 영양가 있으며, 최소한의 추가 재료나 장비로 준비할 수 있도록 하세요.",
  messages: [
    {
      "role": "user",
      "content": [
        {
          "type": "text",
          "text": "사용 가능한 재료: 닭가슴살, 브로콜리, 당근, 양파, 마늘, 올리브오일, 쌀, 글루텐프리 간장, 꿀  \n식단 선호도: 글루텐프리"
        }
      ]
    }
  ]
});
console.log(msg);

```

</Tab>
<Tab title="Vertex AI Python">

```
from anthropic import AnthropicVertex

client = AnthropicVertex()

message = client.messages.create(
    model="claude-sonnet-4@20250514",
    max_tokens=2000,
    temperature=0.5,
    system="당신의 임무는 사용자가 입력한 사용 가능한 재료와 식단 선호도를 바탕으로 개인화된 레시피 아이디어를 생성하는 것입니다. 이 정보를 사용하여 주어진 재료로 만들 수 있으면서 사용자의 식단 요구사항(언급된 경우)을 수용하는 다양하고 창의적이며 맛있는 레시피를 제안하세요. 각 레시피에 대해 간단한 설명, 필요한 재료 목록, 그리고 간단한 조리법을 제공하세요. 레시피가 따라하기 쉽고, 영양가 있으며, 최소한의 추가 재료나 장비로 준비할 수 있도록 하세요.",
    messages=[
        {
            "role": "user",
            "content": [
                {
                    "type": "text",
                    "text": "사용 가능한 재료: 닭가슴살, 브로콜리, 당근, 양파, 마늘, 올리브오일, 쌀, 글루텐프리 간장, 꿀  \n식단 선호도: 글루텐프리"
                }
            ]
        }
    ]
)
print(message.content)

```

</Tab>
<Tab title="Vertex AI TypeScript">

```
import { AnthropicVertex } from '@anthropic-ai/vertex-sdk';

// Reads from the `CLOUD_ML_REGION` & `ANTHROPIC_VERTEX_PROJECT_ID` environment variables.
// Additionally goes through the standard `google-auth-library` flow.
const client = new AnthropicVertex();

const msg = await client.messages.create({
  model: "claude-sonnet-4@20250514",
  max_tokens: 2000,
  temperature: 0.5,
  system: "당신의 임무는 사용자가 입력한 사용 가능한 재료와 식단 선호도를 바탕으로 개인화된 레시피 아이디어를 생성하는 것입니다. 이 정보를 사용하여 주어진 재료로 만들 수 있으면서 사용자의 식단 요구사항(언급된 경우)을 수용하는 다양하고 창의적이며 맛있는 레시피를 제안하세요. 각 레시피에 대해 간단한 설명, 필요한 재료 목록, 그리고 간단한 조리법을 제공하세요. 레시피가 따라하기 쉽고, 영양가 있으며, 최소한의 추가 재료나 장비로 준비할 수 있도록 하세요.",
  messages: [
    {
      "role": "user",
      "content": [
        {
          "type": "text",
          "text": "사용 가능한 재료: 닭가슴살, 브로콜리, 당근, 양파, 마늘, 올리브오일, 쌀, 글루텐프리 간장, 꿀  \n식단 선호도: 글루텐프리"
        }
      ]
    }
  ]
});
console.log(msg);

```

</Tab>
</Tabs>