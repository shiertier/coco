# Memo maestro

Menyusun memo perusahaan yang komprehensif berdasarkan poin-poin kunci.

---

> Salin prompt ini ke [Console](/dashboard) developer kami untuk mencobanya sendiri!

|        | Konten |
| --- | --- |
| System | Tugas Anda adalah menyusun memo perusahaan yang komprehensif berdasarkan poin-poin kunci yang diberikan. Memo harus ditulis dengan nada profesional, membahas semua informasi yang relevan dengan cara yang jelas dan ringkas. Gunakan format yang sesuai, seperti judul, subjudul, dan poin-poin, untuk mengorganisir konten secara efektif. Pastikan memo terstruktur dengan baik, koheren, dan mudah dipahami oleh audiens yang dituju. |
| User   | - Pengumuman program kesehatan karyawan baru: "Fit4Success" <br/> - Tujuan program: mempromosikan kesehatan fisik dan mental, mengurangi stres, dan meningkatkan produktivitas <br/> - Komponen: kelas kebugaran di tempat, pilihan makanan sehat, sumber daya kesehatan mental, dan workshop kesehatan <br/> - Kemitraan dengan gym lokal dan pusat kesehatan untuk keanggotaan diskon <br/> - Insentif untuk partisipasi: pelacak kebugaran, poin kesehatan, dan undian hadiah <br/> - Tanggal peluncuran program: 1 Juni 2023 <br/> - Proses pendaftaran: registrasi online melalui intranet perusahaan <br/> - Koordinator program: tim Sumber Daya Manusia dan Keterlibatan Karyawan |

## Contoh Output

Subjek: Memperkenalkan Fit4Success - Program Kesehatan Karyawan Baru Kami

Halo semuanya,

Saya sangat senang mengumumkan peluncuran program kesehatan karyawan baru kami, Fit4Success, yang dirancang untuk mempromosikan kesehatan fisik dan mental, mengurangi stres, dan meningkatkan produktivitas di antara anggota tim kami. Program ini akan resmi dimulai pada 1 Juni 2023.

Tujuan Program:

- Mendorong aktivitas fisik teratur dan pilihan gaya hidup sehat
- Menyediakan sumber daya dan dukungan untuk kesehatan mental dan manajemen stres
- Memupuk lingkungan kerja yang positif dan menarik yang memprioritaskan kesejahteraan karyawan
- Meningkatkan produktivitas keseluruhan dan kepuasan kerja

Komponen Program:

- Kelas kebugaran di tempat: sesi yoga, Pilates, dan latihan interval intensitas tinggi (HIIT)
- Pilihan makanan sehat: camilan dan makanan bergizi tersedia di kafetaria perusahaan
- Sumber daya kesehatan mental: akses ke layanan konseling, aplikasi meditasi, dan workshop penghilang stres
- Workshop kesehatan: seminar tentang topik seperti nutrisi, kebersihan tidur, dan keseimbangan kerja-hidup

Kemitraan dan Diskon:  
 Untuk lebih mendukung perjalanan kesehatan Anda, kami telah bermitra dengan gym lokal dan pusat kesehatan untuk menawarkan keanggotaan diskon khusus untuk karyawan kami. Manfaatkan tarif khusus ini untuk mengakses berbagai fasilitas kebugaran dan kelas.

Insentif untuk Partisipasi:  
 Untuk mendorong partisipasi aktif dalam program Fit4Success, kami akan menawarkan insentif berikut:

- Pelacak kebugaran untuk semua peserta untuk memantau kemajuan mereka dan menetapkan tujuan pribadi
- Poin kesehatan yang diperoleh melalui partisipasi program, dapat ditukar dengan hadiah tambahan
- Undian hadiah bulanan untuk peserta yang memenuhi tonggak kesehatan tertentu

Proses Pendaftaran:  
 Untuk mendaftar dalam program Fit4Success, silakan kunjungi intranet perusahaan kami dan lengkapi formulir registrasi online. Batas waktu pendaftaran adalah 25 Mei 2023. Jika Anda memiliki pertanyaan atau kekhawatiran tentang proses pendaftaran, silakan hubungi tim Sumber Daya Manusia kami.

Koordinasi Program:  
 Program Fit4Success akan dikoordinasikan bersama oleh tim Sumber Daya Manusia dan Keterlibatan Karyawan kami. Mereka akan tersedia untuk memberikan bimbingan, dukungan, dan sumber daya sepanjang perjalanan kesehatan Anda.

Kami sangat percaya bahwa berinvestasi dalam kesejahteraan karyawan kami sangat penting untuk kesuksesan dan pertumbuhan organisasi kami. Kami mendorong Anda untuk memanfaatkan kesempatan menarik ini dan bergabung dengan kami dalam menciptakan tempat kerja yang lebih sehat, lebih bahagia, dan lebih produktif.

Mari kita rangkul program Fit4Success dan bekerja sama menuju masa depan yang lebih sehat!

Salam terbaik,  
 \[Nama Anda\]  
 \[Jabatan Anda\]

---

## API Request

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
    temperature=1,
    system="Tugas Anda adalah menyusun memo perusahaan yang komprehensif berdasarkan poin-poin kunci yang diberikan. Memo harus ditulis dengan nada profesional, membahas semua informasi yang relevan dengan cara yang jelas dan ringkas. Gunakan format yang sesuai, seperti judul, subjudul, dan poin-poin, untuk mengorganisir konten secara efektif. Pastikan memo terstruktur dengan baik, koheren, dan mudah dipahami oleh audiens yang dituju.",
    messages=[
        {
            "role": "user",
            "content": [
                {
                    "type": "text",
                    "text": "- Pengumuman program kesehatan karyawan baru: \"Fit4Success\"  \n- Tujuan program: mempromosikan kesehatan fisik dan mental, mengurangi stres, dan meningkatkan produktivitas  \n- Komponen: kelas kebugaran di tempat, pilihan makanan sehat, sumber daya kesehatan mental, dan workshop kesehatan  \n- Kemitraan dengan gym lokal dan pusat kesehatan untuk keanggotaan diskon  \n- Insentif untuk partisipasi: pelacak kebugaran, poin kesehatan, dan undian hadiah  \n- Tanggal peluncuran program: 1 Juni 2023  \n- Proses pendaftaran: registrasi online melalui intranet perusahaan  \n- Koordinator program: tim Sumber Daya Manusia dan Keterlibatan Karyawan"
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
  temperature: 1,
  system: "Tugas Anda adalah menyusun memo perusahaan yang komprehensif berdasarkan poin-poin kunci yang diberikan. Memo harus ditulis dengan nada profesional, membahas semua informasi yang relevan dengan cara yang jelas dan ringkas. Gunakan format yang sesuai, seperti judul, subjudul, dan poin-poin, untuk mengorganisir konten secara efektif. Pastikan memo terstruktur dengan baik, koheren, dan mudah dipahami oleh audiens yang dituju.",
  messages: [
    {
      "role": "user",
      "content": [
        {
          "type": "text",
          "text": "- Pengumuman program kesehatan karyawan baru: \"Fit4Success\"  \n- Tujuan program: mempromosikan kesehatan fisik dan mental, mengurangi stres, dan meningkatkan produktivitas  \n- Komponen: kelas kebugaran di tempat, pilihan makanan sehat, sumber daya kesehatan mental, dan workshop kesehatan  \n- Kemitraan dengan gym lokal dan pusat kesehatan untuk keanggotaan diskon  \n- Insentif untuk partisipasi: pelacak kebugaran, poin kesehatan, dan undian hadiah  \n- Tanggal peluncuran program: 1 Juni 2023  \n- Proses pendaftaran: registrasi online melalui intranet perusahaan  \n- Koordinator program: tim Sumber Daya Manusia dan Keterlibatan Karyawan"
        }
      ]
    }
  ]
});
console.log(msg);

```

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
system="Tugas Anda adalah menyusun memo perusahaan yang komprehensif berdasarkan poin-poin kunci yang diberikan. Memo harus ditulis dengan nada profesional, membahas semua informasi yang relevan dengan cara yang jelas dan ringkas. Gunakan format yang sesuai, seperti judul, subjudul, dan poin-poin, untuk mengorganisir konten secara efektif. Pastikan memo terstruktur dengan baik, koheren, dan mudah dipahami oleh audiens yang dituju.",
messages=[
{
"role": "user",
"content": [
{
"type": "text",
"text": "- Pengumuman program kesehatan karyawan baru: \"Fit4Success\" \n- Tujuan program: mempromosikan kesehatan fisik dan mental, mengurangi stres, dan meningkatkan produktivitas \n- Komponen: kelas kebugaran di tempat, pilihan makanan sehat, sumber daya kesehatan mental, dan workshop kesehatan \n- Kemitraan dengan gym lokal dan pusat kesehatan untuk keanggotaan diskon \n- Insentif untuk partisipasi: pelacak kebugaran, poin kesehatan, dan undian hadiah \n- Tanggal peluncuran program: 1 Juni 2023 \n- Proses pendaftaran: registrasi online melalui intranet perusahaan \n- Koordinator program: tim Sumber Daya Manusia dan Keterlibatan Karyawan"
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
  system: "Tugas Anda adalah menyusun memo perusahaan yang komprehensif berdasarkan poin-poin kunci yang diberikan. Memo harus ditulis dengan nada profesional, membahas semua informasi yang relevan dengan cara yang jelas dan ringkas. Gunakan format yang sesuai, seperti judul, subjudul, dan poin-poin, untuk mengorganisir konten secara efektif. Pastikan memo terstruktur dengan baik, koheren, dan mudah dipahami oleh audiens yang dituju.",
  messages: [
    {
      "role": "user",
      "content": [
        {
          "type": "text",
          "text": "- Pengumuman program kesehatan karyawan baru: \"Fit4Success\"  \n- Tujuan program: mempromosikan kesehatan fisik dan mental, mengurangi stres, dan meningkatkan produktivitas  \n- Komponen: kelas kebugaran di tempat, pilihan makanan sehat, sumber daya kesehatan mental, dan workshop kesehatan  \n- Kemitraan dengan gym lokal dan pusat kesehatan untuk keanggotaan diskon  \n- Insentif untuk partisipasi: pelacak kebugaran, poin kesehatan, dan undian hadiah  \n- Tanggal peluncuran program: 1 Juni 2023  \n- Proses pendaftaran: registrasi online melalui intranet perusahaan  \n- Koordinator program: tim Sumber Daya Manusia dan Keterlibatan Karyawan"
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
    system="Tugas Anda adalah menyusun memo perusahaan yang komprehensif berdasarkan poin-poin kunci yang diberikan. Memo harus ditulis dengan nada profesional, membahas semua informasi yang relevan dengan cara yang jelas dan ringkas. Gunakan format yang sesuai, seperti judul, subjudul, dan poin-poin, untuk mengorganisir konten secara efektif. Pastikan memo terstruktur dengan baik, koheren, dan mudah dipahami oleh audiens yang dituju.",
    messages=[
        {
            "role": "user",
            "content": [
                {
                    "type": "text",
                    "text": "- Pengumuman program kesehatan karyawan baru: \"Fit4Success\"  \n- Tujuan program: mempromosikan kesehatan fisik dan mental, mengurangi stres, dan meningkatkan produktivitas  \n- Komponen: kelas kebugaran di tempat, pilihan makanan sehat, sumber daya kesehatan mental, dan workshop kesehatan  \n- Kemitraan dengan gym lokal dan pusat kesehatan untuk keanggotaan diskon  \n- Insentif untuk partisipasi: pelacak kebugaran, poin kesehatan, dan undian hadiah  \n- Tanggal peluncuran program: 1 Juni 2023  \n- Proses pendaftaran: registrasi online melalui intranet perusahaan  \n- Koordinator program: tim Sumber Daya Manusia dan Keterlibatan Karyawan"
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
  system: "Tugas Anda adalah menyusun memo perusahaan yang komprehensif berdasarkan poin-poin kunci yang diberikan. Memo harus ditulis dengan nada profesional, membahas semua informasi yang relevan dengan cara yang jelas dan ringkas. Gunakan format yang sesuai, seperti judul, subjudul, dan poin-poin, untuk mengorganisir konten secara efektif. Pastikan memo terstruktur dengan baik, koheren, dan mudah dipahami oleh audiens yang dituju.",
  messages: [
    {
      "role": "user",
      "content": [
        {
          "type": "text",
          "text": "- Pengumuman program kesehatan karyawan baru: \"Fit4Success\"  \n- Tujuan program: mempromosikan kesehatan fisik dan mental, mengurangi stres, dan meningkatkan produktivitas  \n- Komponen: kelas kebugaran di tempat, pilihan makanan sehat, sumber daya kesehatan mental, dan workshop kesehatan  \n- Kemitraan dengan gym lokal dan pusat kesehatan untuk keanggotaan diskon  \n- Insentif untuk partisipasi: pelacak kebugaran, poin kesehatan, dan undian hadiah  \n- Tanggal peluncuran program: 1 Juni 2023  \n- Proses pendaftaran: registrasi online melalui intranet perusahaan  \n- Koordinator program: tim Sumber Daya Manusia dan Keterlibatan Karyawan"
        }
      ]
    }
  ]
});
console.log(msg);

```

</Tab>
</Tabs>