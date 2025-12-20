# Detector de tono de tweets

Detecta el tono y sentimiento detrás de los tweets.

---

> ¡Copia este prompt en nuestra [Consola](/dashboard) de desarrollador para probarlo tú mismo!

|        | Contenido |
| --- | --- |
| System | Tu tarea es analizar el tweet proporcionado e identificar el tono principal y el sentimiento expresado por el autor. El tono debe clasificarse como uno de los siguientes: Positivo, Negativo, Neutral, Humorístico, Sarcástico, Entusiasta, Enojado, o Informativo. El sentimiento debe clasificarse como Positivo, Negativo, o Neutral. Proporciona una breve explicación para tus clasificaciones, destacando las palabras clave, frases, emoticones, u otros elementos que influyeron en tu decisión. |
| User   | Wow, estoy tan impresionado por el manejo de esta crisis por parte de la empresa. 🙄 Realmente tienen sus prioridades claras. #sarcasmo #fracaso |

### Ejemplo de salida

> Tono: Sarcástico
> Sentimiento: Negativo

### Solicitud de API

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
        system="Tu tarea es analizar el tweet proporcionado e identificar el tono principal y el sentimiento expresado por el autor. El tono debe clasificarse como uno de los siguientes: Positivo, Negativo, Neutral, Humorístico, Sarcástico, Entusiasta, Enojado, o Informativo. El sentimiento debe clasificarse como Positivo, Negativo, o Neutral. Proporciona una breve explicación para tus clasificaciones, destacando las palabras clave, frases, emoticones, u otros elementos que influyeron en tu decisión.",
        messages=[
            {
                "role": "user",
                "content": [
                    {
                        "type": "text",
                        "text": "Wow, estoy tan impresionado por el manejo de esta crisis por parte de la empresa. 🙄 Realmente tienen sus prioridades claras. #sarcasmo #fracaso"
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
      system: "Tu tarea es analizar el tweet proporcionado e identificar el tono principal y el sentimiento expresado por el autor. El tono debe clasificarse como uno de los siguientes: Positivo, Negativo, Neutral, Humorístico, Sarcástico, Entusiasta, Enojado, o Informativo. El sentimiento debe clasificarse como Positivo, Negativo, o Neutral. Proporciona una breve explicación para tus clasificaciones, destacando las palabras clave, frases, emoticones, u otros elementos que influyeron en tu decisión.",
      messages: [
        {
          "role": "user",
          "content": [
            {
              "type": "text",
              "text": "Wow, estoy tan impresionado por el manejo de esta crisis por parte de la empresa. 🙄 Realmente tienen sus prioridades claras. #sarcasmo #fracaso"
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
        system="Tu tarea es analizar el tweet proporcionado e identificar el tono principal y el sentimiento expresado por el autor. El tono debe clasificarse como uno de los siguientes: Positivo, Negativo, Neutral, Humorístico, Sarcástico, Entusiasta, Enojado, o Informativo. El sentimiento debe clasificarse como Positivo, Negativo, o Neutral. Proporciona una breve explicación para tus clasificaciones, destacando las palabras clave, frases, emoticones, u otros elementos que influyeron en tu decisión.",
        messages=[
            {
                "role": "user",
                "content": [
                    {
                        "type": "text",
                        "text": "Wow, estoy tan impresionado por el manejo de esta crisis por parte de la empresa. 🙄 Realmente tienen sus prioridades claras. #sarcasmo #fracaso"
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
      system: "Tu tarea es analizar el tweet proporcionado e identificar el tono principal y el sentimiento expresado por el autor. El tono debe clasificarse como uno de los siguientes: Positivo, Negativo, Neutral, Humorístico, Sarcástico, Entusiasta, Enojado, o Informativo. El sentimiento debe clasificarse como Positivo, Negativo, o Neutral. Proporciona una breve explicación para tus clasificaciones, destacando las palabras clave, frases, emoticones, u otros elementos que influyeron en tu decisión.",
      messages: [
        {
          "role": "user",
          "content": [
            {
              "type": "text",
              "text": "Wow, estoy tan impresionado por el manejo de esta crisis por parte de la empresa. 🙄 Realmente tienen sus prioridades claras. #sarcasmo #fracaso"
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
        system="Tu tarea es analizar el tweet proporcionado e identificar el tono principal y el sentimiento expresado por el autor. El tono debe clasificarse como uno de los siguientes: Positivo, Negativo, Neutral, Humorístico, Sarcástico, Entusiasta, Enojado, o Informativo. El sentimiento debe clasificarse como Positivo, Negativo, o Neutral. Proporciona una breve explicación para tus clasificaciones, destacando las palabras clave, frases, emoticones, u otros elementos que influyeron en tu decisión.",
        messages=[
            {
                "role": "user",
                "content": [
                    {
                        "type": "text",
                        "text": "Wow, estoy tan impresionado por el manejo de esta crisis por parte de la empresa. 🙄 Realmente tienen sus prioridades claras. #sarcasmo #fracaso"
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
      system: "Tu tarea es analizar el tweet proporcionado e identificar el tono principal y el sentimiento expresado por el autor. El tono debe clasificarse como uno de los siguientes: Positivo, Negativo, Neutral, Humorístico, Sarcástico, Entusiasta, Enojado, o Informativo. El sentimiento debe clasificarse como Positivo, Negativo, o Neutral. Proporciona una breve explicación para tus clasificaciones, destacando las palabras clave, frases, emoticones, u otros elementos que influyeron en tu decisión.",
      messages: [
        {
          "role": "user",
          "content": [
            {
              "type": "text",
              "text": "Wow, estoy tan impresionado por el manejo de esta crisis por parte de la empresa. 🙄 Realmente tienen sus prioridades claras. #sarcasmo #fracaso"
            }
          ]
        }
      ]
    });
    console.log(msg);
    
    ```
</CodeGroup>