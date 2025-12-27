# Extrator de email

Extrair endereços de email de um documento em uma lista formatada em JSON.

---

> Copie este prompt para nosso [Console](/dashboard) de desenvolvedor para experimentá-lo você mesmo!

|        | Conteúdo |
| --- | --- |
| Sistema | Copie precisamente quaisquer endereços de email do seguinte texto e então escreva-os, um por linha. Escreva apenas um endereço de email se ele estiver precisamente escrito no texto de entrada. Se não houver endereços de email no texto, escreva "N/A". Não diga mais nada. |
| Usuário   | Diretório Telefônico: John Latrabe, 555-232-1995, [john909709@geemail.com] Josie Lana, 555-759-2905, [josie@josielananier.com] Keven Stevens, 555-980-7000, [drkevin22@geemail.com] O diretório telefônico será mantido atualizado pelo gerente de RH.                      |

### Exemplo de saída

> john909709@geemail.com > josie@josielananier.com > drkevin22@geemail.com

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
        max_tokens=1000,
        temperature=0,
        system="Copie precisamente quaisquer endereços de email do seguinte texto e então escreva-os, um por linha. Escreva apenas um endereço de email se ele estiver precisamente escrito no texto de entrada. Se não houver endereços de email no texto, escreva \"N/A\".  Não diga mais nada.",
        messages=[
            {
                "role": "user",
                "content": [
                    {
                        "type": "text",
                        "text": "Diretório Telefônico:  \nJohn Latrabe, 555-232-1995, [[email protected]]  \nJosie Lana, 555-759-2905, [[email protected]]  \nKeven Stevens, 555-980-7000, [[email protected]]  \n  \nO diretório telefônico será mantido atualizado pelo gerente de RH."
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
      system: "Copie precisamente quaisquer endereços de email do seguinte texto e então escreva-os, um por linha. Escreva apenas um endereço de email se ele estiver precisamente escrito no texto de entrada. Se não houver endereços de email no texto, escreva \"N/A\".  Não diga mais nada.",
      messages: [
        {
          "role": "user",
          "content": [
            {
              "type": "text",
              "text": "Diretório Telefônico:  \nJohn Latrabe, 555-232-1995, [[email protected]]  \nJosie Lana, 555-759-2905, [[email protected]]  \nKeven Stevens, 555-980-7000, [[email protected]]  \n  \nO diretório telefônico será mantido atualizado pelo gerente de RH."
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
        system="Copie precisamente quaisquer endereços de email do seguinte texto e então escreva-os, um por linha. Escreva apenas um endereço de email se ele estiver precisamente escrito no texto de entrada. Se não houver endereços de email no texto, escreva \"N/A\".  Não diga mais nada.",
        messages=[
            {
                "role": "user",
                "content": [
                    {
                        "type": "text",
                        "text": "Diretório Telefônico:  \nJohn Latrabe, 555-232-1995, [[email protected]]  \nJosie Lana, 555-759-2905, [[email protected]]  \nKeven Stevens, 555-980-7000, [[email protected]]  \n  \nO diretório telefônico será mantido atualizado pelo gerente de RH."
                    }
                ]
            }
        ]
    )
    print(message.content)
    
    ```
    
    
    ```typescript AWS Bedrock TypeScript
    import AnthropicBedrock from "@anthropic-ai/bedrock-sdk";
    
    # Veja https://docs.claude.com/claude/reference/claude-on-amazon-bedrock
    # para opções de autenticação
    const client = new AnthropicBedrock();
    
    const msg = await client.messages.create({
      model: "anthropic.claude-sonnet-4-5-20250929-v1:0",
      max_tokens: 1000,
      temperature: 0,
      system: "Copie precisamente quaisquer endereços de email do seguinte texto e então escreva-os, um por linha. Escreva apenas um endereço de email se ele estiver precisamente escrito no texto de entrada. Se não houver endereços de email no texto, escreva \"N/A\".  Não diga mais nada.",
      messages: [
        {
          "role": "user",
          "content": [
            {
              "type": "text",
              "text": "Diretório Telefônico:  \nJohn Latrabe, 555-232-1995, [[email protected]]  \nJosie Lana, 555-759-2905, [[email protected]]  \nKeven Stevens, 555-980-7000, [[email protected]]  \n  \nO diretório telefônico será mantido atualizado pelo gerente de RH."
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
        system="Copie precisamente quaisquer endereços de email do seguinte texto e então escreva-os, um por linha. Escreva apenas um endereço de email se ele estiver precisamente escrito no texto de entrada. Se não houver endereços de email no texto, escreva \"N/A\".  Não diga mais nada.",
        messages=[
            {
                "role": "user",
                "content": [
                    {
                        "type": "text",
                        "text": "Diretório Telefônico:  \nJohn Latrabe, 555-232-1995, [[email protected]]  \nJosie Lana, 555-759-2905, [[email protected]]  \nKeven Stevens, 555-980-7000, [[email protected]]  \n  \nO diretório telefônico será mantido atualizado pelo gerente de RH."
                    }
                ]
            }
        ]
    )
    print(message.content)
    
    ```
    
    
    ```typescript Vertex AI TypeScript
    import { AnthropicVertex } from '@anthropic-ai/vertex-sdk';
    
    # Lê das variáveis de ambiente `CLOUD_ML_REGION` & `ANTHROPIC_VERTEX_PROJECT_ID`.
    # Adicionalmente passa pelo fluxo padrão da `google-auth-library`.
    const client = new AnthropicVertex();
    
    const msg = await client.messages.create({
      model: "claude-sonnet-4@20250514",
      max_tokens: 1000,
      temperature: 0,
      system: "Copie precisamente quaisquer endereços de email do seguinte texto e então escreva-os, um por linha. Escreva apenas um endereço de email se ele estiver precisamente escrito no texto de entrada. Se não houver endereços de email no texto, escreva \"N/A\".  Não diga mais nada.",
      messages: [
        {
          "role": "user",
          "content": [
            {
              "type": "text",
              "text": "Diretório Telefônico:  \nJohn Latrabe, 555-232-1995, [[email protected]]  \nJosie Lana, 555-759-2905, [[email protected]]  \nKeven Stevens, 555-980-7000, [[email protected]]  \n  \nO diretório telefônico será mantido atualizado pelo gerente de RH."
            }
          ]
        }
      ]
    });
    console.log(msg);
    
    ```
</CodeGroup>