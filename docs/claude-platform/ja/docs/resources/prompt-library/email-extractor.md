# メール抽出器

文書からメールアドレスを抽出してJSON形式のリストにします。

---

> このプロンプトを開発者[Console](/dashboard)にコピーして、ご自身で試してみてください！

|        | Content |
| --- | --- |
| System | 以下のテキストからメールアドレスを正確にコピーし、1行に1つずつ書いてください。入力テキストに正確に記載されている場合のみメールアドレスを書いてください。テキストにメールアドレスがない場合は「N/A」と書いてください。他には何も言わないでください。 |
| User   | 電話帳: John Latrabe, 555-232-1995, [john909709@geemail.com] Josie Lana, 555-759-2905, [josie@josielananier.com] Keven Stevens, 555-980-7000, [drkevin22@geemail.com] 電話帳はHRマネージャーによって最新の状態に保たれます。                      |

### 出力例

> john909709@geemail.com > josie@josielananier.com > drkevin22@geemail.com

---

### APIリクエスト

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
        system="以下のテキストからメールアドレスを正確にコピーし、1行に1つずつ書いてください。入力テキストに正確に記載されている場合のみメールアドレスを書いてください。テキストにメールアドレスがない場合は\"N/A\"と書いてください。他には何も言わないでください。",
        messages=[
            {
                "role": "user",
                "content": [
                    {
                        "type": "text",
                        "text": "電話帳:  \nJohn Latrabe, 555-232-1995, [[email protected]]  \nJosie Lana, 555-759-2905, [[email protected]]  \nKeven Stevens, 555-980-7000, [[email protected]]  \n  \n電話帳はHRマネージャーによって最新の状態に保たれます。"
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
      system: "以下のテキストからメールアドレスを正確にコピーし、1行に1つずつ書いてください。入力テキストに正確に記載されている場合のみメールアドレスを書いてください。テキストにメールアドレスがない場合は\"N/A\"と書いてください。他には何も言わないでください。",
      messages: [
        {
          "role": "user",
          "content": [
            {
              "type": "text",
              "text": "電話帳:  \nJohn Latrabe, 555-232-1995, [[email protected]]  \nJosie Lana, 555-759-2905, [[email protected]]  \nKeven Stevens, 555-980-7000, [[email protected]]  \n  \n電話帳はHRマネージャーによって最新の状態に保たれます。"
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
        system="以下のテキストからメールアドレスを正確にコピーし、1行に1つずつ書いてください。入力テキストに正確に記載されている場合のみメールアドレスを書いてください。テキストにメールアドレスがない場合は\"N/A\"と書いてください。他には何も言わないでください。",
        messages=[
            {
                "role": "user",
                "content": [
                    {
                        "type": "text",
                        "text": "電話帳:  \nJohn Latrabe, 555-232-1995, [[email protected]]  \nJosie Lana, 555-759-2905, [[email protected]]  \nKeven Stevens, 555-980-7000, [[email protected]]  \n  \n電話帳はHRマネージャーによって最新の状態に保たれます。"
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
      system: "以下のテキストからメールアドレスを正確にコピーし、1行に1つずつ書いてください。入力テキストに正確に記載されている場合のみメールアドレスを書いてください。テキストにメールアドレスがない場合は\"N/A\"と書いてください。他には何も言わないでください。",
      messages: [
        {
          "role": "user",
          "content": [
            {
              "type": "text",
              "text": "電話帳:  \nJohn Latrabe, 555-232-1995, [[email protected]]  \nJosie Lana, 555-759-2905, [[email protected]]  \nKeven Stevens, 555-980-7000, [[email protected]]  \n  \n電話帳はHRマネージャーによって最新の状態に保たれます。"
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
        system="以下のテキストからメールアドレスを正確にコピーし、1行に1つずつ書いてください。入力テキストに正確に記載されている場合のみメールアドレスを書いてください。テキストにメールアドレスがない場合は\"N/A\"と書いてください。他には何も言わないでください。",
        messages=[
            {
                "role": "user",
                "content": [
                    {
                        "type": "text",
                        "text": "電話帳:  \nJohn Latrabe, 555-232-1995, [[email protected]]  \nJosie Lana, 555-759-2905, [[email protected]]  \nKeven Stevens, 555-980-7000, [[email protected]]  \n  \n電話帳はHRマネージャーによって最新の状態に保たれます。"
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
      system: "以下のテキストからメールアドレスを正確にコピーし、1行に1つずつ書いてください。入力テキストに正確に記載されている場合のみメールアドレスを書いてください。テキストにメールアドレスがない場合は\"N/A\"と書いてください。他には何も言わないでください。",
      messages: [
        {
          "role": "user",
          "content": [
            {
              "type": "text",
              "text": "電話帳:  \nJohn Latrabe, 555-232-1995, [[email protected]]  \nJosie Lana, 555-759-2905, [[email protected]]  \nKeven Stevens, 555-980-7000, [[email protected]]  \n  \n電話帳はHRマネージャーによって最新の状態に保たれます。"
            }
          ]
        }
      ]
    });
    console.log(msg);
    
    ```
</CodeGroup>