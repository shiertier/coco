# Plugin dalam SDK

Muat plugin khusus untuk memperluas Claude Code dengan perintah, agen, keterampilan, dan hook melalui Agent SDK

---

Plugin memungkinkan Anda memperluas Claude Code dengan fungsionalitas khusus yang dapat dibagikan di seluruh proyek. Melalui Agent SDK, Anda dapat secara terprogram memuat plugin dari direktori lokal untuk menambahkan perintah garis miring khusus, agen, keterampilan, hook, dan server MCP ke sesi agen Anda.

## Apa itu plugin?

Plugin adalah paket ekstensi Claude Code yang dapat mencakup:
- **Perintah**: Perintah garis miring khusus
- **Agen**: Subagen khusus untuk tugas-tugas tertentu
- **Keterampilan**: Kemampuan yang dipanggil model yang digunakan Claude secara otonom
- **Hook**: Penanganan peristiwa yang merespons penggunaan alat dan peristiwa lainnya
- **Server MCP**: Integrasi alat eksternal melalui Model Context Protocol

Untuk informasi lengkap tentang struktur plugin dan cara membuat plugin, lihat [Plugin](https://code.claude.com/docs/plugins).

## Memuat plugin

Muat plugin dengan menyediakan jalur sistem file lokal mereka dalam konfigurasi opsi Anda. SDK mendukung pemuatan beberapa plugin dari lokasi berbeda.

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

### Spesifikasi jalur

Jalur plugin dapat berupa:
- **Jalur relatif**: Diselesaikan relatif terhadap direktori kerja saat ini Anda (misalnya, `"./plugins/my-plugin"`)
- **Jalur absolut**: Jalur sistem file lengkap (misalnya, `"/home/user/plugins/my-plugin"`)

<Note>
Jalur harus menunjuk ke direktori root plugin (direktori yang berisi `.claude-plugin/plugin.json`).
</Note>

## Memverifikasi instalasi plugin

Ketika plugin dimuat dengan berhasil, mereka muncul dalam pesan inisialisasi sistem. Anda dapat memverifikasi bahwa plugin Anda tersedia:

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

## Menggunakan perintah plugin

Perintah dari plugin secara otomatis diberi namespace dengan nama plugin untuk menghindari konflik. Formatnya adalah `plugin-name:command-name`.

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
Jika Anda memasang plugin melalui CLI (misalnya, `/plugin install my-plugin@marketplace`), Anda masih dapat menggunakannya di SDK dengan menyediakan jalur instalasinya. Periksa `~/.claude/plugins/` untuk plugin yang dipasang CLI.
</Note>

## Contoh lengkap

Berikut adalah contoh lengkap yang mendemonstrasikan pemuatan dan penggunaan plugin:

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

## Referensi struktur plugin

Direktori plugin harus berisi file manifes `.claude-plugin/plugin.json`. Secara opsional dapat mencakup:

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

Untuk informasi terperinci tentang membuat plugin, lihat:
- [Plugin](https://code.claude.com/docs/plugins) - Panduan pengembangan plugin lengkap
- [Referensi Plugin](https://code.claude.com/docs/plugins-reference) - Spesifikasi teknis dan skema

## Kasus penggunaan umum

### Pengembangan dan pengujian

Muat plugin selama pengembangan tanpa memasangnya secara global:

```typescript
plugins: [
  { type: "local", path: "./dev-plugins/my-plugin" }
]
```

### Ekstensi khusus proyek

Sertakan plugin di repositori proyek Anda untuk konsistensi di seluruh tim:

```typescript
plugins: [
  { type: "local", path: "./project-plugins/team-workflows" }
]
```

### Sumber plugin ganda

Gabungkan plugin dari lokasi berbeda:

```typescript
plugins: [
  { type: "local", path: "./local-plugin" },
  { type: "local", path: "~/.claude/custom-plugins/shared-plugin" }
]
```

## Pemecahan masalah

### Plugin tidak dimuat

Jika plugin Anda tidak muncul dalam pesan init:

1. **Periksa jalurnya**: Pastikan jalur menunjuk ke direktori root plugin (berisi `.claude-plugin/`)
2. **Validasi plugin.json**: Pastikan file manifes Anda memiliki sintaks JSON yang valid
3. **Periksa izin file**: Pastikan direktori plugin dapat dibaca

### Perintah tidak tersedia

Jika perintah plugin tidak berfungsi:

1. **Gunakan namespace**: Perintah plugin memerlukan format `plugin-name:command-name`
2. **Periksa pesan init**: Verifikasi bahwa perintah muncul di `slash_commands` dengan namespace yang benar
3. **Validasi file perintah**: Pastikan file markdown perintah berada di direktori `commands/`

### Masalah resolusi jalur

Jika jalur relatif tidak berfungsi:

1. **Periksa direktori kerja**: Jalur relatif diselesaikan dari direktori kerja saat ini Anda
2. **Gunakan jalur absolut**: Untuk keandalan, pertimbangkan menggunakan jalur absolut
3. **Normalkan jalur**: Gunakan utilitas jalur untuk membuat jalur dengan benar

## Lihat juga

- [Plugin](https://code.claude.com/docs/plugins) - Panduan pengembangan plugin lengkap
- [Referensi Plugin](https://code.claude.com/docs/plugins-reference) - Spesifikasi teknis
- [Perintah Garis Miring](/docs/id/agent-sdk/slash-commands) - Menggunakan perintah garis miring di SDK
- [Subagen](/docs/id/agent-sdk/subagents) - Bekerja dengan agen khusus
- [Keterampilan](/docs/id/agent-sdk/skills) - Menggunakan Agent Skills