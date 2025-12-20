# 電子郵件提取器

從文件中提取電子郵件地址並整理成 JSON 格式的列表。

---

> 將此提示複製到我們的開發者 [Console](/dashboard) 中親自試用！

|        | 內容 |
| --- | --- |
| System | 精確複製以下文本中的任何電子郵件地址，然後將它們逐行寫出。只有在輸入文本中精確拼寫出電子郵件地址時才寫出電子郵件地址。如果文本中沒有電子郵件地址，請寫「N/A」。不要說其他任何內容。 |
| User   | 電話目錄：John Latrabe，555-232-1995，[john909709@geemail.com] Josie Lana，555-759-2905，[josie@josielananier.com] Keven Stevens，555-980-7000，[drkevin22@geemail.com] 電話目錄將由人力資源經理保持更新。                      |

### 範例輸出

> john909709@geemail.com > josie@josielananier.com > drkevin22@geemail.com

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
        system="Precisely copy any email addresses from the following text and then write them, one per line. Only write an email address if it's precisely spelled out in the input text. If there are no email addresses in the text, write \"N/A\".  Do not say anything else.",
        messages=[
            {
                "role": "user",
                "content": [
                    {
                        "type": "text",
                        "text": "Phone Directory:  \nJohn Latrabe, 555-232-1995, [[email protected]]  \nJosie Lana, 555-759-2905, [[email protected]]  \nKeven Stevens, 555-980-7000, [[email protected]]  \n  \nPhone directory will be kept up to date by the HR manager."
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
      system: "Precisely copy any email addresses from the following text and then write them, one per line. Only write an email address if it's precisely spelled out in the input text. If there are no email addresses in the text, write \"N/A\".  Do not say anything else.",
      messages: [
        {
          "role": "user",
          "content": [
            {
              "type": "text",
              "text": "Phone Directory:  \nJohn Latrabe, 555-232-1995, [[email protected]]  \nJosie Lana, 555-759-2905, [[email protected]]  \nKeven Stevens, 555-980-7000, [[email protected]]  \n  \nPhone directory will be kept up to date by the HR manager."
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
        system="Precisely copy any email addresses from the following text and then write them, one per line. Only write an email address if it's precisely spelled out in the input text. If there are no email addresses in the text, write \"N/A\".  Do not say anything else.",
        messages=[
            {
                "role": "user",
                "content": [
                    {
                        "type": "text",
                        "text": "Phone Directory:  \nJohn Latrabe, 555-232-1995, [[email protected]]  \nJosie Lana, 555-759-2905, [[email protected]]  \nKeven Stevens, 555-980-7000, [[email protected]]  \n  \nPhone directory will be kept up to date by the HR manager."
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
      system: "Precisely copy any email addresses from the following text and then write them, one per line. Only write an email address if it's precisely spelled out in the input text. If there are no email addresses in the text, write \"N/A\".  Do not say anything else.",
      messages: [
        {
          "role": "user",
          "content": [
            {
              "type": "text",
              "text": "Phone Directory:  \nJohn Latrabe, 555-232-1995, [[email protected]]  \nJosie Lana, 555-759-2905, [[email protected]]  \nKeven Stevens, 555-980-7000, [[email protected]]  \n  \nPhone directory will be kept up to date by the HR manager."
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
        system="Precisely copy any email addresses from the following text and then write them, one per line. Only write an email address if it's precisely spelled out in the input text. If there are no email addresses in the text, write \"N/A\".  Do not say anything else.",
        messages=[
            {
                "role": "user",
                "content": [
                    {
                        "type": "text",
                        "text": "Phone Directory:  \nJohn Latrabe, 555-232-1995, [[email protected]]  \nJosie Lana, 555-759-2905, [[email protected]]  \nKeven Stevens, 555-980-7000, [[email protected]]  \n  \nPhone directory will be kept up to date by the HR manager."
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
      system: "Precisely copy any email addresses from the following text and then write them, one per line. Only write an email address if it's precisely spelled out in the input text. If there are no email addresses in the text, write \"N/A\".  Do not say anything else.",
      messages: [
        {
          "role": "user",
          "content": [
            {
              "type": "text",
              "text": "Phone Directory:  \nJohn Latrabe, 555-232-1995, [[email protected]]  \nJosie Lana, 555-759-2905, [[email protected]]  \nKeven Stevens, 555-980-7000, [[email protected]]  \n  \nPhone directory will be kept up to date by the HR manager."
            }
          ]
        }
      ]
    });
    console.log(msg);
    
    ```
</CodeGroup>