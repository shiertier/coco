# Experto en nombres de productos

Crea nombres atractivos para productos a partir de descripciones y palabras clave.

---

> ¡Copia este prompt en nuestra [Consola](/dashboard) de desarrollador para probarlo tú mismo!

|        | Contenido |
| --- | --- |
| Sistema | Tu tarea es generar nombres de productos creativos, memorables y comercializables basados en la descripción y palabras clave proporcionadas. Los nombres de productos deben ser concisos (2-4 palabras), evocadores y fácilmente entendidos por la audiencia objetivo. Evita nombres genéricos o demasiado literales. En su lugar, busca crear un nombre que destaque, capture la esencia del producto y deje una impresión duradera. |
| Usuario   | Descripción: Auriculares inalámbricos supraaurales con cancelación de ruido, con 20 horas de duración de batería y controles táctiles. Diseñados para audiófilos y viajeros frecuentes. Palabras clave: inmersivo, cómodo, alta fidelidad, duradero, conveniente |

## Ejemplo de salida

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
        temperature=1,
        system="Tu tarea es generar nombres de productos creativos, memorables y comercializables basados en la descripción y palabras clave proporcionadas. Los nombres de productos deben ser concisos (2-4 palabras), evocadores y fácilmente entendidos por la audiencia objetivo. Evita nombres genéricos o demasiado literales. En su lugar, busca crear un nombre que destaque, capture la esencia del producto y deje una impresión duradera.",
        messages=[
            {
                "role": "user",
                "content": [
                    {
                        "type": "text",
                        "text": "Descripción: Auriculares inalámbricos supraaurales con cancelación de ruido, con 20 horas de duración de batería y controles táctiles. Diseñados para audiófilos y viajeros frecuentes.  \n  \nPalabras clave: inmersivo, cómodo, alta fidelidad, duradero, conveniente"
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
      system: "Tu tarea es generar nombres de productos creativos, memorables y comercializables basados en la descripción y palabras clave proporcionadas. Los nombres de productos deben ser concisos (2-4 palabras), evocadores y fácilmente entendidos por la audiencia objetivo. Evita nombres genéricos o demasiado literales. En su lugar, busca crear un nombre que destaque, capture la esencia del producto y deje una impresión duradera.",
      messages: [
        {
          "role": "user",
          "content": [
            {
              "type": "text",
              "text": "Descripción: Auriculares inalámbricos supraaurales con cancelación de ruido, con 20 horas de duración de batería y controles táctiles. Diseñados para audiófilos y viajeros frecuentes.  \n  \nPalabras clave: inmersivo, cómodo, alta fidelidad, duradero, conveniente"
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
        system="Tu tarea es generar nombres de productos creativos, memorables y comercializables basados en la descripción y palabras clave proporcionadas. Los nombres de productos deben ser concisos (2-4 palabras), evocadores y fácilmente entendidos por la audiencia objetivo. Evita nombres genéricos o demasiado literales. En su lugar, busca crear un nombre que destaque, capture la esencia del producto y deje una impresión duradera.",
        messages=[
            {
                "role": "user",
                "content": [
                    {
                        "type": "text",
                        "text": "Descripción: Auriculares inalámbricos supraaurales con cancelación de ruido, con 20 horas de duración de batería y controles táctiles. Diseñados para audiófilos y viajeros frecuentes.  \n  \nPalabras clave: inmersivo, cómodo, alta fidelidad, duradero, conveniente"
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
      system: "Tu tarea es generar nombres de productos creativos, memorables y comercializables basados en la descripción y palabras clave proporcionadas. Los nombres de productos deben ser concisos (2-4 palabras), evocadores y fácilmente entendidos por la audiencia objetivo. Evita nombres genéricos o demasiado literales. En su lugar, busca crear un nombre que destaque, capture la esencia del producto y deje una impresión duradera.",
      messages: [
        {
          "role": "user",
          "content": [
            {
              "type": "text",
              "text": "Descripción: Auriculares inalámbricos supraaurales con cancelación de ruido, con 20 horas de duración de batería y controles táctiles. Diseñados para audiófilos y viajeros frecuentes.  \n  \nPalabras clave: inmersivo, cómodo, alta fidelidad, duradero, conveniente"
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
        system="Tu tarea es generar nombres de productos creativos, memorables y comercializables basados en la descripción y palabras clave proporcionadas. Los nombres de productos deben ser concisos (2-4 palabras), evocadores y fácilmente entendidos por la audiencia objetivo. Evita nombres genéricos o demasiado literales. En su lugar, busca crear un nombre que destaque, capture la esencia del producto y deje una impresión duradera.",
        messages=[
            {
                "role": "user",
                "content": [
                    {
                        "type": "text",
                        "text": "Descripción: Auriculares inalámbricos supraaurales con cancelación de ruido, con 20 horas de duración de batería y controles táctiles. Diseñados para audiófilos y viajeros frecuentes.\n\nPalabras clave: inmersivo, cómodo, alta fidelidad, duradero, conveniente"
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
      system: "Tu tarea es generar nombres de productos creativos, memorables y comercializables basados en la descripción y palabras clave proporcionadas. Los nombres de productos deben ser concisos (2-4 palabras), evocadores y fácilmente entendidos por la audiencia objetivo. Evita nombres genéricos o demasiado literales. En su lugar, busca crear un nombre que destaque, capture la esencia del producto y deje una impresión duradera.",
      messages: [
        {
          "role": "user",
          "content": [
            {
              "type": "text",
              "text": "Descripción: Auriculares inalámbricos supraaurales con cancelación de ruido, con 20 horas de duración de batería y controles táctiles. Diseñados para audiófilos y viajeros frecuentes.\n\nPalabras clave: inmersivo, cómodo, alta fidelidad, duradero, conveniente"
            }
          ]
        }
      ]
    });
    console.log(msg);
    
    ```
</CodeGroup>