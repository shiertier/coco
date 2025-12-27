# 品牌建構師

為整體品牌識別制定設計簡報。

---

> 將此提示複製到我們的開發者[控制台](/login?selectAccount=true&returnTo=%2Fdashboard%3F)中親自試用！

|        | 內容 |
| --- | --- |
| System | 您的任務是根據給定的規格為整體品牌識別創建一份全面的設計簡報。品牌識別應包含各種元素，如品牌名稱建議、標誌、色彩調色板、字體排版、視覺風格、語調和整體品牌個性。確保所有元素和諧地協同工作，創造出一個有凝聚力且令人難忘的品牌體驗，有效地向目標受眾傳達品牌的價值觀、使命和獨特賣點。請詳細且全面，並提供足夠的具體細節，讓某人能夠創造出真正獨特的品牌識別。 |
| User   | 品牌規格：這是一個專注於使用環保材料和道德生產方法創造高品質、時尚服裝和配飾的品牌。該品牌針對25-40歲重視時尚、可持續性和社會責任的環保意識消費者。品牌識別應實現以下目標：1. 反映品牌對可持續性、道德實踐和環境管理的承諾。2. 通過傳達風格、品質和時尚感來吸引目標受眾。3. 在可持續時尚市場中將品牌與競爭對手區分開來。4. 與消費者建立強烈的情感聯繫，並激勵他們做出更環保的選擇。 |

### 範例輸出

> 設計簡報：環保時尚品牌的品牌識別
> 品牌名稱建議："Sēnsus"（拉丁語中的"感覺"或"感知"，喚起對環境的正念和連結感）
>
> 標誌：
>
> - 一個簡約現代的標誌，結合風格化的葉子或植物圖案與時尚的無襯線字體。
> - 葉子或植物元素象徵品牌對可持續性和環境管理的承諾，而乾淨的字體排版傳達出精緻和風格感。
> - 標誌應該具有多功能性，在各種應用中都能良好運作，包括服裝標籤、包裝和數位平台。
>   色彩調色板：
>
> - 反映品牌環保理念的中性和大地色調色板，如綠色、米色和暖灰色的色調。
> - 深青色或芥末黃等強調色可以適度使用，以增加活力和深度。
> - 色彩調色板應喚起自然和諧與平衡感，同時仍然時尚前衛，吸引目標受眾。
>   字體排版：
>
> - 結合乾淨、現代的無襯線字體用於標題，以及互補的襯線字體用於正文。
> - 字體排版應該清晰易讀且視覺上吸引人，傳達品質和精緻感。
> - 一致使用字體層次和間距將創造出有凝聚力且有組織的品牌識別。
>   視覺風格：
>
> - 簡約且整潔的視覺風格，強調高品質的產品攝影和乾淨、現代的版面設計。
> - 融入自然元素，如植物圖案或紋理，以強化品牌的環保價值觀和與自然的連結。
> - 有效使用負空間創造開放和透氣感，反映品牌對可持續性的承諾。
>   語調：
>
> - 品牌的語調應該平易近人、知識豐富且真實。
> - 使用教育和激勵消費者了解可持續時尚和道德生產方法好處的語言。
> - 融入一絲精緻和風格，以吸引目標受眾的時尚意識敏感度。
>   品牌個性：
>
> - 環保意識和環境覺醒，但時尚且前衛。
> - 對推廣可持續實踐和道德選擇充滿信心和熱情。
> - 包容且歡迎，邀請消費者加入致力於做出積極改變的志同道合社群。
>   總體而言，"Sēnsus"的品牌識別應該在環保價值觀和時尚前衛美學之間取得平衡，吸引那些想要在做出環保負責選擇的同時看起來時尚的消費者。有凝聚力的視覺和語言元素應該與目標受眾建立強烈的情感聯繫，並激勵他們擁抱更可持續的生活方式。

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
        temperature=1,
        system="您的任務是根據給定的規格為整體品牌識別創建一份全面的設計簡報。品牌識別應包含各種元素，如品牌名稱建議、標誌、色彩調色板、字體排版、視覺風格、語調和整體品牌個性。確保所有元素和諧地協同工作，創造出一個有凝聚力且令人難忘的品牌體驗，有效地向目標受眾傳達品牌的價值觀、使命和獨特賣點。請詳細且全面，並提供足夠的具體細節，讓某人能夠創造出真正獨特的品牌識別。",
        messages=[
            {
                "role": "user",
                "content": [
                    {
                        "type": "text",
                        "text": "品牌規格：\n這是一個專注於使用環保材料和道德生產方法創造高品質、時尚服裝和配飾的品牌\n該品牌針對25-40歲重視時尚、可持續性和社會責任的環保意識消費者。\n品牌識別應實現以下目標：\n1. 反映品牌對可持續性、道德實踐和環境管理的承諾。\n2. 通過傳達風格、品質和時尚感來吸引目標受眾。\n3. 在可持續時尚市場中將品牌與競爭對手區分開來。\n4. 與消費者建立強烈的情感聯繫，並激勵他們做出更環保的選擇。"
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
      system: "您的任務是根據給定的規格為整體品牌識別創建一份全面的設計簡報。品牌識別應包含各種元素，如品牌名稱建議、標誌、色彩調色板、字體排版、視覺風格、語調和整體品牌個性。確保所有元素和諧地協同工作，創造出一個有凝聚力且令人難忘的品牌體驗，有效地向目標受眾傳達品牌的價值觀、使命和獨特賣點。請詳細且全面，並提供足夠的具體細節，讓某人能夠創造出真正獨特的品牌識別。",
      messages: [
        {
          "role": "user",
          "content": [
            {
              "type": "text",
              "text": "品牌規格：\n這是一個專注於使用環保材料和道德生產方法創造高品質、時尚服裝和配飾的品牌\n該品牌針對25-40歲重視時尚、可持續性和社會責任的環保意識消費者。\n品牌識別應實現以下目標：\n1. 反映品牌對可持續性、道德實踐和環境管理的承諾。\n2. 通過傳達風格、品質和時尚感來吸引目標受眾。\n3. 在可持續時尚市場中將品牌與競爭對手區分開來。\n4. 與消費者建立強烈的情感聯繫，並激勵他們做出更環保的選擇。"
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
        system="您的任務是根據給定的規格為整體品牌識別創建一份全面的設計簡報。品牌識別應包含各種元素，如品牌名稱建議、標誌、色彩調色板、字體排版、視覺風格、語調和整體品牌個性。確保所有元素和諧地協同工作，創造出一個有凝聚力且令人難忘的品牌體驗，有效地向目標受眾傳達品牌的價值觀、使命和獨特賣點。請詳細且全面，並提供足夠的具體細節，讓某人能夠創造出真正獨特的品牌識別。",
        messages=[
            {
                "role": "user",
                "content": [
                    {
                        "type": "text",
                        "text": "品牌規格：\n這是一個專注於使用環保材料和道德生產方法創造高品質、時尚服裝和配飾的品牌\n該品牌針對25-40歲重視時尚、可持續性和社會責任的環保意識消費者。\n品牌識別應實現以下目標：\n1. 反映品牌對可持續性、道德實踐和環境管理的承諾。\n2. 通過傳達風格、品質和時尚感來吸引目標受眾。\n3. 在可持續時尚市場中將品牌與競爭對手區分開來。\n4. 與消費者建立強烈的情感聯繫，並激勵他們做出更環保的選擇。"
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
      system: "您的任務是根據給定的規格為整體品牌識別創建一份全面的設計簡報。品牌識別應包含各種元素，如品牌名稱建議、標誌、色彩調色板、字體排版、視覺風格、語調和整體品牌個性。確保所有元素和諧地協同工作，創造出一個有凝聚力且令人難忘的品牌體驗，有效地向目標受眾傳達品牌的價值觀、使命和獨特賣點。請詳細且全面，並提供足夠的具體細節，讓某人能夠創造出真正獨特的品牌識別。",
      messages: [
        {
          "role": "user",
          "content": [
            {
              "type": "text",
              "text": "品牌規格：\n這是一個專注於使用環保材料和道德生產方法創造高品質、時尚服裝和配飾的品牌\n該品牌針對25-40歲重視時尚、可持續性和社會責任的環保意識消費者。\n品牌識別應實現以下目標：\n1. 反映品牌對可持續性、道德實踐和環境管理的承諾。\n2. 通過傳達風格、品質和時尚感來吸引目標受眾。\n3. 在可持續時尚市場中將品牌與競爭對手區分開來。\n4. 與消費者建立強烈的情感聯繫，並激勵他們做出更環保的選擇。"
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
        system="您的任務是根據給定的規格為整體品牌識別創建一份全面的設計簡報。品牌識別應包含各種元素，如品牌名稱建議、標誌、色彩調色板、字體排版、視覺風格、語調和整體品牌個性。確保所有元素和諧地協同工作，創造出一個有凝聚力且令人難忘的品牌體驗，有效地向目標受眾傳達品牌的價值觀、使命和獨特賣點。請詳細且全面，並提供足夠的具體細節，讓某人能夠創造出真正獨特的品牌識別。",
        messages=[
            {
                "role": "user",
                "content": [
                    {
                        "type": "text",
                        "text": "品牌規格：\n這是一個專注於使用環保材料和道德生產方法創造高品質、時尚服裝和配飾的品牌\n該品牌針對25-40歲重視時尚、可持續性和社會責任的環保意識消費者。\n品牌識別應實現以下目標：\n1. 反映品牌對可持續性、道德實踐和環境管理的承諾。\n2. 通過傳達風格、品質和時尚感來吸引目標受眾。\n3. 在可持續時尚市場中將品牌與競爭對手區分開來。\n4. 與消費者建立強烈的情感聯繫，並激勵他們做出更環保的選擇。"
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
      system: "您的任務是根據給定的規格為整體品牌識別創建一份全面的設計簡報。品牌識別應包含各種元素，如品牌名稱建議、標誌、色彩調色板、字體排版、視覺風格、語調和整體品牌個性。確保所有元素和諧地協同工作，創造出一個有凝聚力且令人難忘的品牌體驗，有效地向目標受眾傳達品牌的價值觀、使命和獨特賣點。請詳細且全面，並提供足夠的具體細節，讓某人能夠創造出真正獨特的品牌識別。",
      messages: [
        {
          "role": "user",
          "content": [
            {
              "type": "text",
              "text": "品牌規格：\n這是一個專注於使用環保材料和道德生產方法創造高品質、時尚服裝和配飾的品牌\n該品牌針對25-40歲重視時尚、可持續性和社會責任的環保意識消費者。\n品牌識別應實現以下目標：\n1. 反映品牌對可持續性、道德實踐和環境管理的承諾。\n2. 通過傳達風格、品質和時尚感來吸引目標受眾。\n3. 在可持續時尚市場中將品牌與競爭對手區分開來。\n4. 與消費者建立強烈的情感聯繫，並激勵他們做出更環保的選擇。"
            }
          ]
        }
      ]
    });
    console.log(msg);
    
    ```
</CodeGroup>