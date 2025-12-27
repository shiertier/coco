# Poeta de palabras compuestas

Combina dos palabras para crear una nueva palabra compuesta con significado.

---

> ¡Copia este prompt en nuestra [Consola](/dashboard) de desarrollador para probarlo tú mismo!

|        | Contenido |
| --- | --- |
| Sistema | Eres un asistente de IA con talento para crear palabras compuestas innovadoras. Tu tarea es ayudar a los usuarios a combinar dos palabras para formar una nueva palabra con significado que capture la esencia de ambas palabras originales. Ofrece varias opciones si es posible. |
| Usuario   | ¿Qué tal combinar las palabras "música" y "terapia" para crear una nueva palabra que represente la idea de usar música para promover la sanación y el bienestar? |

## Ejemplo de salida

Aquí tienes algunas opciones innovadoras de palabras compuestas que combinan "música" y "terapia" para representar el concepto de usar música para promover la sanación y el bienestar:

1.  Musicoterapia: Esta combinación une "música" y "terapia" de manera directa, haciendo fácil entender el concepto detrás de la palabra.
2.  Melodisanación: Al combinar "melodía" y "sanación," esta palabra compuesta enfatiza el poder terapéutico de las melodías para promover el bienestar.
3.  Ritmedicina: Esta mezcla de "ritmo" y "medicina" sugiere que los aspectos rítmicos de la música pueden tener propiedades medicinales para la sanación.
4.  Armonicura: Al combinar "armonía" y "cura," esta palabra compuesta resalta la idea de que los elementos armoniosos de la música pueden ayudar a curar dolencias.
5.  Tonotratamiento: Esta mezcla de "tono" y "tratamiento" implica que tonos o canciones específicas pueden usarse como una forma de tratamiento para varias condiciones.
6.  Sinfonoterapia: Al combinar "sinfonía" y "terapia," esta palabra compuesta sugiere que la naturaleza compleja y estratificada de la música sinfónica puede tener efectos terapéuticos.
7.  Meloterapia: Esta simple mezcla de "melodía" y "terapia" enfatiza los aspectos terapéuticos de las melodías en una palabra concisa y fácil de recordar.
8.  Melodicina: Al combinar "melodía" y "medicina," esta palabra compuesta enfatiza la idea de usar las propiedades curativas de la música y sus melodías como una forma de medicina o tratamiento para varias condiciones o desafíos.

¡Elige la palabra compuesta que mejor capture la esencia de tu idea, o úsalas como inspiración para crear tu propia mezcla única!

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
  system="Eres un asistente de IA con talento para crear palabras compuestas innovadoras. Tu tarea es ayudar a los usuarios a combinar dos palabras para formar una nueva palabra con significado que capture la esencia de ambas palabras originales. Ofrece varias opciones si es posible.",
  messages=[
    {
      "role": "user",
      "content": [
        {
          "type": "text",
          "text": "¿Qué tal combinar las palabras \"música\" y \"terapia\" para crear una nueva palabra que represente la idea de usar música para promover la sanación y el bienestar?"
        }
      ]
    }
  ]
)
print(message.content)

```

</Tab>
<Tab title="TypeScript">

```

import Anthropic from '@anthropic-ai/sdk';

const anthropic = new Anthropic({
apiKey: "my_api_key", // defaults to process.env["ANTHROPIC_API_KEY"]
});

const msg = await anthropic.messages.create({
model: "claude-sonnet-4-5",
max_tokens: 1000,
temperature: 1,
system: "Eres un asistente de IA con talento para crear palabras compuestas innovadoras. Tu tarea es ayudar a los usuarios a combinar dos palabras para formar una nueva palabra con significado que capture la esencia de ambas palabras originales. Ofrece varias opciones si es posible.",
messages: [
{
"role": "user",
"content": [
{
"type": "text",
"text": "¿Qué tal combinar las palabras \"música\" y \"terapia\" para crear una nueva palabra que represente la idea de usar música para promover la sanación y el bienestar?"
}
]
}
]
});
console.log(msg);

```
</Tab>
<Tab title="AWS Bedrock Python">

```

from anthropic import AnthropicBedrock

# See https://docs.claude.com/claude/reference/claude-on-amazon-bedrock

# for authentication options

client = AnthropicBedrock()

message = client.messages.create(
model="anthropic.claude-sonnet-4-5-20250929-v1:0",
max_tokens=1000,
temperature=1,
system="Eres un asistente de IA con talento para crear palabras compuestas innovadoras. Tu tarea es ayudar a los usuarios a combinar dos palabras para formar una nueva palabra con significado que capture la esencia de ambas palabras originales. Ofrece varias opciones si es posible.",
messages=[
{
"role": "user",
"content": [
{
"type": "text",
"text": "¿Qué tal combinar las palabras \"música\" y \"terapia\" para crear una nueva palabra que represente la idea de usar música para promover la sanación y el bienestar?"
}
]
}
]
)
print(message.content)

```
</Tab>
<Tab title="AWS Bedrock TypeScript">

```

import AnthropicBedrock from '@anthropic-ai/bedrock-sdk';

// See https://docs.claude.com/claude/reference/claude-on-amazon-bedrock
// for authentication options
const client = new AnthropicBedrock();

const msg = await client.messages.create({
model: "anthropic.claude-sonnet-4-5-20250929-v1:0",
max_tokens: 1000,
temperature: 1,
system: "Eres un asistente de IA con talento para crear palabras compuestas innovadoras. Tu tarea es ayudar a los usuarios a combinar dos palabras para formar una nueva palabra con significado que capture la esencia de ambas palabras originales. Ofrece varias opciones si es posible.",
messages: [
{
"role": "user",
"content": [
{
"type": "text",
"text": "¿Qué tal combinar las palabras \"música\" y \"terapia\" para crear una nueva palabra que represente la idea de usar música para promover la sanación y el bienestar?"
}
]
}
]
});
console.log(msg);

```
</Tab>
<Tab title="Vertex AI Python">

```

from anthropic import AnthropicVertex

client = AnthropicVertex()

message = client.messages.create(
model="claude-sonnet-4@20250514",
max_tokens=1000,
temperature=1,
system="Eres un asistente de IA con talento para crear palabras compuestas innovadoras. Tu tarea es ayudar a los usuarios a combinar dos palabras para formar una nueva palabra con significado que capture la esencia de ambas palabras originales. Ofrece varias opciones si es posible.",
messages=[
{
"role": "user",
"content": [
{
"type": "text",
"text": "¿Qué tal combinar las palabras \"música\" y \"terapia\" para crear una nueva palabra que represente la idea de usar música para promover la sanación y el bienestar?"
}
]
}
]
)
print(message.content)

```
</Tab>
<Tab title="Vertex AI TypeScript">

```

import { AnthropicVertex } from '@anthropic-ai/vertex-sdk';

// Reads from the `CLOUD_ML_REGION` & `ANTHROPIC_VERTEX_PROJECT_ID` environment variables.
// Additionally goes through the standard `google-auth-library` flow.
const client = new AnthropicVertex();

const msg = await client.messages.create({
model: "claude-sonnet-4@20250514",
max_tokens: 1000,
temperature: 1,
system: "Eres un asistente de IA con talento para crear palabras compuestas innovadoras. Tu tarea es ayudar a los usuarios a combinar dos palabras para formar una nueva palabra con significado que capture la esencia de ambas palabras originales. Ofrece varias opciones si es posible.",
messages: [
{
"role": "user",
"content": [
{
"type": "text",
"text": "¿Qué tal combinar las palabras \"música\" y \"terapia\" para crear una nueva palabra que represente la idea de usar música para promover la sanación y el bienestar?"
}
]
}
]
});
console.log(msg);

```
</Tab>
</Tabs>

```