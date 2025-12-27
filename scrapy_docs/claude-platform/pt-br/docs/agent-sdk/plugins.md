# Plugins no SDK

Carregue plugins personalizados para estender Claude Code com comandos, agentes, habilidades e hooks através do Agent SDK

---

Plugins permitem que você estenda Claude Code com funcionalidade personalizada que pode ser compartilhada entre projetos. Através do Agent SDK, você pode carregar programaticamente plugins de diretórios locais para adicionar comandos slash personalizados, agentes, habilidades, hooks e servidores MCP às suas sessões de agente.

## O que são plugins?

Plugins são pacotes de extensões Claude Code que podem incluir:
- **Comandos**: Comandos slash personalizados
- **Agentes**: Subagentes especializados para tarefas específicas
- **Habilidades**: Capacidades invocadas pelo modelo que Claude usa autonomamente
- **Hooks**: Manipuladores de eventos que respondem ao uso de ferramentas e outros eventos
- **Servidores MCP**: Integrações de ferramentas externas via Model Context Protocol

Para informações completas sobre a estrutura de plugins e como criar plugins, consulte [Plugins](https://code.claude.com/docs/plugins).

## Carregando plugins

Carregue plugins fornecendo seus caminhos do sistema de arquivos local na configuração de opções. O SDK suporta o carregamento de múltiplos plugins de diferentes locais.

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

### Especificações de caminho

Os caminhos de plugin podem ser:
- **Caminhos relativos**: Resolvidos em relação ao seu diretório de trabalho atual (por exemplo, `"./plugins/my-plugin"`)
- **Caminhos absolutos**: Caminhos completos do sistema de arquivos (por exemplo, `"/home/user/plugins/my-plugin"`)

<Note>
O caminho deve apontar para o diretório raiz do plugin (o diretório contendo `.claude-plugin/plugin.json`).
</Note>

## Verificando a instalação do plugin

Quando os plugins carregam com sucesso, eles aparecem na mensagem de inicialização do sistema. Você pode verificar se seus plugins estão disponíveis:

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

Comandos de plugins são automaticamente nomeados com o nome do plugin para evitar conflitos. O formato é `plugin-name:command-name`.

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
Se você instalou um plugin via CLI (por exemplo, `/plugin install my-plugin@marketplace`), você ainda pode usá-lo no SDK fornecendo seu caminho de instalação. Verifique `~/.claude/plugins/` para plugins instalados via CLI.
</Note>

## Exemplo completo

Aqui está um exemplo completo demonstrando o carregamento e uso de plugins:

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

## Referência de estrutura de plugin

Um diretório de plugin deve conter um arquivo de manifesto `.claude-plugin/plugin.json`. Pode opcionalmente incluir:

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

Para informações detalhadas sobre como criar plugins, consulte:
- [Plugins](https://code.claude.com/docs/plugins) - Guia completo de desenvolvimento de plugins
- [Referência de Plugins](https://code.claude.com/docs/plugins-reference) - Especificações técnicas e esquemas

## Casos de uso comuns

### Desenvolvimento e testes

Carregue plugins durante o desenvolvimento sem instalá-los globalmente:

```typescript
plugins: [
  { type: "local", path: "./dev-plugins/my-plugin" }
]
```

### Extensões específicas do projeto

Inclua plugins no seu repositório de projeto para consistência em toda a equipe:

```typescript
plugins: [
  { type: "local", path: "./project-plugins/team-workflows" }
]
```

### Múltiplas fontes de plugin

Combine plugins de diferentes locais:

```typescript
plugins: [
  { type: "local", path: "./local-plugin" },
  { type: "local", path: "~/.claude/custom-plugins/shared-plugin" }
]
```

## Solução de problemas

### Plugin não carregando

Se seu plugin não aparecer na mensagem de inicialização:

1. **Verifique o caminho**: Certifique-se de que o caminho aponta para o diretório raiz do plugin (contendo `.claude-plugin/`)
2. **Valide plugin.json**: Certifique-se de que seu arquivo de manifesto tem sintaxe JSON válida
3. **Verifique permissões de arquivo**: Certifique-se de que o diretório do plugin é legível

### Comandos não disponíveis

Se os comandos do plugin não funcionarem:

1. **Use o namespace**: Comandos de plugin requerem o formato `plugin-name:command-name`
2. **Verifique a mensagem de inicialização**: Verifique se o comando aparece em `slash_commands` com o namespace correto
3. **Valide arquivos de comando**: Certifique-se de que os arquivos de markdown de comando estão no diretório `commands/`

### Problemas de resolução de caminho

Se caminhos relativos não funcionarem:

1. **Verifique o diretório de trabalho**: Caminhos relativos são resolvidos a partir do seu diretório de trabalho atual
2. **Use caminhos absolutos**: Para confiabilidade, considere usar caminhos absolutos
3. **Normalize caminhos**: Use utilitários de caminho para construir caminhos corretamente

## Veja também

- [Plugins](https://code.claude.com/docs/plugins) - Guia completo de desenvolvimento de plugins
- [Referência de Plugins](https://code.claude.com/docs/plugins-reference) - Especificações técnicas
- [Comandos Slash](/docs/pt-BR/agent-sdk/slash-commands) - Usando comandos slash no SDK
- [Subagentes](/docs/pt-BR/agent-sdk/subagents) - Trabalhando com agentes especializados
- [Habilidades](/docs/pt-BR/agent-sdk/skills) - Usando Agent Skills