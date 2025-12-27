# 方向解碼器

將自然語言轉換為逐步指示。

---

> 將此提示複製到我們的開發者 [Console](/login?selectAccount=true&returnTo=%2Fdashboard%3F) 中親自試用！

|        | 內容 |
| --- | --- |
| System | 您的任務是將提供的自然語言流程或任務描述轉換為清晰、簡潔的逐步指示，這些指示應該是邏輯性的、順序性的且易於遵循。使用祈使語氣，每個步驟都以動作動詞開始。提供必要的細節和解釋，以確保讀者能夠成功完成任務。如果原始描述不清楚、模糊或缺乏足夠的資訊，請要求澄清或額外的細節。 |
| User   | 要泡一杯茶，首先在水壺中煮一些水。在水加熱的同時，拿一個馬克杯並在其中放入一個茶包。一旦水沸騰，小心地將水倒入馬克杯中，倒在茶包上。讓茶浸泡幾分鐘，取決於您喜歡的濃度。浸泡後，取出茶包並根據口味添加牛奶和糖。將所有東西攪拌在一起，享受您的熱茶。 |

### 範例輸出

> 1. 在水壺中裝水並將其放在爐子上煮沸。
> 2. 在等待水沸騰時，選擇一個馬克杯並在其中放入一個茶包。
> 3. 一旦水達到沸點，小心地將熱水倒入馬克杯中，倒在茶包上。
> 4. 讓茶在熱水中浸泡所需的時間，通常為2-5分鐘，取決於偏好的濃度。
> 5. 浸泡時間結束後，從馬克杯中取出茶包並丟棄。
> 6. 根據您的口味偏好，在茶中添加牛奶和糖。
> 7. 使用湯匙將茶、牛奶和糖攪拌在一起，直到充分混合。
> 8. 您的茶現在可以享用了。趁熱飲用。

---

## API 請求

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
        system="您的任務是將提供的自然語言流程或任務描述轉換為清晰、簡潔的逐步指示，這些指示應該是邏輯性的、順序性的且易於遵循。使用祈使語氣，每個步驟都以動作動詞開始。提供必要的細節和解釋，以確保讀者能夠成功完成任務。如果原始描述不清楚、模糊或缺乏足夠的資訊，請要求澄清或額外的細節。",
        messages=[
            {
                "role": "user",
                "content": [
                    {
                        "type": "text",
                        "text": "要泡一杯茶，首先在水壺中煮一些水。在水加熱的同時，拿一個馬克杯並在其中放入一個茶包。一旦水沸騰，小心地將水倒入馬克杯中，倒在茶包上。讓茶浸泡幾分鐘，取決於您喜歡的濃度。浸泡後，取出茶包並根據口味添加牛奶和糖。將所有東西攪拌在一起，享受您的熱茶。"
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
      system: "您的任務是將提供的自然語言流程或任務描述轉換為清晰、簡潔的逐步指示，這些指示應該是邏輯性的、順序性的且易於遵循。使用祈使語氣，每個步驟都以動作動詞開始。提供必要的細節和解釋，以確保讀者能夠成功完成任務。如果原始描述不清楚、模糊或缺乏足夠的資訊，請要求澄清或額外的細節。",
      messages: [
        {
          "role": "user",
          "content": [
            {
              "type": "text",
              "text": "要泡一杯茶，首先在水壺中煮一些水。在水加熱的同時，拿一個馬克杯並在其中放入一個茶包。一旦水沸騰，小心地將水倒入馬克杯中，倒在茶包上。讓茶浸泡幾分鐘，取決於您喜歡的濃度。浸泡後，取出茶包並根據口味添加牛奶和糖。將所有東西攪拌在一起，享受您的熱茶。"
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
        system="您的任務是將提供的自然語言流程或任務描述轉換為清晰、簡潔的逐步指示，這些指示應該是邏輯性的、順序性的且易於遵循。使用祈使語氣，每個步驟都以動作動詞開始。提供必要的細節和解釋，以確保讀者能夠成功完成任務。如果原始描述不清楚、模糊或缺乏足夠的資訊，請要求澄清或額外的細節。",
        messages=[
            {
                "role": "user",
                "content": [
                    {
                        "type": "text",
                        "text": "要泡一杯茶，首先在水壺中煮一些水。在水加熱的同時，拿一個馬克杯並在其中放入一個茶包。一旦水沸騰，小心地將水倒入馬克杯中，倒在茶包上。讓茶浸泡幾分鐘，取決於您喜歡的濃度。浸泡後，取出茶包並根據口味添加牛奶和糖。將所有東西攪拌在一起，享受您的熱茶。"
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
      system: "您的任務是將提供的自然語言流程或任務描述轉換為清晰、簡潔的逐步指示，這些指示應該是邏輯性的、順序性的且易於遵循。使用祈使語氣，每個步驟都以動作動詞開始。提供必要的細節和解釋，以確保讀者能夠成功完成任務。如果原始描述不清楚、模糊或缺乏足夠的資訊，請要求澄清或額外的細節。",
      messages: [
        {
          "role": "user",
          "content": [
            {
              "type": "text",
              "text": "要泡一杯茶，首先在水壺中煮一些水。在水加熱的同時，拿一個馬克杯並在其中放入一個茶包。一旦水沸騰，小心地將水倒入馬克杯中，倒在茶包上。讓茶浸泡幾分鐘，取決於您喜歡的濃度。浸泡後，取出茶包並根據口味添加牛奶和糖。將所有東西攪拌在一起，享受您的熱茶。"
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
        system="您的任務是將提供的自然語言流程或任務描述轉換為清晰、簡潔的逐步指示，這些指示應該是邏輯性的、順序性的且易於遵循。使用祈使語氣，每個步驟都以動作動詞開始。提供必要的細節和解釋，以確保讀者能夠成功完成任務。如果原始描述不清楚、模糊或缺乏足夠的資訊，請要求澄清或額外的細節。",
        messages=[
            {
                "role": "user",
                "content": [
                    {
                        "type": "text",
                        "text": "要泡一杯茶，首先在水壺中煮一些水。在水加熱的同時，拿一個馬克杯並在其中放入一個茶包。一旦水沸騰，小心地將水倒入馬克杯中，倒在茶包上。讓茶浸泡幾分鐘，取決於您喜歡的濃度。浸泡後，取出茶包並根據口味添加牛奶和糖。將所有東西攪拌在一起，享受您的熱茶。"
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
      system: "您的任務是將提供的自然語言流程或任務描述轉換為清晰、簡潔的逐步指示，這些指示應該是邏輯性的、順序性的且易於遵循。使用祈使語氣，每個步驟都以動作動詞開始。提供必要的細節和解釋，以確保讀者能夠成功完成任務。如果原始描述不清楚、模糊或缺乏足夠的資訊，請要求澄清或額外的細節。",
      messages: [
        {
          "role": "user",
          "content": [
            {
              "type": "text",
              "text": "要泡一杯茶，首先在水壺中煮一些水。在水加熱的同時，拿一個馬克杯並在其中放入一個茶包。一旦水沸騰，小心地將水倒入馬克杯中，倒在茶包上。讓茶浸泡幾分鐘，取決於您喜歡的濃度。浸泡後，取出茶包並根據口味添加牛奶和糖。將所有東西攪拌在一起，享受您的熱茶。"
            }
          ]
        }
      ]
    });
    console.log(msg);
    
    ```
</CodeGroup>