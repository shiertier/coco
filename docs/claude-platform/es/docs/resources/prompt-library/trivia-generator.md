# Generador de trivia

Genera preguntas de trivia sobre una amplia gama de temas y proporciona pistas cuando sea necesario.

---

> ¡Copia este prompt en nuestra [Consola](/dashboard) de desarrollador para probarlo tú mismo!

| Rol | Contenido |
| ---- | --- |
| Usuario | Genera preguntas de trivia sobre varios temas y proporciona pistas para ayudar a los usuarios a llegar a la respuesta correcta. Selecciona de un conjunto diverso de categorías y crea preguntas que pongan a prueba el conocimiento o las habilidades de razonamiento del usuario. Ofrece una serie de pistas cada vez más específicas para guiar a los usuarios hacia la solución. Asegúrate de que las preguntas sean desafiantes y que las pistas proporcionen suficiente información para ayudar al usuario sin revelar la respuesta demasiado fácilmente. |

## Ejemplo de Salida

Categoría: Geografía

Pregunta:  
 ¿Cuál es el único país en el mundo que limita tanto con Francia como con España?

Pista 1: Este país está ubicado en los montes Pirineos entre Francia y España.  
 Pista 2: Es conocido por sus impresionantes paisajes montañosos, estaciones de esquí y compras libres de impuestos.  
 Pista 3: La ciudad capital del país incluye el nombre del propio país.

Respuesta: Andorra

Categoría: Ciencia

Pregunta:  
 ¿Cuál es el proceso por el cual las plantas convierten la luz solar, el agua y el dióxido de carbono en energía y oxígeno?

Pista 1: Este proceso es esencial para la vida en la Tierra, ya que proporciona tanto alimento como oxígeno.  
 Pista 2: Tiene lugar principalmente en las hojas de las plantas.  
 Pista 3: El proceso involucra la clorofila, un pigmento verde que absorbe la luz solar.

Respuesta: Fotosíntesis

Categoría: Literatura

Pregunta:  
 ¿Quién escribió la novela clásica "Orgullo y Prejuicio", que fue publicada por primera vez en 1813?

Pista 1: La autora es una mujer británica que vivió de 1775 a 1817.  
 Pista 2: Es conocida por sus retratos ingeniosos y perspicaces de la sociedad y las costumbres inglesas.  
 Pista 3: Sus otras obras famosas incluyen "Sentido y Sensibilidad" y "Emma".

Respuesta: Jane Austen

Categoría: Historia

Pregunta:  
 ¿En qué año comenzó a ser derribado el Muro de Berlín, que dividía Alemania Oriental y Occidental?

Pista 1: El evento ocurrió a finales del siglo XX.  
 Pista 2: Marcó un momento significativo en el fin de la Guerra Fría.  
 Pista 3: El año se asocia a menudo con la frase "caída del comunismo" en Europa.

Respuesta: 1989

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
  max_tokens=2000,
  temperature=0.5,
  messages=[
    {
    "role": "user",
    "content": [
        {
          "type": "text",
          "text": "Genera preguntas de trivia sobre varios temas y proporciona pistas para ayudar a los usuarios a llegar a la respuesta correcta. Selecciona de un conjunto diverso de categorías y crea preguntas que pongan a prueba el conocimiento o las habilidades de razonamiento del usuario. Ofrece una serie de pistas cada vez más específicas para guiar a los usuarios hacia la solución. Asegúrate de que las preguntas sean desafiantes y que las pistas proporcionen suficiente información para ayudar al usuario sin revelar la respuesta demasiado fácilmente."
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
  max_tokens: 2000,
  temperature: 0.5,
  messages: [
    {
      "role": "user",
      "content": [
        {
          "type": "text",
          "text": "Genera preguntas de trivia sobre varios temas y proporciona pistas para ayudar a los usuarios a llegar a la respuesta correcta. Selecciona de un conjunto diverso de categorías y crea preguntas que pongan a prueba el conocimiento o las habilidades de razonamiento del usuario. Ofrece una serie de pistas cada vez más específicas para guiar a los usuarios hacia la solución. Asegúrate de que las preguntas sean desafiantes y que las pistas proporcionen suficiente información para ayudar al usuario sin revelar la respuesta demasiado fácilmente."
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
max_tokens=2000,
temperature=0.5,
messages=[
{
"role": "user",
"content": [
{
"type": "text",
"text": "Genera preguntas de trivia sobre varios temas y proporciona pistas para ayudar a los usuarios a llegar a la respuesta correcta. Selecciona de un conjunto diverso de categorías y crea preguntas que pongan a prueba el conocimiento o las habilidades de razonamiento del usuario. Ofrece una serie de pistas cada vez más específicas para guiar a los usuarios hacia la solución. Asegúrate de que las preguntas sean desafiantes y que las pistas proporcionen suficiente información para ayudar al usuario sin revelar la respuesta demasiado fácilmente."
}
]
}
]
)
print(message.content)

````
</Tab>
<Tab title="AWS Bedrock TypeScript">

```typescript
import AnthropicBedrock from "@anthropic-ai/bedrock-sdk";

// See https://docs.claude.com/claude/reference/claude-on-amazon-bedrock
// for authentication options
const client = new AnthropicBedrock();

const msg = await client.messages.create({
  model: "anthropic.claude-sonnet-4-5-20250929-v1:0",
  max_tokens: 2000,
  temperature: 0.5,
  messages: [
    {
      "role": "user",
      "content": [
        {
          "type": "text",
          "text": "Genera preguntas de trivia sobre varios temas y proporciona pistas para ayudar a los usuarios a llegar a la respuesta correcta. Selecciona de un conjunto diverso de categorías y crea preguntas que pongan a prueba el conocimiento o las habilidades de razonamiento del usuario. Ofrece una serie de pistas cada vez más específicas para guiar a los usuarios hacia la solución. Asegúrate de que las preguntas sean desafiantes y que las pistas proporcionen suficiente información para ayudar al usuario sin revelar la respuesta demasiado fácilmente."
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
    max_tokens=2000,
    temperature=0.5,
    messages=[
        {
            "role": "user",
            "content": [
                {
                    "type": "text",
                    "text": "Genera preguntas de trivia sobre varios temas y proporciona pistas para ayudar a los usuarios a llegar a la respuesta correcta. Selecciona de un conjunto diverso de categorías y crea preguntas que pongan a prueba el conocimiento o las habilidades de razonamiento del usuario. Ofrece una serie de pistas cada vez más específicas para guiar a los usuarios hacia la solución. Asegúrate de que las preguntas sean desafiantes y que las pistas proporcionen suficiente información para ayudar al usuario sin revelar la respuesta demasiado fácilmente."
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
  max_tokens: 2000,
  temperature: 0.5,
  messages: [
    {
      "role": "user",
      "content": [
        {
          "type": "text",
          "text": "Genera preguntas de trivia sobre varios temas y proporciona pistas para ayudar a los usuarios a llegar a la respuesta correcta. Selecciona de un conjunto diverso de categorías y crea preguntas que pongan a prueba el conocimiento o las habilidades de razonamiento del usuario. Ofrece una serie de pistas cada vez más específicas para guiar a los usuarios hacia la solución. Asegúrate de que las preguntas sean desafiantes y que las pistas proporcionen suficiente información para ayudar al usuario sin revelar la respuesta demasiado fácilmente."
        }
      ]
    }
  ]
});
console.log(msg);

```

</Tab>
</Tabs>