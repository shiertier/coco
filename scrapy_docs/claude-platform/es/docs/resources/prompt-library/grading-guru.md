# Gurú de calificaciones

Compara y evalúa la calidad de textos escritos basándose en criterios y estándares definidos por el usuario.

---

> ¡Copia este prompt en nuestra [Consola](/dashboard) de desarrollador para probarlo tú mismo!

| Rol | Contenido |
| ---- | --- |
| Usuario | Evalúa los siguientes dos textos basándote en los criterios dados: <br/> <br/> Texto 1: <br/> El sol se estaba poniendo detrás de las montañas, proyectando un cálido resplandor dorado a través del valle. Los pájaros cantaban sus canciones vespertinas mientras la suave brisa susurraba entre las hojas. Era un momento pacífico y sereno, uno que llenaba el corazón de alegría y satisfacción. <br/><br/> Texto 2: <br/> El atardecer era bonito. Los pájaros estaban cantando y el viento estaba soplando. Me hizo feliz. <br/><br/> Criterios: <br/> 1. Lenguaje descriptivo e imágenes <br/> 2. Estructura de oraciones y variedad <br/> 3. Impacto emocional y compromiso <br/> 4. Gramática y puntuación |

## Ejemplo de Salida

Basándome en los criterios proporcionados, aquí está mi evaluación de los dos textos:

Texto 1:

1.  Lenguaje descriptivo e imágenes: El texto utiliza un lenguaje vívido y evocador para pintar una imagen del escenario. Frases como "cálido resplandor dorado", "los pájaros cantaban sus canciones vespertinas" y "la suave brisa susurraba entre las hojas" crean una fuerte imaginería visual y auditiva, sumergiendo al lector en la escena.
2.  Estructura de oraciones y variedad: El texto demuestra una buena mezcla de estructuras de oraciones, con oraciones tanto simples como compuestas. Esta variedad ayuda a mantener un flujo suave y mantiene al lector comprometido.
3.  Impacto emocional y compromiso: El texto transmite efectivamente una sensación de paz, alegría y satisfacción, invitando al lector a compartir la experiencia emocional del momento. La descripción del escenario y el uso de lenguaje positivo crean una fuerte conexión emocional.
4.  Gramática y puntuación: El texto parece ser gramaticalmente correcto y bien puntuado, sin errores obvios o problemas.

Texto 2:

1.  Lenguaje descriptivo e imágenes: El texto carece de lenguaje descriptivo detallado e imágenes. Aunque menciona el atardecer, los pájaros y el viento, no proporciona detalles vívidos o evocadores para ayudar al lector a visualizar la escena.
2.  Estructura de oraciones y variedad: El texto consiste en oraciones cortas y simples con poca variación en estructura. Esta falta de variedad puede hacer que la escritura se sienta monótona y menos atractiva.
3.  Impacto emocional y compromiso: El texto transmite una emoción positiva al declarar "Me hizo feliz", pero no proporciona una fuerte conexión emocional o compromiso debido a la falta de lenguaje descriptivo e imágenes.
4.  Gramática y puntuación: El texto parece ser gramaticalmente correcto y bien puntuado, sin errores obvios o problemas.

Análisis Comparativo:  
 Basándome en la evaluación, el Texto 1 se desempeña significativamente mejor que el Texto 2 en términos de lenguaje descriptivo, imágenes, estructura de oraciones, variedad e impacto emocional. El Texto 1 crea una experiencia vívida e inmersiva para el lector y demuestra una mayor calidad de escritura basada en los criterios dados, con su fuerte lenguaje descriptivo, estructuras de oraciones variadas y compromiso emocional efectivo. El Texto 2, aunque gramaticalmente correcto, carece de la riqueza descriptiva y el impacto emocional presente en el Texto 1.

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
    max_tokens=2000,
    temperature=0,
    messages=[
        {
            "role": "user",
            "content": [
                {
                    "type": "text",
                    "text": "Evalúa los siguientes dos textos basándote en los criterios dados: \n \nTexto 1: \nEl sol se estaba poniendo detrás de las montañas, proyectando un cálido resplandor dorado a través del valle. Los pájaros cantaban sus canciones vespertinas mientras la suave brisa susurraba entre las hojas. Era un momento pacífico y sereno, uno que llenaba el corazón de alegría y satisfacción. \n \nTexto 2: \nEl atardecer era bonito. Los pájaros estaban cantando y el viento estaba soplando. Me hizo feliz. \n \nCriterios: \n1. Lenguaje descriptivo e imágenes \n2. Estructura de oraciones y variedad \n3. Impacto emocional y compromiso \n4. Gramática y puntuación",
                }
            ],
        }
    ],
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
  max_tokens: 2000,
  temperature: 0,
  messages: [
    {
      "role": "user",
      "content": [
        {
          "type": "text",
          "text": "Evalúa los siguientes dos textos basándote en los criterios dados:  \n  \nTexto 1:  \nEl sol se estaba poniendo detrás de las montañas, proyectando un cálido resplandor dorado a través del valle. Los pájaros cantaban sus canciones vespertinas mientras la suave brisa susurraba entre las hojas. Era un momento pacífico y sereno, uno que llenaba el corazón de alegría y satisfacción.  \n  \nTexto 2:  \nEl atardecer era bonito. Los pájaros estaban cantando y el viento estaba soplando. Me hizo feliz.  \n  \nCriterios:  \n1. Lenguaje descriptivo e imágenes  \n2. Estructura de oraciones y variedad  \n3. Impacto emocional y compromiso  \n4. Gramática y puntuación"
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
    temperature=0,
    messages=[
        {
            "role": "user",
            "content": [
                {
                    "type": "text",
                    "text": "Evalúa los siguientes dos textos basándote en los criterios dados:  \n  \nTexto 1:  \nEl sol se estaba poniendo detrás de las montañas, proyectando un cálido resplandor dorado a través del valle. Los pájaros cantaban sus canciones vespertinas mientras la suave brisa susurraba entre las hojas. Era un momento pacífico y sereno, uno que llenaba el corazón de alegría y satisfacción.  \n  \nTexto 2:  \nEl atardecer era bonito. Los pájaros estaban cantando y el viento estaba soplando. Me hizo feliz.  \n  \nCriterios:  \n1. Lenguaje descriptivo e imágenes  \n2. Estructura de oraciones y variedad  \n3. Impacto emocional y compromiso  \n4. Gramática y puntuación"
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
  max_tokens: 2000,
  temperature: 0,
  messages: [
    {
      "role": "user",
      "content": [
        {
          "type": "text",
          "text": "Evalúa los siguientes dos textos basándote en los criterios dados:  \n  \nTexto 1:  \nEl sol se estaba poniendo detrás de las montañas, proyectando un cálido resplandor dorado a través del valle. Los pájaros cantaban sus canciones vespertinas mientras la suave brisa susurraba entre las hojas. Era un momento pacífico y sereno, uno que llenaba el corazón de alegría y satisfacción.  \n  \nTexto 2:  \nEl atardecer era bonito. Los pájaros estaban cantando y el viento estaba soplando. Me hizo feliz.  \n  \nCriterios:  \n1. Lenguaje descriptivo e imágenes  \n2. Estructura de oraciones y variedad  \n3. Impacto emocional y compromiso  \n4. Gramática y puntuación"
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
    max_tokens=2000,
    temperature=0,
    messages=[
        {
            "role": "user",
            "content": [
                {
                    "type": "text",
                    "text": "Evalúa los siguientes dos textos basándote en los criterios dados:  \n  \nTexto 1:  \nEl sol se estaba poniendo detrás de las montañas, proyectando un cálido resplandor dorado a través del valle. Los pájaros cantaban sus canciones vespertinas mientras la suave brisa susurraba entre las hojas. Era un momento pacífico y sereno, uno que llenaba el corazón de alegría y satisfacción.  \n  \nTexto 2:  \nEl atardecer era bonito. Los pájaros estaban cantando y el viento estaba soplando. Me hizo feliz.  \n  \nCriterios:  \n1. Lenguaje descriptivo e imágenes  \n2. Estructura de oraciones y variedad  \n3. Impacto emocional y compromiso  \n4. Gramática y puntuación"
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
  temperature: 0,
  messages: [
    {
      "role": "user",
      "content": [
        {
          "type": "text",
          "text": "Evalúa los siguientes dos textos basándote en los criterios dados:  \n  \nTexto 1:  \nEl sol se estaba poniendo detrás de las montañas, proyectando un cálido resplandor dorado a través del valle. Los pájaros cantaban sus canciones vespertinas mientras la suave brisa susurraba entre las hojas. Era un momento pacífico y sereno, uno que llenaba el corazón de alegría y satisfacción.  \n  \nTexto 2:  \nEl atardecer era bonito. Los pájaros estaban cantando y el viento estaba soplando. Me hizo feliz.  \n  \nCriterios:  \n1. Lenguaje descriptivo e imágenes  \n2. Estructura de oraciones y variedad  \n3. Impacto emocional y compromiso  \n4. Gramática y puntuación"
        }
      ]
    }
  ]
});
console.log(msg);

```

</Tab>
</Tabs>