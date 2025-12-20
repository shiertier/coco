# Pembuat pertanyaan wawancara

Menghasilkan pertanyaan untuk wawancara.

---

> Salin prompt ini ke [Console](/dashboard) developer kami untuk mencobanya sendiri!

|        | Konten |
| --- | --- |
| System | Tugas Anda adalah menghasilkan serangkaian pertanyaan yang bijaksana dan terbuka untuk wawancara berdasarkan konteks yang diberikan. Pertanyaan-pertanyaan tersebut harus dirancang untuk memunculkan respons yang mendalam dan terperinci dari pewawancara, memungkinkan mereka untuk menunjukkan pengetahuan, pengalaman, dan keterampilan berpikir kritis mereka. Hindari pertanyaan ya/tidak atau pertanyaan dengan jawaban yang jelas. Sebaliknya, fokuslah pada pertanyaan yang mendorong refleksi, penilaian diri, dan berbagi contoh atau anekdot spesifik. |
| User   | Anda sedang mewawancarai kandidat untuk posisi manajer pemasaran di perusahaan e-commerce yang sudah mapan. Kandidat ideal harus memiliki pengalaman dalam mengembangkan dan melaksanakan kampanye pemasaran multi-saluran, keterampilan analitis yang kuat, dan kemampuan untuk berkolaborasi secara efektif dengan tim lintas fungsi. Mereka juga harus bersemangat untuk tetap mengikuti tren dan teknologi pemasaran terbaru. |

## Contoh Output

1.  Bisakah Anda menceritakan kepada saya tentang kampanye pemasaran multi-saluran yang sukses yang Anda kembangkan dan laksanakan dalam peran sebelumnya? Apa tantangan yang Anda hadapi, dan bagaimana Anda mengatasinya?
2.  Bagaimana Anda mengukur keberhasilan kampanye pemasaran Anda, dan metrik apa yang Anda fokuskan? Bisakah Anda memberikan contoh bagaimana Anda menggunakan data dan analitik untuk mengoptimalkan kinerja kampanye?
3.  Berdasarkan pengalaman Anda, apa elemen kunci dari strategi pemasaran e-commerce yang sukses? Bagaimana Anda memastikan bahwa strategi Anda selaras dengan tujuan bisnis keseluruhan perusahaan?
4.  Bisakah Anda berbagi contoh saat Anda harus berkolaborasi dengan tim lintas fungsi (misalnya, produk, penjualan, atau layanan pelanggan) untuk mencapai tujuan pemasaran? Tantangan apa yang Anda temui, dan bagaimana Anda mendorong kolaborasi yang efektif?
5.  Bagaimana Anda tetap mendapat informasi tentang tren dan teknologi pemasaran terbaru, dan bagaimana Anda menerapkan pengetahuan ini untuk meningkatkan strategi atau kampanye pemasaran Anda?
6.  Bisakah Anda menjelaskan saat kampanye pemasaran yang Anda terlibat di dalamnya tidak berkinerja sesuai harapan? Apa yang Anda pelajari dari pengalaman tersebut, dan bagaimana Anda menerapkan pelajaran tersebut pada kampanye masa depan?
7.  Perusahaan kami menghargai inovasi dan perbaikan berkelanjutan. Bisakah Anda berbagi contoh saat Anda memperkenalkan pendekatan pemasaran, alat, atau teknologi baru yang secara signifikan berdampak pada kinerja atau hasil tim Anda?

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
  temperature=0.5,
  system="Tugas Anda adalah menghasilkan serangkaian pertanyaan yang bijaksana dan terbuka untuk wawancara berdasarkan konteks yang diberikan. Pertanyaan-pertanyaan tersebut harus dirancang untuk memunculkan respons yang mendalam dan terperinci dari pewawancara, memungkinkan mereka untuk menunjukkan pengetahuan, pengalaman, dan keterampilan berpikir kritis mereka. Hindari pertanyaan ya/tidak atau pertanyaan dengan jawaban yang jelas. Sebaliknya, fokuslah pada pertanyaan yang mendorong refleksi, penilaian diri, dan berbagi contoh atau anekdot spesifik.",
  messages=[
    {
      "role": "user",
      "content": [
        {
          "type": "text",
          "text": "Anda sedang mewawancarai kandidat untuk posisi manajer pemasaran di perusahaan e-commerce yang sudah mapan. Kandidat ideal harus memiliki pengalaman dalam mengembangkan dan melaksanakan kampanye pemasaran multi-saluran, keterampilan analitis yang kuat, dan kemampuan untuk berkolaborasi secara efektif dengan tim lintas fungsi. Mereka juga harus bersemangat untuk tetap mengikuti tren dan teknologi pemasaran terbaru."
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
  temperature: 0.5,
  system: "Tugas Anda adalah menghasilkan serangkaian pertanyaan yang bijaksana dan terbuka untuk wawancara berdasarkan konteks yang diberikan. Pertanyaan-pertanyaan tersebut harus dirancang untuk memunculkan respons yang mendalam dan terperinci dari pewawancara, memungkinkan mereka untuk menunjukkan pengetahuan, pengalaman, dan keterampilan berpikir kritis mereka. Hindari pertanyaan ya/tidak atau pertanyaan dengan jawaban yang jelas. Sebaliknya, fokuslah pada pertanyaan yang mendorong refleksi, penilaian diri, dan berbagi contoh atau anekdot spesifik.",
  messages: [
    {
      "role": "user",
      "content": [
        {
          "type": "text",
          "text": "Anda sedang mewawancarai kandidat untuk posisi manajer pemasaran di perusahaan e-commerce yang sudah mapan. Kandidat ideal harus memiliki pengalaman dalam mengembangkan dan melaksanakan kampanye pemasaran multi-saluran, keterampilan analitis yang kuat, dan kemampuan untuk berkolaborasi secara efektif dengan tim lintas fungsi. Mereka juga harus bersemangat untuk tetap mengikuti tren dan teknologi pemasaran terbaru."
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
    temperature=0.5,
    system="Tugas Anda adalah menghasilkan serangkaian pertanyaan yang bijaksana dan terbuka untuk wawancara berdasarkan konteks yang diberikan. Pertanyaan-pertanyaan tersebut harus dirancang untuk memunculkan respons yang mendalam dan terperinci dari pewawancara, memungkinkan mereka untuk menunjukkan pengetahuan, pengalaman, dan keterampilan berpikir kritis mereka. Hindari pertanyaan ya/tidak atau pertanyaan dengan jawaban yang jelas. Sebaliknya, fokuslah pada pertanyaan yang mendorong refleksi, penilaian diri, dan berbagi contoh atau anekdot spesifik.",
    messages=[
        {
            "role": "user",
            "content": [
                {
                    "type": "text",
                    "text": "Anda sedang mewawancarai kandidat untuk posisi manajer pemasaran di perusahaan e-commerce yang sudah mapan. Kandidat ideal harus memiliki pengalaman dalam mengembangkan dan melaksanakan kampanye pemasaran multi-saluran, keterampilan analitis yang kuat, dan kemampuan untuk berkolaborasi secara efektif dengan tim lintas fungsi. Mereka juga harus bersemangat untuk tetap mengikuti tren dan teknologi pemasaran terbaru."
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
temperature: 0.5,
system: "Tugas Anda adalah menghasilkan serangkaian pertanyaan yang bijaksana dan terbuka untuk wawancara berdasarkan konteks yang diberikan. Pertanyaan-pertanyaan tersebut harus dirancang untuk memunculkan respons yang mendalam dan terperinci dari pewawancara, memungkinkan mereka untuk menunjukkan pengetahuan, pengalaman, dan keterampilan berpikir kritis mereka. Hindari pertanyaan ya/tidak atau pertanyaan dengan jawaban yang jelas. Sebaliknya, fokuslah pada pertanyaan yang mendorong refleksi, penilaian diri, dan berbagi contoh atau anekdot spesifik.",
messages: [
{
"role": "user",
"content": [
{
"type": "text",
"text": "Anda sedang mewawancarai kandidat untuk posisi manajer pemasaran di perusahaan e-commerce yang sudah mapan. Kandidat ideal harus memiliki pengalaman dalam mengembangkan dan melaksanakan kampanye pemasaran multi-saluran, keterampilan analitis yang kuat, dan kemampuan untuk berkolaborasi secara efektif dengan tim lintas fungsi. Mereka juga harus bersemangat untuk tetap mengikuti tren dan teknologi pemasaran terbaru."
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
    max_tokens=1000,
    temperature=0.5,
    system="Tugas Anda adalah menghasilkan serangkaian pertanyaan yang bijaksana dan terbuka untuk wawancara berdasarkan konteks yang diberikan. Pertanyaan-pertanyaan tersebut harus dirancang untuk memunculkan respons yang mendalam dan terperinci dari pewawancara, memungkinkan mereka untuk menunjukkan pengetahuan, pengalaman, dan keterampilan berpikir kritis mereka. Hindari pertanyaan ya/tidak atau pertanyaan dengan jawaban yang jelas. Sebaliknya, fokuslah pada pertanyaan yang mendorong refleksi, penilaian diri, dan berbagi contoh atau anekdot spesifik.",
    messages=[
        {
            "role": "user",
            "content": [
                {
                    "type": "text",
                    "text": "Anda sedang mewawancarai kandidat untuk posisi manajer pemasaran di perusahaan e-commerce yang sudah mapan. Kandidat ideal harus memiliki pengalaman dalam mengembangkan dan melaksanakan kampanye pemasaran multi-saluran, keterampilan analitis yang kuat, dan kemampuan untuk berkolaborasi secara efektif dengan tim lintas fungsi. Mereka juga harus bersemangat untuk tetap mengikuti tren dan teknologi pemasaran terbaru."
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
  temperature: 0.5,
  system: "Tugas Anda adalah menghasilkan serangkaian pertanyaan yang bijaksana dan terbuka untuk wawancara berdasarkan konteks yang diberikan. Pertanyaan-pertanyaan tersebut harus dirancang untuk memunculkan respons yang mendalam dan terperinci dari pewawancara, memungkinkan mereka untuk menunjukkan pengetahuan, pengalaman, dan keterampilan berpikir kritis mereka. Hindari pertanyaan ya/tidak atau pertanyaan dengan jawaban yang jelas. Sebaliknya, fokuslah pada pertanyaan yang mendorong refleksi, penilaian diri, dan berbagi contoh atau anekdot spesifik.",
  messages: [
    {
      "role": "user",
      "content": [
        {
          "type": "text",
          "text": "Anda sedang mewawancarai kandidat untuk posisi manajer pemasaran di perusahaan e-commerce yang sudah mapan. Kandidat ideal harus memiliki pengalaman dalam mengembangkan dan melaksanakan kampanye pemasaran multi-saluran, keterampilan analitis yang kuat, dan kemampuan untuk berkolaborasi secara efektif dengan tim lintas fungsi. Mereka juga harus bersemangat untuk tetap mengikuti tren dan teknologi pemasaran terbaru."
        }
      ]
    }
  ]
});
console.log(msg);

```

</Tab>
</Tabs>