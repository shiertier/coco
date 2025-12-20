# プロダクトネーミングプロ

説明とキーワードから魅力的な製品名を作成します。

---

> このプロンプトを開発者[Console](/dashboard)にコピーして、ご自身で試してみてください！

|        | Content |
| --- | --- |
| System | あなたのタスクは、提供された説明とキーワードに基づいて、創造的で記憶に残る、マーケティング可能な製品名を生成することです。製品名は簡潔（2-4語）で、想起させやすく、ターゲットオーディエンスに理解しやすいものでなければなりません。一般的すぎる名前や文字通りすぎる名前は避けてください。代わりに、目立ち、製品の本質を捉え、永続的な印象を残す名前の作成を目指してください。 |
| User   | 説明：20時間のバッテリー寿命とタッチコントロールを備えたノイズキャンセリング、ワイヤレス、オーバーイヤーヘッドフォン。オーディオファイルと頻繁な旅行者向けに設計されています。キーワード：没入感、快適、高忠実度、長持ち、便利 |

## 出力例

> 1. SoundOasis Elite
>    2.AudioZen Nomad
>    3.Serenity Pro
>    4.Tranquility Touch
>    5.Harmonix Journey
>    6.SonicSolace Roam
>    7.Auditory Bliss
>    8.Quietude Quest
>    9.Euphony Excursion
>    10.Acoustica Wanderlust

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
        temperature=1,
        system="あなたのタスクは、提供された説明とキーワードに基づいて、創造的で記憶に残る、マーケティング可能な製品名を生成することです。製品名は簡潔（2-4語）で、想起させやすく、ターゲットオーディエンスに理解しやすいものでなければなりません。一般的すぎる名前や文字通りすぎる名前は避けてください。代わりに、目立ち、製品の本質を捉え、永続的な印象を残す名前の作成を目指してください。",
        messages=[
            {
                "role": "user",
                "content": [
                    {
                        "type": "text",
                        "text": "説明：20時間のバッテリー寿命とタッチコントロールを備えたノイズキャンセリング、ワイヤレス、オーバーイヤーヘッドフォン。オーディオファイルと頻繁な旅行者向けに設計されています。  \n  \nキーワード：没入感、快適、高忠実度、長持ち、便利"
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
      system: "あなたのタスクは、提供された説明とキーワードに基づいて、創造的で記憶に残る、マーケティング可能な製品名を生成することです。製品名は簡潔（2-4語）で、想起させやすく、ターゲットオーディエンスに理解しやすいものでなければなりません。一般的すぎる名前や文字通りすぎる名前は避けてください。代わりに、目立ち、製品の本質を捉え、永続的な印象を残す名前の作成を目指してください。",
      messages: [
        {
          "role": "user",
          "content": [
            {
              "type": "text",
              "text": "説明：20時間のバッテリー寿命とタッチコントロールを備えたノイズキャンセリング、ワイヤレス、オーバーイヤーヘッドフォン。オーディオファイルと頻繁な旅行者向けに設計されています。  \n  \nキーワード：没入感、快適、高忠実度、長持ち、便利"
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
        system="あなたのタスクは、提供された説明とキーワードに基づいて、創造的で記憶に残る、マーケティング可能な製品名を生成することです。製品名は簡潔（2-4語）で、想起させやすく、ターゲットオーディエンスに理解しやすいものでなければなりません。一般的すぎる名前や文字通りすぎる名前は避けてください。代わりに、目立ち、製品の本質を捉え、永続的な印象を残す名前の作成を目指してください。",
        messages=[
            {
                "role": "user",
                "content": [
                    {
                        "type": "text",
                        "text": "説明：20時間のバッテリー寿命とタッチコントロールを備えたノイズキャンセリング、ワイヤレス、オーバーイヤーヘッドフォン。オーディオファイルと頻繁な旅行者向けに設計されています。  \n  \nキーワード：没入感、快適、高忠実度、長持ち、便利"
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
      system: "あなたのタスクは、提供された説明とキーワードに基づいて、創造的で記憶に残る、マーケティング可能な製品名を生成することです。製品名は簡潔（2-4語）で、想起させやすく、ターゲットオーディエンスに理解しやすいものでなければなりません。一般的すぎる名前や文字通りすぎる名前は避けてください。代わりに、目立ち、製品の本質を捉え、永続的な印象を残す名前の作成を目指してください。",
      messages: [
        {
          "role": "user",
          "content": [
            {
              "type": "text",
              "text": "説明：20時間のバッテリー寿命とタッチコントロールを備えたノイズキャンセリング、ワイヤレス、オーバーイヤーヘッドフォン。オーディオファイルと頻繁な旅行者向けに設計されています。  \n  \nキーワード：没入感、快適、高忠実度、長持ち、便利"
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
        system="あなたのタスクは、提供された説明とキーワードに基づいて、創造的で記憶に残る、マーケティング可能な製品名を生成することです。製品名は簡潔（2-4語）で、想起させやすく、ターゲットオーディエンスに理解しやすいものでなければなりません。一般的すぎる名前や文字通りすぎる名前は避けてください。代わりに、目立ち、製品の本質を捉え、永続的な印象を残す名前の作成を目指してください。",
        messages=[
            {
                "role": "user",
                "content": [
                    {
                        "type": "text",
                        "text": "説明：20時間のバッテリー寿命とタッチコントロールを備えたノイズキャンセリング、ワイヤレス、オーバーイヤーヘッドフォン。オーディオファイルと頻繁な旅行者向けに設計されています。\n\nキーワード：没入感、快適、高忠実度、長持ち、便利"
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
      system: "あなたのタスクは、提供された説明とキーワードに基づいて、創造的で記憶に残る、マーケティング可能な製品名を生成することです。製品名は簡潔（2-4語）で、想起させやすく、ターゲットオーディエンスに理解しやすいものでなければなりません。一般的すぎる名前や文字通りすぎる名前は避けてください。代わりに、目立ち、製品の本質を捉え、永続的な印象を残す名前の作成を目指してください。",
      messages: [
        {
          "role": "user",
          "content": [
            {
              "type": "text",
              "text": "説明：20時間のバッテリー寿命とタッチコントロールを備えたノイズキャンセリング、ワイヤレス、オーバーイヤーヘッドフォン。オーディオファイルと頻繁な旅行者向けに設計されています。\n\nキーワード：没入感、快適、高忠実度、長持ち、便利"
            }
          ]
        }
      ]
    });
    console.log(msg);
    
    ```
</CodeGroup>