# 면접 질문 작성자

면접을 위한 질문을 생성합니다.

---

> 이 프롬프트를 우리의 개발자 [Console](/dashboard)에 복사해서 직접 시도해보세요!

|        | 내용 |
| --- | --- |
| System | 주어진 맥락을 바탕으로 면접을 위한 사려 깊고 개방형 질문들을 생성하는 것이 당신의 과제입니다. 질문들은 면접자로부터 통찰력 있고 상세한 답변을 이끌어내도록 설계되어야 하며, 그들이 자신의 지식, 경험, 비판적 사고 능력을 보여줄 수 있도록 해야 합니다. 예/아니오 질문이나 뻔한 답변이 있는 질문은 피하세요. 대신 성찰, 자기 평가, 구체적인 예시나 일화 공유를 장려하는 질문에 집중하세요. |
| User   | 당신은 잘 확립된 전자상거래 회사의 마케팅 매니저 직책에 대한 후보자를 면접하고 있습니다. 이상적인 후보자는 다채널 마케팅 캠페인을 개발하고 실행한 경험, 강력한 분석 능력, 그리고 교차 기능 팀과 효과적으로 협업할 수 있는 능력을 갖추어야 합니다. 또한 최신 마케팅 트렌드와 기술에 대해 최신 정보를 유지하는 것에 열정적이어야 합니다. |

## 예시 출력

1.  이전 역할에서 개발하고 실행한 성공적인 다채널 마케팅 캠페인에 대해 설명해 주실 수 있나요? 어떤 도전에 직면했고, 어떻게 극복했나요?
2.  마케팅 캠페인의 성공을 어떻게 측정하며, 어떤 지표에 집중하나요? 데이터와 분석을 사용하여 캠페인의 성과를 최적화한 예시를 제공해 주실 수 있나요?
3.  당신의 경험에서 성공적인 전자상거래 마케팅 전략의 핵심 요소는 무엇인가요? 당신의 전략이 회사의 전반적인 비즈니스 목표와 일치하도록 어떻게 보장하나요?
4.  마케팅 목표를 달성하기 위해 교차 기능 팀(예: 제품, 영업 또는 고객 서비스)과 협업해야 했던 때의 예시를 공유해 주실 수 있나요? 어떤 도전에 직면했고, 효과적인 협업을 어떻게 촉진했나요?
5.  최신 마케팅 트렌드와 기술에 대해 어떻게 정보를 얻고 있으며, 이 지식을 마케팅 전략이나 캠페인 개선에 어떻게 적용했나요?
6.  참여했던 마케팅 캠페인이 예상대로 성과를 내지 못했던 때를 설명해 주실 수 있나요? 그 경험에서 무엇을 배웠고, 그 교훈을 향후 캠페인에 어떻게 적용했나요?
7.  우리 회사는 혁신과 지속적인 개선을 중요하게 생각합니다. 팀의 성과나 결과에 상당한 영향을 미친 새로운 마케팅 접근법, 도구 또는 기술을 도입했던 때의 예시를 공유해 주실 수 있나요?

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
  temperature=0.5,
  system="주어진 맥락을 바탕으로 면접을 위한 사려 깊고 개방형 질문들을 생성하는 것이 당신의 과제입니다. 질문들은 면접자로부터 통찰력 있고 상세한 답변을 이끌어내도록 설계되어야 하며, 그들이 자신의 지식, 경험, 비판적 사고 능력을 보여줄 수 있도록 해야 합니다. 예/아니오 질문이나 뻔한 답변이 있는 질문은 피하세요. 대신 성찰, 자기 평가, 구체적인 예시나 일화 공유를 장려하는 질문에 집중하세요.",
  messages=[
    {
      "role": "user",
      "content": [
        {
          "type": "text",
          "text": "당신은 잘 확립된 전자상거래 회사의 마케팅 매니저 직책에 대한 후보자를 면접하고 있습니다. 이상적인 후보자는 다채널 마케팅 캠페인을 개발하고 실행한 경험, 강력한 분석 능력, 그리고 교차 기능 팀과 효과적으로 협업할 수 있는 능력을 갖추어야 합니다. 또한 최신 마케팅 트렌드와 기술에 대해 최신 정보를 유지하는 것에 열정적이어야 합니다."
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
  temperature: 0.5,
  system: "주어진 맥락을 바탕으로 면접을 위한 사려 깊고 개방형 질문들을 생성하는 것이 당신의 과제입니다. 질문들은 면접자로부터 통찰력 있고 상세한 답변을 이끌어내도록 설계되어야 하며, 그들이 자신의 지식, 경험, 비판적 사고 능력을 보여줄 수 있도록 해야 합니다. 예/아니오 질문이나 뻔한 답변이 있는 질문은 피하세요. 대신 성찰, 자기 평가, 구체적인 예시나 일화 공유를 장려하는 질문에 집중하세요.",
  messages: [
    {
      "role": "user",
      "content": [
        {
          "type": "text",
          "text": "당신은 잘 확립된 전자상거래 회사의 마케팅 매니저 직책에 대한 후보자를 면접하고 있습니다. 이상적인 후보자는 다채널 마케팅 캠페인을 개발하고 실행한 경험, 강력한 분석 능력, 그리고 교차 기능 팀과 효과적으로 협업할 수 있는 능력을 갖추어야 합니다. 또한 최신 마케팅 트렌드와 기술에 대해 최신 정보를 유지하는 것에 열정적이어야 합니다."
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
    temperature=0.5,
    system="주어진 맥락을 바탕으로 면접을 위한 사려 깊고 개방형 질문들을 생성하는 것이 당신의 과제입니다. 질문들은 면접자로부터 통찰력 있고 상세한 답변을 이끌어내도록 설계되어야 하며, 그들이 자신의 지식, 경험, 비판적 사고 능력을 보여줄 수 있도록 해야 합니다. 예/아니오 질문이나 뻔한 답변이 있는 질문은 피하세요. 대신 성찰, 자기 평가, 구체적인 예시나 일화 공유를 장려하는 질문에 집중하세요.",
    messages=[
        {
            "role": "user",
            "content": [
                {
                    "type": "text",
                    "text": "당신은 잘 확립된 전자상거래 회사의 마케팅 매니저 직책에 대한 후보자를 면접하고 있습니다. 이상적인 후보자는 다채널 마케팅 캠페인을 개발하고 실행한 경험, 강력한 분석 능력, 그리고 교차 기능 팀과 효과적으로 협업할 수 있는 능력을 갖추어야 합니다. 또한 최신 마케팅 트렌드와 기술에 대해 최신 정보를 유지하는 것에 열정적이어야 합니다."
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
temperature: 0.5,
system: "주어진 맥락을 바탕으로 면접을 위한 사려 깊고 개방형 질문들을 생성하는 것이 당신의 과제입니다. 질문들은 면접자로부터 통찰력 있고 상세한 답변을 이끌어내도록 설계되어야 하며, 그들이 자신의 지식, 경험, 비판적 사고 능력을 보여줄 수 있도록 해야 합니다. 예/아니오 질문이나 뻔한 답변이 있는 질문은 피하세요. 대신 성찰, 자기 평가, 구체적인 예시나 일화 공유를 장려하는 질문에 집중하세요.",
messages: [
{
"role": "user",
"content": [
{
"type": "text",
"text": "당신은 잘 확립된 전자상거래 회사의 마케팅 매니저 직책에 대한 후보자를 면접하고 있습니다. 이상적인 후보자는 다채널 마케팅 캠페인을 개발하고 실행한 경험, 강력한 분석 능력, 그리고 교차 기능 팀과 효과적으로 협업할 수 있는 능력을 갖추어야 합니다. 또한 최신 마케팅 트렌드와 기술에 대해 최신 정보를 유지하는 것에 열정적이어야 합니다."
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
    max_tokens=1000,
    temperature=0.5,
    system="주어진 맥락을 바탕으로 면접을 위한 사려 깊고 개방형 질문들을 생성하는 것이 당신의 과제입니다. 질문들은 면접자로부터 통찰력 있고 상세한 답변을 이끌어내도록 설계되어야 하며, 그들이 자신의 지식, 경험, 비판적 사고 능력을 보여줄 수 있도록 해야 합니다. 예/아니오 질문이나 뻔한 답변이 있는 질문은 피하세요. 대신 성찰, 자기 평가, 구체적인 예시나 일화 공유를 장려하는 질문에 집중하세요.",
    messages=[
        {
            "role": "user",
            "content": [
                {
                    "type": "text",
                    "text": "당신은 잘 확립된 전자상거래 회사의 마케팅 매니저 직책에 대한 후보자를 면접하고 있습니다. 이상적인 후보자는 다채널 마케팅 캠페인을 개발하고 실행한 경험, 강력한 분석 능력, 그리고 교차 기능 팀과 효과적으로 협업할 수 있는 능력을 갖추어야 합니다. 또한 최신 마케팅 트렌드와 기술에 대해 최신 정보를 유지하는 것에 열정적이어야 합니다."
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
  temperature: 0.5,
  system: "주어진 맥락을 바탕으로 면접을 위한 사려 깊고 개방형 질문들을 생성하는 것이 당신의 과제입니다. 질문들은 면접자로부터 통찰력 있고 상세한 답변을 이끌어내도록 설계되어야 하며, 그들이 자신의 지식, 경험, 비판적 사고 능력을 보여줄 수 있도록 해야 합니다. 예/아니오 질문이나 뻔한 답변이 있는 질문은 피하세요. 대신 성찰, 자기 평가, 구체적인 예시나 일화 공유를 장려하는 질문에 집중하세요.",
  messages: [
    {
      "role": "user",
      "content": [
        {
          "type": "text",
          "text": "당신은 잘 확립된 전자상거래 회사의 마케팅 매니저 직책에 대한 후보자를 면접하고 있습니다. 이상적인 후보자는 다채널 마케팅 캠페인을 개발하고 실행한 경험, 강력한 분석 능력, 그리고 교차 기능 팀과 효과적으로 협업할 수 있는 능력을 갖추어야 합니다. 또한 최신 마케팅 트렌드와 기술에 대해 최신 정보를 유지하는 것에 열정적이어야 합니다."
        }
      ]
    }
  ]
});
console.log(msg);

```

</Tab>
</Tabs>