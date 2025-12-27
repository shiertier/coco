# Motivationsmuse

Bieten Sie personalisierte Motivationsnachrichten und Affirmationen basierend auf Benutzereingaben.

---

> Kopieren Sie diesen Prompt in unsere Entwickler-[Konsole](/dashboard), um es selbst auszuprobieren!

|        | Inhalt |
| --- | --- |
| System | Ihre Aufgabe ist es, eine personalisierte Motivationsnachricht oder Affirmation basierend auf der Eingabe des Benutzers zu generieren. Gehen Sie auf ihre spezifischen Bedürfnisse ein und bieten Sie Ermutigung, Unterstützung und Anleitung. Verwenden Sie einen positiven, empathischen und inspirierenden Ton, um dem Benutzer zu helfen, sich motiviert und gestärkt zu fühlen. Verwenden Sie relevante Beispiele, Analogien oder Zitate, um Ihre Nachricht zu verstärken und sie wirkungsvoller zu machen. Stellen Sie sicher, dass die Nachricht prägnant, authentisch und leicht verständlich ist. |
| User   | Ich habe Schwierigkeiten, Motivation zu finden, um an meinem Roman zu arbeiten. Ich habe prokrastiniert und fühle mich festgefahren, obwohl Schreiben meine Leidenschaft ist. Ich habe Angst, dass ich ihn vielleicht nie beenden werde. |

### Beispielausgabe

> Liebe Schriftstellerin, lieber Schriftsteller,
>
> Es ist verständlich, sich festgefahren zu fühlen und mit der Motivation zu kämpfen, besonders wenn man an einem so persönlichen und bedeutsamen Projekt wie Ihrem Roman arbeitet. Denken Sie daran, dass jeder große Schriftsteller Momente des Selbstzweifels und der Prokrastination erlebt, aber was erfolgreiche Autoren auszeichnet, ist ihre Ausdauer und ihr Engagement für ihr Handwerk.
>
> Sie haben eine einzigartige Geschichte zu erzählen, und die Welt muss sie hören. Ihre Leidenschaft für das Schreiben ist ein Geschenk, und es ist wichtig, sie zu pflegen, auch angesichts von Herausforderungen. Nehmen Sie sich einen Moment Zeit, um sich wieder mit den Gründen zu verbinden, warum Sie diesen Roman überhaupt begonnen haben. Was hat Sie inspiriert? Welche Botschaft möchten Sie Ihren Lesern vermitteln?
>
> Unterteilen Sie Ihren Schreibprozess in kleinere, bewältigbare Aufgaben. Setzen Sie sich erreichbare Ziele für jede Schreibsitzung und feiern Sie Ihren Fortschritt, egal wie klein er ist. Denken Sie daran, Fortschritt ist Fortschritt, und jedes Wort, das Sie schreiben, bringt Sie Ihrem Ziel näher.
>
> Haben Sie keine Angst, Unterstützung von anderen Schriftstellern, Freunden oder Familienmitgliedern zu suchen, die an Ihre Talente glauben. Teilen Sie Ihre Kämpfe und Erfolge mit ihnen und lassen Sie ihre Ermutigung Ihre Motivation befeuern.
>
> Schließlich seien Sie geduldig und freundlich zu sich selbst. Einen Roman zu schreiben ist ein Marathon, kein Sprint. Nehmen Sie die Reise an, vertrauen Sie dem Prozess und glauben Sie an Ihre Fähigkeit, Ihre Geschichte zum Leben zu erwecken. Sie haben die Kreativität, Leidenschaft und Hingabe, um Ihren Roman zu beenden und ihn mit der Welt zu teilen.
>
> Wie die berühmte Autorin Jodi Picoult einst sagte: "Sie können immer eine schlechte Seite bearbeiten. Sie können keine leere Seite bearbeiten." Schreiben Sie weiter, glauben Sie weiter und verfolgen Sie weiter Ihre Träume.

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
        system="Ihre Aufgabe ist es, eine personalisierte Motivationsnachricht oder Affirmation basierend auf der Eingabe des Benutzers zu generieren. Gehen Sie auf ihre spezifischen Bedürfnisse ein und bieten Sie Ermutigung, Unterstützung und Anleitung. Verwenden Sie einen positiven, empathischen und inspirierenden Ton, um dem Benutzer zu helfen, sich motiviert und gestärkt zu fühlen. Verwenden Sie relevante Beispiele, Analogien oder Zitate, um Ihre Nachricht zu verstärken und sie wirkungsvoller zu machen. Stellen Sie sicher, dass die Nachricht prägnant, authentisch und leicht verständlich ist.",
        messages=[
            {
                "role": "user",
                "content": [
                    {
                        "type": "text",
                        "text": "Ich habe Schwierigkeiten, Motivation zu finden, um an meinem Roman zu arbeiten. Ich habe prokrastiniert und fühle mich festgefahren, obwohl Schreiben meine Leidenschaft ist. Ich habe Angst, dass ich ihn vielleicht nie beenden werde."
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
      system: "Ihre Aufgabe ist es, eine personalisierte Motivationsnachricht oder Affirmation basierend auf der Eingabe des Benutzers zu generieren. Gehen Sie auf ihre spezifischen Bedürfnisse ein und bieten Sie Ermutigung, Unterstützung und Anleitung. Verwenden Sie einen positiven, empathischen und inspirierenden Ton, um dem Benutzer zu helfen, sich motiviert und gestärkt zu fühlen. Verwenden Sie relevante Beispiele, Analogien oder Zitate, um Ihre Nachricht zu verstärken und sie wirkungsvoller zu machen. Stellen Sie sicher, dass die Nachricht prägnant, authentisch und leicht verständlich ist.",
      messages: [
        {
          "role": "user",
          "content": [
            {
              "type": "text",
              "text": "Ich habe Schwierigkeiten, Motivation zu finden, um an meinem Roman zu arbeiten. Ich habe prokrastiniert und fühle mich festgefahren, obwohl Schreiben meine Leidenschaft ist. Ich habe Angst, dass ich ihn vielleicht nie beenden werde."
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
        system="Ihre Aufgabe ist es, eine personalisierte Motivationsnachricht oder Affirmation basierend auf der Eingabe des Benutzers zu generieren. Gehen Sie auf ihre spezifischen Bedürfnisse ein und bieten Sie Ermutigung, Unterstützung und Anleitung. Verwenden Sie einen positiven, empathischen und inspirierenden Ton, um dem Benutzer zu helfen, sich motiviert und gestärkt zu fühlen. Verwenden Sie relevante Beispiele, Analogien oder Zitate, um Ihre Nachricht zu verstärken und sie wirkungsvoller zu machen. Stellen Sie sicher, dass die Nachricht prägnant, authentisch und leicht verständlich ist.",
        messages=[
            {
                "role": "user",
                "content": [
                    {
                        "type": "text",
                        "text": "Ich habe Schwierigkeiten, Motivation zu finden, um an meinem Roman zu arbeiten. Ich habe prokrastiniert und fühle mich festgefahren, obwohl Schreiben meine Leidenschaft ist. Ich habe Angst, dass ich ihn vielleicht nie beenden werde."
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
      system: "Ihre Aufgabe ist es, eine personalisierte Motivationsnachricht oder Affirmation basierend auf der Eingabe des Benutzers zu generieren. Gehen Sie auf ihre spezifischen Bedürfnisse ein und bieten Sie Ermutigung, Unterstützung und Anleitung. Verwenden Sie einen positiven, empathischen und inspirierenden Ton, um dem Benutzer zu helfen, sich motiviert und gestärkt zu fühlen. Verwenden Sie relevante Beispiele, Analogien oder Zitate, um Ihre Nachricht zu verstärken und sie wirkungsvoller zu machen. Stellen Sie sicher, dass die Nachricht prägnant, authentisch und leicht verständlich ist.",
      messages: [
        {
          "role": "user",
          "content": [
            {
              "type": "text",
              "text": "Ich habe Schwierigkeiten, Motivation zu finden, um an meinem Roman zu arbeiten. Ich habe prokrastiniert und fühle mich festgefahren, obwohl Schreiben meine Leidenschaft ist. Ich habe Angst, dass ich ihn vielleicht nie beenden werde."
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
        system="Ihre Aufgabe ist es, eine personalisierte Motivationsnachricht oder Affirmation basierend auf der Eingabe des Benutzers zu generieren. Gehen Sie auf ihre spezifischen Bedürfnisse ein und bieten Sie Ermutigung, Unterstützung und Anleitung. Verwenden Sie einen positiven, empathischen und inspirierenden Ton, um dem Benutzer zu helfen, sich motiviert und gestärkt zu fühlen. Verwenden Sie relevante Beispiele, Analogien oder Zitate, um Ihre Nachricht zu verstärken und sie wirkungsvoller zu machen. Stellen Sie sicher, dass die Nachricht prägnant, authentisch und leicht verständlich ist.",
        messages=[
            {
                "role": "user",
                "content": [
                    {
                        "type": "text",
                        "text": "Ich habe Schwierigkeiten, Motivation zu finden, um an meinem Roman zu arbeiten. Ich habe prokrastiniert und fühle mich festgefahren, obwohl Schreiben meine Leidenschaft ist. Ich habe Angst, dass ich ihn vielleicht nie beenden werde."
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
      system: "Ihre Aufgabe ist es, eine personalisierte Motivationsnachricht oder Affirmation basierend auf der Eingabe des Benutzers zu generieren. Gehen Sie auf ihre spezifischen Bedürfnisse ein und bieten Sie Ermutigung, Unterstützung und Anleitung. Verwenden Sie einen positiven, empathischen und inspirierenden Ton, um dem Benutzer zu helfen, sich motiviert und gestärkt zu fühlen. Verwenden Sie relevante Beispiele, Analogien oder Zitate, um Ihre Nachricht zu verstärken und sie wirkungsvoller zu machen. Stellen Sie sicher, dass die Nachricht prägnant, authentisch und leicht verständlich ist.",
      messages: [
        {
          "role": "user",
          "content": [
            {
              "type": "text",
              "text": "Ich habe Schwierigkeiten, Motivation zu finden, um an meinem Roman zu arbeiten. Ich habe prokrastiniert und fühle mich festgefahren, obwohl Schreiben meine Leidenschaft ist. Ich habe Angst, dass ich ihn vielleicht nie beenden werde."
            }
          ]
        }
      ]
    });
    console.log(msg);
    
    ```
    
</CodeGroup>