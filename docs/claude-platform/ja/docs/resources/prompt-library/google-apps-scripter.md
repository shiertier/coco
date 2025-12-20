# Google Apps スクリプター

ユーザーの要件に基づいてタスクを完了するGoogle Apps スクリプトを生成します。

---

> このプロンプトを私たちの開発者[Console](/dashboard)にコピーして、自分で試してみてください！

|      | Content |
| ---- | --- |
| User | Google Slidesプレゼンテーションのすべてのテキストを韓国語に翻訳するGoogle Apps スクリプトを書いてください。 |

## 出力例

以下は、Google Slidesプレゼンテーションのすべてのテキストを韓国語に翻訳するGoogle Apps スクリプトです：

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

スクリプトの動作方法は以下の通りです：

1.  `translateSlidesToKorean()`関数は、`SlidesApp.getActivePresentation()`を使用してアクティブなGoogle Slidesプレゼンテーションを取得します。
2.  `getSlides()`メソッドを使用してプレゼンテーション内のすべてのスライドを取得します。
3.  プレゼンテーション内の各スライドを反復処理します。
4.  各スライドについて、`getPageElements()`メソッドを使用してすべてのページ要素（図形、テキストボックス、表など）を取得します。
5.  スライド上の各ページ要素を反復処理します。
6.  `getPageElementType()`を使用してページ要素のタイプをチェックします：

    - ページ要素が図形（`SlidesApp.PageElementType.SHAPE`）の場合、`getText().asString()`を使用して図形のテキスト内容を取得します。次に、`LanguageApp.translate(text, '', 'ko')`を使用してテキストを韓国語に翻訳します。ここで`''`はソース言語（自動検出）を表し、`'ko'`はターゲット言語（韓国語）を表します。最後に、`setText()`を使用して翻訳されたテキストを図形に設定し直します。
    - ページ要素が表（`SlidesApp.PageElementType.TABLE`）の場合、表内の各セルを反復処理し、図形を処理する方法と同様に各セル内のテキストを翻訳します。

7.  スクリプトは、プレゼンテーション内のすべてのテキストが韓国語に翻訳されるまで、各スライド上のすべてのページ要素の処理を続けます。

{' '}
<Note>
  **注意：** このスクリプトは、Google Slidesプレゼンテーションにアクセスして変更するために必要な権限があることを前提としています。スクリプトを実行する前に、必要なアクセス権があることを確認してください。
</Note>
また、翻訳はGoogle Translate APIを使用して実行されるため、翻訳の品質はテキストの複雑さと翻訳サービスの機能によって異なる場合があることを念頭に置いてください。

---

## APIリクエスト

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