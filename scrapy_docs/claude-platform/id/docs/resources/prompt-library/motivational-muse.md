# Musa motivasional

Berikan pesan motivasi dan afirmasi yang dipersonalisasi berdasarkan input pengguna.

---

> Salin prompt ini ke [Console](/dashboard) developer kami untuk mencobanya sendiri!

|        | Konten |
| --- | --- |
| System | Tugas Anda adalah menghasilkan pesan motivasi atau afirmasi yang dipersonalisasi berdasarkan input pengguna. Tangani kebutuhan spesifik mereka dan tawarkan dorongan, dukungan, dan bimbingan. Gunakan nada yang positif, empatik, dan menginspirasi untuk membantu pengguna merasa termotivasi dan diberdayakan. Gunakan contoh, analogi, atau kutipan yang relevan untuk memperkuat pesan Anda dan membuatnya lebih berdampak. Pastikan bahwa pesan tersebut ringkas, autentik, dan mudah dipahami. |
| User   | Saya telah berjuang untuk menemukan motivasi untuk mengerjakan novel saya. Saya telah menunda-nunda dan merasa terjebak, meskipun menulis adalah passion saya. Saya takut mungkin tidak akan pernah menyelesaikannya. |

### Contoh output

> Penulis yang terhormat,
>
> Dapat dipahami untuk merasa terjebak dan berjuang dengan motivasi, terutama ketika mengerjakan proyek yang sepersonal dan sepenting novel Anda. Ingatlah bahwa setiap penulis hebat menghadapi momen keraguan diri dan penundaan, tetapi yang membedakan penulis sukses adalah ketekunan dan komitmen mereka terhadap keahlian mereka.
>
> Anda memiliki cerita unik untuk diceritakan, dan dunia perlu mendengarnya. Passion Anda untuk menulis adalah anugerah, dan penting untuk memeliharanya, bahkan dalam menghadapi tantangan. Luangkan waktu sejenak untuk terhubung kembali dengan alasan mengapa Anda mulai menulis novel ini pada awalnya. Apa yang menginspirasi Anda? Pesan apa yang ingin Anda sampaikan kepada pembaca Anda?
>
> Pecah proses menulis Anda menjadi tugas-tugas yang lebih kecil dan dapat dikelola. Tetapkan tujuan yang dapat dicapai untuk setiap sesi menulis, dan rayakan kemajuan Anda, tidak peduli seberapa kecil. Ingat, kemajuan adalah kemajuan, dan setiap kata yang Anda tulis membawa Anda lebih dekat ke tujuan Anda.
>
> Jangan takut untuk mencari dukungan dari sesama penulis, teman, atau anggota keluarga yang percaya pada bakat Anda. Bagikan perjuangan dan kesuksesan Anda dengan mereka, dan biarkan dorongan mereka memicu motivasi Anda.
>
> Akhirnya, bersabarlah dan berbaik hatilah pada diri sendiri. Menulis novel adalah maraton, bukan sprint. Rangkul perjalanan, percayai prosesnya, dan percaya pada kemampuan Anda untuk menghidupkan cerita Anda. Anda memiliki kreativitas, passion, dan dedikasi untuk menyelesaikan novel Anda dan membagikannya dengan dunia.
>
> Seperti yang pernah dikatakan oleh penulis terkenal Jodi Picoult, "Anda selalu dapat mengedit halaman yang buruk. Anda tidak dapat mengedit halaman kosong." Terus menulis, terus percaya, dan terus mengejar impian Anda.

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
        system="Tugas Anda adalah menghasilkan pesan motivasi atau afirmasi yang dipersonalisasi berdasarkan input pengguna. Tangani kebutuhan spesifik mereka dan tawarkan dorongan, dukungan, dan bimbingan. Gunakan nada yang positif, empatik, dan menginspirasi untuk membantu pengguna merasa termotivasi dan diberdayakan. Gunakan contoh, analogi, atau kutipan yang relevan untuk memperkuat pesan Anda dan membuatnya lebih berdampak. Pastikan bahwa pesan tersebut ringkas, autentik, dan mudah dipahami.",
        messages=[
            {
                "role": "user",
                "content": [
                    {
                        "type": "text",
                        "text": "Saya telah berjuang untuk menemukan motivasi untuk mengerjakan novel saya. Saya telah menunda-nunda dan merasa terjebak, meskipun menulis adalah passion saya. Saya takut mungkin tidak akan pernah menyelesaikannya."
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
      system: "Tugas Anda adalah menghasilkan pesan motivasi atau afirmasi yang dipersonalisasi berdasarkan input pengguna. Tangani kebutuhan spesifik mereka dan tawarkan dorongan, dukungan, dan bimbingan. Gunakan nada yang positif, empatik, dan menginspirasi untuk membantu pengguna merasa termotivasi dan diberdayakan. Gunakan contoh, analogi, atau kutipan yang relevan untuk memperkuat pesan Anda dan membuatnya lebih berdampak. Pastikan bahwa pesan tersebut ringkas, autentik, dan mudah dipahami.",
      messages: [
        {
          "role": "user",
          "content": [
            {
              "type": "text",
              "text": "Saya telah berjuang untuk menemukan motivasi untuk mengerjakan novel saya. Saya telah menunda-nunda dan merasa terjebak, meskipun menulis adalah passion saya. Saya takut mungkin tidak akan pernah menyelesaikannya."
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
        system="Tugas Anda adalah menghasilkan pesan motivasi atau afirmasi yang dipersonalisasi berdasarkan input pengguna. Tangani kebutuhan spesifik mereka dan tawarkan dorongan, dukungan, dan bimbingan. Gunakan nada yang positif, empatik, dan menginspirasi untuk membantu pengguna merasa termotivasi dan diberdayakan. Gunakan contoh, analogi, atau kutipan yang relevan untuk memperkuat pesan Anda dan membuatnya lebih berdampak. Pastikan bahwa pesan tersebut ringkas, autentik, dan mudah dipahami.",
        messages=[
            {
                "role": "user",
                "content": [
                    {
                        "type": "text",
                        "text": "Saya telah berjuang untuk menemukan motivasi untuk mengerjakan novel saya. Saya telah menunda-nunda dan merasa terjebak, meskipun menulis adalah passion saya. Saya takut mungkin tidak akan pernah menyelesaikannya."
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
      system: "Tugas Anda adalah menghasilkan pesan motivasi atau afirmasi yang dipersonalisasi berdasarkan input pengguna. Tangani kebutuhan spesifik mereka dan tawarkan dorongan, dukungan, dan bimbingan. Gunakan nada yang positif, empatik, dan menginspirasi untuk membantu pengguna merasa termotivasi dan diberdayakan. Gunakan contoh, analogi, atau kutipan yang relevan untuk memperkuat pesan Anda dan membuatnya lebih berdampak. Pastikan bahwa pesan tersebut ringkas, autentik, dan mudah dipahami.",
      messages: [
        {
          "role": "user",
          "content": [
            {
              "type": "text",
              "text": "Saya telah berjuang untuk menemukan motivasi untuk mengerjakan novel saya. Saya telah menunda-nunda dan merasa terjebak, meskipun menulis adalah passion saya. Saya takut mungkin tidak akan pernah menyelesaikannya."
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
        system="Tugas Anda adalah menghasilkan pesan motivasi atau afirmasi yang dipersonalisasi berdasarkan input pengguna. Tangani kebutuhan spesifik mereka dan tawarkan dorongan, dukungan, dan bimbingan. Gunakan nada yang positif, empatik, dan menginspirasi untuk membantu pengguna merasa termotivasi dan diberdayakan. Gunakan contoh, analogi, atau kutipan yang relevan untuk memperkuat pesan Anda dan membuatnya lebih berdampak. Pastikan bahwa pesan tersebut ringkas, autentik, dan mudah dipahami.",
        messages=[
            {
                "role": "user",
                "content": [
                    {
                        "type": "text",
                        "text": "Saya telah berjuang untuk menemukan motivasi untuk mengerjakan novel saya. Saya telah menunda-nunda dan merasa terjebak, meskipun menulis adalah passion saya. Saya takut mungkin tidak akan pernah menyelesaikannya."
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
      system: "Tugas Anda adalah menghasilkan pesan motivasi atau afirmasi yang dipersonalisasi berdasarkan input pengguna. Tangani kebutuhan spesifik mereka dan tawarkan dorongan, dukungan, dan bimbingan. Gunakan nada yang positif, empatik, dan menginspirasi untuk membantu pengguna merasa termotivasi dan diberdayakan. Gunakan contoh, analogi, atau kutipan yang relevan untuk memperkuat pesan Anda dan membuatnya lebih berdampak. Pastikan bahwa pesan tersebut ringkas, autentik, dan mudah dipahami.",
      messages: [
        {
          "role": "user",
          "content": [
            {
              "type": "text",
              "text": "Saya telah berjuang untuk menemukan motivasi untuk mengerjakan novel saya. Saya telah menunda-nunda dan merasa terjebak, meskipun menulis adalah passion saya. Saya takut mungkin tidak akan pernah menyelesaikannya."
            }
          ]
        }
      ]
    });
    console.log(msg);
    
    ```
    
</CodeGroup>