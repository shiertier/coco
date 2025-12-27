# Tabellenkalkulations-Zauberer

Generiere CSV-Tabellenkalkulationen mit verschiedenen Datentypen.

---

> Kopiere diesen Prompt in unsere Entwickler-[Konsole](/dashboard), um es selbst auszuprobieren!

|        | Inhalt |
| --- | --- |
| System | Deine Aufgabe ist es, eine CSV-Tabellenkalkulation zu generieren, die den angegebenen Datentyp enthält. Die Tabellenkalkulation sollte gut organisiert sein, mit klaren Spaltenüberschriften und angemessenen Datentypen für jede Spalte. Stelle sicher, dass die Daten realistisch, vielfältig und konsistent formatiert sind. Füge mindestens 10 Datenzeilen hinzu, ohne die Kopfzeile mitzuzählen. |
| User   | Erstelle eine Tabellenkalkulation mit Informationen über Bücher in einer Bibliothek. Füge die folgenden Spalten hinzu: Buch-ID, Titel, Autor, Genre, Erscheinungsjahr und verfügbare Exemplare. Stelle sicher, dass die Daten eine Vielzahl von Genres und Erscheinungsjahren abdecken und dass die Anzahl der verfügbaren Exemplare zwischen den Büchern variiert.                             |

### Beispielausgabe

> Hier ist eine CSV-Tabellenkalkulation mit Informationen über Bücher in einer Bibliothek, basierend auf deinen Anforderungen:
>
> Buch-ID,Titel,Autor,Genre,Erscheinungsjahr,Verfügbare Exemplare  
> 1,Wer die Nachtigall stört,Harper Lee,Klassiker,1960,5  
> 2,1984,George Orwell,Dystopisch,1949,3  
> 3,Stolz und Vorurteil,Jane Austen,Romantik,1813,7  
> 4,Der Fänger im Roggen,J.D. Salinger,Coming-of-age,1951,2  
> 5,Per Anhalter durch die Galaxis,Douglas Adams,Science Fiction,1979,4  
> 6,Sakrileg,Dan Brown,Mystery,2003,6  
> 7,Drachenläufer,Khaled Hosseini,Historische Fiktion,2003,3  
> 8,Der Alchimist,Paulo Coelho,Philosophische Fiktion,1988,5  
> 9,Die Tribute von Panem,Suzanne Collins,Young Adult,2008,8  
> 10,Verblendung,Stieg Larsson,Krimi,2005,4  
> 11,Der Hobbit,J.R.R. Tolkien,Fantasy,1937,6  
> 12,Das Tagebuch der Anne Frank,Anne Frank,Autobiografie,1947,2  
> 13,Das Schicksal ist ein mieser Verräter,John Green,Young Adult,2012,5  
> 14,Der Fänger im Roggen,J.D. Salinger,Coming-of-age,1951,3  
> 15,Supergute Tage oder Die sonderbare Welt des Christopher Boone,Mark Haddon,Mystery,2003,4

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
        temperature=0,
        system="Deine Aufgabe ist es, eine CSV-Tabellenkalkulation zu generieren, die den angegebenen Datentyp enthält. Die Tabellenkalkulation sollte gut organisiert sein, mit klaren Spaltenüberschriften und angemessenen Datentypen für jede Spalte. Stelle sicher, dass die Daten realistisch, vielfältig und konsistent formatiert sind. Füge mindestens 10 Datenzeilen hinzu, ohne die Kopfzeile mitzuzählen.",
        messages=[
            {
                "role": "user",
                "content": [
                    {
                        "type": "text",
                        "text": "Erstelle eine Tabellenkalkulation mit Informationen über Bücher in einer Bibliothek. Füge die folgenden Spalten hinzu: Buch-ID, Titel, Autor, Genre, Erscheinungsjahr und verfügbare Exemplare. Stelle sicher, dass die Daten eine Vielzahl von Genres und Erscheinungsjahren abdecken und dass die Anzahl der verfügbaren Exemplare zwischen den Büchern variiert."
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
      system: "Deine Aufgabe ist es, eine CSV-Tabellenkalkulation zu generieren, die den angegebenen Datentyp enthält. Die Tabellenkalkulation sollte gut organisiert sein, mit klaren Spaltenüberschriften und angemessenen Datentypen für jede Spalte. Stelle sicher, dass die Daten realistisch, vielfältig und konsistent formatiert sind. Füge mindestens 10 Datenzeilen hinzu, ohne die Kopfzeile mitzuzählen.",
      messages: [
        {
          "role": "user",
          "content": [
            {
              "type": "text",
              "text": "Erstelle eine Tabellenkalkulation mit Informationen über Bücher in einer Bibliothek. Füge die folgenden Spalten hinzu: Buch-ID, Titel, Autor, Genre, Erscheinungsjahr und verfügbare Exemplare. Stelle sicher, dass die Daten eine Vielzahl von Genres und Erscheinungsjahren abdecken und dass die Anzahl der verfügbaren Exemplare zwischen den Büchern variiert."
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
        system="Deine Aufgabe ist es, eine CSV-Tabellenkalkulation zu generieren, die den angegebenen Datentyp enthält. Die Tabellenkalkulation sollte gut organisiert sein, mit klaren Spaltenüberschriften und angemessenen Datentypen für jede Spalte. Stelle sicher, dass die Daten realistisch, vielfältig und konsistent formatiert sind. Füge mindestens 10 Datenzeilen hinzu, ohne die Kopfzeile mitzuzählen.",
        messages=[
            {
                "role": "user",
                "content": [
                    {
                        "type": "text",
                        "text": "Erstelle eine Tabellenkalkulation mit Informationen über Bücher in einer Bibliothek. Füge die folgenden Spalten hinzu: Buch-ID, Titel, Autor, Genre, Erscheinungsjahr und verfügbare Exemplare. Stelle sicher, dass die Daten eine Vielzahl von Genres und Erscheinungsjahren abdecken und dass die Anzahl der verfügbaren Exemplare zwischen den Büchern variiert."
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
      system: "Deine Aufgabe ist es, eine CSV-Tabellenkalkulation zu generieren, die den angegebenen Datentyp enthält. Die Tabellenkalkulation sollte gut organisiert sein, mit klaren Spaltenüberschriften und angemessenen Datentypen für jede Spalte. Stelle sicher, dass die Daten realistisch, vielfältig und konsistent formatiert sind. Füge mindestens 10 Datenzeilen hinzu, ohne die Kopfzeile mitzuzählen.",
      messages: [
        {
          "role": "user",
          "content": [
            {
              "type": "text",
              "text": "Erstelle eine Tabellenkalkulation mit Informationen über Bücher in einer Bibliothek. Füge die folgenden Spalten hinzu: Buch-ID, Titel, Autor, Genre, Erscheinungsjahr und verfügbare Exemplare. Stelle sicher, dass die Daten eine Vielzahl von Genres und Erscheinungsjahren abdecken und dass die Anzahl der verfügbaren Exemplare zwischen den Büchern variiert."
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
        system="Deine Aufgabe ist es, eine CSV-Tabellenkalkulation zu generieren, die den angegebenen Datentyp enthält. Die Tabellenkalkulation sollte gut organisiert sein, mit klaren Spaltenüberschriften und angemessenen Datentypen für jede Spalte. Stelle sicher, dass die Daten realistisch, vielfältig und konsistent formatiert sind. Füge mindestens 10 Datenzeilen hinzu, ohne die Kopfzeile mitzuzählen.",
        messages=[
            {
                "role": "user",
                "content": [
                    {
                        "type": "text",
                        "text": "Erstelle eine Tabellenkalkulation mit Informationen über Bücher in einer Bibliothek. Füge die folgenden Spalten hinzu: Buch-ID, Titel, Autor, Genre, Erscheinungsjahr und verfügbare Exemplare. Stelle sicher, dass die Daten eine Vielzahl von Genres und Erscheinungsjahren abdecken und dass die Anzahl der verfügbaren Exemplare zwischen den Büchern variiert."
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
      system: "Deine Aufgabe ist es, eine CSV-Tabellenkalkulation zu generieren, die den angegebenen Datentyp enthält. Die Tabellenkalkulation sollte gut organisiert sein, mit klaren Spaltenüberschriften und angemessenen Datentypen für jede Spalte. Stelle sicher, dass die Daten realistisch, vielfältig und konsistent formatiert sind. Füge mindestens 10 Datenzeilen hinzu, ohne die Kopfzeile mitzuzählen.",
      messages: [
        {
          "role": "user",
          "content": [
            {
              "type": "text",
              "text": "Erstelle eine Tabellenkalkulation mit Informationen über Bücher in einer Bibliothek. Füge die folgenden Spalten hinzu: Buch-ID, Titel, Autor, Genre, Erscheinungsjahr und verfügbare Exemplare. Stelle sicher, dass die Daten eine Vielzahl von Genres und Erscheinungsjahren abdecken und dass die Anzahl der verfügbaren Exemplare zwischen den Büchern variiert."
            }
          ]
        }
      ]
    });
    console.log(msg);
    
    ```
</CodeGroup>