# Pianificatore di lezioni

Crea piani di lezione approfonditi su qualsiasi argomento.

---

> Copia questo prompt nella nostra [Console](/login?selectAccount=true&returnTo=%2Fdashboard%3F) per sviluppatori per provarlo tu stesso!

|        | Contenuto |
| --- | --- |
| System | Il tuo compito è creare un piano di lezione completo, coinvolgente e ben strutturato sull'argomento dato. Il piano di lezione dovrebbe essere progettato per una sessione di classe di 60 minuti e dovrebbe essere adatto a un livello di classe o gruppo di età specifico. Inizia dichiarando gli obiettivi della lezione, che dovrebbero essere chiari, misurabili e allineati con gli standard educativi pertinenti. Successivamente, fornisci uno schema dettagliato della lezione, suddividendola in un'introduzione, attività principali e una conclusione. Per ogni sezione, descrivi i metodi di insegnamento, le attività di apprendimento e le risorse che utilizzerai per trasmettere efficacemente il contenuto e coinvolgere gli studenti. Infine, descrivi i metodi di valutazione che impiegherai per valutare la comprensione e la padronanza degli obiettivi della lezione da parte degli studenti. Il piano di lezione dovrebbe essere ben organizzato, facile da seguire e promuovere l'apprendimento attivo e il pensiero critico. |
| User   | Argomento: Introduzione alla Fotosintesi Livello di Classe: 7ª Classe (Età 12-13) |

## Esempio di output

> Piano di Lezione: Introduzione alla Fotosintesi
> Livello di Classe: 7ª Classe (Età 12-13)
> Durata: 60 minuti
>
> Obiettivi della Lezione:
> Alla fine della lezione, gli studenti saranno in grado di:
>
> 1. Definire la fotosintesi e spiegare la sua importanza nell'ecosistema.
> 2. Identificare i componenti chiave necessari perché avvenga la fotosintesi.
> 3. Descrivere il processo di fotosintesi, inclusi i ruoli dell'energia luminosa, dell'anidride carbonica e dell'acqua.
> 4. Riconoscere i prodotti della fotosintesi e la loro importanza per gli organismi viventi.
>    Schema della Lezione:
>
> 5. Introduzione (10 minuti)
>    i. Coinvolgi gli studenti mostrando un breve video clip o immagini di piante e discutendo il loro ruolo nell'ambiente.
>    ii. Introduci il concetto di fotosintesi e la sua importanza nel sostenere la vita sulla Terra.
>    iii. Dichiara gli obiettivi della lezione e fornisci una panoramica della struttura della lezione.
> 6. Attività Principali (40 minuti)
>    i. Istruzione Guidata (15 minuti)
>    a. Spiega i componenti chiave necessari per la fotosintesi: luce solare, anidride carbonica, acqua e clorofilla.
>    b. Descrivi il processo di fotosintesi, enfatizzando i ruoli dell'energia luminosa, dell'anidride carbonica e dell'acqua.
>    c. Discuti i prodotti della fotosintesi (glucosio e ossigeno) e la loro importanza per gli organismi viventi.
>    ii. Attività di Piccolo Gruppo (15 minuti)
>    a. Dividi la classe in piccoli gruppi e fornisci a ogni gruppo un foglio di lavoro o un organizzatore grafico.
>    b. Chiedi agli studenti di lavorare insieme per creare una rappresentazione visiva del processo di fotosintesi, etichettando i componenti chiave e i prodotti.
>    c. Incoraggia gli studenti a discutere e spiegare la loro comprensione della fotosintesi all'interno dei loro gruppi.
>    iii. Discussione di Classe (10 minuti)
>    a. Invita ogni gruppo a condividere la loro rappresentazione visiva con la classe.
>    b. Facilita una discussione di classe, incoraggiando gli studenti a fare domande, fornire feedback e chiarire eventuali misconcezioni.
> 7. Conclusione (10 minuti)
>    i. Riassumi i punti chiave della lezione, rafforzando la definizione, i componenti, il processo e i prodotti della fotosintesi.
>    ii. Somministra un breve quiz o biglietto di uscita per valutare la comprensione degli obiettivi della lezione da parte degli studenti.
>    iii. Fornisci agli studenti un compito a casa per esplorare ulteriormente l'importanza della fotosintesi in diversi ecosistemi.
>    Metodi di Valutazione:
>
> 8. Valutazione formativa: Osserva la partecipazione e la comprensione degli studenti durante l'istruzione guidata e l'attività di piccolo gruppo.
> 9. Valutazione sommativa: Valuta le rappresentazioni visive degli studenti e la loro performance nel breve quiz o biglietto di uscita.
> 10. Compito a casa: Rivedi le consegne dei compiti a casa degli studenti per valutare la loro capacità di applicare i concetti appresi nella lezione.
>     Risorse:
>
> 11. Video clip o immagini di piante e il loro ruolo nell'ambiente
> 12. Fogli di lavoro o organizzatori grafici per l'attività di piccolo gruppo
> 13. Breve quiz o biglietto di uscita per la valutazione
> 14. Istruzioni per il compito a casa

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
        max_tokens=4000,
        temperature=0.5,
        system="Il tuo compito è creare un piano di lezione completo, coinvolgente e ben strutturato sull'argomento dato. Il piano di lezione dovrebbe essere progettato per una sessione di classe di 60 minuti e dovrebbe essere adatto a un livello di classe o gruppo di età specifico. Inizia dichiarando gli obiettivi della lezione, che dovrebbero essere chiari, misurabili e allineati con gli standard educativi pertinenti. Successivamente, fornisci uno schema dettagliato della lezione, suddividendola in un'introduzione, attività principali e una conclusione. Per ogni sezione, descrivi i metodi di insegnamento, le attività di apprendimento e le risorse che utilizzerai per trasmettere efficacemente il contenuto e coinvolgere gli studenti. Infine, descrivi i metodi di valutazione che impiegherai per valutare la comprensione e la padronanza degli obiettivi della lezione da parte degli studenti. Il piano di lezione dovrebbe essere ben organizzato, facile da seguire e promuovere l'apprendimento attivo e il pensiero critico.",
        messages=[
            {
                "role": "user",
                "content": [
                    {
                        "type": "text",
                        "text": "Argomento: Introduzione alla Fotosintesi  \nLivello di Classe: 7ª Classe (Età 12-13)"
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
      max_tokens: 4000,
      temperature: 0.5,
      system: "Il tuo compito è creare un piano di lezione completo, coinvolgente e ben strutturato sull'argomento dato. Il piano di lezione dovrebbe essere progettato per una sessione di classe di 60 minuti e dovrebbe essere adatto a un livello di classe o gruppo di età specifico. Inizia dichiarando gli obiettivi della lezione, che dovrebbero essere chiari, misurabili e allineati con gli standard educativi pertinenti. Successivamente, fornisci uno schema dettagliato della lezione, suddividendola in un'introduzione, attività principali e una conclusione. Per ogni sezione, descrivi i metodi di insegnamento, le attività di apprendimento e le risorse che utilizzerai per trasmettere efficacemente il contenuto e coinvolgere gli studenti. Infine, descrivi i metodi di valutazione che impiegherai per valutare la comprensione e la padronanza degli obiettivi della lezione da parte degli studenti. Il piano di lezione dovrebbe essere ben organizzato, facile da seguire e promuovere l'apprendimento attivo e il pensiero critico.",
      messages: [
        {
          "role": "user",
          "content": [
            {
              "type": "text",
              "text": "Argomento: Introduzione alla Fotosintesi  \nLivello di Classe: 7ª Classe (Età 12-13)"
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
        max_tokens=4000,
        temperature=0.5,
        system="Il tuo compito è creare un piano di lezione completo, coinvolgente e ben strutturato sull'argomento dato. Il piano di lezione dovrebbe essere progettato per una sessione di classe di 60 minuti e dovrebbe essere adatto a un livello di classe o gruppo di età specifico. Inizia dichiarando gli obiettivi della lezione, che dovrebbero essere chiari, misurabili e allineati con gli standard educativi pertinenti. Successivamente, fornisci uno schema dettagliato della lezione, suddividendola in un'introduzione, attività principali e una conclusione. Per ogni sezione, descrivi i metodi di insegnamento, le attività di apprendimento e le risorse che utilizzerai per trasmettere efficacemente il contenuto e coinvolgere gli studenti. Infine, descrivi i metodi di valutazione che impiegherai per valutare la comprensione e la padronanza degli obiettivi della lezione da parte degli studenti. Il piano di lezione dovrebbe essere ben organizzato, facile da seguire e promuovere l'apprendimento attivo e il pensiero critico.",
        messages=[
            {
                "role": "user",
                "content": [
                    {
                        "type": "text",
                        "text": "Argomento: Introduzione alla Fotosintesi  \nLivello di Classe: 7ª Classe (Età 12-13)"
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
      max_tokens: 4000,
      temperature: 0.5,
      system: "Il tuo compito è creare un piano di lezione completo, coinvolgente e ben strutturato sull'argomento dato. Il piano di lezione dovrebbe essere progettato per una sessione di classe di 60 minuti e dovrebbe essere adatto a un livello di classe o gruppo di età specifico. Inizia dichiarando gli obiettivi della lezione, che dovrebbero essere chiari, misurabili e allineati con gli standard educativi pertinenti. Successivamente, fornisci uno schema dettagliato della lezione, suddividendola in un'introduzione, attività principali e una conclusione. Per ogni sezione, descrivi i metodi di insegnamento, le attività di apprendimento e le risorse che utilizzerai per trasmettere efficacemente il contenuto e coinvolgere gli studenti. Infine, descrivi i metodi di valutazione che impiegherai per valutare la comprensione e la padronanza degli obiettivi della lezione da parte degli studenti. Il piano di lezione dovrebbe essere ben organizzato, facile da seguire e promuovere l'apprendimento attivo e il pensiero critico.",
      messages: [
        {
          "role": "user",
          "content": [
            {
              "type": "text",
              "text": "Argomento: Introduzione alla Fotosintesi  \nLivello di Classe: 7ª Classe (Età 12-13)"
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
        max_tokens=4000,
        temperature=0.5,
        system="Il tuo compito è creare un piano di lezione completo, coinvolgente e ben strutturato sull'argomento dato. Il piano di lezione dovrebbe essere progettato per una sessione di classe di 60 minuti e dovrebbe essere adatto a un livello di classe o gruppo di età specifico. Inizia dichiarando gli obiettivi della lezione, che dovrebbero essere chiari, misurabili e allineati con gli standard educativi pertinenti. Successivamente, fornisci uno schema dettagliato della lezione, suddividendola in un'introduzione, attività principali e una conclusione. Per ogni sezione, descrivi i metodi di insegnamento, le attività di apprendimento e le risorse che utilizzerai per trasmettere efficacemente il contenuto e coinvolgere gli studenti. Infine, descrivi i metodi di valutazione che impiegherai per valutare la comprensione e la padronanza degli obiettivi della lezione da parte degli studenti. Il piano di lezione dovrebbe essere ben organizzato, facile da seguire e promuovere l'apprendimento attivo e il pensiero critico.",
        messages=[
            {
                "role": "user",
                "content": [
                    {
                        "type": "text",
                        "text": "Argomento: Introduzione alla Fotosintesi  \nLivello di Classe: 7ª Classe (Età 12-13)"
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
      max_tokens: 4000,
      temperature: 0.5,
      system: "Il tuo compito è creare un piano di lezione completo, coinvolgente e ben strutturato sull'argomento dato. Il piano di lezione dovrebbe essere progettato per una sessione di classe di 60 minuti e dovrebbe essere adatto a un livello di classe o gruppo di età specifico. Inizia dichiarando gli obiettivi della lezione, che dovrebbero essere chiari, misurabili e allineati con gli standard educativi pertinenti. Successivamente, fornisci uno schema dettagliato della lezione, suddividendola in un'introduzione, attività principali e una conclusione. Per ogni sezione, descrivi i metodi di insegnamento, le attività di apprendimento e le risorse che utilizzerai per trasmettere efficacemente il contenuto e coinvolgere gli studenti. Infine, descrivi i metodi di valutazione che impiegherai per valutare la comprensione e la padronanza degli obiettivi della lezione da parte degli studenti. Il piano di lezione dovrebbe essere ben organizzato, facile da seguire e promuovere l'apprendimento attivo e il pensiero critico.",
      messages: [
        {
          "role": "user",
          "content": [
            {
              "type": "text",
              "text": "Argomento: Introduzione alla Fotosintesi  \nLivello di Classe: 7ª Classe (Età 12-13)"
            }
          ]
        }
      ]
    });
    console.log(msg);
    
    ```
</CodeGroup>