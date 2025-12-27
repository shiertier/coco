# Penyederhanaan untuk kelas dua

Membuat teks kompleks mudah dipahami oleh pelajar muda.

---

> Salin prompt ini ke [Console](/dashboard) developer kami untuk mencobanya sendiri!

|        | Konten |
| --- | --- |
| System | Tugas Anda adalah mengambil teks yang diberikan dan menulis ulang dengan cara yang mudah dibaca dan dipahami oleh pelajar muda di kelas 3-5. Sederhanakan kosakata tingkat lanjut, pecah kalimat panjang, jelaskan konsep sulit dalam bahasa yang sederhana, dan sajikan informasi dengan cara yang jelas dan menarik. Teks singkat yang ditulis ulang harus menyampaikan ide-ide inti dari teks asli dengan cara yang sesuai untuk usia mereka. |
| User   | Mitokondria, yang sering disebut sebagai pembangkit tenaga sel, adalah organel penting yang memainkan peran krusial dalam produksi adenosin trifosfat (ATP) melalui proses respirasi seluler. ATP adalah mata uang energi utama sel, memungkinkan berbagai fungsi seluler dan reaksi biokimia terjadi. |

## Contoh Output

Mitokondria adalah bagian kecil di dalam sel yang bekerja seperti stasiun pembangkit listrik kecil. Mereka membuat molekul khusus yang disebut ATP, yang seperti uang energi sel. ATP membantu sel melakukan pekerjaan penting dan membuatnya berjalan dengan lancar, sama seperti bagaimana uang membantu orang membeli hal-hal yang mereka butuhkan. Mitokondria sangat penting karena tanpa mereka, sel tidak akan memiliki energi untuk berfungsi dengan baik.

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
  max_tokens=1000,
  temperature=0,
  system="Tugas Anda adalah mengambil teks yang diberikan dan menulis ulang dengan cara yang mudah dibaca dan dipahami oleh pelajar muda di kelas 3-5. Sederhanakan kosakata tingkat lanjut, pecah kalimat panjang, jelaskan konsep sulit dalam bahasa yang sederhana, dan sajikan informasi dengan cara yang jelas dan menarik. Teks singkat yang ditulis ulang harus menyampaikan ide-ide inti dari teks asli dengan cara yang sesuai untuk usia mereka.",
  messages=[
    {
      "role": "user",
      "content": [
        {
          "type": "text",
          "text": "Mitokondria, yang sering disebut sebagai pembangkit tenaga sel, adalah organel penting yang memainkan peran krusial dalam produksi adenosin trifosfat (ATP) melalui proses respirasi seluler. ATP adalah mata uang energi utama sel, memungkinkan berbagai fungsi seluler dan reaksi biokimia terjadi."
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
  max_tokens: 1000,
  temperature: 0,
  system: "Tugas Anda adalah mengambil teks yang diberikan dan menulis ulang dengan cara yang mudah dibaca dan dipahami oleh pelajar muda di kelas 3-5. Sederhanakan kosakata tingkat lanjut, pecah kalimat panjang, jelaskan konsep sulit dalam bahasa yang sederhana, dan sajikan informasi dengan cara yang jelas dan menarik. Teks singkat yang ditulis ulang harus menyampaikan ide-ide inti dari teks asli dengan cara yang sesuai untuk usia mereka.",
  messages: [
    {
      "role": "user",
      "content": [
        {
          "type": "text",
          "text": "Mitokondria, yang sering disebut sebagai pembangkit tenaga sel, adalah organel penting yang memainkan peran krusial dalam produksi adenosin trifosfat (ATP) melalui proses respirasi seluler. ATP adalah mata uang energi utama sel, memungkinkan berbagai fungsi seluler dan reaksi biokimia terjadi."
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
    system="Tugas Anda adalah mengambil teks yang diberikan dan menulis ulang dengan cara yang mudah dibaca dan dipahami oleh pelajar muda di kelas 3-5. Sederhanakan kosakata tingkat lanjut, pecah kalimat panjang, jelaskan konsep sulit dalam bahasa yang sederhana, dan sajikan informasi dengan cara yang jelas dan menarik. Teks singkat yang ditulis ulang harus menyampaikan ide-ide inti dari teks asli dengan cara yang sesuai untuk usia mereka.",
    messages=[
        {
            "role": "user",
            "content": [
                {
                    "type": "text",
                    "text": "Mitokondria, yang sering disebut sebagai pembangkit tenaga sel, adalah organel penting yang memainkan peran krusial dalam produksi adenosin trifosfat (ATP) melalui proses respirasi seluler. ATP adalah mata uang energi utama sel, memungkinkan berbagai fungsi seluler dan reaksi biokimia terjadi."
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
  system: "Tugas Anda adalah mengambil teks yang diberikan dan menulis ulang dengan cara yang mudah dibaca dan dipahami oleh pelajar muda di kelas 3-5. Sederhanakan kosakata tingkat lanjut, pecah kalimat panjang, jelaskan konsep sulit dalam bahasa yang sederhana, dan sajikan informasi dengan cara yang jelas dan menarik. Teks singkat yang ditulis ulang harus menyampaikan ide-ide inti dari teks asli dengan cara yang sesuai untuk usia mereka.",
  messages: [
    {
      "role": "user",
      "content": [
        {
          "type": "text",
          "text": "Mitokondria, yang sering disebut sebagai pembangkit tenaga sel, adalah organel penting yang memainkan peran krusial dalam produksi adenosin trifosfat (ATP) melalui proses respirasi seluler. ATP adalah mata uang energi utama sel, memungkinkan berbagai fungsi seluler dan reaksi biokimia terjadi."
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
system="Tugas Anda adalah mengambil teks yang diberikan dan menulis ulang dengan cara yang mudah dibaca dan dipahami oleh pelajar muda di kelas 3-5. Sederhanakan kosakata tingkat lanjut, pecah kalimat panjang, jelaskan konsep sulit dalam bahasa yang sederhana, dan sajikan informasi dengan cara yang jelas dan menarik. Teks singkat yang ditulis ulang harus menyampaikan ide-ide inti dari teks asli dengan cara yang sesuai untuk usia mereka.",
messages=[
{
"role": "user",
"content": [
{
"type": "text",
"text": "Mitokondria, yang sering disebut sebagai pembangkit tenaga sel, adalah organel penting yang memainkan peran krusial dalam produksi adenosin trifosfat (ATP) melalui proses respirasi seluler. ATP adalah mata uang energi utama sel, memungkinkan berbagai fungsi seluler dan reaksi biokimia terjadi."
}
]
}
]
)
print(message.content)

````

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
  system: "Tugas Anda adalah mengambil teks yang diberikan dan menulis ulang dengan cara yang mudah dibaca dan dipahami oleh pelajar muda di kelas 3-5. Sederhanakan kosakata tingkat lanjut, pecah kalimat panjang, jelaskan konsep sulit dalam bahasa yang sederhana, dan sajikan informasi dengan cara yang jelas dan menarik. Teks singkat yang ditulis ulang harus menyampaikan ide-ide inti dari teks asli dengan cara yang sesuai untuk usia mereka.",
  messages: [
    {
      "role": "user",
      "content": [
        {
          "type": "text",
          "text": "Mitokondria, yang sering disebut sebagai pembangkit tenaga sel, adalah organel penting yang memainkan peran krusial dalam produksi adenosin trifosfat (ATP) melalui proses respirasi seluler. ATP adalah mata uang energi utama sel, memungkinkan berbagai fungsi seluler dan reaksi biokimia terjadi."
        }
      ]
    }
  ]
});
console.log(msg);

````

</Tab>
</Tabs>