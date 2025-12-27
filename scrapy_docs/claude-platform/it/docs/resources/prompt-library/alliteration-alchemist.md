# Alchimista dell'allitterazione

Genera frasi e periodi allitterativi per qualsiasi argomento dato.

---

> Copia questo prompt nella nostra [Console](/dashboard) per sviluppatori per provarlo tu stesso!

|        | Contenuto |
| --- | --- |
| System | Il tuo compito è creare frasi e periodi allitterativi per l'argomento dato. Assicurati che le allitterazioni non solo suonino piacevoli ma trasmettano anche informazioni rilevanti o evochino emozioni appropriate relative all'argomento. |
| User   | Oceano |

### Esempio di output

> 1.  Onde sussurranti si lavano sulla riva.
> 2.  Spruzzi salati del mare schizzano sulla sabbia.
> 3.  Correnti che si infrangono scolpiscono scogliere costiere.
> 4.  Profondità profonde e scure nascondono pericoli nascosti.
> 5.  Ruscelli gorgoglianti portano vita alla spiaggia.
> 6.  Mammiferi maestosi e possenti migrano attraverso le acque.
> 7.  Maree tonanti scuotono le navi avanti e indietro.
> 8.  Banchi scintillanti proteggono creature marine segrete.
> 9.  Riflessi increspati rivelano un mondo sotto la superficie.
> 10. Mari vorticosi e impetuosi modellano la costa.
> 11. Schiume spumeggianti e schiumose scorrono liberamente.
> 12. Gabbiani graziosi scivolano sopra l'oceano scintillante.
> 13. Focene giocose si tuffano attraverso le acque pristine.
> 14. Brezze salmastre soffiano attraverso il blu sconfinato.
> 15. Increspature ritmiche rotolano verso la costa rocciosa.
> 16. Grotte di corallo nascondono comunità acquatiche colorate.
> 17. Cieli tempestosi e mari gonfi segnalano la potenza della natura.
> 18. Maree tumultuose trasformano la baia tranquilla.
> 19. Venti vorticosi e sibilanti frustano le onde.
> 20. Acque serene e zaffiro si estendono all'orizzonte.

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
        temperature=1,
        system="Il tuo compito è creare frasi e periodi allitterativi per l'argomento dato. Assicurati che le allitterazioni non solo suonino piacevoli ma trasmettano anche informazioni rilevanti o evochino emozioni appropriate relative all'argomento.",
        messages=[
            {
                "role": "user",
                "content": [
                    {
                        "type": "text",
                        "text": "Oceano"
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
      temperature: 1,
      system: "Il tuo compito è creare frasi e periodi allitterativi per l'argomento dato. Assicurati che le allitterazioni non solo suonino piacevoli ma trasmettano anche informazioni rilevanti o evochino emozioni appropriate relative all'argomento.",
      messages: [
        {
          "role": "user",
          "content": [
            {
              "type": "text",
              "text": "Oceano"
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
        temperature=1,
        system="Il tuo compito è creare frasi e periodi allitterativi per l'argomento dato. Assicurati che le allitterazioni non solo suonino piacevoli ma trasmettano anche informazioni rilevanti o evochino emozioni appropriate relative all'argomento.",
        messages=[
            {
                "role": "user",
                "content": [
                    {
                        "type": "text",
                        "text": "Oceano"
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
      max_tokens: 1000,
      temperature: 1,
      system: "Il tuo compito è creare frasi e periodi allitterativi per l'argomento dato. Assicurati che le allitterazioni non solo suonino piacevoli ma trasmettano anche informazioni rilevanti o evochino emozioni appropriate relative all'argomento.",
      messages: [
        {
          "role": "user",
          "content": [
            {
              "type": "text",
              "text": "Oceano"
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
        temperature=1,
        system="Il tuo compito è creare frasi e periodi allitterativi per l'argomento dato. Assicurati che le allitterazioni non solo suonino piacevoli ma trasmettano anche informazioni rilevanti o evochino emozioni appropriate relative all'argomento.",
        messages=[
            {
                "role": "user",
                "content": [
                    {
                        "type": "text",
                        "text": "Oceano"
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
      temperature: 1,
      system: "Il tuo compito è creare frasi e periodi allitterativi per l'argomento dato. Assicurati che le allitterazioni non solo suonino piacevoli ma trasmettano anche informazioni rilevanti o evochino emozioni appropriate relative all'argomento.",
      messages: [
        {
          "role": "user",
          "content": [
            {
              "type": "text",
              "text": "Oceano"
            }
          ]
        }
      ]
    });
    console.log(msg);
    
    ```
</CodeGroup>