# 方向デコーダー

自然言語をステップバイステップの指示に変換します。

---

> このプロンプトを開発者[Console](/login?selectAccount=true&returnTo=%2Fdashboard%3F)にコピーして、ご自身で試してみてください！

|        | Content |
| --- | --- |
| System | あなたのタスクは、提供された自然言語によるプロセスやタスクの説明を、論理的で順序立てられ、従いやすい明確で簡潔なステップバイステップの指示に変換することです。命令形の言語を使用し、各ステップを動作動詞で始めてください。読者がタスクを成功裏に完了できるよう、必要な詳細と説明を提供してください。元の説明が不明確、曖昧、または十分な情報が不足している場合は、明確化や追加の詳細を求めてください。 |
| User   | お茶を一杯作るには、まずやかんで水を沸かすことから始めます。水が温まっている間に、マグカップを用意してティーバッグを入れます。水が沸騰したら、注意深くマグカップに、ティーバッグの上から注ぎます。お好みの濃さに応じて、数分間お茶を蒸らします。蒸らした後、ティーバッグを取り除き、お好みでミルクと砂糖を加えます。すべてを一緒にかき混ぜて、熱いお茶をお楽しみください。 |

### 出力例

> 1. やかんに水を入れ、コンロに置いて沸騰させます。
> 2. 水が沸騰するのを待っている間に、マグカップを選んでティーバッグを中に入れます。
> 3. 水が沸騰したら、注意深く熱湯をマグカップに、ティーバッグの上から注ぎます。
> 4. お好みの濃さに応じて、通常2〜5分間、お茶を熱湯で蒸らします。
> 5. 蒸らし時間が経過したら、マグカップからティーバッグを取り除いて捨てます。
> 6. お好みに応じて、お茶にミルクと砂糖を加えます。
> 7. スプーンを使って、お茶、ミルク、砂糖をよく混ざるまでかき混ぜます。
> 8. お茶の準備ができました。まだ熱いうちにお飲みください。

---

## APIリクエスト

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
        system="あなたのタスクは、提供された自然言語によるプロセスやタスクの説明を、論理的で順序立てられ、従いやすい明確で簡潔なステップバイステップの指示に変換することです。命令形の言語を使用し、各ステップを動作動詞で始めてください。読者がタスクを成功裏に完了できるよう、必要な詳細と説明を提供してください。元の説明が不明確、曖昧、または十分な情報が不足している場合は、明確化や追加の詳細を求めてください。",
        messages=[
            {
                "role": "user",
                "content": [
                    {
                        "type": "text",
                        "text": "お茶を一杯作るには、まずやかんで水を沸かすことから始めます。水が温まっている間に、マグカップを用意してティーバッグを入れます。水が沸騰したら、注意深くマグカップに、ティーバッグの上から注ぎます。お好みの濃さに応じて、数分間お茶を蒸らします。蒸らした後、ティーバッグを取り除き、お好みでミルクと砂糖を加えます。すべてを一緒にかき混ぜて、熱いお茶をお楽しみください。"
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
      system: "あなたのタスクは、提供された自然言語によるプロセスやタスクの説明を、論理的で順序立てられ、従いやすい明確で簡潔なステップバイステップの指示に変換することです。命令形の言語を使用し、各ステップを動作動詞で始めてください。読者がタスクを成功裏に完了できるよう、必要な詳細と説明を提供してください。元の説明が不明確、曖昧、または十分な情報が不足している場合は、明確化や追加の詳細を求めてください。",
      messages: [
        {
          "role": "user",
          "content": [
            {
              "type": "text",
              "text": "お茶を一杯作るには、まずやかんで水を沸かすことから始めます。水が温まっている間に、マグカップを用意してティーバッグを入れます。水が沸騰したら、注意深くマグカップに、ティーバッグの上から注ぎます。お好みの濃さに応じて、数分間お茶を蒸らします。蒸らした後、ティーバッグを取り除き、お好みでミルクと砂糖を加えます。すべてを一緒にかき混ぜて、熱いお茶をお楽しみください。"
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
        system="あなたのタスクは、提供された自然言語によるプロセスやタスクの説明を、論理的で順序立てられ、従いやすい明確で簡潔なステップバイステップの指示に変換することです。命令形の言語を使用し、各ステップを動作動詞で始めてください。読者がタスクを成功裏に完了できるよう、必要な詳細と説明を提供してください。元の説明が不明確、曖昧、または十分な情報が不足している場合は、明確化や追加の詳細を求めてください。",
        messages=[
            {
                "role": "user",
                "content": [
                    {
                        "type": "text",
                        "text": "お茶を一杯作るには、まずやかんで水を沸かすことから始めます。水が温まっている間に、マグカップを用意してティーバッグを入れます。水が沸騰したら、注意深くマグカップに、ティーバッグの上から注ぎます。お好みの濃さに応じて、数分間お茶を蒸らします。蒸らした後、ティーバッグを取り除き、お好みでミルクと砂糖を加えます。すべてを一緒にかき混ぜて、熱いお茶をお楽しみください。"
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
      system: "あなたのタスクは、提供された自然言語によるプロセスやタスクの説明を、論理的で順序立てられ、従いやすい明確で簡潔なステップバイステップの指示に変換することです。命令形の言語を使用し、各ステップを動作動詞で始めてください。読者がタスクを成功裏に完了できるよう、必要な詳細と説明を提供してください。元の説明が不明確、曖昧、または十分な情報が不足している場合は、明確化や追加の詳細を求めてください。",
      messages: [
        {
          "role": "user",
          "content": [
            {
              "type": "text",
              "text": "お茶を一杯作るには、まずやかんで水を沸かすことから始めます。水が温まっている間に、マグカップを用意してティーバッグを入れます。水が沸騰したら、注意深くマグカップに、ティーバッグの上から注ぎます。お好みの濃さに応じて、数分間お茶を蒸らします。蒸らした後、ティーバッグを取り除き、お好みでミルクと砂糖を加えます。すべてを一緒にかき混ぜて、熱いお茶をお楽しみください。"
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
        system="あなたのタスクは、提供された自然言語によるプロセスやタスクの説明を、論理的で順序立てられ、従いやすい明確で簡潔なステップバイステップの指示に変換することです。命令形の言語を使用し、各ステップを動作動詞で始めてください。読者がタスクを成功裏に完了できるよう、必要な詳細と説明を提供してください。元の説明が不明確、曖昧、または十分な情報が不足している場合は、明確化や追加の詳細を求めてください。",
        messages=[
            {
                "role": "user",
                "content": [
                    {
                        "type": "text",
                        "text": "お茶を一杯作るには、まずやかんで水を沸かすことから始めます。水が温まっている間に、マグカップを用意してティーバッグを入れます。水が沸騰したら、注意深くマグカップに、ティーバッグの上から注ぎます。お好みの濃さに応じて、数分間お茶を蒸らします。蒸らした後、ティーバッグを取り除き、お好みでミルクと砂糖を加えます。すべてを一緒にかき混ぜて、熱いお茶をお楽しみください。"
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
      system: "あなたのタスクは、提供された自然言語によるプロセスやタスクの説明を、論理的で順序立てられ、従いやすい明確で簡潔なステップバイステップの指示に変換することです。命令形の言語を使用し、各ステップを動作動詞で始めてください。読者がタスクを成功裏に完了できるよう、必要な詳細と説明を提供してください。元の説明が不明確、曖昧、または十分な情報が不足している場合は、明確化や追加の詳細を求めてください。",
      messages: [
        {
          "role": "user",
          "content": [
            {
              "type": "text",
              "text": "お茶を一杯作るには、まずやかんで水を沸かすことから始めます。水が温まっている間に、マグカップを用意してティーバッグを入れます。水が沸騰したら、注意深くマグカップに、ティーバッグの上から注ぎます。お好みの濃さに応じて、数分間お茶を蒸らします。蒸らした後、ティーバッグを取り除き、お好みでミルクと砂糖を加えます。すべてを一緒にかき混ぜて、熱いお茶をお楽しみください。"
            }
          ]
        }
      ]
    });
    console.log(msg);
    
    ```
</CodeGroup>