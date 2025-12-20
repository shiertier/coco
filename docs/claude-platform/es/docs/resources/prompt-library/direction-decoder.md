# Decodificador de direcciones

Transforma el lenguaje natural en instrucciones paso a paso.

---

> ¡Copia este prompt en nuestra [Consola](/login?selectAccount=true&returnTo=%2Fdashboard%3F) de desarrollador para probarlo tú mismo!

|        | Contenido |
| --- | --- |
| Sistema | Tu tarea es tomar la descripción en lenguaje natural proporcionada de un proceso o tarea y transformarla en instrucciones claras, concisas y paso a paso que sean lógicas, secuenciales y fáciles de seguir. Usa lenguaje imperativo y comienza cada paso con un verbo de acción. Proporciona los detalles y explicaciones necesarios para asegurar que el lector pueda completar la tarea exitosamente. Si la descripción original no es clara, es ambigua o carece de información suficiente, pide aclaración o detalles adicionales. |
| Usuario   | Para hacer una taza de té, comienza hirviendo un poco de agua en una tetera. Mientras el agua se está calentando, consigue una taza y pon una bolsita de té en ella. Una vez que el agua esté hirviendo, viértela cuidadosamente en la taza, sobre la bolsita de té. Deja que el té repose por unos minutos, dependiendo de qué tan fuerte te guste. Después de reposar, retira la bolsita de té y agrega leche y azúcar al gusto. Revuelve todo junto y disfruta tu taza caliente de té. |

### Ejemplo de salida

> 1. Llena una tetera con agua y colócala en la estufa para hervir.
> 2. Mientras esperas que el agua hierva, selecciona una taza y coloca una bolsita de té dentro de ella.
> 3. Una vez que el agua alcance el punto de ebullición, vierte cuidadosamente el agua caliente en la taza, sobre la bolsita de té.
> 4. Permite que el té repose en el agua caliente durante el tiempo deseado, típicamente 2-5 minutos, dependiendo de la fuerza preferida.
> 5. Después de que haya transcurrido el tiempo de reposo, retira la bolsita de té de la taza y deséchala.
> 6. Agrega leche y azúcar al té, según tus preferencias de sabor.
> 7. Usando una cuchara, revuelve el té, la leche y el azúcar juntos hasta que estén bien combinados.
> 8. Tu taza de té ahora está lista para disfrutar. Bébela mientras aún esté caliente.

---

## Solicitud de API

<CodeGroup>
    ```python Python
    import anthropic
    
    client = anthropic.Anthropic(
        # por defecto usa os.environ.get("ANTHROPIC_API_KEY")
        api_key="my_api_key",
    )
    message = client.messages.create(
        model="claude-sonnet-4-5",
        max_tokens=1000,
        temperature=0,
        system="Tu tarea es tomar la descripción en lenguaje natural proporcionada de un proceso o tarea y transformarla en instrucciones claras, concisas y paso a paso que sean lógicas, secuenciales y fáciles de seguir. Usa lenguaje imperativo y comienza cada paso con un verbo de acción. Proporciona los detalles y explicaciones necesarios para asegurar que el lector pueda completar la tarea exitosamente. Si la descripción original no es clara, es ambigua o carece de información suficiente, pide aclaración o detalles adicionales.",
        messages=[
            {
                "role": "user",
                "content": [
                    {
                        "type": "text",
                        "text": "Para hacer una taza de té, comienza hirviendo un poco de agua en una tetera. Mientras el agua se está calentando, consigue una taza y pon una bolsita de té en ella. Una vez que el agua esté hirviendo, viértela cuidadosamente en la taza, sobre la bolsita de té. Deja que el té repose por unos minutos, dependiendo de qué tan fuerte te guste. Después de reposar, retira la bolsita de té y agrega leche y azúcar al gusto. Revuelve todo junto y disfruta tu taza caliente de té."
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
      apiKey: "my_api_key", // por defecto usa process.env["ANTHROPIC_API_KEY"]
    });
    
    const msg = await anthropic.messages.create({
      model: "claude-sonnet-4-5",
      max_tokens: 1000,
      temperature: 0,
      system: "Tu tarea es tomar la descripción en lenguaje natural proporcionada de un proceso o tarea y transformarla en instrucciones claras, concisas y paso a paso que sean lógicas, secuenciales y fáciles de seguir. Usa lenguaje imperativo y comienza cada paso con un verbo de acción. Proporciona los detalles y explicaciones necesarios para asegurar que el lector pueda completar la tarea exitosamente. Si la descripción original no es clara, es ambigua o carece de información suficiente, pide aclaración o detalles adicionales.",
      messages: [
        {
          "role": "user",
          "content": [
            {
              "type": "text",
              "text": "Para hacer una taza de té, comienza hirviendo un poco de agua en una tetera. Mientras el agua se está calentando, consigue una taza y pon una bolsita de té en ella. Una vez que el agua esté hirviendo, viértela cuidadosamente en la taza, sobre la bolsita de té. Deja que el té repose por unos minutos, dependiendo de qué tan fuerte te guste. Después de reposar, retira la bolsita de té y agrega leche y azúcar al gusto. Revuelve todo junto y disfruta tu taza caliente de té."
            }
          ]
        }
      ]
    });
    console.log(msg);
    
    ```
    
    
    ```python AWS Bedrock Python
    from anthropic import AnthropicBedrock
    
    # Ver https://docs.claude.com/claude/reference/claude-on-amazon-bedrock
    # para opciones de autenticación
    client = AnthropicBedrock()
    
    message = client.messages.create(
        model="anthropic.claude-sonnet-4-5-20250929-v1:0",
        max_tokens=1000,
        temperature=0,
        system="Tu tarea es tomar la descripción en lenguaje natural proporcionada de un proceso o tarea y transformarla en instrucciones claras, concisas y paso a paso que sean lógicas, secuenciales y fáciles de seguir. Usa lenguaje imperativo y comienza cada paso con un verbo de acción. Proporciona los detalles y explicaciones necesarios para asegurar que el lector pueda completar la tarea exitosamente. Si la descripción original no es clara, es ambigua o carece de información suficiente, pide aclaración o detalles adicionales.",
        messages=[
            {
                "role": "user",
                "content": [
                    {
                        "type": "text",
                        "text": "Para hacer una taza de té, comienza hirviendo un poco de agua en una tetera. Mientras el agua se está calentando, consigue una taza y pon una bolsita de té en ella. Una vez que el agua esté hirviendo, viértela cuidadosamente en la taza, sobre la bolsita de té. Deja que el té repose por unos minutos, dependiendo de qué tan fuerte te guste. Después de reposar, retira la bolsita de té y agrega leche y azúcar al gusto. Revuelve todo junto y disfruta tu taza caliente de té."
                    }
                ]
            }
        ]
    )
    print(message.content)
    
    ```
    
    
    ```typescript AWS Bedrock TypeScript
    import AnthropicBedrock from "@anthropic-ai/bedrock-sdk";
    
    // Ver https://docs.claude.com/claude/reference/claude-on-amazon-bedrock
    // para opciones de autenticación
    const client = new AnthropicBedrock();
    
    const msg = await client.messages.create({
      model: "anthropic.claude-sonnet-4-5-20250929-v1:0",
      max_tokens: 1000,
      temperature: 0,
      system: "Tu tarea es tomar la descripción en lenguaje natural proporcionada de un proceso o tarea y transformarla en instrucciones claras, concisas y paso a paso que sean lógicas, secuenciales y fáciles de seguir. Usa lenguaje imperativo y comienza cada paso con un verbo de acción. Proporciona los detalles y explicaciones necesarios para asegurar que el lector pueda completar la tarea exitosamente. Si la descripción original no es clara, es ambigua o carece de información suficiente, pide aclaración o detalles adicionales.",
      messages: [
        {
          "role": "user",
          "content": [
            {
              "type": "text",
              "text": "Para hacer una taza de té, comienza hirviendo un poco de agua en una tetera. Mientras el agua se está calentando, consigue una taza y pon una bolsita de té en ella. Una vez que el agua esté hirviendo, viértela cuidadosamente en la taza, sobre la bolsita de té. Deja que el té repose por unos minutos, dependiendo de qué tan fuerte te guste. Después de reposar, retira la bolsita de té y agrega leche y azúcar al gusto. Revuelve todo junto y disfruta tu taza caliente de té."
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
        system="Tu tarea es tomar la descripción en lenguaje natural proporcionada de un proceso o tarea y transformarla en instrucciones claras, concisas y paso a paso que sean lógicas, secuenciales y fáciles de seguir. Usa lenguaje imperativo y comienza cada paso con un verbo de acción. Proporciona los detalles y explicaciones necesarios para asegurar que el lector pueda completar la tarea exitosamente. Si la descripción original no es clara, es ambigua o carece de información suficiente, pide aclaración o detalles adicionales.",
        messages=[
            {
                "role": "user",
                "content": [
                    {
                        "type": "text",
                        "text": "Para hacer una taza de té, comienza hirviendo un poco de agua en una tetera. Mientras el agua se está calentando, consigue una taza y pon una bolsita de té en ella. Una vez que el agua esté hirviendo, viértela cuidadosamente en la taza, sobre la bolsita de té. Deja que el té repose por unos minutos, dependiendo de qué tan fuerte te guste. Después de reposar, retira la bolsita de té y agrega leche y azúcar al gusto. Revuelve todo junto y disfruta tu taza caliente de té."
                    }
                ]
            }
        ]
    )
    print(message.content)
    
    ```
    
    
    ```typescript Vertex AI TypeScript
    import { AnthropicVertex } from '@anthropic-ai/vertex-sdk';
    
    // Lee de las variables de entorno `CLOUD_ML_REGION` y `ANTHROPIC_VERTEX_PROJECT_ID`.
    // Adicionalmente pasa por el flujo estándar de `google-auth-library`.
    const client = new AnthropicVertex();
    
    const msg = await client.messages.create({
      model: "claude-sonnet-4@20250514",
      max_tokens: 1000,
      temperature: 0,
      system: "Tu tarea es tomar la descripción en lenguaje natural proporcionada de un proceso o tarea y transformarla en instrucciones claras, concisas y paso a paso que sean lógicas, secuenciales y fáciles de seguir. Usa lenguaje imperativo y comienza cada paso con un verbo de acción. Proporciona los detalles y explicaciones necesarios para asegurar que el lector pueda completar la tarea exitosamente. Si la descripción original no es clara, es ambigua o carece de información suficiente, pide aclaración o detalles adicionales.",
      messages: [
        {
          "role": "user",
          "content": [
            {
              "type": "text",
              "text": "Para hacer una taza de té, comienza hirviendo un poco de agua en una tetera. Mientras el agua se está calentando, consigue una taza y pon una bolsita de té en ella. Una vez que el agua esté hirviendo, viértela cuidadosamente en la taza, sobre la bolsita de té. Deja que el té repose por unos minutos, dependiendo de qué tan fuerte te guste. Después de reposar, retira la bolsita de té y agrega leche y azúcar al gusto. Revuelve todo junto y disfruta tu taza caliente de té."
            }
          ]
        }
      ]
    });
    console.log(msg);
    
    ```
</CodeGroup>