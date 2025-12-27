# Скриптер Google apps

Генерирует скрипты Google Apps для выполнения задач на основе требований пользователя.

---

> Скопируйте этот промпт в нашу [Консоль](/dashboard) разработчика, чтобы попробовать самостоятельно!

|      | Содержание |
| ---- | --- |
| Пользователь | Напишите мне скрипт Google apps, который переведет весь текст в презентации Google Slides на корейский язык. |

## Пример вывода

Вот скрипт Google Apps Script, который переводит весь текст в презентации Google Slides на корейский язык:

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

Вот как работает скрипт:

1.  Функция `translateSlidesToKorean()` получает активную презентацию Google Slides, используя `SlidesApp.getActivePresentation()`.
2.  Она получает все слайды в презентации, используя метод `getSlides()`.
3.  Мы итерируем по каждому слайду в презентации.
4.  Для каждого слайда мы получаем все элементы страницы (фигуры, текстовые поля, таблицы и т.д.), используя метод `getPageElements()`.
5.  Мы итерируем по каждому элементу страницы на слайде.
6.  Мы проверяем тип элемента страницы, используя `getPageElementType()`:

    - Если элемент страницы является фигурой (`SlidesApp.PageElementType.SHAPE`), мы получаем текстовое содержимое фигуры, используя `getText().asString()`. Затем мы переводим текст на корейский язык, используя `LanguageApp.translate(text, '', 'ko')`, где `''` представляет исходный язык (автоопределение), а `'ko'` представляет целевой язык (корейский). Наконец, мы устанавливаем переведенный текст обратно в фигуру, используя `setText()`.
    - Если элемент страницы является таблицей (`SlidesApp.PageElementType.TABLE`), мы итерируем по каждой ячейке в таблице и переводим текст в каждой ячейке аналогично тому, как мы обрабатываем фигуры.

7.  Скрипт продолжает обрабатывать все элементы страницы на каждом слайде, пока весь текст в презентации не будет переведен на корейский язык.

{' '}
<Note>
  **Примечание:** Этот скрипт предполагает, что у вас есть необходимые разрешения для
  доступа и изменения презентации Google Slides. Убедитесь, что у вас есть
  необходимые права доступа перед запуском скрипта.
</Note>
Также имейте в виду, что перевод выполняется с использованием API Google Translate,
поэтому качество перевода может варьироваться в зависимости от сложности
текста и возможностей службы перевода.

---

## API запрос

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