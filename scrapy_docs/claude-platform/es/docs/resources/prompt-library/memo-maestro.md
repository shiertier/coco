# Maestro de memorandos

Redacta memorandos empresariales integrales basados en puntos clave.

---

> ¡Copia este prompt en nuestra [Consola](/dashboard) de desarrolladores para probarlo tú mismo!

|        | Contenido |
| --- | --- |
| Sistema | Tu tarea es redactar un memorando empresarial integral basado en los puntos clave proporcionados. El memorando debe estar escrito en un tono profesional, abordando toda la información relevante de manera clara y concisa. Utiliza el formato apropiado, como encabezados, subencabezados y viñetas, para organizar el contenido de manera efectiva. Asegúrate de que el memorando esté bien estructurado, sea coherente y fácil de entender para la audiencia objetivo. |
| Usuario   | - Anuncio de un nuevo programa de bienestar para empleados: "Fit4Success" <br/> - Objetivos del programa: promover el bienestar físico y mental, reducir el estrés y aumentar la productividad <br/> - Componentes: clases de fitness en el sitio, opciones de comidas saludables, recursos de salud mental y talleres de bienestar <br/> - Asociación con gimnasios locales y centros de bienestar para membresías con descuento <br/> - Incentivos para la participación: rastreadores de fitness, puntos de bienestar y sorteos de premios <br/> - Fecha de lanzamiento del programa: 1 de junio de 2023 <br/> - Proceso de inscripción: registro en línea a través de la intranet de la empresa <br/> - Coordinadores del programa: equipos de Recursos Humanos y Compromiso de Empleados |

## Ejemplo de Salida

Asunto: Presentamos Fit4Success - Nuestro Nuevo Programa de Bienestar para Empleados

Hola a todos,

Me complace anunciar el lanzamiento de nuestro nuevo programa de bienestar para empleados, Fit4Success, diseñado para promover el bienestar físico y mental, reducir el estrés y aumentar la productividad entre los miembros de nuestro equipo. El programa comenzará oficialmente el 1 de junio de 2023.

Objetivos del Programa:

- Fomentar la actividad física regular y las opciones de estilo de vida saludable
- Proporcionar recursos y apoyo para la salud mental y el manejo del estrés
- Fomentar un ambiente de trabajo positivo y atractivo que priorice el bienestar de los empleados
- Mejorar la productividad general y la satisfacción laboral

Componentes del Programa:

- Clases de fitness en el sitio: sesiones de yoga, Pilates y entrenamiento de intervalos de alta intensidad (HIIT)
- Opciones de comidas saludables: bocadillos y comidas nutritivas disponibles en la cafetería de la empresa
- Recursos de salud mental: acceso a servicios de consejería, aplicaciones de meditación y talleres de alivio del estrés
- Talleres de bienestar: seminarios sobre temas como nutrición, higiene del sueño y equilibrio trabajo-vida

Asociaciones y Descuentos:  
 Para apoyar aún más tu viaje de bienestar, nos hemos asociado con gimnasios locales y centros de bienestar para ofrecer membresías con descuento exclusivamente para nuestros empleados. Aprovecha estas tarifas especiales para acceder a una amplia gama de instalaciones de fitness y clases.

Incentivos para la Participación:  
 Para fomentar la participación activa en el programa Fit4Success, ofreceremos los siguientes incentivos:

- Rastreadores de fitness para todos los participantes para monitorear su progreso y establecer metas personales
- Puntos de bienestar obtenidos a través de la participación en el programa, canjeables por recompensas adicionales
- Sorteos mensuales de premios para participantes que cumplan hitos específicos de bienestar

Proceso de Inscripción:  
 Para inscribirse en el programa Fit4Success, visite nuestra intranet de la empresa y complete el formulario de registro en línea. La fecha límite para la inscripción es el 25 de mayo de 2023. Si tiene alguna pregunta o inquietud sobre el proceso de inscripción, comuníquese con nuestro equipo de Recursos Humanos.

Coordinación del Programa:  
 El programa Fit4Success será coordinado conjuntamente por nuestros equipos de Recursos Humanos y Compromiso de Empleados. Estarán disponibles para brindar orientación, apoyo y recursos durante todo tu viaje de bienestar.

Creemos firmemente que invertir en el bienestar de nuestros empleados es crucial para el éxito y crecimiento de nuestra organización. Te animamos a aprovechar esta emocionante oportunidad y unirte a nosotros para crear un lugar de trabajo más saludable, feliz y productivo.

¡Abracemos el programa Fit4Success y trabajemos juntos hacia un futuro más saludable!

Saludos cordiales,  
 \[Tu Nombre\]  
 \[Tu Título\]

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
    system="Tu tarea es redactar un memorando empresarial integral basado en los puntos clave proporcionados. El memorando debe estar escrito en un tono profesional, abordando toda la información relevante de manera clara y concisa. Utiliza el formato apropiado, como encabezados, subencabezados y viñetas, para organizar el contenido de manera efectiva. Asegúrate de que el memorando esté bien estructurado, sea coherente y fácil de entender para la audiencia objetivo.",
    messages=[
        {
            "role": "user",
            "content": [
                {
                    "type": "text",
                    "text": "- Anuncio de un nuevo programa de bienestar para empleados: \"Fit4Success\"  \n- Objetivos del programa: promover el bienestar físico y mental, reducir el estrés y aumentar la productividad  \n- Componentes: clases de fitness en el sitio, opciones de comidas saludables, recursos de salud mental y talleres de bienestar  \n- Asociación con gimnasios locales y centros de bienestar para membresías con descuento  \n- Incentivos para la participación: rastreadores de fitness, puntos de bienestar y sorteos de premios  \n- Fecha de lanzamiento del programa: 1 de junio de 2023  \n- Proceso de inscripción: registro en línea a través de la intranet de la empresa  \n- Coordinadores del programa: equipos de Recursos Humanos y Compromiso de Empleados"
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
  system: "Tu tarea es redactar un memorando empresarial integral basado en los puntos clave proporcionados. El memorando debe estar escrito en un tono profesional, abordando toda la información relevante de manera clara y concisa. Utiliza el formato apropiado, como encabezados, subencabezados y viñetas, para organizar el contenido de manera efectiva. Asegúrate de que el memorando esté bien estructurado, sea coherente y fácil de entender para la audiencia objetivo.",
  messages: [
    {
      "role": "user",
      "content": [
        {
          "type": "text",
          "text": "- Anuncio de un nuevo programa de bienestar para empleados: \"Fit4Success\"  \n- Objetivos del programa: promover el bienestar físico y mental, reducir el estrés y aumentar la productividad  \n- Componentes: clases de fitness en el sitio, opciones de comidas saludables, recursos de salud mental y talleres de bienestar  \n- Asociación con gimnasios locales y centros de bienestar para membresías con descuento  \n- Incentivos para la participación: rastreadores de fitness, puntos de bienestar y sorteos de premios  \n- Fecha de lanzamiento del programa: 1 de junio de 2023  \n- Proceso de inscripción: registro en línea a través de la intranet de la empresa  \n- Coordinadores del programa: equipos de Recursos Humanos y Compromiso de Empleados"
        }
      ]
    }
  ]
});
console.log(msg);

```

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
system="Tu tarea es redactar un memorando empresarial integral basado en los puntos clave proporcionados. El memorando debe estar escrito en un tono profesional, abordando toda la información relevante de manera clara y concisa. Utiliza el formato apropiado, como encabezados, subencabezados y viñetas, para organizar el contenido de manera efectiva. Asegúrate de que el memorando esté bien estructurado, sea coherente y fácil de entender para la audiencia objetivo.",
messages=[
{
"role": "user",
"content": [
{
"type": "text",
"text": "- Anuncio de un nuevo programa de bienestar para empleados: \"Fit4Success\" \n- Objetivos del programa: promover el bienestar físico y mental, reducir el estrés y aumentar la productividad \n- Componentes: clases de fitness en el sitio, opciones de comidas saludables, recursos de salud mental y talleres de bienestar \n- Asociación con gimnasios locales y centros de bienestar para membresías con descuento \n- Incentivos para la participación: rastreadores de fitness, puntos de bienestar y sorteos de premios \n- Fecha de lanzamiento del programa: 1 de junio de 2023 \n- Proceso de inscripción: registro en línea a través de la intranet de la empresa \n- Coordinadores del programa: equipos de Recursos Humanos y Compromiso de Empleados"
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
  system: "Tu tarea es redactar un memorando empresarial integral basado en los puntos clave proporcionados. El memorando debe estar escrito en un tono profesional, abordando toda la información relevante de manera clara y concisa. Utiliza el formato apropiado, como encabezados, subencabezados y viñetas, para organizar el contenido de manera efectiva. Asegúrate de que el memorando esté bien estructurado, sea coherente y fácil de entender para la audiencia objetivo.",
  messages: [
    {
      "role": "user",
      "content": [
        {
          "type": "text",
          "text": "- Anuncio de un nuevo programa de bienestar para empleados: \"Fit4Success\"  \n- Objetivos del programa: promover el bienestar físico y mental, reducir el estrés y aumentar la productividad  \n- Componentes: clases de fitness en el sitio, opciones de comidas saludables, recursos de salud mental y talleres de bienestar  \n- Asociación con gimnasios locales y centros de bienestar para membresías con descuento  \n- Incentivos para la participación: rastreadores de fitness, puntos de bienestar y sorteos de premios  \n- Fecha de lanzamiento del programa: 1 de junio de 2023  \n- Proceso de inscripción: registro en línea a través de la intranet de la empresa  \n- Coordinadores del programa: equipos de Recursos Humanos y Compromiso de Empleados"
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
    system="Tu tarea es redactar un memorando empresarial integral basado en los puntos clave proporcionados. El memorando debe estar escrito en un tono profesional, abordando toda la información relevante de manera clara y concisa. Utiliza el formato apropiado, como encabezados, subencabezados y viñetas, para organizar el contenido de manera efectiva. Asegúrate de que el memorando esté bien estructurado, sea coherente y fácil de entender para la audiencia objetivo.",
    messages=[
        {
            "role": "user",
            "content": [
                {
                    "type": "text",
                    "text": "- Anuncio de un nuevo programa de bienestar para empleados: \"Fit4Success\"  \n- Objetivos del programa: promover el bienestar físico y mental, reducir el estrés y aumentar la productividad  \n- Componentes: clases de fitness en el sitio, opciones de comidas saludables, recursos de salud mental y talleres de bienestar  \n- Asociación con gimnasios locales y centros de bienestar para membresías con descuento  \n- Incentivos para la participación: rastreadores de fitness, puntos de bienestar y sorteos de premios  \n- Fecha de lanzamiento del programa: 1 de junio de 2023  \n- Proceso de inscripción: registro en línea a través de la intranet de la empresa  \n- Coordinadores del programa: equipos de Recursos Humanos y Compromiso de Empleados"
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
  system: "Tu tarea es redactar un memorando empresarial integral basado en los puntos clave proporcionados. El memorando debe estar escrito en un tono profesional, abordando toda la información relevante de manera clara y concisa. Utiliza el formato apropiado, como encabezados, subencabezados y viñetas, para organizar el contenido de manera efectiva. Asegúrate de que el memorando esté bien estructurado, sea coherente y fácil de entender para la audiencia objetivo.",
  messages: [
    {
      "role": "user",
      "content": [
        {
          "type": "text",
          "text": "- Anuncio de un nuevo programa de bienestar para empleados: \"Fit4Success\"  \n- Objetivos del programa: promover el bienestar físico y mental, reducir el estrés y aumentar la productividad  \n- Componentes: clases de fitness en el sitio, opciones de comidas saludables, recursos de salud mental y talleres de bienestar  \n- Asociación con gimnasios locales y centros de bienestar para membresías con descuento  \n- Incentivos para la participación: rastreadores de fitness, puntos de bienestar y sorteos de premios  \n- Fecha de lanzamiento del programa: 1 de junio de 2023  \n- Proceso de inscripción: registro en línea a través de la intranet de la empresa  \n- Coordinadores del programa: equipos de Recursos Humanos y Compromiso de Empleados"
        }
      ]
    }
  ]
});
console.log(msg);

```

</Tab>
</Tabs>