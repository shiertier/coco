# Scrittore di script Google Apps

Genera script Google Apps per completare attività basate sui requisiti dell'utente.

---

> Copia questo prompt nella nostra [Console](/dashboard) per sviluppatori per provarlo tu stesso!

|      | Contenuto |
| ---- | --- |
| Utente | Scrivimi uno script Google Apps che tradurrà tutto il testo in una presentazione Google Slides in coreano. |

## Output di Esempio

Ecco uno script Google Apps che traduce tutto il testo in una presentazione Google Slides in coreano:

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

Ecco come funziona lo script:

1.  La funzione `translateSlidesToKorean()` recupera la presentazione Google Slides attiva utilizzando `SlidesApp.getActivePresentation()`.
2.  Ottiene tutte le slide nella presentazione utilizzando il metodo `getSlides()`.
3.  Iteriamo su ogni slide nella presentazione.
4.  Per ogni slide, recuperiamo tutti gli elementi della pagina (forme, caselle di testo, tabelle, ecc.) utilizzando il metodo `getPageElements()`.
5.  Iteriamo su ogni elemento della pagina sulla slide.
6.  Controlliamo il tipo dell'elemento della pagina utilizzando `getPageElementType()`:

    - Se l'elemento della pagina è una forma (`SlidesApp.PageElementType.SHAPE`), otteniamo il contenuto testuale della forma utilizzando `getText().asString()`. Quindi traduciamo il testo in coreano utilizzando `LanguageApp.translate(text, '', 'ko')`, dove `''` rappresenta la lingua di origine (rilevamento automatico) e `'ko'` rappresenta la lingua di destinazione (coreano). Infine, impostiamo il testo tradotto di nuovo sulla forma utilizzando `setText()`.
    - Se l'elemento della pagina è una tabella (`SlidesApp.PageElementType.TABLE`), iteriamo su ogni cella nella tabella e traduciamo il testo all'interno di ogni cella in modo simile a come gestiamo le forme.

7.  Lo script continua a elaborare tutti gli elementi della pagina su ogni slide finché tutto il testo nella presentazione non è stato tradotto in coreano.

{' '}
<Note>
  **Nota:** Questo script presuppone che tu abbia le autorizzazioni necessarie per
  accedere e modificare la presentazione Google Slides. Assicurati di avere i
  diritti di accesso richiesti prima di eseguire lo script.
</Note>
Inoltre, tieni presente che la traduzione viene eseguita utilizzando l'API di Google Translate,
quindi la qualità della traduzione può variare a seconda della complessità del
testo e delle capacità del servizio di traduzione.

---

## Richiesta API

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