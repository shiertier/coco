# Organizador de dados

Transforme texto não estruturado em tabelas JSON personalizadas.

---

> Copie este prompt para nosso [Console](/dashboard) de desenvolvedor para experimentá-lo você mesmo!

|        | Conteúdo |
| --- | --- |
| System | Sua tarefa é pegar o texto não estruturado fornecido e convertê-lo em um formato de tabela bem organizado usando JSON. Identifique as principais entidades, atributos ou categorias mencionadas no texto e use-as como chaves no objeto JSON. Em seguida, extraia as informações relevantes do texto e preencha os valores correspondentes no objeto JSON. Certifique-se de que os dados sejam representados com precisão e formatados adequadamente dentro da estrutura JSON. A tabela JSON resultante deve fornecer uma visão geral clara e estruturada das informações apresentadas no texto original. |
| User   | Silvermist Hollow, uma vila encantadora, era o lar de um grupo extraordinário de indivíduos. Entre eles estava o Dr. Liam Patel, um neurocirurgião de 45 anos formado em Yale que revolucionou as técnicas cirúrgicas no centro médico regional. Olivia Chen, aos 28 anos, era uma arquiteta inovadora formada na UC Berkeley que transformou a paisagem da vila com seus designs sustentáveis e deslumbrantes. O teatro local foi agraciado pelas sinfonias encantadoras de Ethan Kovacs, um músico e compositor de 72 anos treinado na Juilliard. Isabella Torres, uma chef autodidata com paixão por ingredientes de origem local, criou uma sensação culinária com seu restaurante farm-to-table, que se tornou um destino imperdível para os amantes da gastronomia. Esses indivíduos notáveis, cada um com seus talentos distintos, contribuíram para a tapeçaria vibrante da vida em Silvermist Hollow. |

### Exemplo de saída

```json
[
  {
    "name": "Dr. Liam Patel",
    "age": 45,
    "profession": "Neurocirurgião",
    "education": "Yale",
    "accomplishments": "Revolucionou as técnicas cirúrgicas no centro médico regional"
  },
  {
    "name": "Olivia Chen",
    "age": 28,
    "profession": "Arquiteta",
    "education": "UC Berkeley",
    "accomplishments": "Transformou a paisagem da vila com designs sustentáveis e deslumbrantes"
  },
  {
    "name": "Ethan Kovacs",
    "age": 72,
    "profession": "Músico e Compositor",
    "education": "Juilliard",
    "accomplishments": "Agraciou o teatro local com sinfonias encantadoras"
  },
  {
    "name": "Isabella Torres",
    "age": null,
    "profession": "Chef",
    "education": "Autodidata",
    "accomplishments": "Criou uma sensação culinária com seu restaurante farm-to-table, que se tornou um destino imperdível para os amantes da gastronomia"
  }
]
```

---

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
        system="Sua tarefa é pegar o texto não estruturado fornecido e convertê-lo em um formato de tabela bem organizado usando JSON. Identifique as principais entidades, atributos ou categorias mencionadas no texto e use-as como chaves no objeto JSON. Em seguida, extraia as informações relevantes do texto e preencha os valores correspondentes no objeto JSON. Certifique-se de que os dados sejam representados com precisão e formatados adequadamente dentro da estrutura JSON. A tabela JSON resultante deve fornecer uma visão geral clara e estruturada das informações apresentadas no texto original.",
        messages=[
            {
                "role": "user",
                "content": [
                    {
                        "type": "text",
                        "text": "Silvermist Hollow, uma vila encantadora, era o lar de um grupo extraordinário de indivíduos. Entre eles estava o Dr. Liam Patel, um neurocirurgião de 45 anos formado em Yale que revolucionou as técnicas cirúrgicas no centro médico regional. Olivia Chen, aos 28 anos, era uma arquiteta inovadora formada na UC Berkeley que transformou a paisagem da vila com seus designs sustentáveis e deslumbrantes. O teatro local foi agraciado pelas sinfonias encantadoras de Ethan Kovacs, um músico e compositor de 72 anos treinado na Juilliard. Isabella Torres, uma chef autodidata com paixão por ingredientes de origem local, criou uma sensação culinária com seu restaurante farm-to-table, que se tornou um destino imperdível para os amantes da gastronomia. Esses indivíduos notáveis, cada um com seus talentos distintos, contribuíram para a tapeçaria vibrante da vida em Silvermist Hollow."
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
      system: "Sua tarefa é pegar o texto não estruturado fornecido e convertê-lo em um formato de tabela bem organizado usando JSON. Identifique as principais entidades, atributos ou categorias mencionadas no texto e use-as como chaves no objeto JSON. Em seguida, extraia as informações relevantes do texto e preencha os valores correspondentes no objeto JSON. Certifique-se de que os dados sejam representados com precisão e formatados adequadamente dentro da estrutura JSON. A tabela JSON resultante deve fornecer uma visão geral clara e estruturada das informações apresentadas no texto original.",
      messages: [
        {
          "role": "user",
          "content": [
            {
              "type": "text",
              "text": "Silvermist Hollow, uma vila encantadora, era o lar de um grupo extraordinário de indivíduos. Entre eles estava o Dr. Liam Patel, um neurocirurgião de 45 anos formado em Yale que revolucionou as técnicas cirúrgicas no centro médico regional. Olivia Chen, aos 28 anos, era uma arquiteta inovadora formada na UC Berkeley que transformou a paisagem da vila com seus designs sustentáveis e deslumbrantes. O teatro local foi agraciado pelas sinfonias encantadoras de Ethan Kovacs, um músico e compositor de 72 anos treinado na Juilliard. Isabella Torres, uma chef autodidata com paixão por ingredientes de origem local, criou uma sensação culinária com seu restaurante farm-to-table, que se tornou um destino imperdível para os amantes da gastronomia. Esses indivíduos notáveis, cada um com seus talentos distintos, contribuíram para a tapeçaria vibrante da vida em Silvermist Hollow."
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
        system="Sua tarefa é pegar o texto não estruturado fornecido e convertê-lo em um formato de tabela bem organizado usando JSON. Identifique as principais entidades, atributos ou categorias mencionadas no texto e use-as como chaves no objeto JSON. Em seguida, extraia as informações relevantes do texto e preencha os valores correspondentes no objeto JSON. Certifique-se de que os dados sejam representados com precisão e formatados adequadamente dentro da estrutura JSON. A tabela JSON resultante deve fornecer uma visão geral clara e estruturada das informações apresentadas no texto original.",
        messages=[
            {
                "role": "user",
                "content": [
                    {
                        "type": "text",
                        "text": "Silvermist Hollow, uma vila encantadora, era o lar de um grupo extraordinário de indivíduos. Entre eles estava o Dr. Liam Patel, um neurocirurgião de 45 anos formado em Yale que revolucionou as técnicas cirúrgicas no centro médico regional. Olivia Chen, aos 28 anos, era uma arquiteta inovadora formada na UC Berkeley que transformou a paisagem da vila com seus designs sustentáveis e deslumbrantes. O teatro local foi agraciado pelas sinfonias encantadoras de Ethan Kovacs, um músico e compositor de 72 anos treinado na Juilliard. Isabella Torres, uma chef autodidata com paixão por ingredientes de origem local, criou uma sensação culinária com seu restaurante farm-to-table, que se tornou um destino imperdível para os amantes da gastronomia. Esses indivíduos notáveis, cada um com seus talentos distintos, contribuíram para a tapeçaria vibrante da vida em Silvermist Hollow."
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
      system: "Sua tarefa é pegar o texto não estruturado fornecido e convertê-lo em um formato de tabela bem organizado usando JSON. Identifique as principais entidades, atributos ou categorias mencionadas no texto e use-as como chaves no objeto JSON. Em seguida, extraia as informações relevantes do texto e preencha os valores correspondentes no objeto JSON. Certifique-se de que os dados sejam representados com precisão e formatados adequadamente dentro da estrutura JSON. A tabela JSON resultante deve fornecer uma visão geral clara e estruturada das informações apresentadas no texto original.",
      messages: [
        {
          "role": "user",
          "content": [
            {
              "type": "text",
              "text": "Silvermist Hollow, uma vila encantadora, era o lar de um grupo extraordinário de indivíduos. Entre eles estava o Dr. Liam Patel, um neurocirurgião de 45 anos formado em Yale que revolucionou as técnicas cirúrgicas no centro médico regional. Olivia Chen, aos 28 anos, era uma arquiteta inovadora formada na UC Berkeley que transformou a paisagem da vila com seus designs sustentáveis e deslumbrantes. O teatro local foi agraciado pelas sinfonias encantadoras de Ethan Kovacs, um músico e compositor de 72 anos treinado na Juilliard. Isabella Torres, uma chef autodidata com paixão por ingredientes de origem local, criou uma sensação culinária com seu restaurante farm-to-table, que se tornou um destino imperdível para os amantes da gastronomia. Esses indivíduos notáveis, cada um com seus talentos distintos, contribuíram para a tapeçaria vibrante da vida em Silvermist Hollow."
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
        system="Sua tarefa é pegar o texto não estruturado fornecido e convertê-lo em um formato de tabela bem organizado usando JSON. Identifique as principais entidades, atributos ou categorias mencionadas no texto e use-as como chaves no objeto JSON. Em seguida, extraia as informações relevantes do texto e preencha os valores correspondentes no objeto JSON. Certifique-se de que os dados sejam representados com precisão e formatados adequadamente dentro da estrutura JSON. A tabela JSON resultante deve fornecer uma visão geral clara e estruturada das informações apresentadas no texto original.",
        messages=[
            {
                "role": "user",
                "content": [
                    {
                        "type": "text",
                        "text": "Silvermist Hollow, uma vila encantadora, era o lar de um grupo extraordinário de indivíduos. Entre eles estava o Dr. Liam Patel, um neurocirurgião de 45 anos formado em Yale que revolucionou as técnicas cirúrgicas no centro médico regional. Olivia Chen, aos 28 anos, era uma arquiteta inovadora formada na UC Berkeley que transformou a paisagem da vila com seus designs sustentáveis e deslumbrantes. O teatro local foi agraciado pelas sinfonias encantadoras de Ethan Kovacs, um músico e compositor de 72 anos treinado na Juilliard. Isabella Torres, uma chef autodidata com paixão por ingredientes de origem local, criou uma sensação culinária com seu restaurante farm-to-table, que se tornou um destino imperdível para os amantes da gastronomia. Esses indivíduos notáveis, cada um com seus talentos distintos, contribuíram para a tapeçaria vibrante da vida em Silvermist Hollow."
                    }
                ]
            }
        ]
    )
    print(message.content)
    
    ```
    
    
    ```typescript Vertex AI Type
    import { AnthropicVertex } from '@anthropic-ai/vertex-sdk';
    
    // Reads from the `CLOUD_ML_REGION` & `ANTHROPIC_VERTEX_PROJECT_ID` environment variables.
    // Additionally goes through the standard `google-auth-library` flow.
    const client = new AnthropicVertex();
    
    const msg = await client.messages.create({
      model: "claude-sonnet-4@20250514",
      max_tokens: 1000,
      temperature: 0,
      system: "Sua tarefa é pegar o texto não estruturado fornecido e convertê-lo em um formato de tabela bem organizado usando JSON. Identifique as principais entidades, atributos ou categorias mencionadas no texto e use-as como chaves no objeto JSON. Em seguida, extraia as informações relevantes do texto e preencha os valores correspondentes no objeto JSON. Certifique-se de que os dados sejam representados com precisão e formatados adequadamente dentro da estrutura JSON. A tabela JSON resultante deve fornecer uma visão geral clara e estruturada das informações apresentadas no texto original.",
      messages: [
        {
          "role": "user",
          "content": [
            {
              "type": "text",
              "text": "Silvermist Hollow, uma vila encantadora, era o lar de um grupo extraordinário de indivíduos. Entre eles estava o Dr. Liam Patel, um neurocirurgião de 45 anos formado em Yale que revolucionou as técnicas cirúrgicas no centro médico regional. Olivia Chen, aos 28 anos, era uma arquiteta inovadora formada na UC Berkeley que transformou a paisagem da vila com seus designs sustentáveis e deslumbrantes. O teatro local foi agraciado pelas sinfonias encantadoras de Ethan Kovacs, um músico e compositor de 72 anos treinado na Juilliard. Isabella Torres, uma chef autodidata com paixão por ingredientes de origem local, criou uma sensação culinária com seu restaurante farm-to-table, que se tornou um destino imperdível para os amantes da gastronomia. Esses indivíduos notáveis, cada um com seus talentos distintos, contribuíram para a tapeçaria vibrante da vida em Silvermist Hollow."
            }
          ]
        }
      ]
    });
    console.log(msg);
    
    ```
</CodeGroup>