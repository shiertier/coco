# Editor adattivo

Riscrivi il testo seguendo le istruzioni fornite dall'utente, come con un tono, pubblico o stile diverso.

---

> Copia questo prompt nella nostra [Console](/dashboard) per sviluppatori per provarlo tu stesso!

|      | Contenuto |
| ---- | --- |
| Utente | Riscrivi il seguente paragrafo utilizzando le seguenti istruzioni: nello stile di un pirata. Paragrafo: Nel 1758, il botanico e zoologo svedese Carl Linnaeus pubblicò nel suo Systema Naturae, la denominazione a due parole delle specie (nomenclatura binomiale). Canis è la parola latina che significa "cane", e sotto questo genere, elencò il cane domestico, il lupo e lo sciacallo dorato. |

### Esempio di output

> Ahoy compagni! Nel 1758, quel botanico maledetto e amante degli animali Carl Linnaeus pubblicò il suo libro Systema Naturae, che aveva quei nomi fantasiosi a due parole per ogni sorta di creature (nomenclatura binomiale la chiamava). Canis è la parola pirata per "cane", e sotto quella voce elencò cani normali, lupi e anche sciacalli dorati. Arrr! Tutti quei cani sono uguali per il vecchio Capitano Linnaeus. Potrebbe aver avuto una gamba di legno e un pappagallo sulla spalla quando scrisse quel libro! Haha! Ora lavate il ponte, furfanti, prima che vi faccia camminare sulla tavola!

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
        max_tokens=2000,
        temperature=1,
        messages=[
            {
                "role": "user",
                "content": [
                    {
                        "type": "text",
                        "text": "Riscrivi il seguente paragrafo utilizzando le seguenti istruzioni: nello stile di un pirata.  \n  \nParagrafo:  \nNel 1758, il botanico e zoologo svedese Carl Linnaeus pubblicò nel suo Systema Naturae, la denominazione a due parole delle specie (nomenclatura binomiale). Canis è la parola latina che significa \"cane\", e sotto questo genere, elencò il cane domestico, il lupo e lo sciacallo dorato."
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
              "text": "Riscrivi il seguente paragrafo utilizzando le seguenti istruzioni: nello stile di un pirata.  \n  \nParagrafo:  \nNel 1758, il botanico e zoologo svedese Carl Linnaeus pubblicò nel suo Systema Naturae, la denominazione a due parole delle specie (nomenclatura binomiale). Canis è la parola latina che significa \"cane\", e sotto questo genere, elencò il cane domestico, il lupo e lo sciacallo dorato."
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
                        "text": "Riscrivi il seguente paragrafo utilizzando le seguenti istruzioni: nello stile di un pirata.  \n  \nParagrafo:  \nNel 1758, il botanico e zoologo svedese Carl Linnaeus pubblicò nel suo Systema Naturae, la denominazione a due parole delle specie (nomenclatura binomiale). Canis è la parola latina che significa \"cane\", e sotto questo genere, elencò il cane domestico, il lupo e lo sciacallo dorato."
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
      max_tokens: 2000,
      temperature: 1,
      messages: [
        {
          "role": "user",
          "content": [
            {
              "type": "text",
              "text": "Riscrivi il seguente paragrafo utilizzando le seguenti istruzioni: nello stile di un pirata.  \n  \nParagrafo:  \nNel 1758, il botanico e zoologo svedese Carl Linnaeus pubblicò nel suo Systema Naturae, la denominazione a due parole delle specie (nomenclatura binomiale). Canis è la parola latina che significa \"cane\", e sotto questo genere, elencò il cane domestico, il lupo e lo sciacallo dorato."
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
                        "text": "Riscrivi il seguente paragrafo utilizzando le seguenti istruzioni: nello stile di un pirata.  \n  \nParagrafo:  \nNel 1758, il botanico e zoologo svedese Carl Linnaeus pubblicò nel suo Systema Naturae, la denominazione a due parole delle specie (nomenclatura binomiale). Canis è la parola latina che significa \"cane\", e sotto questo genere, elencò il cane domestico, il lupo e lo sciacallo dorato."
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
      max_tokens: 2000,
      temperature: 1,
      messages: [
        {
          "role": "user",
          "content": [
            {
              "type": "text",
              "text": "Riscrivi il seguente paragrafo utilizzando le seguenti istruzioni: nello stile di un pirata.  \n  \nParagrafo:  \nNel 1758, il botanico e zoologo svedese Carl Linnaeus pubblicò nel suo Systema Naturae, la denominazione a due parole delle specie (nomenclatura binomiale). Canis è la parola latina che significa \"cane\", e sotto questo genere, elencò il cane domestico, il lupo e lo sciacallo dorato."
            }
          ]
        }
      ]
    });
    console.log(msg);
    
    ```
</CodeGroup>