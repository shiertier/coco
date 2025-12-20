# Alojamiento del Agent SDK

Implementar y alojar Claude Agent SDK en entornos de producción

---

El Claude Agent SDK difiere de las API LLM tradicionales sin estado en que mantiene el estado conversacional y ejecuta comandos en un entorno persistente. Esta guía cubre la arquitectura, las consideraciones de alojamiento y las mejores prácticas para implementar agentes basados en SDK en producción.

<Info>
Para endurecimiento de seguridad más allá del sandboxing básico, incluyendo controles de red, gestión de credenciales y opciones de aislamiento, consulte [Implementación Segura](/docs/es/agent-sdk/secure-deployment).
</Info>

## Requisitos de Alojamiento

### Sandboxing Basado en Contenedores

Para seguridad y aislamiento, el SDK debe ejecutarse dentro de un entorno de contenedor aislado. Esto proporciona aislamiento de procesos, límites de recursos, control de red y sistemas de archivos efímeros.

El SDK también admite [configuración de sandbox programática](/docs/es/agent-sdk/typescript#sandbox-settings) para la ejecución de comandos.

### Requisitos del Sistema

Cada instancia de SDK requiere:

- **Dependencias de tiempo de ejecución**
  - Python 3.10+ (para SDK de Python) o Node.js 18+ (para SDK de TypeScript)
  - Node.js (requerido por Claude Code CLI)
  - Claude Code CLI: `npm install -g @anthropic-ai/claude-code`

- **Asignación de recursos**
  - Recomendado: 1GiB de RAM, 5GiB de disco y 1 CPU (varíe esto según su tarea según sea necesario)

- **Acceso de red**
  - HTTPS saliente a `api.anthropic.com`
  - Opcional: Acceso a servidores MCP o herramientas externas

## Comprensión de la Arquitectura del SDK

A diferencia de las llamadas API sin estado, el Claude Agent SDK funciona como un **proceso de larga duración** que:
- **Ejecuta comandos** en un entorno de shell persistente
- **Gestiona operaciones de archivos** dentro de un directorio de trabajo
- **Maneja la ejecución de herramientas** con contexto de interacciones anteriores

## Opciones de Proveedores de Sandbox

Varios proveedores se especializan en entornos de contenedores seguros para la ejecución de código de IA:

- **[Cloudflare Sandboxes](https://github.com/cloudflare/sandbox-sdk)**
- **[Modal Sandboxes](https://modal.com/docs/guide/sandbox)**
- **[Daytona](https://www.daytona.io/)**
- **[E2B](https://e2b.dev/)**
- **[Fly Machines](https://fly.io/docs/machines/)**
- **[Vercel Sandbox](https://vercel.com/docs/functions/sandbox)**

Para opciones autohospedadas (Docker, gVisor, Firecracker) y configuración de aislamiento detallada, consulte [Tecnologías de Aislamiento](/docs/es/agent-sdk/secure-deployment#isolation-technologies).

## Patrones de Implementación en Producción

### Patrón 1: Sesiones Efímeras

Cree un nuevo contenedor para cada tarea de usuario y luego destrúyalo cuando se complete.

Mejor para tareas puntuales, el usuario aún puede interactuar con la IA mientras se completa la tarea, pero una vez completada, el contenedor se destruye.

**Ejemplos:**
- Investigación y Corrección de Errores: Depurar y resolver un problema específico con contexto relevante
- Procesamiento de Facturas: Extraer y estructurar datos de recibos/facturas para sistemas contables
- Tareas de Traducción: Traducir documentos o lotes de contenido entre idiomas
- Procesamiento de Imágenes/Vídeos: Aplicar transformaciones, optimizaciones o extraer metadatos de archivos multimedia

### Patrón 2: Sesiones de Larga Duración

Mantener instancias de contenedor persistentes para tareas de larga duración. A menudo ejecutando _múltiples_ procesos de Claude Agent dentro del contenedor según la demanda.

Mejor para agentes proactivos que toman medidas sin la entrada del usuario, agentes que sirven contenido o agentes que procesan grandes cantidades de mensajes.

**Ejemplos:**
- Agente de Correo Electrónico: Monitorea correos electrónicos entrantes y triaja, responde o toma medidas de forma autónoma según el contenido
- Constructor de Sitios: Aloja sitios web personalizados por usuario con capacidades de edición en vivo servidas a través de puertos de contenedor
- Chatbots de Alta Frecuencia: Maneja flujos continuos de mensajes de plataformas como Slack donde los tiempos de respuesta rápidos son críticos

### Patrón 3: Sesiones Híbridas

Contenedores efímeros que se hidratan con historial y estado, posiblemente desde una base de datos o desde las características de reanudación de sesión del SDK.

Mejor para contenedores con interacción intermitente del usuario que inicia trabajo y se apaga cuando se completa el trabajo pero puede continuarse.

**Ejemplos:**
- Gestor de Proyectos Personal: Ayuda a gestionar proyectos en curso con verificaciones intermitentes, mantiene el contexto de tareas, decisiones y progreso
- Investigación Profunda: Realiza tareas de investigación de varias horas, guarda hallazgos y reanuda la investigación cuando el usuario regresa
- Agente de Soporte al Cliente: Maneja tickets de soporte que abarcan múltiples interacciones, carga el historial de tickets y el contexto del cliente

### Patrón 4: Contenedores Únicos

Ejecutar múltiples procesos de Claude Agent SDK en un contenedor global.

Mejor para agentes que deben colaborar estrechamente. Este es probablemente el patrón menos popular porque tendrá que evitar que los agentes se sobrescriban entre sí.

**Ejemplos:**
- **Simulaciones**: Agentes que interactúan entre sí en simulaciones como videojuegos.

# Preguntas Frecuentes

### ¿Cómo me comunico con mis sandboxes?
Al alojar en contenedores, exponga puertos para comunicarse con sus instancias de SDK. Su aplicación puede exponer puntos finales HTTP/WebSocket para clientes externos mientras el SDK se ejecuta internamente dentro del contenedor.

### ¿Cuál es el costo de alojar un contenedor?
Hemos encontrado que el costo dominante de servir agentes son los tokens, los contenedores varían según lo que aprovisione pero un costo mínimo es aproximadamente 5 centavos por hora de ejecución.

### ¿Cuándo debo apagar contenedores inactivos versus mantenerlos activos?
Esto probablemente dependa del proveedor, diferentes proveedores de sandbox le permitirán establecer diferentes criterios para tiempos de espera de inactividad después de los cuales un sandbox podría apagarse.
Querrá ajustar este tiempo de espera en función de la frecuencia con la que crea que podría haber respuesta del usuario.

### ¿Con qué frecuencia debo actualizar Claude Code CLI?
Claude Code CLI se versiona con semver, por lo que cualquier cambio importante se versionará.

### ¿Cómo monitoreo la salud del contenedor y el rendimiento del agente?
Dado que los contenedores son solo servidores, la misma infraestructura de registro que usa para el backend funcionará para contenedores.

### ¿Cuánto tiempo puede ejecutarse una sesión de agente antes de agotarse el tiempo?
Una sesión de agente no se agotará el tiempo, pero recomendamos establecer una propiedad 'maxTurns' para evitar que Claude se quede atrapado en un bucle.

## Próximos Pasos

- [Implementación Segura](/docs/es/agent-sdk/secure-deployment) - Controles de red, gestión de credenciales y endurecimiento de aislamiento
- [SDK de TypeScript - Configuración de Sandbox](/docs/es/agent-sdk/typescript#sandbox-settings) - Configurar sandbox programáticamente
- [Guía de Sesiones](/docs/es/agent-sdk/sessions) - Aprenda sobre la gestión de sesiones
- [Permisos](/docs/es/agent-sdk/permissions) - Configurar permisos de herramientas
- [Seguimiento de Costos](/docs/es/agent-sdk/cost-tracking) - Monitorear el uso de API
- [Integración MCP](/docs/es/agent-sdk/mcp) - Extender con herramientas personalizadas