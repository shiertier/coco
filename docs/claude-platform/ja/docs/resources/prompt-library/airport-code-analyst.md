# 空港コード分析

テキストから空港コードを見つけて抽出します。

---

> このプロンプトを開発者[Console](/dashboard)にコピーして、ご自身で試してみてください！

|        | Content |
| --- | --- |
| System | あなたのタスクは、提供されたテキストを分析し、その中に記載されている空港コードを特定することです。これらの空港コードを、テキストに現れる順序でリストとして提示してください。空港コードが見つからない場合は、空のリストを返してください。 |
| User   | 私の次の旅行では、シアトルからアムステルダムまで飛行機で移動します。アムステルダムで数日間過ごした後、パリに向かい、そこからローマへの乗り継ぎ便に乗る予定です。 |

### 出力例

> テキストに記載されている空港コードのリストを、現れる順序で示します：
>
> 1. SEA (シアトル)
> 2. AMS (アムステルダム)
> 3. CDG (パリ)
> 4. FCO (ローマ)

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
        system="あなたのタスクは、提供されたテキストを分析し、その中に記載されている空港コードを特定することです。これらの空港コードを、テキストに現れる順序でリストとして提示してください。空港コードが見つからない場合は、空のリストを返してください。",
        messages=[
            {
                "role": "user",
                "content": [
                    {
                        "type": "text",
                        "text": "私の次の旅行では、シアトルからアムステルダムまで飛行機で移動します。アムステルダムで数日間過ごした後、パリに向かい、そこからローマへの乗り継ぎ便に乗る予定です。"
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
      system: "あなたのタスクは、提供されたテキストを分析し、その中に記載されている空港コードを特定することです。これらの空港コードを、テキストに現れる順序でリストとして提示してください。空港コードが見つからない場合は、空のリストを返してください。",
      messages: [
        {
          "role": "user",
          "content": [
            {
              "type": "text",
              "text": "私の次の旅行では、シアトルからアムステルダムまで飛行機で移動します。アムステルダムで数日間過ごした後、パリに向かい、そこからローマへの乗り継ぎ便に乗る予定です。"
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
        system="あなたのタスクは、提供されたテキストを分析し、その中に記載されている空港コードを特定することです。これらの空港コードを、テキストに現れる順序でリストとして提示してください。空港コードが見つからない場合は、空のリストを返してください。",
        messages=[
            {
                "role": "user",
                "content": [
                    {
                        "type": "text",
                        "text": "私の次の旅行では、シアトルからアムステルダムまで飛行機で移動します。アムステルダムで数日間過ごした後、パリに向かい、そこからローマへの乗り継ぎ便に乗る予定です。"
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
      system: "あなたのタスクは、提供されたテキストを分析し、その中に記載されている空港コードを特定することです。これらの空港コードを、テキストに現れる順序でリストとして提示してください。空港コードが見つからない場合は、空のリストを返してください。",
      messages: [
        {
          "role": "user",
          "content": [
            {
              "type": "text",
              "text": "私の次の旅行では、シアトルからアムステルダムまで飛行機で移動します。アムステルダムで数日間過ごした後、パリに向かい、そこからローマへの乗り継ぎ便に乗る予定です。"
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
        system="あなたのタスクは、提供されたテキストを分析し、その中に記載されている空港コードを特定することです。これらの空港コードを、テキストに現れる順序でリストとして提示してください。空港コードが見つからない場合は、空のリストを返してください。",
        messages=[
            {
                "role": "user",
                "content": [
                    {
                        "type": "text",
                        "text": "私の次の旅行では、シアトルからアムステルダムまで飛行機で移動します。アムステルダムで数日間過ごした後、パリに向かい、そこからローマへの乗り継ぎ便に乗る予定です。"
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
      system: "あなたのタスクは、提供されたテキストを分析し、その中に記載されている空港コードを特定することです。これらの空港コードを、テキストに現れる順序でリストとして提示してください。空港コードが見つからない場合は、空のリストを返してください。",
      messages: [
        {
          "role": "user",
          "content": [
            {
              "type": "text",
              "text": "私の次の旅行では、シアトルからアムステルダムまで飛行機で移動します。アムステルダムで数日間過ごした後、パリに向かい、そこからローマへの乗り継ぎ便に乗る予定です。"
            }
          ]
        }
      ]
    });
    console.log(msg);
    
    ```
</CodeGroup>