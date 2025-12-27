# Descripción general de modelos

Claude es una familia de modelos de lenguaje grandes de última generación desarrollados por Anthropic. Esta guía presenta nuestros modelos y compara su rendimiento.

---

## Elegir un modelo

Si no está seguro de qué modelo usar, le recomendamos comenzar con **Claude Sonnet 4.5**. Ofrece el mejor equilibrio de inteligencia, velocidad y costo para la mayoría de los casos de uso, con un rendimiento excepcional en tareas de codificación y agentes.

Todos los modelos Claude actuales admiten entrada de texto e imagen, salida de texto, capacidades multilingües y visión. Los modelos están disponibles a través de la API de Anthropic, AWS Bedrock y Google Vertex AI.

Una vez que haya elegido un modelo, [aprenda cómo hacer su primera llamada a la API](/docs/es/get-started).

### Comparación de modelos más recientes

| Característica | Claude Sonnet 4.5 | Claude Haiku 4.5 | Claude Opus 4.5 |
|:--------|:------------------|:-----------------|:----------------|
| **Descripción** | Nuestro modelo inteligente para agentes complejos y codificación | Nuestro modelo más rápido con inteligencia casi de frontera | Modelo premium que combina inteligencia máxima con rendimiento práctico |
| **ID de API Claude** | claude-sonnet-4-5-20250929 | claude-haiku-4-5-20251001 | claude-opus-4-5-20251101 |
| **Alias de API Claude**<sup>1</sup> | claude-sonnet-4-5 | claude-haiku-4-5 | claude-opus-4-5 |
| **ID de AWS Bedrock** | anthropic.claude-sonnet-4-5-20250929-v1:0 | anthropic.claude-haiku-4-5-20251001-v1:0 | anthropic.claude-opus-4-5-20251101-v1:0 |
| **ID de GCP Vertex AI** | claude-sonnet-4-5@20250929 | claude-haiku-4-5@20251001 | claude-opus-4-5@20251101 |
| **Precios**<sup>2</sup> | \$3 / entrada MTok<br/>\$15 / salida MTok | \$1 / entrada MTok<br/>\$5 / salida MTok | \$5 / entrada MTok<br/>\$25 / salida MTok |
| **[Pensamiento extendido](/docs/es/build-with-claude/extended-thinking)** | Sí | Sí | Sí |
| **[Tier de Prioridad](/docs/es/api/service-tiers)** | Sí | Sí | Sí |
| **Latencia comparativa** | Rápido | Más rápido | Moderado |
| **Ventana de contexto** | <Tooltip tooltipContent="~150K palabras \ ~680K caracteres unicode">200K tokens</Tooltip> / <br/> <Tooltip tooltipContent="~750K palabras \ ~3.4M caracteres unicode">1M tokens</Tooltip> (beta)<sup>3</sup> | <Tooltip tooltipContent="~150K palabras \ ~680K caracteres unicode">200K tokens</Tooltip> | <Tooltip tooltipContent="~150K palabras \ ~680K caracteres unicode">200K tokens</Tooltip> |
| **Salida máxima** | 64K tokens | 64K tokens | 64K tokens |
| **Corte de conocimiento confiable** | Ene 2025<sup>4</sup> | Feb 2025 | May 2025<sup>4</sup> |
| **Corte de datos de entrenamiento** | Jul 2025 | Jul 2025 | Ago 2025 |

_<sup>1 - Los alias apuntan automáticamente a la instantánea de modelo más reciente. Cuando lanzamos nuevas instantáneas de modelo, migramos los alias para apuntar a la versión más nueva de un modelo, típicamente dentro de una semana del nuevo lanzamiento. Aunque los alias son útiles para experimentación, recomendamos usar versiones de modelo específicas (por ejemplo, `claude-sonnet-4-5-20250929`) en aplicaciones de producción para garantizar un comportamiento consistente.</sup>_

_<sup>2 - Consulte nuestra [página de precios](/docs/es/about-claude/pricing) para obtener información completa de precios, incluidos descuentos de API por lotes, tasas de almacenamiento en caché de indicaciones, costos de pensamiento extendido y tarifas de procesamiento de visión.</sup>_

_<sup>3 - Claude Sonnet 4.5 admite una [ventana de contexto de 1M tokens](/docs/es/build-with-claude/context-windows#1m-token-context-window) cuando se usa el encabezado beta `context-1m-2025-08-07`. [Precios de contexto largo](/docs/es/about-claude/pricing#long-context-pricing) se aplica a solicitudes que exceden 200K tokens.</sup>_

_<sup>4 - **Corte de conocimiento confiable** indica la fecha hasta la cual el conocimiento de un modelo es más extenso y confiable. **Corte de datos de entrenamiento** es el rango de fecha más amplio de datos de entrenamiento utilizados. Por ejemplo, Claude Sonnet 4.5 fue entrenado en información disponible públicamente hasta julio de 2025, pero su conocimiento es más extenso y confiable hasta enero de 2025. Para más información, consulte [Centro de Transparencia de Anthropic](https://www.anthropic.com/transparency).</sup>_

<Note>Los modelos con la misma fecha de instantánea (por ejemplo, 20240620) son idénticos en todas las plataformas y no cambian. La fecha de instantánea en el nombre del modelo garantiza consistencia y permite a los desarrolladores confiar en un rendimiento estable en diferentes entornos.</Note>

<Note>A partir de **Claude Sonnet 4.5 y todos los modelos futuros**, AWS Bedrock y Google Vertex AI ofrecen dos tipos de puntos finales: **puntos finales globales** (enrutamiento dinámico para máxima disponibilidad) y **puntos finales regionales** (enrutamiento de datos garantizado a través de regiones geográficas específicas). Para más información, consulte la [sección de precios de plataformas de terceros](/docs/es/about-claude/pricing#third-party-platform-pricing).</Note>

<section title="Modelos heredados">

Los siguientes modelos aún están disponibles pero recomendamos migrar a modelos actuales para mejorar el rendimiento:

| Característica | Claude Opus 4.1 | Claude Sonnet 4 | Claude Sonnet 3.7 | Claude Opus 4 | Claude Haiku 3 |
|:--------|:----------------|:----------------|:------------------|:--------------|:---------------|
| **ID de API Claude** | claude-opus-4-1-20250805 | claude-sonnet-4-20250514 | claude-3-7-sonnet-20250219 | claude-opus-4-20250514 | claude-3-haiku-20240307 |
| **Alias de API Claude** | claude-opus-4-1 | claude-sonnet-4-0 | claude-3-7-sonnet-latest | claude-opus-4-0 | — |
| **ID de AWS Bedrock** | anthropic.claude-opus-4-1-20250805-v1:0 | anthropic.claude-sonnet-4-20250514-v1:0 | anthropic.claude-3-7-sonnet-20250219-v1:0 | anthropic.claude-opus-4-20250514-v1:0 | anthropic.claude-3-haiku-20240307-v1:0 |
| **ID de GCP Vertex AI** | claude-opus-4-1@20250805 | claude-sonnet-4@20250514 | claude-3-7-sonnet@20250219 | claude-opus-4@20250514 | claude-3-haiku@20240307 |
| **Precios** | \$15 / entrada MTok<br/>\$75 / salida MTok | \$3 / entrada MTok<br/>\$15 / salida MTok | \$3 / entrada MTok<br/>\$15 / salida MTok | \$15 / entrada MTok<br/>\$75 / salida MTok | \$0.25 / entrada MTok<br/>\$1.25 / salida MTok |
| **[Pensamiento extendido](/docs/es/build-with-claude/extended-thinking)** | Sí | Sí | Sí | Sí | No |
| **[Tier de Prioridad](/docs/es/api/service-tiers)** | Sí | Sí | Sí | Sí | No |
| **Latencia comparativa** | Moderado | Rápido | Rápido | Moderado | Rápido |
| **Ventana de contexto** | <Tooltip tooltipContent="~150K palabras \ ~680K caracteres unicode">200K tokens</Tooltip> | <Tooltip tooltipContent="~150K palabras \ ~680K caracteres unicode">200K tokens</Tooltip> / <br/> <Tooltip tooltipContent="~750K palabras \ ~3.4M caracteres unicode">1M tokens</Tooltip> (beta)<sup>1</sup> | <Tooltip tooltipContent="~150K palabras \ ~680K caracteres unicode">200K tokens</Tooltip> | <Tooltip tooltipContent="~150K palabras \ ~680K caracteres unicode">200K tokens</Tooltip> | <Tooltip tooltipContent="~150K palabras \ ~680K caracteres unicode">200K tokens</Tooltip> |
| **Salida máxima** | 32K tokens | 64K tokens | 64K tokens / 128K tokens (beta)<sup>4</sup> | 32K tokens | 4K tokens |
| **Corte de conocimiento confiable** | Ene 2025<sup>2</sup> | Ene 2025<sup>2</sup> | Oct 2024<sup>2</sup> | Ene 2025<sup>2</sup> | <sup>3</sup> |
| **Corte de datos de entrenamiento** | Mar 2025 | Mar 2025 | Nov 2024 | Mar 2025 | Ago 2023 |

_<sup>1 - Claude Sonnet 4 admite una [ventana de contexto de 1M tokens](/docs/es/build-with-claude/context-windows#1m-token-context-window) cuando se usa el encabezado beta `context-1m-2025-08-07`. [Precios de contexto largo](/docs/es/about-claude/pricing#long-context-pricing) se aplica a solicitudes que exceden 200K tokens.</sup>_

_<sup>2 - **Corte de conocimiento confiable** indica la fecha hasta la cual el conocimiento de un modelo es más extenso y confiable. **Corte de datos de entrenamiento** es el rango de fecha más amplio de datos de entrenamiento utilizados.</sup>_

_<sup>3 - Algunos modelos Haiku tienen una única fecha de corte de datos de entrenamiento.</sup>_

_<sup>4 - Incluya el encabezado beta `output-128k-2025-02-19` en su solicitud de API para aumentar la longitud máxima de tokens de salida a 128K tokens para Claude Sonnet 3.7. Sugerimos fuertemente usar nuestra [API de Mensajes de transmisión](/docs/es/build-with-claude/streaming) para evitar tiempos de espera al generar salidas más largas. Consulte nuestra guía sobre [solicitudes largas](/docs/es/api/errors#long-requests) para más detalles.</sup>_

</section>

## Rendimiento de indicaciones y salida

Los modelos Claude 4 sobresalen en:
- **Rendimiento**: Resultados de nivel superior en razonamiento, codificación, tareas multilingües, manejo de contexto largo, honestidad y procesamiento de imágenes. Consulte la [publicación del blog Claude 4](http://www.anthropic.com/news/claude-4) para más información.
- **Respuestas atractivas**: Los modelos Claude son ideales para aplicaciones que requieren interacciones ricas y similares a las humanas.

    - Si prefiere respuestas más concisas, puede ajustar sus indicaciones para guiar al modelo hacia la longitud de salida deseada. Consulte nuestras [guías de ingeniería de indicaciones](/docs/es/build-with-claude/prompt-engineering) para más detalles.
    - Para prácticas recomendadas específicas de indicaciones de Claude 4, consulte nuestra [guía de mejores prácticas de Claude 4](/docs/es/build-with-claude/prompt-engineering/claude-4-best-practices).
- **Calidad de salida**: Al migrar de generaciones de modelos anteriores a Claude 4, puede notar mejoras más grandes en el rendimiento general.

## Migración a Claude 4.5

Si actualmente está usando modelos Claude 3, le recomendamos migrar a Claude 4.5 para aprovechar la inteligencia mejorada y las capacidades mejoradas. Para obtener instrucciones detalladas de migración, consulte [Migración a Claude 4.5](/docs/es/about-claude/models/migrating-to-claude-4).

## Comience con Claude

Si está listo para comenzar a explorar lo que Claude puede hacer por usted, ¡sumerjámonos! Ya sea que sea un desarrollador que busca integrar Claude en sus aplicaciones o un usuario que desea experimentar el poder de la IA de primera mano, lo tenemos cubierto.

<Note>¿Buscando chatear con Claude? ¡Visite [claude.ai](http://www.claude.ai)!</Note>

<CardGroup cols={3}>
  <Card title="Introducción a Claude" icon="check" href="/docs/es/intro">
    Explore las capacidades de Claude y el flujo de desarrollo.
  </Card>
  <Card title="Inicio rápido" icon="lightning" href="/docs/es/get-started">
    Aprenda cómo hacer su primera llamada a la API en minutos.
  </Card>
  <Card title="Consola Claude" icon="code" href="/">
    Cree y pruebe indicaciones poderosas directamente en su navegador.
  </Card>
</CardGroup>

Si tiene alguna pregunta o necesita asistencia, no dude en comunicarse con nuestro [equipo de soporte](https://support.claude.com/) o consulte la [comunidad de Discord](https://www.anthropic.com/discord).