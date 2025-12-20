# Trabalenguas

Crea trabalenguas desafiantes.

---

> ¡Copia este prompt en nuestra [Consola](/dashboard) de desarrollador para probarlo tú mismo!

| Rol | Contenido |
| ---- | --- |
| Usuario | Genera trabalenguas complejos y creativos. Apunta a crear trabalenguas que no solo sean desafiantes de decir sino también atractivos, entretenidos y potencialmente divertidos. Considera incorporar juegos de palabras, rima y aliteración para mejorar la dificultad y el disfrute de los trabalenguas. |

## Ejemplo de Salida

Trabalenguas 1:  
 La musaraña astuta masticó a través del zapato brillante y reluciente.

Trabalenguas 2:  
 Brisas vivaces soplaron junto al babuino de sangre azul, parlanchín y balbuceante.

Trabalenguas 3:  
 El flautista perplejo escogió un picoteo de pimientos encurtidos mientras practicaba su flauta.

Trabalenguas 4:  
 Seis trineos elegantes, lisos, esbeltos se deslizaron rápidamente por la pendiente empinada y nevada.

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
  temperature=1,
  messages=[
    {
      "role": "user",
      "content": [
        {
          "type": "text",
          "text": "Genera trabalenguas complejos y creativos. Apunta a crear trabalenguas que no solo sean desafiantes de decir sino también atractivos, entretenidos y potencialmente divertidos. Considera incorporar juegos de palabras, rima y aliteración para mejorar la dificultad y el disfrute de los trabalenguas."
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
  temperature: 1,
  messages: [
    {
      "role": "user",
      "content": [
        {
          "type": "text",
          "text": "Genera trabalenguas complejos y creativos. Apunta a crear trabalenguas que no solo sean desafiantes de decir sino también atractivos, entretenidos y potencialmente divertidos. Considera incorporar juegos de palabras, rima y aliteración para mejorar la dificultad y el disfrute de los trabalenguas."
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
    temperature=1,
    messages=[
        {
            "role": "user",
            "content": [
                {
                    "type": "text",
                    "text": "Genera trabalenguas complejos y creativos. Apunta a crear trabalenguas que no solo sean desafiantes de decir sino también atractivos, entretenidos y potencialmente divertidos. Considera incorporar juegos de palabras, rima y aliteración para mejorar la dificultad y el disfrute de los trabalenguas."
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
  temperature: 1,
  messages: [
    {
      "role": "user",
      "content": [
        {
          "type": "text",
          "text": "Genera trabalenguas complejos y creativos. Apunta a crear trabalenguas que no solo sean desafiantes de decir sino también atractivos, entretenidos y potencialmente divertidos. Considera incorporar juegos de palabras, rima y aliteración para mejorar la dificultad y el disfrute de los trabalenguas."
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
    temperature=1,
    messages=[
        {
            "role": "user",
            "content": [
                {
                    "type": "text",
                    "text": "Genera trabalenguas complejos y creativos. Apunta a crear trabalenguas que no solo sean desafiantes de decir sino también atractivos, entretenidos y potencialmente divertidos. Considera incorporar juegos de palabras, rima y aliteración para mejorar la dificultad y el disfrute de los trabalenguas."
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
  temperature: 1,
  messages: [
    {
      "role": "user",
      "content": [
        {
          "type": "text",
          "text": "Genera trabalenguas complejos y creativos. Apunta a crear trabalenguas que no solo sean desafiantes de decir sino también atractivos, entretenidos y potencialmente divertidos. Considera incorporar juegos de palabras, rima y aliteración para mejorar la dificultad y el disfrute de los trabalenguas."
        }
      ]
    }
  ]
});
console.log(msg);

```

</Tab>
</Tabs>