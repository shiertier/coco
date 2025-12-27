# Pencipta kuliner

Sarankan ide resep berdasarkan bahan-bahan yang tersedia dan preferensi diet pengguna.

---

> Salin prompt ini ke [Console](/dashboard) developer kami untuk mencobanya sendiri!

|        | Konten |
| --- | --- |
| System | Tugas Anda adalah menghasilkan ide resep yang dipersonalisasi berdasarkan input pengguna tentang bahan-bahan yang tersedia dan preferensi diet. Gunakan informasi ini untuk menyarankan berbagai resep kreatif dan lezat yang dapat dibuat menggunakan bahan-bahan yang diberikan sambil mengakomodasi kebutuhan diet pengguna, jika ada yang disebutkan. Untuk setiap resep, berikan deskripsi singkat, daftar bahan yang diperlukan, dan serangkaian instruksi sederhana. Pastikan bahwa resep mudah diikuti, bergizi, dan dapat disiapkan dengan bahan atau peralatan tambahan yang minimal. |
| User   | Bahan yang tersedia: dada ayam, brokoli, wortel, bawang bombay, bawang putih, minyak zaitun, beras, kecap asin bebas gluten, madu <br/> Preferensi diet: bebas gluten |

## Contoh Output

Resep yang disarankan:

Tumis Ayam Madu Bawang Putih

Deskripsi: Tumisan cepat dan beraroma yang menampilkan dada ayam yang lembut, sayuran renyah, dan saus madu bawang putih yang manis dan gurih.  
 Bahan-bahan:

- 2 dada ayam, potong menjadi potongan kecil
- 2 cangkir kuntum brokoli
- 1 cangkir wortel iris
- 1 bawang bombay, potong dadu
- 3 siung bawang putih, cincang halus
- 2 sdm minyak zaitun
- 1/4 cangkir kecap asin (bebas gluten)
- 2 sdm madu

Instruksi:

1.  Dalam wajan besar atau wok, panaskan minyak zaitun dengan api sedang-tinggi.
2.  Tambahkan bawang bombay potong dadu dan bawang putih cincang, tumis hingga harum, sekitar 2 menit.
3.  Tambahkan potongan ayam dan masak hingga tidak lagi berwarna merah muda, sekitar 5-7 menit.
4.  Tambahkan brokoli dan wortel, tumis hingga sayuran renyah-empuk, sekitar 5 menit.
5.  Dalam mangkuk kecil, kocok kecap asin dan madu.
6.  Tuang saus ke atas ayam dan sayuran, aduk hingga rata.
7.  Masak selama 2-3 menit tambahan, atau hingga saus mengental sedikit.
8.  Sajikan panas dengan nasi kukus.

Mangkuk Nasi Ayam dan Sayuran

Deskripsi: Mangkuk nasi yang mengenyangkan dan bergizi dengan topping ayam berbumbu, sayuran panggang, dan tetesan kecap asin.

Bahan-bahan:

- 2 dada ayam, potong menjadi potongan kecil
- 2 cangkir kuntum brokoli
- 1 cangkir wortel iris
- 1 bawang bombay, potong dadu
- 2 siung bawang putih, cincang halus
- 2 sdm minyak zaitun
- 1/4 cangkir kecap asin (bebas gluten)
- 2 cangkir nasi matang

Instruksi:

1.  Panaskan oven hingga 400°F (200°C).
2.  Dalam mangkuk besar, campurkan potongan ayam, brokoli, wortel, bawang bombay, dan bawang putih.
3.  Siram dengan minyak zaitun dan setengah dari kecap asin, aduk hingga rata.
4.  Sebarkan ayam dan sayuran di atas loyang dan panggang dalam oven yang sudah dipanaskan selama 20-25 menit, atau hingga ayam matang dan sayuran empuk.
5.  Bagi nasi matang ke dalam mangkuk saji.
6.  Beri topping setiap mangkuk dengan ayam dan sayuran panggang.
7.  Siram sisa kecap asin di atas setiap mangkuk.
8.  Sajikan panas dan nikmati!

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
  temperature=0.5,
  system="Tugas Anda adalah menghasilkan ide resep yang dipersonalisasi berdasarkan input pengguna tentang bahan-bahan yang tersedia dan preferensi diet. Gunakan informasi ini untuk menyarankan berbagai resep kreatif dan lezat yang dapat dibuat menggunakan bahan-bahan yang diberikan sambil mengakomodasi kebutuhan diet pengguna, jika ada yang disebutkan. Untuk setiap resep, berikan deskripsi singkat, daftar bahan yang diperlukan, dan serangkaian instruksi sederhana. Pastikan bahwa resep mudah diikuti, bergizi, dan dapat disiapkan dengan bahan atau peralatan tambahan yang minimal.",
  messages=[
    {
    "role": "user",
    "content": [
        {
          "type": "text",
          "text": "Bahan yang tersedia: dada ayam, brokoli, wortel, bawang bombay, bawang putih, minyak zaitun, beras, kecap asin bebas gluten, madu \nPreferensi diet: bebas gluten"
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
  temperature: 0.5,
  system: "Tugas Anda adalah menghasilkan ide resep yang dipersonalisasi berdasarkan input pengguna tentang bahan-bahan yang tersedia dan preferensi diet. Gunakan informasi ini untuk menyarankan berbagai resep kreatif dan lezat yang dapat dibuat menggunakan bahan-bahan yang diberikan sambil mengakomodasi kebutuhan diet pengguna, jika ada yang disebutkan. Untuk setiap resep, berikan deskripsi singkat, daftar bahan yang diperlukan, dan serangkaian instruksi sederhana. Pastikan bahwa resep mudah diikuti, bergizi, dan dapat disiapkan dengan bahan atau peralatan tambahan yang minimal.",
  messages: [
    {
      "role": "user",
      "content": [
        {
          "type": "text",
          "text": "Bahan yang tersedia: dada ayam, brokoli, wortel, bawang bombay, bawang putih, minyak zaitun, beras, kecap asin bebas gluten, madu  \nPreferensi diet: bebas gluten"
        }
      ]
    }
  ]
});
console.log(msg);

````

</Tab>
<Tab title="AWS Bedrock Python">

```
from anthropic import AnthropicBedrock

# See https://docs.claude.com/claude/reference/claude-on-amazon-bedrock
# for authentication options
client = AnthropicBedrock()

message = client.messages.create(
    model="anthropic.claude-sonnet-4-5-20250929-v1:0",
    max_tokens=2000,
    temperature=0.5,
    system="Tugas Anda adalah menghasilkan ide resep yang dipersonalisasi berdasarkan input pengguna tentang bahan-bahan yang tersedia dan preferensi diet. Gunakan informasi ini untuk menyarankan berbagai resep kreatif dan lezat yang dapat dibuat menggunakan bahan-bahan yang diberikan sambil mengakomodasi kebutuhan diet pengguna, jika ada yang disebutkan. Untuk setiap resep, berikan deskripsi singkat, daftar bahan yang diperlukan, dan serangkaian instruksi sederhana. Pastikan bahwa resep mudah diikuti, bergizi, dan dapat disiapkan dengan bahan atau peralatan tambahan yang minimal.",
    messages=[
        {
            "role": "user",
            "content": [
                {
                    "type": "text",
                    "text": "Bahan yang tersedia: dada ayam, brokoli, wortel, bawang bombay, bawang putih, minyak zaitun, beras, kecap asin bebas gluten, madu  \nPreferensi diet: bebas gluten"
                }
            ]
        }
    ]
)
print(message.content)

```

</Tab>
<Tab title="AWS Bedrock TypeScript">

```
import AnthropicBedrock from "@anthropic-ai/bedrock-sdk";

// See https://docs.claude.com/claude/reference/claude-on-amazon-bedrock
// for authentication options
const client = new AnthropicBedrock();

const msg = await client.messages.create({
  model: "anthropic.claude-sonnet-4-5-20250929-v1:0",
  max_tokens: 2000,
  temperature: 0.5,
  system: "Tugas Anda adalah menghasilkan ide resep yang dipersonalisasi berdasarkan input pengguna tentang bahan-bahan yang tersedia dan preferensi diet. Gunakan informasi ini untuk menyarankan berbagai resep kreatif dan lezat yang dapat dibuat menggunakan bahan-bahan yang diberikan sambil mengakomodasi kebutuhan diet pengguna, jika ada yang disebutkan. Untuk setiap resep, berikan deskripsi singkat, daftar bahan yang diperlukan, dan serangkaian instruksi sederhana. Pastikan bahwa resep mudah diikuti, bergizi, dan dapat disiapkan dengan bahan atau peralatan tambahan yang minimal.",
  messages: [
    {
      "role": "user",
      "content": [
        {
          "type": "text",
          "text": "Bahan yang tersedia: dada ayam, brokoli, wortel, bawang bombay, bawang putih, minyak zaitun, beras, kecap asin bebas gluten, madu  \nPreferensi diet: bebas gluten"
        }
      ]
    }
  ]
});
console.log(msg);

```

</Tab>
<Tab title="Vertex AI Python">

```
from anthropic import AnthropicVertex

client = AnthropicVertex()

message = client.messages.create(
    model="claude-sonnet-4@20250514",
    max_tokens=2000,
    temperature=0.5,
    system="Tugas Anda adalah menghasilkan ide resep yang dipersonalisasi berdasarkan input pengguna tentang bahan-bahan yang tersedia dan preferensi diet. Gunakan informasi ini untuk menyarankan berbagai resep kreatif dan lezat yang dapat dibuat menggunakan bahan-bahan yang diberikan sambil mengakomodasi kebutuhan diet pengguna, jika ada yang disebutkan. Untuk setiap resep, berikan deskripsi singkat, daftar bahan yang diperlukan, dan serangkaian instruksi sederhana. Pastikan bahwa resep mudah diikuti, bergizi, dan dapat disiapkan dengan bahan atau peralatan tambahan yang minimal.",
    messages=[
        {
            "role": "user",
            "content": [
                {
                    "type": "text",
                    "text": "Bahan yang tersedia: dada ayam, brokoli, wortel, bawang bombay, bawang putih, minyak zaitun, beras, kecap asin bebas gluten, madu  \nPreferensi diet: bebas gluten"
                }
            ]
        }
    ]
)
print(message.content)

```

</Tab>
<Tab title="Vertex AI TypeScript">

```
import { AnthropicVertex } from '@anthropic-ai/vertex-sdk';

// Reads from the `CLOUD_ML_REGION` & `ANTHROPIC_VERTEX_PROJECT_ID` environment variables.
// Additionally goes through the standard `google-auth-library` flow.
const client = new AnthropicVertex();

const msg = await client.messages.create({
  model: "claude-sonnet-4@20250514",
  max_tokens: 2000,
  temperature: 0.5,
  system: "Tugas Anda adalah menghasilkan ide resep yang dipersonalisasi berdasarkan input pengguna tentang bahan-bahan yang tersedia dan preferensi diet. Gunakan informasi ini untuk menyarankan berbagai resep kreatif dan lezat yang dapat dibuat menggunakan bahan-bahan yang diberikan sambil mengakomodasi kebutuhan diet pengguna, jika ada yang disebutkan. Untuk setiap resep, berikan deskripsi singkat, daftar bahan yang diperlukan, dan serangkaian instruksi sederhana. Pastikan bahwa resep mudah diikuti, bergizi, dan dapat disiapkan dengan bahan atau peralatan tambahan yang minimal.",
  messages: [
    {
      "role": "user",
      "content": [
        {
          "type": "text",
          "text": "Bahan yang tersedia: dada ayam, brokoli, wortel, bawang bombay, bawang putih, minyak zaitun, beras, kecap asin bebas gluten, madu  \nPreferensi diet: bebas gluten"
        }
      ]
    }
  ]
});
console.log(msg);

```

</Tab>
</Tabs>