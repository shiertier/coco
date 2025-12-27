# Agente de soporte al cliente

Esta guía explora cómo aprovechar las capacidades conversacionales avanzadas de Claude para manejar consultas de clientes en tiempo real, proporcionando soporte 24/7, reduciendo tiempos de espera y gestionando altos volúmenes de soporte con respuestas precisas e interacciones positivas.

---

## Antes de construir con Claude

### Decide si usar Claude para chat de soporte

Aquí hay algunos indicadores clave de que deberías emplear un LLM como Claude para automatizar partes de tu proceso de soporte al cliente:

  <section title="Alto volumen de consultas repetitivas">

    Claude destaca en el manejo de un gran número de preguntas similares de manera eficiente, liberando a los agentes humanos para problemas más complejos.
  
</section>
  <section title="Necesidad de síntesis rápida de información">

    Claude puede recuperar, procesar y combinar información de bases de conocimiento vastas rápidamente, mientras que los agentes humanos pueden necesitar tiempo para investigar o consultar múltiples fuentes.
  
</section>
  <section title="Requisito de disponibilidad 24/7">

    Claude puede proporcionar soporte ininterrumpido sin fatiga, mientras que el personal de agentes humanos para cobertura continua puede ser costoso y desafiante.
  
</section>
  <section title="Escalado rápido durante períodos pico">

    Claude puede manejar aumentos repentinos en el volumen de consultas sin necesidad de contratar y capacitar personal adicional.
  
</section>
  <section title="Voz de marca consistente">

    Puedes instruir a Claude para que represente consistentemente el tono y los valores de tu marca, mientras que los agentes humanos pueden variar en sus estilos de comunicación.
  
</section>

Algunas consideraciones para elegir Claude sobre otros LLMs:

- Priorizas conversación natural y matizada: La sofisticada comprensión del lenguaje de Claude permite conversaciones más naturales y conscientes del contexto que se sienten más humanas que chats con otros LLMs.
- A menudo recibes consultas complejas y abiertas: Claude puede manejar una amplia gama de temas e inquietudes sin generar respuestas enlatadas o requerir programación extensiva de permutaciones de expresiones de usuarios.
- Necesitas soporte multilingüe escalable: Las capacidades multilingües de Claude le permiten participar en conversaciones en más de 200 idiomas sin necesidad de chatbots separados o procesos de traducción extensivos para cada idioma soportado.

### Define tu interacción de chat ideal

Describe una interacción ideal del cliente para definir cómo y cuándo esperas que el cliente interactúe con Claude. Este esquema te ayudará a determinar los requisitos técnicos de tu solución.

Aquí hay un ejemplo de interacción de chat para soporte al cliente de seguros de automóviles:

* **Cliente**: Inicia la experiencia de chat de soporte
   * **Claude**: Saluda calurosamente al cliente e inicia la conversación
* **Cliente**: Pregunta sobre seguros para su nuevo automóvil eléctrico
   * **Claude**: Proporciona información relevante sobre cobertura de vehículos eléctricos
* **Cliente**: Hace preguntas relacionadas con necesidades únicas para seguros de vehículos eléctricos
   * **Claude**: Responde con respuestas precisas e informativas y proporciona enlaces a las fuentes
* **Cliente**: Hace preguntas fuera de tema no relacionadas con seguros o automóviles
   * **Claude**: Aclara que no discute temas no relacionados y redirige al usuario de vuelta a seguros de automóviles
* **Cliente**: Expresa interés en una cotización de seguros
   * **Claude**: Hace un conjunto de preguntas para determinar la cotización apropiada, adaptándose a sus respuestas
   * **Claude**: Envía una solicitud para usar la herramienta API de generación de cotizaciones junto con la información necesaria recopilada del usuario
   * **Claude**: Recibe la información de respuesta de la herramienta API, sintetiza la información en una respuesta natural y presenta la cotización proporcionada al usuario
* **Cliente**: Hace preguntas de seguimiento
   * **Claude**: Responde preguntas de seguimiento según sea necesario
   * **Claude**: Guía al cliente a los próximos pasos en el proceso de seguros y cierra la conversación

<Tip>En el ejemplo real que escribas para tu propio caso de uso, podrías encontrar útil escribir las palabras reales en esta interacción para que también puedas tener una idea del tono ideal, la duración de la respuesta y el nivel de detalle que deseas que Claude tenga.</Tip>

### Divide la interacción en tareas únicas

El chat de soporte al cliente es una colección de múltiples tareas diferentes, desde responder preguntas hasta recuperación de información hasta tomar acciones en solicitudes, todo envuelto en una única interacción del cliente. Antes de comenzar a construir, divide tu interacción ideal del cliente en cada tarea que deseas que Claude pueda realizar. Esto asegura que puedas hacer prompts y evaluar Claude para cada tarea, y te da una buena idea del rango de interacciones que necesitas considerar al escribir casos de prueba.

<Tip>Los clientes a veces encuentran útil visualizar esto como un diagrama de flujo de interacción de posibles puntos de inflexión de conversación dependiendo de las solicitudes del usuario.</Tip>

Aquí están las tareas clave asociadas con el ejemplo de interacción de seguros anterior:

1. Saludo y orientación general
   - Saluda calurosamente al cliente e inicia la conversación
   - Proporciona información general sobre la empresa e interacción

2. Información del producto
   - Proporciona información sobre cobertura de vehículos eléctricos
   <Note>Esto requerirá que Claude tenga la información necesaria en su contexto, e implicaría que una [integración RAG](https://github.com/anthropics/anthropic-cookbook/blob/main/skills/retrieval_augmented_generation/guide.ipynb) sea necesaria.</Note>
   - Responde preguntas relacionadas con necesidades únicas de seguros de vehículos eléctricos
   - Responde preguntas de seguimiento sobre la cotización o detalles del seguro
   - Ofrece enlaces a fuentes cuando sea apropiado

3. Gestión de conversación
   - Mantente en tema (seguros de automóviles)
   - Redirige preguntas fuera de tema de vuelta a temas relevantes

4. Generación de cotización
   - Haz preguntas apropiadas para determinar elegibilidad de cotización
   - Adapta preguntas basadas en respuestas del cliente
   - Envía información recopilada a API de generación de cotizaciones
   - Presenta la cotización proporcionada al cliente

### Establece criterios de éxito

Trabaja con tu equipo de soporte para [definir criterios de éxito claros](/docs/es/test-and-evaluate/define-success) y escribe [evaluaciones detalladas](/docs/es/test-and-evaluate/develop-tests) con puntos de referencia y objetivos medibles.

Aquí hay criterios y puntos de referencia que se pueden usar para evaluar qué tan exitosamente Claude realiza las tareas definidas:

  <section title="Precisión de comprensión de consultas">

    Esta métrica evalúa qué tan precisamente Claude entiende las consultas de los clientes en varios temas. Mide esto revisando una muestra de conversaciones y evaluando si Claude tiene la interpretación correcta de la intención del cliente, pasos críticos siguientes, qué se ve como resolución exitosa, y más. Apunta a una precisión de comprensión del 95% o superior.
  
</section>
  <section title="Relevancia de la respuesta">

    Esto evalúa qué tan bien la respuesta de Claude aborda la pregunta o problema específico del cliente. Evalúa un conjunto de conversaciones y califica la relevancia de cada respuesta (usando calificación basada en LLM para escala). Apunta a una puntuación de relevancia del 90% o superior.
  
</section>
  <section title="Precisión de la respuesta">

    Evalúa la corrección de la información general de la empresa y del producto proporcionada al usuario, basada en la información proporcionada a Claude en contexto. Apunta a una precisión del 100% en esta información introductoria.
  
</section>
  <section title="Relevancia de provisión de citas">

    Rastrea la frecuencia y relevancia de enlaces o fuentes ofrecidas. Apunta a proporcionar fuentes relevantes en el 80% de las interacciones donde información adicional podría ser beneficiosa.
  
</section>
  <section title="Adherencia al tema">

    Mide qué tan bien Claude se mantiene en tema, como el tema de seguros de automóviles en nuestra implementación de ejemplo. Apunta a que el 95% de las respuestas estén directamente relacionadas con seguros de automóviles o la consulta específica del cliente.
  
</section>
  <section title="Efectividad de generación de contenido">

    Mide qué tan exitoso es Claude en determinar cuándo generar contenido informativo y qué tan relevante es ese contenido. Por ejemplo, en nuestra implementación, estaríamos determinando qué tan bien Claude entiende cuándo generar una cotización y qué tan precisa es esa cotización. Apunta a una precisión del 100%, ya que esta es información vital para una interacción exitosa del cliente.
  
</section>
  <section title="Eficiencia de escalada">

    Esto mide la capacidad de Claude para reconocer cuándo una consulta necesita intervención humana y escalar apropiadamente. Rastrea el porcentaje de conversaciones escaladas correctamente versus aquellas que deberían haber sido escaladas pero no lo fueron. Apunta a una precisión de escalada del 95% o superior.
  
</section>

Aquí hay criterios y puntos de referencia que se pueden usar para evaluar el impacto empresarial de emplear Claude para soporte:

  <section title="Mantenimiento del sentimiento">

    Esto evalúa la capacidad de Claude para mantener o mejorar el sentimiento del cliente a lo largo de la conversación. Usa herramientas de análisis de sentimiento para medir el sentimiento al principio y al final de cada conversación. Apunta a sentimiento mantenido o mejorado en el 90% de las interacciones.
  
</section>
  <section title="Tasa de deflexión">

    El porcentaje de consultas de clientes manejadas exitosamente por el chatbot sin intervención humana. Típicamente apunta a una tasa de deflexión del 70-80%, dependiendo de la complejidad de las consultas.
  
</section>
  <section title="Puntuación de satisfacción del cliente">

    Una medida de qué tan satisfechos están los clientes con su interacción con el chatbot. Usualmente hecho a través de encuestas post-interacción. Apunta a una puntuación CSAT de 4 de 5 o superior.
  
</section>
  <section title="Tiempo promedio de manejo">

    El tiempo promedio que tarda el chatbot en resolver una consulta. Esto varía ampliamente basado en la complejidad de los problemas, pero generalmente, apunta a un AHT más bajo comparado con agentes humanos.
  
</section>

## Cómo implementar Claude como agente de servicio al cliente

### Elige el modelo Claude correcto

La elección del modelo depende de los compromisos entre costo, precisión y tiempo de respuesta.

Para chat de soporte al cliente, Claude Sonnet 4.5 es adecuado para equilibrar inteligencia, latencia y costo. Sin embargo, para instancias donde tienes flujo de conversación con múltiples prompts incluyendo RAG, uso de herramientas, y/o prompts de contexto largo, Claude Haiku 4.5 puede ser más adecuado para optimizar la latencia.

### Construye un prompt fuerte

Usar Claude para soporte al cliente requiere que Claude tenga suficiente dirección y contexto para responder apropiadamente, mientras tiene suficiente flexibilidad para manejar una amplia gama de consultas de clientes.

Comencemos escribiendo los elementos de un prompt fuerte, comenzando con un system prompt:

```python
IDENTITY = """You are Eva, a friendly and knowledgeable AI assistant for Acme Insurance 
Company. Your role is to warmly welcome customers and provide information on 
Acme's insurance offerings, which include car insurance and electric car 
insurance. You can also help customers get quotes for their insurance needs."""
```

<Tip>Aunque podrías estar tentado de poner toda tu información dentro de un system prompt como una forma de separar instrucciones de la conversación del usuario, Claude en realidad funciona mejor con la mayor parte del contenido de su prompt escrito dentro del primer turno `User` (con la única excepción siendo role prompting). Lee más en [Giving Claude a role with a system prompt](/docs/es/build-with-claude/prompt-engineering/system-prompts).</Tip>

Es mejor dividir prompts complejos en subsecciones y escribir una parte a la vez. Para cada tarea, podrías encontrar mayor éxito siguiendo un proceso paso a paso para definir las partes del prompt que Claude necesitaría para hacer la tarea bien. Para este ejemplo de soporte al cliente de seguros de automóviles, estaremos escribiendo por partes todas las partes para un prompt comenzando con la tarea "Greeting and general guidance". Esto también hace que depurar tu prompt sea más fácil ya que puedes ajustar más rápidamente partes individuales del prompt general.

Pondremos todos estos fragmentos en un archivo llamado `config.py`.

```python
STATIC_GREETINGS_AND_GENERAL = """
<static_context>
Acme Auto Insurance: Your Trusted Companion on the Road

About:
At Acme Insurance, we understand that your vehicle is more than just a mode of transportation—it's your ticket to life's adventures. 
Since 1985, we've been crafting auto insurance policies that give drivers the confidence to explore, commute, and travel with peace of mind.
Whether you're navigating city streets or embarking on cross-country road trips, Acme is there to protect you and your vehicle. 
Our innovative auto insurance policies are designed to adapt to your unique needs, covering everything from fender benders to major collisions.
With Acme's award-winning customer service and swift claim resolution, you can focus on the joy of driving while we handle the rest. 
We're not just an insurance provider—we're your co-pilot in life's journeys.
Choose Acme Auto Insurance and experience the assurance that comes with superior coverage and genuine care. Because at Acme, we don't just 
insure your car—we fuel your adventures on the open road.

Note: We also offer specialized coverage for electric vehicles, ensuring that drivers of all car types can benefit from our protection.

Acme Insurance offers the following products:
- Car insurance
- Electric car insurance
- Two-wheeler insurance

Business hours: Monday-Friday, 9 AM - 5 PM EST
Customer service number: 1-800-123-4567
</static_context>
"""
```

Luego haremos lo mismo para nuestra información de seguros de automóviles y seguros de automóviles eléctricos.

```python
STATIC_CAR_INSURANCE="""
<static_context>
Car Insurance Coverage:
Acme's car insurance policies typically cover:
1. Liability coverage: Pays for bodily injury and property damage you cause to others.
2. Collision coverage: Pays for damage to your car in an accident.
3. Comprehensive coverage: Pays for damage to your car from non-collision incidents.
4. Medical payments coverage: Pays for medical expenses after an accident.
5. Uninsured/underinsured motorist coverage: Protects you if you're hit by a driver with insufficient insurance.

Optional coverages include:
- Rental car reimbursement
- Roadside assistance
- New car replacement
</static_context>
"""

STATIC_ELECTRIC_CAR_INSURANCE="""
<static_context>
Electric Car Insurance:
Our specialized electric car insurance goes beyond traditional auto coverage, 
offering tailored protection for your high-tech investment. In addition to standard coverages, we offer:
- Safeguarding expensive battery systems
- Covering home charging equipment
- Adapting to the rapidly evolving world of electric vehicles

Whether you're driving a Rivian, Nissan Leaf, Chevrolet Bolt, 
or any other electric vehicle, our knowledgeable agents are ready to craft a policy 
that gives you the confidence to enjoy every electron-powered mile.
</static_context>
"""
```

Ahora que tenemos nuestro contenido estático, agreguemos al menos 4-5 interacciones de ejemplo "buenas" para guiar las respuestas de Claude. Estos ejemplos deben ser representativos de tu interacción ideal del cliente e pueden incluir guardrails, llamadas de herramientas, etc.

```python
EXAMPLES="""
Here are a few examples of how you can interact with customers:

<example 1>
H: Hi, do you offer commercial insurance for small businesses?

A: Ah! Unfortunately, we don't offer commercial insurance at this time. 
However, we do provide a range of personal insurance products including car insurance and electric car insurance. 
Would you like to know more about these offerings?
</example 1>

<example 2>
H: Hi, what are your business hours?

A: Our business hours are Monday to Friday, 9 AM to 5 PM EST. 
If you have any questions or need assistance, feel free to reach out during those times. 
How else can I assist you today?
</example 2>

<example 3>
H: I'm shopping for car insurance. What does Acme offer?

A: Great choice! Acme's car insurance policies typically include:
- Liability coverage: Protects you if you're at fault in an accident
- Collision coverage: Pays for damage to your car in an accident
- Comprehensive coverage: Covers non-collision incidents like theft or vandalism
- Medical payments coverage: Pays for medical expenses after an accident
- Uninsured/underinsured motorist coverage: Protects you from drivers with insufficient insurance

We also offer optional coverages like rental car reimbursement, roadside assistance, and new car replacement. 
Would you like more details on any of these coverages, or do you have specific questions about insuring your vehicle?
</example 3>

<example 4>
H: I'd like to hear about your electric car insurance.

A: Certainly! Our electric car insurance is specially designed to meet the unique needs of electric vehicle owners. 
In addition to all the standard coverages we offer for regular cars, our electric car policies include:

- Battery coverage: Protection against damage or degradation of your vehicle's expensive battery system
- Tech package: Covers repairs or replacement of advanced electronics and onboard computer systems
- Charging station coverage: Protects both home and portable charging equipment
- Cybersecurity protection: Coverage against hacking or digital threats
- Specialized roadside assistance: Includes flatbed towing to protect the vehicle's undercarriage

We cover a wide range of electric vehicles, including popular models from Tesla, Rivian, Nissan, Chevrolet, and more. 
Is there a specific electric vehicle you're looking to insure, or would you like more details on any of these coverages?
</example 4>

<example 5>
H: I'd like to get a quote for my car insurance.

A: Certainly! I'd be happy to help you get a quote for your car insurance. 
To provide you with an accurate quote, I'll need to collect some information about your vehicle and the primary driver. 
Let's start with the basics:

1. What is the make and model of your vehicle?
2. What year was it manufactured?
3. Approximately how many miles have you driven?
4. What is the age of the primary driver?

Once you provide this information, I'll use our quoting tool to generate a personalized insurance quote for you.
</example 5>
"""
```

También querrás incluir cualquier instrucción importante que describa los Qué hacer y Qué no hacer para cómo Claude debe interactuar con el cliente. 
Esto puede extraerse de guardrails de marca o políticas de soporte.

```python
ADDITIONAL_GUARDRAILS = """Please adhere to the following guardrails:
1. Only provide information about insurance types listed in our offerings.
2. If asked about an insurance type we don't offer, politely state 
that we don't provide that service.
3. Do not speculate about future product offerings or company plans.
4. Don't make promises or enter into agreements it's not authorized to make.
You only provide information and guidance.
5. Do not mention any competitor's products or services.
"""
```

Ahora combinemos todas estas secciones en una única cadena para usar como nuestro prompt.

```python
TASK_SPECIFIC_INSTRUCTIONS = ' '.join([
   STATIC_GREETINGS_AND_GENERAL,
   STATIC_CAR_INSURANCE,
   STATIC_ELECTRIC_CAR_INSURANCE,
   EXAMPLES,
   ADDITIONAL_GUARDRAILS,
])
```

### Agrega capacidades dinámicas y agentivas con uso de herramientas

Claude es capaz de tomar acciones y recuperar información dinámicamente usando la funcionalidad de uso de herramientas del lado del cliente. Comienza listando cualquier herramienta externa o API que el prompt deba utilizar.

Para este ejemplo, comenzaremos con una herramienta para calcular la cotización.

<Tip>Como recordatorio, esta herramienta no realizará el cálculo real, solo señalará a la aplicación que se debe usar una herramienta con los argumentos especificados.</Tip>

Ejemplo de calculadora de cotización de seguros:

```python
TOOLS = [{
  "name": "get_quote",
  "description": "Calculate the insurance quote based on user input. Returned value is per month premium.",
  "input_schema": {
    "type": "object",
    "properties": {
      "make": {"type": "string", "description": "The make of the vehicle."},
      "model": {"type": "string", "description": "The model of the vehicle."},
      "year": {"type": "integer", "description": "The year the vehicle was manufactured."},
      "mileage": {"type": "integer", "description": "The mileage on the vehicle."},
      "driver_age": {"type": "integer", "description": "The age of the primary driver."}
    },
    "required": ["make", "model", "year", "mileage", "driver_age"]
  }
}]

def get_quote(make, model, year, mileage, driver_age):
    """Returns the premium per month in USD"""
    # You can call an http endpoint or a database to get the quote.
    # Here, we simulate a delay of 1 seconds and return a fixed quote of 100.
    time.sleep(1)
    return 100
```

### Despliega tus prompts

Es difícil saber qué tan bien funciona tu prompt sin desplegarlo en un entorno de producción de prueba y [ejecutar evaluaciones](/docs/es/test-and-evaluate/develop-tests) así que construyamos una pequeña aplicación usando nuestro prompt, el SDK de Anthropic, y streamlit para una interfaz de usuario.

En un archivo llamado `chatbot.py`, comienza configurando la clase ChatBot, que encapsulará las interacciones con el SDK de Anthropic.

La clase debe tener dos métodos principales: `generate_message` y `process_user_input`.

```python
from anthropic import Anthropic
from config import IDENTITY, TOOLS, MODEL, get_quote
from dotenv import load_dotenv

load_dotenv()

class ChatBot:
   def __init__(self, session_state):
       self.anthropic = Anthropic()
       self.session_state = session_state

   def generate_message(
       self,
       messages,
       max_tokens,
   ):
       try:
           response = self.anthropic.messages.create(
               model=MODEL,
               system=IDENTITY,
               max_tokens=max_tokens,
               messages=messages,
               tools=TOOLS,
           )
           return response
       except Exception as e:
           return {"error": str(e)}

   def process_user_input(self, user_input):
       self.session_state.messages.append({"role": "user", "content": user_input})

       response_message = self.generate_message(
           messages=self.session_state.messages,
           max_tokens=2048,
       )

       if "error" in response_message:
           return f"An error occurred: {response_message['error']}"

       if response_message.content[-1].type == "tool_use":
           tool_use = response_message.content[-1]
           func_name = tool_use.name
           func_params = tool_use.input
           tool_use_id = tool_use.id

           result = self.handle_tool_use(func_name, func_params)
           self.session_state.messages.append(
               {"role": "assistant", "content": response_message.content}
           )
           self.session_state.messages.append({
               "role": "user",
               "content": [{
                   "type": "tool_result",
                   "tool_use_id": tool_use_id,
                   "content": f"{result}",
               }],
           })

           follow_up_response = self.generate_message(
               messages=self.session_state.messages,
               max_tokens=2048,
           )

           if "error" in follow_up_response:
               return f"An error occurred: {follow_up_response['error']}"

           response_text = follow_up_response.content[0].text
           self.session_state.messages.append(
               {"role": "assistant", "content": response_text}
           )
           return response_text
      
       elif response_message.content[0].type == "text":
           response_text = response_message.content[0].text
           self.session_state.messages.append(
               {"role": "assistant", "content": response_text}
           )
           return response_text
      
       else:
           raise Exception("An error occurred: Unexpected response type")

   def handle_tool_use(self, func_name, func_params):
       if func_name == "get_quote":
           premium = get_quote(**func_params)
           return f"Quote generated: ${premium:.2f} per month"
      
       raise Exception("An unexpected tool was used")
```

### Construye tu interfaz de usuario

Prueba desplegando este código con Streamlit usando un método main. Esta función `main()` configura una interfaz de chat basada en Streamlit.

Haremos esto en un archivo llamado `app.py`

```python
import streamlit as st
from chatbot import ChatBot
from config import TASK_SPECIFIC_INSTRUCTIONS

def main():
   st.title("Chat with Eva, Acme Insurance Company's Assistant🤖")

   if "messages" not in st.session_state:
       st.session_state.messages = [
           {'role': "user", "content": TASK_SPECIFIC_INSTRUCTIONS},
           {'role': "assistant", "content": "Understood"},
       ]

   chatbot = ChatBot(st.session_state)

   # Display user and assistant messages skipping the first two
   for message in st.session_state.messages[2:]:
       # ignore tool use blocks
       if isinstance(message["content"], str):
           with st.chat_message(message["role"]):
               st.markdown(message["content"])

   if user_msg := st.chat_input("Type your message here..."):
       st.chat_message("user").markdown(user_msg)

       with st.chat_message("assistant"):
           with st.spinner("Eva is thinking..."):
               response_placeholder = st.empty()
               full_response = chatbot.process_user_input(user_msg)
               response_placeholder.markdown(full_response)

if __name__ == "__main__":
   main()
```

Ejecuta el programa con:

```
streamlit run app.py
```

### Evalúa tus prompts

El prompting a menudo requiere pruebas y optimización para que esté listo para producción. Para determinar la preparación de tu solución, evalúa el desempeño del chatbot usando un proceso sistemático que combine métodos cuantitativos y cualitativos. Crear una [evaluación empírica fuerte](/docs/es/test-and-evaluate/develop-tests#building-evals-and-test-cases) basada en tus criterios de éxito definidos te permitirá optimizar tus prompts.

<Tip>La [Consola Claude](/dashboard) ahora presenta una herramienta de Evaluación que te permite probar tus prompts bajo varios escenarios.</Tip>

### Mejora el desempeño

En escenarios complejos, puede ser útil considerar estrategias adicionales para mejorar el desempeño más allá de técnicas estándar de [ingeniería de prompts](/docs/es/build-with-claude/prompt-engineering/overview) e [implementación de guardrails](/docs/es/test-and-evaluate/strengthen-guardrails/reduce-hallucinations). Aquí hay algunos escenarios comunes:

#### Reduce latencia de contexto largo con RAG

Cuando se trata de grandes cantidades de contexto estático y dinámico, incluir toda la información en el prompt puede llevar a costos altos, tiempos de respuesta más lentos y alcanzar límites de ventana de contexto. En este escenario, implementar técnicas de Generación Aumentada por Recuperación (RAG) puede mejorar significativamente el desempeño y la eficiencia.

Al usar [modelos de embedding como Voyage](/docs/es/build-with-claude/embeddings) para convertir información en representaciones vectoriales, puedes crear un sistema más escalable y responsivo. Este enfoque permite la recuperación dinámica de información relevante basada en la consulta actual, en lugar de incluir todo el contexto posible en cada prompt.

Implementar RAG para casos de uso de soporte [receta RAG](https://github.com/anthropics/anthropic-cookbook/blob/82675c124e1344639b2a875aa9d3ae854709cd83/skills/classification/guide.ipynb) ha demostrado aumentar la precisión, reducir tiempos de respuesta y reducir costos de API en sistemas con requisitos de contexto extensos.

#### Integra datos en tiempo real con uso de herramientas

Cuando se trata de consultas que requieren información en tiempo real, como saldos de cuenta o detalles de póliza, los enfoques RAG basados en embedding no son suficientes. En su lugar, puedes aprovechar el uso de herramientas para mejorar significativamente la capacidad de tu chatbot de proporcionar respuestas precisas y en tiempo real. Por ejemplo, puedes usar el uso de herramientas para buscar información del cliente, recuperar detalles de pedidos y cancelar pedidos en nombre del cliente.

Este enfoque, [descrito en nuestra receta de uso de herramientas: agente de servicio al cliente](https://github.com/anthropics/anthropic-cookbook/blob/main/tool_use/customer_service_agent.ipynb), te permite integrar sin problemas datos en vivo en las respuestas de Claude y proporcionar una experiencia de cliente más personalizada y eficiente.

#### Fortalece guardrails de entrada y salida

Al desplegar un chatbot, especialmente en escenarios de servicio al cliente, es crucial prevenir riesgos asociados con mal uso, consultas fuera de alcance y respuestas inapropiadas. Aunque Claude es inherentemente resiliente a tales escenarios, aquí hay pasos adicionales para fortalecer los guardrails de tu chatbot:

- [Reduce alucinación](/docs/es/test-and-evaluate/strengthen-guardrails/reduce-hallucinations): Implementa mecanismos de verificación de hechos y [citas](https://github.com/anthropics/anthropic-cookbook/blob/main/misc/using_citations.ipynb) para fundamentar respuestas en información proporcionada.
- Verifica cruzada la información: Verifica que las respuestas del agente se alineen con las políticas de tu empresa y hechos conocidos.
- Evita compromisos contractuales: Asegúrate de que el agente no haga promesas o entre en acuerdos que no esté autorizado a hacer.
- [Mitiga jailbreaks](/docs/es/test-and-evaluate/strengthen-guardrails/mitigate-jailbreaks): Usa métodos como pantallas de inofensividad y validación de entrada para prevenir que los usuarios exploten vulnerabilidades del modelo, apuntando a generar contenido inapropiado.
- Evita mencionar competidores: Implementa un filtro de mención de competidores para mantener el enfoque de marca y no mencionar productos o servicios de ningún competidor.
- [Mantén a Claude en personaje](/docs/es/test-and-evaluate/strengthen-guardrails/keep-claude-in-character): Previene que Claude cambie su estilo de contexto, incluso durante interacciones largas y complejas.
- Elimina Información de Identificación Personal (PII): A menos que sea explícitamente requerido y autorizado, elimina cualquier PII de las respuestas.

#### Reduce tiempo de respuesta percibido con streaming

Cuando se trata de respuestas potencialmente largas, implementar streaming puede mejorar significativamente el compromiso y la satisfacción del usuario. En este escenario, los usuarios reciben la respuesta progresivamente en lugar de esperar a que se genere la respuesta completa.

Aquí está cómo implementar streaming:
1. Usa la [API de Streaming de Anthropic](/docs/es/build-with-claude/streaming) para soportar respuestas de streaming.
2. Configura tu frontend para manejar fragmentos de texto entrantes.
3. Muestra cada fragmento a medida que llega, simulando escritura en tiempo real.
4. Implementa un mecanismo para guardar la respuesta completa, permitiendo a los usuarios verla si navegan lejos y regresan.

En algunos casos, el streaming permite el uso de modelos más avanzados con latencias base más altas, ya que la visualización progresiva mitiga el impacto de tiempos de procesamiento más largos.

#### Escala tu Chatbot

A medida que la complejidad de tu Chatbot crece, tu arquitectura de aplicación puede evolucionar para coincidir. Antes de agregar más capas a tu arquitectura, considera las siguientes opciones menos exhaustivas:

- Asegúrate de que estés aprovechando al máximo tus prompts y optimizando a través de ingeniería de prompts. Usa nuestras [guías de ingeniería de prompts](/docs/es/build-with-claude/prompt-engineering/overview) para escribir los prompts más efectivos.
- Agrega [herramientas](/docs/es/build-with-claude/tool-use) adicionales al prompt (que pueden incluir [cadenas de prompts](/docs/es/build-with-claude/prompt-engineering/chain-prompts)) y ve si puedes lograr la funcionalidad requerida.

Si tu Chatbot maneja tareas increíblemente variadas, podrías querer considerar agregar un [clasificador de intención separado](https://github.com/anthropics/anthropic-cookbook/blob/main/skills/classification/guide.ipynb) para enrutar la consulta inicial del cliente. Para la aplicación existente, esto implicaría crear un árbol de decisión que enrutaría consultas de clientes a través del clasificador y luego a conversaciones especializadas (con su propio conjunto de herramientas y system prompts). Nota, este método requiere una llamada adicional a Claude que puede aumentar la latencia.

### Integra Claude en tu flujo de trabajo de soporte

Mientras que nuestros ejemplos se han enfocado en funciones Python invocables dentro de un entorno Streamlit, desplegar Claude para chatbot de soporte en tiempo real requiere un servicio API.

Aquí está cómo puedes abordar esto:

1. Crea un envoltorio API: Desarrolla un envoltorio API simple alrededor de tu función de clasificación. Por ejemplo, puedes usar Flask API o Fast API para envolver tu código en un Servicio HTTP. Tu servicio HTTP podría aceptar la entrada del usuario y devolver la respuesta del Asistente en su totalidad. Así, tu servicio podría tener las siguientes características:
   - Eventos Enviados por el Servidor (SSE): SSE permite el streaming en tiempo real de respuestas del servidor al cliente. Esto es crucial para proporcionar una experiencia suave e interactiva cuando se trabaja con LLMs.
   - Caché: Implementar caché puede mejorar significativamente los tiempos de respuesta y reducir llamadas API innecesarias.
   - Retención de contexto: Mantener contexto cuando un usuario navega lejos y regresa es importante para la continuidad en conversaciones.

2. Construye una interfaz web: Implementa una interfaz de usuario web amigable para interactuar con el agente impulsado por Claude.

<CardGroup cols={2}>
  <Card title="Retrieval Augmented Generation (RAG) cookbook" icon="link" href="https://github.com/anthropics/anthropic-cookbook/blob/main/skills/retrieval_augmented_generation/guide.ipynb">
    Visita nuestra receta de cookbook RAG para más código de ejemplo y orientación detallada.
  </Card>
  <Card title="Citations cookbook" icon="link" href="https://github.com/anthropics/anthropic-cookbook/blob/main/misc/using_citations.ipynb">
    Explora nuestra receta de cookbook Citations para cómo asegurar precisión y explicabilidad de la información.
  </Card>
</CardGroup>