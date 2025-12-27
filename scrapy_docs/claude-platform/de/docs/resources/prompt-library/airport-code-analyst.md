# Flughafencode-Analyst

Finde und extrahiere Flughafencodes aus Text.

---

> Kopiere diesen Prompt in unsere Entwickler-[Konsole](/dashboard), um es selbst auszuprobieren!

|        | Inhalt |
| --- | --- |
| System | Deine Aufgabe ist es, den bereitgestellten Text zu analysieren und alle darin erwähnten Flughafencodes zu identifizieren. Präsentiere diese Flughafencodes als Liste in der Reihenfolge, in der sie im Text erscheinen. Wenn keine Flughafencodes gefunden werden, gib eine leere Liste zurück. |
| User   | Meine nächste Reise beinhaltet einen Flug von Seattle nach Amsterdam. Ich werde ein paar Tage in Amsterdam verbringen, bevor ich nach Paris weiterreise für einen Anschlussflug nach Rom. |

### Beispielausgabe

> Hier ist die Liste der im Text erwähnten Flughafencodes in der Reihenfolge ihres Erscheinens:
>
> 1. SEA (Seattle)
> 2. AMS (Amsterdam)
> 3. CDG (Paris)
> 4. FCO (Rom)

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
        system="Deine Aufgabe ist es, den bereitgestellten Text zu analysieren und alle darin erwähnten Flughafencodes zu identifizieren. Präsentiere diese Flughafencodes als Liste in der Reihenfolge, in der sie im Text erscheinen. Wenn keine Flughafencodes gefunden werden, gib eine leere Liste zurück.",
        messages=[
            {
                "role": "user",
                "content": [
                    {
                        "type": "text",
                        "text": "Meine nächste Reise beinhaltet einen Flug von Seattle nach Amsterdam. Ich werde ein paar Tage in Amsterdam verbringen, bevor ich nach Paris weiterreise für einen Anschlussflug nach Rom."
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
      system: "Deine Aufgabe ist es, den bereitgestellten Text zu analysieren und alle darin erwähnten Flughafencodes zu identifizieren. Präsentiere diese Flughafencodes als Liste in der Reihenfolge, in der sie im Text erscheinen. Wenn keine Flughafencodes gefunden werden, gib eine leere Liste zurück.",
      messages: [
        {
          "role": "user",
          "content": [
            {
              "type": "text",
              "text": "Meine nächste Reise beinhaltet einen Flug von Seattle nach Amsterdam. Ich werde ein paar Tage in Amsterdam verbringen, bevor ich nach Paris weiterreise für einen Anschlussflug nach Rom."
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
        system="Deine Aufgabe ist es, den bereitgestellten Text zu analysieren und alle darin erwähnten Flughafencodes zu identifizieren. Präsentiere diese Flughafencodes als Liste in der Reihenfolge, in der sie im Text erscheinen. Wenn keine Flughafencodes gefunden werden, gib eine leere Liste zurück.",
        messages=[
            {
                "role": "user",
                "content": [
                    {
                        "type": "text",
                        "text": "Meine nächste Reise beinhaltet einen Flug von Seattle nach Amsterdam. Ich werde ein paar Tage in Amsterdam verbringen, bevor ich nach Paris weiterreise für einen Anschlussflug nach Rom."
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
      system: "Deine Aufgabe ist es, den bereitgestellten Text zu analysieren und alle darin erwähnten Flughafencodes zu identifizieren. Präsentiere diese Flughafencodes als Liste in der Reihenfolge, in der sie im Text erscheinen. Wenn keine Flughafencodes gefunden werden, gib eine leere Liste zurück.",
      messages: [
        {
          "role": "user",
          "content": [
            {
              "type": "text",
              "text": "Meine nächste Reise beinhaltet einen Flug von Seattle nach Amsterdam. Ich werde ein paar Tage in Amsterdam verbringen, bevor ich nach Paris weiterreise für einen Anschlussflug nach Rom."
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
        system="Deine Aufgabe ist es, den bereitgestellten Text zu analysieren und alle darin erwähnten Flughafencodes zu identifizieren. Präsentiere diese Flughafencodes als Liste in der Reihenfolge, in der sie im Text erscheinen. Wenn keine Flughafencodes gefunden werden, gib eine leere Liste zurück.",
        messages=[
            {
                "role": "user",
                "content": [
                    {
                        "type": "text",
                        "text": "Meine nächste Reise beinhaltet einen Flug von Seattle nach Amsterdam. Ich werde ein paar Tage in Amsterdam verbringen, bevor ich nach Paris weiterreise für einen Anschlussflug nach Rom."
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
      system: "Deine Aufgabe ist es, den bereitgestellten Text zu analysieren und alle darin erwähnten Flughafencodes zu identifizieren. Präsentiere diese Flughafencodes als Liste in der Reihenfolge, in der sie im Text erscheinen. Wenn keine Flughafencodes gefunden werden, gib eine leere Liste zurück.",
      messages: [
        {
          "role": "user",
          "content": [
            {
              "type": "text",
              "text": "Meine nächste Reise beinhaltet einen Flug von Seattle nach Amsterdam. Ich werde ein paar Tage in Amsterdam verbringen, bevor ich nach Paris weiterreise für einen Anschlussflug nach Rom."
            }
          ]
        }
      ]
    });
    console.log(msg);
    
    ```
</CodeGroup>