# Pengorganisir data

Ubah teks tidak terstruktur menjadi tabel JSON yang disesuaikan.

---

> Salin prompt ini ke [Console](/dashboard) developer kami untuk mencobanya sendiri!

|        | Konten |
| --- | --- |
| System | Tugas Anda adalah mengambil teks tidak terstruktur yang diberikan dan mengubahnya menjadi format tabel yang terorganisir dengan baik menggunakan JSON. Identifikasi entitas utama, atribut, atau kategori yang disebutkan dalam teks dan gunakan sebagai kunci dalam objek JSON. Kemudian, ekstrak informasi yang relevan dari teks dan isi nilai yang sesuai dalam objek JSON. Pastikan bahwa data direpresentasikan dengan akurat dan diformat dengan benar dalam struktur JSON. Tabel JSON yang dihasilkan harus memberikan gambaran yang jelas dan terstruktur dari informasi yang disajikan dalam teks asli. |
| User   | Silvermist Hollow, sebuah desa yang menawan, adalah rumah bagi sekelompok individu yang luar biasa. Di antara mereka adalah Dr. Liam Patel, seorang ahli bedah saraf berusia 45 tahun lulusan Yale yang merevolusi teknik bedah di pusat medis regional. Olivia Chen, pada usia 28, adalah seorang arsitek inovatif dari UC Berkeley yang mengubah lanskap desa dengan desain berkelanjutan dan menakjubkannya. Teater lokal dimeriahkan oleh simfoni yang memukau dari Ethan Kovacs, seorang musisi dan komposer berusia 72 tahun lulusan Juilliard. Isabella Torres, seorang koki otodidak dengan passion untuk bahan-bahan lokal, menciptakan sensasi kuliner dengan restoran farm-to-table-nya, yang menjadi destinasi wajib dikunjungi bagi para pecinta makanan. Individu-individu luar biasa ini, masing-masing dengan bakat yang berbeda, berkontribusi pada permadani kehidupan yang bersemangat di Silvermist Hollow. |

### Contoh output

```json
[
  {
    "name": "Dr. Liam Patel",
    "age": 45,
    "profession": "Ahli Bedah Saraf",
    "education": "Yale",
    "accomplishments": "Merevolusi teknik bedah di pusat medis regional"
  },
  {
    "name": "Olivia Chen",
    "age": 28,
    "profession": "Arsitek",
    "education": "UC Berkeley",
    "accomplishments": "Mengubah lanskap desa dengan desain berkelanjutan dan menakjubkan"
  },
  {
    "name": "Ethan Kovacs",
    "age": 72,
    "profession": "Musisi dan Komposer",
    "education": "Juilliard",
    "accomplishments": "Memeriahkan teater lokal dengan simfoni yang memukau"
  },
  {
    "name": "Isabella Torres",
    "age": null,
    "profession": "Koki",
    "education": "Otodidak",
    "accomplishments": "Menciptakan sensasi kuliner dengan restoran farm-to-table-nya, yang menjadi destinasi wajib dikunjungi bagi para pecinta makanan"
  }
]
```

---

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
        temperature=0,
        system="Tugas Anda adalah mengambil teks tidak terstruktur yang diberikan dan mengubahnya menjadi format tabel yang terorganisir dengan baik menggunakan JSON. Identifikasi entitas utama, atribut, atau kategori yang disebutkan dalam teks dan gunakan sebagai kunci dalam objek JSON. Kemudian, ekstrak informasi yang relevan dari teks dan isi nilai yang sesuai dalam objek JSON. Pastikan bahwa data direpresentasikan dengan akurat dan diformat dengan benar dalam struktur JSON. Tabel JSON yang dihasilkan harus memberikan gambaran yang jelas dan terstruktur dari informasi yang disajikan dalam teks asli.",
        messages=[
            {
                "role": "user",
                "content": [
                    {
                        "type": "text",
                        "text": "Silvermist Hollow, sebuah desa yang menawan, adalah rumah bagi sekelompok individu yang luar biasa. Di antara mereka adalah Dr. Liam Patel, seorang ahli bedah saraf berusia 45 tahun lulusan Yale yang merevolusi teknik bedah di pusat medis regional. Olivia Chen, pada usia 28, adalah seorang arsitek inovatif dari UC Berkeley yang mengubah lanskap desa dengan desain berkelanjutan dan menakjubkannya. Teater lokal dimeriahkan oleh simfoni yang memukau dari Ethan Kovacs, seorang musisi dan komposer berusia 72 tahun lulusan Juilliard. Isabella Torres, seorang koki otodidak dengan passion untuk bahan-bahan lokal, menciptakan sensasi kuliner dengan restoran farm-to-table-nya, yang menjadi destinasi wajib dikunjungi bagi para pecinta makanan. Individu-individu luar biasa ini, masing-masing dengan bakat yang berbeda, berkontribusi pada permadani kehidupan yang bersemangat di Silvermist Hollow."
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
      temperature: 0,
      system: "Tugas Anda adalah mengambil teks tidak terstruktur yang diberikan dan mengubahnya menjadi format tabel yang terorganisir dengan baik menggunakan JSON. Identifikasi entitas utama, atribut, atau kategori yang disebutkan dalam teks dan gunakan sebagai kunci dalam objek JSON. Kemudian, ekstrak informasi yang relevan dari teks dan isi nilai yang sesuai dalam objek JSON. Pastikan bahwa data direpresentasikan dengan akurat dan diformat dengan benar dalam struktur JSON. Tabel JSON yang dihasilkan harus memberikan gambaran yang jelas dan terstruktur dari informasi yang disajikan dalam teks asli.",
      messages: [
        {
          "role": "user",
          "content": [
            {
              "type": "text",
              "text": "Silvermist Hollow, sebuah desa yang menawan, adalah rumah bagi sekelompok individu yang luar biasa. Di antara mereka adalah Dr. Liam Patel, seorang ahli bedah saraf berusia 45 tahun lulusan Yale yang merevolusi teknik bedah di pusat medis regional. Olivia Chen, pada usia 28, adalah seorang arsitek inovatif dari UC Berkeley yang mengubah lanskap desa dengan desain berkelanjutan dan menakjubkannya. Teater lokal dimeriahkan oleh simfoni yang memukau dari Ethan Kovacs, seorang musisi dan komposer berusia 72 tahun lulusan Juilliard. Isabella Torres, seorang koki otodidak dengan passion untuk bahan-bahan lokal, menciptakan sensasi kuliner dengan restoran farm-to-table-nya, yang menjadi destinasi wajib dikunjungi bagi para pecinta makanan. Individu-individu luar biasa ini, masing-masing dengan bakat yang berbeda, berkontribusi pada permadani kehidupan yang bersemangat di Silvermist Hollow."
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
        temperature=0,
        system="Tugas Anda adalah mengambil teks tidak terstruktur yang diberikan dan mengubahnya menjadi format tabel yang terorganisir dengan baik menggunakan JSON. Identifikasi entitas utama, atribut, atau kategori yang disebutkan dalam teks dan gunakan sebagai kunci dalam objek JSON. Kemudian, ekstrak informasi yang relevan dari teks dan isi nilai yang sesuai dalam objek JSON. Pastikan bahwa data direpresentasikan dengan akurat dan diformat dengan benar dalam struktur JSON. Tabel JSON yang dihasilkan harus memberikan gambaran yang jelas dan terstruktur dari informasi yang disajikan dalam teks asli.",
        messages=[
            {
                "role": "user",
                "content": [
                    {
                        "type": "text",
                        "text": "Silvermist Hollow, sebuah desa yang menawan, adalah rumah bagi sekelompok individu yang luar biasa. Di antara mereka adalah Dr. Liam Patel, seorang ahli bedah saraf berusia 45 tahun lulusan Yale yang merevolusi teknik bedah di pusat medis regional. Olivia Chen, pada usia 28, adalah seorang arsitek inovatif dari UC Berkeley yang mengubah lanskap desa dengan desain berkelanjutan dan menakjubkannya. Teater lokal dimeriahkan oleh simfoni yang memukau dari Ethan Kovacs, seorang musisi dan komposer berusia 72 tahun lulusan Juilliard. Isabella Torres, seorang koki otodidak dengan passion untuk bahan-bahan lokal, menciptakan sensasi kuliner dengan restoran farm-to-table-nya, yang menjadi destinasi wajib dikunjungi bagi para pecinta makanan. Individu-individu luar biasa ini, masing-masing dengan bakat yang berbeda, berkontribusi pada permadani kehidupan yang bersemangat di Silvermist Hollow."
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
      temperature: 0,
      system: "Tugas Anda adalah mengambil teks tidak terstruktur yang diberikan dan mengubahnya menjadi format tabel yang terorganisir dengan baik menggunakan JSON. Identifikasi entitas utama, atribut, atau kategori yang disebutkan dalam teks dan gunakan sebagai kunci dalam objek JSON. Kemudian, ekstrak informasi yang relevan dari teks dan isi nilai yang sesuai dalam objek JSON. Pastikan bahwa data direpresentasikan dengan akurat dan diformat dengan benar dalam struktur JSON. Tabel JSON yang dihasilkan harus memberikan gambaran yang jelas dan terstruktur dari informasi yang disajikan dalam teks asli.",
      messages: [
        {
          "role": "user",
          "content": [
            {
              "type": "text",
              "text": "Silvermist Hollow, sebuah desa yang menawan, adalah rumah bagi sekelompok individu yang luar biasa. Di antara mereka adalah Dr. Liam Patel, seorang ahli bedah saraf berusia 45 tahun lulusan Yale yang merevolusi teknik bedah di pusat medis regional. Olivia Chen, pada usia 28, adalah seorang arsitek inovatif dari UC Berkeley yang mengubah lanskap desa dengan desain berkelanjutan dan menakjubkannya. Teater lokal dimeriahkan oleh simfoni yang memukau dari Ethan Kovacs, seorang musisi dan komposer berusia 72 tahun lulusan Juilliard. Isabella Torres, seorang koki otodidak dengan passion untuk bahan-bahan lokal, menciptakan sensasi kuliner dengan restoran farm-to-table-nya, yang menjadi destinasi wajib dikunjungi bagi para pecinta makanan. Individu-individu luar biasa ini, masing-masing dengan bakat yang berbeda, berkontribusi pada permadani kehidupan yang bersemangat di Silvermist Hollow."
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
        temperature=0,
        system="Tugas Anda adalah mengambil teks tidak terstruktur yang diberikan dan mengubahnya menjadi format tabel yang terorganisir dengan baik menggunakan JSON. Identifikasi entitas utama, atribut, atau kategori yang disebutkan dalam teks dan gunakan sebagai kunci dalam objek JSON. Kemudian, ekstrak informasi yang relevan dari teks dan isi nilai yang sesuai dalam objek JSON. Pastikan bahwa data direpresentasikan dengan akurat dan diformat dengan benar dalam struktur JSON. Tabel JSON yang dihasilkan harus memberikan gambaran yang jelas dan terstruktur dari informasi yang disajikan dalam teks asli.",
        messages=[
            {
                "role": "user",
                "content": [
                    {
                        "type": "text",
                        "text": "Silvermist Hollow, sebuah desa yang menawan, adalah rumah bagi sekelompok individu yang luar biasa. Di antara mereka adalah Dr. Liam Patel, seorang ahli bedah saraf berusia 45 tahun lulusan Yale yang merevolusi teknik bedah di pusat medis regional. Olivia Chen, pada usia 28, adalah seorang arsitek inovatif dari UC Berkeley yang mengubah lanskap desa dengan desain berkelanjutan dan menakjubkannya. Teater lokal dimeriahkan oleh simfoni yang memukau dari Ethan Kovacs, seorang musisi dan komposer berusia 72 tahun lulusan Juilliard. Isabella Torres, seorang koki otodidak dengan passion untuk bahan-bahan lokal, menciptakan sensasi kuliner dengan restoran farm-to-table-nya, yang menjadi destinasi wajib dikunjungi bagi para pecinta makanan. Individu-individu luar biasa ini, masing-masing dengan bakat yang berbeda, berkontribusi pada permadani kehidupan yang bersemangat di Silvermist Hollow."
                    }
                ]
            }
        ]
    )
    print(message.content)
    
    ```
    
    
    ```typescript Vertex AI Type
    import { AnthropicVertex } from '@anthropic-ai/vertex-sdk';
    
    // Reads from the `CLOUD_ML_REGION` & `ANTHROPIC_VERTEX_PROJECT_ID` environment variables.
    // Additionally goes through the standard `google-auth-library` flow.
    const client = new AnthropicVertex();
    
    const msg = await client.messages.create({
      model: "claude-sonnet-4@20250514",
      max_tokens: 1000,
      temperature: 0,
      system: "Tugas Anda adalah mengambil teks tidak terstruktur yang diberikan dan mengubahnya menjadi format tabel yang terorganisir dengan baik menggunakan JSON. Identifikasi entitas utama, atribut, atau kategori yang disebutkan dalam teks dan gunakan sebagai kunci dalam objek JSON. Kemudian, ekstrak informasi yang relevan dari teks dan isi nilai yang sesuai dalam objek JSON. Pastikan bahwa data direpresentasikan dengan akurat dan diformat dengan benar dalam struktur JSON. Tabel JSON yang dihasilkan harus memberikan gambaran yang jelas dan terstruktur dari informasi yang disajikan dalam teks asli.",
      messages: [
        {
          "role": "user",
          "content": [
            {
              "type": "text",
              "text": "Silvermist Hollow, sebuah desa yang menawan, adalah rumah bagi sekelompok individu yang luar biasa. Di antara mereka adalah Dr. Liam Patel, seorang ahli bedah saraf berusia 45 tahun lulusan Yale yang merevolusi teknik bedah di pusat medis regional. Olivia Chen, pada usia 28, adalah seorang arsitek inovatif dari UC Berkeley yang mengubah lanskap desa dengan desain berkelanjutan dan menakjubkannya. Teater lokal dimeriahkan oleh simfoni yang memukau dari Ethan Kovacs, seorang musisi dan komposer berusia 72 tahun lulusan Juilliard. Isabella Torres, seorang koki otodidak dengan passion untuk bahan-bahan lokal, menciptakan sensasi kuliner dengan restoran farm-to-table-nya, yang menjadi destinasi wajib dikunjungi bagi para pecinta makanan. Individu-individu luar biasa ini, masing-masing dengan bakat yang berbeda, berkontribusi pada permadani kehidupan yang bersemangat di Silvermist Hollow."
            }
          ]
        }
      ]
    });
    console.log(msg);
    
    ```
</CodeGroup>