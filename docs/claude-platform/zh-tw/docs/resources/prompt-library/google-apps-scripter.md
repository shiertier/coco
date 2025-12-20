# Google apps 腳本編寫器

根據使用者需求生成 Google Apps 腳本來完成任務。

---

> 將此提示複製到我們的開發者[控制台](/dashboard)中親自試用！

|      | 內容 |
| ---- | --- |
| 使用者 | 為我編寫一個 Google apps 腳本，將 Google Slides 簡報中的所有文字翻譯成韓文。 |

## 範例輸出

這是一個將 Google Slides 簡報中所有文字翻譯成韓文的 Google Apps Script：

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

以下是腳本的工作原理：

1.  `translateSlidesToKorean()` 函數使用 `SlidesApp.getActivePresentation()` 檢索活動的 Google Slides 簡報。
2.  它使用 `getSlides()` 方法獲取簡報中的所有投影片。
3.  我們遍歷簡報中的每張投影片。
4.  對於每張投影片，我們使用 `getPageElements()` 方法檢索所有頁面元素（形狀、文字方塊、表格等）。
5.  我們遍歷投影片上的每個頁面元素。
6.  我們使用 `getPageElementType()` 檢查頁面元素的類型：

    - 如果頁面元素是形狀（`SlidesApp.PageElementType.SHAPE`），我們使用 `getText().asString()` 獲取形狀的文字內容。然後我們使用 `LanguageApp.translate(text, '', 'ko')` 將文字翻譯成韓文，其中 `''` 代表來源語言（自動檢測），`'ko'` 代表目標語言（韓文）。最後，我們使用 `setText()` 將翻譯後的文字設定回形狀。
    - 如果頁面元素是表格（`SlidesApp.PageElementType.TABLE`），我們遍歷表格中的每個儲存格，並以類似處理形狀的方式翻譯每個儲存格內的文字。

7.  腳本繼續處理每張投影片上的所有頁面元素，直到簡報中的所有文字都被翻譯成韓文。

{' '}
<Note>
  **注意：** 此腳本假設您具有存取和修改 Google Slides 簡報的必要權限。在執行腳本之前，請確保您擁有所需的存取權限。
</Note>
另外，請記住翻譯是使用 Google Translate
API 執行的，因此翻譯品質可能會因文字的複雜性和翻譯服務的能力而有所不同。

---

## API 請求

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