# Comandos Slash en el SDK

Aprende cómo usar comandos slash para controlar sesiones de Claude Code a través del SDK

---

Los comandos slash proporcionan una forma de controlar sesiones de Claude Code con comandos especiales que comienzan con `/`. Estos comandos pueden enviarse a través del SDK para realizar acciones como limpiar el historial de conversación, compactar mensajes u obtener ayuda.

## Descubriendo Comandos Slash Disponibles

El Claude Agent SDK proporciona información sobre comandos slash disponibles en el mensaje de inicialización del sistema. Accede a esta información cuando tu sesión comience:

<CodeGroup>

```typescript TypeScript
import { query } from "@anthropic-ai/claude-agent-sdk";

for await (const message of query({
  prompt: "Hola Claude",
  options: { maxTurns: 1 }
})) {
  if (message.type === "system" && message.subtype === "init") {
    console.log("Comandos slash disponibles:", message.slash_commands);
    // Ejemplo de salida: ["/compact", "/clear", "/help"]
  }
}
```

```python Python
import asyncio
from claude_agent_sdk import query

async def main():
    async for message in query(
        prompt="Hola Claude",
        options={"max_turns": 1}
    ):
        if message.type == "system" and message.subtype == "init":
            print("Comandos slash disponibles:", message.slash_commands)
            # Ejemplo de salida: ["/compact", "/clear", "/help"]

asyncio.run(main())
```

</CodeGroup>

## Enviando Comandos Slash

Envía comandos slash incluyéndolos en tu cadena de prompt, igual que texto regular:

<CodeGroup>

```typescript TypeScript
import { query } from "@anthropic-ai/claude-agent-sdk";

// Enviar un comando slash
for await (const message of query({
  prompt: "/compact",
  options: { maxTurns: 1 }
})) {
  if (message.type === "result") {
    console.log("Comando ejecutado:", message.result);
  }
}
```

```python Python
import asyncio
from claude_agent_sdk import query

async def main():
    # Enviar un comando slash
    async for message in query(
        prompt="/compact",
        options={"max_turns": 1}
    ):
        if message.type == "result":
            print("Comando ejecutado:", message.result)

asyncio.run(main())
```

</CodeGroup>

## Comandos Slash Comunes

### `/compact` - Compactar Historial de Conversación

El comando `/compact` reduce el tamaño de tu historial de conversación resumiendo mensajes más antiguos mientras preserva contexto importante:

<CodeGroup>

```typescript TypeScript
import { query } from "@anthropic-ai/claude-agent-sdk";

for await (const message of query({
  prompt: "/compact",
  options: { maxTurns: 1 }
})) {
  if (message.type === "system" && message.subtype === "compact_boundary") {
    console.log("Compactación completada");
    console.log("Tokens pre-compactación:", message.compact_metadata.pre_tokens);
    console.log("Disparador:", message.compact_metadata.trigger);
  }
}
```

```python Python
import asyncio
from claude_agent_sdk import query

async def main():
    async for message in query(
        prompt="/compact",
        options={"max_turns": 1}
    ):
        if (message.type == "system" and 
            message.subtype == "compact_boundary"):
            print("Compactación completada")
            print("Tokens pre-compactación:", 
                  message.compact_metadata.pre_tokens)
            print("Disparador:", message.compact_metadata.trigger)

asyncio.run(main())
```

</CodeGroup>

### `/clear` - Limpiar Conversación

El comando `/clear` inicia una conversación fresca limpiando todo el historial previo:

<CodeGroup>

```typescript TypeScript
import { query } from "@anthropic-ai/claude-agent-sdk";

// Limpiar conversación y comenzar de nuevo
for await (const message of query({
  prompt: "/clear",
  options: { maxTurns: 1 }
})) {
  if (message.type === "system" && message.subtype === "init") {
    console.log("Conversación limpiada, nueva sesión iniciada");
    console.log("ID de sesión:", message.session_id);
  }
}
```

```python Python
import asyncio
from claude_agent_sdk import query

async def main():
    # Limpiar conversación y comenzar de nuevo
    async for message in query(
        prompt="/clear",
        options={"max_turns": 1}
    ):
        if message.type == "system" and message.subtype == "init":
            print("Conversación limpiada, nueva sesión iniciada")
            print("ID de sesión:", message.session_id)

asyncio.run(main())
```

</CodeGroup>

## Creando Comandos Slash Personalizados

Además de usar comandos slash integrados, puedes crear tus propios comandos personalizados que están disponibles a través del SDK. Los comandos personalizados se definen como archivos markdown en directorios específicos, similar a cómo se configuran los subagentes.

### Ubicaciones de Archivos

Los comandos slash personalizados se almacenan en directorios designados basados en su alcance:

- **Comandos de proyecto**: `.claude/commands/` - Disponibles solo en el proyecto actual
- **Comandos personales**: `~/.claude/commands/` - Disponibles en todos tus proyectos

### Formato de Archivo

Cada comando personalizado es un archivo markdown donde:
- El nombre del archivo (sin extensión `.md`) se convierte en el nombre del comando
- El contenido del archivo define qué hace el comando
- El frontmatter YAML opcional proporciona configuración

#### Ejemplo Básico

Crear `.claude/commands/refactor.md`:

```markdown
Refactoriza el código seleccionado para mejorar la legibilidad y mantenibilidad.
Enfócate en principios de código limpio y mejores prácticas.
```

Esto crea el comando `/refactor` que puedes usar a través del SDK.

#### Con Frontmatter

Crear `.claude/commands/security-check.md`:

```markdown
---
allowed-tools: Read, Grep, Glob
description: Ejecutar escaneo de vulnerabilidades de seguridad
model: claude-3-5-sonnet-20241022
---

Analiza la base de código para vulnerabilidades de seguridad incluyendo:
- Riesgos de inyección SQL
- Vulnerabilidades XSS
- Credenciales expuestas
- Configuraciones inseguras
```

### Usando Comandos Personalizados en el SDK

Una vez definidos en el sistema de archivos, los comandos personalizados están automáticamente disponibles a través del SDK:

<CodeGroup>

```typescript TypeScript
import { query } from "@anthropic-ai/claude-agent-sdk";

// Usar un comando personalizado
for await (const message of query({
  prompt: "/refactor src/auth/login.ts",
  options: { maxTurns: 3 }
})) {
  if (message.type === "assistant") {
    console.log("Sugerencias de refactorización:", message.message);
  }
}

// Los comandos personalizados aparecen en la lista slash_commands
for await (const message of query({
  prompt: "Hola",
  options: { maxTurns: 1 }
})) {
  if (message.type === "system" && message.subtype === "init") {
    // Incluirá tanto comandos integrados como personalizados
    console.log("Comandos disponibles:", message.slash_commands);
    // Ejemplo: ["/compact", "/clear", "/help", "/refactor", "/security-check"]
  }
}
```

```python Python
import asyncio
from claude_agent_sdk import query

async def main():
    # Usar un comando personalizado
    async for message in query(
        prompt="/refactor src/auth/login.py",
        options={"max_turns": 3}
    ):
        if message.type == "assistant":
            print("Sugerencias de refactorización:", message.message)
    
    # Los comandos personalizados aparecen en la lista slash_commands
    async for message in query(
        prompt="Hola",
        options={"max_turns": 1}
    ):
        if message.type == "system" and message.subtype == "init":
            # Incluirá tanto comandos integrados como personalizados
            print("Comandos disponibles:", message.slash_commands)
            # Ejemplo: ["/compact", "/clear", "/help", "/refactor", "/security-check"]

asyncio.run(main())
```

</CodeGroup>

### Características Avanzadas

#### Argumentos y Marcadores de Posición

Los comandos personalizados soportan argumentos dinámicos usando marcadores de posición:

Crear `.claude/commands/fix-issue.md`:

```markdown
---
argument-hint: [issue-number] [priority]
description: Arreglar un issue de GitHub
---

Arregla el issue #$1 con prioridad $2.
Revisa la descripción del issue e implementa los cambios necesarios.
```

Usar en SDK:

<CodeGroup>

```typescript TypeScript
import { query } from "@anthropic-ai/claude-agent-sdk";

// Pasar argumentos al comando personalizado
for await (const message of query({
  prompt: "/fix-issue 123 high",
  options: { maxTurns: 5 }
})) {
  // El comando procesará con $1="123" y $2="high"
  if (message.type === "result") {
    console.log("Issue arreglado:", message.result);
  }
}
```

```python Python
import asyncio
from claude_agent_sdk import query

async def main():
    # Pasar argumentos al comando personalizado
    async for message in query(
        prompt="/fix-issue 123 high",
        options={"max_turns": 5}
    ):
        # El comando procesará con $1="123" y $2="high"
        if message.type == "result":
            print("Issue arreglado:", message.result)

asyncio.run(main())
```

</CodeGroup>

#### Ejecución de Comandos Bash

Los comandos personalizados pueden ejecutar comandos bash e incluir su salida:

Crear `.claude/commands/git-commit.md`:

```markdown
---
allowed-tools: Bash(git add:*), Bash(git status:*), Bash(git commit:*)
description: Crear un commit de git
---

## Contexto

- Estado actual: !`git status`
- Diff actual: !`git diff HEAD`

## Tarea

Crear un commit de git con mensaje apropiado basado en los cambios.
```

#### Referencias de Archivos

Incluye contenidos de archivos usando el prefijo `@`:

Crear `.claude/commands/review-config.md`:

```markdown
---
description: Revisar archivos de configuración
---

Revisa los siguientes archivos de configuración para problemas:
- Configuración de paquete: @package.json
- Configuración de TypeScript: @tsconfig.json
- Configuración de entorno: @.env

Busca problemas de seguridad, dependencias desactualizadas y configuraciones incorrectas.
```

### Organización con Espacios de Nombres

Organiza comandos en subdirectorios para mejor estructura:

```bash
.claude/commands/
├── frontend/
│   ├── component.md      # Crea /component (project:frontend)
│   └── style-check.md     # Crea /style-check (project:frontend)
├── backend/
│   ├── api-test.md        # Crea /api-test (project:backend)
│   └── db-migrate.md      # Crea /db-migrate (project:backend)
└── review.md              # Crea /review (project)
```

El subdirectorio aparece en la descripción del comando pero no afecta el nombre del comando en sí.

### Ejemplos Prácticos

#### Comando de Revisión de Código

Crear `.claude/commands/code-review.md`:

```markdown
---
allowed-tools: Read, Grep, Glob, Bash(git diff:*)
description: Revisión de código integral
---

## Archivos Cambiados
!`git diff --name-only HEAD~1`

## Cambios Detallados
!`git diff HEAD~1`

## Lista de Verificación de Revisión

Revisa los cambios anteriores para:
1. Calidad y legibilidad del código
2. Vulnerabilidades de seguridad
3. Implicaciones de rendimiento
4. Cobertura de pruebas
5. Completitud de documentación

Proporciona retroalimentación específica y accionable organizada por prioridad.
```

#### Comando Ejecutor de Pruebas

Crear `.claude/commands/test.md`:

```markdown
---
allowed-tools: Bash, Read, Edit
argument-hint: [test-pattern]
description: Ejecutar pruebas con patrón opcional
---

Ejecutar pruebas que coincidan con el patrón: $ARGUMENTS

1. Detectar el framework de pruebas (Jest, pytest, etc.)
2. Ejecutar pruebas con el patrón proporcionado
3. Si las pruebas fallan, analizarlas y arreglarlas
4. Re-ejecutar para verificar las correcciones
```

Usa estos comandos a través del SDK:

<CodeGroup>

```typescript TypeScript
import { query } from "@anthropic-ai/claude-agent-sdk";

// Ejecutar revisión de código
for await (const message of query({
  prompt: "/code-review",
  options: { maxTurns: 3 }
})) {
  // Procesar retroalimentación de revisión
}

// Ejecutar pruebas específicas
for await (const message of query({
  prompt: "/test auth",
  options: { maxTurns: 5 }
})) {
  // Manejar resultados de pruebas
}
```

```python Python
import asyncio
from claude_agent_sdk import query

async def main():
    # Ejecutar revisión de código
    async for message in query(
        prompt="/code-review",
        options={"max_turns": 3}
    ):
        # Procesar retroalimentación de revisión
        pass
    
    # Ejecutar pruebas específicas
    async for message in query(
        prompt="/test auth",
        options={"max_turns": 5}
    ):
        # Manejar resultados de pruebas
        pass

asyncio.run(main())
```

</CodeGroup>

## Ver También

- [Comandos Slash](https://code.claude.com/docs/slash-commands) - Documentación completa de comandos slash
- [Subagentes en el SDK](/docs/es/agent-sdk/subagents) - Configuración similar basada en sistema de archivos para subagentes
- [Referencia del SDK de TypeScript](https://code.claude.com/docs/typescript-sdk-reference) - Documentación completa de la API
- [Resumen del SDK](/docs/es/agent-sdk/overview) - Conceptos generales del SDK
- [Referencia de CLI](https://code.claude.com/docs/cli-reference) - Interfaz de línea de comandos