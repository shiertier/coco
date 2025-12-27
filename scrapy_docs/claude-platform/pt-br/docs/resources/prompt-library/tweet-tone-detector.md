# Detector de tom de tweet

Detecte o tom e sentimento por trás dos tweets.

---

> Copie este prompt para nosso [Console](/dashboard) de desenvolvedor para experimentar você mesmo!

|        | Conteúdo |
| --- | --- |
| Sistema | Sua tarefa é analisar o tweet fornecido e identificar o tom principal e o sentimento expresso pelo autor. O tom deve ser classificado como um dos seguintes: Positivo, Negativo, Neutro, Humorístico, Sarcástico, Entusiasmado, Raivoso ou Informativo. O sentimento deve ser classificado como Positivo, Negativo ou Neutro. Forneça uma breve explicação para suas classificações, destacando as palavras-chave, frases, emoticons ou outros elementos que influenciaram sua decisão. |
| Usuário   | Uau, estou tão impressionado com o tratamento da empresa desta crise. 🙄 Eles realmente têm suas prioridades certas. #sarcasmo #falha |

### Exemplo de saída

> Tom: Sarcástico
> Sentimento: Negativo

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
        system="Sua tarefa é analisar o tweet fornecido e identificar o tom principal e o sentimento expresso pelo autor. O tom deve ser classificado como um dos seguintes: Positivo, Negativo, Neutro, Humorístico, Sarcástico, Entusiasmado, Raivoso ou Informativo. O sentimento deve ser classificado como Positivo, Negativo ou Neutro. Forneça uma breve explicação para suas classificações, destacando as palavras-chave, frases, emoticons ou outros elementos que influenciaram sua decisão.",
        messages=[
            {
                "role": "user",
                "content": [
                    {
                        "type": "text",
                        "text": "Uau, estou tão impressionado com o tratamento da empresa desta crise. 🙄 Eles realmente têm suas prioridades certas. #sarcasmo #falha"
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
      system: "Sua tarefa é analisar o tweet fornecido e identificar o tom principal e o sentimento expresso pelo autor. O tom deve ser classificado como um dos seguintes: Positivo, Negativo, Neutro, Humorístico, Sarcástico, Entusiasmado, Raivoso ou Informativo. O sentimento deve ser classificado como Positivo, Negativo ou Neutro. Forneça uma breve explicação para suas classificações, destacando as palavras-chave, frases, emoticons ou outros elementos que influenciaram sua decisão.",
      messages: [
        {
          "role": "user",
          "content": [
            {
              "type": "text",
              "text": "Uau, estou tão impressionado com o tratamento da empresa desta crise. 🙄 Eles realmente têm suas prioridades certas. #sarcasmo #falha"
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
        system="Sua tarefa é analisar o tweet fornecido e identificar o tom principal e o sentimento expresso pelo autor. O tom deve ser classificado como um dos seguintes: Positivo, Negativo, Neutro, Humorístico, Sarcástico, Entusiasmado, Raivoso ou Informativo. O sentimento deve ser classificado como Positivo, Negativo ou Neutro. Forneça uma breve explicação para suas classificações, destacando as palavras-chave, frases, emoticons ou outros elementos que influenciaram sua decisão.",
        messages=[
            {
                "role": "user",
                "content": [
                    {
                        "type": "text",
                        "text": "Uau, estou tão impressionado com o tratamento da empresa desta crise. 🙄 Eles realmente têm suas prioridades certas. #sarcasmo #falha"
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
      system: "Sua tarefa é analisar o tweet fornecido e identificar o tom principal e o sentimento expresso pelo autor. O tom deve ser classificado como um dos seguintes: Positivo, Negativo, Neutro, Humorístico, Sarcástico, Entusiasmado, Raivoso ou Informativo. O sentimento deve ser classificado como Positivo, Negativo ou Neutro. Forneça uma breve explicação para suas classificações, destacando as palavras-chave, frases, emoticons ou outros elementos que influenciaram sua decisão.",
      messages: [
        {
          "role": "user",
          "content": [
            {
              "type": "text",
              "text": "Uau, estou tão impressionado com o tratamento da empresa desta crise. 🙄 Eles realmente têm suas prioridades certas. #sarcasmo #falha"
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
        system="Sua tarefa é analisar o tweet fornecido e identificar o tom principal e o sentimento expresso pelo autor. O tom deve ser classificado como um dos seguintes: Positivo, Negativo, Neutro, Humorístico, Sarcástico, Entusiasmado, Raivoso ou Informativo. O sentimento deve ser classificado como Positivo, Negativo ou Neutro. Forneça uma breve explicação para suas classificações, destacando as palavras-chave, frases, emoticons ou outros elementos que influenciaram sua decisão.",
        messages=[
            {
                "role": "user",
                "content": [
                    {
                        "type": "text",
                        "text": "Uau, estou tão impressionado com o tratamento da empresa desta crise. 🙄 Eles realmente têm suas prioridades certas. #sarcasmo #falha"
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
      system: "Sua tarefa é analisar o tweet fornecido e identificar o tom principal e o sentimento expresso pelo autor. O tom deve ser classificado como um dos seguintes: Positivo, Negativo, Neutro, Humorístico, Sarcástico, Entusiasmado, Raivoso ou Informativo. O sentimento deve ser classificado como Positivo, Negativo ou Neutro. Forneça uma breve explicação para suas classificações, destacando as palavras-chave, frases, emoticons ou outros elementos que influenciaram sua decisão.",
      messages: [
        {
          "role": "user",
          "content": [
            {
              "type": "text",
              "text": "Uau, estou tão impressionado com o tratamento da empresa desta crise. 🙄 Eles realmente têm suas prioridades certas. #sarcasmo #falha"
            }
          ]
        }
      ]
    });
    console.log(msg);
    
    ```
</CodeGroup>