# 효율성 추정기

함수와 알고리즘의 시간 복잡도를 계산합니다.

---

> 이 프롬프트를 우리 개발자 [Console](/dashboard)에 복사해서 직접 시도해보세요!

|        | 내용 |
| --- | --- |
| System | 당신의 작업은 제공된 함수나 알고리즘을 분석하고 Big O 표기법을 사용하여 시간 복잡도를 계산하는 것입니다. 최종 시간 복잡도에 어떻게 도달했는지 단계별로 추론을 설명하세요. 시간 복잡도를 결정할 때 최악의 경우 시나리오를 고려하세요. 함수나 알고리즘이 여러 단계나 중첩된 루프를 포함하는 경우, 각 단계의 시간 복잡도를 제공한 다음 전체 함수나 알고리즘의 전체 시간 복잡도를 제공하세요. 달리 명시되지 않는 한 사용된 모든 내장 함수나 연산은 O(1)의 시간 복잡도를 가진다고 가정하세요. |
| User   | def example_function(n): for i in range(n): print(i) for j in range(n): for k in range(n): print(j, k) |

### 예시 출력

> 주어진 함수 `example_function(n)`의 시간 복잡도를 분석하기 위해, 단계별로 나누어 최악의 경우 시나리오를 고려하겠습니다.
>
> 1단계: 첫 번째 루프 분석
>
> ```
> for i in range(n):
>    print(i)
>
> ```

> 이 루프는 `n`번 반복되고 각 반복에서 상수 시간 연산(`print(i)`)을 수행합니다. 따라서 이 단계의 시간 복잡도는 O(n)입니다.
>
> 2단계: 중첩된 루프 분석
>
> ```
> for j in range(n):
>     for k in range(n):
>         print(j, k)
>
> ```
>
> 외부 루프는 `n`번 반복되고, 각 반복마다 내부 루프도 `n`번 반복됩니다. 내부 루프의 각 반복에서 상수 시간 연산(`print(j, k)`)이 수행됩니다. 따라서 중첩된 루프의 시간 복잡도는 O(n \* n) = O(n^2)입니다.
>
> 3단계: 시간 복잡도 결합  
> 함수 `example_function(n)`은 두 단계로 구성됩니다: 첫 번째 루프와 중첩된 루프. 전체 시간 복잡도는 각 단계의 시간 복잡도의 합입니다.
>
> 첫 번째 루프의 시간 복잡도: O(n)  
> 중첩된 루프의 시간 복잡도: O(n^2)
>
> 전체 시간 복잡도를 결정하기 위해, 가장 높은 성장 차수를 가진 항인 지배적인 항을 고려합니다. 이 경우, O(n^2)가 O(n)을 지배합니다.
>
> 따라서 함수 `example_function(n)`의 전체 시간 복잡도는 O(n^2)입니다.
>
> 최악의 경우 시나리오에서, `n`이 클 때, 중첩된 루프가 실행 시간을 지배하게 되고, 함수는 이차 시간 복잡도를 가지게 됩니다.

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
        max_tokens=1000,
        temperature=0,
        system="당신의 작업은 제공된 함수나 알고리즘을 분석하고 Big O 표기법을 사용하여 시간 복잡도를 계산하는 것입니다. 최종 시간 복잡도에 어떻게 도달했는지 단계별로 추론을 설명하세요. 시간 복잡도를 결정할 때 최악의 경우 시나리오를 고려하세요. 함수나 알고리즘이 여러 단계나 중첩된 루프를 포함하는 경우, 각 단계의 시간 복잡도를 제공한 다음 전체 함수나 알고리즘의 전체 시간 복잡도를 제공하세요. 달리 명시되지 않는 한 사용된 모든 내장 함수나 연산은 O(1)의 시간 복잡도를 가진다고 가정하세요.",
        messages=[
            {
                "role": "user",
                "content": [
                    {
                        "type": "text",
                        "text": "def example_function(n):\n    for i in range(n):\n        print(i)\n\n    for j in range(n):\n        for k in range(n):\n            print(j, k)"
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
      system: "당신의 작업은 제공된 함수나 알고리즘을 분석하고 Big O 표기법을 사용하여 시간 복잡도를 계산하는 것입니다. 최종 시간 복잡도에 어떻게 도달했는지 단계별로 추론을 설명하세요. 시간 복잡도를 결정할 때 최악의 경우 시나리오를 고려하세요. 함수나 알고리즘이 여러 단계나 중첩된 루프를 포함하는 경우, 각 단계의 시간 복잡도를 제공한 다음 전체 함수나 알고리즘의 전체 시간 복잡도를 제공하세요. 달리 명시되지 않는 한 사용된 모든 내장 함수나 연산은 O(1)의 시간 복잡도를 가진다고 가정하세요.",
      messages: [
        {
          "role": "user",
          "content": [
            {
              "type": "text",
              "text": "def example_function(n):\n    for i in range(n):\n        print(i)\n\n    for j in range(n):\n        for k in range(n):\n            print(j, k)"
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
        system="당신의 작업은 제공된 함수나 알고리즘을 분석하고 Big O 표기법을 사용하여 시간 복잡도를 계산하는 것입니다. 최종 시간 복잡도에 어떻게 도달했는지 단계별로 추론을 설명하세요. 시간 복잡도를 결정할 때 최악의 경우 시나리오를 고려하세요. 함수나 알고리즘이 여러 단계나 중첩된 루프를 포함하는 경우, 각 단계의 시간 복잡도를 제공한 다음 전체 함수나 알고리즘의 전체 시간 복잡도를 제공하세요. 달리 명시되지 않는 한 사용된 모든 내장 함수나 연산은 O(1)의 시간 복잡도를 가진다고 가정하세요.",
        messages=[
            {
                "role": "user",
                "content": [
                    {
                        "type": "text",
                        "text": "def example_function(n):\n    for i in range(n):\n        print(i)\n\n    for j in range(n):\n        for k in range(n):\n            print(j, k)"
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
      system: "당신의 작업은 제공된 함수나 알고리즘을 분석하고 Big O 표기법을 사용하여 시간 복잡도를 계산하는 것입니다. 최종 시간 복잡도에 어떻게 도달했는지 단계별로 추론을 설명하세요. 시간 복잡도를 결정할 때 최악의 경우 시나리오를 고려하세요. 함수나 알고리즘이 여러 단계나 중첩된 루프를 포함하는 경우, 각 단계의 시간 복잡도를 제공한 다음 전체 함수나 알고리즘의 전체 시간 복잡도를 제공하세요. 달리 명시되지 않는 한 사용된 모든 내장 함수나 연산은 O(1)의 시간 복잡도를 가진다고 가정하세요.",
      messages: [
        {
          "role": "user",
          "content": [
            {
              "type": "text",
              "text": "def example_function(n):\n    for i in range(n):\n        print(i)\n\n    for j in range(n):\n        for k in range(n):\n            print(j, k)"
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
        system="당신의 작업은 제공된 함수나 알고리즘을 분석하고 Big O 표기법을 사용하여 시간 복잡도를 계산하는 것입니다. 최종 시간 복잡도에 어떻게 도달했는지 단계별로 추론을 설명하세요. 시간 복잡도를 결정할 때 최악의 경우 시나리오를 고려하세요. 함수나 알고리즘이 여러 단계나 중첩된 루프를 포함하는 경우, 각 단계의 시간 복잡도를 제공한 다음 전체 함수나 알고리즘의 전체 시간 복잡도를 제공하세요. 달리 명시되지 않는 한 사용된 모든 내장 함수나 연산은 O(1)의 시간 복잡도를 가진다고 가정하세요.",
        messages=[
            {
                "role": "user",
                "content": [
                    {
                        "type": "text",
                        "text": "def example_function(n):\n    for i in range(n):\n        print(i)\n\n    for j in range(n):\n        for k in range(n):\n            print(j, k)"
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
      system: "당신의 작업은 제공된 함수나 알고리즘을 분석하고 Big O 표기법을 사용하여 시간 복잡도를 계산하는 것입니다. 최종 시간 복잡도에 어떻게 도달했는지 단계별로 추론을 설명하세요. 시간 복잡도를 결정할 때 최악의 경우 시나리오를 고려하세요. 함수나 알고리즘이 여러 단계나 중첩된 루프를 포함하는 경우, 각 단계의 시간 복잡도를 제공한 다음 전체 함수나 알고리즘의 전체 시간 복잡도를 제공하세요. 달리 명시되지 않는 한 사용된 모든 내장 함수나 연산은 O(1)의 시간 복잡도를 가진다고 가정하세요.",
      temperature: 0,
      messages: [
        {
          "role": "user",
          "content": [
            {
              "type": "text",
              "text": "def example_function(n):\n    for i in range(n):\n        print(i)\n\n    for j in range(n):\n        for k in range(n):\n            print(j, k)"
            }
          ]
        }
      ]
    });
    console.log(msg);
    
    ```
    
</CodeGroup>