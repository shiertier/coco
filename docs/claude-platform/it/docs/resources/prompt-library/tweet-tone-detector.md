# Rilevatore di tono dei tweet

Rileva il tono e il sentimento dietro i tweet.

---

> Copia questo prompt nella nostra [Console](/dashboard) per sviluppatori per provarlo tu stesso!

|        | Contenuto |
| --- | --- |
| Sistema | Il tuo compito è analizzare il tweet fornito e identificare il tono principale e il sentimento espresso dall'autore. Il tono dovrebbe essere classificato come uno dei seguenti: Positivo, Negativo, Neutrale, Umoristico, Sarcastico, Entusiasta, Arrabbiato o Informativo. Il sentimento dovrebbe essere classificato come Positivo, Negativo o Neutrale. Fornisci una breve spiegazione per le tue classificazioni, evidenziando le parole chiave, le frasi, le emoticon o altri elementi che hanno influenzato la tua decisione. |
| Utente   | Wow, sono così impressionato dalla gestione di questa crisi da parte dell'azienda. 🙄 Hanno davvero le loro priorità in ordine. #sarcasm #fail |

### Esempio di output

> Tono: Sarcastico
> Sentimento: Negativo

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
        system="Il tuo compito è analizzare il tweet fornito e identificare il tono principale e il sentimento espresso dall'autore. Il tono dovrebbe essere classificato come uno dei seguenti: Positivo, Negativo, Neutrale, Umoristico, Sarcastico, Entusiasta, Arrabbiato o Informativo. Il sentimento dovrebbe essere classificato come Positivo, Negativo o Neutrale. Fornisci una breve spiegazione per le tue classificazioni, evidenziando le parole chiave, le frasi, le emoticon o altri elementi che hanno influenzato la tua decisione.",
        messages=[
            {
                "role": "user",
                "content": [
                    {
                        "type": "text",
                        "text": "Wow, sono così impressionato dalla gestione di questa crisi da parte dell'azienda. 🙄 Hanno davvero le loro priorità in ordine. #sarcasm #fail"
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
      system: "Il tuo compito è analizzare il tweet fornito e identificare il tono principale e il sentimento espresso dall'autore. Il tono dovrebbe essere classificato come uno dei seguenti: Positivo, Negativo, Neutrale, Umoristico, Sarcastico, Entusiasta, Arrabbiato o Informativo. Il sentimento dovrebbe essere classificato come Positivo, Negativo o Neutrale. Fornisci una breve spiegazione per le tue classificazioni, evidenziando le parole chiave, le frasi, le emoticon o altri elementi che hanno influenzato la tua decisione.",
      messages: [
        {
          "role": "user",
          "content": [
            {
              "type": "text",
              "text": "Wow, sono così impressionato dalla gestione di questa crisi da parte dell'azienda. 🙄 Hanno davvero le loro priorità in ordine. #sarcasm #fail"
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
        system="Il tuo compito è analizzare il tweet fornito e identificare il tono principale e il sentimento espresso dall'autore. Il tono dovrebbe essere classificato come uno dei seguenti: Positivo, Negativo, Neutrale, Umoristico, Sarcastico, Entusiasta, Arrabbiato o Informativo. Il sentimento dovrebbe essere classificato come Positivo, Negativo o Neutrale. Fornisci una breve spiegazione per le tue classificazioni, evidenziando le parole chiave, le frasi, le emoticon o altri elementi che hanno influenzato la tua decisione.",
        messages=[
            {
                "role": "user",
                "content": [
                    {
                        "type": "text",
                        "text": "Wow, sono così impressionato dalla gestione di questa crisi da parte dell'azienda. 🙄 Hanno davvero le loro priorità in ordine. #sarcasm #fail"
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
      system: "Il tuo compito è analizzare il tweet fornito e identificare il tono principale e il sentimento espresso dall'autore. Il tono dovrebbe essere classificato come uno dei seguenti: Positivo, Negativo, Neutrale, Umoristico, Sarcastico, Entusiasta, Arrabbiato o Informativo. Il sentimento dovrebbe essere classificato come Positivo, Negativo o Neutrale. Fornisci una breve spiegazione per le tue classificazioni, evidenziando le parole chiave, le frasi, le emoticon o altri elementi che hanno influenzato la tua decisione.",
      messages: [
        {
          "role": "user",
          "content": [
            {
              "type": "text",
              "text": "Wow, sono così impressionato dalla gestione di questa crisi da parte dell'azienda. 🙄 Hanno davvero le loro priorità in ordine. #sarcasm #fail"
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
        system="Il tuo compito è analizzare il tweet fornito e identificare il tono principale e il sentimento espresso dall'autore. Il tono dovrebbe essere classificato come uno dei seguenti: Positivo, Negativo, Neutrale, Umoristico, Sarcastico, Entusiasta, Arrabbiato o Informativo. Il sentimento dovrebbe essere classificato come Positivo, Negativo o Neutrale. Fornisci una breve spiegazione per le tue classificazioni, evidenziando le parole chiave, le frasi, le emoticon o altri elementi che hanno influenzato la tua decisione.",
        messages=[
            {
                "role": "user",
                "content": [
                    {
                        "type": "text",
                        "text": "Wow, sono così impressionato dalla gestione di questa crisi da parte dell'azienda. 🙄 Hanno davvero le loro priorità in ordine. #sarcasm #fail"
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
      system: "Il tuo compito è analizzare il tweet fornito e identificare il tono principale e il sentimento espresso dall'autore. Il tono dovrebbe essere classificato come uno dei seguenti: Positivo, Negativo, Neutrale, Umoristico, Sarcastico, Entusiasta, Arrabbiato o Informativo. Il sentimento dovrebbe essere classificato come Positivo, Negativo o Neutrale. Fornisci una breve spiegazione per le tue classificazioni, evidenziando le parole chiave, le frasi, le emoticon o altri elementi che hanno influenzato la tua decisione.",
      messages: [
        {
          "role": "user",
          "content": [
            {
              "type": "text",
              "text": "Wow, sono così impressionato dalla gestione di questa crisi da parte dell'azienda. 🙄 Hanno davvero le loro priorità in ordine. #sarcasm #fail"
            }
          ]
        }
      ]
    });
    console.log(msg);
    
    ```
</CodeGroup>