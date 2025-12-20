# Pewarna suasana hati

Mengubah deskripsi teks suasana hati menjadi kode HEX yang sesuai.

---

> Salin prompt ini ke [Console](/dashboard) developer kami untuk mencobanya sendiri!

|        | Konten |
| --- | --- |
| System | Tugas Anda adalah mengambil deskripsi teks yang diberikan tentang suasana hati atau emosi dan menghasilkan kode warna HEX yang secara visual mewakili suasana hati tersebut. Gunakan prinsip-prinsip psikologi warna dan asosiasi umum untuk menentukan warna yang paling sesuai untuk suasana hati yang diberikan. Jika deskripsi teks tidak jelas, ambigu, atau tidak memberikan informasi yang cukup untuk menentukan warna yang sesuai, tanggapi dengan "Tidak dapat menentukan kode warna HEX untuk suasana hati yang diberikan." |
| User   | Emosi yang penuh gairah, intens, dan berapi-api, penuh dengan cinta dan hasrat. |

## Contoh Output

`# FF1744`

Kode warna HEX ini mewakili nuansa merah yang cerah dan berani. Merah umumnya dikaitkan dengan gairah, intensitas, cinta, dan hasrat. Nuansa spesifik yang dipilih di sini adalah merah yang berapi-api dan energik yang menangkap panas dan semangat dari emosi yang digambarkan. Merah dikenal dapat merangsang perasaan kegembiraan dan gairah, membuatnya menjadi pilihan yang tepat untuk mewakili suasana hati ini.

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
  max_tokens=500,
  temperature=0.5,
  system="Tugas Anda adalah mengambil deskripsi teks yang diberikan tentang suasana hati atau emosi dan menghasilkan kode warna HEX yang secara visual mewakili suasana hati tersebut. Gunakan prinsip-prinsip psikologi warna dan asosiasi umum untuk menentukan warna yang paling sesuai untuk suasana hati yang diberikan. Jika deskripsi teks tidak jelas, ambigu, atau tidak memberikan informasi yang cukup untuk menentukan warna yang sesuai, tanggapi dengan \"Tidak dapat menentukan kode warna HEX untuk suasana hati yang diberikan.\"",
  messages=[
    {
    "role": "user",
    "content": [
        {
          "type": "text",
          "text": "Emosi yang penuh gairah, intens, dan berapi-api, penuh dengan cinta dan hasrat."
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
  max_tokens: 500,
  temperature: 0.5,
  system: "Tugas Anda adalah mengambil deskripsi teks yang diberikan tentang suasana hati atau emosi dan menghasilkan kode warna HEX yang secara visual mewakili suasana hati tersebut. Gunakan prinsip-prinsip psikologi warna dan asosiasi umum untuk menentukan warna yang paling sesuai untuk suasana hati yang diberikan. Jika deskripsi teks tidak jelas, ambigu, atau tidak memberikan informasi yang cukup untuk menentukan warna yang sesuai, tanggapi dengan \"Tidak dapat menentukan kode warna HEX untuk suasana hati yang diberikan.\"",
  messages: [
    {
      "role": "user",
      "content": [
        {
          "type": "text",
          "text": "Emosi yang penuh gairah, intens, dan berapi-api, penuh dengan cinta dan hasrat."
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
    max_tokens=500,
    temperature=0.5,
    system="Tugas Anda adalah mengambil deskripsi teks yang diberikan tentang suasana hati atau emosi dan menghasilkan kode warna HEX yang secara visual mewakili suasana hati tersebut. Gunakan prinsip-prinsip psikologi warna dan asosiasi umum untuk menentukan warna yang paling sesuai untuk suasana hati yang diberikan. Jika deskripsi teks tidak jelas, ambigu, atau tidak memberikan informasi yang cukup untuk menentukan warna yang sesuai, tanggapi dengan \"Tidak dapat menentukan kode warna HEX untuk suasana hati yang diberikan.\"",
    messages=[
        {
            "role": "user",
            "content": [
                {
                    "type": "text",
                    "text": "Emosi yang penuh gairah, intens, dan berapi-api, penuh dengan cinta dan hasrat."
                }
            ]
        }
    ]
)
print(message.content)

```

</Tab>
<Tab title="AWS Bedrock TypeScript">

```typescript
import AnthropicBedrock from "@anthropic-ai/bedrock-sdk";

// See https://docs.claude.com/claude/reference/claude-on-amazon-bedrock
// for authentication options
const client = new AnthropicBedrock();

const msg = await client.messages.create({
  model: "anthropic.claude-sonnet-4-5-20250929-v1:0",
  max_tokens: 500,
  temperature: 0.5,
  system: "Tugas Anda adalah mengambil deskripsi teks yang diberikan tentang suasana hati atau emosi dan menghasilkan kode warna HEX yang secara visual mewakili suasana hati tersebut. Gunakan prinsip-prinsip psikologi warna dan asosiasi umum untuk menentukan warna yang paling sesuai untuk suasana hati yang diberikan. Jika deskripsi teks tidak jelas, ambigu, atau tidak memberikan informasi yang cukup untuk menentukan warna yang sesuai, tanggapi dengan \"Tidak dapat menentukan kode warna HEX untuk suasana hati yang diberikan.\"",
  messages: [
    {
      "role": "user",
      "content": [
        {
          "type": "text",
          "text": "Emosi yang penuh gairah, intens, dan berapi-api, penuh dengan cinta dan hasrat."
        }
      ]
    }
  ]
});
console.log(msg);

```

</Tab>
<Tab title="Vertex AI Python">

```python
from anthropic import AnthropicVertex

client = AnthropicVertex()

message = client.messages.create(
    model="claude-sonnet-4@20250514",
    max_tokens=500,
    temperature=0.5,
    system="Tugas Anda adalah mengambil deskripsi teks yang diberikan tentang suasana hati atau emosi dan menghasilkan kode warna HEX yang secara visual mewakili suasana hati tersebut. Gunakan prinsip-prinsip psikologi warna dan asosiasi umum untuk menentukan warna yang paling sesuai untuk suasana hati yang diberikan. Jika deskripsi teks tidak jelas, ambigu, atau tidak memberikan informasi yang cukup untuk menentukan warna yang sesuai, tanggapi dengan \"Tidak dapat menentukan kode warna HEX untuk suasana hati yang diberikan.\"",
    messages=[
        {
            "role": "user",
            "content": [
                {
                    "type": "text",
                    "text": "Emosi yang penuh gairah, intens, dan berapi-api, penuh dengan cinta dan hasrat."
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
  max_tokens: 500,
  temperature: 0.5,
  system: "Tugas Anda adalah mengambil deskripsi teks yang diberikan tentang suasana hati atau emosi dan menghasilkan kode warna HEX yang secara visual mewakili suasana hati tersebut. Gunakan prinsip-prinsip psikologi warna dan asosiasi umum untuk menentukan warna yang paling sesuai untuk suasana hati yang diberikan. Jika deskripsi teks tidak jelas, ambigu, atau tidak memberikan informasi yang cukup untuk menentukan warna yang sesuai, tanggapi dengan \"Tidak dapat menentukan kode warna HEX untuk suasana hati yang diberikan.\"",
  messages: [
    {
      "role": "user",
      "content": [
        {
          "type": "text",
          "text": "Emosi yang penuh gairah, intens, dan berapi-api, penuh dengan cinta dan hasrat."
        }
      ]
    }
  ]
});
console.log(msg);

```

</Tab>
</Tabs>