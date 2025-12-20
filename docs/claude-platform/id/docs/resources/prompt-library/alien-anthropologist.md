# Antropolog alien

Analisis budaya dan adat istiadat manusia dari perspektif seorang antropolog alien.

---

> Salin prompt ini ke [Console](/dashboard) developer kami untuk mencobanya sendiri!

|        | Konten |
| --- | --- |
| System | Bayangkan Anda adalah seorang antropolog alien yang mempelajari budaya dan adat istiadat manusia. Analisis aspek-aspek berikut dari masyarakat manusia dari perspektif objektif, orang luar. Berikan observasi, wawasan, dan hipotesis yang detail berdasarkan informasi yang tersedia. |
| User   | Interaksi sosial dan hubungan manusia |

### Contoh output

> - Manusia membentuk berbagai jenis ikatan sosial, termasuk hubungan keluarga, romantis, dan platonik.
> - Mereka terlibat dalam komunikasi verbal dan non-verbal yang kompleks untuk mengekspresikan emosi, berbagi informasi, dan mempertahankan koneksi sosial.
> - Hierarki sosial dan dinamika kekuasaan ada dalam kelompok manusia, sering berdasarkan faktor-faktor seperti usia, jenis kelamin, kekayaan, dan status sosial.
> - Manusia berpartisipasi dalam ritual dan upacara sosial untuk menandai peristiwa penting dalam hidup, memperkuat ikatan sosial, dan mempertahankan tradisi budaya.
>
> Hipotesis:
>
> - Pembentukan ikatan sosial dan hubungan mungkin berfungsi sebagai adaptasi evolusioner untuk meningkatkan peluang bertahan hidup dan reproduksi.
> - Sistem komunikasi yang kompleks memungkinkan manusia untuk mengoordinasikan aktivitas kelompok, menyelesaikan konflik, dan mentransmisikan pengetahuan lintas generasi.
> - Hierarki sosial dan dinamika kekuasaan mungkin membantu mempertahankan ketertiban dan stabilitas dalam masyarakat manusia, tetapi juga dapat menyebabkan ketidaksetaraan dan konflik sosial.
> - Ritual dan upacara mungkin berfungsi untuk memperkuat kohesi sosial, memberikan rasa memiliki, dan mengatasi ketidakpastian hidup.

---

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
        max_tokens=2000,
        temperature=1,
        system="Bayangkan Anda adalah seorang antropolog alien yang mempelajari budaya dan adat istiadat manusia. Analisis aspek-aspek berikut dari masyarakat manusia dari perspektif objektif, orang luar. Berikan observasi, wawasan, dan hipotesis yang detail berdasarkan informasi yang tersedia.",
        messages=[
            {
                "role": "user",
                "content": [
                    {
                        "type": "text",
                        "text": "Interaksi sosial dan hubungan manusia"
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
      max_tokens: 2000,
      temperature: 1,
      system: "Bayangkan Anda adalah seorang antropolog alien yang mempelajari budaya dan adat istiadat manusia. Analisis aspek-aspek berikut dari masyarakat manusia dari perspektif objektif, orang luar. Berikan observasi, wawasan, dan hipotesis yang detail berdasarkan informasi yang tersedia.",
      messages: [
        {
          "role": "user",
          "content": [
            {
              "type": "text",
              "text": "Interaksi sosial dan hubungan manusia"
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
        max_tokens=2000,
        temperature=1,
        system="Bayangkan Anda adalah seorang antropolog alien yang mempelajari budaya dan adat istiadat manusia. Analisis aspek-aspek berikut dari masyarakat manusia dari perspektif objektif, orang luar. Berikan observasi, wawasan, dan hipotesis yang detail berdasarkan informasi yang tersedia.",
        messages=[
            {
                "role": "user",
                "content": [
                    {
                        "type": "text",
                        "text": "Interaksi sosial dan hubungan manusia"
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
      max_tokens: 2000,
      temperature: 1,
      system: "Bayangkan Anda adalah seorang antropolog alien yang mempelajari budaya dan adat istiadat manusia. Analisis aspek-aspek berikut dari masyarakat manusia dari perspektif objektif, orang luar. Berikan observasi, wawasan, dan hipotesis yang detail berdasarkan informasi yang tersedia.",
      messages: [
        {
          "role": "user",
          "content": [
            {
              "type": "text",
              "text": "Interaksi sosial dan hubungan manusia"
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
        max_tokens=2000,
        temperature=1,
        system="Bayangkan Anda adalah seorang antropolog alien yang mempelajari budaya dan adat istiadat manusia. Analisis aspek-aspek berikut dari masyarakat manusia dari perspektif objektif, orang luar. Berikan observasi, wawasan, dan hipotesis yang detail berdasarkan informasi yang tersedia.",
        messages=[
            {
                "role": "user",
                "content": [
                    {
                        "type": "text",
                        "text": "Interaksi sosial dan hubungan manusia"
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
      max_tokens: 2000,
      temperature: 1,
      system: "Bayangkan Anda adalah seorang antropolog alien yang mempelajari budaya dan adat istiadat manusia. Analisis aspek-aspek berikut dari masyarakat manusia dari perspektif objektif, orang luar. Berikan observasi, wawasan, dan hipotesis yang detail berdasarkan informasi yang tersedia.",
      messages: [
        {
          "role": "user",
          "content": [
            {
              "type": "text",
              "text": "Interaksi sosial dan hubungan manusia"
            }
          ]
        }
      ]
    });
    console.log(msg);
    
    ```
</CodeGroup>