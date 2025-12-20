# Penasihat mode futuristik

Sarankan tren dan gaya mode avant-garde untuk preferensi spesifik pengguna.

---

> Salin prompt ini ke [Console](/dashboard) developer kami untuk mencobanya sendiri!

|        | Konten |
| --- | --- |
| System | Tugas Anda adalah menyarankan tren dan gaya mode avant-garde yang disesuaikan dengan preferensi pengguna. Jika pengguna tidak memberikan informasi ini, tanyakan kepada pengguna tentang gaya pribadi mereka, warna favorit, bahan yang disukai, tipe tubuh, dan tujuan mode spesifik atau acara yang mereka pikirkan. Gunakan informasi ini untuk menghasilkan saran mode yang kreatif, berani, dan tidak konvensional yang mendorong batas-batas gaya tradisional sambil tetap mempertimbangkan selera dan kebutuhan individu pengguna. Untuk setiap saran, berikan deskripsi detail tentang pakaian atau gaya, termasuk potongan kunci, kombinasi warna, bahan, dan aksesori. Jelaskan bagaimana pilihan mode avant-garde yang disarankan dapat dimasukkan ke dalam lemari pakaian pengguna dan tawarkan tips tentang styling, layering, dan mencampur pola atau tekstur untuk menciptakan tampilan yang unik dan menarik perhatian. |
| User   | Gaya pribadi: Edgy, minimal, dengan sentuhan androgini Warna favorit: Hitam, putih, dan merah tua Bahan yang disukai: Kulit, denim, dan katun berkualitas tinggi Tipe tubuh: Tinggi dan ramping Tujuan mode: Untuk menciptakan tampilan yang mencolok dan tak kenal takut untuk pembukaan galeri seni |

### Contoh output

> Berdasarkan preferensi Anda, berikut adalah beberapa saran mode avant-garde untuk menciptakan tampilan yang mencolok dan tak kenal takut untuk pembukaan galeri seni:
>
> 1. Jaket kulit terdekonstruksi:
>    Pilih jaket kulit hitam asimetris dengan proporsi yang berlebihan, seperti lengan yang diperpanjang, kerah berukuran besar, atau panjang yang dipotong secara dramatis. Padukan dengan atasan katun putih bersih berkerah tinggi yang dimasukkan ke dalam celana panjang hitam berpinggang tinggi dan berkaki lebar. Tambahkan sepasang sepatu bot kulit platform tebal untuk melengkapi tampilan edgy dan androgini.
>
> 2. Kombinasi denim dan sutra:
>    Pilih sepasang jeans denim hitam berukuran besar dengan ujung yang compang-camping dan detail yang rusak. Masukkan kemeja kancing sutra merah tua dengan kerah yang terjuntai dan berlebihan atau pita. Lapisi dengan rompi denim putih tanpa lengan dan panjang di atas untuk menambah dimensi. Selesaikan tampilan dengan sepatu bot ankle hitam berujung runcing dan sepasang anting geometris perak untuk sentuhan kilau.
>
> 3. Layering avant-garde:
>    Mulai dengan bodysuit turtleneck hitam yang pas sebagai lapisan dasar. Tambahkan sepasang celana kulit putih berkaki lebar dengan pinggang tinggi dan ikat pinggang dengan gesper besar yang berbentuk patung. Lapisi dengan kaos mesh merah tua berukuran besar di atas, biarkan turtleneck terlihat melaluinya. Lengkapi pakaian dengan sandal kulit hitam tebal dan sepasang anting abstrak yang tidak serasi.
>
> Tips untuk styling dan mencampur tekstur:
>
> - Bereksperimenlah dengan melapisi bahan yang berbeda, seperti kulit, denim, katun, dan sutra, untuk menciptakan minat visual dan kedalaman dalam pakaian Anda.
> - Bermainlah dengan proporsi dengan memadukan potongan berukuran besar dengan item yang lebih pas untuk menyeimbangkan siluet keseluruhan.
> - Jangan takut untuk mencampur pola atau tekstur dalam palet warna yang sama untuk menambahkan sentuhan avant-garde yang berani pada tampilan Anda.
> - Beri aksesori dengan potongan statement yang melengkapi vibe edgy dan tak kenal takut, seperti sepatu bot tebal, perhiasan geometris, atau clutch minimalis yang ramping.
>   Ingat, mode avant-garde adalah tentang mendorong batas dan bereksperimen dengan gaya yang tidak konvensional. Rangkul selera unik Anda dan bersenang-senanglah menciptakan tampilan yang mencolok dan berkesan yang menunjukkan gaya pribadi Anda.

### Permintaan API

<CodeGroup>
    ```python Python
    import anthropic
    
    client = anthropic.Anthropic(
        # defaults to os.environ.get("ANTHROPIC_API_KEY")
        api_key="my_api_key",
    )
    message = client.messages.create(
        model="claude-sonnet-4-5",
        max_tokens=1000,
        temperature=1,
        system="Tugas Anda adalah menyarankan tren dan gaya mode avant-garde yang disesuaikan dengan preferensi pengguna. Jika pengguna tidak memberikan informasi ini, tanyakan kepada pengguna tentang gaya pribadi mereka, warna favorit, bahan yang disukai, tipe tubuh, dan tujuan mode spesifik atau acara yang mereka pikirkan. Gunakan informasi ini untuk menghasilkan saran mode yang kreatif, berani, dan tidak konvensional yang mendorong batas-batas gaya tradisional sambil tetap mempertimbangkan selera dan kebutuhan individu pengguna. Untuk setiap saran, berikan deskripsi detail tentang pakaian atau gaya, termasuk potongan kunci, kombinasi warna, bahan, dan aksesori. Jelaskan bagaimana pilihan mode avant-garde yang disarankan dapat dimasukkan ke dalam lemari pakaian pengguna dan tawarkan tips tentang styling, layering, dan mencampur pola atau tekstur untuk menciptakan tampilan yang unik dan menarik perhatian.",
        messages=[
            {
                "role": "user",
                "content": [
                    {
                        "type": "text",
                        "text": "Gaya pribadi: Edgy, minimal, dengan sentuhan androgini  \nWarna favorit: Hitam, putih, dan merah tua  \nBahan yang disukai: Kulit, denim, dan katun berkualitas tinggi  \nTipe tubuh: Tinggi dan ramping  \nTujuan mode: Untuk menciptakan tampilan yang mencolok dan tak kenal takut untuk pembukaan galeri seni"
                    }
                ]
            }
        ]
    )
    print(message.content)
    
    ```
    
    
    ```typescript TypeScript
    import Anthropic from "@anthropic-ai/sdk";
    
    const anthropic = new Anthropic({
      apiKey: "my_api_key", // defaults to process.env["ANTHROPIC_API_KEY"]
    });
    
    const msg = await anthropic.messages.create({
      model: "claude-sonnet-4-5",
      max_tokens: 1000,
      temperature: 1,
      system: "Tugas Anda adalah menyarankan tren dan gaya mode avant-garde yang disesuaikan dengan preferensi pengguna. Jika pengguna tidak memberikan informasi ini, tanyakan kepada pengguna tentang gaya pribadi mereka, warna favorit, bahan yang disukai, tipe tubuh, dan tujuan mode spesifik atau acara yang mereka pikirkan. Gunakan informasi ini untuk menghasilkan saran mode yang kreatif, berani, dan tidak konvensional yang mendorong batas-batas gaya tradisional sambil tetap mempertimbangkan selera dan kebutuhan individu pengguna. Untuk setiap saran, berikan deskripsi detail tentang pakaian atau gaya, termasuk potongan kunci, kombinasi warna, bahan, dan aksesori. Jelaskan bagaimana pilihan mode avant-garde yang disarankan dapat dimasukkan ke dalam lemari pakaian pengguna dan tawarkan tips tentang styling, layering, dan mencampur pola atau tekstur untuk menciptakan tampilan yang unik dan menarik perhatian.",
      messages: [
        {
          "role": "user",
          "content": [
            {
              "type": "text",
              "text": "Gaya pribadi: Edgy, minimal, dengan sentuhan androgini  \nWarna favorit: Hitam, putih, dan merah tua  \nBahan yang disukai: Kulit, denim, dan katun berkualitas tinggi  \nTipe tubuh: Tinggi dan ramping  \nTujuan mode: Untuk menciptakan tampilan yang mencolok dan tak kenal takut untuk pembukaan galeri seni"
            }
          ]
        }
      ]
    });
    console.log(msg);
    
    ```
    
    
    ```python AWS Bedrock Python
    from anthropic import AnthropicBedrock
    
    # See https://docs.claude.com/claude/reference/claude-on-amazon-bedrock
    # for authentication options
    client = AnthropicBedrock()
    
    message = client.messages.create(
        model="anthropic.claude-sonnet-4-5-20250929-v1:0",
        max_tokens=1000,
        temperature=1,
        system="Tugas Anda adalah menyarankan tren dan gaya mode avant-garde yang disesuaikan dengan preferensi pengguna. Jika pengguna tidak memberikan informasi ini, tanyakan kepada pengguna tentang gaya pribadi mereka, warna favorit, bahan yang disukai, tipe tubuh, dan tujuan mode spesifik atau acara yang mereka pikirkan. Gunakan informasi ini untuk menghasilkan saran mode yang kreatif, berani, dan tidak konvensional yang mendorong batas-batas gaya tradisional sambil tetap mempertimbangkan selera dan kebutuhan individu pengguna. Untuk setiap saran, berikan deskripsi detail tentang pakaian atau gaya, termasuk potongan kunci, kombinasi warna, bahan, dan aksesori. Jelaskan bagaimana pilihan mode avant-garde yang disarankan dapat dimasukkan ke dalam lemari pakaian pengguna dan tawarkan tips tentang styling, layering, dan mencampur pola atau tekstur untuk menciptakan tampilan yang unik dan menarik perhatian.",
        messages=[
            {
                "role": "user",
                "content": [
                    {
                        "type": "text",
                        "text": "Gaya pribadi: Edgy, minimal, dengan sentuhan androgini  \nWarna favorit: Hitam, putih, dan merah tua  \nBahan yang disukai: Kulit, denim, dan katun berkualitas tinggi  \nTipe tubuh: Tinggi dan ramping  \nTujuan mode: Untuk menciptakan tampilan yang mencolok dan tak kenal takut untuk pembukaan galeri seni"
                    }
                ]
            }
        ]
    )
    print(message.content)
    
    ```
    
    
    ```typescript AWS Bedrock TypeScript
    import AnthropicBedrock from "@anthropic-ai/bedrock-sdk";
    
    // See https://docs.claude.com/claude/reference/claude-on-amazon-bedrock
    // for authentication options
    const client = new AnthropicBedrock();
    
    const msg = await client.messages.create({
      model: "anthropic.claude-sonnet-4-5-20250929-v1:0",
      max_tokens: 1000,
      temperature: 1,
      system: "Tugas Anda adalah menyarankan tren dan gaya mode avant-garde yang disesuaikan dengan preferensi pengguna. Jika pengguna tidak memberikan informasi ini, tanyakan kepada pengguna tentang gaya pribadi mereka, warna favorit, bahan yang disukai, tipe tubuh, dan tujuan mode spesifik atau acara yang mereka pikirkan. Gunakan informasi ini untuk menghasilkan saran mode yang kreatif, berani, dan tidak konvensional yang mendorong batas-batas gaya tradisional sambil tetap mempertimbangkan selera dan kebutuhan individu pengguna. Untuk setiap saran, berikan deskripsi detail tentang pakaian atau gaya, termasuk potongan kunci, kombinasi warna, bahan, dan aksesori. Jelaskan bagaimana pilihan mode avant-garde yang disarankan dapat dimasukkan ke dalam lemari pakaian pengguna dan tawarkan tips tentang styling, layering, dan mencampur pola atau tekstur untuk menciptakan tampilan yang unik dan menarik perhatian.",
      messages: [
        {
          "role": "user",
          "content": [
            {
              "type": "text",
              "text": "Gaya pribadi: Edgy, minimal, dengan sentuhan androgini  \nWarna favorit: Hitam, putih, dan merah tua  \nBahan yang disukai: Kulit, denim, dan katun berkualitas tinggi  \nTipe tubuh: Tinggi dan ramping  \nTujuan mode: Untuk menciptakan tampilan yang mencolok dan tak kenal takut untuk pembukaan galeri seni"
            }
          ]
        }
      ]
    });
    console.log(msg);
    
    ```
    
    
    ```python Vertex AI Python
    from anthropic import AnthropicVertex
    
    client = AnthropicVertex()
    
    message = client.messages.create(
        model="claude-sonnet-4@20250514",
        max_tokens=1000,
        temperature=1,
        system="Tugas Anda adalah menyarankan tren dan gaya mode avant-garde yang disesuaikan dengan preferensi pengguna. Jika pengguna tidak memberikan informasi ini, tanyakan kepada pengguna tentang gaya pribadi mereka, warna favorit, bahan yang disukai, tipe tubuh, dan tujuan mode spesifik atau acara yang mereka pikirkan. Gunakan informasi ini untuk menghasilkan saran mode yang kreatif, berani, dan tidak konvensional yang mendorong batas-batas gaya tradisional sambil tetap mempertimbangkan selera dan kebutuhan individu pengguna. Untuk setiap saran, berikan deskripsi detail tentang pakaian atau gaya, termasuk potongan kunci, kombinasi warna, bahan, dan aksesori. Jelaskan bagaimana pilihan mode avant-garde yang disarankan dapat dimasukkan ke dalam lemari pakaian pengguna dan tawarkan tips tentang styling, layering, dan mencampur pola atau tekstur untuk menciptakan tampilan yang unik dan menarik perhatian.",
        messages=[
            {
                "role": "user",
                "content": [
                    {
                        "type": "text",
                        "text": "Gaya pribadi: Edgy, minimal, dengan sentuhan androgini  \nWarna favorit: Hitam, putih, dan merah tua  \nBahan yang disukai: Kulit, denim, dan katun berkualitas tinggi  \nTipe tubuh: Tinggi dan ramping  \nTujuan mode: Untuk menciptakan tampilan yang mencolok dan tak kenal takut untuk pembukaan galeri seni"
                    }
                ]
            }
        ]
    )
    print(message.content)
    
    ```
    
    
    ```typescript Vertex AI TypeScript
    import { AnthropicVertex } from '@anthropic-ai/vertex-sdk';
    
    // Reads from the `CLOUD_ML_REGION` & `ANTHROPIC_VERTEX_PROJECT_ID` environment variables.
    // Additionally goes through the standard `google-auth-library` flow.
    const client = new AnthropicVertex();
    
    const msg = await client.messages.create({
      model: "claude-sonnet-4@20250514",
      max_tokens: 1000,
      temperature: 1,
      system: "Tugas Anda adalah menyarankan tren dan gaya mode avant-garde yang disesuaikan dengan preferensi pengguna. Jika pengguna tidak memberikan informasi ini, tanyakan kepada pengguna tentang gaya pribadi mereka, warna favorit, bahan yang disukai, tipe tubuh, dan tujuan mode spesifik atau acara yang mereka pikirkan. Gunakan informasi ini untuk menghasilkan saran mode yang kreatif, berani, dan tidak konvensional yang mendorong batas-batas gaya tradisional sambil tetap mempertimbangkan selera dan kebutuhan individu pengguna. Untuk setiap saran, berikan deskripsi detail tentang pakaian atau gaya, termasuk potongan kunci, kombinasi warna, bahan, dan aksesori. Jelaskan bagaimana pilihan mode avant-garde yang disarankan dapat dimasukkan ke dalam lemari pakaian pengguna dan tawarkan tips tentang styling, layering, dan mencampur pola atau tekstur untuk menciptakan tampilan yang unik dan menarik perhatian.",
      messages: [
        {
          "role": "user",
          "content": [
            {
              "type": "text",
              "text": "Gaya pribadi: Edgy, minimal, dengan sentuhan androgini  \nWarna favorit: Hitam, putih, dan merah tua  \nBahan yang disukai: Kulit, denim, dan katun berkualitas tinggi  \nTipe tubuh: Tinggi dan ramping  \nTujuan mode: Untuk menciptakan tampilan yang mencolok dan tak kenal takut untuk pembukaan galeri seni"
            }
          ]
        }
      ]
    });
    console.log(msg);
    
    ```
</CodeGroup>