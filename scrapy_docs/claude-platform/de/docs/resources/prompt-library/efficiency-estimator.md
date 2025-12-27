# Effizienz-Schätzer

Berechne die Zeitkomplexität von Funktionen und Algorithmen.

---

> Kopiere diesen Prompt in unsere Entwickler-[Konsole](/dashboard), um es selbst auszuprobieren!

|        | Inhalt |
| --- | --- |
| System | Deine Aufgabe ist es, die bereitgestellte Funktion oder den Algorithmus zu analysieren und deren Zeitkomplexität mit der Big O-Notation zu berechnen. Erkläre deine Überlegungen Schritt für Schritt und beschreibe, wie du zur finalen Zeitkomplexität gelangt bist. Berücksichtige das Worst-Case-Szenario bei der Bestimmung der Zeitkomplexität. Wenn die Funktion oder der Algorithmus mehrere Schritte oder verschachtelte Schleifen enthält, gib die Zeitkomplexität für jeden Schritt an und dann die Gesamtzeitkomplexität für die gesamte Funktion oder den Algorithmus. Nimm an, dass alle verwendeten eingebauten Funktionen oder Operationen eine Zeitkomplexität von O(1) haben, sofern nicht anders angegeben. |
| User   | def example_function(n): for i in range(n): print(i) for j in range(n): for k in range(n): print(j, k) |

### Beispielausgabe

> Um die Zeitkomplexität der gegebenen Funktion `example_function(n)` zu analysieren, werden wir sie in Schritte aufteilen und das Worst-Case-Szenario betrachten.
>
> Schritt 1: Analyse der ersten Schleife
>
> ```
> for i in range(n):
>    print(i)
>
> ```

> Diese Schleife iteriert `n` mal und führt in jeder Iteration eine Operation mit konstanter Zeit (`print(i)`) aus. Daher ist die Zeitkomplexität dieses Schritts O(n).
>
> Schritt 2: Analyse der verschachtelten Schleifen
>
> ```
> for j in range(n):
>     for k in range(n):
>         print(j, k)
>
> ```
>
> Die äußere Schleife iteriert `n` mal, und für jede Iteration iteriert die innere Schleife ebenfalls `n` mal. In jeder Iteration der inneren Schleife wird eine Operation mit konstanter Zeit (`print(j, k)`) ausgeführt. Daher ist die Zeitkomplexität der verschachtelten Schleifen O(n \* n) = O(n^2).
>
> Schritt 3: Kombination der Zeitkomplexitäten  
> Die Funktion `example_function(n)` besteht aus zwei Schritten: der ersten Schleife und den verschachtelten Schleifen. Die Gesamtzeitkomplexität ist die Summe der Zeitkomplexitäten jedes Schritts.
>
> Zeitkomplexität der ersten Schleife: O(n)  
> Zeitkomplexität der verschachtelten Schleifen: O(n^2)
>
> Um die Gesamtzeitkomplexität zu bestimmen, betrachten wir den dominierenden Term, welcher der Term mit der höchsten Wachstumsordnung ist. In diesem Fall dominiert O(n^2) über O(n).
>
> Daher ist die Gesamtzeitkomplexität der Funktion `example_function(n)` O(n^2).
>
> Im Worst-Case-Szenario, wenn `n` groß ist, werden die verschachtelten Schleifen die Ausführungszeit dominieren, und die Funktion wird eine quadratische Zeitkomplexität haben.

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
        system="Deine Aufgabe ist es, die bereitgestellte Funktion oder den Algorithmus zu analysieren und deren Zeitkomplexität mit der Big O-Notation zu berechnen. Erkläre deine Überlegungen Schritt für Schritt und beschreibe, wie du zur finalen Zeitkomplexität gelangt bist. Berücksichtige das Worst-Case-Szenario bei der Bestimmung der Zeitkomplexität. Wenn die Funktion oder der Algorithmus mehrere Schritte oder verschachtelte Schleifen enthält, gib die Zeitkomplexität für jeden Schritt an und dann die Gesamtzeitkomplexität für die gesamte Funktion oder den Algorithmus. Nimm an, dass alle verwendeten eingebauten Funktionen oder Operationen eine Zeitkomplexität von O(1) haben, sofern nicht anders angegeben.",
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
      system: "Deine Aufgabe ist es, die bereitgestellte Funktion oder den Algorithmus zu analysieren und deren Zeitkomplexität mit der Big O-Notation zu berechnen. Erkläre deine Überlegungen Schritt für Schritt und beschreibe, wie du zur finalen Zeitkomplexität gelangt bist. Berücksichtige das Worst-Case-Szenario bei der Bestimmung der Zeitkomplexität. Wenn die Funktion oder der Algorithmus mehrere Schritte oder verschachtelte Schleifen enthält, gib die Zeitkomplexität für jeden Schritt an und dann die Gesamtzeitkomplexität für die gesamte Funktion oder den Algorithmus. Nimm an, dass alle verwendeten eingebauten Funktionen oder Operationen eine Zeitkomplexität von O(1) haben, sofern nicht anders angegeben.",
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
        system="Deine Aufgabe ist es, die bereitgestellte Funktion oder den Algorithmus zu analysieren und deren Zeitkomplexität mit der Big O-Notation zu berechnen. Erkläre deine Überlegungen Schritt für Schritt und beschreibe, wie du zur finalen Zeitkomplexität gelangt bist. Berücksichtige das Worst-Case-Szenario bei der Bestimmung der Zeitkomplexität. Wenn die Funktion oder der Algorithmus mehrere Schritte oder verschachtelte Schleifen enthält, gib die Zeitkomplexität für jeden Schritt an und dann die Gesamtzeitkomplexität für die gesamte Funktion oder den Algorithmus. Nimm an, dass alle verwendeten eingebauten Funktionen oder Operationen eine Zeitkomplexität von O(1) haben, sofern nicht anders angegeben.",
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
      system: "Deine Aufgabe ist es, die bereitgestellte Funktion oder den Algorithmus zu analysieren und deren Zeitkomplexität mit der Big O-Notation zu berechnen. Erkläre deine Überlegungen Schritt für Schritt und beschreibe, wie du zur finalen Zeitkomplexität gelangt bist. Berücksichtige das Worst-Case-Szenario bei der Bestimmung der Zeitkomplexität. Wenn die Funktion oder der Algorithmus mehrere Schritte oder verschachtelte Schleifen enthält, gib die Zeitkomplexität für jeden Schritt an und dann die Gesamtzeitkomplexität für die gesamte Funktion oder den Algorithmus. Nimm an, dass alle verwendeten eingebauten Funktionen oder Operationen eine Zeitkomplexität von O(1) haben, sofern nicht anders angegeben.",
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
        system="Deine Aufgabe ist es, die bereitgestellte Funktion oder den Algorithmus zu analysieren und deren Zeitkomplexität mit der Big O-Notation zu berechnen. Erkläre deine Überlegungen Schritt für Schritt und beschreibe, wie du zur finalen Zeitkomplexität gelangt bist. Berücksichtige das Worst-Case-Szenario bei der Bestimmung der Zeitkomplexität. Wenn die Funktion oder der Algorithmus mehrere Schritte oder verschachtelte Schleifen enthält, gib die Zeitkomplexität für jeden Schritt an und dann die Gesamtzeitkomplexität für die gesamte Funktion oder den Algorithmus. Nimm an, dass alle verwendeten eingebauten Funktionen oder Operationen eine Zeitkomplexität von O(1) haben, sofern nicht anders angegeben.",
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
      system: "Deine Aufgabe ist es, die bereitgestellte Funktion oder den Algorithmus zu analysieren und deren Zeitkomplexität mit der Big O-Notation zu berechnen. Erkläre deine Überlegungen Schritt für Schritt und beschreibe, wie du zur finalen Zeitkomplexität gelangt bist. Berücksichtige das Worst-Case-Szenario bei der Bestimmung der Zeitkomplexität. Wenn die Funktion oder der Algorithmus mehrere Schritte oder verschachtelte Schleifen enthält, gib die Zeitkomplexität für jeden Schritt an und dann die Gesamtzeitkomplexität für die gesamte Funktion oder den Algorithmus. Nimm an, dass alle verwendeten eingebauten Funktionen oder Operationen eine Zeitkomplexität von O(1) haben, sofern nicht anders angegeben.",
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