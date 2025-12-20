# Organizzatore di dati

Trasforma testo non strutturato in tabelle JSON personalizzate.

---

> Copia questo prompt nella nostra [Console](/dashboard) per sviluppatori per provarlo tu stesso!

|        | Contenuto |
| --- | --- |
| System | Il tuo compito è prendere il testo non strutturato fornito e convertirlo in un formato tabellare ben organizzato utilizzando JSON. Identifica le entità principali, gli attributi o le categorie menzionate nel testo e usale come chiavi nell'oggetto JSON. Quindi, estrai le informazioni rilevanti dal testo e popola i valori corrispondenti nell'oggetto JSON. Assicurati che i dati siano rappresentati accuratamente e formattati correttamente all'interno della struttura JSON. La tabella JSON risultante dovrebbe fornire una panoramica chiara e strutturata delle informazioni presentate nel testo originale. |
| User   | Silvermist Hollow, un villaggio incantevole, era la casa di un gruppo straordinario di individui. Tra loro c'era il Dr. Liam Patel, un neurochirurgo di 45 anni formato a Yale che rivoluzionò le tecniche chirurgiche presso il centro medico regionale. Olivia Chen, a 28 anni, era un'architetta innovativa di UC Berkeley che trasformò il paesaggio del villaggio con i suoi progetti sostenibili e mozzafiato. Il teatro locale fu impreziosito dalle sinfonie incantevoli di Ethan Kovacs, un musicista e compositore di 72 anni formato alla Juilliard. Isabella Torres, una chef autodidatta con una passione per gli ingredienti di provenienza locale, creò una sensazione culinaria con il suo ristorante farm-to-table, che divenne una destinazione imperdibile per gli amanti del cibo. Questi individui straordinari, ognuno con i propri talenti distinti, contribuirono al vibrante arazzo della vita a Silvermist Hollow. |

### Esempio di output

```json
[
  {
    "name": "Dr. Liam Patel",
    "age": 45,
    "profession": "Neurochirurgo",
    "education": "Yale",
    "accomplishments": "Rivoluzionò le tecniche chirurgiche presso il centro medico regionale"
  },
  {
    "name": "Olivia Chen",
    "age": 28,
    "profession": "Architetta",
    "education": "UC Berkeley",
    "accomplishments": "Trasformò il paesaggio del villaggio con progetti sostenibili e mozzafiato"
  },
  {
    "name": "Ethan Kovacs",
    "age": 72,
    "profession": "Musicista e Compositore",
    "education": "Juilliard",
    "accomplishments": "Impreziosì il teatro locale con sinfonie incantevoli"
  },
  {
    "name": "Isabella Torres",
    "age": null,
    "profession": "Chef",
    "education": "Autodidatta",
    "accomplishments": "Creò una sensazione culinaria con il suo ristorante farm-to-table, che divenne una destinazione imperdibile per gli amanti del cibo"
  }
]
```

---

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
        system="Il tuo compito è prendere il testo non strutturato fornito e convertirlo in un formato tabellare ben organizzato utilizzando JSON. Identifica le entità principali, gli attributi o le categorie menzionate nel testo e usale come chiavi nell'oggetto JSON. Quindi, estrai le informazioni rilevanti dal testo e popola i valori corrispondenti nell'oggetto JSON. Assicurati che i dati siano rappresentati accuratamente e formattati correttamente all'interno della struttura JSON. La tabella JSON risultante dovrebbe fornire una panoramica chiara e strutturata delle informazioni presentate nel testo originale.",
        messages=[
            {
                "role": "user",
                "content": [
                    {
                        "type": "text",
                        "text": "Silvermist Hollow, un villaggio incantevole, era la casa di un gruppo straordinario di individui. Tra loro c'era il Dr. Liam Patel, un neurochirurgo di 45 anni formato a Yale che rivoluzionò le tecniche chirurgiche presso il centro medico regionale. Olivia Chen, a 28 anni, era un'architetta innovativa di UC Berkeley che trasformò il paesaggio del villaggio con i suoi progetti sostenibili e mozzafiato. Il teatro locale fu impreziosito dalle sinfonie incantevoli di Ethan Kovacs, un musicista e compositore di 72 anni formato alla Juilliard. Isabella Torres, una chef autodidatta con una passione per gli ingredienti di provenienza locale, creò una sensazione culinaria con il suo ristorante farm-to-table, che divenne una destinazione imperdibile per gli amanti del cibo. Questi individui straordinari, ognuno con i propri talenti distinti, contribuirono al vibrante arazzo della vita a Silvermist Hollow."
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
      system: "Il tuo compito è prendere il testo non strutturato fornito e convertirlo in un formato tabellare ben organizzato utilizzando JSON. Identifica le entità principali, gli attributi o le categorie menzionate nel testo e usale come chiavi nell'oggetto JSON. Quindi, estrai le informazioni rilevanti dal testo e popola i valori corrispondenti nell'oggetto JSON. Assicurati che i dati siano rappresentati accuratamente e formattati correttamente all'interno della struttura JSON. La tabella JSON risultante dovrebbe fornire una panoramica chiara e strutturata delle informazioni presentate nel testo originale.",
      messages: [
        {
          "role": "user",
          "content": [
            {
              "type": "text",
              "text": "Silvermist Hollow, un villaggio incantevole, era la casa di un gruppo straordinario di individui. Tra loro c'era il Dr. Liam Patel, un neurochirurgo di 45 anni formato a Yale che rivoluzionò le tecniche chirurgiche presso il centro medico regionale. Olivia Chen, a 28 anni, era un'architetta innovativa di UC Berkeley che trasformò il paesaggio del villaggio con i suoi progetti sostenibili e mozzafiato. Il teatro locale fu impreziosito dalle sinfonie incantevoli di Ethan Kovacs, un musicista e compositore di 72 anni formato alla Juilliard. Isabella Torres, una chef autodidatta con una passione per gli ingredienti di provenienza locale, creò una sensazione culinaria con il suo ristorante farm-to-table, che divenne una destinazione imperdibile per gli amanti del cibo. Questi individui straordinari, ognuno con i propri talenti distinti, contribuirono al vibrante arazzo della vita a Silvermist Hollow."
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
        system="Il tuo compito è prendere il testo non strutturato fornito e convertirlo in un formato tabellare ben organizzato utilizzando JSON. Identifica le entità principali, gli attributi o le categorie menzionate nel testo e usale come chiavi nell'oggetto JSON. Quindi, estrai le informazioni rilevanti dal testo e popola i valori corrispondenti nell'oggetto JSON. Assicurati che i dati siano rappresentati accuratamente e formattati correttamente all'interno della struttura JSON. La tabella JSON risultante dovrebbe fornire una panoramica chiara e strutturata delle informazioni presentate nel testo originale.",
        messages=[
            {
                "role": "user",
                "content": [
                    {
                        "type": "text",
                        "text": "Silvermist Hollow, un villaggio incantevole, era la casa di un gruppo straordinario di individui. Tra loro c'era il Dr. Liam Patel, un neurochirurgo di 45 anni formato a Yale che rivoluzionò le tecniche chirurgiche presso il centro medico regionale. Olivia Chen, a 28 anni, era un'architetta innovativa di UC Berkeley che trasformò il paesaggio del villaggio con i suoi progetti sostenibili e mozzafiato. Il teatro locale fu impreziosito dalle sinfonie incantevoli di Ethan Kovacs, un musicista e compositore di 72 anni formato alla Juilliard. Isabella Torres, una chef autodidatta con una passione per gli ingredienti di provenienza locale, creò una sensazione culinaria con il suo ristorante farm-to-table, che divenne una destinazione imperdibile per gli amanti del cibo. Questi individui straordinari, ognuno con i propri talenti distinti, contribuirono al vibrante arazzo della vita a Silvermist Hollow."
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
      system: "Il tuo compito è prendere il testo non strutturato fornito e convertirlo in un formato tabellare ben organizzato utilizzando JSON. Identifica le entità principali, gli attributi o le categorie menzionate nel testo e usale come chiavi nell'oggetto JSON. Quindi, estrai le informazioni rilevanti dal testo e popola i valori corrispondenti nell'oggetto JSON. Assicurati che i dati siano rappresentati accuratamente e formattati correttamente all'interno della struttura JSON. La tabella JSON risultante dovrebbe fornire una panoramica chiara e strutturata delle informazioni presentate nel testo originale.",
      messages: [
        {
          "role": "user",
          "content": [
            {
              "type": "text",
              "text": "Silvermist Hollow, un villaggio incantevole, era la casa di un gruppo straordinario di individui. Tra loro c'era il Dr. Liam Patel, un neurochirurgo di 45 anni formato a Yale che rivoluzionò le tecniche chirurgiche presso il centro medico regionale. Olivia Chen, a 28 anni, era un'architetta innovativa di UC Berkeley che trasformò il paesaggio del villaggio con i suoi progetti sostenibili e mozzafiato. Il teatro locale fu impreziosito dalle sinfonie incantevoli di Ethan Kovacs, un musicista e compositore di 72 anni formato alla Juilliard. Isabella Torres, una chef autodidatta con una passione per gli ingredienti di provenienza locale, creò una sensazione culinaria con il suo ristorante farm-to-table, che divenne una destinazione imperdibile per gli amanti del cibo. Questi individui straordinari, ognuno con i propri talenti distinti, contribuirono al vibrante arazzo della vita a Silvermist Hollow."
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
        system="Il tuo compito è prendere il testo non strutturato fornito e convertirlo in un formato tabellare ben organizzato utilizzando JSON. Identifica le entità principali, gli attributi o le categorie menzionate nel testo e usale come chiavi nell'oggetto JSON. Quindi, estrai le informazioni rilevanti dal testo e popola i valori corrispondenti nell'oggetto JSON. Assicurati che i dati siano rappresentati accuratamente e formattati correttamente all'interno della struttura JSON. La tabella JSON risultante dovrebbe fornire una panoramica chiara e strutturata delle informazioni presentate nel testo originale.",
        messages=[
            {
                "role": "user",
                "content": [
                    {
                        "type": "text",
                        "text": "Silvermist Hollow, un villaggio incantevole, era la casa di un gruppo straordinario di individui. Tra loro c'era il Dr. Liam Patel, un neurochirurgo di 45 anni formato a Yale che rivoluzionò le tecniche chirurgiche presso il centro medico regionale. Olivia Chen, a 28 anni, era un'architetta innovativa di UC Berkeley che trasformò il paesaggio del villaggio con i suoi progetti sostenibili e mozzafiato. Il teatro locale fu impreziosito dalle sinfonie incantevoli di Ethan Kovacs, un musicista e compositore di 72 anni formato alla Juilliard. Isabella Torres, una chef autodidatta con una passione per gli ingredienti di provenienza locale, creò una sensazione culinaria con il suo ristorante farm-to-table, che divenne una destinazione imperdibile per gli amanti del cibo. Questi individui straordinari, ognuno con i propri talenti distinti, contribuirono al vibrante arazzo della vita a Silvermist Hollow."
                    }
                ]
            }
        ]
    )
    print(message.content)
    
    ```
    
    
    ```typescript Vertex AI Type
    import { AnthropicVertex } from '@anthropic-ai/vertex-sdk';
    
    // Reads from the `CLOUD_ML_REGION` & `ANTHROPIC_VERTEX_PROJECT_ID` environment variables.
    // Additionally goes through the standard `google-auth-library` flow.
    const client = new AnthropicVertex();
    
    const msg = await client.messages.create({
      model: "claude-sonnet-4@20250514",
      max_tokens: 1000,
      temperature: 0,
      system: "Il tuo compito è prendere il testo non strutturato fornito e convertirlo in un formato tabellare ben organizzato utilizzando JSON. Identifica le entità principali, gli attributi o le categorie menzionate nel testo e usale come chiavi nell'oggetto JSON. Quindi, estrai le informazioni rilevanti dal testo e popola i valori corrispondenti nell'oggetto JSON. Assicurati che i dati siano rappresentati accuratamente e formattati correttamente all'interno della struttura JSON. La tabella JSON risultante dovrebbe fornire una panoramica chiara e strutturata delle informazioni presentate nel testo originale.",
      messages: [
        {
          "role": "user",
          "content": [
            {
              "type": "text",
              "text": "Silvermist Hollow, un villaggio incantevole, era la casa di un gruppo straordinario di individui. Tra loro c'era il Dr. Liam Patel, un neurochirurgo di 45 anni formato a Yale che rivoluzionò le tecniche chirurgiche presso il centro medico regionale. Olivia Chen, a 28 anni, era un'architetta innovativa di UC Berkeley che trasformò il paesaggio del villaggio con i suoi progetti sostenibili e mozzafiato. Il teatro locale fu impreziosito dalle sinfonie incantevoli di Ethan Kovacs, un musicista e compositore di 72 anni formato alla Juilliard. Isabella Torres, una chef autodidatta con una passione per gli ingredienti di provenienza locale, creò una sensazione culinaria con il suo ristorante farm-to-table, che divenne una destinazione imperdibile per gli amanti del cibo. Questi individui straordinari, ognuno con i propri talenti distinti, contribuirono al vibrante arazzo della vita a Silvermist Hollow."
            }
          ]
        }
      ]
    });
    console.log(msg);
    
    ```
</CodeGroup>