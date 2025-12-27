# Costruttore di brand

Crea un brief di design per un'identità di marca olistica.

---

> Copia questo prompt nella nostra [Console](/login?selectAccount=true&returnTo=%2Fdashboard%3F) per sviluppatori per provarlo tu stesso!

|        | Contenuto |
| --- | --- |
| System | Il tuo compito è creare un brief di design completo per un'identità di marca olistica basata sulle specifiche fornite. L'identità di marca dovrebbe comprendere vari elementi come suggerimenti per il nome del brand, logo, palette di colori, tipografia, stile visivo, tono di voce e personalità complessiva del brand. Assicurati che tutti gli elementi lavorino insieme armoniosamente per creare un'esperienza di marca coesa e memorabile che comunichi efficacemente i valori del brand, la missione e la proposta di valore unica al suo pubblico target. Sii dettagliato e completo e fornisci abbastanza dettagli specifici perché qualcuno possa creare un'identità di marca veramente unica. |
| User   | Specifiche del brand: Questo è un brand che si concentra sulla creazione di abbigliamento e accessori di alta qualità e stilosi utilizzando materiali eco-compatibili e metodi di produzione etici. Il brand si rivolge a consumatori attenti all'ambiente di età compresa tra 25-40 anni che apprezzano la moda, la sostenibilità e la responsabilità sociale. L'identità del brand dovrebbe raggiungere i seguenti obiettivi: 1. Riflettere l'impegno del brand verso la sostenibilità, le pratiche etiche e la tutela ambientale. 2. Attrarre il pubblico target trasmettendo un senso di stile, qualità e tendenza. 3. Differenziare il brand dai concorrenti nel mercato della moda sostenibile. 4. Creare una forte connessione emotiva con i consumatori e ispirarli a fare scelte più rispettose dell'ambiente. |

### Esempio di output

> Brief di Design: Identità di Marca per un Brand di Moda Eco-Compatibile
> Suggerimento Nome del Brand: "Sēnsus" (Latino per "senso" o "percezione", evocando un senso di consapevolezza e connessione con l'ambiente)
>
> Logo:
>
> - Un logo minimalista e moderno che combina un motivo stilizzato di foglia o pianta con un carattere tipografico elegante e sans-serif.
> - L'elemento foglia o pianta simboleggia l'impegno del brand verso la sostenibilità e la tutela ambientale, mentre la tipografia pulita trasmette un senso di sofisticazione e stile.
> - Il logo dovrebbe essere versatile e funzionare bene in varie applicazioni, inclusi cartellini per abbigliamento, packaging e piattaforme digitali.
>   Palette di Colori:
>
> - Una palette di colori neutri e terrosi che riflette l'ethos eco-compatibile del brand, come sfumature di verde, beige e grigi caldi.
> - Colori di accento come il teal profondo o il giallo senape possono essere utilizzati con parsimonia per aggiungere vivacità e profondità.
> - La palette di colori dovrebbe evocare un senso di armonia e equilibrio naturale, pur rimanendo all'avanguardia nella moda e attraente per il pubblico target.
>   Tipografia:
>
> - Una combinazione di un carattere tipografico sans-serif pulito e moderno per i titoli e un carattere serif complementare per il corpo del testo.
> - La tipografia dovrebbe essere leggibile e visivamente attraente, trasmettendo un senso di qualità e sofisticazione.
> - L'uso coerente della gerarchia tipografica e della spaziatura creerà un'identità di marca coesa e organizzata.
>   Stile Visivo:
>
> - Uno stile visivo minimalista e ordinato che enfatizza la fotografia di prodotto di alta qualità e layout puliti e moderni.
> - Incorporare elementi naturali, come motivi di piante o texture, per rafforzare i valori eco-compatibili del brand e la connessione con la natura.
> - Utilizzare efficacemente lo spazio negativo per creare un senso di apertura e respiro, riflettendo l'impegno del brand verso la sostenibilità.
>   Tono di Voce:
>
> - Il tono di voce del brand dovrebbe essere accessibile, competente e autentico.
> - Utilizzare un linguaggio che educhi e ispiri i consumatori sui benefici della moda sostenibile e dei metodi di produzione etici.
> - Incorporare un tocco di sofisticazione e stile per attrarre le sensibilità attente alla moda del pubblico target.
>   Personalità del Brand:
>
> - Eco-consapevole e attento all'ambiente, ma stiloso e all'avanguardia nella moda.
> - Sicuro e appassionato nel promuovere pratiche sostenibili e scelte etiche.
> - Inclusivo e accogliente, invitando i consumatori a unirsi a una comunità di persone che la pensano allo stesso modo impegnate a fare cambiamenti positivi.
>   Nel complesso, l'identità di marca per "Sēnsus" dovrebbe trovare un equilibrio tra valori eco-compatibili ed estetica all'avanguardia nella moda, attraendo consumatori che vogliono apparire stilosi mentre fanno scelte ambientalmente responsabili. Gli elementi visivi e verbali coesi dovrebbero creare una forte connessione emotiva con il pubblico target e ispirarli ad abbracciare uno stile di vita più sostenibile.

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
        system="Il tuo compito è creare un brief di design completo per un'identità di marca olistica basata sulle specifiche fornite. L'identità di marca dovrebbe comprendere vari elementi come suggerimenti per il nome del brand, logo, palette di colori, tipografia, stile visivo, tono di voce e personalità complessiva del brand. Assicurati che tutti gli elementi lavorino insieme armoniosamente per creare un'esperienza di marca coesa e memorabile che comunichi efficacemente i valori del brand, la missione e la proposta di valore unica al suo pubblico target. Sii dettagliato e completo e fornisci abbastanza dettagli specifici perché qualcuno possa creare un'identità di marca veramente unica.",
        messages=[
            {
                "role": "user",
                "content": [
                    {
                        "type": "text",
                        "text": "Specifiche del brand:\nQuesto è un brand che si concentra sulla creazione di abbigliamento e accessori di alta qualità e stilosi utilizzando materiali eco-compatibili e metodi di produzione etici\nIl brand si rivolge a consumatori attenti all'ambiente di età compresa tra 25-40 anni che apprezzano la moda, la sostenibilità e la responsabilità sociale.\nL'identità del brand dovrebbe raggiungere i seguenti obiettivi:\n1. Riflettere l'impegno del brand verso la sostenibilità, le pratiche etiche e la tutela ambientale.\n2. Attrarre il pubblico target trasmettendo un senso di stile, qualità e tendenza.\n3. Differenziare il brand dai concorrenti nel mercato della moda sostenibile.\n4. Creare una forte connessione emotiva con i consumatori e ispirarli a fare scelte più rispettose dell'ambiente."
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
      system: "Il tuo compito è creare un brief di design completo per un'identità di marca olistica basata sulle specifiche fornite. L'identità di marca dovrebbe comprendere vari elementi come suggerimenti per il nome del brand, logo, palette di colori, tipografia, stile visivo, tono di voce e personalità complessiva del brand. Assicurati che tutti gli elementi lavorino insieme armoniosamente per creare un'esperienza di marca coesa e memorabile che comunichi efficacemente i valori del brand, la missione e la proposta di valore unica al suo pubblico target. Sii dettagliato e completo e fornisci abbastanza dettagli specifici perché qualcuno possa creare un'identità di marca veramente unica.",
      messages: [
        {
          "role": "user",
          "content": [
            {
              "type": "text",
              "text": "Specifiche del brand:\nQuesto è un brand che si concentra sulla creazione di abbigliamento e accessori di alta qualità e stilosi utilizzando materiali eco-compatibili e metodi di produzione etici\nIl brand si rivolge a consumatori attenti all'ambiente di età compresa tra 25-40 anni che apprezzano la moda, la sostenibilità e la responsabilità sociale.\nL'identità del brand dovrebbe raggiungere i seguenti obiettivi:\n1. Riflettere l'impegno del brand verso la sostenibilità, le pratiche etiche e la tutela ambientale.\n2. Attrarre il pubblico target trasmettendo un senso di stile, qualità e tendenza.\n3. Differenziare il brand dai concorrenti nel mercato della moda sostenibile.\n4. Creare una forte connessione emotiva con i consumatori e ispirarli a fare scelte più rispettose dell'ambiente."
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
        system="Il tuo compito è creare un brief di design completo per un'identità di marca olistica basata sulle specifiche fornite. L'identità di marca dovrebbe comprendere vari elementi come suggerimenti per il nome del brand, logo, palette di colori, tipografia, stile visivo, tono di voce e personalità complessiva del brand. Assicurati che tutti gli elementi lavorino insieme armoniosamente per creare un'esperienza di marca coesa e memorabile che comunichi efficacemente i valori del brand, la missione e la proposta di valore unica al suo pubblico target. Sii dettagliato e completo e fornisci abbastanza dettagli specifici perché qualcuno possa creare un'identità di marca veramente unica.",
        messages=[
            {
                "role": "user",
                "content": [
                    {
                        "type": "text",
                        "text": "Specifiche del brand:\nQuesto è un brand che si concentra sulla creazione di abbigliamento e accessori di alta qualità e stilosi utilizzando materiali eco-compatibili e metodi di produzione etici\nIl brand si rivolge a consumatori attenti all'ambiente di età compresa tra 25-40 anni che apprezzano la moda, la sostenibilità e la responsabilità sociale.\nL'identità del brand dovrebbe raggiungere i seguenti obiettivi:\n1. Riflettere l'impegno del brand verso la sostenibilità, le pratiche etiche e la tutela ambientale.\n2. Attrarre il pubblico target trasmettendo un senso di stile, qualità e tendenza.\n3. Differenziare il brand dai concorrenti nel mercato della moda sostenibile.\n4. Creare una forte connessione emotiva con i consumatori e ispirarli a fare scelte più rispettose dell'ambiente."
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
      system: "Il tuo compito è creare un brief di design completo per un'identità di marca olistica basata sulle specifiche fornite. L'identità di marca dovrebbe comprendere vari elementi come suggerimenti per il nome del brand, logo, palette di colori, tipografia, stile visivo, tono di voce e personalità complessiva del brand. Assicurati che tutti gli elementi lavorino insieme armoniosamente per creare un'esperienza di marca coesa e memorabile che comunichi efficacemente i valori del brand, la missione e la proposta di valore unica al suo pubblico target. Sii dettagliato e completo e fornisci abbastanza dettagli specifici perché qualcuno possa creare un'identità di marca veramente unica.",
      messages: [
        {
          "role": "user",
          "content": [
            {
              "type": "text",
              "text": "Specifiche del brand:\nQuesto è un brand che si concentra sulla creazione di abbigliamento e accessori di alta qualità e stilosi utilizzando materiali eco-compatibili e metodi di produzione etici\nIl brand si rivolge a consumatori attenti all'ambiente di età compresa tra 25-40 anni che apprezzano la moda, la sostenibilità e la responsabilità sociale.\nL'identità del brand dovrebbe raggiungere i seguenti obiettivi:\n1. Riflettere l'impegno del brand verso la sostenibilità, le pratiche etiche e la tutela ambientale.\n2. Attrarre il pubblico target trasmettendo un senso di stile, qualità e tendenza.\n3. Differenziare il brand dai concorrenti nel mercato della moda sostenibile.\n4. Creare una forte connessione emotiva con i consumatori e ispirarli a fare scelte più rispettose dell'ambiente."
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
        system="Il tuo compito è creare un brief di design completo per un'identità di marca olistica basata sulle specifiche fornite. L'identità di marca dovrebbe comprendere vari elementi come suggerimenti per il nome del brand, logo, palette di colori, tipografia, stile visivo, tono di voce e personalità complessiva del brand. Assicurati che tutti gli elementi lavorino insieme armoniosamente per creare un'esperienza di marca coesa e memorabile che comunichi efficacemente i valori del brand, la missione e la proposta di valore unica al suo pubblico target. Sii dettagliato e completo e fornisci abbastanza dettagli specifici perché qualcuno possa creare un'identità di marca veramente unica.",
        messages=[
            {
                "role": "user",
                "content": [
                    {
                        "type": "text",
                        "text": "Specifiche del brand:\nQuesto è un brand che si concentra sulla creazione di abbigliamento e accessori di alta qualità e stilosi utilizzando materiali eco-compatibili e metodi di produzione etici\nIl brand si rivolge a consumatori attenti all'ambiente di età compresa tra 25-40 anni che apprezzano la moda, la sostenibilità e la responsabilità sociale.\nL'identità del brand dovrebbe raggiungere i seguenti obiettivi:\n1. Riflettere l'impegno del brand verso la sostenibilità, le pratiche etiche e la tutela ambientale.\n2. Attrarre il pubblico target trasmettendo un senso di stile, qualità e tendenza.\n3. Differenziare il brand dai concorrenti nel mercato della moda sostenibile.\n4. Creare una forte connessione emotiva con i consumatori e ispirarli a fare scelte più rispettose dell'ambiente."
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
      system: "Il tuo compito è creare un brief di design completo per un'identità di marca olistica basata sulle specifiche fornite. L'identità di marca dovrebbe comprendere vari elementi come suggerimenti per il nome del brand, logo, palette di colori, tipografia, stile visivo, tono di voce e personalità complessiva del brand. Assicurati che tutti gli elementi lavorino insieme armoniosamente per creare un'esperienza di marca coesa e memorabile che comunichi efficacemente i valori del brand, la missione e la proposta di valore unica al suo pubblico target. Sii dettagliato e completo e fornisci abbastanza dettagli specifici perché qualcuno possa creare un'identità di marca veramente unica.",
      messages: [
        {
          "role": "user",
          "content": [
            {
              "type": "text",
              "text": "Specifiche del brand:\nQuesto è un brand che si concentra sulla creazione di abbigliamento e accessori di alta qualità e stilosi utilizzando materiali eco-compatibili e metodi di produzione etici\nIl brand si rivolge a consumatori attenti all'ambiente di età compresa tra 25-40 anni che apprezzano la moda, la sostenibilità e la responsabilità sociale.\nL'identità del brand dovrebbe raggiungere i seguenti obiettivi:\n1. Riflettere l'impegno del brand verso la sostenibilità, le pratiche etiche e la tutela ambientale.\n2. Attrarre il pubblico target trasmettendo un senso di stile, qualità e tendenza.\n3. Differenziare il brand dai concorrenti nel mercato della moda sostenibile.\n4. Creare una forte connessione emotiva con i consumatori e ispirarli a fare scelte più rispettose dell'ambiente."
            }
          ]
        }
      ]
    });
    console.log(msg);
    
    ```
</CodeGroup>