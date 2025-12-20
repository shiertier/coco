# Datenorganisator

Verwandeln Sie unstrukturierten Text in maßgeschneiderte JSON-Tabellen.

---

> Kopieren Sie diesen Prompt in unsere Entwickler-[Konsole](/dashboard), um es selbst auszuprobieren!

|        | Inhalt |
| --- | --- |
| System | Ihre Aufgabe ist es, den bereitgestellten unstrukturierten Text zu nehmen und ihn mit JSON in ein gut organisiertes Tabellenformat zu konvertieren. Identifizieren Sie die Hauptentitäten, Attribute oder Kategorien, die im Text erwähnt werden, und verwenden Sie sie als Schlüssel im JSON-Objekt. Extrahieren Sie dann die relevanten Informationen aus dem Text und füllen Sie die entsprechenden Werte im JSON-Objekt aus. Stellen Sie sicher, dass die Daten genau dargestellt und ordnungsgemäß innerhalb der JSON-Struktur formatiert sind. Die resultierende JSON-Tabelle sollte eine klare, strukturierte Übersicht über die im ursprünglichen Text präsentierten Informationen bieten. |
| User   | Silvermist Hollow, ein charmantes Dorf, war die Heimat einer außergewöhnlichen Gruppe von Individuen. Unter ihnen war Dr. Liam Patel, ein 45-jähriger, in Yale ausgebildeter Neurochirurg, der chirurgische Techniken im regionalen medizinischen Zentrum revolutionierte. Olivia Chen, 28 Jahre alt, war eine innovative Architektin von der UC Berkeley, die die Landschaft des Dorfes mit ihren nachhaltigen und atemberaubenden Designs verwandelte. Das örtliche Theater wurde von den bezaubernden Symphonien von Ethan Kovacs bereichert, einem 72-jährigen, in Juilliard ausgebildeten Musiker und Komponisten. Isabella Torres, eine autodidaktische Köchin mit einer Leidenschaft für lokal bezogene Zutaten, schuf eine kulinarische Sensation mit ihrem Farm-to-Table-Restaurant, das zu einem Muss-Besuch-Ziel für Feinschmecker wurde. Diese bemerkenswerten Individuen, jeder mit seinen eigenen Talenten, trugen zum lebendigen Gewebe des Lebens in Silvermist Hollow bei. |

### Beispielausgabe

```json
[
  {
    "name": "Dr. Liam Patel",
    "age": 45,
    "profession": "Neurochirurg",
    "education": "Yale",
    "accomplishments": "Revolutionierte chirurgische Techniken im regionalen medizinischen Zentrum"
  },
  {
    "name": "Olivia Chen",
    "age": 28,
    "profession": "Architektin",
    "education": "UC Berkeley",
    "accomplishments": "Verwandelte die Landschaft des Dorfes mit nachhaltigen und atemberaubenden Designs"
  },
  {
    "name": "Ethan Kovacs",
    "age": 72,
    "profession": "Musiker und Komponist",
    "education": "Juilliard",
    "accomplishments": "Bereicherte das örtliche Theater mit bezaubernden Symphonien"
  },
  {
    "name": "Isabella Torres",
    "age": null,
    "profession": "Köchin",
    "education": "Autodidaktisch",
    "accomplishments": "Schuf eine kulinarische Sensation mit ihrem Farm-to-Table-Restaurant, das zu einem Muss-Besuch-Ziel für Feinschmecker wurde"
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
        system="Ihre Aufgabe ist es, den bereitgestellten unstrukturierten Text zu nehmen und ihn mit JSON in ein gut organisiertes Tabellenformat zu konvertieren. Identifizieren Sie die Hauptentitäten, Attribute oder Kategorien, die im Text erwähnt werden, und verwenden Sie sie als Schlüssel im JSON-Objekt. Extrahieren Sie dann die relevanten Informationen aus dem Text und füllen Sie die entsprechenden Werte im JSON-Objekt aus. Stellen Sie sicher, dass die Daten genau dargestellt und ordnungsgemäß innerhalb der JSON-Struktur formatiert sind. Die resultierende JSON-Tabelle sollte eine klare, strukturierte Übersicht über die im ursprünglichen Text präsentierten Informationen bieten.",
        messages=[
            {
                "role": "user",
                "content": [
                    {
                        "type": "text",
                        "text": "Silvermist Hollow, ein charmantes Dorf, war die Heimat einer außergewöhnlichen Gruppe von Individuen. Unter ihnen war Dr. Liam Patel, ein 45-jähriger, in Yale ausgebildeter Neurochirurg, der chirurgische Techniken im regionalen medizinischen Zentrum revolutionierte. Olivia Chen, 28 Jahre alt, war eine innovative Architektin von der UC Berkeley, die die Landschaft des Dorfes mit ihren nachhaltigen und atemberaubenden Designs verwandelte. Das örtliche Theater wurde von den bezaubernden Symphonien von Ethan Kovacs bereichert, einem 72-jährigen, in Juilliard ausgebildeten Musiker und Komponisten. Isabella Torres, eine autodidaktische Köchin mit einer Leidenschaft für lokal bezogene Zutaten, schuf eine kulinarische Sensation mit ihrem Farm-to-Table-Restaurant, das zu einem Muss-Besuch-Ziel für Feinschmecker wurde. Diese bemerkenswerten Individuen, jeder mit seinen eigenen Talenten, trugen zum lebendigen Gewebe des Lebens in Silvermist Hollow bei."
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
      system: "Ihre Aufgabe ist es, den bereitgestellten unstrukturierten Text zu nehmen und ihn mit JSON in ein gut organisiertes Tabellenformat zu konvertieren. Identifizieren Sie die Hauptentitäten, Attribute oder Kategorien, die im Text erwähnt werden, und verwenden Sie sie als Schlüssel im JSON-Objekt. Extrahieren Sie dann die relevanten Informationen aus dem Text und füllen Sie die entsprechenden Werte im JSON-Objekt aus. Stellen Sie sicher, dass die Daten genau dargestellt und ordnungsgemäß innerhalb der JSON-Struktur formatiert sind. Die resultierende JSON-Tabelle sollte eine klare, strukturierte Übersicht über die im ursprünglichen Text präsentierten Informationen bieten.",
      messages: [
        {
          "role": "user",
          "content": [
            {
              "type": "text",
              "text": "Silvermist Hollow, ein charmantes Dorf, war die Heimat einer außergewöhnlichen Gruppe von Individuen. Unter ihnen war Dr. Liam Patel, ein 45-jähriger, in Yale ausgebildeter Neurochirurg, der chirurgische Techniken im regionalen medizinischen Zentrum revolutionierte. Olivia Chen, 28 Jahre alt, war eine innovative Architektin von der UC Berkeley, die die Landschaft des Dorfes mit ihren nachhaltigen und atemberaubenden Designs verwandelte. Das örtliche Theater wurde von den bezaubernden Symphonien von Ethan Kovacs bereichert, einem 72-jährigen, in Juilliard ausgebildeten Musiker und Komponisten. Isabella Torres, eine autodidaktische Köchin mit einer Leidenschaft für lokal bezogene Zutaten, schuf eine kulinarische Sensation mit ihrem Farm-to-Table-Restaurant, das zu einem Muss-Besuch-Ziel für Feinschmecker wurde. Diese bemerkenswerten Individuen, jeder mit seinen eigenen Talenten, trugen zum lebendigen Gewebe des Lebens in Silvermist Hollow bei."
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
        system="Ihre Aufgabe ist es, den bereitgestellten unstrukturierten Text zu nehmen und ihn mit JSON in ein gut organisiertes Tabellenformat zu konvertieren. Identifizieren Sie die Hauptentitäten, Attribute oder Kategorien, die im Text erwähnt werden, und verwenden Sie sie als Schlüssel im JSON-Objekt. Extrahieren Sie dann die relevanten Informationen aus dem Text und füllen Sie die entsprechenden Werte im JSON-Objekt aus. Stellen Sie sicher, dass die Daten genau dargestellt und ordnungsgemäß innerhalb der JSON-Struktur formatiert sind. Die resultierende JSON-Tabelle sollte eine klare, strukturierte Übersicht über die im ursprünglichen Text präsentierten Informationen bieten.",
        messages=[
            {
                "role": "user",
                "content": [
                    {
                        "type": "text",
                        "text": "Silvermist Hollow, ein charmantes Dorf, war die Heimat einer außergewöhnlichen Gruppe von Individuen. Unter ihnen war Dr. Liam Patel, ein 45-jähriger, in Yale ausgebildeter Neurochirurg, der chirurgische Techniken im regionalen medizinischen Zentrum revolutionierte. Olivia Chen, 28 Jahre alt, war eine innovative Architektin von der UC Berkeley, die die Landschaft des Dorfes mit ihren nachhaltigen und atemberaubenden Designs verwandelte. Das örtliche Theater wurde von den bezaubernden Symphonien von Ethan Kovacs bereichert, einem 72-jährigen, in Juilliard ausgebildeten Musiker und Komponisten. Isabella Torres, eine autodidaktische Köchin mit einer Leidenschaft für lokal bezogene Zutaten, schuf eine kulinarische Sensation mit ihrem Farm-to-Table-Restaurant, das zu einem Muss-Besuch-Ziel für Feinschmecker wurde. Diese bemerkenswerten Individuen, jeder mit seinen eigenen Talenten, trugen zum lebendigen Gewebe des Lebens in Silvermist Hollow bei."
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
      system: "Ihre Aufgabe ist es, den bereitgestellten unstrukturierten Text zu nehmen und ihn mit JSON in ein gut organisiertes Tabellenformat zu konvertieren. Identifizieren Sie die Hauptentitäten, Attribute oder Kategorien, die im Text erwähnt werden, und verwenden Sie sie als Schlüssel im JSON-Objekt. Extrahieren Sie dann die relevanten Informationen aus dem Text und füllen Sie die entsprechenden Werte im JSON-Objekt aus. Stellen Sie sicher, dass die Daten genau dargestellt und ordnungsgemäß innerhalb der JSON-Struktur formatiert sind. Die resultierende JSON-Tabelle sollte eine klare, strukturierte Übersicht über die im ursprünglichen Text präsentierten Informationen bieten.",
      messages: [
        {
          "role": "user",
          "content": [
            {
              "type": "text",
              "text": "Silvermist Hollow, ein charmantes Dorf, war die Heimat einer außergewöhnlichen Gruppe von Individuen. Unter ihnen war Dr. Liam Patel, ein 45-jähriger, in Yale ausgebildeter Neurochirurg, der chirurgische Techniken im regionalen medizinischen Zentrum revolutionierte. Olivia Chen, 28 Jahre alt, war eine innovative Architektin von der UC Berkeley, die die Landschaft des Dorfes mit ihren nachhaltigen und atemberaubenden Designs verwandelte. Das örtliche Theater wurde von den bezaubernden Symphonien von Ethan Kovacs bereichert, einem 72-jährigen, in Juilliard ausgebildeten Musiker und Komponisten. Isabella Torres, eine autodidaktische Köchin mit einer Leidenschaft für lokal bezogene Zutaten, schuf eine kulinarische Sensation mit ihrem Farm-to-Table-Restaurant, das zu einem Muss-Besuch-Ziel für Feinschmecker wurde. Diese bemerkenswerten Individuen, jeder mit seinen eigenen Talenten, trugen zum lebendigen Gewebe des Lebens in Silvermist Hollow bei."
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
        system="Ihre Aufgabe ist es, den bereitgestellten unstrukturierten Text zu nehmen und ihn mit JSON in ein gut organisiertes Tabellenformat zu konvertieren. Identifizieren Sie die Hauptentitäten, Attribute oder Kategorien, die im Text erwähnt werden, und verwenden Sie sie als Schlüssel im JSON-Objekt. Extrahieren Sie dann die relevanten Informationen aus dem Text und füllen Sie die entsprechenden Werte im JSON-Objekt aus. Stellen Sie sicher, dass die Daten genau dargestellt und ordnungsgemäß innerhalb der JSON-Struktur formatiert sind. Die resultierende JSON-Tabelle sollte eine klare, strukturierte Übersicht über die im ursprünglichen Text präsentierten Informationen bieten.",
        messages=[
            {
                "role": "user",
                "content": [
                    {
                        "type": "text",
                        "text": "Silvermist Hollow, ein charmantes Dorf, war die Heimat einer außergewöhnlichen Gruppe von Individuen. Unter ihnen war Dr. Liam Patel, ein 45-jähriger, in Yale ausgebildeter Neurochirurg, der chirurgische Techniken im regionalen medizinischen Zentrum revolutionierte. Olivia Chen, 28 Jahre alt, war eine innovative Architektin von der UC Berkeley, die die Landschaft des Dorfes mit ihren nachhaltigen und atemberaubenden Designs verwandelte. Das örtliche Theater wurde von den bezaubernden Symphonien von Ethan Kovacs bereichert, einem 72-jährigen, in Juilliard ausgebildeten Musiker und Komponisten. Isabella Torres, eine autodidaktische Köchin mit einer Leidenschaft für lokal bezogene Zutaten, schuf eine kulinarische Sensation mit ihrem Farm-to-Table-Restaurant, das zu einem Muss-Besuch-Ziel für Feinschmecker wurde. Diese bemerkenswerten Individuen, jeder mit seinen eigenen Talenten, trugen zum lebendigen Gewebe des Lebens in Silvermist Hollow bei."
                    }
                ]
            }
        ]
    )
    print(message.content)
    
    ```
    
    
    ```typescript Vertex AI Type
    import { AnthropicVertex } from '@anthropic-ai/vertex-sdk';
    
    # Reads from the `CLOUD_ML_REGION` & `ANTHROPIC_VERTEX_PROJECT_ID` environment variables.
    # Additionally goes through the standard `google-auth-library` flow.
    const client = new AnthropicVertex();
    
    const msg = await client.messages.create({
      model: "claude-sonnet-4@20250514",
      max_tokens: 1000,
      temperature: 0,
      system: "Ihre Aufgabe ist es, den bereitgestellten unstrukturierten Text zu nehmen und ihn mit JSON in ein gut organisiertes Tabellenformat zu konvertieren. Identifizieren Sie die Hauptentitäten, Attribute oder Kategorien, die im Text erwähnt werden, und verwenden Sie sie als Schlüssel im JSON-Objekt. Extrahieren Sie dann die relevanten Informationen aus dem Text und füllen Sie die entsprechenden Werte im JSON-Objekt aus. Stellen Sie sicher, dass die Daten genau dargestellt und ordnungsgemäß innerhalb der JSON-Struktur formatiert sind. Die resultierende JSON-Tabelle sollte eine klare, strukturierte Übersicht über die im ursprünglichen Text präsentierten Informationen bieten.",
      messages: [
        {
          "role": "user",
          "content": [
            {
              "type": "text",
              "text": "Silvermist Hollow, ein charmantes Dorf, war die Heimat einer außergewöhnlichen Gruppe von Individuen. Unter ihnen war Dr. Liam Patel, ein 45-jähriger, in Yale ausgebildeter Neurochirurg, der chirurgische Techniken im regionalen medizinischen Zentrum revolutionierte. Olivia Chen, 28 Jahre alt, war eine innovative Architektin von der UC Berkeley, die die Landschaft des Dorfes mit ihren nachhaltigen und atemberaubenden Designs verwandelte. Das örtliche Theater wurde von den bezaubernden Symphonien von Ethan Kovacs bereichert, einem 72-jährigen, in Juilliard ausgebildeten Musiker und Komponisten. Isabella Torres, eine autodidaktische Köchin mit einer Leidenschaft für lokal bezogene Zutaten, schuf eine kulinarische Sensation mit ihrem Farm-to-Table-Restaurant, das zu einem Muss-Besuch-Ziel für Feinschmecker wurde. Diese bemerkenswerten Individuen, jeder mit seinen eigenen Talenten, trugen zum lebendigen Gewebe des Lebens in Silvermist Hollow bei."
            }
          ]
        }
      ]
    });
    console.log(msg);
    
    ```
</CodeGroup>