# Analis kode bandara

Temukan dan ekstrak kode bandara dari teks.

---

> Salin prompt ini ke [Console](/dashboard) developer kami untuk mencobanya sendiri!

|        | Konten |
| --- | --- |
| System | Tugas Anda adalah menganalisis teks yang diberikan dan mengidentifikasi kode bandara apa pun yang disebutkan di dalamnya. Sajikan kode bandara ini sebagai daftar sesuai urutan kemunculannya dalam teks. Jika tidak ada kode bandara yang ditemukan, kembalikan daftar kosong. |
| User   | Perjalanan saya selanjutnya melibatkan penerbangan dari Seattle ke Amsterdam. Saya akan menghabiskan beberapa hari di Amsterdam sebelum menuju ke Paris untuk penerbangan transit ke Roma. |

### Contoh Output

> Berikut adalah daftar kode bandara yang disebutkan dalam teks, sesuai urutan kemunculannya:
>
> 1. SEA (Seattle)
> 2. AMS (Amsterdam)
> 3. CDG (Paris)
> 4. FCO (Roma)

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
        system="Tugas Anda adalah menganalisis teks yang diberikan dan mengidentifikasi kode bandara apa pun yang disebutkan di dalamnya. Sajikan kode bandara ini sebagai daftar sesuai urutan kemunculannya dalam teks. Jika tidak ada kode bandara yang ditemukan, kembalikan daftar kosong.",
        messages=[
            {
                "role": "user",
                "content": [
                    {
                        "type": "text",
                        "text": "Perjalanan saya selanjutnya melibatkan penerbangan dari Seattle ke Amsterdam. Saya akan menghabiskan beberapa hari di Amsterdam sebelum menuju ke Paris untuk penerbangan transit ke Roma."
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
      system: "Tugas Anda adalah menganalisis teks yang diberikan dan mengidentifikasi kode bandara apa pun yang disebutkan di dalamnya. Sajikan kode bandara ini sebagai daftar sesuai urutan kemunculannya dalam teks. Jika tidak ada kode bandara yang ditemukan, kembalikan daftar kosong.",
      messages: [
        {
          "role": "user",
          "content": [
            {
              "type": "text",
              "text": "Perjalanan saya selanjutnya melibatkan penerbangan dari Seattle ke Amsterdam. Saya akan menghabiskan beberapa hari di Amsterdam sebelum menuju ke Paris untuk penerbangan transit ke Roma."
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
        system="Tugas Anda adalah menganalisis teks yang diberikan dan mengidentifikasi kode bandara apa pun yang disebutkan di dalamnya. Sajikan kode bandara ini sebagai daftar sesuai urutan kemunculannya dalam teks. Jika tidak ada kode bandara yang ditemukan, kembalikan daftar kosong.",
        messages=[
            {
                "role": "user",
                "content": [
                    {
                        "type": "text",
                        "text": "Perjalanan saya selanjutnya melibatkan penerbangan dari Seattle ke Amsterdam. Saya akan menghabiskan beberapa hari di Amsterdam sebelum menuju ke Paris untuk penerbangan transit ke Roma."
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
      system: "Tugas Anda adalah menganalisis teks yang diberikan dan mengidentifikasi kode bandara apa pun yang disebutkan di dalamnya. Sajikan kode bandara ini sebagai daftar sesuai urutan kemunculannya dalam teks. Jika tidak ada kode bandara yang ditemukan, kembalikan daftar kosong.",
      messages: [
        {
          "role": "user",
          "content": [
            {
              "type": "text",
              "text": "Perjalanan saya selanjutnya melibatkan penerbangan dari Seattle ke Amsterdam. Saya akan menghabiskan beberapa hari di Amsterdam sebelum menuju ke Paris untuk penerbangan transit ke Roma."
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
        system="Tugas Anda adalah menganalisis teks yang diberikan dan mengidentifikasi kode bandara apa pun yang disebutkan di dalamnya. Sajikan kode bandara ini sebagai daftar sesuai urutan kemunculannya dalam teks. Jika tidak ada kode bandara yang ditemukan, kembalikan daftar kosong.",
        messages=[
            {
                "role": "user",
                "content": [
                    {
                        "type": "text",
                        "text": "Perjalanan saya selanjutnya melibatkan penerbangan dari Seattle ke Amsterdam. Saya akan menghabiskan beberapa hari di Amsterdam sebelum menuju ke Paris untuk penerbangan transit ke Roma."
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
      system: "Tugas Anda adalah menganalisis teks yang diberikan dan mengidentifikasi kode bandara apa pun yang disebutkan di dalamnya. Sajikan kode bandara ini sebagai daftar sesuai urutan kemunculannya dalam teks. Jika tidak ada kode bandara yang ditemukan, kembalikan daftar kosong.",
      messages: [
        {
          "role": "user",
          "content": [
            {
              "type": "text",
              "text": "Perjalanan saya selanjutnya melibatkan penerbangan dari Seattle ke Amsterdam. Saya akan menghabiskan beberapa hari di Amsterdam sebelum menuju ke Paris untuk penerbangan transit ke Roma."
            }
          ]
        }
      ]
    });
    console.log(msg);
    
    ```
</CodeGroup>