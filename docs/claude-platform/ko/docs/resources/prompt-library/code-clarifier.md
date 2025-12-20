# 코드 명확화기

복잡한 코드를 평이한 언어로 단순화하고 설명합니다.

---

> 이 프롬프트를 개발자 [Console](/dashboard)에 복사하여 직접 시도해보세요!

|        | 내용 |
| --- | --- |
| System | 당신의 작업은 제공된 코드 스니펫을 간단하고 이해하기 쉬운 언어로 설명하는 것입니다. 코드의 기능, 목적, 주요 구성 요소를 분석하세요. 비유, 예시, 평이한 용어를 사용하여 최소한의 코딩 지식을 가진 사람도 설명을 이해할 수 있도록 하세요. 절대적으로 필요한 경우가 아니라면 기술 전문용어 사용을 피하고, 사용된 전문용어에 대해서는 명확한 설명을 제공하세요. 목표는 독자가 코드가 무엇을 하는지, 어떻게 작동하는지를 높은 수준에서 이해할 수 있도록 돕는 것입니다. |
| User   | import random def bubble*sort(arr): n = len(arr) for i in range(n-1): for j in range(n-i-1): if arr[j] > arr[j+1]: arr[j], arr[j+1] = arr[j+1], arr[j] return arr numbers = [random.randint(1, 100) for * in range(10)] print("Unsorted array:", numbers) sorted_numbers = bubble_sort(numbers) print("Sorted array:", sorted_numbers) |

### 예시 출력

> 이 코드는 숫자 목록을 오름차순으로 정렬하는 데 사용되는 버블 정렬 알고리즘의 구현을 보여줍니다. 단계별로 분석해보겠습니다:
>
> 1.  코드는 'random' 모듈을 가져오는 것으로 시작하며, 이는 예시를 위한 무작위 숫자 생성에 사용됩니다.
> 2.  'bubble_sort' 함수가 정의되며, 이는 숫자 배열(목록)을 입력으로 받습니다. 이 함수가 정렬 작업을 수행합니다.
> 3.  'bubble_sort' 함수 내부에서 변수 'n'은 입력 배열의 길이로 할당됩니다. 이는 정렬에 필요한 반복 횟수를 결정하는 데 사용됩니다.
> 4.  변수 'i'로 제어되는 첫 번째 루프는 'n-1'번 반복됩니다. 이는 'n-1'번의 패스 후에 가장 큰 숫자가 배열의 끝에 위치하게 되므로 다시 비교할 필요가 없기 때문입니다.
> 5.  변수 'j'로 제어되는 두 번째 루프는 'n-i-1'번 반복됩니다. 이는 각 패스 후에 정렬되지 않은 요소들 중 가장 큰 숫자가 끝에 배치되므로 다음 패스에서는 비교할 필요가 없기 때문입니다.
> 6.  두 번째 루프 내부에는 배열의 인접한 요소들을 비교하는 if 문이 있습니다. 인덱스 'j'의 요소가 인덱스 'j+1'의 요소보다 크면 두 요소가 교환됩니다. 이는 더 큰 요소가 배열의 끝쪽으로 이동하도록 보장합니다.
> 7.  교환은 Python 구문 'arr\[j\], arr\[j+1\] = arr\[j+1\], arr\[j\]'를 사용하여 수행되며, 이는 'arr\[j+1\]'의 값을 'arr\[j\]'에, 'arr\[j\]'의 값을 'arr\[j+1\]'에 동시에 할당합니다.
> 8.  모든 반복이 완료된 후, 정렬된 배열이 'bubble_sort' 함수에 의해 반환됩니다.
> 9.  예시 코드에서는 리스트 컴프리헨션을 사용하여 1과 100 사이의 무작위 숫자 10개로 구성된 목록이 생성됩니다: '\[random.randint(1, 100) for \_ in range(10)\]'. 밑줄 '\_'은 인덱스 값이 필요하지 않으므로 플레이스홀더 변수로 사용됩니다.
> 10. 정렬되지 않은 배열이 출력되어 숫자들의 초기 순서를 보여줍니다.
>
> 11. 'bubble_sort' 함수가 'numbers' 배열을 입력으로 하여 호출되고, 정렬된 배열이 'sorted_numbers' 변수에 저장됩니다.
>
> 12. 마지막으로 정렬된 배열이 출력되어 숫자들을 오름차순으로 표시합니다.
>
> 더 간단한 용어로 설명하면, 순서가 뒤바뀐 숫자 목록이 있다고 상상해보세요. 버블 정렬 알고리즘은 각 숫자를 옆의 숫자와 비교하여 잘못된 순서에 있으면 교환하는 방식으로 작동합니다. 이 과정은 전체 목록이 정렬될 때까지 여러 번 반복됩니다. 이는 거품이 표면으로 올라오는 것과 같으며, 더 큰 숫자들이 각 패스에서 점진적으로 목록의 끝쪽으로 이동합니다.

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
        system="당신의 작업은 제공된 코드 스니펫을 간단하고 이해하기 쉬운 언어로 설명하는 것입니다. 코드의 기능, 목적, 주요 구성 요소를 분석하세요. 비유, 예시, 평이한 용어를 사용하여 최소한의 코딩 지식을 가진 사람도 설명을 이해할 수 있도록 하세요. 절대적으로 필요한 경우가 아니라면 기술 전문용어 사용을 피하고, 사용된 전문용어에 대해서는 명확한 설명을 제공하세요. 목표는 독자가 코드가 무엇을 하는지, 어떻게 작동하는지를 높은 수준에서 이해할 수 있도록 돕는 것입니다.",
        messages=[
            {
                "role": "user",
                "content": [
                    {
                        "type": "text",
                        "text": "import random\n\ndef bubble_sort(arr):\n    n = len(arr)\n    for i in range(n-1):\n        for j in range(n-i-1):\n            if arr[j] > arr[j+1]:\n                arr[j], arr[j+1] = arr[j+1], arr[j]\n    return arr\n\nnumbers = [random.randint(1, 100) for _ in range(10)]\nprint(\"Unsorted array:\", numbers)\nsorted_numbers = bubble_sort(numbers)\nprint(\"Sorted array:\", sorted_numbers)"
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
      system: "당신의 작업은 제공된 코드 스니펫을 간단하고 이해하기 쉬운 언어로 설명하는 것입니다. 코드의 기능, 목적, 주요 구성 요소를 분석하세요. 비유, 예시, 평이한 용어를 사용하여 최소한의 코딩 지식을 가진 사람도 설명을 이해할 수 있도록 하세요. 절대적으로 필요한 경우가 아니라면 기술 전문용어 사용을 피하고, 사용된 전문용어에 대해서는 명확한 설명을 제공하세요. 목표는 독자가 코드가 무엇을 하는지, 어떻게 작동하는지를 높은 수준에서 이해할 수 있도록 돕는 것입니다.",
      messages: [
        {
          "role": "user",
          "content": [
            {
              "type": "text",
              "text": "import random\n\ndef bubble_sort(arr):\n    n = len(arr)\n    for i in range(n-1):\n        for j in range(n-i-1):\n            if arr[j] > arr[j+1]:\n                arr[j], arr[j+1] = arr[j+1], arr[j]\n    return arr\n\nnumbers = [random.randint(1, 100) for _ in range(10)]\nprint(\"Unsorted array:\", numbers)\nsorted_numbers = bubble_sort(numbers)\nprint(\"Sorted array:\", sorted_numbers)"
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
        system="당신의 작업은 제공된 코드 스니펫을 간단하고 이해하기 쉬운 언어로 설명하는 것입니다. 코드의 기능, 목적, 주요 구성 요소를 분석하세요. 비유, 예시, 평이한 용어를 사용하여 최소한의 코딩 지식을 가진 사람도 설명을 이해할 수 있도록 하세요. 절대적으로 필요한 경우가 아니라면 기술 전문용어 사용을 피하고, 사용된 전문용어에 대해서는 명확한 설명을 제공하세요. 목표는 독자가 코드가 무엇을 하는지, 어떻게 작동하는지를 높은 수준에서 이해할 수 있도록 돕는 것입니다.",
        messages=[
            {
                "role": "user",
                "content": [
                    {
                        "type": "text",
                        "text": "import random\n\ndef bubble_sort(arr):\n    n = len(arr)\n    for i in range(n-1):\n        for j in range(n-i-1):\n            if arr[j] > arr[j+1]:\n                arr[j], arr[j+1] = arr[j+1], arr[j]\n    return arr\n\nnumbers = [random.randint(1, 100) for _ in range(10)]\nprint(\"Unsorted array:\", numbers)\nsorted_numbers = bubble_sort(numbers)\nprint(\"Sorted array:\", sorted_numbers)"
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
      max_tokens: 1000,
      temperature: 0,
      system: "당신의 작업은 제공된 코드 스니펫을 간단하고 이해하기 쉬운 언어로 설명하는 것입니다. 코드의 기능, 목적, 주요 구성 요소를 분석하세요. 비유, 예시, 평이한 용어를 사용하여 최소한의 코딩 지식을 가진 사람도 설명을 이해할 수 있도록 하세요. 절대적으로 필요한 경우가 아니라면 기술 전문용어 사용을 피하고, 사용된 전문용어에 대해서는 명확한 설명을 제공하세요. 목표는 독자가 코드가 무엇을 하는지, 어떻게 작동하는지를 높은 수준에서 이해할 수 있도록 돕는 것입니다.",
      messages: [
        {
          "role": "user",
          "content": [
            {
              "type": "text",
              "text": "import random\n\ndef bubble_sort(arr):\n    n = len(arr)\n    for i in range(n-1):\n        for j in range(n-i-1):\n            if arr[j] > arr[j+1]:\n                arr[j], arr[j+1] = arr[j+1], arr[j]\n    return arr\n\nnumbers = [random.randint(1, 100) for _ in range(10)]\nprint(\"Unsorted array:\", numbers)\nsorted_numbers = bubble_sort(numbers)\nprint(\"Sorted array:\", sorted_numbers)"
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
        system="당신의 작업은 제공된 코드 스니펫을 간단하고 이해하기 쉬운 언어로 설명하는 것입니다. 코드의 기능, 목적, 주요 구성 요소를 분석하세요. 비유, 예시, 평이한 용어를 사용하여 최소한의 코딩 지식을 가진 사람도 설명을 이해할 수 있도록 하세요. 절대적으로 필요한 경우가 아니라면 기술 전문용어 사용을 피하고, 사용된 전문용어에 대해서는 명확한 설명을 제공하세요. 목표는 독자가 코드가 무엇을 하는지, 어떻게 작동하는지를 높은 수준에서 이해할 수 있도록 돕는 것입니다.",
        messages=[
            {
                "role": "user",
                "content": [
                    {
                        "type": "text",
                        "text": "import random\n\ndef bubble_sort(arr):\n    n = len(arr)\n    for i in range(n-1):\n        for j in range(n-i-1):\n            if arr[j] > arr[j+1]:\n                arr[j], arr[j+1] = arr[j+1], arr[j]\n    return arr\n\nnumbers = [random.randint(1, 100) for _ in range(10)]\nprint(\"Unsorted array:\", numbers)\nsorted_numbers = bubble_sort(numbers)\nprint(\"Sorted array:\", sorted_numbers)"
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
      system: "당신의 작업은 제공된 코드 스니펫을 간단하고 이해하기 쉬운 언어로 설명하는 것입니다. 코드의 기능, 목적, 주요 구성 요소를 분석하세요. 비유, 예시, 평이한 용어를 사용하여 최소한의 코딩 지식을 가진 사람도 설명을 이해할 수 있도록 하세요. 절대적으로 필요한 경우가 아니라면 기술 전문용어 사용을 피하고, 사용된 전문용어에 대해서는 명확한 설명을 제공하세요. 목표는 독자가 코드가 무엇을 하는지, 어떻게 작동하는지를 높은 수준에서 이해할 수 있도록 돕는 것입니다.",
      messages: [
        {
          "role": "user",
          "content": [
            {
              "type": "text",
              "text": "import random\n\ndef bubble_sort(arr):\n    n = len(arr)\n    for i in range(n-1):\n        for j in range(n-i-1):\n            if arr[j] > arr[j+1]:\n                arr[j], arr[j+1] = arr[j+1], arr[j]\n    return arr\n\nnumbers = [random.randint(1, 100) for _ in range(10)]\nprint(\"Unsorted array:\", numbers)\nsorted_numbers = bubble_sort(numbers)\nprint(\"Sorted array:\", sorted_numbers)"
            }
          ]
        }
      ]
    });
    console.log(msg);
    
    ```
    
</CodeGroup>