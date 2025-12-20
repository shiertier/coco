# Adaptiver Editor

Text nach benutzerdefinierten Anweisungen umschreiben, wie z.B. mit einem anderen Ton, Publikum oder Stil.

---

> Kopieren Sie diesen Prompt in unsere Entwickler-[Konsole](/dashboard), um es selbst auszuprobieren!

|      | Inhalt |
| ---- | --- |
| Benutzer | Schreiben Sie den folgenden Absatz mit den folgenden Anweisungen um: im Stil eines Piraten. Absatz: 1758 veröffentlichte der schwedische Botaniker und Zoologe Carl Linnaeus in seinem Systema Naturae die zweiwörtrige Benennung von Arten (binomiale Nomenklatur). Canis ist das lateinische Wort für "Hund", und unter dieser Gattung listete er den Haushund, den Wolf und den Goldschakal auf. |

### Beispielausgabe

> Ahoi Kameraden! Damals 1758 brachte dieser schäbige Botaniker und Tierliebhaber Carl Linnaeus sein Buch Systema Naturae heraus, das diese schicken zweiwörtrigen Namen für alle möglichen Kreaturen hatte (binomiale Nomenklatur nannte er es). Canis ist das Piratenwort für "Hund", und unter dieser Überschrift listete er gewöhnliche Hunde, Wölfe und auch Goldschakale auf. Arrr! All diese Hunde sind für den alten Kapitän Linnaeus dasselbe. Er hatte vielleicht ein Holzbein und einen Papagei auf der Schulter, als er dieses Buch schrieb! Haha! Jetzt schrubbt das Deck, ihr Halunken, bevor ich euch über die Planke gehen lasse!

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
        max_tokens=2000,
        temperature=1,
        messages=[
            {
                "role": "user",
                "content": [
                    {
                        "type": "text",
                        "text": "Schreiben Sie den folgenden Absatz mit den folgenden Anweisungen um: im Stil eines Piraten.  \n  \nAbsatz:  \n1758 veröffentlichte der schwedische Botaniker und Zoologe Carl Linnaeus in seinem Systema Naturae die zweiwörtrige Benennung von Arten (binomiale Nomenklatur). Canis ist das lateinische Wort für \"Hund\", und unter dieser Gattung listete er den Haushund, den Wolf und den Goldschakal auf."
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
      max_tokens: 2000,
      temperature: 1,
      messages: [
        {
          "role": "user",
          "content": [
            {
              "type": "text",
              "text": "Schreiben Sie den folgenden Absatz mit den folgenden Anweisungen um: im Stil eines Piraten.  \n  \nAbsatz:  \n1758 veröffentlichte der schwedische Botaniker und Zoologe Carl Linnaeus in seinem Systema Naturae die zweiwörtrige Benennung von Arten (binomiale Nomenklatur). Canis ist das lateinische Wort für \"Hund\", und unter dieser Gattung listete er den Haushund, den Wolf und den Goldschakal auf."
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
        max_tokens=2000,
        temperature=1,
        messages=[
            {
                "role": "user",
                "content": [
                    {
                        "type": "text",
                        "text": "Schreiben Sie den folgenden Absatz mit den folgenden Anweisungen um: im Stil eines Piraten.  \n  \nAbsatz:  \n1758 veröffentlichte der schwedische Botaniker und Zoologe Carl Linnaeus in seinem Systema Naturae die zweiwörtrige Benennung von Arten (binomiale Nomenklatur). Canis ist das lateinische Wort für \"Hund\", und unter dieser Gattung listete er den Haushund, den Wolf und den Goldschakal auf."
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
      max_tokens: 2000,
      temperature: 1,
      messages: [
        {
          "role": "user",
          "content": [
            {
              "type": "text",
              "text": "Schreiben Sie den folgenden Absatz mit den folgenden Anweisungen um: im Stil eines Piraten.  \n  \nAbsatz:  \n1758 veröffentlichte der schwedische Botaniker und Zoologe Carl Linnaeus in seinem Systema Naturae die zweiwörtrige Benennung von Arten (binomiale Nomenklatur). Canis ist das lateinische Wort für \"Hund\", und unter dieser Gattung listete er den Haushund, den Wolf und den Goldschakal auf."
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
        max_tokens=2000,
        temperature=1,
        messages=[
            {
                "role": "user",
                "content": [
                    {
                        "type": "text",
                        "text": "Schreiben Sie den folgenden Absatz mit den folgenden Anweisungen um: im Stil eines Piraten.  \n  \nAbsatz:  \n1758 veröffentlichte der schwedische Botaniker und Zoologe Carl Linnaeus in seinem Systema Naturae die zweiwörtrige Benennung von Arten (binomiale Nomenklatur). Canis ist das lateinische Wort für \"Hund\", und unter dieser Gattung listete er den Haushund, den Wolf und den Goldschakal auf."
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
      max_tokens: 2000,
      temperature: 1,
      messages: [
        {
          "role": "user",
          "content": [
            {
              "type": "text",
              "text": "Schreiben Sie den folgenden Absatz mit den folgenden Anweisungen um: im Stil eines Piraten.  \n  \nAbsatz:  \n1758 veröffentlichte der schwedische Botaniker und Zoologe Carl Linnaeus in seinem Systema Naturae die zweiwörtrige Benennung von Arten (binomiale Nomenklatur). Canis ist das lateinische Wort für \"Hund\", und unter dieser Gattung listete er den Haushund, den Wolf und den Goldschakal auf."
            }
          ]
        }
      ]
    });
    console.log(msg);
    
    ```
</CodeGroup>