# SDK内のエージェントスキル

Claude Agent SDKを使用して、エージェントスキルで Claudeを特殊な機能で拡張します

---

## 概要

エージェントスキルは、Claudeが関連する場合に自律的に呼び出す特殊な機能でClaudeを拡張します。スキルは、指示、説明、およびオプションのサポートリソースを含む`SKILL.md`ファイルとしてパッケージ化されます。

スキルの利点、アーキテクチャ、および作成ガイドラインを含む包括的な情報については、[エージェントスキルの概要](/docs/ja/agents-and-tools/agent-skills/overview)を参照してください。

## SDKでのスキルの動作方法

Claude Agent SDKを使用する場合、スキルは以下のようになります：

1. **ファイルシステムアーティファクトとして定義される**：特定のディレクトリ（`.claude/skills/`）に`SKILL.md`ファイルとして作成されます
2. **ファイルシステムから読み込まれる**：スキルは設定されたファイルシステムの場所から読み込まれます。ファイルシステムからスキルを読み込むには、`settingSources`（TypeScript）または`setting_sources`（Python）を指定する必要があります
3. **自動的に検出される**：ファイルシステム設定が読み込まれると、スキルメタデータはスタートアップ時にユーザーおよびプロジェクトディレクトリから検出されます。完全なコンテンツはトリガーされたときに読み込まれます
4. **モデルによって呼び出される**：Claudeはコンテキストに基づいて自律的に使用するかどうかを選択します
5. **allowed_toolsで有効化される**：`allowed_tools`に`"Skill"`を追加してスキルを有効にします

サブエージェント（プログラムで定義できる）とは異なり、スキルはファイルシステムアーティファクトとして作成する必要があります。SDKはスキルを登録するためのプログラマティックAPIを提供しません。

<Note>
**デフォルトの動作**：デフォルトでは、SDKはファイルシステム設定を読み込みません。スキルを使用するには、オプションで`settingSources: ['user', 'project']`（TypeScript）または`setting_sources=["user", "project"]`（Python）を明示的に設定する必要があります。
</Note>

## SDKでのスキルの使用

SDKでスキルを使用するには、以下を行う必要があります：

1. `allowed_tools`設定に`"Skill"`を含める
2. `settingSources`/`setting_sources`を設定してファイルシステムからスキルを読み込む

設定されると、Claudeは指定されたディレクトリからスキルを自動的に検出し、ユーザーのリクエストに関連する場合に呼び出します。

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

## スキルの場所

スキルは`settingSources`/`setting_sources`設定に基づいてファイルシステムディレクトリから読み込まれます：

- **プロジェクトスキル**（`.claude/skills/`）：gitを介してチームと共有されます。`setting_sources`に`"project"`が含まれている場合に読み込まれます
- **ユーザースキル**（`~/.claude/skills/`）：すべてのプロジェクト全体の個人スキル。`setting_sources`に`"user"`が含まれている場合に読み込まれます
- **プラグインスキル**：インストール済みのClaude Codeプラグインにバンドルされています

## スキルの作成

スキルは、YAMLフロントマターとMarkdownコンテンツを含む`SKILL.md`ファイルを含むディレクトリとして定義されます。`description`フィールドは、Claudeがスキルを呼び出すタイミングを決定します。

**ディレクトリ構造の例**：
```bash
.claude/skills/processing-pdfs/
└── SKILL.md
```

スキルの作成に関する完全なガイダンス（SKILL.md構造、複数ファイルスキル、例を含む）については、以下を参照してください：
- [Claude Codeのエージェントスキル](https://code.claude.com/docs/skills)：例を含む完全なガイド
- [エージェントスキルのベストプラクティス](/docs/ja/agents-and-tools/agent-skills/best-practices)：作成ガイドラインと命名規則

## ツール制限

<Note>
SKILL.mdのフロントマター`allowed-tools`フィールドは、Claude Code CLIを直接使用する場合にのみサポートされます。**SDKを通じてスキルを使用する場合には適用されません**。

SDKを使用する場合は、クエリ設定のメイン`allowedTools`オプションを通じてツールアクセスを制御します。
</Note>

SDK アプリケーションでスキルのツールを制限するには、`allowedTools`オプションを使用します：

<Note>
最初の例のインポートステートメントは、以下のコードスニペットで想定されています。
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

## 利用可能なスキルの検出

SDKアプリケーションで利用可能なスキルを確認するには、単にClaudeに尋ねます：

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

Claudeは現在の作業ディレクトリとインストール済みプラグインに基づいて利用可能なスキルをリストします。

## スキルのテスト

説明に一致する質問をすることでスキルをテストします：

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

説明がリクエストに一致する場合、Claudeは関連するスキルを自動的に呼び出します。

## トラブルシューティング

### スキルが見つからない

**settingSources設定を確認する**：スキルは`settingSources`/`setting_sources`を明示的に設定した場合にのみ読み込まれます。これが最も一般的な問題です：

<CodeGroup>

```python Python
# 間違い - スキルは読み込まれません
options = ClaudeAgentOptions(
    allowed_tools=["Skill"]
)

# 正しい - スキルが読み込まれます
options = ClaudeAgentOptions(
    setting_sources=["user", "project"],  # Required to load Skills
    allowed_tools=["Skill"]
)
```

```typescript TypeScript
// 間違い - スキルは読み込まれません
const options = {
  allowedTools: ["Skill"]
};

// 正しい - スキルが読み込まれます
const options = {
  settingSources: ["user", "project"],  // Required to load Skills
  allowedTools: ["Skill"]
};
```

</CodeGroup>

`settingSources`/`setting_sources`の詳細については、[TypeScript SDKリファレンス](/docs/ja/agent-sdk/typescript#settingsource)または[Python SDKリファレンス](/docs/ja/agent-sdk/python#settingsource)を参照してください。

**作業ディレクトリを確認する**：SDKは`cwd`オプションに相対的にスキルを読み込みます。`.claude/skills/`を含むディレクトリを指していることを確認してください：

<CodeGroup>

```python Python
# cwdが.claude/skills/を含むディレクトリを指していることを確認します
options = ClaudeAgentOptions(
    cwd="/path/to/project",  # Must contain .claude/skills/
    setting_sources=["user", "project"],  # Required to load Skills
    allowed_tools=["Skill"]
)
```

```typescript TypeScript
// cwdが.claude/skills/を含むディレクトリを指していることを確認します
const options = {
  cwd: "/path/to/project",  // Must contain .claude/skills/
  settingSources: ["user", "project"],  // Required to load Skills
  allowedTools: ["Skill"]
};
```

</CodeGroup>

上記の「SDKでのスキルの使用」セクションを参照して、完全なパターンを確認してください。

**ファイルシステムの場所を確認する**：
```bash
# プロジェクトスキルを確認
ls .claude/skills/*/SKILL.md

# 個人スキルを確認
ls ~/.claude/skills/*/SKILL.md
```

### スキルが使用されていない

**スキルツールが有効になっていることを確認する**：`allowedTools`に`"Skill"`が含まれていることを確認します。

**説明を確認する**：具体的で関連するキーワードが含まれていることを確認してください。効果的な説明の書き方については、[エージェントスキルのベストプラクティス](/docs/ja/agents-and-tools/agent-skills/best-practices#writing-effective-descriptions)を参照してください。

### 追加のトラブルシューティング

一般的なスキルのトラブルシューティング（YAML構文、デバッグなど）については、[Claude Codeスキルのトラブルシューティングセクション](https://code.claude.com/docs/skills#troubleshooting)を参照してください。

## 関連ドキュメント

### スキルガイド
- [Claude Codeのエージェントスキル](https://code.claude.com/docs/skills)：作成、例、トラブルシューティングを含む完全なスキルガイド
- [エージェントスキルの概要](/docs/ja/agents-and-tools/agent-skills/overview)：概念的な概要、利点、およびアーキテクチャ
- [エージェントスキルのベストプラクティス](/docs/ja/agents-and-tools/agent-skills/best-practices)：効果的なスキルの作成ガイドライン
- [エージェントスキルクックブック](https://github.com/anthropics/claude-cookbooks/tree/main/skills)：スキルの例とテンプレート

### SDKリソース
- [SDKのサブエージェント](/docs/ja/agent-sdk/subagents)：プログラマティックオプションを備えた同様のファイルシステムベースのエージェント
- [SDKのスラッシュコマンド](/docs/ja/agent-sdk/slash-commands)：ユーザーが呼び出すコマンド
- [SDKの概要](/docs/ja/agent-sdk/overview)：一般的なSDKの概念
- [TypeScript SDKリファレンス](/docs/ja/agent-sdk/typescript)：完全なAPI ドキュメント
- [Python SDKリファレンス](/docs/ja/agent-sdk/python)：完全なAPIドキュメント