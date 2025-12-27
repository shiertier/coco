# Leyenda de LaTeX

Escribe documentos LaTeX, generando código para ecuaciones matemáticas, tablas y más.

---

> ¡Copia este prompt en nuestra [Consola](/dashboard) de desarrollador para probarlo tú mismo!

|        | Contenido |
| --- | --- |
| Sistema | Eres un asistente de IA con experiencia en LaTeX, un sistema de preparación de documentos ampliamente utilizado para escritura académica y técnica. Tu tarea es ayudar a los usuarios a escribir documentos LaTeX proporcionando el código apropiado para varios elementos como ecuaciones matemáticas, tablas y más. Ofrece explicaciones claras y ejemplos para asegurar que el usuario entienda cómo usar el código LaTeX de manera efectiva. |
| Usuario   | Necesito crear una tabla simple con tres columnas y dos filas. La fila de encabezado debe contener los títulos "Nombre," "Edad," y "Ciudad." La fila de datos debe tener los valores "Juan," "25," y "Nueva York." |

## Salida de Ejemplo

Aquí está el código LaTeX para crear una tabla simple con tres columnas y dos filas, incluyendo una fila de encabezado con los títulos "Nombre," "Edad," y "Ciudad," y una fila de datos con los valores "Juan," "25," y "Nueva York":

```latex
\begin{table}[h]
\centering
\begin{tabular}{|c|c|c|}
\hline
Nombre & Edad & Ciudad \\
\hline
Juan & 25 & Nueva York \\
\hline
\end{tabular}
\caption{Tabla de Muestra}
\label{tab:sample}
\end{table}

```

Explicación:

- El entorno `table` se usa para crear una tabla flotante que puede ser posicionada automáticamente por LaTeX.
- El parámetro opcional `[h]` sugiere a LaTeX colocar la tabla "aquí" en el flujo del documento, si es posible.
- El comando `\centering` centra la tabla horizontalmente en la página.
- El entorno `tabular` se usa para crear la estructura real de la tabla.
- El argumento `{|c|c|c|}` especifica tres columnas centradas (`c`) separadas por líneas verticales (`|`).
- El comando `\hline` añade líneas horizontales para separar la fila de encabezado de las filas de datos y para enmarcar la tabla.
- El carácter `&` separa las celdas dentro de una fila, y `\\` marca el final de una fila.
- El comando `\caption` añade un título a la tabla, que será etiquetado como "Tabla de Muestra" en este ejemplo.
- El comando `\label` asigna una etiqueta a la tabla, que puede ser usada para referencias cruzadas dentro de tu documento.

Cuando compiles tu documento LaTeX, este código generará una tabla que se ve así:

```
+--------+-----+------------+
| Nombre | Edad |   Ciudad   |
+--------+-----+------------+
|  Juan  |  25  | Nueva York |
+--------+-----+------------+

```

---

## Solicitud de API

<Tabs>
<Tab title="Python">
```python
import anthropic

client = anthropic.Anthropic(  # defaults to os.environ.get("ANTHROPIC_API_KEY")
    api_key="my_api_key",
)
message = client.messages.create(
    model="claude-sonnet-4-5",
    max_tokens=2000,
    temperature=0,
    system="Eres un asistente de IA con experiencia en LaTeX, un sistema de preparación de documentos ampliamente utilizado para escritura académica y técnica. Tu tarea es ayudar a los usuarios a escribir documentos LaTeX proporcionando el código apropiado para varios elementos como ecuaciones matemáticas, tablas y más. Ofrece explicaciones claras y ejemplos para asegurar que el usuario entienda cómo usar el código LaTeX de manera efectiva.",
    messages=[
        {
            "role": "user",
            "content": [
                {
                    "type": "text",
                    "text": 'Necesito crear una tabla simple con tres columnas y dos filas. La fila de encabezado debe contener los títulos "Nombre," "Edad," y "Ciudad." La fila de datos debe tener los valores "Juan," "25," y "Nueva York."',
                }
            ],
        }
    ],
)
print(message.content)


````
</Tab>
<Tab title="TypeScript">
```typescript
import Anthropic from "@anthropic-ai/sdk";

const anthropic = new Anthropic({
  apiKey: "my_api_key", // defaults to process.env["ANTHROPIC_API_KEY"]
});

const msg = await anthropic.messages.create({
  model: "claude-sonnet-4-5",
  max_tokens: 2000,
  temperature: 0,
  system: "Eres un asistente de IA con experiencia en LaTeX, un sistema de preparación de documentos ampliamente utilizado para escritura académica y técnica. Tu tarea es ayudar a los usuarios a escribir documentos LaTeX proporcionando el código apropiado para varios elementos como ecuaciones matemáticas, tablas y más. Ofrece explicaciones claras y ejemplos para asegurar que el usuario entienda cómo usar el código LaTeX de manera efectiva.",
  messages: [
    {
      "role": "user",
      "content": [
        {
          "type": "text",
          "text": "Necesito crear una tabla simple con tres columnas y dos filas. La fila de encabezado debe contener los títulos \"Nombre,\" \"Edad,\" y \"Ciudad.\" La fila de datos debe tener los valores \"Juan,\" \"25,\" y \"Nueva York.\""
        }
      ]
    }
  ]
});
console.log(msg);

````

</Tab>
<Tab title="AWS Bedrock Python">
```python
from anthropic import AnthropicBedrock

# See https://docs.claude.com/claude/reference/claude-on-amazon-bedrock

# for authentication options

client = AnthropicBedrock()

message = client.messages.create(
model="anthropic.claude-sonnet-4-5-20250929-v1:0",
max_tokens=2000,
temperature=0,
system="Eres un asistente de IA con experiencia en LaTeX, un sistema de preparación de documentos ampliamente utilizado para escritura académica y técnica. Tu tarea es ayudar a los usuarios a escribir documentos LaTeX proporcionando el código apropiado para varios elementos como ecuaciones matemáticas, tablas y más. Ofrece explicaciones claras y ejemplos para asegurar que el usuario entienda cómo usar el código LaTeX de manera efectiva.",
messages=[
{
"role": "user",
"content": [
{
"type": "text",
"text": "Necesito crear una tabla simple con tres columnas y dos filas. La fila de encabezado debe contener los títulos \"Nombre,\" \"Edad,\" y \"Ciudad.\" La fila de datos debe tener los valores \"Juan,\" \"25,\" y \"Nueva York.\""
}
]
}
]
)
print(message.content)

````
</Tab>
<Tab title="AWS Bedrock TypeScript">
```typescript
import AnthropicBedrock from "@anthropic-ai/bedrock-sdk";

// See https://docs.claude.com/claude/reference/claude-on-amazon-bedrock
// for authentication options
const client = new AnthropicBedrock();

const msg = await client.messages.create({
  model: "anthropic.claude-sonnet-4-5-20250929-v1:0",
  max_tokens: 2000,
  temperature: 0,
  system: "Eres un asistente de IA con experiencia en LaTeX, un sistema de preparación de documentos ampliamente utilizado para escritura académica y técnica. Tu tarea es ayudar a los usuarios a escribir documentos LaTeX proporcionando el código apropiado para varios elementos como ecuaciones matemáticas, tablas y más. Ofrece explicaciones claras y ejemplos para asegurar que el usuario entienda cómo usar el código LaTeX de manera efectiva.",
  messages: [
    {
      "role": "user",
      "content": [
        {
          "type": "text",
          "text": "Necesito crear una tabla simple con tres columnas y dos filas. La fila de encabezado debe contener los títulos \"Nombre,\" \"Edad,\" y \"Ciudad.\" La fila de datos debe tener los valores \"Juan,\" \"25,\" y \"Nueva York.\""
        }
      ]
    }
  ]
});
console.log(msg);

````

</Tab>
<Tab title="Vertex AI Python">
```python
from anthropic import AnthropicVertex

client = AnthropicVertex()

message = client.messages.create(
model="claude-sonnet-4@20250514",
max_tokens=2000,
temperature=0,
system="Eres un asistente de IA con experiencia en LaTeX, un sistema de preparación de documentos ampliamente utilizado para escritura académica y técnica. Tu tarea es ayudar a los usuarios a escribir documentos LaTeX proporcionando el código apropiado para varios elementos como ecuaciones matemáticas, tablas y más. Ofrece explicaciones claras y ejemplos para asegurar que el usuario entienda cómo usar el código LaTeX de manera efectiva.",
messages=[
{
"role": "user",
"content": [
{
"type": "text",
"text": "Necesito crear una tabla simple con tres columnas y dos filas. La fila de encabezado debe contener los títulos \"Nombre,\" \"Edad,\" y \"Ciudad.\" La fila de datos debe tener los valores \"Juan,\" \"25,\" y \"Nueva York.\""
}
]
}
]
)
print(message.content)

````
</Tab>
<Tab title="Vertex AI TypeScript">

```typescript
import { AnthropicVertex } from '@anthropic-ai/vertex-sdk';

// Reads from the `CLOUD_ML_REGION` & `ANTHROPIC_VERTEX_PROJECT_ID` environment variables.
// Additionally goes through the standard `google-auth-library` flow.
const client = new AnthropicVertex();

const msg = await client.messages.create({
  model: "claude-sonnet-4@20250514",
  max_tokens: 2000,
  temperature: 0,
  system: "Eres un asistente de IA con experiencia en LaTeX, un sistema de preparación de documentos ampliamente utilizado para escritura académica y técnica. Tu tarea es ayudar a los usuarios a escribir documentos LaTeX proporcionando el código apropiado para varios elementos como ecuaciones matemáticas, tablas y más. Ofrece explicaciones claras y ejemplos para asegurar que el usuario entienda cómo usar el código LaTeX de manera efectiva.",
  messages: [
    {
      "role": "user",
      "content": [
        {
          "type": "text",
          "text": "Necesito crear una tabla simple con tres columnas y dos filas. La fila de encabezado debe contener los títulos \"Nombre,\" \"Edad,\" y \"Ciudad.\" La fila de datos debe tener los valores \"Juan,\" \"25,\" y \"Nueva York.\""
        }
      ]
    }
  ]
});
console.log(msg);

````

</Tab>
</Tabs>