# 메모 마에스트로

핵심 요점을 바탕으로 포괄적인 회사 메모를 작성합니다.

---

> 이 프롬프트를 개발자 [Console](/dashboard)에 복사하여 직접 시도해보세요!

|        | 내용 |
| --- | --- |
| System | 제공된 핵심 요점을 바탕으로 포괄적인 회사 메모를 작성하는 것이 당신의 임무입니다. 메모는 전문적인 톤으로 작성되어야 하며, 모든 관련 정보를 명확하고 간결한 방식으로 다루어야 합니다. 제목, 부제목, 글머리 기호와 같은 적절한 서식을 사용하여 내용을 효과적으로 구성하세요. 메모가 잘 구조화되고 일관성 있으며 대상 독자가 이해하기 쉽도록 하세요. |
| User   | - 새로운 직원 웰니스 프로그램 발표: "Fit4Success" <br/> - 프로그램 목표: 신체적, 정신적 웰빙 증진, 스트레스 감소, 생산성 향상 <br/> - 구성 요소: 사내 피트니스 수업, 건강한 식사 옵션, 정신 건강 자원, 웰니스 워크샵 <br/> - 할인된 멤버십을 위한 지역 체육관 및 웰니스 센터와의 파트너십 <br/> - 참여 인센티브: 피트니스 트래커, 웰니스 포인트, 경품 추첨 <br/> - 프로그램 시작일: 2023년 6월 1일 <br/> - 등록 과정: 회사 인트라넷을 통한 온라인 등록 <br/> - 프로그램 코디네이터: 인사팀 및 직원 참여팀 |

## 예시 출력

제목: Fit4Success - 새로운 직원 웰니스 프로그램 소개

안녕하세요,

팀 구성원들의 신체적, 정신적 웰빙을 증진하고 스트레스를 줄이며 생산성을 높이기 위해 설계된 새로운 직원 웰니스 프로그램인 Fit4Success의 출시를 발표하게 되어 매우 기쁩니다. 이 프로그램은 2023년 6월 1일에 공식적으로 시작됩니다.

프로그램 목표:

- 규칙적인 신체 활동과 건강한 라이프스타일 선택 장려
- 정신 건강과 스트레스 관리를 위한 자원과 지원 제공
- 직원 웰빙을 우선시하는 긍정적이고 매력적인 업무 환경 조성
- 전반적인 생산성과 직무 만족도 향상

프로그램 구성 요소:

- 사내 피트니스 수업: 요가, 필라테스, 고강도 인터벌 트레이닝(HIIT) 세션
- 건강한 식사 옵션: 회사 카페테리아에서 제공되는 영양가 있는 간식과 식사
- 정신 건강 자원: 상담 서비스, 명상 앱, 스트레스 해소 워크샵 이용
- 웰니스 워크샵: 영양, 수면 위생, 일과 삶의 균형과 같은 주제의 세미나

파트너십 및 할인:  
 여러분의 웰니스 여정을 더욱 지원하기 위해, 저희는 지역 체육관 및 웰니스 센터와 파트너십을 맺어 직원들에게만 할인된 멤버십을 제공합니다. 이러한 특별 요금을 활용하여 다양한 피트니스 시설과 수업에 접근하세요.

참여 인센티브:  
 Fit4Success 프로그램에 적극적인 참여를 장려하기 위해 다음과 같은 인센티브를 제공할 예정입니다:

- 모든 참가자에게 진행 상황을 모니터링하고 개인 목표를 설정할 수 있는 피트니스 트래커
- 프로그램 참여를 통해 획득하는 웰니스 포인트, 추가 보상으로 교환 가능
- 특정 웰니스 마일스톤을 달성한 참가자를 위한 월별 경품 추첨

등록 과정:  
 Fit4Success 프로그램에 등록하려면 회사 인트라넷을 방문하여 온라인 등록 양식을 작성해 주세요. 등록 마감일은 2023년 5월 25일입니다. 등록 과정에 대한 질문이나 우려사항이 있으시면 인사팀에 문의해 주세요.

프로그램 조정:  
 Fit4Success 프로그램은 인사팀과 직원 참여팀이 공동으로 조정할 예정입니다. 이들은 여러분의 웰니스 여정 전반에 걸쳐 지침, 지원, 자원을 제공할 것입니다.

저희는 직원들의 웰빙에 투자하는 것이 조직의 성공과 성장에 중요하다고 굳게 믿습니다. 이 흥미진진한 기회를 활용하여 더 건강하고 행복하며 생산적인 직장을 만드는 데 함께해 주시기 바랍니다.

Fit4Success 프로그램을 받아들이고 더 건강한 미래를 향해 함께 노력합시다!

감사합니다,  
 \[귀하의 이름\]  
 \[귀하의 직책\]

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
    system="제공된 핵심 요점을 바탕으로 포괄적인 회사 메모를 작성하는 것이 당신의 임무입니다. 메모는 전문적인 톤으로 작성되어야 하며, 모든 관련 정보를 명확하고 간결한 방식으로 다루어야 합니다. 제목, 부제목, 글머리 기호와 같은 적절한 서식을 사용하여 내용을 효과적으로 구성하세요. 메모가 잘 구조화되고 일관성 있으며 대상 독자가 이해하기 쉽도록 하세요.",
    messages=[
        {
            "role": "user",
            "content": [
                {
                    "type": "text",
                    "text": "- 새로운 직원 웰니스 프로그램 발표: \"Fit4Success\"  \n- 프로그램 목표: 신체적, 정신적 웰빙 증진, 스트레스 감소, 생산성 향상  \n- 구성 요소: 사내 피트니스 수업, 건강한 식사 옵션, 정신 건강 자원, 웰니스 워크샵  \n- 할인된 멤버십을 위한 지역 체육관 및 웰니스 센터와의 파트너십  \n- 참여 인센티브: 피트니스 트래커, 웰니스 포인트, 경품 추첨  \n- 프로그램 시작일: 2023년 6월 1일  \n- 등록 과정: 회사 인트라넷을 통한 온라인 등록  \n- 프로그램 코디네이터: 인사팀 및 직원 참여팀"
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
  system: "제공된 핵심 요점을 바탕으로 포괄적인 회사 메모를 작성하는 것이 당신의 임무입니다. 메모는 전문적인 톤으로 작성되어야 하며, 모든 관련 정보를 명확하고 간결한 방식으로 다루어야 합니다. 제목, 부제목, 글머리 기호와 같은 적절한 서식을 사용하여 내용을 효과적으로 구성하세요. 메모가 잘 구조화되고 일관성 있으며 대상 독자가 이해하기 쉽도록 하세요.",
  messages: [
    {
      "role": "user",
      "content": [
        {
          "type": "text",
          "text": "- 새로운 직원 웰니스 프로그램 발표: \"Fit4Success\"  \n- 프로그램 목표: 신체적, 정신적 웰빙 증진, 스트레스 감소, 생산성 향상  \n- 구성 요소: 사내 피트니스 수업, 건강한 식사 옵션, 정신 건강 자원, 웰니스 워크샵  \n- 할인된 멤버십을 위한 지역 체육관 및 웰니스 센터와의 파트너십  \n- 참여 인센티브: 피트니스 트래커, 웰니스 포인트, 경품 추첨  \n- 프로그램 시작일: 2023년 6월 1일  \n- 등록 과정: 회사 인트라넷을 통한 온라인 등록  \n- 프로그램 코디네이터: 인사팀 및 직원 참여팀"
        }
      ]
    }
  ]
});
console.log(msg);

```

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
system="제공된 핵심 요점을 바탕으로 포괄적인 회사 메모를 작성하는 것이 당신의 임무입니다. 메모는 전문적인 톤으로 작성되어야 하며, 모든 관련 정보를 명확하고 간결한 방식으로 다루어야 합니다. 제목, 부제목, 글머리 기호와 같은 적절한 서식을 사용하여 내용을 효과적으로 구성하세요. 메모가 잘 구조화되고 일관성 있으며 대상 독자가 이해하기 쉽도록 하세요.",
messages=[
{
"role": "user",
"content": [
{
"type": "text",
"text": "- 새로운 직원 웰니스 프로그램 발표: \"Fit4Success\" \n- 프로그램 목표: 신체적, 정신적 웰빙 증진, 스트레스 감소, 생산성 향상 \n- 구성 요소: 사내 피트니스 수업, 건강한 식사 옵션, 정신 건강 자원, 웰니스 워크샵 \n- 할인된 멤버십을 위한 지역 체육관 및 웰니스 센터와의 파트너십 \n- 참여 인센티브: 피트니스 트래커, 웰니스 포인트, 경품 추첨 \n- 프로그램 시작일: 2023년 6월 1일 \n- 등록 과정: 회사 인트라넷을 통한 온라인 등록 \n- 프로그램 코디네이터: 인사팀 및 직원 참여팀"
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
  system: "제공된 핵심 요점을 바탕으로 포괄적인 회사 메모를 작성하는 것이 당신의 임무입니다. 메모는 전문적인 톤으로 작성되어야 하며, 모든 관련 정보를 명확하고 간결한 방식으로 다루어야 합니다. 제목, 부제목, 글머리 기호와 같은 적절한 서식을 사용하여 내용을 효과적으로 구성하세요. 메모가 잘 구조화되고 일관성 있으며 대상 독자가 이해하기 쉽도록 하세요.",
  messages: [
    {
      "role": "user",
      "content": [
        {
          "type": "text",
          "text": "- 새로운 직원 웰니스 프로그램 발표: \"Fit4Success\"  \n- 프로그램 목표: 신체적, 정신적 웰빙 증진, 스트레스 감소, 생산성 향상  \n- 구성 요소: 사내 피트니스 수업, 건강한 식사 옵션, 정신 건강 자원, 웰니스 워크샵  \n- 할인된 멤버십을 위한 지역 체육관 및 웰니스 센터와의 파트너십  \n- 참여 인센티브: 피트니스 트래커, 웰니스 포인트, 경품 추첨  \n- 프로그램 시작일: 2023년 6월 1일  \n- 등록 과정: 회사 인트라넷을 통한 온라인 등록  \n- 프로그램 코디네이터: 인사팀 및 직원 참여팀"
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
    system="제공된 핵심 요점을 바탕으로 포괄적인 회사 메모를 작성하는 것이 당신의 임무입니다. 메모는 전문적인 톤으로 작성되어야 하며, 모든 관련 정보를 명확하고 간결한 방식으로 다루어야 합니다. 제목, 부제목, 글머리 기호와 같은 적절한 서식을 사용하여 내용을 효과적으로 구성하세요. 메모가 잘 구조화되고 일관성 있으며 대상 독자가 이해하기 쉽도록 하세요.",
    messages=[
        {
            "role": "user",
            "content": [
                {
                    "type": "text",
                    "text": "- 새로운 직원 웰니스 프로그램 발표: \"Fit4Success\"  \n- 프로그램 목표: 신체적, 정신적 웰빙 증진, 스트레스 감소, 생산성 향상  \n- 구성 요소: 사내 피트니스 수업, 건강한 식사 옵션, 정신 건강 자원, 웰니스 워크샵  \n- 할인된 멤버십을 위한 지역 체육관 및 웰니스 센터와의 파트너십  \n- 참여 인센티브: 피트니스 트래커, 웰니스 포인트, 경품 추첨  \n- 프로그램 시작일: 2023년 6월 1일  \n- 등록 과정: 회사 인트라넷을 통한 온라인 등록  \n- 프로그램 코디네이터: 인사팀 및 직원 참여팀"
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
  system: "제공된 핵심 요점을 바탕으로 포괄적인 회사 메모를 작성하는 것이 당신의 임무입니다. 메모는 전문적인 톤으로 작성되어야 하며, 모든 관련 정보를 명확하고 간결한 방식으로 다루어야 합니다. 제목, 부제목, 글머리 기호와 같은 적절한 서식을 사용하여 내용을 효과적으로 구성하세요. 메모가 잘 구조화되고 일관성 있으며 대상 독자가 이해하기 쉽도록 하세요.",
  messages: [
    {
      "role": "user",
      "content": [
        {
          "type": "text",
          "text": "- 새로운 직원 웰니스 프로그램 발표: \"Fit4Success\"  \n- 프로그램 목표: 신체적, 정신적 웰빙 증진, 스트레스 감소, 생산성 향상  \n- 구성 요소: 사내 피트니스 수업, 건강한 식사 옵션, 정신 건강 자원, 웰니스 워크샵  \n- 할인된 멤버십을 위한 지역 체육관 및 웰니스 센터와의 파트너십  \n- 참여 인센티브: 피트니스 트래커, 웰니스 포인트, 경품 추첨  \n- 프로그램 시작일: 2023년 6월 1일  \n- 등록 과정: 회사 인트라넷을 통한 온라인 등록  \n- 프로그램 코디네이터: 인사팀 및 직원 참여팀"
        }
      ]
    }
  ]
});
console.log(msg);

```

</Tab>
</Tabs>