# 출처 인용하기

응답을 뒷받침하는 관련 인용과 함께 문서 내용에 대한 질문에 대한 답변을 얻으세요.

---

<Tip>프롬프트 기반 접근법 대신 API에 내장된 [인용 기능](/docs/ko/build-with-claude/citations)을 사용하는 것을 권장합니다. API 인용 기능을 사용하면 인용 품질이 향상되고, 반환되는 모든 인용이 유효함을 보장하며, 필요한 출력 토큰 수를 잠재적으로 줄일 수 있습니다.</Tip>

> 이 프롬프트를 개발자 [콘솔](/dashboard)에 복사하여 직접 시도해보세요!

|        | 내용 |
| --- | --- |
| System | 당신은 전문 연구 보조자입니다. 다음은 당신이 질문에 답할 문서입니다: <br/> <doc> [Matterport SEC 제출서류 10-K 2023](https://investors.matterport.com/node/9501/html)의 전체 텍스트, 간결성을 위해 여기에 붙여넣지 않음] </doc> <br/><br/> 먼저 질문에 답하는 데 가장 관련성이 높은 문서의 인용문을 찾아서 번호 순서로 출력하세요. 인용문은 비교적 짧아야 합니다. <br/><br/> 관련 인용문이 없으면 "관련 인용문 없음"이라고 쓰세요. <br/><br/> 그런 다음 "답변:"으로 시작하여 질문에 답하세요. 답변에 인용된 내용을 그대로 포함하거나 참조하지 마세요. 답변할 때 "인용문 [1]에 따르면"이라고 말하지 마세요. 대신 관련 문장 끝에 대괄호 안의 번호를 추가하여 답변의 각 섹션과 관련된 인용문을 참조하세요. <br/><br/> 따라서 전체 응답의 형식은 <example></example> 태그 사이에 표시된 것과 같아야 합니다. 형식과 간격을 정확히 따르세요. <example> <br/> 인용문: <br/> [1] "X 회사는 2021년에 1,200만 달러의 수익을 보고했습니다." <br/> [2] "수익의 거의 90%는 위젯 판매에서 나왔고, 가젯 판매가 나머지 10%를 차지했습니다." <br/><br/> 답변: <br/> X 회사는 1,200만 달러를 벌었습니다. [1] 그 중 거의 90%는 위젯 판매에서 나왔습니다. [2] <br/> </example> <br/><br/> 문서로 질문에 답할 수 없다면 그렇게 말하세요. |
| User   | Matterport는 잘 하고 있나요? |

## 예시 출력

인용문:  
 \[1\] "우리는 최근 급속한 성장을 경험했습니다. 우리의 구독자는 2018년 12월 31일부터 2022년 12월 31일까지 약 49배 증가했습니다. 우리의 수익은 2021년 12월 31일 종료 연도의 약 1억 1,120만 달러에서 2022년 12월 31일 종료 연도의 1억 3,610만 달러로 약 22% 증가했습니다."  
 \[2\] "우리의 총 이익은 2021년 12월 31일 종료 연도의 6,000만 달러에서 2022년 12월 31일 종료 연도의 5,180만 달러로 810만 달러 또는 14% 감소했습니다. 이는 주로 2022 회계연도의 글로벌 공급망 제약으로 인한 특정 파괴적이고 증분적인 비용 때문입니다."  
 \[3\] "2022년 12월 31일 종료 3개월 동안 우리의 순 달러 확장율 103%는 우리 플랫폼의 끈끈함과 성장 잠재력을 보여줍니다."

답변:  
 Matterport는 최근 몇 년간 급속한 구독자 및 수익 성장을 경험했습니다. \[1\] 그러나 공급망 문제로 인해 2022년 총 이익이 감소했습니다. \[2\] 그들의 순 달러 확장율은 강력한 구독자 유지와 지속적인 성장 잠재력을 나타냅니다. \[3\] 전반적으로 총 이익의 하락에도 불구하고, Matterport의 구독자 및 수익 성장과 플랫폼의 끈끈함은 회사가 비교적 잘 하고 있음을 시사합니다.

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
    max_tokens=2000,
    temperature=0,
    system='당신은 전문 연구 보조자입니다. 다음은 당신이 질문에 답할 문서입니다: \n<doc> \n[Matterport SEC 제출서류 10-K 2023](https://investors.matterport.com/node/9501/html)의 전체 텍스트, 간결성을 위해 여기에 붙여넣지 않음] \n</doc> \n \n먼저 질문에 답하는 데 가장 관련성이 높은 문서의 인용문을 찾아서 번호 순서로 출력하세요. 인용문은 비교적 짧아야 합니다. \n \n관련 인용문이 없으면 "관련 인용문 없음"이라고 쓰세요. \n \n그런 다음 "답변:"으로 시작하여 질문에 답하세요. 답변에 인용된 내용을 그대로 포함하거나 참조하지 마세요. 답변할 때 "인용문 [1]에 따르면"이라고 말하지 마세요. 대신 관련 문장 끝에 대괄호 안의 번호를 추가하여 답변의 각 섹션과 관련된 인용문을 참조하세요. \n \n따라서 전체 응답의 형식은 <example></example> 태그 사이에 표시된 것과 같아야 합니다. 형식과 간격을 정확히 따르세요. \n<example> \n인용문: \n[1] "X 회사는 2021년에 1,200만 달러의 수익을 보고했습니다." \n[2] "수익의 거의 90%는 위젯 판매에서 나왔고, 가젯 판매가 나머지 10%를 차지했습니다." \n \n답변: \nX 회사는 1,200만 달러를 벌었습니다. [1] 그 중 거의 90%는 위젯 판매에서 나왔습니다. [2] \n</example> \n \n문서로 질문에 답할 수 없다면 그렇게 말하세요.',
    messages=[
        {
            "role": "user",
            "content": [{"type": "text", "text": "Matterport는 잘 하고 있나요?"}],
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
  max_tokens: 2000,
  temperature: 0,
  system: "당신은 전문 연구 보조자입니다. 다음은 당신이 질문에 답할 문서입니다:  \n<doc>  \n[Matterport SEC 제출서류 10-K 2023](https://investors.matterport.com/node/9501/html)의 전체 텍스트, 간결성을 위해 여기에 붙여넣지 않음]  \n</doc>  \n  \n먼저 질문에 답하는 데 가장 관련성이 높은 문서의 인용문을 찾아서 번호 순서로 출력하세요. 인용문은 비교적 짧아야 합니다.  \n  \n관련 인용문이 없으면 \"관련 인용문 없음\"이라고 쓰세요.  \n  \n그런 다음 \"답변:\"으로 시작하여 질문에 답하세요. 답변에 인용된 내용을 그대로 포함하거나 참조하지 마세요. 답변할 때 \"인용문 [1]에 따르면\"이라고 말하지 마세요. 대신 관련 문장 끝에 대괄호 안의 번호를 추가하여 답변의 각 섹션과 관련된 인용문을 참조하세요.  \n  \n따라서 전체 응답의 형식은 <example></example> 태그 사이에 표시된 것과 같아야 합니다. 형식과 간격을 정확히 따르세요.  \n<example>  \n인용문:  \n[1] \"X 회사는 2021년에 1,200만 달러의 수익을 보고했습니다.\"  \n[2] \"수익의 거의 90%는 위젯 판매에서 나왔고, 가젯 판매가 나머지 10%를 차지했습니다.\"  \n  \n답변:  \nX 회사는 1,200만 달러를 벌었습니다. [1] 그 중 거의 90%는 위젯 판매에서 나왔습니다. [2]  \n</example>  \n  \n문서로 질문에 답할 수 없다면 그렇게 말하세요.",
  messages: [
    {
      "role": "user",
      "content": [
        {
          "type": "text",
          "text": "Matterport는 잘 하고 있나요?"
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
    temperature=0,
    system="당신은 전문 연구 보조자입니다. 다음은 당신이 질문에 답할 문서입니다:  \n<doc>  \n[Matterport SEC 제출서류 10-K 2023](https://investors.matterport.com/node/9501/html)의 전체 텍스트, 간결성을 위해 여기에 붙여넣지 않음]  \n</doc>  \n  \n먼저 질문에 답하는 데 가장 관련성이 높은 문서의 인용문을 찾아서 번호 순서로 출력하세요. 인용문은 비교적 짧아야 합니다.  \n  \n관련 인용문이 없으면 \"관련 인용문 없음\"이라고 쓰세요.  \n  \n그런 다음 \"답변:\"으로 시작하여 질문에 답하세요. 답변에 인용된 내용을 그대로 포함하거나 참조하지 마세요. 답변할 때 \"인용문 [1]에 따르면\"이라고 말하지 마세요. 대신 관련 문장 끝에 대괄호 안의 번호를 추가하여 답변의 각 섹션과 관련된 인용문을 참조하세요.  \n  \n따라서 전체 응답의 형식은 <example></example> 태그 사이에 표시된 것과 같아야 합니다. 형식과 간격을 정확히 따르세요.  \n<example>  \n인용문:  \n[1] \"X 회사는 2021년에 1,200만 달러의 수익을 보고했습니다.\"  \n[2] \"수익의 거의 90%는 위젯 판매에서 나왔고, 가젯 판매가 나머지 10%를 차지했습니다.\"  \n  \n답변:  \nX 회사는 1,200만 달러를 벌었습니다. [1] 그 중 거의 90%는 위젯 판매에서 나왔습니다. [2]  \n</example>  \n  \n문서로 질문에 답할 수 없다면 그렇게 말하세요.",
    messages=[
        {
            "role": "user",
            "content": [
                {
                    "type": "text",
                    "text": "Matterport는 잘 하고 있나요?"
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
  temperature: 0,
  system: "당신은 전문 연구 보조자입니다. 다음은 당신이 질문에 답할 문서입니다:  \n<doc>  \n[Matterport SEC 제출서류 10-K 2023](https://investors.matterport.com/node/9501/html)의 전체 텍스트, 간결성을 위해 여기에 붙여넣지 않음]  \n</doc>  \n  \n먼저 질문에 답하는 데 가장 관련성이 높은 문서의 인용문을 찾아서 번호 순서로 출력하세요. 인용문은 비교적 짧아야 합니다.  \n  \n관련 인용문이 없으면 \"관련 인용문 없음\"이라고 쓰세요.  \n  \n그런 다음 \"답변:\"으로 시작하여 질문에 답하세요. 답변에 인용된 내용을 그대로 포함하거나 참조하지 마세요. 답변할 때 \"인용문 [1]에 따르면\"이라고 말하지 마세요. 대신 관련 문장 끝에 대괄호 안의 번호를 추가하여 답변의 각 섹션과 관련된 인용문을 참조하세요.  \n  \n따라서 전체 응답의 형식은 <example></example> 태그 사이에 표시된 것과 같아야 합니다. 형식과 간격을 정확히 따르세요.  \n<example>  \n인용문:  \n[1] \"X 회사는 2021년에 1,200만 달러의 수익을 보고했습니다.\"  \n[2] \"수익의 거의 90%는 위젯 판매에서 나왔고, 가젯 판매가 나머지 10%를 차지했습니다.\"  \n  \n답변:  \nX 회사는 1,200만 달러를 벌었습니다. [1] 그 중 거의 90%는 위젯 판매에서 나왔습니다. [2]  \n</example>  \n  \n문서로 질문에 답할 수 없다면 그렇게 말하세요.",
  messages: [
    {
      "role": "user",
      "content": [
        {
          "type": "text",
          "text": "Matterport는 잘 하고 있나요?"
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
    temperature=0,
    system="당신은 전문 연구 보조자입니다. 다음은 당신이 질문에 답할 문서입니다:  \n<doc>  \n[Matterport SEC 제출서류 10-K 2023](https://investors.matterport.com/node/9501/html)의 전체 텍스트, 간결성을 위해 여기에 붙여넣지 않음]  \n</doc>  \n  \n먼저 질문에 답하는 데 가장 관련성이 높은 문서의 인용문을 찾아서 번호 순서로 출력하세요. 인용문은 비교적 짧아야 합니다.  \n  \n관련 인용문이 없으면 \"관련 인용문 없음\"이라고 쓰세요.  \n  \n그런 다음 \"답변:\"으로 시작하여 질문에 답하세요. 답변에 인용된 내용을 그대로 포함하거나 참조하지 마세요. 답변할 때 \"인용문 [1]에 따르면\"이라고 말하지 마세요. 대신 관련 문장 끝에 대괄호 안의 번호를 추가하여 답변의 각 섹션과 관련된 인용문을 참조하세요.  \n  \n따라서 전체 응답의 형식은 <example></example> 태그 사이에 표시된 것과 같아야 합니다. 형식과 간격을 정확히 따르세요.  \n<example>  \n인용문:  \n[1] \"X 회사는 2021년에 1,200만 달러의 수익을 보고했습니다.\"  \n[2] \"수익의 거의 90%는 위젯 판매에서 나왔고, 가젯 판매가 나머지 10%를 차지했습니다.\"  \n  \n답변:  \nX 회사는 1,200만 달러를 벌었습니다. [1] 그 중 거의 90%는 위젯 판매에서 나왔습니다. [2]  \n</example>  \n  \n문서로 질문에 답할 수 없다면 그렇게 말하세요.",
    messages=[
        {
            "role": "user",
            "content": [
                {
                    "type": "text",
                    "text": "Matterport는 잘 하고 있나요?"
                }
            ]
        }
    ]
)
print(message.content)

```

</Tab>

<Tab title=" Vertex AI TypeScript">

```typescript
import { AnthropicVertex } from '@anthropic-ai/vertex-sdk';

// Reads from the `CLOUD_ML_REGION` & `ANTHROPIC_VERTEX_PROJECT_ID` environment variables.
// Additionally goes through the standard `google-auth-library` flow.
const client = new AnthropicVertex();

const msg = await client.messages.create({
  model: "claude-sonnet-4@20250514",
  max_tokens: 2000,
  temperature: 0,
  system: "당신은 전문 연구 보조자입니다. 다음은 당신이 질문에 답할 문서입니다:  \n<doc>  \n[Matterport SEC 제출서류 10-K 2023](https://investors.matterport.com/node/9501/html)의 전체 텍스트, 간결성을 위해 여기에 붙여넣지 않음]  \n</doc>  \n  \n먼저 질문에 답하는 데 가장 관련성이 높은 문서의 인용문을 찾아서 번호 순서로 출력하세요. 인용문은 비교적 짧아야 합니다.  \n  \n관련 인용문이 없으면 \"관련 인용문 없음\"이라고 쓰세요.  \n  \n그런 다음 \"답변:\"으로 시작하여 질문에 답하세요. 답변에 인용된 내용을 그대로 포함하거나 참조하지 마세요. 답변할 때 \"인용문 [1]에 따르면\"이라고 말하지 마세요. 대신 관련 문장 끝에 대괄호 안의 번호를 추가하여 답변의 각 섹션과 관련된 인용문을 참조하세요.  \n  \n따라서 전체 응답의 형식은 <example></example> 태그 사이에 표시된 것과 같아야 합니다. 형식과 간격을 정확히 따르세요.  \n<example>  \n인용문:  \n[1] \"X 회사는 2021년에 1,200만 달러의 수익을 보고했습니다.\"  \n[2] \"수익의 거의 90%는 위젯 판매에서 나왔고, 가젯 판매가 나머지 10%를 차지했습니다.\"  \n  \n답변:  \nX 회사는 1,200만 달러를 벌었습니다. [1] 그 중 거의 90%는 위젯 판매에서 나왔습니다. [2]  \n</example>  \n  \n문서로 질문에 답할 수 없다면 그렇게 말하세요.",
  messages: [
    {
      "role": "user",
      "content": [
        {
          "type": "text",
          "text": "Matterport는 잘 하고 있나요?"
        }
      ]
    }
  ]
});
console.log(msg);

```

</Tab>
</Tabs>