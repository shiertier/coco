# 頭韻煉金術師

為任何給定主題生成頭韻短語和句子。

---

> 將此提示複製到我們的開發者 [Console](/dashboard) 中親自試用！

|        | 內容 |
| --- | --- |
| System | 您的任務是為給定主題創建頭韻短語和句子。確保頭韻不僅聽起來悅耳，還能傳達相關信息或喚起與主題相關的適當情感。 |
| User   | 海洋 |

### 範例輸出

> 1.  輕聲細語的波浪沖刷著海岸。
> 2.  鹹鹹的海浪飛濺在沙灘上。
> 3.  衝擊的洋流雕刻著海岸懸崖。
> 4.  深邃、黑暗的深處隱藏著隱秘的危險。
> 5.  冒泡的小溪為海灘帶來生機。
> 6.  強大、雄偉的哺乳動物在水中遷徙。
> 7.  雷鳴般的潮汐將船隻拋來拋去。
> 8.  閃閃發光的淺灘庇護著神秘的海洋生物。
> 9.  波紋般的倒影揭示了水面下的世界。
> 10. 旋轉、洶湧的海洋塑造著海岸線。
> 11. 泡沫、起泡的浮冰自由流動。
> 12. 優雅的海鷗在閃閃發光的海洋上空滑翔。
> 13. 頑皮的海豚在純淨的水中跳躍。
> 14. 鹹味的微風吹過無邊的藍色。
> 15. 有節奏的漣漪向岩石海岸滾動。
> 16. 珊瑚洞穴隱藏著色彩繽紛的水生群落。
> 17. 暴風雨的天空和洶湧的海洋標誌著大自然的威力。
> 18. 翻滾的潮汐改變著寧靜的海灣。
> 19. 旋轉、呼嘯的風掀起波浪。
> 20. 寧靜、藍寶石般的海水延伸到地平線。

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
        max_tokens=1000,
        temperature=1,
        system="您的任務是為給定主題創建頭韻短語和句子。確保頭韻不僅聽起來悅耳，還能傳達相關信息或喚起與主題相關的適當情感。",
        messages=[
            {
                "role": "user",
                "content": [
                    {
                        "type": "text",
                        "text": "海洋"
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
      system: "您的任務是為給定主題創建頭韻短語和句子。確保頭韻不僅聽起來悅耳，還能傳達相關信息或喚起與主題相關的適當情感。",
      messages: [
        {
          "role": "user",
          "content": [
            {
              "type": "text",
              "text": "海洋"
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
        system="您的任務是為給定主題創建頭韻短語和句子。確保頭韻不僅聽起來悅耳，還能傳達相關信息或喚起與主題相關的適當情感。",
        messages=[
            {
                "role": "user",
                "content": [
                    {
                        "type": "text",
                        "text": "海洋"
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
      system: "您的任務是為給定主題創建頭韻短語和句子。確保頭韻不僅聽起來悅耳，還能傳達相關信息或喚起與主題相關的適當情感。",
      messages: [
        {
          "role": "user",
          "content": [
            {
              "type": "text",
              "text": "海洋"
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
        system="您的任務是為給定主題創建頭韻短語和句子。確保頭韻不僅聽起來悅耳，還能傳達相關信息或喚起與主題相關的適當情感。",
        messages=[
            {
                "role": "user",
                "content": [
                    {
                        "type": "text",
                        "text": "海洋"
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
      system: "您的任務是為給定主題創建頭韻短語和句子。確保頭韻不僅聽起來悅耳，還能傳達相關信息或喚起與主題相關的適當情感。",
      messages: [
        {
          "role": "user",
          "content": [
            {
              "type": "text",
              "text": "海洋"
            }
          ]
        }
      ]
    });
    console.log(msg);
    
    ```
</CodeGroup>