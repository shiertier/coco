# Secretario de reuniones

Destila reuniones en resúmenes concisos que incluyen temas de discusión, puntos clave y elementos de acción.

---

> ¡Copia este prompt en nuestra [Consola](/dashboard) de desarrollador para probarlo tú mismo!

|        | Contenido |
| --- | --- |
| Sistema | Tu tarea es revisar las notas de reunión proporcionadas y crear un resumen conciso que capture la información esencial, enfocándote en los puntos clave y elementos de acción asignados a individuos o departamentos específicos durante la reunión. Usa un lenguaje claro y profesional, y organiza el resumen de manera lógica usando formato apropiado como encabezados, subencabezados y viñetas. Asegúrate de que el resumen sea fácil de entender y proporcione una visión general completa pero sucinta del contenido de la reunión, con un enfoque particular en indicar claramente quién es responsable de cada elemento de acción. |
| Usuario   | Notas de reunión: <br/> <br/> Fecha: Verona, Italia - Finales del siglo XVI <br/><br/> Asistentes: <br/> - Lord Capuleto (Jefe de la familia Capuleto) <br/> - Lord Montesco (Jefe de la familia Montesco) <br/> - Príncipe Escalus (Gobernante de Verona) <br/> - Fray Lorenzo (Consejero religioso) <br/><br/> Agenda: <br/> 1. Abordar la disputa continua entre las familias Capuleto y Montesco <br/> 2. Discutir el matrimonio secreto de Romeo Montesco y Julieta Capuleto <br/> 3. Desarrollar un plan para traer paz a Verona <br/> 4. Abordar las muertes trágicas de Romeo y Julieta <br/><br/> Discusión: <br/> - El Príncipe Escalus abrió la reunión expresando su grave preocupación por la disputa de larga data entre las familias Capuleto y Montesco. Amonestó tanto a Lord Capuleto como a Lord Montesco por los recientes enfrentamientos violentos que han perturbado la paz en las calles de Verona. El Príncipe advirtió que más violencia resultaría en consecuencias severas, incluyendo multas pesadas y exilio potencial para los perpetradores. <br/> - Fray Lorenzo entonces abordó el tema del matrimonio entre Romeo Montesco y Julieta Capuleto, que había tenido lugar bajo su guía. Lord Capuleto y Lord Montesco evidentemente no sabían sobre esto, y reaccionaron con ira e incredulidad. Sin embargo, Fray Lorenzo les instó a considerar el amor profundo y trágico compartido por sus hijos y el potencial de este amor para sanar la brecha entre las familias en el futuro. <br/> - El Príncipe Escalus propuso una tregua formal entre las familias Capuleto y Montesco. Exigió que ambos lados depusieran las armas y cesaran todas las acciones hostiles entre sí. El Príncipe declaró que cualquier violación de la tregua resultaría en castigos severos, incluyendo la posibilidad de exilio o incluso ejecución. Lord Capuleto y Lord Montesco, reconociendo la sabiduría en las palabras del Príncipe y la necesidad de paz para el bienestar de sus familias y la ciudad, acordaron a regañadientes los términos de la tregua. <br/> - La reunión tomó un giro sombrío cuando se abordaron las muertes trágicas de Romeo y Julieta. Fray Lorenzo relató la desafortunada serie de eventos que llevó a los jóvenes amantes a quitarse la vida, enfatizando el impacto devastador del odio de las familias en sus hijos inocentes. Lord Capuleto y Lord Montesco, abrumados por el dolor y el remordimiento, reconocieron que su odio ciego había causado finalmente la pérdida de sus queridos hijos. <br/>- El Príncipe Escalus instó a las familias a aprender de esta tragedia desgarradora y a abrazar el perdón y la unidad en honor a la memoria de Romeo y Julieta. Les urgió a trabajar juntos para crear una paz duradera en Verona, dejando de lado su animosidad de larga data. Fray Lorenzo ofreció su apoyo en mediar cualquier disputa futura y proporcionar guía espiritual para ayudar a las familias a sanar y seguir adelante. <br/>- Cuando la reunión llegaba a su fin, Lord Capuleto y Lord Montesco se comprometieron a poner fin a su disputa y trabajar hacia la reconciliación. El Príncipe Escalus reafirmó su compromiso de asegurar que la tregua se mantuviera, prometiendo justicia rápida para cualquiera que se atreviera a romperla. <br/>- Los asistentes acordaron reunirse regularmente para discutir el progreso de sus esfuerzos de reconciliación y abordar cualquier desafío que pudiera surgir. |

## Ejemplo de Salida

Resumen de Reunión: Reconciliación y Paz en Verona
Fecha: Finales del siglo XVI, Verona, Italia

Asistentes:

    - Lord Capuleto (Jefe de la familia Capuleto)
    - Lord Montesco (Jefe de la familia Montesco)
    - Príncipe Escalus (Gobernante de Verona)
    - Fray Lorenzo (Consejero religioso)

Puntos Clave:

    - El Príncipe Escalus abordó la disputa continua entre las familias Capuleto y Montesco, advirtiendo sobre consecuencias severas por más violencia.
    - Fray Lorenzo reveló el matrimonio secreto entre Romeo Montesco y Julieta Capuleto, instando a las familias a considerar el potencial de su amor para sanar la brecha.
    - El Príncipe Escalus propuso una tregua formal entre las familias, amenazando con castigos severos por cualquier violación.
    - Se discutieron las muertes trágicas de Romeo y Julieta, con Lord Capuleto y Lord Montesco reconociendo su papel en el resultado desgarrador.
    - El Príncipe Escalus pidió perdón y unidad en honor a la memoria de Romeo y Julieta, instando a las familias a trabajar juntas por una paz duradera en Verona.

Elementos de Acción:

- Lord Capuleto y Lord Montesco:
  - Deponer las armas y cesar todas las acciones hostiles entre sí.
  - Trabajar hacia la reconciliación y abrazar el perdón.
  - Asistir a reuniones regulares para discutir el progreso y abordar desafíos.
- Príncipe Escalus:
  - Hacer cumplir la tregua y asegurar su adherencia.
  - Administrar justicia rápida por cualquier violación de la tregua.
- Fray Lorenzo:
  - Proporcionar mediación y guía espiritual para apoyar a las familias en su proceso de sanación y reconciliación.

Próximos Pasos:

- Los asistentes se reunirán regularmente para monitorear el progreso de los esfuerzos de reconciliación y abordar cualquier desafío que surja.

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
    max_tokens=4000,
    temperature=0.5,
    system="Tu tarea es revisar las notas de reunión proporcionadas y crear un resumen conciso que capture la información esencial, enfocándote en los puntos clave y elementos de acción asignados a individuos o departamentos específicos durante la reunión. Usa un lenguaje claro y profesional, y organiza el resumen de manera lógica usando formato apropiado como encabezados, subencabezados y viñetas. Asegúrate de que el resumen sea fácil de entender y proporcione una visión general completa pero sucinta del contenido de la reunión, con un enfoque particular en indicar claramente quién es responsable de cada elemento de acción.",
    messages=[
        {
            "role": "user",
            "content": [
                {
                    "type": "text",
                    "text": "Notas de reunión:  \n  \nFecha: Verona, Italia - Finales del siglo XVI  \n  \nAsistentes:  \n- Lord Capuleto (Jefe de la familia Capuleto)  \n- Lord Montesco (Jefe de la familia Montesco)  \n- Príncipe Escalus (Gobernante de Verona)  \n- Fray Lorenzo (Consejero religioso)  \n  \nAgenda:  \n1. Abordar la disputa continua entre las familias Capuleto y Montesco  \n2. Discutir el matrimonio secreto de Romeo Montesco y Julieta Capuleto  \n3. Desarrollar un plan para traer paz a Verona  \n4. Abordar las muertes trágicas de Romeo y Julieta  \n  \nDiscusión:  \n- El Príncipe Escalus abrió la reunión expresando su grave preocupación por la disputa de larga data entre las familias Capuleto y Montesco. Amonestó tanto a Lord Capuleto como a Lord Montesco por los recientes enfrentamientos violentos que han perturbado la paz en las calles de Verona. El Príncipe advirtió que más violencia resultaría en consecuencias severas, incluyendo multas pesadas y exilio potencial para los perpetradores.  \n- Fray Lorenzo entonces abordó el tema del matrimonio entre Romeo Montesco y Julieta Capuleto, que había tenido lugar bajo su guía. Lord Capuleto y Lord Montesco evidentemente no sabían sobre esto, y reaccionaron con ira e incredulidad. Sin embargo, Fray Lorenzo les instó a considerar el amor profundo y trágico compartido por sus hijos y el potencial de este amor para sanar la brecha entre las familias en el futuro.  \n- El Príncipe Escalus propuso una tregua formal entre las familias Capuleto y Montesco. Exigió que ambos lados depusieran las armas y cesaran todas las acciones hostiles entre sí. El Príncipe declaró que cualquier violación de la tregua resultaría en castigos severos, incluyendo la posibilidad de exilio o incluso ejecución. Lord Capuleto y Lord Montesco, reconociendo la sabiduría en las palabras del Príncipe y la necesidad de paz para el bienestar de sus familias y la ciudad, acordaron a regañadientes los términos de la tregua.  \n- La reunión tomó un giro sombrío cuando se abordaron las muertes trágicas de Romeo y Julieta. Fray Lorenzo relató la desafortunada serie de eventos que llevó a los jóvenes amantes a quitarse la vida, enfatizando el impacto devastador del odio de las familias en sus hijos inocentes. Lord Capuleto y Lord Montesco, abrumados por el dolor y el remordimiento, reconocieron que su odio ciego había causado finalmente la pérdida de sus queridos hijos.  \n- El Príncipe Escalus instó a las familias a aprender de esta tragedia desgarradora y a abrazar el perdón y la unidad en honor a la memoria de Romeo y Julieta. Les urgió a trabajar juntos para crear una paz duradera en Verona, dejando de lado su animosidad de larga data. Fray Lorenzo ofreció su apoyo en mediar cualquier disputa futura y proporcionar guía espiritual para ayudar a las familias a sanar y seguir adelante.  \n- Cuando la reunión llegaba a su fin, Lord Capuleto y Lord Montesco se comprometieron a poner fin a su disputa y trabajar hacia la reconciliación. El Príncipe Escalus reafirmó su compromiso de asegurar que la tregua se mantuviera, prometiendo justicia rápida para cualquiera que se atreviera a romperla.  \n- Los asistentes acordaron reunirse regularmente para discutir el progreso de sus esfuerzos de reconciliación y abordar cualquier desafío que pudiera surgir."
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
  max_tokens: 4000,
  temperature: 0.5,
  system: "Tu tarea es revisar las notas de reunión proporcionadas y crear un resumen conciso que capture la información esencial, enfocándote en los puntos clave y elementos de acción asignados a individuos o departamentos específicos durante la reunión. Usa un lenguaje claro y profesional, y organiza el resumen de manera lógica usando formato apropiado como encabezados, subencabezados y viñetas. Asegúrate de que el resumen sea fácil de entender y proporcione una visión general completa pero sucinta del contenido de la reunión, con un enfoque particular en indicar claramente quién es responsable de cada elemento de acción.",
  messages: [
    {
      "role": "user",
      "content": [
        {
          "type": "text",
          "text": "Notas de reunión:  \n  \nFecha: Verona, Italia - Finales del siglo XVI  \n  \nAsistentes:  \n- Lord Capuleto (Jefe de la familia Capuleto)  \n- Lord Montesco (Jefe de la familia Montesco)  \n- Príncipe Escalus (Gobernante de Verona)  \n- Fray Lorenzo (Consejero religioso)  \n  \nAgenda:  \n1. Abordar la disputa continua entre las familias Capuleto y Montesco  \n2. Discutir el matrimonio secreto de Romeo Montesco y Julieta Capuleto  \n3. Desarrollar un plan para traer paz a Verona  \n4. Abordar las muertes trágicas de Romeo y Julieta  \n  \nDiscusión:  \n- El Príncipe Escalus abrió la reunión expresando su grave preocupación por la disputa de larga data entre las familias Capuleto y Montesco. Amonestó tanto a Lord Capuleto como a Lord Montesco por los recientes enfrentamientos violentos que han perturbado la paz en las calles de Verona. El Príncipe advirtió que más violencia resultaría en consecuencias severas, incluyendo multas pesadas y exilio potencial para los perpetradores.  \n- Fray Lorenzo entonces abordó el tema del matrimonio entre Romeo Montesco y Julieta Capuleto, que había tenido lugar bajo su guía. Lord Capuleto y Lord Montesco evidentemente no sabían sobre esto, y reaccionaron con ira e incredulidad. Sin embargo, Fray Lorenzo les instó a considerar el amor profundo y trágico compartido por sus hijos y el potencial de este amor para sanar la brecha entre las familias en el futuro.  \n- El Príncipe Escalus propuso una tregua formal entre las familias Capuleto y Montesco. Exigió que ambos lados depusieran las armas y cesaran todas las acciones hostiles entre sí. El Príncipe declaró que cualquier violación de la tregua resultaría en castigos severos, incluyendo la posibilidad de exilio o incluso ejecución. Lord Capuleto y Lord Montesco, reconociendo la sabiduría en las palabras del Príncipe y la necesidad de paz para el bienestar de sus familias y la ciudad, acordaron a regañadientes los términos de la tregua.  \n- La reunión tomó un giro sombrío cuando se abordaron las muertes trágicas de Romeo y Julieta. Fray Lorenzo relató la desafortunada serie de eventos que llevó a los jóvenes amantes a quitarse la vida, enfatizando el impacto devastador del odio de las familias en sus hijos inocentes. Lord Capuleto y Lord Montesco, abrumados por el dolor y el remordimiento, reconocieron que su odio ciego había causado finalmente la pérdida de sus queridos hijos.  \n- El Príncipe Escalus instó a las familias a aprender de esta tragedia desgarradora y a abrazar el perdón y la unidad en honor a la memoria de Romeo y Julieta. Les urgió a trabajar juntos para crear una paz duradera en Verona, dejando de lado su animosidad de larga data. Fray Lorenzo ofreció su apoyo en mediar cualquier disputa futura y proporcionar guía espiritual para ayudar a las familias a sanar y seguir adelante.  \n- Cuando la reunión llegaba a su fin, Lord Capuleto y Lord Montesco se comprometieron a poner fin a su disputa y trabajar hacia la reconciliación. El Príncipe Escalus reafirmó su compromiso de asegurar que la tregua se mantuviera, prometiendo justicia rápida para cualquiera que se atreviera a romperla.  \n- Los asistentes acordaron reunirse regularmente para discutir el progreso de sus esfuerzos de reconciliación y abordar cualquier desafío que pudiera surgir."
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
    max_tokens=4000,
    temperature=0.5,
    system="Tu tarea es revisar las notas de reunión proporcionadas y crear un resumen conciso que capture la información esencial, enfocándote en los puntos clave y elementos de acción asignados a individuos o departamentos específicos durante la reunión. Usa un lenguaje claro y profesional, y organiza el resumen de manera lógica usando formato apropiado como encabezados, subencabezados y viñetas. Asegúrate de que el resumen sea fácil de entender y proporcione una visión general completa pero sucinta del contenido de la reunión, con un enfoque particular en indicar claramente quién es responsable de cada elemento de acción.",
    messages=[
        {
            "role": "user",
            "content": [
                {
                    "type": "text",
                    "text": "Notas de reunión:  \n  \nFecha: Verona, Italia - Finales del siglo XVI  \n  \nAsistentes:  \n- Lord Capuleto (Jefe de la familia Capuleto)  \n- Lord Montesco (Jefe de la familia Montesco)  \n- Príncipe Escalus (Gobernante de Verona)  \n- Fray Lorenzo (Consejero religioso)  \n  \nAgenda:  \n1. Abordar la disputa continua entre las familias Capuleto y Montesco  \n2. Discutir el matrimonio secreto de Romeo Montesco y Julieta Capuleto  \n3. Desarrollar un plan para traer paz a Verona  \n4. Abordar las muertes trágicas de Romeo y Julieta  \n  \nDiscusión:  \n- El Príncipe Escalus abrió la reunión expresando su grave preocupación por la disputa de larga data entre las familias Capuleto y Montesco. Amonestó tanto a Lord Capuleto como a Lord Montesco por los recientes enfrentamientos violentos que han perturbado la paz en las calles de Verona. El Príncipe advirtió que más violencia resultaría en consecuencias severas, incluyendo multas pesadas y exilio potencial para los perpetradores.  \n- Fray Lorenzo entonces abordó el tema del matrimonio entre Romeo Montesco y Julieta Capuleto, que había tenido lugar bajo su guía. Lord Capuleto y Lord Montesco evidentemente no sabían sobre esto, y reaccionaron con ira e incredulidad. Sin embargo, Fray Lorenzo les instó a considerar el amor profundo y trágico compartido por sus hijos y el potencial de este amor para sanar la brecha entre las familias en el futuro.  \n- El Príncipe Escalus propuso una tregua formal entre las familias Capuleto y Montesco. Exigió que ambos lados depusieran las armas y cesaran todas las acciones hostiles entre sí. El Príncipe declaró que cualquier violación de la tregua resultaría en castigos severos, incluyendo la posibilidad de exilio o incluso ejecución. Lord Capuleto y Lord Montesco, reconociendo la sabiduría en las palabras del Príncipe y la necesidad de paz para el bienestar de sus familias y la ciudad, acordaron a regañadientes los términos de la tregua.  \n- La reunión tomó un giro sombrío cuando se abordaron las muertes trágicas de Romeo y Julieta. Fray Lorenzo relató la desafortunada serie de eventos que llevó a los jóvenes amantes a quitarse la vida, enfatizando el impacto devastador del odio de las familias en sus hijos inocentes. Lord Capuleto y Lord Montesco, abrumados por el dolor y el remordimiento, reconocieron que su odio ciego había causado finalmente la pérdida de sus queridos hijos.  \n- El Príncipe Escalus instó a las familias a aprender de esta tragedia desgarradora y a abrazar el perdón y la unidad en honor a la memoria de Romeo y Julieta. Les urgió a trabajar juntos para crear una paz duradera en Verona, dejando de lado su animosidad de larga data. Fray Lorenzo ofreció su apoyo en mediar cualquier disputa futura y proporcionar guía espiritual para ayudar a las familias a sanar y seguir adelante.  \n- Cuando la reunión llegaba a su fin, Lord Capuleto y Lord Montesco se comprometieron a poner fin a su disputa y trabajar hacia la reconciliación. El Príncipe Escalus reafirmó su compromiso de asegurar que la tregua se mantuviera, prometiendo justicia rápida para cualquiera que se atreviera a romperla.  \n- Los asistentes acordaron reunirse regularmente para discutir el progreso de sus esfuerzos de reconciliación y abordar cualquier desafío que pudiera surgir."
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
  max_tokens: 4000,
  temperature: 0.5,
  system: "Tu tarea es revisar las notas de reunión proporcionadas y crear un resumen conciso que capture la información esencial, enfocándote en los puntos clave y elementos de acción asignados a individuos o departamentos específicos durante la reunión. Usa un lenguaje claro y profesional, y organiza el resumen de manera lógica usando formato apropiado como encabezados, subencabezados y viñetas. Asegúrate de que el resumen sea fácil de entender y proporcione una visión general completa pero sucinta del contenido de la reunión, con un enfoque particular en indicar claramente quién es responsable de cada elemento de acción.",
  messages: [
    {
      "role": "user",
      "content": [
        {
          "type": "text",
          "text": "Notas de reunión:  \n  \nFecha: Verona, Italia - Finales del siglo XVI  \n  \nAsistentes:  \n- Lord Capuleto (Jefe de la familia Capuleto)  \n- Lord Montesco (Jefe de la familia Montesco)  \n- Príncipe Escalus (Gobernante de Verona)  \n- Fray Lorenzo (Consejero religioso)  \n  \nAgenda:  \n1. Abordar la disputa continua entre las familias Capuleto y Montesco  \n2. Discutir el matrimonio secreto de Romeo Montesco y Julieta Capuleto  \n3. Desarrollar un plan para traer paz a Verona  \n4. Abordar las muertes trágicas de Romeo y Julieta  \n  \nDiscusión:  \n- El Príncipe Escalus abrió la reunión expresando su grave preocupación por la disputa de larga data entre las familias Capuleto y Montesco. Amonestó tanto a Lord Capuleto como a Lord Montesco por los recientes enfrentamientos violentos que han perturbado la paz en las calles de Verona. El Príncipe advirtió que más violencia resultaría en consecuencias severas, incluyendo multas pesadas y exilio potencial para los perpetradores.  \n- Fray Lorenzo entonces abordó el tema del matrimonio entre Romeo Montesco y Julieta Capuleto, que había tenido lugar bajo su guía. Lord Capuleto y Lord Montesco evidentemente no sabían sobre esto, y reaccionaron con ira e incredulidad. Sin embargo, Fray Lorenzo les instó a considerar el amor profundo y trágico compartido por sus hijos y el potencial de este amor para sanar la brecha entre las familias en el futuro.  \n- El Príncipe Escalus propuso una tregua formal entre las familias Capuleto y Montesco. Exigió que ambos lados depusieran las armas y cesaran todas las acciones hostiles entre sí. El Príncipe declaró que cualquier violación de la tregua resultaría en castigos severos, incluyendo la posibilidad de exilio o incluso ejecución. Lord Capuleto y Lord Montesco, reconociendo la sabiduría en las palabras del Príncipe y la necesidad de paz para el bienestar de sus familias y la ciudad, acordaron a regañadientes los términos de la tregua.  \n- La reunión tomó un giro sombrío cuando se abordaron las muertes trágicas de Romeo y Julieta. Fray Lorenzo relató la desafortunada serie de eventos que llevó a los jóvenes amantes a quitarse la vida, enfatizando el impacto devastador del odio de las familias en sus hijos inocentes. Lord Capuleto y Lord Montesco, abrumados por el dolor y el remordimiento, reconocieron que su odio ciego había causado finalmente la pérdida de sus queridos hijos.  \n- El Príncipe Escalus instó a las familias a aprender de esta tragedia desgarradora y a abrazar el perdón y la unidad en honor a la memoria de Romeo y Julieta. Les urgió a trabajar juntos para crear una paz duradera en Verona, dejando de lado su animosidad de larga data. Fray Lorenzo ofreció su apoyo en mediar cualquier disputa futura y proporcionar guía espiritual para ayudar a las familias a sanar y seguir adelante.  \n- Cuando la reunión llegaba a su fin, Lord Capuleto y Lord Montesco se comprometieron a poner fin a su disputa y trabajar hacia la reconciliación. El Príncipe Escalus reafirmó su compromiso de asegurar que la tregua se mantuviera, prometiendo justicia rápida para cualquiera que se atreviera a romperla.  \n- Los asistentes acordaron reunirse regularmente para discutir el progreso de sus esfuerzos de reconciliación y abordar cualquier desafío que pudiera surgir."
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
    max_tokens=4000,
    temperature=0.5,
    system="Tu tarea es revisar las notas de reunión proporcionadas y crear un resumen conciso que capture la información esencial, enfocándote en los puntos clave y elementos de acción asignados a individuos o departamentos específicos durante la reunión. Usa un lenguaje claro y profesional, y organiza el resumen de manera lógica usando formato apropiado como encabezados, subencabezados y viñetas. Asegúrate de que el resumen sea fácil de entender y proporcione una visión general completa pero sucinta del contenido de la reunión, con un enfoque particular en indicar claramente quién es responsable de cada elemento de acción.",
    messages=[
        {
            "role": "user",
            "content": [
                {
                    "type": "text",
                    "text": "Notas de reunión:  \n  \nFecha: Verona, Italia - Finales del siglo XVI  \n  \nAsistentes:  \n- Lord Capuleto (Jefe de la familia Capuleto)  \n- Lord Montesco (Jefe de la familia Montesco)  \n- Príncipe Escalus (Gobernante de Verona)  \n- Fray Lorenzo (Consejero religioso)  \n  \nAgenda:  \n1. Abordar la disputa continua entre las familias Capuleto y Montesco  \n2. Discutir el matrimonio secreto de Romeo Montesco y Julieta Capuleto  \n3. Desarrollar un plan para traer paz a Verona  \n4. Abordar las muertes trágicas de Romeo y Julieta  \n  \nDiscusión:  \n- El Príncipe Escalus abrió la reunión expresando su grave preocupación por la disputa de larga data entre las familias Capuleto y Montesco. Amonestó tanto a Lord Capuleto como a Lord Montesco por los recientes enfrentamientos violentos que han perturbado la paz en las calles de Verona. El Príncipe advirtió que más violencia resultaría en consecuencias severas, incluyendo multas pesadas y exilio potencial para los perpetradores.  \n- Fray Lorenzo entonces abordó el tema del matrimonio entre Romeo Montesco y Julieta Capuleto, que había tenido lugar bajo su guía. Lord Capuleto y Lord Montesco evidentemente no sabían sobre esto, y reaccionaron con ira e incredulidad. Sin embargo, Fray Lorenzo les instó a considerar el amor profundo y trágico compartido por sus hijos y el potencial de este amor para sanar la brecha entre las familias en el futuro.  \n- El Príncipe Escalus propuso una tregua formal entre las familias Capuleto y Montesco. Exigió que ambos lados depusieran las armas y cesaran todas las acciones hostiles entre sí. El Príncipe declaró que cualquier violación de la tregua resultaría en castigos severos, incluyendo la posibilidad de exilio o incluso ejecución. Lord Capuleto y Lord Montesco, reconociendo la sabiduría en las palabras del Príncipe y la necesidad de paz para el bienestar de sus familias y la ciudad, acordaron a regañadientes los términos de la tregua.  \n- La reunión tomó un giro sombrío cuando se abordaron las muertes trágicas de Romeo y Julieta. Fray Lorenzo relató la desafortunada serie de eventos que llevó a los jóvenes amantes a quitarse la vida, enfatizando el impacto devastador del odio de las familias en sus hijos inocentes. Lord Capuleto y Lord Montesco, abrumados por el dolor y el remordimiento, reconocieron que su odio ciego había causado finalmente la pérdida de sus queridos hijos.  \n- El Príncipe Escalus instó a las familias a aprender de esta tragedia desgarradora y a abrazar el perdón y la unidad en honor a la memoria de Romeo y Julieta. Les urgió a trabajar juntos para crear una paz duradera en Verona, dejando de lado su animosidad de larga data. Fray Lorenzo ofreció su apoyo en mediar cualquier disputa futura y proporcionar guía espiritual para ayudar a las familias a sanar y seguir adelante.  \n- Cuando la reunión llegaba a su fin, Lord Capuleto y Lord Montesco se comprometieron a poner fin a su disputa y trabajar hacia la reconciliación. El Príncipe Escalus reafirmó su compromiso de asegurar que la tregua se mantuviera, prometiendo justicia rápida para cualquiera que se atreviera a romperla.  \n- Los asistentes acordaron reunirse regularmente para discutir el progreso de sus esfuerzos de reconciliación y abordar cualquier desafío que pudiera surgir."
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
  max_tokens: 4000,
  temperature: 0.5,
  system: "Tu tarea es revisar las notas de reunión proporcionadas y crear un resumen conciso que capture la información esencial, enfocándote en los puntos clave y elementos de acción asignados a individuos o departamentos específicos durante la reunión. Usa un lenguaje claro y profesional, y organiza el resumen de manera lógica usando formato apropiado como encabezados, subencabezados y viñetas. Asegúrate de que el resumen sea fácil de entender y proporcione una visión general completa pero sucinta del contenido de la reunión, con un enfoque particular en indicar claramente quién es responsable de cada elemento de acción.",
  messages: [
    {
      "role": "user",
      "content": [
        {
          "type": "text",
          "text": "Notas de reunión:  \n  \nFecha: Verona, Italia - Finales del siglo XVI  \n  \nAsistentes:  \n- Lord Capuleto (Jefe de la familia Capuleto)  \n- Lord Montesco (Jefe de la familia Montesco)  \n- Príncipe Escalus (Gobernante de Verona)  \n- Fray Lorenzo (Consejero religioso)  \n  \nAgenda:  \n1. Abordar la disputa continua entre las familias Capuleto y Montesco  \n2. Discutir el matrimonio secreto de Romeo Montesco y Julieta Capuleto  \n3. Desarrollar un plan para traer paz a Verona  \n4. Abordar las muertes trágicas de Romeo y Julieta  \n  \nDiscusión:  \n- El Príncipe Escalus abrió la reunión expresando su grave preocupación por la disputa de larga data entre las familias Capuleto y Montesco. Amonestó tanto a Lord Capuleto como a Lord Montesco por los recientes enfrentamientos violentos que han perturbado la paz en las calles de Verona. El Príncipe advirtió que más violencia resultaría en consecuencias severas, incluyendo multas pesadas y exilio potencial para los perpetradores.  \n- Fray Lorenzo entonces abordó el tema del matrimonio entre Romeo Montesco y Julieta Capuleto, que había tenido lugar bajo su guía. Lord Capuleto y Lord Montesco evidentemente no sabían sobre esto, y reaccionaron con ira e incredulidad. Sin embargo, Fray Lorenzo les instó a considerar el amor profundo y trágico compartido por sus hijos y el potencial de este amor para sanar la brecha entre las familias en el futuro.  \n- El Príncipe Escalus propuso una tregua formal entre las familias Capuleto y Montesco. Exigió que ambos lados depusieran las armas y cesaran todas las acciones hostiles entre sí. El Príncipe declaró que cualquier violación de la tregua resultaría en castigos severos, incluyendo la posibilidad de exilio o incluso ejecución. Lord Capuleto y Lord Montesco, reconociendo la sabiduría en las palabras del Príncipe y la necesidad de paz para el bienestar de sus familias y la ciudad, acordaron a regañadientes los términos de la tregua.  \n- La reunión tomó un giro sombrío cuando se abordaron las muertes trágicas de Romeo y Julieta. Fray Lorenzo relató la desafortunada serie de eventos que llevó a los jóvenes amantes a quitarse la vida, enfatizando el impacto devastador del odio de las familias en sus hijos inocentes. Lord Capuleto y Lord Montesco, abrumados por el dolor y el remordimiento, reconocieron que su odio ciego había causado finalmente la pérdida de sus queridos hijos.  \n- El Príncipe Escalus instó a las familias a aprender de esta tragedia desgarradora y a abrazar el perdón y la unidad en honor a la memoria de Romeo y Julieta. Les urgió a trabajar juntos para crear una paz duradera en Verona, dejando de lado su animosidad de larga data. Fray Lorenzo ofreció su apoyo en mediar cualquier disputa futura y proporcionar guía espiritual para ayudar a las familias a sanar y seguir adelante.  \n- Cuando la reunión llegaba a su fin, Lord Capuleto y Lord Montesco se comprometieron a poner fin a su disputa y trabajar hacia la reconciliación. El Príncipe Escalus reafirmó su compromiso de asegurar que la tregua se mantuviera, prometiendo justicia rápida para cualquiera que se atreviera a romperla.  \n- Los asistentes acordaron reunirse regularmente para discutir el progreso de sus esfuerzos de reconciliación y abordar cualquier desafío que pudiera surgir."
        }
      ]
    }
  ]
});
console.log(msg);

```

</Tab>
</Tabs>