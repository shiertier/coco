# Simplificador de segundo grado

Haz que los textos complejos sean fáciles de entender para los jóvenes estudiantes.

---

> ¡Copia este prompt en nuestra [Consola](/dashboard) de desarrolladores para probarlo tú mismo!

|        | Contenido |
| --- | --- |
| Sistema | Tu tarea es tomar el texto proporcionado y reescribirlo de una manera que sea fácil de leer y entender para jóvenes estudiantes de los grados 3-5. Simplifica el vocabulario avanzado, divide las oraciones largas, explica conceptos difíciles en lenguaje sencillo, y presenta la información de manera clara y atractiva. El texto reescrito corto debe transmitir las ideas centrales del texto original de manera apropiada para la edad. |
| Usuario   | Las mitocondrias, a menudo referidas como las centrales eléctricas de la célula, son orgánulos esenciales que juegan un papel crucial en la producción de trifosfato de adenosina (ATP) a través del proceso de respiración celular. El ATP es la moneda energética principal de la célula, permitiendo que ocurran varias funciones celulares y reacciones bioquímicas. |

## Ejemplo de Salida

Las mitocondrias son partes diminutas dentro de las células que funcionan como pequeñas estaciones de energía. Hacen una molécula especial llamada ATP, que es como el dinero energético de la célula. El ATP ayuda a la célula a hacer trabajos importantes y la mantiene funcionando sin problemas, igual que como el dinero ayuda a las personas a comprar cosas que necesitan. Las mitocondrias son muy importantes porque sin ellas, las células no tendrían la energía para funcionar correctamente.

---

## Solicitud de API

<Tabs>
<Tab title="Python">

```python
import anthropic

client = anthropic.Anthropic(
  # defaults to os.environ.get("ANTHROPIC_API_KEY")
  api_key="my_api_key",
)
message = client.messages.create(
  model="claude-sonnet-4-5",
  max_tokens=1000,
  temperature=0,
  system="Tu tarea es tomar el texto proporcionado y reescribirlo de una manera que sea fácil de leer y entender para jóvenes estudiantes de los grados 3-5. Simplifica el vocabulario avanzado, divide las oraciones largas, explica conceptos difíciles en lenguaje sencillo, y presenta la información de manera clara y atractiva. El texto reescrito corto debe transmitir las ideas centrales del texto original de manera apropiada para la edad.",
  messages=[
    {
      "role": "user",
      "content": [
        {
          "type": "text",
          "text": "Las mitocondrias, a menudo referidas como las centrales eléctricas de la célula, son orgánulos esenciales que juegan un papel crucial en la producción de trifosfato de adenosina (ATP) a través del proceso de respiración celular. El ATP es la moneda energética principal de la célula, permitiendo que ocurran varias funciones celulares y reacciones bioquímicas."
        }
      ]
    }
  ]
)
print(message.content)

```

</Tab>
<Tab title="TypeScript">

```typescript
import Anthropic from "@anthropic-ai/sdk";

const anthropic = new Anthropic({
  apiKey: "my_api_key", // defaults to process.env["ANTHROPIC_API_KEY"]
});

const msg = await anthropic.messages.create({
  model: "claude-sonnet-4-5",
  max_tokens: 1000,
  temperature: 0,
  system: "Tu tarea es tomar el texto proporcionado y reescribirlo de una manera que sea fácil de leer y entender para jóvenes estudiantes de los grados 3-5. Simplifica el vocabulario avanzado, divide las oraciones largas, explica conceptos difíciles en lenguaje sencillo, y presenta la información de manera clara y atractiva. El texto reescrito corto debe transmitir las ideas centrales del texto original de manera apropiada para la edad.",
  messages: [
    {
      "role": "user",
      "content": [
        {
          "type": "text",
          "text": "Las mitocondrias, a menudo referidas como las centrales eléctricas de la célula, son orgánulos esenciales que juegan un papel crucial en la producción de trifosfato de adenosina (ATP) a través del proceso de respiración celular. El ATP es la moneda energética principal de la célula, permitiendo que ocurran varias funciones celulares y reacciones bioquímicas."
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
    max_tokens=1000,
    temperature=0,
    system="Tu tarea es tomar el texto proporcionado y reescribirlo de una manera que sea fácil de leer y entender para jóvenes estudiantes de los grados 3-5. Simplifica el vocabulario avanzado, divide las oraciones largas, explica conceptos difíciles en lenguaje sencillo, y presenta la información de manera clara y atractiva. El texto reescrito corto debe transmitir las ideas centrales del texto original de manera apropiada para la edad.",
    messages=[
        {
            "role": "user",
            "content": [
                {
                    "type": "text",
                    "text": "Las mitocondrias, a menudo referidas como las centrales eléctricas de la célula, son orgánulos esenciales que juegan un papel crucial en la producción de trifosfato de adenosina (ATP) a través del proceso de respiración celular. El ATP es la moneda energética principal de la célula, permitiendo que ocurran varias funciones celulares y reacciones bioquímicas."
                }
            ]
        }
    ]
)
print(message.content)

```

</Tab>
<Tab title="AWS Bedrock TypeScript">

```typescript
import AnthropicBedrock from "@anthropic-ai/bedrock-sdk";

// See https://docs.claude.com/claude/reference/claude-on-amazon-bedrock
// for authentication options
const client = new AnthropicBedrock();

const msg = await client.messages.create({
  model: "anthropic.claude-sonnet-4-5-20250929-v1:0",
  max_tokens: 1000,
  temperature: 0,
  system: "Tu tarea es tomar el texto proporcionado y reescribirlo de una manera que sea fácil de leer y entender para jóvenes estudiantes de los grados 3-5. Simplifica el vocabulario avanzado, divide las oraciones largas, explica conceptos difíciles en lenguaje sencillo, y presenta la información de manera clara y atractiva. El texto reescrito corto debe transmitir las ideas centrales del texto original de manera apropiada para la edad.",
  messages: [
    {
      "role": "user",
      "content": [
        {
          "type": "text",
          "text": "Las mitocondrias, a menudo referidas como las centrales eléctricas de la célula, son orgánulos esenciales que juegan un papel crucial en la producción de trifosfato de adenosina (ATP) a través del proceso de respiración celular. El ATP es la moneda energética principal de la célula, permitiendo que ocurran varias funciones celulares y reacciones bioquímicas."
        }
      ]
    }
  ]
});
console.log(msg);

```

</Tab>
<Tab title="Vertex AI Python">
```python
from anthropic import AnthropicVertex

client = AnthropicVertex()

message = client.messages.create(
model="claude-sonnet-4@20250514",
max_tokens=1000,
temperature=0,
system="Tu tarea es tomar el texto proporcionado y reescribirlo de una manera que sea fácil de leer y entender para jóvenes estudiantes de los grados 3-5. Simplifica el vocabulario avanzado, divide las oraciones largas, explica conceptos difíciles en lenguaje sencillo, y presenta la información de manera clara y atractiva. El texto reescrito corto debe transmitir las ideas centrales del texto original de manera apropiada para la edad.",
messages=[
{
"role": "user",
"content": [
{
"type": "text",
"text": "Las mitocondrias, a menudo referidas como las centrales eléctricas de la célula, son orgánulos esenciales que juegan un papel crucial en la producción de trifosfato de adenosina (ATP) a través del proceso de respiración celular. El ATP es la moneda energética principal de la célula, permitiendo que ocurran varias funciones celulares y reacciones bioquímicas."
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
  max_tokens: 1000,
  temperature: 0,
  system: "Tu tarea es tomar el texto proporcionado y reescribirlo de una manera que sea fácil de leer y entender para jóvenes estudiantes de los grados 3-5. Simplifica el vocabulario avanzado, divide las oraciones largas, explica conceptos difíciles en lenguaje sencillo, y presenta la información de manera clara y atractiva. El texto reescrito corto debe transmitir las ideas centrales del texto original de manera apropiada para la edad.",
  messages: [
    {
      "role": "user",
      "content": [
        {
          "type": "text",
          "text": "Las mitocondrias, a menudo referidas como las centrales eléctricas de la célula, son orgánulos esenciales que juegan un papel crucial en la producción de trifosfato de adenosina (ATP) a través del proceso de respiración celular. El ATP es la moneda energética principal de la célula, permitiendo que ocurran varias funciones celulares y reacciones bioquímicas."
        }
      ]
    }
  ]
});
console.log(msg);

````

</Tab>
</Tabs>