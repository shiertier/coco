# Chiarificatore di codice

Semplifica e spiega codice complesso in linguaggio semplice.

---

> Copia questo prompt nella nostra [Console](/dashboard) per sviluppatori per provarlo tu stesso!

|        | Contenuto |
| --- | --- |
| System | Il tuo compito è prendere il frammento di codice fornito e spiegarlo in un linguaggio semplice e facile da capire. Scomponi la funzionalità, lo scopo e i componenti chiave del codice. Usa analogie, esempi e termini semplici per rendere la spiegazione accessibile a qualcuno con conoscenze minime di programmazione. Evita di usare gergo tecnico a meno che non sia assolutamente necessario, e fornisci spiegazioni chiare per qualsiasi gergo utilizzato. L'obiettivo è aiutare il lettore a capire cosa fa il codice e come funziona ad alto livello. |
| User   | import random def bubble*sort(arr): n = len(arr) for i in range(n-1): for j in range(n-i-1): if arr[j] > arr[j+1]: arr[j], arr[j+1] = arr[j+1], arr[j] return arr numbers = [random.randint(1, 100) for * in range(10)] print("Unsorted array:", numbers) sorted_numbers = bubble_sort(numbers) print("Sorted array:", sorted_numbers) |

### Esempio di Output

> Questo codice dimostra l'implementazione dell'algoritmo bubble sort, che viene utilizzato per ordinare una lista di numeri in ordine crescente. Analizziamolo passo dopo passo:
>
> 1.  Il codice inizia importando il modulo 'random', che verrà utilizzato per generare numeri casuali per l'esempio.
> 2.  Viene definita la funzione 'bubble_sort', che prende un array (lista) di numeri come input. Questa funzione eseguirà l'operazione di ordinamento.
> 3.  All'interno della funzione 'bubble_sort', alla variabile 'n' viene assegnata la lunghezza dell'array di input. Questo verrà utilizzato per determinare il numero di iterazioni necessarie per l'ordinamento.
> 4.  Il primo ciclo, controllato dalla variabile 'i', itera 'n-1' volte. Questo perché dopo 'n-1' passaggi, il numero più grande sarà alla fine dell'array, quindi non abbiamo bisogno di confrontarlo di nuovo.
> 5.  Il secondo ciclo, controllato dalla variabile 'j', itera 'n-i-1' volte. Questo perché dopo ogni passaggio, il numero più grande tra gli elementi non ordinati verrà posizionato alla fine, quindi non abbiamo bisogno di confrontarlo nel passaggio successivo.
> 6.  All'interno del secondo ciclo, c'è un'istruzione if che confronta elementi adiacenti nell'array. Se l'elemento all'indice 'j' è maggiore dell'elemento all'indice 'j+1', vengono scambiati. Questo assicura che l'elemento più grande si sposti verso la fine dell'array.
> 7.  Lo scambio viene fatto usando la sintassi Python 'arr\[j\], arr\[j+1\] = arr\[j+1\], arr\[j\]', che assegna simultaneamente i valori di 'arr\[j+1\]' a 'arr\[j\]' e 'arr\[j\]' a 'arr\[j+1\]'.
> 8.  Dopo che tutte le iterazioni sono complete, l'array ordinato viene restituito dalla funzione 'bubble_sort'.
> 9.  Nel codice di esempio, viene generata una lista di 10 numeri casuali tra 1 e 100 usando una list comprehension: '\[random.randint(1, 100) for \_ in range(10)\]'. Il trattino basso '\_' viene utilizzato come variabile segnaposto poiché non abbiamo bisogno dei valori degli indici.
> 10. L'array non ordinato viene stampato per mostrare l'ordine iniziale dei numeri.
>
> 11. La funzione 'bubble_sort' viene chiamata con l'array 'numbers' come input, e l'array ordinato viene memorizzato nella variabile 'sorted_numbers'.
>
> 12. Infine, l'array ordinato viene stampato per visualizzare i numeri in ordine crescente.
>
> In termini più semplici, immagina di avere una lista di numeri che sono fuori ordine. L'algoritmo bubble sort funziona confrontando ogni numero con quello accanto, e se sono nell'ordine sbagliato, li scambia. Questo processo viene ripetuto più volte fino a quando l'intera lista è ordinata. È come bolle che salgono in superficie, con i numeri più grandi che gradualmente si spostano verso la fine della lista in ogni passaggio.

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
        system="Il tuo compito è prendere il frammento di codice fornito e spiegarlo in un linguaggio semplice e facile da capire. Scomponi la funzionalità, lo scopo e i componenti chiave del codice. Usa analogie, esempi e termini semplici per rendere la spiegazione accessibile a qualcuno con conoscenze minime di programmazione. Evita di usare gergo tecnico a meno che non sia assolutamente necessario, e fornisci spiegazioni chiare per qualsiasi gergo utilizzato. L'obiettivo è aiutare il lettore a capire cosa fa il codice e come funziona ad alto livello.",
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
      system: "Il tuo compito è prendere il frammento di codice fornito e spiegarlo in un linguaggio semplice e facile da capire. Scomponi la funzionalità, lo scopo e i componenti chiave del codice. Usa analogie, esempi e termini semplici per rendere la spiegazione accessibile a qualcuno con conoscenze minime di programmazione. Evita di usare gergo tecnico a meno che non sia assolutamente necessario, e fornisci spiegazioni chiare per qualsiasi gergo utilizzato. L'obiettivo è aiutare il lettore a capire cosa fa il codice e come funziona ad alto livello.",
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
        system="Il tuo compito è prendere il frammento di codice fornito e spiegarlo in un linguaggio semplice e facile da capire. Scomponi la funzionalità, lo scopo e i componenti chiave del codice. Usa analogie, esempi e termini semplici per rendere la spiegazione accessibile a qualcuno con conoscenze minime di programmazione. Evita di usare gergo tecnico a meno che non sia assolutamente necessario, e fornisci spiegazioni chiare per qualsiasi gergo utilizzato. L'obiettivo è aiutare il lettore a capire cosa fa il codice e come funziona ad alto livello.",
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
      system: "Il tuo compito è prendere il frammento di codice fornito e spiegarlo in un linguaggio semplice e facile da capire. Scomponi la funzionalità, lo scopo e i componenti chiave del codice. Usa analogie, esempi e termini semplici per rendere la spiegazione accessibile a qualcuno con conoscenze minime di programmazione. Evita di usare gergo tecnico a meno che non sia assolutamente necessario, e fornisci spiegazioni chiare per qualsiasi gergo utilizzato. L'obiettivo è aiutare il lettore a capire cosa fa il codice e come funziona ad alto livello.",
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
        system="Il tuo compito è prendere il frammento di codice fornito e spiegarlo in un linguaggio semplice e facile da capire. Scomponi la funzionalità, lo scopo e i componenti chiave del codice. Usa analogie, esempi e termini semplici per rendere la spiegazione accessibile a qualcuno con conoscenze minime di programmazione. Evita di usare gergo tecnico a meno che non sia assolutamente necessario, e fornisci spiegazioni chiare per qualsiasi gergo utilizzato. L'obiettivo è aiutare il lettore a capire cosa fa il codice e come funziona ad alto livello.",
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
      system: "Il tuo compito è prendere il frammento di codice fornito e spiegarlo in un linguaggio semplice e facile da capire. Scomponi la funzionalità, lo scopo e i componenti chiave del codice. Usa analogie, esempi e termini semplici per rendere la spiegazione accessibile a qualcuno con conoscenze minime di programmazione. Evita di usare gergo tecnico a meno che non sia assolutamente necessario, e fornisci spiegazioni chiare per qualsiasi gergo utilizzato. L'obiettivo è aiutare il lettore a capire cosa fa il codice e come funziona ad alto livello.",
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