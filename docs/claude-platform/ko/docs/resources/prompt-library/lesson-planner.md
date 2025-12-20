# 수업 계획서 작성자

어떤 주제든 심도 있는 수업 계획서를 작성합니다.

---

> 이 프롬프트를 개발자 [Console](/login?selectAccount=true&returnTo=%2Fdashboard%3F)에 복사하여 직접 시도해보세요!

|        | 내용 |
| --- | --- |
| System | 주어진 주제에 대해 포괄적이고 매력적이며 잘 구성된 수업 계획서를 작성하는 것이 당신의 임무입니다. 수업 계획서는 60분 수업 세션을 위해 설계되어야 하며 특정 학년 수준이나 연령대에 맞춰져야 합니다. 명확하고 측정 가능하며 관련 교육 표준에 부합하는 수업 목표를 명시하는 것부터 시작하세요. 다음으로, 수업을 도입부, 주요 활동, 결론으로 나누어 상세한 수업 개요를 제공하세요. 각 섹션에 대해 내용을 효과적으로 전달하고 학생들의 참여를 유도하기 위해 사용할 교수법, 학습 활동, 자료를 설명하세요. 마지막으로, 수업 목표에 대한 학생들의 이해와 숙달을 평가하기 위해 사용할 평가 방법을 설명하세요. 수업 계획서는 잘 정리되고 따라하기 쉬우며 능동적 학습과 비판적 사고를 촉진해야 합니다. |
| User   | 주제: 광합성 입문 학년: 7학년 (12-13세) |

## 예시 출력

> 수업 계획서: 광합성 입문
> 학년: 7학년 (12-13세)
> 수업 시간: 60분
>
> 수업 목표:
> 수업이 끝날 때까지 학생들은 다음을 할 수 있어야 합니다:
>
> 1. 광합성을 정의하고 생태계에서의 중요성을 설명한다.
> 2. 광합성이 일어나는 데 필요한 핵심 구성 요소를 식별한다.
> 3. 빛 에너지, 이산화탄소, 물의 역할을 포함하여 광합성 과정을 설명한다.
> 4. 광합성의 산물과 생명체에 대한 그 중요성을 인식한다.
>    수업 개요:
>
> 5. 도입부 (10분)
>    i. 식물과 환경에서의 역할에 대한 짧은 비디오 클립이나 이미지를 보여주며 학생들의 관심을 유도한다.
>    ii. 광합성의 개념과 지구상 생명 유지에서의 중요성을 소개한다.
>    iii. 수업 목표를 명시하고 수업 구조의 개요를 제공한다.
> 6. 주요 활동 (40분)
>    i. 안내된 수업 (15분)
>    a. 광합성에 필요한 핵심 구성 요소를 설명한다: 햇빛, 이산화탄소, 물, 엽록소.
>    b. 빛 에너지, 이산화탄소, 물의 역할을 강조하며 광합성 과정을 설명한다.
>    c. 광합성의 산물(포도당과 산소)과 생명체에 대한 그 중요성을 논의한다.
>    ii. 소그룹 활동 (15분)
>    a. 학급을 소그룹으로 나누고 각 그룹에 워크시트나 그래픽 조직도를 제공한다.
>    b. 학생들이 함께 협력하여 광합성 과정의 시각적 표현을 만들고 핵심 구성 요소와 산물에 라벨을 붙이도록 한다.
>    c. 학생들이 그룹 내에서 광합성에 대한 이해를 논의하고 설명하도록 격려한다.
>    iii. 학급 토론 (10분)
>    a. 각 그룹이 자신들의 시각적 표현을 학급과 공유하도록 초대한다.
>    b. 학생들이 질문하고, 피드백을 제공하며, 오해를 명확히 하도록 격려하며 학급 토론을 진행한다.
> 7. 결론 (10분)
>    i. 광합성의 정의, 구성 요소, 과정, 산물을 강화하며 수업의 핵심 포인트를 요약한다.
>    ii. 수업 목표에 대한 학생들의 이해를 평가하기 위해 짧은 퀴즈나 출구 티켓을 실시한다.
>    iii. 다양한 생태계에서 광합성의 중요성을 더 탐구할 숙제를 학생들에게 제공한다.
>    평가 방법:
>
> 8. 형성 평가: 안내된 수업과 소그룹 활동 중 학생 참여와 이해를 관찰한다.
> 9. 총괄 평가: 학생들의 시각적 표현과 짧은 퀴즈나 출구 티켓에서의 성과를 평가한다.
> 10. 숙제: 수업에서 배운 개념을 적용하는 능력을 측정하기 위해 학생들의 숙제 제출물을 검토한다.
>     자료:
>
> 11. 식물과 환경에서의 역할에 대한 비디오 클립이나 이미지
> 12. 소그룹 활동을 위한 워크시트나 그래픽 조직도
> 13. 평가를 위한 짧은 퀴즈나 출구 티켓
> 14. 숙제 지시사항

### API 요청

<CodeGroup>
    ```python Python
    import anthropic
    
    client = anthropic.Anthropic(
        # defaults to os.environ.get("ANTHROPIC_API_KEY")
        api_key="my_api_key",
    )
    message = client.messages.create(
        model="claude-sonnet-4-5",
        max_tokens=4000,
        temperature=0.5,
        system="주어진 주제에 대해 포괄적이고 매력적이며 잘 구성된 수업 계획서를 작성하는 것이 당신의 임무입니다. 수업 계획서는 60분 수업 세션을 위해 설계되어야 하며 특정 학년 수준이나 연령대에 맞춰져야 합니다. 명확하고 측정 가능하며 관련 교육 표준에 부합하는 수업 목표를 명시하는 것부터 시작하세요. 다음으로, 수업을 도입부, 주요 활동, 결론으로 나누어 상세한 수업 개요를 제공하세요. 각 섹션에 대해 내용을 효과적으로 전달하고 학생들의 참여를 유도하기 위해 사용할 교수법, 학습 활동, 자료를 설명하세요. 마지막으로, 수업 목표에 대한 학생들의 이해와 숙달을 평가하기 위해 사용할 평가 방법을 설명하세요. 수업 계획서는 잘 정리되고 따라하기 쉬우며 능동적 학습과 비판적 사고를 촉진해야 합니다.",
        messages=[
            {
                "role": "user",
                "content": [
                    {
                        "type": "text",
                        "text": "주제: 광합성 입문  \n학년: 7학년 (12-13세)"
                    }
                ]
            }
        ]
    )
    print(message.content)
    
    ```
    
    
    ```typescript TypeScript
    import Anthropic from "@anthropic-ai/sdk";
    
    const anthropic = new Anthropic({
      apiKey: "my_api_key", // defaults to process.env["ANTHROPIC_API_KEY"]
    });
    
    const msg = await anthropic.messages.create({
      model: "claude-sonnet-4-5",
      max_tokens: 4000,
      temperature: 0.5,
      system: "주어진 주제에 대해 포괄적이고 매력적이며 잘 구성된 수업 계획서를 작성하는 것이 당신의 임무입니다. 수업 계획서는 60분 수업 세션을 위해 설계되어야 하며 특정 학년 수준이나 연령대에 맞춰져야 합니다. 명확하고 측정 가능하며 관련 교육 표준에 부합하는 수업 목표를 명시하는 것부터 시작하세요. 다음으로, 수업을 도입부, 주요 활동, 결론으로 나누어 상세한 수업 개요를 제공하세요. 각 섹션에 대해 내용을 효과적으로 전달하고 학생들의 참여를 유도하기 위해 사용할 교수법, 학습 활동, 자료를 설명하세요. 마지막으로, 수업 목표에 대한 학생들의 이해와 숙달을 평가하기 위해 사용할 평가 방법을 설명하세요. 수업 계획서는 잘 정리되고 따라하기 쉬우며 능동적 학습과 비판적 사고를 촉진해야 합니다.",
      messages: [
        {
          "role": "user",
          "content": [
            {
              "type": "text",
              "text": "주제: 광합성 입문  \n학년: 7학년 (12-13세)"
            }
          ]
        }
      ]
    });
    console.log(msg);
    
    ```
    
    
    ```python AWS Bedrock Python
    from anthropic import AnthropicBedrock
    
    # See https://docs.claude.com/claude/reference/claude-on-amazon-bedrock
    # for authentication options
    client = AnthropicBedrock()
    
    message = client.messages.create(
        model="anthropic.claude-sonnet-4-5-20250929-v1:0",
        max_tokens=4000,
        temperature=0.5,
        system="주어진 주제에 대해 포괄적이고 매력적이며 잘 구성된 수업 계획서를 작성하는 것이 당신의 임무입니다. 수업 계획서는 60분 수업 세션을 위해 설계되어야 하며 특정 학년 수준이나 연령대에 맞춰져야 합니다. 명확하고 측정 가능하며 관련 교육 표준에 부합하는 수업 목표를 명시하는 것부터 시작하세요. 다음으로, 수업을 도입부, 주요 활동, 결론으로 나누어 상세한 수업 개요를 제공하세요. 각 섹션에 대해 내용을 효과적으로 전달하고 학생들의 참여를 유도하기 위해 사용할 교수법, 학습 활동, 자료를 설명하세요. 마지막으로, 수업 목표에 대한 학생들의 이해와 숙달을 평가하기 위해 사용할 평가 방법을 설명하세요. 수업 계획서는 잘 정리되고 따라하기 쉬우며 능동적 학습과 비판적 사고를 촉진해야 합니다.",
        messages=[
            {
                "role": "user",
                "content": [
                    {
                        "type": "text",
                        "text": "주제: 광합성 입문  \n학년: 7학년 (12-13세)"
                    }
                ]
            }
        ]
    )
    print(message.content)
    
    ```
    
    
    ```typescript AWS Bedrock TypeScript
    import AnthropicBedrock from "@anthropic-ai/bedrock-sdk";
    
    // See https://docs.claude.com/claude/reference/claude-on-amazon-bedrock
    // for authentication options
    const client = new AnthropicBedrock();
    
    const msg = await client.messages.create({
      model: "anthropic.claude-sonnet-4-5-20250929-v1:0",
      max_tokens: 4000,
      temperature: 0.5,
      system: "주어진 주제에 대해 포괄적이고 매력적이며 잘 구성된 수업 계획서를 작성하는 것이 당신의 임무입니다. 수업 계획서는 60분 수업 세션을 위해 설계되어야 하며 특정 학년 수준이나 연령대에 맞춰져야 합니다. 명확하고 측정 가능하며 관련 교육 표준에 부합하는 수업 목표를 명시하는 것부터 시작하세요. 다음으로, 수업을 도입부, 주요 활동, 결론으로 나누어 상세한 수업 개요를 제공하세요. 각 섹션에 대해 내용을 효과적으로 전달하고 학생들의 참여를 유도하기 위해 사용할 교수법, 학습 활동, 자료를 설명하세요. 마지막으로, 수업 목표에 대한 학생들의 이해와 숙달을 평가하기 위해 사용할 평가 방법을 설명하세요. 수업 계획서는 잘 정리되고 따라하기 쉬우며 능동적 학습과 비판적 사고를 촉진해야 합니다.",
      messages: [
        {
          "role": "user",
          "content": [
            {
              "type": "text",
              "text": "주제: 광합성 입문  \n학년: 7학년 (12-13세)"
            }
          ]
        }
      ]
    });
    console.log(msg);
    
    ```
    
    
    ```python Vertex AI Python
    from anthropic import AnthropicVertex
    
    client = AnthropicVertex()
    
    message = client.messages.create(
        model="claude-sonnet-4@20250514",
        max_tokens=4000,
        temperature=0.5,
        system="주어진 주제에 대해 포괄적이고 매력적이며 잘 구성된 수업 계획서를 작성하는 것이 당신의 임무입니다. 수업 계획서는 60분 수업 세션을 위해 설계되어야 하며 특정 학년 수준이나 연령대에 맞춰져야 합니다. 명확하고 측정 가능하며 관련 교육 표준에 부합하는 수업 목표를 명시하는 것부터 시작하세요. 다음으로, 수업을 도입부, 주요 활동, 결론으로 나누어 상세한 수업 개요를 제공하세요. 각 섹션에 대해 내용을 효과적으로 전달하고 학생들의 참여를 유도하기 위해 사용할 교수법, 학습 활동, 자료를 설명하세요. 마지막으로, 수업 목표에 대한 학생들의 이해와 숙달을 평가하기 위해 사용할 평가 방법을 설명하세요. 수업 계획서는 잘 정리되고 따라하기 쉬우며 능동적 학습과 비판적 사고를 촉진해야 합니다.",
        messages=[
            {
                "role": "user",
                "content": [
                    {
                        "type": "text",
                        "text": "주제: 광합성 입문  \n학년: 7학년 (12-13세)"
                    }
                ]
            }
        ]
    )
    print(message.content)
    
    ```
    
    
    ```typescript Vertex AI TypeScript
    import { AnthropicVertex } from '@anthropic-ai/vertex-sdk';
    
    // Reads from the `CLOUD_ML_REGION` & `ANTHROPIC_VERTEX_PROJECT_ID` environment variables.
    // Additionally goes through the standard `google-auth-library` flow.
    const client = new AnthropicVertex();
    
    const msg = await client.messages.create({
      model: "claude-sonnet-4@20250514",
      max_tokens: 4000,
      temperature: 0.5,
      system: "주어진 주제에 대해 포괄적이고 매력적이며 잘 구성된 수업 계획서를 작성하는 것이 당신의 임무입니다. 수업 계획서는 60분 수업 세션을 위해 설계되어야 하며 특정 학년 수준이나 연령대에 맞춰져야 합니다. 명확하고 측정 가능하며 관련 교육 표준에 부합하는 수업 목표를 명시하는 것부터 시작하세요. 다음으로, 수업을 도입부, 주요 활동, 결론으로 나누어 상세한 수업 개요를 제공하세요. 각 섹션에 대해 내용을 효과적으로 전달하고 학생들의 참여를 유도하기 위해 사용할 교수법, 학습 활동, 자료를 설명하세요. 마지막으로, 수업 목표에 대한 학생들의 이해와 숙달을 평가하기 위해 사용할 평가 방법을 설명하세요. 수업 계획서는 잘 정리되고 따라하기 쉬우며 능동적 학습과 비판적 사고를 촉진해야 합니다.",
      messages: [
        {
          "role": "user",
          "content": [
            {
              "type": "text",
              "text": "주제: 광합성 입문  \n학년: 7학년 (12-13세)"
            }
          ]
        }
      ]
    });
    console.log(msg);
    
    ```
</CodeGroup>