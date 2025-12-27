# Estimador de eficiência

Calcule a complexidade de tempo de funções e algoritmos.

---

> Copie este prompt para nosso [Console](/dashboard) de desenvolvedor para experimentá-lo você mesmo!

|        | Conteúdo |
| --- | --- |
| Sistema | Sua tarefa é analisar a função ou algoritmo fornecido e calcular sua complexidade de tempo usando a notação Big O. Explique seu raciocínio passo a passo, descrevendo como chegou à complexidade de tempo final. Considere o cenário de pior caso ao determinar a complexidade de tempo. Se a função ou algoritmo contém múltiplas etapas ou loops aninhados, forneça a complexidade de tempo para cada etapa e então dê a complexidade de tempo geral para toda a função ou algoritmo. Assuma que quaisquer funções ou operações integradas usadas têm uma complexidade de tempo de O(1) a menos que especificado de outra forma. |
| Usuário   | def example_function(n): for i in range(n): print(i) for j in range(n): for k in range(n): print(j, k) |

### Exemplo de saída

> Para analisar a complexidade de tempo da função dada `example_function(n)`, vamos dividi-la em etapas e considerar o cenário de pior caso.
>
> Etapa 1: Analisando o primeiro loop
>
> ```
> for i in range(n):
>    print(i)
>
> ```

> Este loop itera `n` vezes e executa uma operação de tempo constante (`print(i)`) em cada iteração. Portanto, a complexidade de tempo desta etapa é O(n).
>
> Etapa 2: Analisando os loops aninhados
>
> ```
> for j in range(n):
>     for k in range(n):
>         print(j, k)
>
> ```
>
> O loop externo itera `n` vezes, e para cada iteração, o loop interno também itera `n` vezes. Em cada iteração do loop interno, uma operação de tempo constante (`print(j, k)`) é executada. Portanto, a complexidade de tempo dos loops aninhados é O(n \* n) = O(n^2).
>
> Etapa 3: Combinando as complexidades de tempo  
> A função `example_function(n)` consiste em duas etapas: o primeiro loop e os loops aninhados. A complexidade de tempo geral é a soma das complexidades de tempo de cada etapa.
>
> Complexidade de tempo do primeiro loop: O(n)  
> Complexidade de tempo dos loops aninhados: O(n^2)
>
> Para determinar a complexidade de tempo geral, consideramos o termo dominante, que é o termo com a maior ordem de crescimento. Neste caso, O(n^2) domina O(n).
>
> Portanto, a complexidade de tempo geral da função `example_function(n)` é O(n^2).
>
> No cenário de pior caso, quando `n` é grande, os loops aninhados dominarão o tempo de execução, e a função terá uma complexidade de tempo quadrática.

---

### Solicitação de API

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
        system="Sua tarefa é analisar a função ou algoritmo fornecido e calcular sua complexidade de tempo usando a notação Big O. Explique seu raciocínio passo a passo, descrevendo como chegou à complexidade de tempo final. Considere o cenário de pior caso ao determinar a complexidade de tempo. Se a função ou algoritmo contém múltiplas etapas ou loops aninhados, forneça a complexidade de tempo para cada etapa e então dê a complexidade de tempo geral para toda a função ou algoritmo. Assuma que quaisquer funções ou operações integradas usadas têm uma complexidade de tempo de O(1) a menos que especificado de outra forma.",
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
      system: "Sua tarefa é analisar a função ou algoritmo fornecido e calcular sua complexidade de tempo usando a notação Big O. Explique seu raciocínio passo a passo, descrevendo como chegou à complexidade de tempo final. Considere o cenário de pior caso ao determinar a complexidade de tempo. Se a função ou algoritmo contém múltiplas etapas ou loops aninhados, forneça a complexidade de tempo para cada etapa e então dê a complexidade de tempo geral para toda a função ou algoritmo. Assuma que quaisquer funções ou operações integradas usadas têm uma complexidade de tempo de O(1) a menos que especificado de outra forma.",
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
        system="Sua tarefa é analisar a função ou algoritmo fornecido e calcular sua complexidade de tempo usando a notação Big O. Explique seu raciocínio passo a passo, descrevendo como chegou à complexidade de tempo final. Considere o cenário de pior caso ao determinar a complexidade de tempo. Se a função ou algoritmo contém múltiplas etapas ou loops aninhados, forneça a complexidade de tempo para cada etapa e então dê a complexidade de tempo geral para toda a função ou algoritmo. Assuma que quaisquer funções ou operações integradas usadas têm uma complexidade de tempo de O(1) a menos que especificado de outra forma.",
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
      system: "Sua tarefa é analisar a função ou algoritmo fornecido e calcular sua complexidade de tempo usando a notação Big O. Explique seu raciocínio passo a passo, descrevendo como chegou à complexidade de tempo final. Considere o cenário de pior caso ao determinar a complexidade de tempo. Se a função ou algoritmo contém múltiplas etapas ou loops aninhados, forneça a complexidade de tempo para cada etapa e então dê a complexidade de tempo geral para toda a função ou algoritmo. Assuma que quaisquer funções ou operações integradas usadas têm uma complexidade de tempo de O(1) a menos que especificado de outra forma.",
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
        system="Sua tarefa é analisar a função ou algoritmo fornecido e calcular sua complexidade de tempo usando a notação Big O. Explique seu raciocínio passo a passo, descrevendo como chegou à complexidade de tempo final. Considere o cenário de pior caso ao determinar a complexidade de tempo. Se a função ou algoritmo contém múltiplas etapas ou loops aninhados, forneça a complexidade de tempo para cada etapa e então dê a complexidade de tempo geral para toda a função ou algoritmo. Assuma que quaisquer funções ou operações integradas usadas têm uma complexidade de tempo de O(1) a menos que especificado de outra forma.",
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
      system: "Sua tarefa é analisar a função ou algoritmo fornecido e calcular sua complexidade de tempo usando a notação Big O. Explique seu raciocínio passo a passo, descrevendo como chegou à complexidade de tempo final. Considere o cenário de pior caso ao determinar a complexidade de tempo. Se a função ou algoritmo contém múltiplas etapas ou loops aninhados, forneça a complexidade de tempo para cada etapa e então dê a complexidade de tempo geral para toda a função ou algoritmo. Assuma que quaisquer funções ou operações integradas usadas têm uma complexidade de tempo de O(1) a menos que especificado de outra forma.",
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