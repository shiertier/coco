# Glosario

Estos conceptos no son únicos de los modelos de lenguaje de Anthropic, pero presentamos un breve resumen de términos clave a continuación.

---

## Ventana de contexto

La "ventana de contexto" se refiere a la cantidad de texto que un modelo de lenguaje puede revisar y referenciar al generar nuevo texto. Esto es diferente del gran corpus de datos en el que se entrenó el modelo de lenguaje, y en su lugar representa una "memoria de trabajo" para el modelo. Una ventana de contexto más grande permite al modelo entender y responder a prompts más complejos y extensos, mientras que una ventana de contexto más pequeña puede limitar la capacidad del modelo para manejar prompts más largos o mantener coherencia durante conversaciones extendidas.

Consulta nuestra [guía para entender las ventanas de contexto](/docs/es/build-with-claude/context-windows) para aprender más.

## Ajuste fino

El ajuste fino es el proceso de entrenar adicionalmente un modelo de lenguaje preentrenado usando datos adicionales. Esto hace que el modelo comience a representar e imitar los patrones y características del conjunto de datos de ajuste fino. Claude no es un modelo de lenguaje básico; ya ha sido ajustado finamente para ser un asistente útil. Nuestra API actualmente no ofrece ajuste fino, pero por favor pregunta a tu contacto de Anthropic si estás interesado en explorar esta opción. El ajuste fino puede ser útil para adaptar un modelo de lenguaje a un dominio específico, tarea o estilo de escritura, pero requiere consideración cuidadosa de los datos de ajuste fino y el impacto potencial en el rendimiento y sesgos del modelo.

## HHH

Estas tres H representan los objetivos de Anthropic para asegurar que Claude sea beneficioso para la sociedad:

- Una IA **útil** intentará realizar la tarea o responder la pregunta planteada lo mejor que pueda, proporcionando información relevante y útil.
- Una IA **honesta** dará información precisa, y no alucinará o confabulará. Reconocerá sus limitaciones e incertidumbres cuando sea apropiado.
- Una IA **inofensiva** no será ofensiva o discriminatoria, y cuando se le pida ayudar en un acto peligroso o poco ético, la IA debe rechazar cortésmente y explicar por qué no puede cumplir.

## Latencia

La latencia, en el contexto de IA generativa y modelos de lenguaje grandes, se refiere al tiempo que toma al modelo responder a un prompt dado. Es el retraso entre enviar un prompt y recibir la salida generada. Una latencia menor indica tiempos de respuesta más rápidos, lo cual es crucial para aplicaciones en tiempo real, chatbots y experiencias interactivas. Los factores que pueden afectar la latencia incluyen el tamaño del modelo, capacidades de hardware, condiciones de red, y la complejidad del prompt y la respuesta generada.

## LLM

Los modelos de lenguaje grandes (LLMs) son modelos de lenguaje de IA con muchos parámetros que son capaces de realizar una variedad de tareas sorprendentemente útiles. Estos modelos se entrenan en vastas cantidades de datos de texto y pueden generar texto similar al humano, responder preguntas, resumir información, y más. Claude es un asistente conversacional basado en un modelo de lenguaje grande que ha sido ajustado finamente y entrenado usando RLHF para ser más útil, honesto e inofensivo.

## MCP (Protocolo de Contexto del Modelo)

El Protocolo de Contexto del Modelo (MCP) es un protocolo abierto que estandariza cómo las aplicaciones proporcionan contexto a los LLMs. Como un puerto USB-C para aplicaciones de IA, MCP proporciona una forma unificada de conectar modelos de IA a diferentes fuentes de datos y herramientas. MCP permite a los sistemas de IA mantener contexto consistente a través de interacciones y acceder a recursos externos de manera estandarizada. Consulta nuestra [documentación de MCP](/docs/es/mcp) para aprender más.

## Conector MCP

El conector MCP es una característica que permite a los usuarios de API conectarse a servidores MCP directamente desde la API de Mensajes sin construir un cliente MCP. Esto permite integración perfecta con herramientas y servicios compatibles con MCP a través de la API de Claude. El conector MCP soporta características como llamadas de herramientas y está disponible en beta pública. Consulta nuestra [documentación del conector MCP](/docs/es/agents-and-tools/mcp-connector) para aprender más.

## Preentrenamiento

El preentrenamiento es el proceso inicial de entrenar modelos de lenguaje en un gran corpus de texto sin etiquetar. En el caso de Claude, los modelos de lenguaje autorregresivos (como el modelo subyacente de Claude) se preentrenan para predecir la siguiente palabra, dado el contexto previo de texto en el documento. Estos modelos preentrenados no son inherentemente buenos respondiendo preguntas o siguiendo instrucciones, y a menudo requieren habilidad profunda en ingeniería de prompts para obtener comportamientos deseados. El ajuste fino y RLHF se usan para refinar estos modelos preentrenados, haciéndolos más útiles para una amplia gama de tareas.

## RAG (Generación aumentada por recuperación)

La generación aumentada por recuperación (RAG) es una técnica que combina recuperación de información con generación de modelos de lenguaje para mejorar la precisión y relevancia del texto generado, y para fundamentar mejor la respuesta del modelo en evidencia. En RAG, un modelo de lenguaje se aumenta con una base de conocimiento externa o un conjunto de documentos que se pasa a la ventana de contexto. Los datos se recuperan en tiempo de ejecución cuando se envía una consulta al modelo, aunque el modelo mismo no necesariamente recupera los datos (pero puede con [uso de herramientas](/docs/es/agents-and-tools/tool-use/overview) y una función de recuperación). Al generar texto, primero se debe recuperar información relevante de la base de conocimiento basada en el prompt de entrada, y luego pasarla al modelo junto con la consulta original. El modelo usa esta información para guiar la salida que genera. Esto permite al modelo acceder y utilizar información más allá de sus datos de entrenamiento, reduciendo la dependencia en memorización y mejorando la precisión factual del texto generado. RAG puede ser particularmente útil para tareas que requieren información actualizada, conocimiento específico del dominio, o citación explícita de fuentes. Sin embargo, la efectividad de RAG depende de la calidad y relevancia de la base de conocimiento externa y el conocimiento que se recupera en tiempo de ejecución.

## RLHF

El Aprendizaje por Refuerzo desde Retroalimentación Humana (RLHF) es una técnica usada para entrenar un modelo de lenguaje preentrenado para comportarse de maneras que son consistentes con las preferencias humanas. Esto puede incluir ayudar al modelo a seguir instrucciones más efectivamente o actuar más como un chatbot. La retroalimentación humana consiste en clasificar un conjunto de dos o más textos de ejemplo, y el proceso de aprendizaje por refuerzo alienta al modelo a preferir salidas que son similares a las clasificadas más alto. Claude ha sido entrenado usando RLHF para ser un asistente más útil. Para más detalles, puedes leer [el artículo de Anthropic sobre el tema](https://arxiv.org/abs/2204.05862).

## Temperatura

La temperatura es un parámetro que controla la aleatoriedad de las predicciones de un modelo durante la generación de texto. Temperaturas más altas llevan a salidas más creativas y diversas, permitiendo múltiples variaciones en fraseología y, en el caso de ficción, variación en respuestas también. Temperaturas más bajas resultan en salidas más conservadoras y determinísticas que se adhieren a la fraseología y respuestas más probables. Ajustar la temperatura permite a los usuarios alentar a un modelo de lenguaje a explorar elecciones y secuencias de palabras raras, poco comunes o sorprendentes, en lugar de solo seleccionar las predicciones más probables.

## TTFT (Tiempo al primer token)

El Tiempo al Primer Token (TTFT) es una métrica de rendimiento que mide el tiempo que toma a un modelo de lenguaje generar el primer token de su salida después de recibir un prompt. Es un indicador importante de la capacidad de respuesta del modelo y es particularmente relevante para aplicaciones interactivas, chatbots y sistemas en tiempo real donde los usuarios esperan retroalimentación inicial rápida. Un TTFT menor indica que el modelo puede comenzar a generar una respuesta más rápido, proporcionando una experiencia de usuario más fluida y atractiva. Los factores que pueden influir en TTFT incluyen el tamaño del modelo, capacidades de hardware, condiciones de red, y la complejidad del prompt.

## Tokens

Los tokens son las unidades individuales más pequeñas de un modelo de lenguaje, y pueden corresponder a palabras, subpalabras, caracteres, o incluso bytes (en el caso de Unicode). Para Claude, un token representa aproximadamente 3.5 caracteres en inglés, aunque el número exacto puede variar dependiendo del idioma usado. Los tokens típicamente están ocultos cuando se interactúa con modelos de lenguaje a nivel de "texto" pero se vuelven relevantes cuando se examinan las entradas y salidas exactas de un modelo de lenguaje. Cuando se proporciona texto a Claude para evaluar, el texto (consistente en una serie de caracteres) se codifica en una serie de tokens para que el modelo procese. Tokens más grandes permiten eficiencia de datos durante inferencia y preentrenamiento (y se utilizan cuando es posible), mientras que tokens más pequeños permiten a un modelo manejar palabras poco comunes o nunca antes vistas. La elección del método de tokenización puede impactar el rendimiento del modelo, tamaño de vocabulario, y capacidad para manejar palabras fuera del vocabulario.