# Organizador de datos

Convierte texto no estructurado en tablas JSON personalizadas.

---

> ¡Copia este prompt en nuestra [Consola](/dashboard) de desarrollador para probarlo tú mismo!

|        | Contenido |
| --- | --- |
| Sistema | Tu tarea es tomar el texto no estructurado proporcionado y convertirlo en un formato de tabla bien organizado usando JSON. Identifica las entidades principales, atributos o categorías mencionadas en el texto y úsalas como claves en el objeto JSON. Luego, extrae la información relevante del texto y completa los valores correspondientes en el objeto JSON. Asegúrate de que los datos estén representados con precisión y formateados correctamente dentro de la estructura JSON. La tabla JSON resultante debe proporcionar una visión general clara y estructurada de la información presentada en el texto original. |
| Usuario   | Silvermist Hollow, un pueblo encantador, era el hogar de un grupo extraordinario de individuos. Entre ellos estaba el Dr. Liam Patel, un neurocirujano de 45 años educado en Yale que revolucionó las técnicas quirúrgicas en el centro médico regional. Olivia Chen, a los 28 años, era una arquitecta innovadora de UC Berkeley que transformó el paisaje del pueblo con sus diseños sostenibles e impresionantes. El teatro local fue honrado por las sinfonías encantadoras de Ethan Kovacs, un músico y compositor de 72 años entrenado en Juilliard. Isabella Torres, una chef autodidacta con una pasión por los ingredientes de origen local, creó una sensación culinaria con su restaurante de la granja a la mesa, que se convirtió en un destino imperdible para los amantes de la comida. Estos individuos extraordinarios, cada uno con sus talentos distintivos, contribuyeron al tapiz vibrante de la vida en Silvermist Hollow. |

### Ejemplo de salida

```json
[
  {
    "name": "Dr. Liam Patel",
    "age": 45,
    "profession": "Neurocirujano",
    "education": "Yale",
    "accomplishments": "Revolucionó las técnicas quirúrgicas en el centro médico regional"
  },
  {
    "name": "Olivia Chen",
    "age": 28,
    "profession": "Arquitecta",
    "education": "UC Berkeley",
    "accomplishments": "Transformó el paisaje del pueblo con diseños sostenibles e impresionantes"
  },
  {
    "name": "Ethan Kovacs",
    "age": 72,
    "profession": "Músico y Compositor",
    "education": "Juilliard",
    "accomplishments": "Honró el teatro local con sinfonías encantadoras"
  },
  {
    "name": "Isabella Torres",
    "age": null,
    "profession": "Chef",
    "education": "Autodidacta",
    "accomplishments": "Creó una sensación culinaria con su restaurante de la granja a la mesa, que se convirtió en un destino imperdible para los amantes de la comida"
  }
]
```

---

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
        system="Tu tarea es tomar el texto no estructurado proporcionado y convertirlo en un formato de tabla bien organizado usando JSON. Identifica las entidades principales, atributos o categorías mencionadas en el texto y úsalas como claves en el objeto JSON. Luego, extrae la información relevante del texto y completa los valores correspondientes en el objeto JSON. Asegúrate de que los datos estén representados con precisión y formateados correctamente dentro de la estructura JSON. La tabla JSON resultante debe proporcionar una visión general clara y estructurada de la información presentada en el texto original.",
        messages=[
            {
                "role": "user",
                "content": [
                    {
                        "type": "text",
                        "text": "Silvermist Hollow, un pueblo encantador, era el hogar de un grupo extraordinario de individuos. Entre ellos estaba el Dr. Liam Patel, un neurocirujano de 45 años educado en Yale que revolucionó las técnicas quirúrgicas en el centro médico regional. Olivia Chen, a los 28 años, era una arquitecta innovadora de UC Berkeley que transformó el paisaje del pueblo con sus diseños sostenibles e impresionantes. El teatro local fue honrado por las sinfonías encantadoras de Ethan Kovacs, un músico y compositor de 72 años entrenado en Juilliard. Isabella Torres, una chef autodidacta con una pasión por los ingredientes de origen local, creó una sensación culinaria con su restaurante de la granja a la mesa, que se convirtió en un destino imperdible para los amantes de la comida. Estos individuos extraordinarios, cada uno con sus talentos distintivos, contribuyeron al tapiz vibrante de la vida en Silvermist Hollow."
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
      system: "Tu tarea es tomar el texto no estructurado proporcionado y convertirlo en un formato de tabla bien organizado usando JSON. Identifica las entidades principales, atributos o categorías mencionadas en el texto y úsalas como claves en el objeto JSON. Luego, extrae la información relevante del texto y completa los valores correspondientes en el objeto JSON. Asegúrate de que los datos estén representados con precisión y formateados correctamente dentro de la estructura JSON. La tabla JSON resultante debe proporcionar una visión general clara y estructurada de la información presentada en el texto original.",
      messages: [
        {
          "role": "user",
          "content": [
            {
              "type": "text",
              "text": "Silvermist Hollow, un pueblo encantador, era el hogar de un grupo extraordinario de individuos. Entre ellos estaba el Dr. Liam Patel, un neurocirujano de 45 años educado en Yale que revolucionó las técnicas quirúrgicas en el centro médico regional. Olivia Chen, a los 28 años, era una arquitecta innovadora de UC Berkeley que transformó el paisaje del pueblo con sus diseños sostenibles e impresionantes. El teatro local fue honrado por las sinfonías encantadoras de Ethan Kovacs, un músico y compositor de 72 años entrenado en Juilliard. Isabella Torres, una chef autodidacta con una pasión por los ingredientes de origen local, creó una sensación culinaria con su restaurante de la granja a la mesa, que se convirtió en un destino imperdible para los amantes de la comida. Estos individuos extraordinarios, cada uno con sus talentos distintivos, contribuyeron al tapiz vibrante de la vida en Silvermist Hollow."
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
        system="Tu tarea es tomar el texto no estructurado proporcionado y convertirlo en un formato de tabla bien organizado usando JSON. Identifica las entidades principales, atributos o categorías mencionadas en el texto y úsalas como claves en el objeto JSON. Luego, extrae la información relevante del texto y completa los valores correspondientes en el objeto JSON. Asegúrate de que los datos estén representados con precisión y formateados correctamente dentro de la estructura JSON. La tabla JSON resultante debe proporcionar una visión general clara y estructurada de la información presentada en el texto original.",
        messages=[
            {
                "role": "user",
                "content": [
                    {
                        "type": "text",
                        "text": "Silvermist Hollow, un pueblo encantador, era el hogar de un grupo extraordinario de individuos. Entre ellos estaba el Dr. Liam Patel, un neurocirujano de 45 años educado en Yale que revolucionó las técnicas quirúrgicas en el centro médico regional. Olivia Chen, a los 28 años, era una arquitecta innovadora de UC Berkeley que transformó el paisaje del pueblo con sus diseños sostenibles e impresionantes. El teatro local fue honrado por las sinfonías encantadoras de Ethan Kovacs, un músico y compositor de 72 años entrenado en Juilliard. Isabella Torres, una chef autodidacta con una pasión por los ingredientes de origen local, creó una sensación culinaria con su restaurante de la granja a la mesa, que se convirtió en un destino imperdible para los amantes de la comida. Estos individuos extraordinarios, cada uno con sus talentos distintivos, contribuyeron al tapiz vibrante de la vida en Silvermist Hollow."
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
      system: "Tu tarea es tomar el texto no estructurado proporcionado y convertirlo en un formato de tabla bien organizado usando JSON. Identifica las entidades principales, atributos o categorías mencionadas en el texto y úsalas como claves en el objeto JSON. Luego, extrae la información relevante del texto y completa los valores correspondientes en el objeto JSON. Asegúrate de que los datos estén representados con precisión y formateados correctamente dentro de la estructura JSON. La tabla JSON resultante debe proporcionar una visión general clara y estructurada de la información presentada en el texto original.",
      messages: [
        {
          "role": "user",
          "content": [
            {
              "type": "text",
              "text": "Silvermist Hollow, un pueblo encantador, era el hogar de un grupo extraordinario de individuos. Entre ellos estaba el Dr. Liam Patel, un neurocirujano de 45 años educado en Yale que revolucionó las técnicas quirúrgicas en el centro médico regional. Olivia Chen, a los 28 años, era una arquitecta innovadora de UC Berkeley que transformó el paisaje del pueblo con sus diseños sostenibles e impresionantes. El teatro local fue honrado por las sinfonías encantadoras de Ethan Kovacs, un músico y compositor de 72 años entrenado en Juilliard. Isabella Torres, una chef autodidacta con una pasión por los ingredientes de origen local, creó una sensación culinaria con su restaurante de la granja a la mesa, que se convirtió en un destino imperdible para los amantes de la comida. Estos individuos extraordinarios, cada uno con sus talentos distintivos, contribuyeron al tapiz vibrante de la vida en Silvermist Hollow."
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
        system="Tu tarea es tomar el texto no estructurado proporcionado y convertirlo en un formato de tabla bien organizado usando JSON. Identifica las entidades principales, atributos o categorías mencionadas en el texto y úsalas como claves en el objeto JSON. Luego, extrae la información relevante del texto y completa los valores correspondientes en el objeto JSON. Asegúrate de que los datos estén representados con precisión y formateados correctamente dentro de la estructura JSON. La tabla JSON resultante debe proporcionar una visión general clara y estructurada de la información presentada en el texto original.",
        messages=[
            {
                "role": "user",
                "content": [
                    {
                        "type": "text",
                        "text": "Silvermist Hollow, un pueblo encantador, era el hogar de un grupo extraordinario de individuos. Entre ellos estaba el Dr. Liam Patel, un neurocirujano de 45 años educado en Yale que revolucionó las técnicas quirúrgicas en el centro médico regional. Olivia Chen, a los 28 años, era una arquitecta innovadora de UC Berkeley que transformó el paisaje del pueblo con sus diseños sostenibles e impresionantes. El teatro local fue honrado por las sinfonías encantadoras de Ethan Kovacs, un músico y compositor de 72 años entrenado en Juilliard. Isabella Torres, una chef autodidacta con una pasión por los ingredientes de origen local, creó una sensación culinaria con su restaurante de la granja a la mesa, que se convirtió en un destino imperdible para los amantes de la comida. Estos individuos extraordinarios, cada uno con sus talentos distintivos, contribuyeron al tapiz vibrante de la vida en Silvermist Hollow."
                    }
                ]
            }
        ]
    )
    print(message.content)
    
    ```
    
    
    ```typescript Vertex AI Type
    import { AnthropicVertex } from '@anthropic-ai/vertex-sdk';
    
    // Reads from the `CLOUD_ML_REGION` & `ANTHROPIC_VERTEX_PROJECT_ID` environment variables.
    // Additionally goes through the standard `google-auth-library` flow.
    const client = new AnthropicVertex();
    
    const msg = await client.messages.create({
      model: "claude-sonnet-4@20250514",
      max_tokens: 1000,
      temperature: 0,
      system: "Tu tarea es tomar el texto no estructurado proporcionado y convertirlo en un formato de tabla bien organizado usando JSON. Identifica las entidades principales, atributos o categorías mencionadas en el texto y úsalas como claves en el objeto JSON. Luego, extrae la información relevante del texto y completa los valores correspondientes en el objeto JSON. Asegúrate de que los datos estén representados con precisión y formateados correctamente dentro de la estructura JSON. La tabla JSON resultante debe proporcionar una visión general clara y estructurada de la información presentada en el texto original.",
      messages: [
        {
          "role": "user",
          "content": [
            {
              "type": "text",
              "text": "Silvermist Hollow, un pueblo encantador, era el hogar de un grupo extraordinario de individuos. Entre ellos estaba el Dr. Liam Patel, un neurocirujano de 45 años educado en Yale que revolucionó las técnicas quirúrgicas en el centro médico regional. Olivia Chen, a los 28 años, era una arquitecta innovadora de UC Berkeley que transformó el paisaje del pueblo con sus diseños sostenibles e impresionantes. El teatro local fue honrado por las sinfonías encantadoras de Ethan Kovacs, un músico y compositor de 72 años entrenado en Juilliard. Isabella Torres, una chef autodidacta con una pasión por los ingredientes de origen local, creó una sensación culinaria con su restaurante de la granja a la mesa, que se convirtió en un destino imperdible para los amantes de la comida. Estos individuos extraordinarios, cada uno con sus talentos distintivos, contribuyeron al tapiz vibrante de la vida en Silvermist Hollow."
            }
          ]
        }
      ]
    });
    console.log(msg);
    
    ```
</CodeGroup>