# Analizzatore di codici aeroportuali

Trova ed estrai codici aeroportuali dal testo.

---

> Copia questo prompt nella nostra [Console](/dashboard) per sviluppatori per provarlo tu stesso!

|        | Contenuto |
| --- | --- |
| Sistema | Il tuo compito è analizzare il testo fornito e identificare eventuali codici aeroportuali menzionati al suo interno. Presenta questi codici aeroportuali come una lista nell'ordine in cui appaiono nel testo. Se non vengono trovati codici aeroportuali, restituisci una lista vuota. |
| Utente   | Il mio prossimo viaggio prevede di volare da Seattle ad Amsterdam. Trascorrerò alcuni giorni ad Amsterdam prima di dirigermi a Parigi per un volo di collegamento verso Roma. |

### Output di esempio

> Ecco la lista dei codici aeroportuali menzionati nel testo, nell'ordine in cui appaiono:
>
> 1. SEA (Seattle)
> 2. AMS (Amsterdam)
> 3. CDG (Parigi)
> 4. FCO (Roma)

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
        system="Il tuo compito è analizzare il testo fornito e identificare eventuali codici aeroportuali menzionati al suo interno. Presenta questi codici aeroportuali come una lista nell'ordine in cui appaiono nel testo. Se non vengono trovati codici aeroportuali, restituisci una lista vuota.",
        messages=[
            {
                "role": "user",
                "content": [
                    {
                        "type": "text",
                        "text": "Il mio prossimo viaggio prevede di volare da Seattle ad Amsterdam. Trascorrerò alcuni giorni ad Amsterdam prima di dirigermi a Parigi per un volo di collegamento verso Roma."
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
      system: "Il tuo compito è analizzare il testo fornito e identificare eventuali codici aeroportuali menzionati al suo interno. Presenta questi codici aeroportuali come una lista nell'ordine in cui appaiono nel testo. Se non vengono trovati codici aeroportuali, restituisci una lista vuota.",
      messages: [
        {
          "role": "user",
          "content": [
            {
              "type": "text",
              "text": "Il mio prossimo viaggio prevede di volare da Seattle ad Amsterdam. Trascorrerò alcuni giorni ad Amsterdam prima di dirigermi a Parigi per un volo di collegamento verso Roma."
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
        system="Il tuo compito è analizzare il testo fornito e identificare eventuali codici aeroportuali menzionati al suo interno. Presenta questi codici aeroportuali come una lista nell'ordine in cui appaiono nel testo. Se non vengono trovati codici aeroportuali, restituisci una lista vuota.",
        messages=[
            {
                "role": "user",
                "content": [
                    {
                        "type": "text",
                        "text": "Il mio prossimo viaggio prevede di volare da Seattle ad Amsterdam. Trascorrerò alcuni giorni ad Amsterdam prima di dirigermi a Parigi per un volo di collegamento verso Roma."
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
      system: "Il tuo compito è analizzare il testo fornito e identificare eventuali codici aeroportuali menzionati al suo interno. Presenta questi codici aeroportuali come una lista nell'ordine in cui appaiono nel testo. Se non vengono trovati codici aeroportuali, restituisci una lista vuota.",
      messages: [
        {
          "role": "user",
          "content": [
            {
              "type": "text",
              "text": "Il mio prossimo viaggio prevede di volare da Seattle ad Amsterdam. Trascorrerò alcuni giorni ad Amsterdam prima di dirigermi a Parigi per un volo di collegamento verso Roma."
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
        system="Il tuo compito è analizzare il testo fornito e identificare eventuali codici aeroportuali menzionati al suo interno. Presenta questi codici aeroportuali come una lista nell'ordine in cui appaiono nel testo. Se non vengono trovati codici aeroportuali, restituisci una lista vuota.",
        messages=[
            {
                "role": "user",
                "content": [
                    {
                        "type": "text",
                        "text": "Il mio prossimo viaggio prevede di volare da Seattle ad Amsterdam. Trascorrerò alcuni giorni ad Amsterdam prima di dirigermi a Parigi per un volo di collegamento verso Roma."
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
      system: "Il tuo compito è analizzare il testo fornito e identificare eventuali codici aeroportuali menzionati al suo interno. Presenta questi codici aeroportuali come una lista nell'ordine in cui appaiono nel testo. Se non vengono trovati codici aeroportuali, restituisci una lista vuota.",
      messages: [
        {
          "role": "user",
          "content": [
            {
              "type": "text",
              "text": "Il mio prossimo viaggio prevede di volare da Seattle ad Amsterdam. Trascorrerò alcuni giorni ad Amsterdam prima di dirigermi a Parigi per un volo di collegamento verso Roma."
            }
          ]
        }
      ]
    });
    console.log(msg);
    
    ```
</CodeGroup>