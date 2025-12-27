# Constructor de marca

Crea un brief de diseño para una identidad de marca holística.

---

> ¡Copia este prompt en nuestra [Consola](/login?selectAccount=true&returnTo=%2Fdashboard%3F) de desarrollador para probarlo tú mismo!

|        | Contenido |
| --- | --- |
| Sistema | Tu tarea es crear un brief de diseño integral para una identidad de marca holística basada en las especificaciones dadas. La identidad de marca debe abarcar varios elementos como sugerencias para el nombre de la marca, logo, paleta de colores, tipografía, estilo visual, tono de voz y personalidad general de la marca. Asegúrate de que todos los elementos trabajen juntos armoniosamente para crear una experiencia de marca cohesiva y memorable que comunique efectivamente los valores, misión y propuesta única de venta de la marca a su audiencia objetivo. Sé detallado e integral y proporciona suficientes detalles específicos para que alguien pueda crear una identidad de marca verdaderamente única. |
| Usuario   | Especificaciones de marca: Esta es una marca que se enfoca en crear ropa y accesorios de alta calidad y elegantes usando materiales ecológicos y métodos de producción éticos. La marca se dirige a consumidores ambientalmente conscientes de 25-40 años que valoran la moda, sostenibilidad y responsabilidad social. La identidad de marca debe lograr los siguientes objetivos: 1. Reflejar el compromiso de la marca con la sostenibilidad, prácticas éticas y administración ambiental. 2. Atraer a la audiencia objetivo transmitiendo un sentido de estilo, calidad y tendencia. 3. Diferenciar la marca de los competidores en el mercado de moda sostenible. 4. Crear una conexión emocional fuerte con los consumidores e inspirarlos a tomar decisiones más amigables con el medio ambiente. |

### Ejemplo de salida

> Brief de Diseño: Identidad de Marca para una Marca de Moda Ecológica
> Sugerencia de Nombre de Marca: "Sēnsus" (Latín para "sentido" o "percepción," evocando un sentido de atención plena y conexión con el medio ambiente)
>
> Logo:
>
> - Un logo minimalista y moderno que combina un motivo estilizado de hoja o planta con una tipografía elegante y sans-serif.
> - El elemento de hoja o planta simboliza el compromiso de la marca con la sostenibilidad y administración ambiental, mientras que la tipografía limpia transmite un sentido de sofisticación y estilo.
> - El logo debe ser versátil y funcionar bien en varias aplicaciones, incluyendo etiquetas de ropa, empaque y plataformas digitales.
>   Paleta de Colores:
>
> - Una paleta de colores neutral y terrosa que refleje el ethos ecológico de la marca, como tonos de verde, beige y grises cálidos.
> - Los colores de acento como teal profundo o amarillo mostaza pueden usarse con moderación para agregar vibración y profundidad.
> - La paleta de colores debe evocar un sentido de armonía y equilibrio natural, mientras sigue siendo vanguardista en moda y atractiva para la audiencia objetivo.
>   Tipografía:
>
> - Una combinación de una tipografía sans-serif limpia y moderna para titulares y una tipografía serif complementaria para el texto del cuerpo.
> - La tipografía debe ser legible y visualmente atractiva, transmitiendo un sentido de calidad y sofisticación.
> - El uso consistente de jerarquía tipográfica y espaciado creará una identidad de marca cohesiva y organizada.
>   Estilo Visual:
>
> - Un estilo visual minimalista y despejado que enfatice la fotografía de producto de alta calidad y diseños limpios y modernos.
> - Incorporar elementos naturales, como motivos de plantas o texturas, para reforzar los valores ecológicos de la marca y la conexión con la naturaleza.
> - Usar el espacio negativo efectivamente para crear un sentido de apertura y respirabilidad, reflejando el compromiso de la marca con la sostenibilidad.
>   Tono de Voz:
>
> - El tono de voz de la marca debe ser accesible, conocedor y auténtico.
> - Usar lenguaje que eduque e inspire a los consumidores sobre los beneficios de la moda sostenible y métodos de producción éticos.
> - Incorporar un toque de sofisticación y estilo para atraer a las sensibilidades conscientes de la moda de la audiencia objetivo.
>   Personalidad de Marca:
>
> - Eco-consciente y ambientalmente consciente, pero elegante y vanguardista en moda.
> - Confiada y apasionada sobre promover prácticas sostenibles y elecciones éticas.
> - Inclusiva y acogedora, invitando a los consumidores a unirse a una comunidad de ideas afines comprometida con hacer cambios positivos.
>   En general, la identidad de marca para "Sēnsus" debe lograr un equilibrio entre valores ecológicos y estética vanguardista en moda, atrayendo a consumidores que quieren verse elegantes mientras toman decisiones ambientalmente responsables. Los elementos visuales y verbales cohesivos deben crear una conexión emocional fuerte con la audiencia objetivo e inspirarlos a abrazar un estilo de vida más sostenible.

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
        system="Tu tarea es crear un brief de diseño integral para una identidad de marca holística basada en las especificaciones dadas. La identidad de marca debe abarcar varios elementos como sugerencias para el nombre de la marca, logo, paleta de colores, tipografía, estilo visual, tono de voz y personalidad general de la marca. Asegúrate de que todos los elementos trabajen juntos armoniosamente para crear una experiencia de marca cohesiva y memorable que comunique efectivamente los valores, misión y propuesta única de venta de la marca a su audiencia objetivo. Sé detallado e integral y proporciona suficientes detalles específicos para que alguien pueda crear una identidad de marca verdaderamente única.",
        messages=[
            {
                "role": "user",
                "content": [
                    {
                        "type": "text",
                        "text": "Especificaciones de marca:\nEsta es una marca que se enfoca en crear ropa y accesorios de alta calidad y elegantes usando materiales ecológicos y métodos de producción éticos\nLa marca se dirige a consumidores ambientalmente conscientes de 25-40 años que valoran la moda, sostenibilidad y responsabilidad social.\nLa identidad de marca debe lograr los siguientes objetivos:\n1. Reflejar el compromiso de la marca con la sostenibilidad, prácticas éticas y administración ambiental.\n2. Atraer a la audiencia objetivo transmitiendo un sentido de estilo, calidad y tendencia.\n3. Diferenciar la marca de los competidores en el mercado de moda sostenible.\n4. Crear una conexión emocional fuerte con los consumidores e inspirarlos a tomar decisiones más amigables con el medio ambiente."
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
      system: "Tu tarea es crear un brief de diseño integral para una identidad de marca holística basada en las especificaciones dadas. La identidad de marca debe abarcar varios elementos como sugerencias para el nombre de la marca, logo, paleta de colores, tipografía, estilo visual, tono de voz y personalidad general de la marca. Asegúrate de que todos los elementos trabajen juntos armoniosamente para crear una experiencia de marca cohesiva y memorable que comunique efectivamente los valores, misión y propuesta única de venta de la marca a su audiencia objetivo. Sé detallado e integral y proporciona suficientes detalles específicos para que alguien pueda crear una identidad de marca verdaderamente única.",
      messages: [
        {
          "role": "user",
          "content": [
            {
              "type": "text",
              "text": "Especificaciones de marca:\nEsta es una marca que se enfoca en crear ropa y accesorios de alta calidad y elegantes usando materiales ecológicos y métodos de producción éticos\nLa marca se dirige a consumidores ambientalmente conscientes de 25-40 años que valoran la moda, sostenibilidad y responsabilidad social.\nLa identidad de marca debe lograr los siguientes objetivos:\n1. Reflejar el compromiso de la marca con la sostenibilidad, prácticas éticas y administración ambiental.\n2. Atraer a la audiencia objetivo transmitiendo un sentido de estilo, calidad y tendencia.\n3. Diferenciar la marca de los competidores en el mercado de moda sostenible.\n4. Crear una conexión emocional fuerte con los consumidores e inspirarlos a tomar decisiones más amigables con el medio ambiente."
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
        system="Tu tarea es crear un brief de diseño integral para una identidad de marca holística basada en las especificaciones dadas. La identidad de marca debe abarcar varios elementos como sugerencias para el nombre de la marca, logo, paleta de colores, tipografía, estilo visual, tono de voz y personalidad general de la marca. Asegúrate de que todos los elementos trabajen juntos armoniosamente para crear una experiencia de marca cohesiva y memorable que comunique efectivamente los valores, misión y propuesta única de venta de la marca a su audiencia objetivo. Sé detallado e integral y proporciona suficientes detalles específicos para que alguien pueda crear una identidad de marca verdaderamente única.",
        messages=[
            {
                "role": "user",
                "content": [
                    {
                        "type": "text",
                        "text": "Especificaciones de marca:\nEsta es una marca que se enfoca en crear ropa y accesorios de alta calidad y elegantes usando materiales ecológicos y métodos de producción éticos\nLa marca se dirige a consumidores ambientalmente conscientes de 25-40 años que valoran la moda, sostenibilidad y responsabilidad social.\nLa identidad de marca debe lograr los siguientes objetivos:\n1. Reflejar el compromiso de la marca con la sostenibilidad, prácticas éticas y administración ambiental.\n2. Atraer a la audiencia objetivo transmitiendo un sentido de estilo, calidad y tendencia.\n3. Diferenciar la marca de los competidores en el mercado de moda sostenible.\n4. Crear una conexión emocional fuerte con los consumidores e inspirarlos a tomar decisiones más amigables con el medio ambiente."
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
      system: "Tu tarea es crear un brief de diseño integral para una identidad de marca holística basada en las especificaciones dadas. La identidad de marca debe abarcar varios elementos como sugerencias para el nombre de la marca, logo, paleta de colores, tipografía, estilo visual, tono de voz y personalidad general de la marca. Asegúrate de que todos los elementos trabajen juntos armoniosamente para crear una experiencia de marca cohesiva y memorable que comunique efectivamente los valores, misión y propuesta única de venta de la marca a su audiencia objetivo. Sé detallado e integral y proporciona suficientes detalles específicos para que alguien pueda crear una identidad de marca verdaderamente única.",
      messages: [
        {
          "role": "user",
          "content": [
            {
              "type": "text",
              "text": "Especificaciones de marca:\nEsta es una marca que se enfoca en crear ropa y accesorios de alta calidad y elegantes usando materiales ecológicos y métodos de producción éticos\nLa marca se dirige a consumidores ambientalmente conscientes de 25-40 años que valoran la moda, sostenibilidad y responsabilidad social.\nLa identidad de marca debe lograr los siguientes objetivos:\n1. Reflejar el compromiso de la marca con la sostenibilidad, prácticas éticas y administración ambiental.\n2. Atraer a la audiencia objetivo transmitiendo un sentido de estilo, calidad y tendencia.\n3. Diferenciar la marca de los competidores en el mercado de moda sostenible.\n4. Crear una conexión emocional fuerte con los consumidores e inspirarlos a tomar decisiones más amigables con el medio ambiente."
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
        system="Tu tarea es crear un brief de diseño integral para una identidad de marca holística basada en las especificaciones dadas. La identidad de marca debe abarcar varios elementos como sugerencias para el nombre de la marca, logo, paleta de colores, tipografía, estilo visual, tono de voz y personalidad general de la marca. Asegúrate de que todos los elementos trabajen juntos armoniosamente para crear una experiencia de marca cohesiva y memorable que comunique efectivamente los valores, misión y propuesta única de venta de la marca a su audiencia objetivo. Sé detallado e integral y proporciona suficientes detalles específicos para que alguien pueda crear una identidad de marca verdaderamente única.",
        messages=[
            {
                "role": "user",
                "content": [
                    {
                        "type": "text",
                        "text": "Especificaciones de marca:\nEsta es una marca que se enfoca en crear ropa y accesorios de alta calidad y elegantes usando materiales ecológicos y métodos de producción éticos\nLa marca se dirige a consumidores ambientalmente conscientes de 25-40 años que valoran la moda, sostenibilidad y responsabilidad social.\nLa identidad de marca debe lograr los siguientes objetivos:\n1. Reflejar el compromiso de la marca con la sostenibilidad, prácticas éticas y administración ambiental.\n2. Atraer a la audiencia objetivo transmitiendo un sentido de estilo, calidad y tendencia.\n3. Diferenciar la marca de los competidores en el mercado de moda sostenible.\n4. Crear una conexión emocional fuerte con los consumidores e inspirarlos a tomar decisiones más amigables con el medio ambiente."
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
      system: "Tu tarea es crear un brief de diseño integral para una identidad de marca holística basada en las especificaciones dadas. La identidad de marca debe abarcar varios elementos como sugerencias para el nombre de la marca, logo, paleta de colores, tipografía, estilo visual, tono de voz y personalidad general de la marca. Asegúrate de que todos los elementos trabajen juntos armoniosamente para crear una experiencia de marca cohesiva y memorable que comunique efectivamente los valores, misión y propuesta única de venta de la marca a su audiencia objetivo. Sé detallado e integral y proporciona suficientes detalles específicos para que alguien pueda crear una identidad de marca verdaderamente única.",
      messages: [
        {
          "role": "user",
          "content": [
            {
              "type": "text",
              "text": "Especificaciones de marca:\nEsta es una marca que se enfoca en crear ropa y accesorios de alta calidad y elegantes usando materiales ecológicos y métodos de producción éticos\nLa marca se dirige a consumidores ambientalmente conscientes de 25-40 años que valoran la moda, sostenibilidad y responsabilidad social.\nLa identidad de marca debe lograr los siguientes objetivos:\n1. Reflejar el compromiso de la marca con la sostenibilidad, prácticas éticas y administración ambiental.\n2. Atraer a la audiencia objetivo transmitiendo un sentido de estilo, calidad y tendencia.\n3. Diferenciar la marca de los competidores en el mercado de moda sostenible.\n4. Crear una conexión emocional fuerte con los consumidores e inspirarlos a tomar decisiones más amigables con el medio ambiente."
            }
          ]
        }
      ]
    });
    console.log(msg);
    
    ```
</CodeGroup>