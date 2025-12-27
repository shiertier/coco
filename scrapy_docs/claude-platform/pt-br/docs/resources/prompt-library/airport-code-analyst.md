# Analisador de códigos de aeroporto

Encontre e extraia códigos de aeroporto do texto.

---

> Copie este prompt para nosso [Console](/dashboard) de desenvolvedor para experimentar você mesmo!

|        | Conteúdo |
| --- | --- |
| Sistema | Sua tarefa é analisar o texto fornecido e identificar quaisquer códigos de aeroporto mencionados nele. Apresente esses códigos de aeroporto como uma lista na ordem em que aparecem no texto. Se nenhum código de aeroporto for encontrado, retorne uma lista vazia. |
| Usuário   | Minha próxima viagem envolve voar de Seattle para Amsterdam. Vou passar alguns dias em Amsterdam antes de seguir para Paris para um voo de conexão para Roma. |

### Exemplo de Saída

> Aqui está a lista de códigos de aeroporto mencionados no texto, na ordem em que aparecem:
>
> 1. SEA (Seattle)
> 2. AMS (Amsterdam)
> 3. CDG (Paris)
> 4. FCO (Roma)

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
        max_tokens=1000,
        temperature=0,
        system="Sua tarefa é analisar o texto fornecido e identificar quaisquer códigos de aeroporto mencionados nele. Apresente esses códigos de aeroporto como uma lista na ordem em que aparecem no texto. Se nenhum código de aeroporto for encontrado, retorne uma lista vazia.",
        messages=[
            {
                "role": "user",
                "content": [
                    {
                        "type": "text",
                        "text": "Minha próxima viagem envolve voar de Seattle para Amsterdam. Vou passar alguns dias em Amsterdam antes de seguir para Paris para um voo de conexão para Roma."
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
      system: "Sua tarefa é analisar o texto fornecido e identificar quaisquer códigos de aeroporto mencionados nele. Apresente esses códigos de aeroporto como uma lista na ordem em que aparecem no texto. Se nenhum código de aeroporto for encontrado, retorne uma lista vazia.",
      messages: [
        {
          "role": "user",
          "content": [
            {
              "type": "text",
              "text": "Minha próxima viagem envolve voar de Seattle para Amsterdam. Vou passar alguns dias em Amsterdam antes de seguir para Paris para um voo de conexão para Roma."
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
        system="Sua tarefa é analisar o texto fornecido e identificar quaisquer códigos de aeroporto mencionados nele. Apresente esses códigos de aeroporto como uma lista na ordem em que aparecem no texto. Se nenhum código de aeroporto for encontrado, retorne uma lista vazia.",
        messages=[
            {
                "role": "user",
                "content": [
                    {
                        "type": "text",
                        "text": "Minha próxima viagem envolve voar de Seattle para Amsterdam. Vou passar alguns dias em Amsterdam antes de seguir para Paris para um voo de conexão para Roma."
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
      system: "Sua tarefa é analisar o texto fornecido e identificar quaisquer códigos de aeroporto mencionados nele. Apresente esses códigos de aeroporto como uma lista na ordem em que aparecem no texto. Se nenhum código de aeroporto for encontrado, retorne uma lista vazia.",
      messages: [
        {
          "role": "user",
          "content": [
            {
              "type": "text",
              "text": "Minha próxima viagem envolve voar de Seattle para Amsterdam. Vou passar alguns dias em Amsterdam antes de seguir para Paris para um voo de conexão para Roma."
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
        system="Sua tarefa é analisar o texto fornecido e identificar quaisquer códigos de aeroporto mencionados nele. Apresente esses códigos de aeroporto como uma lista na ordem em que aparecem no texto. Se nenhum código de aeroporto for encontrado, retorne uma lista vazia.",
        messages=[
            {
                "role": "user",
                "content": [
                    {
                        "type": "text",
                        "text": "Minha próxima viagem envolve voar de Seattle para Amsterdam. Vou passar alguns dias em Amsterdam antes de seguir para Paris para um voo de conexão para Roma."
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
      system: "Sua tarefa é analisar o texto fornecido e identificar quaisquer códigos de aeroporto mencionados nele. Apresente esses códigos de aeroporto como uma lista na ordem em que aparecem no texto. Se nenhum código de aeroporto for encontrado, retorne uma lista vazia.",
      messages: [
        {
          "role": "user",
          "content": [
            {
              "type": "text",
              "text": "Minha próxima viagem envolve voar de Seattle para Amsterdam. Vou passar alguns dias em Amsterdam antes de seguir para Paris para um voo de conexão para Roma."
            }
          ]
        }
      ]
    });
    console.log(msg);
    
    ```
</CodeGroup>