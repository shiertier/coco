# SDK 中的外掛程式

透過 Agent SDK 載入自訂外掛程式，以使用命令、代理、技能和掛鉤來擴展 Claude Code

---

外掛程式可讓您使用可在專案間共享的自訂功能來擴展 Claude Code。透過 Agent SDK，您可以以程式設計方式從本機目錄載入外掛程式，以將自訂斜線命令、代理、技能、掛鉤和 MCP 伺服器新增到您的代理工作階段。

## 什麼是外掛程式？

外掛程式是 Claude Code 擴充功能的套件，可以包括：
- **命令**：自訂斜線命令
- **代理**：用於特定任務的專門子代理
- **技能**：Claude 自主使用的模型叫用功能
- **掛鉤**：回應工具使用和其他事件的事件處理程式
- **MCP 伺服器**：透過模型上下文協議的外部工具整合

如需有關外掛程式結構以及如何建立外掛程式的完整資訊，請參閱[外掛程式](https://code.claude.com/docs/plugins)。

## 載入外掛程式

透過在選項設定中提供外掛程式的本機檔案系統路徑來載入外掛程式。SDK 支援從不同位置載入多個外掛程式。

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

### 路徑規格

外掛程式路徑可以是：
- **相對路徑**：相對於您目前工作目錄解析（例如，`"./plugins/my-plugin"`）
- **絕對路徑**：完整檔案系統路徑（例如，`"/home/user/plugins/my-plugin"`）

<Note>
路徑應指向外掛程式的根目錄（包含 `.claude-plugin/plugin.json` 的目錄）。
</Note>

## 驗證外掛程式安裝

當外掛程式成功載入時，它們會出現在系統初始化訊息中。您可以驗證您的外掛程式是否可用：

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

## 使用外掛程式命令

來自外掛程式的命令會自動以外掛程式名稱作為命名空間，以避免衝突。格式為 `plugin-name:command-name`。

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
如果您透過 CLI 安裝了外掛程式（例如，`/plugin install my-plugin@marketplace`），您仍然可以透過提供其安裝路徑在 SDK 中使用它。檢查 `~/.claude/plugins/` 以查找 CLI 安裝的外掛程式。
</Note>

## 完整範例

以下是示範外掛程式載入和使用的完整範例：

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

## 外掛程式結構參考

外掛程式目錄必須包含 `.claude-plugin/plugin.json` 資訊清單檔案。它可以選擇性地包括：

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

如需有關建立外掛程式的詳細資訊，請參閱：
- [外掛程式](https://code.claude.com/docs/plugins) - 完整外掛程式開發指南
- [外掛程式參考](https://code.claude.com/docs/plugins-reference) - 技術規格和結構描述

## 常見使用案例

### 開發和測試

在開發期間載入外掛程式，無需全域安裝：

```typescript
plugins: [
  { type: "local", path: "./dev-plugins/my-plugin" }
]
```

### 專案特定擴充功能

在您的專案存放庫中包含外掛程式，以確保團隊範圍內的一致性：

```typescript
plugins: [
  { type: "local", path: "./project-plugins/team-workflows" }
]
```

### 多個外掛程式來源

結合來自不同位置的外掛程式：

```typescript
plugins: [
  { type: "local", path: "./local-plugin" },
  { type: "local", path: "~/.claude/custom-plugins/shared-plugin" }
]
```

## 疑難排解

### 外掛程式未載入

如果您的外掛程式未出現在初始化訊息中：

1. **檢查路徑**：確保路徑指向外掛程式根目錄（包含 `.claude-plugin/`）
2. **驗證 plugin.json**：確保您的資訊清單檔案具有有效的 JSON 語法
3. **檢查檔案權限**：確保外掛程式目錄可讀

### 命令不可用

如果外掛程式命令不起作用：

1. **使用命名空間**：外掛程式命令需要 `plugin-name:command-name` 格式
2. **檢查初始化訊息**：驗證命令是否以正確的命名空間出現在 `slash_commands` 中
3. **驗證命令檔案**：確保命令 markdown 檔案位於 `commands/` 目錄中

### 路徑解析問題

如果相對路徑不起作用：

1. **檢查工作目錄**：相對路徑從您目前的工作目錄解析
2. **使用絕對路徑**：為了可靠性，請考慮使用絕對路徑
3. **正規化路徑**：使用路徑公用程式正確建構路徑

## 另請參閱

- [外掛程式](https://code.claude.com/docs/plugins) - 完整外掛程式開發指南
- [外掛程式參考](https://code.claude.com/docs/plugins-reference) - 技術規格
- [斜線命令](/docs/zh-TW/agent-sdk/slash-commands) - 在 SDK 中使用斜線命令
- [子代理](/docs/zh-TW/agent-sdk/subagents) - 使用專門代理
- [技能](/docs/zh-TW/agent-sdk/skills) - 使用 Agent 技能