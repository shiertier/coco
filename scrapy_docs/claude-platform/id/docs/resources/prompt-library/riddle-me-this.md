# Teka-teki untukku

Buat teka-teki dan pandu pengguna menuju solusinya.

---

> Salin prompt ini ke [Console](/dashboard) developer kami untuk mencobanya sendiri!

| Role | Content |
| ---- | --- |
| User | Buat teka-teki yang cerdas dan berikan panduan langkah demi langkah untuk membantu pengguna mencapai solusi yang benar. Teka-teki harus menantang tetapi dapat dipecahkan dengan pemikiran logis dan perhatian terhadap detail. Setelah menyajikan setiap teka-teki, tawarkan serangkaian petunjuk atau pertanyaan yang secara progresif mengarahkan pengguna menuju jawaban. Pastikan bahwa petunjuk tidak terlalu jelas tetapi masih memberikan informasi yang cukup untuk memandu proses berpikir pengguna. Akhirnya, ungkapkan solusinya dan berikan penjelasan singkat tentang bagaimana teka-teki dapat dipecahkan menggunakan petunjuk yang diberikan. |

## Contoh Output

Saya tidak hidup, tetapi saya tumbuh; Saya tidak memiliki paru-paru, tetapi saya membutuhkan udara; Saya tidak memiliki mulut, tetapi air membunuh saya. Apa saya?

Petunjuk 1: Pertimbangkan berbagai keadaan materi dan bagaimana mereka dapat berubah.  
Petunjuk 2: Pikirkan tentang hubungan antara tiga elemen yang disebutkan: udara, air, dan objek misterius.  
Petunjuk 3: Jawabannya terkait dengan fenomena umum yang terjadi dalam kehidupan sehari-hari.

Solusi: Api.  
Penjelasan: Api tidak hidup, tetapi ia tumbuh dan menyebar saat mengonsumsi bahan bakar. Ia tidak memiliki paru-paru, tetapi memerlukan oksigen (udara) untuk mempertahankan dirinya. Meskipun tidak memiliki mulut, air sering digunakan untuk memadamkan api, secara efektif "membunuhnya".

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
  messages=[
    {
      "role": "user",
      "content": [
        {
          "type": "text",
          "text": "Buat teka-teki yang cerdas dan berikan panduan langkah demi langkah untuk membantu pengguna mencapai solusi yang benar. Teka-teki harus menantang tetapi dapat dipecahkan dengan pemikiran logis dan perhatian terhadap detail. Setelah menyajikan setiap teka-teki, tawarkan serangkaian petunjuk atau pertanyaan yang secara progresif mengarahkan pengguna menuju jawaban. Pastikan bahwa petunjuk tidak terlalu jelas tetapi masih memberikan informasi yang cukup untuk memandu proses berpikir pengguna. Akhirnya, ungkapkan solusinya dan berikan penjelasan singkat tentang bagaimana teka-teki dapat dipecahkan menggunakan petunjuk yang diberikan."
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
  messages: [
    {
      "role": "user",
      "content": [
        {
          "type": "text",
          "text": "Buat teka-teki yang cerdas dan berikan panduan langkah demi langkah untuk membantu pengguna mencapai solusi yang benar. Teka-teki harus menantang tetapi dapat dipecahkan dengan pemikiran logis dan perhatian terhadap detail. Setelah menyajikan setiap teka-teki, tawarkan serangkaian petunjuk atau pertanyaan yang secara progresif mengarahkan pengguna menuju jawaban. Pastikan bahwa petunjuk tidak terlalu jelas tetapi masih memberikan informasi yang cukup untuk memandu proses berpikir pengguna. Akhirnya, ungkapkan solusinya dan berikan penjelasan singkat tentang bagaimana teka-teki dapat dipecahkan menggunakan petunjuk yang diberikan."
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
                    "text": "Buat teka-teki yang cerdas dan berikan panduan langkah demi langkah untuk membantu pengguna mencapai solusi yang benar. Teka-teki harus menantang tetapi dapat dipecahkan dengan pemikiran logis dan perhatian terhadap detail. Setelah menyajikan setiap teka-teki, tawarkan serangkaian petunjuk atau pertanyaan yang secara progresif mengarahkan pengguna menuju jawaban. Pastikan bahwa petunjuk tidak terlalu jelas tetapi masih memberikan informasi yang cukup untuk memandu proses berpikir pengguna. Akhirnya, ungkapkan solusinya dan berikan penjelasan singkat tentang bagaimana teka-teki dapat dipecahkan menggunakan petunjuk yang diberikan."
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
          "text": "Buat teka-teki yang cerdas dan berikan panduan langkah demi langkah untuk membantu pengguna mencapai solusi yang benar. Teka-teki harus menantang tetapi dapat dipecahkan dengan pemikiran logis dan perhatian terhadap detail. Setelah menyajikan setiap teka-teki, tawarkan serangkaian petunjuk atau pertanyaan yang secara progresif mengarahkan pengguna menuju jawaban. Pastikan bahwa petunjuk tidak terlalu jelas tetapi masih memberikan informasi yang cukup untuk memandu proses berpikir pengguna. Akhirnya, ungkapkan solusinya dan berikan penjelasan singkat tentang bagaimana teka-teki dapat dipecahkan menggunakan petunjuk yang diberikan."
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
                    "text": "Buat teka-teki yang cerdas dan berikan panduan langkah demi langkah untuk membantu pengguna mencapai solusi yang benar. Teka-teki harus menantang tetapi dapat dipecahkan dengan pemikiran logis dan perhatian terhadap detail. Setelah menyajikan setiap teka-teki, tawarkan serangkaian petunjuk atau pertanyaan yang secara progresif mengarahkan pengguna menuju jawaban. Pastikan bahwa petunjuk tidak terlalu jelas tetapi masih memberikan informasi yang cukup untuk memandu proses berpikir pengguna. Akhirnya, ungkapkan solusinya dan berikan penjelasan singkat tentang bagaimana teka-teki dapat dipecahkan menggunakan petunjuk yang diberikan."
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
          "text": "Buat teka-teki yang cerdas dan berikan panduan langkah demi langkah untuk membantu pengguna mencapai solusi yang benar. Teka-teki harus menantang tetapi dapat dipecahkan dengan pemikiran logis dan perhatian terhadap detail. Setelah menyajikan setiap teka-teki, tawarkan serangkaian petunjuk atau pertanyaan yang secara progresif mengarahkan pengguna menuju jawaban. Pastikan bahwa petunjuk tidak terlalu jelas tetapi masih memberikan informasi yang cukup untuk memandu proses berpikir pengguna. Akhirnya, ungkapkan solusinya dan berikan penjelasan singkat tentang bagaimana teka-teki dapat dipecahkan menggunakan petunjuk yang diberikan."
        }
      ]
    }
  ]
});
console.log(msg);

```

</Tab>
</Tabs>