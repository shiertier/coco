# Git gud

Menghasilkan perintah Git yang sesuai berdasarkan tindakan kontrol versi yang dijelaskan pengguna.

---

> Salin prompt ini ke [Console](/dashboard) developer kami untuk mencobanya sendiri!

| Role | Content |
| ---- | --- |
| User | Saya telah membuat beberapa perubahan pada file lokal saya dan ingin menyimpannya di repositori Git lokal saya. Perintah Git apa yang harus saya gunakan? |

## Example Output

Untuk menyimpan perubahan yang telah Anda buat pada file lokal Anda di repositori Git lokal Anda, Anda harus menggunakan perintah Git berikut:

1.  `git add .` atau `git add <file>`

    - Perintah ini melakukan staging pada perubahan yang telah Anda buat, mempersiapkannya untuk di-commit. Tanda `.` menambahkan semua perubahan di direktori saat ini dan subdirektorinya. Jika Anda hanya ingin melakukan staging pada file tertentu, ganti `.` dengan nama file atau path.

2.  `git commit -m "Pesan commit Anda"`

    - Perintah ini melakukan commit pada perubahan yang telah di-staging ke repositori Git lokal Anda. Flag `-m` memungkinkan Anda memberikan pesan commit yang menjelaskan perubahan yang telah Anda buat. Ganti `"Pesan commit Anda"` dengan ringkasan singkat dan deskriptif dari perubahan Anda.

Sebagai contoh, jika Anda telah membuat perubahan pada file bernama `example.txt`, Anda akan menjalankan:

```
git add example.txt
git commit -m "Update example.txt dengan konten baru"

```

Perintah-perintah ini akan menyimpan perubahan Anda di repositori Git lokal Anda, membuat commit baru dengan pesan yang diberikan. Anda dapat kemudian melakukan push perubahan ini ke repositori remote atau melanjutkan bekerja pada branch lokal Anda.

Ingat untuk menjalankan perintah-perintah ini dari dalam direktori repositori Git Anda di terminal atau command prompt Anda.

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
    messages=[
        {
            "role": "user",
            "content": [
                {
                    "type": "text",
                    "text": "I have made some changes to my local files and I want to save them in my local Git repository. What Git command should I use?",
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
  messages: [
    {
      "role": "user",
      "content": [
        {
          "type": "text",
          "text": "I have made some changes to my local files and I want to save them in my local Git repository. What Git command should I use?"
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
    messages=[
        {
            "role": "user",
            "content": [
                {
                    "type": "text",
                    "text": "I have made some changes to my local files and I want to save them in my local Git repository. What Git command should I use?"
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
  messages: [
    {
      "role": "user",
      "content": [
        {
          "type": "text",
          "text": "I have made some changes to my local files and I want to save them in my local Git repository. What Git command should I use?"
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
    messages=[
        {
            "role": "user",
            "content": [
                {
                    "type": "text",
                    "text": "I have made some changes to my local files and I want to save them in my local Git repository. What Git command should I use?"
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
  messages: [
    {
      "role": "user",
      "content": [
        {
          "type": "text",
          "text": "I have made some changes to my local files and I want to save them in my local Git repository. What Git command should I use?"
        }
      ]
    }
  ]
});
console.log(msg);

```

</Tab>
</Tabs>