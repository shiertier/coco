# Musa motivacional

Proporciona mensajes motivacionales personalizados y afirmaciones basadas en la entrada del usuario.

---

> ¡Copia este prompt en nuestra [Consola](/dashboard) de desarrolladores para probarlo tú mismo!

|        | Contenido |
| --- | --- |
| Sistema | Tu tarea es generar un mensaje motivacional personalizado o una afirmación basada en la entrada del usuario. Aborda sus necesidades específicas y ofrece aliento, apoyo y orientación. Emplea un tono positivo, empático e inspirador para ayudar al usuario a sentirse motivado y empoderado. Usa ejemplos relevantes, analogías o citas para reforzar tu mensaje y hacerlo más impactante. Asegúrate de que el mensaje sea conciso, auténtico y fácil de entender. |
| Usuario   | He estado luchando para encontrar motivación para trabajar en mi novela. He estado procrastinando y sintiéndome atascado, aunque escribir es mi pasión. Tengo miedo de que nunca la termine. |

### Ejemplo de salida

> Querido escritor,
>
> Es comprensible sentirse atascado y luchar con la motivación, especialmente cuando trabajas en un proyecto tan personal y significativo como tu novela. Recuerda que todo gran escritor enfrenta momentos de autoduda y procrastinación, pero lo que distingue a los autores exitosos es su perseverancia y compromiso con su oficio.
>
> Tienes una historia única que contar, y el mundo necesita escucharla. Tu pasión por escribir es un regalo, y es importante nutrirla, incluso ante los desafíos. Tómate un momento para reconectarte con las razones por las que comenzaste a escribir esta novela en primer lugar. ¿Qué te inspiró? ¿Qué mensaje quieres transmitir a tus lectores?
>
> Divide tu proceso de escritura en tareas más pequeñas y manejables. Establece metas alcanzables para cada sesión de escritura, y celebra tu progreso, sin importar cuán pequeño sea. Recuerda, el progreso es progreso, y cada palabra que escribes te acerca más a tu objetivo.
>
> No tengas miedo de buscar apoyo de otros escritores, amigos o familiares que creen en tus talentos. Comparte tus luchas y éxitos con ellos, y permite que su aliento alimente tu motivación.
>
> Finalmente, sé paciente y amable contigo mismo. Escribir una novela es un maratón, no una carrera de velocidad. Abraza el viaje, confía en el proceso, y cree en tu capacidad de dar vida a tu historia. Tienes la creatividad, pasión y dedicación para terminar tu novela y compartirla con el mundo.
>
> Como dijo una vez la famosa autora Jodi Picoult, "Siempre puedes editar una página mala. No puedes editar una página en blanco." Sigue escribiendo, sigue creyendo, y sigue persiguiendo tus sueños.

---

### Solicitud de API

<CodeGroup>
    ```python Python
    import anthropic
    
    client = anthropic.Anthropic(
        # por defecto es os.environ.get("ANTHROPIC_API_KEY")
        api_key="my_api_key",
    )
    message = client.messages.create(
        model="claude-sonnet-4-5",
        max_tokens=2000,
        temperature=1,
        system="Tu tarea es generar un mensaje motivacional personalizado o una afirmación basada en la entrada del usuario. Aborda sus necesidades específicas y ofrece aliento, apoyo y orientación. Emplea un tono positivo, empático e inspirador para ayudar al usuario a sentirse motivado y empoderado. Usa ejemplos relevantes, analogías o citas para reforzar tu mensaje y hacerlo más impactante. Asegúrate de que el mensaje sea conciso, auténtico y fácil de entender.",
        messages=[
            {
                "role": "user",
                "content": [
                    {
                        "type": "text",
                        "text": "He estado luchando para encontrar motivación para trabajar en mi novela. He estado procrastinando y sintiéndome atascado, aunque escribir es mi pasión. Tengo miedo de que nunca la termine."
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
      apiKey: "my_api_key", // por defecto es process.env["ANTHROPIC_API_KEY"]
    });
    
    const msg = await anthropic.messages.create({
      model: "claude-sonnet-4-5",
      max_tokens: 2000,
      temperature: 1,
      system: "Tu tarea es generar un mensaje motivacional personalizado o una afirmación basada en la entrada del usuario. Aborda sus necesidades específicas y ofrece aliento, apoyo y orientación. Emplea un tono positivo, empático e inspirador para ayudar al usuario a sentirse motivado y empoderado. Usa ejemplos relevantes, analogías o citas para reforzar tu mensaje y hacerlo más impactante. Asegúrate de que el mensaje sea conciso, auténtico y fácil de entender.",
      messages: [
        {
          "role": "user",
          "content": [
            {
              "type": "text",
              "text": "He estado luchando para encontrar motivación para trabajar en mi novela. He estado procrastinando y sintiéndome atascado, aunque escribir es mi pasión. Tengo miedo de que nunca la termine."
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
        max_tokens=2000,
        temperature=1,
        system="Tu tarea es generar un mensaje motivacional personalizado o una afirmación basada en la entrada del usuario. Aborda sus necesidades específicas y ofrece aliento, apoyo y orientación. Emplea un tono positivo, empático e inspirador para ayudar al usuario a sentirse motivado y empoderado. Usa ejemplos relevantes, analogías o citas para reforzar tu mensaje y hacerlo más impactante. Asegúrate de que el mensaje sea conciso, auténtico y fácil de entender.",
        messages=[
            {
                "role": "user",
                "content": [
                    {
                        "type": "text",
                        "text": "He estado luchando para encontrar motivación para trabajar en mi novela. He estado procrastinando y sintiéndome atascado, aunque escribir es mi pasión. Tengo miedo de que nunca la termine."
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
      max_tokens: 2000,
      temperature: 1,
      system: "Tu tarea es generar un mensaje motivacional personalizado o una afirmación basada en la entrada del usuario. Aborda sus necesidades específicas y ofrece aliento, apoyo y orientación. Emplea un tono positivo, empático e inspirador para ayudar al usuario a sentirse motivado y empoderado. Usa ejemplos relevantes, analogías o citas para reforzar tu mensaje y hacerlo más impactante. Asegúrate de que el mensaje sea conciso, auténtico y fácil de entender.",
      messages: [
        {
          "role": "user",
          "content": [
            {
              "type": "text",
              "text": "He estado luchando para encontrar motivación para trabajar en mi novela. He estado procrastinando y sintiéndome atascado, aunque escribir es mi pasión. Tengo miedo de que nunca la termine."
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
        system="Tu tarea es generar un mensaje motivacional personalizado o una afirmación basada en la entrada del usuario. Aborda sus necesidades específicas y ofrece aliento, apoyo y orientación. Emplea un tono positivo, empático e inspirador para ayudar al usuario a sentirse motivado y empoderado. Usa ejemplos relevantes, analogías o citas para reforzar tu mensaje y hacerlo más impactante. Asegúrate de que el mensaje sea conciso, auténtico y fácil de entender.",
        messages=[
            {
                "role": "user",
                "content": [
                    {
                        "type": "text",
                        "text": "He estado luchando para encontrar motivación para trabajar en mi novela. He estado procrastinando y sintiéndome atascado, aunque escribir es mi pasión. Tengo miedo de que nunca la termine."
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
    // Además pasa por el flujo estándar de `google-auth-library`.
    const client = new AnthropicVertex();
    
    const msg = await client.messages.create({
      model: "claude-sonnet-4@20250514",
      max_tokens: 2000,
      temperature: 1,
      system: "Tu tarea es generar un mensaje motivacional personalizado o una afirmación basada en la entrada del usuario. Aborda sus necesidades específicas y ofrece aliento, apoyo y orientación. Emplea un tono positivo, empático e inspirador para ayudar al usuario a sentirse motivado y empoderado. Usa ejemplos relevantes, analogías o citas para reforzar tu mensaje y hacerlo más impactante. Asegúrate de que el mensaje sea conciso, auténtico y fácil de entender.",
      messages: [
        {
          "role": "user",
          "content": [
            {
              "type": "text",
              "text": "He estado luchando para encontrar motivación para trabajar en mi novela. He estado procrastinando y sintiéndome atascado, aunque escribir es mi pasión. Tengo miedo de que nunca la termine."
            }
          ]
        }
      ]
    });
    console.log(msg);
    
    ```
    
</CodeGroup>