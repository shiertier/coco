# 課程規劃師

針對任何主題製作深入的課程計畫。

---

> 將此提示複製到我們的開發者 [Console](/login?selectAccount=true&returnTo=%2Fdashboard%3F) 中親自試用！

|        | 內容 |
| --- | --- |
| System | 您的任務是針對給定主題創建一個全面、引人入勝且結構良好的課程計畫。課程計畫應設計為60分鐘的課堂時間，並應迎合特定的年級水平或年齡組。首先陳述課程目標，這些目標應該清晰、可衡量，並與相關的教育標準保持一致。接下來，提供課程的詳細大綱，將其分解為引言、主要活動和結論。對於每個部分，描述您將使用的教學方法、學習活動和資源，以有效傳達內容並吸引學生參與。最後，描述您將採用的評估方法來評估學生對課程目標的理解和掌握程度。課程計畫應該組織良好、易於遵循，並促進主動學習和批判性思維。 |
| User   | 主題：光合作用簡介 年級水平：七年級（12-13歲） |

## 範例輸出

> 課程計畫：光合作用簡介
> 年級水平：七年級（12-13歲）
> 時長：60分鐘
>
> 課程目標：
> 在課程結束時，學生將能夠：
>
> 1. 定義光合作用並解釋其在生態系統中的重要性。
> 2. 識別光合作用發生所需的關鍵組成部分。
> 3. 描述光合作用的過程，包括光能、二氧化碳和水的作用。
> 4. 認識光合作用的產物及其對生物體的重要性。
>    課程大綱：
>
> 5. 引言（10分鐘）
>    i. 通過展示植物的短視頻片段或圖像並討論它們在環境中的作用來吸引學生。
>    ii. 介紹光合作用的概念及其在維持地球生命中的重要性。
>    iii. 陳述課程目標並提供課程結構概述。
> 6. 主要活動（40分鐘）
>    i. 指導教學（15分鐘）
>    a. 解釋光合作用所需的關鍵組成部分：陽光、二氧化碳、水和葉綠素。
>    b. 描述光合作用的過程，強調光能、二氧化碳和水的作用。
>    c. 討論光合作用的產物（葡萄糖和氧氣）及其對生物體的重要性。
>    ii. 小組活動（15分鐘）
>    a. 將班級分成小組，為每組提供工作表或圖形組織器。
>    b. 要求學生合作創建光合作用過程的視覺表示，標記關鍵組成部分和產物。
>    c. 鼓勵學生在小組內討論並解釋他們對光合作用的理解。
>    iii. 班級討論（10分鐘）
>    a. 邀請每組與全班分享他們的視覺表示。
>    b. 促進班級討論，鼓勵學生提問、提供反饋並澄清任何誤解。
> 7. 結論（10分鐘）
>    i. 總結課程的要點，強化光合作用的定義、組成部分、過程和產物。
>    ii. 進行簡短測驗或出場券來評估學生對課程目標的理解。
>    iii. 為學生提供家庭作業，進一步探索光合作用在不同生態系統中的重要性。
>    評估方法：
>
> 8. 形成性評估：在指導教學和小組活動期間觀察學生的參與度和理解程度。
> 9. 總結性評估：評估學生的視覺表示以及他們在簡短測驗或出場券上的表現。
> 10. 家庭作業：審查學生的家庭作業提交，以評估他們應用課程中所學概念的能力。
>     資源：
>
> 11. 植物及其在環境中作用的視頻片段或圖像
> 12. 小組活動的工作表或圖形組織器
> 13. 用於評估的簡短測驗或出場券
> 14. 家庭作業指導說明

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
        max_tokens=4000,
        temperature=0.5,
        system="您的任務是針對給定主題創建一個全面、引人入勝且結構良好的課程計畫。課程計畫應設計為60分鐘的課堂時間，並應迎合特定的年級水平或年齡組。首先陳述課程目標，這些目標應該清晰、可衡量，並與相關的教育標準保持一致。接下來，提供課程的詳細大綱，將其分解為引言、主要活動和結論。對於每個部分，描述您將使用的教學方法、學習活動和資源，以有效傳達內容並吸引學生參與。最後，描述您將採用的評估方法來評估學生對課程目標的理解和掌握程度。課程計畫應該組織良好、易於遵循，並促進主動學習和批判性思維。",
        messages=[
            {
                "role": "user",
                "content": [
                    {
                        "type": "text",
                        "text": "主題：光合作用簡介  \n年級水平：七年級（12-13歲）"
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
      max_tokens: 4000,
      temperature: 0.5,
      system: "您的任務是針對給定主題創建一個全面、引人入勝且結構良好的課程計畫。課程計畫應設計為60分鐘的課堂時間，並應迎合特定的年級水平或年齡組。首先陳述課程目標，這些目標應該清晰、可衡量，並與相關的教育標準保持一致。接下來，提供課程的詳細大綱，將其分解為引言、主要活動和結論。對於每個部分，描述您將使用的教學方法、學習活動和資源，以有效傳達內容並吸引學生參與。最後，描述您將採用的評估方法來評估學生對課程目標的理解和掌握程度。課程計畫應該組織良好、易於遵循，並促進主動學習和批判性思維。",
      messages: [
        {
          "role": "user",
          "content": [
            {
              "type": "text",
              "text": "主題：光合作用簡介  \n年級水平：七年級（12-13歲）"
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
        max_tokens=4000,
        temperature=0.5,
        system="您的任務是針對給定主題創建一個全面、引人入勝且結構良好的課程計畫。課程計畫應設計為60分鐘的課堂時間，並應迎合特定的年級水平或年齡組。首先陳述課程目標，這些目標應該清晰、可衡量，並與相關的教育標準保持一致。接下來，提供課程的詳細大綱，將其分解為引言、主要活動和結論。對於每個部分，描述您將使用的教學方法、學習活動和資源，以有效傳達內容並吸引學生參與。最後，描述您將採用的評估方法來評估學生對課程目標的理解和掌握程度。課程計畫應該組織良好、易於遵循，並促進主動學習和批判性思維。",
        messages=[
            {
                "role": "user",
                "content": [
                    {
                        "type": "text",
                        "text": "主題：光合作用簡介  \n年級水平：七年級（12-13歲）"
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
      max_tokens: 4000,
      temperature: 0.5,
      system: "您的任務是針對給定主題創建一個全面、引人入勝且結構良好的課程計畫。課程計畫應設計為60分鐘的課堂時間，並應迎合特定的年級水平或年齡組。首先陳述課程目標，這些目標應該清晰、可衡量，並與相關的教育標準保持一致。接下來，提供課程的詳細大綱，將其分解為引言、主要活動和結論。對於每個部分，描述您將使用的教學方法、學習活動和資源，以有效傳達內容並吸引學生參與。最後，描述您將採用的評估方法來評估學生對課程目標的理解和掌握程度。課程計畫應該組織良好、易於遵循，並促進主動學習和批判性思維。",
      messages: [
        {
          "role": "user",
          "content": [
            {
              "type": "text",
              "text": "主題：光合作用簡介  \n年級水平：七年級（12-13歲）"
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
        max_tokens=4000,
        temperature=0.5,
        system="您的任務是針對給定主題創建一個全面、引人入勝且結構良好的課程計畫。課程計畫應設計為60分鐘的課堂時間，並應迎合特定的年級水平或年齡組。首先陳述課程目標，這些目標應該清晰、可衡量，並與相關的教育標準保持一致。接下來，提供課程的詳細大綱，將其分解為引言、主要活動和結論。對於每個部分，描述您將使用的教學方法、學習活動和資源，以有效傳達內容並吸引學生參與。最後，描述您將採用的評估方法來評估學生對課程目標的理解和掌握程度。課程計畫應該組織良好、易於遵循，並促進主動學習和批判性思維。",
        messages=[
            {
                "role": "user",
                "content": [
                    {
                        "type": "text",
                        "text": "主題：光合作用簡介  \n年級水平：七年級（12-13歲）"
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
      max_tokens: 4000,
      temperature: 0.5,
      system: "您的任務是針對給定主題創建一個全面、引人入勝且結構良好的課程計畫。課程計畫應設計為60分鐘的課堂時間，並應迎合特定的年級水平或年齡組。首先陳述課程目標，這些目標應該清晰、可衡量，並與相關的教育標準保持一致。接下來，提供課程的詳細大綱，將其分解為引言、主要活動和結論。對於每個部分，描述您將使用的教學方法、學習活動和資源，以有效傳達內容並吸引學生參與。最後，描述您將採用的評估方法來評估學生對課程目標的理解和掌握程度。課程計畫應該組織良好、易於遵循，並促進主動學習和批判性思維。",
      messages: [
        {
          "role": "user",
          "content": [
            {
              "type": "text",
              "text": "主題：光合作用簡介  \n年級水平：七年級（12-13歲）"
            }
          ]
        }
      ]
    });
    console.log(msg);
    
    ```
</CodeGroup>