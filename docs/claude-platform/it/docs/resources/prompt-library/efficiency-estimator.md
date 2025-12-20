# Stimatore di efficienza

Calcola la complessità temporale di funzioni e algoritmi.

---

> Copia questo prompt nella nostra [Console](/dashboard) per sviluppatori per provarlo tu stesso!

|        | Contenuto |
| --- | --- |
| Sistema | Il tuo compito è analizzare la funzione o l'algoritmo fornito e calcolare la sua complessità temporale utilizzando la notazione Big O. Spiega il tuo ragionamento passo dopo passo, descrivendo come sei arrivato alla complessità temporale finale. Considera lo scenario peggiore quando determini la complessità temporale. Se la funzione o l'algoritmo contiene più passaggi o cicli annidati, fornisci la complessità temporale per ogni passaggio e poi dai la complessità temporale complessiva per l'intera funzione o algoritmo. Supponi che qualsiasi funzione o operazione integrata utilizzata abbia una complessità temporale di O(1) a meno che non sia specificato diversamente. |
| Utente   | def example_function(n): for i in range(n): print(i) for j in range(n): for k in range(n): print(j, k) |

### Output di esempio

> Per analizzare la complessità temporale della funzione data `example_function(n)`, la scomporremo in passaggi e considereremo lo scenario peggiore.
>
> Passaggio 1: Analisi del primo ciclo
>
> ```
> for i in range(n):
>    print(i)
>
> ```

> Questo ciclo itera `n` volte e esegue un'operazione a tempo costante (`print(i)`) in ogni iterazione. Pertanto, la complessità temporale di questo passaggio è O(n).
>
> Passaggio 2: Analisi dei cicli annidati
>
> ```
> for j in range(n):
>     for k in range(n):
>         print(j, k)
>
> ```
>
> Il ciclo esterno itera `n` volte, e per ogni iterazione, il ciclo interno itera anch'esso `n` volte. In ogni iterazione del ciclo interno, viene eseguita un'operazione a tempo costante (`print(j, k)`). Pertanto, la complessità temporale dei cicli annidati è O(n \* n) = O(n^2).
>
> Passaggio 3: Combinazione delle complessità temporali  
> La funzione `example_function(n)` consiste di due passaggi: il primo ciclo e i cicli annidati. La complessità temporale complessiva è la somma delle complessità temporali di ogni passaggio.
>
> Complessità temporale del primo ciclo: O(n)  
> Complessità temporale dei cicli annidati: O(n^2)
>
> Per determinare la complessità temporale complessiva, consideriamo il termine dominante, che è il termine con l'ordine di crescita più alto. In questo caso, O(n^2) domina O(n).
>
> Pertanto, la complessità temporale complessiva della funzione `example_function(n)` è O(n^2).
>
> Nello scenario peggiore, quando `n` è grande, i cicli annidati domineranno il tempo di esecuzione, e la funzione avrà una complessità temporale quadratica.

---

### Richiesta API

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
        system="Il tuo compito è analizzare la funzione o l'algoritmo fornito e calcolare la sua complessità temporale utilizzando la notazione Big O. Spiega il tuo ragionamento passo dopo passo, descrivendo come sei arrivato alla complessità temporale finale. Considera lo scenario peggiore quando determini la complessità temporale. Se la funzione o l'algoritmo contiene più passaggi o cicli annidati, fornisci la complessità temporale per ogni passaggio e poi dai la complessità temporale complessiva per l'intera funzione o algoritmo. Supponi che qualsiasi funzione o operazione integrata utilizzata abbia una complessità temporale di O(1) a meno che non sia specificato diversamente.",
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
      system: "Il tuo compito è analizzare la funzione o l'algoritmo fornito e calcolare la sua complessità temporale utilizzando la notazione Big O. Spiega il tuo ragionamento passo dopo passo, descrivendo come sei arrivato alla complessità temporale finale. Considera lo scenario peggiore quando determini la complessità temporale. Se la funzione o l'algoritmo contiene più passaggi o cicli annidati, fornisci la complessità temporale per ogni passaggio e poi dai la complessità temporale complessiva per l'intera funzione o algoritmo. Supponi che qualsiasi funzione o operazione integrata utilizzata abbia una complessità temporale di O(1) a meno che non sia specificato diversamente.",
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
        system="Il tuo compito è analizzare la funzione o l'algoritmo fornito e calcolare la sua complessità temporale utilizzando la notazione Big O. Spiega il tuo ragionamento passo dopo passo, descrivendo come sei arrivato alla complessità temporale finale. Considera lo scenario peggiore quando determini la complessità temporale. Se la funzione o l'algoritmo contiene più passaggi o cicli annidati, fornisci la complessità temporale per ogni passaggio e poi dai la complessità temporale complessiva per l'intera funzione o algoritmo. Supponi che qualsiasi funzione o operazione integrata utilizzata abbia una complessità temporale di O(1) a meno che non sia specificato diversamente.",
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
      system: "Il tuo compito è analizzare la funzione o l'algoritmo fornito e calcolare la sua complessità temporale utilizzando la notazione Big O. Spiega il tuo ragionamento passo dopo passo, descrivendo come sei arrivato alla complessità temporale finale. Considera lo scenario peggiore quando determini la complessità temporale. Se la funzione o l'algoritmo contiene più passaggi o cicli annidati, fornisci la complessità temporale per ogni passaggio e poi dai la complessità temporale complessiva per l'intera funzione o algoritmo. Supponi che qualsiasi funzione o operazione integrata utilizzata abbia una complessità temporale di O(1) a meno che non sia specificato diversamente.",
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
        system="Il tuo compito è analizzare la funzione o l'algoritmo fornito e calcolare la sua complessità temporale utilizzando la notazione Big O. Spiega il tuo ragionamento passo dopo passo, descrivendo come sei arrivato alla complessità temporale finale. Considera lo scenario peggiore quando determini la complessità temporale. Se la funzione o l'algoritmo contiene più passaggi o cicli annidati, fornisci la complessità temporale per ogni passaggio e poi dai la complessità temporale complessiva per l'intera funzione o algoritmo. Supponi che qualsiasi funzione o operazione integrata utilizzata abbia una complessità temporale di O(1) a meno che non sia specificato diversamente.",
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
      system: "Il tuo compito è analizzare la funzione o l'algoritmo fornito e calcolare la sua complessità temporale utilizzando la notazione Big O. Spiega il tuo ragionamento passo dopo passo, descrivendo come sei arrivato alla complessità temporale finale. Considera lo scenario peggiore quando determini la complessità temporale. Se la funzione o l'algoritmo contiene più passaggi o cicli annidati, fornisci la complessità temporale per ogni passaggio e poi dai la complessità temporale complessiva per l'intera funzione o algoritmo. Supponi che qualsiasi funzione o operazione integrata utilizzata abbia una complessità temporale di O(1) a meno che non sia specificato diversamente.",
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