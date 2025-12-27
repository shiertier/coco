# Scripteur d'applications Google

Générer des scripts Google Apps pour accomplir des tâches basées sur les exigences de l'utilisateur.

---

> Copiez cette invite dans notre [Console](/dashboard) développeur pour l'essayer vous-même !

|      | Contenu |
| ---- | --- |
| Utilisateur | Écrivez-moi un script Google Apps qui traduira tout le texte d'une présentation Google Slides en coréen. |

## Exemple de sortie

Voici un script Google Apps qui traduit tout le texte d'une présentation Google Slides en coréen :

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

Voici comment fonctionne le script :

1.  La fonction `translateSlidesToKorean()` récupère la présentation Google Slides active en utilisant `SlidesApp.getActivePresentation()`.
2.  Elle obtient toutes les diapositives de la présentation en utilisant la méthode `getSlides()`.
3.  Nous itérons sur chaque diapositive de la présentation.
4.  Pour chaque diapositive, nous récupérons tous les éléments de page (formes, zones de texte, tableaux, etc.) en utilisant la méthode `getPageElements()`.
5.  Nous itérons sur chaque élément de page de la diapositive.
6.  Nous vérifions le type de l'élément de page en utilisant `getPageElementType()` :

    - Si l'élément de page est une forme (`SlidesApp.PageElementType.SHAPE`), nous obtenons le contenu textuel de la forme en utilisant `getText().asString()`. Nous traduisons ensuite le texte en coréen en utilisant `LanguageApp.translate(text, '', 'ko')`, où `''` représente la langue source (détection automatique) et `'ko'` représente la langue cible (coréen). Enfin, nous remettons le texte traduit dans la forme en utilisant `setText()`.
    - Si l'élément de page est un tableau (`SlidesApp.PageElementType.TABLE`), nous itérons sur chaque cellule du tableau et traduisons le texte dans chaque cellule de manière similaire à la façon dont nous traitons les formes.

7.  Le script continue à traiter tous les éléments de page de chaque diapositive jusqu'à ce que tout le texte de la présentation ait été traduit en coréen.

{' '}
<Note>
  **Note :** Ce script suppose que vous avez les permissions nécessaires pour
  accéder et modifier la présentation Google Slides. Assurez-vous d'avoir les
  droits d'accès requis avant d'exécuter le script.
</Note>
De plus, gardez à l'esprit que la traduction est effectuée en utilisant l'API Google Translate, donc la qualité de la traduction peut varier selon la complexité du texte et les capacités du service de traduction.

---

## Requête API

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