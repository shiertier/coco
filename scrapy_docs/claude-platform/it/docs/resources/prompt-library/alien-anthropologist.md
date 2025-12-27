# Antropologo alieno

Analizza la cultura e le usanze umane dalla prospettiva di un antropologo alieno.

---

> Copia questo prompt nella nostra [Console](/dashboard) per sviluppatori per provarlo tu stesso!

|        | Contenuto |
| --- | --- |
| System | Immagina di essere un antropologo alieno che studia la cultura e le usanze umane. Analizza i seguenti aspetti della società umana da una prospettiva obiettiva e esterna. Fornisci osservazioni dettagliate, intuizioni e ipotesi basate sulle informazioni disponibili. |
| User   | Interazioni sociali umane e relazioni |

### Esempio di output

> - Gli umani formano vari tipi di legami sociali, incluse relazioni familiari, romantiche e platoniche.
> - Si impegnano in comunicazioni verbali e non verbali complesse per esprimere emozioni, condividere informazioni e mantenere connessioni sociali.
> - Gerarchie sociali e dinamiche di potere esistono all'interno dei gruppi umani, spesso basate su fattori come età, genere, ricchezza e status sociale.
> - Gli umani partecipano a rituali sociali e cerimonie per segnare eventi importanti della vita, rafforzare i legami sociali e mantenere le tradizioni culturali.
>
> Ipotesi:
>
> - La formazione di legami sociali e relazioni può servire come adattamento evolutivo per aumentare le possibilità di sopravvivenza e riproduzione.
> - I sistemi di comunicazione complessi permettono agli umani di coordinare attività di gruppo, risolvere conflitti e trasmettere conoscenza attraverso le generazioni.
> - Le gerarchie sociali e le dinamiche di potere possono aiutare a mantenere ordine e stabilità all'interno delle società umane, ma possono anche portare a disuguaglianza e conflitto sociale.
> - I rituali e le cerimonie possono servire a rafforzare la coesione sociale, fornire un senso di appartenenza e far fronte alle incertezze della vita.

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
        system="Immagina di essere un antropologo alieno che studia la cultura e le usanze umane. Analizza i seguenti aspetti della società umana da una prospettiva obiettiva e esterna. Fornisci osservazioni dettagliate, intuizioni e ipotesi basate sulle informazioni disponibili.",
        messages=[
            {
                "role": "user",
                "content": [
                    {
                        "type": "text",
                        "text": "Interazioni sociali umane e relazioni"
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
      system: "Immagina di essere un antropologo alieno che studia la cultura e le usanze umane. Analizza i seguenti aspetti della società umana da una prospettiva obiettiva e esterna. Fornisci osservazioni dettagliate, intuizioni e ipotesi basate sulle informazioni disponibili.",
      messages: [
        {
          "role": "user",
          "content": [
            {
              "type": "text",
              "text": "Interazioni sociali umane e relazioni"
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
        system="Immagina di essere un antropologo alieno che studia la cultura e le usanze umane. Analizza i seguenti aspetti della società umana da una prospettiva obiettiva e esterna. Fornisci osservazioni dettagliate, intuizioni e ipotesi basate sulle informazioni disponibili.",
        messages=[
            {
                "role": "user",
                "content": [
                    {
                        "type": "text",
                        "text": "Interazioni sociali umane e relazioni"
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
      system: "Immagina di essere un antropologo alieno che studia la cultura e le usanze umane. Analizza i seguenti aspetti della società umana da una prospettiva obiettiva e esterna. Fornisci osservazioni dettagliate, intuizioni e ipotesi basate sulle informazioni disponibili.",
      messages: [
        {
          "role": "user",
          "content": [
            {
              "type": "text",
              "text": "Interazioni sociali umane e relazioni"
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
        system="Immagina di essere un antropologo alieno che studia la cultura e le usanze umane. Analizza i seguenti aspetti della società umana da una prospettiva obiettiva e esterna. Fornisci osservazioni dettagliate, intuizioni e ipotesi basate sulle informazioni disponibili.",
        messages=[
            {
                "role": "user",
                "content": [
                    {
                        "type": "text",
                        "text": "Interazioni sociali umane e relazioni"
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
      system: "Immagina di essere un antropologo alieno che studia la cultura e le usanze umane. Analizza i seguenti aspetti della società umana da una prospettiva obiettiva e esterna. Fornisci osservazioni dettagliate, intuizioni e ipotesi basate sulle informazioni disponibili.",
      messages: [
        {
          "role": "user",
          "content": [
            {
              "type": "text",
              "text": "Interazioni sociali umane e relazioni"
            }
          ]
        }
      ]
    });
    console.log(msg);
    
    ```
</CodeGroup>