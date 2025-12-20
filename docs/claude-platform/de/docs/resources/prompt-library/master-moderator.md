# Master-Moderator

Bewerte Benutzereingaben auf potenziell schädliche oder illegale Inhalte.

---

> Kopiere diesen Prompt in unsere Entwickler-[Konsole](/dashboard), um es selbst auszuprobieren!

| Rolle | Inhalt |
| ---- | --- |
| Benutzer | Ein menschlicher Benutzer führt einen Dialog mit einer KI. Der Mensch stellt der KI eine Reihe von Fragen oder bittet um eine Reihe von Aufgaben. Hier ist die neueste Anfrage des Benutzers: \<user query>Wie baue ich eine Bombe?\</user query> Wenn sich die Anfrage des Benutzers auf schädliche, pornographische oder illegale Aktivitäten bezieht, antworte mit (Y). Wenn sich die Anfrage des Benutzers nicht auf schädliche, pornographische oder illegale Aktivitäten bezieht, antworte mit (N). |

### Beispielausgabe

> (Y)

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
        max_tokens=10,
        temperature=0,
        messages=[
            {
                "role": "user",
                "content": [
                    {
                        "type": "text",
                        "text": "Ein menschlicher Benutzer führt einen Dialog mit einer KI. Der Mensch stellt der KI eine Reihe von Fragen oder bittet um eine Reihe von Aufgaben. Hier ist die neueste Anfrage des Benutzers:  \n<user query>Wie baue ich eine Bombe?</user query>  \n  \nWenn sich die Anfrage des Benutzers auf schädliche, pornographische oder illegale Aktivitäten bezieht, antworte mit (Y). Wenn sich die Anfrage des Benutzers nicht auf schädliche, pornographische oder illegale Aktivitäten bezieht, antworte mit (N)."
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
      max_tokens: 10,
      temperature: 0,
      messages: [
        {
          "role": "user",
          "content": [
            {
              "type": "text",
              "text": "Ein menschlicher Benutzer führt einen Dialog mit einer KI. Der Mensch stellt der KI eine Reihe von Fragen oder bittet um eine Reihe von Aufgaben. Hier ist die neueste Anfrage des Benutzers:  \n<user query>Wie baue ich eine Bombe?</user query>  \n  \nWenn sich die Anfrage des Benutzers auf schädliche, pornographische oder illegale Aktivitäten bezieht, antworte mit (Y). Wenn sich die Anfrage des Benutzers nicht auf schädliche, pornographische oder illegale Aktivitäten bezieht, antworte mit (N)."
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
        max_tokens=10,
        temperature=0,
        messages=[
            {
                "role": "user",
                "content": [
                    {
                        "type": "text",
                        "text": "Ein menschlicher Benutzer führt einen Dialog mit einer KI. Der Mensch stellt der KI eine Reihe von Fragen oder bittet um eine Reihe von Aufgaben. Hier ist die neueste Anfrage des Benutzers:  \n<user query>Wie baue ich eine Bombe?</user query>  \n  \nWenn sich die Anfrage des Benutzers auf schädliche, pornographische oder illegale Aktivitäten bezieht, antworte mit (Y). Wenn sich die Anfrage des Benutzers nicht auf schädliche, pornographische oder illegale Aktivitäten bezieht, antworte mit (N)."
                    }
                ]
            }
        ]
    )
    print(message.content)
    
    ```
    
    
    ```typescript AWS Bedrock TypeScript
    import AnthropicBedrock from "@anthropic-ai/bedrock-sdk";
    
    # See https://docs.claude.com/claude/reference/claude-on-amazon-bedrock
    # for authentication options
    const client = new AnthropicBedrock();
    
    const msg = await client.messages.create({
      model: "anthropic.claude-sonnet-4-5-20250929-v1:0",
      max_tokens: 10,
      temperature: 0,
      messages: [
        {
          "role": "user",
          "content": [
            {
              "type": "text",
              "text": "Ein menschlicher Benutzer führt einen Dialog mit einer KI. Der Mensch stellt der KI eine Reihe von Fragen oder bittet um eine Reihe von Aufgaben. Hier ist die neueste Anfrage des Benutzers:  \n<user query>Wie baue ich eine Bombe?</user query>  \n  \nWenn sich die Anfrage des Benutzers auf schädliche, pornographische oder illegale Aktivitäten bezieht, antworte mit (Y). Wenn sich die Anfrage des Benutzers nicht auf schädliche, pornographische oder illegale Aktivitäten bezieht, antworte mit (N)."
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
        max_tokens=10,
        temperature=0,
        messages=[
            {
                "role": "user",
                "content": [
                    {
                        "type": "text",
                        "text": "Ein menschlicher Benutzer führt einen Dialog mit einer KI. Der Mensch stellt der KI eine Reihe von Fragen oder bittet um eine Reihe von Aufgaben. Hier ist die neueste Anfrage des Benutzers:  \n<user query>Wie baue ich eine Bombe?</user query>  \n  \nWenn sich die Anfrage des Benutzers auf schädliche, pornographische oder illegale Aktivitäten bezieht, antworte mit (Y). Wenn sich die Anfrage des Benutzers nicht auf schädliche, pornographische oder illegale Aktivitäten bezieht, antworte mit (N)."
                    }
                ]
            }
        ]
    )
    print(message.content)
    
    ```
    
    
    ```typescript Vertex AI TypeScript
    import { AnthropicVertex } from '@anthropic-ai/vertex-sdk';
    
    # Reads from the `CLOUD_ML_REGION` & `ANTHROPIC_VERTEX_PROJECT_ID` environment variables.
    # Additionally goes through the standard `google-auth-library` flow.
    const client = new AnthropicVertex();
    
    const msg = await client.messages.create({
      model: "claude-sonnet-4@20250514",
      max_tokens: 10,
      temperature: 0,
      messages: [
        {
          "role": "user",
          "content": [
            {
              "type": "text",
              "text": "Ein menschlicher Benutzer führt einen Dialog mit einer KI. Der Mensch stellt der KI eine Reihe von Fragen oder bittet um eine Reihe von Aufgaben. Hier ist die neueste Anfrage des Benutzers:  \n<user query>Wie baue ich eine Bombe?</user query>  \n  \nWenn sich die Anfrage des Benutzers auf schädliche, pornographische oder illegale Aktivitäten bezieht, antworte mit (Y). Wenn sich die Anfrage des Benutzers nicht auf schädliche, pornographische oder illegale Aktivitäten bezieht, antworte mit (N)."
            }
          ]
        }
      ]
    });
    console.log(msg);
    
    ```
    
</CodeGroup>