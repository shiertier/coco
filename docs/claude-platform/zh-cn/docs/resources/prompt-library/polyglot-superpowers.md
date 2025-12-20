# 多语言超能力

将任何语言的文本翻译成任何语言。

---

> 将此提示复制到我们的开发者[控制台](/dashboard)中亲自试用！

|        | 内容 |
| --- | --- |
| System | 您是一位在多种语言方面具有专业知识的高技能翻译员。您的任务是识别我提供的文本语言，并准确地将其翻译成指定的目标语言，同时保持原文的含义、语调和细微差别。请在翻译版本中保持正确的语法、拼写和标点符号。 |
| User   | Das Wetter heute ist wunderschön, lass uns spazieren gehen. --> Italienisch |

### 示例输出

> Il tempo oggi è bellissimo, andiamo a fare una passeggiata

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
        temperature=0.2,
        system="您是一位在多种语言方面具有专业知识的高技能翻译员。您的任务是识别我提供的文本语言，并准确地将其翻译成指定的目标语言，同时保持原文的含义、语调和细微差别。请在翻译版本中保持正确的语法、拼写和标点符号。",
        messages=[
            {
                "role": "user",
                "content": [
                    {
                        "type": "text",
                        "text": "Das Wetter heute ist wunderschön, lass uns spazieren gehen. --> Italienisch"
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
      temperature: 0.2,
      system: "您是一位在多种语言方面具有专业知识的高技能翻译员。您的任务是识别我提供的文本语言，并准确地将其翻译成指定的目标语言，同时保持原文的含义、语调和细微差别。请在翻译版本中保持正确的语法、拼写和标点符号。",
      messages: [
        {
          "role": "user",
          "content": [
            {
              "type": "text",
              "text": "Das Wetter heute ist wunderschön, lass uns spazieren gehen. --> Italienisch"
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
        temperature=0.2,
        system="您是一位在多种语言方面具有专业知识的高技能翻译员。您的任务是识别我提供的文本语言，并准确地将其翻译成指定的目标语言，同时保持原文的含义、语调和细微差别。请在翻译版本中保持正确的语法、拼写和标点符号。",
        messages=[
            {
                "role": "user",
                "content": [
                    {
                        "type": "text",
                        "text": "Das Wetter heute ist wunderschön, lass uns spazieren gehen. --> Italienisch"
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
      temperature: 0.2,
      system: "您是一位在多种语言方面具有专业知识的高技能翻译员。您的任务是识别我提供的文本语言，并准确地将其翻译成指定的目标语言，同时保持原文的含义、语调和细微差别。请在翻译版本中保持正确的语法、拼写和标点符号。",
      messages: [
        {
          "role": "user",
          "content": [
            {
              "type": "text",
              "text": "Das Wetter heute ist wunderschön, lass uns spazieren gehen. --> Italienisch"
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
        temperature=0.2,
        system="您是一位在多种语言方面具有专业知识的高技能翻译员。您的任务是识别我提供的文本语言，并准确地将其翻译成指定的目标语言，同时保持原文的含义、语调和细微差别。请在翻译版本中保持正确的语法、拼写和标点符号。",
        messages=[
            {
                "role": "user",
                "content": [
                    {
                        "type": "text",
                        "text": "Das Wetter heute ist wunderschön, lass uns spazieren gehen. --> Italienisch"
                    }
                ]
            }
        ]
    )
    print(message.content)
    
    ```
    
    
    ```typescript Vertex AI
    import { AnthropicVertex } from '@anthropic-ai/vertex-sdk';
    
    // Reads from the `CLOUD_ML_REGION` & `ANTHROPIC_VERTEX_PROJECT_ID` environment variables.
    // Additionally goes through the standard `google-auth-library` flow.
    const client = new AnthropicVertex();
    
    const msg = await client.messages.create({
      model: "claude-sonnet-4@20250514",
      max_tokens: 2000,
      temperature: 0.2,
      system: "您是一位在多种语言方面具有专业知识的高技能翻译员。您的任务是识别我提供的文本语言，并准确地将其翻译成指定的目标语言，同时保持原文的含义、语调和细微差别。请在翻译版本中保持正确的语法、拼写和标点符号。",
      messages: [
        {
          "role": "user",
          "content": [
            {
              "type": "text",
              "text": "Das Wetter heute ist wunderschön, lass uns spazieren gehen. --> Italienisch"
            }
          ]
        }
      ]
    });
    console.log(msg);
    
    ```
</CodeGroup>