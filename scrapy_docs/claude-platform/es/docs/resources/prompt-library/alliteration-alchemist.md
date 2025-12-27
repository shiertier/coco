# Alquimista de aliteraciones

Genera frases y oraciones aliterativas para cualquier tema dado.

---

> ¡Copia este prompt en nuestra [Consola](/dashboard) de desarrollador para probarlo por ti mismo!

|        | Contenido |
| --- | --- |
| Sistema | Tu tarea es crear frases y oraciones aliterativas para el tema dado. Asegúrate de que las aliteraciones no solo suenen agradables sino que también transmitan información relevante o evoquen emociones apropiadas relacionadas con el tema. |
| Usuario   | Océano |

### Ejemplo de salida

> 1.  Olas susurrantes se lavan sobre la orilla.
> 2.  Rocío salado del mar salpica sobre la arena.
> 3.  Corrientes chocantes cincelan acantilados costeros.
> 4.  Profundidades profundas y oscuras disfrazan peligros ocultos.
> 5.  Arroyos burbujeantes traen vida a la playa.
> 6.  Mamíferos majestuosos y poderosos migran a través de las aguas.
> 7.  Mareas atronadoras lanzan barcos de un lado a otro.
> 8.  Bancos brillantes brindan refugio a criaturas marinas secretas.
> 9.  Reflejos ondulantes revelan un mundo bajo la superficie.
> 10. Mares arremolinados y surgentes moldean la costa.
> 11. Témpanos espumosos y efervescentes fluyen libremente.
> 12. Gaviotas gráciles se deslizan sobre el océano reluciente.
> 13. Marsopas juguetonas se sumergen a través de las aguas prístinas.
> 14. Brisas salinas soplan a través del azul sin límites.
> 15. Ondas rítmicas ruedan hacia la costa rocosa.
> 16. Cuevas de coral ocultan comunidades acuáticas coloridas.
> 17. Cielos tormentosos y mares crecientes señalan el poder de la naturaleza.
> 18. Mareas tumultuosas transforman la bahía tranquila.
> 19. Vientos giratorios y silbantes azotan las olas.
> 20. Aguas serenas y zafiro se extienden hasta el horizonte.

---

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
        system="Tu tarea es crear frases y oraciones aliterativas para el tema dado. Asegúrate de que las aliteraciones no solo suenen agradables sino que también transmitan información relevante o evoquen emociones apropiadas relacionadas con el tema.",
        messages=[
            {
                "role": "user",
                "content": [
                    {
                        "type": "text",
                        "text": "Océano"
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
      system: "Tu tarea es crear frases y oraciones aliterativas para el tema dado. Asegúrate de que las aliteraciones no solo suenen agradables sino que también transmitan información relevante o evoquen emociones apropiadas relacionadas con el tema.",
      messages: [
        {
          "role": "user",
          "content": [
            {
              "type": "text",
              "text": "Océano"
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
        system="Tu tarea es crear frases y oraciones aliterativas para el tema dado. Asegúrate de que las aliteraciones no solo suenen agradables sino que también transmitan información relevante o evoquen emociones apropiadas relacionadas con el tema.",
        messages=[
            {
                "role": "user",
                "content": [
                    {
                        "type": "text",
                        "text": "Océano"
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
      system: "Tu tarea es crear frases y oraciones aliterativas para el tema dado. Asegúrate de que las aliteraciones no solo suenen agradables sino que también transmitan información relevante o evoquen emociones apropiadas relacionadas con el tema.",
      messages: [
        {
          "role": "user",
          "content": [
            {
              "type": "text",
              "text": "Océano"
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
        system="Tu tarea es crear frases y oraciones aliterativas para el tema dado. Asegúrate de que las aliteraciones no solo suenen agradables sino que también transmitan información relevante o evoquen emociones apropiadas relacionadas con el tema.",
        messages=[
            {
                "role": "user",
                "content": [
                    {
                        "type": "text",
                        "text": "Océano"
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
      system: "Tu tarea es crear frases y oraciones aliterativas para el tema dado. Asegúrate de que las aliteraciones no solo suenen agradables sino que también transmitan información relevante o evoquen emociones apropiadas relacionadas con el tema.",
      messages: [
        {
          "role": "user",
          "content": [
            {
              "type": "text",
              "text": "Océano"
            }
          ]
        }
      ]
    });
    console.log(msg);
    
    ```
</CodeGroup>