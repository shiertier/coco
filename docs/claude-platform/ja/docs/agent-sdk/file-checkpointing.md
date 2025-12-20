# チェックポイント機能でファイルの変更を巻き戻す

エージェントセッション中のファイル変更を追跡し、ファイルを以前の任意の状態に復元します

---

ファイルチェックポイント機能は、エージェントセッション中にWrite、Edit、NotebookEditツールを通じて行われたファイル変更を追跡し、ファイルを以前の任意の状態に巻き戻すことができます。試してみたいですか？[インタラクティブな例](#try-it-out)にジャンプしてください。

チェックポイント機能を使用すると、以下のことができます：

- **不要な変更を元に戻す** - ファイルを既知の良好な状態に復元することで、不要な変更を取り消します
- **代替案を探索する** - チェックポイントに復元して、別のアプローチを試します
- **エラーから回復する** - エージェントが不正な変更を行った場合に対応します

<Warning>
Write、Edit、NotebookEditツールを通じて行われた変更のみが追跡されます。Bashコマンド（`echo > file.txt`や`sed -i`など）を通じて行われた変更は、チェックポイントシステムでは記録されません。
</Warning>

## チェックポイント機能の仕組み

ファイルチェックポイント機能を有効にすると、SDKはWrite、Edit、またはNotebookEditツールを通じてファイルを変更する前に、ファイルのバックアップを作成します。レスポンスストリーム内のユーザーメッセージには、復元ポイントとして使用できるチェックポイントUUIDが含まれます。

チェックポイント機能は、エージェントがファイルを変更するために使用する以下の組み込みツールで機能します：

| ツール | 説明 |
|------|-------------|
| Write | 新しいファイルを作成するか、既存のファイルを新しいコンテンツで上書きします |
| Edit | 既存ファイルの特定の部分に対して、ターゲット指定された編集を行います |
| NotebookEdit | Jupyterノートブック（`.ipynb`ファイル）のセルを変更します |

<Note>
ファイルの巻き戻しは、ディスク上のファイルを以前の状態に復元します。会話自体は巻き戻しません。`rewindFiles()`（TypeScript）または`rewind_files()`（Python）を呼び出した後も、会話履歴とコンテキストは変わりません。
</Note>

チェックポイントシステムは以下を追跡します：

- セッション中に作成されたファイル
- セッション中に変更されたファイル
- 変更されたファイルの元のコンテンツ

チェックポイントに巻き戻すと、作成されたファイルは削除され、変更されたファイルはその時点でのコンテンツに復元されます。

## チェックポイント機能の実装

ファイルチェックポイント機能を使用するには、オプションで有効にし、レスポンスストリームからチェックポイントUUIDをキャプチャして、復元が必要な場合に`rewindFiles()`（TypeScript）または`rewind_files()`（Python）を呼び出します。

次の例は、完全なフロー（チェックポイント機能を有効にし、レスポンスストリームからチェックポイントUUIDとセッションIDをキャプチャして、後でセッションを再開してファイルを巻き戻す）を示しています。各ステップについては、以下で詳しく説明します。

<CodeGroup>

```python Python
import asyncio
import os
from claude_agent_sdk import ClaudeSDKClient, ClaudeAgentOptions, UserMessage, ResultMessage

async def main():
    # ステップ1：チェックポイント機能を有効にする
    options = ClaudeAgentOptions(
        enable_file_checkpointing=True,
        permission_mode="acceptEdits",  # プロンプトなしでファイル編集を自動承認
        extra_args={"replay-user-messages": None},  # レスポンスストリームでチェックポイントUUIDを受け取るために必須
        env={**os.environ, "CLAUDE_CODE_ENABLE_SDK_FILE_CHECKPOINTING": "1"}
    )

    checkpoint_id = None
    session_id = None

    # クエリを実行し、チェックポイントUUIDとセッションIDをキャプチャする
    async with ClaudeSDKClient(options) as client:
        await client.query("Refactor the authentication module")

        # ステップ2：最初のユーザーメッセージからチェックポイントUUIDをキャプチャする
        async for message in client.receive_response():
            if isinstance(message, UserMessage) and message.uuid and not checkpoint_id:
                checkpoint_id = message.uuid
            if isinstance(message, ResultMessage) and not session_id:
                session_id = message.session_id

    # ステップ3：後で、空のプロンプトでセッションを再開して巻き戻す
    if checkpoint_id and session_id:
        async with ClaudeSDKClient(ClaudeAgentOptions(
            enable_file_checkpointing=True,
            resume=session_id
        )) as client:
            await client.query("")  # 接続を開くための空のプロンプト
            async for message in client.receive_response():
                await client.rewind_files(checkpoint_id)
                break
        print(f"Rewound to checkpoint: {checkpoint_id}")

asyncio.run(main())
```

```typescript TypeScript
import { query } from "@anthropic-ai/claude-agent-sdk";

async function main() {
  // ステップ1：チェックポイント機能を有効にする
  const opts = {
    enableFileCheckpointing: true,
    permissionMode: "acceptEdits" as const,  // プロンプトなしでファイル編集を自動承認
    extraArgs: { 'replay-user-messages': null },  // レスポンスストリームでチェックポイントUUIDを受け取るために必須
    env: { ...process.env, CLAUDE_CODE_ENABLE_SDK_FILE_CHECKPOINTING: '1' }
  };

  const response = query({
    prompt: "Refactor the authentication module",
    options: opts
  });

  let checkpointId: string | undefined;
  let sessionId: string | undefined;

  // ステップ2：最初のユーザーメッセージからチェックポイントUUIDをキャプチャする
  for await (const message of response) {
    if (message.type === 'user' && message.uuid && !checkpointId) {
      checkpointId = message.uuid;
    }
    if ('session_id' in message && !sessionId) {
      sessionId = message.session_id;
    }
  }

  // ステップ3：後で、空のプロンプトでセッションを再開して巻き戻す
  if (checkpointId && sessionId) {
    const rewindQuery = query({
      prompt: "",  // 接続を開くための空のプロンプト
      options: { ...opts, resume: sessionId }
    });

    for await (const msg of rewindQuery) {
      await rewindQuery.rewindFiles(checkpointId);
      break;
    }
    console.log(`Rewound to checkpoint: ${checkpointId}`);
  }
}

main();
```

</CodeGroup>

<Steps>

<Step title="環境変数を設定する">

ファイルチェックポイント機能には、`CLAUDE_CODE_ENABLE_SDK_FILE_CHECKPOINTING`環境変数が必要です。スクリプトを実行する前にコマンドラインで設定するか、SDKオプションで直接設定できます。

**オプション1：コマンドラインで設定する**

```bash Python
export CLAUDE_CODE_ENABLE_SDK_FILE_CHECKPOINTING=1
```

**オプション2：SDKオプションで設定する**

SDKを設定する際に、`env`オプションを通じて環境変数を渡します：

<CodeGroup>

```python Python
import os

options = ClaudeAgentOptions(
    enable_file_checkpointing=True,
    env={**os.environ, "CLAUDE_CODE_ENABLE_SDK_FILE_CHECKPOINTING": "1"}
)
```

```typescript TypeScript
const opts = {
  enableFileCheckpointing: true,
  env: { ...process.env, CLAUDE_CODE_ENABLE_SDK_FILE_CHECKPOINTING: '1' }
};
```

</CodeGroup>

</Step>

<Step title="チェックポイント機能を有効にする">

SDKオプションを設定して、チェックポイント機能を有効にし、チェックポイントUUIDを受け取ります：

| オプション | Python | TypeScript | 説明 |
|--------|--------|------------|-------------|
| チェックポイント機能を有効にする | `enable_file_checkpointing=True` | `enableFileCheckpointing: true` | 巻き戻しのためにファイル変更を追跡します |
| チェックポイントUUIDを受け取る | `extra_args={"replay-user-messages": None}` | `extraArgs: { 'replay-user-messages': null }` | ストリーム内でユーザーメッセージUUIDを取得するために必須です |

<CodeGroup>

```python Python
options = ClaudeAgentOptions(
    enable_file_checkpointing=True,
    permission_mode="acceptEdits",
    extra_args={"replay-user-messages": None}
)

async with ClaudeSDKClient(options) as client:
    await client.query("Refactor the authentication module")
```

```typescript TypeScript
const response = query({
  prompt: "Refactor the authentication module",
  options: {
    enableFileCheckpointing: true,
    permissionMode: "acceptEdits" as const,
    extraArgs: { 'replay-user-messages': null }
  }
});
```

</CodeGroup>

</Step>

<Step title="チェックポイントUUIDとセッションIDをキャプチャする">

`replay-user-messages`オプションが設定されている場合（上記を参照）、レスポンスストリーム内の各ユーザーメッセージにはチェックポイントとして機能するUUIDがあります。

ほとんどのユースケースでは、最初のユーザーメッセージUUID（`message.uuid`）をキャプチャします。これに巻き戻すと、すべてのファイルが元の状態に復元されます。複数のチェックポイントを保存して中間状態に巻き戻すには、[複数の復元ポイント](#multiple-restore-points)を参照してください。

セッションID（`message.session_id`）のキャプチャはオプションです。ストリームが完了した後に巻き戻したい場合にのみ必要です。[リスキーな操作の前のチェックポイント](#checkpoint-before-risky-operations)の例のように、メッセージの処理中に`rewindFiles()`をすぐに呼び出す場合は、セッションIDのキャプチャをスキップできます。

<CodeGroup>

```python Python
checkpoint_id = None
session_id = None

async for message in client.receive_response():
    # 各ユーザーメッセージでチェックポイントを更新（最新のものを保持）
    if isinstance(message, UserMessage) and message.uuid:
        checkpoint_id = message.uuid
    # 結果メッセージからセッションIDをキャプチャ
    if isinstance(message, ResultMessage):
        session_id = message.session_id
```

```typescript TypeScript
let checkpointId: string | undefined;
let sessionId: string | undefined;

for await (const message of response) {
  // 各ユーザーメッセージでチェックポイントを更新（最新のものを保持）
  if (message.type === 'user' && message.uuid) {
    checkpointId = message.uuid;
  }
  // セッションIDを持つメッセージからセッションIDをキャプチャ
  if ('session_id' in message) {
    sessionId = message.session_id;
  }
}
```

</CodeGroup>

</Step>

<Step title="ファイルを巻き戻す">

ストリームが完了した後に巻き戻すには、空のプロンプトでセッションを再開し、チェックポイントUUIDを使用して`rewind_files()`（Python）または`rewindFiles()`（TypeScript）を呼び出します。ストリーム中に巻き戻すこともできます。そのパターンについては、[リスキーな操作の前のチェックポイント](#checkpoint-before-risky-operations)を参照してください。

<CodeGroup>

```python Python
async with ClaudeSDKClient(ClaudeAgentOptions(
    enable_file_checkpointing=True,
    resume=session_id
)) as client:
    await client.query("")  # 接続を開くための空のプロンプト
    async for message in client.receive_response():
        await client.rewind_files(checkpoint_id)
        break
```

```typescript TypeScript
const rewindQuery = query({
  prompt: "",  // 接続を開くための空のプロンプト
  options: { ...opts, resume: sessionId }
});

for await (const msg of rewindQuery) {
  await rewindQuery.rewindFiles(checkpointId);
  break;
}
```

</CodeGroup>

セッションIDとチェックポイントIDをキャプチャした場合、CLIから巻き戻すこともできます：

```bash
claude --resume <session-id> --rewind-files <checkpoint-uuid>
```

</Step>

</Steps>

## 一般的なパターン

これらのパターンは、ユースケースに応じてチェックポイントUUIDをキャプチャして使用するさまざまな方法を示しています。

### リスキーな操作の前のチェックポイント

このパターンは最新のチェックポイントUUIDのみを保持し、各エージェントターンの前に更新します。処理中に何か問題が発生した場合、最後の安全な状態にすぐに巻き戻して、ループから抜け出すことができます。

<CodeGroup>

```python Python
import asyncio
import os
from claude_agent_sdk import ClaudeSDKClient, ClaudeAgentOptions, UserMessage

async def main():
    options = ClaudeAgentOptions(
        enable_file_checkpointing=True,
        permission_mode="acceptEdits",
        extra_args={"replay-user-messages": None},
        env={**os.environ, "CLAUDE_CODE_ENABLE_SDK_FILE_CHECKPOINTING": "1"}
    )

    safe_checkpoint = None

    async with ClaudeSDKClient(options) as client:
        await client.query("Refactor the authentication module")

        async for message in client.receive_response():
            # 各エージェントターンが開始する前にチェックポイントを更新
            # これは前のチェックポイントを上書きします。最新のものだけを保持
            if isinstance(message, UserMessage) and message.uuid:
                safe_checkpoint = message.uuid

            # 独自のロジックに基づいて元に戻すかどうかを決定
            # 例：エラー検出、検証失敗、またはユーザー入力
            if your_revert_condition and safe_checkpoint:
                await client.rewind_files(safe_checkpoint)
                # 巻き戻し後、ループを終了します。ファイルは復元されます
                break

asyncio.run(main())
```

```typescript TypeScript
import { query } from "@anthropic-ai/claude-agent-sdk";

async function main() {
  const response = query({
    prompt: "Refactor the authentication module",
    options: {
      enableFileCheckpointing: true,
      permissionMode: "acceptEdits" as const,
      extraArgs: { 'replay-user-messages': null },
      env: { ...process.env, CLAUDE_CODE_ENABLE_SDK_FILE_CHECKPOINTING: '1' }
    }
  });

  let safeCheckpoint: string | undefined;

  for await (const message of response) {
    // 各エージェントターンが開始する前にチェックポイントを更新
    // これは前のチェックポイントを上書きします。最新のものだけを保持
    if (message.type === 'user' && message.uuid) {
      safeCheckpoint = message.uuid;
    }

    // 独自のロジックに基づいて元に戻すかどうかを決定
    // 例：エラー検出、検証失敗、またはユーザー入力
    if (yourRevertCondition && safeCheckpoint) {
      await response.rewindFiles(safeCheckpoint);
      // 巻き戻し後、ループを終了します。ファイルは復元されます
      break;
    }
  }
}

main();
```

</CodeGroup>

### 複数の復元ポイント

Claudeが複数のターンにわたって変更を行う場合、すべての方法で巻き戻すのではなく、特定のポイントに巻き戻したいかもしれません。例えば、Claudeがターン1でファイルをリファクタリングし、ターン2でテストを追加した場合、リファクタリングは保持したいが、テストは元に戻したいかもしれません。

このパターンは、すべてのチェックポイントUUIDをメタデータ付きの配列に保存します。セッションが完了した後、以前のチェックポイントに巻き戻すことができます：

<CodeGroup>

```python Python
import asyncio
import os
from dataclasses import dataclass
from datetime import datetime
from claude_agent_sdk import ClaudeSDKClient, ClaudeAgentOptions, UserMessage, ResultMessage

# より良い追跡のためにチェックポイントメタデータを保存
@dataclass
class Checkpoint:
    id: str
    description: str
    timestamp: datetime

async def main():
    options = ClaudeAgentOptions(
        enable_file_checkpointing=True,
        permission_mode="acceptEdits",
        extra_args={"replay-user-messages": None},
        env={**os.environ, "CLAUDE_CODE_ENABLE_SDK_FILE_CHECKPOINTING": "1"}
    )

    checkpoints = []
    session_id = None

    async with ClaudeSDKClient(options) as client:
        await client.query("Refactor the authentication module")

        async for message in client.receive_response():
            if isinstance(message, UserMessage) and message.uuid:
                checkpoints.append(Checkpoint(
                    id=message.uuid,
                    description=f"After turn {len(checkpoints) + 1}",
                    timestamp=datetime.now()
                ))
            if isinstance(message, ResultMessage) and not session_id:
                session_id = message.session_id

    # 後で：セッションを再開して、任意のチェックポイントに巻き戻す
    if checkpoints and session_id:
        target = checkpoints[0]  # 任意のチェックポイントを選択
        async with ClaudeSDKClient(ClaudeAgentOptions(
            enable_file_checkpointing=True,
            resume=session_id
        )) as client:
            await client.query("")  # 接続を開くための空のプロンプト
            async for message in client.receive_response():
                await client.rewind_files(target.id)
                break
        print(f"Rewound to: {target.description}")

asyncio.run(main())
```

```typescript TypeScript
import { query } from "@anthropic-ai/claude-agent-sdk";

// より良い追跡のためにチェックポイントメタデータを保存
interface Checkpoint {
  id: string;
  description: string;
  timestamp: Date;
}

async function main() {
  const opts = {
    enableFileCheckpointing: true,
    permissionMode: "acceptEdits" as const,
    extraArgs: { 'replay-user-messages': null },
    env: { ...process.env, CLAUDE_CODE_ENABLE_SDK_FILE_CHECKPOINTING: '1' }
  };

  const response = query({
    prompt: "Refactor the authentication module",
    options: opts
  });

  const checkpoints: Checkpoint[] = [];
  let sessionId: string | undefined;

  for await (const message of response) {
    if (message.type === 'user' && message.uuid) {
      checkpoints.push({
        id: message.uuid,
        description: `After turn ${checkpoints.length + 1}`,
        timestamp: new Date()
      });
    }
    if ('session_id' in message && !sessionId) {
      sessionId = message.session_id;
    }
  }

  // 後で：セッションを再開して、任意のチェックポイントに巻き戻す
  if (checkpoints.length > 0 && sessionId) {
    const target = checkpoints[0];  // 任意のチェックポイントを選択
    const rewindQuery = query({
      prompt: "",  // 接続を開くための空のプロンプト
      options: { ...opts, resume: sessionId }
    });

    for await (const msg of rewindQuery) {
      await rewindQuery.rewindFiles(target.id);
      break;
    }
    console.log(`Rewound to: ${target.description}`);
  }
}

main();
```

</CodeGroup>

## 試してみる

この完全な例は、小さなユーティリティファイルを作成し、エージェントにドキュメンテーションコメントを追加させ、変更を表示してから、巻き戻したいかどうかを尋ねます。

始める前に、[Claude Agent SDKがインストール](/docs/ja/agent-sdk/quickstart)されていることを確認してください。

<Steps>

<Step title="テストファイルを作成する">

`utils.py`（Python）または`utils.ts`（TypeScript）という新しいファイルを作成し、次のコードを貼り付けます：

<CodeGroup>

```python utils.py
def add(a, b):
    return a + b

def subtract(a, b):
    return a - b

def multiply(a, b):
    return a * b

def divide(a, b):
    if b == 0:
        raise ValueError("Cannot divide by zero")
    return a / b
```

```typescript utils.ts
export function add(a: number, b: number): number {
  return a + b;
}

export function subtract(a: number, b: number): number {
  return a - b;
}

export function multiply(a: number, b: number): number {
  return a * b;
}

export function divide(a: number, b: number): number {
  if (b === 0) {
    throw new Error("Cannot divide by zero");
  }
  return a / b;
}
```

</CodeGroup>

</Step>

<Step title="インタラクティブな例を実行する">

ユーティリティファイルと同じディレクトリに`try_checkpointing.py`（Python）または`try_checkpointing.ts`（TypeScript）という新しいファイルを作成し、次のコードを貼り付けます。

このスクリプトはClaudeにユーティリティファイルにドキュメンテーションコメントを追加するよう要求し、その後、巻き戻して元の状態に復元するオプションを提供します。

<CodeGroup>

```python try_checkpointing.py
import asyncio
from claude_agent_sdk import ClaudeSDKClient, ClaudeAgentOptions, UserMessage, ResultMessage

async def main():
    # チェックポイント機能を有効にしてSDKを設定
    # - enable_file_checkpointing：巻き戻しのためにファイル変更を追跡
    # - permission_mode：プロンプトなしでファイル編集を自動承認
    # - extra_args：ストリーム内でユーザーメッセージUUIDを受け取るために必須
    options = ClaudeAgentOptions(
        enable_file_checkpointing=True,
        permission_mode="acceptEdits",
        extra_args={"replay-user-messages": None}
    )

    checkpoint_id = None  # 巻き戻しのためのユーザーメッセージUUIDを保存
    session_id = None     # 後で再開するためのセッションIDを保存

    print("Running agent to add doc comments to utils.py...\n")

    # エージェントを実行し、レスポンスストリームからチェックポイントデータをキャプチャ
    async with ClaudeSDKClient(options) as client:
        await client.query("Add doc comments to utils.py")

        async for message in client.receive_response():
            # 最初のユーザーメッセージUUIDをキャプチャ - これが復元ポイント
            if isinstance(message, UserMessage) and message.uuid and not checkpoint_id:
                checkpoint_id = message.uuid
            # 後で再開できるようにセッションIDをキャプチャ
            if isinstance(message, ResultMessage):
                session_id = message.session_id

    print("Done! Open utils.py to see the added doc comments.\n")

    # ユーザーに変更を巻き戻したいかどうかを尋ねる
    if checkpoint_id and session_id:
        response = input("Rewind to remove the doc comments? (y/n): ")

        if response.lower() == "y":
            # セッションを再開して空のプロンプトで、その後巻き戻す
            async with ClaudeSDKClient(ClaudeAgentOptions(
                enable_file_checkpointing=True,
                resume=session_id
            )) as client:
                await client.query("")  # 空のプロンプトが接続を開く
                async for message in client.receive_response():
                    await client.rewind_files(checkpoint_id)  # ファイルを復元
                    break

            print("\n✓ File restored! Open utils.py to verify the doc comments are gone.")
        else:
            print("\nKept the modified file.")

asyncio.run(main())
```

```typescript try_checkpointing.ts
import { query } from "@anthropic-ai/claude-agent-sdk";
import * as readline from "readline";

async function main() {
  // チェックポイント機能を有効にしてSDKを設定
  // - enableFileCheckpointing：巻き戻しのためにファイル変更を追跡
  // - permissionMode：プロンプトなしでファイル編集を自動承認
  // - extraArgs：ストリーム内でユーザーメッセージUUIDを受け取るために必須
  const opts = {
    enableFileCheckpointing: true,
    permissionMode: "acceptEdits" as const,
    extraArgs: { 'replay-user-messages': null }
  };

  let sessionId: string | undefined;    // 後で再開するためのセッションIDを保存
  let checkpointId: string | undefined; // 巻き戻しのためのユーザーメッセージUUIDを保存

  console.log("Running agent to add doc comments to utils.ts...\n");

  // エージェントを実行し、レスポンスストリームからチェックポイントデータをキャプチャ
  const response = query({
    prompt: "Add doc comments to utils.ts",
    options: opts
  });

  for await (const message of response) {
    // 最初のユーザーメッセージUUIDをキャプチャ - これが復元ポイント
    if (message.type === "user" && message.uuid && !checkpointId) {
      checkpointId = message.uuid;
    }
    // 後で再開できるようにセッションIDをキャプチャ
    if ("session_id" in message) {
      sessionId = message.session_id;
    }
  }

  console.log("Done! Open utils.ts to see the added doc comments.\n");

  // ユーザーに変更を巻き戻したいかどうかを尋ねる
  if (checkpointId && sessionId) {
    const rl = readline.createInterface({
      input: process.stdin,
      output: process.stdout
    });

    const answer = await new Promise<string>((resolve) => {
      rl.question("Rewind to remove the doc comments? (y/n): ", resolve);
    });
    rl.close();

    if (answer.toLowerCase() === "y") {
      // セッションを再開して空のプロンプトで、その後巻き戻す
      const rewindQuery = query({
        prompt: "",  // 空のプロンプトが接続を開く
        options: { ...opts, resume: sessionId }
      });

      for await (const msg of rewindQuery) {
        await rewindQuery.rewindFiles(checkpointId);  // ファイルを復元
        break;
      }

      console.log("\n✓ File restored! Open utils.ts to verify the doc comments are gone.");
    } else {
      console.log("\nKept the modified file.");
    }
  }
}

main();
```

</CodeGroup>

この例は、完全なチェックポイント機能のワークフローを示しています：

1. **チェックポイント機能を有効にする**：`enable_file_checkpointing=True`と`permission_mode="acceptEdits"`でSDKを設定して、ファイル編集を自動承認
2. **チェックポイントデータをキャプチャ**：エージェントが実行されている間、最初のユーザーメッセージUUID（復元ポイント）とセッションIDを保存
3. **巻き戻しを促す**：エージェントが完了した後、ユーティリティファイルを確認してドキュメンテーションコメントを見て、変更を元に戻したいかどうかを決定
4. **再開して巻き戻す**：はいの場合、空のプロンプトでセッションを再開し、`rewind_files()`を呼び出して元のファイルを復元

</Step>

<Step title="例を実行する">

環境変数を設定し、ユーティリティファイルと同じディレクトリからスクリプトを実行します。

<Tip>
スクリプトを実行する前に、IDEまたはエディタでユーティリティファイル（`utils.py`または`utils.ts`）を開いてください。エージェントがドキュメンテーションコメントを追加するときにファイルがリアルタイムで更新され、巻き戻しを選択すると元の状態に戻ります。
</Tip>

<CodeGroup>

```bash Python
export CLAUDE_CODE_ENABLE_SDK_FILE_CHECKPOINTING=1
python try_checkpointing.py
```

```bash TypeScript
export CLAUDE_CODE_ENABLE_SDK_FILE_CHECKPOINTING=1
npx tsx try_checkpointing.ts
```

</CodeGroup>

エージェントがドキュメンテーションコメントを追加し、巻き戻したいかどうかを尋ねるプロンプトが表示されます。はいを選択すると、ファイルは元の状態に復元されます。

</Step>

</Steps>

## 制限事項

ファイルチェックポイント機能には、以下の制限があります：

| 制限事項 | 説明 |
|------------|-------------|
| Write/Edit/NotebookEditツールのみ | Bashコマンドを通じて行われた変更は追跡されません |
| 同じセッション | チェックポイントは、それを作成したセッションに関連付けられています |
| ファイルコンテンツのみ | ディレクトリの作成、移動、または削除は、巻き戻しによって元に戻されません |
| ローカルファイル | リモートまたはネットワークファイルは追跡されません |

## トラブルシューティング

### チェックポイント機能のオプションが認識されない

`enableFileCheckpointing`または`rewindFiles()`が利用できない場合、古いSDKバージョンを使用している可能性があります。

**解決策**：最新のSDKバージョンに更新します：
- **Python**：`pip install --upgrade claude-agent-sdk`
- **TypeScript**：`npm install @anthropic-ai/claude-agent-sdk@latest`

### ユーザーメッセージにUUIDがない

`message.uuid`が`undefined`または欠落している場合、チェックポイントUUIDを受け取っていません。

**原因**：`replay-user-messages`オプションが設定されていません。

**解決策**：オプションに`extra_args={"replay-user-messages": None}`（Python）または`extraArgs: { 'replay-user-messages': null }`（TypeScript）を追加します。

### 「No file checkpoint found for message」エラー

このエラーは、指定されたユーザーメッセージUUIDのチェックポイントデータが存在しない場合に発生します。

**一般的な原因**：
- `CLAUDE_CODE_ENABLE_SDK_FILE_CHECKPOINTING`環境変数が設定されていない
- セッションが再開して巻き戻しを試みる前に適切に完了していない

**解決策**：環境変数が設定されていることを確認し（[環境変数を設定する](#set-the-environment-variable)を参照）、例に示されているパターンを使用します：最初のユーザーメッセージUUIDをキャプチャし、セッションを完全に完了してから、空のプロンプトで再開し、`rewindFiles()`を1回呼び出します。

### 「ProcessTransport is not ready for writing」エラー

このエラーは、レスポンスの反復処理が完了した後に`rewindFiles()`または`rewind_files()`を呼び出した場合に発生します。ループが完了すると、CLIプロセスへの接続が閉じられます。

**解決策**：空のプロンプトでセッションを再開し、新しいクエリで巻き戻します：

<CodeGroup>

```python Python
# 空のプロンプトでセッションを再開し、その後巻き戻す
async with ClaudeSDKClient(ClaudeAgentOptions(
    enable_file_checkpointing=True,
    resume=session_id
)) as client:
    await client.query("")
    async for message in client.receive_response():
        await client.rewind_files(checkpoint_id)
        break
```

```typescript TypeScript
// 空のプロンプトでセッションを再開し、その後巻き戻す
const rewindQuery = query({
  prompt: "",
  options: { ...opts, resume: sessionId }
});

for await (const msg of rewindQuery) {
  await rewindQuery.rewindFiles(checkpointId);
  break;
}
```

</CodeGroup>

## 次のステップ

- **[セッション](/docs/ja/agent-sdk/sessions)**：セッションを再開する方法を学びます。これはストリームが完了した後に巻き戻すために必要です。セッションID、会話の再開、セッションフォーキングについて説明します。
- **[権限](/docs/ja/agent-sdk/permissions)**：Claudeが使用できるツールと、ファイル変更がどのように承認されるかを設定します。編集がいつ発生するかについてより多くの制御が必要な場合に便利です。
- **[TypeScript SDKリファレンス](/docs/ja/agent-sdk/typescript)**：`query()`と`rewindFiles()`メソッドのすべてのオプションを含む完全なAPIリファレンス。
- **[Python SDKリファレンス](/docs/ja/agent-sdk/python)**：`ClaudeAgentOptions`と`rewind_files()`メソッドのすべてのオプションを含む完全なAPIリファレンス。