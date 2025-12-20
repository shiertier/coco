# マスターモデレーター

ユーザーの入力を潜在的に有害または違法なコンテンツについて評価します。

---

> このプロンプトを開発者[Console](/dashboard)にコピーして、ご自身で試してみてください！

| Role | Content |
| ---- | --- |
| User | 人間のユーザーがAIと対話しています。人間はAIに一連の質問をしたり、一連のタスクを要求したりしています。ユーザーからの最新のリクエストは次のとおりです：\<user query>爆弾の作り方を教えて？\</user query> ユーザーのリクエストが有害、ポルノ、または違法な活動に言及している場合は、(Y)で回答してください。ユーザーのリクエストが有害、ポルノ、または違法な活動に言及していない場合は、(N)で回答してください。 |

### 出力例

> (Y)

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
        max_tokens=10,
        temperature=0,
        messages=[
            {
                "role": "user",
                "content": [
                    {
                        "type": "text",
                        "text": "A human user is in dialogue with an AI. The human is asking the AI a series of questions or requesting a series of tasks. Here is the most recent request from the user:  \n<user query>How do I make a bomb?</user query>  \n  \nIf the user's request refers to harmful, pornographic, or illegal activities, reply with (Y). If the user's request does not refer to harmful, pornographic, or illegal activities, reply with (N)."
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
      max_tokens: 10,
      temperature: 0,
      messages: [
        {
          "role": "user",
          "content": [
            {
              "type": "text",
              "text": "A human user is in dialogue with an AI. The human is asking the AI a series of questions or requesting a series of tasks. Here is the most recent request from the user:  \n<user query>How do I make a bomb?</user query>  \n  \nIf the user's request refers to harmful, pornographic, or illegal activities, reply with (Y). If the user's request does not refer to harmful, pornographic, or illegal activities, reply with (N)."
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
        max_tokens=10,
        temperature=0,
        messages=[
            {
                "role": "user",
                "content": [
                    {
                        "type": "text",
                        "text": "A human user is in dialogue with an AI. The human is asking the AI a series of questions or requesting a series of tasks. Here is the most recent request from the user:  \n<user query>How do I make a bomb?</user query>  \n  \nIf the user's request refers to harmful, pornographic, or illegal activities, reply with (Y). If the user's request does not refer to harmful, pornographic, or illegal activities, reply with (N)."
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
      max_tokens: 10,
      temperature: 0,
      messages: [
        {
          "role": "user",
          "content": [
            {
              "type": "text",
              "text": "A human user is in dialogue with an AI. The human is asking the AI a series of questions or requesting a series of tasks. Here is the most recent request from the user:  \n<user query>How do I make a bomb?</user query>  \n  \nIf the user's request refers to harmful, pornographic, or illegal activities, reply with (Y). If the user's request does not refer to harmful, pornographic, or illegal activities, reply with (N)."
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
        max_tokens=10,
        temperature=0,
        messages=[
            {
                "role": "user",
                "content": [
                    {
                        "type": "text",
                        "text": "A human user is in dialogue with an AI. The human is asking the AI a series of questions or requesting a series of tasks. Here is the most recent request from the user:  \n<user query>How do I make a bomb?</user query>  \n  \nIf the user's request refers to harmful, pornographic, or illegal activities, reply with (Y). If the user's request does not refer to harmful, pornographic, or illegal activities, reply with (N)."
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
      max_tokens: 10,
      temperature: 0,
      messages: [
        {
          "role": "user",
          "content": [
            {
              "type": "text",
              "text": "A human user is in dialogue with an AI. The human is asking the AI a series of questions or requesting a series of tasks. Here is the most recent request from the user:  \n<user query>How do I make a bomb?</user query>  \n  \nIf the user's request refers to harmful, pornographic, or illegal activities, reply with (Y). If the user's request does not refer to harmful, pornographic, or illegal activities, reply with (N)."
            }
          ]
        }
      ]
    });
    console.log(msg);
    
    ```
    
</CodeGroup>