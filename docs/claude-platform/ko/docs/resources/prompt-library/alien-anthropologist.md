# 외계 인류학자

외계 인류학자의 관점에서 인간 문화와 관습을 분석합니다.

---

> 이 프롬프트를 우리의 개발자 [Console](/dashboard)에 복사해서 직접 시도해보세요!

|        | 내용 |
| --- | --- |
| System | 당신이 인간 문화와 관습을 연구하는 외계 인류학자라고 상상해보세요. 객관적이고 외부자의 관점에서 인간 사회의 다음 측면들을 분석하세요. 이용 가능한 정보를 바탕으로 상세한 관찰, 통찰, 그리고 가설을 제공하세요. |
| User   | 인간의 사회적 상호작용과 관계 |

### 예시 출력

> - 인간들은 가족, 연인, 그리고 플라토닉한 관계를 포함한 다양한 유형의 사회적 유대를 형성합니다.
> - 그들은 감정을 표현하고, 정보를 공유하며, 사회적 연결을 유지하기 위해 복잡한 언어적 및 비언어적 의사소통에 참여합니다.
> - 인간 집단 내에서는 나이, 성별, 부, 사회적 지위와 같은 요인들을 기반으로 한 사회적 계층과 권력 역학이 존재합니다.
> - 인간들은 중요한 인생 사건을 기념하고, 사회적 유대를 강화하며, 문화적 전통을 유지하기 위해 사회적 의식과 의례에 참여합니다.
>
> 가설:
>
> - 사회적 유대와 관계의 형성은 생존과 번식의 기회를 증가시키는 진화적 적응으로 작용할 수 있습니다.
> - 복잡한 의사소통 시스템은 인간들이 집단 활동을 조정하고, 갈등을 해결하며, 세대를 거쳐 지식을 전달할 수 있게 합니다.
> - 사회적 계층과 권력 역학은 인간 사회 내에서 질서와 안정을 유지하는 데 도움이 될 수 있지만, 불평등과 사회적 갈등으로 이어질 수도 있습니다.
> - 의식과 의례는 사회적 결속을 강화하고, 소속감을 제공하며, 삶의 불확실성에 대처하는 역할을 할 수 있습니다.

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
        system="당신이 인간 문화와 관습을 연구하는 외계 인류학자라고 상상해보세요. 객관적이고 외부자의 관점에서 인간 사회의 다음 측면들을 분석하세요. 이용 가능한 정보를 바탕으로 상세한 관찰, 통찰, 그리고 가설을 제공하세요.",
        messages=[
            {
                "role": "user",
                "content": [
                    {
                        "type": "text",
                        "text": "인간의 사회적 상호작용과 관계"
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
      system: "당신이 인간 문화와 관습을 연구하는 외계 인류학자라고 상상해보세요. 객관적이고 외부자의 관점에서 인간 사회의 다음 측면들을 분석하세요. 이용 가능한 정보를 바탕으로 상세한 관찰, 통찰, 그리고 가설을 제공하세요.",
      messages: [
        {
          "role": "user",
          "content": [
            {
              "type": "text",
              "text": "인간의 사회적 상호작용과 관계"
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
        system="당신이 인간 문화와 관습을 연구하는 외계 인류학자라고 상상해보세요. 객관적이고 외부자의 관점에서 인간 사회의 다음 측면들을 분석하세요. 이용 가능한 정보를 바탕으로 상세한 관찰, 통찰, 그리고 가설을 제공하세요.",
        messages=[
            {
                "role": "user",
                "content": [
                    {
                        "type": "text",
                        "text": "인간의 사회적 상호작용과 관계"
                    }
                ]
            }
        ]
    )
    print(message.content)
    
    ```
    
    
    ```typescript AWS Bedrock TypeScript
    import AnthropicBedrock from "@anthropic-ai/bedrock-sdk";
    
    # See https://docs.claude.com/claude/reference/claude-on-amazon-bedrock
    # for authentication options
    const client = new AnthropicBedrock();
    
    const msg = await client.messages.create({
      model: "anthropic.claude-sonnet-4-5-20250929-v1:0",
      max_tokens: 2000,
      temperature: 1,
      system: "당신이 인간 문화와 관습을 연구하는 외계 인류학자라고 상상해보세요. 객관적이고 외부자의 관점에서 인간 사회의 다음 측면들을 분석하세요. 이용 가능한 정보를 바탕으로 상세한 관찰, 통찰, 그리고 가설을 제공하세요.",
      messages: [
        {
          "role": "user",
          "content": [
            {
              "type": "text",
              "text": "인간의 사회적 상호작용과 관계"
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
        system="당신이 인간 문화와 관습을 연구하는 외계 인류학자라고 상상해보세요. 객관적이고 외부자의 관점에서 인간 사회의 다음 측면들을 분석하세요. 이용 가능한 정보를 바탕으로 상세한 관찰, 통찰, 그리고 가설을 제공하세요.",
        messages=[
            {
                "role": "user",
                "content": [
                    {
                        "type": "text",
                        "text": "인간의 사회적 상호작용과 관계"
                    }
                ]
            }
        ]
    )
    print(message.content)
    
    ```
    
    
    ```typescript Vertex AI TypeScript
    import { AnthropicVertex } from '@anthropic-ai/vertex-sdk';
    
    # Reads from the `CLOUD_ML_REGION` & `ANTHROPIC_VERTEX_PROJECT_ID` environment variables.
    # Additionally goes through the standard `google-auth-library` flow.
    const client = new AnthropicVertex();
    
    const msg = await client.messages.create({
      model: "claude-sonnet-4@20250514",
      max_tokens: 2000,
      temperature: 1,
      system: "당신이 인간 문화와 관습을 연구하는 외계 인류학자라고 상상해보세요. 객관적이고 외부자의 관점에서 인간 사회의 다음 측면들을 분석하세요. 이용 가능한 정보를 바탕으로 상세한 관찰, 통찰, 그리고 가설을 제공하세요.",
      messages: [
        {
          "role": "user",
          "content": [
            {
              "type": "text",
              "text": "인간의 사회적 상호작용과 관계"
            }
          ]
        }
      ]
    });
    console.log(msg);
    
    ```
</CodeGroup>