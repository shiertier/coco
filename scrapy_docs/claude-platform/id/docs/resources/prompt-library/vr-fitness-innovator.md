# Inovator kebugaran VR

Brainstorm ide-ide kreatif untuk permainan kebugaran realitas virtual.

---

> Salin prompt ini ke [Console](/dashboard) developer kami untuk mencobanya sendiri!

| Peran | Konten |
| ---- | --- |
| User | Tugas Anda adalah menghasilkan daftar ide-ide inovatif dan menarik untuk permainan kebugaran realitas virtual (VR). Pertimbangkan berbagai genre permainan, mekanik gameplay yang unik, lingkungan yang imersif, dan cara-cara kreatif untuk menggabungkan latihan fisik dan gerakan. Ide-ide tersebut harus menarik bagi berbagai macam penggemar kebugaran dan gamer, mendorong mereka untuk tetap aktif dan bersenang-senang sambil berolahraga di VR. Untuk setiap ide, berikan deskripsi singkat tentang konsep permainan, fitur-fitur utama, dan bagaimana cara mempromosikan kebugaran. |

## Contoh Output

1.  Ide: Petualangan Parkour VR  
    Deskripsi: Pemain menavigasi melalui lingkungan perkotaan yang dinamis, berlari, melompat, memanjat, dan melakukan aksi parkour untuk mengatasi rintangan dan mencapai tujuan. Permainan beradaptasi dengan gerakan pemain, memberikan latihan seluruh tubuh yang intens sambil menjelajahi kota virtual.  
    Fitur utama: Mekanik parkour yang realistis, lingkungan yang dihasilkan secara prosedural, balapan multiplayer, dan tantangan.  
    Aspek kebugaran: Kardio, kelincahan, koordinasi, dan kekuatan tubuh bagian bawah.
2.  Ide: Rhythm Boxing  
    Deskripsi: Permainan tinju berenergi tinggi yang disetel mengikuti irama trek musik populer. Pemain harus memukul, menghindar, dan memblokir selaras dengan ritme untuk mencetak poin dan maju melalui level yang semakin menantang. Permainan menawarkan berbagai gaya tinju dan mode pelatihan.  
    Fitur utama: Musik berlisensi, petinju yang dapat disesuaikan, multiplayer online, dan beragam teknik tinju.  
    Aspek kebugaran: Kardio, kekuatan tubuh bagian atas, refleks, dan daya tahan.
3.  Ide: VR Fitness RPG  
    Deskripsi: Permainan role-playing yang imersif di mana pemain membuat karakter mereka sendiri dan memulai pencarian untuk menyelamatkan dunia fantasi. Permainan menggabungkan elemen RPG tradisional dengan tantangan kebugaran, mengharuskan pemain melakukan latihan fisik untuk melemparkan mantra, mengalahkan musuh, dan meningkatkan level karakter mereka.  
    Fitur utama: Kustomisasi karakter, pohon keterampilan, pertarungan bos epik, dan campuran latihan kekuatan, kardio, dan fleksibilitas.  
    Aspek kebugaran: Latihan seluruh tubuh, latihan kekuatan, kardio, dan fleksibilitas.

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
  temperature=1,
  messages=[
    {
      "role": "user",
      "content": [
        {
          "type": "text",
          "text": "Tugas Anda adalah menghasilkan daftar ide-ide inovatif dan menarik untuk permainan kebugaran realitas virtual (VR). Pertimbangkan berbagai genre permainan, mekanik gameplay yang unik, lingkungan yang imersif, dan cara-cara kreatif untuk menggabungkan latihan fisik dan gerakan. Ide-ide tersebut harus menarik bagi berbagai macam penggemar kebugaran dan gamer, mendorong mereka untuk tetap aktif dan bersenang-senang sambil berolahraga di VR. Untuk setiap ide, berikan deskripsi singkat tentang konsep permainan, fitur-fitur utama, dan bagaimana cara mempromosikan kebugaran."
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
  messages: [
    {
      "role": "user",
      "content": [
        {
          "type": "text",
          "text": "Tugas Anda adalah menghasilkan daftar ide-ide inovatif dan menarik untuk permainan kebugaran realitas virtual (VR). Pertimbangkan berbagai genre permainan, mekanik gameplay yang unik, lingkungan yang imersif, dan cara-cara kreatif untuk menggabungkan latihan fisik dan gerakan. Ide-ide tersebut harus menarik bagi berbagai macam penggemar kebugaran dan gamer, mendorong mereka untuk tetap aktif dan bersenang-senang sambil berolahraga di VR. Untuk setiap ide, berikan deskripsi singkat tentang konsep permainan, fitur-fitur utama, dan bagaimana cara mempromosikan kebugaran."
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
    messages=[
        {
            "role": "user",
            "content": [
                {
                    "type": "text",
                    "text": "Tugas Anda adalah menghasilkan daftar ide-ide inovatif dan menarik untuk permainan kebugaran realitas virtual (VR). Pertimbangkan berbagai genre permainan, mekanik gameplay yang unik, lingkungan yang imersif, dan cara-cara kreatif untuk menggabungkan latihan fisik dan gerakan. Ide-ide tersebut harus menarik bagi berbagai macam penggemar kebugaran dan gamer, mendorong mereka untuk tetap aktif dan bersenang-senang sambil berolahraga di VR. Untuk setiap ide, berikan deskripsi singkat tentang konsep permainan, fitur-fitur utama, dan bagaimana cara mempromosikan kebugaran."
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
  messages: [
    {
      "role": "user",
      "content": [
        {
          "type": "text",
          "text": "Tugas Anda adalah menghasilkan daftar ide-ide inovatif dan menarik untuk permainan kebugaran realitas virtual (VR). Pertimbangkan berbagai genre permainan, mekanik gameplay yang unik, lingkungan yang imersif, dan cara-cara kreatif untuk menggabungkan latihan fisik dan gerakan. Ide-ide tersebut harus menarik bagi berbagai macam penggemar kebugaran dan gamer, mendorong mereka untuk tetap aktif dan bersenang-senang sambil berolahraga di VR. Untuk setiap ide, berikan deskripsi singkat tentang konsep permainan, fitur-fitur utama, dan bagaimana cara mempromosikan kebugaran."
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
    messages=[
        {
            "role": "user",
            "content": [
                {
                    "type": "text",
                    "text": "Tugas Anda adalah menghasilkan daftar ide-ide inovatif dan menarik untuk permainan kebugaran realitas virtual (VR). Pertimbangkan berbagai genre permainan, mekanik gameplay yang unik, lingkungan yang imersif, dan cara-cara kreatif untuk menggabungkan latihan fisik dan gerakan. Ide-ide tersebut harus menarik bagi berbagai macam penggemar kebugaran dan gamer, mendorong mereka untuk tetap aktif dan bersenang-senang sambil berolahraga di VR. Untuk setiap ide, berikan deskripsi singkat tentang konsep permainan, fitur-fitur utama, dan bagaimana cara mempromosikan kebugaran."
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
  messages: [
    {
      "role": "user",
      "content": [
        {
          "type": "text",
          "text": "Tugas Anda adalah menghasilkan daftar ide-ide inovatif dan menarik untuk permainan kebugaran realitas virtual (VR). Pertimbangkan berbagai genre permainan, mekanik gameplay yang unik, lingkungan yang imersif, dan cara-cara kreatif untuk menggabungkan latihan fisik dan gerakan. Ide-ide tersebut harus menarik bagi berbagai macam penggemar kebugaran dan gamer, mendorong mereka untuk tetap aktif dan bersenang-senang sambil berolahraga di VR. Untuk setiap ide, berikan deskripsi singkat tentang konsep permainan, fitur-fitur utama, dan bagaimana cara mempromosikan kebugaran."
        }
      ]
    }
  ]
});
console.log(msg);

```

</Tab>
</Tabs>