# Tweet-Ton-Detektor

Erkennen Sie den Ton und die Stimmung hinter Tweets.

---

> Kopieren Sie diesen Prompt in unsere Entwickler-[Konsole](/dashboard), um es selbst auszuprobieren!

|        | Inhalt |
| --- | --- |
| System | Ihre Aufgabe ist es, den bereitgestellten Tweet zu analysieren und den primären Ton und die Stimmung zu identifizieren, die der Autor ausdrückt. Der Ton sollte als einer der folgenden klassifiziert werden: Positiv, Negativ, Neutral, Humorvoll, Sarkastisch, Enthusiastisch, Wütend oder Informativ. Die Stimmung sollte als Positiv, Negativ oder Neutral klassifiziert werden. Geben Sie eine kurze Erklärung für Ihre Klassifizierungen ab und heben Sie die Schlüsselwörter, Phrasen, Emoticons oder andere Elemente hervor, die Ihre Entscheidung beeinflusst haben. |
| User   | Wow, ich bin so beeindruckt von der Handhabung dieser Krise durch das Unternehmen. 🙄 Sie haben wirklich ihre Prioritäten richtig gesetzt. #sarcasm #fail |

### Beispielausgabe

> Ton: Sarkastisch
> Stimmung: Negativ

### API-Anfrage

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
        system="Ihre Aufgabe ist es, den bereitgestellten Tweet zu analysieren und den primären Ton und die Stimmung zu identifizieren, die der Autor ausdrückt. Der Ton sollte als einer der folgenden klassifiziert werden: Positiv, Negativ, Neutral, Humorvoll, Sarkastisch, Enthusiastisch, Wütend oder Informativ. Die Stimmung sollte als Positiv, Negativ oder Neutral klassifiziert werden. Geben Sie eine kurze Erklärung für Ihre Klassifizierungen ab und heben Sie die Schlüsselwörter, Phrasen, Emoticons oder andere Elemente hervor, die Ihre Entscheidung beeinflusst haben.",
        messages=[
            {
                "role": "user",
                "content": [
                    {
                        "type": "text",
                        "text": "Wow, ich bin so beeindruckt von der Handhabung dieser Krise durch das Unternehmen. 🙄 Sie haben wirklich ihre Prioritäten richtig gesetzt. #sarcasm #fail"
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
      system: "Ihre Aufgabe ist es, den bereitgestellten Tweet zu analysieren und den primären Ton und die Stimmung zu identifizieren, die der Autor ausdrückt. Der Ton sollte als einer der folgenden klassifiziert werden: Positiv, Negativ, Neutral, Humorvoll, Sarkastisch, Enthusiastisch, Wütend oder Informativ. Die Stimmung sollte als Positiv, Negativ oder Neutral klassifiziert werden. Geben Sie eine kurze Erklärung für Ihre Klassifizierungen ab und heben Sie die Schlüsselwörter, Phrasen, Emoticons oder andere Elemente hervor, die Ihre Entscheidung beeinflusst haben.",
      messages: [
        {
          "role": "user",
          "content": [
            {
              "type": "text",
              "text": "Wow, ich bin so beeindruckt von der Handhabung dieser Krise durch das Unternehmen. 🙄 Sie haben wirklich ihre Prioritäten richtig gesetzt. #sarcasm #fail"
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
        system="Ihre Aufgabe ist es, den bereitgestellten Tweet zu analysieren und den primären Ton und die Stimmung zu identifizieren, die der Autor ausdrückt. Der Ton sollte als einer der folgenden klassifiziert werden: Positiv, Negativ, Neutral, Humorvoll, Sarkastisch, Enthusiastisch, Wütend oder Informativ. Die Stimmung sollte als Positiv, Negativ oder Neutral klassifiziert werden. Geben Sie eine kurze Erklärung für Ihre Klassifizierungen ab und heben Sie die Schlüsselwörter, Phrasen, Emoticons oder andere Elemente hervor, die Ihre Entscheidung beeinflusst haben.",
        messages=[
            {
                "role": "user",
                "content": [
                    {
                        "type": "text",
                        "text": "Wow, ich bin so beeindruckt von der Handhabung dieser Krise durch das Unternehmen. 🙄 Sie haben wirklich ihre Prioritäten richtig gesetzt. #sarcasm #fail"
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
      system: "Ihre Aufgabe ist es, den bereitgestellten Tweet zu analysieren und den primären Ton und die Stimmung zu identifizieren, die der Autor ausdrückt. Der Ton sollte als einer der folgenden klassifiziert werden: Positiv, Negativ, Neutral, Humorvoll, Sarkastisch, Enthusiastisch, Wütend oder Informativ. Die Stimmung sollte als Positiv, Negativ oder Neutral klassifiziert werden. Geben Sie eine kurze Erklärung für Ihre Klassifizierungen ab und heben Sie die Schlüsselwörter, Phrasen, Emoticons oder andere Elemente hervor, die Ihre Entscheidung beeinflusst haben.",
      messages: [
        {
          "role": "user",
          "content": [
            {
              "type": "text",
              "text": "Wow, ich bin so beeindruckt von der Handhabung dieser Krise durch das Unternehmen. 🙄 Sie haben wirklich ihre Prioritäten richtig gesetzt. #sarcasm #fail"
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
        system="Ihre Aufgabe ist es, den bereitgestellten Tweet zu analysieren und den primären Ton und die Stimmung zu identifizieren, die der Autor ausdrückt. Der Ton sollte als einer der folgenden klassifiziert werden: Positiv, Negativ, Neutral, Humorvoll, Sarkastisch, Enthusiastisch, Wütend oder Informativ. Die Stimmung sollte als Positiv, Negativ oder Neutral klassifiziert werden. Geben Sie eine kurze Erklärung für Ihre Klassifizierungen ab und heben Sie die Schlüsselwörter, Phrasen, Emoticons oder andere Elemente hervor, die Ihre Entscheidung beeinflusst haben.",
        messages=[
            {
                "role": "user",
                "content": [
                    {
                        "type": "text",
                        "text": "Wow, ich bin so beeindruckt von der Handhabung dieser Krise durch das Unternehmen. 🙄 Sie haben wirklich ihre Prioritäten richtig gesetzt. #sarcasm #fail"
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
      system: "Ihre Aufgabe ist es, den bereitgestellten Tweet zu analysieren und den primären Ton und die Stimmung zu identifizieren, die der Autor ausdrückt. Der Ton sollte als einer der folgenden klassifiziert werden: Positiv, Negativ, Neutral, Humorvoll, Sarkastisch, Enthusiastisch, Wütend oder Informativ. Die Stimmung sollte als Positiv, Negativ oder Neutral klassifiziert werden. Geben Sie eine kurze Erklärung für Ihre Klassifizierungen ab und heben Sie die Schlüsselwörter, Phrasen, Emoticons oder andere Elemente hervor, die Ihre Entscheidung beeinflusst haben.",
      messages: [
        {
          "role": "user",
          "content": [
            {
              "type": "text",
              "text": "Wow, ich bin so beeindruckt von der Handhabung dieser Krise durch das Unternehmen. 🙄 Sie haben wirklich ihre Prioritäten richtig gesetzt. #sarcasm #fail"
            }
          ]
        }
      ]
    });
    console.log(msg);
    
    ```
</CodeGroup>