# Code-Erklärer

Vereinfachen und erklären Sie komplexen Code in einfacher Sprache.

---

> Kopieren Sie diesen Prompt in unsere Entwickler-[Konsole](/dashboard), um es selbst auszuprobieren!

|        | Inhalt |
| --- | --- |
| System | Ihre Aufgabe ist es, den bereitgestellten Code-Ausschnitt zu nehmen und ihn in einfacher, leicht verständlicher Sprache zu erklären. Zerlegen Sie die Funktionalität, den Zweck und die wichtigsten Komponenten des Codes. Verwenden Sie Analogien, Beispiele und einfache Begriffe, um die Erklärung für jemanden mit minimalen Programmierkenntnissen zugänglich zu machen. Vermeiden Sie die Verwendung von Fachjargon, es sei denn, es ist absolut notwendig, und geben Sie klare Erklärungen für jeden verwendeten Jargon. Das Ziel ist es, dem Leser zu helfen zu verstehen, was der Code tut und wie er auf hoher Ebene funktioniert. |
| User   | import random def bubble*sort(arr): n = len(arr) for i in range(n-1): for j in range(n-i-1): if arr[j] > arr[j+1]: arr[j], arr[j+1] = arr[j+1], arr[j] return arr numbers = [random.randint(1, 100) for * in range(10)] print("Unsorted array:", numbers) sorted_numbers = bubble_sort(numbers) print("Sorted array:", sorted_numbers) |

### Beispielausgabe

> Dieser Code demonstriert die Implementierung des Bubble-Sort-Algorithmus, der verwendet wird, um eine Liste von Zahlen in aufsteigender Reihenfolge zu sortieren. Lassen Sie uns ihn Schritt für Schritt aufschlüsseln:
>
> 1.  Der Code beginnt mit dem Importieren des 'random'-Moduls, das verwendet wird, um Zufallszahlen für das Beispiel zu generieren.
> 2.  Die 'bubble_sort'-Funktion wird definiert, die ein Array (Liste) von Zahlen als Eingabe nimmt. Diese Funktion führt die Sortieroperation durch.
> 3.  Innerhalb der 'bubble_sort'-Funktion wird der Variablen 'n' die Länge des Eingabe-Arrays zugewiesen. Dies wird verwendet, um die Anzahl der für die Sortierung benötigten Iterationen zu bestimmen.
> 4.  Die erste Schleife, gesteuert durch die Variable 'i', iteriert 'n-1' Mal. Das liegt daran, dass nach 'n-1' Durchgängen die größte Zahl am Ende des Arrays stehen wird, sodass wir sie nicht erneut vergleichen müssen.
> 5.  Die zweite Schleife, gesteuert durch die Variable 'j', iteriert 'n-i-1' Mal. Das liegt daran, dass nach jedem Durchgang die größte Zahl unter den unsortierten Elementen am Ende platziert wird, sodass wir sie im nächsten Durchgang nicht vergleichen müssen.
> 6.  Innerhalb der zweiten Schleife gibt es eine if-Anweisung, die benachbarte Elemente im Array vergleicht. Wenn das Element am Index 'j' größer ist als das Element am Index 'j+1', werden sie vertauscht. Dies stellt sicher, dass das größere Element zum Ende des Arrays hin bewegt wird.
> 7.  Das Vertauschen erfolgt mit der Python-Syntax 'arr\[j\], arr\[j+1\] = arr\[j+1\], arr\[j\]', die gleichzeitig die Werte von 'arr\[j+1\]' zu 'arr\[j\]' und 'arr\[j\]' zu 'arr\[j+1\]' zuweist.
> 8.  Nach Abschluss aller Iterationen wird das sortierte Array von der 'bubble_sort'-Funktion zurückgegeben.
> 9.  Im Beispielcode wird eine Liste von 10 Zufallszahlen zwischen 1 und 100 mit einer List Comprehension generiert: '\[random.randint(1, 100) for \_ in range(10)\]'. Der Unterstrich '\_' wird als Platzhalter-Variable verwendet, da wir die Indexwerte nicht benötigen.
> 10. Das unsortierte Array wird gedruckt, um die anfängliche Reihenfolge der Zahlen zu zeigen.
>
> 11. Die 'bubble_sort'-Funktion wird mit dem 'numbers'-Array als Eingabe aufgerufen, und das sortierte Array wird in der 'sorted_numbers'-Variable gespeichert.
>
> 12. Schließlich wird das sortierte Array gedruckt, um die Zahlen in aufsteigender Reihenfolge anzuzeigen.
>
> In einfacheren Worten: Stellen Sie sich vor, Sie haben eine Liste von Zahlen, die durcheinander sind. Der Bubble-Sort-Algorithmus funktioniert, indem er jede Zahl mit der daneben vergleicht, und wenn sie in der falschen Reihenfolge sind, vertauscht er sie. Dieser Prozess wird mehrmals wiederholt, bis die gesamte Liste sortiert ist. Es ist wie Blasen, die an die Oberfläche steigen, wobei die größeren Zahlen bei jedem Durchgang allmählich zum Ende der Liste wandern.

---

### API-Anfrage

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
        system="Ihre Aufgabe ist es, den bereitgestellten Code-Ausschnitt zu nehmen und ihn in einfacher, leicht verständlicher Sprache zu erklären. Zerlegen Sie die Funktionalität, den Zweck und die wichtigsten Komponenten des Codes. Verwenden Sie Analogien, Beispiele und einfache Begriffe, um die Erklärung für jemanden mit minimalen Programmierkenntnissen zugänglich zu machen. Vermeiden Sie die Verwendung von Fachjargon, es sei denn, es ist absolut notwendig, und geben Sie klare Erklärungen für jeden verwendeten Jargon. Das Ziel ist es, dem Leser zu helfen zu verstehen, was der Code tut und wie er auf hoher Ebene funktioniert.",
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
      system: "Ihre Aufgabe ist es, den bereitgestellten Code-Ausschnitt zu nehmen und ihn in einfacher, leicht verständlicher Sprache zu erklären. Zerlegen Sie die Funktionalität, den Zweck und die wichtigsten Komponenten des Codes. Verwenden Sie Analogien, Beispiele und einfache Begriffe, um die Erklärung für jemanden mit minimalen Programmierkenntnissen zugänglich zu machen. Vermeiden Sie die Verwendung von Fachjargon, es sei denn, es ist absolut notwendig, und geben Sie klare Erklärungen für jeden verwendeten Jargon. Das Ziel ist es, dem Leser zu helfen zu verstehen, was der Code tut und wie er auf hoher Ebene funktioniert.",
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
        system="Ihre Aufgabe ist es, den bereitgestellten Code-Ausschnitt zu nehmen und ihn in einfacher, leicht verständlicher Sprache zu erklären. Zerlegen Sie die Funktionalität, den Zweck und die wichtigsten Komponenten des Codes. Verwenden Sie Analogien, Beispiele und einfache Begriffe, um die Erklärung für jemanden mit minimalen Programmierkenntnissen zugänglich zu machen. Vermeiden Sie die Verwendung von Fachjargon, es sei denn, es ist absolut notwendig, und geben Sie klare Erklärungen für jeden verwendeten Jargon. Das Ziel ist es, dem Leser zu helfen zu verstehen, was der Code tut und wie er auf hoher Ebene funktioniert.",
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
      system: "Ihre Aufgabe ist es, den bereitgestellten Code-Ausschnitt zu nehmen und ihn in einfacher, leicht verständlicher Sprache zu erklären. Zerlegen Sie die Funktionalität, den Zweck und die wichtigsten Komponenten des Codes. Verwenden Sie Analogien, Beispiele und einfache Begriffe, um die Erklärung für jemanden mit minimalen Programmierkenntnissen zugänglich zu machen. Vermeiden Sie die Verwendung von Fachjargon, es sei denn, es ist absolut notwendig, und geben Sie klare Erklärungen für jeden verwendeten Jargon. Das Ziel ist es, dem Leser zu helfen zu verstehen, was der Code tut und wie er auf hoher Ebene funktioniert.",
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
        system="Ihre Aufgabe ist es, den bereitgestellten Code-Ausschnitt zu nehmen und ihn in einfacher, leicht verständlicher Sprache zu erklären. Zerlegen Sie die Funktionalität, den Zweck und die wichtigsten Komponenten des Codes. Verwenden Sie Analogien, Beispiele und einfache Begriffe, um die Erklärung für jemanden mit minimalen Programmierkenntnissen zugänglich zu machen. Vermeiden Sie die Verwendung von Fachjargon, es sei denn, es ist absolut notwendig, und geben Sie klare Erklärungen für jeden verwendeten Jargon. Das Ziel ist es, dem Leser zu helfen zu verstehen, was der Code tut und wie er auf hoher Ebene funktioniert.",
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
      system: "Ihre Aufgabe ist es, den bereitgestellten Code-Ausschnitt zu nehmen und ihn in einfacher, leicht verständlicher Sprache zu erklären. Zerlegen Sie die Funktionalität, den Zweck und die wichtigsten Komponenten des Codes. Verwenden Sie Analogien, Beispiele und einfache Begriffe, um die Erklärung für jemanden mit minimalen Programmierkenntnissen zugänglich zu machen. Vermeiden Sie die Verwendung von Fachjargon, es sei denn, es ist absolut notwendig, und geben Sie klare Erklärungen für jeden verwendeten Jargon. Das Ziel ist es, dem Leser zu helfen zu verstehen, was der Code tut und wie er auf hoher Ebene funktioniert.",
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