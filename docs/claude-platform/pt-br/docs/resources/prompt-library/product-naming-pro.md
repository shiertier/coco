# Especialista em nomes de produtos

Crie nomes cativantes de produtos a partir de descrições e palavras-chave.

---

> Copie este prompt para o nosso [Console](/dashboard) de desenvolvedor para experimentá-lo você mesmo!

|        | Conteúdo |
| --- | --- |
| System | Sua tarefa é gerar nomes de produtos criativos, memoráveis e comercializáveis com base na descrição e palavras-chave fornecidas. Os nomes dos produtos devem ser concisos (2-4 palavras), evocativos e facilmente compreendidos pelo público-alvo. Evite nomes genéricos ou excessivamente literais. Em vez disso, procure criar um nome que se destaque, capture a essência do produto e deixe uma impressão duradoura. |
| User   | Descrição: Um fone de ouvido over-ear sem fio com cancelamento de ruído, com bateria de 20 horas e controles por toque. Projetado para audiófilos e viajantes frequentes. Palavras-chave: imersivo, confortável, alta fidelidade, duradouro, conveniente |

## Exemplo de saída

> 1. SoundOasis Elite
>    2.AudioZen Nomad
>    3.Serenity Pro
>    4.Tranquility Touch
>    5.Harmonix Journey
>    6.SonicSolace Roam
>    7.Auditory Bliss
>    8.Quietude Quest
>    9.Euphony Excursion
>    10.Acoustica Wanderlust

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
        temperature=1,
        system="Sua tarefa é gerar nomes de produtos criativos, memoráveis e comercializáveis com base na descrição e palavras-chave fornecidas. Os nomes dos produtos devem ser concisos (2-4 palavras), evocativos e facilmente compreendidos pelo público-alvo. Evite nomes genéricos ou excessivamente literais. Em vez disso, procure criar um nome que se destaque, capture a essência do produto e deixe uma impressão duradoura.",
        messages=[
            {
                "role": "user",
                "content": [
                    {
                        "type": "text",
                        "text": "Descrição: Um fone de ouvido over-ear sem fio com cancelamento de ruído, com bateria de 20 horas e controles por toque. Projetado para audiófilos e viajantes frequentes.  \n  \nPalavras-chave: imersivo, confortável, alta fidelidade, duradouro, conveniente"
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
      temperature: 1,
      system: "Sua tarefa é gerar nomes de produtos criativos, memoráveis e comercializáveis com base na descrição e palavras-chave fornecidas. Os nomes dos produtos devem ser concisos (2-4 palavras), evocativos e facilmente compreendidos pelo público-alvo. Evite nomes genéricos ou excessivamente literais. Em vez disso, procure criar um nome que se destaque, capture a essência do produto e deixe uma impressão duradoura.",
      messages: [
        {
          "role": "user",
          "content": [
            {
              "type": "text",
              "text": "Descrição: Um fone de ouvido over-ear sem fio com cancelamento de ruído, com bateria de 20 horas e controles por toque. Projetado para audiófilos e viajantes frequentes.  \n  \nPalavras-chave: imersivo, confortável, alta fidelidade, duradouro, conveniente"
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
        temperature=1,
        system="Sua tarefa é gerar nomes de produtos criativos, memoráveis e comercializáveis com base na descrição e palavras-chave fornecidas. Os nomes dos produtos devem ser concisos (2-4 palavras), evocativos e facilmente compreendidos pelo público-alvo. Evite nomes genéricos ou excessivamente literais. Em vez disso, procure criar um nome que se destaque, capture a essência do produto e deixe uma impressão duradoura.",
        messages=[
            {
                "role": "user",
                "content": [
                    {
                        "type": "text",
                        "text": "Descrição: Um fone de ouvido over-ear sem fio com cancelamento de ruído, com bateria de 20 horas e controles por toque. Projetado para audiófilos e viajantes frequentes.  \n  \nPalavras-chave: imersivo, confortável, alta fidelidade, duradouro, conveniente"
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
      temperature: 1,
      system: "Sua tarefa é gerar nomes de produtos criativos, memoráveis e comercializáveis com base na descrição e palavras-chave fornecidas. Os nomes dos produtos devem ser concisos (2-4 palavras), evocativos e facilmente compreendidos pelo público-alvo. Evite nomes genéricos ou excessivamente literais. Em vez disso, procure criar um nome que se destaque, capture a essência do produto e deixe uma impressão duradoura.",
      messages: [
        {
          "role": "user",
          "content": [
            {
              "type": "text",
              "text": "Descrição: Um fone de ouvido over-ear sem fio com cancelamento de ruído, com bateria de 20 horas e controles por toque. Projetado para audiófilos e viajantes frequentes.  \n  \nPalavras-chave: imersivo, confortável, alta fidelidade, duradouro, conveniente"
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
        temperature=1,
        system="Sua tarefa é gerar nomes de produtos criativos, memoráveis e comercializáveis com base na descrição e palavras-chave fornecidas. Os nomes dos produtos devem ser concisos (2-4 palavras), evocativos e facilmente compreendidos pelo público-alvo. Evite nomes genéricos ou excessivamente literais. Em vez disso, procure criar um nome que se destaque, capture a essência do produto e deixe uma impressão duradoura.",
        messages=[
            {
                "role": "user",
                "content": [
                    {
                        "type": "text",
                        "text": "Descrição: Um fone de ouvido over-ear sem fio com cancelamento de ruído, com bateria de 20 horas e controles por toque. Projetado para audiófilos e viajantes frequentes.\n\nPalavras-chave: imersivo, confortável, alta fidelidade, duradouro, conveniente"
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
      temperature: 1,
      system: "Sua tarefa é gerar nomes de produtos criativos, memoráveis e comercializáveis com base na descrição e palavras-chave fornecidas. Os nomes dos produtos devem ser concisos (2-4 palavras), evocativos e facilmente compreendidos pelo público-alvo. Evite nomes genéricos ou excessivamente literais. Em vez disso, procure criar um nome que se destaque, capture a essência do produto e deixe uma impressão duradoura.",
      messages: [
        {
          "role": "user",
          "content": [
            {
              "type": "text",
              "text": "Descrição: Um fone de ouvido over-ear sem fio com cancelamento de ruído, com bateria de 20 horas e controles por toque. Projetado para audiófilos e viajantes frequentes.\n\nPalavras-chave: imersivo, confortável, alta fidelidade, duradouro, conveniente"
            }
          ]
        }
      ]
    });
    console.log(msg);
    
    ```
</CodeGroup>