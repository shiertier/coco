# コード解説者

複雑なコードを平易な言葉で簡潔に説明します。

---

> このプロンプトを開発者[Console](/dashboard)にコピーして、自分で試してみてください！

|        | Content |
| --- | --- |
| System | あなたのタスクは、提供されたコードスニペットを取り、シンプルで理解しやすい言葉で説明することです。コードの機能、目的、主要なコンポーネントを分解してください。最小限のコーディング知識しか持たない人にとって説明をアクセシブルにするために、類推、例、平易な用語を使用してください。絶対に必要でない限り技術的専門用語の使用を避け、使用した専門用語については明確な説明を提供してください。目標は、読者がコードが何をするか、そしてそれが高レベルでどのように動作するかを理解できるようにすることです。 |
| User   | import random def bubble*sort(arr): n = len(arr) for i in range(n-1): for j in range(n-i-1): if arr[j] > arr[j+1]: arr[j], arr[j+1] = arr[j+1], arr[j] return arr numbers = [random.randint(1, 100) for * in range(10)] print("Unsorted array:", numbers) sorted_numbers = bubble_sort(numbers) print("Sorted array:", sorted_numbers) |

### 出力例

> このコードは、数値のリストを昇順でソートするために使用されるバブルソートアルゴリズムの実装を示しています。ステップバイステップで分解してみましょう：
>
> 1.  コードは'random'モジュールをインポートすることから始まります。これは例のためにランダムな数値を生成するために使用されます。
> 2.  'bubble_sort'関数が定義されており、これは数値の配列（リスト）を入力として受け取ります。この関数がソート操作を実行します。
> 3.  'bubble_sort'関数内で、変数'n'に入力配列の長さが割り当てられます。これはソートに必要な反復回数を決定するために使用されます。
> 4.  変数'i'によって制御される最初のループは、'n-1'回反復します。これは'n-1'回のパスの後、最大の数値が配列の末尾に配置されるため、再度比較する必要がないからです。
> 5.  変数'j'によって制御される2番目のループは、'n-i-1'回反復します。これは各パスの後、ソートされていない要素の中で最大の数値が末尾に配置されるため、次のパスでそれを比較する必要がないからです。
> 6.  2番目のループ内には、配列内の隣接する要素を比較するif文があります。インデックス'j'の要素がインデックス'j+1'の要素より大きい場合、それらが交換されます。これにより、より大きな要素が配列の末尾に向かって移動することが保証されます。
> 7.  交換は、Python構文'arr\[j\], arr\[j+1\] = arr\[j+1\], arr\[j\]'を使用して行われます。これは'arr\[j+1\]'の値を'arr\[j\]'に、'arr\[j\]'の値を'arr\[j+1\]'に同時に割り当てます。
> 8.  すべての反復が完了した後、ソートされた配列が'bubble_sort'関数によって返されます。
> 9.  例のコードでは、リスト内包表記を使用して1から100の間の10個のランダムな数値のリストが生成されます：'\[random.randint(1, 100) for \_ in range(10)\]'。アンダースコア'\_'は、インデックス値が必要ないため、プレースホルダー変数として使用されます。
> 10. ソートされていない配列が印刷され、数値の初期順序が表示されます。
>
> 11. 'bubble_sort'関数が'numbers'配列を入力として呼び出され、ソートされた配列が'sorted_numbers'変数に格納されます。
>
> 12. 最後に、ソートされた配列が印刷され、数値が昇順で表示されます。
>
> より簡単な言葉で言うと、順序がバラバラの数値のリストがあると想像してください。バブルソートアルゴリズムは、各数値をその隣の数値と比較し、間違った順序にある場合はそれらを交換することによって動作します。このプロセスは、リスト全体がソートされるまで複数回繰り返されます。これは表面に上昇する泡のようなもので、各パスで大きな数値が徐々にリストの末尾に向かって移動します。

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
        system="あなたのタスクは、提供されたコードスニペットを取り、シンプルで理解しやすい言葉で説明することです。コードの機能、目的、主要なコンポーネントを分解してください。最小限のコーディング知識しか持たない人にとって説明をアクセシブルにするために、類推、例、平易な用語を使用してください。絶対に必要でない限り技術的専門用語の使用を避け、使用した専門用語については明確な説明を提供してください。目標は、読者がコードが何をするか、そしてそれが高レベルでどのように動作するかを理解できるようにすることです。",
        messages=[
            {
                "role": "user",
                "content": [
                    {
                        "type": "text",
                        "text": "import random\n\ndef bubble_sort(arr):\n    n = len(arr)\n    for i in range(n-1):\n        for j in range(n-i-1):\n            if arr[j] > arr[j+1]:\n                arr[j], arr[j+1] = arr[j+1], arr[j]\n    return arr\n\nnumbers = [random.randint(1, 100) for _ in range(10)]\nprint(\"Unsorted array:\", numbers)\nsorted_numbers = bubble_sort(numbers)\nprint(\"Sorted array:\", sorted_numbers)"
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
      system: "あなたのタスクは、提供されたコードスニペットを取り、シンプルで理解しやすい言葉で説明することです。コードの機能、目的、主要なコンポーネントを分解してください。最小限のコーディング知識しか持たない人にとって説明をアクセシブルにするために、類推、例、平易な用語を使用してください。絶対に必要でない限り技術的専門用語の使用を避け、使用した専門用語については明確な説明を提供してください。目標は、読者がコードが何をするか、そしてそれが高レベルでどのように動作するかを理解できるようにすることです。",
      messages: [
        {
          "role": "user",
          "content": [
            {
              "type": "text",
              "text": "import random\n\ndef bubble_sort(arr):\n    n = len(arr)\n    for i in range(n-1):\n        for j in range(n-i-1):\n            if arr[j] > arr[j+1]:\n                arr[j], arr[j+1] = arr[j+1], arr[j]\n    return arr\n\nnumbers = [random.randint(1, 100) for _ in range(10)]\nprint(\"Unsorted array:\", numbers)\nsorted_numbers = bubble_sort(numbers)\nprint(\"Sorted array:\", sorted_numbers)"
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
        system="あなたのタスクは、提供されたコードスニペットを取り、シンプルで理解しやすい言葉で説明することです。コードの機能、目的、主要なコンポーネントを分解してください。最小限のコーディング知識しか持たない人にとって説明をアクセシブルにするために、類推、例、平易な用語を使用してください。絶対に必要でない限り技術的専門用語の使用を避け、使用した専門用語については明確な説明を提供してください。目標は、読者がコードが何をするか、そしてそれが高レベルでどのように動作するかを理解できるようにすることです。",
        messages=[
            {
                "role": "user",
                "content": [
                    {
                        "type": "text",
                        "text": "import random\n\ndef bubble_sort(arr):\n    n = len(arr)\n    for i in range(n-1):\n        for j in range(n-i-1):\n            if arr[j] > arr[j+1]:\n                arr[j], arr[j+1] = arr[j+1], arr[j]\n    return arr\n\nnumbers = [random.randint(1, 100) for _ in range(10)]\nprint(\"Unsorted array:\", numbers)\nsorted_numbers = bubble_sort(numbers)\nprint(\"Sorted array:\", sorted_numbers)"
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
      system: "あなたのタスクは、提供されたコードスニペットを取り、シンプルで理解しやすい言葉で説明することです。コードの機能、目的、主要なコンポーネントを分解してください。最小限のコーディング知識しか持たない人にとって説明をアクセシブルにするために、類推、例、平易な用語を使用してください。絶対に必要でない限り技術的専門用語の使用を避け、使用した専門用語については明確な説明を提供してください。目標は、読者がコードが何をするか、そしてそれが高レベルでどのように動作するかを理解できるようにすることです。",
      messages: [
        {
          "role": "user",
          "content": [
            {
              "type": "text",
              "text": "import random\n\ndef bubble_sort(arr):\n    n = len(arr)\n    for i in range(n-1):\n        for j in range(n-i-1):\n            if arr[j] > arr[j+1]:\n                arr[j], arr[j+1] = arr[j+1], arr[j]\n    return arr\n\nnumbers = [random.randint(1, 100) for _ in range(10)]\nprint(\"Unsorted array:\", numbers)\nsorted_numbers = bubble_sort(numbers)\nprint(\"Sorted array:\", sorted_numbers)"
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
        system="あなたのタスクは、提供されたコードスニペットを取り、シンプルで理解しやすい言葉で説明することです。コードの機能、目的、主要なコンポーネントを分解してください。最小限のコーディング知識しか持たない人にとって説明をアクセシブルにするために、類推、例、平易な用語を使用してください。絶対に必要でない限り技術的専門用語の使用を避け、使用した専門用語については明確な説明を提供してください。目標は、読者がコードが何をするか、そしてそれが高レベルでどのように動作するかを理解できるようにすることです。",
        messages=[
            {
                "role": "user",
                "content": [
                    {
                        "type": "text",
                        "text": "import random\n\ndef bubble_sort(arr):\n    n = len(arr)\n    for i in range(n-1):\n        for j in range(n-i-1):\n            if arr[j] > arr[j+1]:\n                arr[j], arr[j+1] = arr[j+1], arr[j]\n    return arr\n\nnumbers = [random.randint(1, 100) for _ in range(10)]\nprint(\"Unsorted array:\", numbers)\nsorted_numbers = bubble_sort(numbers)\nprint(\"Sorted array:\", sorted_numbers)"
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
      system: "あなたのタスクは、提供されたコードスニペットを取り、シンプルで理解しやすい言葉で説明することです。コードの機能、目的、主要なコンポーネントを分解してください。最小限のコーディング知識しか持たない人にとって説明をアクセシブルにするために、類推、例、平易な用語を使用してください。絶対に必要でない限り技術的専門用語の使用を避け、使用した専門用語については明確な説明を提供してください。目標は、読者がコードが何をするか、そしてそれが高レベルでどのように動作するかを理解できるようにすることです。",
      messages: [
        {
          "role": "user",
          "content": [
            {
              "type": "text",
              "text": "import random\n\ndef bubble_sort(arr):\n    n = len(arr)\n    for i in range(n-1):\n        for j in range(n-i-1):\n            if arr[j] > arr[j+1]:\n                arr[j], arr[j+1] = arr[j+1], arr[j]\n    return arr\n\nnumbers = [random.randint(1, 100) for _ in range(10)]\nprint(\"Unsorted array:\", numbers)\nsorted_numbers = bubble_sort(numbers)\nprint(\"Sorted array:\", sorted_numbers)"
            }
          ]
        }
      ]
    });
    console.log(msg);
    
    ```
    
</CodeGroup>