# Codificador de emojis

Convierte texto plano en mensajes divertidos y expresivos con emojis.

---

> ¡Copia este prompt en nuestra [Consola](/dashboard) de desarrollador para probarlo tú mismo!

|        | Contenido |
| --- | --- |
| Sistema | Tu tarea es tomar el mensaje de texto plano proporcionado y convertirlo en un mensaje expresivo y rico en emojis que transmita el mismo significado e intención. Reemplaza palabras y frases clave con emojis relevantes cuando sea apropiado para agregar interés visual y emoción. Usa emojis de manera creativa pero asegúrate de que el mensaje permanezca claro y fácil de entender. No cambies el mensaje principal ni agregues nueva información. |
| Usuario   | Todo el mundo es un escenario, y todos los hombres y mujeres son meramente actores. Tienen sus salidas y sus entradas; Y un hombre en su tiempo interpreta muchos papeles. |

## Ejemplo de salida

Todo el 🌍 es un 🎭, y todos los 👨 y 👩 son meramente 🎭🎬. Tienen sus 🚪🚶‍♂️ y sus 🚶‍♀️🚪; Y un 👨 en su ⌛ interpreta muchos 🎭.

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
    max_tokens=1000,
    temperature=0,
    system="Tu tarea es tomar el mensaje de texto plano proporcionado y convertirlo en un mensaje expresivo y rico en emojis que transmita el mismo significado e intención. Reemplaza palabras y frases clave con emojis relevantes cuando sea apropiado para agregar interés visual y emoción. Usa emojis de manera creativa pero asegúrate de que el mensaje permanezca claro y fácil de entender. No cambies el mensaje principal ni agregues nueva información.",
    messages=[
        {
            "role": "user",
            "content": [
                {
                    "type": "text",
                    "text": "Todo el mundo es un escenario, y todos los hombres y mujeres son meramente actores. Tienen sus salidas y sus entradas; Y un hombre en su tiempo interpreta muchos papeles.",
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
  max_tokens: 1000,
  temperature: 0,
  system: "Tu tarea es tomar el mensaje de texto plano proporcionado y convertirlo en un mensaje expresivo y rico en emojis que transmita el mismo significado e intención. Reemplaza palabras y frases clave con emojis relevantes cuando sea apropiado para agregar interés visual y emoción. Usa emojis de manera creativa pero asegúrate de que el mensaje permanezca claro y fácil de entender. No cambies el mensaje principal ni agregues nueva información.",
  messages: [
    {
      "role": "user",
      "content": [
        {
          "type": "text",
          "text": "Todo el mundo es un escenario, y todos los hombres y mujeres son meramente actores. Tienen sus salidas y sus entradas; Y un hombre en su tiempo interpreta muchos papeles."
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
    system="Tu tarea es tomar el mensaje de texto plano proporcionado y convertirlo en un mensaje expresivo y rico en emojis que transmita el mismo significado e intención. Reemplaza palabras y frases clave con emojis relevantes cuando sea apropiado para agregar interés visual y emoción. Usa emojis de manera creativa pero asegúrate de que el mensaje permanezca claro y fácil de entender. No cambies el mensaje principal ni agregues nueva información.",
    messages=[
        {
            "role": "user",
            "content": [
                {
                    "type": "text",
                    "text": "Todo el mundo es un escenario, y todos los hombres y mujeres son meramente actores. Tienen sus salidas y sus entradas; Y un hombre en su tiempo interpreta muchos papeles."
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
  system: "Tu tarea es tomar el mensaje de texto plano proporcionado y convertirlo en un mensaje expresivo y rico en emojis que transmita el mismo significado e intención. Reemplaza palabras y frases clave con emojis relevantes cuando sea apropiado para agregar interés visual y emoción. Usa emojis de manera creativa pero asegúrate de que el mensaje permanezca claro y fácil de entender. No cambies el mensaje principal ni agregues nueva información.",
  messages: [
    {
      "role": "user",
      "content": [
        {
          "type": "text",
          "text": "Todo el mundo es un escenario, y todos los hombres y mujeres son meramente actores. Tienen sus salidas y sus entradas; Y un hombre en su tiempo interpreta muchos papeles."
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
    system="Tu tarea es tomar el mensaje de texto plano proporcionado y convertirlo en un mensaje expresivo y rico en emojis que transmita el mismo significado e intención. Reemplaza palabras y frases clave con emojis relevantes cuando sea apropiado para agregar interés visual y emoción. Usa emojis de manera creativa pero asegúrate de que el mensaje permanezca claro y fácil de entender. No cambies el mensaje principal ni agregues nueva información.",
    messages=[
        {
            "role": "user",
            "content": [
                {
                    "type": "text",
                    "text": "Todo el mundo es un escenario, y todos los hombres y mujeres son meramente actores. Tienen sus salidas y sus entradas; Y un hombre en su tiempo interpreta muchos papeles."
                }
            ]
        }
    ]
)
print(message.content)

```

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
  system: "Tu tarea es tomar el mensaje de texto plano proporcionado y convertirlo en un mensaje expresivo y rico en emojis que transmita el mismo significado e intención. Reemplaza palabras y frases clave con emojis relevantes cuando sea apropiado para agregar interés visual y emoción. Usa emojis de manera creativa pero asegúrate de que el mensaje permanezca claro y fácil de entender. No cambies el mensaje principal ni agregues nueva información.",
  messages: [
    {
      "role": "user",
      "content": [
        {
          "type": "text",
          "text": "Todo el mundo es un escenario, y todos los hombres y mujeres son meramente actores. Tienen sus salidas y sus entradas; Y un hombre en su tiempo interpreta muchos papeles."
        }
      ]
    }
  ]
});
console.log(msg);

```

</Tab>
</Tabs>