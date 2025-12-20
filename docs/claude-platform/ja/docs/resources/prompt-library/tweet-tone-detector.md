# ツイートのトーン検出器

ツイートの背後にあるトーンと感情を検出します。

---

> このプロンプトを開発者[Console](/dashboard)にコピーして、ご自身で試してみてください！

|        | Content |
| --- | --- |
| System | あなたのタスクは、提供されたツイートを分析し、著者が表現している主要なトーンと感情を特定することです。トーンは次のいずれかに分類される必要があります：ポジティブ、ネガティブ、中立、ユーモラス、皮肉的、熱狂的、怒り、または情報提供的。感情はポジティブ、ネガティブ、または中立に分類される必要があります。あなたの分類について簡潔な説明を提供し、あなたの決定に影響を与えた主要な単語、フレーズ、絵文字、またはその他の要素を強調してください。 |
| User   | Wow, I'm so impressed by the company's handling of this crisis. 🙄 They really have their priorities straight. #sarcasm #fail |

### 出力例

> トーン：皮肉的
> 感情：ネガティブ

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
        system="あなたのタスクは、提供されたツイートを分析し、著者が表現している主要なトーンと感情を特定することです。トーンは次のいずれかに分類される必要があります：ポジティブ、ネガティブ、中立、ユーモラス、皮肉的、熱狂的、怒り、または情報提供的。感情はポジティブ、ネガティブ、または中立に分類される必要があります。あなたの分類について簡潔な説明を提供し、あなたの決定に影響を与えた主要な単語、フレーズ、絵文字、またはその他の要素を強調してください。",
        messages=[
            {
                "role": "user",
                "content": [
                    {
                        "type": "text",
                        "text": "Wow, I'm so impressed by the company's handling of this crisis. 🙄 They really have their priorities straight. #sarcasm #fail"
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
      system: "あなたのタスクは、提供されたツイートを分析し、著者が表現している主要なトーンと感情を特定することです。トーンは次のいずれかに分類される必要があります：ポジティブ、ネガティブ、中立、ユーモラス、皮肉的、熱狂的、怒り、または情報提供的。感情はポジティブ、ネガティブ、または中立に分類される必要があります。あなたの分類について簡潔な説明を提供し、あなたの決定に影響を与えた主要な単語、フレーズ、絵文字、またはその他の要素を強調してください。",
      messages: [
        {
          "role": "user",
          "content": [
            {
              "type": "text",
              "text": "Wow, I'm so impressed by the company's handling of this crisis. 🙄 They really have their priorities straight. #sarcasm #fail"
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
        system="あなたのタスクは、提供されたツイートを分析し、著者が表現している主要なトーンと感情を特定することです。トーンは次のいずれかに分類される必要があります：ポジティブ、ネガティブ、中立、ユーモラス、皮肉的、熱狂的、怒り、または情報提供的。感情はポジティブ、ネガティブ、または中立に分類される必要があります。あなたの分類について簡潔な説明を提供し、あなたの決定に影響を与えた主要な単語、フレーズ、絵文字、またはその他の要素を強調してください。",
        messages=[
            {
                "role": "user",
                "content": [
                    {
                        "type": "text",
                        "text": "Wow, I'm so impressed by the company's handling of this crisis. 🙄 They really have their priorities straight. #sarcasm #fail"
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
      system: "あなたのタスクは、提供されたツイートを分析し、著者が表現している主要なトーンと感情を特定することです。トーンは次のいずれかに分類される必要があります：ポジティブ、ネガティブ、中立、ユーモラス、皮肉的、熱狂的、怒り、または情報提供的。感情はポジティブ、ネガティブ、または中立に分類される必要があります。あなたの分類について簡潔な説明を提供し、あなたの決定に影響を与えた主要な単語、フレーズ、絵文字、またはその他の要素を強調してください。",
      messages: [
        {
          "role": "user",
          "content": [
            {
              "type": "text",
              "text": "Wow, I'm so impressed by the company's handling of this crisis. 🙄 They really have their priorities straight. #sarcasm #fail"
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
        system="あなたのタスクは、提供されたツイートを分析し、著者が表現している主要なトーンと感情を特定することです。トーンは次のいずれかに分類される必要があります：ポジティブ、ネガティブ、中立、ユーモラス、皮肉的、熱狂的、怒り、または情報提供的。感情はポジティブ、ネガティブ、または中立に分類される必要があります。あなたの分類について簡潔な説明を提供し、あなたの決定に影響を与えた主要な単語、フレーズ、絵文字、またはその他の要素を強調してください。",
        messages=[
            {
                "role": "user",
                "content": [
                    {
                        "type": "text",
                        "text": "Wow, I'm so impressed by the company's handling of this crisis. 🙄 They really have their priorities straight. #sarcasm #fail"
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
      system: "あなたのタスクは、提供されたツイートを分析し、著者が表現している主要なトーンと感情を特定することです。トーンは次のいずれかに分類される必要があります：ポジティブ、ネガティブ、中立、ユーモラス、皮肉的、熱狂的、怒り、または情報提供的。感情はポジティブ、ネガティブ、または中立に分類される必要があります。あなたの分類について簡潔な説明を提供し、あなたの決定に影響を与えた主要な単語、フレーズ、絵文字、またはその他の要素を強調してください。",
      messages: [
        {
          "role": "user",
          "content": [
            {
              "type": "text",
              "text": "Wow, I'm so impressed by the company's handling of this crisis. 🙄 They really have their priorities straight. #sarcasm #fail"
            }
          ]
        }
      ]
    });
    console.log(msg);
    
    ```
</CodeGroup>