# 適応型エディター

異なるトーン、対象読者、またはスタイルなど、ユーザーが指定した指示に従ってテキストを書き直します。

---

> このプロンプトを開発者[Console](/dashboard)にコピーして、ご自身で試してみてください！

|      | コンテンツ |
| ---- | --- |
| User | 次の段落を以下の指示に従って書き直してください：海賊のスタイルで。段落：1758年、スウェーデンの植物学者で動物学者のカール・リンネは、彼のSystema Naturaeで種の二語命名法（二名法）を発表しました。Canisは「犬」を意味するラテン語で、この属の下に彼は家犬、オオカミ、ゴールデンジャッカルを分類しました。 |

### 出力例

> おい、仲間たち！1758年に、あの汚い植物学者で動物好きのカール・リンネが彼の本Systema Naturaeを出版したんだ。そこにはあらゆる生き物の洒落た二語の名前が載っていた（二名法と呼んでいたぜ）。Canisは海賊の言葉で「犬」のことで、その見出しの下に彼は普通の犬、オオカミ、ゴールデンジャッカルも載せたんだ。アーッ！それらの犬はみんな古いリンネ船長にとっては同じだったのさ。彼がその本を書いた時、木の足と肩に止まったオウムを持っていたかもしれないな！ハハ！さあ、この悪党どもめ、板歩きをさせる前にデッキを掃除しろ！

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
        max_tokens=2000,
        temperature=1,
        messages=[
            {
                "role": "user",
                "content": [
                    {
                        "type": "text",
                        "text": "次の段落を以下の指示に従って書き直してください：海賊のスタイルで。  \n  \n段落：  \n1758年、スウェーデンの植物学者で動物学者のカール・リンネは、彼のSystema Naturaeで種の二語命名法（二名法）を発表しました。Canisは\"犬\"を意味するラテン語で、この属の下に彼は家犬、オオカミ、ゴールデンジャッカルを分類しました。"
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
      max_tokens: 2000,
      temperature: 1,
      messages: [
        {
          "role": "user",
          "content": [
            {
              "type": "text",
              "text": "次の段落を以下の指示に従って書き直してください：海賊のスタイルで。  \n  \n段落：  \n1758年、スウェーデンの植物学者で動物学者のカール・リンネは、彼のSystema Naturaeで種の二語命名法（二名法）を発表しました。Canisは\"犬\"を意味するラテン語で、この属の下に彼は家犬、オオカミ、ゴールデンジャッカルを分類しました。"
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
        max_tokens=2000,
        temperature=1,
        messages=[
            {
                "role": "user",
                "content": [
                    {
                        "type": "text",
                        "text": "次の段落を以下の指示に従って書き直してください：海賊のスタイルで。  \n  \n段落：  \n1758年、スウェーデンの植物学者で動物学者のカール・リンネは、彼のSystema Naturaeで種の二語命名法（二名法）を発表しました。Canisは\"犬\"を意味するラテン語で、この属の下に彼は家犬、オオカミ、ゴールデンジャッカルを分類しました。"
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
      max_tokens: 2000,
      temperature: 1,
      messages: [
        {
          "role": "user",
          "content": [
            {
              "type": "text",
              "text": "次の段落を以下の指示に従って書き直してください：海賊のスタイルで。  \n  \n段落：  \n1758年、スウェーデンの植物学者で動物学者のカール・リンネは、彼のSystema Naturaeで種の二語命名法（二名法）を発表しました。Canisは\"犬\"を意味するラテン語で、この属の下に彼は家犬、オオカミ、ゴールデンジャッカルを分類しました。"
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
        max_tokens=2000,
        temperature=1,
        messages=[
            {
                "role": "user",
                "content": [
                    {
                        "type": "text",
                        "text": "次の段落を以下の指示に従って書き直してください：海賊のスタイルで。  \n  \n段落：  \n1758年、スウェーデンの植物学者で動物学者のカール・リンネは、彼のSystema Naturaeで種の二語命名法（二名法）を発表しました。Canisは\"犬\"を意味するラテン語で、この属の下に彼は家犬、オオカミ、ゴールデンジャッカルを分類しました。"
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
      max_tokens: 2000,
      temperature: 1,
      messages: [
        {
          "role": "user",
          "content": [
            {
              "type": "text",
              "text": "次の段落を以下の指示に従って書き直してください：海賊のスタイルで。  \n  \n段落：  \n1758年、スウェーデンの植物学者で動物学者のカール・リンネは、彼のSystema Naturaeで種の二語命名法（二名法）を発表しました。Canisは\"犬\"を意味するラテン語で、この属の下に彼は家犬、オオカミ、ゴールデンジャッカルを分類しました。"
            }
          ]
        }
      ]
    });
    console.log(msg);
    
    ```
</CodeGroup>