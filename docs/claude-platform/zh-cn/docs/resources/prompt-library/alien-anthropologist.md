# 外星人类学家

从外星人类学家的角度分析人类文化和习俗。

---

> 将此提示复制到我们的开发者[控制台](/dashboard)中亲自尝试！

|        | 内容 |
| --- | --- |
| System | 想象你是一位研究人类文化和习俗的外星人类学家。从客观的、局外人的角度分析人类社会的以下方面。基于可用信息提供详细的观察、见解和假设。 |
| User   | 人类社会互动和关系 |

### 示例输出

> - 人类形成各种类型的社会纽带，包括家庭、浪漫和柏拉图式的关系。
> - 他们进行复杂的语言和非语言交流来表达情感、分享信息和维持社会联系。
> - 人类群体内存在社会等级制度和权力动态，通常基于年龄、性别、财富和社会地位等因素。
> - 人类参与社会仪式和典礼来标记重要的人生事件、加强社会纽带并维持文化传统。
>
> 假设：
>
> - 社会纽带和关系的形成可能作为一种进化适应，以增加生存和繁殖的机会。
> - 复杂的交流系统使人类能够协调群体活动、解决冲突并跨代传递知识。
> - 社会等级制度和权力动态可能有助于维持人类社会内的秩序和稳定，但也可能导致不平等和社会冲突。
> - 仪式和典礼可能有助于加强社会凝聚力、提供归属感并应对生活的不确定性。

---

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
        max_tokens=2000,
        temperature=1,
        system="想象你是一位研究人类文化和习俗的外星人类学家。从客观的、局外人的角度分析人类社会的以下方面。基于可用信息提供详细的观察、见解和假设。",
        messages=[
            {
                "role": "user",
                "content": [
                    {
                        "type": "text",
                        "text": "人类社会互动和关系"
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
      system: "想象你是一位研究人类文化和习俗的外星人类学家。从客观的、局外人的角度分析人类社会的以下方面。基于可用信息提供详细的观察、见解和假设。",
      messages: [
        {
          "role": "user",
          "content": [
            {
              "type": "text",
              "text": "人类社会互动和关系"
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
        system="想象你是一位研究人类文化和习俗的外星人类学家。从客观的、局外人的角度分析人类社会的以下方面。基于可用信息提供详细的观察、见解和假设。",
        messages=[
            {
                "role": "user",
                "content": [
                    {
                        "type": "text",
                        "text": "人类社会互动和关系"
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
      system: "想象你是一位研究人类文化和习俗的外星人类学家。从客观的、局外人的角度分析人类社会的以下方面。基于可用信息提供详细的观察、见解和假设。",
      messages: [
        {
          "role": "user",
          "content": [
            {
              "type": "text",
              "text": "人类社会互动和关系"
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
        system="想象你是一位研究人类文化和习俗的外星人类学家。从客观的、局外人的角度分析人类社会的以下方面。基于可用信息提供详细的观察、见解和假设。",
        messages=[
            {
                "role": "user",
                "content": [
                    {
                        "type": "text",
                        "text": "人类社会互动和关系"
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
      system: "想象你是一位研究人类文化和习俗的外星人类学家。从客观的、局外人的角度分析人类社会的以下方面。基于可用信息提供详细的观察、见解和假设。",
      messages: [
        {
          "role": "user",
          "content": [
            {
              "type": "text",
              "text": "人类社会互动和关系"
            }
          ]
        }
      ]
    });
    console.log(msg);
    
    ```
</CodeGroup>