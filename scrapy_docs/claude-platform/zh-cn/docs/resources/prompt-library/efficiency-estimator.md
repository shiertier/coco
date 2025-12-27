# 效率估算器

计算函数和算法的时间复杂度。

---

> 将此提示复制到我们的开发者[控制台](/dashboard)中亲自试用！

|        | 内容 |
| --- | --- |
| System | 您的任务是分析提供的函数或算法，并使用大O记号计算其时间复杂度。逐步解释您的推理过程，描述您如何得出最终的时间复杂度。在确定时间复杂度时考虑最坏情况。如果函数或算法包含多个步骤或嵌套循环，请提供每个步骤的时间复杂度，然后给出整个函数或算法的总体时间复杂度。假设使用的任何内置函数或操作的时间复杂度为O(1)，除非另有说明。 |
| User   | def example_function(n): for i in range(n): print(i) for j in range(n): for k in range(n): print(j, k) |

### 示例输出

> 为了分析给定函数`example_function(n)`的时间复杂度，我们将其分解为步骤并考虑最坏情况。
>
> 步骤1：分析第一个循环
>
> ```
> for i in range(n):
>    print(i)
>
> ```

> 此循环迭代`n`次，并在每次迭代中执行常数时间操作（`print(i)`）。因此，此步骤的时间复杂度为O(n)。
>
> 步骤2：分析嵌套循环
>
> ```
> for j in range(n):
>     for k in range(n):
>         print(j, k)
>
> ```
>
> 外层循环迭代`n`次，对于每次迭代，内层循环也迭代`n`次。在内层循环的每次迭代中，执行常数时间操作（`print(j, k)`）。因此，嵌套循环的时间复杂度为O(n \* n) = O(n^2)。
>
> 步骤3：合并时间复杂度  
> 函数`example_function(n)`由两个步骤组成：第一个循环和嵌套循环。总体时间复杂度是每个步骤时间复杂度的总和。
>
> 第一个循环的时间复杂度：O(n)  
> 嵌套循环的时间复杂度：O(n^2)
>
> 为了确定总体时间复杂度，我们考虑主导项，即增长阶数最高的项。在这种情况下，O(n^2)主导O(n)。
>
> 因此，函数`example_function(n)`的总体时间复杂度为O(n^2)。
>
> 在最坏情况下，当`n`很大时，嵌套循环将主导执行时间，函数将具有二次时间复杂度。

---

### API请求

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
        system="您的任务是分析提供的函数或算法，并使用大O记号计算其时间复杂度。逐步解释您的推理过程，描述您如何得出最终的时间复杂度。在确定时间复杂度时考虑最坏情况。如果函数或算法包含多个步骤或嵌套循环，请提供每个步骤的时间复杂度，然后给出整个函数或算法的总体时间复杂度。假设使用的任何内置函数或操作的时间复杂度为O(1)，除非另有说明。",
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
      system: "您的任务是分析提供的函数或算法，并使用大O记号计算其时间复杂度。逐步解释您的推理过程，描述您如何得出最终的时间复杂度。在确定时间复杂度时考虑最坏情况。如果函数或算法包含多个步骤或嵌套循环，请提供每个步骤的时间复杂度，然后给出整个函数或算法的总体时间复杂度。假设使用的任何内置函数或操作的时间复杂度为O(1)，除非另有说明。",
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
        system="您的任务是分析提供的函数或算法，并使用大O记号计算其时间复杂度。逐步解释您的推理过程，描述您如何得出最终的时间复杂度。在确定时间复杂度时考虑最坏情况。如果函数或算法包含多个步骤或嵌套循环，请提供每个步骤的时间复杂度，然后给出整个函数或算法的总体时间复杂度。假设使用的任何内置函数或操作的时间复杂度为O(1)，除非另有说明。",
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
    
    // See https://docs.claude.com/claude/reference/claude-on-amazon-bedrock
    // for authentication options
    const client = new AnthropicBedrock();
    
    const msg = await client.messages.create({
      model: "anthropic.claude-sonnet-4-5-20250929-v1:0",
      max_tokens: 1000,
      temperature: 0,
      system: "您的任务是分析提供的函数或算法，并使用大O记号计算其时间复杂度。逐步解释您的推理过程，描述您如何得出最终的时间复杂度。在确定时间复杂度时考虑最坏情况。如果函数或算法包含多个步骤或嵌套循环，请提供每个步骤的时间复杂度，然后给出整个函数或算法的总体时间复杂度。假设使用的任何内置函数或操作的时间复杂度为O(1)，除非另有说明。",
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
        system="您的任务是分析提供的函数或算法，并使用大O记号计算其时间复杂度。逐步解释您的推理过程，描述您如何得出最终的时间复杂度。在确定时间复杂度时考虑最坏情况。如果函数或算法包含多个步骤或嵌套循环，请提供每个步骤的时间复杂度，然后给出整个函数或算法的总体时间复杂度。假设使用的任何内置函数或操作的时间复杂度为O(1)，除非另有说明。",
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
      system: "您的任务是分析提供的函数或算法，并使用大O记号计算其时间复杂度。逐步解释您的推理过程，描述您如何得出最终的时间复杂度。在确定时间复杂度时考虑最坏情况。如果函数或算法包含多个步骤或嵌套循环，请提供每个步骤的时间复杂度，然后给出整个函数或算法的总体时间复杂度。假设使用的任何内置函数或操作的时间复杂度为O(1)，除非另有说明。",
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