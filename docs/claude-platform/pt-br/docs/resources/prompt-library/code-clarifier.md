# Esclarecedor de código

Simplifique e explique código complexo em linguagem simples.

---

> Copie este prompt para nosso [Console](/dashboard) de desenvolvedor para experimentá-lo você mesmo!

|        | Conteúdo |
| --- | --- |
| System | Sua tarefa é pegar o trecho de código fornecido e explicá-lo em linguagem simples e fácil de entender. Detalhe a funcionalidade, propósito e componentes principais do código. Use analogias, exemplos e termos simples para tornar a explicação acessível a alguém com conhecimento mínimo de programação. Evite usar jargão técnico, a menos que seja absolutamente necessário, e forneça explicações claras para qualquer jargão usado. O objetivo é ajudar o leitor a entender o que o código faz e como funciona em alto nível. |
| User   | import random def bubble*sort(arr): n = len(arr) for i in range(n-1): for j in range(n-i-1): if arr[j] > arr[j+1]: arr[j], arr[j+1] = arr[j+1], arr[j] return arr numbers = [random.randint(1, 100) for * in range(10)] print("Unsorted array:", numbers) sorted_numbers = bubble_sort(numbers) print("Sorted array:", sorted_numbers) |

### Exemplo de Saída

> Este código demonstra a implementação do algoritmo bubble sort, que é usado para ordenar uma lista de números em ordem crescente. Vamos detalhar passo a passo:
>
> 1.  O código começa importando o módulo 'random', que será usado para gerar números aleatórios para o exemplo.
> 2.  A função 'bubble_sort' é definida, que recebe um array (lista) de números como entrada. Esta função realizará a operação de ordenação.
> 3.  Dentro da função 'bubble_sort', a variável 'n' recebe o comprimento do array de entrada. Isso será usado para determinar o número de iterações necessárias para a ordenação.
> 4.  O primeiro loop, controlado pela variável 'i', itera 'n-1' vezes. Isso ocorre porque após 'n-1' passadas, o maior número estará no final do array, então não precisamos compará-lo novamente.
> 5.  O segundo loop, controlado pela variável 'j', itera 'n-i-1' vezes. Isso ocorre porque após cada passada, o maior número entre os elementos não ordenados será colocado no final, então não precisamos compará-lo na próxima passada.
> 6.  Dentro do segundo loop, há uma declaração if que compara elementos adjacentes no array. Se o elemento no índice 'j' for maior que o elemento no índice 'j+1', eles são trocados. Isso garante que o elemento maior se mova em direção ao final do array.
> 7.  A troca é feita usando a sintaxe Python 'arr\[j\], arr\[j+1\] = arr\[j+1\], arr\[j\]', que simultaneamente atribui os valores de 'arr\[j+1\]' para 'arr\[j\]' e 'arr\[j\]' para 'arr\[j+1\]'.
> 8.  Após todas as iterações serem concluídas, o array ordenado é retornado pela função 'bubble_sort'.
> 9.  No código de exemplo, uma lista de 10 números aleatórios entre 1 e 100 é gerada usando uma list comprehension: '\[random.randint(1, 100) for \_ in range(10)\]'. O sublinhado '\_' é usado como uma variável placeholder já que não precisamos dos valores de índice.
> 10. O array não ordenado é impresso para mostrar a ordem inicial dos números.
>
> 11. A função 'bubble_sort' é chamada com o array 'numbers' como entrada, e o array ordenado é armazenado na variável 'sorted_numbers'.
>
> 12. Finalmente, o array ordenado é impresso para exibir os números em ordem crescente.
>
> Em termos mais simples, imagine que você tem uma lista de números que estão fora de ordem. O algoritmo bubble sort funciona comparando cada número com o próximo, e se eles estão na ordem errada, ele os troca. Este processo é repetido várias vezes até que toda a lista esteja ordenada. É como bolhas subindo à superfície, com os números maiores gradualmente se movendo em direção ao final da lista em cada passada.

---

### Solicitação da API

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
        system="Sua tarefa é pegar o trecho de código fornecido e explicá-lo em linguagem simples e fácil de entender. Detalhe a funcionalidade, propósito e componentes principais do código. Use analogias, exemplos e termos simples para tornar a explicação acessível a alguém com conhecimento mínimo de programação. Evite usar jargão técnico, a menos que seja absolutamente necessário, e forneça explicações claras para qualquer jargão usado. O objetivo é ajudar o leitor a entender o que o código faz e como funciona em alto nível.",
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
      system: "Sua tarefa é pegar o trecho de código fornecido e explicá-lo em linguagem simples e fácil de entender. Detalhe a funcionalidade, propósito e componentes principais do código. Use analogias, exemplos e termos simples para tornar a explicação acessível a alguém com conhecimento mínimo de programação. Evite usar jargão técnico, a menos que seja absolutamente necessário, e forneça explicações claras para qualquer jargão usado. O objetivo é ajudar o leitor a entender o que o código faz e como funciona em alto nível.",
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
        system="Sua tarefa é pegar o trecho de código fornecido e explicá-lo em linguagem simples e fácil de entender. Detalhe a funcionalidade, propósito e componentes principais do código. Use analogias, exemplos e termos simples para tornar a explicação acessível a alguém com conhecimento mínimo de programação. Evite usar jargão técnico, a menos que seja absolutamente necessário, e forneça explicações claras para qualquer jargão usado. O objetivo é ajudar o leitor a entender o que o código faz e como funciona em alto nível.",
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
      system: "Sua tarefa é pegar o trecho de código fornecido e explicá-lo em linguagem simples e fácil de entender. Detalhe a funcionalidade, propósito e componentes principais do código. Use analogias, exemplos e termos simples para tornar a explicação acessível a alguém com conhecimento mínimo de programação. Evite usar jargão técnico, a menos que seja absolutamente necessário, e forneça explicações claras para qualquer jargão usado. O objetivo é ajudar o leitor a entender o que o código faz e como funciona em alto nível.",
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
        system="Sua tarefa é pegar o trecho de código fornecido e explicá-lo em linguagem simples e fácil de entender. Detalhe a funcionalidade, propósito e componentes principais do código. Use analogias, exemplos e termos simples para tornar a explicação acessível a alguém com conhecimento mínimo de programação. Evite usar jargão técnico, a menos que seja absolutamente necessário, e forneça explicações claras para qualquer jargão usado. O objetivo é ajudar o leitor a entender o que o código faz e como funciona em alto nível.",
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
      system: "Sua tarefa é pegar o trecho de código fornecido e explicá-lo em linguagem simples e fácil de entender. Detalhe a funcionalidade, propósito e componentes principais do código. Use analogias, exemplos e termos simples para tornar a explicação acessível a alguém com conhecimento mínimo de programação. Evite usar jargão técnico, a menos que seja absolutamente necessário, e forneça explicações claras para qualquer jargão usado. O objetivo é ajudar o leitor a entender o que o código faz e como funciona em alto nível.",
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