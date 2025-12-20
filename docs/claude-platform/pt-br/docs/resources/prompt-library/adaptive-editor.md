# Editor adaptativo

Reescreva texto seguindo instruções dadas pelo usuário, como com um tom, público ou estilo diferente.

---

> Copie este prompt para nosso [Console](/dashboard) de desenvolvedor para experimentá-lo você mesmo!

|      | Conteúdo |
| ---- | --- |
| Usuário | Reescreva o seguinte parágrafo usando as seguintes instruções: no estilo de um pirata. Parágrafo: Em 1758, o botânico e zoólogo sueco Carl Linnaeus publicou em seu Systema Naturae, a nomenclatura de duas palavras das espécies (nomenclatura binomial). Canis é a palavra latina que significa "cão", e sob este gênero, ele listou o cão doméstico, o lobo e o chacal dourado. |

### Exemplo de saída

> Ahoy companheiros! Lá em 1758, aquele botânico escorbútico e amante dos animais Carl Linnaeus publicou seu livro Systema Naturae, que tinha aqueles nomes chiques de duas palavras para todo tipo de criaturas (nomenclatura binomial ele chamava). Canis é a palavra pirata para "cão", e sob esse título ele listou cães comuns, lobos e chacais dourados também. Arrr! Todos esses cães são iguais para o velho Capitão Linnaeus. Ele pode ter tido uma perna de pau e um papagaio no ombro quando escreveu aquele livro! Haha! Agora esfreguem o convés, seus patifes, antes que eu os faça caminhar na prancha!

---

### Solicitação da API

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