# Compatibilidad con OpenAI SDK

Anthropic proporciona una capa de compatibilidad que te permite usar OpenAI SDK para probar Claude API. Con algunos cambios de código, puedes evaluar rápidamente las capacidades del modelo Claude.

---

<Note>
Esta capa de compatibilidad está principalmente destinada a probar y comparar capacidades de modelos, y no se considera una solución a largo plazo o lista para producción para la mayoría de casos de uso. Aunque tenemos la intención de mantenerla completamente funcional y no hacer cambios que rompan la compatibilidad, nuestra prioridad es la confiabilidad y efectividad de [Claude API](/docs/es/api/overview).

Para más información sobre limitaciones de compatibilidad conocidas, consulta [Limitaciones importantes de compatibilidad con OpenAI](#important-openai-compatibility-limitations).

Si encuentras algún problema con la función de compatibilidad con OpenAI SDK, por favor cuéntanos [aquí](https://forms.gle/oQV4McQNiuuNbz9n8).
</Note>

<Tip>
Para la mejor experiencia y acceso al conjunto completo de características de Claude API ([procesamiento de PDF](/docs/es/build-with-claude/pdf-support), [citas](/docs/es/build-with-claude/citations), [pensamiento extendido](/docs/es/build-with-claude/extended-thinking), y [almacenamiento en caché de prompts](/docs/es/build-with-claude/prompt-caching)), recomendamos usar la [Claude API](/docs/es/api/overview) nativa.
</Tip>

## Comenzar con OpenAI SDK

Para usar la función de compatibilidad con OpenAI SDK, necesitarás:

1. Usar un OpenAI SDK oficial
2. Cambiar lo siguiente
   * Actualiza tu URL base para que apunte a Claude API
   * Reemplaza tu clave API con una [clave de Claude API](/settings/keys)
   * Actualiza el nombre de tu modelo para usar un [modelo Claude](/docs/es/about-claude/models/overview)
3. Revisa la documentación a continuación para ver qué características son compatibles

### Ejemplo de inicio rápido

<CodeGroup>
    ```python Python
    from openai import OpenAI

    client = OpenAI(
        api_key="ANTHROPIC_API_KEY",  # Your Claude API key
        base_url="https://api.anthropic.com/v1/"  # the Claude API endpoint
    )

    response = client.chat.completions.create(
        model="claude-sonnet-4-5", # Anthropic model name
        messages=[
            {"role": "system", "content": "You are a helpful assistant."},
            {"role": "user", "content": "Who are you?"}
        ],
    )

    print(response.choices[0].message.content)
    ```
    
    ```typescript TypeScript
    import OpenAI from 'openai';

    const openai = new OpenAI({
        apiKey: "ANTHROPIC_API_KEY",   // Your Claude API key
        baseURL: "https://api.anthropic.com/v1/",  // Claude API endpoint
    });

    const response = await openai.chat.completions.create({
        messages: [
            { role: "user", content: "Who are you?" }
        ],
        model: "claude-sonnet-4-5", // Claude model name
    });

    console.log(response.choices[0].message.content);
    ```
</CodeGroup>

## Limitaciones importantes de compatibilidad con OpenAI

#### Comportamiento de API

Aquí están las diferencias más sustanciales respecto a usar OpenAI:

* El parámetro `strict` para llamadas de función se ignora, lo que significa que el JSON de uso de herramientas no está garantizado que siga el esquema proporcionado. Para conformidad de esquema garantizada, usa la [Claude API nativa con Structured Outputs](/docs/es/build-with-claude/structured-outputs).
* La entrada de audio no es compatible; simplemente se ignorará y se eliminará de la entrada
* El almacenamiento en caché de prompts no es compatible, pero sí lo es en [Anthropic SDK](/docs/es/api/client-sdks)
* Los mensajes del sistema/desarrollador se elevan y se concatenan al principio de la conversación, ya que Anthropic solo admite un único mensaje de sistema inicial.

La mayoría de campos no compatibles se ignoran silenciosamente en lugar de producir errores. Todos estos se documentan a continuación.

#### Consideraciones de calidad de salida

Si has hecho muchos ajustes a tu prompt, es probable que esté bien sintonizado específicamente para OpenAI. Considera usar nuestro [mejorador de prompts en Claude Console](/dashboard) como punto de partida.

#### Elevación de mensajes del sistema / Desarrollador

La mayoría de las entradas al OpenAI SDK se asignan claramente directamente a los parámetros de la API de Anthropic, pero una diferencia distinta es el manejo de prompts del sistema / desarrollador. Estos dos prompts pueden colocarse en toda una conversación de chat a través de OpenAI. Dado que Anthropic solo admite un mensaje de sistema inicial, tomamos todos los mensajes del sistema/desarrollador y los concatenamos juntos con una única nueva línea (`\n`) entre ellos. Esta cadena completa se proporciona como un único mensaje de sistema al inicio de los mensajes.

#### Soporte de pensamiento extendido

Puedes habilitar capacidades de [pensamiento extendido](/docs/es/build-with-claude/extended-thinking) agregando el parámetro `thinking`. Aunque esto mejorará el razonamiento de Claude para tareas complejas, OpenAI SDK no devolverá el proceso de pensamiento detallado de Claude. Para características completas de pensamiento extendido, incluyendo acceso a la salida de razonamiento paso a paso de Claude, usa la Claude API nativa.

<CodeGroup>
    ```python Python
    response = client.chat.completions.create(
        model="claude-sonnet-4-5",
        messages=...,
        extra_body={
            "thinking": { "type": "enabled", "budget_tokens": 2000 }
        }
    )
    ```
    
    ```typescript TypeScript
    const response = await openai.chat.completions.create({
        messages: [
            { role: "user", content: "Who are you?" }
        ],
        model: "claude-sonnet-4-5",
        // @ts-expect-error
        thinking: { type: "enabled", budget_tokens: 2000 }
    });

    ```
</CodeGroup>

## Límites de velocidad

Los límites de velocidad siguen los [límites estándar](/docs/es/api/rate-limits) de Anthropic para el endpoint `/v1/messages`.

## Soporte detallado de API compatible con OpenAI
### Campos de solicitud
#### Campos simples
| Campo | Estado de compatibilidad |
|--------|----------------|
| `model` | Usar nombres de modelos Claude |
| `max_tokens` | Completamente compatible |
| `max_completion_tokens` | Completamente compatible |
| `stream` | Completamente compatible |
| `stream_options` | Completamente compatible |
| `top_p` | Completamente compatible |
| `parallel_tool_calls` | Completamente compatible |
| `stop` | Todas las secuencias de parada sin espacios en blanco funcionan |
| `temperature` | Entre 0 y 1 (inclusive). Los valores mayores que 1 se limitan a 1. |
| `n` | Debe ser exactamente 1 |
| `logprobs` | Ignorado |
| `metadata` | Ignorado |
| `response_format` | Ignorado. Para salida JSON, usa [Structured Outputs](/docs/es/build-with-claude/structured-outputs) con la Claude API nativa |
| `prediction` | Ignorado |
| `presence_penalty` | Ignorado |
| `frequency_penalty` | Ignorado |
| `seed` | Ignorado |
| `service_tier` | Ignorado |
| `audio` | Ignorado |
| `logit_bias` | Ignorado |
| `store` | Ignorado |
| `user` | Ignorado |
| `modalities` | Ignorado |
| `top_logprobs` | Ignorado |
| `reasoning_effort` | Ignorado |

#### Campos `tools` / `functions`
<section title="Mostrar campos">

<Tabs>
<Tab title="Tools">
Campos `tools[n].function`
| Campo        | Estado de compatibilidad         |
|--------------|-----------------|
| `name`       | Completamente compatible |
| `description`| Completamente compatible |
| `parameters` | Completamente compatible |
| `strict`     | Ignorado. Usa [Structured Outputs](/docs/es/build-with-claude/structured-outputs) con Claude API nativa para validación de esquema estricta |
</Tab>
<Tab title="Functions">

Campos `functions[n]`
<Info>
OpenAI ha descontinuado el campo `functions` y sugiere usar `tools` en su lugar.
</Info>
| Campo        | Estado de compatibilidad         |
|--------------|-----------------|
| `name`       | Completamente compatible |
| `description`| Completamente compatible |
| `parameters` | Completamente compatible |
| `strict`     | Ignorado. Usa [Structured Outputs](/docs/es/build-with-claude/structured-outputs) con Claude API nativa para validación de esquema estricta |
</Tab>
</Tabs>

</section>

#### Campos del array `messages`
<section title="Mostrar campos">

<Tabs>
<Tab title="Developer role">
Campos para `messages[n].role == "developer"`
<Info>
Los mensajes del desarrollador se elevan al principio de la conversación como parte del mensaje de sistema inicial
</Info>
| Campo | Estado de compatibilidad |
|-------|---------|
| `content` | Completamente compatible, pero elevado |
| `name` | Ignorado |

</Tab>
<Tab title="System role">
Campos para `messages[n].role == "system"`

<Info>
Los mensajes del sistema se elevan al principio de la conversación como parte del mensaje de sistema inicial
</Info>
| Campo | Estado de compatibilidad |
|-------|---------|
| `content` | Completamente compatible, pero elevado |
| `name` | Ignorado |

</Tab>
<Tab title="User role">
Campos para `messages[n].role == "user"`

| Campo | Variante | Sub-campo | Estado de compatibilidad |
|-------|---------|-----------|----------------|
| `content` | `string` | | Completamente compatible |
| | `array`, `type == "text"` | | Completamente compatible |
| | `array`, `type == "image_url"` | `url` | Completamente compatible |
| | | `detail` | Ignorado |
| | `array`, `type == "input_audio"` | | Ignorado |
| | `array`, `type == "file"` | | Ignorado |
| `name` | | | Ignorado |

</Tab>

<Tab title="Assistant role">
Campos para `messages[n].role == "assistant"`
| Campo | Variante | Estado de compatibilidad |
|-------|---------|----------------|
| `content` | `string` | Completamente compatible |
| | `array`, `type == "text"` | Completamente compatible |
| | `array`, `type == "refusal"` | Ignorado |
| `tool_calls` | | Completamente compatible |
| `function_call` | | Completamente compatible |
| `audio` | | Ignorado |
| `refusal` | | Ignorado |

</Tab>

<Tab title="Tool role">
Campos para `messages[n].role == "tool"`
| Campo | Variante | Estado de compatibilidad |
|-------|---------|----------------|
| `content` | `string` | Completamente compatible |
| | `array`, `type == "text"` | Completamente compatible |
| `tool_call_id` | | Completamente compatible |
| `tool_choice` | | Completamente compatible |
| `name` | | Ignorado |
</Tab>

<Tab title="Function role">
Campos para `messages[n].role == "function"`
| Campo | Variante | Estado de compatibilidad |
|-------|---------|----------------|
| `content` | `string` | Completamente compatible |
| | `array`, `type == "text"` | Completamente compatible |
| `tool_choice` | | Completamente compatible |
| `name` | | Ignorado |
</Tab>
</Tabs>

</section>

### Campos de respuesta

| Campo | Estado de compatibilidad |
|---------------------------|----------------|
| `id` | Completamente compatible |
| `choices[]` | Siempre tendrá una longitud de 1 |
| `choices[].finish_reason` | Completamente compatible |
| `choices[].index` | Completamente compatible |
| `choices[].message.role` | Completamente compatible |
| `choices[].message.content` | Completamente compatible |
| `choices[].message.tool_calls` | Completamente compatible |
| `object` | Completamente compatible |
| `created` | Completamente compatible |
| `model` | Completamente compatible |
| `finish_reason` | Completamente compatible |
| `content` | Completamente compatible |
| `usage.completion_tokens` | Completamente compatible |
| `usage.prompt_tokens` | Completamente compatible |
| `usage.total_tokens` | Completamente compatible |
| `usage.completion_tokens_details` | Siempre vacío |
| `usage.prompt_tokens_details` | Siempre vacío |
| `choices[].message.refusal` | Siempre vacío |
| `choices[].message.audio` | Siempre vacío |
| `logprobs` | Siempre vacío |
| `service_tier` | Siempre vacío |
| `system_fingerprint` | Siempre vacío |

### Compatibilidad de mensajes de error

La capa de compatibilidad mantiene formatos de error consistentes con la API de OpenAI. Sin embargo, los mensajes de error detallados no serán equivalentes. Recomendamos usar solo los mensajes de error para registro y depuración.

### Compatibilidad de encabezados

Aunque OpenAI SDK gestiona automáticamente los encabezados, aquí está la lista completa de encabezados compatibles con Claude API para desarrolladores que necesiten trabajar con ellos directamente.

| Encabezado | Estado de compatibilidad |
|---------|----------------|
| `x-ratelimit-limit-requests` | Completamente compatible |
| `x-ratelimit-limit-tokens` | Completamente compatible |
| `x-ratelimit-remaining-requests` | Completamente compatible |
| `x-ratelimit-remaining-tokens` | Completamente compatible |
| `x-ratelimit-reset-requests` | Completamente compatible |
| `x-ratelimit-reset-tokens` | Completamente compatible |
| `retry-after` | Completamente compatible |
| `request-id` | Completamente compatible |
| `openai-version` | Siempre `2020-10-01` |
| `authorization` | Completamente compatible |
| `openai-processing-ms` | Siempre vacío |