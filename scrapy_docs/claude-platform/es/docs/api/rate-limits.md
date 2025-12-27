# Límites de velocidad

Para mitigar el abuso y gestionar la capacidad en nuestra API, hemos implementado límites sobre cuánto puede usar una organización la API de Claude.

---

Tenemos dos tipos de límites:

1. **Límites de gasto** establecen un costo mensual máximo que una organización puede incurrir por el uso de la API.
2. **Límites de velocidad** establecen el número máximo de solicitudes de API que una organización puede hacer durante un período de tiempo definido.

Aplicamos límites configurados por el servicio a nivel de organización, pero también puede establecer límites configurables por el usuario para los espacios de trabajo de su organización.

Estos límites se aplican tanto al uso de Tier Estándar como de Tier Prioritario. Para obtener más información sobre Tier Prioritario, que ofrece niveles de servicio mejorados a cambio de gasto comprometido, consulte [Niveles de servicio](/docs/es/api/service-tiers).

## Acerca de nuestros límites

* Los límites están diseñados para prevenir el abuso de la API, mientras se minimiza el impacto en los patrones de uso común de los clientes.
* Los límites se definen por **nivel de uso**, donde cada nivel está asociado con un conjunto diferente de límites de gasto y velocidad.
* Su organización aumentará automáticamente de nivel a medida que alcance ciertos umbrales mientras usa la API.
  Los límites se establecen a nivel de organización. Puede ver los límites de su organización en la [página de Límites](/settings/limits) en la [Consola Claude](/).
* Puede alcanzar límites de velocidad en intervalos de tiempo más cortos. Por ejemplo, una velocidad de 60 solicitudes por minuto (RPM) puede aplicarse como 1 solicitud por segundo. Ráfagas cortas de solicitudes con alto volumen pueden superar el límite de velocidad y resultar en errores de límite de velocidad.
* Los límites descritos a continuación son nuestros límites de nivel estándar. Si busca límites más altos y personalizados o Tier Prioritario para niveles de servicio mejorados, póngase en contacto con ventas a través de la [Consola Claude](/settings/limits).
* Utilizamos el [algoritmo de cubo de tokens](https://en.wikipedia.org/wiki/Token_bucket) para hacer limitación de velocidad. Esto significa que su capacidad se repone continuamente hasta su límite máximo, en lugar de restablecerse en intervalos fijos.
* Todos los límites descritos aquí representan el uso máximo permitido, no mínimos garantizados. Estos límites están destinados a reducir el gasto excesivo involuntario y garantizar una distribución justa de recursos entre usuarios.

## Límites de gasto

Cada nivel de uso tiene un límite sobre cuánto puede gastar en la API cada mes calendario. Una vez que alcance el límite de gasto de su nivel, hasta que califique para el siguiente nivel, deberá esperar hasta el próximo mes para poder usar la API nuevamente.

Para calificar para el siguiente nivel, debe cumplir con un requisito de depósito. Para minimizar el riesgo de sobrefondear su cuenta, no puede depositar más que su límite de gasto mensual.

### Requisitos para avanzar de nivel
<table>
  <thead>
    <tr>
      <th>Nivel de uso</th>
      <th>Compra de crédito</th>
      <th>Compra máxima de crédito</th>
    </tr>
  </thead>
  <tbody>
    <tr>
      <td>Nivel 1</td>
      <td>\$5</td>
      <td>\$100</td>
    </tr>
    <tr>
      <td>Nivel 2</td>
      <td>\$40</td>
      <td>\$500</td>
    </tr>
    <tr>
      <td>Nivel 3</td>
      <td>\$200</td>
      <td>\$1,000</td>
    </tr>
    <tr>
      <td>Nivel 4</td>
      <td>\$400</td>
      <td>\$5,000</td>
    </tr>
    <tr>
      <td>Facturación mensual</td>
      <td>N/A</td>
      <td>N/A</td>
    </tr>
  </tbody>
</table>

<Note>
**Compra de crédito** muestra las compras de crédito acumuladas (excluyendo impuestos) requeridas para avanzar a ese nivel. Avanza inmediatamente al alcanzar el umbral.

**Compra máxima de crédito** limita la cantidad máxima que puede agregar a su cuenta en una sola transacción para evitar el sobrefondeo de la cuenta.
</Note>

## Límites de velocidad

Nuestros límites de velocidad para la API de Mensajes se miden en solicitudes por minuto (RPM), tokens de entrada por minuto (ITPM) y tokens de salida por minuto (OTPM) para cada clase de modelo.
Si excede cualquiera de los límites de velocidad, obtendrá un [error 429](/docs/es/api/errors) que describe qué límite de velocidad se excedió, junto con un encabezado `retry-after` que indica cuánto tiempo esperar.

<Note>
También puede encontrar errores 429 debido a límites de aceleración en la API si su organización tiene un aumento brusco en el uso. Para evitar alcanzar límites de aceleración, aumente su tráfico gradualmente y mantenga patrones de uso consistentes.
</Note>

### ITPM consciente de caché

Muchos proveedores de API utilizan un límite combinado de "tokens por minuto" (TPM) que puede incluir todos los tokens, tanto en caché como sin caché, entrada y salida. **Para la mayoría de los modelos de Claude, solo los tokens de entrada sin caché cuentan hacia sus límites de velocidad ITPM.** Esta es una ventaja clave que hace que nuestros límites de velocidad sean efectivamente más altos de lo que podrían parecer inicialmente.

Los límites de velocidad ITPM se estiman al principio de cada solicitud, y la estimación se ajusta durante la solicitud para reflejar el número real de tokens de entrada utilizados.

Esto es lo que cuenta hacia ITPM:
- `input_tokens` (tokens después del último punto de ruptura de caché) ✓ **Cuentan hacia ITPM**
- `cache_creation_input_tokens` (tokens que se escriben en caché) ✓ **Cuentan hacia ITPM**
- `cache_read_input_tokens` (tokens leídos del caché) ✗ **NO cuentan hacia ITPM** para la mayoría de los modelos

<Note>
El campo `input_tokens` solo representa tokens que aparecen **después de su último punto de ruptura de caché**, no todos los tokens de entrada en su solicitud. Para calcular los tokens de entrada totales:

```
total_input_tokens = cache_read_input_tokens + cache_creation_input_tokens + input_tokens
```

Esto significa que cuando tiene contenido en caché, `input_tokens` será típicamente mucho más pequeño que su entrada total. Por ejemplo, con un documento en caché de 200K tokens y una pregunta de usuario de 50 tokens, vería `input_tokens: 50` aunque la entrada total sea 200,050 tokens.

Para propósitos de límite de velocidad en la mayoría de los modelos, solo `input_tokens` + `cache_creation_input_tokens` cuentan hacia su límite ITPM, haciendo que [almacenamiento en caché de indicaciones](/docs/es/build-with-claude/prompt-caching) sea una forma efectiva de aumentar su rendimiento efectivo.
</Note>

**Ejemplo**: Con un límite ITPM de 2,000,000 y una tasa de acierto de caché del 80%, podría procesar efectivamente 10,000,000 tokens de entrada totales por minuto (2M sin caché + 8M en caché), ya que los tokens en caché no cuentan hacia su límite de velocidad.

<Note>
Algunos modelos más antiguos (marcados con † en las tablas de límites de velocidad a continuación) también cuentan `cache_read_input_tokens` hacia los límites de velocidad ITPM.

Para todos los modelos sin el marcador †, los tokens de entrada en caché no cuentan hacia los límites de velocidad y se facturan a una tasa reducida (10% del precio del token de entrada base). Esto significa que puede lograr un rendimiento efectivo significativamente más alto utilizando [almacenamiento en caché de indicaciones](/docs/es/build-with-claude/prompt-caching).
</Note>

<Tip>
**Maximice sus límites de velocidad con almacenamiento en caché de indicaciones**

Para aprovechar al máximo sus límites de velocidad, use [almacenamiento en caché de indicaciones](/docs/es/build-with-claude/prompt-caching) para contenido repetido como:
- Instrucciones del sistema e indicaciones
- Documentos de contexto grande
- Definiciones de herramientas
- Historial de conversación

Con almacenamiento en caché efectivo, puede aumentar dramáticamente su rendimiento real sin aumentar sus límites de velocidad. Monitoree su tasa de acierto de caché en la [página de Uso](/settings/usage) para optimizar su estrategia de almacenamiento en caché.
</Tip>

Los límites de velocidad OTPM se estiman basándose en `max_tokens` al principio de cada solicitud, y la estimación se ajusta al final de la solicitud para reflejar el número real de tokens de salida utilizados.
Si está alcanzando límites OTPM antes de lo esperado, intente reducir `max_tokens` para aproximar mejor el tamaño de sus finalizaciones.

Los límites de velocidad se aplican por separado para cada modelo; por lo tanto, puede usar diferentes modelos hasta sus límites respectivos simultáneamente.
Puede verificar sus límites de velocidad actuales y comportamiento en la [Consola Claude](/settings/limits).

<Note>
Para solicitudes de contexto largo (>200K tokens) al usar el encabezado beta `context-1m-2025-08-07` con Claude Sonnet 4.x, se aplican límites de velocidad separados. Consulte [Límites de velocidad de contexto largo](#long-context-rate-limits) a continuación.
</Note>

<Tabs>
<Tab title="Nivel 1">
| Modelo                                                                                       | Máximo de solicitudes por minuto (RPM) | Máximo de tokens de entrada por minuto (ITPM) | Máximo de tokens de salida por minuto (OTPM) |
| -------------------------------------------------------------------------------------------- | -------------------------------------- | ---------------------------------------------- | -------------------------------------------- |
| Claude Sonnet 4.x<sup>**</sup>                                                               | 50                                     | 30,000                                         | 8,000                                        |
| Claude Sonnet 3.7 ([deprecated](/docs/es/about-claude/model-deprecations))                   | 50                                     | 20,000                                         | 8,000                                        |
| Claude Haiku 4.5                                                                             | 50                                     | 50,000                                         | 10,000                                       |
| Claude Haiku 3.5 ([deprecated](/docs/es/about-claude/model-deprecations))                    | 50                                     | 50,000<sup>†</sup>                             | 10,000                                       |
| Claude Haiku 3                                                                               | 50                                     | 50,000<sup>†</sup>                             | 10,000                                       |
| Claude Opus 4.x<sup>*</sup>                                                                  | 50                                     | 30,000                                         | 8,000                                        |
| Claude Opus 3 ([deprecated](/docs/es/about-claude/model-deprecations))                      | 50                                     | 20,000<sup>†</sup>                             | 4,000                                        |

</Tab>
<Tab title="Nivel 2">
| Modelo                                                                                       | Máximo de solicitudes por minuto (RPM) | Máximo de tokens de entrada por minuto (ITPM) | Máximo de tokens de salida por minuto (OTPM) |
| -------------------------------------------------------------------------------------------- | -------------------------------------- | ---------------------------------------------- | -------------------------------------------- |
| Claude Sonnet 4.x<sup>**</sup>                                                               | 1,000                                  | 450,000                                        | 90,000                                       |
| Claude Sonnet 3.7 ([deprecated](/docs/es/about-claude/model-deprecations))                   | 1,000                                  | 40,000                                         | 16,000                                       |
| Claude Haiku 4.5                                                                             | 1,000                                  | 450,000                                        | 90,000                                       |
| Claude Haiku 3.5 ([deprecated](/docs/es/about-claude/model-deprecations))                    | 1,000                                  | 100,000<sup>†</sup>                            | 20,000                                       |
| Claude Haiku 3                                                                               | 1,000                                  | 100,000<sup>†</sup>                            | 20,000                                       |
| Claude Opus 4.x<sup>*</sup>                                                                  | 1,000                                  | 450,000                                        | 90,000                                       |
| Claude Opus 3 ([deprecated](/docs/es/about-claude/model-deprecations))                      | 1,000                                  | 40,000<sup>†</sup>                             | 8,000                                        |

</Tab>
<Tab title="Nivel 3">
| Modelo                                                                                       | Máximo de solicitudes por minuto (RPM) | Máximo de tokens de entrada por minuto (ITPM) | Máximo de tokens de salida por minuto (OTPM) |
| -------------------------------------------------------------------------------------------- | -------------------------------------- | ---------------------------------------------- | -------------------------------------------- |
| Claude Sonnet 4.x<sup>**</sup>                                                               | 2,000                                  | 800,000                                        | 160,000                                      |
| Claude Sonnet 3.7 ([deprecated](/docs/es/about-claude/model-deprecations))                   | 2,000                                  | 80,000                                         | 32,000                                       |
| Claude Haiku 4.5                                                                             | 2,000                                  | 1,000,000                                      | 200,000                                      |
| Claude Haiku 3.5 ([deprecated](/docs/es/about-claude/model-deprecations))                    | 2,000                                  | 200,000<sup>†</sup>                            | 40,000                                       |
| Claude Haiku 3                                                                               | 2,000                                  | 200,000<sup>†</sup>                            | 40,000                                       |
| Claude Opus 4.x<sup>*</sup>                                                                  | 2,000                                  | 800,000                                        | 160,000                                      |
| Claude Opus 3 ([deprecated](/docs/es/about-claude/model-deprecations))                      | 2,000                                  | 80,000<sup>†</sup>                             | 16,000                                       |

</Tab>
<Tab title="Nivel 4">
| Modelo                                                                                       | Máximo de solicitudes por minuto (RPM) | Máximo de tokens de entrada por minuto (ITPM) | Máximo de tokens de salida por minuto (OTPM) |
| -------------------------------------------------------------------------------------------- | -------------------------------------- | ---------------------------------------------- | -------------------------------------------- |
| Claude Sonnet 4.x<sup>**</sup>                                                               | 4,000                                  | 2,000,000                                      | 400,000                                      |
| Claude Sonnet 3.7 ([deprecated](/docs/es/about-claude/model-deprecations))                   | 4,000                                  | 200,000                                        | 80,000                                       |
| Claude Haiku 4.5                                                                             | 4,000                                  | 4,000,000                                      | 800,000                                      |
| Claude Haiku 3.5 ([deprecated](/docs/es/about-claude/model-deprecations))                    | 4,000                                  | 400,000<sup>†</sup>                            | 80,000                                       |
| Claude Haiku 3                                                                               | 4,000                                  | 400,000<sup>†</sup>                            | 80,000                                       |
| Claude Opus 4.x<sup>*</sup>                                                                  | 4,000                                  | 2,000,000                                      | 400,000                                      |
| Claude Opus 3 ([deprecated](/docs/es/about-claude/model-deprecations))                      | 4,000                                  | 400,000<sup>†</sup>                            | 80,000                                       |

</Tab>
<Tab title="Personalizado">
Si busca límites más altos para un caso de uso empresarial, póngase en contacto con ventas a través de la [Consola Claude](/settings/limits).
</Tab>
</Tabs>

_<sup>* - El límite de velocidad Opus 4.x es un límite total que se aplica al tráfico combinado en Opus 4, Opus 4.1 y Opus 4.5.</sup>_

_<sup>** - El límite de velocidad Sonnet 4.x es un límite total que se aplica al tráfico combinado en Sonnet 4 y Sonnet 4.5.</sup>_

_<sup>† - El límite cuenta `cache_read_input_tokens` hacia el uso ITPM.</sup>_

### API de lotes de mensajes

La API de lotes de mensajes tiene su propio conjunto de límites de velocidad que se comparten entre todos los modelos. Estos incluyen un límite de solicitudes por minuto (RPM) a todos los puntos finales de la API y un límite en el número de solicitudes de lote que pueden estar en la cola de procesamiento al mismo tiempo. Una "solicitud de lote" aquí se refiere a parte de un lote de mensajes. Puede crear un lote de mensajes que contenga miles de solicitudes de lote, cada una de las cuales cuenta hacia este límite. Una solicitud de lote se considera parte de la cola de procesamiento cuando aún no ha sido procesada exitosamente por el modelo.

<Tabs>
<Tab title="Nivel 1">
| Máximo de solicitudes por minuto (RPM) | Máximo de solicitudes de lote en cola de procesamiento | Máximo de solicitudes de lote por lote |
| -------------------------------------- | ------------------------------------------------------ | -------------------------------------- |
| 50                                     | 100,000                                                | 100,000                                |
</Tab>
<Tab title="Nivel 2">
| Máximo de solicitudes por minuto (RPM) | Máximo de solicitudes de lote en cola de procesamiento | Máximo de solicitudes de lote por lote |
| -------------------------------------- | ------------------------------------------------------ | -------------------------------------- |
| 1,000                                  | 200,000                                                | 100,000                                |
</Tab>
<Tab title="Nivel 3">
| Máximo de solicitudes por minuto (RPM) | Máximo de solicitudes de lote en cola de procesamiento | Máximo de solicitudes de lote por lote |
| -------------------------------------- | ------------------------------------------------------ | -------------------------------------- |
| 2,000                                  | 300,000                                                | 100,000                                |
</Tab>
<Tab title="Nivel 4">
| Máximo de solicitudes por minuto (RPM) | Máximo de solicitudes de lote en cola de procesamiento | Máximo de solicitudes de lote por lote |
| -------------------------------------- | ------------------------------------------------------ | -------------------------------------- |
| 4,000                                  | 500,000                                                | 100,000                                |
</Tab>
<Tab title="Personalizado">
Si busca límites más altos para un caso de uso empresarial, póngase en contacto con ventas a través de la [Consola Claude](/settings/limits).
</Tab>
</Tabs>

### Límites de velocidad de contexto largo

Al usar Claude Sonnet 4 y Sonnet 4.5 con la [ventana de contexto de 1M tokens habilitada](/docs/es/build-with-claude/context-windows#1m-token-context-window), se aplican los siguientes límites de velocidad dedicados a solicitudes que exceden 200K tokens.

<Note>
La ventana de contexto de 1M tokens está actualmente en beta para organizaciones en el nivel de uso 4 y organizaciones con límites de velocidad personalizados. La ventana de contexto de 1M tokens solo está disponible para Claude Sonnet 4 y Sonnet 4.5.
</Note>

<Tabs>
<Tab title="Nivel 4">
| Máximo de tokens de entrada por minuto (ITPM) | Máximo de tokens de salida por minuto (OTPM) |
| --------------------------------------------- | -------------------------------------------- |
| 1,000,000                                     | 200,000                                      |
</Tab>
<Tab title="Personalizado">
Para límites de velocidad de contexto largo personalizados para casos de uso empresarial, póngase en contacto con ventas a través de la [Consola Claude](/settings/limits).
</Tab>
</Tabs>

<Tip>
Para aprovechar al máximo la ventana de contexto de 1M tokens con límites de velocidad, use [almacenamiento en caché de indicaciones](/docs/es/build-with-claude/prompt-caching).
</Tip>

### Monitoreo de sus límites de velocidad en la Consola

Puede monitorear el uso de su límite de velocidad en la página [Uso](/settings/usage) de la [Consola Claude](/).

Además de proporcionar gráficos de tokens y solicitudes, la página de Uso proporciona dos gráficos de límites de velocidad separados. Use estos gráficos para ver cuánto espacio tiene para crecer, cuándo puede estar alcanzando el uso máximo, comprender mejor qué límites de velocidad solicitar, o cómo puede mejorar sus tasas de almacenamiento en caché. Los gráficos visualizan una serie de métricas para un límite de velocidad dado (por ejemplo, por modelo):

- El gráfico **Límite de velocidad - Tokens de entrada** incluye:
  - Máximo horario de tokens de entrada sin caché por minuto
  - Su límite de velocidad actual de tokens de entrada por minuto
  - La tasa de caché para sus tokens de entrada (es decir, el porcentaje de tokens de entrada leídos del caché)
- El gráfico **Límite de velocidad - Tokens de salida** incluye:
  - Máximo horario de tokens de salida por minuto
  - Su límite de velocidad actual de tokens de salida por minuto

## Establecer límites más bajos para espacios de trabajo

Para proteger los espacios de trabajo en su organización del posible uso excesivo, puede establecer límites de gasto y velocidad personalizados por espacio de trabajo.

Ejemplo: Si el límite de su organización es 40,000 tokens de entrada por minuto y 8,000 tokens de salida por minuto, podría limitar un espacio de trabajo a 30,000 tokens totales por minuto. Esto protege otros espacios de trabajo del posible uso excesivo y garantiza una distribución más equitativa de recursos en su organización. Los tokens por minuto no utilizados restantes (o más, si ese espacio de trabajo no usa el límite) están disponibles para que otros espacios de trabajo los usen.

Nota:
- No puede establecer límites en el espacio de trabajo predeterminado.
- Si no se establece, los límites del espacio de trabajo coinciden con el límite de la organización.
- Los límites de toda la organización siempre se aplican, incluso si los límites del espacio de trabajo suman más.
- El soporte para límites de tokens de entrada y salida se agregará a los espacios de trabajo en el futuro.

## Encabezados de respuesta

La respuesta de la API incluye encabezados que muestran el límite de velocidad aplicado, el uso actual y cuándo se restablecerá el límite.

Se devuelven los siguientes encabezados:

| Encabezado                                    | Descripción                                                                                                                                     |
| --------------------------------------------- | ----------------------------------------------------------------------------------------------------------------------------------------------- |
| `retry-after`                                 | El número de segundos a esperar hasta que pueda reintentar la solicitud. Los reintentos anteriores fallarán.                                   |
| `anthropic-ratelimit-requests-limit`          | El número máximo de solicitudes permitidas dentro de cualquier período de límite de velocidad.                                                  |
| `anthropic-ratelimit-requests-remaining`      | El número de solicitudes restantes antes de ser limitado por velocidad.                                                                        |
| `anthropic-ratelimit-requests-reset`          | La hora en que se restablecerá completamente el límite de velocidad de solicitudes, proporcionada en formato RFC 3339.                         |
| `anthropic-ratelimit-tokens-limit`            | El número máximo de tokens permitidos dentro de cualquier período de límite de velocidad.                                                      |
| `anthropic-ratelimit-tokens-remaining`        | El número de tokens restantes (redondeado al millar más cercano) antes de ser limitado por velocidad.                                         |
| `anthropic-ratelimit-tokens-reset`            | La hora en que se restablecerá completamente el límite de velocidad de tokens, proporcionada en formato RFC 3339.                             |
| `anthropic-ratelimit-input-tokens-limit`      | El número máximo de tokens de entrada permitidos dentro de cualquier período de límite de velocidad.                                           |
| `anthropic-ratelimit-input-tokens-remaining`  | El número de tokens de entrada restantes (redondeado al millar más cercano) antes de ser limitado por velocidad.                              |
| `anthropic-ratelimit-input-tokens-reset`      | La hora en que se restablecerá completamente el límite de velocidad de tokens de entrada, proporcionada en formato RFC 3339.                  |
| `anthropic-ratelimit-output-tokens-limit`     | El número máximo de tokens de salida permitidos dentro de cualquier período de límite de velocidad.                                            |
| `anthropic-ratelimit-output-tokens-remaining` | El número de tokens de salida restantes (redondeado al millar más cercano) antes de ser limitado por velocidad.                               |
| `anthropic-ratelimit-output-tokens-reset`     | La hora en que se restablecerá completamente el límite de velocidad de tokens de salida, proporcionada en formato RFC 3339.                   |
| `anthropic-priority-input-tokens-limit`       | El número máximo de tokens de entrada de Tier Prioritario permitidos dentro de cualquier período de límite de velocidad. (Solo Tier Prioritario) |
| `anthropic-priority-input-tokens-remaining`   | El número de tokens de entrada de Tier Prioritario restantes (redondeado al millar más cercano) antes de ser limitado por velocidad. (Solo Tier Prioritario) |
| `anthropic-priority-input-tokens-reset`       | La hora en que se restablecerá completamente el límite de velocidad de tokens de entrada de Tier Prioritario, proporcionada en formato RFC 3339. (Solo Tier Prioritario) |
| `anthropic-priority-output-tokens-limit`      | El número máximo de tokens de salida de Tier Prioritario permitidos dentro de cualquier período de límite de velocidad. (Solo Tier Prioritario) |
| `anthropic-priority-output-tokens-remaining`  | El número de tokens de salida de Tier Prioritario restantes (redondeado al millar más cercano) antes de ser limitado por velocidad. (Solo Tier Prioritario) |
| `anthropic-priority-output-tokens-reset`      | La hora en que se restablecerá completamente el límite de velocidad de tokens de salida de Tier Prioritario, proporcionada en formato RFC 3339. (Solo Tier Prioritario) |

Los encabezados `anthropic-ratelimit-tokens-*` muestran los valores para el límite más restrictivo actualmente en vigor. Por ejemplo, si ha excedido el límite de tokens por minuto del espacio de trabajo, los encabezados contendrán los valores del límite de velocidad de tokens por minuto del espacio de trabajo. Si los límites del espacio de trabajo no se aplican, los encabezados devolverán los tokens totales restantes, donde el total es la suma de tokens de entrada y salida. Este enfoque garantiza que tenga visibilidad del restricción más relevante en su uso actual de la API.