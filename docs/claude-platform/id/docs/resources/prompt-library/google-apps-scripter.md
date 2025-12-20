# Pembuat skrip Google apps

Menghasilkan skrip Google Apps untuk menyelesaikan tugas berdasarkan kebutuhan pengguna.

---

> Salin prompt ini ke [Console](/dashboard) developer kami untuk mencobanya sendiri!

|      | Konten |
| ---- | --- |
| User | Buatkan saya skrip Google apps yang akan menerjemahkan semua teks dalam presentasi Google Slides ke bahasa Korea. |

## Contoh Output

Berikut adalah skrip Google Apps yang menerjemahkan semua teks dalam presentasi Google Slides ke bahasa Korea:

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

Berikut cara kerja skrip ini:

1.  Fungsi `translateSlidesToKorean()` mengambil presentasi Google Slides yang aktif menggunakan `SlidesApp.getActivePresentation()`.
2.  Fungsi ini mendapatkan semua slide dalam presentasi menggunakan metode `getSlides()`.
3.  Kami melakukan iterasi pada setiap slide dalam presentasi.
4.  Untuk setiap slide, kami mengambil semua elemen halaman (bentuk, kotak teks, tabel, dll.) menggunakan metode `getPageElements()`.
5.  Kami melakukan iterasi pada setiap elemen halaman di slide.
6.  Kami memeriksa jenis elemen halaman menggunakan `getPageElementType()`:

    - Jika elemen halaman adalah bentuk (`SlidesApp.PageElementType.SHAPE`), kami mendapatkan konten teks dari bentuk menggunakan `getText().asString()`. Kemudian kami menerjemahkan teks ke bahasa Korea menggunakan `LanguageApp.translate(text, '', 'ko')`, di mana `''` mewakili bahasa sumber (deteksi otomatis) dan `'ko'` mewakili bahasa target (Korea). Akhirnya, kami mengatur teks yang diterjemahkan kembali ke bentuk menggunakan `setText()`.
    - Jika elemen halaman adalah tabel (`SlidesApp.PageElementType.TABLE`), kami melakukan iterasi pada setiap sel dalam tabel dan menerjemahkan teks dalam setiap sel dengan cara yang sama seperti cara kami menangani bentuk.

7.  Skrip terus memproses semua elemen halaman di setiap slide hingga semua teks dalam presentasi telah diterjemahkan ke bahasa Korea.

{' '}
<Note>
  **Catatan:** Skrip ini mengasumsikan bahwa Anda memiliki izin yang diperlukan untuk
  mengakses dan memodifikasi presentasi Google Slides. Pastikan Anda memiliki
  hak akses yang diperlukan sebelum menjalankan skrip.
</Note>
Selain itu, perlu diingat bahwa terjemahan dilakukan menggunakan API Google Translate,
sehingga kualitas terjemahan dapat bervariasi tergantung pada kompleksitas
teks dan kemampuan layanan terjemahan.

---

## Permintaan API

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