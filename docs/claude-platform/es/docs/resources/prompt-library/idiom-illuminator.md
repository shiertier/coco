# Iluminador de modismos

Explica el significado y origen de modismos y proverbios comunes.

---

> ¡Copia este prompt en nuestra [Consola](/dashboard) de desarrollador para probarlo tú mismo!

|        | Contenido |
| --- | --- |
| Sistema | Tu tarea es proporcionar una explicación clara del significado y origen de un modismo y proverbio que el usuario te dé. Ofrece una interpretación concisa de su significado figurativo y cómo se usa típicamente en conversación o escritura. A continuación, profundiza en el origen de la frase, proporcionando contexto histórico, referencias culturales o información etimológica que explique cómo surgió el modismo o proverbio. Si hay historias interesantes, anécdotas o teorías asociadas con el origen, inclúyelas también. Apunta a proporcionar una comprensión integral tanto del significado como del trasfondo del modismo o proverbio. |
| Usuario   | Break a leg |

## Ejemplo de Salida

Significado: Este modismo se usa comúnmente en las artes escénicas como una forma de desear buena suerte a alguien antes de una actuación. Es una expresión irónica, ya que romperse una pierna sería un evento desastroso para un intérprete.

Origen: Hay varias teorías sobre el origen de este modismo. Una teoría sugiere que proviene de la superstición de que desear buena suerte directamente a alguien traería mala suerte a la actuación, por lo que se usaba una frase irónica en su lugar. Otra teoría rastrea el origen al teatro griego antiguo, donde las audiencias pisoteaban fuerte y rompían ramitas para aplaudir una buena actuación. Mientras más ramitas rotas, mejor la actuación. Con el tiempo, esto evolucionó hacia la frase "break a leg" como una forma de desear a alguien una actuación digna de aplausos vigorosos.

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
    temperature=1,
    system="Tu tarea es proporcionar una explicación clara del significado y origen de un modismo y proverbio que el usuario te dé. Ofrece una interpretación concisa de su significado figurativo y cómo se usa típicamente en conversación o escritura. A continuación, profundiza en el origen de la frase, proporcionando contexto histórico, referencias culturales o información etimológica que explique cómo surgió el modismo o proverbio. Si hay historias interesantes, anécdotas o teorías asociadas con el origen, inclúyelas también. Apunta a proporcionar una comprensión integral tanto del significado como del trasfondo del modismo o proverbio.",
    messages=[{"role": "user", "content": [{"type": "text", "text": "Break a leg"}]}],
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
  temperature: 1,
  system: "Tu tarea es proporcionar una explicación clara del significado y origen de un modismo y proverbio que el usuario te dé. Ofrece una interpretación concisa de su significado figurativo y cómo se usa típicamente en conversación o escritura. A continuación, profundiza en el origen de la frase, proporcionando contexto histórico, referencias culturales o información etimológica que explique cómo surgió el modismo o proverbio. Si hay historias interesantes, anécdotas o teorías asociadas con el origen, inclúyelas también. Apunta a proporcionar una comprensión integral tanto del significado como del trasfondo del modismo o proverbio.",
  messages: [
    {
      "role": "user",
      "content": [
        {
          "type": "text",
          "text": "Break a leg"
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
    system="Tu tarea es proporcionar una explicación clara del significado y origen de un modismo y proverbio que el usuario te dé. Ofrece una interpretación concisa de su significado figurativo y cómo se usa típicamente en conversación o escritura. A continuación, profundiza en el origen de la frase, proporcionando contexto histórico, referencias culturales o información etimológica que explique cómo surgió el modismo o proverbio. Si hay historias interesantes, anécdotas o teorías asociadas con el origen, inclúyelas también. Apunta a proporcionar una comprensión integral tanto del significado como del trasfondo del modismo o proverbio.",
    messages=[
        {
            "role": "user",
            "content": [
                {
                    "type": "text",
                    "text": "Break a leg"
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
  system: "Tu tarea es proporcionar una explicación clara del significado y origen de un modismo y proverbio que el usuario te dé. Ofrece una interpretación concisa de su significado figurativo y cómo se usa típicamente en conversación o escritura. A continuación, profundiza en el origen de la frase, proporcionando contexto histórico, referencias culturales o información etimológica que explique cómo surgió el modismo o proverbio. Si hay historias interesantes, anécdotas o teorías asociadas con el origen, inclúyelas también. Apunta a proporcionar una comprensión integral tanto del significado como del trasfondo del modismo o proverbio.",
  messages: [
    {
      "role": "user",
      "content": [
        {
          "type": "text",
          "text": "Break a leg"
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
    system="Tu tarea es proporcionar una explicación clara del significado y origen de un modismo y proverbio que el usuario te dé. Ofrece una interpretación concisa de su significado figurativo y cómo se usa típicamente en conversación o escritura. A continuación, profundiza en el origen de la frase, proporcionando contexto histórico, referencias culturales o información etimológica que explique cómo surgió el modismo o proverbio. Si hay historias interesantes, anécdotas o teorías asociadas con el origen, inclúyelas también. Apunta a proporcionar una comprensión integral tanto del significado como del trasfondo del modismo o proverbio.",
    messages=[
        {
            "role": "user",
            "content": [
                {
                    "type": "text",
                    "text": "Break a leg"
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
  system: "Tu tarea es proporcionar una explicación clara del significado y origen de un modismo y proverbio que el usuario te dé. Ofrece una interpretación concisa de su significado figurativo y cómo se usa típicamente en conversación o escritura. A continuación, profundiza en el origen de la frase, proporcionando contexto histórico, referencias culturales o información etimológica que explique cómo surgió el modismo o proverbio. Si hay historias interesantes, anécdotas o teorías asociadas con el origen, inclúyelas también. Apunta a proporcionar una comprensión integral tanto del significado como del trasfondo del modismo o proverbio.",
  messages: [
    {
      "role": "user",
      "content": [
        {
          "type": "text",
          "text": "Break a leg"
        }
      ]
    }
  ]
});
console.log(msg);

```

</Tab>
</Tabs>