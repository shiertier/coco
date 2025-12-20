# Editor adaptif

Tulis ulang teks mengikuti instruksi yang diberikan pengguna, seperti dengan nada, audiens, atau gaya yang berbeda.

---

> Salin prompt ini ke [Console](/dashboard) developer kami untuk mencobanya sendiri!

|      | Konten |
| ---- | --- |
| User | Tulis ulang paragraf berikut menggunakan instruksi berikut: dengan gaya bajak laut. Paragraf: Pada tahun 1758, ahli botani dan zoologi Swedia Carl Linnaeus menerbitkan dalam Systema Naturae-nya, penamaan dua kata untuk spesies (nomenklatur binomial). Canis adalah kata Latin yang berarti "anjing", dan di bawah genus ini, dia mencantumkan anjing domestik, serigala, dan serigala emas. |

### Contoh output

> Ahoy kawan! Kembali pada tahun 1758, ahli botani dan pecinta hewan yang kotor itu Carl Linnaeus mengeluarkan bukunya Systema Naturae, yang memiliki nama dua kata mewah untuk segala macam makhluk (nomenklatur binomial dia menyebutnya). Canis adalah kata bajak laut untuk "anjing", dan di bawah judul itu dia mencantumkan anjing biasa, serigala, dan serigala emas juga. Arrr! Semua anjing itu sama saja bagi Kapten Linnaeus tua. Dia mungkin memiliki kaki kayu dan burung beo di bahunya ketika dia menulis buku itu! Haha! Sekarang pel dek kalian para bajingan sebelum aku membuat kalian berjalan di papan!

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
        messages=[
            {
                "role": "user",
                "content": [
                    {
                        "type": "text",
                        "text": "Rewrite the following paragraph using the following instructions: in the style of a pirate.  \n  \nParagraph:  \nIn 1758, the Swedish botanist and zoologist Carl Linnaeus published in his Systema Naturae, the two-word naming of species (binomial nomenclature). Canis is the Latin word meaning \"dog\", and under this genus, he listed the domestic dog, the wolf, and the golden jackal."
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
      messages: [
        {
          "role": "user",
          "content": [
            {
              "type": "text",
              "text": "Rewrite the following paragraph using the following instructions: in the style of a pirate.  \n  \nParagraph:  \nIn 1758, the Swedish botanist and zoologist Carl Linnaeus published in his Systema Naturae, the two-word naming of species (binomial nomenclature). Canis is the Latin word meaning \"dog\", and under this genus, he listed the domestic dog, the wolf, and the golden jackal."
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
        messages=[
            {
                "role": "user",
                "content": [
                    {
                        "type": "text",
                        "text": "Rewrite the following paragraph using the following instructions: in the style of a pirate.  \n  \nParagraph:  \nIn 1758, the Swedish botanist and zoologist Carl Linnaeus published in his Systema Naturae, the two-word naming of species (binomial nomenclature). Canis is the Latin word meaning \"dog\", and under this genus, he listed the domestic dog, the wolf, and the golden jackal."
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
      messages: [
        {
          "role": "user",
          "content": [
            {
              "type": "text",
              "text": "Rewrite the following paragraph using the following instructions: in the style of a pirate.  \n  \nParagraph:  \nIn 1758, the Swedish botanist and zoologist Carl Linnaeus published in his Systema Naturae, the two-word naming of species (binomial nomenclature). Canis is the Latin word meaning \"dog\", and under this genus, he listed the domestic dog, the wolf, and the golden jackal."
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
        messages=[
            {
                "role": "user",
                "content": [
                    {
                        "type": "text",
                        "text": "Rewrite the following paragraph using the following instructions: in the style of a pirate.  \n  \nParagraph:  \nIn 1758, the Swedish botanist and zoologist Carl Linnaeus published in his Systema Naturae, the two-word naming of species (binomial nomenclature). Canis is the Latin word meaning \"dog\", and under this genus, he listed the domestic dog, the wolf, and the golden jackal."
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
      messages: [
        {
          "role": "user",
          "content": [
            {
              "type": "text",
              "text": "Rewrite the following paragraph using the following instructions: in the style of a pirate.  \n  \nParagraph:  \nIn 1758, the Swedish botanist and zoologist Carl Linnaeus published in his Systema Naturae, the two-word naming of species (binomial nomenclature). Canis is the Latin word meaning \"dog\", and under this genus, he listed the domestic dog, the wolf, and the golden jackal."
            }
          ]
        }
      ]
    });
    console.log(msg);
    
    ```
</CodeGroup>