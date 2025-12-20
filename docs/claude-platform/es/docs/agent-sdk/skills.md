# Habilidades de Agente en el SDK

Extiende Claude con capacidades especializadas usando Habilidades de Agente en el SDK del Agente Claude

---

## Descripción General

Las Habilidades de Agente extienden Claude con capacidades especializadas que Claude invoca autónomamente cuando es relevante. Las Habilidades se empaquetan como archivos `SKILL.md` que contienen instrucciones, descripciones y recursos de apoyo opcionales.

Para obtener información completa sobre Habilidades, incluidos beneficios, arquitectura y directrices de autoría, consulta la [descripción general de Habilidades de Agente](/docs/es/agents-and-tools/agent-skills/overview).

## Cómo funcionan las Habilidades con el SDK

Cuando se utiliza el SDK del Agente Claude, las Habilidades son:

1. **Definidas como artefactos del sistema de archivos**: Creadas como archivos `SKILL.md` en directorios específicos (`.claude/skills/`)
2. **Cargadas desde el sistema de archivos**: Las Habilidades se cargan desde ubicaciones del sistema de archivos configuradas. Debes especificar `settingSources` (TypeScript) o `setting_sources` (Python) para cargar Habilidades desde el sistema de archivos
3. **Descubiertas automáticamente**: Una vez que se cargan las configuraciones del sistema de archivos, los metadatos de Habilidades se descubren al inicio desde directorios de usuario y proyecto; el contenido completo se carga cuando se activa
4. **Invocadas por el modelo**: Claude elige autónomamente cuándo usarlas según el contexto
5. **Habilitadas a través de allowed_tools**: Agrega `"Skill"` a tu `allowed_tools` para habilitar Habilidades

A diferencia de los subagentes (que se pueden definir programáticamente), las Habilidades deben crearse como artefactos del sistema de archivos. El SDK no proporciona una API programática para registrar Habilidades.

<Note>
**Comportamiento predeterminado**: Por defecto, el SDK no carga ninguna configuración del sistema de archivos. Para usar Habilidades, debes configurar explícitamente `settingSources: ['user', 'project']` (TypeScript) o `setting_sources=["user", "project"]` (Python) en tus opciones.
</Note>

## Usando Habilidades con el SDK

Para usar Habilidades con el SDK, necesitas:

1. Incluir `"Skill"` en tu configuración de `allowed_tools`
2. Configurar `settingSources`/`setting_sources` para cargar Habilidades desde el sistema de archivos

Una vez configurado, Claude descubre automáticamente Habilidades desde los directorios especificados e las invoca cuando son relevantes para la solicitud del usuario.

<CodeGroup>

```python Python
import asyncio
from claude_agent_sdk import query, ClaudeAgentOptions

async def main():
    options = ClaudeAgentOptions(
        cwd="/path/to/project",  # Project with .claude/skills/
        setting_sources=["user", "project"],  # Load Skills from filesystem
        allowed_tools=["Skill", "Read", "Write", "Bash"]  # Enable Skill tool
    )

    async for message in query(
        prompt="Help me process this PDF document",
        options=options
    ):
        print(message)

asyncio.run(main())
```

```typescript TypeScript
import { query } from "@anthropic-ai/claude-agent-sdk";

for await (const message of query({
  prompt: "Help me process this PDF document",
  options: {
    cwd: "/path/to/project",  // Project with .claude/skills/
    settingSources: ["user", "project"],  // Load Skills from filesystem
    allowedTools: ["Skill", "Read", "Write", "Bash"]  // Enable Skill tool
  }
})) {
  console.log(message);
}
```

</CodeGroup>

## Ubicaciones de Habilidades

Las Habilidades se cargan desde directorios del sistema de archivos según tu configuración de `settingSources`/`setting_sources`:

- **Habilidades de Proyecto** (`.claude/skills/`): Compartidas con tu equipo a través de git - cargadas cuando `setting_sources` incluye `"project"`
- **Habilidades de Usuario** (`~/.claude/skills/`): Habilidades personales en todos los proyectos - cargadas cuando `setting_sources` incluye `"user"`
- **Habilidades de Plugin**: Incluidas con los plugins de Claude Code instalados

## Creando Habilidades

Las Habilidades se definen como directorios que contienen un archivo `SKILL.md` con frontmatter YAML y contenido Markdown. El campo `description` determina cuándo Claude invoca tu Habilidad.

**Estructura de directorio de ejemplo**:
```bash
.claude/skills/processing-pdfs/
└── SKILL.md
```

Para obtener orientación completa sobre la creación de Habilidades, incluida la estructura de SKILL.md, Habilidades de múltiples archivos y ejemplos, consulta:
- [Habilidades de Agente en Claude Code](https://code.claude.com/docs/skills): Guía completa con ejemplos
- [Mejores Prácticas de Habilidades de Agente](/docs/es/agents-and-tools/agent-skills/best-practices): Directrices de autoría y convenciones de nomenclatura

## Restricciones de Herramientas

<Note>
El campo frontmatter `allowed-tools` en SKILL.md solo se admite cuando se usa Claude Code CLI directamente. **No se aplica cuando se usan Habilidades a través del SDK**.

Cuando se usa el SDK, controla el acceso a herramientas a través de la opción principal `allowedTools` en tu configuración de consulta.
</Note>

Para restringir herramientas para Habilidades en aplicaciones SDK, usa la opción `allowedTools`:

<Note>
Se asume que las declaraciones de importación del primer ejemplo están en los siguientes fragmentos de código.
</Note>

<CodeGroup>

```python Python
options = ClaudeAgentOptions(
    setting_sources=["user", "project"],  # Load Skills from filesystem
    allowed_tools=["Skill", "Read", "Grep", "Glob"]  # Restricted toolset
)

async for message in query(
    prompt="Analyze the codebase structure",
    options=options
):
    print(message)
```

```typescript TypeScript
// Skills can only use Read, Grep, and Glob tools
for await (const message of query({
  prompt: "Analyze the codebase structure",
  options: {
    settingSources: ["user", "project"],  // Load Skills from filesystem
    allowedTools: ["Skill", "Read", "Grep", "Glob"]  // Restricted toolset
  }
})) {
  console.log(message);
}
```

</CodeGroup>

## Descubriendo Habilidades Disponibles

Para ver qué Habilidades están disponibles en tu aplicación SDK, simplemente pregúntale a Claude:

<CodeGroup>

```python Python
options = ClaudeAgentOptions(
    setting_sources=["user", "project"],  # Load Skills from filesystem
    allowed_tools=["Skill"]
)

async for message in query(
    prompt="What Skills are available?",
    options=options
):
    print(message)
```

```typescript TypeScript
for await (const message of query({
  prompt: "What Skills are available?",
  options: {
    settingSources: ["user", "project"],  // Load Skills from filesystem
    allowedTools: ["Skill"]
  }
})) {
  console.log(message);
}
```

</CodeGroup>

Claude listará las Habilidades disponibles según tu directorio de trabajo actual y plugins instalados.

## Probando Habilidades

Prueba Habilidades haciendo preguntas que coincidan con sus descripciones:

<CodeGroup>

```python Python
options = ClaudeAgentOptions(
    cwd="/path/to/project",
    setting_sources=["user", "project"],  # Load Skills from filesystem
    allowed_tools=["Skill", "Read", "Bash"]
)

async for message in query(
    prompt="Extract text from invoice.pdf",
    options=options
):
    print(message)
```

```typescript TypeScript
for await (const message of query({
  prompt: "Extract text from invoice.pdf",
  options: {
    cwd: "/path/to/project",
    settingSources: ["user", "project"],  // Load Skills from filesystem
    allowedTools: ["Skill", "Read", "Bash"]
  }
})) {
  console.log(message);
}
```

</CodeGroup>

Claude invoca automáticamente la Habilidad relevante si la descripción coincide con tu solicitud.

## Solución de Problemas

### Habilidades No Encontradas

**Verifica la configuración de settingSources**: Las Habilidades solo se cargan cuando configuras explícitamente `settingSources`/`setting_sources`. Este es el problema más común:

<CodeGroup>

```python Python
# Incorrecto - Las Habilidades no se cargarán
options = ClaudeAgentOptions(
    allowed_tools=["Skill"]
)

# Correcto - Las Habilidades se cargarán
options = ClaudeAgentOptions(
    setting_sources=["user", "project"],  # Required to load Skills
    allowed_tools=["Skill"]
)
```

```typescript TypeScript
// Incorrecto - Las Habilidades no se cargarán
const options = {
  allowedTools: ["Skill"]
};

// Correcto - Las Habilidades se cargarán
const options = {
  settingSources: ["user", "project"],  // Required to load Skills
  allowedTools: ["Skill"]
};
```

</CodeGroup>

Para más detalles sobre `settingSources`/`setting_sources`, consulta la [referencia del SDK de TypeScript](/docs/es/agent-sdk/typescript#settingsource) o la [referencia del SDK de Python](/docs/es/agent-sdk/python#settingsource).

**Verifica el directorio de trabajo**: El SDK carga Habilidades relativas a la opción `cwd`. Asegúrate de que apunte a un directorio que contenga `.claude/skills/`:

<CodeGroup>

```python Python
# Asegúrate de que tu cwd apunte al directorio que contiene .claude/skills/
options = ClaudeAgentOptions(
    cwd="/path/to/project",  # Must contain .claude/skills/
    setting_sources=["user", "project"],  # Required to load Skills
    allowed_tools=["Skill"]
)
```

```typescript TypeScript
// Asegúrate de que tu cwd apunte al directorio que contiene .claude/skills/
const options = {
  cwd: "/path/to/project",  // Must contain .claude/skills/
  settingSources: ["user", "project"],  // Required to load Skills
  allowedTools: ["Skill"]
};
```

</CodeGroup>

Consulta la sección "Usando Habilidades con el SDK" anterior para el patrón completo.

**Verifica la ubicación del sistema de archivos**:
```bash
# Check project Skills
ls .claude/skills/*/SKILL.md

# Check personal Skills
ls ~/.claude/skills/*/SKILL.md
```

### Habilidad No Siendo Usada

**Verifica que la herramienta Skill esté habilitada**: Confirma que `"Skill"` está en tu `allowedTools`.

**Verifica la descripción**: Asegúrate de que sea específica e incluya palabras clave relevantes. Consulta [Mejores Prácticas de Habilidades de Agente](/docs/es/agents-and-tools/agent-skills/best-practices#writing-effective-descriptions) para obtener orientación sobre cómo escribir descripciones efectivas.

### Solución de Problemas Adicional

Para la solución de problemas general de Habilidades (sintaxis YAML, depuración, etc.), consulta la [sección de solución de problemas de Habilidades de Claude Code](https://code.claude.com/docs/skills#troubleshooting).

## Documentación Relacionada

### Guías de Habilidades
- [Habilidades de Agente en Claude Code](https://code.claude.com/docs/skills): Guía completa de Habilidades con creación, ejemplos y solución de problemas
- [Descripción General de Habilidades de Agente](/docs/es/agents-and-tools/agent-skills/overview): Descripción conceptual, beneficios y arquitectura
- [Mejores Prácticas de Habilidades de Agente](/docs/es/agents-and-tools/agent-skills/best-practices): Directrices de autoría para Habilidades efectivas
- [Libro de Recetas de Habilidades de Agente](https://github.com/anthropics/claude-cookbooks/tree/main/skills): Habilidades de ejemplo y plantillas

### Recursos del SDK
- [Subagentes en el SDK](/docs/es/agent-sdk/subagents): Agentes similares basados en sistema de archivos con opciones programáticas
- [Comandos Slash en el SDK](/docs/es/agent-sdk/slash-commands): Comandos invocados por el usuario
- [Descripción General del SDK](/docs/es/agent-sdk/overview): Conceptos generales del SDK
- [Referencia del SDK de TypeScript](/docs/es/agent-sdk/typescript): Documentación completa de la API
- [Referencia del SDK de Python](/docs/es/agent-sdk/python): Documentación completa de la API