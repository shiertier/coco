# 機場代碼分析器

從文本中查找並提取機場代碼。

---

> 將此提示複製到我們的開發者 [Console](/dashboard) 中親自試用！

|        | 內容 |
| --- | --- |
| System | 您的任務是分析提供的文本並識別其中提到的任何機場代碼。按照這些機場代碼在文本中出現的順序將它們以列表形式呈現。如果沒有找到機場代碼，則返回空列表。 |
| User   | 我的下一次旅行涉及從西雅圖飛往阿姆斯特丹。我將在阿姆斯特丹停留幾天，然後前往巴黎轉機飛往羅馬。 |

### 示例輸出

> 以下是文本中提到的機場代碼列表，按出現順序排列：
>
> 1. SEA（西雅圖）
> 2. AMS（阿姆斯特丹）
> 3. CDG（巴黎）
> 4. FCO（羅馬）

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
        system="您的任務是分析提供的文本並識別其中提到的任何機場代碼。按照這些機場代碼在文本中出現的順序將它們以列表形式呈現。如果沒有找到機場代碼，則返回空列表。",
        messages=[
            {
                "role": "user",
                "content": [
                    {
                        "type": "text",
                        "text": "我的下一次旅行涉及從西雅圖飛往阿姆斯特丹。我將在阿姆斯特丹停留幾天，然後前往巴黎轉機飛往羅馬。"
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
      system: "您的任務是分析提供的文本並識別其中提到的任何機場代碼。按照這些機場代碼在文本中出現的順序將它們以列表形式呈現。如果沒有找到機場代碼，則返回空列表。",
      messages: [
        {
          "role": "user",
          "content": [
            {
              "type": "text",
              "text": "我的下一次旅行涉及從西雅圖飛往阿姆斯特丹。我將在阿姆斯特丹停留幾天，然後前往巴黎轉機飛往羅馬。"
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
        system="您的任務是分析提供的文本並識別其中提到的任何機場代碼。按照這些機場代碼在文本中出現的順序將它們以列表形式呈現。如果沒有找到機場代碼，則返回空列表。",
        messages=[
            {
                "role": "user",
                "content": [
                    {
                        "type": "text",
                        "text": "我的下一次旅行涉及從西雅圖飛往阿姆斯特丹。我將在阿姆斯特丹停留幾天，然後前往巴黎轉機飛往羅馬。"
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
      system: "您的任務是分析提供的文本並識別其中提到的任何機場代碼。按照這些機場代碼在文本中出現的順序將它們以列表形式呈現。如果沒有找到機場代碼，則返回空列表。",
      messages: [
        {
          "role": "user",
          "content": [
            {
              "type": "text",
              "text": "我的下一次旅行涉及從西雅圖飛往阿姆斯特丹。我將在阿姆斯特丹停留幾天，然後前往巴黎轉機飛往羅馬。"
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
        system="您的任務是分析提供的文本並識別其中提到的任何機場代碼。按照這些機場代碼在文本中出現的順序將它們以列表形式呈現。如果沒有找到機場代碼，則返回空列表。",
        messages=[
            {
                "role": "user",
                "content": [
                    {
                        "type": "text",
                        "text": "我的下一次旅行涉及從西雅圖飛往阿姆斯特丹。我將在阿姆斯特丹停留幾天，然後前往巴黎轉機飛往羅馬。"
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
      system: "您的任務是分析提供的文本並識別其中提到的任何機場代碼。按照這些機場代碼在文本中出現的順序將它們以列表形式呈現。如果沒有找到機場代碼，則返回空列表。",
      messages: [
        {
          "role": "user",
          "content": [
            {
              "type": "text",
              "text": "我的下一次旅行涉及從西雅圖飛往阿姆斯特丹。我將在阿姆斯特丹停留幾天，然後前往巴黎轉機飛往羅馬。"
            }
          ]
        }
      ]
    });
    console.log(msg);
    
    ```
</CodeGroup>