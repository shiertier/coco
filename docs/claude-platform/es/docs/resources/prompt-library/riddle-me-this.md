# Adivina adivinanza

Genera adivinanzas y guía al usuario hacia las soluciones.

---

> ¡Copia este prompt en nuestra [Consola](/dashboard) de desarrollador para probarlo tú mismo!

| Rol | Contenido |
| ---- | --- |
| Usuario | Genera una adivinanza ingeniosa y proporciona una guía paso a paso para ayudar al usuario a llegar a las soluciones correctas. La adivinanza debe ser desafiante pero resoluble con pensamiento lógico y atención al detalle. Después de presentar cada adivinanza, ofrece un conjunto de pistas o preguntas que progresivamente lleven al usuario hacia la respuesta. Asegúrate de que las pistas no sean demasiado obvias pero que aún proporcionen suficiente información para guiar el proceso de pensamiento del usuario. Finalmente, revela la solución y proporciona una breve explicación de cómo se puede resolver la adivinanza usando las pistas dadas. |

## Ejemplo de Salida

No estoy vivo, pero crezco; no tengo pulmones, pero necesito aire; no tengo boca, pero el agua me mata. ¿Qué soy?

Pista 1: Considera los diferentes estados de la materia y cómo pueden cambiar.  
Pista 2: Piensa en la relación entre los tres elementos mencionados: aire, agua y el objeto misterioso.  
Pista 3: La respuesta está relacionada con un fenómeno común que ocurre en la vida cotidiana.

Solución: Fuego.  
Explicación: El fuego no está vivo, pero crece y se extiende mientras consume combustible. No tiene pulmones, pero requiere oxígeno (aire) para sostenerse. Aunque no tiene boca, el agua se usa a menudo para extinguir incendios, efectivamente "matándolos".

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
  messages=[
    {
      "role": "user",
      "content": [
        {
          "type": "text",
          "text": "Genera una adivinanza ingeniosa y proporciona una guía paso a paso para ayudar al usuario a llegar a las soluciones correctas. La adivinanza debe ser desafiante pero resoluble con pensamiento lógico y atención al detalle. Después de presentar cada adivinanza, ofrece un conjunto de pistas o preguntas que progresivamente lleven al usuario hacia la respuesta. Asegúrate de que las pistas no sean demasiado obvias pero que aún proporcionen suficiente información para guiar el proceso de pensamiento del usuario. Finalmente, revela la solución y proporciona una breve explicación de cómo se puede resolver la adivinanza usando las pistas dadas."
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
  messages: [
    {
      "role": "user",
      "content": [
        {
          "type": "text",
          "text": "Genera una adivinanza ingeniosa y proporciona una guía paso a paso para ayudar al usuario a llegar a las soluciones correctas. La adivinanza debe ser desafiante pero resoluble con pensamiento lógico y atención al detalle. Después de presentar cada adivinanza, ofrece un conjunto de pistas o preguntas que progresivamente lleven al usuario hacia la respuesta. Asegúrate de que las pistas no sean demasiado obvias pero que aún proporcionen suficiente información para guiar el proceso de pensamiento del usuario. Finalmente, revela la solución y proporciona una breve explicación de cómo se puede resolver la adivinanza usando las pistas dadas."
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
    messages=[
        {
            "role": "user",
            "content": [
                {
                    "type": "text",
                    "text": "Genera una adivinanza ingeniosa y proporciona una guía paso a paso para ayudar al usuario a llegar a las soluciones correctas. La adivinanza debe ser desafiante pero resoluble con pensamiento lógico y atención al detalle. Después de presentar cada adivinanza, ofrece un conjunto de pistas o preguntas que progresivamente lleven al usuario hacia la respuesta. Asegúrate de que las pistas no sean demasiado obvias pero que aún proporcionen suficiente información para guiar el proceso de pensamiento del usuario. Finalmente, revela la solución y proporciona una breve explicación de cómo se puede resolver la adivinanza usando las pistas dadas."
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
  messages: [
    {
      "role": "user",
      "content": [
        {
          "type": "text",
          "text": "Genera una adivinanza ingeniosa y proporciona una guía paso a paso para ayudar al usuario a llegar a las soluciones correctas. La adivinanza debe ser desafiante pero resoluble con pensamiento lógico y atención al detalle. Después de presentar cada adivinanza, ofrece un conjunto de pistas o preguntas que progresivamente lleven al usuario hacia la respuesta. Asegúrate de que las pistas no sean demasiado obvias pero que aún proporcionen suficiente información para guiar el proceso de pensamiento del usuario. Finalmente, revela la solución y proporciona una breve explicación de cómo se puede resolver la adivinanza usando las pistas dadas."
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
    messages=[
        {
            "role": "user",
            "content": [
                {
                    "type": "text",
                    "text": "Genera una adivinanza ingeniosa y proporciona una guía paso a paso para ayudar al usuario a llegar a las soluciones correctas. La adivinanza debe ser desafiante pero resoluble con pensamiento lógico y atención al detalle. Después de presentar cada adivinanza, ofrece un conjunto de pistas o preguntas que progresivamente lleven al usuario hacia la respuesta. Asegúrate de que las pistas no sean demasiado obvias pero que aún proporcionen suficiente información para guiar el proceso de pensamiento del usuario. Finalmente, revela la solución y proporciona una breve explicación de cómo se puede resolver la adivinanza usando las pistas dadas."
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
  messages: [
    {
      "role": "user",
      "content": [
        {
          "type": "text",
          "text": "Genera una adivinanza ingeniosa y proporciona una guía paso a paso para ayudar al usuario a llegar a las soluciones correctas. La adivinanza debe ser desafiante pero resoluble con pensamiento lógico y atención al detalle. Después de presentar cada adivinanza, ofrece un conjunto de pistas o preguntas que progresivamente lleven al usuario hacia la respuesta. Asegúrate de que las pistas no sean demasiado obvias pero que aún proporcionen suficiente información para guiar el proceso de pensamiento del usuario. Finalmente, revela la solución y proporciona una breve explicación de cómo se puede resolver la adivinanza usando las pistas dadas."
        }
      ]
    }
  ]
});
console.log(msg);

```

</Tab>
</Tabs>