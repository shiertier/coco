# SDKでのスラッシュコマンド

SDKを通じてClaude Codeセッションを制御するためのスラッシュコマンドの使用方法を学ぶ

---

スラッシュコマンドは、`/`で始まる特別なコマンドでClaude Codeセッションを制御する方法を提供します。これらのコマンドは、会話履歴のクリア、メッセージの圧縮、ヘルプの取得などのアクションを実行するためにSDKを通じて送信できます。

## 利用可能なスラッシュコマンドの発見

Claude Agent SDKは、システム初期化メッセージで利用可能なスラッシュコマンドに関する情報を提供します。セッションが開始されたときにこの情報にアクセスします：

<CodeGroup>

```typescript TypeScript
import { query } from "@anthropic-ai/claude-agent-sdk";

for await (const message of query({
  prompt: "Hello Claude",
  options: { maxTurns: 1 }
})) {
  if (message.type === "system" && message.subtype === "init") {
    console.log("Available slash commands:", message.slash_commands);
    // Example output: ["/compact", "/clear", "/help"]
  }
}
```

```python Python
import asyncio
from claude_agent_sdk import query

async def main():
    async for message in query(
        prompt="Hello Claude",
        options={"max_turns": 1}
    ):
        if message.type == "system" and message.subtype == "init":
            print("Available slash commands:", message.slash_commands)
            # Example output: ["/compact", "/clear", "/help"]

asyncio.run(main())
```

</CodeGroup>

## スラッシュコマンドの送信

通常のテキストと同様に、プロンプト文字列にスラッシュコマンドを含めることで送信します：

<CodeGroup>

```typescript TypeScript
import { query } from "@anthropic-ai/claude-agent-sdk";

// スラッシュコマンドを送信
for await (const message of query({
  prompt: "/compact",
  options: { maxTurns: 1 }
})) {
  if (message.type === "result") {
    console.log("Command executed:", message.result);
  }
}
```

```python Python
import asyncio
from claude_agent_sdk import query

async def main():
    # スラッシュコマンドを送信
    async for message in query(
        prompt="/compact",
        options={"max_turns": 1}
    ):
        if message.type == "result":
            print("Command executed:", message.result)

asyncio.run(main())
```

</CodeGroup>

## 一般的なスラッシュコマンド

### `/compact` - 会話履歴の圧縮

`/compact`コマンドは、重要なコンテキストを保持しながら古いメッセージを要約することで、会話履歴のサイズを削減します：

<CodeGroup>

```typescript TypeScript
import { query } from "@anthropic-ai/claude-agent-sdk";

for await (const message of query({
  prompt: "/compact",
  options: { maxTurns: 1 }
})) {
  if (message.type === "system" && message.subtype === "compact_boundary") {
    console.log("Compaction completed");
    console.log("Pre-compaction tokens:", message.compact_metadata.pre_tokens);
    console.log("Trigger:", message.compact_metadata.trigger);
  }
}
```

```python Python
import asyncio
from claude_agent_sdk import query

async def main():
    async for message in query(
        prompt="/compact",
        options={"max_turns": 1}
    ):
        if (message.type == "system" and 
            message.subtype == "compact_boundary"):
            print("Compaction completed")
            print("Pre-compaction tokens:", 
                  message.compact_metadata.pre_tokens)
            print("Trigger:", message.compact_metadata.trigger)

asyncio.run(main())
```

</CodeGroup>

### `/clear` - 会話のクリア

`/clear`コマンドは、すべての以前の履歴をクリアして新しい会話を開始します：

<CodeGroup>

```typescript TypeScript
import { query } from "@anthropic-ai/claude-agent-sdk";

// 会話をクリアして新しく開始
for await (const message of query({
  prompt: "/clear",
  options: { maxTurns: 1 }
})) {
  if (message.type === "system" && message.subtype === "init") {
    console.log("Conversation cleared, new session started");
    console.log("Session ID:", message.session_id);
  }
}
```

```python Python
import asyncio
from claude_agent_sdk import query

async def main():
    # 会話をクリアして新しく開始
    async for message in query(
        prompt="/clear",
        options={"max_turns": 1}
    ):
        if message.type == "system" and message.subtype == "init":
            print("Conversation cleared, new session started")
            print("Session ID:", message.session_id)

asyncio.run(main())
```

</CodeGroup>

## カスタムスラッシュコマンドの作成

組み込みのスラッシュコマンドを使用することに加えて、SDKを通じて利用可能な独自のカスタムコマンドを作成できます。カスタムコマンドは、サブエージェントの設定方法と同様に、特定のディレクトリにマークダウンファイルとして定義されます。

### ファイルの場所

カスタムスラッシュコマンドは、その範囲に基づいて指定されたディレクトリに保存されます：

- **プロジェクトコマンド**: `.claude/commands/` - 現在のプロジェクトでのみ利用可能
- **個人コマンド**: `~/.claude/commands/` - すべてのプロジェクトで利用可能

### ファイル形式

各カスタムコマンドはマークダウンファイルで、以下のようになります：
- ファイル名（`.md`拡張子なし）がコマンド名になります
- ファイルの内容がコマンドの動作を定義します
- オプションのYAMLフロントマターが設定を提供します

#### 基本例

`.claude/commands/refactor.md`を作成：

```markdown
選択されたコードをリファクタリングして、可読性と保守性を向上させます。
クリーンコードの原則とベストプラクティスに焦点を当てます。
```

これにより、SDKを通じて使用できる`/refactor`コマンドが作成されます。

#### フロントマター付き

`.claude/commands/security-check.md`を作成：

```markdown
---
allowed-tools: Read, Grep, Glob
description: セキュリティ脆弱性スキャンを実行
model: claude-3-5-sonnet-20241022
---

以下を含むセキュリティ脆弱性についてコードベースを分析します：
- SQLインジェクションリスク
- XSS脆弱性
- 露出した認証情報
- 安全でない設定
```

### SDKでのカスタムコマンドの使用

ファイルシステムで定義されると、カスタムコマンドはSDKを通じて自動的に利用可能になります：

<CodeGroup>

```typescript TypeScript
import { query } from "@anthropic-ai/claude-agent-sdk";

// カスタムコマンドを使用
for await (const message of query({
  prompt: "/refactor src/auth/login.ts",
  options: { maxTurns: 3 }
})) {
  if (message.type === "assistant") {
    console.log("Refactoring suggestions:", message.message);
  }
}

// カスタムコマンドはslash_commandsリストに表示されます
for await (const message of query({
  prompt: "Hello",
  options: { maxTurns: 1 }
})) {
  if (message.type === "system" && message.subtype === "init") {
    // 組み込みとカスタムの両方のコマンドが含まれます
    console.log("Available commands:", message.slash_commands);
    // Example: ["/compact", "/clear", "/help", "/refactor", "/security-check"]
  }
}
```

```python Python
import asyncio
from claude_agent_sdk import query

async def main():
    # カスタムコマンドを使用
    async for message in query(
        prompt="/refactor src/auth/login.py",
        options={"max_turns": 3}
    ):
        if message.type == "assistant":
            print("Refactoring suggestions:", message.message)
    
    # カスタムコマンドはslash_commandsリストに表示されます
    async for message in query(
        prompt="Hello",
        options={"max_turns": 1}
    ):
        if message.type == "system" and message.subtype == "init":
            # 組み込みとカスタムの両方のコマンドが含まれます
            print("Available commands:", message.slash_commands)
            # Example: ["/compact", "/clear", "/help", "/refactor", "/security-check"]

asyncio.run(main())
```

</CodeGroup>

### 高度な機能

#### 引数とプレースホルダー

カスタムコマンドは、プレースホルダーを使用した動的引数をサポートします：

`.claude/commands/fix-issue.md`を作成：

```markdown
---
argument-hint: [issue-number] [priority]
description: GitHubの問題を修正
---

優先度$2で問題#$1を修正します。
問題の説明を確認し、必要な変更を実装します。
```

SDKで使用：

<CodeGroup>

```typescript TypeScript
import { query } from "@anthropic-ai/claude-agent-sdk";

// カスタムコマンドに引数を渡す
for await (const message of query({
  prompt: "/fix-issue 123 high",
  options: { maxTurns: 5 }
})) {
  // コマンドは$1="123"と$2="high"で処理されます
  if (message.type === "result") {
    console.log("Issue fixed:", message.result);
  }
}
```

```python Python
import asyncio
from claude_agent_sdk import query

async def main():
    # カスタムコマンドに引数を渡す
    async for message in query(
        prompt="/fix-issue 123 high",
        options={"max_turns": 5}
    ):
        # コマンドは$1="123"と$2="high"で処理されます
        if message.type == "result":
            print("Issue fixed:", message.result)

asyncio.run(main())
```

</CodeGroup>

#### Bashコマンドの実行

カスタムコマンドはbashコマンドを実行し、その出力を含めることができます：

`.claude/commands/git-commit.md`を作成：

```markdown
---
allowed-tools: Bash(git add:*), Bash(git status:*), Bash(git commit:*)
description: gitコミットを作成
---

## コンテキスト

- 現在のステータス: !`git status`
- 現在の差分: !`git diff HEAD`

## タスク

変更に基づいて適切なメッセージでgitコミットを作成します。
```

#### ファイル参照

`@`プレフィックスを使用してファイルの内容を含めます：

`.claude/commands/review-config.md`を作成：

```markdown
---
description: 設定ファイルをレビュー
---

以下の設定ファイルの問題をレビューします：
- パッケージ設定: @package.json
- TypeScript設定: @tsconfig.json
- 環境設定: @.env

セキュリティ問題、古い依存関係、設定ミスをチェックします。
```

### 名前空間による整理

より良い構造のためにサブディレクトリでコマンドを整理します：

```bash
.claude/commands/
├── frontend/
│   ├── component.md      # /component (project:frontend)を作成
│   └── style-check.md     # /style-check (project:frontend)を作成
├── backend/
│   ├── api-test.md        # /api-test (project:backend)を作成
│   └── db-migrate.md      # /db-migrate (project:backend)を作成
└── review.md              # /review (project)を作成
```

サブディレクトリはコマンドの説明に表示されますが、コマンド名自体には影響しません。

### 実用的な例

#### コードレビューコマンド

`.claude/commands/code-review.md`を作成：

```markdown
---
allowed-tools: Read, Grep, Glob, Bash(git diff:*)
description: 包括的なコードレビュー
---

## 変更されたファイル
!`git diff --name-only HEAD~1`

## 詳細な変更
!`git diff HEAD~1`

## レビューチェックリスト

上記の変更について以下をレビューします：
1. コード品質と可読性
2. セキュリティ脆弱性
3. パフォーマンスへの影響
4. テストカバレッジ
5. ドキュメントの完全性

優先度別に整理された具体的で実行可能なフィードバックを提供します。
```

#### テストランナーコマンド

`.claude/commands/test.md`を作成：

```markdown
---
allowed-tools: Bash, Read, Edit
argument-hint: [test-pattern]
description: オプションのパターンでテストを実行
---

パターンに一致するテストを実行: $ARGUMENTS

1. テストフレームワーク（Jest、pytestなど）を検出
2. 提供されたパターンでテストを実行
3. テストが失敗した場合、分析して修正
4. 修正を確認するために再実行
```

これらのコマンドをSDKを通じて使用：

<CodeGroup>

```typescript TypeScript
import { query } from "@anthropic-ai/claude-agent-sdk";

// コードレビューを実行
for await (const message of query({
  prompt: "/code-review",
  options: { maxTurns: 3 }
})) {
  // レビューフィードバックを処理
}

// 特定のテストを実行
for await (const message of query({
  prompt: "/test auth",
  options: { maxTurns: 5 }
})) {
  // テスト結果を処理
}
```

```python Python
import asyncio
from claude_agent_sdk import query

async def main():
    # コードレビューを実行
    async for message in query(
        prompt="/code-review",
        options={"max_turns": 3}
    ):
        # レビューフィードバックを処理
        pass
    
    # 特定のテストを実行
    async for message in query(
        prompt="/test auth",
        options={"max_turns": 5}
    ):
        # テスト結果を処理
        pass

asyncio.run(main())
```

</CodeGroup>

## 関連項目

- [スラッシュコマンド](https://code.claude.com/docs/slash-commands) - 完全なスラッシュコマンドドキュメント
- [SDKでのサブエージェント](/docs/ja/agent-sdk/subagents) - サブエージェントの類似したファイルシステムベースの設定
- [TypeScript SDKリファレンス](https://code.claude.com/docs/typescript-sdk-reference) - 完全なAPIドキュメント
- [SDK概要](/docs/ja/agent-sdk/overview) - 一般的なSDKの概念
- [CLIリファレンス](https://code.claude.com/docs/cli-reference) - コマンドラインインターフェース