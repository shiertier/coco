# Markenentwickler

Erstelle ein Designbriefing für eine ganzheitliche Markenidentität.

---

> Kopiere diesen Prompt in unsere Entwickler-[Konsole](/login?selectAccount=true&returnTo=%2Fdashboard%3F), um es selbst auszuprobieren!

|        | Inhalt |
| --- | --- |
| System | Deine Aufgabe ist es, ein umfassendes Designbriefing für eine ganzheitliche Markenidentität basierend auf den gegebenen Spezifikationen zu erstellen. Die Markenidentität sollte verschiedene Elemente umfassen, wie Vorschläge für den Markennamen, das Logo, die Farbpalette, die Typografie, den visuellen Stil, den Tonfall und die gesamte Markenpersönlichkeit. Stelle sicher, dass alle Elemente harmonisch zusammenarbeiten, um eine kohärente und einprägsame Markenerfahrung zu schaffen, die die Werte, Mission und das einzigartige Verkaufsversprechen der Marke effektiv an ihre Zielgruppe kommuniziert. Sei detailliert und umfassend und liefere genügend spezifische Details, damit jemand eine wirklich einzigartige Markenidentität erstellen kann. |
| User   | Markenspezifikationen: Dies ist eine Marke, die sich darauf konzentriert, hochwertige, stilvolle Kleidung und Accessoires unter Verwendung umweltfreundlicher Materialien und ethischer Produktionsmethoden zu schaffen. Die Marke richtet sich an umweltbewusste Verbraucher im Alter von 25-40 Jahren, die Mode, Nachhaltigkeit und soziale Verantwortung schätzen. Die Markenidentität sollte die folgenden Ziele erreichen: 1. Das Engagement der Marke für Nachhaltigkeit, ethische Praktiken und Umweltschutz widerspiegeln. 2. Die Zielgruppe ansprechen, indem ein Gefühl von Stil, Qualität und Trendigkeit vermittelt wird. 3. Die Marke von Konkurrenten im nachhaltigen Modemarkt differenzieren. 4. Eine starke emotionale Verbindung zu den Verbrauchern schaffen und sie dazu inspirieren, umweltfreundlichere Entscheidungen zu treffen. |

### Beispielausgabe

> Designbriefing: Markenidentität für eine umweltfreundliche Modemarke
> Markennamenvorschlag: "Sēnsus" (Lateinisch für "Sinn" oder "Wahrnehmung", was ein Gefühl von Achtsamkeit und Verbindung mit der Umwelt hervorruft)
>
> Logo:
>
> - Ein minimalistisches und modernes Logo, das ein stilisiertes Blatt- oder Pflanzenmotiv mit einer eleganten, serifenlosen Schriftart kombiniert.
> - Das Blatt- oder Pflanzenelement symbolisiert das Engagement der Marke für Nachhaltigkeit und Umweltschutz, während die klare Typografie ein Gefühl von Raffinesse und Stil vermittelt.
> - Das Logo sollte vielseitig sein und in verschiedenen Anwendungen gut funktionieren, einschließlich Kleidungsetiketten, Verpackungen und digitalen Plattformen.
>   Farbpalette:
>
> - Eine neutrale und erdige Farbpalette, die das umweltfreundliche Ethos der Marke widerspiegelt, wie Grüntöne, Beige und warme Grautöne.
> - Akzentfarben wie tiefes Türkis oder Senfgelb können sparsam verwendet werden, um Lebendigkeit und Tiefe hinzuzufügen.
> - Die Farbpalette sollte ein Gefühl natürlicher Harmonie und Balance hervorrufen, während sie dennoch modisch und ansprechend für die Zielgruppe ist.
>   Typografie:
>
> - Eine Kombination aus einer klaren, modernen serifenlosen Schriftart für Überschriften und einer ergänzenden Serifenschriftart für Fließtext.
> - Die Typografie sollte lesbar und visuell ansprechend sein und ein Gefühl von Qualität und Raffinesse vermitteln.
> - Die konsequente Verwendung typografischer Hierarchie und Abstände wird eine kohärente und organisierte Markenidentität schaffen.
>   Visueller Stil:
>
> - Ein minimalistischer und aufgeräumter visueller Stil, der hochwertige Produktfotografie und klare, moderne Layouts betont.
> - Natürliche Elemente wie Pflanzenmotive oder Texturen einbeziehen, um die umweltfreundlichen Werte der Marke und die Verbindung zur Natur zu verstärken.
> - Negativraum effektiv nutzen, um ein Gefühl von Offenheit und Atmungsaktivität zu schaffen, was das Engagement der Marke für Nachhaltigkeit widerspiegelt.
>   Tonfall:
>
> - Der Tonfall der Marke sollte zugänglich, sachkundig und authentisch sein.
> - Sprache verwenden, die Verbraucher über die Vorteile nachhaltiger Mode und ethischer Produktionsmethoden aufklärt und inspiriert.
> - Einen Hauch von Raffinesse und Stil einbeziehen, um die modebewussten Sensibilitäten der Zielgruppe anzusprechen.
>   Markenpersönlichkeit:
>
> - Umweltbewusst und umweltorientiert, aber dennoch stilvoll und modisch.
> - Selbstbewusst und leidenschaftlich bei der Förderung nachhaltiger Praktiken und ethischer Entscheidungen.
> - Inklusiv und einladend, lädt Verbraucher ein, sich einer gleichgesinnten Gemeinschaft anzuschließen, die sich für positive Veränderungen einsetzt.
>   Insgesamt sollte die Markenidentität für "Sēnsus" ein Gleichgewicht zwischen umweltfreundlichen Werten und modischer Ästhetik schaffen und Verbraucher ansprechen, die stilvoll aussehen möchten, während sie umweltverantwortliche Entscheidungen treffen. Die kohärenten visuellen und verbalen Elemente sollten eine starke emotionale Verbindung zur Zielgruppe schaffen und sie dazu inspirieren, einen nachhaltigeren Lebensstil zu übernehmen.

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
        system="Deine Aufgabe ist es, ein umfassendes Designbriefing für eine ganzheitliche Markenidentität basierend auf den gegebenen Spezifikationen zu erstellen. Die Markenidentität sollte verschiedene Elemente umfassen, wie Vorschläge für den Markennamen, das Logo, die Farbpalette, die Typografie, den visuellen Stil, den Tonfall und die gesamte Markenpersönlichkeit. Stelle sicher, dass alle Elemente harmonisch zusammenarbeiten, um eine kohärente und einprägsame Markenerfahrung zu schaffen, die die Werte, Mission und das einzigartige Verkaufsversprechen der Marke effektiv an ihre Zielgruppe kommuniziert. Sei detailliert und umfassend und liefere genügend spezifische Details, damit jemand eine wirklich einzigartige Markenidentität erstellen kann.",
        messages=[
            {
                "role": "user",
                "content": [
                    {
                        "type": "text",
                        "text": "Markenspezifikationen:\nDies ist eine Marke, die sich darauf konzentriert, hochwertige, stilvolle Kleidung und Accessoires unter Verwendung umweltfreundlicher Materialien und ethischer Produktionsmethoden zu schaffen\nDie Marke richtet sich an umweltbewusste Verbraucher im Alter von 25-40 Jahren, die Mode, Nachhaltigkeit und soziale Verantwortung schätzen.\nDie Markenidentität sollte die folgenden Ziele erreichen:\n1. Das Engagement der Marke für Nachhaltigkeit, ethische Praktiken und Umweltschutz widerspiegeln.\n2. Die Zielgruppe ansprechen, indem ein Gefühl von Stil, Qualität und Trendigkeit vermittelt wird.\n3. Die Marke von Konkurrenten im nachhaltigen Modemarkt differenzieren.\n4. Eine starke emotionale Verbindung zu den Verbrauchern schaffen und sie dazu inspirieren, umweltfreundlichere Entscheidungen zu treffen."
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
      system: "Deine Aufgabe ist es, ein umfassendes Designbriefing für eine ganzheitliche Markenidentität basierend auf den gegebenen Spezifikationen zu erstellen. Die Markenidentität sollte verschiedene Elemente umfassen, wie Vorschläge für den Markennamen, das Logo, die Farbpalette, die Typografie, den visuellen Stil, den Tonfall und die gesamte Markenpersönlichkeit. Stelle sicher, dass alle Elemente harmonisch zusammenarbeiten, um eine kohärente und einprägsame Markenerfahrung zu schaffen, die die Werte, Mission und das einzigartige Verkaufsversprechen der Marke effektiv an ihre Zielgruppe kommuniziert. Sei detailliert und umfassend und liefere genügend spezifische Details, damit jemand eine wirklich einzigartige Markenidentität erstellen kann.",
      messages: [
        {
          "role": "user",
          "content": [
            {
              "type": "text",
              "text": "Markenspezifikationen:\nDies ist eine Marke, die sich darauf konzentriert, hochwertige, stilvolle Kleidung und Accessoires unter Verwendung umweltfreundlicher Materialien und ethischer Produktionsmethoden zu schaffen\nDie Marke richtet sich an umweltbewusste Verbraucher im Alter von 25-40 Jahren, die Mode, Nachhaltigkeit und soziale Verantwortung schätzen.\nDie Markenidentität sollte die folgenden Ziele erreichen:\n1. Das Engagement der Marke für Nachhaltigkeit, ethische Praktiken und Umweltschutz widerspiegeln.\n2. Die Zielgruppe ansprechen, indem ein Gefühl von Stil, Qualität und Trendigkeit vermittelt wird.\n3. Die Marke von Konkurrenten im nachhaltigen Modemarkt differenzieren.\n4. Eine starke emotionale Verbindung zu den Verbrauchern schaffen und sie dazu inspirieren, umweltfreundlichere Entscheidungen zu treffen."
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
        system="Deine Aufgabe ist es, ein umfassendes Designbriefing für eine ganzheitliche Markenidentität basierend auf den gegebenen Spezifikationen zu erstellen. Die Markenidentität sollte verschiedene Elemente umfassen, wie Vorschläge für den Markennamen, das Logo, die Farbpalette, die Typografie, den visuellen Stil, den Tonfall und die gesamte Markenpersönlichkeit. Stelle sicher, dass alle Elemente harmonisch zusammenarbeiten, um eine kohärente und einprägsame Markenerfahrung zu schaffen, die die Werte, Mission und das einzigartige Verkaufsversprechen der Marke effektiv an ihre Zielgruppe kommuniziert. Sei detailliert und umfassend und liefere genügend spezifische Details, damit jemand eine wirklich einzigartige Markenidentität erstellen kann.",
        messages=[
            {
                "role": "user",
                "content": [
                    {
                        "type": "text",
                        "text": "Markenspezifikationen:\nDies ist eine Marke, die sich darauf konzentriert, hochwertige, stilvolle Kleidung und Accessoires unter Verwendung umweltfreundlicher Materialien und ethischer Produktionsmethoden zu schaffen\nDie Marke richtet sich an umweltbewusste Verbraucher im Alter von 25-40 Jahren, die Mode, Nachhaltigkeit und soziale Verantwortung schätzen.\nDie Markenidentität sollte die folgenden Ziele erreichen:\n1. Das Engagement der Marke für Nachhaltigkeit, ethische Praktiken und Umweltschutz widerspiegeln.\n2. Die Zielgruppe ansprechen, indem ein Gefühl von Stil, Qualität und Trendigkeit vermittelt wird.\n3. Die Marke von Konkurrenten im nachhaltigen Modemarkt differenzieren.\n4. Eine starke emotionale Verbindung zu den Verbrauchern schaffen und sie dazu inspirieren, umweltfreundlichere Entscheidungen zu treffen."
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
      system: "Deine Aufgabe ist es, ein umfassendes Designbriefing für eine ganzheitliche Markenidentität basierend auf den gegebenen Spezifikationen zu erstellen. Die Markenidentität sollte verschiedene Elemente umfassen, wie Vorschläge für den Markennamen, das Logo, die Farbpalette, die Typografie, den visuellen Stil, den Tonfall und die gesamte Markenpersönlichkeit. Stelle sicher, dass alle Elemente harmonisch zusammenarbeiten, um eine kohärente und einprägsame Markenerfahrung zu schaffen, die die Werte, Mission und das einzigartige Verkaufsversprechen der Marke effektiv an ihre Zielgruppe kommuniziert. Sei detailliert und umfassend und liefere genügend spezifische Details, damit jemand eine wirklich einzigartige Markenidentität erstellen kann.",
      messages: [
        {
          "role": "user",
          "content": [
            {
              "type": "text",
              "text": "Markenspezifikationen:\nDies ist eine Marke, die sich darauf konzentriert, hochwertige, stilvolle Kleidung und Accessoires unter Verwendung umweltfreundlicher Materialien und ethischer Produktionsmethoden zu schaffen\nDie Marke richtet sich an umweltbewusste Verbraucher im Alter von 25-40 Jahren, die Mode, Nachhaltigkeit und soziale Verantwortung schätzen.\nDie Markenidentität sollte die folgenden Ziele erreichen:\n1. Das Engagement der Marke für Nachhaltigkeit, ethische Praktiken und Umweltschutz widerspiegeln.\n2. Die Zielgruppe ansprechen, indem ein Gefühl von Stil, Qualität und Trendigkeit vermittelt wird.\n3. Die Marke von Konkurrenten im nachhaltigen Modemarkt differenzieren.\n4. Eine starke emotionale Verbindung zu den Verbrauchern schaffen und sie dazu inspirieren, umweltfreundlichere Entscheidungen zu treffen."
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
        system="Deine Aufgabe ist es, ein umfassendes Designbriefing für eine ganzheitliche Markenidentität basierend auf den gegebenen Spezifikationen zu erstellen. Die Markenidentität sollte verschiedene Elemente umfassen, wie Vorschläge für den Markennamen, das Logo, die Farbpalette, die Typografie, den visuellen Stil, den Tonfall und die gesamte Markenpersönlichkeit. Stelle sicher, dass alle Elemente harmonisch zusammenarbeiten, um eine kohärente und einprägsame Markenerfahrung zu schaffen, die die Werte, Mission und das einzigartige Verkaufsversprechen der Marke effektiv an ihre Zielgruppe kommuniziert. Sei detailliert und umfassend und liefere genügend spezifische Details, damit jemand eine wirklich einzigartige Markenidentität erstellen kann.",
        messages=[
            {
                "role": "user",
                "content": [
                    {
                        "type": "text",
                        "text": "Markenspezifikationen:\nDies ist eine Marke, die sich darauf konzentriert, hochwertige, stilvolle Kleidung und Accessoires unter Verwendung umweltfreundlicher Materialien und ethischer Produktionsmethoden zu schaffen\nDie Marke richtet sich an umweltbewusste Verbraucher im Alter von 25-40 Jahren, die Mode, Nachhaltigkeit und soziale Verantwortung schätzen.\nDie Markenidentität sollte die folgenden Ziele erreichen:\n1. Das Engagement der Marke für Nachhaltigkeit, ethische Praktiken und Umweltschutz widerspiegeln.\n2. Die Zielgruppe ansprechen, indem ein Gefühl von Stil, Qualität und Trendigkeit vermittelt wird.\n3. Die Marke von Konkurrenten im nachhaltigen Modemarkt differenzieren.\n4. Eine starke emotionale Verbindung zu den Verbrauchern schaffen und sie dazu inspirieren, umweltfreundlichere Entscheidungen zu treffen."
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
      system: "Deine Aufgabe ist es, ein umfassendes Designbriefing für eine ganzheitliche Markenidentität basierend auf den gegebenen Spezifikationen zu erstellen. Die Markenidentität sollte verschiedene Elemente umfassen, wie Vorschläge für den Markennamen, das Logo, die Farbpalette, die Typografie, den visuellen Stil, den Tonfall und die gesamte Markenpersönlichkeit. Stelle sicher, dass alle Elemente harmonisch zusammenarbeiten, um eine kohärente und einprägsame Markenerfahrung zu schaffen, die die Werte, Mission und das einzigartige Verkaufsversprechen der Marke effektiv an ihre Zielgruppe kommuniziert. Sei detailliert und umfassend und liefere genügend spezifische Details, damit jemand eine wirklich einzigartige Markenidentität erstellen kann.",
      messages: [
        {
          "role": "user",
          "content": [
            {
              "type": "text",
              "text": "Markenspezifikationen:\nDies ist eine Marke, die sich darauf konzentriert, hochwertige, stilvolle Kleidung und Accessoires unter Verwendung umweltfreundlicher Materialien und ethischer Produktionsmethoden zu schaffen\nDie Marke richtet sich an umweltbewusste Verbraucher im Alter von 25-40 Jahren, die Mode, Nachhaltigkeit und soziale Verantwortung schätzen.\nDie Markenidentität sollte die folgenden Ziele erreichen:\n1. Das Engagement der Marke für Nachhaltigkeit, ethische Praktiken und Umweltschutz widerspiegeln.\n2. Die Zielgruppe ansprechen, indem ein Gefühl von Stil, Qualität und Trendigkeit vermittelt wird.\n3. Die Marke von Konkurrenten im nachhaltigen Modemarkt differenzieren.\n4. Eine starke emotionale Verbindung zu den Verbrauchern schaffen und sie dazu inspirieren, umweltfreundlichere Entscheidungen zu treffen."
            }
          ]
        }
      ]
    });
    console.log(msg);
    
    
    
    ```
</CodeGroup>