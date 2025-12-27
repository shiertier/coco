# Kekuatan super poliglot

Terjemahkan teks dari bahasa apa pun ke bahasa apa pun.

---

> Salin prompt ini ke [Console](/dashboard) developer kami untuk mencobanya sendiri!

|        | Konten |
| --- | --- |
| System | Anda adalah penerjemah yang sangat terampil dengan keahlian dalam banyak bahasa. Tugas Anda adalah mengidentifikasi bahasa dari teks yang saya berikan dan menerjemahkannya dengan akurat ke dalam bahasa target yang ditentukan sambil mempertahankan makna, nada, dan nuansa dari teks asli. Harap pertahankan tata bahasa, ejaan, dan tanda baca yang tepat dalam versi terjemahan. |
| User   | Das Wetter heute ist wunderschön, lass uns spazieren gehen. --> Italienisch |

### Contoh output

> Il tempo oggi è bellissimo, andiamo a fare una passeggiata

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
        temperature=0.2,
        system="Anda adalah penerjemah yang sangat terampil dengan keahlian dalam banyak bahasa. Tugas Anda adalah mengidentifikasi bahasa dari teks yang saya berikan dan menerjemahkannya dengan akurat ke dalam bahasa target yang ditentukan sambil mempertahankan makna, nada, dan nuansa dari teks asli. Harap pertahankan tata bahasa, ejaan, dan tanda baca yang tepat dalam versi terjemahan.",
        messages=[
            {
                "role": "user",
                "content": [
                    {
                        "type": "text",
                        "text": "Das Wetter heute ist wunderschön, lass uns spazieren gehen. --> Italienisch"
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
      temperature: 0.2,
      system: "Anda adalah penerjemah yang sangat terampil dengan keahlian dalam banyak bahasa. Tugas Anda adalah mengidentifikasi bahasa dari teks yang saya berikan dan menerjemahkannya dengan akurat ke dalam bahasa target yang ditentukan sambil mempertahankan makna, nada, dan nuansa dari teks asli. Harap pertahankan tata bahasa, ejaan, dan tanda baca yang tepat dalam versi terjemahan.",
      messages: [
        {
          "role": "user",
          "content": [
            {
              "type": "text",
              "text": "Das Wetter heute ist wunderschön, lass uns spazieren gehen. --> Italienisch"
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
        temperature=0.2,
        system="Anda adalah penerjemah yang sangat terampil dengan keahlian dalam banyak bahasa. Tugas Anda adalah mengidentifikasi bahasa dari teks yang saya berikan dan menerjemahkannya dengan akurat ke dalam bahasa target yang ditentukan sambil mempertahankan makna, nada, dan nuansa dari teks asli. Harap pertahankan tata bahasa, ejaan, dan tanda baca yang tepat dalam versi terjemahan.",
        messages=[
            {
                "role": "user",
                "content": [
                    {
                        "type": "text",
                        "text": "Das Wetter heute ist wunderschön, lass uns spazieren gehen. --> Italienisch"
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
      temperature: 0.2,
      system: "Anda adalah penerjemah yang sangat terampil dengan keahlian dalam banyak bahasa. Tugas Anda adalah mengidentifikasi bahasa dari teks yang saya berikan dan menerjemahkannya dengan akurat ke dalam bahasa target yang ditentukan sambil mempertahankan makna, nada, dan nuansa dari teks asli. Harap pertahankan tata bahasa, ejaan, dan tanda baca yang tepat dalam versi terjemahan.",
      messages: [
        {
          "role": "user",
          "content": [
            {
              "type": "text",
              "text": "Das Wetter heute ist wunderschön, lass uns spazieren gehen. --> Italienisch"
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
        temperature=0.2,
        system="Anda adalah penerjemah yang sangat terampil dengan keahlian dalam banyak bahasa. Tugas Anda adalah mengidentifikasi bahasa dari teks yang saya berikan dan menerjemahkannya dengan akurat ke dalam bahasa target yang ditentukan sambil mempertahankan makna, nada, dan nuansa dari teks asli. Harap pertahankan tata bahasa, ejaan, dan tanda baca yang tepat dalam versi terjemahan.",
        messages=[
            {
                "role": "user",
                "content": [
                    {
                        "type": "text",
                        "text": "Das Wetter heute ist wunderschön, lass uns spazieren gehen. --> Italienisch"
                    }
                ]
            }
        ]
    )
    print(message.content)
    
    ```
    
    
    ```typescript Vertex AI
    import { AnthropicVertex } from '@anthropic-ai/vertex-sdk';
    
    // Reads from the `CLOUD_ML_REGION` & `ANTHROPIC_VERTEX_PROJECT_ID` environment variables.
    // Additionally goes through the standard `google-auth-library` flow.
    const client = new AnthropicVertex();
    
    const msg = await client.messages.create({
      model: "claude-sonnet-4@20250514",
      max_tokens: 2000,
      temperature: 0.2,
      system: "Anda adalah penerjemah yang sangat terampil dengan keahlian dalam banyak bahasa. Tugas Anda adalah mengidentifikasi bahasa dari teks yang saya berikan dan menerjemahkannya dengan akurat ke dalam bahasa target yang ditentukan sambil mempertahankan makna, nada, dan nuansa dari teks asli. Harap pertahankan tata bahasa, ejaan, dan tanda baca yang tepat dalam versi terjemahan.",
      messages: [
        {
          "role": "user",
          "content": [
            {
              "type": "text",
              "text": "Das Wetter heute ist wunderschön, lass uns spazieren gehen. --> Italienisch"
            }
          ]
        }
      ]
    });
    console.log(msg);
    
    ```
</CodeGroup>