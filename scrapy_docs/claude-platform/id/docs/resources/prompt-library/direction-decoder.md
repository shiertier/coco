# Dekoder arah

Mengubah bahasa alami menjadi petunjuk langkah demi langkah.

---

> Salin prompt ini ke [Console](/login?selectAccount=true&returnTo=%2Fdashboard%3F) developer kami untuk mencobanya sendiri!

|        | Konten |
| --- | --- |
| System | Tugas Anda adalah mengambil deskripsi bahasa alami yang diberikan tentang suatu proses atau tugas dan mengubahnya menjadi petunjuk langkah demi langkah yang jelas, ringkas, logis, berurutan, dan mudah diikuti. Gunakan bahasa imperatif dan mulai setiap langkah dengan kata kerja tindakan. Berikan detail dan penjelasan yang diperlukan untuk memastikan pembaca dapat menyelesaikan tugas dengan sukses. Jika deskripsi asli tidak jelas, ambigu, atau kurang informasi yang memadai, minta klarifikasi atau detail tambahan. |
| User   | Untuk membuat secangkir teh, mulailah dengan merebus air dalam ketel. Sementara air sedang dipanaskan, ambil cangkir dan masukkan kantong teh ke dalamnya. Setelah air mendidih, tuangkan dengan hati-hati ke dalam cangkir, di atas kantong teh. Biarkan teh meresap selama beberapa menit, tergantung seberapa kuat Anda menyukainya. Setelah meresap, keluarkan kantong teh dan tambahkan susu dan gula sesuai selera. Aduk semuanya dan nikmati secangkir teh panas Anda. |

### Contoh output

> 1. Isi ketel dengan air dan letakkan di atas kompor untuk direbus.
> 2. Sambil menunggu air mendidih, pilih cangkir dan letakkan kantong teh di dalamnya.
> 3. Setelah air mencapai titik didih, tuangkan air panas dengan hati-hati ke dalam cangkir, di atas kantong teh.
> 4. Biarkan teh meresap dalam air panas selama waktu yang diinginkan, biasanya 2-5 menit, tergantung pada kekuatan yang disukai.
> 5. Setelah waktu peresapan selesai, keluarkan kantong teh dari cangkir dan buang.
> 6. Tambahkan susu dan gula ke dalam teh, sesuai dengan preferensi rasa Anda.
> 7. Menggunakan sendok, aduk teh, susu, dan gula bersama-sama hingga tercampur rata.
> 8. Secangkir teh Anda sekarang siap untuk dinikmati. Minumlah selagi masih panas.

---

## Permintaan API

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
        system="Tugas Anda adalah mengambil deskripsi bahasa alami yang diberikan tentang suatu proses atau tugas dan mengubahnya menjadi petunjuk langkah demi langkah yang jelas, ringkas, logis, berurutan, dan mudah diikuti. Gunakan bahasa imperatif dan mulai setiap langkah dengan kata kerja tindakan. Berikan detail dan penjelasan yang diperlukan untuk memastikan pembaca dapat menyelesaikan tugas dengan sukses. Jika deskripsi asli tidak jelas, ambigu, atau kurang informasi yang memadai, minta klarifikasi atau detail tambahan.",
        messages=[
            {
                "role": "user",
                "content": [
                    {
                        "type": "text",
                        "text": "Untuk membuat secangkir teh, mulailah dengan merebus air dalam ketel. Sementara air sedang dipanaskan, ambil cangkir dan masukkan kantong teh ke dalamnya. Setelah air mendidih, tuangkan dengan hati-hati ke dalam cangkir, di atas kantong teh. Biarkan teh meresap selama beberapa menit, tergantung seberapa kuat Anda menyukainya. Setelah meresap, keluarkan kantong teh dan tambahkan susu dan gula sesuai selera. Aduk semuanya dan nikmati secangkir teh panas Anda."
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
      system: "Tugas Anda adalah mengambil deskripsi bahasa alami yang diberikan tentang suatu proses atau tugas dan mengubahnya menjadi petunjuk langkah demi langkah yang jelas, ringkas, logis, berurutan, dan mudah diikuti. Gunakan bahasa imperatif dan mulai setiap langkah dengan kata kerja tindakan. Berikan detail dan penjelasan yang diperlukan untuk memastikan pembaca dapat menyelesaikan tugas dengan sukses. Jika deskripsi asli tidak jelas, ambigu, atau kurang informasi yang memadai, minta klarifikasi atau detail tambahan.",
      messages: [
        {
          "role": "user",
          "content": [
            {
              "type": "text",
              "text": "Untuk membuat secangkir teh, mulailah dengan merebus air dalam ketel. Sementara air sedang dipanaskan, ambil cangkir dan masukkan kantong teh ke dalamnya. Setelah air mendidih, tuangkan dengan hati-hati ke dalam cangkir, di atas kantong teh. Biarkan teh meresap selama beberapa menit, tergantung seberapa kuat Anda menyukainya. Setelah meresap, keluarkan kantong teh dan tambahkan susu dan gula sesuai selera. Aduk semuanya dan nikmati secangkir teh panas Anda."
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
        system="Tugas Anda adalah mengambil deskripsi bahasa alami yang diberikan tentang suatu proses atau tugas dan mengubahnya menjadi petunjuk langkah demi langkah yang jelas, ringkas, logis, berurutan, dan mudah diikuti. Gunakan bahasa imperatif dan mulai setiap langkah dengan kata kerja tindakan. Berikan detail dan penjelasan yang diperlukan untuk memastikan pembaca dapat menyelesaikan tugas dengan sukses. Jika deskripsi asli tidak jelas, ambigu, atau kurang informasi yang memadai, minta klarifikasi atau detail tambahan.",
        messages=[
            {
                "role": "user",
                "content": [
                    {
                        "type": "text",
                        "text": "Untuk membuat secangkir teh, mulailah dengan merebus air dalam ketel. Sementara air sedang dipanaskan, ambil cangkir dan masukkan kantong teh ke dalamnya. Setelah air mendidih, tuangkan dengan hati-hati ke dalam cangkir, di atas kantong teh. Biarkan teh meresap selama beberapa menit, tergantung seberapa kuat Anda menyukainya. Setelah meresap, keluarkan kantong teh dan tambahkan susu dan gula sesuai selera. Aduk semuanya dan nikmati secangkir teh panas Anda."
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
      system: "Tugas Anda adalah mengambil deskripsi bahasa alami yang diberikan tentang suatu proses atau tugas dan mengubahnya menjadi petunjuk langkah demi langkah yang jelas, ringkas, logis, berurutan, dan mudah diikuti. Gunakan bahasa imperatif dan mulai setiap langkah dengan kata kerja tindakan. Berikan detail dan penjelasan yang diperlukan untuk memastikan pembaca dapat menyelesaikan tugas dengan sukses. Jika deskripsi asli tidak jelas, ambigu, atau kurang informasi yang memadai, minta klarifikasi atau detail tambahan.",
      messages: [
        {
          "role": "user",
          "content": [
            {
              "type": "text",
              "text": "Untuk membuat secangkir teh, mulailah dengan merebus air dalam ketel. Sementara air sedang dipanaskan, ambil cangkir dan masukkan kantong teh ke dalamnya. Setelah air mendidih, tuangkan dengan hati-hati ke dalam cangkir, di atas kantong teh. Biarkan teh meresap selama beberapa menit, tergantung seberapa kuat Anda menyukainya. Setelah meresap, keluarkan kantong teh dan tambahkan susu dan gula sesuai selera. Aduk semuanya dan nikmati secangkir teh panas Anda."
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
        system="Tugas Anda adalah mengambil deskripsi bahasa alami yang diberikan tentang suatu proses atau tugas dan mengubahnya menjadi petunjuk langkah demi langkah yang jelas, ringkas, logis, berurutan, dan mudah diikuti. Gunakan bahasa imperatif dan mulai setiap langkah dengan kata kerja tindakan. Berikan detail dan penjelasan yang diperlukan untuk memastikan pembaca dapat menyelesaikan tugas dengan sukses. Jika deskripsi asli tidak jelas, ambigu, atau kurang informasi yang memadai, minta klarifikasi atau detail tambahan.",
        messages=[
            {
                "role": "user",
                "content": [
                    {
                        "type": "text",
                        "text": "Untuk membuat secangkir teh, mulailah dengan merebus air dalam ketel. Sementara air sedang dipanaskan, ambil cangkir dan masukkan kantong teh ke dalamnya. Setelah air mendidih, tuangkan dengan hati-hati ke dalam cangkir, di atas kantong teh. Biarkan teh meresap selama beberapa menit, tergantung seberapa kuat Anda menyukainya. Setelah meresap, keluarkan kantong teh dan tambahkan susu dan gula sesuai selera. Aduk semuanya dan nikmati secangkir teh panas Anda."
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
      temperature: 0,
      system: "Tugas Anda adalah mengambil deskripsi bahasa alami yang diberikan tentang suatu proses atau tugas dan mengubahnya menjadi petunjuk langkah demi langkah yang jelas, ringkas, logis, berurutan, dan mudah diikuti. Gunakan bahasa imperatif dan mulai setiap langkah dengan kata kerja tindakan. Berikan detail dan penjelasan yang diperlukan untuk memastikan pembaca dapat menyelesaikan tugas dengan sukses. Jika deskripsi asli tidak jelas, ambigu, atau kurang informasi yang memadai, minta klarifikasi atau detail tambahan.",
      messages: [
        {
          "role": "user",
          "content": [
            {
              "type": "text",
              "text": "Untuk membuat secangkir teh, mulailah dengan merebus air dalam ketel. Sementara air sedang dipanaskan, ambil cangkir dan masukkan kantong teh ke dalamnya. Setelah air mendidih, tuangkan dengan hati-hati ke dalam cangkir, di atas kantong teh. Biarkan teh meresap selama beberapa menit, tergantung seberapa kuat Anda menyukainya. Setelah meresap, keluarkan kantong teh dan tambahkan susu dan gula sesuai selera. Aduk semuanya dan nikmati secangkir teh panas Anda."
            }
          ]
        }
      ]
    });
    console.log(msg);
    
    ```
</CodeGroup>