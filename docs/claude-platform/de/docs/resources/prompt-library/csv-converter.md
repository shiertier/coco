# CSV-Konverter

Konvertieren Sie Daten aus verschiedenen Formaten (JSON, XML, etc.) in ordnungsgemäß formatierte CSV-Dateien.

---

> Kopieren Sie diesen Prompt in unsere Entwickler-[Konsole](/dashboard), um es selbst auszuprobieren!

|        | Inhalt |
| --- | --- |
| System | Als Datenkonvertierungsexperte ist es Ihre Aufgabe, Daten aus verschiedenen Formaten (JSON, XML, etc.) in ordnungsgemäß formatierte CSV-Dateien zu konvertieren. Der Benutzer wird die Eingabedaten im ursprünglichen Format bereitstellen, zusammen mit spezifischen Anforderungen oder Präferenzen für die CSV-Ausgabe (z.B. Spaltenreihenfolge, Trennzeichen, Kodierung). Stellen Sie sicher, dass Sie ein klares Verständnis der Datenstruktur und des gewünschten CSV-Formats haben, und stellen Sie bei Bedarf klärende Fragen. Sobald Sie die notwendigen Informationen haben, generieren Sie die CSV-Ausgabe, indem Sie die entsprechenden Formatierungsregeln befolgen, wie z.B. die Verwendung von Kommas als Trennzeichen, das Einschließen von Werten in Anführungszeichen bei Bedarf und die korrekte Behandlung von Sonderzeichen oder Zeilenumbrüchen. Geben Sie schließlich zusätzliche Anweisungen oder Tipps zum Speichern oder Verwenden der CSV-Datei. |
| User   | Bitte konvertieren Sie die folgenden JSON-Daten in eine CSV-Datei: <br/> <br/> [ <br/> \{ <br/> "name": "John Doe", <br/> "age": 30, <br/> "city": "New York", <br/> "email": "john.doe@example.com" <br/> }, <br/> \{ <br/> "name": "Jane Smith", <br/> "age": 25, <br/> "city": "London", <br/> "email": "jane.smith@example.com" <br/> }, <br/> \{ <br/> "name": "Bob Johnson", <br/> "age": 35, <br/> "city": "Paris", <br/> "email": "bob.johnson@example.com" <br/> } <br/> ] <br/> Anforderungen: <br/> - Spalten in der CSV sollten in der Reihenfolge sein: name, age, city, email <br/> - Verwenden Sie Semikolons (;) als Trennzeichen <br/> - Schließen Sie alle Werte in doppelte Anführungszeichen (") ein |

## Beispielausgabe

Basierend auf den bereitgestellten JSON-Daten und Anforderungen ist hier die konvertierte CSV-Datei:

```csv csv
"name";"age";"city";"email"
"John Doe";"30";"New York";"john.doe@example.com"
"Jane Smith";"25";"London";"jane.smith@example.com"
"Bob Johnson";"35";"Paris";"bob.johnson@example.com"
```

Erklärung:

    - Die erste Zeile der CSV-Datei enthält die Spaltenüberschriften: "name", "age", "city" und "email", getrennt durch Semikolons (;) und in doppelte Anführungszeichen (") eingeschlossen.
    - Jede nachfolgende Zeile repräsentiert eine Datenzeile, mit Werten getrennt durch Semikolons (;) und in doppelte Anführungszeichen (") eingeschlossen.
    - Die Reihenfolge der Werte in jeder Zeile entspricht der angegebenen Spaltenreihenfolge: name, age, city und email.
    - Sonderzeichen, falls in den Werten vorhanden, werden innerhalb der doppelten Anführungszeichen beibehalten.

So speichern Sie die CSV-Datei:

    1. Kopieren Sie die oben bereitgestellten CSV-Daten.
    2. Öffnen Sie einen Texteditor (z.B. Notepad, TextEdit oder einen anderen Nur-Text-Editor).
    3. Fügen Sie die CSV-Daten in den Texteditor ein.
    4. Speichern Sie die Datei mit einer .csv-Erweiterung (z.B. data.csv).

Hinweis: Beim Öffnen der CSV-Datei in einer Tabellenkalkulationsanwendung (z.B. Microsoft Excel, Google Sheets) stellen Sie sicher, dass Sie das korrekte Trennzeichen (Semikolon) und das Vorhandensein von doppelten Anführungszeichen um die Werte angeben, um eine ordnungsgemäße Formatierung zu gewährleisten.

---

## API-Anfrage

<Tabs>
<Tab title="Python">

```python
import anthropic

client = anthropic.Anthropic(  # defaults to os.environ.get("ANTHROPIC_API_KEY")
    api_key="my_api_key",
)
message = client.messages.create(
    model="claude-sonnet-4-5",
    max_tokens=1000,
    temperature=0,
    system="Als Datenkonvertierungsexperte ist es Ihre Aufgabe, Daten aus verschiedenen Formaten (JSON, XML, etc.) in ordnungsgemäß formatierte CSV-Dateien zu konvertieren. Der Benutzer wird die Eingabedaten im ursprünglichen Format bereitstellen, zusammen mit spezifischen Anforderungen oder Präferenzen für die CSV-Ausgabe (z.B. Spaltenreihenfolge, Trennzeichen, Kodierung). Stellen Sie sicher, dass Sie ein klares Verständnis der Datenstruktur und des gewünschten CSV-Formats haben, und stellen Sie bei Bedarf klärende Fragen. Sobald Sie die notwendigen Informationen haben, generieren Sie die CSV-Ausgabe, indem Sie die entsprechenden Formatierungsregeln befolgen, wie z.B. die Verwendung von Kommas als Trennzeichen, das Einschließen von Werten in Anführungszeichen bei Bedarf und die korrekte Behandlung von Sonderzeichen oder Zeilenumbrüchen. Geben Sie schließlich zusätzliche Anweisungen oder Tipps zum Speichern oder Verwenden der CSV-Datei.",
    messages=[
        {
            "role": "user",
            "content": [
                {
                    "type": "text",
                    "text": 'Bitte konvertieren Sie die folgenden JSON-Daten in eine CSV-Datei: \n \n[ \n { \n "name": "John Doe", \n "age": 30, \n "city": "New York", \n "email": "[email protected]" \n }, \n { \n "name": "Jane Smith", \n "age": 25, \n "city": "London", \n "email": "[email protected]" \n }, \n { \n "name": "Bob Johnson", \n "age": 35, \n "city": "Paris", \n "email": "[email protected]" \n } \n] \n \nAnforderungen: \n- Spalten in der CSV sollten in der Reihenfolge sein: name, age, city, email \n- Verwenden Sie Semikolons (;) als Trennzeichen \n- Schließen Sie alle Werte in doppelte Anführungszeichen (") ein',
                }
            ],
        }
    ],
)
print(message.content)


```

</Tab>
<Tab title="TypeScript">

```typescript
import Anthropic from "@anthropic-ai/sdk";

const anthropic = new Anthropic({
  apiKey: "my_api_key", // defaults to process.env["ANTHROPIC_API_KEY"]
});

const msg = await anthropic.messages.create({
  model: "claude-sonnet-4-5",
  max_tokens: 1000,
  temperature: 0,
  system: "Als Datenkonvertierungsexperte ist es Ihre Aufgabe, Daten aus verschiedenen Formaten (JSON, XML, etc.) in ordnungsgemäß formatierte CSV-Dateien zu konvertieren. Der Benutzer wird die Eingabedaten im ursprünglichen Format bereitstellen, zusammen mit spezifischen Anforderungen oder Präferenzen für die CSV-Ausgabe (z.B. Spaltenreihenfolge, Trennzeichen, Kodierung). Stellen Sie sicher, dass Sie ein klares Verständnis der Datenstruktur und des gewünschten CSV-Formats haben, und stellen Sie bei Bedarf klärende Fragen. Sobald Sie die notwendigen Informationen haben, generieren Sie die CSV-Ausgabe, indem Sie die entsprechenden Formatierungsregeln befolgen, wie z.B. die Verwendung von Kommas als Trennzeichen, das Einschließen von Werten in Anführungszeichen bei Bedarf und die korrekte Behandlung von Sonderzeichen oder Zeilenumbrüchen. Geben Sie schließlich zusätzliche Anweisungen oder Tipps zum Speichern oder Verwenden der CSV-Datei.",
  messages: [
    {
      "role": "user",
      "content": [
        {
          "type": "text",
          "text": "Bitte konvertieren Sie die folgenden JSON-Daten in eine CSV-Datei:  \n  \n[  \n  {  \n    \"name\": \"John Doe\",  \n    \"age\": 30,  \n    \"city\": \"New York\",  \n    \"email\": \"[email protected]\"  \n  },  \n  {  \n    \"name\": \"Jane Smith\",  \n    \"age\": 25,  \n    \"city\": \"London\",  \n    \"email\": \"[email protected]\"  \n  },  \n  {  \n    \"name\": \"Bob Johnson\",  \n    \"age\": 35,  \n    \"city\": \"Paris\",  \n    \"email\": \"[email protected]\"  \n  }  \n]  \n  \nAnforderungen:  \n- Spalten in der CSV sollten in der Reihenfolge sein: name, age, city, email  \n- Verwenden Sie Semikolons (;) als Trennzeichen  \n- Schließen Sie alle Werte in doppelte Anführungszeichen (\") ein"
        }
      ]
    }
  ]
});
console.log(msg);

```

</Tab>
<Tab title="AWS Bedrock Python">
```python
from anthropic import AnthropicBedrock

# See https://docs.claude.com/claude/reference/claude-on-amazon-bedrock

# for authentication options

client = AnthropicBedrock()

message = client.messages.create(
model="anthropic.claude-sonnet-4-5-20250929-v1:0",
max_tokens=1000,
temperature=0,
system="Als Datenkonvertierungsexperte ist es Ihre Aufgabe, Daten aus verschiedenen Formaten (JSON, XML, etc.) in ordnungsgemäß formatierte CSV-Dateien zu konvertieren. Der Benutzer wird die Eingabedaten im ursprünglichen Format bereitstellen, zusammen mit spezifischen Anforderungen oder Präferenzen für die CSV-Ausgabe (z.B. Spaltenreihenfolge, Trennzeichen, Kodierung). Stellen Sie sicher, dass Sie ein klares Verständnis der Datenstruktur und des gewünschten CSV-Formats haben, und stellen Sie bei Bedarf klärende Fragen. Sobald Sie die notwendigen Informationen haben, generieren Sie die CSV-Ausgabe, indem Sie die entsprechenden Formatierungsregeln befolgen, wie z.B. die Verwendung von Kommas als Trennzeichen, das Einschließen von Werten in Anführungszeichen bei Bedarf und die korrekte Behandlung von Sonderzeichen oder Zeilenumbrüchen. Geben Sie schließlich zusätzliche Anweisungen oder Tipps zum Speichern oder Verwenden der CSV-Datei.",
messages=[
{
"role": "user",
"content": [
{
"type": "text",
"text": "Bitte konvertieren Sie die folgenden JSON-Daten in eine CSV-Datei: \n \n[ \n { \n \"name\": \"John Doe\", \n \"age\": 30, \n \"city\": \"New York\", \n \"email\": \"[email protected]\" \n }, \n { \n \"name\": \"Jane Smith\", \n \"age\": 25, \n \"city\": \"London\", \n \"email\": \"[email protected]\" \n }, \n { \n \"name\": \"Bob Johnson\", \n \"age\": 35, \n \"city\": \"Paris\", \n \"email\": \"[email protected]\" \n } \n] \n \nAnforderungen: \n- Spalten in der CSV sollten in der Reihenfolge sein: name, age, city, email \n- Verwenden Sie Semikolons (;) als Trennzeichen \n- Schließen Sie alle Werte in doppelte Anführungszeichen (\") ein"
}
]
}
]
)
print(message.content)

```
</Tab>
<Tab title="AWS Bedrock TypeScript">

```typescript
import AnthropicBedrock from "@anthropic-ai/bedrock-sdk";

// See https://docs.claude.com/claude/reference/claude-on-amazon-bedrock
// for authentication options
const client = new AnthropicBedrock();

const msg = await client.messages.create({
  model: "anthropic.claude-sonnet-4-5-20250929-v1:0",
  max_tokens: 1000,
  temperature: 0,
  system: "Als Datenkonvertierungsexperte ist es Ihre Aufgabe, Daten aus verschiedenen Formaten (JSON, XML, etc.) in ordnungsgemäß formatierte CSV-Dateien zu konvertieren. Der Benutzer wird die Eingabedaten im ursprünglichen Format bereitstellen, zusammen mit spezifischen Anforderungen oder Präferenzen für die CSV-Ausgabe (z.B. Spaltenreihenfolge, Trennzeichen, Kodierung). Stellen Sie sicher, dass Sie ein klares Verständnis der Datenstruktur und des gewünschten CSV-Formats haben, und stellen Sie bei Bedarf klärende Fragen. Sobald Sie die notwendigen Informationen haben, generieren Sie die CSV-Ausgabe, indem Sie die entsprechenden Formatierungsregeln befolgen, wie z.B. die Verwendung von Kommas als Trennzeichen, das Einschließen von Werten in Anführungszeichen bei Bedarf und die korrekte Behandlung von Sonderzeichen oder Zeilenumbrüchen. Geben Sie schließlich zusätzliche Anweisungen oder Tipps zum Speichern oder Verwenden der CSV-Datei.",
  messages: [
    {
      "role": "user",
      "content": [
        {
          "type": "text",
          "text": "Bitte konvertieren Sie die folgenden JSON-Daten in eine CSV-Datei:  \n  \n[  \n  {  \n    \"name\": \"John Doe\",  \n    \"age\": 30,  \n    \"city\": \"New York\",  \n    \"email\": \"[email protected]\"  \n  },  \n  {  \n    \"name\": \"Jane Smith\",  \n    \"age\": 25,  \n    \"city\": \"London\",  \n    \"email\": \"[email protected]\"  \n  },  \n  {  \n    \"name\": \"Bob Johnson\",  \n    \"age\": 35,  \n    \"city\": \"Paris\",  \n    \"email\": \"[email protected]\"  \n  }  \n]  \n  \nAnforderungen:  \n- Spalten in der CSV sollten in der Reihenfolge sein: name, age, city, email  \n- Verwenden Sie Semikolons (;) als Trennzeichen  \n- Schließen Sie alle Werte in doppelte Anführungszeichen (\") ein"
        }
      ]
    }
  ]
});
console.log(msg);

```

</Tab>
<Tab title="Vertex AI Python">

```python
from anthropic import AnthropicVertex

client = AnthropicVertex()

message = client.messages.create(
    model="claude-sonnet-4@20250514",
    max_tokens=1000,
    temperature=0,
    system="Als Datenkonvertierungsexperte ist es Ihre Aufgabe, Daten aus verschiedenen Formaten (JSON, XML, etc.) in ordnungsgemäß formatierte CSV-Dateien zu konvertieren. Der Benutzer wird die Eingabedaten im ursprünglichen Format bereitstellen, zusammen mit spezifischen Anforderungen oder Präferenzen für die CSV-Ausgabe (z.B. Spaltenreihenfolge, Trennzeichen, Kodierung). Stellen Sie sicher, dass Sie ein klares Verständnis der Datenstruktur und des gewünschten CSV-Formats haben, und stellen Sie bei Bedarf klärende Fragen. Sobald Sie die notwendigen Informationen haben, generieren Sie die CSV-Ausgabe, indem Sie die entsprechenden Formatierungsregeln befolgen, wie z.B. die Verwendung von Kommas als Trennzeichen, das Einschließen von Werten in Anführungszeichen bei Bedarf und die korrekte Behandlung von Sonderzeichen oder Zeilenumbrüchen. Geben Sie schließlich zusätzliche Anweisungen oder Tipps zum Speichern oder Verwenden der CSV-Datei.",
    messages=[
        {
            "role": "user",
            "content": [
                {
                    "type": "text",
                    "text": "Bitte konvertieren Sie die folgenden JSON-Daten in eine CSV-Datei:  \n  \n[  \n  {  \n    \"name\": \"John Doe\",  \n    \"age\": 30,  \n    \"city\": \"New York\",  \n    \"email\": \"[email protected]\"  \n  },  \n  {  \n    \"name\": \"Jane Smith\",  \n    \"age\": 25,  \n    \"city\": \"London\",  \n    \"email\": \"[email protected]\"  \n  },  \n  {  \n    \"name\": \"Bob Johnson\",  \n    \"age\": 35,  \n    \"city\": \"Paris\",  \n    \"email\": \"[email protected]\"  \n  }  \n]  \n  \nAnforderungen:  \n- Spalten in der CSV sollten in der Reihenfolge sein: name, age, city, email  \n- Verwenden Sie Semikolons (;) als Trennzeichen  \n- Schließen Sie alle Werte in doppelte Anführungszeichen (\") ein"
                }
            ]
        }
    ]
)
print(message.content)

```

</Tab>
<Tab title="Vertex AI TypeScript">

```typescript
import { AnthropicVertex } from '@anthropic-ai/vertex-sdk';

// Reads from the `CLOUD_ML_REGION` & `ANTHROPIC_VERTEX_PROJECT_ID` environment variables.
// Additionally goes through the standard `google-auth-library` flow.
const client = new AnthropicVertex();

const msg = await client.messages.create({
  model: "claude-sonnet-4@20250514",
  max_tokens: 1000,
  temperature: 0,
  system: "Als Datenkonvertierungsexperte ist es Ihre Aufgabe, Daten aus verschiedenen Formaten (JSON, XML, etc.) in ordnungsgemäß formatierte CSV-Dateien zu konvertieren. Der Benutzer wird die Eingabedaten im ursprünglichen Format bereitstellen, zusammen mit spezifischen Anforderungen oder Präferenzen für die CSV-Ausgabe (z.B. Spaltenreihenfolge, Trennzeichen, Kodierung). Stellen Sie sicher, dass Sie ein klares Verständnis der Datenstruktur und des gewünschten CSV-Formats haben, und stellen Sie bei Bedarf klärende Fragen. Sobald Sie die notwendigen Informationen haben, generieren Sie die CSV-Ausgabe, indem Sie die entsprechenden Formatierungsregeln befolgen, wie z.B. die Verwendung von Kommas als Trennzeichen, das Einschließen von Werten in Anführungszeichen bei Bedarf und die korrekte Behandlung von Sonderzeichen oder Zeilenumbrüchen. Geben Sie schließlich zusätzliche Anweisungen oder Tipps zum Speichern oder Verwenden der CSV-Datei.",
  messages: [
    {
      "role": "user",
      "content": [
        {
          "type": "text",
          "text": "Bitte konvertieren Sie die folgenden JSON-Daten in eine CSV-Datei:  \n  \n[  \n  {  \n    \"name\": \"John Doe\",  \n    \"age\": 30,  \n    \"city\": \"New York\",  \n    \"email\": \"[email protected]\"  \n  },  \n  {  \n    \"name\": \"Jane Smith\",  \n    \"age\": 25,  \n    \"city\": \"London\",  \n    \"email\": \"[email protected]\"  \n  },  \n  {  \n    \"name\": \"Bob Johnson\",  \n    \"age\": 35,  \n    \"city\": \"Paris\",  \n    \"email\": \"[email protected]\"  \n  }  \n]  \n  \nAnforderungen:  \n- Spalten in der CSV sollten in der Reihenfolge sein: name, age, city, email  \n- Verwenden Sie Semikolons (;) als Trennzeichen  \n- Schließen Sie alle Werte in doppelte Anführungszeichen (\") ein"
        }
      ]
    }
  ]
});
console.log(msg);

```

</Tab>
</Tabs>