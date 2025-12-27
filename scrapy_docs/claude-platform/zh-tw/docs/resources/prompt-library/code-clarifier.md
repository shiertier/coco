# 程式碼解釋器

用簡單的語言簡化並解釋複雜的程式碼。

---

> 將此提示複製到我們的開發者 [Console](/dashboard) 中親自試用！

|        | 內容 |
| --- | --- |
| System | 您的任務是接收提供的程式碼片段，並用簡單易懂的語言進行解釋。分解程式碼的功能、目的和關鍵組件。使用類比、範例和通俗術語，讓程式設計知識有限的人也能理解解釋內容。除非絕對必要，否則避免使用技術術語，並對使用的任何術語提供清楚的解釋。目標是幫助讀者在高層次上理解程式碼的作用和運作方式。 |
| User   | import random def bubble*sort(arr): n = len(arr) for i in range(n-1): for j in range(n-i-1): if arr[j] > arr[j+1]: arr[j], arr[j+1] = arr[j+1], arr[j] return arr numbers = [random.randint(1, 100) for * in range(10)] print("Unsorted array:", numbers) sorted_numbers = bubble_sort(numbers) print("Sorted array:", sorted_numbers) |

### 範例輸出

> 這段程式碼展示了氣泡排序演算法的實作，用於將數字列表按升序排列。讓我們逐步分解：
>
> 1.  程式碼首先匯入 'random' 模組，這將用於為範例生成隨機數字。
> 2.  定義了 'bubble_sort' 函數，它接收一個數字陣列（列表）作為輸入。此函數將執行排序操作。
> 3.  在 'bubble_sort' 函數內部，變數 'n' 被賦予輸入陣列的長度。這將用於確定排序所需的迭代次數。
> 4.  第一個迴圈由變數 'i' 控制，迭代 'n-1' 次。這是因為經過 'n-1' 次遍歷後，最大的數字將位於陣列末尾，所以我們不需要再次比較它。
> 5.  第二個迴圈由變數 'j' 控制，迭代 'n-i-1' 次。這是因為每次遍歷後，未排序元素中的最大數字將被放置在末尾，所以我們在下一次遍歷中不需要比較它。
> 6.  在第二個迴圈內部，有一個 if 語句比較陣列中的相鄰元素。如果索引 'j' 處的元素大於索引 'j+1' 處的元素，它們就會被交換。這確保較大的元素向陣列末尾移動。
> 7.  交換使用 Python 語法 'arr\[j\], arr\[j+1\] = arr\[j+1\], arr\[j\]' 完成，它同時將 'arr\[j+1\]' 的值賦給 'arr\[j\]'，將 'arr\[j\]' 的值賦給 'arr\[j+1\]'。
> 8.  所有迭代完成後，'bubble_sort' 函數返回排序後的陣列。
> 9.  在範例程式碼中，使用列表推導式生成了 10 個介於 1 到 100 之間的隨機數字列表：'\[random.randint(1, 100) for \_ in range(10)\]'。底線 '\_' 用作佔位符變數，因為我們不需要索引值。
> 10. 列印未排序的陣列以顯示數字的初始順序。
>
> 11. 以 'numbers' 陣列作為輸入呼叫 'bubble_sort' 函數，排序後的陣列儲存在 'sorted_numbers' 變數中。
>
> 12. 最後，列印排序後的陣列以按升序顯示數字。
>
> 簡單來說，想像您有一個順序混亂的數字列表。氣泡排序演算法的工作原理是將每個數字與旁邊的數字進行比較，如果它們的順序錯誤，就交換它們。這個過程重複多次，直到整個列表被排序。就像氣泡上升到表面一樣，較大的數字在每次遍歷中逐漸向列表末尾移動。

---

### API 請求

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
        system="您的任務是接收提供的程式碼片段，並用簡單易懂的語言進行解釋。分解程式碼的功能、目的和關鍵組件。使用類比、範例和通俗術語，讓程式設計知識有限的人也能理解解釋內容。除非絕對必要，否則避免使用技術術語，並對使用的任何術語提供清楚的解釋。目標是幫助讀者在高層次上理解程式碼的作用和運作方式。",
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
      system: "您的任務是接收提供的程式碼片段，並用簡單易懂的語言進行解釋。分解程式碼的功能、目的和關鍵組件。使用類比、範例和通俗術語，讓程式設計知識有限的人也能理解解釋內容。除非絕對必要，否則避免使用技術術語，並對使用的任何術語提供清楚的解釋。目標是幫助讀者在高層次上理解程式碼的作用和運作方式。",
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
        system="您的任務是接收提供的程式碼片段，並用簡單易懂的語言進行解釋。分解程式碼的功能、目的和關鍵組件。使用類比、範例和通俗術語，讓程式設計知識有限的人也能理解解釋內容。除非絕對必要，否則避免使用技術術語，並對使用的任何術語提供清楚的解釋。目標是幫助讀者在高層次上理解程式碼的作用和運作方式。",
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
      system: "您的任務是接收提供的程式碼片段，並用簡單易懂的語言進行解釋。分解程式碼的功能、目的和關鍵組件。使用類比、範例和通俗術語，讓程式設計知識有限的人也能理解解釋內容。除非絕對必要，否則避免使用技術術語，並對使用的任何術語提供清楚的解釋。目標是幫助讀者在高層次上理解程式碼的作用和運作方式。",
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
        system="您的任務是接收提供的程式碼片段，並用簡單易懂的語言進行解釋。分解程式碼的功能、目的和關鍵組件。使用類比、範例和通俗術語，讓程式設計知識有限的人也能理解解釋內容。除非絕對必要，否則避免使用技術術語，並對使用的任何術語提供清楚的解釋。目標是幫助讀者在高層次上理解程式碼的作用和運作方式。",
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
      system: "您的任務是接收提供的程式碼片段，並用簡單易懂的語言進行解釋。分解程式碼的功能、目的和關鍵組件。使用類比、範例和通俗術語，讓程式設計知識有限的人也能理解解釋內容。除非絕對必要，否則避免使用技術術語，並對使用的任何術語提供清楚的解釋。目標是幫助讀者在高層次上理解程式碼的作用和運作方式。",
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