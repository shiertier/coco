# Philosophische Betrachtungen

Führen Sie tiefgreifende philosophische Diskussionen und Gedankenexperimente.

---

> Kopieren Sie diesen Prompt in unsere Entwickler-[Konsole](/dashboard), um es selbst auszuprobieren!

|        | Inhalt |
| --- | --- |
| System | Ihre Aufgabe ist es, ein philosophisches Konzept oder Gedankenexperiment zu dem gegebenen Thema zu diskutieren. Erklären Sie das Konzept kurz, präsentieren Sie die Hauptargumente und Implikationen und fördern Sie kritisches Denken durch offene Fragen. Bewahren Sie einen ausgewogenen, objektiven Ton, der intellektuelle Neugier fördert. |
| User   | Das Trolley-Problem und die Ethik des Utilitarismus |

### Beispielausgabe

> Das Trolley-Problem ist ein Gedankenexperiment, das unsere moralischen Intuitionen herausfordert und Fragen zur Ethik des Utilitarismus aufwirft. Das grundlegende Szenario ist wie folgt:
>
> Eine außer Kontrolle geratene Straßenbahn ist dabei, fünf Menschen auf den Gleisen vor ihr zu töten. Sie stehen neben einem Hebel, der die Straßenbahn auf ein Nebengleis umleiten kann, wo sie nur eine Person töten wird. Sollten Sie den Hebel ziehen, um fünf Leben auf Kosten eines Lebens zu retten?
>
> Dieses Dilemma hebt das Kernprinzip des Utilitarismus hervor, das besagt, dass die moralisch richtige Handlung diejenige ist, die das größte Wohl für die größte Anzahl von Menschen hervorbringt. In diesem Fall würde das Ziehen des Hebels den Nutzen maximieren, indem es fünf Leben rettet, obwohl es direkt den Tod einer Person verursachen würde.
>
> Das Trolley-Problem wirft jedoch auch Bedenken über die Grenzen des Utilitarismus und die Ethik auf, aktiv Schaden zu verursachen, um ein größeres Wohl zu erreichen. Es veranlasst uns, Fragen zu bedenken wie:
>
> - Ist es moralisch akzeptabel, eine unschuldige Person zu opfern, um eine größere Anzahl von Menschen zu retten?
> - Rechtfertigt das Prinzip des Nutzens jedes Mittel, um das größte Wohl zu erreichen, oder gibt es ethische Beschränkungen, die nicht verletzt werden sollten?
> - Wie bewerten wir den Wert verschiedener Leben, und ist es fair, solche Berechnungen anzustellen?
>
> Was sind Ihre Gedanken zum Trolley-Problem und zur Ethik des Utilitarismus? Glauben Sie, dass die Maximierung des Nutzens das primäre Ziel moralischer Entscheidungsfindung sein sollte, oder gibt es andere wichtige Überlegungen? Wie würden Sie das im Trolley-Problem präsentierte Dilemma angehen, und welche ethischen Prinzipien würden Ihre Wahl leiten?

---

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
        max_tokens=2000,
        temperature=1,
        system="Ihre Aufgabe ist es, ein philosophisches Konzept oder Gedankenexperiment zu dem gegebenen Thema zu diskutieren. Erklären Sie das Konzept kurz, präsentieren Sie die Hauptargumente und Implikationen und fördern Sie kritisches Denken durch offene Fragen. Bewahren Sie einen ausgewogenen, objektiven Ton, der intellektuelle Neugier fördert.",
        messages=[
            {
                "role": "user",
                "content": [
                    {
                        "type": "text",
                        "text": "Das Trolley-Problem und die Ethik des Utilitarismus"
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
      system: "Ihre Aufgabe ist es, ein philosophisches Konzept oder Gedankenexperiment zu dem gegebenen Thema zu diskutieren. Erklären Sie das Konzept kurz, präsentieren Sie die Hauptargumente und Implikationen und fördern Sie kritisches Denken durch offene Fragen. Bewahren Sie einen ausgewogenen, objektiven Ton, der intellektuelle Neugier fördert.",
      messages: [
        {
          "role": "user",
          "content": [
            {
              "type": "text",
              "text": "Das Trolley-Problem und die Ethik des Utilitarismus"
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
        system="Ihre Aufgabe ist es, ein philosophisches Konzept oder Gedankenexperiment zu dem gegebenen Thema zu diskutieren. Erklären Sie das Konzept kurz, präsentieren Sie die Hauptargumente und Implikationen und fördern Sie kritisches Denken durch offene Fragen. Bewahren Sie einen ausgewogenen, objektiven Ton, der intellektuelle Neugier fördert.",
        messages=[
            {
                "role": "user",
                "content": [
                    {
                        "type": "text",
                        "text": "Das Trolley-Problem und die Ethik des Utilitarismus"
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
      system: "Ihre Aufgabe ist es, ein philosophisches Konzept oder Gedankenexperiment zu dem gegebenen Thema zu diskutieren. Erklären Sie das Konzept kurz, präsentieren Sie die Hauptargumente und Implikationen und fördern Sie kritisches Denken durch offene Fragen. Bewahren Sie einen ausgewogenen, objektiven Ton, der intellektuelle Neugier fördert.",
      messages: [
        {
          "role": "user",
          "content": [
            {
              "type": "text",
              "text": "Das Trolley-Problem und die Ethik des Utilitarismus"
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
        system="Ihre Aufgabe ist es, ein philosophisches Konzept oder Gedankenexperiment zu dem gegebenen Thema zu diskutieren. Erklären Sie das Konzept kurz, präsentieren Sie die Hauptargumente und Implikationen und fördern Sie kritisches Denken durch offene Fragen. Bewahren Sie einen ausgewogenen, objektiven Ton, der intellektuelle Neugier fördert.",
        messages=[
            {
                "role": "user",
                "content": [
                    {
                        "type": "text",
                        "text": "Das Trolley-Problem und die Ethik des Utilitarismus"
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
      system: "Ihre Aufgabe ist es, ein philosophisches Konzept oder Gedankenexperiment zu dem gegebenen Thema zu diskutieren. Erklären Sie das Konzept kurz, präsentieren Sie die Hauptargumente und Implikationen und fördern Sie kritisches Denken durch offene Fragen. Bewahren Sie einen ausgewogenen, objektiven Ton, der intellektuelle Neugier fördert.",
      messages: [
        {
          "role": "user",
          "content": [
            {
              "type": "text",
              "text": "Das Trolley-Problem und die Ethik des Utilitarismus"
            }
          ]
        }
      ]
    });
    console.log(msg);
    
    ```
</CodeGroup>