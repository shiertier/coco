# Guru penilaian

Membandingkan dan mengevaluasi kualitas teks tertulis berdasarkan kriteria dan standar yang ditentukan pengguna.

---

> Salin prompt ini ke [Console](/dashboard) developer kami untuk mencobanya sendiri!

| Role | Content |
| ---- | --- |
| User | Evaluasi kedua teks berikut berdasarkan kriteria yang diberikan: <br/> <br/> Teks 1: <br/> Matahari sedang terbenam di balik pegunungan, memancarkan cahaya keemasan yang hangat melintasi lembah. Burung-burung berkicau dengan lagu sore mereka saat angin sepoi-sepoi berdesir melalui dedaunan. Itu adalah momen yang damai dan tenang, yang memenuhi hati dengan kegembiraan dan kepuasan. <br/><br/> Teks 2: <br/> Matahari terbenam itu cantik. Burung-burung sedang bernyanyi dan angin sedang bertiup. Itu membuat saya bahagia. <br/><br/> Kriteria: <br/> 1. Bahasa deskriptif dan citra <br/> 2. Struktur kalimat dan variasi <br/> 3. Dampak emosional dan keterlibatan <br/> 4. Tata bahasa dan tanda baca |

## Contoh Output

Berdasarkan kriteria yang diberikan, berikut adalah evaluasi saya terhadap kedua teks:

Teks 1:

1.  Bahasa deskriptif dan citra: Teks menggunakan bahasa yang hidup dan menggugah untuk melukiskan gambaran latar. Frasa seperti "cahaya keemasan yang hangat," "burung-burung berkicau dengan lagu sore mereka," dan "angin sepoi-sepoi berdesir melalui dedaunan" menciptakan citra visual dan auditori yang kuat, menyelimuti pembaca dalam adegan tersebut.
2.  Struktur kalimat dan variasi: Teks menunjukkan campuran yang baik dari struktur kalimat, dengan kalimat sederhana dan majemuk. Variasi ini membantu mempertahankan alur yang lancar dan menjaga keterlibatan pembaca.
3.  Dampak emosional dan keterlibatan: Teks secara efektif menyampaikan rasa damai, kegembiraan, dan kepuasan, mengundang pembaca untuk berbagi dalam pengalaman emosional momen tersebut. Deskripsi latar dan penggunaan bahasa positif menciptakan koneksi emosional yang kuat.
4.  Tata bahasa dan tanda baca: Teks tampak secara tata bahasa benar dan tanda bacanya baik, tanpa kesalahan atau masalah yang jelas.

Teks 2:

1.  Bahasa deskriptif dan citra: Teks kekurangan bahasa deskriptif dan citra yang detail. Meskipun menyebutkan matahari terbenam, burung, dan angin, teks tidak memberikan detail yang hidup atau menggugah untuk membantu pembaca memvisualisasikan adegan.
2.  Struktur kalimat dan variasi: Teks terdiri dari kalimat pendek dan sederhana dengan sedikit variasi dalam struktur. Kurangnya variasi ini dapat membuat tulisan terasa monoton dan kurang menarik.
3.  Dampak emosional dan keterlibatan: Teks menyampaikan emosi positif dengan menyatakan "Itu membuat saya bahagia," tetapi tidak memberikan koneksi emosional yang kuat atau keterlibatan karena kurangnya bahasa deskriptif dan citra.
4.  Tata bahasa dan tanda baca: Teks tampak secara tata bahasa benar dan tanda bacanya baik, tanpa kesalahan atau masalah yang jelas.

Analisis Perbandingan:  
 Berdasarkan evaluasi, Teks 1 berkinerja secara signifikan lebih baik daripada Teks 2 dalam hal bahasa deskriptif, citra, struktur kalimat, variasi, dan dampak emosional. Teks 1 menciptakan pengalaman yang hidup dan imersif bagi pembaca dan menunjukkan kualitas tulisan yang lebih tinggi berdasarkan kriteria yang diberikan, dengan bahasa deskriptifnya yang kuat, struktur kalimat yang bervariasi, dan keterlibatan emosional yang efektif. Teks 2, meskipun secara tata bahasa benar, kekurangan kekayaan deskriptif dan dampak emosional yang ada dalam Teks 1.

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
    messages=[
        {
            "role": "user",
            "content": [
                {
                    "type": "text",
                    "text": "Evaluasi kedua teks berikut berdasarkan kriteria yang diberikan: \n \nTeks 1: \nMatahari sedang terbenam di balik pegunungan, memancarkan cahaya keemasan yang hangat melintasi lembah. Burung-burung berkicau dengan lagu sore mereka saat angin sepoi-sepoi berdesir melalui dedaunan. Itu adalah momen yang damai dan tenang, yang memenuhi hati dengan kegembiraan dan kepuasan. \n \nTeks 2: \nMatahari terbenam itu cantik. Burung-burung sedang bernyanyi dan angin sedang bertiup. Itu membuat saya bahagia. \n \nKriteria: \n1. Bahasa deskriptif dan citra \n2. Struktur kalimat dan variasi \n3. Dampak emosional dan keterlibatan \n4. Tata bahasa dan tanda baca",
                }
            ],
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
  messages: [
    {
      "role": "user",
      "content": [
        {
          "type": "text",
          "text": "Evaluasi kedua teks berikut berdasarkan kriteria yang diberikan:  \n  \nTeks 1:  \nMatahari sedang terbenam di balik pegunungan, memancarkan cahaya keemasan yang hangat melintasi lembah. Burung-burung berkicau dengan lagu sore mereka saat angin sepoi-sepoi berdesir melalui dedaunan. Itu adalah momen yang damai dan tenang, yang memenuhi hati dengan kegembiraan dan kepuasan.  \n  \nTeks 2:  \nMatahari terbenam itu cantik. Burung-burung sedang bernyanyi dan angin sedang bertiup. Itu membuat saya bahagia.  \n  \nKriteria:  \n1. Bahasa deskriptif dan citra  \n2. Struktur kalimat dan variasi  \n3. Dampak emosional dan keterlibatan  \n4. Tata bahasa dan tanda baca"
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
                    "text": "Evaluasi kedua teks berikut berdasarkan kriteria yang diberikan:  \n  \nTeks 1:  \nMatahari sedang terbenam di balik pegunungan, memancarkan cahaya keemasan yang hangat melintasi lembah. Burung-burung berkicau dengan lagu sore mereka saat angin sepoi-sepoi berdesir melalui dedaunan. Itu adalah momen yang damai dan tenang, yang memenuhi hati dengan kegembiraan dan kepuasan.  \n  \nTeks 2:  \nMatahari terbenam itu cantik. Burung-burung sedang bernyanyi dan angin sedang bertiup. Itu membuat saya bahagia.  \n  \nKriteria:  \n1. Bahasa deskriptif dan citra  \n2. Struktur kalimat dan variasi  \n3. Dampak emosional dan keterlibatan  \n4. Tata bahasa dan tanda baca"
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
  messages: [
    {
      "role": "user",
      "content": [
        {
          "type": "text",
          "text": "Evaluasi kedua teks berikut berdasarkan kriteria yang diberikan:  \n  \nTeks 1:  \nMatahari sedang terbenam di balik pegunungan, memancarkan cahaya keemasan yang hangat melintasi lembah. Burung-burung berkicau dengan lagu sore mereka saat angin sepoi-sepoi berdesir melalui dedaunan. Itu adalah momen yang damai dan tenang, yang memenuhi hati dengan kegembiraan dan kepuasan.  \n  \nTeks 2:  \nMatahari terbenam itu cantik. Burung-burung sedang bernyanyi dan angin sedang bertiup. Itu membuat saya bahagia.  \n  \nKriteria:  \n1. Bahasa deskriptif dan citra  \n2. Struktur kalimat dan variasi  \n3. Dampak emosional dan keterlibatan  \n4. Tata bahasa dan tanda baca"
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
    messages=[
        {
            "role": "user",
            "content": [
                {
                    "type": "text",
                    "text": "Evaluasi kedua teks berikut berdasarkan kriteria yang diberikan:  \n  \nTeks 1:  \nMatahari sedang terbenam di balik pegunungan, memancarkan cahaya keemasan yang hangat melintasi lembah. Burung-burung berkicau dengan lagu sore mereka saat angin sepoi-sepoi berdesir melalui dedaunan. Itu adalah momen yang damai dan tenang, yang memenuhi hati dengan kegembiraan dan kepuasan.  \n  \nTeks 2:  \nMatahari terbenam itu cantik. Burung-burung sedang bernyanyi dan angin sedang bertiup. Itu membuat saya bahagia.  \n  \nKriteria:  \n1. Bahasa deskriptif dan citra  \n2. Struktur kalimat dan variasi  \n3. Dampak emosional dan keterlibatan  \n4. Tata bahasa dan tanda baca"
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
          "text": "Evaluasi kedua teks berikut berdasarkan kriteria yang diberikan:  \n  \nTeks 1:  \nMatahari sedang terbenam di balik pegunungan, memancarkan cahaya keemasan yang hangat melintasi lembah. Burung-burung berkicau dengan lagu sore mereka saat angin sepoi-sepoi berdesir melalui dedaunan. Itu adalah momen yang damai dan tenang, yang memenuhi hati dengan kegembiraan dan kepuasan.  \n  \nTeks 2:  \nMatahari terbenam itu cantik. Burung-burung sedang bernyanyi dan angin sedang bertiup. Itu membuat saya bahagia.  \n  \nKriteria:  \n1. Bahasa deskriptif dan citra  \n2. Struktur kalimat dan variasi  \n3. Dampak emosional dan keterlibatan  \n4. Tata bahasa dan tanda baca"
        }
      ]
    }
  ]
});
console.log(msg);

```

</Tab>
</Tabs>