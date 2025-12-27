# 試算表魔法師

生成包含各種類型資料的 CSV 試算表。

---

> 將此提示複製到我們的開發者 [Console](/dashboard) 中親自試用！

|        | 內容 |
| --- | --- |
| System | 您的任務是生成包含指定類型資料的 CSV 試算表。試算表應該組織良好，具有清晰的欄位標題和每個欄位適當的資料類型。確保資料真實、多樣化且格式一致。包含至少 10 行資料，不計算標題行。 |
| User   | 建立一個包含圖書館書籍資訊的試算表。包含以下欄位：書籍 ID、標題、作者、類型、出版年份和可用副本數。確保資料涵蓋各種類型和出版年份，且可用副本數在書籍之間有所不同。                             |

### 範例輸出

> 以下是根據您的要求包含圖書館書籍資訊的 CSV 試算表：
>
> Book ID,Title,Author,Genre,Publication Year,Available Copies  
> 1,To Kill a Mockingbird,Harper Lee,Classic,1960,5  
> 2,1984,George Orwell,Dystopian,1949,3  
> 3,Pride and Prejudice,Jane Austen,Romance,1813,7  
> 4,The Catcher in the Rye,J.D. Salinger,Coming-of-age,1951,2  
> 5,The Hitchhiker's Guide to the Galaxy,Douglas Adams,Science Fiction,1979,4  
> 6,The Da Vinci Code,Dan Brown,Mystery,2003,6  
> 7,The Kite Runner,Khaled Hosseini,Historical Fiction,2003,3  
> 8,The Alchemist,Paulo Coelho,Philosophical Fiction,1988,5  
> 9,The Hunger Games,Suzanne Collins,Young Adult,2008,8  
> 10,The Girl with the Dragon Tattoo,Stieg Larsson,Crime,2005,4  
> 11,The Hobbit,J.R.R. Tolkien,Fantasy,1937,6  
> 12,The Diary of a Young Girl,Anne Frank,Autobiography,1947,2  
> 13,The Fault in Our Stars,John Green,Young Adult,2012,5  
> 14,The Catcher in the Rye,J.D. Salinger,Coming-of-age,1951,3  
> 15,The Curious Incident of the Dog in the Night-Time,Mark Haddon,Mystery,2003,4

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
        temperature=0,
        system="您的任務是生成包含指定類型資料的 CSV 試算表。試算表應該組織良好，具有清晰的欄位標題和每個欄位適當的資料類型。確保資料真實、多樣化且格式一致。包含至少 10 行資料，不計算標題行。",
        messages=[
            {
                "role": "user",
                "content": [
                    {
                        "type": "text",
                        "text": "建立一個包含圖書館書籍資訊的試算表。包含以下欄位：書籍 ID、標題、作者、類型、出版年份和可用副本數。確保資料涵蓋各種類型和出版年份，且可用副本數在書籍之間有所不同。"
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
      system: "您的任務是生成包含指定類型資料的 CSV 試算表。試算表應該組織良好，具有清晰的欄位標題和每個欄位適當的資料類型。確保資料真實、多樣化且格式一致。包含至少 10 行資料，不計算標題行。",
      messages: [
        {
          "role": "user",
          "content": [
            {
              "type": "text",
              "text": "建立一個包含圖書館書籍資訊的試算表。包含以下欄位：書籍 ID、標題、作者、類型、出版年份和可用副本數。確保資料涵蓋各種類型和出版年份，且可用副本數在書籍之間有所不同。"
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
        system="您的任務是生成包含指定類型資料的 CSV 試算表。試算表應該組織良好，具有清晰的欄位標題和每個欄位適當的資料類型。確保資料真實、多樣化且格式一致。包含至少 10 行資料，不計算標題行。",
        messages=[
            {
                "role": "user",
                "content": [
                    {
                        "type": "text",
                        "text": "建立一個包含圖書館書籍資訊的試算表。包含以下欄位：書籍 ID、標題、作者、類型、出版年份和可用副本數。確保資料涵蓋各種類型和出版年份，且可用副本數在書籍之間有所不同。"
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
      system: "您的任務是生成包含指定類型資料的 CSV 試算表。試算表應該組織良好，具有清晰的欄位標題和每個欄位適當的資料類型。確保資料真實、多樣化且格式一致。包含至少 10 行資料，不計算標題行。",
      messages: [
        {
          "role": "user",
          "content": [
            {
              "type": "text",
              "text": "建立一個包含圖書館書籍資訊的試算表。包含以下欄位：書籍 ID、標題、作者、類型、出版年份和可用副本數。確保資料涵蓋各種類型和出版年份，且可用副本數在書籍之間有所不同。"
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
        system="您的任務是生成包含指定類型資料的 CSV 試算表。試算表應該組織良好，具有清晰的欄位標題和每個欄位適當的資料類型。確保資料真實、多樣化且格式一致。包含至少 10 行資料，不計算標題行。",
        messages=[
            {
                "role": "user",
                "content": [
                    {
                        "type": "text",
                        "text": "建立一個包含圖書館書籍資訊的試算表。包含以下欄位：書籍 ID、標題、作者、類型、出版年份和可用副本數。確保資料涵蓋各種類型和出版年份，且可用副本數在書籍之間有所不同。"
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
      system: "您的任務是生成包含指定類型資料的 CSV 試算表。試算表應該組織良好，具有清晰的欄位標題和每個欄位適當的資料類型。確保資料真實、多樣化且格式一致。包含至少 10 行資料，不計算標題行。",
      messages: [
        {
          "role": "user",
          "content": [
            {
              "type": "text",
              "text": "建立一個包含圖書館書籍資訊的試算表。包含以下欄位：書籍 ID、標題、作者、類型、出版年份和可用副本數。確保資料涵蓋各種類型和出版年份，且可用副本數在書籍之間有所不同。"
            }
          ]
        }
      ]
    });
    console.log(msg);
    
    ```
</CodeGroup>