# Niveles de servicio

Diferentes niveles de servicio te permiten equilibrar disponibilidad, rendimiento y costos predecibles según las necesidades de tu aplicación.

---

Ofrecemos tres niveles de servicio:
- **Nivel Priority:** Ideal para flujos de trabajo implementados en producción donde el tiempo, la disponibilidad y los precios predecibles son importantes
- **Standard:** Nivel de servicio predeterminado tanto para pruebas piloto como para escalar casos de uso cotidianos
- **Batch:** Ideal para flujos de trabajo asincronos que pueden esperar o beneficiarse de estar fuera de tu capacidad normal

## Nivel Standard

El nivel standard es el nivel de servicio predeterminado para todas las solicitudes de API. Las solicitudes en este nivel se priorizan junto con todas las demás solicitudes y observan disponibilidad de mejor esfuerzo.

## Nivel Priority

Las solicitudes en este nivel se priorizan sobre todas las demás solicitudes a Anthropic. Esta priorización ayuda a minimizar [errores de "servidor sobrecargado"](/docs/es/api/errors#http-errors), incluso durante horas pico.

Para más información, consulta [Comenzar con el Nivel Priority](#get-started-with-priority-tier)

## Cómo se asignan los niveles a las solicitudes

Al manejar una solicitud, Anthropic decide asignar una solicitud al Nivel Priority en los siguientes escenarios:
- Tu organización tiene suficiente capacidad de nivel priority **input** tokens por minuto
- Tu organización tiene suficiente capacidad de nivel priority **output** tokens por minuto

Anthropic cuenta el uso contra la capacidad del Nivel Priority de la siguiente manera:

**Tokens de Entrada**
- Las lecturas de caché como 0.1 tokens por token leído del caché
- Las escrituras de caché como 1.25 tokens por token escrito en el caché con un TTL de 5 minutos
- Las escrituras de caché como 2.00 tokens por token escrito en el caché con un TTL de 1 hora
- Para solicitudes de [contexto largo](/docs/es/build-with-claude/context-windows) (>200k tokens de entrada), los tokens de entrada son 2 tokens por token
- Todos los demás tokens de entrada son 1 token por token

**Tokens de Salida**
- Para solicitudes de [contexto largo](/docs/es/build-with-claude/context-windows) (>200k tokens de entrada), los tokens de salida son 1.5 tokens por token
- Todos los demás tokens de salida son 1 token por token

De lo contrario, las solicitudes proceden en el nivel standard.

<Note>
Las solicitudes asignadas al Nivel Priority se extraen tanto de la capacidad del Nivel Priority como de los límites de velocidad regulares.
Si servir la solicitud excedería los límites de velocidad, la solicitud se rechaza.
</Note>

## Usar niveles de servicio

Puedes controlar qué niveles de servicio se pueden usar para una solicitud estableciendo el parámetro `service_tier`:

```python
message = client.messages.create(
    model="claude-sonnet-4-5",
    max_tokens=1024,
    messages=[{"role": "user", "content": "Hello, Claude!"}],
    service_tier="auto"  # Automatically use Priority Tier when available, fallback to standard
)
```

El parámetro `service_tier` acepta los siguientes valores:

- `"auto"` (predeterminado) - Usa la capacidad del Nivel Priority si está disponible, retrocediendo a tu otra capacidad si no
- `"standard_only"` - Solo usa la capacidad del nivel standard, útil si no deseas usar tu capacidad del Nivel Priority

El objeto `usage` de la respuesta también incluye el nivel de servicio asignado a la solicitud:

```json
{
  "usage": {
    "input_tokens": 410,
    "cache_creation_input_tokens": 0,
    "cache_read_input_tokens": 0,
    "output_tokens": 585,
    "service_tier": "priority"
  }
}
```
Esto te permite determinar qué nivel de servicio fue asignado a la solicitud.

Al solicitar `service_tier="auto"` con un modelo con un compromiso de Nivel Priority, estos encabezados de respuesta proporcionan información:
```
anthropic-priority-input-tokens-limit: 10000
anthropic-priority-input-tokens-remaining: 9618
anthropic-priority-input-tokens-reset: 2025-01-12T23:11:59Z
anthropic-priority-output-tokens-limit: 10000
anthropic-priority-output-tokens-remaining: 6000
anthropic-priority-output-tokens-reset: 2025-01-12T23:12:21Z
```
Puedes usar la presencia de estos encabezados para detectar si tu solicitud era elegible para el Nivel Priority, incluso si estaba fuera del límite.

## Comenzar con el Nivel Priority

Es posible que desees comprometerte con la capacidad del Nivel Priority si estás interesado en:
- **Mayor disponibilidad**: Objetivo de 99.5% de tiempo de actividad con recursos computacionales priorizados
- **Control de Costos**: Gasto predecible y descuentos para compromisos más largos
- **Desbordamiento Flexible**: Retrocede automáticamente al nivel standard cuando excedes tu capacidad comprometida

Comprometerse con el Nivel Priority implicará decidir:
- Un número de tokens de entrada por minuto
- Un número de tokens de salida por minuto
- Una duración de compromiso (1, 3, 6 o 12 meses)
- Una versión de modelo específica

<Note>
La proporción de tokens de entrada a salida que compres importa. Dimensionar tu capacidad del Nivel Priority para alinearse con tus patrones de tráfico reales te ayuda a maximizar la utilización de tus tokens comprados.
</Note>

### Modelos compatibles

El Nivel Priority es compatible con:

- Claude Opus 4.5
- Claude Sonnet 4.5
- Claude Haiku 4.5
- Claude Opus 4.1
- Claude Opus 4
- Claude Sonnet 4
- Claude Sonnet 3.7 ([deprecated](/docs/es/about-claude/model-deprecations))
- Claude Haiku 3.5 ([deprecated](/docs/es/about-claude/model-deprecations))

Consulta la [página de descripción general de modelos](/docs/es/about-claude/models/overview) para más detalles sobre nuestros modelos.

### Cómo acceder al Nivel Priority

Para comenzar a usar el Nivel Priority:

1. [Contacta con ventas](https://claude.com/contact-sales/priority-tier) para completar el aprovisionamiento
2. (Opcional) Actualiza tus solicitudes de API para establecer opcionalmente el parámetro `service_tier` en `auto`
3. Monitorea tu uso a través de encabezados de respuesta y la Consola Claude