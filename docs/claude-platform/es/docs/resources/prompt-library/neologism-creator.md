# Creador de neologismos

Inventa nuevas palabras y proporciona sus definiciones basándose en conceptos o ideas proporcionadas por el usuario.

---

> ¡Copia este prompt en nuestra [Consola](/dashboard) de desarrollador para probarlo tú mismo!

|      | Contenido |
| ---- | --- |
| Usuario | ¿Puedes ayudarme a crear una nueva palabra para el acto de fingir entender algo para evitar parecer ignorante o desinformado? |

## Ejemplo de salida

¿Qué tal el neologismo "fauxprensión" para describir este concepto?

Fauxprensión (fəʊprɪˈhɛnʃ(ə)n) - sustantivo  
 Definición: El acto de fingir entendimiento o comprensión de un tema, concepto o situación para evitar parecer ignorante, desinformado o fuera de lugar. Es la pretensión de conocimiento o dominio de una materia, a menudo empleada en entornos sociales o profesionales para mantener la imagen o estatus de uno.

Etimología: La palabra "fauxprensión" es una mezcla de "faux", que proviene de la palabra francesa que significa falso o fingido, y "comprensión", que significa la acción o capacidad de entender algo.

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
          "text": "Can you help me create a new word for the act of pretending to understand something in order to avoid looking ignorant or uninformed?"
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
          "text": "Can you help me create a new word for the act of pretending to understand something in order to avoid looking ignorant or uninformed?"
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
                    "text": "Can you help me create a new word for the act of pretending to understand something in order to avoid looking ignorant or uninformed?"
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
          "text": "Can you help me create a new word for the act of pretending to understand something in order to avoid looking ignorant or uninformed?"
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
                    "text": "Can you help me create a new word for the act of pretending to understand something in order to avoid looking ignorant or uninformed?"
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
          "text": "Can you help me create a new word for the act of pretending to understand something in order to avoid looking ignorant or uninformed?"
        }
      ]
    }
  ]
});
console.log(msg);

```

</Tab>
</Tabs>