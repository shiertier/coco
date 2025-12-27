# Hechicero de hojas de cálculo

Genera hojas de cálculo CSV con varios tipos de datos.

---

> ¡Copia este prompt en nuestra [Consola](/dashboard) de desarrollador para probarlo tú mismo!

|        | Contenido |
| --- | --- |
| Sistema | Tu tarea es generar una hoja de cálculo CSV que contenga el tipo de datos especificado. La hoja de cálculo debe estar bien organizada, con encabezados de columna claros y tipos de datos apropiados para cada columna. Asegúrate de que los datos sean realistas, diversos y formateados de manera consistente. Incluye un mínimo de 10 filas de datos, sin contar la fila de encabezado. |
| Usuario   | Crea una hoja de cálculo que contenga información sobre libros en una biblioteca. Incluye las siguientes columnas: ID del Libro, Título, Autor, Género, Año de Publicación y Copias Disponibles. Asegúrate de que los datos cubran una variedad de géneros y años de publicación, y que el número de copias disponibles varíe entre libros.                             |

### Salida de ejemplo

> Aquí tienes una hoja de cálculo CSV que contiene información sobre libros en una biblioteca, basada en tus requisitos:
>
> ID del Libro,Título,Autor,Género,Año de Publicación,Copias Disponibles  
> 1,Matar a un ruiseñor,Harper Lee,Clásico,1960,5  
> 2,1984,George Orwell,Distópico,1949,3  
> 3,Orgullo y prejuicio,Jane Austen,Romance,1813,7  
> 4,El guardián entre el centeno,J.D. Salinger,Mayoría de edad,1951,2  
> 5,Guía del autoestopista galáctico,Douglas Adams,Ciencia Ficción,1979,4  
> 6,El código Da Vinci,Dan Brown,Misterio,2003,6  
> 7,Cometas en el cielo,Khaled Hosseini,Ficción Histórica,2003,3  
> 8,El alquimista,Paulo Coelho,Ficción Filosófica,1988,5  
> 9,Los juegos del hambre,Suzanne Collins,Adulto Joven,2008,8  
> 10,La chica del dragón tatuado,Stieg Larsson,Crimen,2005,4  
> 11,El hobbit,J.R.R. Tolkien,Fantasía,1937,6  
> 12,El diario de una niña,Anne Frank,Autobiografía,1947,2  
> 13,Bajo la misma estrella,John Green,Adulto Joven,2012,5  
> 14,El guardián entre el centeno,J.D. Salinger,Mayoría de edad,1951,3  
> 15,El curioso incidente del perro a medianoche,Mark Haddon,Misterio,2003,4

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
        max_tokens=1000,
        temperature=0,
        system="Tu tarea es generar una hoja de cálculo CSV que contenga el tipo de datos especificado. La hoja de cálculo debe estar bien organizada, con encabezados de columna claros y tipos de datos apropiados para cada columna. Asegúrate de que los datos sean realistas, diversos y formateados de manera consistente. Incluye un mínimo de 10 filas de datos, sin contar la fila de encabezado.",
        messages=[
            {
                "role": "user",
                "content": [
                    {
                        "type": "text",
                        "text": "Crea una hoja de cálculo que contenga información sobre libros en una biblioteca. Incluye las siguientes columnas: ID del Libro, Título, Autor, Género, Año de Publicación y Copias Disponibles. Asegúrate de que los datos cubran una variedad de géneros y años de publicación, y que el número de copias disponibles varíe entre libros."
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
      max_tokens: 1000,
      temperature: 0,
      system: "Tu tarea es generar una hoja de cálculo CSV que contenga el tipo de datos especificado. La hoja de cálculo debe estar bien organizada, con encabezados de columna claros y tipos de datos apropiados para cada columna. Asegúrate de que los datos sean realistas, diversos y formateados de manera consistente. Incluye un mínimo de 10 filas de datos, sin contar la fila de encabezado.",
      messages: [
        {
          "role": "user",
          "content": [
            {
              "type": "text",
              "text": "Crea una hoja de cálculo que contenga información sobre libros en una biblioteca. Incluye las siguientes columnas: ID del Libro, Título, Autor, Género, Año de Publicación y Copias Disponibles. Asegúrate de que los datos cubran una variedad de géneros y años de publicación, y que el número de copias disponibles varíe entre libros."
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
        system="Tu tarea es generar una hoja de cálculo CSV que contenga el tipo de datos especificado. La hoja de cálculo debe estar bien organizada, con encabezados de columna claros y tipos de datos apropiados para cada columna. Asegúrate de que los datos sean realistas, diversos y formateados de manera consistente. Incluye un mínimo de 10 filas de datos, sin contar la fila de encabezado.",
        messages=[
            {
                "role": "user",
                "content": [
                    {
                        "type": "text",
                        "text": "Crea una hoja de cálculo que contenga información sobre libros en una biblioteca. Incluye las siguientes columnas: ID del Libro, Título, Autor, Género, Año de Publicación y Copias Disponibles. Asegúrate de que los datos cubran una variedad de géneros y años de publicación, y que el número de copias disponibles varíe entre libros."
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
      system: "Tu tarea es generar una hoja de cálculo CSV que contenga el tipo de datos especificado. La hoja de cálculo debe estar bien organizada, con encabezados de columna claros y tipos de datos apropiados para cada columna. Asegúrate de que los datos sean realistas, diversos y formateados de manera consistente. Incluye un mínimo de 10 filas de datos, sin contar la fila de encabezado.",
      messages: [
        {
          "role": "user",
          "content": [
            {
              "type": "text",
              "text": "Crea una hoja de cálculo que contenga información sobre libros en una biblioteca. Incluye las siguientes columnas: ID del Libro, Título, Autor, Género, Año de Publicación y Copias Disponibles. Asegúrate de que los datos cubran una variedad de géneros y años de publicación, y que el número de copias disponibles varíe entre libros."
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
        system="Tu tarea es generar una hoja de cálculo CSV que contenga el tipo de datos especificado. La hoja de cálculo debe estar bien organizada, con encabezados de columna claros y tipos de datos apropiados para cada columna. Asegúrate de que los datos sean realistas, diversos y formateados de manera consistente. Incluye un mínimo de 10 filas de datos, sin contar la fila de encabezado.",
        messages=[
            {
                "role": "user",
                "content": [
                    {
                        "type": "text",
                        "text": "Crea una hoja de cálculo que contenga información sobre libros en una biblioteca. Incluye las siguientes columnas: ID del Libro, Título, Autor, Género, Año de Publicación y Copias Disponibles. Asegúrate de que los datos cubran una variedad de géneros y años de publicación, y que el número de copias disponibles varíe entre libros."
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
      max_tokens: 1000,
      temperature: 0,
      system: "Tu tarea es generar una hoja de cálculo CSV que contenga el tipo de datos especificado. La hoja de cálculo debe estar bien organizada, con encabezados de columna claros y tipos de datos apropiados para cada columna. Asegúrate de que los datos sean realistas, diversos y formateados de manera consistente. Incluye un mínimo de 10 filas de datos, sin contar la fila de encabezado.",
      messages: [
        {
          "role": "user",
          "content": [
            {
              "type": "text",
              "text": "Crea una hoja de cálculo que contenga información sobre libros en una biblioteca. Incluye las siguientes columnas: ID del Libro, Título, Autor, Género, Año de Publicación y Copias Disponibles. Asegúrate de que los datos cubran una variedad de géneros y años de publicación, y que el número de copias disponibles varíe entre libros."
            }
          ]
        }
      ]
    });
    console.log(msg);
    
    ```
</CodeGroup>