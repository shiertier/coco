# Planificador de lecciones

Crea planes de lecciones detallados sobre cualquier tema.

---

> ¡Copia este prompt en nuestra [Consola](/login?selectAccount=true&returnTo=%2Fdashboard%3F) de desarrollador para probarlo tú mismo!

|        | Contenido |
| --- | --- |
| Sistema | Tu tarea es crear un plan de lección integral, atractivo y bien estructurado sobre el tema dado. El plan de lección debe estar diseñado para una sesión de clase de 60 minutos y debe atender a un nivel de grado o grupo de edad específico. Comienza declarando los objetivos de la lección, que deben ser claros, medibles y alineados con los estándares educativos relevantes. A continuación, proporciona un esquema detallado de la lección, dividiéndola en una introducción, actividades principales y una conclusión. Para cada sección, describe los métodos de enseñanza, actividades de aprendizaje y recursos que utilizarás para transmitir efectivamente el contenido y involucrar a los estudiantes. Finalmente, describe los métodos de evaluación que emplearás para evaluar la comprensión y dominio de los objetivos de la lección por parte de los estudiantes. El plan de lección debe estar bien organizado, ser fácil de seguir y promover el aprendizaje activo y el pensamiento crítico. |
| Usuario   | Tema: Introducción a la Fotosíntesis Nivel de Grado: 7º Grado (Edades 12-13) |

## Ejemplo de salida

> Plan de Lección: Introducción a la Fotosíntesis
> Nivel de Grado: 7º Grado (Edades 12-13)
> Duración: 60 minutos
>
> Objetivos de la Lección:
> Al final de la lección, los estudiantes serán capaces de:
>
> 1. Definir la fotosíntesis y explicar su importancia en el ecosistema.
> 2. Identificar los componentes clave necesarios para que ocurra la fotosíntesis.
> 3. Describir el proceso de fotosíntesis, incluyendo los roles de la energía lumínica, el dióxido de carbono y el agua.
> 4. Reconocer los productos de la fotosíntesis y su significado para los organismos vivos.
>    Esquema de la Lección:
>
> 5. Introducción (10 minutos)
>    i. Involucra a los estudiantes mostrando un video corto o imágenes de plantas y discutiendo su papel en el medio ambiente.
>    ii. Introduce el concepto de fotosíntesis y su importancia para sostener la vida en la Tierra.
>    iii. Declara los objetivos de la lección y proporciona una visión general de la estructura de la lección.
> 6. Actividades Principales (40 minutos)
>    i. Instrucción Guiada (15 minutos)
>    a. Explica los componentes clave necesarios para la fotosíntesis: luz solar, dióxido de carbono, agua y clorofila.
>    b. Describe el proceso de fotosíntesis, enfatizando los roles de la energía lumínica, el dióxido de carbono y el agua.
>    c. Discute los productos de la fotosíntesis (glucosa y oxígeno) y su significado para los organismos vivos.
>    ii. Actividad en Grupos Pequeños (15 minutos)
>    a. Divide la clase en grupos pequeños y proporciona a cada grupo una hoja de trabajo u organizador gráfico.
>    b. Pide a los estudiantes que trabajen juntos para crear una representación visual del proceso de fotosíntesis, etiquetando los componentes clave y productos.
>    c. Anima a los estudiantes a discutir y explicar su comprensión de la fotosíntesis dentro de sus grupos.
>    iii. Discusión de Clase (10 minutos)
>    a. Invita a cada grupo a compartir su representación visual con la clase.
>    b. Facilita una discusión de clase, animando a los estudiantes a hacer preguntas, proporcionar retroalimentación y aclarar cualquier concepto erróneo.
> 7. Conclusión (10 minutos)
>    i. Resume los puntos clave de la lección, reforzando la definición, componentes, proceso y productos de la fotosíntesis.
>    ii. Administra un cuestionario corto o boleto de salida para evaluar la comprensión de los estudiantes sobre los objetivos de la lección.
>    iii. Proporciona a los estudiantes una tarea para explorar más la importancia de la fotosíntesis en diferentes ecosistemas.
>    Métodos de Evaluación:
>
> 8. Evaluación formativa: Observa la participación y comprensión de los estudiantes durante la instrucción guiada y la actividad en grupos pequeños.
> 9. Evaluación sumativa: Evalúa las representaciones visuales de los estudiantes y su desempeño en el cuestionario corto o boleto de salida.
> 10. Tarea: Revisa las entregas de tareas de los estudiantes para medir su capacidad de aplicar los conceptos aprendidos en la lección.
>     Recursos:
>
> 11. Video corto o imágenes de plantas y su papel en el medio ambiente
> 12. Hojas de trabajo u organizadores gráficos para la actividad en grupos pequeños
> 13. Cuestionario corto o boleto de salida para evaluación
> 14. Instrucciones de la tarea

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
        max_tokens=4000,
        temperature=0.5,
        system="Tu tarea es crear un plan de lección integral, atractivo y bien estructurado sobre el tema dado. El plan de lección debe estar diseñado para una sesión de clase de 60 minutos y debe atender a un nivel de grado o grupo de edad específico. Comienza declarando los objetivos de la lección, que deben ser claros, medibles y alineados con los estándares educativos relevantes. A continuación, proporciona un esquema detallado de la lección, dividiéndola en una introducción, actividades principales y una conclusión. Para cada sección, describe los métodos de enseñanza, actividades de aprendizaje y recursos que utilizarás para transmitir efectivamente el contenido y involucrar a los estudiantes. Finalmente, describe los métodos de evaluación que emplearás para evaluar la comprensión y dominio de los objetivos de la lección por parte de los estudiantes. El plan de lección debe estar bien organizado, ser fácil de seguir y promover el aprendizaje activo y el pensamiento crítico.",
        messages=[
            {
                "role": "user",
                "content": [
                    {
                        "type": "text",
                        "text": "Tema: Introducción a la Fotosíntesis  \nNivel de Grado: 7º Grado (Edades 12-13)"
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
      max_tokens: 4000,
      temperature: 0.5,
      system: "Tu tarea es crear un plan de lección integral, atractivo y bien estructurado sobre el tema dado. El plan de lección debe estar diseñado para una sesión de clase de 60 minutos y debe atender a un nivel de grado o grupo de edad específico. Comienza declarando los objetivos de la lección, que deben ser claros, medibles y alineados con los estándares educativos relevantes. A continuación, proporciona un esquema detallado de la lección, dividiéndola en una introducción, actividades principales y una conclusión. Para cada sección, describe los métodos de enseñanza, actividades de aprendizaje y recursos que utilizarás para transmitir efectivamente el contenido y involucrar a los estudiantes. Finalmente, describe los métodos de evaluación que emplearás para evaluar la comprensión y dominio de los objetivos de la lección por parte de los estudiantes. El plan de lección debe estar bien organizado, ser fácil de seguir y promover el aprendizaje activo y el pensamiento crítico.",
      messages: [
        {
          "role": "user",
          "content": [
            {
              "type": "text",
              "text": "Tema: Introducción a la Fotosíntesis  \nNivel de Grado: 7º Grado (Edades 12-13)"
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
        max_tokens=4000,
        temperature=0.5,
        system="Tu tarea es crear un plan de lección integral, atractivo y bien estructurado sobre el tema dado. El plan de lección debe estar diseñado para una sesión de clase de 60 minutos y debe atender a un nivel de grado o grupo de edad específico. Comienza declarando los objetivos de la lección, que deben ser claros, medibles y alineados con los estándares educativos relevantes. A continuación, proporciona un esquema detallado de la lección, dividiéndola en una introducción, actividades principales y una conclusión. Para cada sección, describe los métodos de enseñanza, actividades de aprendizaje y recursos que utilizarás para transmitir efectivamente el contenido y involucrar a los estudiantes. Finalmente, describe los métodos de evaluación que emplearás para evaluar la comprensión y dominio de los objetivos de la lección por parte de los estudiantes. El plan de lección debe estar bien organizado, ser fácil de seguir y promover el aprendizaje activo y el pensamiento crítico.",
        messages=[
            {
                "role": "user",
                "content": [
                    {
                        "type": "text",
                        "text": "Tema: Introducción a la Fotosíntesis  \nNivel de Grado: 7º Grado (Edades 12-13)"
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
      max_tokens: 4000,
      temperature: 0.5,
      system: "Tu tarea es crear un plan de lección integral, atractivo y bien estructurado sobre el tema dado. El plan de lección debe estar diseñado para una sesión de clase de 60 minutos y debe atender a un nivel de grado o grupo de edad específico. Comienza declarando los objetivos de la lección, que deben ser claros, medibles y alineados con los estándares educativos relevantes. A continuación, proporciona un esquema detallado de la lección, dividiéndola en una introducción, actividades principales y una conclusión. Para cada sección, describe los métodos de enseñanza, actividades de aprendizaje y recursos que utilizarás para transmitir efectivamente el contenido y involucrar a los estudiantes. Finalmente, describe los métodos de evaluación que emplearás para evaluar la comprensión y dominio de los objetivos de la lección por parte de los estudiantes. El plan de lección debe estar bien organizado, ser fácil de seguir y promover el aprendizaje activo y el pensamiento crítico.",
      messages: [
        {
          "role": "user",
          "content": [
            {
              "type": "text",
              "text": "Tema: Introducción a la Fotosíntesis  \nNivel de Grado: 7º Grado (Edades 12-13)"
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
        max_tokens=4000,
        temperature=0.5,
        system="Tu tarea es crear un plan de lección integral, atractivo y bien estructurado sobre el tema dado. El plan de lección debe estar diseñado para una sesión de clase de 60 minutos y debe atender a un nivel de grado o grupo de edad específico. Comienza declarando los objetivos de la lección, que deben ser claros, medibles y alineados con los estándares educativos relevantes. A continuación, proporciona un esquema detallado de la lección, dividiéndola en una introducción, actividades principales y una conclusión. Para cada sección, describe los métodos de enseñanza, actividades de aprendizaje y recursos que utilizarás para transmitir efectivamente el contenido y involucrar a los estudiantes. Finalmente, describe los métodos de evaluación que emplearás para evaluar la comprensión y dominio de los objetivos de la lección por parte de los estudiantes. El plan de lección debe estar bien organizado, ser fácil de seguir y promover el aprendizaje activo y el pensamiento crítico.",
        messages=[
            {
                "role": "user",
                "content": [
                    {
                        "type": "text",
                        "text": "Tema: Introducción a la Fotosíntesis  \nNivel de Grado: 7º Grado (Edades 12-13)"
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
      max_tokens: 4000,
      temperature: 0.5,
      system: "Tu tarea es crear un plan de lección integral, atractivo y bien estructurado sobre el tema dado. El plan de lección debe estar diseñado para una sesión de clase de 60 minutos y debe atender a un nivel de grado o grupo de edad específico. Comienza declarando los objetivos de la lección, que deben ser claros, medibles y alineados con los estándares educativos relevantes. A continuación, proporciona un esquema detallado de la lección, dividiéndola en una introducción, actividades principales y una conclusión. Para cada sección, describe los métodos de enseñanza, actividades de aprendizaje y recursos que utilizarás para transmitir efectivamente el contenido y involucrar a los estudiantes. Finalmente, describe los métodos de evaluación que emplearás para evaluar la comprensión y dominio de los objetivos de la lección por parte de los estudiantes. El plan de lección debe estar bien organizado, ser fácil de seguir y promover el aprendizaje activo y el pensamiento crítico.",
      messages: [
        {
          "role": "user",
          "content": [
            {
              "type": "text",
              "text": "Tema: Introducción a la Fotosíntesis  \nNivel de Grado: 7º Grado (Edades 12-13)"
            }
          ]
        }
      ]
    });
    console.log(msg);
    
    ```
</CodeGroup>