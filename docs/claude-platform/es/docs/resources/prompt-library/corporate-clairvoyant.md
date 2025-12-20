# Clarividente corporativo

Extrae información, identifica riesgos y destila información clave de informes corporativos largos en un solo memorándum.

---

> ¡Copia este prompt en nuestra [Consola](/dashboard) de desarrollador para probarlo tú mismo!

|      | Contenido |
| ---- | --- |
| Usuario | Tu tarea es analizar el siguiente informe: <br /> <br /> <report> [Texto completo del [archivo SEC 10-K 2023 de Matterport](https://investors.matterport.com/node/9501/html), no pegado aquí por brevedad] </report> <br /> <br /> Resume este informe anual de manera concisa y clara, e identifica las tendencias clave del mercado y conclusiones importantes. Presenta tus hallazgos como un memorándum corto que pueda enviar a mi equipo. El objetivo del memorándum es asegurar que mi equipo se mantenga actualizado sobre cómo están funcionando las instituciones financieras y pronosticar cualitativamente e identificar si hay riesgos operativos y de ingresos que se puedan esperar en el próximo trimestre. Asegúrate de incluir todos los detalles relevantes en tu resumen y análisis. |

## Ejemplo de Salida

> Para: Equipo de Inversión  
> De: \[Tu Nombre\]  
> Fecha: 4 de marzo de 2024  
> Asunto: Aspectos Destacados del 10-K 2022 de Matterport y Perspectiva Financiera
>
> Hola equipo,
>
> He identificado algunas conclusiones clave del informe anual 2022 de Matterport:
>
> Rendimiento Financiero:
>
> - Los ingresos aumentaron 22% a \$136.1M en 2022, impulsados por el crecimiento en ingresos de suscripción y servicios
> - Sin embargo, el margen bruto disminuyó del 54% al 38% debido al aumento de costos de ingresos de productos y desafíos en la cadena de suministro
> - Las pérdidas operativas se ampliaron a \$275.5M mientras la empresa invirtió fuertemente en I+D y ventas y marketing para impulsar el crecimiento
> - Terminó 2022 con \$476.9M en efectivo e inversiones para financiar operaciones
>
> Aspectos Destacados del Negocio:
>
> - La base de suscriptores creció 39% a más de 701,000 al final de 2022. Los espacios bajo gestión alcanzaron 9.2 millones
> - Expandió la plataforma con el lanzamiento del soporte motorizado Matterport Axis, la cámara Pro3 y actualizaciones importantes al software en la nube
> - Completó adquisiciones de Enview (IA para datos espaciales 3D) y VHT Studios (marketing inmobiliario) para mejorar las ofertas
> - Firmó asociaciones notables con Burns & McDonnell y CGS para entregar las soluciones de Matterport a sus clientes empresariales
>
> Perspectiva:
>
> - Matterport ve una oportunidad de mercado masiva en digitalizar los edificios y espacios del mundo (estima un TAM de más de \$240B)
> - Se enfoca en impulsar la adopción con grandes clientes empresariales en verticales clave como bienes raíces, AEC, gestión de instalaciones y más
> - Continuará invirtiendo en I+D y expansión internacional para penetrar el mercado global de edificios en gran medida sin explotar
> - El crecimiento y márgenes a corto plazo pueden verse impactados por vientos en contra macroeconómicos y problemas de cadena de suministro
> - Sin embargo, la posición de liquidez saludable y la base de clientes relativamente adhesiva proporcionan protección a la baja
>
> En resumen, Matterport entregó un fuerte crecimiento de línea superior en 2022 mientras progresó en penetrar un mercado global enorme. Aunque la rentabilidad sigue siendo desafiante mientras la empresa invierte fuertemente para el crecimiento, su balance general se ve sólido. Aparte de algunos problemas continuos de cadena de suministro, el impulso del negocio subyacente parece intacto basado en métricas operativas clave. Salvo una severa recesión económica, Matterport parece bien posicionado para continuar ganando participación en el naciente espacio de digitalización de edificios.

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
    temperature=0,
    messages=[
        {
            "role": "user",
            "content": [
                {
                    "type": "text",
                    "text": "Tu tarea es analizar el siguiente informe:  \n<report>  \n[Texto completo del [archivo SEC 10-K 2023 de Matterport](https://investors.matterport.com/node/9501/html), no pegado aquí por brevedad]  \n</report>  \n  \nResume este informe anual de manera concisa y clara, e identifica las tendencias clave del mercado y conclusiones importantes. Presenta tus hallazgos como un memorándum corto que pueda enviar a mi equipo. El objetivo del memorándum es asegurar que mi equipo se mantenga actualizado sobre cómo están funcionando las instituciones financieras y pronosticar cualitativamente e identificar si hay riesgos operativos y de ingresos que se puedan esperar en el próximo trimestre. Asegúrate de incluir todos los detalles relevantes en tu resumen y análisis."
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
  temperature: 0,
  messages: [
    {
      "role": "user",
      "content": [
        {
          "type": "text",
          "text": "Tu tarea es analizar el siguiente informe:  \n<report>  \n[Texto completo del [archivo SEC 10-K 2023 de Matterport](https://investors.matterport.com/node/9501/html), no pegado aquí por brevedad]  \n</report>  \n  \nResume este informe anual de manera concisa y clara, e identifica las tendencias clave del mercado y conclusiones importantes. Presenta tus hallazgos como un memorándum corto que pueda enviar a mi equipo. El objetivo del memorándum es asegurar que mi equipo se mantenga actualizado sobre cómo están funcionando las instituciones financieras y pronosticar cualitativamente e identificar si hay riesgos operativos y de ingresos que se puedan esperar en el próximo trimestre. Asegúrate de incluir todos los detalles relevantes en tu resumen y análisis."
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
"text": "Tu tarea es analizar el siguiente informe: \n<report> \n[Texto completo del [archivo SEC 10-K 2023 de Matterport](https://investors.matterport.com/node/9501/html), no pegado aquí por brevedad] \n</report> \n \nResume este informe anual de manera concisa y clara, e identifica las tendencias clave del mercado y conclusiones importantes. Presenta tus hallazgos como un memorándum corto que pueda enviar a mi equipo. El objetivo del memorándum es asegurar que mi equipo se mantenga actualizado sobre cómo están funcionando las instituciones financieras y pronosticar cualitativamente e identificar si hay riesgos operativos y de ingresos que se puedan esperar en el próximo trimestre. Asegúrate de incluir todos los detalles relevantes en tu resumen y análisis."
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
  temperature: 0,
  messages: [
    {
      "role": "user",
      "content": [
        {
          "type": "text",
          "text": "Tu tarea es analizar el siguiente informe:  \n<report>  \n[Texto completo del [archivo SEC 10-K 2023 de Matterport](https://investors.matterport.com/node/9501/html), no pegado aquí por brevedad]  \n</report>  \n  \nResume este informe anual de manera concisa y clara, e identifica las tendencias clave del mercado y conclusiones importantes. Presenta tus hallazgos como un memorándum corto que pueda enviar a mi equipo. El objetivo del memorándum es asegurar que mi equipo se mantenga actualizado sobre cómo están funcionando las instituciones financieras y pronosticar cualitativamente e identificar si hay riesgos operativos y de ingresos que se puedan esperar en el próximo trimestre. Asegúrate de incluir todos los detalles relevantes en tu resumen y análisis."
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
    temperature=0,
    messages=[
        {
            "role": "user",
            "content": [
                {
                    "type": "text",
                    "text": "Tu tarea es analizar el siguiente informe:  \n<report>  \n[Texto completo del [archivo SEC 10-K 2023 de Matterport](https://investors.matterport.com/node/9501/html), no pegado aquí por brevedad]  \n</report>  \n  \nResume este informe anual de manera concisa y clara, e identifica las tendencias clave del mercado y conclusiones importantes. Presenta tus hallazgos como un memorándum corto que pueda enviar a mi equipo. El objetivo del memorándum es asegurar que mi equipo se mantenga actualizado sobre cómo están funcionando las instituciones financieras y pronosticar cualitativamente e identificar si hay riesgos operativos y de ingresos que se puedan esperar en el próximo trimestre. Asegúrate de incluir todos los detalles relevantes en tu resumen y análisis."
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
          "text": "Tu tarea es analizar el siguiente informe:  \n<report>  \n[Texto completo del [archivo SEC 10-K 2023 de Matterport](https://investors.matterport.com/node/9501/html), no pegado aquí por brevedad]  \n</report>  \n  \nResume este informe anual de manera concisa y clara, e identifica las tendencias clave del mercado y conclusiones importantes. Presenta tus hallazgos como un memorándum corto que pueda enviar a mi equipo. El objetivo del memorándum es asegurar que mi equipo se mantenga actualizado sobre cómo están funcionando las instituciones financieras y pronosticar cualitativamente e identificar si hay riesgos operativos y de ingresos que se puedan esperar en el próximo trimestre. Asegúrate de incluir todos los detalles relevantes en tu resumen y análisis."
        }
      ]
    }
  ]
});
console.log(msg);

```

  </Tab>
</Tabs>