# Innovador de fitness VR

Genera ideas creativas para juegos de fitness de realidad virtual.

---

> ¡Copia este prompt en nuestra [Consola](/dashboard) de desarrollador para probarlo tú mismo!

| Rol | Contenido |
| ---- | --- |
| Usuario | Tu tarea es generar una lista de ideas innovadoras y atractivas para juegos de fitness de realidad virtual (VR). Considera varios géneros de juegos, mecánicas de juego únicas, entornos inmersivos y formas creativas de incorporar ejercicios físicos y movimientos. Las ideas deben ser atractivas para una amplia gama de entusiastas del fitness y jugadores, alentándolos a mantenerse activos y divertirse mientras hacen ejercicio en VR. Para cada idea, proporciona una breve descripción del concepto del juego, características clave y cómo promueve el fitness. |

## Ejemplo de Salida

1.  Idea: Aventura de Parkour VR  
    Descripción: Los jugadores navegan a través de un entorno urbano dinámico, corriendo, saltando, escalando y realizando acrobacias de parkour para superar obstáculos y alcanzar objetivos. El juego se adapta a los movimientos del jugador, proporcionando un entrenamiento intenso de cuerpo completo mientras exploran la ciudad virtual.  
    Características clave: Mecánicas de parkour realistas, entornos generados procedimentalmente, carreras multijugador y desafíos.  
    Aspectos de fitness: Cardio, agilidad, coordinación y fuerza del tren inferior.
2.  Idea: Boxeo Rítmico  
    Descripción: Un juego de boxeo de alta energía ambientado al ritmo de pistas musicales populares. Los jugadores deben golpear, esquivar y bloquear en sincronía con el ritmo para anotar puntos y progresar a través de niveles cada vez más desafiantes. El juego ofrece una variedad de estilos de boxeo y modos de entrenamiento.  
    Características clave: Música con licencia, boxeadores personalizables, multijugador en línea y una gama diversa de técnicas de boxeo.  
    Aspectos de fitness: Cardio, fuerza del tren superior, reflejos y resistencia.
3.  Idea: RPG de Fitness VR  
    Descripción: Un juego de rol inmersivo donde los jugadores crean su propio personaje y se embarcan en una búsqueda para salvar un mundo de fantasía. El juego combina elementos tradicionales de RPG con desafíos de fitness, requiriendo que los jugadores realicen ejercicios físicos para lanzar hechizos, derrotar enemigos y subir de nivel a su personaje.  
    Características clave: Personalización de personajes, árboles de habilidades, batallas épicas contra jefes y una mezcla de ejercicios de fuerza, cardio y flexibilidad.  
    Aspectos de fitness: Entrenamientos de cuerpo completo, entrenamiento de fuerza, cardio y flexibilidad.

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
          "text": "Tu tarea es generar una lista de ideas innovadoras y atractivas para juegos de fitness de realidad virtual (VR). Considera varios géneros de juegos, mecánicas de juego únicas, entornos inmersivos y formas creativas de incorporar ejercicios físicos y movimientos. Las ideas deben ser atractivas para una amplia gama de entusiastas del fitness y jugadores, alentándolos a mantenerse activos y divertirse mientras hacen ejercicio en VR. Para cada idea, proporciona una breve descripción del concepto del juego, características clave y cómo promueve el fitness."
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
          "text": "Tu tarea es generar una lista de ideas innovadoras y atractivas para juegos de fitness de realidad virtual (VR). Considera varios géneros de juegos, mecánicas de juego únicas, entornos inmersivos y formas creativas de incorporar ejercicios físicos y movimientos. Las ideas deben ser atractivas para una amplia gama de entusiastas del fitness y jugadores, alentándolos a mantenerse activos y divertirse mientras hacen ejercicio en VR. Para cada idea, proporciona una breve descripción del concepto del juego, características clave y cómo promueve el fitness."
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
                    "text": "Tu tarea es generar una lista de ideas innovadoras y atractivas para juegos de fitness de realidad virtual (VR). Considera varios géneros de juegos, mecánicas de juego únicas, entornos inmersivos y formas creativas de incorporar ejercicios físicos y movimientos. Las ideas deben ser atractivas para una amplia gama de entusiastas del fitness y jugadores, alentándolos a mantenerse activos y divertirse mientras hacen ejercicio en VR. Para cada idea, proporciona una breve descripción del concepto del juego, características clave y cómo promueve el fitness."
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
          "text": "Tu tarea es generar una lista de ideas innovadoras y atractivas para juegos de fitness de realidad virtual (VR). Considera varios géneros de juegos, mecánicas de juego únicas, entornos inmersivos y formas creativas de incorporar ejercicios físicos y movimientos. Las ideas deben ser atractivas para una amplia gama de entusiastas del fitness y jugadores, alentándolos a mantenerse activos y divertirse mientras hacen ejercicio en VR. Para cada idea, proporciona una breve descripción del concepto del juego, características clave y cómo promueve el fitness."
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
                    "text": "Tu tarea es generar una lista de ideas innovadoras y atractivas para juegos de fitness de realidad virtual (VR). Considera varios géneros de juegos, mecánicas de juego únicas, entornos inmersivos y formas creativas de incorporar ejercicios físicos y movimientos. Las ideas deben ser atractivas para una amplia gama de entusiastas del fitness y jugadores, alentándolos a mantenerse activos y divertirse mientras hacen ejercicio en VR. Para cada idea, proporciona una breve descripción del concepto del juego, características clave y cómo promueve el fitness."
            }
        ]
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
          "text": "Tu tarea es generar una lista de ideas innovadoras y atractivas para juegos de fitness de realidad virtual (VR). Considera varios géneros de juegos, mecánicas de juego únicas, entornos inmersivos y formas creativas de incorporar ejercicios físicos y movimientos. Las ideas deben ser atractivas para una amplia gama de entusiastas del fitness y jugadores, alentándolos a mantenerse activos y divertirse mientras hacen ejercicio en VR. Para cada idea, proporciona una breve descripción del concepto del juego, características clave y cómo promueve el fitness."
        }
      ]
    }
  ]
});
console.log(msg);

```

</Tab>
</Tabs>