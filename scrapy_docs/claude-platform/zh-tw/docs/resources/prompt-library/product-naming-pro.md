# 產品命名專家

根據描述和關鍵字創造吸引人的產品名稱。

---

> 將此提示複製到我們的開發者 [Console](/dashboard) 來親自試用！

|        | 內容 |
| --- | --- |
| System | 您的任務是根據提供的描述和關鍵字生成創意、令人難忘且具有市場價值的產品名稱。產品名稱應該簡潔（2-4個詞），富有感染力，並且目標受眾容易理解。避免通用或過於字面的名稱。相反，應該創造一個突出、捕捉產品精髓並留下持久印象的名稱。 |
| User   | 描述：一款降噪、無線、頭戴式耳機，具有20小時電池續航和觸控功能。專為音響發燒友和經常旅行的人設計。關鍵字：沉浸式、舒適、高保真、持久、便利 |

## 範例輸出

> 1. SoundOasis Elite
>    2.AudioZen Nomad
>    3.Serenity Pro
>    4.Tranquility Touch
>    5.Harmonix Journey
>    6.SonicSolace Roam
>    7.Auditory Bliss
>    8.Quietude Quest
>    9.Euphony Excursion
>    10.Acoustica Wanderlust

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
        temperature=1,
        system="您的任務是根據提供的描述和關鍵字生成創意、令人難忘且具有市場價值的產品名稱。產品名稱應該簡潔（2-4個詞），富有感染力，並且目標受眾容易理解。避免通用或過於字面的名稱。相反，應該創造一個突出、捕捉產品精髓並留下持久印象的名稱。",
        messages=[
            {
                "role": "user",
                "content": [
                    {
                        "type": "text",
                        "text": "描述：一款降噪、無線、頭戴式耳機，具有20小時電池續航和觸控功能。專為音響發燒友和經常旅行的人設計。  \n  \n關鍵字：沉浸式、舒適、高保真、持久、便利"
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
      temperature: 1,
      system: "您的任務是根據提供的描述和關鍵字生成創意、令人難忘且具有市場價值的產品名稱。產品名稱應該簡潔（2-4個詞），富有感染力，並且目標受眾容易理解。避免通用或過於字面的名稱。相反，應該創造一個突出、捕捉產品精髓並留下持久印象的名稱。",
      messages: [
        {
          "role": "user",
          "content": [
            {
              "type": "text",
              "text": "描述：一款降噪、無線、頭戴式耳機，具有20小時電池續航和觸控功能。專為音響發燒友和經常旅行的人設計。  \n  \n關鍵字：沉浸式、舒適、高保真、持久、便利"
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
        temperature=1,
        system="您的任務是根據提供的描述和關鍵字生成創意、令人難忘且具有市場價值的產品名稱。產品名稱應該簡潔（2-4個詞），富有感染力，並且目標受眾容易理解。避免通用或過於字面的名稱。相反，應該創造一個突出、捕捉產品精髓並留下持久印象的名稱。",
        messages=[
            {
                "role": "user",
                "content": [
                    {
                        "type": "text",
                        "text": "描述：一款降噪、無線、頭戴式耳機，具有20小時電池續航和觸控功能。專為音響發燒友和經常旅行的人設計。  \n  \n關鍵字：沉浸式、舒適、高保真、持久、便利"
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
      temperature: 1,
      system: "您的任務是根據提供的描述和關鍵字生成創意、令人難忘且具有市場價值的產品名稱。產品名稱應該簡潔（2-4個詞），富有感染力，並且目標受眾容易理解。避免通用或過於字面的名稱。相反，應該創造一個突出、捕捉產品精髓並留下持久印象的名稱。",
      messages: [
        {
          "role": "user",
          "content": [
            {
              "type": "text",
              "text": "描述：一款降噪、無線、頭戴式耳機，具有20小時電池續航和觸控功能。專為音響發燒友和經常旅行的人設計。  \n  \n關鍵字：沉浸式、舒適、高保真、持久、便利"
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
        temperature=1,
        system="您的任務是根據提供的描述和關鍵字生成創意、令人難忘且具有市場價值的產品名稱。產品名稱應該簡潔（2-4個詞），富有感染力，並且目標受眾容易理解。避免通用或過於字面的名稱。相反，應該創造一個突出、捕捉產品精髓並留下持久印象的名稱。",
        messages=[
            {
                "role": "user",
                "content": [
                    {
                        "type": "text",
                        "text": "描述：一款降噪、無線、頭戴式耳機，具有20小時電池續航和觸控功能。專為音響發燒友和經常旅行的人設計。\n\n關鍵字：沉浸式、舒適、高保真、持久、便利"
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
      temperature: 1,
      system: "您的任務是根據提供的描述和關鍵字生成創意、令人難忘且具有市場價值的產品名稱。產品名稱應該簡潔（2-4個詞），富有感染力，並且目標受眾容易理解。避免通用或過於字面的名稱。相反，應該創造一個突出、捕捉產品精髓並留下持久印象的名稱。",
      messages: [
        {
          "role": "user",
          "content": [
            {
              "type": "text",
              "text": "描述：一款降噪、無線、頭戴式耳機，具有20小時電池續航和觸控功能。專為音響發燒友和經常旅行的人設計。\n\n關鍵字：沉浸式、舒適、高保真、持久、便利"
            }
          ]
        }
      ]
    });
    console.log(msg);
    
    ```
</CodeGroup>