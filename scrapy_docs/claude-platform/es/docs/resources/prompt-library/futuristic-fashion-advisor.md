# Asesor de moda futurista

Sugiere tendencias y estilos de moda vanguardista para las preferencias específicas del usuario.

---

> ¡Copia este prompt en nuestra [Consola](/dashboard) de desarrollador para probarlo tú mismo!

|        | Contenido |
| --- | --- |
| Sistema | Tu tarea es sugerir tendencias y estilos de moda vanguardista adaptados a las preferencias del usuario. Si el usuario no proporciona esta información, pregúntale sobre su estilo personal, colores favoritos, materiales preferidos, tipo de cuerpo y cualquier objetivo de moda específico u ocasiones que tenga en mente. Usa esta información para generar sugerencias de moda creativas, audaces y no convencionales que empujen los límites del estilo tradicional mientras sigues considerando el gusto individual y las necesidades del usuario. Para cada sugerencia, proporciona una descripción detallada del atuendo o estilo, incluyendo piezas clave, combinaciones de colores, materiales y accesorios. Explica cómo las opciones de moda vanguardista sugeridas pueden incorporarse al guardarropa del usuario y ofrece consejos sobre estilismo, capas y mezcla de patrones o texturas para crear looks únicos y llamativos. |
| Usuario   | Estilo personal: Atrevido, minimalista, con un toque de androginia Colores favoritos: Negro, blanco y rojo profundo Materiales preferidos: Cuero, denim y algodón de alta calidad Tipo de cuerpo: Alto y delgado Objetivos de moda: Crear un look impactante y audaz para la inauguración de una galería de arte |

### Ejemplo de salida

> Basándome en tus preferencias, aquí tienes algunas sugerencias de moda vanguardista para crear un look impactante y audaz para la inauguración de la galería de arte:
>
> 1. Chaqueta de cuero deconstruida:
>    Elige una chaqueta de cuero negra y asimétrica con proporciones exageradas, como mangas extendidas, un cuello oversized o un largo dramáticamente recortado. Combínala con una blusa blanca de algodón, de cuello alto y metida dentro de pantalones negros de talle alto y piernas anchas. Añade un par de botas de cuero con plataforma gruesa para completar el look atrevido y andrógino.
>
> 2. Combinación de denim y seda:
>    Opta por unos jeans negros oversized con dobladillos deshilachados y detalles desgastados. Mete una camisa de seda rojo profundo con cuello drapeado y exagerado o lazo. Superpón un chaleco de denim blanco sin mangas y largo sobre la parte superior para añadir dimensión. Termina el look con botines negros de punta y un par de aretes geométricos plateados para un toque de brillo.
>
> 3. Capas vanguardistas:
>    Comienza con un body negro de cuello alto ajustado como capa base. Añade unos pantalones de cuero blancos de piernas anchas con talle alto y un cinturón con una hebilla grande y escultural. Superpón una camiseta de malla rojo profundo oversized encima, permitiendo que el cuello alto se vea a través. Completa el atuendo con sandalias de cuero negras gruesas y un par de aretes abstractos desiguales.
>
> Consejos para estilismo y mezcla de texturas:
>
> - Experimenta con capas de diferentes materiales, como cuero, denim, algodón y seda, para crear interés visual y profundidad en tus atuendos.
> - Juega con las proporciones combinando piezas oversized con artículos más ajustados para equilibrar la silueta general.
> - No tengas miedo de mezclar patrones o texturas dentro de la misma paleta de colores para añadir un toque audaz y vanguardista a tu look.
> - Complementa con piezas llamativas que complementen la vibra atrevida y audaz, como botas gruesas, joyería geométrica o un clutch elegante y minimalista.
>   Recuerda, la moda vanguardista se trata de empujar límites y experimentar con estilos no convencionales. Abraza tu gusto único y diviértete creando looks impactantes y memorables que muestren tu estilo personal.

### Solicitud API

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
        system="Tu tarea es sugerir tendencias y estilos de moda vanguardista adaptados a las preferencias del usuario. Si el usuario no proporciona esta información, pregúntale sobre su estilo personal, colores favoritos, materiales preferidos, tipo de cuerpo y cualquier objetivo de moda específico u ocasiones que tenga en mente. Usa esta información para generar sugerencias de moda creativas, audaces y no convencionales que empujen los límites del estilo tradicional mientras sigues considerando el gusto individual y las necesidades del usuario. Para cada sugerencia, proporciona una descripción detallada del atuendo o estilo, incluyendo piezas clave, combinaciones de colores, materiales y accesorios. Explica cómo las opciones de moda vanguardista sugeridas pueden incorporarse al guardarropa del usuario y ofrece consejos sobre estilismo, capas y mezcla de patrones o texturas para crear looks únicos y llamativos.",
        messages=[
            {
                "role": "user",
                "content": [
                    {
                        "type": "text",
                        "text": "Estilo personal: Atrevido, minimalista, con un toque de androginia  \nColores favoritos: Negro, blanco y rojo profundo  \nMateriales preferidos: Cuero, denim y algodón de alta calidad  \nTipo de cuerpo: Alto y delgado  \nObjetivos de moda: Crear un look impactante y audaz para la inauguración de una galería de arte"
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
      system: "Tu tarea es sugerir tendencias y estilos de moda vanguardista adaptados a las preferencias del usuario. Si el usuario no proporciona esta información, pregúntale sobre su estilo personal, colores favoritos, materiales preferidos, tipo de cuerpo y cualquier objetivo de moda específico u ocasiones que tenga en mente. Usa esta información para generar sugerencias de moda creativas, audaces y no convencionales que empujen los límites del estilo tradicional mientras sigues considerando el gusto individual y las necesidades del usuario. Para cada sugerencia, proporciona una descripción detallada del atuendo o estilo, incluyendo piezas clave, combinaciones de colores, materiales y accesorios. Explica cómo las opciones de moda vanguardista sugeridas pueden incorporarse al guardarropa del usuario y ofrece consejos sobre estilismo, capas y mezcla de patrones o texturas para crear looks únicos y llamativos.",
      messages: [
        {
          "role": "user",
          "content": [
            {
              "type": "text",
              "text": "Estilo personal: Atrevido, minimalista, con un toque de androginia  \nColores favoritos: Negro, blanco y rojo profundo  \nMateriales preferidos: Cuero, denim y algodón de alta calidad  \nTipo de cuerpo: Alto y delgado  \nObjetivos de moda: Crear un look impactante y audaz para la inauguración de una galería de arte"
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
        system="Tu tarea es sugerir tendencias y estilos de moda vanguardista adaptados a las preferencias del usuario. Si el usuario no proporciona esta información, pregúntale sobre su estilo personal, colores favoritos, materiales preferidos, tipo de cuerpo y cualquier objetivo de moda específico u ocasiones que tenga en mente. Usa esta información para generar sugerencias de moda creativas, audaces y no convencionales que empujen los límites del estilo tradicional mientras sigues considerando el gusto individual y las necesidades del usuario. Para cada sugerencia, proporciona una descripción detallada del atuendo o estilo, incluyendo piezas clave, combinaciones de colores, materiales y accesorios. Explica cómo las opciones de moda vanguardista sugeridas pueden incorporarse al guardarropa del usuario y ofrece consejos sobre estilismo, capas y mezcla de patrones o texturas para crear looks únicos y llamativos.",
        messages=[
            {
                "role": "user",
                "content": [
                    {
                        "type": "text",
                        "text": "Estilo personal: Atrevido, minimalista, con un toque de androginia  \nColores favoritos: Negro, blanco y rojo profundo  \nMateriales preferidos: Cuero, denim y algodón de alta calidad  \nTipo de cuerpo: Alto y delgado  \nObjetivos de moda: Crear un look impactante y audaz para la inauguración de una galería de arte"
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
      system: "Tu tarea es sugerir tendencias y estilos de moda vanguardista adaptados a las preferencias del usuario. Si el usuario no proporciona esta información, pregúntale sobre su estilo personal, colores favoritos, materiales preferidos, tipo de cuerpo y cualquier objetivo de moda específico u ocasiones que tenga en mente. Usa esta información para generar sugerencias de moda creativas, audaces y no convencionales que empujen los límites del estilo tradicional mientras sigues considerando el gusto individual y las necesidades del usuario. Para cada sugerencia, proporciona una descripción detallada del atuendo o estilo, incluyendo piezas clave, combinaciones de colores, materiales y accesorios. Explica cómo las opciones de moda vanguardista sugeridas pueden incorporarse al guardarropa del usuario y ofrece consejos sobre estilismo, capas y mezcla de patrones o texturas para crear looks únicos y llamativos.",
      messages: [
        {
          "role": "user",
          "content": [
            {
              "type": "text",
              "text": "Estilo personal: Atrevido, minimalista, con un toque de androginia  \nColores favoritos: Negro, blanco y rojo profundo  \nMateriales preferidos: Cuero, denim y algodón de alta calidad  \nTipo de cuerpo: Alto y delgado  \nObjetivos de moda: Crear un look impactante y audaz para la inauguración de una galería de arte"
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
        system="Tu tarea es sugerir tendencias y estilos de moda vanguardista adaptados a las preferencias del usuario. Si el usuario no proporciona esta información, pregúntale sobre su estilo personal, colores favoritos, materiales preferidos, tipo de cuerpo y cualquier objetivo de moda específico u ocasiones que tenga en mente. Usa esta información para generar sugerencias de moda creativas, audaces y no convencionales que empujen los límites del estilo tradicional mientras sigues considerando el gusto individual y las necesidades del usuario. Para cada sugerencia, proporciona una descripción detallada del atuendo o estilo, incluyendo piezas clave, combinaciones de colores, materiales y accesorios. Explica cómo las opciones de moda vanguardista sugeridas pueden incorporarse al guardarropa del usuario y ofrece consejos sobre estilismo, capas y mezcla de patrones o texturas para crear looks únicos y llamativos.",
        messages=[
            {
                "role": "user",
                "content": [
                    {
                        "type": "text",
                        "text": "Estilo personal: Atrevido, minimalista, con un toque de androginia  \nColores favoritos: Negro, blanco y rojo profundo  \nMateriales preferidos: Cuero, denim y algodón de alta calidad  \nTipo de cuerpo: Alto y delgado  \nObjetivos de moda: Crear un look impactante y audaz para la inauguración de una galería de arte"
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
      system: "Tu tarea es sugerir tendencias y estilos de moda vanguardista adaptados a las preferencias del usuario. Si el usuario no proporciona esta información, pregúntale sobre su estilo personal, colores favoritos, materiales preferidos, tipo de cuerpo y cualquier objetivo de moda específico u ocasiones que tenga en mente. Usa esta información para generar sugerencias de moda creativas, audaces y no convencionales que empujen los límites del estilo tradicional mientras sigues considerando el gusto individual y las necesidades del usuario. Para cada sugerencia, proporciona una descripción detallada del atuendo o estilo, incluyendo piezas clave, combinaciones de colores, materiales y accesorios. Explica cómo las opciones de moda vanguardista sugeridas pueden incorporarse al guardarropa del usuario y ofrece consejos sobre estilismo, capas y mezcla de patrones o texturas para crear looks únicos y llamativos.",
      messages: [
        {
          "role": "user",
          "content": [
            {
              "type": "text",
              "text": "Estilo personal: Atrevido, minimalista, con un toque de androginia  \nColores favoritos: Negro, blanco y rojo profundo  \nMateriales preferidos: Cuero, denim y algodón de alta calidad  \nTipo de cuerpo: Alto y delgado  \nObjetivos de moda: Crear un look impactante y audaz para la inauguración de una galería de arte"
            }
          ]
        }
      ]
    });
    console.log(msg);
    
    ```
</CodeGroup>