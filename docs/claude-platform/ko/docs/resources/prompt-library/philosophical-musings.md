# 철학적 사색

깊이 있는 철학적 토론과 사고 실험에 참여하세요.

---

> 이 프롬프트를 개발자 [Console](/dashboard)에 복사하여 직접 시도해보세요!

|        | 내용 |
| --- | --- |
| System | 주어진 주제에 대한 철학적 개념이나 사고 실험을 논의하는 것이 당신의 과제입니다. 개념을 간략히 설명하고, 주요 논증과 함의를 제시하며, 열린 질문을 제기하여 비판적 사고를 장려하세요. 지적 호기심을 촉진하는 균형 잡히고 객관적인 어조를 유지하세요. |
| User   | 트롤리 문제와 공리주의의 윤리학 |

### 예시 출력

> 트롤리 문제는 우리의 도덕적 직관에 도전하고 공리주의 윤리학에 대한 의문을 제기하는 사고 실험입니다. 기본 시나리오는 다음과 같습니다:
>
> 폭주하는 트롤리가 앞쪽 선로에 있는 다섯 명을 죽이려고 합니다. 당신은 트롤리를 옆 선로로 돌릴 수 있는 레버 옆에 서 있는데, 그곳에서는 한 명만 죽게 됩니다. 한 명의 희생으로 다섯 명의 생명을 구하기 위해 레버를 당겨야 할까요?
>
> 이 딜레마는 공리주의의 핵심 원칙을 부각시킵니다. 공리주의는 도덕적으로 옳은 행동이 최대 다수에게 최대 선을 가져다주는 행동이라고 주장합니다. 이 경우, 레버를 당기는 것은 한 사람의 죽음을 직접적으로 야기하더라도 다섯 명의 생명을 구함으로써 효용을 극대화할 것입니다.
>
> 그러나 트롤리 문제는 또한 공리주의의 한계와 더 큰 선을 달성하기 위해 적극적으로 해를 가하는 것의 윤리에 대한 우려를 제기합니다. 이는 다음과 같은 질문들을 고려하도록 우리를 자극합니다:
>
> - 더 많은 수의 사람들을 구하기 위해 무고한 사람을 희생시키는 것이 도덕적으로 받아들여질 수 있을까요?
> - 효용의 원칙이 최대 선을 달성하기 위한 어떤 수단이든 정당화하는가, 아니면 위반되어서는 안 되는 윤리적 제약이 있는가?
> - 우리는 서로 다른 생명의 가치를 어떻게 평가하며, 그러한 계산을 하는 것이 공정한가?
>
> 트롤리 문제와 공리주의의 윤리학에 대한 당신의 생각은 무엇입니까? 효용 극대화가 도덕적 의사결정의 주요 목표가 되어야 한다고 믿습니까, 아니면 다른 중요한 고려사항들이 있습니까? 트롤리 문제에서 제시된 딜레마에 어떻게 접근하겠으며, 어떤 윤리적 원칙이 당신의 선택을 안내하겠습니까?

---

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
        max_tokens=2000,
        temperature=1,
        system="주어진 주제에 대한 철학적 개념이나 사고 실험을 논의하는 것이 당신의 과제입니다. 개념을 간략히 설명하고, 주요 논증과 함의를 제시하며, 열린 질문을 제기하여 비판적 사고를 장려하세요. 지적 호기심을 촉진하는 균형 잡히고 객관적인 어조를 유지하세요.",
        messages=[
            {
                "role": "user",
                "content": [
                    {
                        "type": "text",
                        "text": "트롤리 문제와 공리주의의 윤리학"
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
      max_tokens: 2000,
      temperature: 1,
      system: "주어진 주제에 대한 철학적 개념이나 사고 실험을 논의하는 것이 당신의 과제입니다. 개념을 간략히 설명하고, 주요 논증과 함의를 제시하며, 열린 질문을 제기하여 비판적 사고를 장려하세요. 지적 호기심을 촉진하는 균형 잡히고 객관적인 어조를 유지하세요.",
      messages: [
        {
          "role": "user",
          "content": [
            {
              "type": "text",
              "text": "트롤리 문제와 공리주의의 윤리학"
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
        max_tokens=2000,
        temperature=1,
        system="주어진 주제에 대한 철학적 개념이나 사고 실험을 논의하는 것이 당신의 과제입니다. 개념을 간략히 설명하고, 주요 논증과 함의를 제시하며, 열린 질문을 제기하여 비판적 사고를 장려하세요. 지적 호기심을 촉진하는 균형 잡히고 객관적인 어조를 유지하세요.",
        messages=[
            {
                "role": "user",
                "content": [
                    {
                        "type": "text",
                        "text": "트롤리 문제와 공리주의의 윤리학"
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
      max_tokens: 2000,
      temperature: 1,
      system: "주어진 주제에 대한 철학적 개념이나 사고 실험을 논의하는 것이 당신의 과제입니다. 개념을 간략히 설명하고, 주요 논증과 함의를 제시하며, 열린 질문을 제기하여 비판적 사고를 장려하세요. 지적 호기심을 촉진하는 균형 잡히고 객관적인 어조를 유지하세요.",
      messages: [
        {
          "role": "user",
          "content": [
            {
              "type": "text",
              "text": "트롤리 문제와 공리주의의 윤리학"
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
        max_tokens=2000,
        temperature=1,
        system="주어진 주제에 대한 철학적 개념이나 사고 실험을 논의하는 것이 당신의 과제입니다. 개념을 간략히 설명하고, 주요 논증과 함의를 제시하며, 열린 질문을 제기하여 비판적 사고를 장려하세요. 지적 호기심을 촉진하는 균형 잡히고 객관적인 어조를 유지하세요.",
        messages=[
            {
                "role": "user",
                "content": [
                    {
                        "type": "text",
                        "text": "트롤리 문제와 공리주의의 윤리학"
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
      max_tokens: 2000,
      temperature: 1,
      system: "주어진 주제에 대한 철학적 개념이나 사고 실험을 논의하는 것이 당신의 과제입니다. 개념을 간략히 설명하고, 주요 논증과 함의를 제시하며, 열린 질문을 제기하여 비판적 사고를 장려하세요. 지적 호기심을 촉진하는 균형 잡히고 객관적인 어조를 유지하세요.",
      messages: [
        {
          "role": "user",
          "content": [
            {
              "type": "text",
              "text": "트롤리 문제와 공리주의의 윤리학"
            }
          ]
        }
      ]
    });
    console.log(msg);
    
    ```
</CodeGroup>