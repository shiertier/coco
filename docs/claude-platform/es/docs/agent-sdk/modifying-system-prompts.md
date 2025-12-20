# Modificación de prompts del sistema

Aprende cómo personalizar el comportamiento de Claude modificando los prompts del sistema usando tres enfoques: estilos de salida, systemPrompt con append, y prompts del sistema personalizados.

---

Los prompts del sistema definen el comportamiento, capacidades y estilo de respuesta de Claude. El SDK del Agente Claude proporciona tres formas de personalizar los prompts del sistema: usando estilos de salida (configuraciones persistentes basadas en archivos), agregando al prompt de Claude Code, o usando un prompt completamente personalizado.

## Entendiendo los prompts del sistema

Un prompt del sistema es el conjunto de instrucciones inicial que moldea cómo se comporta Claude a lo largo de una conversación.

<Note>
**Comportamiento predeterminado:** El SDK del Agente usa un **prompt del sistema vacío** por defecto para máxima flexibilidad. Para usar el prompt del sistema de Claude Code (instrucciones de herramientas, pautas de código, etc.), especifica `systemPrompt: { preset: "claude_code" }` en TypeScript o `system_prompt="claude_code"` en Python.
</Note>

El prompt del sistema de Claude Code incluye:

- Instrucciones de uso de herramientas y herramientas disponibles
- Pautas de estilo y formato de código
- Configuraciones de tono de respuesta y verbosidad
- Instrucciones de seguridad y protección
- Contexto sobre el directorio de trabajo actual y el entorno

## Métodos de modificación

### Método 1: Archivos CLAUDE.md (instrucciones a nivel de proyecto)

Los archivos CLAUDE.md proporcionan contexto e instrucciones específicas del proyecto que son leídas automáticamente por el SDK del Agente cuando se ejecuta en un directorio. Sirven como "memoria" persistente para tu proyecto.

#### Cómo funciona CLAUDE.md con el SDK

**Ubicación y descubrimiento:**

- **Nivel de proyecto:** `CLAUDE.md` o `.claude/CLAUDE.md` en tu directorio de trabajo
- **Nivel de usuario:** `~/.claude/CLAUDE.md` para instrucciones globales en todos los proyectos

**IMPORTANTE:** El SDK solo lee archivos CLAUDE.md cuando configuras explícitamente `settingSources` (TypeScript) o `setting_sources` (Python):

- Incluye `'project'` para cargar CLAUDE.md a nivel de proyecto
- Incluye `'user'` para cargar CLAUDE.md a nivel de usuario (`~/.claude/CLAUDE.md`)

El preset del prompt del sistema `claude_code` NO carga automáticamente CLAUDE.md - también debes especificar fuentes de configuración.

**Formato de contenido:**
Los archivos CLAUDE.md usan markdown plano y pueden contener:

- Pautas y estándares de codificación
- Contexto específico del proyecto
- Comandos o flujos de trabajo comunes
- Convenciones de API
- Requisitos de pruebas

#### Ejemplo de CLAUDE.md

```markdown
# Pautas del Proyecto

## Estilo de Código

- Usar modo estricto de TypeScript
- Preferir componentes funcionales en React
- Incluir siempre comentarios JSDoc para APIs públicas

## Pruebas

- Ejecutar `npm test` antes de hacer commit
- Mantener >80% de cobertura de código
- Usar jest para pruebas unitarias, playwright para E2E

## Comandos

- Build: `npm run build`
- Servidor dev: `npm run dev`
- Verificación de tipos: `npm run typecheck`
```

#### Usando CLAUDE.md con el SDK

<CodeGroup>

```typescript TypeScript
import { query } from "@anthropic-ai/claude-agent-sdk";

// IMPORTANTE: Debes especificar settingSources para cargar CLAUDE.md
// El preset claude_code solo NO carga archivos CLAUDE.md
const messages = [];

for await (const message of query({
  prompt: "Agrega un nuevo componente React para perfiles de usuario",
  options: {
    systemPrompt: {
      type: "preset",
      preset: "claude_code", // Usar el prompt del sistema de Claude Code
    },
    settingSources: ["project"], // Requerido para cargar CLAUDE.md del proyecto
  },
})) {
  messages.push(message);
}

// Ahora Claude tiene acceso a las pautas de tu proyecto desde CLAUDE.md
```

```python Python
from claude_agent_sdk import query, ClaudeAgentOptions

# IMPORTANTE: Debes especificar setting_sources para cargar CLAUDE.md
# El preset claude_code solo NO carga archivos CLAUDE.md
messages = []

async for message in query(
    prompt="Agrega un nuevo componente React para perfiles de usuario",
    options=ClaudeAgentOptions(
        system_prompt={
            "type": "preset",
            "preset": "claude_code"  # Usar el prompt del sistema de Claude Code
        },
        setting_sources=["project"]  # Requerido para cargar CLAUDE.md del proyecto
    )
):
    messages.append(message)

# Ahora Claude tiene acceso a las pautas de tu proyecto desde CLAUDE.md
```

</CodeGroup>

#### Cuándo usar CLAUDE.md

**Mejor para:**

- **Contexto compartido del equipo** - Pautas que todos deben seguir
- **Convenciones del proyecto** - Estándares de codificación, estructura de archivos, patrones de nomenclatura
- **Comandos comunes** - Comandos de build, test, deploy específicos de tu proyecto
- **Memoria a largo plazo** - Contexto que debe persistir en todas las sesiones
- **Instrucciones controladas por versión** - Hacer commit a git para que el equipo se mantenga sincronizado

**Características clave:**

- ✅ Persistente en todas las sesiones de un proyecto
- ✅ Compartido con el equipo vía git
- ✅ Descubrimiento automático (no se necesitan cambios de código)
- ⚠️ Requiere cargar configuraciones vía `settingSources`

### Método 2: Estilos de salida (configuraciones persistentes)

Los estilos de salida son configuraciones guardadas que modifican el prompt del sistema de Claude. Se almacenan como archivos markdown y pueden reutilizarse en sesiones y proyectos.

#### Creando un estilo de salida

<CodeGroup>

```typescript TypeScript
import { writeFile, mkdir } from "fs/promises";
import { join } from "path";
import { homedir } from "os";

async function createOutputStyle(
  name: string,
  description: string,
  prompt: string
) {
  // Nivel de usuario: ~/.claude/output-styles
  // Nivel de proyecto: .claude/output-styles
  const outputStylesDir = join(homedir(), ".claude", "output-styles");

  await mkdir(outputStylesDir, { recursive: true });

  const content = `---
name: ${name}
description: ${description}
---

${prompt}`;

  const filePath = join(
    outputStylesDir,
    `${name.toLowerCase().replace(/\s+/g, "-")}.md`
  );
  await writeFile(filePath, content, "utf-8");
}

// Ejemplo: Crear un especialista en revisión de código
await createOutputStyle(
  "Code Reviewer",
  "Asistente exhaustivo de revisión de código",
  `Eres un experto revisor de código.

Para cada envío de código:
1. Verificar bugs y problemas de seguridad
2. Evaluar rendimiento
3. Sugerir mejoras
4. Calificar calidad del código (1-10)`
);
```

```python Python
from pathlib import Path

async def create_output_style(name: str, description: str, prompt: str):
    # Nivel de usuario: ~/.claude/output-styles
    # Nivel de proyecto: .claude/output-styles
    output_styles_dir = Path.home() / '.claude' / 'output-styles'

    output_styles_dir.mkdir(parents=True, exist_ok=True)

    content = f"""---
name: {name}
description: {description}
---

{prompt}"""

    file_name = name.lower().replace(' ', '-') + '.md'
    file_path = output_styles_dir / file_name
    file_path.write_text(content, encoding='utf-8')

# Ejemplo: Crear un especialista en revisión de código
await create_output_style(
    'Code Reviewer',
    'Asistente exhaustivo de revisión de código',
    """Eres un experto revisor de código.

Para cada envío de código:
1. Verificar bugs y problemas de seguridad
2. Evaluar rendimiento
3. Sugerir mejoras
4. Calificar calidad del código (1-10)"""
)
```

</CodeGroup>

#### Usando estilos de salida

Una vez creados, activa los estilos de salida vía:

- **CLI**: `/output-style [nombre-estilo]`
- **Configuraciones**: `.claude/settings.local.json`
- **Crear nuevo**: `/output-style:new [descripción]`

**Nota para usuarios del SDK:** Los estilos de salida se cargan cuando incluyes `settingSources: ['user']` o `settingSources: ['project']` (TypeScript) / `setting_sources=["user"]` o `setting_sources=["project"]` (Python) en tus opciones.

### Método 3: Usando `systemPrompt` con append

Puedes usar el preset de Claude Code con una propiedad `append` para agregar tus instrucciones personalizadas mientras preservas toda la funcionalidad incorporada.

<CodeGroup>

```typescript TypeScript
import { query } from "@anthropic-ai/claude-agent-sdk";

const messages = [];

for await (const message of query({
  prompt: "Ayúdame a escribir una función Python para calcular números fibonacci",
  options: {
    systemPrompt: {
      type: "preset",
      preset: "claude_code",
      append:
        "Incluye siempre docstrings detallados y type hints en el código Python.",
    },
  },
})) {
  messages.push(message);
  if (message.type === "assistant") {
    console.log(message.message.content);
  }
}
```

```python Python
from claude_agent_sdk import query, ClaudeAgentOptions

messages = []

async for message in query(
    prompt="Ayúdame a escribir una función Python para calcular números fibonacci",
    options=ClaudeAgentOptions(
        system_prompt={
            "type": "preset",
            "preset": "claude_code",
            "append": "Incluye siempre docstrings detallados y type hints en el código Python."
        }
    )
):
    messages.append(message)
    if message.type == 'assistant':
        print(message.message.content)
```

</CodeGroup>

### Método 4: Prompts del sistema personalizados

Puedes proporcionar una cadena personalizada como `systemPrompt` para reemplazar completamente el predeterminado con tus propias instrucciones.

<CodeGroup>

```typescript TypeScript
import { query } from "@anthropic-ai/claude-agent-sdk";

const customPrompt = `Eres un especialista en codificación Python.
Sigue estas pautas:
- Escribir código limpio y bien documentado
- Usar type hints para todas las funciones
- Incluir docstrings comprensivos
- Preferir patrones de programación funcional cuando sea apropiado
- Explicar siempre tus decisiones de código`;

const messages = [];

for await (const message of query({
  prompt: "Crea un pipeline de procesamiento de datos",
  options: {
    systemPrompt: customPrompt,
  },
})) {
  messages.push(message);
  if (message.type === "assistant") {
    console.log(message.message.content);
  }
}
```

```python Python
from claude_agent_sdk import query, ClaudeAgentOptions

custom_prompt = """Eres un especialista en codificación Python.
Sigue estas pautas:
- Escribir código limpio y bien documentado
- Usar type hints para todas las funciones
- Incluir docstrings comprensivos
- Preferir patrones de programación funcional cuando sea apropiado
- Explicar siempre tus decisiones de código"""

messages = []

async for message in query(
    prompt="Crea un pipeline de procesamiento de datos",
    options=ClaudeAgentOptions(
        system_prompt=custom_prompt
    )
):
    messages.append(message)
    if message.type == 'assistant':
        print(message.message.content)
```

</CodeGroup>

## Comparación de los cuatro enfoques

| Característica          | CLAUDE.md           | Estilos de Salida  | `systemPrompt` con append | `systemPrompt` Personalizado |
| --- | --- | --- | --- | --- |
| **Persistencia**        | Archivo por proyecto | Guardado como archivos | Solo sesión            | Solo sesión            |
| **Reutilización**       | Por proyecto        | Entre proyectos    | Duplicación de código  | Duplicación de código  |
| **Gestión**             | En sistema de archivos | CLI + archivos   | En código              | En código              |
| **Herramientas predeterminadas** | Preservadas | Preservadas       | Preservadas            | Perdidas (a menos que se incluyan) |
| **Seguridad incorporada** | Mantenida         | Mantenida         | Mantenida              | Debe agregarse         |
| **Contexto del entorno** | Automático         | Automático        | Automático             | Debe proporcionarse    |
| **Nivel de personalización** | Solo adiciones | Reemplazar predeterminado | Solo adiciones    | Control completo       |
| **Control de versiones** | Con proyecto       | Sí                | Con código             | Con código             |
| **Alcance**             | Específico del proyecto | Usuario o proyecto | Sesión de código      | Sesión de código       |

**Nota:** "Con append" significa usar `systemPrompt: { type: "preset", preset: "claude_code", append: "..." }` en TypeScript o `system_prompt={"type": "preset", "preset": "claude_code", "append": "..."}` en Python.

## Casos de uso y mejores prácticas

### Cuándo usar CLAUDE.md

**Mejor para:**

- Estándares y convenciones de codificación específicos del proyecto
- Documentar estructura y arquitectura del proyecto
- Listar comandos comunes (build, test, deploy)
- Contexto compartido del equipo que debe estar controlado por versión
- Instrucciones que se aplican a todo el uso del SDK en un proyecto

**Ejemplos:**

- "Todos los endpoints de API deben usar patrones async/await"
- "Ejecutar `npm run lint:fix` antes de hacer commit"
- "Las migraciones de base de datos están en el directorio `migrations/`"

**Importante:** Para cargar archivos CLAUDE.md, debes establecer explícitamente `settingSources: ['project']` (TypeScript) o `setting_sources=["project"]` (Python). El preset del prompt del sistema `claude_code` NO carga automáticamente CLAUDE.md sin esta configuración.

### Cuándo usar estilos de salida

**Mejor para:**

- Cambios de comportamiento persistentes entre sesiones
- Configuraciones compartidas del equipo
- Asistentes especializados (revisor de código, científico de datos, DevOps)
- Modificaciones complejas de prompt que necesitan versionado

**Ejemplos:**

- Crear un asistente dedicado de optimización SQL
- Construir un revisor de código enfocado en seguridad
- Desarrollar un asistente de enseñanza con pedagogía específica

### Cuándo usar `systemPrompt` con append

**Mejor para:**

- Agregar estándares o preferencias de codificación específicos
- Personalizar formato de salida
- Agregar conocimiento específico del dominio
- Modificar verbosidad de respuesta
- Mejorar el comportamiento predeterminado de Claude Code sin perder instrucciones de herramientas

### Cuándo usar `systemPrompt` personalizado

**Mejor para:**

- Control completo sobre el comportamiento de Claude
- Tareas especializadas de una sola sesión
- Probar nuevas estrategias de prompt
- Situaciones donde las herramientas predeterminadas no son necesarias
- Construir agentes especializados con comportamiento único

## Combinando enfoques

Puedes combinar estos métodos para máxima flexibilidad:

### Ejemplo: Estilo de salida con adiciones específicas de sesión

<CodeGroup>

```typescript TypeScript
import { query } from "@anthropic-ai/claude-agent-sdk";

// Asumiendo que el estilo de salida "Code Reviewer" está activo (vía /output-style)
// Agregar áreas de enfoque específicas de la sesión
const messages = [];

for await (const message of query({
  prompt: "Revisa este módulo de autenticación",
  options: {
    systemPrompt: {
      type: "preset",
      preset: "claude_code",
      append: `
        Para esta revisión, prioriza:
        - Cumplimiento OAuth 2.0
        - Seguridad de almacenamiento de tokens
        - Gestión de sesiones
      `,
    },
  },
})) {
  messages.push(message);
}
```

```python Python
from claude_agent_sdk import query, ClaudeAgentOptions

# Asumiendo que el estilo de salida "Code Reviewer" está activo (vía /output-style)
# Agregar áreas de enfoque específicas de la sesión
messages = []

async for message in query(
    prompt="Revisa este módulo de autenticación",
    options=ClaudeAgentOptions(
        system_prompt={
            "type": "preset",
            "preset": "claude_code",
            "append": """
            Para esta revisión, prioriza:
            - Cumplimiento OAuth 2.0
            - Seguridad de almacenamiento de tokens
            - Gestión de sesiones
            """
        }
    )
):
    messages.append(message)
```

</CodeGroup>

## Ver también

- [Estilos de salida](https://code.claude.com/docs/output-styles) - Documentación completa de estilos de salida
- [Guía del SDK TypeScript](/docs/es/agent-sdk/typescript) - Guía completa de uso del SDK
- [Referencia del SDK TypeScript](https://code.claude.com/docs/typescript-sdk-reference) - Documentación completa de la API
- [Guía de configuración](https://code.claude.com/docs/configuration) - Opciones generales de configuración