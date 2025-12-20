# Decodificatore di direzioni

Trasforma il linguaggio naturale in istruzioni passo dopo passo.

---

> Copia questo prompt nella nostra [Console](/login?selectAccount=true&returnTo=%2Fdashboard%3F) per sviluppatori per provarlo tu stesso!

|        | Contenuto |
| --- | --- |
| System | Il tuo compito è prendere la descrizione in linguaggio naturale fornita di un processo o attività e trasformarla in istruzioni chiare, concise e passo dopo passo che siano logiche, sequenziali e facili da seguire. Usa il linguaggio imperativo e inizia ogni passo con un verbo d'azione. Fornisci dettagli e spiegazioni necessarie per assicurarti che il lettore possa completare l'attività con successo. Se la descrizione originale non è chiara, ambigua o manca di informazioni sufficienti, chiedi chiarimenti o dettagli aggiuntivi. |
| User   | Per preparare una tazza di tè, inizia facendo bollire dell'acqua in un bollitore. Mentre l'acqua si sta riscaldando, prendi una tazza e metti una bustina di tè dentro. Una volta che l'acqua bolle, versala con attenzione nella tazza, sopra la bustina di tè. Lascia il tè in infusione per alcuni minuti, a seconda di quanto forte ti piace. Dopo l'infusione, rimuovi la bustina di tè e aggiungi latte e zucchero a piacere. Mescola tutto insieme e goditi la tua tazza di tè caldo. |

### Esempio di output

> 1. Riempi un bollitore con acqua e mettilo sul fornello per farlo bollire.
> 2. Mentre aspetti che l'acqua bolla, seleziona una tazza e metti una bustina di tè dentro.
> 3. Una volta che l'acqua raggiunge l'ebollizione, versa con attenzione l'acqua calda nella tazza, sopra la bustina di tè.
> 4. Lascia il tè in infusione nell'acqua calda per il tempo desiderato, tipicamente 2-5 minuti, a seconda della forza preferita.
> 5. Dopo che il tempo di infusione è trascorso, rimuovi la bustina di tè dalla tazza e gettala.
> 6. Aggiungi latte e zucchero al tè, secondo le tue preferenze di gusto.
> 7. Usando un cucchiaio, mescola il tè, il latte e lo zucchero insieme fino a quando sono ben combinati.
> 8. La tua tazza di tè è ora pronta per essere gustata. Bevila mentre è ancora calda.

---

## Richiesta API

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
        system="Il tuo compito è prendere la descrizione in linguaggio naturale fornita di un processo o attività e trasformarla in istruzioni chiare, concise e passo dopo passo che siano logiche, sequenziali e facili da seguire. Usa il linguaggio imperativo e inizia ogni passo con un verbo d'azione. Fornisci dettagli e spiegazioni necessarie per assicurarti che il lettore possa completare l'attività con successo. Se la descrizione originale non è chiara, ambigua o manca di informazioni sufficienti, chiedi chiarimenti o dettagli aggiuntivi.",
        messages=[
            {
                "role": "user",
                "content": [
                    {
                        "type": "text",
                        "text": "Per preparare una tazza di tè, inizia facendo bollire dell'acqua in un bollitore. Mentre l'acqua si sta riscaldando, prendi una tazza e metti una bustina di tè dentro. Una volta che l'acqua bolle, versala con attenzione nella tazza, sopra la bustina di tè. Lascia il tè in infusione per alcuni minuti, a seconda di quanto forte ti piace. Dopo l'infusione, rimuovi la bustina di tè e aggiungi latte e zucchero a piacere. Mescola tutto insieme e goditi la tua tazza di tè caldo."
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
      system: "Il tuo compito è prendere la descrizione in linguaggio naturale fornita di un processo o attività e trasformarla in istruzioni chiare, concise e passo dopo passo che siano logiche, sequenziali e facili da seguire. Usa il linguaggio imperativo e inizia ogni passo con un verbo d'azione. Fornisci dettagli e spiegazioni necessarie per assicurarti che il lettore possa completare l'attività con successo. Se la descrizione originale non è chiara, ambigua o manca di informazioni sufficienti, chiedi chiarimenti o dettagli aggiuntivi.",
      messages: [
        {
          "role": "user",
          "content": [
            {
              "type": "text",
              "text": "Per preparare una tazza di tè, inizia facendo bollire dell'acqua in un bollitore. Mentre l'acqua si sta riscaldando, prendi una tazza e metti una bustina di tè dentro. Una volta che l'acqua bolle, versala con attenzione nella tazza, sopra la bustina di tè. Lascia il tè in infusione per alcuni minuti, a seconda di quanto forte ti piace. Dopo l'infusione, rimuovi la bustina di tè e aggiungi latte e zucchero a piacere. Mescola tutto insieme e goditi la tua tazza di tè caldo."
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
        system="Il tuo compito è prendere la descrizione in linguaggio naturale fornita di un processo o attività e trasformarla in istruzioni chiare, concise e passo dopo passo che siano logiche, sequenziali e facili da seguire. Usa il linguaggio imperativo e inizia ogni passo con un verbo d'azione. Fornisci dettagli e spiegazioni necessarie per assicurarti che il lettore possa completare l'attività con successo. Se la descrizione originale non è chiara, ambigua o manca di informazioni sufficienti, chiedi chiarimenti o dettagli aggiuntivi.",
        messages=[
            {
                "role": "user",
                "content": [
                    {
                        "type": "text",
                        "text": "Per preparare una tazza di tè, inizia facendo bollire dell'acqua in un bollitore. Mentre l'acqua si sta riscaldando, prendi una tazza e metti una bustina di tè dentro. Una volta che l'acqua bolle, versala con attenzione nella tazza, sopra la bustina di tè. Lascia il tè in infusione per alcuni minuti, a seconda di quanto forte ti piace. Dopo l'infusione, rimuovi la bustina di tè e aggiungi latte e zucchero a piacere. Mescola tutto insieme e goditi la tua tazza di tè caldo."
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
      system: "Il tuo compito è prendere la descrizione in linguaggio naturale fornita di un processo o attività e trasformarla in istruzioni chiare, concise e passo dopo passo che siano logiche, sequenziali e facili da seguire. Usa il linguaggio imperativo e inizia ogni passo con un verbo d'azione. Fornisci dettagli e spiegazioni necessarie per assicurarti che il lettore possa completare l'attività con successo. Se la descrizione originale non è chiara, ambigua o manca di informazioni sufficienti, chiedi chiarimenti o dettagli aggiuntivi.",
      messages: [
        {
          "role": "user",
          "content": [
            {
              "type": "text",
              "text": "Per preparare una tazza di tè, inizia facendo bollire dell'acqua in un bollitore. Mentre l'acqua si sta riscaldando, prendi una tazza e metti una bustina di tè dentro. Una volta che l'acqua bolle, versala con attenzione nella tazza, sopra la bustina di tè. Lascia il tè in infusione per alcuni minuti, a seconda di quanto forte ti piace. Dopo l'infusione, rimuovi la bustina di tè e aggiungi latte e zucchero a piacere. Mescola tutto insieme e goditi la tua tazza di tè caldo."
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
        system="Il tuo compito è prendere la descrizione in linguaggio naturale fornita di un processo o attività e trasformarla in istruzioni chiare, concise e passo dopo passo che siano logiche, sequenziali e facili da seguire. Usa il linguaggio imperativo e inizia ogni passo con un verbo d'azione. Fornisci dettagli e spiegazioni necessarie per assicurarti che il lettore possa completare l'attività con successo. Se la descrizione originale non è chiara, ambigua o manca di informazioni sufficienti, chiedi chiarimenti o dettagli aggiuntivi.",
        messages=[
            {
                "role": "user",
                "content": [
                    {
                        "type": "text",
                        "text": "Per preparare una tazza di tè, inizia facendo bollire dell'acqua in un bollitore. Mentre l'acqua si sta riscaldando, prendi una tazza e metti una bustina di tè dentro. Una volta che l'acqua bolle, versala con attenzione nella tazza, sopra la bustina di tè. Lascia il tè in infusione per alcuni minuti, a seconda di quanto forte ti piace. Dopo l'infusione, rimuovi la bustina di tè e aggiungi latte e zucchero a piacere. Mescola tutto insieme e goditi la tua tazza di tè caldo."
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
      system: "Il tuo compito è prendere la descrizione in linguaggio naturale fornita di un processo o attività e trasformarla in istruzioni chiare, concise e passo dopo passo che siano logiche, sequenziali e facili da seguire. Usa il linguaggio imperativo e inizia ogni passo con un verbo d'azione. Fornisci dettagli e spiegazioni necessarie per assicurarti che il lettore possa completare l'attività con successo. Se la descrizione originale non è chiara, ambigua o manca di informazioni sufficienti, chiedi chiarimenti o dettagli aggiuntivi.",
      messages: [
        {
          "role": "user",
          "content": [
            {
              "type": "text",
              "text": "Per preparare una tazza di tè, inizia facendo bollire dell'acqua in un bollitore. Mentre l'acqua si sta riscaldando, prendi una tazza e metti una bustina di tè dentro. Una volta che l'acqua bolle, versala con attenzione nella tazza, sopra la bustina di tè. Lascia il tè in infusione per alcuni minuti, a seconda di quanto forte ti piace. Dopo l'infusione, rimuovi la bustina di tè e aggiungi latte e zucchero a piacere. Mescola tutto insieme e goditi la tua tazza di tè caldo."
            }
          ]
        }
      ]
    });
    console.log(msg);
    
    ```
</CodeGroup>