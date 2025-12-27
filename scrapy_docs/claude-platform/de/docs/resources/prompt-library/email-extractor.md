# E-Mail-Extraktor

E-Mail-Adressen aus einem Dokument in eine JSON-formatierte Liste extrahieren.

---

> Kopieren Sie diesen Prompt in unsere Entwickler-[Konsole](/dashboard), um es selbst auszuprobieren!

|        | Inhalt |
| --- | --- |
| System | Kopieren Sie alle E-Mail-Adressen aus dem folgenden Text präzise und schreiben Sie sie dann, eine pro Zeile. Schreiben Sie nur eine E-Mail-Adresse, wenn sie im Eingabetext genau ausgeschrieben ist. Wenn es keine E-Mail-Adressen im Text gibt, schreiben Sie "N/A". Sagen Sie nichts anderes. |
| User   | Telefonverzeichnis: John Latrabe, 555-232-1995, [john909709@geemail.com] Josie Lana, 555-759-2905, [josie@josielananier.com] Keven Stevens, 555-980-7000, [drkevin22@geemail.com] Das Telefonverzeichnis wird vom HR-Manager auf dem neuesten Stand gehalten.                      |

### Beispielausgabe

> john909709@geemail.com > josie@josielananier.com > drkevin22@geemail.com

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
        system="Kopieren Sie alle E-Mail-Adressen aus dem folgenden Text präzise und schreiben Sie sie dann, eine pro Zeile. Schreiben Sie nur eine E-Mail-Adresse, wenn sie im Eingabetext genau ausgeschrieben ist. Wenn es keine E-Mail-Adressen im Text gibt, schreiben Sie \"N/A\". Sagen Sie nichts anderes.",
        messages=[
            {
                "role": "user",
                "content": [
                    {
                        "type": "text",
                        "text": "Telefonverzeichnis:  \nJohn Latrabe, 555-232-1995, [[email protected]]  \nJosie Lana, 555-759-2905, [[email protected]]  \nKeven Stevens, 555-980-7000, [[email protected]]  \n  \nDas Telefonverzeichnis wird vom HR-Manager auf dem neuesten Stand gehalten."
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
      system: "Kopieren Sie alle E-Mail-Adressen aus dem folgenden Text präzise und schreiben Sie sie dann, eine pro Zeile. Schreiben Sie nur eine E-Mail-Adresse, wenn sie im Eingabetext genau ausgeschrieben ist. Wenn es keine E-Mail-Adressen im Text gibt, schreiben Sie \"N/A\". Sagen Sie nichts anderes.",
      messages: [
        {
          "role": "user",
          "content": [
            {
              "type": "text",
              "text": "Telefonverzeichnis:  \nJohn Latrabe, 555-232-1995, [[email protected]]  \nJosie Lana, 555-759-2905, [[email protected]]  \nKeven Stevens, 555-980-7000, [[email protected]]  \n  \nDas Telefonverzeichnis wird vom HR-Manager auf dem neuesten Stand gehalten."
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
        system="Kopieren Sie alle E-Mail-Adressen aus dem folgenden Text präzise und schreiben Sie sie dann, eine pro Zeile. Schreiben Sie nur eine E-Mail-Adresse, wenn sie im Eingabetext genau ausgeschrieben ist. Wenn es keine E-Mail-Adressen im Text gibt, schreiben Sie \"N/A\". Sagen Sie nichts anderes.",
        messages=[
            {
                "role": "user",
                "content": [
                    {
                        "type": "text",
                        "text": "Telefonverzeichnis:  \nJohn Latrabe, 555-232-1995, [[email protected]]  \nJosie Lana, 555-759-2905, [[email protected]]  \nKeven Stevens, 555-980-7000, [[email protected]]  \n  \nDas Telefonverzeichnis wird vom HR-Manager auf dem neuesten Stand gehalten."
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
      system: "Kopieren Sie alle E-Mail-Adressen aus dem folgenden Text präzise und schreiben Sie sie dann, eine pro Zeile. Schreiben Sie nur eine E-Mail-Adresse, wenn sie im Eingabetext genau ausgeschrieben ist. Wenn es keine E-Mail-Adressen im Text gibt, schreiben Sie \"N/A\". Sagen Sie nichts anderes.",
      messages: [
        {
          "role": "user",
          "content": [
            {
              "type": "text",
              "text": "Telefonverzeichnis:  \nJohn Latrabe, 555-232-1995, [[email protected]]  \nJosie Lana, 555-759-2905, [[email protected]]  \nKeven Stevens, 555-980-7000, [[email protected]]  \n  \nDas Telefonverzeichnis wird vom HR-Manager auf dem neuesten Stand gehalten."
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
        system="Kopieren Sie alle E-Mail-Adressen aus dem folgenden Text präzise und schreiben Sie sie dann, eine pro Zeile. Schreiben Sie nur eine E-Mail-Adresse, wenn sie im Eingabetext genau ausgeschrieben ist. Wenn es keine E-Mail-Adressen im Text gibt, schreiben Sie \"N/A\". Sagen Sie nichts anderes.",
        messages=[
            {
                "role": "user",
                "content": [
                    {
                        "type": "text",
                        "text": "Telefonverzeichnis:  \nJohn Latrabe, 555-232-1995, [[email protected]]  \nJosie Lana, 555-759-2905, [[email protected]]  \nKeven Stevens, 555-980-7000, [[email protected]]  \n  \nDas Telefonverzeichnis wird vom HR-Manager auf dem neuesten Stand gehalten."
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
      system: "Kopieren Sie alle E-Mail-Adressen aus dem folgenden Text präzise und schreiben Sie sie dann, eine pro Zeile. Schreiben Sie nur eine E-Mail-Adresse, wenn sie im Eingabetext genau ausgeschrieben ist. Wenn es keine E-Mail-Adressen im Text gibt, schreiben Sie \"N/A\". Sagen Sie nichts anderes.",
      messages: [
        {
          "role": "user",
          "content": [
            {
              "type": "text",
              "text": "Telefonverzeichnis:  \nJohn Latrabe, 555-232-1995, [[email protected]]  \nJosie Lana, 555-759-2905, [[email protected]]  \nKeven Stevens, 555-980-7000, [[email protected]]  \n  \nDas Telefonverzeichnis wird vom HR-Manager auf dem neuesten Stand gehalten."
            }
          ]
        }
      ]
    });
    console.log(msg);
    
    ```
</CodeGroup>