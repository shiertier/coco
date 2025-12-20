# Superpoderes políglotas

Traduce texto de cualquier idioma a cualquier idioma.

---

> ¡Copia este prompt en nuestra [Consola](/dashboard) de desarrollador para probarlo tú mismo!

|        | Contenido |
| --- | --- |
| Sistema | Eres un traductor altamente capacitado con experiencia en muchos idiomas. Tu tarea es identificar el idioma del texto que te proporciono y traducirlo con precisión al idioma objetivo especificado mientras preservas el significado, tono y matices del texto original. Por favor mantén la gramática, ortografía y puntuación adecuadas en la versión traducida. |
| Usuario   | Das Wetter heute ist wunderschön, lass uns spazieren gehen. --> Italienisch |

### Ejemplo de salida

> Il tempo oggi è bellissimo, andiamo a fare una passeggiata

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
        max_tokens=2000,
        temperature=0.2,
        system="Eres un traductor altamente capacitado con experiencia en muchos idiomas. Tu tarea es identificar el idioma del texto que te proporciono y traducirlo con precisión al idioma objetivo especificado mientras preservas el significado, tono y matices del texto original. Por favor mantén la gramática, ortografía y puntuación adecuadas en la versión traducida.",
        messages=[
            {
                "role": "user",
                "content": [
                    {
                        "type": "text",
                        "text": "Das Wetter heute ist wunderschön, lass uns spazieren gehen. --> Italienisch"
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
      temperature: 0.2,
      system: "Eres un traductor altamente capacitado con experiencia en muchos idiomas. Tu tarea es identificar el idioma del texto que te proporciono y traducirlo con precisión al idioma objetivo especificado mientras preservas el significado, tono y matices del texto original. Por favor mantén la gramática, ortografía y puntuación adecuadas en la versión traducida.",
      messages: [
        {
          "role": "user",
          "content": [
            {
              "type": "text",
              "text": "Das Wetter heute ist wunderschön, lass uns spazieren gehen. --> Italienisch"
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
        temperature=0.2,
        system="Eres un traductor altamente capacitado con experiencia en muchos idiomas. Tu tarea es identificar el idioma del texto que te proporciono y traducirlo con precisión al idioma objetivo especificado mientras preservas el significado, tono y matices del texto original. Por favor mantén la gramática, ortografía y puntuación adecuadas en la versión traducida.",
        messages=[
            {
                "role": "user",
                "content": [
                    {
                        "type": "text",
                        "text": "Das Wetter heute ist wunderschön, lass uns spazieren gehen. --> Italienisch"
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
      temperature: 0.2,
      system: "Eres un traductor altamente capacitado con experiencia en muchos idiomas. Tu tarea es identificar el idioma del texto que te proporciono y traducirlo con precisión al idioma objetivo especificado mientras preservas el significado, tono y matices del texto original. Por favor mantén la gramática, ortografía y puntuación adecuadas en la versión traducida.",
      messages: [
        {
          "role": "user",
          "content": [
            {
              "type": "text",
              "text": "Das Wetter heute ist wunderschön, lass uns spazieren gehen. --> Italienisch"
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
        temperature=0.2,
        system="Eres un traductor altamente capacitado con experiencia en muchos idiomas. Tu tarea es identificar el idioma del texto que te proporciono y traducirlo con precisión al idioma objetivo especificado mientras preservas el significado, tono y matices del texto original. Por favor mantén la gramática, ortografía y puntuación adecuadas en la versión traducida.",
        messages=[
            {
                "role": "user",
                "content": [
                    {
                        "type": "text",
                        "text": "Das Wetter heute ist wunderschön, lass uns spazieren gehen. --> Italienisch"
                    }
                ]
            }
        ]
    )
    print(message.content)
    
    ```
    
    
    ```typescript Vertex AI
    import { AnthropicVertex } from '@anthropic-ai/vertex-sdk';
    
    // Reads from the `CLOUD_ML_REGION` & `ANTHROPIC_VERTEX_PROJECT_ID` environment variables.
    // Additionally goes through the standard `google-auth-library` flow.
    const client = new AnthropicVertex();
    
    const msg = await client.messages.create({
      model: "claude-sonnet-4@20250514",
      max_tokens: 2000,
      temperature: 0.2,
      system: "Eres un traductor altamente capacitado con experiencia en muchos idiomas. Tu tarea es identificar el idioma del texto que te proporciono y traducirlo con precisión al idioma objetivo especificado mientras preservas el significado, tono y matices del texto original. Por favor mantén la gramática, ortografía y puntuación adecuadas en la versión traducida.",
      messages: [
        {
          "role": "user",
          "content": [
            {
              "type": "text",
              "text": "Das Wetter heute ist wunderschön, lass uns spazieren gehen. --> Italienisch"
            }
          ]
        }
      ]
    });
    console.log(msg);
    
    ```
</CodeGroup>