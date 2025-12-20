# Reflexiones filosóficas

Participa en discusiones filosóficas profundas y experimentos mentales.

---

> ¡Copia este prompt en nuestra [Consola](/dashboard) de desarrollador para probarlo por ti mismo!

|        | Contenido |
| --- | --- |
| Sistema | Tu tarea es discutir un concepto filosófico o experimento mental sobre el tema dado. Explica brevemente el concepto, presenta los principales argumentos e implicaciones, y fomenta el pensamiento crítico planteando preguntas abiertas. Mantén un tono equilibrado y objetivo que fomente la curiosidad intelectual. |
| Usuario   | El problema del tranvía y la ética del utilitarismo |

### Ejemplo de salida

> El problema del tranvía es un experimento mental que desafía nuestras intuiciones morales y plantea preguntas sobre la ética del utilitarismo. El escenario básico es el siguiente:
>
> Un tranvía fuera de control está a punto de matar a cinco personas en las vías que tiene adelante. Estás parado junto a una palanca que puede desviar el tranvía hacia una vía lateral, donde matará solo a una persona. ¿Deberías tirar de la palanca para salvar cinco vidas a costa de una?
>
> Este dilema destaca el principio central del utilitarismo, que sostiene que la acción moralmente correcta es aquella que produce el mayor bien para el mayor número de personas. En este caso, tirar de la palanca maximizaría la utilidad al salvar cinco vidas, aunque causaría directamente la muerte de una persona.
>
> Sin embargo, el problema del tranvía también plantea preocupaciones sobre los límites del utilitarismo y la ética de causar daño activamente para lograr un bien mayor. Nos impulsa a considerar preguntas como:
>
> - ¿Es moralmente aceptable sacrificar a una persona inocente por el bien de salvar a un mayor número de personas?
> - ¿El principio de utilidad justifica cualquier medio para lograr el mayor bien, o hay restricciones éticas que no deberían violarse?
> - ¿Cómo evaluamos el valor de diferentes vidas, y es justo hacer tales cálculos?
>
> ¿Cuáles son tus pensamientos sobre el problema del tranvía y la ética del utilitarismo? ¿Crees que maximizar la utilidad debería ser el objetivo principal de la toma de decisiones morales, o hay otras consideraciones importantes? ¿Cómo abordarías el dilema presentado en el problema del tranvía, y qué principios éticos guiarían tu elección?

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
        temperature=1,
        system="Tu tarea es discutir un concepto filosófico o experimento mental sobre el tema dado. Explica brevemente el concepto, presenta los principales argumentos e implicaciones, y fomenta el pensamiento crítico planteando preguntas abiertas. Mantén un tono equilibrado y objetivo que fomente la curiosidad intelectual.",
        messages=[
            {
                "role": "user",
                "content": [
                    {
                        "type": "text",
                        "text": "El problema del tranvía y la ética del utilitarismo"
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
      system: "Tu tarea es discutir un concepto filosófico o experimento mental sobre el tema dado. Explica brevemente el concepto, presenta los principales argumentos e implicaciones, y fomenta el pensamiento crítico planteando preguntas abiertas. Mantén un tono equilibrado y objetivo que fomente la curiosidad intelectual.",
      messages: [
        {
          "role": "user",
          "content": [
            {
              "type": "text",
              "text": "El problema del tranvía y la ética del utilitarismo"
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
        system="Tu tarea es discutir un concepto filosófico o experimento mental sobre el tema dado. Explica brevemente el concepto, presenta los principales argumentos e implicaciones, y fomenta el pensamiento crítico planteando preguntas abiertas. Mantén un tono equilibrado y objetivo que fomente la curiosidad intelectual.",
        messages=[
            {
                "role": "user",
                "content": [
                    {
                        "type": "text",
                        "text": "El problema del tranvía y la ética del utilitarismo"
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
      system: "Tu tarea es discutir un concepto filosófico o experimento mental sobre el tema dado. Explica brevemente el concepto, presenta los principales argumentos e implicaciones, y fomenta el pensamiento crítico planteando preguntas abiertas. Mantén un tono equilibrado y objetivo que fomente la curiosidad intelectual.",
      messages: [
        {
          "role": "user",
          "content": [
            {
              "type": "text",
              "text": "El problema del tranvía y la ética del utilitarismo"
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
        system="Tu tarea es discutir un concepto filosófico o experimento mental sobre el tema dado. Explica brevemente el concepto, presenta los principales argumentos e implicaciones, y fomenta el pensamiento crítico planteando preguntas abiertas. Mantén un tono equilibrado y objetivo que fomente la curiosidad intelectual.",
        messages=[
            {
                "role": "user",
                "content": [
                    {
                        "type": "text",
                        "text": "El problema del tranvía y la ética del utilitarismo"
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
      system: "Tu tarea es discutir un concepto filosófico o experimento mental sobre el tema dado. Explica brevemente el concepto, presenta los principales argumentos e implicaciones, y fomenta el pensamiento crítico planteando preguntas abiertas. Mantén un tono equilibrado y objetivo que fomente la curiosidad intelectual.",
      messages: [
        {
          "role": "user",
          "content": [
            {
              "type": "text",
              "text": "El problema del tranvía y la ética del utilitarismo"
            }
          ]
        }
      ]
    });
    console.log(msg);
    
    ```
</CodeGroup>