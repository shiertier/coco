# Alliterations-Alchemist

Generiere alliterative Phrasen und Sätze für jedes gegebene Thema.

---

> Kopiere diesen Prompt in unsere Entwickler-[Konsole](/dashboard), um es selbst auszuprobieren!

|        | Inhalt |
| --- | --- |
| System | Deine Aufgabe ist es, alliterative Phrasen und Sätze für das gegebene Thema zu erstellen. Stelle sicher, dass die Alliterationen nicht nur wohlklingend sind, sondern auch relevante Informationen vermitteln oder angemessene Emotionen im Zusammenhang mit dem Thema hervorrufen. |
| User   | Ozean |

### Beispielausgabe

> 1.  Flüsternde Wellen waschen über das Ufer.
> 2.  Salzige Seegischt spritzt auf den Sand.
> 3.  Krachende Strömungen schnitzen Küstenklippen.
> 4.  Tiefe, dunkle Tiefen tarnen versteckte Gefahren.
> 5.  Blubbernde Bäche bringen Leben an den Strand.
> 6.  Mächtige, majestätische Meeressäuger migrieren durch die Gewässer.
> 7.  Donnernde Gezeiten drehen Schiffe hin und her.
> 8.  Schimmernde Schwärme schützen scheue Seetiere.
> 9.  Kräuselnde Reflexionen enthüllen eine Welt unter der Oberfläche.
> 10. Wirbelnde, wogende Seen formen die Küstenlinie.
> 11. Schäumende, sprudelnde Schollen strömen frei.
> 12. Graziöse Möwen gleiten über den glitzernden Ozean.
> 13. Verspielte Schweinswale planschen durch die pristinen Gewässer.
> 14. Salzige Brisen blasen über das grenzenlose Blau.
> 15. Rhythmische Wellen rollen zur felsigen Küste.
> 16. Korallengrotten verbergen farbenfrohe aquatische Gemeinschaften.
> 17. Stürmische Himmel und schwellende Seen signalisieren Naturgewalt.
> 18. Taumelnde Gezeiten transformieren die ruhige Bucht.
> 19. Wirbelnde, sausende Winde peitschen die Wellen auf.
> 20. Ruhige, saphirblaue Gewässer strecken sich zum Horizont.

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
        max_tokens=1000,
        temperature=1,
        system="Deine Aufgabe ist es, alliterative Phrasen und Sätze für das gegebene Thema zu erstellen. Stelle sicher, dass die Alliterationen nicht nur wohlklingend sind, sondern auch relevante Informationen vermitteln oder angemessene Emotionen im Zusammenhang mit dem Thema hervorrufen.",
        messages=[
            {
                "role": "user",
                "content": [
                    {
                        "type": "text",
                        "text": "Ozean"
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
      temperature: 1,
      system: "Deine Aufgabe ist es, alliterative Phrasen und Sätze für das gegebene Thema zu erstellen. Stelle sicher, dass die Alliterationen nicht nur wohlklingend sind, sondern auch relevante Informationen vermitteln oder angemessene Emotionen im Zusammenhang mit dem Thema hervorrufen.",
      messages: [
        {
          "role": "user",
          "content": [
            {
              "type": "text",
              "text": "Ozean"
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
        temperature=1,
        system="Deine Aufgabe ist es, alliterative Phrasen und Sätze für das gegebene Thema zu erstellen. Stelle sicher, dass die Alliterationen nicht nur wohlklingend sind, sondern auch relevante Informationen vermitteln oder angemessene Emotionen im Zusammenhang mit dem Thema hervorrufen.",
        messages=[
            {
                "role": "user",
                "content": [
                    {
                        "type": "text",
                        "text": "Ozean"
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
      temperature: 1,
      system: "Deine Aufgabe ist es, alliterative Phrasen und Sätze für das gegebene Thema zu erstellen. Stelle sicher, dass die Alliterationen nicht nur wohlklingend sind, sondern auch relevante Informationen vermitteln oder angemessene Emotionen im Zusammenhang mit dem Thema hervorrufen.",
      messages: [
        {
          "role": "user",
          "content": [
            {
              "type": "text",
              "text": "Ozean"
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
        temperature=1,
        system="Deine Aufgabe ist es, alliterative Phrasen und Sätze für das gegebene Thema zu erstellen. Stelle sicher, dass die Alliterationen nicht nur wohlklingend sind, sondern auch relevante Informationen vermitteln oder angemessene Emotionen im Zusammenhang mit dem Thema hervorrufen.",
        messages=[
            {
                "role": "user",
                "content": [
                    {
                        "type": "text",
                        "text": "Ozean"
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
      temperature: 1,
      system: "Deine Aufgabe ist es, alliterative Phrasen und Sätze für das gegebene Thema zu erstellen. Stelle sicher, dass die Alliterationen nicht nur wohlklingend sind, sondern auch relevante Informationen vermitteln oder angemessene Emotionen im Zusammenhang mit dem Thema hervorrufen.",
      messages: [
        {
          "role": "user",
          "content": [
            {
              "type": "text",
              "text": "Ozean"
            }
          ]
        }
      ]
    });
    console.log(msg);
    
    ```
</CodeGroup>