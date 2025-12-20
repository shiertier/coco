# 방향 디코더

자연어를 단계별 지침으로 변환합니다.

---

> 이 프롬프트를 개발자 [Console](/login?selectAccount=true&returnTo=%2Fdashboard%3F)에 복사하여 직접 시도해보세요!

|        | 내용 |
| --- | --- |
| System | 당신의 작업은 제공된 프로세스나 작업의 자연어 설명을 논리적이고 순차적이며 따라하기 쉬운 명확하고 간결한 단계별 지침으로 변환하는 것입니다. 명령형 언어를 사용하고 각 단계를 동작 동사로 시작하세요. 독자가 작업을 성공적으로 완료할 수 있도록 필요한 세부 사항과 설명을 제공하세요. 원래 설명이 불분명하거나 모호하거나 충분한 정보가 부족한 경우 명확화나 추가 세부 사항을 요청하세요. |
| User   | 차 한 잔을 만들려면 주전자에 물을 끓이는 것부터 시작하세요. 물이 가열되는 동안 머그컵을 가져와서 티백을 넣으세요. 물이 끓으면 조심스럽게 머그컵에 티백 위로 물을 부으세요. 원하는 진한 정도에 따라 몇 분간 우려내세요. 우린 후 티백을 제거하고 취향에 따라 우유와 설탕을 넣으세요. 모든 것을 함께 저어서 뜨거운 차 한 잔을 즐기세요. |

### 예시 출력

> 1. 주전자에 물을 채우고 스토브에 올려 끓이세요.
> 2. 물이 끓기를 기다리는 동안 머그컵을 선택하고 그 안에 티백을 넣으세요.
> 3. 물이 끓으면 조심스럽게 뜨거운 물을 머그컵에 티백 위로 부으세요.
> 4. 원하는 시간 동안, 일반적으로 선호하는 진한 정도에 따라 2-5분간 뜨거운 물에 차를 우려내세요.
> 5. 우리는 시간이 지난 후 머그컵에서 티백을 제거하고 버리세요.
> 6. 취향에 따라 차에 우유와 설탕을 넣으세요.
> 7. 스푼을 사용하여 차, 우유, 설탕을 잘 섞일 때까지 저으세요.
> 8. 이제 차 한 잔이 즐길 준비가 되었습니다. 아직 뜨거울 때 마시세요.

---

## API 요청

<CodeGroup>
    ```python Python
    import anthropic
    
    client = anthropic.Anthropic(
        # defaults to os.environ.get("ANTHROPIC_API_KEY")
        api_key="my_api_key",
    )
    message = client.messages.create(
        model="claude-sonnet-4-5",
        max_tokens=1000,
        temperature=0,
        system="당신의 작업은 제공된 프로세스나 작업의 자연어 설명을 논리적이고 순차적이며 따라하기 쉬운 명확하고 간결한 단계별 지침으로 변환하는 것입니다. 명령형 언어를 사용하고 각 단계를 동작 동사로 시작하세요. 독자가 작업을 성공적으로 완료할 수 있도록 필요한 세부 사항과 설명을 제공하세요. 원래 설명이 불분명하거나 모호하거나 충분한 정보가 부족한 경우 명확화나 추가 세부 사항을 요청하세요.",
        messages=[
            {
                "role": "user",
                "content": [
                    {
                        "type": "text",
                        "text": "차 한 잔을 만들려면 주전자에 물을 끓이는 것부터 시작하세요. 물이 가열되는 동안 머그컵을 가져와서 티백을 넣으세요. 물이 끓으면 조심스럽게 머그컵에 티백 위로 물을 부으세요. 원하는 진한 정도에 따라 몇 분간 우려내세요. 우린 후 티백을 제거하고 취향에 따라 우유와 설탕을 넣으세요. 모든 것을 함께 저어서 뜨거운 차 한 잔을 즐기세요."
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
      max_tokens: 1000,
      temperature: 0,
      system: "당신의 작업은 제공된 프로세스나 작업의 자연어 설명을 논리적이고 순차적이며 따라하기 쉬운 명확하고 간결한 단계별 지침으로 변환하는 것입니다. 명령형 언어를 사용하고 각 단계를 동작 동사로 시작하세요. 독자가 작업을 성공적으로 완료할 수 있도록 필요한 세부 사항과 설명을 제공하세요. 원래 설명이 불분명하거나 모호하거나 충분한 정보가 부족한 경우 명확화나 추가 세부 사항을 요청하세요.",
      messages: [
        {
          "role": "user",
          "content": [
            {
              "type": "text",
              "text": "차 한 잔을 만들려면 주전자에 물을 끓이는 것부터 시작하세요. 물이 가열되는 동안 머그컵을 가져와서 티백을 넣으세요. 물이 끓으면 조심스럽게 머그컵에 티백 위로 물을 부으세요. 원하는 진한 정도에 따라 몇 분간 우려내세요. 우린 후 티백을 제거하고 취향에 따라 우유와 설탕을 넣으세요. 모든 것을 함께 저어서 뜨거운 차 한 잔을 즐기세요."
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
        max_tokens=1000,
        temperature=0,
        system="당신의 작업은 제공된 프로세스나 작업의 자연어 설명을 논리적이고 순차적이며 따라하기 쉬운 명확하고 간결한 단계별 지침으로 변환하는 것입니다. 명령형 언어를 사용하고 각 단계를 동작 동사로 시작하세요. 독자가 작업을 성공적으로 완료할 수 있도록 필요한 세부 사항과 설명을 제공하세요. 원래 설명이 불분명하거나 모호하거나 충분한 정보가 부족한 경우 명확화나 추가 세부 사항을 요청하세요.",
        messages=[
            {
                "role": "user",
                "content": [
                    {
                        "type": "text",
                        "text": "차 한 잔을 만들려면 주전자에 물을 끓이는 것부터 시작하세요. 물이 가열되는 동안 머그컵을 가져와서 티백을 넣으세요. 물이 끓으면 조심스럽게 머그컵에 티백 위로 물을 부으세요. 원하는 진한 정도에 따라 몇 분간 우려내세요. 우린 후 티백을 제거하고 취향에 따라 우유와 설탕을 넣으세요. 모든 것을 함께 저어서 뜨거운 차 한 잔을 즐기세요."
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
      max_tokens: 1000,
      temperature: 0,
      system: "당신의 작업은 제공된 프로세스나 작업의 자연어 설명을 논리적이고 순차적이며 따라하기 쉬운 명확하고 간결한 단계별 지침으로 변환하는 것입니다. 명령형 언어를 사용하고 각 단계를 동작 동사로 시작하세요. 독자가 작업을 성공적으로 완료할 수 있도록 필요한 세부 사항과 설명을 제공하세요. 원래 설명이 불분명하거나 모호하거나 충분한 정보가 부족한 경우 명확화나 추가 세부 사항을 요청하세요.",
      messages: [
        {
          "role": "user",
          "content": [
            {
              "type": "text",
              "text": "차 한 잔을 만들려면 주전자에 물을 끓이는 것부터 시작하세요. 물이 가열되는 동안 머그컵을 가져와서 티백을 넣으세요. 물이 끓으면 조심스럽게 머그컵에 티백 위로 물을 부으세요. 원하는 진한 정도에 따라 몇 분간 우려내세요. 우린 후 티백을 제거하고 취향에 따라 우유와 설탕을 넣으세요. 모든 것을 함께 저어서 뜨거운 차 한 잔을 즐기세요."
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
        max_tokens=1000,
        temperature=0,
        system="당신의 작업은 제공된 프로세스나 작업의 자연어 설명을 논리적이고 순차적이며 따라하기 쉬운 명확하고 간결한 단계별 지침으로 변환하는 것입니다. 명령형 언어를 사용하고 각 단계를 동작 동사로 시작하세요. 독자가 작업을 성공적으로 완료할 수 있도록 필요한 세부 사항과 설명을 제공하세요. 원래 설명이 불분명하거나 모호하거나 충분한 정보가 부족한 경우 명확화나 추가 세부 사항을 요청하세요.",
        messages=[
            {
                "role": "user",
                "content": [
                    {
                        "type": "text",
                        "text": "차 한 잔을 만들려면 주전자에 물을 끓이는 것부터 시작하세요. 물이 가열되는 동안 머그컵을 가져와서 티백을 넣으세요. 물이 끓으면 조심스럽게 머그컵에 티백 위로 물을 부으세요. 원하는 진한 정도에 따라 몇 분간 우려내세요. 우린 후 티백을 제거하고 취향에 따라 우유와 설탕을 넣으세요. 모든 것을 함께 저어서 뜨거운 차 한 잔을 즐기세요."
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
      max_tokens: 1000,
      temperature: 0,
      system: "당신의 작업은 제공된 프로세스나 작업의 자연어 설명을 논리적이고 순차적이며 따라하기 쉬운 명확하고 간결한 단계별 지침으로 변환하는 것입니다. 명령형 언어를 사용하고 각 단계를 동작 동사로 시작하세요. 독자가 작업을 성공적으로 완료할 수 있도록 필요한 세부 사항과 설명을 제공하세요. 원래 설명이 불분명하거나 모호하거나 충분한 정보가 부족한 경우 명확화나 추가 세부 사항을 요청하세요.",
      messages: [
        {
          "role": "user",
          "content": [
            {
              "type": "text",
              "text": "차 한 잔을 만들려면 주전자에 물을 끓이는 것부터 시작하세요. 물이 가열되는 동안 머그컵을 가져와서 티백을 넣으세요. 물이 끓으면 조심스럽게 머그컵에 티백 위로 물을 부으세요. 원하는 진한 정도에 따라 몇 분간 우려내세요. 우린 후 티백을 제거하고 취향에 따라 우유와 설탕을 넣으세요. 모든 것을 함께 저어서 뜨거운 차 한 잔을 즐기세요."
            }
          ]
        }
      ]
    });
    console.log(msg);
    
    ```
</CodeGroup>