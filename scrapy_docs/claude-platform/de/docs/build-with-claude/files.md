# Files API

Laden Sie Dateien hoch und verwalten Sie sie zur Verwendung mit der Claude API, ohne Inhalte bei jeder Anfrage erneut hochzuladen.

---

Die Files API ermöglicht es Ihnen, Dateien hochzuladen und zu verwalten, um sie mit der Claude API zu verwenden, ohne Inhalte bei jeder Anfrage erneut hochzuladen. Dies ist besonders nützlich, wenn Sie das [Code-Ausführungs-Tool](/docs/de/agents-and-tools/tool-use/code-execution-tool) verwenden, um Eingaben (z. B. Datensätze und Dokumente) bereitzustellen und dann Ausgaben (z. B. Diagramme) herunterzuladen. Sie können die Files API auch verwenden, um zu vermeiden, dass häufig verwendete Dokumente und Bilder bei mehreren API-Aufrufen kontinuierlich erneut hochgeladen werden müssen. Sie können [die API-Referenz direkt erkunden](/docs/de/api/files-create), zusätzlich zu diesem Leitfaden.

<Note>
Die Files API befindet sich derzeit in der Beta-Phase. Bitte kontaktieren Sie uns über unser [Feedback-Formular](https://forms.gle/tisHyierGwgN4DUE9), um Ihre Erfahrungen mit der Files API zu teilen.
</Note>

## Unterstützte Modelle

Das Referenzieren einer `file_id` in einer Messages-Anfrage wird in allen Modellen unterstützt, die den angegebenen Dateityp unterstützen. Beispielsweise werden [Bilder](/docs/de/build-with-claude/vision) in allen Claude 3+-Modellen unterstützt, [PDFs](/docs/de/build-with-claude/pdf-support) in allen Claude 3.5+-Modellen und [verschiedene andere Dateitypen](/docs/de/agents-and-tools/tool-use/code-execution-tool#supported-file-types) für das Code-Ausführungs-Tool in Claude Haiku 4.5 sowie allen Claude 3.7+-Modellen.

Die Files API wird derzeit nicht auf Amazon Bedrock oder Google Vertex AI unterstützt.

## Wie die Files API funktioniert

Die Files API bietet einen einfachen Ansatz zum einmaligen Hochladen und mehrfachen Verwenden von Dateien:

- **Dateien hochladen** zu unserem sicheren Speicher und erhalten Sie eine eindeutige `file_id`
- **Dateien herunterladen**, die von Skills oder dem Code-Ausführungs-Tool erstellt wurden
- **Dateien referenzieren** in [Messages](/docs/de/api/messages)-Anfragen mit der `file_id`, anstatt Inhalte erneut hochzuladen
- **Ihre Dateien verwalten** mit List-, Retrieve- und Delete-Operationen

## Wie man die Files API verwendet

<Note>
Um die Files API zu verwenden, müssen Sie den Beta-Feature-Header einschließen: `anthropic-beta: files-api-2025-04-14`.
</Note>

### Eine Datei hochladen

Laden Sie eine Datei hoch, um sie in zukünftigen API-Aufrufen zu referenzieren:

<CodeGroup>
```bash Shell
curl -X POST https://api.anthropic.com/v1/files \
  -H "x-api-key: $ANTHROPIC_API_KEY" \
  -H "anthropic-version: 2023-06-01" \
  -H "anthropic-beta: files-api-2025-04-14" \
  -F "file=@/path/to/document.pdf"
```

```python Python
import anthropic

client = anthropic.Anthropic()
client.beta.files.upload(
  file=("document.pdf", open("/path/to/document.pdf", "rb"), "application/pdf"),
)
```

```typescript TypeScript
import Anthropic, { toFile } from '@anthropic-ai/sdk';
import fs from "fs";

const anthropic = new Anthropic();

await anthropic.beta.files.upload({
  file: await toFile(fs.createReadStream('/path/to/document.pdf'), undefined, { type: 'application/pdf' })
}, {
  betas: ['files-api-2025-04-14']
});
```
</CodeGroup>

Die Antwort vom Hochladen einer Datei enthält:

```json
{
  "id": "file_011CNha8iCJcU1wXNR6q4V8w",
  "type": "file",
  "filename": "document.pdf",
  "mime_type": "application/pdf",
  "size_bytes": 1024000,
  "created_at": "2025-01-01T00:00:00Z",
  "downloadable": false
}
```

### Eine Datei in Nachrichten verwenden

Sobald hochgeladen, referenzieren Sie die Datei mit ihrer `file_id`:

<CodeGroup>
```bash Shell
curl -X POST https://api.anthropic.com/v1/messages \
  -H "x-api-key: $ANTHROPIC_API_KEY" \
  -H "anthropic-version: 2023-06-01" \
  -H "anthropic-beta: files-api-2025-04-14" \
  -H "content-type: application/json" \
  -d '{
    "model": "claude-sonnet-4-5",
    "max_tokens": 1024,
    "messages": [
      {
        "role": "user",
        "content": [
          {
            "type": "text",
            "text": "Please summarize this document for me."          
          },
          {
            "type": "document",
            "source": {
              "type": "file",
              "file_id": "file_011CNha8iCJcU1wXNR6q4V8w"
            }
          }
        ]
      }
    ]
  }'
```

```python Python
import anthropic

client = anthropic.Anthropic()

response = client.beta.messages.create(
    model="claude-sonnet-4-5",
    max_tokens=1024,
    messages=[
        {
            "role": "user",
            "content": [
                {
                    "type": "text",
                    "text": "Please summarize this document for me."
                },
                {
                    "type": "document",
                    "source": {
                        "type": "file",
                        "file_id": "file_011CNha8iCJcU1wXNR6q4V8w"
                    }
                }
            ]
        }
    ],
    betas=["files-api-2025-04-14"],
)
print(response)
```

```typescript TypeScript
import { Anthropic } from '@anthropic-ai/sdk';

const anthropic = new Anthropic();

const response = await anthropic.beta.messages.create({
  model: "claude-sonnet-4-5",
  max_tokens: 1024,
  messages: [
    {
      role: "user",
      content: [
        {
          type: "text",
          text: "Please summarize this document for me."
        },
        {
          type: "document",
          source: {
            type: "file",
            file_id: "file_011CNha8iCJcU1wXNR6q4V8w"
          }
        }
      ]
    }
  ],
  betas: ["files-api-2025-04-14"],
});

console.log(response);
```
</CodeGroup>

### Dateitypen und Inhaltsblöcke

Die Files API unterstützt verschiedene Dateitypen, die verschiedenen Inhaltsblocktypen entsprechen:

| Dateityp | MIME-Typ | Inhaltsblocktyp | Anwendungsfall |
| :--- | :--- | :--- | :--- |
| PDF | `application/pdf` | `document` | Textanalyse, Dokumentverarbeitung |
| Nur-Text | `text/plain` | `document` | Textanalyse, Verarbeitung |
| Bilder | `image/jpeg`, `image/png`, `image/gif`, `image/webp` | `image` | Bildanalyse, visuelle Aufgaben |
| [Datensätze, andere](/docs/de/agents-and-tools/tool-use/code-execution-tool#supported-file-types) | Variiert | `container_upload` | Daten analysieren, Visualisierungen erstellen  |

### Arbeiten mit anderen Dateiformaten

Für Dateitypen, die nicht als `document`-Blöcke unterstützt werden (.csv, .txt, .md, .docx, .xlsx), konvertieren Sie die Dateien in Nur-Text und fügen Sie den Inhalt direkt in Ihre Nachricht ein:

<CodeGroup>
```bash Shell
# Beispiel: Lesen einer Textdatei und Senden als Nur-Text
# Hinweis: Für Dateien mit Sonderzeichen sollten Sie Base64-Codierung in Betracht ziehen
TEXT_CONTENT=$(cat document.txt | jq -Rs .)

curl https://api.anthropic.com/v1/messages \
  -H "content-type: application/json" \
  -H "x-api-key: $ANTHROPIC_API_KEY" \
  -H "anthropic-version: 2023-06-01" \
  -d @- <<EOF
{
  "model": "claude-sonnet-4-5",
  "max_tokens": 1024,
  "messages": [
    {
      "role": "user",
      "content": [
        {
          "type": "text",
          "text": "Here's the document content:\n\n${TEXT_CONTENT}\n\nPlease summarize this document."
        }
      ]
    }
  ]
}
EOF
```

```python Python
import pandas as pd
import anthropic

client = anthropic.Anthropic()

# Beispiel: Lesen einer CSV-Datei
df = pd.read_csv('data.csv')
csv_content = df.to_string()

# Als Nur-Text in der Nachricht senden
response = client.messages.create(
    model="claude-sonnet-4-5",
    max_tokens=1024,
    messages=[
        {
            "role": "user",
            "content": [
                {
                    "type": "text",
                    "text": f"Here's the CSV data:\n\n{csv_content}\n\nPlease analyze this data."
                }
            ]
        }
    ]
)

print(response.content[0].text)
```

```typescript TypeScript
import { Anthropic } from '@anthropic-ai/sdk';
import fs from 'fs';

const anthropic = new Anthropic();

async function analyzeDocument() {
  // Beispiel: Lesen einer Textdatei
  const textContent = fs.readFileSync('document.txt', 'utf-8');

  // Als Nur-Text in der Nachricht senden
  const response = await anthropic.messages.create({
    model: 'claude-sonnet-4-5',
    max_tokens: 1024,
    messages: [
      {
        role: 'user',
        content: [
          {
            type: 'text',
            text: `Here's the document content:\n\n${textContent}\n\nPlease summarize this document.`
          }
        ]
      }
    ]
  });

  console.log(response.content[0].text);
}

analyzeDocument();
```
</CodeGroup>

<Note>
Für .docx-Dateien mit Bildern konvertieren Sie diese zuerst in das PDF-Format und verwenden Sie dann [PDF-Unterstützung](/docs/de/build-with-claude/pdf-support), um die integrierte Bildanalyse zu nutzen. Dies ermöglicht die Verwendung von Zitaten aus dem PDF-Dokument.
</Note>

#### Dokumentblöcke

Für PDFs und Textdateien verwenden Sie den `document`-Inhaltsblock:

```json
{
  "type": "document",
  "source": {
    "type": "file",
    "file_id": "file_011CNha8iCJcU1wXNR6q4V8w"
  },
  "title": "Document Title", // Optional
  "context": "Context about the document", // Optional  
  "citations": {"enabled": true} // Optional, enables citations
}
```

#### Bildblöcke

Für Bilder verwenden Sie den `image`-Inhaltsblock:

```json
{
  "type": "image",
  "source": {
    "type": "file",
    "file_id": "file_011CPMxVD3fHLUhvTqtsQA5w"
  }
}
```

### Dateien verwalten

#### Dateien auflisten

Rufen Sie eine Liste Ihrer hochgeladenen Dateien ab:

<CodeGroup>
```bash Shell
curl https://api.anthropic.com/v1/files \
  -H "x-api-key: $ANTHROPIC_API_KEY" \
  -H "anthropic-version: 2023-06-01" \
  -H "anthropic-beta: files-api-2025-04-14"
```

```python Python
import anthropic

client = anthropic.Anthropic()
files = client.beta.files.list()
```

```typescript TypeScript
import { Anthropic } from '@anthropic-ai/sdk';

const anthropic = new Anthropic();
const files = await anthropic.beta.files.list({
  betas: ['files-api-2025-04-14'],
});
```
</CodeGroup>

#### Datei-Metadaten abrufen

Rufen Sie Informationen über eine bestimmte Datei ab:

<CodeGroup>
```bash Shell
curl https://api.anthropic.com/v1/files/file_011CNha8iCJcU1wXNR6q4V8w \
  -H "x-api-key: $ANTHROPIC_API_KEY" \
  -H "anthropic-version: 2023-06-01" \
  -H "anthropic-beta: files-api-2025-04-14"
```

```python Python
import anthropic

client = anthropic.Anthropic()
file = client.beta.files.retrieve_metadata("file_011CNha8iCJcU1wXNR6q4V8w")
```

```typescript TypeScript
import { Anthropic } from '@anthropic-ai/sdk';

const anthropic = new Anthropic();
const file = await anthropic.beta.files.retrieveMetadata(
  "file_011CNha8iCJcU1wXNR6q4V8w",
  { betas: ['files-api-2025-04-14'] },
);
```
</CodeGroup>

#### Eine Datei löschen

Entfernen Sie eine Datei aus Ihrem Arbeitsbereich:

<CodeGroup>
```bash Shell
curl -X DELETE https://api.anthropic.com/v1/files/file_011CNha8iCJcU1wXNR6q4V8w \
  -H "x-api-key: $ANTHROPIC_API_KEY" \
  -H "anthropic-version: 2023-06-01" \
  -H "anthropic-beta: files-api-2025-04-14"
```

```python Python
import anthropic

client = anthropic.Anthropic()
result = client.beta.files.delete("file_011CNha8iCJcU1wXNR6q4V8w")
```

```typescript TypeScript
import { Anthropic } from '@anthropic-ai/sdk';

const anthropic = new Anthropic();
const result = await anthropic.beta.files.delete(
  "file_011CNha8iCJcU1wXNR6q4V8w",
  { betas: ['files-api-2025-04-14'] },
);
```
</CodeGroup>

### Eine Datei herunterladen

Laden Sie Dateien herunter, die von Skills oder dem Code-Ausführungs-Tool erstellt wurden:

<CodeGroup>
```bash Shell
curl -X GET "https://api.anthropic.com/v1/files/file_011CNha8iCJcU1wXNR6q4V8w/content" \
  -H "x-api-key: $ANTHROPIC_API_KEY" \
  -H "anthropic-version: 2023-06-01" \
  -H "anthropic-beta: files-api-2025-04-14" \
  --output downloaded_file.txt
```

```python Python
import anthropic

client = anthropic.Anthropic()
file_content = client.beta.files.download("file_011CNha8iCJcU1wXNR6q4V8w")

# In Datei speichern
with open("downloaded_file.txt", "w") as f:
    f.write(file_content.decode('utf-8'))
```

```typescript TypeScript
import { Anthropic } from '@anthropic-ai/sdk';
import fs from 'fs';

const anthropic = new Anthropic();

const fileContent = await anthropic.beta.files.download(
  "file_011CNha8iCJcU1wXNR6q4V8w",
  { betas: ['files-api-2025-04-14'] },
);

// In Datei speichern
fs.writeFileSync("downloaded_file.txt", fileContent);
```
</CodeGroup>

<Note>
Sie können nur Dateien herunterladen, die von [Skills](/docs/de/build-with-claude/skills-guide) oder dem [Code-Ausführungs-Tool](/docs/de/agents-and-tools/tool-use/code-execution-tool) erstellt wurden. Dateien, die Sie hochgeladen haben, können nicht heruntergeladen werden.
</Note>

---

## Dateispeicher und Limits

### Speicherlimits

- **Maximale Dateigröße:** 500 MB pro Datei
- **Gesamtspeicher:** 100 GB pro Organisation

### Datei-Lebenszyklus

- Dateien sind auf den Arbeitsbereich des API-Schlüssels beschränkt. Andere API-Schlüssel können Dateien verwenden, die von einem anderen API-Schlüssel erstellt wurden, der mit demselben Arbeitsbereich verknüpft ist
- Dateien bleiben bestehen, bis Sie sie löschen
- Gelöschte Dateien können nicht wiederhergestellt werden
- Dateien sind über die API kurz nach dem Löschen nicht zugänglich, können aber in aktiven `Messages`-API-Aufrufen und zugehörigen Tool-Verwendungen bestehen bleiben
- Dateien, die Benutzer löschen, werden gemäß unserer [Datenspeicherungsrichtlinie](https://privacy.claude.com/en/articles/7996866-how-long-do-you-store-my-organization-s-data) gelöscht.

---

## Fehlerbehandlung

Häufige Fehler bei der Verwendung der Files API sind:

- **Datei nicht gefunden (404):** Die angegebene `file_id` existiert nicht oder Sie haben keinen Zugriff darauf
- **Ungültiger Dateityp (400):** Der Dateityp stimmt nicht mit dem Inhaltsblocktyp überein (z. B. Verwendung einer Bilddatei in einem Dokumentblock)
- **Überschreitet die Kontextfenstergröße (400):** Die Datei ist größer als die Kontextfenstergröße (z. B. Verwendung einer 500 MB großen Nur-Text-Datei in einer `/v1/messages`-Anfrage)
- **Ungültiger Dateiname (400):** Der Dateiname erfüllt nicht die Längenvorgaben (1-255 Zeichen) oder enthält verbotene Zeichen (`<`, `>`, `:`, `"`, `|`, `?`, `*`, `\`, `/` oder Unicode-Zeichen 0-31)
- **Datei zu groß (413):** Datei überschreitet das 500 MB-Limit
- **Speicherlimit überschritten (403):** Ihre Organisation hat das 100 GB-Speicherlimit erreicht

```json
{
  "type": "error",
  "error": {
    "type": "invalid_request_error",
    "message": "File not found: file_011CNha8iCJcU1wXNR6q4V8w"
  }
}
```

## Nutzung und Abrechnung

File API-Operationen sind **kostenlos**:
- Dateien hochladen
- Dateien herunterladen
- Dateien auflisten
- Datei-Metadaten abrufen  
- Dateien löschen

Dateiinhalte, die in `Messages`-Anfragen verwendet werden, werden als Eingabe-Token abgerechnet. Sie können nur Dateien herunterladen, die von [Skills](/docs/de/build-with-claude/skills-guide) oder dem [Code-Ausführungs-Tool](/docs/de/agents-and-tools/tool-use/code-execution-tool) erstellt wurden.

### Rate Limits

Während der Beta-Phase:
- File-bezogene API-Aufrufe sind auf ungefähr 100 Anfragen pro Minute begrenzt
- [Kontaktieren Sie uns](mailto:sales@anthropic.com), wenn Sie höhere Limits für Ihren Anwendungsfall benötigen