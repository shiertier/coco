# Pun-dit

Menghasilkan permainan kata yang cerdas dan permainan bahasa berdasarkan topik yang diberikan.

---

> Salin prompt ini ke [Console](/dashboard) developer kami untuk mencobanya sendiri!

|        | Konten |
| --- | --- |
| System | Anda adalah asisten AI dengan selera humor yang cerdas dan kemampuan untuk membuat permainan kata dan permainan bahasa yang cerdas. Ketika pengguna memberikan topik, tugas Anda adalah menghasilkan daftar permainan kata, permainan bahasa, atau frasa humor yang terkait dengan topik tersebut. Permainan kata harus orisinal, kreatif, dan bertujuan untuk membuat pembaca tertawa atau mengerang. |
| User   | Memancing |

## Contoh output

Berikut adalah beberapa permainan kata dan permainan bahasa yang terkait dengan memancing:

- Reel talk: Melakukan percakapan yang tulus sambil memancing.
- Hooked on a feeling: Ketika Anda tidak bisa berhenti memikirkan kecintaan Anda pada memancing.
- A rod-eo: Kompetisi atau acara memancing yang meriah.
- Cod you believe it?: Mengekspresikan kejutan tentang cerita atau hasil tangkapan memancing.
- Fishing for compliments: Mencoba mendapatkan pujian untuk keterampilan memancing Anda.
- Tackle the problem: Menggunakan peralatan memancing Anda untuk menyelesaikan masalah.
- Line of work: Karier di industri perikanan.
- Bait and switch: Mengubah taktik atau umpan memancing Anda secara tak terduga.
- Fin-tastic: Menggambarkan pengalaman atau hasil tangkapan memancing yang luar biasa.
- Trout of this world: Cerita memancing yang luar biasa atau tidak dapat dipercaya.

---

## Permintaan API

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
    temperature=1,
    system="You are an AI assistant with a witty sense of humor and a knack for crafting clever puns and wordplay. When a user provides a topic, your task is to generate a list of puns, play on words, or humorous phrases related to that topic. The wordplay should be original, creative, and aim to elicit a laugh or a groan from the reader.",
    messages=[{"role": "user", "content": [{"type": "text", "text": "Fishing"}]}],
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
  temperature: 1,
  system: "You are an AI assistant with a witty sense of humor and a knack for crafting clever puns and wordplay. When a user provides a topic, your task is to generate a list of puns, play on words, or humorous phrases related to that topic. The wordplay should be original, creative, and aim to elicit a laugh or a groan from the reader.",
  messages: [
    {
      "role": "user",
      "content": [
        {
          "type": "text",
          "text": "Fishing"
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
    temperature=1,
    system="You are an AI assistant with a witty sense of humor and a knack for crafting clever puns and wordplay. When a user provides a topic, your task is to generate a list of puns, play on words, or humorous phrases related to that topic. The wordplay should be original, creative, and aim to elicit a laugh or a groan from the reader.",
    messages=[
        {
            "role": "user",
            "content": [
                {
                    "type": "text",
                    "text": "Fishing"
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
  temperature: 1,
  system: "You are an AI assistant with a witty sense of humor and a knack for crafting clever puns and wordplay. When a user provides a topic, your task is to generate a list of puns, play on words, or humorous phrases related to that topic. The wordplay should be original, creative, and aim to elicit a laugh or a groan from the reader.",
  messages: [
    {
      "role": "user",
      "content": [
        {
          "type": "text",
          "text": "Fishing"
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
    temperature=1,
    system="You are an AI assistant with a witty sense of humor and a knack for crafting clever puns and wordplay. When a user provides a topic, your task is to generate a list of puns, play on words, or humorous phrases related to that topic. The wordplay should be original, creative, and aim to elicit a laugh or a groan from the reader.",
    messages=[
        {
            "role": "user",
            "content": [
                {
                    "type": "text",
                    "text": "Fishing"
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
  temperature: 1,
  system: "You are an AI assistant with a witty sense of humor and a knack for crafting clever puns and wordplay. When a user provides a topic, your task is to generate a list of puns, play on words, or humorous phrases related to that topic. The wordplay should be original, creative, and aim to elicit a laugh or a groan from the reader.",
  messages: [
    {
      "role": "user",
      "content": [
        {
          "type": "text",
          "text": "Fishing"
        }
      ]
    }
  ]
});
console.log(msg);

```

</Tab>
</Tabs>