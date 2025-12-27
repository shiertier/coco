# Пояснитель кода

Упрощайте и объясняйте сложный код простым языком.

---

> Скопируйте этот промпт в нашу [Консоль](/dashboard) разработчика, чтобы попробовать самостоятельно!

|        | Содержание |
| --- | --- |
| System | Ваша задача — взять предоставленный фрагмент кода и объяснить его простым, понятным языком. Разберите функциональность кода, его назначение и ключевые компоненты. Используйте аналогии, примеры и простые термины, чтобы сделать объяснение доступным для человека с минимальными знаниями программирования. Избегайте использования технического жаргона, если это не является абсолютно необходимым, и предоставляйте четкие объяснения для любого используемого жаргона. Цель состоит в том, чтобы помочь читателю понять, что делает код и как он работает на высоком уровне. |
| User   | import random def bubble*sort(arr): n = len(arr) for i in range(n-1): for j in range(n-i-1): if arr[j] > arr[j+1]: arr[j], arr[j+1] = arr[j+1], arr[j] return arr numbers = [random.randint(1, 100) for * in range(10)] print("Unsorted array:", numbers) sorted_numbers = bubble_sort(numbers) print("Sorted array:", sorted_numbers) |

### Пример вывода

> Этот код демонстрирует реализацию алгоритма пузырьковой сортировки, который используется для сортировки списка чисел в порядке возрастания. Давайте разберем его пошагово:
>
> 1.  Код начинается с импорта модуля 'random', который будет использоваться для генерации случайных чисел в примере.
> 2.  Определяется функция 'bubble_sort', которая принимает массив (список) чисел в качестве входных данных. Эта функция будет выполнять операцию сортировки.
> 3.  Внутри функции 'bubble_sort' переменной 'n' присваивается длина входного массива. Это будет использоваться для определения количества итераций, необходимых для сортировки.
> 4.  Первый цикл, управляемый переменной 'i', выполняется 'n-1' раз. Это потому, что после 'n-1' проходов самое большое число окажется в конце массива, поэтому нам не нужно сравнивать его снова.
> 5.  Второй цикл, управляемый переменной 'j', выполняется 'n-i-1' раз. Это потому, что после каждого прохода самое большое число среди несортированных элементов будет помещено в конец, поэтому нам не нужно сравнивать его в следующем проходе.
> 6.  Внутри второго цикла есть условие if, которое сравнивает соседние элементы в массиве. Если элемент с индексом 'j' больше элемента с индексом 'j+1', они меняются местами. Это гарантирует, что больший элемент движется к концу массива.
> 7.  Обмен выполняется с использованием синтаксиса Python 'arr\[j\], arr\[j+1\] = arr\[j+1\], arr\[j\]', который одновременно присваивает значения 'arr\[j+1\]' к 'arr\[j\]' и 'arr\[j\]' к 'arr\[j+1\]'.
> 8.  После завершения всех итераций отсортированный массив возвращается функцией 'bubble_sort'.
> 9.  В примере кода список из 10 случайных чисел от 1 до 100 генерируется с использованием генератора списка: '\[random.randint(1, 100) for \_ in range(10)\]'. Подчеркивание '\_' используется как переменная-заполнитель, поскольку нам не нужны значения индексов.
> 10. Несортированный массив выводится для показа первоначального порядка чисел.
>
> 11. Функция 'bubble_sort' вызывается с массивом 'numbers' в качестве входных данных, и отсортированный массив сохраняется в переменной 'sorted_numbers'.
>
> 12. Наконец, отсортированный массив выводится для отображения чисел в порядке возрастания.
>
> Проще говоря, представьте, что у вас есть список чисел, которые расположены не по порядку. Алгоритм пузырьковой сортировки работает, сравнивая каждое число с соседним, и если они расположены в неправильном порядке, он меняет их местами. Этот процесс повторяется несколько раз, пока весь список не будет отсортирован. Это похоже на пузырьки, поднимающиеся на поверхность, при этом большие числа постепенно перемещаются к концу списка при каждом проходе.

---

### API запрос

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
        system="Ваша задача — взять предоставленный фрагмент кода и объяснить его простым, понятным языком. Разберите функциональность кода, его назначение и ключевые компоненты. Используйте аналогии, примеры и простые термины, чтобы сделать объяснение доступным для человека с минимальными знаниями программирования. Избегайте использования технического жаргона, если это не является абсолютно необходимым, и предоставляйте четкие объяснения для любого используемого жаргона. Цель состоит в том, чтобы помочь читателю понять, что делает код и как он работает на высоком уровне.",
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
      system: "Ваша задача — взять предоставленный фрагмент кода и объяснить его простым, понятным языком. Разберите функциональность кода, его назначение и ключевые компоненты. Используйте аналогии, примеры и простые термины, чтобы сделать объяснение доступным для человека с минимальными знаниями программирования. Избегайте использования технического жаргона, если это не является абсолютно необходимым, и предоставляйте четкие объяснения для любого используемого жаргона. Цель состоит в том, чтобы помочь читателю понять, что делает код и как он работает на высоком уровне.",
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
        system="Ваша задача — взять предоставленный фрагмент кода и объяснить его простым, понятным языком. Разберите функциональность кода, его назначение и ключевые компоненты. Используйте аналогии, примеры и простые термины, чтобы сделать объяснение доступным для человека с минимальными знаниями программирования. Избегайте использования технического жаргона, если это не является абсолютно необходимым, и предоставляйте четкие объяснения для любого используемого жаргона. Цель состоит в том, чтобы помочь читателю понять, что делает код и как он работает на высоком уровне.",
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
      system: "Ваша задача — взять предоставленный фрагмент кода и объяснить его простым, понятным языком. Разберите функциональность кода, его назначение и ключевые компоненты. Используйте аналогии, примеры и простые термины, чтобы сделать объяснение доступным для человека с минимальными знаниями программирования. Избегайте использования технического жаргона, если это не является абсолютно необходимым, и предоставляйте четкие объяснения для любого используемого жаргона. Цель состоит в том, чтобы помочь читателю понять, что делает код и как он работает на высоком уровне.",
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
        system="Ваша задача — взять предоставленный фрагмент кода и объяснить его простым, понятным языком. Разберите функциональность кода, его назначение и ключевые компоненты. Используйте аналогии, примеры и простые термины, чтобы сделать объяснение доступным для человека с минимальными знаниями программирования. Избегайте использования технического жаргона, если это не является абсолютно необходимым, и предоставляйте четкие объяснения для любого используемого жаргона. Цель состоит в том, чтобы помочь читателю понять, что делает код и как он работает на высоком уровне.",
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
      system: "Ваша задача — взять предоставленный фрагмент кода и объяснить его простым, понятным языком. Разберите функциональность кода, его назначение и ключевые компоненты. Используйте аналогии, примеры и простые термины, чтобы сделать объяснение доступным для человека с минимальными знаниями программирования. Избегайте использования технического жаргона, если это не является абсолютно необходимым, и предоставляйте четкие объяснения для любого используемого жаргона. Цель состоит в том, чтобы помочь читателю понять, что делает код и как он работает на высоком уровне.",
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