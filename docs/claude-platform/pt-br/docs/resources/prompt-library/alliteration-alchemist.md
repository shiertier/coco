# Alquimista da aliteração

Gere frases e sentenças aliterativas para qualquer assunto dado.

---

> Copie este prompt para nosso [Console](/dashboard) de desenvolvedor para experimentá-lo você mesmo!

|        | Conteúdo |
| --- | --- |
| Sistema | Sua tarefa é criar frases e sentenças aliterativas para o assunto dado. Certifique-se de que as aliterações não apenas soem agradáveis, mas também transmitam informações relevantes ou evoquem emoções apropriadas relacionadas ao assunto. |
| Usuário   | Oceano |

### Exemplo de saída

> 1.  Ondas sussurrantes se espalham sobre a costa.
> 2.  Borrifos salgados do mar respingam na areia.
> 3.  Correntes colidentes cortam penhascos costeiros.
> 4.  Profundezas profundas e escuras disfarçam perigos ocultos.
> 5.  Riachos borbulhantes trazem vida à praia.
> 6.  Mamíferos majestosos e poderosos migram através das águas.
> 7.  Marés trovejantes jogam navios de um lado para outro.
> 8.  Cardumes cintilantes abrigam criaturas marinhas secretas.
> 9.  Reflexos ondulantes revelam um mundo sob a superfície.
> 10. Mares rodopiantes e agitados moldam a linha costeira.
> 11. Espumas espumantes fluem livremente.
> 12. Gaivotas graciosas deslizam sobre o oceano reluzente.
> 13. Golfinhos brincalhões mergulham através das águas pristinas.
> 14. Brisas salgadas sopram através do azul sem limites.
> 15. Ondulações rítmicas rolam em direção à costa rochosa.
> 16. Cavernas de coral ocultam comunidades aquáticas coloridas.
> 17. Céus tempestuosos e mares inchados sinalizam o poder da natureza.
> 18. Marés tumultuosas transformam a baía tranquila.
> 19. Ventos rodopiantes e assobiantes açoitam as ondas.
> 20. Águas serenas e safira se estendem até o horizonte.

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
        temperature=1,
        system="Sua tarefa é criar frases e sentenças aliterativas para o assunto dado. Certifique-se de que as aliterações não apenas soem agradáveis, mas também transmitam informações relevantes ou evoquem emoções apropriadas relacionadas ao assunto.",
        messages=[
            {
                "role": "user",
                "content": [
                    {
                        "type": "text",
                        "text": "Oceano"
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
      system: "Sua tarefa é criar frases e sentenças aliterativas para o assunto dado. Certifique-se de que as aliterações não apenas soem agradáveis, mas também transmitam informações relevantes ou evoquem emoções apropriadas relacionadas ao assunto.",
      messages: [
        {
          "role": "user",
          "content": [
            {
              "type": "text",
              "text": "Oceano"
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
        system="Sua tarefa é criar frases e sentenças aliterativas para o assunto dado. Certifique-se de que as aliterações não apenas soem agradáveis, mas também transmitam informações relevantes ou evoquem emoções apropriadas relacionadas ao assunto.",
        messages=[
            {
                "role": "user",
                "content": [
                    {
                        "type": "text",
                        "text": "Oceano"
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
      system: "Sua tarefa é criar frases e sentenças aliterativas para o assunto dado. Certifique-se de que as aliterações não apenas soem agradáveis, mas também transmitam informações relevantes ou evoquem emoções apropriadas relacionadas ao assunto.",
      messages: [
        {
          "role": "user",
          "content": [
            {
              "type": "text",
              "text": "Oceano"
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
        system="Sua tarefa é criar frases e sentenças aliterativas para o assunto dado. Certifique-se de que as aliterações não apenas soem agradáveis, mas também transmitam informações relevantes ou evoquem emoções apropriadas relacionadas ao assunto.",
        messages=[
            {
                "role": "user",
                "content": [
                    {
                        "type": "text",
                        "text": "Oceano"
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
      system: "Sua tarefa é criar frases e sentenças aliterativas para o assunto dado. Certifique-se de que as aliterações não apenas soem agradáveis, mas também transmitam informações relevantes ou evoquem emoções apropriadas relacionadas ao assunto.",
      messages: [
        {
          "role": "user",
          "content": [
            {
              "type": "text",
              "text": "Oceano"
            }
          ]
        }
      ]
    });
    console.log(msg);
    
    ```
</CodeGroup>