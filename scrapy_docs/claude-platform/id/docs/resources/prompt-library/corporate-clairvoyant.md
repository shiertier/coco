# Peramal korporat

Ekstrak wawasan, identifikasi risiko, dan suling informasi kunci dari laporan korporat yang panjang menjadi satu memo.

---

> Salin prompt ini ke [Console](/dashboard) developer kami untuk mencobanya sendiri!

|      | Konten |
| ---- | --- |
| User | Tugas Anda adalah menganalisis laporan berikut: <br /> <br /> <report> [Teks lengkap dari [pengajuan SEC 10-K 2023 Matterport](https://investors.matterport.com/node/9501/html), tidak ditempel di sini untuk singkatnya] </report> <br /> <br /> Ringkas laporan tahunan ini dengan cara yang ringkas dan jelas, dan identifikasi tren pasar utama dan poin penting. Keluarkan temuan Anda sebagai memo singkat yang dapat saya kirim ke tim saya. Tujuan memo ini adalah untuk memastikan tim saya tetap terkini tentang bagaimana kinerja institusi keuangan dan secara kualitatif meramalkan dan mengidentifikasi apakah ada risiko operasional dan pendapatan yang diharapkan pada kuartal mendatang. Pastikan untuk menyertakan semua detail relevan dalam ringkasan dan analisis Anda. |

## Contoh Output

> Kepada: Tim Investasi  
> Dari: \[Nama Anda\]  
> Tanggal: 4 Maret 2024  
> Subjek: Sorotan 10-K 2022 Matterport dan Outlook Keuangan
>
> Hai tim,
>
> Saya telah mengidentifikasi beberapa poin penting dari laporan tahunan 2022 Matterport:
>
> Kinerja Keuangan:
>
> - Pendapatan meningkat 22% menjadi \$136,1M pada 2022, didorong oleh pertumbuhan pendapatan langganan dan layanan
> - Namun, margin kotor menurun dari 54% menjadi 38% karena peningkatan biaya dari pendapatan produk dan tantangan rantai pasokan
> - Kerugian operasional melebar menjadi \$275,5M karena perusahaan berinvestasi besar dalam R&D dan penjualan & pemasaran untuk mendorong pertumbuhan
> - Mengakhiri 2022 dengan \$476,9M dalam kas dan investasi untuk mendanai operasi
>
> Sorotan Bisnis:
>
> - Basis pelanggan tumbuh 39% menjadi lebih dari 701.000 pada akhir 2022. Ruang yang dikelola mencapai 9,2 juta
> - Memperluas platform dengan peluncuran mount bermotor Matterport Axis, kamera Pro3, dan pembaruan besar pada perangkat lunak cloud
> - Menyelesaikan akuisisi Enview (AI untuk data spasial 3D) dan VHT Studios (pemasaran real estat) untuk meningkatkan penawaran
> - Menandatangani kemitraan penting dengan Burns & McDonnell dan CGS untuk memberikan solusi Matterport kepada pelanggan perusahaan mereka
>
> Outlook:
>
> - Matterport melihat peluang pasar yang besar dalam digitalisasi bangunan dan ruang dunia (memperkirakan TAM \$240B+)
> - Fokus pada mendorong adopsi dengan pelanggan perusahaan besar di seluruh vertikal kunci seperti real estat, AEC, manajemen fasilitas, dan lainnya
> - Akan terus berinvestasi dalam R&D dan ekspansi internasional untuk menembus pasar bangunan global yang sebagian besar belum dimanfaatkan
> - Pertumbuhan dan margin jangka pendek mungkin terdampak oleh angin sakal makroekonomi dan masalah rantai pasokan
> - Namun, posisi likuiditas yang sehat dan basis pelanggan yang relatif lengket memberikan perlindungan downside
>
> Singkatnya, Matterport memberikan pertumbuhan top-line yang kuat pada 2022 karena membuat kemajuan dalam menembus pasar global yang besar. Meskipun profitabilitas tetap tertantang karena perusahaan berinvestasi besar untuk pertumbuhan, neraca keuangannya terlihat solid. Selain beberapa masalah rantai pasokan yang berkelanjutan, momentum bisnis yang mendasari tampak utuh berdasarkan metrik operasional kunci. Kecuali ada penurunan ekonomi yang parah, Matterport tampak berposisi baik untuk terus meraih pangsa di ruang digitalisasi bangunan yang baru berkembang.

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
    max_tokens=2000,
    temperature=0,
    messages=[
        {
            "role": "user",
            "content": [
                {
                    "type": "text",
                    "text": "Tugas Anda adalah menganalisis laporan berikut:  \n<report>  \n[Teks lengkap dari [pengajuan SEC 10-K 2023 Matterport](https://investors.matterport.com/node/9501/html), tidak ditempel di sini untuk singkatnya]  \n</report>  \n  \nRingkas laporan tahunan ini dengan cara yang ringkas dan jelas, dan identifikasi tren pasar utama dan poin penting. Keluarkan temuan Anda sebagai memo singkat yang dapat saya kirim ke tim saya. Tujuan memo ini adalah untuk memastikan tim saya tetap terkini tentang bagaimana kinerja institusi keuangan dan secara kualitatif meramalkan dan mengidentifikasi apakah ada risiko operasional dan pendapatan yang diharapkan pada kuartal mendatang. Pastikan untuk menyertakan semua detail relevan dalam ringkasan dan analisis Anda."
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
  temperature: 0,
  messages: [
    {
      "role": "user",
      "content": [
        {
          "type": "text",
          "text": "Tugas Anda adalah menganalisis laporan berikut:  \n<report>  \n[Teks lengkap dari [pengajuan SEC 10-K 2023 Matterport](https://investors.matterport.com/node/9501/html), tidak ditempel di sini untuk singkatnya]  \n</report>  \n  \nRingkas laporan tahunan ini dengan cara yang ringkas dan jelas, dan identifikasi tren pasar utama dan poin penting. Keluarkan temuan Anda sebagai memo singkat yang dapat saya kirim ke tim saya. Tujuan memo ini adalah untuk memastikan tim saya tetap terkini tentang bagaimana kinerja institusi keuangan dan secara kualitatif meramalkan dan mengidentifikasi apakah ada risiko operasional dan pendapatan yang diharapkan pada kuartal mendatang. Pastikan untuk menyertakan semua detail relevan dalam ringkasan dan analisis Anda."
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
messages=[
{
"role": "user",
"content": [
{
"type": "text",
"text": "Tugas Anda adalah menganalisis laporan berikut: \n<report> \n[Teks lengkap dari [pengajuan SEC 10-K 2023 Matterport](https://investors.matterport.com/node/9501/html), tidak ditempel di sini untuk singkatnya] \n</report> \n \nRingkas laporan tahunan ini dengan cara yang ringkas dan jelas, dan identifikasi tren pasar utama dan poin penting. Keluarkan temuan Anda sebagai memo singkat yang dapat saya kirim ke tim saya. Tujuan memo ini adalah untuk memastikan tim saya tetap terkini tentang bagaimana kinerja institusi keuangan dan secara kualitatif meramalkan dan mengidentifikasi apakah ada risiko operasional dan pendapatan yang diharapkan pada kuartal mendatang. Pastikan untuk menyertakan semua detail relevan dalam ringkasan dan analisis Anda."
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
  temperature: 0,
  messages: [
    {
      "role": "user",
      "content": [
        {
          "type": "text",
          "text": "Tugas Anda adalah menganalisis laporan berikut:  \n<report>  \n[Teks lengkap dari [pengajuan SEC 10-K 2023 Matterport](https://investors.matterport.com/node/9501/html), tidak ditempel di sini untuk singkatnya]  \n</report>  \n  \nRingkas laporan tahunan ini dengan cara yang ringkas dan jelas, dan identifikasi tren pasar utama dan poin penting. Keluarkan temuan Anda sebagai memo singkat yang dapat saya kirim ke tim saya. Tujuan memo ini adalah untuk memastikan tim saya tetap terkini tentang bagaimana kinerja institusi keuangan dan secara kualitatif meramalkan dan mengidentifikasi apakah ada risiko operasional dan pendapatan yang diharapkan pada kuartal mendatang. Pastikan untuk menyertakan semua detail relevan dalam ringkasan dan analisis Anda."
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
    temperature=0,
    messages=[
        {
            "role": "user",
            "content": [
                {
                    "type": "text",
                    "text": "Tugas Anda adalah menganalisis laporan berikut:  \n<report>  \n[Teks lengkap dari [pengajuan SEC 10-K 2023 Matterport](https://investors.matterport.com/node/9501/html), tidak ditempel di sini untuk singkatnya]  \n</report>  \n  \nRingkas laporan tahunan ini dengan cara yang ringkas dan jelas, dan identifikasi tren pasar utama dan poin penting. Keluarkan temuan Anda sebagai memo singkat yang dapat saya kirim ke tim saya. Tujuan memo ini adalah untuk memastikan tim saya tetap terkini tentang bagaimana kinerja institusi keuangan dan secara kualitatif meramalkan dan mengidentifikasi apakah ada risiko operasional dan pendapatan yang diharapkan pada kuartal mendatang. Pastikan untuk menyertakan semua detail relevan dalam ringkasan dan analisis Anda."
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
  temperature: 0,
  messages: [
    {
      "role": "user",
      "content": [
        {
          "type": "text",
          "text": "Tugas Anda adalah menganalisis laporan berikut:  \n<report>  \n[Teks lengkap dari [pengajuan SEC 10-K 2023 Matterport](https://investors.matterport.com/node/9501/html), tidak ditempel di sini untuk singkatnya]  \n</report>  \n  \nRingkas laporan tahunan ini dengan cara yang ringkas dan jelas, dan identifikasi tren pasar utama dan poin penting. Keluarkan temuan Anda sebagai memo singkat yang dapat saya kirim ke tim saya. Tujuan memo ini adalah untuk memastikan tim saya tetap terkini tentang bagaimana kinerja institusi keuangan dan secara kualitatif meramalkan dan mengidentifikasi apakah ada risiko operasional dan pendapatan yang diharapkan pada kuartal mendatang. Pastikan untuk menyertakan semua detail relevan dalam ringkasan dan analisis Anda."
        }
      ]
    }
  ]
});
console.log(msg);

```

  </Tab>
</Tabs>