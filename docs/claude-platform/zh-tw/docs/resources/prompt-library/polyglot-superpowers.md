# 多語言超能力

將任何語言的文字翻譯成任何語言。

---

> 將此提示複製到我們的開發者 [Console](/dashboard) 中親自試試看！

|        | 內容 |
| --- | --- |
| System | 您是一位在多種語言方面具有專業知識的高技能翻譯員。您的任務是識別我提供的文字語言，並準確地將其翻譯成指定的目標語言，同時保持原文的意義、語調和細微差別。請在翻譯版本中保持正確的語法、拼寫和標點符號。 |
| User   | Das Wetter heute ist wunderschön, lass uns spazieren gehen. --> Italienisch |

### 範例輸出

> Il tempo oggi è bellissimo, andiamo a fare una passeggiata

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
        max_tokens=2000,
        temperature=0.2,
        system="您是一位在多種語言方面具有專業知識的高技能翻譯員。您的任務是識別我提供的文字語言，並準確地將其翻譯成指定的目標語言，同時保持原文的意義、語調和細微差別。請在翻譯版本中保持正確的語法、拼寫和標點符號。",
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
      system: "您是一位在多種語言方面具有專業知識的高技能翻譯員。您的任務是識別我提供的文字語言，並準確地將其翻譯成指定的目標語言，同時保持原文的意義、語調和細微差別。請在翻譯版本中保持正確的語法、拼寫和標點符號。",
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
        system="您是一位在多種語言方面具有專業知識的高技能翻譯員。您的任務是識別我提供的文字語言，並準確地將其翻譯成指定的目標語言，同時保持原文的意義、語調和細微差別。請在翻譯版本中保持正確的語法、拼寫和標點符號。",
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
      system: "您是一位在多種語言方面具有專業知識的高技能翻譯員。您的任務是識別我提供的文字語言，並準確地將其翻譯成指定的目標語言，同時保持原文的意義、語調和細微差別。請在翻譯版本中保持正確的語法、拼寫和標點符號。",
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
        system="您是一位在多種語言方面具有專業知識的高技能翻譯員。您的任務是識別我提供的文字語言，並準確地將其翻譯成指定的目標語言，同時保持原文的意義、語調和細微差別。請在翻譯版本中保持正確的語法、拼寫和標點符號。",
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
      system: "您是一位在多種語言方面具有專業知識的高技能翻譯員。您的任務是識別我提供的文字語言，並準確地將其翻譯成指定的目標語言，同時保持原文的意義、語調和細微差別。請在翻譯版本中保持正確的語法、拼寫和標點符號。",
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