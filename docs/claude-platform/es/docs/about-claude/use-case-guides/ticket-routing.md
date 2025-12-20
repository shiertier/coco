# Enrutamiento de tickets

Esta guía te muestra cómo aprovechar las capacidades avanzadas de comprensión del lenguaje natural de Claude para clasificar tickets de soporte al cliente a escala basándose en la intención del cliente, urgencia, priorización, perfil del cliente y más.

---

## Define si usar Claude para enrutamiento de tickets

Aquí hay algunos indicadores clave de que deberías usar un LLM como Claude en lugar de enfoques tradicionales de ML para tu tarea de clasificación:

    <section title="Tienes datos de entrenamiento etiquetados limitados disponibles">

        Los procesos tradicionales de ML requieren conjuntos de datos etiquetados masivos. El modelo preentrenado de Claude puede clasificar efectivamente tickets con solo unos pocos docenas de ejemplos etiquetados, reduciendo significativamente el tiempo de preparación de datos y los costos.
    
</section>
    <section title="Es probable que tus categorías de clasificación cambien o evolucionen con el tiempo">

        Una vez que se ha establecido un enfoque tradicional de ML, cambiarlo es una tarea laboriosa e intensiva en datos. Por otro lado, a medida que tu producto o las necesidades de los clientes evolucionan, Claude puede adaptarse fácilmente a cambios en las definiciones de clases o nuevas clases sin un reetiquetar extenso de datos de entrenamiento.
    
</section>
    <section title="Necesitas manejar entradas de texto complejas y no estructuradas">

        Los modelos tradicionales de ML a menudo luchan con datos no estructurados y requieren ingeniería de características extensiva. La comprensión avanzada del lenguaje de Claude permite una clasificación precisa basada en contenido y contexto, en lugar de depender de estructuras ontológicas estrictas.
    
</section>
    <section title="Tus reglas de clasificación se basan en la comprensión semántica">

        Los enfoques tradicionales de ML a menudo se basan en modelos de bolsa de palabras o coincidencia de patrones simples. Claude destaca en la comprensión y aplicación de reglas subyacentes cuando las clases se definen por condiciones en lugar de ejemplos.
    
</section>
    <section title="Requieres razonamiento interpretable para decisiones de clasificación">

        Muchos modelos tradicionales de ML proporcionan poca información sobre su proceso de toma de decisiones. Claude puede proporcionar explicaciones legibles por humanos para sus decisiones de clasificación, generando confianza en el sistema de automatización y facilitando una fácil adaptación si es necesario.
    
</section>
    <section title="Quieres manejar casos límite y tickets ambiguos de manera más efectiva">

        Los sistemas tradicionales de ML a menudo luchan con valores atípicos e entradas ambiguas, frecuentemente clasificándolos incorrectamente o usando una categoría general. Las capacidades de procesamiento del lenguaje natural de Claude le permiten interpretar mejor el contexto y los matices en los tickets de soporte, reduciendo potencialmente el número de tickets mal enrutados o sin clasificar que requieren intervención manual.
    
</section>
    <section title="Necesitas soporte multilingüe sin mantener modelos separados">

        Los enfoques tradicionales de ML típicamente requieren modelos separados o procesos de traducción extensivos para cada idioma soportado. Las capacidades multilingües de Claude le permiten clasificar tickets en varios idiomas sin la necesidad de modelos separados o procesos de traducción extensivos, simplificando el soporte para bases de clientes globales.
    
</section>

***

##  Construye e implementa tu flujo de trabajo de soporte LLM

### Comprende tu enfoque de soporte actual
Antes de sumergirte en la automatización, es crucial entender tu sistema de tickets existente. Comienza investigando cómo tu equipo de soporte actualmente maneja el enrutamiento de tickets.

Considera preguntas como:
* ¿Qué criterios se utilizan para determinar qué SLA/oferta de servicio se aplica?
* ¿Se utiliza el enrutamiento de tickets para determinar a qué nivel de soporte o especialista de producto va un ticket?
* ¿Hay alguna regla automatizada o flujos de trabajo ya en su lugar? ¿En qué casos fallan?
* ¿Cómo se manejan los casos límite o tickets ambiguos?
* ¿Cómo el equipo prioriza los tickets?

Cuanto más sepas sobre cómo los humanos manejan ciertos casos, mejor podrás trabajar con Claude para hacer la tarea.

### Define categorías de intención del usuario
Una lista bien definida de categorías de intención del usuario es crucial para una clasificación precisa de tickets de soporte con Claude. La capacidad de Claude para enrutar tickets efectivamente dentro de tu sistema es directamente proporcional a qué tan bien definidas estén las categorías de tu sistema.

Aquí hay algunos ejemplos de categorías de intención del usuario y subcategorías.

    <section title="Problema técnico">

        * Problema de hardware
        * Error de software
        * Problema de compatibilidad
        * Problema de rendimiento
    
</section>
    <section title="Gestión de cuenta">

        * Restablecimiento de contraseña
        * Problemas de acceso a la cuenta
        * Consultas de facturación
        * Cambios de suscripción
    
</section>
    <section title="Información del producto">

        * Consultas de características
        * Preguntas de compatibilidad del producto
        * Información de precios
        * Consultas de disponibilidad
    
</section>
    <section title="Orientación del usuario">

        * Preguntas de cómo hacer
        * Asistencia de uso de características
        * Consejo de mejores prácticas
        * Orientación de solución de problemas
    
</section>
    <section title="Retroalimentación">

        * Reportes de errores
        * Solicitudes de características
        * Retroalimentación general o sugerencias
        * Quejas
    
</section>
    <section title="Relacionado con pedidos">

        * Consultas de estado de pedido
        * Información de envío
        * Devoluciones e intercambios
        * Modificaciones de pedidos
    
</section>
    <section title="Solicitud de servicio">

        * Asistencia de instalación
        * Solicitudes de actualización
        * Programación de mantenimiento
        * Cancelación de servicio
    
</section>
    <section title="Preocupaciones de seguridad">

        * Consultas de privacidad de datos
        * Reportes de actividad sospechosa
        * Asistencia de características de seguridad
    
</section>
    <section title="Cumplimiento y legal">

        * Preguntas de cumplimiento normativo
        * Consultas de términos de servicio
        * Solicitudes de documentación legal
    
</section>
    <section title="Soporte de emergencia">

        * Fallos críticos del sistema
        * Problemas de seguridad urgentes
        * Problemas sensibles al tiempo
    
</section>
    <section title="Capacitación y educación">

        * Solicitudes de capacitación de productos
        * Consultas de documentación
        * Información de seminarios web o talleres
    
</section>
    <section title="Integración e API">

        * Asistencia de integración
        * Preguntas de uso de API
        * Consultas de compatibilidad de terceros
    
</section>

Además de la intención, el enrutamiento de tickets y la priorización también pueden ser influenciados por otros factores como urgencia, tipo de cliente, SLAs o idioma. Asegúrate de considerar otros criterios de enrutamiento al construir tu sistema de enrutamiento automatizado.

### Establece criterios de éxito

Trabaja con tu equipo de soporte para [definir criterios de éxito claros](/docs/es/test-and-evaluate/define-success) con puntos de referencia medibles, umbrales y objetivos.

Aquí hay algunos criterios estándar y puntos de referencia al usar LLMs para enrutamiento de tickets de soporte:

    <section title="Consistencia de clasificación">

        Esta métrica evalúa qué tan consistentemente Claude clasifica tickets similares a lo largo del tiempo. Es crucial para mantener la confiabilidad del enrutamiento. Mide esto probando periódicamente el modelo con un conjunto de entradas estandarizadas y apuntando a una tasa de consistencia del 95% o superior.
    
</section>
    <section title="Velocidad de adaptación">

        Esto mide qué tan rápido Claude puede adaptarse a nuevas categorías o patrones de tickets cambiantes. Prueba esto introduciendo nuevos tipos de tickets y midiendo el tiempo que tarda el modelo en lograr una precisión satisfactoria (por ejemplo, >90%) en estas nuevas categorías. Apunta a la adaptación dentro de 50-100 tickets de muestra.
    
</section>
    <section title="Manejo multilingüe">

        Esto evalúa la capacidad de Claude para enrutar con precisión tickets en múltiples idiomas. Mide la precisión del enrutamiento en diferentes idiomas, apuntando a no más de una caída de precisión del 5-10% para idiomas no primarios.
    
</section>
    <section title="Manejo de casos límite">

        Esto evalúa el rendimiento de Claude en tickets inusuales o complejos. Crea un conjunto de prueba de casos límite y mide la precisión del enrutamiento, apuntando a al menos 80% de precisión en estas entradas desafiantes.
    
</section>
    <section title="Mitigación de sesgos">

        Esto mide la equidad de Claude en el enrutamiento entre diferentes demografías de clientes. Audita regularmente las decisiones de enrutamiento para posibles sesgos, apuntando a una precisión de enrutamiento consistente (dentro del 2-3%) en todos los grupos de clientes.
    
</section>
    <section title="Eficiencia de prompt">

        En situaciones donde minimizar el conteo de tokens es crucial, este criterio evalúa qué tan bien Claude se desempeña con contexto mínimo. Mide la precisión del enrutamiento con cantidades variables de contexto proporcionado, apuntando a 90%+ de precisión con solo el título del ticket y una breve descripción.
    
</section>
    <section title="Puntuación de explicabilidad">

        Esto evalúa la calidad y relevancia de las explicaciones de Claude para sus decisiones de enrutamiento. Los evaluadores humanos pueden puntuar explicaciones en una escala (por ejemplo, 1-5), con el objetivo de lograr una puntuación promedio de 4 o superior.
    
</section>

Aquí hay algunos criterios de éxito comunes que pueden ser útiles independientemente de si se usa un LLM:

    <section title="Precisión del enrutamiento">

        La precisión del enrutamiento mide con qué frecuencia los tickets se asignan correctamente al equipo o individuo apropiado en el primer intento. Esto se mide típicamente como un porcentaje de tickets enrutados correctamente del total de tickets. Los puntos de referencia de la industria a menudo apuntan a una precisión del 90-95%, aunque esto puede variar según la complejidad de la estructura de soporte.
    
</section>
    <section title="Tiempo para asignación">

        Esta métrica rastrea qué tan rápido se asignan los tickets después de ser enviados. Los tiempos de asignación más rápidos generalmente conducen a resoluciones más rápidas y una mayor satisfacción del cliente. Los sistemas de clase mundial a menudo logran tiempos de asignación promedio de menos de 5 minutos, con muchos apuntando a enrutamiento casi instantáneo (que es posible con implementaciones de LLM).
    
</section>
    <section title="Tasa de reenrutamiento">

        La tasa de reenrutamiento indica con qué frecuencia los tickets necesitan ser reasignados después del enrutamiento inicial. Una tasa más baja sugiere un enrutamiento inicial más preciso. Apunta a una tasa de reenrutamiento por debajo del 10%, con sistemas de alto rendimiento logrando tasas tan bajas como 5% o menos.
    
</section>
    <section title="Tasa de resolución en primer contacto">

        Esto mide el porcentaje de tickets resueltos durante la primera interacción con el cliente. Las tasas más altas indican un enrutamiento eficiente y equipos de soporte bien preparados. Los puntos de referencia de la industria típicamente oscilan entre 70-75%, con los mejores desempeños logrando tasas del 80% o superior.
    
</section>
    <section title="Tiempo promedio de manejo">

        El tiempo promedio de manejo mide cuánto tiempo tarda en resolver un ticket de principio a fin. El enrutamiento eficiente puede reducir significativamente este tiempo. Los puntos de referencia varían ampliamente por industria y complejidad, pero muchas organizaciones apuntan a mantener el tiempo promedio de manejo por debajo de 24 horas para problemas no críticos.
    
</section>
    <section title="Puntuaciones de satisfacción del cliente">

        A menudo medidas a través de encuestas posteriores a la interacción, estas puntuaciones reflejan la felicidad general del cliente con el proceso de soporte. El enrutamiento efectivo contribuye a una mayor satisfacción. Apunta a puntuaciones CSAT del 90% o superior, con los mejores desempeños a menudo logrando tasas de satisfacción del 95%+.
    
</section>
    <section title="Tasa de escalada">

        Esto mide con qué frecuencia los tickets necesitan ser escalados a niveles superiores de soporte. Las tasas de escalada más bajas a menudo indican un enrutamiento inicial más preciso. Esfuérzate por una tasa de escalada por debajo del 20%, con sistemas de clase mundial logrando tasas del 10% o menos.
    
</section>
    <section title="Productividad del agente">

        Esta métrica analiza cuántos tickets pueden manejar efectivamente los agentes después de implementar la solución de enrutamiento. El enrutamiento mejorado debe aumentar la productividad. Mide esto rastreando tickets resueltos por agente por día u hora, apuntando a una mejora del 10-20% después de implementar un nuevo sistema de enrutamiento.
    
</section>
    <section title="Tasa de deflexión de autoservicio">

        Esto mide el porcentaje de tickets potenciales resueltos a través de opciones de autoservicio antes de entrar en el sistema de enrutamiento. Las tasas más altas indican un triaje previo al enrutamiento efectivo. Apunta a una tasa de deflexión del 20-30%, con los mejores desempeños logrando tasas del 40% o superior.
    
</section>
    <section title="Costo por ticket">

        Esta métrica calcula el costo promedio para resolver cada ticket de soporte. El enrutamiento eficiente debe ayudar a reducir este costo con el tiempo. Aunque los puntos de referencia varían ampliamente, muchas organizaciones apuntan a reducir el costo por ticket en un 10-15% después de implementar un sistema de enrutamiento mejorado.
    
</section>

### Elige el modelo Claude correcto

La elección del modelo depende de los compromisos entre costo, precisión y tiempo de respuesta.

Muchos clientes han encontrado que `claude-haiku-4-5-20251001` es un modelo ideal para enrutamiento de tickets, ya que es el modelo más rápido y rentable de la familia Claude 4 mientras sigue ofreciendo excelentes resultados. Si tu problema de clasificación requiere experiencia profunda en la materia o un gran volumen de categorías de intención con razonamiento complejo, puedes optar por el [modelo Sonnet más grande](/docs/es/about-claude/models).

### Construye un prompt fuerte

El enrutamiento de tickets es un tipo de tarea de clasificación. Claude analiza el contenido de un ticket de soporte y lo clasifica en categorías predefinidas basándose en el tipo de problema, urgencia, experiencia requerida u otros factores relevantes.

Escribamos un prompt de clasificación de tickets. Nuestro prompt inicial debe contener el contenido de la solicitud del usuario y devolver tanto el razonamiento como la intención.

<Tip>
Prueba el [generador de prompts](/docs/es/prompt-generator) en la [Consola de Claude](/login) para que Claude escriba un primer borrador para ti.
</Tip>

Aquí hay un ejemplo de prompt de clasificación de enrutamiento de tickets:
```python
def classify_support_request(ticket_contents):
    # Define the prompt for the classification task
    classification_prompt = f"""You will be acting as a customer support ticket classification system. Your task is to analyze customer support requests and output the appropriate classification intent for each request, along with your reasoning. 

        Here is the customer support request you need to classify:

        <request>{ticket_contents}</request>

        Please carefully analyze the above request to determine the customer's core intent and needs. Consider what the customer is asking for has concerns about.

        First, write out your reasoning and analysis of how to classify this request inside <reasoning> tags.

        Then, output the appropriate classification label for the request inside a <intent> tag. The valid intents are:
        <intents>
        <intent>Support, Feedback, Complaint</intent>
        <intent>Order Tracking</intent>
        <intent>Refund/Exchange</intent>
        </intents>

        A request may have ONLY ONE applicable intent. Only include the intent that is most applicable to the request.

        As an example, consider the following request:
        <request>Hello! I had high-speed fiber internet installed on Saturday and my installer, Kevin, was absolutely fantastic! Where can I send my positive review? Thanks for your help!</request>

        Here is an example of how your output should be formatted (for the above example request):
        <reasoning>The user seeks information in order to leave positive feedback.</reasoning>
        <intent>Support, Feedback, Complaint</intent>

        Here are a few more examples:
        <examples>
        <example 2>
        Example 2 Input:
        <request>I wanted to write and personally thank you for the compassion you showed towards my family during my father's funeral this past weekend. Your staff was so considerate and helpful throughout this whole process; it really took a load off our shoulders. The visitation brochures were beautiful. We'll never forget the kindness you showed us and we are so appreciative of how smoothly the proceedings went. Thank you, again, Amarantha Hill on behalf of the Hill Family.</request>

        Example 2 Output:
        <reasoning>User leaves a positive review of their experience.</reasoning>
        <intent>Support, Feedback, Complaint</intent>
        </example 2>
        <example 3>

        ...

        </example 8>
        <example 9>
        Example 9 Input:
        <request>Your website keeps sending ad-popups that block the entire screen. It took me twenty minutes just to finally find the phone number to call and complain. How can I possibly access my account information with all of these popups? Can you access my account for me, since your website is broken? I need to know what the address is on file.</request>

        Example 9 Output:
        <reasoning>The user requests help accessing their web account information.</reasoning>
        <intent>Support, Feedback, Complaint</intent>
        </example 9>

        Remember to always include your classification reasoning before your actual intent output. The reasoning should be enclosed in <reasoning> tags and the intent in <intent> tags. Return only the reasoning and the intent.
        """
```

Desglosemos los componentes clave de este prompt:
* Usamos f-strings de Python para crear la plantilla de prompt, permitiendo que `ticket_contents` se inserte en las etiquetas `<request>`.
* Le damos a Claude un rol claramente definido como un sistema de clasificación que analiza cuidadosamente el contenido del ticket para determinar la intención central del cliente y las necesidades.
* Instruimos a Claude sobre el formato de salida adecuado, en este caso proporcionar su razonamiento y análisis dentro de etiquetas `<reasoning>`, seguido por la etiqueta de clasificación apropiada dentro de etiquetas `<intent>`.
* Especificamos las categorías de intención válidas: "Support, Feedback, Complaint", "Order Tracking" y "Refund/Exchange".
* Incluimos algunos ejemplos (también conocido como few-shot prompting) para ilustrar cómo debe formatearse la salida, lo que mejora la precisión y consistencia.

La razón por la que queremos que Claude divida su respuesta en varias secciones de etiquetas XML es para que podamos usar expresiones regulares para extraer por separado el razonamiento y la intención de la salida. Esto nos permite crear pasos siguientes dirigidos en el flujo de trabajo de enrutamiento de tickets, como usar solo la intención para decidir a quién enrutar el ticket.

### Implementa tu prompt

Es difícil saber qué tan bien funciona tu prompt sin implementarlo en un entorno de producción de prueba y [ejecutar evaluaciones](/docs/es/test-and-evaluate/develop-tests).

Construyamos la estructura de implementación. Comienza definiendo la firma del método para envolver nuestra llamada a Claude. Tomaremos el método que ya hemos comenzado a escribir, que tiene `ticket_contents` como entrada, y ahora devolveremos una tupla de `reasoning` e `intent` como salida. Si tienes una automatización existente usando ML tradicional, querrás seguir esa firma de método en su lugar.

```python
import anthropic
import re

# Create an instance of the Claude API client
client = anthropic.Anthropic()

# Set the default model
DEFAULT_MODEL="claude-haiku-4-5-20251001"

def classify_support_request(ticket_contents):
    # Define the prompt for the classification task
    classification_prompt = f"""You will be acting as a customer support ticket classification system. 
        ...
        ... The reasoning should be enclosed in <reasoning> tags and the intent in <intent> tags. Return only the reasoning and the intent.
        """
    # Send the prompt to the API to classify the support request.
    message = client.messages.create(
        model=DEFAULT_MODEL,
        max_tokens=500,
        temperature=0,
        messages=[{"role": "user", "content": classification_prompt}],
        stream=False,
    )
    reasoning_and_intent = message.content[0].text

    # Use Python's regular expressions library to extract `reasoning`.
    reasoning_match = re.search(
        r"<reasoning>(.*?)</reasoning>", reasoning_and_intent, re.DOTALL
    )
    reasoning = reasoning_match.group(1).strip() if reasoning_match else ""

    # Similarly, also extract the `intent`.
    intent_match = re.search(r"<intent>(.*?)</intent>", reasoning_and_intent, re.DOTALL)
    intent = intent_match.group(1).strip() if intent_match else ""

    return reasoning, intent
```

Este código:
* Importa la biblioteca Anthropic y crea una instancia de cliente usando tu clave API.
* Define una función `classify_support_request` que toma una cadena `ticket_contents`.
* Envía `ticket_contents` a Claude para clasificación usando `classification_prompt`
* Devuelve el `reasoning` e `intent` del modelo extraído de la respuesta.

Como necesitamos esperar a que se genere todo el texto de razonamiento e intención antes de analizar, establecemos `stream=False` (el predeterminado).

***

## Evalúa tu prompt

El prompting a menudo requiere pruebas y optimización para estar listo para producción. Para determinar la preparación de tu solución, evalúa el rendimiento basándote en los criterios de éxito y umbrales que estableciste anteriormente.

Para ejecutar tu evaluación, necesitarás casos de prueba para ejecutarla. El resto de esta guía asume que ya has [desarrollado tus casos de prueba](/docs/es/test-and-evaluate/develop-tests).

### Construye una función de evaluación

Nuestra evaluación de ejemplo para esta guía mide el rendimiento de Claude a lo largo de tres métricas clave:
* Precisión
* Costo por clasificación

Es posible que necesites evaluar a Claude en otros ejes dependiendo de qué factores sean importantes para ti.

Para evaluar esto, primero tenemos que modificar el script que escribimos y agregar una función para comparar la intención predicha con la intención real y calcular el porcentaje de predicciones correctas. También tenemos que agregar funcionalidad de cálculo de costos y medición de tiempo.

```python
import anthropic
import re

# Create an instance of the Claude API client
client = anthropic.Anthropic()

# Set the default model
DEFAULT_MODEL="claude-haiku-4-5-20251001"

def classify_support_request(request, actual_intent):
    # Define the prompt for the classification task
    classification_prompt = f"""You will be acting as a customer support ticket classification system. 
        ...
        ...The reasoning should be enclosed in <reasoning> tags and the intent in <intent> tags. Return only the reasoning and the intent.
        """

    message = client.messages.create(
        model=DEFAULT_MODEL,
        max_tokens=500,
        temperature=0,
        messages=[{"role": "user", "content": classification_prompt}],
    )
    usage = message.usage  # Get the usage statistics for the API call for how many input and output tokens were used.
    reasoning_and_intent = message.content[0].text

    # Use Python's regular expressions library to extract `reasoning`.
    reasoning_match = re.search(
        r"<reasoning>(.*?)</reasoning>", reasoning_and_intent, re.DOTALL
    )
    reasoning = reasoning_match.group(1).strip() if reasoning_match else ""

    # Similarly, also extract the `intent`.
    intent_match = re.search(r"<intent>(.*?)</intent>", reasoning_and_intent, re.DOTALL)
    intent = intent_match.group(1).strip() if intent_match else ""

      # Check if the model's prediction is correct.
    correct = actual_intent.strip() == intent.strip()

    # Return the reasoning, intent, correct, and usage.
    return reasoning, intent, correct, usage
```

Desglosemos las ediciones que hemos hecho:
* Agregamos `actual_intent` de nuestros casos de prueba al método `classify_support_request` y configuramos una comparación para evaluar si la clasificación de intención de Claude coincide con nuestra clasificación de intención dorada.
* Extrajimos estadísticas de uso para la llamada API para calcular el costo basado en tokens de entrada y salida utilizados

### Ejecuta tu evaluación

Una evaluación adecuada requiere umbrales y puntos de referencia claros para determinar qué es un buen resultado. El script anterior nos dará los valores de tiempo de ejecución para precisión, tiempo de respuesta y costo por clasificación, pero aún necesitaríamos umbrales claramente establecidos. Por ejemplo:
* **Precisión:** 95% (de 100 pruebas)
* **Costo por clasificación:** Reducción del 50% en promedio (en 100 pruebas) del método de enrutamiento actual

Tener estos umbrales te permite decir rápida y fácilmente a escala, y con empirismo imparcial, qué método es mejor para ti y qué cambios podrían necesitarse para adaptarse mejor a tus requisitos.

***

## Mejora el rendimiento

En escenarios complejos, puede ser útil considerar estrategias adicionales para mejorar el rendimiento más allá de las [técnicas estándar de ingeniería de prompts](/docs/es/build-with-claude/prompt-engineering/overview) y [estrategias de implementación de guardrails](/docs/es/test-and-evaluate/strengthen-guardrails/reduce-hallucinations). Aquí hay algunos escenarios comunes:

### Usa una jerarquía taxonómica para casos con 20+ categorías de intención

A medida que crece el número de clases, el número de ejemplos requeridos también se expande, potencialmente haciendo que el prompt sea difícil de manejar. Como alternativa, puedes considerar implementar un sistema de clasificación jerárquico usando una mezcla de clasificadores.
1. Organiza tus intenciones en una estructura de árbol taxonómico.
2. Crea una serie de clasificadores en cada nivel del árbol, habilitando un enfoque de enrutamiento en cascada.

Por ejemplo, podrías tener un clasificador de nivel superior que categoriza ampliamente los tickets en "Problemas Técnicos", "Preguntas de Facturación" e "Consultas Generales". Cada una de estas categorías puede tener su propio subclasificador para refinar aún más la clasificación.

![](/docs/images/ticket-hierarchy.png)

* **Pros - mayor matiz y precisión:** Puedes crear diferentes prompts para cada ruta padre, permitiendo una clasificación más dirigida y específica del contexto. Esto puede conducir a una precisión mejorada y un manejo más matizado de las solicitudes de los clientes.

* **Contras - latencia aumentada:** Ten en cuenta que múltiples clasificadores pueden conducir a una latencia aumentada, y recomendamos implementar este enfoque con nuestro modelo más rápido, Haiku.

### Usa bases de datos vectoriales y búsqueda de similitud de recuperación para manejar tickets altamente variables

A pesar de que proporcionar ejemplos es la forma más efectiva de mejorar el rendimiento, si las solicitudes de soporte son altamente variables, puede ser difícil incluir suficientes ejemplos en un único prompt.

En este escenario, podrías emplear una base de datos vectorial para hacer búsquedas de similitud desde un conjunto de datos de ejemplos y recuperar los ejemplos más relevantes para una consulta dada.

Este enfoque, descrito en detalle en nuestra [receta de clasificación](https://github.com/anthropics/anthropic-cookbook/blob/82675c124e1344639b2a875aa9d3ae854709cd83/skills/classification/guide.ipynb), ha demostrado mejorar el rendimiento del 71% de precisión al 93% de precisión.

### Cuenta específicamente para casos límite esperados

Aquí hay algunos escenarios donde Claude puede clasificar incorrectamente tickets (puede haber otros que sean únicos para tu situación). En estos escenarios, considera proporcionar instrucciones explícitas o ejemplos en el prompt de cómo Claude debe manejar el caso límite:

    <section title="Los clientes hacen solicitudes implícitas">

        Los clientes a menudo expresan necesidades indirectamente. Por ejemplo, "He estado esperando mi paquete durante más de dos semanas" puede ser una solicitud indirecta de estado del pedido.
        * **Solución:** Proporciona a Claude algunos ejemplos reales de clientes de este tipo de solicitudes, junto con cuál es la intención subyacente. Puedes obtener resultados aún mejores si incluyes una justificación de clasificación para intenciones de tickets particularmente matizadas, para que Claude pueda generalizar mejor la lógica a otros tickets.
    
</section>
    <section title="Claude prioriza la emoción sobre la intención">

        Cuando los clientes expresan insatisfacción, Claude puede priorizar abordar la emoción sobre resolver el problema subyacente.
        * **Solución:** Proporciona a Claude direcciones sobre cuándo priorizar el sentimiento del cliente o no. Puede ser algo tan simple como "Ignora todas las emociones del cliente. Enfócate solo en analizar la intención de la solicitud del cliente y qué información el cliente podría estar pidiendo."
    
</section>
    <section title="Múltiples problemas causan confusión en la priorización de problemas">

        Cuando los clientes presentan múltiples problemas en una única interacción, Claude puede tener dificultades para identificar la preocupación principal.
        * **Solución:** Clarifica la priorización de intenciones para que Claude pueda clasificar mejor las intenciones extraídas e identificar la preocupación principal.
    
</section>

***

## Integra Claude en tu flujo de trabajo de soporte más amplio

La integración adecuada requiere que tomes algunas decisiones sobre cómo tu script de enrutamiento de tickets basado en Claude se ajusta a la arquitectura de tu sistema de enrutamiento de tickets más amplio. Hay dos formas en que podrías hacer esto:
* **Basado en push:** El sistema de tickets de soporte que estás usando (por ejemplo, Zendesk) activa tu código enviando un evento webhook a tu servicio de enrutamiento, que luego clasifica la intención y la enruta.
    * Este enfoque es más escalable en la web, pero necesita que expongas un punto final público.
* **Basado en pull:** Tu código extrae los últimos tickets basándose en un cronograma dado y los enruta en el momento del pull.
    * Este enfoque es más fácil de implementar pero podría hacer llamadas innecesarias al sistema de tickets de soporte cuando la frecuencia de pull es demasiado alta o podría ser demasiado lento cuando la frecuencia de pull es demasiado baja.

Para cualquiera de estos enfoques, necesitarás envolver tu script en un servicio. La elección del enfoque depende de qué APIs proporcione tu sistema de tickets de soporte.

***

<CardGroup cols={2}>
    <Card title="Libro de recetas de clasificación" icon="link" href="https://github.com/anthropics/anthropic-cookbook/tree/main/capabilities/classification">
        Visita nuestro libro de recetas de clasificación para más código de ejemplo y orientación de evaluación detallada.
    </Card>
    <Card title="Consola de Claude" icon="link" href="/dashboard">
        Comienza a construir y evaluar tu flujo de trabajo en la Consola de Claude.
    </Card>
</CardGroup>