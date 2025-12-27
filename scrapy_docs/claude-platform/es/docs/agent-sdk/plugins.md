# Plugins en el SDK

Carga plugins personalizados para extender Claude Code con comandos, agentes, habilidades y hooks a través del Agent SDK

---

Los plugins te permiten extender Claude Code con funcionalidad personalizada que puede compartirse entre proyectos. A través del Agent SDK, puedes cargar programáticamente plugins desde directorios locales para agregar comandos de barra diagonal personalizados, agentes, habilidades, hooks y servidores MCP a tus sesiones de agente.

## ¿Qué son los plugins?

Los plugins son paquetes de extensiones de Claude Code que pueden incluir:
- **Comandos**: Comandos de barra diagonal personalizados
- **Agentes**: Subagentes especializados para tareas específicas
- **Habilidades**: Capacidades invocadas por el modelo que Claude utiliza de forma autónoma
- **Hooks**: Manejadores de eventos que responden al uso de herramientas y otros eventos
- **Servidores MCP**: Integraciones de herramientas externas a través del Protocolo de Contexto de Modelo

Para obtener información completa sobre la estructura de plugins y cómo crear plugins, consulta [Plugins](https://code.claude.com/docs/plugins).

## Cargando plugins

Carga plugins proporcionando sus rutas del sistema de archivos local en tu configuración de opciones. El SDK admite cargar múltiples plugins desde diferentes ubicaciones.

<CodeGroup>

```typescript TypeScript
import { query } from "@anthropic-ai/claude-agent-sdk";

for await (const message of query({
  prompt: "Hello",
  options: {
    plugins: [
      { type: "local", path: "./my-plugin" },
      { type: "local", path: "/absolute/path/to/another-plugin" }
    ]
  }
})) {
  // Plugin commands, agents, and other features are now available
}
```

```python Python
import asyncio
from claude_agent_sdk import query

async def main():
    async for message in query(
        prompt="Hello",
        options={
            "plugins": [
                {"type": "local", "path": "./my-plugin"},
                {"type": "local", "path": "/absolute/path/to/another-plugin"}
            ]
        }
    ):
        # Plugin commands, agents, and other features are now available
        pass

asyncio.run(main())
```

</CodeGroup>

### Especificaciones de ruta

Las rutas de plugins pueden ser:
- **Rutas relativas**: Resueltas relativas a tu directorio de trabajo actual (por ejemplo, `"./plugins/my-plugin"`)
- **Rutas absolutas**: Rutas completas del sistema de archivos (por ejemplo, `"/home/user/plugins/my-plugin"`)

<Note>
La ruta debe apuntar al directorio raíz del plugin (el directorio que contiene `.claude-plugin/plugin.json`).
</Note>

## Verificando la instalación del plugin

Cuando los plugins se cargan correctamente, aparecen en el mensaje de inicialización del sistema. Puedes verificar que tus plugins estén disponibles:

<CodeGroup>

```typescript TypeScript
import { query } from "@anthropic-ai/claude-agent-sdk";

for await (const message of query({
  prompt: "Hello",
  options: {
    plugins: [{ type: "local", path: "./my-plugin" }]
  }
})) {
  if (message.type === "system" && message.subtype === "init") {
    // Check loaded plugins
    console.log("Plugins:", message.plugins);
    // Example: [{ name: "my-plugin", path: "./my-plugin" }]

    // Check available commands from plugins
    console.log("Commands:", message.slash_commands);
    // Example: ["/help", "/compact", "my-plugin:custom-command"]
  }
}
```

```python Python
import asyncio
from claude_agent_sdk import query

async def main():
    async for message in query(
        prompt="Hello",
        options={"plugins": [{"type": "local", "path": "./my-plugin"}]}
    ):
        if message.type == "system" and message.subtype == "init":
            # Check loaded plugins
            print("Plugins:", message.data.get("plugins"))
            # Example: [{"name": "my-plugin", "path": "./my-plugin"}]

            # Check available commands from plugins
            print("Commands:", message.data.get("slash_commands"))
            # Example: ["/help", "/compact", "my-plugin:custom-command"]

asyncio.run(main())
```

</CodeGroup>

## Usando comandos de plugin

Los comandos de los plugins se espacian automáticamente con el nombre del plugin para evitar conflictos. El formato es `plugin-name:command-name`.

<CodeGroup>

```typescript TypeScript
import { query } from "@anthropic-ai/claude-agent-sdk";

// Load a plugin with a custom /greet command
for await (const message of query({
  prompt: "/my-plugin:greet",  // Use plugin command with namespace
  options: {
    plugins: [{ type: "local", path: "./my-plugin" }]
  }
})) {
  // Claude executes the custom greeting command from the plugin
  if (message.type === "assistant") {
    console.log(message.content);
  }
}
```

```python Python
import asyncio
from claude_agent_sdk import query, AssistantMessage, TextBlock

async def main():
    # Load a plugin with a custom /greet command
    async for message in query(
        prompt="/demo-plugin:greet",  # Use plugin command with namespace
        options={"plugins": [{"type": "local", "path": "./plugins/demo-plugin"}]}
    ):
        # Claude executes the custom greeting command from the plugin
        if isinstance(message, AssistantMessage):
            for block in message.content:
                if isinstance(block, TextBlock):
                    print(f"Claude: {block.text}")

asyncio.run(main())
```

</CodeGroup>

<Note>
Si instalaste un plugin a través de la CLI (por ejemplo, `/plugin install my-plugin@marketplace`), aún puedes usarlo en el SDK proporcionando su ruta de instalación. Verifica `~/.claude/plugins/` para plugins instalados por CLI.
</Note>

## Ejemplo completo

Aquí hay un ejemplo completo que demuestra la carga y el uso de plugins:

<CodeGroup>

```typescript TypeScript
import { query } from "@anthropic-ai/claude-agent-sdk";
import * as path from "path";

async function runWithPlugin() {
  const pluginPath = path.join(__dirname, "plugins", "my-plugin");

  console.log("Loading plugin from:", pluginPath);

  for await (const message of query({
    prompt: "What custom commands do you have available?",
    options: {
      plugins: [
        { type: "local", path: pluginPath }
      ],
      maxTurns: 3
    }
  })) {
    if (message.type === "system" && message.subtype === "init") {
      console.log("Loaded plugins:", message.plugins);
      console.log("Available commands:", message.slash_commands);
    }

    if (message.type === "assistant") {
      console.log("Assistant:", message.content);
    }
  }
}

runWithPlugin().catch(console.error);
```

```python Python
#!/usr/bin/env python3
"""Example demonstrating how to use plugins with the Agent SDK."""

from pathlib import Path
import anyio
from claude_agent_sdk import (
    AssistantMessage,
    ClaudeAgentOptions,
    TextBlock,
    query,
)


async def run_with_plugin():
    """Example using a custom plugin."""
    plugin_path = Path(__file__).parent / "plugins" / "demo-plugin"

    print(f"Loading plugin from: {plugin_path}")

    options = ClaudeAgentOptions(
        plugins=[
            {"type": "local", "path": str(plugin_path)}
        ],
        max_turns=3,
    )

    async for message in query(
        prompt="What custom commands do you have available?",
        options=options
    ):
        if message.type == "system" and message.subtype == "init":
            print(f"Loaded plugins: {message.data.get('plugins')}")
            print(f"Available commands: {message.data.get('slash_commands')}")

        if isinstance(message, AssistantMessage):
            for block in message.content:
                if isinstance(block, TextBlock):
                    print(f"Assistant: {block.text}")


if __name__ == "__main__":
    anyio.run(run_with_plugin)
```

</CodeGroup>

## Referencia de estructura de plugin

Un directorio de plugin debe contener un archivo de manifiesto `.claude-plugin/plugin.json`. Opcionalmente puede incluir:

```
my-plugin/
├── .claude-plugin/
│   └── plugin.json          # Required: plugin manifest
├── commands/                 # Custom slash commands
│   └── custom-cmd.md
├── agents/                   # Custom agents
│   └── specialist.md
├── skills/                   # Agent Skills
│   └── my-skill/
│       └── SKILL.md
├── hooks/                    # Event handlers
│   └── hooks.json
└── .mcp.json                # MCP server definitions
```

Para obtener información detallada sobre la creación de plugins, consulta:
- [Plugins](https://code.claude.com/docs/plugins) - Guía completa de desarrollo de plugins
- [Referencia de plugins](https://code.claude.com/docs/plugins-reference) - Especificaciones técnicas y esquemas

## Casos de uso comunes

### Desarrollo y pruebas

Carga plugins durante el desarrollo sin instalarlos globalmente:

```typescript
plugins: [
  { type: "local", path: "./dev-plugins/my-plugin" }
]
```

### Extensiones específicas del proyecto

Incluye plugins en tu repositorio de proyecto para consistencia en todo el equipo:

```typescript
plugins: [
  { type: "local", path: "./project-plugins/team-workflows" }
]
```

### Múltiples fuentes de plugins

Combina plugins de diferentes ubicaciones:

```typescript
plugins: [
  { type: "local", path: "./local-plugin" },
  { type: "local", path: "~/.claude/custom-plugins/shared-plugin" }
]
```

## Solución de problemas

### Plugin no se carga

Si tu plugin no aparece en el mensaje de inicialización:

1. **Verifica la ruta**: Asegúrate de que la ruta apunte al directorio raíz del plugin (que contiene `.claude-plugin/`)
2. **Valida plugin.json**: Asegúrate de que tu archivo de manifiesto tenga una sintaxis JSON válida
3. **Verifica permisos de archivo**: Asegúrate de que el directorio del plugin sea legible

### Comandos no disponibles

Si los comandos del plugin no funcionan:

1. **Usa el espacio de nombres**: Los comandos del plugin requieren el formato `plugin-name:command-name`
2. **Verifica el mensaje de inicialización**: Verifica que el comando aparezca en `slash_commands` con el espacio de nombres correcto
3. **Valida archivos de comando**: Asegúrate de que los archivos de comando markdown estén en el directorio `commands/`

### Problemas de resolución de ruta

Si las rutas relativas no funcionan:

1. **Verifica el directorio de trabajo**: Las rutas relativas se resuelven desde tu directorio de trabajo actual
2. **Usa rutas absolutas**: Para mayor confiabilidad, considera usar rutas absolutas
3. **Normaliza rutas**: Usa utilidades de ruta para construir rutas correctamente

## Ver también

- [Plugins](https://code.claude.com/docs/plugins) - Guía completa de desarrollo de plugins
- [Referencia de plugins](https://code.claude.com/docs/plugins-reference) - Especificaciones técnicas
- [Comandos de barra diagonal](/docs/es/agent-sdk/slash-commands) - Usando comandos de barra diagonal en el SDK
- [Subagentes](/docs/es/agent-sdk/subagents) - Trabajando con agentes especializados
- [Habilidades](/docs/es/agent-sdk/skills) - Usando Agent Skills