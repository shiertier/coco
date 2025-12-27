# 推文语调检测器

检测推文背后的语调和情感。

---

> 将此提示复制到我们的开发者[控制台](/dashboard)中亲自试用！

|        | 内容 |
| --- | --- |
| System | 您的任务是分析提供的推文并识别作者表达的主要语调和情感。语调应分类为以下之一：积极、消极、中性、幽默、讽刺、热情、愤怒或信息性。情感应分类为积极、消极或中性。为您的分类提供简要解释，突出影响您决定的关键词、短语、表情符号或其他元素。 |
| User   | 哇，我对公司处理这次危机的方式印象深刻。🙄 他们真的有正确的优先级。#sarcasm #fail |

### 示例输出

> 语调：讽刺
> 情感：消极

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
        system="您的任务是分析提供的推文并识别作者表达的主要语调和情感。语调应分类为以下之一：积极、消极、中性、幽默、讽刺、热情、愤怒或信息性。情感应分类为积极、消极或中性。为您的分类提供简要解释，突出影响您决定的关键词、短语、表情符号或其他元素。",
        messages=[
            {
                "role": "user",
                "content": [
                    {
                        "type": "text",
                        "text": "哇，我对公司处理这次危机的方式印象深刻。🙄 他们真的有正确的优先级。#sarcasm #fail"
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
      system: "您的任务是分析提供的推文并识别作者表达的主要语调和情感。语调应分类为以下之一：积极、消极、中性、幽默、讽刺、热情、愤怒或信息性。情感应分类为积极、消极或中性。为您的分类提供简要解释，突出影响您决定的关键词、短语、表情符号或其他元素。",
      messages: [
        {
          "role": "user",
          "content": [
            {
              "type": "text",
              "text": "哇，我对公司处理这次危机的方式印象深刻。🙄 他们真的有正确的优先级。#sarcasm #fail"
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
        system="您的任务是分析提供的推文并识别作者表达的主要语调和情感。语调应分类为以下之一：积极、消极、中性、幽默、讽刺、热情、愤怒或信息性。情感应分类为积极、消极或中性。为您的分类提供简要解释，突出影响您决定的关键词、短语、表情符号或其他元素。",
        messages=[
            {
                "role": "user",
                "content": [
                    {
                        "type": "text",
                        "text": "哇，我对公司处理这次危机的方式印象深刻。🙄 他们真的有正确的优先级。#sarcasm #fail"
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
      system: "您的任务是分析提供的推文并识别作者表达的主要语调和情感。语调应分类为以下之一：积极、消极、中性、幽默、讽刺、热情、愤怒或信息性。情感应分类为积极、消极或中性。为您的分类提供简要解释，突出影响您决定的关键词、短语、表情符号或其他元素。",
      messages: [
        {
          "role": "user",
          "content": [
            {
              "type": "text",
              "text": "哇，我对公司处理这次危机的方式印象深刻。🙄 他们真的有正确的优先级。#sarcasm #fail"
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
        system="您的任务是分析提供的推文并识别作者表达的主要语调和情感。语调应分类为以下之一：积极、消极、中性、幽默、讽刺、热情、愤怒或信息性。情感应分类为积极、消极或中性。为您的分类提供简要解释，突出影响您决定的关键词、短语、表情符号或其他元素。",
        messages=[
            {
                "role": "user",
                "content": [
                    {
                        "type": "text",
                        "text": "哇，我对公司处理这次危机的方式印象深刻。🙄 他们真的有正确的优先级。#sarcasm #fail"
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
      system: "您的任务是分析提供的推文并识别作者表达的主要语调和情感。语调应分类为以下之一：积极、消极、中性、幽默、讽刺、热情、愤怒或信息性。情感应分类为积极、消极或中性。为您的分类提供简要解释，突出影响您决定的关键词、短语、表情符号或其他元素。",
      messages: [
        {
          "role": "user",
          "content": [
            {
              "type": "text",
              "text": "哇，我对公司处理这次危机的方式印象深刻。🙄 他们真的有正确的优先级。#sarcasm #fail"
            }
          ]
        }
      ]
    });
    console.log(msg);
    
    ```
</CodeGroup>