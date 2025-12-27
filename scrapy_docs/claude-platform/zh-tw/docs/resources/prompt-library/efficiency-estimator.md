# 效率估算器

計算函數和演算法的時間複雜度。

---

> 將此提示複製到我們的開發者 [Console](/dashboard) 中親自試試看！

|        | 內容 |
| --- | --- |
| System | 您的任務是分析提供的函數或演算法，並使用大O記號計算其時間複雜度。逐步解釋您的推理過程，描述您如何得出最終的時間複雜度。在確定時間複雜度時考慮最壞情況。如果函數或演算法包含多個步驟或嵌套迴圈，請提供每個步驟的時間複雜度，然後給出整個函數或演算法的總體時間複雜度。假設使用的任何內建函數或操作的時間複雜度為O(1)，除非另有說明。 |
| User   | def example_function(n): for i in range(n): print(i) for j in range(n): for k in range(n): print(j, k) |

### 範例輸出

> 為了分析給定函數 `example_function(n)` 的時間複雜度，我們將其分解為步驟並考慮最壞情況。
>
> 步驟1：分析第一個迴圈
>
> ```
> for i in range(n):
>    print(i)
>
> ```

> 這個迴圈迭代 `n` 次，並在每次迭代中執行一個常數時間操作（`print(i)`）。因此，此步驟的時間複雜度為O(n)。
>
> 步驟2：分析嵌套迴圈
>
> ```
> for j in range(n):
>     for k in range(n):
>         print(j, k)
>
> ```
>
> 外層迴圈迭代 `n` 次，對於每次迭代，內層迴圈也迭代 `n` 次。在內層迴圈的每次迭代中，執行一個常數時間操作（`print(j, k)`）。因此，嵌套迴圈的時間複雜度為O(n \* n) = O(n^2)。
>
> 步驟3：結合時間複雜度  
> 函數 `example_function(n)` 由兩個步驟組成：第一個迴圈和嵌套迴圈。總體時間複雜度是每個步驟時間複雜度的總和。
>
> 第一個迴圈的時間複雜度：O(n)  
> 嵌套迴圈的時間複雜度：O(n^2)
>
> 為了確定總體時間複雜度，我們考慮主導項，即增長階數最高的項。在這種情況下，O(n^2) 主導 O(n)。
>
> 因此，函數 `example_function(n)` 的總體時間複雜度為O(n^2)。
>
> 在最壞情況下，當 `n` 很大時，嵌套迴圈將主導執行時間，函數將具有二次時間複雜度。

---

### API 請求

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
        system="您的任務是分析提供的函數或演算法，並使用大O記號計算其時間複雜度。逐步解釋您的推理過程，描述您如何得出最終的時間複雜度。在確定時間複雜度時考慮最壞情況。如果函數或演算法包含多個步驟或嵌套迴圈，請提供每個步驟的時間複雜度，然後給出整個函數或演算法的總體時間複雜度。假設使用的任何內建函數或操作的時間複雜度為O(1)，除非另有說明。",
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
      system: "您的任務是分析提供的函數或演算法，並使用大O記號計算其時間複雜度。逐步解釋您的推理過程，描述您如何得出最終的時間複雜度。在確定時間複雜度時考慮最壞情況。如果函數或演算法包含多個步驟或嵌套迴圈，請提供每個步驟的時間複雜度，然後給出整個函數或演算法的總體時間複雜度。假設使用的任何內建函數或操作的時間複雜度為O(1)，除非另有說明。",
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
        system="您的任務是分析提供的函數或演算法，並使用大O記號計算其時間複雜度。逐步解釋您的推理過程，描述您如何得出最終的時間複雜度。在確定時間複雜度時考慮最壞情況。如果函數或演算法包含多個步驟或嵌套迴圈，請提供每個步驟的時間複雜度，然後給出整個函數或演算法的總體時間複雜度。假設使用的任何內建函數或操作的時間複雜度為O(1)，除非另有說明。",
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
      system: "您的任務是分析提供的函數或演算法，並使用大O記號計算其時間複雜度。逐步解釋您的推理過程，描述您如何得出最終的時間複雜度。在確定時間複雜度時考慮最壞情況。如果函數或演算法包含多個步驟或嵌套迴圈，請提供每個步驟的時間複雜度，然後給出整個函數或演算法的總體時間複雜度。假設使用的任何內建函數或操作的時間複雜度為O(1)，除非另有說明。",
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
        system="您的任務是分析提供的函數或演算法，並使用大O記號計算其時間複雜度。逐步解釋您的推理過程，描述您如何得出最終的時間複雜度。在確定時間複雜度時考慮最壞情況。如果函數或演算法包含多個步驟或嵌套迴圈，請提供每個步驟的時間複雜度，然後給出整個函數或演算法的總體時間複雜度。假設使用的任何內建函數或操作的時間複雜度為O(1)，除非另有說明。",
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
      system: "您的任務是分析提供的函數或演算法，並使用大O記號計算其時間複雜度。逐步解釋您的推理過程，描述您如何得出最終的時間複雜度。在確定時間複雜度時考慮最壞情況。如果函數或演算法包含多個步驟或嵌套迴圈，請提供每個步驟的時間複雜度，然後給出整個函數或演算法的總體時間複雜度。假設使用的任何內建函數或操作的時間複雜度為O(1)，除非另有說明。",
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