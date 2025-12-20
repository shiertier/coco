# Plugins dans le SDK

Chargez des plugins personnalisés pour étendre Claude Code avec des commandes, des agents, des compétences et des hooks via le SDK Agent

---

Les plugins vous permettent d'étendre Claude Code avec des fonctionnalités personnalisées qui peuvent être partagées entre les projets. Via le SDK Agent, vous pouvez charger par programme des plugins à partir de répertoires locaux pour ajouter des commandes slash personnalisées, des agents, des compétences, des hooks et des serveurs MCP à vos sessions d'agent.

## Que sont les plugins ?

Les plugins sont des packages d'extensions Claude Code qui peuvent inclure :
- **Commandes** : Commandes slash personnalisées
- **Agents** : Sous-agents spécialisés pour des tâches spécifiques
- **Compétences** : Capacités invoquées par le modèle que Claude utilise de manière autonome
- **Hooks** : Gestionnaires d'événements qui répondent à l'utilisation d'outils et à d'autres événements
- **Serveurs MCP** : Intégrations d'outils externes via Model Context Protocol

Pour des informations complètes sur la structure des plugins et comment créer des plugins, consultez [Plugins](https://code.claude.com/docs/plugins).

## Chargement des plugins

Chargez les plugins en fournissant leurs chemins du système de fichiers local dans votre configuration d'options. Le SDK prend en charge le chargement de plusieurs plugins à partir de différents emplacements.

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

### Spécifications de chemin

Les chemins des plugins peuvent être :
- **Chemins relatifs** : Résolus par rapport à votre répertoire de travail actuel (par exemple, `"./plugins/my-plugin"`)
- **Chemins absolus** : Chemins complets du système de fichiers (par exemple, `"/home/user/plugins/my-plugin"`)

<Note>
Le chemin doit pointer vers le répertoire racine du plugin (le répertoire contenant `.claude-plugin/plugin.json`).
</Note>

## Vérification de l'installation du plugin

Lorsque les plugins se chargent avec succès, ils apparaissent dans le message d'initialisation du système. Vous pouvez vérifier que vos plugins sont disponibles :

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

## Utilisation des commandes de plugin

Les commandes des plugins sont automatiquement espacées avec le nom du plugin pour éviter les conflits. Le format est `plugin-name:command-name`.

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
Si vous avez installé un plugin via la CLI (par exemple, `/plugin install my-plugin@marketplace`), vous pouvez toujours l'utiliser dans le SDK en fournissant son chemin d'installation. Vérifiez `~/.claude/plugins/` pour les plugins installés via la CLI.
</Note>

## Exemple complet

Voici un exemple complet démontrant le chargement et l'utilisation de plugins :

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

## Référence de structure de plugin

Un répertoire de plugin doit contenir un fichier manifeste `.claude-plugin/plugin.json`. Il peut éventuellement inclure :

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

Pour des informations détaillées sur la création de plugins, consultez :
- [Plugins](https://code.claude.com/docs/plugins) - Guide complet de développement de plugins
- [Référence des plugins](https://code.claude.com/docs/plugins-reference) - Spécifications techniques et schémas

## Cas d'usage courants

### Développement et test

Chargez les plugins pendant le développement sans les installer globalement :

```typescript
plugins: [
  { type: "local", path: "./dev-plugins/my-plugin" }
]
```

### Extensions spécifiques au projet

Incluez les plugins dans votre référentiel de projet pour la cohérence à l'échelle de l'équipe :

```typescript
plugins: [
  { type: "local", path: "./project-plugins/team-workflows" }
]
```

### Plusieurs sources de plugins

Combinez les plugins de différents emplacements :

```typescript
plugins: [
  { type: "local", path: "./local-plugin" },
  { type: "local", path: "~/.claude/custom-plugins/shared-plugin" }
]
```

## Dépannage

### Le plugin ne se charge pas

Si votre plugin n'apparaît pas dans le message d'initialisation :

1. **Vérifiez le chemin** : Assurez-vous que le chemin pointe vers le répertoire racine du plugin (contenant `.claude-plugin/`)
2. **Validez plugin.json** : Assurez-vous que votre fichier manifeste a une syntaxe JSON valide
3. **Vérifiez les permissions de fichier** : Assurez-vous que le répertoire du plugin est lisible

### Les commandes ne sont pas disponibles

Si les commandes de plugin ne fonctionnent pas :

1. **Utilisez l'espace de noms** : Les commandes de plugin nécessitent le format `plugin-name:command-name`
2. **Vérifiez le message d'initialisation** : Vérifiez que la commande apparaît dans `slash_commands` avec l'espace de noms correct
3. **Validez les fichiers de commande** : Assurez-vous que les fichiers markdown de commande se trouvent dans le répertoire `commands/`

### Problèmes de résolution de chemin

Si les chemins relatifs ne fonctionnent pas :

1. **Vérifiez le répertoire de travail** : Les chemins relatifs sont résolus à partir de votre répertoire de travail actuel
2. **Utilisez des chemins absolus** : Pour la fiabilité, envisagez d'utiliser des chemins absolus
3. **Normalisez les chemins** : Utilisez les utilitaires de chemin pour construire les chemins correctement

## Voir aussi

- [Plugins](https://code.claude.com/docs/plugins) - Guide complet de développement de plugins
- [Référence des plugins](https://code.claude.com/docs/plugins-reference) - Spécifications techniques
- [Commandes slash](/docs/fr/agent-sdk/slash-commands) - Utilisation des commandes slash dans le SDK
- [Sous-agents](/docs/fr/agent-sdk/subagents) - Travail avec des agents spécialisés
- [Compétences](/docs/fr/agent-sdk/skills) - Utilisation des compétences d'agent