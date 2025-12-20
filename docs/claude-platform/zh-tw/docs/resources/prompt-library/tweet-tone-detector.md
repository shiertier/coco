# 推文語調檢測器

檢測推文背後的語調和情感。

---

> 將此提示複製到我們的開發者[控制台](/dashboard)中親自試用！

|        | 內容 |
| --- | --- |
| System | 您的任務是分析提供的推文並識別作者表達的主要語調和情感。語調應分類為以下之一：正面、負面、中性、幽默、諷刺、熱情、憤怒或資訊性。情感應分類為正面、負面或中性。請為您的分類提供簡要說明，突出影響您決定的關鍵詞、短語、表情符號或其他元素。 |
| User   | Wow, I'm so impressed by the company's handling of this crisis. 🙄 They really have their priorities straight. #sarcasm #fail |

### 範例輸出

> 語調：諷刺
> 情感：負面

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
        system="您的任務是分析提供的推文並識別作者表達的主要語調和情感。語調應分類為以下之一：正面、負面、中性、幽默、諷刺、熱情、憤怒或資訊性。情感應分類為正面、負面或中性。請為您的分類提供簡要說明，突出影響您決定的關鍵詞、短語、表情符號或其他元素。",
        messages=[
            {
                "role": "user",
                "content": [
                    {
                        "type": "text",
                        "text": "Wow, I'm so impressed by the company's handling of this crisis. 🙄 They really have their priorities straight. #sarcasm #fail"
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
      system: "您的任務是分析提供的推文並識別作者表達的主要語調和情感。語調應分類為以下之一：正面、負面、中性、幽默、諷刺、熱情、憤怒或資訊性。情感應分類為正面、負面或中性。請為您的分類提供簡要說明，突出影響您決定的關鍵詞、短語、表情符號或其他元素。",
      messages: [
        {
          "role": "user",
          "content": [
            {
              "type": "text",
              "text": "Wow, I'm so impressed by the company's handling of this crisis. 🙄 They really have their priorities straight. #sarcasm #fail"
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
        system="您的任務是分析提供的推文並識別作者表達的主要語調和情感。語調應分類為以下之一：正面、負面、中性、幽默、諷刺、熱情、憤怒或資訊性。情感應分類為正面、負面或中性。請為您的分類提供簡要說明，突出影響您決定的關鍵詞、短語、表情符號或其他元素。",
        messages=[
            {
                "role": "user",
                "content": [
                    {
                        "type": "text",
                        "text": "Wow, I'm so impressed by the company's handling of this crisis. 🙄 They really have their priorities straight. #sarcasm #fail"
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
      system: "您的任務是分析提供的推文並識別作者表達的主要語調和情感。語調應分類為以下之一：正面、負面、中性、幽默、諷刺、熱情、憤怒或資訊性。情感應分類為正面、負面或中性。請為您的分類提供簡要說明，突出影響您決定的關鍵詞、短語、表情符號或其他元素。",
      messages: [
        {
          "role": "user",
          "content": [
            {
              "type": "text",
              "text": "Wow, I'm so impressed by the company's handling of this crisis. 🙄 They really have their priorities straight. #sarcasm #fail"
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
        system="您的任務是分析提供的推文並識別作者表達的主要語調和情感。語調應分類為以下之一：正面、負面、中性、幽默、諷刺、熱情、憤怒或資訊性。情感應分類為正面、負面或中性。請為您的分類提供簡要說明，突出影響您決定的關鍵詞、短語、表情符號或其他元素。",
        messages=[
            {
                "role": "user",
                "content": [
                    {
                        "type": "text",
                        "text": "Wow, I'm so impressed by the company's handling of this crisis. 🙄 They really have their priorities straight. #sarcasm #fail"
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
      system: "您的任務是分析提供的推文並識別作者表達的主要語調和情感。語調應分類為以下之一：正面、負面、中性、幽默、諷刺、熱情、憤怒或資訊性。情感應分類為正面、負面或中性。請為您的分類提供簡要說明，突出影響您決定的關鍵詞、短語、表情符號或其他元素。",
      messages: [
        {
          "role": "user",
          "content": [
            {
              "type": "text",
              "text": "Wow, I'm so impressed by the company's handling of this crisis. 🙄 They really have their priorities straight. #sarcasm #fail"
            }
          ]
        }
      ]
    });
    console.log(msg);
    
    ```
</CodeGroup>