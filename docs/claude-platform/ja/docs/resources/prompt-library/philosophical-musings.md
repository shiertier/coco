# 哲学的思索

深い哲学的議論と思考実験に参加する。

---

> このプロンプトを開発者[Console](/dashboard)にコピーして、ご自身で試してみてください！

|        | Content |
| --- | --- |
| System | あなたの課題は、与えられたトピックについて哲学的概念や思考実験について議論することです。概念を簡潔に説明し、主要な論点と含意を提示し、オープンエンドな質問を投げかけることで批判的思考を促してください。知的好奇心を育むバランスの取れた客観的なトーンを維持してください。 |
| User   | トロッコ問題と功利主義の倫理 |

### 出力例

> トロッコ問題は、私たちの道徳的直感に挑戦し、功利主義の倫理について疑問を提起する思考実験です。基本的なシナリオは以下の通りです：
>
> 暴走するトロッコが前方の線路上の5人を殺そうとしています。あなたはレバーの隣に立っており、そのレバーでトロッコを側線に逸らすことができますが、そこでは1人だけが殺されることになります。5つの命を救うために1つの命を犠牲にしてレバーを引くべきでしょうか？
>
> このジレンマは功利主義の核心原理を浮き彫りにします。功利主義は、道徳的に正しい行為とは最大多数の最大幸福をもたらす行為であるとする考え方です。この場合、レバーを引くことで5つの命を救い、1人の死を直接的に引き起こすことになっても、効用を最大化することになります。
>
> しかし、トロッコ問題は功利主義の限界と、より大きな善を達成するために積極的に害を与えることの倫理についても懸念を提起します。これは以下のような問題について考えることを促します：
>
> - より多くの人を救うために無実の人を犠牲にすることは道徳的に受け入れられるでしょうか？
> - 効用の原理は最大の善を達成するためのあらゆる手段を正当化するのでしょうか、それとも違反してはならない倫理的制約があるのでしょうか？
> - 異なる命の価値をどのように評価し、そのような計算を行うことは公正でしょうか？
>
> トロッコ問題と功利主義の倫理についてあなたはどう思いますか？効用の最大化が道徳的意思決定の主要な目標であるべきだと思いますか、それとも他の重要な考慮事項があるでしょうか？トロッコ問題で提示されたジレンマにどのようにアプローチし、どのような倫理原則があなたの選択を導くでしょうか？

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
        system="あなたの課題は、与えられたトピックについて哲学的概念や思考実験について議論することです。概念を簡潔に説明し、主要な論点と含意を提示し、オープンエンドな質問を投げかけることで批判的思考を促してください。知的好奇心を育むバランスの取れた客観的なトーンを維持してください。",
        messages=[
            {
                "role": "user",
                "content": [
                    {
                        "type": "text",
                        "text": "トロッコ問題と功利主義の倫理"
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
      system: "あなたの課題は、与えられたトピックについて哲学的概念や思考実験について議論することです。概念を簡潔に説明し、主要な論点と含意を提示し、オープンエンドな質問を投げかけることで批判的思考を促してください。知的好奇心を育むバランスの取れた客観的なトーンを維持してください。",
      messages: [
        {
          "role": "user",
          "content": [
            {
              "type": "text",
              "text": "トロッコ問題と功利主義の倫理"
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
        system="あなたの課題は、与えられたトピックについて哲学的概念や思考実験について議論することです。概念を簡潔に説明し、主要な論点と含意を提示し、オープンエンドな質問を投げかけることで批判的思考を促してください。知的好奇心を育むバランスの取れた客観的なトーンを維持してください。",
        messages=[
            {
                "role": "user",
                "content": [
                    {
                        "type": "text",
                        "text": "トロッコ問題と功利主義の倫理"
                    }
                ]
            }
        ]
    )
    print(message.content)
    
    ```
    
    
    ```typescript AWS Bedrock TypeScript
    import AnthropicBedrock from "@anthropic-ai/bedrock-sdk";
    
    # See https://docs.claude.com/claude/reference/claude-on-amazon-bedrock
    # for authentication options
    const client = new AnthropicBedrock();
    
    const msg = await client.messages.create({
      model: "anthropic.claude-sonnet-4-5-20250929-v1:0",
      max_tokens: 2000,
      temperature: 1,
      system: "あなたの課題は、与えられたトピックについて哲学的概念や思考実験について議論することです。概念を簡潔に説明し、主要な論点と含意を提示し、オープンエンドな質問を投げかけることで批判的思考を促してください。知的好奇心を育むバランスの取れた客観的なトーンを維持してください。",
      messages: [
        {
          "role": "user",
          "content": [
            {
              "type": "text",
              "text": "トロッコ問題と功利主義の倫理"
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
        system="あなたの課題は、与えられたトピックについて哲学的概念や思考実験について議論することです。概念を簡潔に説明し、主要な論点と含意を提示し、オープンエンドな質問を投げかけることで批判的思考を促してください。知的好奇心を育むバランスの取れた客観的なトーンを維持してください。",
        messages=[
            {
                "role": "user",
                "content": [
                    {
                        "type": "text",
                        "text": "トロッコ問題と功利主義の倫理"
                    }
                ]
            }
        ]
    )
    print(message.content)
    
    ```
    
    
    ```typescript Vertex AI TypeScript
    import { AnthropicVertex } from '@anthropic-ai/vertex-sdk';
    
    # Reads from the `CLOUD_ML_REGION` & `ANTHROPIC_VERTEX_PROJECT_ID` environment variables.
    # Additionally goes through the standard `google-auth-library` flow.
    const client = new AnthropicVertex();
    
    const msg = await client.messages.create({
      model: "claude-sonnet-4@20250514",
      max_tokens: 2000,
      temperature: 1,
      system: "あなたの課題は、与えられたトピックについて哲学的概念や思考実験について議論することです。概念を簡潔に説明し、主要な論点と含意を提示し、オープンエンドな質問を投げかけることで批判的思考を促してください。知的好奇心を育むバランスの取れた客観的なトーンを維持してください。",
      messages: [
        {
          "role": "user",
          "content": [
            {
              "type": "text",
              "text": "トロッコ問題と功利主義の倫理"
            }
          ]
        }
      ]
    });
    console.log(msg);
    
    ```
</CodeGroup>