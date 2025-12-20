# 기업 투시자

긴 기업 보고서에서 통찰력을 추출하고, 위험을 식별하며, 핵심 정보를 하나의 메모로 요약합니다.

---

> 이 프롬프트를 우리 개발자 [Console](/dashboard)에 복사하여 직접 시도해보세요!

|      | 내용 |
| ---- | --- |
| 사용자 | 다음 보고서를 분석하는 것이 당신의 과제입니다: <br /> <br /> <report> [Matterport SEC 제출 10-K 2023](https://investors.matterport.com/node/9501/html)의 전체 텍스트, 간결함을 위해 여기에 붙여넣지 않음] </report> <br /> <br /> 이 연간 보고서를 간결하고 명확한 방식으로 요약하고, 주요 시장 동향과 시사점을 식별하세요. 내 팀에게 보낼 수 있는 짧은 메모로 결과를 출력하세요. 메모의 목표는 내 팀이 금융 기관들이 어떻게 지내고 있는지 최신 정보를 유지하고 다음 분기에 예상될 수 있는 운영 및 수익 위험이 있는지 정성적으로 예측하고 식별하는 것입니다. 요약과 분석에 모든 관련 세부사항을 포함해야 합니다. |

## 예시 출력

> 수신: 투자팀  
> 발신: \[귀하의 이름\]  
> 날짜: 2024년 3월 4일  
> 제목: Matterport 2022 10-K 하이라이트 및 재무 전망
>
> 안녕하세요 팀,
>
> Matterport의 2022년 연간 보고서에서 몇 가지 주요 시사점을 식별했습니다:
>
> 재무 성과:
>
> - 2022년 매출이 22% 증가하여 1억 3,610만 달러를 기록했으며, 이는 구독 및 서비스 매출 성장에 의해 주도되었습니다
> - 그러나 제품 매출 증가 비용과 공급망 문제로 인해 총 마진이 54%에서 38%로 감소했습니다
> - 회사가 성장을 위해 R&D와 영업 및 마케팅에 대규모 투자를 하면서 운영 손실이 2억 7,550만 달러로 확대되었습니다
> - 2022년 말 운영 자금 조달을 위해 4억 7,690만 달러의 현금과 투자를 보유하고 있었습니다
>
> 사업 하이라이트:
>
> - 구독자 기반이 39% 성장하여 2022년 말 70만 1천 명을 넘었습니다. 관리 중인 공간은 920만 개에 달했습니다
> - Matterport Axis 모터화 마운트, Pro3 카메라, 클라우드 소프트웨어의 주요 업데이트 출시로 플랫폼을 확장했습니다
> - Enview(3D 공간 데이터용 AI)와 VHT Studios(부동산 마케팅) 인수를 완료하여 제품을 강화했습니다
> - Burns & McDonnell 및 CGS와 주목할 만한 파트너십을 체결하여 기업 고객에게 Matterport의 솔루션을 제공했습니다
>
> 전망:
>
> - Matterport는 세계 건물과 공간을 디지털화하는 거대한 시장 기회를 보고 있습니다(2,400억 달러 이상의 TAM 추정)
> - 부동산, AEC, 시설 관리 등 주요 수직 분야의 대기업 고객 채택 촉진에 집중하고 있습니다
> - 대부분 미개척된 글로벌 건물 시장을 침투하기 위해 R&D와 국제 확장에 계속 투자할 것입니다
> - 거시경제적 역풍과 공급망 문제로 인해 단기 성장과 마진이 영향을 받을 수 있습니다
> - 그러나 건전한 유동성 포지션과 상대적으로 끈끈한 고객 기반이 하방 보호를 제공합니다
>
> 요약하면, Matterport는 거대한 글로벌 시장 침투에서 진전을 이루면서 2022년에 강력한 매출 성장을 달성했습니다. 회사가 성장을 위해 대규모 투자를 하면서 수익성은 여전히 도전적이지만, 대차대조표는 견고해 보입니다. 일부 지속적인 공급망 문제를 제외하고는 주요 운영 지표를 기반으로 기본 사업 모멘텀이 그대로 유지되는 것으로 보입니다. 심각한 경기 침체가 없다면, Matterport는 초기 단계의 건물 디지털화 공간에서 계속해서 점유율을 확보할 수 있는 좋은 위치에 있는 것으로 보입니다.

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
    temperature=0,
    messages=[
        {
            "role": "user",
            "content": [
                {
                    "type": "text",
                    "text": "다음 보고서를 분석하는 것이 당신의 과제입니다:  \n<report>  \n[Matterport SEC 제출 10-K 2023](https://investors.matterport.com/node/9501/html)의 전체 텍스트, 간결함을 위해 여기에 붙여넣지 않음]  \n</report>  \n  \n이 연간 보고서를 간결하고 명확한 방식으로 요약하고, 주요 시장 동향과 시사점을 식별하세요. 내 팀에게 보낼 수 있는 짧은 메모로 결과를 출력하세요. 메모의 목표는 내 팀이 금융 기관들이 어떻게 지내고 있는지 최신 정보를 유지하고 다음 분기에 예상될 수 있는 운영 및 수익 위험이 있는지 정성적으로 예측하고 식별하는 것입니다. 요약과 분석에 모든 관련 세부사항을 포함해야 합니다."
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
  temperature: 0,
  messages: [
    {
      "role": "user",
      "content": [
        {
          "type": "text",
          "text": "다음 보고서를 분석하는 것이 당신의 과제입니다:  \n<report>  \n[Matterport SEC 제출 10-K 2023](https://investors.matterport.com/node/9501/html)의 전체 텍스트, 간결함을 위해 여기에 붙여넣지 않음]  \n</report>  \n  \n이 연간 보고서를 간결하고 명확한 방식으로 요약하고, 주요 시장 동향과 시사점을 식별하세요. 내 팀에게 보낼 수 있는 짧은 메모로 결과를 출력하세요. 메모의 목표는 내 팀이 금융 기관들이 어떻게 지내고 있는지 최신 정보를 유지하고 다음 분기에 예상될 수 있는 운영 및 수익 위험이 있는지 정성적으로 예측하고 식별하는 것입니다. 요약과 분석에 모든 관련 세부사항을 포함해야 합니다."
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
messages=[
{
"role": "user",
"content": [
{
"type": "text",
"text": "다음 보고서를 분석하는 것이 당신의 과제입니다: \n<report> \n[Matterport SEC 제출 10-K 2023](https://investors.matterport.com/node/9501/html)의 전체 텍스트, 간결함을 위해 여기에 붙여넣지 않음] \n</report> \n \n이 연간 보고서를 간결하고 명확한 방식으로 요약하고, 주요 시장 동향과 시사점을 식별하세요. 내 팀에게 보낼 수 있는 짧은 메모로 결과를 출력하세요. 메모의 목표는 내 팀이 금융 기관들이 어떻게 지내고 있는지 최신 정보를 유지하고 다음 분기에 예상될 수 있는 운영 및 수익 위험이 있는지 정성적으로 예측하고 식별하는 것입니다. 요약과 분석에 모든 관련 세부사항을 포함해야 합니다."
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
  temperature: 0,
  messages: [
    {
      "role": "user",
      "content": [
        {
          "type": "text",
          "text": "다음 보고서를 분석하는 것이 당신의 과제입니다:  \n<report>  \n[Matterport SEC 제출 10-K 2023](https://investors.matterport.com/node/9501/html)의 전체 텍스트, 간결함을 위해 여기에 붙여넣지 않음]  \n</report>  \n  \n이 연간 보고서를 간결하고 명확한 방식으로 요약하고, 주요 시장 동향과 시사점을 식별하세요. 내 팀에게 보낼 수 있는 짧은 메모로 결과를 출력하세요. 메모의 목표는 내 팀이 금융 기관들이 어떻게 지내고 있는지 최신 정보를 유지하고 다음 분기에 예상될 수 있는 운영 및 수익 위험이 있는지 정성적으로 예측하고 식별하는 것입니다. 요약과 분석에 모든 관련 세부사항을 포함해야 합니다."
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
    temperature=0,
    messages=[
        {
            "role": "user",
            "content": [
                {
                    "type": "text",
                    "text": "다음 보고서를 분석하는 것이 당신의 과제입니다:  \n<report>  \n[Matterport SEC 제출 10-K 2023](https://investors.matterport.com/node/9501/html)의 전체 텍스트, 간결함을 위해 여기에 붙여넣지 않음]  \n</report>  \n  \n이 연간 보고서를 간결하고 명확한 방식으로 요약하고, 주요 시장 동향과 시사점을 식별하세요. 내 팀에게 보낼 수 있는 짧은 메모로 결과를 출력하세요. 메모의 목표는 내 팀이 금융 기관들이 어떻게 지내고 있는지 최신 정보를 유지하고 다음 분기에 예상될 수 있는 운영 및 수익 위험이 있는지 정성적으로 예측하고 식별하는 것입니다. 요약과 분석에 모든 관련 세부사항을 포함해야 합니다."
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
  temperature: 0,
  messages: [
    {
      "role": "user",
      "content": [
        {
          "type": "text",
          "text": "다음 보고서를 분석하는 것이 당신의 과제입니다:  \n<report>  \n[Matterport SEC 제출 10-K 2023](https://investors.matterport.com/node/9501/html)의 전체 텍스트, 간결함을 위해 여기에 붙여넣지 않음]  \n</report>  \n  \n이 연간 보고서를 간결하고 명확한 방식으로 요약하고, 주요 시장 동향과 시사점을 식별하세요. 내 팀에게 보낼 수 있는 짧은 메모로 결과를 출력하세요. 메모의 목표는 내 팀이 금융 기관들이 어떻게 지내고 있는지 최신 정보를 유지하고 다음 분기에 예상될 수 있는 운영 및 수익 위험이 있는지 정성적으로 예측하고 식별하는 것입니다. 요약과 분석에 모든 관련 세부사항을 포함해야 합니다."
        }
      ]
    }
  ]
});
console.log(msg);

```

  </Tab>
</Tabs>