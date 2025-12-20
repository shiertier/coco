# Плагины в SDK

Загружайте пользовательские плагины для расширения Claude Code с помощью команд, агентов, навыков и хуков через Agent SDK

---

Плагины позволяют расширить Claude Code пользовательской функциональностью, которая может быть общей для нескольких проектов. Через Agent SDK вы можете программно загружать плагины из локальных директорий, чтобы добавить пользовательские слэш-команды, агентов, навыки, хуки и MCP серверы в сеансы вашего агента.

## Что такое плагины?

Плагины — это пакеты расширений Claude Code, которые могут включать:
- **Команды**: Пользовательские слэш-команды
- **Агенты**: Специализированные подагенты для конкретных задач
- **Навыки**: Возможности, вызываемые моделью, которые Claude использует автономно
- **Хуки**: Обработчики событий, которые реагируют на использование инструментов и другие события
- **MCP серверы**: Интеграции внешних инструментов через Model Context Protocol

Полную информацию о структуре плагина и способах создания плагинов см. в разделе [Плагины](https://code.claude.com/docs/plugins).

## Загрузка плагинов

Загружайте плагины, указав пути их локальной файловой системы в конфигурации параметров. SDK поддерживает загрузку нескольких плагинов из разных мест.

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

### Спецификации путей

Пути плагинов могут быть:
- **Относительные пути**: Разрешаются относительно вашей текущей рабочей директории (например, `"./plugins/my-plugin"`)
- **Абсолютные пути**: Полные пути файловой системы (например, `"/home/user/plugins/my-plugin"`)

<Note>
Путь должен указывать на корневую директорию плагина (директорию, содержащую `.claude-plugin/plugin.json`).
</Note>

## Проверка установки плагина

Когда плагины загружаются успешно, они появляются в системном сообщении инициализации. Вы можете проверить, что ваши плагины доступны:

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

## Использование команд плагина

Команды из плагинов автоматически получают пространство имён с именем плагина, чтобы избежать конфликтов. Формат: `plugin-name:command-name`.

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
Если вы установили плагин через CLI (например, `/plugin install my-plugin@marketplace`), вы все равно можете использовать его в SDK, указав путь его установки. Проверьте `~/.claude/plugins/` для плагинов, установленных через CLI.
</Note>

## Полный пример

Вот полный пример, демонстрирующий загрузку и использование плагина:

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

## Справочник структуры плагина

Директория плагина должна содержать файл манифеста `.claude-plugin/plugin.json`. Она может опционально включать:

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

Для подробной информации о создании плагинов см.:
- [Плагины](https://code.claude.com/docs/plugins) - Полное руководство по разработке плагинов
- [Справочник плагинов](https://code.claude.com/docs/plugins-reference) - Технические спецификации и схемы

## Типичные варианты использования

### Разработка и тестирование

Загружайте плагины во время разработки без их глобальной установки:

```typescript
plugins: [
  { type: "local", path: "./dev-plugins/my-plugin" }
]
```

### Расширения для конкретного проекта

Включайте плагины в репозиторий вашего проекта для согласованности в команде:

```typescript
plugins: [
  { type: "local", path: "./project-plugins/team-workflows" }
]
```

### Несколько источников плагинов

Объединяйте плагины из разных мест:

```typescript
plugins: [
  { type: "local", path: "./local-plugin" },
  { type: "local", path: "~/.claude/custom-plugins/shared-plugin" }
]
```

## Устранение неполадок

### Плагин не загружается

Если ваш плагин не появляется в сообщении инициализации:

1. **Проверьте путь**: Убедитесь, что путь указывает на корневую директорию плагина (содержащую `.claude-plugin/`)
2. **Проверьте plugin.json**: Убедитесь, что ваш файл манифеста имеет корректный синтаксис JSON
3. **Проверьте разрешения файлов**: Убедитесь, что директория плагина доступна для чтения

### Команды недоступны

Если команды плагина не работают:

1. **Используйте пространство имён**: Команды плагина требуют формат `plugin-name:command-name`
2. **Проверьте сообщение инициализации**: Убедитесь, что команда появляется в `slash_commands` с правильным пространством имён
3. **Проверьте файлы команд**: Убедитесь, что файлы markdown команд находятся в директории `commands/`

### Проблемы с разрешением пути

Если относительные пути не работают:

1. **Проверьте рабочую директорию**: Относительные пути разрешаются из вашей текущей рабочей директории
2. **Используйте абсолютные пути**: Для надёжности рассмотрите использование абсолютных путей
3. **Нормализуйте пути**: Используйте утилиты путей для правильного построения путей

## См. также

- [Плагины](https://code.claude.com/docs/plugins) - Полное руководство по разработке плагинов
- [Справочник плагинов](https://code.claude.com/docs/plugins-reference) - Технические спецификации
- [Слэш-команды](/docs/ru/agent-sdk/slash-commands) - Использование слэш-команд в SDK
- [Подагенты](/docs/ru/agent-sdk/subagents) - Работа со специализированными агентами
- [Навыки](/docs/ru/agent-sdk/skills) - Использование Agent Skills