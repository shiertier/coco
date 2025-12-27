# システムプロンプトの変更

出力スタイル、appendを使用したsystemPrompt、カスタムシステムプロンプトの3つのアプローチを使用してシステムプロンプトを変更し、Claudeの動作をカスタマイズする方法を学びます。

---

システムプロンプトは、Claudeの動作、機能、応答スタイルを定義します。Claude Agent SDKは、システムプロンプトをカスタマイズする3つの方法を提供します：出力スタイル（永続的なファイルベースの設定）の使用、Claude Codeのプロンプトへの追加、または完全にカスタムなプロンプトの使用です。

## システムプロンプトの理解

システムプロンプトは、会話全体を通してClaudeがどのように動作するかを形作る初期の指示セットです。

<Note>
**デフォルトの動作：** Agent SDKは最大限の柔軟性のために、デフォルトで**空のシステムプロンプト**を使用します。Claude Codeのシステムプロンプト（ツール指示、コードガイドライン等）を使用するには、TypeScriptで`systemPrompt: { preset: "claude_code" }`、Pythonで`system_prompt="claude_code"`を指定してください。
</Note>

Claude Codeのシステムプロンプトには以下が含まれます：

- ツール使用指示と利用可能なツール
- コードスタイルとフォーマットガイドライン
- 応答トーンと詳細度設定
- セキュリティと安全性の指示
- 現在の作業ディレクトリと環境に関するコンテキスト

## 変更方法

### 方法1：CLAUDE.mdファイル（プロジェクトレベルの指示）

CLAUDE.mdファイルは、Agent SDKがディレクトリで実行される際に自動的に読み取られる、プロジェクト固有のコンテキストと指示を提供します。これらはプロジェクトの永続的な「メモリ」として機能します。

#### SDKでのCLAUDE.mdの動作

**場所と発見：**

- **プロジェクトレベル：** 作業ディレクトリ内の`CLAUDE.md`または`.claude/CLAUDE.md`
- **ユーザーレベル：** すべてのプロジェクトにわたるグローバル指示のための`~/.claude/CLAUDE.md`

**重要：** SDKは、`settingSources`（TypeScript）または`setting_sources`（Python）を明示的に設定した場合にのみCLAUDE.mdファイルを読み取ります：

- プロジェクトレベルのCLAUDE.mdを読み込むには`'project'`を含める
- ユーザーレベルのCLAUDE.md（`~/.claude/CLAUDE.md`）を読み込むには`'user'`を含める

`claude_code`システムプロンプトプリセットはCLAUDE.mdを自動的に読み込みません - 設定ソースも指定する必要があります。

**コンテンツ形式：**
CLAUDE.mdファイルはプレーンマークダウンを使用し、以下を含むことができます：

- コーディングガイドラインと標準
- プロジェクト固有のコンテキスト
- 一般的なコマンドやワークフロー
- API規約
- テスト要件

#### CLAUDE.mdの例

```markdown
# プロジェクトガイドライン

## コードスタイル

- TypeScript strict modeを使用
- Reactでは関数コンポーネントを優先
- パブリックAPIには常にJSDocコメントを含める

## テスト

- コミット前に`npm test`を実行
- 80%以上のコードカバレッジを維持
- ユニットテストにはjest、E2Eにはplaywrightを使用

## コマンド

- ビルド：`npm run build`
- 開発サーバー：`npm run dev`
- 型チェック：`npm run typecheck`
```

#### SDKでのCLAUDE.mdの使用

<CodeGroup>

```typescript TypeScript
import { query } from "@anthropic-ai/claude-agent-sdk";

// 重要：CLAUDE.mdを読み込むにはsettingSourcesを指定する必要があります
// claude_codeプリセット単体ではCLAUDE.mdファイルを読み込みません
const messages = [];

for await (const message of query({
  prompt: "ユーザープロファイル用の新しいReactコンポーネントを追加してください",
  options: {
    systemPrompt: {
      type: "preset",
      preset: "claude_code", // Claude Codeのシステムプロンプトを使用
    },
    settingSources: ["project"], // プロジェクトからCLAUDE.mdを読み込むために必要
  },
})) {
  messages.push(message);
}

// これでClaudeはCLAUDE.mdからのプロジェクトガイドラインにアクセスできます
```

```python Python
from claude_agent_sdk import query, ClaudeAgentOptions

# 重要：CLAUDE.mdを読み込むにはsetting_sourcesを指定する必要があります
# claude_codeプリセット単体ではCLAUDE.mdファイルを読み込みません
messages = []

async for message in query(
    prompt="ユーザープロファイル用の新しいReactコンポーネントを追加してください",
    options=ClaudeAgentOptions(
        system_prompt={
            "type": "preset",
            "preset": "claude_code"  # Claude Codeのシステムプロンプトを使用
        },
        setting_sources=["project"]  # プロジェクトからCLAUDE.mdを読み込むために必要
    )
):
    messages.append(message)

# これでClaudeはCLAUDE.mdからのプロジェクトガイドラインにアクセスできます
```

</CodeGroup>

#### CLAUDE.mdを使用するタイミング

**最適な用途：**

- **チーム共有コンテキスト** - 全員が従うべきガイドライン
- **プロジェクト規約** - コーディング標準、ファイル構造、命名パターン
- **一般的なコマンド** - プロジェクト固有のビルド、テスト、デプロイコマンド
- **長期記憶** - すべてのセッションにわたって持続すべきコンテキスト
- **バージョン管理された指示** - チームが同期を保つためにgitにコミット

**主な特徴：**

- ✅ プロジェクト内のすべてのセッションにわたって永続
- ✅ gitを通じてチームと共有
- ✅ 自動発見（コード変更不要）
- ⚠️ `settingSources`を通じた設定読み込みが必要

### 方法2：出力スタイル（永続的な設定）

出力スタイルは、Claudeのシステムプロンプトを変更する保存された設定です。これらはマークダウンファイルとして保存され、セッションやプロジェクトをまたいで再利用できます。

#### 出力スタイルの作成

<CodeGroup>

```typescript TypeScript
import { writeFile, mkdir } from "fs/promises";
import { join } from "path";
import { homedir } from "os";

async function createOutputStyle(
  name: string,
  description: string,
  prompt: string
) {
  // ユーザーレベル：~/.claude/output-styles
  // プロジェクトレベル：.claude/output-styles
  const outputStylesDir = join(homedir(), ".claude", "output-styles");

  await mkdir(outputStylesDir, { recursive: true });

  const content = `---
name: ${name}
description: ${description}
---

${prompt}`;

  const filePath = join(
    outputStylesDir,
    `${name.toLowerCase().replace(/\s+/g, "-")}.md`
  );
  await writeFile(filePath, content, "utf-8");
}

// 例：コードレビュー専門家を作成
await createOutputStyle(
  "Code Reviewer",
  "徹底的なコードレビューアシスタント",
  `あなたは専門のコードレビュアーです。

すべてのコード提出に対して：
1. バグとセキュリティ問題をチェック
2. パフォーマンスを評価
3. 改善提案
4. コード品質を評価（1-10）`
);
```

```python Python
from pathlib import Path

async def create_output_style(name: str, description: str, prompt: str):
    # ユーザーレベル：~/.claude/output-styles
    # プロジェクトレベル：.claude/output-styles
    output_styles_dir = Path.home() / '.claude' / 'output-styles'

    output_styles_dir.mkdir(parents=True, exist_ok=True)

    content = f"""---
name: {name}
description: {description}
---

{prompt}"""

    file_name = name.lower().replace(' ', '-') + '.md'
    file_path = output_styles_dir / file_name
    file_path.write_text(content, encoding='utf-8')

# 例：コードレビュー専門家を作成
await create_output_style(
    'Code Reviewer',
    '徹底的なコードレビューアシスタント',
    """あなたは専門のコードレビュアーです。

すべてのコード提出に対して：
1. バグとセキュリティ問題をチェック
2. パフォーマンスを評価
3. 改善提案
4. コード品質を評価（1-10）"""
)
```

</CodeGroup>

#### 出力スタイルの使用

作成後、出力スタイルは以下を通じて有効化できます：

- **CLI**: `/output-style [style-name]`
- **設定**: `.claude/settings.local.json`
- **新規作成**: `/output-style:new [description]`

**SDKユーザーへの注意：** 出力スタイルは、オプションで`settingSources: ['user']`または`settingSources: ['project']`（TypeScript）/`setting_sources=["user"]`または`setting_sources=["project"]`（Python）を含める場合に読み込まれます。

### 方法3：appendを使用した`systemPrompt`

Claude Codeプリセットを`append`プロパティと組み合わせて使用し、すべての組み込み機能を保持しながらカスタム指示を追加できます。

<CodeGroup>

```typescript TypeScript
import { query } from "@anthropic-ai/claude-agent-sdk";

const messages = [];

for await (const message of query({
  prompt: "フィボナッチ数を計算するPython関数を書くのを手伝ってください",
  options: {
    systemPrompt: {
      type: "preset",
      preset: "claude_code",
      append:
        "Pythonコードには常に詳細なdocstringと型ヒントを含めてください。",
    },
  },
})) {
  messages.push(message);
  if (message.type === "assistant") {
    console.log(message.message.content);
  }
}
```

```python Python
from claude_agent_sdk import query, ClaudeAgentOptions

messages = []

async for message in query(
    prompt="フィボナッチ数を計算するPython関数を書くのを手伝ってください",
    options=ClaudeAgentOptions(
        system_prompt={
            "type": "preset",
            "preset": "claude_code",
            "append": "Pythonコードには常に詳細なdocstringと型ヒントを含めてください。"
        }
    )
):
    messages.append(message)
    if message.type == 'assistant':
        print(message.message.content)
```

</CodeGroup>

### 方法4：カスタムシステムプロンプト

`systemPrompt`にカスタム文字列を提供して、デフォルトを完全に独自の指示に置き換えることができます。

<CodeGroup>

```typescript TypeScript
import { query } from "@anthropic-ai/claude-agent-sdk";

const customPrompt = `あなたはPythonコーディング専門家です。
以下のガイドラインに従ってください：
- クリーンで十分に文書化されたコードを書く
- すべての関数に型ヒントを使用
- 包括的なdocstringを含める
- 適切な場合は関数型プログラミングパターンを優先
- 常にコードの選択を説明する`;

const messages = [];

for await (const message of query({
  prompt: "データ処理パイプラインを作成してください",
  options: {
    systemPrompt: customPrompt,
  },
})) {
  messages.push(message);
  if (message.type === "assistant") {
    console.log(message.message.content);
  }
}
```

```python Python
from claude_agent_sdk import query, ClaudeAgentOptions

custom_prompt = """あなたはPythonコーディング専門家です。
以下のガイドラインに従ってください：
- クリーンで十分に文書化されたコードを書く
- すべての関数に型ヒントを使用
- 包括的なdocstringを含める
- 適切な場合は関数型プログラミングパターンを優先
- 常にコードの選択を説明する"""

messages = []

async for message in query(
    prompt="データ処理パイプラインを作成してください",
    options=ClaudeAgentOptions(
        system_prompt=custom_prompt
    )
):
    messages.append(message)
    if message.type == 'assistant':
        print(message.message.content)
```

</CodeGroup>

## 4つのアプローチすべての比較

| 機能                 | CLAUDE.md           | 出力スタイル      | appendを使用した`systemPrompt` | カスタム`systemPrompt`     |
| --- | --- | --- | --- | --- |
| **永続性**         | プロジェクトごとのファイル | ファイルとして保存  | セッションのみ            | セッションのみ           |
| **再利用性**         | プロジェクトごと      | プロジェクトをまたいで | コードの重複        | コードの重複       |
| **管理**          | ファイルシステム上    | CLI + ファイル     | コード内                 | コード内                |
| **デフォルトツール**       | 保持        | 保持       | 保持               | 失われる（含めない限り） |
| **組み込み安全性**     | 維持       | 維持      | 維持              | 追加する必要がある          |
| **環境コンテキスト** | 自動        | 自動       | 自動               | 提供する必要がある       |
| **カスタマイズレベル** | 追加のみ   | デフォルトを置換 | 追加のみ          | 完全な制御       |
| **バージョン管理**     | プロジェクトと共に     | はい             | コードと共に               | コードと共に              |
| **スコープ**               | プロジェクト固有 | ユーザーまたはプロジェクト | コードセッション            | コードセッション           |

**注意：** 「appendを使用」とは、TypeScriptで`systemPrompt: { type: "preset", preset: "claude_code", append: "..." }`、Pythonで`system_prompt={"type": "preset", "preset": "claude_code", "append": "..."}`を使用することを意味します。

## 使用例とベストプラクティス

### CLAUDE.mdを使用するタイミング

**最適な用途：**

- プロジェクト固有のコーディング標準と規約
- プロジェクト構造とアーキテクチャの文書化
- 一般的なコマンドのリスト（ビルド、テスト、デプロイ）
- バージョン管理すべきチーム共有コンテキスト
- プロジェクト内のすべてのSDK使用に適用される指示

**例：**

- 「すべてのAPIエンドポイントはasync/awaitパターンを使用すべき」
- 「コミット前に`npm run lint:fix`を実行」
- 「データベースマイグレーションは`migrations/`ディレクトリにある」

**重要：** CLAUDE.mdファイルを読み込むには、`settingSources: ['project']`（TypeScript）または`setting_sources=["project"]`（Python）を明示的に設定する必要があります。`claude_code`システムプロンプトプリセットは、この設定なしにはCLAUDE.mdを自動的に読み込みません。

### 出力スタイルを使用するタイミング

**最適な用途：**

- セッションをまたいだ永続的な動作変更
- チーム共有設定
- 専門アシスタント（コードレビュアー、データサイエンティスト、DevOps）
- バージョン管理が必要な複雑なプロンプト変更

**例：**

- 専用のSQL最適化アシスタントの作成
- セキュリティ重視のコードレビュアーの構築
- 特定の教育法を持つ教育アシスタントの開発

### appendを使用した`systemPrompt`を使用するタイミング

**最適な用途：**

- 特定のコーディング標準や好みの追加
- 出力フォーマットのカスタマイズ
- ドメイン固有の知識の追加
- 応答の詳細度の変更
- ツール指示を失うことなくClaude Codeのデフォルト動作を強化

### カスタム`systemPrompt`を使用するタイミング

**最適な用途：**

- Claudeの動作の完全な制御
- 専門的な単一セッションタスク
- 新しいプロンプト戦略のテスト
- デフォルトツールが不要な状況
- 独特な動作を持つ専門エージェントの構築

## アプローチの組み合わせ

最大限の柔軟性のために、これらの方法を組み合わせることができます：

### 例：セッション固有の追加を持つ出力スタイル

<CodeGroup>

```typescript TypeScript
import { query } from "@anthropic-ai/claude-agent-sdk";

// "Code Reviewer"出力スタイルがアクティブであると仮定（/output-styleを通じて）
// セッション固有の焦点領域を追加
const messages = [];

for await (const message of query({
  prompt: "この認証モジュールをレビューしてください",
  options: {
    systemPrompt: {
      type: "preset",
      preset: "claude_code",
      append: `
        このレビューでは、以下を優先してください：
        - OAuth 2.0準拠
        - トークンストレージセキュリティ
        - セッション管理
      `,
    },
  },
})) {
  messages.push(message);
}
```

```python Python
from claude_agent_sdk import query, ClaudeAgentOptions

# "Code Reviewer"出力スタイルがアクティブであると仮定（/output-styleを通じて）
# セッション固有の焦点領域を追加
messages = []

async for message in query(
    prompt="この認証モジュールをレビューしてください",
    options=ClaudeAgentOptions(
        system_prompt={
            "type": "preset",
            "preset": "claude_code",
            "append": """
            このレビューでは、以下を優先してください：
            - OAuth 2.0準拠
            - トークンストレージセキュリティ
            - セッション管理
            """
        }
    )
):
    messages.append(message)
```

</CodeGroup>

## 関連項目

- [出力スタイル](https://code.claude.com/docs/output-styles) - 完全な出力スタイルドキュメント
- [TypeScript SDKガイド](/docs/ja/agent-sdk/typescript) - 完全なSDK使用ガイド
- [TypeScript SDKリファレンス](https://code.claude.com/docs/typescript-sdk-reference) - 完全なAPIドキュメント
- [設定ガイド](https://code.claude.com/docs/configuration) - 一般的な設定オプション