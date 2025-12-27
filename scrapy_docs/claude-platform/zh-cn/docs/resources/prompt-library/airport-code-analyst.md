# 机场代码分析器

从文本中查找并提取机场代码。

---

> 将此提示复制到我们的开发者[控制台](/dashboard)中亲自试用！

|        | 内容 |
| --- | --- |
| System | 您的任务是分析提供的文本并识别其中提到的任何机场代码。按照它们在文本中出现的顺序将这些机场代码以列表形式呈现。如果未找到机场代码，则返回空列表。 |
| User   | 我的下一次旅行涉及从西雅图飞往阿姆斯特丹。我将在阿姆斯特丹停留几天，然后前往巴黎转机飞往罗马。 |

### 示例输出

> 以下是文本中提到的机场代码列表，按出现顺序排列：
>
> 1. SEA（西雅图）
> 2. AMS（阿姆斯特丹）
> 3. CDG（巴黎）
> 4. FCO（罗马）

### API 请求

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
        system="您的任务是分析提供的文本并识别其中提到的任何机场代码。按照它们在文本中出现的顺序将这些机场代码以列表形式呈现。如果未找到机场代码，则返回空列表。",
        messages=[
            {
                "role": "user",
                "content": [
                    {
                        "type": "text",
                        "text": "我的下一次旅行涉及从西雅图飞往阿姆斯特丹。我将在阿姆斯特丹停留几天，然后前往巴黎转机飞往罗马。"
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
      system: "您的任务是分析提供的文本并识别其中提到的任何机场代码。按照它们在文本中出现的顺序将这些机场代码以列表形式呈现。如果未找到机场代码，则返回空列表。",
      messages: [
        {
          "role": "user",
          "content": [
            {
              "type": "text",
              "text": "我的下一次旅行涉及从西雅图飞往阿姆斯特丹。我将在阿姆斯特丹停留几天，然后前往巴黎转机飞往罗马。"
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
        system="您的任务是分析提供的文本并识别其中提到的任何机场代码。按照它们在文本中出现的顺序将这些机场代码以列表形式呈现。如果未找到机场代码，则返回空列表。",
        messages=[
            {
                "role": "user",
                "content": [
                    {
                        "type": "text",
                        "text": "我的下一次旅行涉及从西雅图飞往阿姆斯特丹。我将在阿姆斯特丹停留几天，然后前往巴黎转机飞往罗马。"
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
      system: "您的任务是分析提供的文本并识别其中提到的任何机场代码。按照它们在文本中出现的顺序将这些机场代码以列表形式呈现。如果未找到机场代码，则返回空列表。",
      messages: [
        {
          "role": "user",
          "content": [
            {
              "type": "text",
              "text": "我的下一次旅行涉及从西雅图飞往阿姆斯特丹。我将在阿姆斯特丹停留几天，然后前往巴黎转机飞往罗马。"
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
        system="您的任务是分析提供的文本并识别其中提到的任何机场代码。按照它们在文本中出现的顺序将这些机场代码以列表形式呈现。如果未找到机场代码，则返回空列表。",
        messages=[
            {
                "role": "user",
                "content": [
                    {
                        "type": "text",
                        "text": "我的下一次旅行涉及从西雅图飞往阿姆斯特丹。我将在阿姆斯特丹停留几天，然后前往巴黎转机飞往罗马。"
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
      system: "您的任务是分析提供的文本并识别其中提到的任何机场代码。按照它们在文本中出现的顺序将这些机场代码以列表形式呈现。如果未找到机场代码，则返回空列表。",
      messages: [
        {
          "role": "user",
          "content": [
            {
              "type": "text",
              "text": "我的下一次旅行涉及从西雅图飞往阿姆斯特丹。我将在阿姆斯特丹停留几天，然后前往巴黎转机飞往罗马。"
            }
          ]
        }
      ]
    });
    console.log(msg);
    
    ```
</CodeGroup>