# フックを使用してエージェントの動作をインターセプトして制御する

フックを使用して、実行の重要なポイントでエージェントの動作をインターセプトしてカスタマイズする

---

フックを使用すると、エージェント実行の重要なポイントでインターセプトして、検証、ログ記録、セキュリティ制御、またはカスタムロジックを追加できます。フックを使用すると、以下のことができます：

- **危険な操作をブロック**する：破壊的なシェルコマンドや不正なファイルアクセスなど、実行前に危険な操作をブロックします
- **ログと監査**：コンプライアンス、デバッグ、またはアナリティクスのために、すべてのツール呼び出しをログして監査します
- **入力と出力を変換**する：データをサニタイズしたり、認証情報を注入したり、ファイルパスをリダイレクトします
- **人間の承認を要求**する：データベース書き込みやAPI呼び出しなどの機密アクションに対して人間の承認を要求します
- **セッションライフサイクルを追跡**する：状態を管理し、リソースをクリーンアップし、通知を送信します

フックには2つの部分があります：

1. **コールバック関数**：フックが発火したときに実行されるロジック
2. **フック設定**：SDKにどのイベントをフックするか（例：`PreToolUse`）、どのツールをマッチするかを指示します

次の例は、エージェントが`.env`ファイルを変更するのをブロックします。まず、ファイルパスをチェックするコールバックを定義し、次にそれを`query()`に渡して、Write または Edit ツール呼び出しの前に実行します：

<CodeGroup>

```python Python
import asyncio
from claude_agent_sdk import query, ClaudeAgentOptions, HookMatcher

# ツール呼び出しの詳細を受け取るフックコールバックを定義する
async def protect_env_files(input_data, tool_use_id, context):
    # ツールの入力引数からファイルパスを抽出する
    file_path = input_data['tool_input'].get('file_path', '')
    file_name = file_path.split('/')[-1]

    # .envファイルをターゲットにしている場合、操作をブロックする
    if file_name == '.env':
        return {
            'hookSpecificOutput': {
                'hookEventName': input_data['hook_event_name'],
                'permissionDecision': 'deny',
                'permissionDecisionReason': 'Cannot modify .env files'
            }
        }

    # 空のオブジェクトを返して操作を許可する
    return {}

async def main():
    async for message in query(
        prompt="Update the database configuration",
        options=ClaudeAgentOptions(
            hooks={
                # PreToolUseイベントのフックを登録する
                # マッチャーはWrite と Edit ツール呼び出しのみにフィルタリングする
                'PreToolUse': [HookMatcher(matcher='Write|Edit', hooks=[protect_env_files])]
            }
        )
    ):
        print(message)

asyncio.run(main())
```

```typescript TypeScript
import { query, HookCallback, PreToolUseHookInput } from "@anthropic-ai/claude-agent-sdk";

// HookCallbackタイプでフックコールバックを定義する
const protectEnvFiles: HookCallback = async (input, toolUseID, { signal }) => {
  // 型安全性のために入力を特定のフック型にキャストする
  const preInput = input as PreToolUseHookInput;

  // ツールの入力引数からファイルパスを抽出する
  const filePath = preInput.tool_input?.file_path as string;
  const fileName = filePath?.split('/').pop();

  // .envファイルをターゲットにしている場合、操作をブロックする
  if (fileName === '.env') {
    return {
      hookSpecificOutput: {
        hookEventName: input.hook_event_name,
        permissionDecision: 'deny',
        permissionDecisionReason: 'Cannot modify .env files'
      }
    };
  }

  // 空のオブジェクトを返して操作を許可する
  return {};
};

for await (const message of query({
  prompt: "Update the database configuration",
  options: {
    hooks: {
      // PreToolUseイベントのフックを登録する
      // マッチャーはWrite と Edit ツール呼び出しのみにフィルタリングする
      PreToolUse: [{ matcher: 'Write|Edit', hooks: [protectEnvFiles] }]
    }
  }
})) {
  console.log(message);
}
```

</CodeGroup>

これは`PreToolUse`フックです。ツール実行前に実行され、ロジックに基づいて操作をブロックまたは許可できます。このガイドの残りの部分では、利用可能なすべてのフック、その設定オプション、および一般的なユースケースのパターンについて説明します。

## 利用可能なフック

SDKは、エージェント実行のさまざまなステージのフックを提供します。一部のフックは両方のSDKで利用可能ですが、他のフックはTypeScript専用です。これはPython SDKがそれらをサポートしていないためです。

| フックイベント | Python SDK | TypeScript SDK | トリガーされるもの | 使用例 |
|------------|------------|----------------|------------------|------------------|
| `PreToolUse` | はい | はい | ツール呼び出しリクエスト（ブロックまたは変更可能） | 危険なシェルコマンドをブロック |
| `PostToolUse` | はい | はい | ツール実行結果 | すべてのファイル変更を監査証跡にログ |
| `PostToolUseFailure` | いいえ | はい | ツール実行失敗 | ツールエラーを処理またはログ |
| `UserPromptSubmit` | はい | はい | ユーザープロンプト送信 | プロンプトに追加コンテキストを注入 |
| `Stop` | はい | はい | エージェント実行停止 | 終了前にセッション状態を保存 |
| `SubagentStart` | いいえ | はい | サブエージェント初期化 | 並列タスク生成を追跡 |
| `SubagentStop` | はい | はい | サブエージェント完了 | 並列タスクの結果を集約 |
| `PreCompact` | はい | はい | 会話圧縮リクエスト | 要約前に完全なトランスクリプトをアーカイブ |
| `PermissionRequest` | いいえ | はい | パーミッションダイアログが表示される | カスタムパーミッション処理 |
| `SessionStart` | いいえ | はい | セッション初期化 | ログとテレメトリを初期化 |
| `SessionEnd` | いいえ | はい | セッション終了 | 一時的なリソースをクリーンアップ |
| `Notification` | いいえ | はい | エージェントステータスメッセージ | エージェントステータス更新をSlackまたはPagerDutyに送信 |

## 一般的なユースケース

フックは多くの異なるシナリオを処理するのに十分な柔軟性があります。以下は、カテゴリ別に整理された最も一般的なパターンの一部です。

<Tabs>
  <Tab title="セキュリティ">
    - 危険なコマンドをブロック（`rm -rf /`、破壊的なSQL など）
    - 書き込み操作前にファイルパスを検証
    - ツール使用のための許可リスト/ブロックリストを適用
  </Tab>
  <Tab title="ログ記録">
    - すべてのエージェントアクションの監査証跡を作成
    - 実行メトリクスとパフォーマンスを追跡
    - 開発でエージェント動作をデバッグ
  </Tab>
  <Tab title="ツールインターセプション">
    - ファイル操作をサンドボックス化されたディレクトリにリダイレクト
    - 環境変数または認証情報を注入
    - ツール入力または出力を変換
  </Tab>
  <Tab title="認可">
    - ロールベースのアクセス制御を実装
    - 機密操作に対して人間の承認を要求
    - 特定のツール使用をレート制限
  </Tab>
</Tabs>

## フックを設定する

エージェントのフックを設定するには、`query()`を呼び出すときに`options.hooks`パラメータでフックを渡します：

<CodeGroup>

```python Python
async for message in query(
    prompt="Your prompt",
    options=ClaudeAgentOptions(
        hooks={
            'PreToolUse': [HookMatcher(matcher='Bash', hooks=[my_callback])]
        }
    )
):
    print(message)
```

```typescript TypeScript
for await (const message of query({
  prompt: "Your prompt",
  options: {
    hooks: {
      PreToolUse: [{ matcher: 'Bash', hooks: [myCallback] }]
    }
  }
})) {
  console.log(message);
}
```

</CodeGroup>

`hooks`オプションは辞書（Python）またはオブジェクト（TypeScript）です。ここで：
- **キー**は[フックイベント名](#available-hooks)（例：`'PreToolUse'`、`'PostToolUse'`、`'Stop'`）です
- **値**は[マッチャー](#matchers)の配列で、各マッチャーはオプションのフィルタパターンと[コールバック関数](#callback-function-inputs)を含みます

フックコールバック関数は、イベントについての[入力データ](#input-data)を受け取り、[レスポンス](#callback-outputs)を返して、エージェントに操作を許可、ブロック、または変更するよう指示します。

### マッチャー

マッチャーを使用して、どのツールがコールバックをトリガーするかをフィルタリングします：

| オプション | 型 | デフォルト | 説明 |
|--------|------|---------|-------------|
| `matcher` | `string` | `undefined` | ツール名をマッチするための正規表現パターン。組み込みツールには`Bash`、`Read`、`Write`、`Edit`、`Glob`、`Grep`、`WebFetch`、`Task`などが含まれます。MCPツールはパターン`mcp__<server>__<action>`を使用します。 |
| `hooks` | `HookCallback[]` | - | 必須。パターンがマッチしたときに実行するコールバック関数の配列 |
| `timeout` | `number` | `60` | タイムアウト（秒単位）。外部APIを呼び出すフックの場合は増加させます |

可能な限り`matcher`パターンを使用して特定のツールをターゲットにします。`'Bash'`のマッチャーはBashコマンドのみで実行され、パターンを省略するとすべてのツール呼び出しに対してコールバックが実行されます。マッチャーはツール名でのみフィルタリングすることに注意してください。ファイルパスまたは他の引数でフィルタリングするには、コールバック内で`tool_input.file_path`をチェックします。

マッチャーはツールベースのフック（`PreToolUse`、`PostToolUse`、`PostToolUseFailure`、`PermissionRequest`）にのみ適用されます。`Stop`、`SessionStart`、`Notification`などのライフサイクルフックの場合、マッチャーは無視され、そのタイプのすべてのイベントに対してフックが発火します。

<Tip>
**ツール名の発見：**セッション開始時の初期システムメッセージの`tools`配列をチェックするか、マッチャーなしでフックを追加してすべてのツール呼び出しをログします。

**MCPツール命名：** MCPツールは常に`mcp__`で始まり、その後にサーバー名とアクション：`mcp__<server>__<action>`が続きます。例えば、`playwright`という名前のサーバーを設定する場合、そのツールは`mcp__playwright__browser_screenshot`、`mcp__playwright__browser_click`などという名前になります。サーバー名は`mcpServers`設定で使用するキーから来ます。
</Tip>

この例は、マッチャーを使用して`PreToolUse`イベントが発火したときにファイル変更ツールのみでフックを実行します：

<CodeGroup>

```python Python
options = ClaudeAgentOptions(
    hooks={
        'PreToolUse': [
            HookMatcher(matcher='Write|Edit', hooks=[validate_file_path])
        ]
    }
)
```

```typescript TypeScript
const options = {
  hooks: {
    PreToolUse: [
      { matcher: 'Write|Edit', hooks: [validateFilePath] }
    ]
  }
};
```

</CodeGroup>

### コールバック関数の入力

すべてのフックコールバックは3つの引数を受け取ります：

1. **入力データ**（`dict` / `HookInput`）：イベント詳細。フィールドについては[入力データ](#input-data)を参照してください
2. **ツール使用ID**（`str | None` / `string | null`）：`PreToolUse`と`PostToolUse`イベントを相関させます
3. **コンテキスト**（`HookContext`）：TypeScriptでは、キャンセル用の`signal`プロパティ（`AbortSignal`）を含みます。`fetch()`などの非同期操作にこれを渡して、フックがタイムアウトした場合に自動的にキャンセルされるようにします。Pythonでは、この引数は将来の使用のために予約されています。

### 入力データ

フックコールバックの最初の引数には、イベントに関する情報が含まれています。フィールド名はSDK全体で同じです（両方ともsnake_caseを使用）。

**すべてのフックタイプに存在する共通フィールド：**

| フィールド | 型 | 説明 |
|-------|------|-------------|
| `hook_event_name` | `string` | フックタイプ（`PreToolUse`、`PostToolUse`など） |
| `session_id` | `string` | 現在のセッション識別子 |
| `transcript_path` | `string` | 会話トランスクリプトへのパス |
| `cwd` | `string` | 現在の作業ディレクトリ |

**フック固有フィールド**はフックタイプによって異なります。<sup>TS</sup>でマークされた項目はTypeScript SDKでのみ利用可能です：

| フィールド | 型 | 説明 | フック |
|-------|------|-------------|-------|
| `tool_name` | `string` | 呼び出されるツールの名前 | PreToolUse、PostToolUse、PostToolUseFailure<sup>TS</sup>、PermissionRequest<sup>TS</sup> |
| `tool_input` | `object` | ツールに渡される引数 | PreToolUse、PostToolUse、PostToolUseFailure<sup>TS</sup>、PermissionRequest<sup>TS</sup> |
| `tool_response` | `any` | ツール実行から返された結果 | PostToolUse |
| `error` | `string` | ツール実行失敗からのエラーメッセージ | PostToolUseFailure<sup>TS</sup> |
| `is_interrupt` | `boolean` | 失敗が割り込みによって引き起こされたかどうか | PostToolUseFailure<sup>TS</sup> |
| `prompt` | `string` | ユーザーのプロンプトテキスト | UserPromptSubmit |
| `stop_hook_active` | `boolean` | ストップフックが現在処理中かどうか | Stop、SubagentStop |
| `agent_id` | `string` | サブエージェントの一意の識別子 | SubagentStart<sup>TS</sup>、SubagentStop<sup>TS</sup> |
| `agent_type` | `string` | サブエージェントのタイプ/ロール | SubagentStart<sup>TS</sup> |
| `agent_transcript_path` | `string` | サブエージェントの会話トランスクリプトへのパス | SubagentStop<sup>TS</sup> |
| `trigger` | `string` | 圧縮をトリガーしたもの：`manual`または`auto` | PreCompact |
| `custom_instructions` | `string` | 圧縮用に提供されたカスタム指示 | PreCompact |
| `permission_suggestions` | `array` | ツールの推奨パーミッション更新 | PermissionRequest<sup>TS</sup> |
| `source` | `string` | セッションの開始方法：`startup`、`resume`、`clear`、または`compact` | SessionStart<sup>TS</sup> |
| `reason` | `string` | セッションが終了した理由：`clear`、`logout`、`prompt_input_exit`、`bypass_permissions_disabled`、または`other` | SessionEnd<sup>TS</sup> |
| `message` | `string` | エージェントからのステータスメッセージ | Notification<sup>TS</sup> |
| `notification_type` | `string` | 通知のタイプ：`permission_prompt`、`idle_prompt`、`auth_success`、または`elicitation_dialog` | Notification<sup>TS</sup> |
| `title` | `string` | エージェントによって設定されたオプションのタイトル | Notification<sup>TS</sup> |

以下のコードは、`tool_name`と`tool_input`を使用して各ツール呼び出しの詳細をログするフックコールバックを定義しています：

<CodeGroup>

```python Python
async def log_tool_calls(input_data, tool_use_id, context):
    if input_data['hook_event_name'] == 'PreToolUse':
        print(f"Tool: {input_data['tool_name']}")
        print(f"Input: {input_data['tool_input']}")
    return {}
```

```typescript TypeScript
const logToolCalls: HookCallback = async (input, toolUseID, { signal }) => {
  if (input.hook_event_name === 'PreToolUse') {
    const preInput = input as PreToolUseHookInput;
    console.log(`Tool: ${preInput.tool_name}`);
    console.log(`Input:`, preInput.tool_input);
  }
  return {};
};
```

</CodeGroup>

### コールバック出力

コールバック関数は、SDKに続行方法を指示するオブジェクトを返します。操作を変更せずに許可するには、空のオブジェクト`{}`を返します。操作をブロック、変更、またはコンテキストを追加するには、`hookSpecificOutput`フィールドを含む決定を含むオブジェクトを返します。

**トップレベルフィールド**（`hookSpecificOutput`の外）：

| フィールド | 型 | 説明 |
|-------|------|-------------|
| `continue` | `boolean` | このフック後にエージェントが続行するかどうか（デフォルト：`true`） |
| `stopReason` | `string` | `continue`が`false`の場合に表示されるメッセージ |
| `suppressOutput` | `boolean` | トランスクリプトからstdoutを非表示にする（デフォルト：`false`） |
| `systemMessage` | `string` | Claudeが見るための会話に注入されるメッセージ |

**`hookSpecificOutput`内のフィールド**：

| フィールド | 型 | フック | 説明 |
|-------|------|-------|-------------|
| `hookEventName` | `string` | すべて | 必須。現在のイベントをマッチするために`input.hook_event_name`を使用します |
| `permissionDecision` | `'allow'` \| `'deny'` \| `'ask'` | PreToolUse | ツールが実行されるかどうかを制御します |
| `permissionDecisionReason` | `string` | PreToolUse | 決定についてClaudeに表示される説明 |
| `updatedInput` | `object` | PreToolUse | 変更されたツール入力（`permissionDecision: 'allow'`が必要） |
| `additionalContext` | `string` | PostToolUse、UserPromptSubmit、SessionStart<sup>TS</sup>、SubagentStart<sup>TS</sup> | 会話に追加されるコンテキスト |

この例は`/etc`ディレクトリへの書き込み操作をブロックしながら、Claudeに安全なファイルプラクティスについて思い出させるシステムメッセージを注入します：

<CodeGroup>

```python Python
async def block_etc_writes(input_data, tool_use_id, context):
    file_path = input_data['tool_input'].get('file_path', '')

    if file_path.startswith('/etc'):
        return {
            # トップレベルフィールド：会話にガイダンスを注入する
            'systemMessage': 'Remember: system directories like /etc are protected.',
            # hookSpecificOutput：操作をブロックする
            'hookSpecificOutput': {
                'hookEventName': input_data['hook_event_name'],
                'permissionDecision': 'deny',
                'permissionDecisionReason': 'Writing to /etc is not allowed'
            }
        }
    return {}
```

```typescript TypeScript
const blockEtcWrites: HookCallback = async (input, toolUseID, { signal }) => {
  const filePath = (input as PreToolUseHookInput).tool_input?.file_path as string;

  if (filePath?.startsWith('/etc')) {
    return {
      // トップレベルフィールド：会話にガイダンスを注入する
      systemMessage: 'Remember: system directories like /etc are protected.',
      // hookSpecificOutput：操作をブロックする
      hookSpecificOutput: {
        hookEventName: input.hook_event_name,
        permissionDecision: 'deny',
        permissionDecisionReason: 'Writing to /etc is not allowed'
      }
    };
  }
  return {};
};
```

</CodeGroup>

#### パーミッション決定フロー

複数のフックまたはパーミッションルールが適用される場合、SDKはこの順序で評価します：

1. **Deny**ルールが最初にチェックされます（いずれかがマッチ = 即座に拒否）。
2. **Ask**ルールが2番目にチェックされます。
3. **Allow**ルールが3番目にチェックされます。
4. **デフォルトはAsk**です（何もマッチしない場合）。

フックが`deny`を返す場合、操作はブロックされます。`allow`を返す他のフックはそれをオーバーライドしません。

#### ツールをブロックする

deny決定を返してツール実行を防止します：

<CodeGroup>

```python Python
async def block_dangerous_commands(input_data, tool_use_id, context):
    if input_data['hook_event_name'] != 'PreToolUse':
        return {}

    command = input_data['tool_input'].get('command', '')

    if 'rm -rf /' in command:
        return {
            'hookSpecificOutput': {
                'hookEventName': input_data['hook_event_name'],
                'permissionDecision': 'deny',
                'permissionDecisionReason': 'Dangerous command blocked: rm -rf /'
            }
        }
    return {}
```

```typescript TypeScript
const blockDangerousCommands: HookCallback = async (input, toolUseID, { signal }) => {
  if (input.hook_event_name !== 'PreToolUse') return {};

  const command = (input as PreToolUseHookInput).tool_input.command as string;

  if (command?.includes('rm -rf /')) {
    return {
      hookSpecificOutput: {
        hookEventName: input.hook_event_name,
        permissionDecision: 'deny',
        permissionDecisionReason: 'Dangerous command blocked: rm -rf /'
      }
    };
  }
  return {};
};
```

</CodeGroup>

#### ツール入力を変更する

更新された入力を返してツールが受け取るものを変更します：

<CodeGroup>

```python Python
async def redirect_to_sandbox(input_data, tool_use_id, context):
    if input_data['hook_event_name'] != 'PreToolUse':
        return {}

    if input_data['tool_name'] == 'Write':
        original_path = input_data['tool_input'].get('file_path', '')
        return {
            'hookSpecificOutput': {
                'hookEventName': input_data['hook_event_name'],
                'permissionDecision': 'allow',
                'updatedInput': {
                    **input_data['tool_input'],
                    'file_path': f'/sandbox{original_path}'
                }
            }
        }
    return {}
```

```typescript TypeScript
const redirectToSandbox: HookCallback = async (input, toolUseID, { signal }) => {
  if (input.hook_event_name !== 'PreToolUse') return {};

  const preInput = input as PreToolUseHookInput;
  if (preInput.tool_name === 'Write') {
    const originalPath = preInput.tool_input.file_path as string;
    return {
      hookSpecificOutput: {
        hookEventName: input.hook_event_name,
        permissionDecision: 'allow',
        updatedInput: {
          ...preInput.tool_input,
          file_path: `/sandbox${originalPath}`
        }
      }
    };
  }
  return {};
};
```

</CodeGroup>

<Note>
`updatedInput`を使用する場合、`permissionDecision`も含める必要があります。常に新しいオブジェクトを返し、元の`tool_input`を変更しないでください。
</Note>

#### システムメッセージを追加する

会話にコンテキストを注入します：

<CodeGroup>

```python Python
async def add_security_reminder(input_data, tool_use_id, context):
    return {
        'systemMessage': 'Remember to follow security best practices.'
    }
```

```typescript TypeScript
const addSecurityReminder: HookCallback = async (input, toolUseID, { signal }) => {
  return {
    systemMessage: 'Remember to follow security best practices.'
  };
};
```

</CodeGroup>

#### 特定のツールを自動承認する

信頼できるツールのパーミッションプロンプトをバイパスします。これは、特定の操作がユーザー確認なしで実行されるようにしたい場合に便利です：

<CodeGroup>

```python Python
async def auto_approve_read_only(input_data, tool_use_id, context):
    if input_data['hook_event_name'] != 'PreToolUse':
        return {}

    read_only_tools = ['Read', 'Glob', 'Grep', 'LS']
    if input_data['tool_name'] in read_only_tools:
        return {
            'hookSpecificOutput': {
                'hookEventName': input_data['hook_event_name'],
                'permissionDecision': 'allow',
                'permissionDecisionReason': 'Read-only tool auto-approved'
            }
        }
    return {}
```

```typescript TypeScript
const autoApproveReadOnly: HookCallback = async (input, toolUseID, { signal }) => {
  if (input.hook_event_name !== 'PreToolUse') return {};

  const preInput = input as PreToolUseHookInput;
  const readOnlyTools = ['Read', 'Glob', 'Grep', 'LS'];
  if (readOnlyTools.includes(preInput.tool_name)) {
    return {
      hookSpecificOutput: {
        hookEventName: input.hook_event_name,
        permissionDecision: 'allow',
        permissionDecisionReason: 'Read-only tool auto-approved'
      }
    };
  }
  return {};
};
```

</CodeGroup>

<Note>
`permissionDecision`フィールドは3つの値を受け入れます：`'allow'`（自動承認）、`'deny'`（ブロック）、または`'ask'`（確認を促す）。
</Note>

## 高度なシナリオを処理する

これらのパターンは、複雑なユースケースのためのより洗練されたフックシステムを構築するのに役立ちます。

### 複数のフックをチェーンする

フックは配列に表示される順序で実行されます。各フックを単一の責任に焦点を当てて、複雑なロジックのために複数のフックをチェーンします。この例は、すべてのツール呼び出しに対して4つのフックすべてを実行します（マッチャーが指定されていません）：

<CodeGroup>

```python Python
options = ClaudeAgentOptions(
    hooks={
        'PreToolUse': [
            HookMatcher(hooks=[rate_limiter]),        # 最初：レート制限をチェック
            HookMatcher(hooks=[authorization_check]), # 2番目：パーミッションを確認
            HookMatcher(hooks=[input_sanitizer]),     # 3番目：入力をサニタイズ
            HookMatcher(hooks=[audit_logger])         # 最後：アクションをログ
        ]
    }
)
```

```typescript TypeScript
const options = {
  hooks: {
    'PreToolUse': [
      { hooks: [rateLimiter] },        // 最初：レート制限をチェック
      { hooks: [authorizationCheck] }, // 2番目：パーミッションを確認
      { hooks: [inputSanitizer] },     // 3番目：入力をサニタイズ
      { hooks: [auditLogger] }         // 最後：アクションをログ
    ]
  }
};
```

</CodeGroup>

### ツール固有のマッチャーと正規表現

正規表現パターンを使用して複数のツールをマッチします：

<CodeGroup>

```python Python
options = ClaudeAgentOptions(
    hooks={
        'PreToolUse': [
            # ファイル変更ツールをマッチ
            HookMatcher(matcher='Write|Edit|Delete', hooks=[file_security_hook]),

            # すべてのMCPツールをマッチ
            HookMatcher(matcher='^mcp__', hooks=[mcp_audit_hook]),

            # すべてをマッチ（マッチャーなし）
            HookMatcher(hooks=[global_logger])
        ]
    }
)
```

```typescript TypeScript
const options = {
  hooks: {
    'PreToolUse': [
      // ファイル変更ツールをマッチ
      { matcher: 'Write|Edit|Delete', hooks: [fileSecurityHook] },

      // すべてのMCPツールをマッチ
      { matcher: '^mcp__', hooks: [mcpAuditHook] },

      // すべてをマッチ（マッチャーなし）
      { hooks: [globalLogger] }
    ]
  }
};
```

</CodeGroup>

<Note>
マッチャーは**ツール名**のみをマッチします。ファイルパスまたは他の引数でフィルタリングするには、フックコールバック内で`tool_input.file_path`をチェックします。
</Note>

### サブエージェント活動を追跡する

`SubagentStop`フックを使用してサブエージェント完了を監視します。`tool_use_id`は親エージェント呼び出しとそのサブエージェントを相関させるのに役立ちます：

<CodeGroup>

```python Python
async def subagent_tracker(input_data, tool_use_id, context):
    if input_data['hook_event_name'] == 'SubagentStop':
        print(f"[SUBAGENT] Completed")
        print(f"  Tool use ID: {tool_use_id}")
        print(f"  Stop hook active: {input_data.get('stop_hook_active')}")
    return {}

options = ClaudeAgentOptions(
    hooks={
        'SubagentStop': [HookMatcher(hooks=[subagent_tracker])]
    }
)
```

```typescript TypeScript
const subagentTracker: HookCallback = async (input, toolUseID, { signal }) => {
  if (input.hook_event_name === 'SubagentStop') {
    console.log(`[SUBAGENT] Completed`);
    console.log(`  Tool use ID: ${toolUseID}`);
    console.log(`  Stop hook active: ${input.stop_hook_active}`);
  }
  return {};
};

const options = {
  hooks: {
    SubagentStop: [{ hooks: [subagentTracker] }]
  }
};
```

</CodeGroup>

### フック内の非同期操作

フックはHTTPリクエストなどの非同期操作を実行できます。例外をスローする代わりに例外をキャッチして、エラーを適切に処理します。TypeScriptでは、`signal`を`fetch()`に渡して、フックがタイムアウトした場合にリクエストがキャンセルされるようにします：

<CodeGroup>

```python Python
import aiohttp
from datetime import datetime

async def webhook_notifier(input_data, tool_use_id, context):
    if input_data['hook_event_name'] != 'PostToolUse':
        return {}

    try:
        async with aiohttp.ClientSession() as session:
            await session.post(
                'https://api.example.com/webhook',
                json={
                    'tool': input_data['tool_name'],
                    'timestamp': datetime.now().isoformat()
                }
            )
    except Exception as e:
        print(f'Webhook request failed: {e}')

    return {}
```

```typescript TypeScript
const webhookNotifier: HookCallback = async (input, toolUseID, { signal }) => {
  if (input.hook_event_name !== 'PostToolUse') return {};

  try {
    // 適切なキャンセルのためにsignalを渡す
    await fetch('https://api.example.com/webhook', {
      method: 'POST',
      body: JSON.stringify({
        tool: (input as PostToolUseHookInput).tool_name,
        timestamp: new Date().toISOString()
      }),
      signal
    });
  } catch (error) {
    if (error instanceof Error && error.name === 'AbortError') {
      console.log('Webhook request cancelled');
    }
  }

  return {};
};
```

</CodeGroup>

### 通知を送信する（TypeScript のみ）

`Notification`フックを使用してエージェントからステータス更新を受け取り、Slackやモニタリングダッシュボードなどの外部サービスに転送します：

```typescript TypeScript
import { query, HookCallback, NotificationHookInput } from "@anthropic-ai/claude-agent-sdk";

const notificationHandler: HookCallback = async (input, toolUseID, { signal }) => {
  const notification = input as NotificationHookInput;

  await fetch('https://hooks.slack.com/services/YOUR/WEBHOOK/URL', {
    method: 'POST',
    body: JSON.stringify({
      text: `Agent status: ${notification.message}`
    }),
    signal
  });

  return {};
};

for await (const message of query({
  prompt: "Analyze this codebase",
  options: {
    hooks: {
      Notification: [{ hooks: [notificationHandler] }]
    }
  }
})) {
  console.log(message);
}
```

## 一般的な問題を修正する

このセクションでは、一般的な問題とその解決方法について説明します。

### フックが発火しない

- フックイベント名が正しく、大文字と小文字が区別されていることを確認してください（`preToolUse`ではなく`PreToolUse`）
- マッチャーパターンがツール名と正確にマッチしていることを確認してください
- フックが`options.hooks`の正しいイベントタイプの下にあることを確認してください
- `SubagentStop`、`Stop`、`SessionStart`、`SessionEnd`、`Notification`フックの場合、マッチャーは無視されます。これらのフックはそのタイプのすべてのイベントに対して発火します。
- エージェントが[`max_turns`](/docs/ja/agent-sdk/python#configuration-options)制限に達した場合、フックが実行される前にセッションが終了するため、フックが発火しない可能性があります

### マッチャーが期待通りにフィルタリングしない

マッチャーは**ツール名**のみをマッチします。ファイルパスまたは他の引数でフィルタリングするには、フック内で`tool_input.file_path`をチェックします：

```typescript
const myHook: HookCallback = async (input, toolUseID, { signal }) => {
  const preInput = input as PreToolUseHookInput;
  const filePath = preInput.tool_input?.file_path as string;
  if (!filePath?.endsWith('.md')) return {};  // マークダウンファイル以外をスキップ
  // マークダウンファイルを処理...
};
```

### フックタイムアウト

- `HookMatcher`設定で`timeout`値を増加させます
- 3番目のコールバック引数から`AbortSignal`を使用して、TypeScriptでキャンセルを適切に処理します

### ツールが予期せずブロックされた

- すべての`PreToolUse`フックで`permissionDecision: 'deny'`の戻り値をチェックします
- フックにログを追加して、返されている`permissionDecisionReason`を確認します
- マッチャーパターンが広すぎないことを確認します（空のマッチャーはすべてのツールをマッチします）

### 変更された入力が適用されない

- `updatedInput`が`hookSpecificOutput`内にあり、トップレベルにないことを確認します：

  ```typescript
  return {
    hookSpecificOutput: {
      hookEventName: input.hook_event_name,
      permissionDecision: 'allow',
      updatedInput: { command: 'new command' }
    }
  };
  ```

- 入力変更が有効になるには、`permissionDecision: 'allow'`も返す必要があります
- どのフックタイプの出力かを識別するために、`hookSpecificOutput`に`hookEventName`を含めます

### セッションフックが利用できない

`SessionStart`、`SessionEnd`、`Notification`フックはTypeScript SDKでのみ利用可能です。Python SDKはセットアップの制限により、これらのイベントをサポートしていません。

### サブエージェントパーミッションプロンプトが増加する

複数のサブエージェントを生成する場合、各サブエージェントは個別にパーミッションをリクエストする可能性があります。サブエージェントは親エージェントのパーミッションを自動的に継承しません。繰り返されるプロンプトを避けるには、`PreToolUse`フックを使用して特定のツールを自動承認するか、サブエージェントセッションに適用されるパーミッションルールを設定します。

### サブエージェントとの再帰的フックループ

サブエージェントを生成する`UserPromptSubmit`フックは、それらのサブエージェントが同じフックをトリガーする場合、無限ループを作成できます。これを防ぐには：

- フック入力でサブエージェント指標をチェックしてからサブエージェントを生成します
- `parent_tool_use_id`フィールドを使用して、既にサブエージェントコンテキストにいるかどうかを検出します
- フックをトップレベルエージェントセッションのみで実行するようにスコープします

### systemMessageが出力に表示されない

`systemMessage`フィールドはモデルが見るための会話にコンテキストを追加しますが、すべてのSDK出力モードに表示されない場合があります。フック決定をアプリケーションに表示する必要がある場合は、別途ログするか、専用の出力チャネルを使用します。

## さらに詳しく

- [パーミッション](/docs/ja/agent-sdk/permissions)：エージェントが何ができるかを制御する
- [カスタムツール](/docs/ja/agent-sdk/custom-tools)：エージェント機能を拡張するツールを構築する
- [TypeScript SDK リファレンス](/docs/ja/agent-sdk/typescript)
- [Python SDK リファレンス](/docs/ja/agent-sdk/python)