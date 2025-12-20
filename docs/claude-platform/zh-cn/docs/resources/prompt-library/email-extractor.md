# 邮箱地址提取器

从文档中提取邮箱地址并生成JSON格式的列表。

---

> 将此提示复制到我们的开发者[控制台](/dashboard)中亲自试用！

|        | 内容 |
| --- | --- |
| System | 精确复制以下文本中的任何邮箱地址，然后逐行写出。只有在输入文本中精确拼写出邮箱地址时才写出邮箱地址。如果文本中没有邮箱地址，请写"N/A"。不要说其他任何内容。 |
| User   | 电话目录：John Latrabe，555-232-1995，[john909709@geemail.com] Josie Lana，555-759-2905，[josie@josielananier.com] Keven Stevens，555-980-7000，[drkevin22@geemail.com] 电话目录将由HR经理保持更新。                      |

### 示例输出

> john909709@geemail.com > josie@josielananier.com > drkevin22@geemail.com

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
        system="精确复制以下文本中的任何邮箱地址，然后逐行写出。只有在输入文本中精确拼写出邮箱地址时才写出邮箱地址。如果文本中没有邮箱地址，请写\"N/A\"。不要说其他任何内容。",
        messages=[
            {
                "role": "user",
                "content": [
                    {
                        "type": "text",
                        "text": "电话目录：  \nJohn Latrabe，555-232-1995，[[email protected]]  \nJosie Lana，555-759-2905，[[email protected]]  \nKeven Stevens，555-980-7000，[[email protected]]  \n  \n电话目录将由HR经理保持更新。"
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
      system: "精确复制以下文本中的任何邮箱地址，然后逐行写出。只有在输入文本中精确拼写出邮箱地址时才写出邮箱地址。如果文本中没有邮箱地址，请写\"N/A\"。不要说其他任何内容。",
      messages: [
        {
          "role": "user",
          "content": [
            {
              "type": "text",
              "text": "电话目录：  \nJohn Latrabe，555-232-1995，[[email protected]]  \nJosie Lana，555-759-2905，[[email protected]]  \nKeven Stevens，555-980-7000，[[email protected]]  \n  \n电话目录将由HR经理保持更新。"
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
        system="精确复制以下文本中的任何邮箱地址，然后逐行写出。只有在输入文本中精确拼写出邮箱地址时才写出邮箱地址。如果文本中没有邮箱地址，请写\"N/A\"。不要说其他任何内容。",
        messages=[
            {
                "role": "user",
                "content": [
                    {
                        "type": "text",
                        "text": "电话目录：  \nJohn Latrabe，555-232-1995，[[email protected]]  \nJosie Lana，555-759-2905，[[email protected]]  \nKeven Stevens，555-980-7000，[[email protected]]  \n  \n电话目录将由HR经理保持更新。"
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
      system: "精确复制以下文本中的任何邮箱地址，然后逐行写出。只有在输入文本中精确拼写出邮箱地址时才写出邮箱地址。如果文本中没有邮箱地址，请写\"N/A\"。不要说其他任何内容。",
      messages: [
        {
          "role": "user",
          "content": [
            {
              "type": "text",
              "text": "电话目录：  \nJohn Latrabe，555-232-1995，[[email protected]]  \nJosie Lana，555-759-2905，[[email protected]]  \nKeven Stevens，555-980-7000，[[email protected]]  \n  \n电话目录将由HR经理保持更新。"
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
        system="精确复制以下文本中的任何邮箱地址，然后逐行写出。只有在输入文本中精确拼写出邮箱地址时才写出邮箱地址。如果文本中没有邮箱地址，请写\"N/A\"。不要说其他任何内容。",
        messages=[
            {
                "role": "user",
                "content": [
                    {
                        "type": "text",
                        "text": "电话目录：  \nJohn Latrabe，555-232-1995，[[email protected]]  \nJosie Lana，555-759-2905，[[email protected]]  \nKeven Stevens，555-980-7000，[[email protected]]  \n  \n电话目录将由HR经理保持更新。"
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
      system: "精确复制以下文本中的任何邮箱地址，然后逐行写出。只有在输入文本中精确拼写出邮箱地址时才写出邮箱地址。如果文本中没有邮箱地址，请写\"N/A\"。不要说其他任何内容。",
      messages: [
        {
          "role": "user",
          "content": [
            {
              "type": "text",
              "text": "电话目录：  \nJohn Latrabe，555-232-1995，[[email protected]]  \nJosie Lana，555-759-2905，[[email protected]]  \nKeven Stevens，555-980-7000，[[email protected]]  \n  \n电话目录将由HR经理保持更新。"
            }
          ]
        }
      ]
    });
    console.log(msg);
    
    ```
</CodeGroup>