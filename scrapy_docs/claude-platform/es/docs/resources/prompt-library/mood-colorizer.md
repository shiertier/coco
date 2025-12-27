# Colorizador de estados de ánimo

Transforma descripciones de texto de estados de ánimo en códigos HEX correspondientes.

---

> ¡Copia este prompt en nuestra [Consola](/dashboard) de desarrollador para probarlo tú mismo!

|        | Contenido |
| --- | --- |
| Sistema | Tu tarea es tomar la descripción de texto proporcionada de un estado de ánimo o emoción y generar un código de color HEX que represente visualmente ese estado de ánimo. Usa principios de psicología del color y asociaciones comunes para determinar el color más apropiado para el estado de ánimo dado. Si la descripción de texto no es clara, es ambigua, o no proporciona suficiente información para determinar un color adecuado, responde con "No se puede determinar un código de color HEX para el estado de ánimo dado." |
| Usuario   | Una emoción apasionada, intensa y ardiente, llena de amor y deseo. |

## Ejemplo de Salida

`# FF1744`

Este código de color HEX representa un tono vívido y audaz de rojo. El rojo se asocia comúnmente con la pasión, intensidad, amor y deseo. El tono específico elegido aquí es un rojo ardiente y enérgico que captura el calor y fervor de la emoción descrita. Se sabe que el rojo estimula sentimientos de emoción y pasión, lo que lo convierte en una elección apropiada para representar este estado de ánimo.

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
  max_tokens=500,
  temperature=0.5,
  system="Tu tarea es tomar la descripción de texto proporcionada de un estado de ánimo o emoción y generar un código de color HEX que represente visualmente ese estado de ánimo. Usa principios de psicología del color y asociaciones comunes para determinar el color más apropiado para el estado de ánimo dado. Si la descripción de texto no es clara, es ambigua, o no proporciona suficiente información para determinar un color adecuado, responde con \"No se puede determinar un código de color HEX para el estado de ánimo dado.\"",
  messages=[
    {
    "role": "user",
    "content": [
        {
          "type": "text",
          "text": "Una emoción apasionada, intensa y ardiente, llena de amor y deseo."
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
  max_tokens: 500,
  temperature: 0.5,
  system: "Tu tarea es tomar la descripción de texto proporcionada de un estado de ánimo o emoción y generar un código de color HEX que represente visualmente ese estado de ánimo. Usa principios de psicología del color y asociaciones comunes para determinar el color más apropiado para el estado de ánimo dado. Si la descripción de texto no es clara, es ambigua, o no proporciona suficiente información para determinar un color adecuado, responde con \"No se puede determinar un código de color HEX para el estado de ánimo dado.\"",
  messages: [
    {
      "role": "user",
      "content": [
        {
          "type": "text",
          "text": "Una emoción apasionada, intensa y ardiente, llena de amor y deseo."
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
    max_tokens=500,
    temperature=0.5,
    system="Tu tarea es tomar la descripción de texto proporcionada de un estado de ánimo o emoción y generar un código de color HEX que represente visualmente ese estado de ánimo. Usa principios de psicología del color y asociaciones comunes para determinar el color más apropiado para el estado de ánimo dado. Si la descripción de texto no es clara, es ambigua, o no proporciona suficiente información para determinar un color adecuado, responde con \"No se puede determinar un código de color HEX para el estado de ánimo dado.\"",
    messages=[
        {
            "role": "user",
            "content": [
                {
                    "type": "text",
                    "text": "Una emoción apasionada, intensa y ardiente, llena de amor y deseo."
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
  max_tokens: 500,
  temperature: 0.5,
  system: "Tu tarea es tomar la descripción de texto proporcionada de un estado de ánimo o emoción y generar un código de color HEX que represente visualmente ese estado de ánimo. Usa principios de psicología del color y asociaciones comunes para determinar el color más apropiado para el estado de ánimo dado. Si la descripción de texto no es clara, es ambigua, o no proporciona suficiente información para determinar un color adecuado, responde con \"No se puede determinar un código de color HEX para el estado de ánimo dado.\"",
  messages: [
    {
      "role": "user",
      "content": [
        {
          "type": "text",
          "text": "Una emoción apasionada, intensa y ardiente, llena de amor y deseo."
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
    max_tokens=500,
    temperature=0.5,
    system="Tu tarea es tomar la descripción de texto proporcionada de un estado de ánimo o emoción y generar un código de color HEX que represente visualmente ese estado de ánimo. Usa principios de psicología del color y asociaciones comunes para determinar el color más apropiado para el estado de ánimo dado. Si la descripción de texto no es clara, es ambigua, o no proporciona suficiente información para determinar un color adecuado, responde con \"No se puede determinar un código de color HEX para el estado de ánimo dado.\"",
    messages=[
        {
            "role": "user",
            "content": [
                {
                    "type": "text",
                    "text": "Una emoción apasionada, intensa y ardiente, llena de amor y deseo."
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
  max_tokens: 500,
  temperature: 0.5,
  system: "Tu tarea es tomar la descripción de texto proporcionada de un estado de ánimo o emoción y generar un código de color HEX que represente visualmente ese estado de ánimo. Usa principios de psicología del color y asociaciones comunes para determinar el color más apropiado para el estado de ánimo dado. Si la descripción de texto no es clara, es ambigua, o no proporciona suficiente información para determinar un color adecuado, responde con \"No se puede determinar un código de color HEX para el estado de ánimo dado.\"",
  messages: [
    {
      "role": "user",
      "content": [
        {
          "type": "text",
          "text": "Una emoción apasionada, intensa y ardiente, llena de amor y deseo."
        }
      ]
    }
  ]
});
console.log(msg);

```

</Tab>
</Tabs>