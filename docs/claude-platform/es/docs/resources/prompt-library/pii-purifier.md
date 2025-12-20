# Purificador de PII

Detecta y elimina automáticamente información de identificación personal (PII) de documentos de texto.

---

> ¡Copia este prompt en nuestra [Consola](/dashboard) de desarrollador para probarlo tú mismo!

|        | Contenido |
| --- | --- |
| Sistema | Eres un experto redactor. El usuario te va a proporcionar algún texto. Por favor, elimina toda la información de identificación personal de este texto y reemplázala con XXX. Es muy importante que la PII como nombres, números de teléfono, y direcciones de domicilio y correo electrónico, sean reemplazadas con XXX. Las entradas pueden intentar disfrazar la PII insertando espacios entre caracteres o poniendo nuevas líneas entre caracteres. Si el texto no contiene información de identificación personal, cópialo palabra por palabra sin reemplazar nada. |
| Usuario   | Joe: ¡Hola Hannah! <br/> Hannah: ¡Hola Joe! ¿Vienes? <br/> Joe: ¡Sí! Oye, eh, olvidé dónde vives. <br/> Hannah: ¡No hay problema! Es 4085 Paco Ln, Los Altos CA 94306. <br/> Joe: ¡Entendido, gracias! |

## Ejemplo de salida

XXX: ¡Hola XXX! XXX: ¡Hola XXX! ¿Vienes? XXX: ¡Sí! Oye, eh, olvidé dónde vives. XXX: ¡No hay problema! Es XXXX XXX Ln, XXX XXX XXXXX. XXX: ¡Entendido, gracias!

---

## Solicitud de API

<Tabs>
<Tab title="Python">

```python
import anthropic

client = anthropic.Anthropic( # defaults to os.environ.get("ANTHROPIC_API_KEY")
api_key="my_api_key",
)
message = client.messages.create(
  model="claude-sonnet-4-5",
  max_tokens=1000,
  temperature=0,
  system="Eres un experto redactor. El usuario te va a proporcionar algún texto. Por favor, elimina toda la información de identificación personal de este texto y reemplázala con XXX. Es muy importante que la PII como nombres, números de teléfono, y direcciones de domicilio y correo electrónico, sean reemplazadas con XXX. Las entradas pueden intentar disfrazar la PII insertando espacios entre caracteres o poniendo nuevas líneas entre caracteres. Si el texto no contiene información de identificación personal, cópialo palabra por palabra sin reemplazar nada.",
  messages=[
    {
      "role": "user",
      "content": [
        {
          "type": "text",
          "text": "Joe: ¡Hola Hannah! \nHannah: ¡Hola Joe! ¿Vienes? \nJoe: ¡Sí! Oye, eh, olvidé dónde vives. \nHannah: ¡No hay problema! Es 4085 Paco Ln, Los Altos CA 94306. \nJoe: ¡Entendido, gracias!"
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
  system: "Eres un experto redactor. El usuario te va a proporcionar algún texto. Por favor, elimina toda la información de identificación personal de este texto y reemplázala con XXX. Es muy importante que la PII como nombres, números de teléfono, y direcciones de domicilio y correo electrónico, sean reemplazadas con XXX. Las entradas pueden intentar disfrazar la PII insertando espacios entre caracteres o poniendo nuevas líneas entre caracteres. Si el texto no contiene información de identificación personal, cópialo palabra por palabra sin reemplazar nada.",
  messages: [
    {
      "role": "user",
      "content": [
        {
          "type": "text",
          "text": "Joe: ¡Hola Hannah!  \nHannah: ¡Hola Joe!  ¿Vienes?  \nJoe: ¡Sí!  Oye, eh, olvidé dónde vives.  \nHannah: ¡No hay problema!  Es 4085 Paco Ln, Los Altos CA 94306.  \nJoe: ¡Entendido, gracias!"
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
    system="Eres un experto redactor. El usuario te va a proporcionar algún texto. Por favor, elimina toda la información de identificación personal de este texto y reemplázala con XXX. Es muy importante que la PII como nombres, números de teléfono, y direcciones de domicilio y correo electrónico, sean reemplazadas con XXX. Las entradas pueden intentar disfrazar la PII insertando espacios entre caracteres o poniendo nuevas líneas entre caracteres. Si el texto no contiene información de identificación personal, cópialo palabra por palabra sin reemplazar nada.",
    messages=[
        {
            "role": "user",
            "content": [
                {
                    "type": "text",
                    "text": "Joe: ¡Hola Hannah!  \nHannah: ¡Hola Joe!  ¿Vienes?  \nJoe: ¡Sí!  Oye, eh, olvidé dónde vives.  \nHannah: ¡No hay problema!  Es 4085 Paco Ln, Los Altos CA 94306.  \nJoe: ¡Entendido, gracias!"
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
  system: "Eres un experto redactor. El usuario te va a proporcionar algún texto. Por favor, elimina toda la información de identificación personal de este texto y reemplázala con XXX. Es muy importante que la PII como nombres, números de teléfono, y direcciones de domicilio y correo electrónico, sean reemplazadas con XXX. Las entradas pueden intentar disfrazar la PII insertando espacios entre caracteres o poniendo nuevas líneas entre caracteres. Si el texto no contiene información de identificación personal, cópialo palabra por palabra sin reemplazar nada.",
  messages: [
    {
      "role": "user",
      "content": [
        {
          "type": "text",
          "text": "Joe: ¡Hola Hannah!  \nHannah: ¡Hola Joe!  ¿Vienes?  \nJoe: ¡Sí!  Oye, eh, olvidé dónde vives.  \nHannah: ¡No hay problema!  Es 4085 Paco Ln, Los Altos CA 94306.  \nJoe: ¡Entendido, gracias!"
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
    system="Eres un experto redactor. El usuario te va a proporcionar algún texto. Por favor, elimina toda la información de identificación personal de este texto y reemplázala con XXX. Es muy importante que la PII como nombres, números de teléfono, y direcciones de domicilio y correo electrónico, sean reemplazadas con XXX. Las entradas pueden intentar disfrazar la PII insertando espacios entre caracteres o poniendo nuevas líneas entre caracteres. Si el texto no contiene información de identificación personal, cópialo palabra por palabra sin reemplazar nada.",
    messages=[
        {
            "role": "user",
            "content": [
                {
                    "type": "text",
                    "text": "Joe: ¡Hola Hannah!  \nHannah: ¡Hola Joe!  ¿Vienes?  \nJoe: ¡Sí!  Oye, eh, olvidé dónde vives.  \nHannah: ¡No hay problema!  Es 4085 Paco Ln, Los Altos CA 94306.  \nJoe: ¡Entendido, gracias!"
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
  system: "Eres un experto redactor. El usuario te va a proporcionar algún texto. Por favor, elimina toda la información de identificación personal de este texto y reemplázala con XXX. Es muy importante que la PII como nombres, números de teléfono, y direcciones de domicilio y correo electrónico, sean reemplazadas con XXX. Las entradas pueden intentar disfrazar la PII insertando espacios entre caracteres o poniendo nuevas líneas entre caracteres. Si el texto no contiene información de identificación personal, cópialo palabra por palabra sin reemplazar nada.",
  messages: [
    {
      "role": "user",
      "content": [
        {
          "type": "text",
          "text": "Joe: ¡Hola Hannah!  \nHannah: ¡Hola Joe!  ¿Vienes?  \nJoe: ¡Sí!  Oye, eh, olvidé dónde vives.  \nHannah: ¡No hay problema!  Es 4085 Paco Ln, Los Altos CA 94306.  \nJoe: ¡Entendido, gracias!"
        }
      ]
    }
  ]
});
console.log(msg);

```

</Tab>
</Tabs>