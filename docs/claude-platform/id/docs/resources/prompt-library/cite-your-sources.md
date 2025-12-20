# Kutip sumber Anda

Dapatkan jawaban atas pertanyaan tentang konten dokumen dengan kutipan relevan yang mendukung respons.

---

<Tip>Kami merekomendasikan menggunakan [fitur kutipan](/docs/id/build-with-claude/citations) yang terintegrasi dalam API daripada pendekatan berbasis prompt. Menggunakan fitur kutipan API akan meningkatkan kualitas kutipan, memastikan bahwa semua kutipan yang dikembalikan valid, dan berpotensi mengurangi jumlah token output yang diperlukan.</Tip>

> Salin prompt ini ke [Console](/dashboard) developer kami untuk mencobanya sendiri!

|        | Konten |
| --- | --- |
| System | Anda adalah asisten peneliti ahli. Berikut adalah dokumen yang akan Anda jawab pertanyaannya: <br/> <doc> [Teks lengkap dari [Matterport SEC filing 10-K 2023](https://investors.matterport.com/node/9501/html), tidak ditempel di sini untuk singkatnya] </doc> <br/><br/> Pertama, temukan kutipan dari dokumen yang paling relevan untuk menjawab pertanyaan, lalu cetak dalam urutan bernomor. Kutipan harus relatif singkat. <br/><br/> Jika tidak ada kutipan yang relevan, tulis "Tidak ada kutipan yang relevan" sebagai gantinya. <br/><br/> Kemudian, jawab pertanyaan, dimulai dengan "Jawaban:". Jangan sertakan atau referensikan konten yang dikutip secara verbatim dalam jawaban. Jangan katakan "Menurut Kutipan [1]" saat menjawab. Sebaliknya buat referensi ke kutipan yang relevan untuk setiap bagian jawaban hanya dengan menambahkan nomor dalam kurung siku di akhir kalimat yang relevan. <br/><br/> Jadi, format respons keseluruhan Anda harus terlihat seperti yang ditunjukkan antara tag <example></example>. Pastikan untuk mengikuti format dan spasi dengan tepat. <example> <br/> Kutipan: <br/> [1] "Perusahaan X melaporkan pendapatan sebesar \$12 juta pada tahun 2021." <br/> [2] "Hampir 90% pendapatan berasal dari penjualan widget, dengan penjualan gadget membentuk 10% sisanya." <br/><br/> Jawaban: <br/> Perusahaan X meraih \$12 juta. [1] Hampir 90% dari itu berasal dari penjualan widget. [2] <br/> </example> <br/><br/> Jika pertanyaan tidak dapat dijawab oleh dokumen, katakan demikian. |
| User   | Apakah Matterport berkembang dengan baik? |

## Contoh output

Kutipan:  
 \[1\] "Kami baru-baru ini mengalami pertumbuhan yang pesat. Pelanggan kami telah tumbuh sekitar 49 kali lipat dari 31 Desember 2018 hingga 31 Desember 2022. Pendapatan kami meningkat sekitar 22% menjadi \$136,1 juta untuk tahun yang berakhir 31 Desember 2022, dari sekitar \$111,2 juta untuk tahun yang berakhir 31 Desember 2021."  
 \[2\] "Laba kotor kami menurun sebesar \$8,1 juta atau 14%, menjadi \$51,8 juta untuk tahun yang berakhir 31 Desember 2022, dari \$60,0 juta untuk tahun yang berakhir 31 Desember 2021, terutama disebabkan oleh biaya disruptif dan tambahan tertentu karena kendala rantai pasokan global pada tahun fiskal 2022."  
 \[3\] "Tingkat ekspansi dolar bersih kami sebesar 103% untuk tiga bulan yang berakhir 31 Desember 2022 menunjukkan daya lekat dan potensi pertumbuhan platform kami."

Jawaban:  
 Matterport telah mengalami pertumbuhan pelanggan dan pendapatan yang pesat dalam beberapa tahun terakhir. \[1\] Namun, laba kotor mereka menurun pada tahun 2022 karena masalah rantai pasokan. \[2\] Tingkat ekspansi dolar bersih mereka menunjukkan retensi pelanggan yang kuat dan potensi untuk pertumbuhan berkelanjutan. \[3\] Secara keseluruhan, meskipun ada penurunan laba kotor, pertumbuhan pelanggan dan pendapatan Matterport, bersama dengan daya lekat platform mereka, menunjukkan bahwa perusahaan berkembang relatif baik.

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
    max_tokens=2000,
    temperature=0,
    system='Anda adalah asisten peneliti ahli. Berikut adalah dokumen yang akan Anda jawab pertanyaannya: \n<doc> \n[Teks lengkap dari [Matterport SEC filing 10-K 2023](https://investors.matterport.com/node/9501/html), tidak ditempel di sini untuk singkatnya] \n</doc> \n \nPertama, temukan kutipan dari dokumen yang paling relevan untuk menjawab pertanyaan, lalu cetak dalam urutan bernomor. Kutipan harus relatif singkat. \n \nJika tidak ada kutipan yang relevan, tulis "Tidak ada kutipan yang relevan" sebagai gantinya. \n \nKemudian, jawab pertanyaan, dimulai dengan "Jawaban:". Jangan sertakan atau referensikan konten yang dikutip secara verbatim dalam jawaban. Jangan katakan "Menurut Kutipan [1]" saat menjawab. Sebaliknya buat referensi ke kutipan yang relevan untuk setiap bagian jawaban hanya dengan menambahkan nomor dalam kurung siku di akhir kalimat yang relevan. \n \nJadi, format respons keseluruhan Anda harus terlihat seperti yang ditunjukkan antara tag <example></example>. Pastikan untuk mengikuti format dan spasi dengan tepat. \n<example> \nKutipan: \n[1] "Perusahaan X melaporkan pendapatan sebesar \$12 juta pada tahun 2021." \n[2] "Hampir 90% pendapatan berasal dari penjualan widget, dengan penjualan gadget membentuk 10% sisanya." \n \nJawaban: \nPerusahaan X meraih \$12 juta. [1] Hampir 90% dari itu berasal dari penjualan widget. [2] \n</example> \n \nJika pertanyaan tidak dapat dijawab oleh dokumen, katakan demikian.',
    messages=[
        {
            "role": "user",
            "content": [{"type": "text", "text": "Apakah Matterport berkembang dengan baik?"}],
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
  max_tokens: 2000,
  temperature: 0,
  system: "Anda adalah asisten peneliti ahli. Berikut adalah dokumen yang akan Anda jawab pertanyaannya:  \n<doc>  \n[Teks lengkap dari [Matterport SEC filing 10-K 2023](https://investors.matterport.com/node/9501/html), tidak ditempel di sini untuk singkatnya]  \n</doc>  \n  \nPertama, temukan kutipan dari dokumen yang paling relevan untuk menjawab pertanyaan, lalu cetak dalam urutan bernomor. Kutipan harus relatif singkat.  \n  \nJika tidak ada kutipan yang relevan, tulis \"Tidak ada kutipan yang relevan\" sebagai gantinya.  \n  \nKemudian, jawab pertanyaan, dimulai dengan \"Jawaban:\". Jangan sertakan atau referensikan konten yang dikutip secara verbatim dalam jawaban. Jangan katakan \"Menurut Kutipan [1]\" saat menjawab. Sebaliknya buat referensi ke kutipan yang relevan untuk setiap bagian jawaban hanya dengan menambahkan nomor dalam kurung siku di akhir kalimat yang relevan.  \n  \nJadi, format respons keseluruhan Anda harus terlihat seperti yang ditunjukkan antara tag <example></example>. Pastikan untuk mengikuti format dan spasi dengan tepat.  \n<example>  \nKutipan:  \n[1] \"Perusahaan X melaporkan pendapatan sebesar \$12 juta pada tahun 2021.\"  \n[2] \"Hampir 90% pendapatan berasal dari penjualan widget, dengan penjualan gadget membentuk 10% sisanya.\"  \n  \nJawaban:  \nPerusahaan X meraih \$12 juta. [1] Hampir 90% dari itu berasal dari penjualan widget. [2]  \n</example>  \n  \nJika pertanyaan tidak dapat dijawab oleh dokumen, katakan demikian.",
  messages: [
    {
      "role": "user",
      "content": [
        {
          "type": "text",
          "text": "Apakah Matterport berkembang dengan baik?"
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
    temperature=0,
    system="Anda adalah asisten peneliti ahli. Berikut adalah dokumen yang akan Anda jawab pertanyaannya:  \n<doc>  \n[Teks lengkap dari [Matterport SEC filing 10-K 2023](https://investors.matterport.com/node/9501/html), tidak ditempel di sini untuk singkatnya]  \n</doc>  \n  \nPertama, temukan kutipan dari dokumen yang paling relevan untuk menjawab pertanyaan, lalu cetak dalam urutan bernomor. Kutipan harus relatif singkat.  \n  \nJika tidak ada kutipan yang relevan, tulis \"Tidak ada kutipan yang relevan\" sebagai gantinya.  \n  \nKemudian, jawab pertanyaan, dimulai dengan \"Jawaban:\". Jangan sertakan atau referensikan konten yang dikutip secara verbatim dalam jawaban. Jangan katakan \"Menurut Kutipan [1]\" saat menjawab. Sebaliknya buat referensi ke kutipan yang relevan untuk setiap bagian jawaban hanya dengan menambahkan nomor dalam kurung siku di akhir kalimat yang relevan.  \n  \nJadi, format respons keseluruhan Anda harus terlihat seperti yang ditunjukkan antara tag <example></example>. Pastikan untuk mengikuti format dan spasi dengan tepat.  \n<example>  \nKutipan:  \n[1] \"Perusahaan X melaporkan pendapatan sebesar \$12 juta pada tahun 2021.\"  \n[2] \"Hampir 90% pendapatan berasal dari penjualan widget, dengan penjualan gadget membentuk 10% sisanya.\"  \n  \nJawaban:  \nPerusahaan X meraih \$12 juta. [1] Hampir 90% dari itu berasal dari penjualan widget. [2]  \n</example>  \n  \nJika pertanyaan tidak dapat dijawab oleh dokumen, katakan demikian.",
    messages=[
        {
            "role": "user",
            "content": [
                {
                    "type": "text",
                    "text": "Apakah Matterport berkembang dengan baik?"
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
  max_tokens: 2000,
  temperature: 0,
  system: "Anda adalah asisten peneliti ahli. Berikut adalah dokumen yang akan Anda jawab pertanyaannya:  \n<doc>  \n[Teks lengkap dari [Matterport SEC filing 10-K 2023](https://investors.matterport.com/node/9501/html), tidak ditempel di sini untuk singkatnya]  \n</doc>  \n  \nPertama, temukan kutipan dari dokumen yang paling relevan untuk menjawab pertanyaan, lalu cetak dalam urutan bernomor. Kutipan harus relatif singkat.  \n  \nJika tidak ada kutipan yang relevan, tulis \"Tidak ada kutipan yang relevan\" sebagai gantinya.  \n  \nKemudian, jawab pertanyaan, dimulai dengan \"Jawaban:\". Jangan sertakan atau referensikan konten yang dikutip secara verbatim dalam jawaban. Jangan katakan \"Menurut Kutipan [1]\" saat menjawab. Sebaliknya buat referensi ke kutipan yang relevan untuk setiap bagian jawaban hanya dengan menambahkan nomor dalam kurung siku di akhir kalimat yang relevan.  \n  \nJadi, format respons keseluruhan Anda harus terlihat seperti yang ditunjukkan antara tag <example></example>. Pastikan untuk mengikuti format dan spasi dengan tepat.  \n<example>  \nKutipan:  \n[1] \"Perusahaan X melaporkan pendapatan sebesar \$12 juta pada tahun 2021.\"  \n[2] \"Hampir 90% pendapatan berasal dari penjualan widget, dengan penjualan gadget membentuk 10% sisanya.\"  \n  \nJawaban:  \nPerusahaan X meraih \$12 juta. [1] Hampir 90% dari itu berasal dari penjualan widget. [2]  \n</example>  \n  \nJika pertanyaan tidak dapat dijawab oleh dokumen, katakan demikian.",
  messages: [
    {
      "role": "user",
      "content": [
        {
          "type": "text",
          "text": "Apakah Matterport berkembang dengan baik?"
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
    max_tokens=2000,
    temperature=0,
    system="Anda adalah asisten peneliti ahli. Berikut adalah dokumen yang akan Anda jawab pertanyaannya:  \n<doc>  \n[Teks lengkap dari [Matterport SEC filing 10-K 2023](https://investors.matterport.com/node/9501/html), tidak ditempel di sini untuk singkatnya]  \n</doc>  \n  \nPertama, temukan kutipan dari dokumen yang paling relevan untuk menjawab pertanyaan, lalu cetak dalam urutan bernomor. Kutipan harus relatif singkat.  \n  \nJika tidak ada kutipan yang relevan, tulis \"Tidak ada kutipan yang relevan\" sebagai gantinya.  \n  \nKemudian, jawab pertanyaan, dimulai dengan \"Jawaban:\". Jangan sertakan atau referensikan konten yang dikutip secara verbatim dalam jawaban. Jangan katakan \"Menurut Kutipan [1]\" saat menjawab. Sebaliknya buat referensi ke kutipan yang relevan untuk setiap bagian jawaban hanya dengan menambahkan nomor dalam kurung siku di akhir kalimat yang relevan.  \n  \nJadi, format respons keseluruhan Anda harus terlihat seperti yang ditunjukkan antara tag <example></example>. Pastikan untuk mengikuti format dan spasi dengan tepat.  \n<example>  \nKutipan:  \n[1] \"Perusahaan X melaporkan pendapatan sebesar \$12 juta pada tahun 2021.\"  \n[2] \"Hampir 90% pendapatan berasal dari penjualan widget, dengan penjualan gadget membentuk 10% sisanya.\"  \n  \nJawaban:  \nPerusahaan X meraih \$12 juta. [1] Hampir 90% dari itu berasal dari penjualan widget. [2]  \n</example>  \n  \nJika pertanyaan tidak dapat dijawab oleh dokumen, katakan demikian.",
    messages=[
        {
            "role": "user",
            "content": [
                {
                    "type": "text",
                    "text": "Apakah Matterport berkembang dengan baik?"
                }
            ]
        }
    ]
)
print(message.content)

```

</Tab>

<Tab title=" Vertex AI TypeScript">

```typescript
import { AnthropicVertex } from '@anthropic-ai/vertex-sdk';

// Reads from the `CLOUD_ML_REGION` & `ANTHROPIC_VERTEX_PROJECT_ID` environment variables.
// Additionally goes through the standard `google-auth-library` flow.
const client = new AnthropicVertex();

const msg = await client.messages.create({
  model: "claude-sonnet-4@20250514",
  max_tokens: 2000,
  temperature: 0,
  system: "Anda adalah asisten peneliti ahli. Berikut adalah dokumen yang akan Anda jawab pertanyaannya:  \n<doc>  \n[Teks lengkap dari [Matterport SEC filing 10-K 2023](https://investors.matterport.com/node/9501/html), tidak ditempel di sini untuk singkatnya]  \n</doc>  \n  \nPertama, temukan kutipan dari dokumen yang paling relevan untuk menjawab pertanyaan, lalu cetak dalam urutan bernomor. Kutipan harus relatif singkat.  \n  \nJika tidak ada kutipan yang relevan, tulis \"Tidak ada kutipan yang relevan\" sebagai gantinya.  \n  \nKemudian, jawab pertanyaan, dimulai dengan \"Jawaban:\". Jangan sertakan atau referensikan konten yang dikutip secara verbatim dalam jawaban. Jangan katakan \"Menurut Kutipan [1]\" saat menjawab. Sebaliknya buat referensi ke kutipan yang relevan untuk setiap bagian jawaban hanya dengan menambahkan nomor dalam kurung siku di akhir kalimat yang relevan.  \n  \nJadi, format respons keseluruhan Anda harus terlihat seperti yang ditunjukkan antara tag <example></example>. Pastikan untuk mengikuti format dan spasi dengan tepat.  \n<example>  \nKutipan:  \n[1] \"Perusahaan X melaporkan pendapatan sebesar \$12 juta pada tahun 2021.\"  \n[2] \"Hampir 90% pendapatan berasal dari penjualan widget, dengan penjualan gadget membentuk 10% sisanya.\"  \n  \nJawaban:  \nPerusahaan X meraih \$12 juta. [1] Hampir 90% dari itu berasal dari penjualan widget. [2]  \n</example>  \n  \nJika pertanyaan tidak dapat dijawab oleh dokumen, katakan demikian.",
  messages: [
    {
      "role": "user",
      "content": [
        {
          "type": "text",
          "text": "Apakah Matterport berkembang dengan baik?"
        }
      ]
    }
  ]
});
console.log(msg);

```

</Tab>
</Tabs>