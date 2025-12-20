# Google Apps Scripter

Generiere Google Apps Scripts, um Aufgaben basierend auf Benutzeranforderungen zu erfüllen.

---

> Kopiere diesen Prompt in unsere Entwickler-[Konsole](/dashboard), um es selbst auszuprobieren!

|      | Inhalt |
| ---- | --- |
| Benutzer | Schreibe mir ein Google Apps Script, das den gesamten Text in einer Google Slides-Präsentation ins Koreanische übersetzt. |

## Beispielausgabe

Hier ist ein Google Apps Script, das den gesamten Text in einer Google Slides-Präsentation ins Koreanische übersetzt:

```javascript
function translateSlidesToKorean() {
  var presentation = SlidesApp.getActivePresentation();
  var slides = presentation.getSlides();

  for (var i = 0; i < slides.length; i++) {
    var slide = slides[i];
    var pageElements = slide.getPageElements();

    for (var j = 0; j < pageElements.length; j++) {
      var pageElement = pageElements[j];

      if (pageElement.getPageElementType() === SlidesApp.PageElementType.SHAPE) {
        var shape = pageElement.asShape();
        if (shape.getText() !== null) {
          var text = shape.getText().asString();
          var translatedText = LanguageApp.translate(text, '', 'ko');
          shape.getText().setText(translatedText);
        }
      } else if (pageElement.getPageElementType() === SlidesApp.PageElementType.TABLE) {
        var table = pageElement.asTable();
        var numRows = table.getNumRows();
        var numColumns = table.getNumColumns();

        for (var row = 0; row < numRows; row++) {
          for (var col = 0; col < numColumns; col++) {
            var tableCell = table.getCell(row, col);
            var text = tableCell.getText().asString();
            var translatedText = LanguageApp.translate(text, '', 'ko');
            tableCell.getText().setText(translatedText);
          }
        }
      }
    }
  }
}
```

So funktioniert das Script:

1.  Die Funktion `translateSlidesToKorean()` ruft die aktive Google Slides-Präsentation mit `SlidesApp.getActivePresentation()` ab.
2.  Sie erhält alle Folien in der Präsentation mit der Methode `getSlides()`.
3.  Wir iterieren über jede Folie in der Präsentation.
4.  Für jede Folie rufen wir alle Seitenelemente (Formen, Textfelder, Tabellen usw.) mit der Methode `getPageElements()` ab.
5.  Wir iterieren über jedes Seitenelement auf der Folie.
6.  Wir überprüfen den Typ des Seitenelements mit `getPageElementType()`:

    - Wenn das Seitenelement eine Form ist (`SlidesApp.PageElementType.SHAPE`), erhalten wir den Textinhalt der Form mit `getText().asString()`. Dann übersetzen wir den Text ins Koreanische mit `LanguageApp.translate(text, '', 'ko')`, wobei `''` die Quellsprache (automatische Erkennung) und `'ko'` die Zielsprache (Koreanisch) darstellt. Schließlich setzen wir den übersetzten Text zurück in die Form mit `setText()`.
    - Wenn das Seitenelement eine Tabelle ist (`SlidesApp.PageElementType.TABLE`), iterieren wir über jede Zelle in der Tabelle und übersetzen den Text in jeder Zelle ähnlich wie bei Formen.

7.  Das Script verarbeitet weiterhin alle Seitenelemente auf jeder Folie, bis der gesamte Text in der Präsentation ins Koreanische übersetzt wurde.

{' '}
<Note>
  **Hinweis:** Dieses Script geht davon aus, dass Sie die notwendigen Berechtigungen haben, um
  auf die Google Slides-Präsentation zuzugreifen und sie zu ändern. Stellen Sie sicher, dass Sie die
  erforderlichen Zugriffsrechte haben, bevor Sie das Script ausführen.
</Note>
Beachten Sie auch, dass die Übersetzung mit der Google Translate
API durchgeführt wird, sodass die Qualität der Übersetzung je nach Komplexität des
Textes und den Fähigkeiten des Übersetzungsdienstes variieren kann.

---

## API-Anfrage

<Tabs>
<Tab title="Python">

```python
import anthropic

client = anthropic.Anthropic(
  # defaults to os.environ.get("ANTHROPIC_API_KEY")
  api_key="my_api_key",
)
message = client.messages.create(
  model="claude-sonnet-4-5",
  max_tokens=2000,
  temperature=0,
  messages=[
    {
      "role": "user",
      "content": [
        {
          "type": "text",
          "text": "Write me a Google apps script that will translate all text in a Google Slides presentation to Korean."
        }
      ]
    }
  ]
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
  max_tokens: 2000,
  temperature: 0,
  messages: [
    {
      "role": "user",
      "content": [
        {
          "type": "text",
          "text": "Write me a Google apps script that will translate all text in a Google Slides presentation to Korean."
        }
      ]
    }
  ]
});
console.log(msg);

```

</Tab>

<Tab title="AWS Bedrock AI TypeScript">

```
from anthropic import AnthropicBedrock

# See https://docs.claude.com/claude/reference/claude-on-amazon-bedrock
# for authentication options
client = AnthropicBedrock()

message = client.messages.create(
    model="anthropic.claude-sonnet-4-5-20250929-v1:0",
    max_tokens=2000,
    temperature=0,
    messages=[
        {
            "role": "user",
            "content": [
                {
                    "type": "text",
                    "text": "Write me a Google apps script that will translate all text in a Google Slides presentation to Korean."
                }
            ]
        }
    ]
)
print(message.content)

```

</Tab>

<Tab title="AWS Bedrock AI Python">

```
import AnthropicBedrock from "@anthropic-ai/bedrock-sdk";

// See https://docs.claude.com/claude/reference/claude-on-amazon-bedrock
// for authentication options
const client = new AnthropicBedrock();

const msg = await client.messages.create({
  model: "anthropic.claude-sonnet-4-5-20250929-v1:0",
  max_tokens: 2000,
  temperature: 0,
  messages: [
    {
      "role": "user",
      "content": [
        {
          "type": "text",
          "text": "Write me a Google apps script that will translate all text in a Google Slides presentation to Korean."
        }
      ]
    }
  ]
});
console.log(msg);

```

</Tab>

<Tab title="Vertex AI Python">

```
from anthropic import AnthropicVertex

client = AnthropicVertex()

message = client.messages.create(
    model="claude-sonnet-4@20250514",
    max_tokens=2000,
    temperature=0,
    messages=[
        {
            "role": "user",
            "content": [
                {
                    "type": "text",
                    "text": "Write me a Google apps script that will translate all text in a Google Slides presentation to Korean."
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
max_tokens: 2000,
temperature: 0,
messages: [
{
"role": "user",
"content": [
{
"type": "text",
"text": "Write me a Google apps script that will translate all text in a Google Slides presentation to Korean."
}
]
}
]
});
console.log(msg);

```
</Tab>
</Tabs>