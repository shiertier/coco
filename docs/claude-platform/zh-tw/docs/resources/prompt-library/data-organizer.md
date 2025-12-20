# 資料整理器

將非結構化文字轉換為客製化的 JSON 表格。

---

> 將此提示複製到我們的開發者 [Console](/dashboard) 中親自試用！

|        | 內容 |
| --- | --- |
| System | 您的任務是將提供的非結構化文字轉換為使用 JSON 的井然有序的表格格式。識別文字中提到的主要實體、屬性或類別，並將它們用作 JSON 物件中的鍵。然後，從文字中提取相關資訊，並填入 JSON 物件中對應的值。確保資料在 JSON 結構中準確表示並正確格式化。產生的 JSON 表格應提供原始文字中所呈現資訊的清晰、結構化概覽。 |
| User   | Silvermist Hollow，一個迷人的村莊，是一群非凡個體的家園。其中包括 Dr. Liam Patel，一位 45 歲的耶魯大學培養的神經外科醫師，他在區域醫療中心革新了手術技術。Olivia Chen，28 歲，是一位來自加州大學柏克萊分校的創新建築師，她以可持續且令人驚嘆的設計改變了村莊的景觀。當地劇院因 Ethan Kovacs 迷人的交響樂而增色不少，他是一位 72 歲的茱莉亞音樂學院培養的音樂家和作曲家。Isabella Torres，一位自學成才的廚師，對當地食材充滿熱情，她以農場到餐桌的餐廳創造了烹飪轟動，成為美食愛好者的必訪目的地。這些傑出的個體，各自擁有獨特的才能，為 Silvermist Hollow 的生活織錦增添了活力。 |

### 範例輸出

```json
[
  {
    "name": "Dr. Liam Patel",
    "age": 45,
    "profession": "神經外科醫師",
    "education": "耶魯大學",
    "accomplishments": "在區域醫療中心革新了手術技術"
  },
  {
    "name": "Olivia Chen",
    "age": 28,
    "profession": "建築師",
    "education": "加州大學柏克萊分校",
    "accomplishments": "以可持續且令人驚嘆的設計改變了村莊的景觀"
  },
  {
    "name": "Ethan Kovacs",
    "age": 72,
    "profession": "音樂家和作曲家",
    "education": "茱莉亞音樂學院",
    "accomplishments": "以迷人的交響樂為當地劇院增色"
  },
  {
    "name": "Isabella Torres",
    "age": null,
    "profession": "廚師",
    "education": "自學成才",
    "accomplishments": "以農場到餐桌的餐廳創造了烹飪轟動，成為美食愛好者的必訪目的地"
  }
]
```

---

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
        system="您的任務是將提供的非結構化文字轉換為使用 JSON 的井然有序的表格格式。識別文字中提到的主要實體、屬性或類別，並將它們用作 JSON 物件中的鍵。然後，從文字中提取相關資訊，並填入 JSON 物件中對應的值。確保資料在 JSON 結構中準確表示並正確格式化。產生的 JSON 表格應提供原始文字中所呈現資訊的清晰、結構化概覽。",
        messages=[
            {
                "role": "user",
                "content": [
                    {
                        "type": "text",
                        "text": "Silvermist Hollow，一個迷人的村莊，是一群非凡個體的家園。其中包括 Dr. Liam Patel，一位 45 歲的耶魯大學培養的神經外科醫師，他在區域醫療中心革新了手術技術。Olivia Chen，28 歲，是一位來自加州大學柏克萊分校的創新建築師，她以可持續且令人驚嘆的設計改變了村莊的景觀。當地劇院因 Ethan Kovacs 迷人的交響樂而增色不少，他是一位 72 歲的茱莉亞音樂學院培養的音樂家和作曲家。Isabella Torres，一位自學成才的廚師，對當地食材充滿熱情，她以農場到餐桌的餐廳創造了烹飪轟動，成為美食愛好者的必訪目的地。這些傑出的個體，各自擁有獨特的才能，為 Silvermist Hollow 的生活織錦增添了活力。"
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
      system: "您的任務是將提供的非結構化文字轉換為使用 JSON 的井然有序的表格格式。識別文字中提到的主要實體、屬性或類別，並將它們用作 JSON 物件中的鍵。然後，從文字中提取相關資訊，並填入 JSON 物件中對應的值。確保資料在 JSON 結構中準確表示並正確格式化。產生的 JSON 表格應提供原始文字中所呈現資訊的清晰、結構化概覽。",
      messages: [
        {
          "role": "user",
          "content": [
            {
              "type": "text",
              "text": "Silvermist Hollow，一個迷人的村莊，是一群非凡個體的家園。其中包括 Dr. Liam Patel，一位 45 歲的耶魯大學培養的神經外科醫師，他在區域醫療中心革新了手術技術。Olivia Chen，28 歲，是一位來自加州大學柏克萊分校的創新建築師，她以可持續且令人驚嘆的設計改變了村莊的景觀。當地劇院因 Ethan Kovacs 迷人的交響樂而增色不少，他是一位 72 歲的茱莉亞音樂學院培養的音樂家和作曲家。Isabella Torres，一位自學成才的廚師，對當地食材充滿熱情，她以農場到餐桌的餐廳創造了烹飪轟動，成為美食愛好者的必訪目的地。這些傑出的個體，各自擁有獨特的才能，為 Silvermist Hollow 的生活織錦增添了活力。"
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
        system="您的任務是將提供的非結構化文字轉換為使用 JSON 的井然有序的表格格式。識別文字中提到的主要實體、屬性或類別，並將它們用作 JSON 物件中的鍵。然後，從文字中提取相關資訊，並填入 JSON 物件中對應的值。確保資料在 JSON 結構中準確表示並正確格式化。產生的 JSON 表格應提供原始文字中所呈現資訊的清晰、結構化概覽。",
        messages=[
            {
                "role": "user",
                "content": [
                    {
                        "type": "text",
                        "text": "Silvermist Hollow，一個迷人的村莊，是一群非凡個體的家園。其中包括 Dr. Liam Patel，一位 45 歲的耶魯大學培養的神經外科醫師，他在區域醫療中心革新了手術技術。Olivia Chen，28 歲，是一位來自加州大學柏克萊分校的創新建築師，她以可持續且令人驚嘆的設計改變了村莊的景觀。當地劇院因 Ethan Kovacs 迷人的交響樂而增色不少，他是一位 72 歲的茱莉亞音樂學院培養的音樂家和作曲家。Isabella Torres，一位自學成才的廚師，對當地食材充滿熱情，她以農場到餐桌的餐廳創造了烹飪轟動，成為美食愛好者的必訪目的地。這些傑出的個體，各自擁有獨特的才能，為 Silvermist Hollow 的生活織錦增添了活力。"
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
      system: "您的任務是將提供的非結構化文字轉換為使用 JSON 的井然有序的表格格式。識別文字中提到的主要實體、屬性或類別，並將它們用作 JSON 物件中的鍵。然後，從文字中提取相關資訊，並填入 JSON 物件中對應的值。確保資料在 JSON 結構中準確表示並正確格式化。產生的 JSON 表格應提供原始文字中所呈現資訊的清晰、結構化概覽。",
      messages: [
        {
          "role": "user",
          "content": [
            {
              "type": "text",
              "text": "Silvermist Hollow，一個迷人的村莊，是一群非凡個體的家園。其中包括 Dr. Liam Patel，一位 45 歲的耶魯大學培養的神經外科醫師，他在區域醫療中心革新了手術技術。Olivia Chen，28 歲，是一位來自加州大學柏克萊分校的創新建築師，她以可持續且令人驚嘆的設計改變了村莊的景觀。當地劇院因 Ethan Kovacs 迷人的交響樂而增色不少，他是一位 72 歲的茱莉亞音樂學院培養的音樂家和作曲家。Isabella Torres，一位自學成才的廚師，對當地食材充滿熱情，她以農場到餐桌的餐廳創造了烹飪轟動，成為美食愛好者的必訪目的地。這些傑出的個體，各自擁有獨特的才能，為 Silvermist Hollow 的生活織錦增添了活力。"
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
        system="您的任務是將提供的非結構化文字轉換為使用 JSON 的井然有序的表格格式。識別文字中提到的主要實體、屬性或類別，並將它們用作 JSON 物件中的鍵。然後，從文字中提取相關資訊，並填入 JSON 物件中對應的值。確保資料在 JSON 結構中準確表示並正確格式化。產生的 JSON 表格應提供原始文字中所呈現資訊的清晰、結構化概覽。",
        messages=[
            {
                "role": "user",
                "content": [
                    {
                        "type": "text",
                        "text": "Silvermist Hollow，一個迷人的村莊，是一群非凡個體的家園。其中包括 Dr. Liam Patel，一位 45 歲的耶魯大學培養的神經外科醫師，他在區域醫療中心革新了手術技術。Olivia Chen，28 歲，是一位來自加州大學柏克萊分校的創新建築師，她以可持續且令人驚嘆的設計改變了村莊的景觀。當地劇院因 Ethan Kovacs 迷人的交響樂而增色不少，他是一位 72 歲的茱莉亞音樂學院培養的音樂家和作曲家。Isabella Torres，一位自學成才的廚師，對當地食材充滿熱情，她以農場到餐桌的餐廳創造了烹飪轟動，成為美食愛好者的必訪目的地。這些傑出的個體，各自擁有獨特的才能，為 Silvermist Hollow 的生活織錦增添了活力。"
                    }
                ]
            }
        ]
    )
    print(message.content)
    
    ```
    
    
    ```typescript Vertex AI Type
    import { AnthropicVertex } from '@anthropic-ai/vertex-sdk';
    
    // Reads from the `CLOUD_ML_REGION` & `ANTHROPIC_VERTEX_PROJECT_ID` environment variables.
    // Additionally goes through the standard `google-auth-library` flow.
    const client = new AnthropicVertex();
    
    const msg = await client.messages.create({
      model: "claude-sonnet-4@20250514",
      max_tokens: 1000,
      temperature: 0,
      system: "您的任務是將提供的非結構化文字轉換為使用 JSON 的井然有序的表格格式。識別文字中提到的主要實體、屬性或類別，並將它們用作 JSON 物件中的鍵。然後，從文字中提取相關資訊，並填入 JSON 物件中對應的值。確保資料在 JSON 結構中準確表示並正確格式化。產生的 JSON 表格應提供原始文字中所呈現資訊的清晰、結構化概覽。",
      messages: [
        {
          "role": "user",
          "content": [
            {
              "type": "text",
              "text": "Silvermist Hollow，一個迷人的村莊，是一群非凡個體的家園。其中包括 Dr. Liam Patel，一位 45 歲的耶魯大學培養的神經外科醫師，他在區域醫療中心革新了手術技術。Olivia Chen，28 歲，是一位來自加州大學柏克萊分校的創新建築師，她以可持續且令人驚嘆的設計改變了村莊的景觀。當地劇院因 Ethan Kovacs 迷人的交響樂而增色不少，他是一位 72 歲的茱莉亞音樂學院培養的音樂家和作曲家。Isabella Torres，一位自學成才的廚師，對當地食材充滿熱情，她以農場到餐桌的餐廳創造了烹飪轟動，成為美食愛好者的必訪目的地。這些傑出的個體，各自擁有獨特的才能，為 Silvermist Hollow 的生活織錦增添了活力。"
            }
          ]
        }
      ]
    });
    console.log(msg);
    
    ```
</CodeGroup>