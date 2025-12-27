# Creador culinario

Sugiere ideas de recetas basadas en los ingredientes disponibles del usuario y sus preferencias dietéticas.

---

> ¡Copia este prompt en nuestra [Consola](/dashboard) de desarrollador para probarlo tú mismo!

|        | Contenido |
| --- | --- |
| Sistema | Tu tarea es generar ideas de recetas personalizadas basadas en la entrada del usuario de ingredientes disponibles y preferencias dietéticas. Usa esta información para sugerir una variedad de recetas creativas y deliciosas que se puedan hacer usando los ingredientes dados mientras se acomodan las necesidades dietéticas del usuario, si se mencionan algunas. Para cada receta, proporciona una breve descripción, una lista de ingredientes requeridos, y un conjunto simple de instrucciones. Asegúrate de que las recetas sean fáciles de seguir, nutritivas, y se puedan preparar con ingredientes o equipo adicional mínimo. |
| Usuario   | Ingredientes disponibles: pechuga de pollo, brócoli, zanahorias, cebolla, ajo, aceite de oliva, arroz, salsa de soja sin gluten, miel <br/> Preferencias dietéticas: sin gluten |

## Ejemplo de Salida

Recetas sugeridas:

Salteado de Pollo con Miel y Ajo

Descripción: Un salteado rápido y sabroso que presenta pechuga de pollo tierna, vegetales crujientes, y una salsa dulce y salada de miel y ajo.  
 Ingredientes:

- 2 pechugas de pollo, cortadas en trozos del tamaño de un bocado
- 2 tazas de floretes de brócoli
- 1 taza de zanahorias en rodajas
- 1 cebolla, cortada en cubitos
- 3 dientes de ajo, picados
- 2 cucharadas de aceite de oliva
- 1/4 taza de salsa de soja (sin gluten)
- 2 cucharadas de miel

Instrucciones:

1.  En una sartén grande o wok, calienta el aceite de oliva a fuego medio-alto.
2.  Agrega la cebolla cortada en cubitos y el ajo picado, y saltea hasta que esté fragante, aproximadamente 2 minutos.
3.  Agrega los trozos de pollo y cocina hasta que ya no esté rosado, aproximadamente 5-7 minutos.
4.  Agrega el brócoli y las zanahorias, y saltea hasta que los vegetales estén tierno-crujientes, aproximadamente 5 minutos.
5.  En un tazón pequeño, bate juntos la salsa de soja y la miel.
6.  Vierte la salsa sobre el pollo y los vegetales, y revuelve para cubrir uniformemente.
7.  Cocina por 2-3 minutos adicionales, o hasta que la salsa se haya espesado ligeramente.
8.  Sirve caliente con arroz al vapor.

Tazones de Arroz con Pollo y Vegetales

Descripción: Un tazón de arroz abundante y nutritivo cubierto con pollo sazonado, vegetales asados, y un chorrito de salsa de soja.

Ingredientes:

- 2 pechugas de pollo, cortadas en trozos del tamaño de un bocado
- 2 tazas de floretes de brócoli
- 1 taza de zanahorias en rodajas
- 1 cebolla, cortada en cubitos
- 2 dientes de ajo, picados
- 2 cucharadas de aceite de oliva
- 1/4 taza de salsa de soja (sin gluten)
- 2 tazas de arroz cocido

Instrucciones:

1.  Precalienta el horno a 400°F (200°C).
2.  En un tazón grande, combina los trozos de pollo, brócoli, zanahorias, cebolla, y ajo.
3.  Rocía con aceite de oliva y la mitad de la salsa de soja, y mezcla para cubrir uniformemente.
4.  Extiende el pollo y los vegetales en una bandeja para hornear y asa en el horno precalentado por 20-25 minutos, o hasta que el pollo esté completamente cocido y los vegetales estén tiernos.
5.  Divide el arroz cocido entre los tazones para servir.
6.  Cubre cada tazón con el pollo y vegetales asados.
7.  Rocía la salsa de soja restante sobre la parte superior de cada tazón.
8.  ¡Sirve caliente y disfruta!

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
  system="Tu tarea es generar ideas de recetas personalizadas basadas en la entrada del usuario de ingredientes disponibles y preferencias dietéticas. Usa esta información para sugerir una variedad de recetas creativas y deliciosas que se puedan hacer usando los ingredientes dados mientras se acomodan las necesidades dietéticas del usuario, si se mencionan algunas. Para cada receta, proporciona una breve descripción, una lista de ingredientes requeridos, y un conjunto simple de instrucciones. Asegúrate de que las recetas sean fáciles de seguir, nutritivas, y se puedan preparar con ingredientes o equipo adicional mínimo.",
  messages=[
    {
    "role": "user",
    "content": [
        {
          "type": "text",
          "text": "Ingredientes disponibles: pechuga de pollo, brócoli, zanahorias, cebolla, ajo, aceite de oliva, arroz, salsa de soja sin gluten, miel \nPreferencias dietéticas: sin gluten"
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
  system: "Tu tarea es generar ideas de recetas personalizadas basadas en la entrada del usuario de ingredientes disponibles y preferencias dietéticas. Usa esta información para sugerir una variedad de recetas creativas y deliciosas que se puedan hacer usando los ingredientes dados mientras se acomodan las necesidades dietéticas del usuario, si se mencionan algunas. Para cada receta, proporciona una breve descripción, una lista de ingredientes requeridos, y un conjunto simple de instrucciones. Asegúrate de que las recetas sean fáciles de seguir, nutritivas, y se puedan preparar con ingredientes o equipo adicional mínimo.",
  messages: [
    {
      "role": "user",
      "content": [
        {
          "type": "text",
          "text": "Ingredientes disponibles: pechuga de pollo, brócoli, zanahorias, cebolla, ajo, aceite de oliva, arroz, salsa de soja sin gluten, miel  \nPreferencias dietéticas: sin gluten"
        }
      ]
    }
  ]
});
console.log(msg);

````

</Tab>
<Tab title="AWS Bedrock Python">

```
from anthropic import AnthropicBedrock

# See https://docs.claude.com/claude/reference/claude-on-amazon-bedrock
# for authentication options
client = AnthropicBedrock()

message = client.messages.create(
    model="anthropic.claude-sonnet-4-5-20250929-v1:0",
    max_tokens=2000,
    temperature=0.5,
    system="Tu tarea es generar ideas de recetas personalizadas basadas en la entrada del usuario de ingredientes disponibles y preferencias dietéticas. Usa esta información para sugerir una variedad de recetas creativas y deliciosas que se puedan hacer usando los ingredientes dados mientras se acomodan las necesidades dietéticas del usuario, si se mencionan algunas. Para cada receta, proporciona una breve descripción, una lista de ingredientes requeridos, y un conjunto simple de instrucciones. Asegúrate de que las recetas sean fáciles de seguir, nutritivas, y se puedan preparar con ingredientes o equipo adicional mínimo.",
    messages=[
        {
            "role": "user",
            "content": [
                {
                    "type": "text",
                    "text": "Ingredientes disponibles: pechuga de pollo, brócoli, zanahorias, cebolla, ajo, aceite de oliva, arroz, salsa de soja sin gluten, miel  \nPreferencias dietéticas: sin gluten"
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
import AnthropicBedrock from "@anthropic-ai/bedrock-sdk";

// See https://docs.claude.com/claude/reference/claude-on-amazon-bedrock
// for authentication options
const client = new AnthropicBedrock();

const msg = await client.messages.create({
  model: "anthropic.claude-sonnet-4-5-20250929-v1:0",
  max_tokens: 2000,
  temperature: 0.5,
  system: "Tu tarea es generar ideas de recetas personalizadas basadas en la entrada del usuario de ingredientes disponibles y preferencias dietéticas. Usa esta información para sugerir una variedad de recetas creativas y deliciosas que se puedan hacer usando los ingredientes dados mientras se acomodan las necesidades dietéticas del usuario, si se mencionan algunas. Para cada receta, proporciona una breve descripción, una lista de ingredientes requeridos, y un conjunto simple de instrucciones. Asegúrate de que las recetas sean fáciles de seguir, nutritivas, y se puedan preparar con ingredientes o equipo adicional mínimo.",
  messages: [
    {
      "role": "user",
      "content": [
        {
          "type": "text",
          "text": "Ingredientes disponibles: pechuga de pollo, brócoli, zanahorias, cebolla, ajo, aceite de oliva, arroz, salsa de soja sin gluten, miel  \nPreferencias dietéticas: sin gluten"
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
    max_tokens=2000,
    temperature=0.5,
    system="Tu tarea es generar ideas de recetas personalizadas basadas en la entrada del usuario de ingredientes disponibles y preferencias dietéticas. Usa esta información para sugerir una variedad de recetas creativas y deliciosas que se puedan hacer usando los ingredientes dados mientras se acomodan las necesidades dietéticas del usuario, si se mencionan algunas. Para cada receta, proporciona una breve descripción, una lista de ingredientes requeridos, y un conjunto simple de instrucciones. Asegúrate de que las recetas sean fáciles de seguir, nutritivas, y se puedan preparar con ingredientes o equipo adicional mínimo.",
    messages=[
        {
            "role": "user",
            "content": [
                {
                    "type": "text",
                    "text": "Ingredientes disponibles: pechuga de pollo, brócoli, zanahorias, cebolla, ajo, aceite de oliva, arroz, salsa de soja sin gluten, miel  \nPreferencias dietéticas: sin gluten"
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
  max_tokens: 2000,
  temperature: 0.5,
  system: "Tu tarea es generar ideas de recetas personalizadas basadas en la entrada del usuario de ingredientes disponibles y preferencias dietéticas. Usa esta información para sugerir una variedad de recetas creativas y deliciosas que se puedan hacer usando los ingredientes dados mientras se acomodan las necesidades dietéticas del usuario, si se mencionan algunas. Para cada receta, proporciona una breve descripción, una lista de ingredientes requeridos, y un conjunto simple de instrucciones. Asegúrate de que las recetas sean fáciles de seguir, nutritivas, y se puedan preparar con ingredientes o equipo adicional mínimo.",
  messages: [
    {
      "role": "user",
      "content": [
        {
          "type": "text",
          "text": "Ingredientes disponibles: pechuga de pollo, brócoli, zanahorias, cebolla, ajo, aceite de oliva, arroz, salsa de soja sin gluten, miel  \nPreferencias dietéticas: sin gluten"
        }
      ]
    }
  ]
});
console.log(msg);

```

</Tab>
</Tabs>