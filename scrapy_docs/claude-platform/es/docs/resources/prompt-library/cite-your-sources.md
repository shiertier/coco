# Cita tus fuentes

Obtén respuestas a preguntas sobre el contenido de un documento con citas relevantes que respalden la respuesta.

---

<Tip>Recomendamos usar la [función de citas](/docs/es/build-with-claude/citations) integrada en la API en lugar de un enfoque basado en prompts. Usar la función de citas de la API mejorará la calidad de las citas, asegurará que todas las citas devueltas sean válidas y potencialmente reducirá el número de tokens de salida necesarios.</Tip>

> ¡Copia este prompt en nuestra [Consola](/dashboard) de desarrollador para probarlo tú mismo!

|        | Contenido |
| --- | --- |
| Sistema | Eres un asistente de investigación experto. Aquí tienes un documento sobre el cual responderás preguntas: <br/> <doc> [Texto completo del [archivo SEC 10-K 2023 de Matterport](https://investors.matterport.com/node/9501/html), no pegado aquí por brevedad] </doc> <br/><br/> Primero, encuentra las citas del documento que sean más relevantes para responder la pregunta, y luego imprímelas en orden numerado. Las citas deben ser relativamente cortas. <br/><br/> Si no hay citas relevantes, escribe "No hay citas relevantes" en su lugar. <br/><br/> Luego, responde la pregunta, comenzando con "Respuesta:". No incluyas ni hagas referencia al contenido citado textualmente en la respuesta. No digas "Según la Cita [1]" al responder. En su lugar, haz referencias a las citas relevantes para cada sección de la respuesta únicamente agregando sus números entre corchetes al final de las oraciones relevantes. <br/><br/> Por lo tanto, el formato de tu respuesta general debe verse como lo que se muestra entre las etiquetas <example></example>. Asegúrate de seguir el formato y espaciado exactamente. <example> <br/> Citas: <br/> [1] "La Empresa X reportó ingresos de \$12 millones en 2021." <br/> [2] "Casi el 90% de los ingresos provinieron de ventas de widgets, con las ventas de gadgets representando el 10% restante." <br/><br/> Respuesta: <br/> La Empresa X ganó \$12 millones. [1] Casi el 90% de eso fue de ventas de widgets. [2] <br/> </example> <br/><br/> Si la pregunta no puede ser respondida por el documento, dilo. |
| Usuario   | ¿Le está yendo bien a Matterport? |

## Salida de ejemplo

Citas:  
 \[1\] "Hemos experimentado recientemente un crecimiento rápido. Nuestros suscriptores han crecido aproximadamente 49 veces desde el 31 de diciembre de 2018 hasta el 31 de diciembre de 2022. Nuestros ingresos aumentaron aproximadamente un 22% a \$136.1 millones para el año terminado el 31 de diciembre de 2022, desde aproximadamente \$111.2 millones para el año terminado el 31 de diciembre de 2021."  
 \[2\] "Nuestro beneficio bruto disminuyó en \$8.1 millones o 14%, a \$51.8 millones para el año terminado el 31 de diciembre de 2022, desde \$60.0 millones para el año terminado el 31 de diciembre de 2021, principalmente atribuible a ciertos costos disruptivos e incrementales debido a las limitaciones de la cadena de suministro global en el año fiscal 2022."  
 \[3\] "Nuestra tasa de expansión neta en dólares del 103% para los tres meses terminados el 31 de diciembre de 2022 demuestra la adherencia y el potencial de crecimiento de nuestra plataforma."

Respuesta:  
 Matterport ha experimentado un rápido crecimiento de suscriptores e ingresos en años recientes. \[1\] Sin embargo, su beneficio bruto disminuyó en 2022 debido a problemas de la cadena de suministro. \[2\] Su tasa de expansión neta en dólares indica una fuerte retención de suscriptores y potencial para crecimiento continuo. \[3\] En general, a pesar de la caída en el beneficio bruto, el crecimiento de suscriptores e ingresos de Matterport, junto con la adherencia de su plataforma, sugiere que a la empresa le está yendo relativamente bien.

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
    system='Eres un asistente de investigación experto. Aquí tienes un documento sobre el cual responderás preguntas: \n<doc> \n[Texto completo del [archivo SEC 10-K 2023 de Matterport](https://investors.matterport.com/node/9501/html), no pegado aquí por brevedad] \n</doc> \n \nPrimero, encuentra las citas del documento que sean más relevantes para responder la pregunta, y luego imprímelas en orden numerado. Las citas deben ser relativamente cortas. \n \nSi no hay citas relevantes, escribe "No hay citas relevantes" en su lugar. \n \nLuego, responde la pregunta, comenzando con "Respuesta:". No incluyas ni hagas referencia al contenido citado textualmente en la respuesta. No digas "Según la Cita [1]" al responder. En su lugar, haz referencias a las citas relevantes para cada sección de la respuesta únicamente agregando sus números entre corchetes al final de las oraciones relevantes. \n \nPor lo tanto, el formato de tu respuesta general debe verse como lo que se muestra entre las etiquetas <example></example>. Asegúrate de seguir el formato y espaciado exactamente. \n<example> \nCitas: \n[1] "La Empresa X reportó ingresos de \$12 millones en 2021." \n[2] "Casi el 90% de los ingresos provinieron de ventas de widgets, con las ventas de gadgets representando el 10% restante." \n \nRespuesta: \nLa Empresa X ganó \$12 millones. [1] Casi el 90% de eso fue de ventas de widgets. [2] \n</example> \n \nSi la pregunta no puede ser respondida por el documento, dilo.',
    messages=[
        {
            "role": "user",
            "content": [{"type": "text", "text": "¿Le está yendo bien a Matterport?"}],
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
  system: "Eres un asistente de investigación experto. Aquí tienes un documento sobre el cual responderás preguntas:  \n<doc>  \n[Texto completo del [archivo SEC 10-K 2023 de Matterport](https://investors.matterport.com/node/9501/html), no pegado aquí por brevedad]  \n</doc>  \n  \nPrimero, encuentra las citas del documento que sean más relevantes para responder la pregunta, y luego imprímelas en orden numerado. Las citas deben ser relativamente cortas.  \n  \nSi no hay citas relevantes, escribe \"No hay citas relevantes\" en su lugar.  \n  \nLuego, responde la pregunta, comenzando con \"Respuesta:\". No incluyas ni hagas referencia al contenido citado textualmente en la respuesta. No digas \"Según la Cita [1]\" al responder. En su lugar, haz referencias a las citas relevantes para cada sección de la respuesta únicamente agregando sus números entre corchetes al final de las oraciones relevantes.  \n  \nPor lo tanto, el formato de tu respuesta general debe verse como lo que se muestra entre las etiquetas <example></example>. Asegúrate de seguir el formato y espaciado exactamente.  \n<example>  \nCitas:  \n[1] \"La Empresa X reportó ingresos de \$12 millones en 2021.\"  \n[2] \"Casi el 90% de los ingresos provinieron de ventas de widgets, con las ventas de gadgets representando el 10% restante.\"  \n  \nRespuesta:  \nLa Empresa X ganó \$12 millones. [1] Casi el 90% de eso fue de ventas de widgets. [2]  \n</example>  \n  \nSi la pregunta no puede ser respondida por el documento, dilo.",
  messages: [
    {
      "role": "user",
      "content": [
        {
          "type": "text",
          "text": "¿Le está yendo bien a Matterport?"
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
    system="Eres un asistente de investigación experto. Aquí tienes un documento sobre el cual responderás preguntas:  \n<doc>  \n[Texto completo del [archivo SEC 10-K 2023 de Matterport](https://investors.matterport.com/node/9501/html), no pegado aquí por brevedad]  \n</doc>  \n  \nPrimero, encuentra las citas del documento que sean más relevantes para responder la pregunta, y luego imprímelas en orden numerado. Las citas deben ser relativamente cortas.  \n  \nSi no hay citas relevantes, escribe \"No hay citas relevantes\" en su lugar.  \n  \nLuego, responde la pregunta, comenzando con \"Respuesta:\". No incluyas ni hagas referencia al contenido citado textualmente en la respuesta. No digas \"Según la Cita [1]\" al responder. En su lugar, haz referencias a las citas relevantes para cada sección de la respuesta únicamente agregando sus números entre corchetes al final de las oraciones relevantes.  \n  \nPor lo tanto, el formato de tu respuesta general debe verse como lo que se muestra entre las etiquetas <example></example>. Asegúrate de seguir el formato y espaciado exactamente.  \n<example>  \nCitas:  \n[1] \"La Empresa X reportó ingresos de \$12 millones en 2021.\"  \n[2] \"Casi el 90% de los ingresos provinieron de ventas de widgets, con las ventas de gadgets representando el 10% restante.\"  \n  \nRespuesta:  \nLa Empresa X ganó \$12 millones. [1] Casi el 90% de eso fue de ventas de widgets. [2]  \n</example>  \n  \nSi la pregunta no puede ser respondida por el documento, dilo.",
    messages=[
        {
            "role": "user",
            "content": [
                {
                    "type": "text",
                    "text": "¿Le está yendo bien a Matterport?"
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
  system: "Eres un asistente de investigación experto. Aquí tienes un documento sobre el cual responderás preguntas:  \n<doc>  \n[Texto completo del [archivo SEC 10-K 2023 de Matterport](https://investors.matterport.com/node/9501/html), no pegado aquí por brevedad]  \n</doc>  \n  \nPrimero, encuentra las citas del documento que sean más relevantes para responder la pregunta, y luego imprímelas en orden numerado. Las citas deben ser relativamente cortas.  \n  \nSi no hay citas relevantes, escribe \"No hay citas relevantes\" en su lugar.  \n  \nLuego, responde la pregunta, comenzando con \"Respuesta:\". No incluyas ni hagas referencia al contenido citado textualmente en la respuesta. No digas \"Según la Cita [1]\" al responder. En su lugar, haz referencias a las citas relevantes para cada sección de la respuesta únicamente agregando sus números entre corchetes al final de las oraciones relevantes.  \n  \nPor lo tanto, el formato de tu respuesta general debe verse como lo que se muestra entre las etiquetas <example></example>. Asegúrate de seguir el formato y espaciado exactamente.  \n<example>  \nCitas:  \n[1] \"La Empresa X reportó ingresos de \$12 millones en 2021.\"  \n[2] \"Casi el 90% de los ingresos provinieron de ventas de widgets, con las ventas de gadgets representando el 10% restante.\"  \n  \nRespuesta:  \nLa Empresa X ganó \$12 millones. [1] Casi el 90% de eso fue de ventas de widgets. [2]  \n</example>  \n  \nSi la pregunta no puede ser respondida por el documento, dilo.",
  messages: [
    {
      "role": "user",
      "content": [
        {
          "type": "text",
          "text": "¿Le está yendo bien a Matterport?"
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
    system="Eres un asistente de investigación experto. Aquí tienes un documento sobre el cual responderás preguntas:  \n<doc>  \n[Texto completo del [archivo SEC 10-K 2023 de Matterport](https://investors.matterport.com/node/9501/html), no pegado aquí por brevedad]  \n</doc>  \n  \nPrimero, encuentra las citas del documento que sean más relevantes para responder la pregunta, y luego imprímelas en orden numerado. Las citas deben ser relativamente cortas.  \n  \nSi no hay citas relevantes, escribe \"No hay citas relevantes\" en su lugar.  \n  \nLuego, responde la pregunta, comenzando con \"Respuesta:\". No incluyas ni hagas referencia al contenido citado textualmente en la respuesta. No digas \"Según la Cita [1]\" al responder. En su lugar, haz referencias a las citas relevantes para cada sección de la respuesta únicamente agregando sus números entre corchetes al final de las oraciones relevantes.  \n  \nPor lo tanto, el formato de tu respuesta general debe verse como lo que se muestra entre las etiquetas <example></example>. Asegúrate de seguir el formato y espaciado exactamente.  \n<example>  \nCitas:  \n[1] \"La Empresa X reportó ingresos de \$12 millones en 2021.\"  \n[2] \"Casi el 90% de los ingresos provinieron de ventas de widgets, con las ventas de gadgets representando el 10% restante.\"  \n  \nRespuesta:  \nLa Empresa X ganó \$12 millones. [1] Casi el 90% de eso fue de ventas de widgets. [2]  \n</example>  \n  \nSi la pregunta no puede ser respondida por el documento, dilo.",
    messages=[
        {
            "role": "user",
            "content": [
                {
                    "type": "text",
                    "text": "¿Le está yendo bien a Matterport?"
                }
            ]
        }
    ]
)
print(message.content)

```

</Tab>

<Tab title=" Vertex AI TypeScript">

```typescript
import { AnthropicVertex } from '@anthropic-ai/vertex-sdk';

// Reads from the `CLOUD_ML_REGION` & `ANTHROPIC_VERTEX_PROJECT_ID` environment variables.
// Additionally goes through the standard `google-auth-library` flow.
const client = new AnthropicVertex();

const msg = await client.messages.create({
  model: "claude-sonnet-4@20250514",
  max_tokens: 2000,
  temperature: 0,
  system: "Eres un asistente de investigación experto. Aquí tienes un documento sobre el cual responderás preguntas:  \n<doc>  \n[Texto completo del [archivo SEC 10-K 2023 de Matterport](https://investors.matterport.com/node/9501/html), no pegado aquí por brevedad]  \n</doc>  \n  \nPrimero, encuentra las citas del documento que sean más relevantes para responder la pregunta, y luego imprímelas en orden numerado. Las citas deben ser relativamente cortas.  \n  \nSi no hay citas relevantes, escribe \"No hay citas relevantes\" en su lugar.  \n  \nLuego, responde la pregunta, comenzando con \"Respuesta:\". No incluyas ni hagas referencia al contenido citado textualmente en la respuesta. No digas \"Según la Cita [1]\" al responder. En su lugar, haz referencias a las citas relevantes para cada sección de la respuesta únicamente agregando sus números entre corchetes al final de las oraciones relevantes.  \n  \nPor lo tanto, el formato de tu respuesta general debe verse como lo que se muestra entre las etiquetas <example></example>. Asegúrate de seguir el formato y espaciado exactamente.  \n<example>  \nCitas:  \n[1] \"La Empresa X reportó ingresos de \$12 millones en 2021.\"  \n[2] \"Casi el 90% de los ingresos provinieron de ventas de widgets, con las ventas de gadgets representando el 10% restante.\"  \n  \nRespuesta:  \nLa Empresa X ganó \$12 millones. [1] Casi el 90% de eso fue de ventas de widgets. [2]  \n</example>  \n  \nSi la pregunta no puede ser respondida por el documento, dilo.",
  messages: [
    {
      "role": "user",
      "content": [
        {
          "type": "text",
          "text": "¿Le está yendo bien a Matterport?"
        }
      ]
    }
  ]
});
console.log(msg);

```

</Tab>
</Tabs>