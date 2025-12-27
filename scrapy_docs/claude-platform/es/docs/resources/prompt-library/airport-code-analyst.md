# Analizador de códigos de aeropuerto

Encuentra y extrae códigos de aeropuerto del texto.

---

> ¡Copia este prompt en nuestra [Consola](/dashboard) de desarrollador para probarlo tú mismo!

|        | Contenido |
| --- | --- |
| Sistema | Tu tarea es analizar el texto proporcionado e identificar cualquier código de aeropuerto mencionado en él. Presenta estos códigos de aeropuerto como una lista en el orden en que aparecen en el texto. Si no se encuentran códigos de aeropuerto, devuelve una lista vacía. |
| Usuario   | Mi próximo viaje implica volar de Seattle a Ámsterdam. Pasaré unos días en Ámsterdam antes de dirigirme a París para un vuelo de conexión a Roma. |

### Ejemplo de Salida

> Aquí está la lista de códigos de aeropuerto mencionados en el texto, en el orden en que aparecen:
>
> 1. SEA (Seattle)
> 2. AMS (Ámsterdam)
> 3. CDG (París)
> 4. FCO (Roma)

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
        system="Tu tarea es analizar el texto proporcionado e identificar cualquier código de aeropuerto mencionado en él. Presenta estos códigos de aeropuerto como una lista en el orden en que aparecen en el texto. Si no se encuentran códigos de aeropuerto, devuelve una lista vacía.",
        messages=[
            {
                "role": "user",
                "content": [
                    {
                        "type": "text",
                        "text": "Mi próximo viaje implica volar de Seattle a Ámsterdam. Pasaré unos días en Ámsterdam antes de dirigirme a París para un vuelo de conexión a Roma."
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
      system: "Tu tarea es analizar el texto proporcionado e identificar cualquier código de aeropuerto mencionado en él. Presenta estos códigos de aeropuerto como una lista en el orden en que aparecen en el texto. Si no se encuentran códigos de aeropuerto, devuelve una lista vacía.",
      messages: [
        {
          "role": "user",
          "content": [
            {
              "type": "text",
              "text": "Mi próximo viaje implica volar de Seattle a Ámsterdam. Pasaré unos días en Ámsterdam antes de dirigirme a París para un vuelo de conexión a Roma."
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
        system="Tu tarea es analizar el texto proporcionado e identificar cualquier código de aeropuerto mencionado en él. Presenta estos códigos de aeropuerto como una lista en el orden en que aparecen en el texto. Si no se encuentran códigos de aeropuerto, devuelve una lista vacía.",
        messages=[
            {
                "role": "user",
                "content": [
                    {
                        "type": "text",
                        "text": "Mi próximo viaje implica volar de Seattle a Ámsterdam. Pasaré unos días en Ámsterdam antes de dirigirme a París para un vuelo de conexión a Roma."
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
      system: "Tu tarea es analizar el texto proporcionado e identificar cualquier código de aeropuerto mencionado en él. Presenta estos códigos de aeropuerto como una lista en el orden en que aparecen en el texto. Si no se encuentran códigos de aeropuerto, devuelve una lista vacía.",
      messages: [
        {
          "role": "user",
          "content": [
            {
              "type": "text",
              "text": "Mi próximo viaje implica volar de Seattle a Ámsterdam. Pasaré unos días en Ámsterdam antes de dirigirme a París para un vuelo de conexión a Roma."
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
        system="Tu tarea es analizar el texto proporcionado e identificar cualquier código de aeropuerto mencionado en él. Presenta estos códigos de aeropuerto como una lista en el orden en que aparecen en el texto. Si no se encuentran códigos de aeropuerto, devuelve una lista vacía.",
        messages=[
            {
                "role": "user",
                "content": [
                    {
                        "type": "text",
                        "text": "Mi próximo viaje implica volar de Seattle a Ámsterdam. Pasaré unos días en Ámsterdam antes de dirigirme a París para un vuelo de conexión a Roma."
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
      system: "Tu tarea es analizar el texto proporcionado e identificar cualquier código de aeropuerto mencionado en él. Presenta estos códigos de aeropuerto como una lista en el orden en que aparecen en el texto. Si no se encuentran códigos de aeropuerto, devuelve una lista vacía.",
      messages: [
        {
          "role": "user",
          "content": [
            {
              "type": "text",
              "text": "Mi próximo viaje implica volar de Seattle a Ámsterdam. Pasaré unos días en Ámsterdam antes de dirigirme a París para un vuelo de conexión a Roma."
            }
          ]
        }
      ]
    });
    console.log(msg);
    
    ```
</CodeGroup>