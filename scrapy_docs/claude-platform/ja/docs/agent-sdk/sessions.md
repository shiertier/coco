# セッション管理

Claude Agent SDKがセッションとセッション再開をどのように処理するかを理解する

---

# セッション管理

Claude Agent SDKは、会話状態とその再開を処理するためのセッション管理機能を提供します。セッションを使用すると、完全なコンテキストを維持しながら、複数のインタラクション全体で会話を続行できます。

## セッションの仕組み

新しいクエリを開始すると、SDKは自動的にセッションを作成し、初期システムメッセージでセッションIDを返します。このIDをキャプチャして、後でセッションを再開できます。

### セッションIDの取得

<CodeGroup>

```typescript TypeScript
import { query } from "@anthropic-ai/claude-agent-sdk"

let sessionId: string | undefined

const response = query({
  prompt: "Help me build a web application",
  options: {
    model: "claude-sonnet-4-5"
  }
})

for await (const message of response) {
  // The first message is a system init message with the session ID
  if (message.type === 'system' && message.subtype === 'init') {
    sessionId = message.session_id
    console.log(`Session started with ID: ${sessionId}`)
    // You can save this ID for later resumption
  }

  // Process other messages...
  console.log(message)
}

// Later, you can use the saved sessionId to resume
if (sessionId) {
  const resumedResponse = query({
    prompt: "Continue where we left off",
    options: {
      resume: sessionId
    }
  })
}
```

```python Python
from claude_agent_sdk import query, ClaudeAgentOptions

session_id = None

async for message in query(
    prompt="Help me build a web application",
    options=ClaudeAgentOptions(
        model="claude-sonnet-4-5"
    )
):
    # The first message is a system init message with the session ID
    if hasattr(message, 'subtype') and message.subtype == 'init':
        session_id = message.data.get('session_id')
        print(f"Session started with ID: {session_id}")
        # You can save this ID for later resumption

    # Process other messages...
    print(message)

# Later, you can use the saved session_id to resume
if session_id:
    async for message in query(
        prompt="Continue where we left off",
        options=ClaudeAgentOptions(
            resume=session_id
        )
    ):
        print(message)
```

</CodeGroup>

## セッションの再開

SDKは以前の会話状態からセッションを再開することをサポートしており、継続的な開発ワークフローを実現します。`resume`オプションとセッションIDを使用して、以前の会話を続行します。

<CodeGroup>

```typescript TypeScript
import { query } from "@anthropic-ai/claude-agent-sdk"

// Resume a previous session using its ID
const response = query({
  prompt: "Continue implementing the authentication system from where we left off",
  options: {
    resume: "session-xyz", // Session ID from previous conversation
    model: "claude-sonnet-4-5",
    allowedTools: ["Read", "Edit", "Write", "Glob", "Grep", "Bash"]
  }
})

// The conversation continues with full context from the previous session
for await (const message of response) {
  console.log(message)
}
```

```python Python
from claude_agent_sdk import query, ClaudeAgentOptions

# Resume a previous session using its ID
async for message in query(
    prompt="Continue implementing the authentication system from where we left off",
    options=ClaudeAgentOptions(
        resume="session-xyz",  # Session ID from previous conversation
        model="claude-sonnet-4-5",
        allowed_tools=["Read", "Edit", "Write", "Glob", "Grep", "Bash"]
    )
):
    print(message)

# The conversation continues with full context from the previous session
```

</CodeGroup>

SDKは、セッションを再開するときに会話履歴とコンテキストの読み込みを自動的に処理し、Claudeが正確に中断したところから続行できるようにします。

<Tip>
セッション全体でファイルの変更を追跡し、元に戻すには、[ファイルチェックポイント](/docs/ja/agent-sdk/file-checkpointing)を参照してください。
</Tip>

## セッションのフォーク

セッションを再開するときに、元のセッションを続行するか、新しいブランチにフォークするかを選択できます。デフォルトでは、再開すると元のセッションが続行されます。`forkSession`オプション（TypeScript）または`fork_session`オプション（Python）を使用して、再開状態から開始する新しいセッションIDを作成します。

### セッションをフォークする場合

フォークは以下の場合に便利です：
- 同じ開始点から異なるアプローチを探索したい
- 元のセッションを変更せずに複数の会話ブランチを作成したい
- 元のセッション履歴に影響を与えずに変更をテストしたい
- 異なる実験用に個別の会話パスを維持したい

### フォークと継続の比較

| 動作 | `forkSession: false`（デフォルト） | `forkSession: true` |
|----------|-------------------------------|---------------------|
| **セッションID** | 元のセッションと同じ | 新しいセッションIDが生成される |
| **履歴** | 元のセッションに追加される | 再開ポイントから新しいブランチを作成 |
| **元のセッション** | 変更される | 変更されずに保持される |
| **ユースケース** | 線形会話を続行 | 代替案を探索するためにブランチ化 |

### 例：セッションのフォーク

<CodeGroup>

```typescript TypeScript
import { query } from "@anthropic-ai/claude-agent-sdk"

// First, capture the session ID
let sessionId: string | undefined

const response = query({
  prompt: "Help me design a REST API",
  options: { model: "claude-sonnet-4-5" }
})

for await (const message of response) {
  if (message.type === 'system' && message.subtype === 'init') {
    sessionId = message.session_id
    console.log(`Original session: ${sessionId}`)
  }
}

// Fork the session to try a different approach
const forkedResponse = query({
  prompt: "Now let's redesign this as a GraphQL API instead",
  options: {
    resume: sessionId,
    forkSession: true,  // Creates a new session ID
    model: "claude-sonnet-4-5"
  }
})

for await (const message of forkedResponse) {
  if (message.type === 'system' && message.subtype === 'init') {
    console.log(`Forked session: ${message.session_id}`)
    // This will be a different session ID
  }
}

// The original session remains unchanged and can still be resumed
const originalContinued = query({
  prompt: "Add authentication to the REST API",
  options: {
    resume: sessionId,
    forkSession: false,  // Continue original session (default)
    model: "claude-sonnet-4-5"
  }
})
```

```python Python
from claude_agent_sdk import query, ClaudeAgentOptions

# First, capture the session ID
session_id = None

async for message in query(
    prompt="Help me design a REST API",
    options=ClaudeAgentOptions(model="claude-sonnet-4-5")
):
    if hasattr(message, 'subtype') and message.subtype == 'init':
        session_id = message.data.get('session_id')
        print(f"Original session: {session_id}")

# Fork the session to try a different approach
async for message in query(
    prompt="Now let's redesign this as a GraphQL API instead",
    options=ClaudeAgentOptions(
        resume=session_id,
        fork_session=True,  # Creates a new session ID
        model="claude-sonnet-4-5"
    )
):
    if hasattr(message, 'subtype') and message.subtype == 'init':
        forked_id = message.data.get('session_id')
        print(f"Forked session: {forked_id}")
        # This will be a different session ID

# The original session remains unchanged and can still be resumed
async for message in query(
    prompt="Add authentication to the REST API",
    options=ClaudeAgentOptions(
        resume=session_id,
        fork_session=False,  # Continue original session (default)
        model="claude-sonnet-4-5"
    )
):
    print(message)
```

</CodeGroup>