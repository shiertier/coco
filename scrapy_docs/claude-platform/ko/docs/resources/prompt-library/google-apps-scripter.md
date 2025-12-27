# Google 앱스 스크립터

사용자 요구사항에 따라 작업을 완료하기 위한 Google Apps 스크립트를 생성합니다.

---

> 이 프롬프트를 우리의 개발자 [Console](/dashboard)에 복사해서 직접 시도해보세요!

|      | 내용 |
| ---- | --- |
| 사용자 | Google Slides 프레젠테이션의 모든 텍스트를 한국어로 번역하는 Google 앱스 스크립트를 작성해주세요. |

## 예시 출력

다음은 Google Slides 프레젠테이션의 모든 텍스트를 한국어로 번역하는 Google Apps Script입니다:

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

스크립트가 작동하는 방식은 다음과 같습니다:

1.  `translateSlidesToKorean()` 함수는 `SlidesApp.getActivePresentation()`을 사용하여 활성 Google Slides 프레젠테이션을 검색합니다.
2.  `getSlides()` 메서드를 사용하여 프레젠테이션의 모든 슬라이드를 가져옵니다.
3.  프레젠테이션의 각 슬라이드를 반복합니다.
4.  각 슬라이드에 대해 `getPageElements()` 메서드를 사용하여 모든 페이지 요소(도형, 텍스트 상자, 표 등)를 검색합니다.
5.  슬라이드의 각 페이지 요소를 반복합니다.
6.  `getPageElementType()`을 사용하여 페이지 요소의 유형을 확인합니다:

    - 페이지 요소가 도형(`SlidesApp.PageElementType.SHAPE`)인 경우, `getText().asString()`을 사용하여 도형의 텍스트 내용을 가져옵니다. 그런 다음 `LanguageApp.translate(text, '', 'ko')`를 사용하여 텍스트를 한국어로 번역합니다. 여기서 `''`는 소스 언어(자동 감지)를 나타내고 `'ko'`는 대상 언어(한국어)를 나타냅니다. 마지막으로 `setText()`를 사용하여 번역된 텍스트를 도형에 다시 설정합니다.
    - 페이지 요소가 표(`SlidesApp.PageElementType.TABLE`)인 경우, 표의 각 셀을 반복하고 도형을 처리하는 방식과 유사하게 각 셀 내의 텍스트를 번역합니다.

7.  스크립트는 프레젠테이션의 모든 텍스트가 한국어로 번역될 때까지 각 슬라이드의 모든 페이지 요소를 계속 처리합니다.

{' '}
<Note>
  **참고:** 이 스크립트는 Google Slides 프레젠테이션에 액세스하고 수정할 수 있는 필요한 권한이 있다고 가정합니다. 스크립트를 실행하기 전에 필요한 액세스 권한이 있는지 확인하세요.
</Note>
또한 번역은 Google Translate API를 사용하여 수행되므로 텍스트의 복잡성과 번역 서비스의 기능에 따라 번역 품질이 달라질 수 있다는 점을 염두에 두세요.

---

## API 요청

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