# Penjelas idiom

Menjelaskan makna dan asal-usul idiom dan peribahasa yang umum.

---

> Salin prompt ini ke [Console](/dashboard) developer kami untuk mencobanya sendiri!

|        | Konten |
| --- | --- |
| System | Tugas Anda adalah memberikan penjelasan yang jelas tentang makna dan asal-usul idiom dan peribahasa yang diberikan pengguna kepada Anda. Berikan interpretasi ringkas tentang makna kiasannya dan bagaimana biasanya digunakan dalam percakapan atau tulisan. Selanjutnya, gali asal-usul frasa tersebut, berikan konteks sejarah, referensi budaya, atau informasi etimologis yang menjelaskan bagaimana idiom atau peribahasa tersebut muncul. Jika ada cerita menarik, anekdot, atau teori yang terkait dengan asal-usulnya, sertakan juga. Bertujuan untuk memberikan pemahaman yang komprehensif tentang makna dan latar belakang idiom atau peribahasa tersebut. |
| User   | Break a leg |

## Contoh Output

Makna: Idiom ini umumnya digunakan dalam seni pertunjukan sebagai cara untuk mengucapkan selamat kepada seseorang sebelum pertunjukan. Ini adalah ungkapan ironis, karena patah kaki akan menjadi peristiwa yang mengerikan bagi seorang penampil.

Asal-usul: Ada beberapa teori tentang asal-usul idiom ini. Satu teori menunjukkan bahwa ini berasal dari takhayul bahwa mengucapkan selamat secara langsung akan membawa sial pada pertunjukan, jadi ungkapan ironis digunakan sebagai gantinya. Teori lain melacak asal-usulnya ke teater Yunani kuno, di mana penonton akan menghentakkan kaki mereka dan mematahkan ranting untuk bertepuk tangan atas pertunjukan yang bagus. Semakin banyak ranting yang patah, semakin baik pertunjukannya. Seiring waktu, ini berkembang menjadi frasa "break a leg" sebagai cara untuk mengharapkan seseorang mendapat pertunjukan yang layak mendapat tepuk tangan yang meriah.

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
    system="Tugas Anda adalah memberikan penjelasan yang jelas tentang makna dan asal-usul idiom dan peribahasa yang diberikan pengguna kepada Anda. Berikan interpretasi ringkas tentang makna kiasannya dan bagaimana biasanya digunakan dalam percakapan atau tulisan. Selanjutnya, gali asal-usul frasa tersebut, berikan konteks sejarah, referensi budaya, atau informasi etimologis yang menjelaskan bagaimana idiom atau peribahasa tersebut muncul. Jika ada cerita menarik, anekdot, atau teori yang terkait dengan asal-usulnya, sertakan juga. Bertujuan untuk memberikan pemahaman yang komprehensif tentang makna dan latar belakang idiom atau peribahasa tersebut.",
    messages=[{"role": "user", "content": [{"type": "text", "text": "Break a leg"}]}],
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
  system: "Tugas Anda adalah memberikan penjelasan yang jelas tentang makna dan asal-usul idiom dan peribahasa yang diberikan pengguna kepada Anda. Berikan interpretasi ringkas tentang makna kiasannya dan bagaimana biasanya digunakan dalam percakapan atau tulisan. Selanjutnya, gali asal-usul frasa tersebut, berikan konteks sejarah, referensi budaya, atau informasi etimologis yang menjelaskan bagaimana idiom atau peribahasa tersebut muncul. Jika ada cerita menarik, anekdot, atau teori yang terkait dengan asal-usulnya, sertakan juga. Bertujuan untuk memberikan pemahaman yang komprehensif tentang makna dan latar belakang idiom atau peribahasa tersebut.",
  messages: [
    {
      "role": "user",
      "content": [
        {
          "type": "text",
          "text": "Break a leg"
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
    system="Tugas Anda adalah memberikan penjelasan yang jelas tentang makna dan asal-usul idiom dan peribahasa yang diberikan pengguna kepada Anda. Berikan interpretasi ringkas tentang makna kiasannya dan bagaimana biasanya digunakan dalam percakapan atau tulisan. Selanjutnya, gali asal-usul frasa tersebut, berikan konteks sejarah, referensi budaya, atau informasi etimologis yang menjelaskan bagaimana idiom atau peribahasa tersebut muncul. Jika ada cerita menarik, anekdot, atau teori yang terkait dengan asal-usulnya, sertakan juga. Bertujuan untuk memberikan pemahaman yang komprehensif tentang makna dan latar belakang idiom atau peribahasa tersebut.",
    messages=[
        {
            "role": "user",
            "content": [
                {
                    "type": "text",
                    "text": "Break a leg"
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
  system: "Tugas Anda adalah memberikan penjelasan yang jelas tentang makna dan asal-usul idiom dan peribahasa yang diberikan pengguna kepada Anda. Berikan interpretasi ringkas tentang makna kiasannya dan bagaimana biasanya digunakan dalam percakapan atau tulisan. Selanjutnya, gali asal-usul frasa tersebut, berikan konteks sejarah, referensi budaya, atau informasi etimologis yang menjelaskan bagaimana idiom atau peribahasa tersebut muncul. Jika ada cerita menarik, anekdot, atau teori yang terkait dengan asal-usulnya, sertakan juga. Bertujuan untuk memberikan pemahaman yang komprehensif tentang makna dan latar belakang idiom atau peribahasa tersebut.",
  messages: [
    {
      "role": "user",
      "content": [
        {
          "type": "text",
          "text": "Break a leg"
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
    system="Tugas Anda adalah memberikan penjelasan yang jelas tentang makna dan asal-usul idiom dan peribahasa yang diberikan pengguna kepada Anda. Berikan interpretasi ringkas tentang makna kiasannya dan bagaimana biasanya digunakan dalam percakapan atau tulisan. Selanjutnya, gali asal-usul frasa tersebut, berikan konteks sejarah, referensi budaya, atau informasi etimologis yang menjelaskan bagaimana idiom atau peribahasa tersebut muncul. Jika ada cerita menarik, anekdot, atau teori yang terkait dengan asal-usulnya, sertakan juga. Bertujuan untuk memberikan pemahaman yang komprehensif tentang makna dan latar belakang idiom atau peribahasa tersebut.",
    messages=[
        {
            "role": "user",
            "content": [
                {
                    "type": "text",
                    "text": "Break a leg"
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
  system: "Tugas Anda adalah memberikan penjelasan yang jelas tentang makna dan asal-usul idiom dan peribahasa yang diberikan pengguna kepada Anda. Berikan interpretasi ringkas tentang makna kiasannya dan bagaimana biasanya digunakan dalam percakapan atau tulisan. Selanjutnya, gali asal-usul frasa tersebut, berikan konteks sejarah, referensi budaya, atau informasi etimologis yang menjelaskan bagaimana idiom atau peribahasa tersebut muncul. Jika ada cerita menarik, anekdot, atau teori yang terkait dengan asal-usulnya, sertakan juga. Bertujuan untuk memberikan pemahaman yang komprehensif tentang makna dan latar belakang idiom atau peribahasa tersebut.",
  messages: [
    {
      "role": "user",
      "content": [
        {
          "type": "text",
          "text": "Break a leg"
        }
      ]
    }
  ]
});
console.log(msg);

```

</Tab>
</Tabs>