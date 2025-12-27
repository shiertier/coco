# Git gud

Generiere angemessene Git-Befehle basierend auf benutzerbeschriebenen Versionskontrollaktionen.

---

> Kopiere diesen Prompt in unsere Entwickler-[Konsole](/dashboard), um es selbst auszuprobieren!

| Rolle | Inhalt |
| ---- | --- |
| Benutzer | Ich habe einige Änderungen an meinen lokalen Dateien vorgenommen und möchte sie in meinem lokalen Git-Repository speichern. Welchen Git-Befehl sollte ich verwenden? |

## Beispielausgabe

Um die Änderungen, die Sie an Ihren lokalen Dateien vorgenommen haben, in Ihrem lokalen Git-Repository zu speichern, sollten Sie die folgenden Git-Befehle verwenden:

1.  `git add .` oder `git add <datei>`

    - Dieser Befehl stellt die Änderungen, die Sie vorgenommen haben, bereit und bereitet sie für den Commit vor. Der `.` fügt alle Änderungen im aktuellen Verzeichnis und seinen Unterverzeichnissen hinzu. Wenn Sie nur bestimmte Dateien bereitstellen möchten, ersetzen Sie `.` durch den Dateinamen oder Pfad.

2.  `git commit -m "Ihre Commit-Nachricht"`

    - Dieser Befehl committet die bereitgestellten Änderungen in Ihr lokales Git-Repository. Das `-m` Flag ermöglicht es Ihnen, eine Commit-Nachricht bereitzustellen, die die Änderungen beschreibt, die Sie vorgenommen haben. Ersetzen Sie `"Ihre Commit-Nachricht"` durch eine kurze, beschreibende Zusammenfassung Ihrer Änderungen.

Zum Beispiel, wenn Sie Änderungen an einer Datei namens `example.txt` vorgenommen haben, würden Sie ausführen:

```
git add example.txt
git commit -m "Aktualisiere example.txt mit neuem Inhalt"

```

Diese Befehle werden Ihre Änderungen in Ihrem lokalen Git-Repository speichern und einen neuen Commit mit der bereitgestellten Nachricht erstellen. Sie können diese Änderungen später zu einem Remote-Repository pushen oder weiter an Ihrem lokalen Branch arbeiten.

Denken Sie daran, diese Befehle von innerhalb Ihres Git-Repository-Verzeichnisses in Ihrem Terminal oder Ihrer Eingabeaufforderung auszuführen.

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
    messages=[
        {
            "role": "user",
            "content": [
                {
                    "type": "text",
                    "text": "I have made some changes to my local files and I want to save them in my local Git repository. What Git command should I use?",
                }
            ],
        }
    ],
)
print(message.content)


````
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
  messages: [
    {
      "role": "user",
      "content": [
        {
          "type": "text",
          "text": "I have made some changes to my local files and I want to save them in my local Git repository. What Git command should I use?"
        }
      ]
    }
  ]
});
console.log(msg);

````

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
    messages=[
        {
            "role": "user",
            "content": [
                {
                    "type": "text",
                    "text": "I have made some changes to my local files and I want to save them in my local Git repository. What Git command should I use?"
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
  messages: [
    {
      "role": "user",
      "content": [
        {
          "type": "text",
          "text": "I have made some changes to my local files and I want to save them in my local Git repository. What Git command should I use?"
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
    messages=[
        {
            "role": "user",
            "content": [
                {
                    "type": "text",
                    "text": "I have made some changes to my local files and I want to save them in my local Git repository. What Git command should I use?"
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
  messages: [
    {
      "role": "user",
      "content": [
        {
          "type": "text",
          "text": "I have made some changes to my local files and I want to save them in my local Git repository. What Git command should I use?"
        }
      ]
    }
  ]
});
console.log(msg);

```

</Tab>
</Tabs>