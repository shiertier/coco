# Plugin nell'SDK

Carica plugin personalizzati per estendere Claude Code con comandi, agenti, competenze e hook tramite l'Agent SDK

---

I plugin ti permettono di estendere Claude Code con funzionalità personalizzate che possono essere condivise tra i progetti. Attraverso l'Agent SDK, puoi caricare programmaticamente i plugin da directory locali per aggiungere comandi slash personalizzati, agenti, competenze, hook e server MCP alle tue sessioni di agente.

## Cosa sono i plugin?

I plugin sono pacchetti di estensioni di Claude Code che possono includere:
- **Comandi**: Comandi slash personalizzati
- **Agenti**: Sottoagenti specializzati per attività specifiche
- **Competenze**: Capacità richiamate dal modello che Claude utilizza autonomamente
- **Hook**: Gestori di eventi che rispondono all'uso di strumenti e altri eventi
- **Server MCP**: Integrazioni di strumenti esterni tramite Model Context Protocol

Per informazioni complete sulla struttura dei plugin e su come creare plugin, vedi [Plugin](https://code.claude.com/docs/plugins).

## Caricamento dei plugin

Carica i plugin fornendo i percorsi del file system locale nella configurazione delle opzioni. L'SDK supporta il caricamento di più plugin da posizioni diverse.

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

### Specifiche dei percorsi

I percorsi dei plugin possono essere:
- **Percorsi relativi**: Risolti rispetto alla tua directory di lavoro corrente (ad es., `"./plugins/my-plugin"`)
- **Percorsi assoluti**: Percorsi completi del file system (ad es., `"/home/user/plugins/my-plugin"`)

<Note>
Il percorso deve puntare alla directory radice del plugin (la directory contenente `.claude-plugin/plugin.json`).
</Note>

## Verifica dell'installazione del plugin

Quando i plugin si caricano correttamente, appaiono nel messaggio di inizializzazione del sistema. Puoi verificare che i tuoi plugin siano disponibili:

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

## Utilizzo dei comandi dei plugin

I comandi dai plugin sono automaticamente nello spazio dei nomi con il nome del plugin per evitare conflitti. Il formato è `plugin-name:command-name`.

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
Se hai installato un plugin tramite la CLI (ad es., `/plugin install my-plugin@marketplace`), puoi comunque utilizzarlo nell'SDK fornendo il suo percorso di installazione. Controlla `~/.claude/plugins/` per i plugin installati tramite CLI.
</Note>

## Esempio completo

Ecco un esempio completo che dimostra il caricamento e l'utilizzo dei plugin:

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

## Riferimento della struttura del plugin

Una directory di plugin deve contenere un file manifest `.claude-plugin/plugin.json`. Può facoltativamente includere:

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

Per informazioni dettagliate sulla creazione di plugin, vedi:
- [Plugin](https://code.claude.com/docs/plugins) - Guida completa allo sviluppo dei plugin
- [Riferimento dei plugin](https://code.claude.com/docs/plugins-reference) - Specifiche tecniche e schemi

## Casi d'uso comuni

### Sviluppo e test

Carica i plugin durante lo sviluppo senza installarli globalmente:

```typescript
plugins: [
  { type: "local", path: "./dev-plugins/my-plugin" }
]
```

### Estensioni specifiche del progetto

Includi i plugin nel tuo repository di progetto per la coerenza in tutto il team:

```typescript
plugins: [
  { type: "local", path: "./project-plugins/team-workflows" }
]
```

### Più fonti di plugin

Combina i plugin da posizioni diverse:

```typescript
plugins: [
  { type: "local", path: "./local-plugin" },
  { type: "local", path: "~/.claude/custom-plugins/shared-plugin" }
]
```

## Risoluzione dei problemi

### Plugin non caricato

Se il tuo plugin non appare nel messaggio di inizializzazione:

1. **Controlla il percorso**: Assicurati che il percorso punti alla directory radice del plugin (contenente `.claude-plugin/`)
2. **Convalida plugin.json**: Assicurati che il tuo file manifest abbia una sintassi JSON valida
3. **Controlla i permessi dei file**: Assicurati che la directory del plugin sia leggibile

### Comandi non disponibili

Se i comandi dei plugin non funzionano:

1. **Usa lo spazio dei nomi**: I comandi dei plugin richiedono il formato `plugin-name:command-name`
2. **Controlla il messaggio di inizializzazione**: Verifica che il comando appaia in `slash_commands` con lo spazio dei nomi corretto
3. **Convalida i file di comando**: Assicurati che i file markdown dei comandi siano nella directory `commands/`

### Problemi di risoluzione del percorso

Se i percorsi relativi non funzionano:

1. **Controlla la directory di lavoro**: I percorsi relativi vengono risolti dalla tua directory di lavoro corrente
2. **Usa percorsi assoluti**: Per affidabilità, considera l'utilizzo di percorsi assoluti
3. **Normalizza i percorsi**: Usa le utilità di percorso per costruire i percorsi correttamente

## Vedi anche

- [Plugin](https://code.claude.com/docs/plugins) - Guida completa allo sviluppo dei plugin
- [Riferimento dei plugin](https://code.claude.com/docs/plugins-reference) - Specifiche tecniche
- [Comandi slash](/docs/it/agent-sdk/slash-commands) - Utilizzo dei comandi slash nell'SDK
- [Sottoagenti](/docs/it/agent-sdk/subagents) - Lavoro con agenti specializzati
- [Competenze](/docs/it/agent-sdk/skills) - Utilizzo delle competenze dell'agente