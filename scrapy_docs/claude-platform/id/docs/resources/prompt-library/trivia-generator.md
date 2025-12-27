# Generator trivia

Buat pertanyaan trivia tentang berbagai topik dan berikan petunjuk saat diperlukan.

---

> Salin prompt ini ke [Console](/dashboard) developer kami untuk mencobanya sendiri!

| Role | Content |
| ---- | --- |
| User | Buat pertanyaan trivia tentang berbagai topik dan berikan petunjuk untuk membantu pengguna mencapai jawaban yang benar. Pilih dari beragam kategori dan buat pertanyaan yang menguji pengetahuan atau keterampilan penalaran pengguna. Tawarkan serangkaian petunjuk yang semakin spesifik untuk memandu pengguna menuju solusi. Pastikan bahwa pertanyaan-pertanyaan tersebut menantang dan petunjuk memberikan informasi yang cukup untuk membantu pengguna tanpa memberikan jawaban terlalu mudah. |

## Contoh Output

Kategori: Geografi

Pertanyaan:  
 Apa satu-satunya negara di dunia yang berbatasan dengan Prancis dan Spanyol?

Petunjuk 1: Negara ini terletak di pegunungan Pyrenees antara Prancis dan Spanyol.  
 Petunjuk 2: Negara ini dikenal karena pemandangan pegunungannya yang menakjubkan, resor ski, dan belanja bebas pajak.  
 Petunjuk 3: Ibu kota negara ini menggunakan nama negara itu sendiri.

Jawaban: Andorra

Kategori: Sains

Pertanyaan:  
 Apa proses di mana tumbuhan mengubah sinar matahari, air, dan karbon dioksida menjadi energi dan oksigen?

Petunjuk 1: Proses ini sangat penting bagi kehidupan di Bumi, karena menyediakan makanan dan oksigen.  
 Petunjuk 2: Proses ini terjadi terutama di daun tumbuhan.  
 Petunjuk 3: Proses ini melibatkan klorofil, pigmen hijau yang menyerap sinar matahari.

Jawaban: Fotosintesis

Kategori: Sastra

Pertanyaan:  
 Siapa yang menulis novel klasik "Pride and Prejudice," yang pertama kali diterbitkan pada tahun 1813?

Petunjuk 1: Penulisnya adalah seorang wanita Inggris yang hidup dari tahun 1775 hingga 1817.  
 Petunjuk 2: Dia dikenal karena penggambaran masyarakat dan tata krama Inggris yang cerdas dan mendalam.  
 Petunjuk 3: Karya terkenalnya yang lain termasuk "Sense and Sensibility" dan "Emma."

Jawaban: Jane Austen

Kategori: Sejarah

Pertanyaan:  
 Pada tahun berapa Tembok Berlin, yang memisahkan Jerman Timur dan Barat, mulai diruntuhkan?

Petunjuk 1: Peristiwa ini terjadi pada akhir abad ke-20.  
 Petunjuk 2: Ini menandai momen penting dalam berakhirnya Perang Dingin.  
 Petunjuk 3: Tahun ini sering dikaitkan dengan frasa "kejatuhan komunisme" di Eropa.

Jawaban: 1989

---

## API Request

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
  temperature=0.5,
  messages=[
    {
    "role": "user",
    "content": [
        {
          "type": "text",
          "text": "Buat pertanyaan trivia tentang berbagai topik dan berikan petunjuk untuk membantu pengguna mencapai jawaban yang benar. Pilih dari beragam kategori dan buat pertanyaan yang menguji pengetahuan atau keterampilan penalaran pengguna. Tawarkan serangkaian petunjuk yang semakin spesifik untuk memandu pengguna menuju solusi. Pastikan bahwa pertanyaan-pertanyaan tersebut menantang dan petunjuk memberikan informasi yang cukup untuk membantu pengguna tanpa memberikan jawaban terlalu mudah."
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
  temperature: 0.5,
  messages: [
    {
      "role": "user",
      "content": [
        {
          "type": "text",
          "text": "Buat pertanyaan trivia tentang berbagai topik dan berikan petunjuk untuk membantu pengguna mencapai jawaban yang benar. Pilih dari beragam kategori dan buat pertanyaan yang menguji pengetahuan atau keterampilan penalaran pengguna. Tawarkan serangkaian petunjuk yang semakin spesifik untuk memandu pengguna menuju solusi. Pastikan bahwa pertanyaan-pertanyaan tersebut menantang dan petunjuk memberikan informasi yang cukup untuk membantu pengguna tanpa memberikan jawaban terlalu mudah."
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
max_tokens=2000,
temperature=0.5,
messages=[
{
"role": "user",
"content": [
{
"type": "text",
"text": "Buat pertanyaan trivia tentang berbagai topik dan berikan petunjuk untuk membantu pengguna mencapai jawaban yang benar. Pilih dari beragam kategori dan buat pertanyaan yang menguji pengetahuan atau keterampilan penalaran pengguna. Tawarkan serangkaian petunjuk yang semakin spesifik untuk memandu pengguna menuju solusi. Pastikan bahwa pertanyaan-pertanyaan tersebut menantang dan petunjuk memberikan informasi yang cukup untuk membantu pengguna tanpa memberikan jawaban terlalu mudah."
}
]
}
]
)
print(message.content)

````
</Tab>
<Tab title="AWS Bedrock TypeScript">

```typescript
import AnthropicBedrock from "@anthropic-ai/bedrock-sdk";

// See https://docs.claude.com/claude/reference/claude-on-amazon-bedrock
// for authentication options
const client = new AnthropicBedrock();

const msg = await client.messages.create({
  model: "anthropic.claude-sonnet-4-5-20250929-v1:0",
  max_tokens: 2000,
  temperature: 0.5,
  messages: [
    {
      "role": "user",
      "content": [
        {
          "type": "text",
          "text": "Buat pertanyaan trivia tentang berbagai topik dan berikan petunjuk untuk membantu pengguna mencapai jawaban yang benar. Pilih dari beragam kategori dan buat pertanyaan yang menguji pengetahuan atau keterampilan penalaran pengguna. Tawarkan serangkaian petunjuk yang semakin spesifik untuk memandu pengguna menuju solusi. Pastikan bahwa pertanyaan-pertanyaan tersebut menantang dan petunjuk memberikan informasi yang cukup untuk membantu pengguna tanpa memberikan jawaban terlalu mudah."
        }
      ]
    }
  ]
});
console.log(msg);

````

</Tab>
<Tab title="Vertex AI Python">

```python
from anthropic import AnthropicVertex

client = AnthropicVertex()

message = client.messages.create(
    model="claude-sonnet-4@20250514",
    max_tokens=2000,
    temperature=0.5,
    messages=[
        {
            "role": "user",
            "content": [
                {
                    "type": "text",
                    "text": "Buat pertanyaan trivia tentang berbagai topik dan berikan petunjuk untuk membantu pengguna mencapai jawaban yang benar. Pilih dari beragam kategori dan buat pertanyaan yang menguji pengetahuan atau keterampilan penalaran pengguna. Tawarkan serangkaian petunjuk yang semakin spesifik untuk memandu pengguna menuju solusi. Pastikan bahwa pertanyaan-pertanyaan tersebut menantang dan petunjuk memberikan informasi yang cukup untuk membantu pengguna tanpa memberikan jawaban terlalu mudah."
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
  temperature: 0.5,
  messages: [
    {
      "role": "user",
      "content": [
        {
          "type": "text",
          "text": "Buat pertanyaan trivia tentang berbagai topik dan berikan petunjuk untuk membantu pengguna mencapai jawaban yang benar. Pilih dari beragam kategori dan buat pertanyaan yang menguji pengetahuan atau keterampilan penalaran pengguna. Tawarkan serangkaian petunjuk yang semakin spesifik untuk memandu pengguna menuju solusi. Pastikan bahwa pertanyaan-pertanyaan tersebut menantang dan petunjuk memberikan informasi yang cukup untuk membantu pengguna tanpa memberikan jawaban terlalu mudah."
        }
      ]
    }
  ]
});
console.log(msg);

```

</Tab>
</Tabs>