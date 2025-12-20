# Anweisungsdekoder

Verwandeln Sie natürliche Sprache in schrittweise Anweisungen.

---

> Kopieren Sie diesen Prompt in unsere Entwickler-[Konsole](/login?selectAccount=true&returnTo=%2Fdashboard%3F), um es selbst auszuprobieren!

|        | Inhalt |
| --- | --- |
| System | Ihre Aufgabe ist es, die bereitgestellte natürlichsprachliche Beschreibung eines Prozesses oder einer Aufgabe zu nehmen und sie in klare, prägnante schrittweise Anweisungen zu verwandeln, die logisch, sequenziell und leicht zu befolgen sind. Verwenden Sie imperative Sprache und beginnen Sie jeden Schritt mit einem Aktionsverb. Geben Sie notwendige Details und Erklärungen an, um sicherzustellen, dass der Leser die Aufgabe erfolgreich abschließen kann. Wenn die ursprüngliche Beschreibung unklar, mehrdeutig oder unzureichend informativ ist, bitten Sie um Klarstellung oder zusätzliche Details. |
| User   | Um eine Tasse Tee zu machen, beginnen Sie damit, etwas Wasser in einem Wasserkocher zu kochen. Während das Wasser sich erhitzt, holen Sie eine Tasse und legen Sie einen Teebeutel hinein. Sobald das Wasser kocht, gießen Sie es vorsichtig in die Tasse, über den Teebeutel. Lassen Sie den Tee einige Minuten ziehen, je nachdem, wie stark Sie ihn mögen. Nach dem Ziehen entfernen Sie den Teebeutel und fügen Milch und Zucker nach Geschmack hinzu. Rühren Sie alles zusammen und genießen Sie Ihre heiße Tasse Tee. |

### Beispielausgabe

> 1. Füllen Sie einen Wasserkocher mit Wasser und stellen Sie ihn auf den Herd zum Kochen.
> 2. Während Sie auf das kochende Wasser warten, wählen Sie eine Tasse aus und legen Sie einen Teebeutel hinein.
> 3. Sobald das Wasser kocht, gießen Sie das heiße Wasser vorsichtig in die Tasse, über den Teebeutel.
> 4. Lassen Sie den Tee für die gewünschte Zeit im heißen Wasser ziehen, typischerweise 2-5 Minuten, je nach bevorzugter Stärke.
> 5. Nach Ablauf der Ziehzeit entfernen Sie den Teebeutel aus der Tasse und entsorgen Sie ihn.
> 6. Fügen Sie Milch und Zucker zum Tee hinzu, entsprechend Ihren Geschmacksvorlieben.
> 7. Verwenden Sie einen Löffel, um den Tee, die Milch und den Zucker zusammen zu rühren, bis alles gut vermischt ist.
> 8. Ihre Tasse Tee ist nun bereit zum Genießen. Trinken Sie sie, solange sie noch heiß ist.

---

## API-Anfrage

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
        system="Ihre Aufgabe ist es, die bereitgestellte natürlichsprachliche Beschreibung eines Prozesses oder einer Aufgabe zu nehmen und sie in klare, prägnante schrittweise Anweisungen zu verwandeln, die logisch, sequenziell und leicht zu befolgen sind. Verwenden Sie imperative Sprache und beginnen Sie jeden Schritt mit einem Aktionsverb. Geben Sie notwendige Details und Erklärungen an, um sicherzustellen, dass der Leser die Aufgabe erfolgreich abschließen kann. Wenn die ursprüngliche Beschreibung unklar, mehrdeutig oder unzureichend informativ ist, bitten Sie um Klarstellung oder zusätzliche Details.",
        messages=[
            {
                "role": "user",
                "content": [
                    {
                        "type": "text",
                        "text": "Um eine Tasse Tee zu machen, beginnen Sie damit, etwas Wasser in einem Wasserkocher zu kochen. Während das Wasser sich erhitzt, holen Sie eine Tasse und legen Sie einen Teebeutel hinein. Sobald das Wasser kocht, gießen Sie es vorsichtig in die Tasse, über den Teebeutel. Lassen Sie den Tee einige Minuten ziehen, je nachdem, wie stark Sie ihn mögen. Nach dem Ziehen entfernen Sie den Teebeutel und fügen Milch und Zucker nach Geschmack hinzu. Rühren Sie alles zusammen und genießen Sie Ihre heiße Tasse Tee."
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
      system: "Ihre Aufgabe ist es, die bereitgestellte natürlichsprachliche Beschreibung eines Prozesses oder einer Aufgabe zu nehmen und sie in klare, prägnante schrittweise Anweisungen zu verwandeln, die logisch, sequenziell und leicht zu befolgen sind. Verwenden Sie imperative Sprache und beginnen Sie jeden Schritt mit einem Aktionsverb. Geben Sie notwendige Details und Erklärungen an, um sicherzustellen, dass der Leser die Aufgabe erfolgreich abschließen kann. Wenn die ursprüngliche Beschreibung unklar, mehrdeutig oder unzureichend informativ ist, bitten Sie um Klarstellung oder zusätzliche Details.",
      messages: [
        {
          "role": "user",
          "content": [
            {
              "type": "text",
              "text": "Um eine Tasse Tee zu machen, beginnen Sie damit, etwas Wasser in einem Wasserkocher zu kochen. Während das Wasser sich erhitzt, holen Sie eine Tasse und legen Sie einen Teebeutel hinein. Sobald das Wasser kocht, gießen Sie es vorsichtig in die Tasse, über den Teebeutel. Lassen Sie den Tee einige Minuten ziehen, je nachdem, wie stark Sie ihn mögen. Nach dem Ziehen entfernen Sie den Teebeutel und fügen Milch und Zucker nach Geschmack hinzu. Rühren Sie alles zusammen und genießen Sie Ihre heiße Tasse Tee."
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
        system="Ihre Aufgabe ist es, die bereitgestellte natürlichsprachliche Beschreibung eines Prozesses oder einer Aufgabe zu nehmen und sie in klare, prägnante schrittweise Anweisungen zu verwandeln, die logisch, sequenziell und leicht zu befolgen sind. Verwenden Sie imperative Sprache und beginnen Sie jeden Schritt mit einem Aktionsverb. Geben Sie notwendige Details und Erklärungen an, um sicherzustellen, dass der Leser die Aufgabe erfolgreich abschließen kann. Wenn die ursprüngliche Beschreibung unklar, mehrdeutig oder unzureichend informativ ist, bitten Sie um Klarstellung oder zusätzliche Details.",
        messages=[
            {
                "role": "user",
                "content": [
                    {
                        "type": "text",
                        "text": "Um eine Tasse Tee zu machen, beginnen Sie damit, etwas Wasser in einem Wasserkocher zu kochen. Während das Wasser sich erhitzt, holen Sie eine Tasse und legen Sie einen Teebeutel hinein. Sobald das Wasser kocht, gießen Sie es vorsichtig in die Tasse, über den Teebeutel. Lassen Sie den Tee einige Minuten ziehen, je nachdem, wie stark Sie ihn mögen. Nach dem Ziehen entfernen Sie den Teebeutel und fügen Milch und Zucker nach Geschmack hinzu. Rühren Sie alles zusammen und genießen Sie Ihre heiße Tasse Tee."
                    }
                ]
            }
        ]
    )
    print(message.content)
    
    ```
    
    
    ```typescript AWS Bedrock TypeScript
    import AnthropicBedrock from "@anthropic-ai/bedrock-sdk";
    
    # See https://docs.claude.com/claude/reference/claude-on-amazon-bedrock
    # for authentication options
    const client = new AnthropicBedrock();
    
    const msg = await client.messages.create({
      model: "anthropic.claude-sonnet-4-5-20250929-v1:0",
      max_tokens: 1000,
      temperature: 0,
      system: "Ihre Aufgabe ist es, die bereitgestellte natürlichsprachliche Beschreibung eines Prozesses oder einer Aufgabe zu nehmen und sie in klare, prägnante schrittweise Anweisungen zu verwandeln, die logisch, sequenziell und leicht zu befolgen sind. Verwenden Sie imperative Sprache und beginnen Sie jeden Schritt mit einem Aktionsverb. Geben Sie notwendige Details und Erklärungen an, um sicherzustellen, dass der Leser die Aufgabe erfolgreich abschließen kann. Wenn die ursprüngliche Beschreibung unklar, mehrdeutig oder unzureichend informativ ist, bitten Sie um Klarstellung oder zusätzliche Details.",
      messages: [
        {
          "role": "user",
          "content": [
            {
              "type": "text",
              "text": "Um eine Tasse Tee zu machen, beginnen Sie damit, etwas Wasser in einem Wasserkocher zu kochen. Während das Wasser sich erhitzt, holen Sie eine Tasse und legen Sie einen Teebeutel hinein. Sobald das Wasser kocht, gießen Sie es vorsichtig in die Tasse, über den Teebeutel. Lassen Sie den Tee einige Minuten ziehen, je nachdem, wie stark Sie ihn mögen. Nach dem Ziehen entfernen Sie den Teebeutel und fügen Milch und Zucker nach Geschmack hinzu. Rühren Sie alles zusammen und genießen Sie Ihre heiße Tasse Tee."
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
        system="Ihre Aufgabe ist es, die bereitgestellte natürlichsprachliche Beschreibung eines Prozesses oder einer Aufgabe zu nehmen und sie in klare, prägnante schrittweise Anweisungen zu verwandeln, die logisch, sequenziell und leicht zu befolgen sind. Verwenden Sie imperative Sprache und beginnen Sie jeden Schritt mit einem Aktionsverb. Geben Sie notwendige Details und Erklärungen an, um sicherzustellen, dass der Leser die Aufgabe erfolgreich abschließen kann. Wenn die ursprüngliche Beschreibung unklar, mehrdeutig oder unzureichend informativ ist, bitten Sie um Klarstellung oder zusätzliche Details.",
        messages=[
            {
                "role": "user",
                "content": [
                    {
                        "type": "text",
                        "text": "Um eine Tasse Tee zu machen, beginnen Sie damit, etwas Wasser in einem Wasserkocher zu kochen. Während das Wasser sich erhitzt, holen Sie eine Tasse und legen Sie einen Teebeutel hinein. Sobald das Wasser kocht, gießen Sie es vorsichtig in die Tasse, über den Teebeutel. Lassen Sie den Tee einige Minuten ziehen, je nachdem, wie stark Sie ihn mögen. Nach dem Ziehen entfernen Sie den Teebeutel und fügen Milch und Zucker nach Geschmack hinzu. Rühren Sie alles zusammen und genießen Sie Ihre heiße Tasse Tee."
                    }
                ]
            }
        ]
    )
    print(message.content)
    
    ```
    
    
    ```typescript Vertex AI TypeScript
    import { AnthropicVertex } from '@anthropic-ai/vertex-sdk';
    
    # Reads from the `CLOUD_ML_REGION` & `ANTHROPIC_VERTEX_PROJECT_ID` environment variables.
    # Additionally goes through the standard `google-auth-library` flow.
    const client = new AnthropicVertex();
    
    const msg = await client.messages.create({
      model: "claude-sonnet-4@20250514",
      max_tokens: 1000,
      temperature: 0,
      system: "Ihre Aufgabe ist es, die bereitgestellte natürlichsprachliche Beschreibung eines Prozesses oder einer Aufgabe zu nehmen und sie in klare, prägnante schrittweise Anweisungen zu verwandeln, die logisch, sequenziell und leicht zu befolgen sind. Verwenden Sie imperative Sprache und beginnen Sie jeden Schritt mit einem Aktionsverb. Geben Sie notwendige Details und Erklärungen an, um sicherzustellen, dass der Leser die Aufgabe erfolgreich abschließen kann. Wenn die ursprüngliche Beschreibung unklar, mehrdeutig oder unzureichend informativ ist, bitten Sie um Klarstellung oder zusätzliche Details.",
      messages: [
        {
          "role": "user",
          "content": [
            {
              "type": "text",
              "text": "Um eine Tasse Tee zu machen, beginnen Sie damit, etwas Wasser in einem Wasserkocher zu kochen. Während das Wasser sich erhitzt, holen Sie eine Tasse und legen Sie einen Teebeutel hinein. Sobald das Wasser kocht, gießen Sie es vorsichtig in die Tasse, über den Teebeutel. Lassen Sie den Tee einige Minuten ziehen, je nachdem, wie stark Sie ihn mögen. Nach dem Ziehen entfernen Sie den Teebeutel und fügen Milch und Zucker nach Geschmack hinzu. Rühren Sie alles zusammen und genießen Sie Ihre heiße Tasse Tee."
            }
          ]
        }
      ]
    });
    console.log(msg);
    
    ```
</CodeGroup>