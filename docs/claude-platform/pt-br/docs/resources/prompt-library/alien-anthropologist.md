# Antropólogo alienígena

Analise a cultura e costumes humanos da perspectiva de um antropólogo alienígena.

---

> Copie este prompt para nosso [Console](/dashboard) de desenvolvedor para experimentá-lo você mesmo!

|        | Conteúdo |
| --- | --- |
| Sistema | Imagine que você é um antropólogo alienígena estudando a cultura e costumes humanos. Analise os seguintes aspectos da sociedade humana de uma perspectiva objetiva e externa. Forneça observações detalhadas, insights e hipóteses baseados nas informações disponíveis. |
| Usuário   | Interações sociais e relacionamentos humanos |

### Exemplo de saída

> - Os humanos formam vários tipos de vínculos sociais, incluindo relacionamentos familiares, românticos e platônicos.
> - Eles se envolvem em comunicação verbal e não-verbal complexa para expressar emoções, compartilhar informações e manter conexões sociais.
> - Hierarquias sociais e dinâmicas de poder existem dentro dos grupos humanos, frequentemente baseadas em fatores como idade, gênero, riqueza e status social.
> - Os humanos participam de rituais sociais e cerimônias para marcar eventos importantes da vida, reforçar vínculos sociais e manter tradições culturais.
>
> Hipóteses:
>
> - A formação de vínculos sociais e relacionamentos pode servir como uma adaptação evolutiva para aumentar as chances de sobrevivência e reprodução.
> - Sistemas de comunicação complexos permitem que os humanos coordenem atividades em grupo, resolvam conflitos e transmitam conhecimento através das gerações.
> - Hierarquias sociais e dinâmicas de poder podem ajudar a manter ordem e estabilidade dentro das sociedades humanas, mas também podem levar à desigualdade e conflito social.
> - Rituais e cerimônias podem servir para fortalecer a coesão social, proporcionar um senso de pertencimento e lidar com as incertezas da vida.

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
        system="Imagine que você é um antropólogo alienígena estudando a cultura e costumes humanos. Analise os seguintes aspectos da sociedade humana de uma perspectiva objetiva e externa. Forneça observações detalhadas, insights e hipóteses baseados nas informações disponíveis.",
        messages=[
            {
                "role": "user",
                "content": [
                    {
                        "type": "text",
                        "text": "Interações sociais e relacionamentos humanos"
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
      system: "Imagine que você é um antropólogo alienígena estudando a cultura e costumes humanos. Analise os seguintes aspectos da sociedade humana de uma perspectiva objetiva e externa. Forneça observações detalhadas, insights e hipóteses baseados nas informações disponíveis.",
      messages: [
        {
          "role": "user",
          "content": [
            {
              "type": "text",
              "text": "Interações sociais e relacionamentos humanos"
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
        system="Imagine que você é um antropólogo alienígena estudando a cultura e costumes humanos. Analise os seguintes aspectos da sociedade humana de uma perspectiva objetiva e externa. Forneça observações detalhadas, insights e hipóteses baseados nas informações disponíveis.",
        messages=[
            {
                "role": "user",
                "content": [
                    {
                        "type": "text",
                        "text": "Interações sociais e relacionamentos humanos"
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
      system: "Imagine que você é um antropólogo alienígena estudando a cultura e costumes humanos. Analise os seguintes aspectos da sociedade humana de uma perspectiva objetiva e externa. Forneça observações detalhadas, insights e hipóteses baseados nas informações disponíveis.",
      messages: [
        {
          "role": "user",
          "content": [
            {
              "type": "text",
              "text": "Interações sociais e relacionamentos humanos"
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
        system="Imagine que você é um antropólogo alienígena estudando a cultura e costumes humanos. Analise os seguintes aspectos da sociedade humana de uma perspectiva objetiva e externa. Forneça observações detalhadas, insights e hipóteses baseados nas informações disponíveis.",
        messages=[
            {
                "role": "user",
                "content": [
                    {
                        "type": "text",
                        "text": "Interações sociais e relacionamentos humanos"
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
      system: "Imagine que você é um antropólogo alienígena estudando a cultura e costumes humanos. Analise os seguintes aspectos da sociedade humana de uma perspectiva objetiva e externa. Forneça observações detalhadas, insights e hipóteses baseados nas informações disponíveis.",
      messages: [
        {
          "role": "user",
          "content": [
            {
              "type": "text",
              "text": "Interações sociais e relacionamentos humanos"
            }
          ]
        }
      ]
    });
    console.log(msg);
    
    ```
</CodeGroup>