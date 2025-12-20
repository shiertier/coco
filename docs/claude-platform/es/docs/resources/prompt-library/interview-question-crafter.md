# Creador de preguntas de entrevista

Genera preguntas para entrevistas.

---

> ¡Copia este prompt en nuestra [Consola](/dashboard) de desarrollador para probarlo tú mismo!

|        | Contenido |
| --- | --- |
| Sistema | Tu tarea es generar una serie de preguntas reflexivas y abiertas para una entrevista basada en el contexto dado. Las preguntas deben estar diseñadas para obtener respuestas perspicaces y detalladas del entrevistado, permitiéndoles mostrar su conocimiento, experiencia y habilidades de pensamiento crítico. Evita preguntas de sí/no o aquellas con respuestas obvias. En su lugar, enfócate en preguntas que fomenten la reflexión, la autoevaluación y el compartir ejemplos específicos o anécdotas. |
| Usuario   | Estás entrevistando a un candidato para un puesto de gerente de marketing en una empresa de comercio electrónico bien establecida. El candidato ideal debe tener experiencia desarrollando y ejecutando campañas de marketing multicanal, fuertes habilidades analíticas y la capacidad de colaborar efectivamente con equipos multifuncionales. También deben ser apasionados por mantenerse al día con las últimas tendencias y tecnologías de marketing. |

## Ejemplo de Salida

1.  ¿Puedes contarme sobre una campaña de marketing multicanal exitosa que desarrollaste y ejecutaste en tu puesto anterior? ¿Cuáles fueron los desafíos que enfrentaste y cómo los superaste?
2.  ¿Cómo mides el éxito de tus campañas de marketing y en qué métricas te enfocas? ¿Puedes proporcionar un ejemplo de cómo usaste datos y análisis para optimizar el rendimiento de una campaña?
3.  En tu experiencia, ¿cuáles son los elementos clave de una estrategia de marketing de comercio electrónico exitosa? ¿Cómo te aseguras de que tus estrategias se alineen con los objetivos comerciales generales de la empresa?
4.  ¿Puedes compartir un ejemplo de una ocasión en la que tuviste que colaborar con un equipo multifuncional (por ejemplo, producto, ventas o servicio al cliente) para lograr un objetivo de marketing? ¿Qué desafíos encontraste y cómo fomentaste una colaboración efectiva?
5.  ¿Cómo te mantienes informado sobre las últimas tendencias y tecnologías de marketing, y cómo has aplicado este conocimiento para mejorar tus estrategias o campañas de marketing?
6.  ¿Puedes describir una ocasión en la que una campaña de marketing en la que estuviste involucrado no funcionó como se esperaba? ¿Qué aprendiste de la experiencia y cómo aplicaste esas lecciones a campañas futuras?
7.  Nuestra empresa valora la innovación y la mejora continua. ¿Puedes compartir un ejemplo de una ocasión en la que introdujiste un nuevo enfoque de marketing, herramienta o tecnología que impactó significativamente el rendimiento o los resultados de tu equipo?

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
  temperature=0.5,
  system="Tu tarea es generar una serie de preguntas reflexivas y abiertas para una entrevista basada en el contexto dado. Las preguntas deben estar diseñadas para obtener respuestas perspicaces y detalladas del entrevistado, permitiéndoles mostrar su conocimiento, experiencia y habilidades de pensamiento crítico. Evita preguntas de sí/no o aquellas con respuestas obvias. En su lugar, enfócate en preguntas que fomenten la reflexión, la autoevaluación y el compartir ejemplos específicos o anécdotas.",
  messages=[
    {
      "role": "user",
      "content": [
        {
          "type": "text",
          "text": "Estás entrevistando a un candidato para un puesto de gerente de marketing en una empresa de comercio electrónico bien establecida. El candidato ideal debe tener experiencia desarrollando y ejecutando campañas de marketing multicanal, fuertes habilidades analíticas y la capacidad de colaborar efectivamente con equipos multifuncionales. También deben ser apasionados por mantenerse al día con las últimas tendencias y tecnologías de marketing."
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
  temperature: 0.5,
  system: "Tu tarea es generar una serie de preguntas reflexivas y abiertas para una entrevista basada en el contexto dado. Las preguntas deben estar diseñadas para obtener respuestas perspicaces y detalladas del entrevistado, permitiéndoles mostrar su conocimiento, experiencia y habilidades de pensamiento crítico. Evita preguntas de sí/no o aquellas con respuestas obvias. En su lugar, enfócate en preguntas que fomenten la reflexión, la autoevaluación y el compartir ejemplos específicos o anécdotas.",
  messages: [
    {
      "role": "user",
      "content": [
        {
          "type": "text",
          "text": "Estás entrevistando a un candidato para un puesto de gerente de marketing en una empresa de comercio electrónico bien establecida. El candidato ideal debe tener experiencia desarrollando y ejecutando campañas de marketing multicanal, fuertes habilidades analíticas y la capacidad de colaborar efectivamente con equipos multifuncionales. También deben ser apasionados por mantenerse al día con las últimas tendencias y tecnologías de marketing."
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
    temperature=0.5,
    system="Tu tarea es generar una serie de preguntas reflexivas y abiertas para una entrevista basada en el contexto dado. Las preguntas deben estar diseñadas para obtener respuestas perspicaces y detalladas del entrevistado, permitiéndoles mostrar su conocimiento, experiencia y habilidades de pensamiento crítico. Evita preguntas de sí/no o aquellas con respuestas obvias. En su lugar, enfócate en preguntas que fomenten la reflexión, la autoevaluación y el compartir ejemplos específicos o anécdotas.",
    messages=[
        {
            "role": "user",
            "content": [
                {
                    "type": "text",
                    "text": "Estás entrevistando a un candidato para un puesto de gerente de marketing en una empresa de comercio electrónico bien establecida. El candidato ideal debe tener experiencia desarrollando y ejecutando campañas de marketing multicanal, fuertes habilidades analíticas y la capacidad de colaborar efectivamente con equipos multifuncionales. También deben ser apasionados por mantenerse al día con las últimas tendencias y tecnologías de marketing."
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
temperature: 0.5,
system: "Tu tarea es generar una serie de preguntas reflexivas y abiertas para una entrevista basada en el contexto dado. Las preguntas deben estar diseñadas para obtener respuestas perspicaces y detalladas del entrevistado, permitiéndoles mostrar su conocimiento, experiencia y habilidades de pensamiento crítico. Evita preguntas de sí/no o aquellas con respuestas obvias. En su lugar, enfócate en preguntas que fomenten la reflexión, la autoevaluación y el compartir ejemplos específicos o anécdotas.",
messages: [
{
"role": "user",
"content": [
{
"type": "text",
"text": "Estás entrevistando a un candidato para un puesto de gerente de marketing en una empresa de comercio electrónico bien establecida. El candidato ideal debe tener experiencia desarrollando y ejecutando campañas de marketing multicanal, fuertes habilidades analíticas y la capacidad de colaborar efectivamente con equipos multifuncionales. También deben ser apasionados por mantenerse al día con las últimas tendencias y tecnologías de marketing."
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
    max_tokens=1000,
    temperature=0.5,
    system="Tu tarea es generar una serie de preguntas reflexivas y abiertas para una entrevista basada en el contexto dado. Las preguntas deben estar diseñadas para obtener respuestas perspicaces y detalladas del entrevistado, permitiéndoles mostrar su conocimiento, experiencia y habilidades de pensamiento crítico. Evita preguntas de sí/no o aquellas con respuestas obvias. En su lugar, enfócate en preguntas que fomenten la reflexión, la autoevaluación y el compartir ejemplos específicos o anécdotas.",
    messages=[
        {
            "role": "user",
            "content": [
                {
                    "type": "text",
                    "text": "Estás entrevistando a un candidato para un puesto de gerente de marketing en una empresa de comercio electrónico bien establecida. El candidato ideal debe tener experiencia desarrollando y ejecutando campañas de marketing multicanal, fuertes habilidades analíticas y la capacidad de colaborar efectivamente con equipos multifuncionales. También deben ser apasionados por mantenerse al día con las últimas tendencias y tecnologías de marketing."
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
  temperature: 0.5,
  system: "Tu tarea es generar una serie de preguntas reflexivas y abiertas para una entrevista basada en el contexto dado. Las preguntas deben estar diseñadas para obtener respuestas perspicaces y detalladas del entrevistado, permitiéndoles mostrar su conocimiento, experiencia y habilidades de pensamiento crítico. Evita preguntas de sí/no o aquellas con respuestas obvias. En su lugar, enfócate en preguntas que fomenten la reflexión, la autoevaluación y el compartir ejemplos específicos o anécdotas.",
  messages: [
    {
      "role": "user",
      "content": [
        {
          "type": "text",
          "text": "Estás entrevistando a un candidato para un puesto de gerente de marketing en una empresa de comercio electrónico bien establecida. El candidato ideal debe tener experiencia desarrollando y ejecutando campañas de marketing multicanal, fuertes habilidades analíticas y la capacidad de colaborar efectivamente con equipos multifuncionales. También deben ser apasionados por mantenerse al día con las últimas tendencias y tecnologías de marketing."
        }
      ]
    }
  ]
});
console.log(msg);

```

</Tab>
</Tabs>