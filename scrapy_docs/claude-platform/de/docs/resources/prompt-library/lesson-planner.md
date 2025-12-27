# Unterrichtsplaner

Erstelle detaillierte Unterrichtspläne zu jedem Thema.

---

> Kopiere diesen Prompt in unsere Entwickler-[Konsole](/login?selectAccount=true&returnTo=%2Fdashboard%3F), um es selbst auszuprobieren!

|        | Inhalt |
| --- | --- |
| System | Deine Aufgabe ist es, einen umfassenden, ansprechenden und gut strukturierten Unterrichtsplan zu dem gegebenen Thema zu erstellen. Der Unterrichtsplan sollte für eine 60-minütige Unterrichtsstunde konzipiert sein und auf eine bestimmte Klassenstufe oder Altersgruppe zugeschnitten werden. Beginne mit der Angabe der Lernziele, die klar, messbar und an relevanten Bildungsstandards ausgerichtet sein sollten. Gib als nächstes eine detaillierte Gliederung der Unterrichtsstunde an, unterteilt in eine Einführung, Hauptaktivitäten und einen Abschluss. Beschreibe für jeden Abschnitt die Lehrmethoden, Lernaktivitäten und Ressourcen, die du verwenden wirst, um den Inhalt effektiv zu vermitteln und die Schüler zu engagieren. Beschreibe schließlich die Bewertungsmethoden, die du einsetzen wirst, um das Verständnis und die Beherrschung der Lernziele durch die Schüler zu bewerten. Der Unterrichtsplan sollte gut organisiert, leicht zu befolgen sein und aktives Lernen und kritisches Denken fördern. |
| User   | Thema: Einführung in die Photosynthese Klassenstufe: 7. Klasse (Alter 12-13) |

## Beispielausgabe

> Unterrichtsplan: Einführung in die Photosynthese
> Klassenstufe: 7. Klasse (Alter 12-13)
> Dauer: 60 Minuten
>
> Lernziele:
> Am Ende der Unterrichtsstunde werden die Schüler in der Lage sein:
>
> 1. Photosynthese zu definieren und ihre Bedeutung im Ökosystem zu erklären.
> 2. Die wichtigsten Komponenten zu identifizieren, die für das Auftreten der Photosynthese notwendig sind.
> 3. Den Prozess der Photosynthese zu beschreiben, einschließlich der Rollen von Lichtenergie, Kohlendioxid und Wasser.
> 4. Die Produkte der Photosynthese und ihre Bedeutung für lebende Organismen zu erkennen.
>    Unterrichtsgliederung:
>
> 5. Einführung (10 Minuten)
>    i. Schüler durch das Zeigen eines kurzen Videoclips oder von Bildern von Pflanzen engagieren und ihre Rolle in der Umwelt diskutieren.
>    ii. Das Konzept der Photosynthese und ihre Bedeutung für die Aufrechterhaltung des Lebens auf der Erde einführen.
>    iii. Die Lernziele angeben und einen Überblick über die Unterrichtsstruktur geben.
> 6. Hauptaktivitäten (40 Minuten)
>    i. Angeleiteter Unterricht (15 Minuten)
>    a. Die wichtigsten Komponenten erklären, die für die Photosynthese notwendig sind: Sonnenlicht, Kohlendioxid, Wasser und Chlorophyll.
>    b. Den Prozess der Photosynthese beschreiben und dabei die Rollen von Lichtenergie, Kohlendioxid und Wasser betonen.
>    c. Die Produkte der Photosynthese (Glukose und Sauerstoff) und ihre Bedeutung für lebende Organismen diskutieren.
>    ii. Kleingruppenaktivität (15 Minuten)
>    a. Die Klasse in kleine Gruppen aufteilen und jeder Gruppe ein Arbeitsblatt oder einen grafischen Organizer zur Verfügung stellen.
>    b. Die Schüler bitten, zusammenzuarbeiten, um eine visuelle Darstellung des Photosyntheseprozesses zu erstellen und dabei die wichtigsten Komponenten und Produkte zu beschriften.
>    c. Die Schüler ermutigen, ihr Verständnis der Photosynthese innerhalb ihrer Gruppen zu diskutieren und zu erklären.
>    iii. Klassendiskussion (10 Minuten)
>    a. Jede Gruppe einladen, ihre visuelle Darstellung mit der Klasse zu teilen.
>    b. Eine Klassendiskussion moderieren und die Schüler ermutigen, Fragen zu stellen, Feedback zu geben und eventuelle Missverständnisse zu klären.
> 7. Abschluss (10 Minuten)
>    i. Die wichtigsten Punkte der Unterrichtsstunde zusammenfassen und dabei die Definition, Komponenten, den Prozess und die Produkte der Photosynthese verstärken.
>    ii. Ein kurzes Quiz oder Exit-Ticket durchführen, um das Verständnis der Schüler bezüglich der Lernziele zu bewerten.
>    iii. Den Schülern eine Hausaufgabe geben, um die Bedeutung der Photosynthese in verschiedenen Ökosystemen weiter zu erforschen.
>    Bewertungsmethoden:
>
> 8. Formative Bewertung: Die Teilnahme und das Verständnis der Schüler während des angeleiteten Unterrichts und der Kleingruppenaktivität beobachten.
> 9. Summative Bewertung: Die visuellen Darstellungen der Schüler und ihre Leistung beim kurzen Quiz oder Exit-Ticket bewerten.
> 10. Hausaufgabe: Die Hausaufgabenabgaben der Schüler überprüfen, um ihre Fähigkeit zu beurteilen, die in der Unterrichtsstunde gelernten Konzepte anzuwenden.
>     Ressourcen:
>
> 11. Videoclip oder Bilder von Pflanzen und ihrer Rolle in der Umwelt
> 12. Arbeitsblätter oder grafische Organizer für die Kleingruppenaktivität
> 13. Kurzes Quiz oder Exit-Ticket für die Bewertung
> 14. Anweisungen für die Hausaufgabe

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
        max_tokens=4000,
        temperature=0.5,
        system="Deine Aufgabe ist es, einen umfassenden, ansprechenden und gut strukturierten Unterrichtsplan zu dem gegebenen Thema zu erstellen. Der Unterrichtsplan sollte für eine 60-minütige Unterrichtsstunde konzipiert sein und auf eine bestimmte Klassenstufe oder Altersgruppe zugeschnitten werden. Beginne mit der Angabe der Lernziele, die klar, messbar und an relevanten Bildungsstandards ausgerichtet sein sollten. Gib als nächstes eine detaillierte Gliederung der Unterrichtsstunde an, unterteilt in eine Einführung, Hauptaktivitäten und einen Abschluss. Beschreibe für jeden Abschnitt die Lehrmethoden, Lernaktivitäten und Ressourcen, die du verwenden wirst, um den Inhalt effektiv zu vermitteln und die Schüler zu engagieren. Beschreibe schließlich die Bewertungsmethoden, die du einsetzen wirst, um das Verständnis und die Beherrschung der Lernziele durch die Schüler zu bewerten. Der Unterrichtsplan sollte gut organisiert, leicht zu befolgen sein und aktives Lernen und kritisches Denken fördern.",
        messages=[
            {
                "role": "user",
                "content": [
                    {
                        "type": "text",
                        "text": "Thema: Einführung in die Photosynthese  \nKlassenstufe: 7. Klasse (Alter 12-13)"
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
      system: "Deine Aufgabe ist es, einen umfassenden, ansprechenden und gut strukturierten Unterrichtsplan zu dem gegebenen Thema zu erstellen. Der Unterrichtsplan sollte für eine 60-minütige Unterrichtsstunde konzipiert sein und auf eine bestimmte Klassenstufe oder Altersgruppe zugeschnitten werden. Beginne mit der Angabe der Lernziele, die klar, messbar und an relevanten Bildungsstandards ausgerichtet sein sollten. Gib als nächstes eine detaillierte Gliederung der Unterrichtsstunde an, unterteilt in eine Einführung, Hauptaktivitäten und einen Abschluss. Beschreibe für jeden Abschnitt die Lehrmethoden, Lernaktivitäten und Ressourcen, die du verwenden wirst, um den Inhalt effektiv zu vermitteln und die Schüler zu engagieren. Beschreibe schließlich die Bewertungsmethoden, die du einsetzen wirst, um das Verständnis und die Beherrschung der Lernziele durch die Schüler zu bewerten. Der Unterrichtsplan sollte gut organisiert, leicht zu befolgen sein und aktives Lernen und kritisches Denken fördern.",
      messages: [
        {
          "role": "user",
          "content": [
            {
              "type": "text",
              "text": "Thema: Einführung in die Photosynthese  \nKlassenstufe: 7. Klasse (Alter 12-13)"
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
        system="Deine Aufgabe ist es, einen umfassenden, ansprechenden und gut strukturierten Unterrichtsplan zu dem gegebenen Thema zu erstellen. Der Unterrichtsplan sollte für eine 60-minütige Unterrichtsstunde konzipiert sein und auf eine bestimmte Klassenstufe oder Altersgruppe zugeschnitten werden. Beginne mit der Angabe der Lernziele, die klar, messbar und an relevanten Bildungsstandards ausgerichtet sein sollten. Gib als nächstes eine detaillierte Gliederung der Unterrichtsstunde an, unterteilt in eine Einführung, Hauptaktivitäten und einen Abschluss. Beschreibe für jeden Abschnitt die Lehrmethoden, Lernaktivitäten und Ressourcen, die du verwenden wirst, um den Inhalt effektiv zu vermitteln und die Schüler zu engagieren. Beschreibe schließlich die Bewertungsmethoden, die du einsetzen wirst, um das Verständnis und die Beherrschung der Lernziele durch die Schüler zu bewerten. Der Unterrichtsplan sollte gut organisiert, leicht zu befolgen sein und aktives Lernen und kritisches Denken fördern.",
        messages=[
            {
                "role": "user",
                "content": [
                    {
                        "type": "text",
                        "text": "Thema: Einführung in die Photosynthese  \nKlassenstufe: 7. Klasse (Alter 12-13)"
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
      system: "Deine Aufgabe ist es, einen umfassenden, ansprechenden und gut strukturierten Unterrichtsplan zu dem gegebenen Thema zu erstellen. Der Unterrichtsplan sollte für eine 60-minütige Unterrichtsstunde konzipiert sein und auf eine bestimmte Klassenstufe oder Altersgruppe zugeschnitten werden. Beginne mit der Angabe der Lernziele, die klar, messbar und an relevanten Bildungsstandards ausgerichtet sein sollten. Gib als nächstes eine detaillierte Gliederung der Unterrichtsstunde an, unterteilt in eine Einführung, Hauptaktivitäten und einen Abschluss. Beschreibe für jeden Abschnitt die Lehrmethoden, Lernaktivitäten und Ressourcen, die du verwenden wirst, um den Inhalt effektiv zu vermitteln und die Schüler zu engagieren. Beschreibe schließlich die Bewertungsmethoden, die du einsetzen wirst, um das Verständnis und die Beherrschung der Lernziele durch die Schüler zu bewerten. Der Unterrichtsplan sollte gut organisiert, leicht zu befolgen sein und aktives Lernen und kritisches Denken fördern.",
      messages: [
        {
          "role": "user",
          "content": [
            {
              "type": "text",
              "text": "Thema: Einführung in die Photosynthese  \nKlassenstufe: 7. Klasse (Alter 12-13)"
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
        system="Deine Aufgabe ist es, einen umfassenden, ansprechenden und gut strukturierten Unterrichtsplan zu dem gegebenen Thema zu erstellen. Der Unterrichtsplan sollte für eine 60-minütige Unterrichtsstunde konzipiert sein und auf eine bestimmte Klassenstufe oder Altersgruppe zugeschnitten werden. Beginne mit der Angabe der Lernziele, die klar, messbar und an relevanten Bildungsstandards ausgerichtet sein sollten. Gib als nächstes eine detaillierte Gliederung der Unterrichtsstunde an, unterteilt in eine Einführung, Hauptaktivitäten und einen Abschluss. Beschreibe für jeden Abschnitt die Lehrmethoden, Lernaktivitäten und Ressourcen, die du verwenden wirst, um den Inhalt effektiv zu vermitteln und die Schüler zu engagieren. Beschreibe schließlich die Bewertungsmethoden, die du einsetzen wirst, um das Verständnis und die Beherrschung der Lernziele durch die Schüler zu bewerten. Der Unterrichtsplan sollte gut organisiert, leicht zu befolgen sein und aktives Lernen und kritisches Denken fördern.",
        messages=[
            {
                "role": "user",
                "content": [
                    {
                        "type": "text",
                        "text": "Thema: Einführung in die Photosynthese  \nKlassenstufe: 7. Klasse (Alter 12-13)"
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
      system: "Deine Aufgabe ist es, einen umfassenden, ansprechenden und gut strukturierten Unterrichtsplan zu dem gegebenen Thema zu erstellen. Der Unterrichtsplan sollte für eine 60-minütige Unterrichtsstunde konzipiert sein und auf eine bestimmte Klassenstufe oder Altersgruppe zugeschnitten werden. Beginne mit der Angabe der Lernziele, die klar, messbar und an relevanten Bildungsstandards ausgerichtet sein sollten. Gib als nächstes eine detaillierte Gliederung der Unterrichtsstunde an, unterteilt in eine Einführung, Hauptaktivitäten und einen Abschluss. Beschreibe für jeden Abschnitt die Lehrmethoden, Lernaktivitäten und Ressourcen, die du verwenden wirst, um den Inhalt effektiv zu vermitteln und die Schüler zu engagieren. Beschreibe schließlich die Bewertungsmethoden, die du einsetzen wirst, um das Verständnis und die Beherrschung der Lernziele durch die Schüler zu bewerten. Der Unterrichtsplan sollte gut organisiert, leicht zu befolgen sein und aktives Lernen und kritisches Denken fördern.",
      messages: [
        {
          "role": "user",
          "content": [
            {
              "type": "text",
              "text": "Thema: Einführung in die Photosynthese  \nKlassenstufe: 7. Klasse (Alter 12-13)"
            }
          ]
        }
      ]
    });
    console.log(msg);
    
    ```
</CodeGroup>