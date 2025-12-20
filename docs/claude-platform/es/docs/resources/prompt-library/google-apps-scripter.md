# Programador de Google Apps Script

Genera scripts de Google Apps para completar tareas basadas en los requisitos del usuario.

---

> ¡Copia este prompt en nuestra [Consola](/dashboard) de desarrollador para probarlo tú mismo!

|      | Contenido |
| ---- | --- |
| Usuario | Escríbeme un script de Google Apps que traduzca todo el texto en una presentación de Google Slides al coreano. |

## Ejemplo de Salida

Aquí tienes un script de Google Apps que traduce todo el texto en una presentación de Google Slides al coreano:

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

Así es como funciona el script:

1.  La función `translateSlidesToKorean()` recupera la presentación activa de Google Slides usando `SlidesApp.getActivePresentation()`.
2.  Obtiene todas las diapositivas en la presentación usando el método `getSlides()`.
3.  Iteramos sobre cada diapositiva en la presentación.
4.  Para cada diapositiva, recuperamos todos los elementos de página (formas, cuadros de texto, tablas, etc.) usando el método `getPageElements()`.
5.  Iteramos sobre cada elemento de página en la diapositiva.
6.  Verificamos el tipo del elemento de página usando `getPageElementType()`:

    - Si el elemento de página es una forma (`SlidesApp.PageElementType.SHAPE`), obtenemos el contenido de texto de la forma usando `getText().asString()`. Luego traducimos el texto al coreano usando `LanguageApp.translate(text, '', 'ko')`, donde `''` representa el idioma de origen (detección automática) y `'ko'` representa el idioma de destino (coreano). Finalmente, establecemos el texto traducido de vuelta a la forma usando `setText()`.
    - Si el elemento de página es una tabla (`SlidesApp.PageElementType.TABLE`), iteramos sobre cada celda en la tabla y traducimos el texto dentro de cada celda de manera similar a como manejamos las formas.

7.  El script continúa procesando todos los elementos de página en cada diapositiva hasta que todo el texto en la presentación haya sido traducido al coreano.

{' '}
<Note>
  **Nota:** Este script asume que tienes los permisos necesarios para
  acceder y modificar la presentación de Google Slides. Asegúrate de tener los
  derechos de acceso requeridos antes de ejecutar el script.
</Note>
Además, ten en cuenta que la traducción se realiza usando la API de Google Translate,
por lo que la calidad de la traducción puede variar dependiendo de la complejidad del
texto y las capacidades del servicio de traducción.

---

## Solicitud de API

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