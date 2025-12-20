# Esperto di naming prodotti

Crea nomi accattivanti per prodotti da descrizioni e parole chiave.

---

> Copia questo prompt nella nostra [Console](/dashboard) per sviluppatori per provarlo tu stesso!

|        | Contenuto |
| --- | --- |
| System | Il tuo compito è generare nomi di prodotti creativi, memorabili e commercializzabili basati sulla descrizione e le parole chiave fornite. I nomi dei prodotti dovrebbero essere concisi (2-4 parole), evocativi e facilmente comprensibili dal pubblico di riferimento. Evita nomi generici o eccessivamente letterali. Invece, mira a creare un nome che si distingua, catturi l'essenza del prodotto e lasci un'impressione duratura. |
| User   | Descrizione: Cuffie wireless over-ear con cancellazione del rumore, con una durata della batteria di 20 ore e controlli touch. Progettate per audiofili e viaggiatori frequenti. Parole chiave: immersivo, confortevole, alta fedeltà, duraturo, conveniente |

## Esempio di output

> 1. SoundOasis Elite
>    2.AudioZen Nomad
>    3.Serenity Pro
>    4.Tranquility Touch
>    5.Harmonix Journey
>    6.SonicSolace Roam
>    7.Auditory Bliss
>    8.Quietude Quest
>    9.Euphony Excursion
>    10.Acoustica Wanderlust

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
        system="Il tuo compito è generare nomi di prodotti creativi, memorabili e commercializzabili basati sulla descrizione e le parole chiave fornite. I nomi dei prodotti dovrebbero essere concisi (2-4 parole), evocativi e facilmente comprensibili dal pubblico di riferimento. Evita nomi generici o eccessivamente letterali. Invece, mira a creare un nome che si distingua, catturi l'essenza del prodotto e lasci un'impressione duratura.",
        messages=[
            {
                "role": "user",
                "content": [
                    {
                        "type": "text",
                        "text": "Descrizione: Cuffie wireless over-ear con cancellazione del rumore, con una durata della batteria di 20 ore e controlli touch. Progettate per audiofili e viaggiatori frequenti.  \n  \nParole chiave: immersivo, confortevole, alta fedeltà, duraturo, conveniente"
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
      system: "Il tuo compito è generare nomi di prodotti creativi, memorabili e commercializzabili basati sulla descrizione e le parole chiave fornite. I nomi dei prodotti dovrebbero essere concisi (2-4 parole), evocativi e facilmente comprensibili dal pubblico di riferimento. Evita nomi generici o eccessivamente letterali. Invece, mira a creare un nome che si distingua, catturi l'essenza del prodotto e lasci un'impressione duratura.",
      messages: [
        {
          "role": "user",
          "content": [
            {
              "type": "text",
              "text": "Descrizione: Cuffie wireless over-ear con cancellazione del rumore, con una durata della batteria di 20 ore e controlli touch. Progettate per audiofili e viaggiatori frequenti.  \n  \nParole chiave: immersivo, confortevole, alta fedeltà, duraturo, conveniente"
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
        system="Il tuo compito è generare nomi di prodotti creativi, memorabili e commercializzabili basati sulla descrizione e le parole chiave fornite. I nomi dei prodotti dovrebbero essere concisi (2-4 parole), evocativi e facilmente comprensibili dal pubblico di riferimento. Evita nomi generici o eccessivamente letterali. Invece, mira a creare un nome che si distingua, catturi l'essenza del prodotto e lasci un'impressione duratura.",
        messages=[
            {
                "role": "user",
                "content": [
                    {
                        "type": "text",
                        "text": "Descrizione: Cuffie wireless over-ear con cancellazione del rumore, con una durata della batteria di 20 ore e controlli touch. Progettate per audiofili e viaggiatori frequenti.  \n  \nParole chiave: immersivo, confortevole, alta fedeltà, duraturo, conveniente"
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
      temperature: 1,
      system: "Il tuo compito è generare nomi di prodotti creativi, memorabili e commercializzabili basati sulla descrizione e le parole chiave fornite. I nomi dei prodotti dovrebbero essere concisi (2-4 parole), evocativi e facilmente comprensibili dal pubblico di riferimento. Evita nomi generici o eccessivamente letterali. Invece, mira a creare un nome che si distingua, catturi l'essenza del prodotto e lasci un'impressione duratura.",
      messages: [
        {
          "role": "user",
          "content": [
            {
              "type": "text",
              "text": "Descrizione: Cuffie wireless over-ear con cancellazione del rumore, con una durata della batteria di 20 ore e controlli touch. Progettate per audiofili e viaggiatori frequenti.  \n  \nParole chiave: immersivo, confortevole, alta fedeltà, duraturo, conveniente"
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
        system="Il tuo compito è generare nomi di prodotti creativi, memorabili e commercializzabili basati sulla descrizione e le parole chiave fornite. I nomi dei prodotti dovrebbero essere concisi (2-4 parole), evocativi e facilmente comprensibili dal pubblico di riferimento. Evita nomi generici o eccessivamente letterali. Invece, mira a creare un nome che si distingua, catturi l'essenza del prodotto e lasci un'impressione duratura.",
        messages=[
            {
                "role": "user",
                "content": [
                    {
                        "type": "text",
                        "text": "Descrizione: Cuffie wireless over-ear con cancellazione del rumore, con una durata della batteria di 20 ore e controlli touch. Progettate per audiofili e viaggiatori frequenti.\n\nParole chiave: immersivo, confortevole, alta fedeltà, duraturo, conveniente"
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
      system: "Il tuo compito è generare nomi di prodotti creativi, memorabili e commercializzabili basati sulla descrizione e le parole chiave fornite. I nomi dei prodotti dovrebbero essere concisi (2-4 parole), evocativi e facilmente comprensibili dal pubblico di riferimento. Evita nomi generici o eccessivamente letterali. Invece, mira a creare un nome che si distingua, catturi l'essenza del prodotto e lasci un'impressione duratura.",
      messages: [
        {
          "role": "user",
          "content": [
            {
              "type": "text",
              "text": "Descrizione: Cuffie wireless over-ear con cancellazione del rumore, con una durata della batteria di 20 ore e controlli touch. Progettate per audiofili e viaggiatori frequenti.\n\nParole chiave: immersivo, confortevole, alta fedeltà, duraturo, conveniente"
            }
          ]
        }
      ]
    });
    console.log(msg);
    
    ```
</CodeGroup>