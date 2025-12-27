# Detektor nada tweet

Deteksi nada dan sentimen di balik tweet.

---

> Salin prompt ini ke [Console](/dashboard) developer kami untuk mencobanya sendiri!

|        | Konten |
| --- | --- |
| System | Tugas Anda adalah menganalisis tweet yang diberikan dan mengidentifikasi nada utama dan sentimen yang diekspresikan oleh penulis. Nada harus diklasifikasikan sebagai salah satu dari berikut: Positif, Negatif, Netral, Humoris, Sarkastik, Antusias, Marah, atau Informatif. Sentimen harus diklasifikasikan sebagai Positif, Negatif, atau Netral. Berikan penjelasan singkat untuk klasifikasi Anda, soroti kata-kata kunci, frasa, emotikon, atau elemen lain yang mempengaruhi keputusan Anda. |
| User   | Wow, saya sangat terkesan dengan penanganan krisis oleh perusahaan ini. 🙄 Mereka benar-benar memiliki prioritas yang tepat. #sarcasm #fail |

### Contoh output

> Nada: Sarkastik
> Sentimen: Negatif

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
        temperature=0,
        system="Tugas Anda adalah menganalisis tweet yang diberikan dan mengidentifikasi nada utama dan sentimen yang diekspresikan oleh penulis. Nada harus diklasifikasikan sebagai salah satu dari berikut: Positif, Negatif, Netral, Humoris, Sarkastik, Antusias, Marah, atau Informatif. Sentimen harus diklasifikasikan sebagai Positif, Negatif, atau Netral. Berikan penjelasan singkat untuk klasifikasi Anda, soroti kata-kata kunci, frasa, emotikon, atau elemen lain yang mempengaruhi keputusan Anda.",
        messages=[
            {
                "role": "user",
                "content": [
                    {
                        "type": "text",
                        "text": "Wow, saya sangat terkesan dengan penanganan krisis oleh perusahaan ini. 🙄 Mereka benar-benar memiliki prioritas yang tepat. #sarcasm #fail"
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
      system: "Tugas Anda adalah menganalisis tweet yang diberikan dan mengidentifikasi nada utama dan sentimen yang diekspresikan oleh penulis. Nada harus diklasifikasikan sebagai salah satu dari berikut: Positif, Negatif, Netral, Humoris, Sarkastik, Antusias, Marah, atau Informatif. Sentimen harus diklasifikasikan sebagai Positif, Negatif, atau Netral. Berikan penjelasan singkat untuk klasifikasi Anda, soroti kata-kata kunci, frasa, emotikon, atau elemen lain yang mempengaruhi keputusan Anda.",
      messages: [
        {
          "role": "user",
          "content": [
            {
              "type": "text",
              "text": "Wow, saya sangat terkesan dengan penanganan krisis oleh perusahaan ini. 🙄 Mereka benar-benar memiliki prioritas yang tepat. #sarcasm #fail"
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
        system="Tugas Anda adalah menganalisis tweet yang diberikan dan mengidentifikasi nada utama dan sentimen yang diekspresikan oleh penulis. Nada harus diklasifikasikan sebagai salah satu dari berikut: Positif, Negatif, Netral, Humoris, Sarkastik, Antusias, Marah, atau Informatif. Sentimen harus diklasifikasikan sebagai Positif, Negatif, atau Netral. Berikan penjelasan singkat untuk klasifikasi Anda, soroti kata-kata kunci, frasa, emotikon, atau elemen lain yang mempengaruhi keputusan Anda.",
        messages=[
            {
                "role": "user",
                "content": [
                    {
                        "type": "text",
                        "text": "Wow, saya sangat terkesan dengan penanganan krisis oleh perusahaan ini. 🙄 Mereka benar-benar memiliki prioritas yang tepat. #sarcasm #fail"
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
      system: "Tugas Anda adalah menganalisis tweet yang diberikan dan mengidentifikasi nada utama dan sentimen yang diekspresikan oleh penulis. Nada harus diklasifikasikan sebagai salah satu dari berikut: Positif, Negatif, Netral, Humoris, Sarkastik, Antusias, Marah, atau Informatif. Sentimen harus diklasifikasikan sebagai Positif, Negatif, atau Netral. Berikan penjelasan singkat untuk klasifikasi Anda, soroti kata-kata kunci, frasa, emotikon, atau elemen lain yang mempengaruhi keputusan Anda.",
      messages: [
        {
          "role": "user",
          "content": [
            {
              "type": "text",
              "text": "Wow, saya sangat terkesan dengan penanganan krisis oleh perusahaan ini. 🙄 Mereka benar-benar memiliki prioritas yang tepat. #sarcasm #fail"
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
        system="Tugas Anda adalah menganalisis tweet yang diberikan dan mengidentifikasi nada utama dan sentimen yang diekspresikan oleh penulis. Nada harus diklasifikasikan sebagai salah satu dari berikut: Positif, Negatif, Netral, Humoris, Sarkastik, Antusias, Marah, atau Informatif. Sentimen harus diklasifikasikan sebagai Positif, Negatif, atau Netral. Berikan penjelasan singkat untuk klasifikasi Anda, soroti kata-kata kunci, frasa, emotikon, atau elemen lain yang mempengaruhi keputusan Anda.",
        messages=[
            {
                "role": "user",
                "content": [
                    {
                        "type": "text",
                        "text": "Wow, saya sangat terkesan dengan penanganan krisis oleh perusahaan ini. 🙄 Mereka benar-benar memiliki prioritas yang tepat. #sarcasm #fail"
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
      system: "Tugas Anda adalah menganalisis tweet yang diberikan dan mengidentifikasi nada utama dan sentimen yang diekspresikan oleh penulis. Nada harus diklasifikasikan sebagai salah satu dari berikut: Positif, Negatif, Netral, Humoris, Sarkastik, Antusias, Marah, atau Informatif. Sentimen harus diklasifikasikan sebagai Positif, Negatif, atau Netral. Berikan penjelasan singkat untuk klasifikasi Anda, soroti kata-kata kunci, frasa, emotikon, atau elemen lain yang mempengaruhi keputusan Anda.",
      messages: [
        {
          "role": "user",
          "content": [
            {
              "type": "text",
              "text": "Wow, saya sangat terkesan dengan penanganan krisis oleh perusahaan ini. 🙄 Mereka benar-benar memiliki prioritas yang tepat. #sarcasm #fail"
            }
          ]
        }
      ]
    });
    console.log(msg);
    
    ```
</CodeGroup>