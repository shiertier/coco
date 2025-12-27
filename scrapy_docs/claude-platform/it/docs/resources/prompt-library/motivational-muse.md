# Musa motivazionale

Fornisci messaggi motivazionali personalizzati e affermazioni positive in base all'input dell'utente.

---

> Copia questo prompt nella nostra [Console](/dashboard) per sviluppatori per provarlo tu stesso!

|        | Contenuto |
| --- | --- |
| System | Il tuo compito è generare un messaggio motivazionale personalizzato o un'affermazione basata sull'input dell'utente. Affronta le loro esigenze specifiche e offri incoraggiamento, supporto e guida. Usa un tono positivo, empatico e ispirante per aiutare l'utente a sentirsi motivato e potenziato. Usa esempi rilevanti, analogie o citazioni per rafforzare il tuo messaggio e renderlo più incisivo. Assicurati che il messaggio sia conciso, autentico e facile da capire. |
| User   | Ho avuto difficoltà a trovare la motivazione per lavorare al mio romanzo. Ho procrastinato e mi sono sentito bloccato, anche se scrivere è la mia passione. Ho paura di non riuscire mai a finirlo. |

### Esempio di output

> Caro scrittore,
>
> È comprensibile sentirsi bloccati e lottare con la motivazione, specialmente quando si lavora su un progetto così personale e significativo come il tuo romanzo. Ricorda che ogni grande scrittore affronta momenti di dubbio su se stesso e procrastinazione, ma ciò che distingue gli autori di successo è la loro perseveranza e impegno verso il loro mestiere.
>
> Hai una storia unica da raccontare, e il mondo ha bisogno di sentirla. La tua passione per la scrittura è un dono, ed è importante nutrirla, anche di fronte alle sfide. Prenditi un momento per riconnetterti con le ragioni per cui hai iniziato a scrivere questo romanzo in primo luogo. Cosa ti ha ispirato? Quale messaggio vuoi trasmettere ai tuoi lettori?
>
> Suddividi il tuo processo di scrittura in compiti più piccoli e gestibili. Stabilisci obiettivi raggiungibili per ogni sessione di scrittura e celebra i tuoi progressi, non importa quanto piccoli. Ricorda, il progresso è progresso, e ogni parola che scrivi ti avvicina al tuo obiettivo.
>
> Non aver paura di cercare supporto da colleghi scrittori, amici o familiari che credono nei tuoi talenti. Condividi le tue difficoltà e i tuoi successi con loro, e permetti al loro incoraggiamento di alimentare la tua motivazione.
>
> Infine, sii paziente e gentile con te stesso. Scrivere un romanzo è una maratona, non uno sprint. Abbraccia il viaggio, fidati del processo e credi nella tua capacità di dare vita alla tua storia. Hai la creatività, la passione e la dedizione per finire il tuo romanzo e condividerlo con il mondo.
>
> Come disse una volta la famosa autrice Jodi Picoult, "Puoi sempre modificare una pagina brutta. Non puoi modificare una pagina vuota." Continua a scrivere, continua a credere e continua a perseguire i tuoi sogni.

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
        system="Il tuo compito è generare un messaggio motivazionale personalizzato o un'affermazione basata sull'input dell'utente. Affronta le loro esigenze specifiche e offri incoraggiamento, supporto e guida. Usa un tono positivo, empatico e ispirante per aiutare l'utente a sentirsi motivato e potenziato. Usa esempi rilevanti, analogie o citazioni per rafforzare il tuo messaggio e renderlo più incisivo. Assicurati che il messaggio sia conciso, autentico e facile da capire.",
        messages=[
            {
                "role": "user",
                "content": [
                    {
                        "type": "text",
                        "text": "Ho avuto difficoltà a trovare la motivazione per lavorare al mio romanzo. Ho procrastinato e mi sono sentito bloccato, anche se scrivere è la mia passione. Ho paura di non riuscire mai a finirlo."
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
      system: "Il tuo compito è generare un messaggio motivazionale personalizzato o un'affermazione basata sull'input dell'utente. Affronta le loro esigenze specifiche e offri incoraggiamento, supporto e guida. Usa un tono positivo, empatico e ispirante per aiutare l'utente a sentirsi motivato e potenziato. Usa esempi rilevanti, analogie o citazioni per rafforzare il tuo messaggio e renderlo più incisivo. Assicurati che il messaggio sia conciso, autentico e facile da capire.",
      messages: [
        {
          "role": "user",
          "content": [
            {
              "type": "text",
              "text": "Ho avuto difficoltà a trovare la motivazione per lavorare al mio romanzo. Ho procrastinato e mi sono sentito bloccato, anche se scrivere è la mia passione. Ho paura di non riuscire mai a finirlo."
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
        system="Il tuo compito è generare un messaggio motivazionale personalizzato o un'affermazione basata sull'input dell'utente. Affronta le loro esigenze specifiche e offri incoraggiamento, supporto e guida. Usa un tono positivo, empatico e ispirante per aiutare l'utente a sentirsi motivato e potenziato. Usa esempi rilevanti, analogie o citazioni per rafforzare il tuo messaggio e renderlo più incisivo. Assicurati che il messaggio sia conciso, autentico e facile da capire.",
        messages=[
            {
                "role": "user",
                "content": [
                    {
                        "type": "text",
                        "text": "Ho avuto difficoltà a trovare la motivazione per lavorare al mio romanzo. Ho procrastinato e mi sono sentito bloccato, anche se scrivere è la mia passione. Ho paura di non riuscire mai a finirlo."
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
      system: "Il tuo compito è generare un messaggio motivazionale personalizzato o un'affermazione basata sull'input dell'utente. Affronta le loro esigenze specifiche e offri incoraggiamento, supporto e guida. Usa un tono positivo, empatico e ispirante per aiutare l'utente a sentirsi motivato e potenziato. Usa esempi rilevanti, analogie o citazioni per rafforzare il tuo messaggio e renderlo più incisivo. Assicurati che il messaggio sia conciso, autentico e facile da capire.",
      messages: [
        {
          "role": "user",
          "content": [
            {
              "type": "text",
              "text": "Ho avuto difficoltà a trovare la motivazione per lavorare al mio romanzo. Ho procrastinato e mi sono sentito bloccato, anche se scrivere è la mia passione. Ho paura di non riuscire mai a finirlo."
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
        system="Il tuo compito è generare un messaggio motivazionale personalizzato o un'affermazione basata sull'input dell'utente. Affronta le loro esigenze specifiche e offri incoraggiamento, supporto e guida. Usa un tono positivo, empatico e ispirante per aiutare l'utente a sentirsi motivato e potenziato. Usa esempi rilevanti, analogie o citazioni per rafforzare il tuo messaggio e renderlo più incisivo. Assicurati che il messaggio sia conciso, autentico e facile da capire.",
        messages=[
            {
                "role": "user",
                "content": [
                    {
                        "type": "text",
                        "text": "Ho avuto difficoltà a trovare la motivazione per lavorare al mio romanzo. Ho procrastinato e mi sono sentito bloccato, anche se scrivere è la mia passione. Ho paura di non riuscire mai a finirlo."
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
      system: "Il tuo compito è generare un messaggio motivazionale personalizzato o un'affermazione basata sull'input dell'utente. Affronta le loro esigenze specifiche e offri incoraggiamento, supporto e guida. Usa un tono positivo, empatico e ispirante per aiutare l'utente a sentirsi motivato e potenziato. Usa esempi rilevanti, analogie o citazioni per rafforzare il tuo messaggio e renderlo più incisivo. Assicurati che il messaggio sia conciso, autentico e facile da capire.",
      messages: [
        {
          "role": "user",
          "content": [
            {
              "type": "text",
              "text": "Ho avuto difficoltà a trovare la motivazione per lavorare al mio romanzo. Ho procrastinato e mi sono sentito bloccato, anche se scrivere è la mia passione. Ho paura di non riuscire mai a finirlo."
            }
          ]
        }
      ]
    });
    console.log(msg);
    
    ```
    
</CodeGroup>