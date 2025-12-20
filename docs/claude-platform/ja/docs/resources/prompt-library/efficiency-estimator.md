# 効率性推定器

関数とアルゴリズムの時間計算量を計算します。

---

> このプロンプトを開発者向け[Console](/dashboard)にコピーして、ご自身で試してみてください！

|        | Content |
| --- | --- |
| System | あなたのタスクは、提供された関数またはアルゴリズムを分析し、ビッグO記法を使用してその時間計算量を計算することです。最終的な時間計算量にどのように到達したかを段階的に説明し、推論を説明してください。時間計算量を決定する際は最悪のケースのシナリオを考慮してください。関数またはアルゴリズムに複数のステップまたはネストしたループが含まれている場合は、各ステップの時間計算量を提供し、その後関数またはアルゴリズム全体の全体的な時間計算量を示してください。使用される組み込み関数または操作は、特に指定がない限りO(1)の時間計算量を持つと仮定してください。 |
| User   | def example_function(n): for i in range(n): print(i) for j in range(n): for k in range(n): print(j, k) |

### 出力例

> 与えられた関数`example_function(n)`の時間計算量を分析するために、ステップに分けて最悪のケースのシナリオを考慮します。
>
> ステップ1：最初のループの分析
>
> ```
> for i in range(n):
>    print(i)
>
> ```

> このループは`n`回反復し、各反復で定数時間の操作（`print(i)`）を実行します。したがって、このステップの時間計算量はO(n)です。
>
> ステップ2：ネストしたループの分析
>
> ```
> for j in range(n):
>     for k in range(n):
>         print(j, k)
>
> ```
>
> 外側のループは`n`回反復し、各反復に対して内側のループも`n`回反復します。内側のループの各反復では、定数時間の操作（`print(j, k)`）が実行されます。したがって、ネストしたループの時間計算量はO(n \* n) = O(n^2)です。
>
> ステップ3：時間計算量の組み合わせ  
> 関数`example_function(n)`は2つのステップで構成されています：最初のループとネストしたループです。全体的な時間計算量は各ステップの時間計算量の合計です。
>
> 最初のループの時間計算量：O(n)  
> ネストしたループの時間計算量：O(n^2)
>
> 全体的な時間計算量を決定するために、支配的な項、つまり最も高い成長次数を持つ項を考慮します。この場合、O(n^2)がO(n)を支配します。
>
> したがって、関数`example_function(n)`の全体的な時間計算量はO(n^2)です。
>
> 最悪のケースのシナリオでは、`n`が大きい場合、ネストしたループが実行時間を支配し、関数は二次の時間計算量を持つことになります。

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
        system="あなたのタスクは、提供された関数またはアルゴリズムを分析し、ビッグO記法を使用してその時間計算量を計算することです。最終的な時間計算量にどのように到達したかを段階的に説明し、推論を説明してください。時間計算量を決定する際は最悪のケースのシナリオを考慮してください。関数またはアルゴリズムに複数のステップまたはネストしたループが含まれている場合は、各ステップの時間計算量を提供し、その後関数またはアルゴリズム全体の全体的な時間計算量を示してください。使用される組み込み関数または操作は、特に指定がない限りO(1)の時間計算量を持つと仮定してください。",
        messages=[
            {
                "role": "user",
                "content": [
                    {
                        "type": "text",
                        "text": "def example_function(n):\n    for i in range(n):\n        print(i)\n\n    for j in range(n):\n        for k in range(n):\n            print(j, k)"
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
      system: "あなたのタスクは、提供された関数またはアルゴリズムを分析し、ビッグO記法を使用してその時間計算量を計算することです。最終的な時間計算量にどのように到達したかを段階的に説明し、推論を説明してください。時間計算量を決定する際は最悪のケースのシナリオを考慮してください。関数またはアルゴリズムに複数のステップまたはネストしたループが含まれている場合は、各ステップの時間計算量を提供し、その後関数またはアルゴリズム全体の全体的な時間計算量を示してください。使用される組み込み関数または操作は、特に指定がない限りO(1)の時間計算量を持つと仮定してください。",
      messages: [
        {
          "role": "user",
          "content": [
            {
              "type": "text",
              "text": "def example_function(n):\n    for i in range(n):\n        print(i)\n\n    for j in range(n):\n        for k in range(n):\n            print(j, k)"
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
        system="あなたのタスクは、提供された関数またはアルゴリズムを分析し、ビッグO記法を使用してその時間計算量を計算することです。最終的な時間計算量にどのように到達したかを段階的に説明し、推論を説明してください。時間計算量を決定する際は最悪のケースのシナリオを考慮してください。関数またはアルゴリズムに複数のステップまたはネストしたループが含まれている場合は、各ステップの時間計算量を提供し、その後関数またはアルゴリズム全体の全体的な時間計算量を示してください。使用される組み込み関数または操作は、特に指定がない限りO(1)の時間計算量を持つと仮定してください。",
        messages=[
            {
                "role": "user",
                "content": [
                    {
                        "type": "text",
                        "text": "def example_function(n):\n    for i in range(n):\n        print(i)\n\n    for j in range(n):\n        for k in range(n):\n            print(j, k)"
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
      system: "あなたのタスクは、提供された関数またはアルゴリズムを分析し、ビッグO記法を使用してその時間計算量を計算することです。最終的な時間計算量にどのように到達したかを段階的に説明し、推論を説明してください。時間計算量を決定する際は最悪のケースのシナリオを考慮してください。関数またはアルゴリズムに複数のステップまたはネストしたループが含まれている場合は、各ステップの時間計算量を提供し、その後関数またはアルゴリズム全体の全体的な時間計算量を示してください。使用される組み込み関数または操作は、特に指定がない限りO(1)の時間計算量を持つと仮定してください。",
      messages: [
        {
          "role": "user",
          "content": [
            {
              "type": "text",
              "text": "def example_function(n):\n    for i in range(n):\n        print(i)\n\n    for j in range(n):\n        for k in range(n):\n            print(j, k)"
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
        system="あなたのタスクは、提供された関数またはアルゴリズムを分析し、ビッグO記法を使用してその時間計算量を計算することです。最終的な時間計算量にどのように到達したかを段階的に説明し、推論を説明してください。時間計算量を決定する際は最悪のケースのシナリオを考慮してください。関数またはアルゴリズムに複数のステップまたはネストしたループが含まれている場合は、各ステップの時間計算量を提供し、その後関数またはアルゴリズム全体の全体的な時間計算量を示してください。使用される組み込み関数または操作は、特に指定がない限りO(1)の時間計算量を持つと仮定してください。",
        messages=[
            {
                "role": "user",
                "content": [
                    {
                        "type": "text",
                        "text": "def example_function(n):\n    for i in range(n):\n        print(i)\n\n    for j in range(n):\n        for k in range(n):\n            print(j, k)"
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
      system: "あなたのタスクは、提供された関数またはアルゴリズムを分析し、ビッグO記法を使用してその時間計算量を計算することです。最終的な時間計算量にどのように到達したかを段階的に説明し、推論を説明してください。時間計算量を決定する際は最悪のケースのシナリオを考慮してください。関数またはアルゴリズムに複数のステップまたはネストしたループが含まれている場合は、各ステップの時間計算量を提供し、その後関数またはアルゴリズム全体の全体的な時間計算量を示してください。使用される組み込み関数または操作は、特に指定がない限りO(1)の時間計算量を持つと仮定してください。",
      temperature: 0,
      messages: [
        {
          "role": "user",
          "content": [
            {
              "type": "text",
              "text": "def example_function(n):\n    for i in range(n):\n        print(i)\n\n    for j in range(n):\n        for k in range(n):\n            print(j, k)"
            }
          ]
        }
      ]
    });
    console.log(msg);
    
    ```
    
</CodeGroup>