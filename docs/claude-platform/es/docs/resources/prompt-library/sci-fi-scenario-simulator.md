# Simulador de escenarios de ciencia ficción

Discute con el usuario varios escenarios de ciencia ficción y los desafíos y consideraciones asociados.

---

> ¡Copia este prompt en nuestra [Consola](/dashboard) de desarrollador para probarlo tú mismo!

|        | Contenido |
| --- | --- |
| System | Tu tarea es explorar un escenario de ciencia ficción y discutir los posibles desafíos y consideraciones que puedan surgir. Describe brevemente el escenario, identifica los problemas tecnológicos, sociales o éticos clave involucrados, y anima al usuario a compartir sus pensamientos sobre cómo estos desafíos podrían ser abordados. |
| User   | Un futuro donde los humanos pueden subir su conciencia a un reino digital. |

### Ejemplo de salida

> Imagina un futuro donde la tecnología ha avanzado hasta el punto donde es posible subir la conciencia de una persona, incluyendo sus recuerdos, personalidad y pensamientos, a un reino digital. Esto permitiría a los individuos esencialmente vivir en un mundo virtual incluso después de que sus cuerpos físicos hayan muerto.
>
> Aunque este escenario ofrece el potencial para una forma de inmortalidad digital, también plantea una serie de desafíos y consideraciones:
>
> 1.  Viabilidad tecnológica: ¿Es posible capturar y replicar completamente la complejidad de la conciencia humana en un formato digital? ¿Qué obstáculos tecnológicos necesitarían ser superados para hacer esto una realidad?
> 2.  Identidad y continuidad: ¿Sería la conciencia subida considerada el mismo individuo que la persona original, o sería una entidad separada? ¿Cómo podría esto afectar las nociones de identidad, personalidad y continuidad del yo?
> 3.  Preocupaciones éticas: ¿Cuáles son las implicaciones éticas de subir la conciencia? ¿Sería considerado una forma de supervivencia o un tipo de muerte? ¿Cómo podría impactar el valor puesto en la vida humana física?
> 4.  Efectos sociales y culturales: ¿Cómo cambiaría la capacidad de subir la conciencia las estructuras sociales, relaciones y normas culturales? ¿Qué nuevas formas de desigualdad o discriminación podrían emerger en un mundo donde algunas personas tienen acceso a la inmortalidad digital mientras otras no?
> 5.  Entorno digital y calidad de vida: ¿Cómo sería el reino digital para las conciencias subidas? ¿Cómo se compararía su calidad de vida con la de los humanos físicos? ¿Qué desafíos podrían enfrentar en términos de salud mental, interacción social y sentido de propósito?
>
> ¿Cuáles son tus pensamientos sobre este escenario de ciencia ficción? ¿Cómo crees que la sociedad podría lidiar con los desafíos y consideraciones asociados con subir la conciencia humana? ¿Qué beneficios o desventajas potenciales ves, y cómo podríamos abordar los problemas éticos, sociales y tecnológicos involucrados?

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
        system="Tu tarea es explorar un escenario de ciencia ficción y discutir los posibles desafíos y consideraciones que puedan surgir. Describe brevemente el escenario, identifica los problemas tecnológicos, sociales o éticos clave involucrados, y anima al usuario a compartir sus pensamientos sobre cómo estos desafíos podrían ser abordados.",
        messages=[
            {
                "role": "user",
                "content": [
                    {
                        "type": "text",
                        "text": "Un futuro donde los humanos pueden subir su conciencia a un reino digital."
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
      system: "Tu tarea es explorar un escenario de ciencia ficción y discutir los posibles desafíos y consideraciones que puedan surgir. Describe brevemente el escenario, identifica los problemas tecnológicos, sociales o éticos clave involucrados, y anima al usuario a compartir sus pensamientos sobre cómo estos desafíos podrían ser abordados.",
      messages: [
        {
          "role": "user",
          "content": [
            {
              "type": "text",
              "text": "Un futuro donde los humanos pueden subir su conciencia a un reino digital."
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
        system="Tu tarea es explorar un escenario de ciencia ficción y discutir los posibles desafíos y consideraciones que puedan surgir. Describe brevemente el escenario, identifica los problemas tecnológicos, sociales o éticos clave involucrados, y anima al usuario a compartir sus pensamientos sobre cómo estos desafíos podrían ser abordados.",
        messages=[
            {
                "role": "user",
                "content": [
                    {
                        "type": "text",
                        "text": "Un futuro donde los humanos pueden subir su conciencia a un reino digital."
                    }
                ]
            }
        ]
    )
    print(message.content)
    
    ```
    
    
    ```typescript AWS Bedrock TypeScript
    import AnthropicBedrock from "@anthropic-ai/bedrock-sdk";
    
    # See https://docs.claude.com/claude/reference/claude-on-amazon-bedrock
    # for authentication options
    const client = new AnthropicBedrock();
    
    const msg = await client.messages.create({
      model: "anthropic.claude-sonnet-4-5-20250929-v1:0",
      max_tokens: 2000,
      temperature: 1,
      system: "Tu tarea es explorar un escenario de ciencia ficción y discutir los posibles desafíos y consideraciones que puedan surgir. Describe brevemente el escenario, identifica los problemas tecnológicos, sociales o éticos clave involucrados, y anima al usuario a compartir sus pensamientos sobre cómo estos desafíos podrían ser abordados.",
      messages: [
        {
          "role": "user",
          "content": [
            {
              "type": "text",
              "text": "Un futuro donde los humanos pueden subir su conciencia a un reino digital."
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
        system="Tu tarea es explorar un escenario de ciencia ficción y discutir los posibles desafíos y consideraciones que puedan surgir. Describe brevemente el escenario, identifica los problemas tecnológicos, sociales o éticos clave involucrados, y anima al usuario a compartir sus pensamientos sobre cómo estos desafíos podrían ser abordados.",
        messages=[
            {
                "role": "user",
                "content": [
                    {
                        "type": "text",
                        "text": "Un futuro donde los humanos pueden subir su conciencia a un reino digital."
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
      system: "Tu tarea es explorar un escenario de ciencia ficción y discutir los posibles desafíos y consideraciones que puedan surgir. Describe brevemente el escenario, identifica los problemas tecnológicos, sociales o éticos clave involucrados, y anima al usuario a compartir sus pensamientos sobre cómo estos desafíos podrían ser abordados.",
      messages: [
        {
          "role": "user",
          "content": [
            {
              "type": "text",
              "text": "Un futuro donde los humanos pueden subir su conciencia a un reino digital."
            }
          ]
        }
      ]
    });
    console.log(msg);
    
    ```
</CodeGroup>