# SDK のプラグイン

Agent SDK を通じてコマンド、エージェント、スキル、フックを使用してカスタムプラグインを読み込み、Claude Code を拡張します

---

プラグインを使用すると、プロジェクト全体で共有できるカスタム機能を使用して Claude Code を拡張できます。Agent SDK を通じて、ローカルディレクトリからプログラムでプラグインを読み込んで、カスタムスラッシュコマンド、エージェント、スキル、フック、MCP サーバーをエージェントセッションに追加できます。

## プラグインとは何ですか？

プラグインは Claude Code 拡張機能のパッケージであり、以下を含むことができます：
- **コマンド**: カスタムスラッシュコマンド
- **エージェント**: 特定のタスク用の特殊なサブエージェント
- **スキル**: Claude が自律的に使用するモデル呼び出し機能
- **フック**: ツール使用およびその他のイベントに応答するイベントハンドラー
- **MCP サーバー**: Model Context Protocol 経由の外部ツール統合

プラグイン構造とプラグインの作成方法に関する完全な情報については、[プラグイン](https://code.claude.com/docs/plugins)を参照してください。

## プラグインの読み込み

オプション設定でローカルファイルシステムパスを指定してプラグインを読み込みます。SDK は複数の場所から複数のプラグインを読み込むことをサポートしています。

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

### パス指定

プラグインパスは以下のいずれかです：
- **相対パス**: 現在の作業ディレクトリを基準に解決されます（例：`"./plugins/my-plugin"`）
- **絶対パス**: ファイルシステムの完全なパス（例：`"/home/user/plugins/my-plugin"`）

<Note>
パスはプラグインのルートディレクトリ（`.claude-plugin/plugin.json` を含むディレクトリ）を指す必要があります。
</Note>

## プラグインインストールの確認

プラグインが正常に読み込まれると、システム初期化メッセージに表示されます。プラグインが利用可能であることを確認できます：

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

## プラグインコマンドの使用

プラグインからのコマンドは、競合を避けるためにプラグイン名で自動的に名前空間化されます。形式は `plugin-name:command-name` です。

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
CLI 経由でプラグインをインストールした場合（例：`/plugin install my-plugin@marketplace`）、SDK でもそのインストールパスを指定することで使用できます。CLI でインストールされたプラグインについては `~/.claude/plugins/` を確認してください。
</Note>

## 完全な例

プラグインの読み込みと使用を示す完全な例を以下に示します：

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

## プラグイン構造リファレンス

プラグインディレクトリには `.claude-plugin/plugin.json` マニフェストファイルが含まれている必要があります。オプションで以下を含めることができます：

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

プラグイン作成の詳細については、以下を参照してください：
- [プラグイン](https://code.claude.com/docs/plugins) - 完全なプラグイン開発ガイド
- [プラグインリファレンス](https://code.claude.com/docs/plugins-reference) - 技術仕様とスキーマ

## 一般的なユースケース

### 開発とテスト

グローバルにインストールせずに開発中にプラグインを読み込みます：

```typescript
plugins: [
  { type: "local", path: "./dev-plugins/my-plugin" }
]
```

### プロジェクト固有の拡張機能

チーム全体の一貫性のためにプロジェクトリポジトリにプラグインを含めます：

```typescript
plugins: [
  { type: "local", path: "./project-plugins/team-workflows" }
]
```

### 複数のプラグインソース

異なる場所からプラグインを組み合わせます：

```typescript
plugins: [
  { type: "local", path: "./local-plugin" },
  { type: "local", path: "~/.claude/custom-plugins/shared-plugin" }
]
```

## トラブルシューティング

### プラグインが読み込まれない

プラグインが init メッセージに表示されない場合：

1. **パスを確認する**: パスがプラグインルートディレクトリ（`.claude-plugin/` を含む）を指していることを確認してください
2. **plugin.json を検証する**: マニフェストファイルが有効な JSON 構文を持っていることを確認してください
3. **ファイルパーミッションを確認する**: プラグインディレクトリが読み取り可能であることを確認してください

### コマンドが利用できない

プラグインコマンドが機能しない場合：

1. **名前空間を使用する**: プラグインコマンドには `plugin-name:command-name` 形式が必要です
2. **init メッセージを確認する**: コマンドが正しい名前空間で `slash_commands` に表示されていることを確認してください
3. **コマンドファイルを検証する**: コマンドマークダウンファイルが `commands/` ディレクトリにあることを確認してください

### パス解決の問題

相対パスが機能しない場合：

1. **作業ディレクトリを確認する**: 相対パスは現在の作業ディレクトリから解決されます
2. **絶対パスを使用する**: 信頼性のために絶対パスの使用を検討してください
3. **パスを正規化する**: パスユーティリティを使用してパスを正しく構築してください

## 関連項目

- [プラグイン](https://code.claude.com/docs/plugins) - 完全なプラグイン開発ガイド
- [プラグインリファレンス](https://code.claude.com/docs/plugins-reference) - 技術仕様
- [スラッシュコマンド](/docs/ja/agent-sdk/slash-commands) - SDK でのスラッシュコマンドの使用
- [サブエージェント](/docs/ja/agent-sdk/subagents) - 特殊なエージェントの操作
- [スキル](/docs/ja/agent-sdk/skills) - Agent Skills の使用