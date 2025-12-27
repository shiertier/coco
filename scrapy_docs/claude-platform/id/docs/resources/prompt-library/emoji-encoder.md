# Encoder emoji

Mengubah teks biasa menjadi pesan emoji yang menyenangkan dan ekspresif.

---

> Salin prompt ini ke [Console](/dashboard) developer kami untuk mencobanya sendiri!

|        | Konten |
| --- | --- |
| System | Tugas Anda adalah mengambil pesan teks biasa yang diberikan dan mengubahnya menjadi pesan yang ekspresif dan kaya emoji yang menyampaikan makna dan maksud yang sama. Ganti kata-kata dan frasa kunci dengan emoji yang relevan jika sesuai untuk menambah minat visual dan emosi. Gunakan emoji secara kreatif tetapi pastikan pesan tetap jelas dan mudah dipahami. Jangan mengubah pesan inti atau menambahkan informasi baru. |
| User   | Seluruh dunia adalah panggung, dan semua pria dan wanita hanyalah pemain. Mereka memiliki pintu keluar dan pintu masuk mereka; Dan satu orang dalam waktunya memainkan banyak peran. |

## Contoh output

Seluruh 🌍 adalah 🎭, dan semua 👨 dan 👩 hanyalah 🎭🎬. Mereka memiliki 🚪🚶‍♂️ dan 🚶‍♀️🚪 mereka; Dan satu 👨 dalam ⌛nya memainkan banyak 🎭.

---

## API Request

<Tabs>
<Tab title="Python">
```python
import anthropic

client = anthropic.Anthropic(  # defaults to os.environ.get("ANTHROPIC_API_KEY")
    api_key="my_api_key",
)
message = client.messages.create(
    model="claude-sonnet-4-5",
    max_tokens=1000,
    temperature=0,
    system="Tugas Anda adalah mengambil pesan teks biasa yang diberikan dan mengubahnya menjadi pesan yang ekspresif dan kaya emoji yang menyampaikan makna dan maksud yang sama. Ganti kata-kata dan frasa kunci dengan emoji yang relevan jika sesuai untuk menambah minat visual dan emosi. Gunakan emoji secara kreatif tetapi pastikan pesan tetap jelas dan mudah dipahami. Jangan mengubah pesan inti atau menambahkan informasi baru.",
    messages=[
        {
            "role": "user",
            "content": [
                {
                    "type": "text",
                    "text": "Seluruh dunia adalah panggung, dan semua pria dan wanita hanyalah pemain. Mereka memiliki pintu keluar dan pintu masuk mereka; Dan satu orang dalam waktunya memainkan banyak peran.",
                }
            ],
        }
    ],
)
print(message.content)


````
</Tab>
<Tab title="TypeScript">

```typescript
import Anthropic from "@anthropic-ai/sdk";

const anthropic = new Anthropic({
  apiKey: "my_api_key", // defaults to process.env["ANTHROPIC_API_KEY"]
});

const msg = await anthropic.messages.create({
  model: "claude-sonnet-4-5",
  max_tokens: 1000,
  temperature: 0,
  system: "Tugas Anda adalah mengambil pesan teks biasa yang diberikan dan mengubahnya menjadi pesan yang ekspresif dan kaya emoji yang menyampaikan makna dan maksud yang sama. Ganti kata-kata dan frasa kunci dengan emoji yang relevan jika sesuai untuk menambah minat visual dan emosi. Gunakan emoji secara kreatif tetapi pastikan pesan tetap jelas dan mudah dipahami. Jangan mengubah pesan inti atau menambahkan informasi baru.",
  messages: [
    {
      "role": "user",
      "content": [
        {
          "type": "text",
          "text": "Seluruh dunia adalah panggung, dan semua pria dan wanita hanyalah pemain. Mereka memiliki pintu keluar dan pintu masuk mereka; Dan satu orang dalam waktunya memainkan banyak peran."
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
    max_tokens=1000,
    temperature=0,
    system="Tugas Anda adalah mengambil pesan teks biasa yang diberikan dan mengubahnya menjadi pesan yang ekspresif dan kaya emoji yang menyampaikan makna dan maksud yang sama. Ganti kata-kata dan frasa kunci dengan emoji yang relevan jika sesuai untuk menambah minat visual dan emosi. Gunakan emoji secara kreatif tetapi pastikan pesan tetap jelas dan mudah dipahami. Jangan mengubah pesan inti atau menambahkan informasi baru.",
    messages=[
        {
            "role": "user",
            "content": [
                {
                    "type": "text",
                    "text": "Seluruh dunia adalah panggung, dan semua pria dan wanita hanyalah pemain. Mereka memiliki pintu keluar dan pintu masuk mereka; Dan satu orang dalam waktunya memainkan banyak peran."
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
  max_tokens: 1000,
  temperature: 0,
  system: "Tugas Anda adalah mengambil pesan teks biasa yang diberikan dan mengubahnya menjadi pesan yang ekspresif dan kaya emoji yang menyampaikan makna dan maksud yang sama. Ganti kata-kata dan frasa kunci dengan emoji yang relevan jika sesuai untuk menambah minat visual dan emosi. Gunakan emoji secara kreatif tetapi pastikan pesan tetap jelas dan mudah dipahami. Jangan mengubah pesan inti atau menambahkan informasi baru.",
  messages: [
    {
      "role": "user",
      "content": [
        {
          "type": "text",
          "text": "Seluruh dunia adalah panggung, dan semua pria dan wanita hanyalah pemain. Mereka memiliki pintu keluar dan pintu masuk mereka; Dan satu orang dalam waktunya memainkan banyak peran."
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
    max_tokens=1000,
    temperature=0,
    system="Tugas Anda adalah mengambil pesan teks biasa yang diberikan dan mengubahnya menjadi pesan yang ekspresif dan kaya emoji yang menyampaikan makna dan maksud yang sama. Ganti kata-kata dan frasa kunci dengan emoji yang relevan jika sesuai untuk menambah minat visual dan emosi. Gunakan emoji secara kreatif tetapi pastikan pesan tetap jelas dan mudah dipahami. Jangan mengubah pesan inti atau menambahkan informasi baru.",
    messages=[
        {
            "role": "user",
            "content": [
                {
                    "type": "text",
                    "text": "Seluruh dunia adalah panggung, dan semua pria dan wanita hanyalah pemain. Mereka memiliki pintu keluar dan pintu masuk mereka; Dan satu orang dalam waktunya memainkan banyak peran."
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
  max_tokens: 1000,
  temperature: 0,
  system: "Tugas Anda adalah mengambil pesan teks biasa yang diberikan dan mengubahnya menjadi pesan yang ekspresif dan kaya emoji yang menyampaikan makna dan maksud yang sama. Ganti kata-kata dan frasa kunci dengan emoji yang relevan jika sesuai untuk menambah minat visual dan emosi. Gunakan emoji secara kreatif tetapi pastikan pesan tetap jelas dan mudah dipahami. Jangan mengubah pesan inti atau menambahkan informasi baru.",
  messages: [
    {
      "role": "user",
      "content": [
        {
          "type": "text",
          "text": "Seluruh dunia adalah panggung, dan semua pria dan wanita hanyalah pemain. Mereka memiliki pintu keluar dan pintu masuk mereka; Dan satu orang dalam waktunya memainkan banyak peran."
        }
      ]
    }
  ]
});
console.log(msg);

```

</Tab>
</Tabs>