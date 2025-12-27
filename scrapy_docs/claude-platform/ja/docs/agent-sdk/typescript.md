# Agent SDK reference - TypeScript

TypeScript Agent SDKの完全なAPIリファレンス。すべての関数、型、およびインターフェースを含みます。

---

<script src="/components/typescript-sdk-type-links.js" defer />

<Note>
**新しいV2インターフェース（プレビュー）を試してください：** `send()`と`receive()`パターンを備えた簡略化されたインターフェースが利用可能になり、マルチターン会話がより簡単になりました。[詳細を確認](/docs/ja/agent-sdk/typescript-v2-preview)
</Note>

## インストール

```bash
npm install @anthropic-ai/claude-agent-sdk
```

## 関数

### `query()`

Claude Codeと対話するための主要な関数。メッセージが到着するにつれてストリーミングする非同期ジェネレータを作成します。

```typescript
function query({
  prompt,
  options
}: {
  prompt: string | AsyncIterable<SDKUserMessage>;
  options?: Options;
}): Query
```

#### パラメータ

| パラメータ | 型 | 説明 |
| :-------- | :--- | :---------- |
| `prompt` | `string \| AsyncIterable<`[`SDKUserMessage`](#sdkusermessage)`>` | 文字列またはストリーミングモード用の非同期イテラブルとしての入力プロンプト |
| `options` | [`Options`](#options) | オプションの設定オブジェクト（以下のOptionsタイプを参照） |

#### 戻り値

[`Query`](#query-1)オブジェクトを返します。これは`AsyncGenerator<`[`SDKMessage`](#sdkmessage)`, void>`を拡張し、追加のメソッドを持ちます。

### `tool()`

SDK MCPサーバーで使用するためのタイプセーフなMCPツール定義を作成します。

```typescript
function tool<Schema extends ZodRawShape>(
  name: string,
  description: string,
  inputSchema: Schema,
  handler: (args: z.infer<ZodObject<Schema>>, extra: unknown) => Promise<CallToolResult>
): SdkMcpToolDefinition<Schema>
```

#### パラメータ

| パラメータ | 型 | 説明 |
| :-------- | :--- | :---------- |
| `name` | `string` | ツールの名前 |
| `description` | `string` | ツールが何をするかの説明 |
| `inputSchema` | `Schema extends ZodRawShape` | ツールの入力パラメータを定義するZodスキーマ |
| `handler` | `(args, extra) => Promise<`[`CallToolResult`](#calltoolresult)`>` | ツールロジックを実行する非同期関数 |

### `createSdkMcpServer()`

アプリケーションと同じプロセスで実行されるMCPサーバーインスタンスを作成します。

```typescript
function createSdkMcpServer(options: {
  name: string;
  version?: string;
  tools?: Array<SdkMcpToolDefinition<any>>;
}): McpSdkServerConfigWithInstance
```

#### パラメータ

| パラメータ | 型 | 説明 |
| :-------- | :--- | :---------- |
| `options.name` | `string` | MCPサーバーの名前 |
| `options.version` | `string` | オプションのバージョン文字列 |
| `options.tools` | `Array<SdkMcpToolDefinition>` | [`tool()`](#tool)で作成されたツール定義の配列 |

## 型

### `Options`

`query()`関数の設定オブジェクト。

| プロパティ | 型 | デフォルト | 説明 |
| :------- | :--- | :------ | :---------- |
| `abortController` | `AbortController` | `new AbortController()` | 操作をキャンセルするためのコントローラー |
| `additionalDirectories` | `string[]` | `[]` | Claudeがアクセスできる追加ディレクトリ |
| `agents` | `Record<string, [`AgentDefinition`](#agentdefinition)>` | `undefined` | プログラムでサブエージェントを定義 |
| `allowDangerouslySkipPermissions` | `boolean` | `false` | パーミッションのバイパスを有効にします。`permissionMode: 'bypassPermissions'`を使用する場合に必須 |
| `allowedTools` | `string[]` | すべてのツール | 許可されたツール名のリスト |
| `betas` | [`SdkBeta`](#sdkbeta)`[]` | `[]` | ベータ機能を有効にします（例：`['context-1m-2025-08-07']`） |
| `canUseTool` | [`CanUseTool`](#canusetool) | `undefined` | ツール使用のカスタムパーミッション関数 |
| `continue` | `boolean` | `false` | 最新の会話を続行 |
| `cwd` | `string` | `process.cwd()` | 現在の作業ディレクトリ |
| `disallowedTools` | `string[]` | `[]` | 許可されていないツール名のリスト |
| `enableFileCheckpointing` | `boolean` | `false` | ファイル変更追跡を有効にして巻き戻しを可能にします。[ファイルチェックポイント](/docs/ja/agent-sdk/file-checkpointing)を参照 |
| `env` | `Dict<string>` | `process.env` | 環境変数 |
| `executable` | `'bun' \| 'deno' \| 'node'` | 自動検出 | 使用するJavaScriptランタイム |
| `executableArgs` | `string[]` | `[]` | 実行可能ファイルに渡す引数 |
| `extraArgs` | `Record<string, string \| null>` | `{}` | 追加の引数 |
| `fallbackModel` | `string` | `undefined` | プライマリが失敗した場合に使用するモデル |
| `forkSession` | `boolean` | `false` | `resume`で再開する場合、元のセッションを続行する代わりに新しいセッションIDにフォークします |
| `hooks` | `Partial<Record<`[`HookEvent`](#hookevent)`, `[`HookCallbackMatcher`](#hookcallbackmatcher)`[]>>` | `{}` | イベントのフックコールバック |
| `includePartialMessages` | `boolean` | `false` | 部分的なメッセージイベントを含める |
| `maxBudgetUsd` | `number` | `undefined` | クエリの最大予算（USD） |
| `maxThinkingTokens` | `number` | `undefined` | 思考プロセスの最大トークン数 |
| `maxTurns` | `number` | `undefined` | 最大会話ターン数 |
| `mcpServers` | `Record<string, [`McpServerConfig`](#mcpserverconfig)>` | `{}` | MCPサーバー設定 |
| `model` | `string` | CLIからのデフォルト | 使用するClaudeモデル |
| `outputFormat` | `{ type: 'json_schema', schema: JSONSchema }` | `undefined` | エージェント結果の出力形式を定義します。詳細は[構造化出力](/docs/ja/agent-sdk/structured-outputs)を参照 |
| `pathToClaudeCodeExecutable` | `string` | 組み込み実行可能ファイルを使用 | Claude Code実行可能ファイルへのパス |
| `permissionMode` | [`PermissionMode`](#permissionmode) | `'default'` | セッションのパーミッションモード |
| `permissionPromptToolName` | `string` | `undefined` | パーミッションプロンプト用のMCPツール名 |
| `plugins` | [`SdkPluginConfig`](#sdkpluginconfig)`[]` | `[]` | ローカルパスからカスタムプラグインをロードします。詳細は[プラグイン](/docs/ja/agent-sdk/plugins)を参照 |
| `resume` | `string` | `undefined` | 再開するセッションID |
| `resumeSessionAt` | `string` | `undefined` | 特定のメッセージUUIDでセッションを再開 |
| `sandbox` | [`SandboxSettings`](#sandboxsettings) | `undefined` | サンドボックス動作をプログラムで設定します。詳細は[サンドボックス設定](#sandboxsettings)を参照 |
| `settingSources` | [`SettingSource`](#settingsource)`[]` | `[]`（設定なし） | どのファイルシステム設定をロードするかを制御します。省略された場合、設定はロードされません。**注：** CLAUDE.mdファイルをロードするには`'project'`を含める必要があります |
| `stderr` | `(data: string) => void` | `undefined` | stderr出力のコールバック |
| `strictMcpConfig` | `boolean` | `false` | 厳密なMCP検証を強制 |
| `systemPrompt` | `string \| { type: 'preset'; preset: 'claude_code'; append?: string }` | `undefined`（空のプロンプト） | システムプロンプト設定。カスタムプロンプト用の文字列を渡すか、`{ type: 'preset', preset: 'claude_code' }`を使用してClaude Codeのシステムプロンプトを使用します。プリセットオブジェクト形式を使用する場合、`append`を追加して追加の指示でシステムプロンプトを拡張します |
| `tools` | `string[] \| { type: 'preset'; preset: 'claude_code' }` | `undefined` | ツール設定。ツール名の配列を渡すか、プリセットを使用してClaude Codeのデフォルトツールを取得します |

### `Query`

`query()`関数によって返されるインターフェース。

```typescript
interface Query extends AsyncGenerator<SDKMessage, void> {
  interrupt(): Promise<void>;
  rewindFiles(userMessageUuid: string): Promise<void>;
  setPermissionMode(mode: PermissionMode): Promise<void>;
  setModel(model?: string): Promise<void>;
  setMaxThinkingTokens(maxThinkingTokens: number | null): Promise<void>;
  supportedCommands(): Promise<SlashCommand[]>;
  supportedModels(): Promise<ModelInfo[]>;
  mcpServerStatus(): Promise<McpServerStatus[]>;
  accountInfo(): Promise<AccountInfo>;
}
```

#### メソッド

| メソッド | 説明 |
| :----- | :---------- |
| `interrupt()` | クエリを中断します（ストリーミング入力モードでのみ利用可能） |
| `rewindFiles(userMessageUuid)` | ファイルを指定されたユーザーメッセージの状態に復元します。`enableFileCheckpointing: true`が必須です。[ファイルチェックポイント](/docs/ja/agent-sdk/file-checkpointing)を参照 |
| `setPermissionMode()` | パーミッションモードを変更します（ストリーミング入力モードでのみ利用可能） |
| `setModel()` | モデルを変更します（ストリーミング入力モードでのみ利用可能） |
| `setMaxThinkingTokens()` | 最大思考トークン数を変更します（ストリーミング入力モードでのみ利用可能） |
| `supportedCommands()` | 利用可能なスラッシュコマンドを返します |
| `supportedModels()` | 表示情報付きの利用可能なモデルを返します |
| `mcpServerStatus()` | 接続されたMCPサーバーのステータスを返します |
| `accountInfo()` | アカウント情報を返します |

### `AgentDefinition`

プログラムで定義されたサブエージェントの設定。

```typescript
type AgentDefinition = {
  description: string;
  tools?: string[];
  prompt: string;
  model?: 'sonnet' | 'opus' | 'haiku' | 'inherit';
}
```

| フィールド | 必須 | 説明 |
|:------|:---------|:------------|
| `description` | はい | このエージェントをいつ使用するかの自然言語説明 |
| `tools` | いいえ | 許可されたツール名の配列。省略された場合、すべてのツールを継承します |
| `prompt` | はい | エージェントのシステムプロンプト |
| `model` | いいえ | このエージェントのモデルオーバーライド。省略された場合、メインモデルを使用します |

### `SettingSource`

SDKがどのファイルシステムベースの設定ソースから設定をロードするかを制御します。

```typescript
type SettingSource = 'user' | 'project' | 'local';
```

| 値 | 説明 | 場所 |
|:------|:------------|:---------|
| `'user'` | グローバルユーザー設定 | `~/.claude/settings.json` |
| `'project'` | 共有プロジェクト設定（バージョン管理） | `.claude/settings.json` |
| `'local'` | ローカルプロジェクト設定（gitignored） | `.claude/settings.local.json` |

#### デフォルト動作

`settingSources`が**省略**または**未定義**の場合、SDKはファイルシステム設定をロード**しません**。これはSDKアプリケーションの分離を提供します。

#### settingSourcesを使用する理由

**すべてのファイルシステム設定をロード（レガシー動作）：**
```typescript
// SDK v0.0.xのようにすべての設定をロード
const result = query({
  prompt: "Analyze this code",
  options: {
    settingSources: ['user', 'project', 'local']  // すべての設定をロード
  }
});
```

**特定の設定ソースのみをロード：**
```typescript
// プロジェクト設定のみをロード、ユーザーとローカルは無視
const result = query({
  prompt: "Run CI checks",
  options: {
    settingSources: ['project']  // .claude/settings.jsonのみ
  }
});
```

**テストとCI環境：**
```typescript
// ローカル設定を除外してCI内で一貫した動作を確保
const result = query({
  prompt: "Run tests",
  options: {
    settingSources: ['project'],  // チーム共有設定のみ
    permissionMode: 'bypassPermissions'
  }
});
```

**SDKのみのアプリケーション：**
```typescript
// すべてをプログラムで定義（デフォルト動作）
// ファイルシステムの依存関係なし - settingSourcesはデフォルトで[]
const result = query({
  prompt: "Review this PR",
  options: {
    // settingSources: [] がデフォルト、指定する必要はありません
    agents: { /* ... */ },
    mcpServers: { /* ... */ },
    allowedTools: ['Read', 'Grep', 'Glob']
  }
});
```

**CLAUDE.mdプロジェクト指示をロード：**
```typescript
// プロジェクト設定をロードしてCLAUDE.mdファイルを含める
const result = query({
  prompt: "Add a new feature following project conventions",
  options: {
    systemPrompt: {
      type: 'preset',
      preset: 'claude_code'  // CLAUDE.mdを使用するために必須
    },
    settingSources: ['project'],  // プロジェクトディレクトリからCLAUDE.mdをロード
    allowedTools: ['Read', 'Write', 'Edit']
  }
});
```

#### 設定の優先順位

複数のソースがロードされる場合、設定は次の優先順位（高から低）でマージされます：
1. ローカル設定（`.claude/settings.local.json`）
2. プロジェクト設定（`.claude/settings.json`）
3. ユーザー設定（`~/.claude/settings.json`）

プログラムオプション（`agents`、`allowedTools`など）は常にファイルシステム設定をオーバーライドします。

### `PermissionMode`

```typescript
type PermissionMode =
  | 'default'           // 標準パーミッション動作
  | 'acceptEdits'       // ファイル編集を自動承認
  | 'bypassPermissions' // すべてのパーミッションチェックをバイパス
  | 'plan'              // 計画モード - 実行なし
```

### `CanUseTool`

ツール使用を制御するためのカスタムパーミッション関数型。

```typescript
type CanUseTool = (
  toolName: string,
  input: ToolInput,
  options: {
    signal: AbortSignal;
    suggestions?: PermissionUpdate[];
  }
) => Promise<PermissionResult>;
```

### `PermissionResult`

パーミッションチェックの結果。

```typescript
type PermissionResult = 
  | {
      behavior: 'allow';
      updatedInput: ToolInput;
      updatedPermissions?: PermissionUpdate[];
    }
  | {
      behavior: 'deny';
      message: string;
      interrupt?: boolean;
    }
```

### `McpServerConfig`

MCPサーバーの設定。

```typescript
type McpServerConfig = 
  | McpStdioServerConfig
  | McpSSEServerConfig
  | McpHttpServerConfig
  | McpSdkServerConfigWithInstance;
```

#### `McpStdioServerConfig`

```typescript
type McpStdioServerConfig = {
  type?: 'stdio';
  command: string;
  args?: string[];
  env?: Record<string, string>;
}
```

#### `McpSSEServerConfig`

```typescript
type McpSSEServerConfig = {
  type: 'sse';
  url: string;
  headers?: Record<string, string>;
}
```

#### `McpHttpServerConfig`

```typescript
type McpHttpServerConfig = {
  type: 'http';
  url: string;
  headers?: Record<string, string>;
}
```

#### `McpSdkServerConfigWithInstance`

```typescript
type McpSdkServerConfigWithInstance = {
  type: 'sdk';
  name: string;
  instance: McpServer;
}
```

### `SdkPluginConfig`

SDKでプラグインをロードするための設定。

```typescript
type SdkPluginConfig = {
  type: 'local';
  path: string;
}
```

| フィールド | 型 | 説明 |
|:------|:-----|:------------|
| `type` | `'local'` | `'local'`である必要があります（現在ローカルプラグインのみサポート） |
| `path` | `string` | プラグインディレクトリへの絶対パスまたは相対パス |

**例：**
```typescript
plugins: [
  { type: 'local', path: './my-plugin' },
  { type: 'local', path: '/absolute/path/to/plugin' }
]
```

プラグインの作成と使用に関する完全な情報については、[プラグイン](/docs/ja/agent-sdk/plugins)を参照してください。

## メッセージ型

### `SDKMessage`

クエリによって返されるすべての可能なメッセージの共用体型。

```typescript
type SDKMessage = 
  | SDKAssistantMessage
  | SDKUserMessage
  | SDKUserMessageReplay
  | SDKResultMessage
  | SDKSystemMessage
  | SDKPartialAssistantMessage
  | SDKCompactBoundaryMessage;
```

### `SDKAssistantMessage`

アシスタント応答メッセージ。

```typescript
type SDKAssistantMessage = {
  type: 'assistant';
  uuid: UUID;
  session_id: string;
  message: APIAssistantMessage; // Anthropic SDKから
  parent_tool_use_id: string | null;
}
```

### `SDKUserMessage`

ユーザー入力メッセージ。

```typescript
type SDKUserMessage = {
  type: 'user';
  uuid?: UUID;
  session_id: string;
  message: APIUserMessage; // Anthropic SDKから
  parent_tool_use_id: string | null;
}
```

### `SDKUserMessageReplay`

必須UUIDを持つ再生されたユーザーメッセージ。

```typescript
type SDKUserMessageReplay = {
  type: 'user';
  uuid: UUID;
  session_id: string;
  message: APIUserMessage;
  parent_tool_use_id: string | null;
}
```

### `SDKResultMessage`

最終結果メッセージ。

```typescript
type SDKResultMessage =
  | {
      type: 'result';
      subtype: 'success';
      uuid: UUID;
      session_id: string;
      duration_ms: number;
      duration_api_ms: number;
      is_error: boolean;
      num_turns: number;
      result: string;
      total_cost_usd: number;
      usage: NonNullableUsage;
      modelUsage: { [modelName: string]: ModelUsage };
      permission_denials: SDKPermissionDenial[];
      structured_output?: unknown;
    }
  | {
      type: 'result';
      subtype:
        | 'error_max_turns'
        | 'error_during_execution'
        | 'error_max_budget_usd'
        | 'error_max_structured_output_retries';
      uuid: UUID;
      session_id: string;
      duration_ms: number;
      duration_api_ms: number;
      is_error: boolean;
      num_turns: number;
      total_cost_usd: number;
      usage: NonNullableUsage;
      modelUsage: { [modelName: string]: ModelUsage };
      permission_denials: SDKPermissionDenial[];
      errors: string[];
    }
```

### `SDKSystemMessage`

システム初期化メッセージ。

```typescript
type SDKSystemMessage = {
  type: 'system';
  subtype: 'init';
  uuid: UUID;
  session_id: string;
  apiKeySource: ApiKeySource;
  cwd: string;
  tools: string[];
  mcp_servers: {
    name: string;
    status: string;
  }[];
  model: string;
  permissionMode: PermissionMode;
  slash_commands: string[];
  output_style: string;
}
```

### `SDKPartialAssistantMessage`

ストリーミング部分メッセージ（`includePartialMessages`がtrueの場合のみ）。

```typescript
type SDKPartialAssistantMessage = {
  type: 'stream_event';
  event: RawMessageStreamEvent; // Anthropic SDKから
  parent_tool_use_id: string | null;
  uuid: UUID;
  session_id: string;
}
```

### `SDKCompactBoundaryMessage`

会話圧縮境界を示すメッセージ。

```typescript
type SDKCompactBoundaryMessage = {
  type: 'system';
  subtype: 'compact_boundary';
  uuid: UUID;
  session_id: string;
  compact_metadata: {
    trigger: 'manual' | 'auto';
    pre_tokens: number;
  };
}
```

### `SDKPermissionDenial`

拒否されたツール使用に関する情報。

```typescript
type SDKPermissionDenial = {
  tool_name: string;
  tool_use_id: string;
  tool_input: ToolInput;
}
```

## フック型

フックの使用に関する包括的なガイド、例、および一般的なパターンについては、[フックガイド](/docs/ja/agent-sdk/hooks)を参照してください。

### `HookEvent`

利用可能なフックイベント。

```typescript
type HookEvent =
  | 'PreToolUse'
  | 'PostToolUse'
  | 'PostToolUseFailure'
  | 'Notification'
  | 'UserPromptSubmit'
  | 'SessionStart'
  | 'SessionEnd'
  | 'Stop'
  | 'SubagentStart'
  | 'SubagentStop'
  | 'PreCompact'
  | 'PermissionRequest';
```

### `HookCallback`

フックコールバック関数型。

```typescript
type HookCallback = (
  input: HookInput, // すべてのフック入力型の共用体
  toolUseID: string | undefined,
  options: { signal: AbortSignal }
) => Promise<HookJSONOutput>;
```

### `HookCallbackMatcher`

オプションのマッチャー付きフック設定。

```typescript
interface HookCallbackMatcher {
  matcher?: string;
  hooks: HookCallback[];
}
```

### `HookInput`

すべてのフック入力型の共用体型。

```typescript
type HookInput =
  | PreToolUseHookInput
  | PostToolUseHookInput
  | PostToolUseFailureHookInput
  | NotificationHookInput
  | UserPromptSubmitHookInput
  | SessionStartHookInput
  | SessionEndHookInput
  | StopHookInput
  | SubagentStartHookInput
  | SubagentStopHookInput
  | PreCompactHookInput
  | PermissionRequestHookInput;
```

### `BaseHookInput`

すべてのフック入力型が拡張する基本インターフェース。

```typescript
type BaseHookInput = {
  session_id: string;
  transcript_path: string;
  cwd: string;
  permission_mode?: string;
}
```

#### `PreToolUseHookInput`

```typescript
type PreToolUseHookInput = BaseHookInput & {
  hook_event_name: 'PreToolUse';
  tool_name: string;
  tool_input: unknown;
}
```

#### `PostToolUseHookInput`

```typescript
type PostToolUseHookInput = BaseHookInput & {
  hook_event_name: 'PostToolUse';
  tool_name: string;
  tool_input: unknown;
  tool_response: unknown;
}
```

#### `PostToolUseFailureHookInput`

```typescript
type PostToolUseFailureHookInput = BaseHookInput & {
  hook_event_name: 'PostToolUseFailure';
  tool_name: string;
  tool_input: unknown;
  error: string;
  is_interrupt?: boolean;
}
```

#### `NotificationHookInput`

```typescript
type NotificationHookInput = BaseHookInput & {
  hook_event_name: 'Notification';
  message: string;
  title?: string;
}
```

#### `UserPromptSubmitHookInput`

```typescript
type UserPromptSubmitHookInput = BaseHookInput & {
  hook_event_name: 'UserPromptSubmit';
  prompt: string;
}
```

#### `SessionStartHookInput`

```typescript
type SessionStartHookInput = BaseHookInput & {
  hook_event_name: 'SessionStart';
  source: 'startup' | 'resume' | 'clear' | 'compact';
}
```

#### `SessionEndHookInput`

```typescript
type SessionEndHookInput = BaseHookInput & {
  hook_event_name: 'SessionEnd';
  reason: ExitReason;  // EXIT_REASONS配列からの文字列
}
```

#### `StopHookInput`

```typescript
type StopHookInput = BaseHookInput & {
  hook_event_name: 'Stop';
  stop_hook_active: boolean;
}
```

#### `SubagentStartHookInput`

```typescript
type SubagentStartHookInput = BaseHookInput & {
  hook_event_name: 'SubagentStart';
  agent_id: string;
  agent_type: string;
}
```

#### `SubagentStopHookInput`

```typescript
type SubagentStopHookInput = BaseHookInput & {
  hook_event_name: 'SubagentStop';
  stop_hook_active: boolean;
}
```

#### `PreCompactHookInput`

```typescript
type PreCompactHookInput = BaseHookInput & {
  hook_event_name: 'PreCompact';
  trigger: 'manual' | 'auto';
  custom_instructions: string | null;
}
```

#### `PermissionRequestHookInput`

```typescript
type PermissionRequestHookInput = BaseHookInput & {
  hook_event_name: 'PermissionRequest';
  tool_name: string;
  tool_input: unknown;
  permission_suggestions?: PermissionUpdate[];
}
```

### `HookJSONOutput`

フック戻り値。

```typescript
type HookJSONOutput = AsyncHookJSONOutput | SyncHookJSONOutput;
```

#### `AsyncHookJSONOutput`

```typescript
type AsyncHookJSONOutput = {
  async: true;
  asyncTimeout?: number;
}
```

#### `SyncHookJSONOutput`

```typescript
type SyncHookJSONOutput = {
  continue?: boolean;
  suppressOutput?: boolean;
  stopReason?: string;
  decision?: 'approve' | 'block';
  systemMessage?: string;
  reason?: string;
  hookSpecificOutput?:
    | {
        hookEventName: 'PreToolUse';
        permissionDecision?: 'allow' | 'deny' | 'ask';
        permissionDecisionReason?: string;
        updatedInput?: Record<string, unknown>;
      }
    | {
        hookEventName: 'UserPromptSubmit';
        additionalContext?: string;
      }
    | {
        hookEventName: 'SessionStart';
        additionalContext?: string;
      }
    | {
        hookEventName: 'PostToolUse';
        additionalContext?: string;
      };
}
```

## ツール入力型

すべての組み込みClaude Codeツールの入力スキーマのドキュメント。これらの型は`@anthropic-ai/claude-agent-sdk`からエクスポートされ、タイプセーフなツール相互作用に使用できます。

### `ToolInput`

**注：** これはわかりやすくするためのドキュメント専用型です。すべてのツール入力型の共用体を表します。

```typescript
type ToolInput =
  | AgentInput
  | AskUserQuestionInput
  | BashInput
  | BashOutputInput
  | FileEditInput
  | FileReadInput
  | FileWriteInput
  | GlobInput
  | GrepInput
  | KillShellInput
  | NotebookEditInput
  | WebFetchInput
  | WebSearchInput
  | TodoWriteInput
  | ExitPlanModeInput
  | ListMcpResourcesInput
  | ReadMcpResourceInput;
```

### Task

**ツール名：** `Task`

```typescript
interface AgentInput {
  /**
   * タスクの短い説明（3～5語）
   */
  description: string;
  /**
   * エージェントが実行するタスク
   */
  prompt: string;
  /**
   * このタスクに使用する専門的なエージェントのタイプ
   */
  subagent_type: string;
}
```

複雑なマルチステップタスクを自律的に処理するための新しいエージェントを起動します。

### AskUserQuestion

**ツール名：** `AskUserQuestion`

```typescript
interface AskUserQuestionInput {
  /**
   * ユーザーに尋ねる質問（1～4個の質問）
   */
  questions: Array<{
    /**
     * ユーザーに尋ねる完全な質問。明確で具体的で、
     * 疑問符で終わる必要があります。
     */
    question: string;
    /**
     * チップ/タグとして表示される非常に短いラベル（最大12文字）。
     * 例：「認証方法」、「ライブラリ」、「アプローチ」
     */
    header: string;
    /**
     * 利用可能な選択肢（2～4オプション）。「その他」オプションは
     * 自動的に提供されます。
     */
    options: Array<{
      /**
       * このオプションの表示テキスト（1～5語）
       */
      label: string;
      /**
       * このオプションが何を意味するかの説明
       */
      description: string;
    }>;
    /**
     * 複数選択を許可する場合はtrueに設定
     */
    multiSelect: boolean;
  }>;
  /**
   * パーミッションシステムによって入力されたユーザーの回答。
   * 質問テキストを選択されたオプションラベルにマップします。
   * マルチセレクト回答はカンマで区切られます。
   */
  answers?: Record<string, string>;
}
```

実行中にユーザーに明確化の質問をします。使用方法の詳細については、[AskUserQuestionツールの処理](/docs/ja/agent-sdk/permissions#handling-the-askuserquestion-tool)を参照してください。

### Bash

**ツール名：** `Bash`

```typescript
interface BashInput {
  /**
   * 実行するコマンド
   */
  command: string;
  /**
   * オプションのタイムアウト（ミリ秒、最大600000）
   */
  timeout?: number;
  /**
   * このコマンドが何をするかの明確で簡潔な説明（5～10語）
   */
  description?: string;
  /**
   * このコマンドをバックグラウンドで実行する場合はtrueに設定
   */
  run_in_background?: boolean;
}
```

オプションのタイムアウトとバックグラウンド実行を備えた永続的なシェルセッションでbashコマンドを実行します。

### BashOutput

**ツール名：** `BashOutput`

```typescript
interface BashOutputInput {
  /**
   * 出力を取得するバックグラウンドシェルのID
   */
  bash_id: string;
  /**
   * 出力行をフィルタリングするオプションの正規表現
   */
  filter?: string;
}
```

実行中または完了したバックグラウンドbashシェルから出力を取得します。

### Edit

**ツール名：** `Edit`

```typescript
interface FileEditInput {
  /**
   * 変更するファイルの絶対パス
   */
  file_path: string;
  /**
   * 置換するテキスト
   */
  old_string: string;
  /**
   * 置換するテキスト（old_stringと異なる必要があります）
   */
  new_string: string;
  /**
   * old_stringのすべての出現を置換（デフォルトfalse）
   */
  replace_all?: boolean;
}
```

ファイル内で正確な文字列置換を実行します。

### Read

**ツール名：** `Read`

```typescript
interface FileReadInput {
  /**
   * 読み込むファイルの絶対パス
   */
  file_path: string;
  /**
   * 読み込みを開始する行番号
   */
  offset?: number;
  /**
   * 読み込む行数
   */
  limit?: number;
}
```

テキスト、画像、PDF、Jupyterノートブックを含むローカルファイルシステムからファイルを読み込みます。

### Write

**ツール名：** `Write`

```typescript
interface FileWriteInput {
  /**
   * 書き込むファイルの絶対パス
   */
  file_path: string;
  /**
   * ファイルに書き込むコンテンツ
   */
  content: string;
}
```

ローカルファイルシステムにファイルを書き込み、存在する場合は上書きします。

### Glob

**ツール名：** `Glob`

```typescript
interface GlobInput {
  /**
   * ファイルと照合するglobパターン
   */
  pattern: string;
  /**
   * 検索するディレクトリ（デフォルトはcwd）
   */
  path?: string;
}
```

任意のコードベースサイズで機能する高速ファイルパターンマッチング。

### Grep

**ツール名：** `Grep`

```typescript
interface GrepInput {
  /**
   * 検索する正規表現パターン
   */
  pattern: string;
  /**
   * 検索するファイルまたはディレクトリ（デフォルトはcwd）
   */
  path?: string;
  /**
   * ファイルをフィルタリングするglobパターン（例：「*.js」）
   */
  glob?: string;
  /**
   * 検索するファイルタイプ（例：「js」、「py」、「rust」）
   */
  type?: string;
  /**
   * 出力モード：「content」、「files_with_matches」、または「count」
   */
  output_mode?: 'content' | 'files_with_matches' | 'count';
  /**
   * 大文字小文字を区別しない検索
   */
  '-i'?: boolean;
  /**
   * 行番号を表示（contentモード用）
   */
  '-n'?: boolean;
  /**
   * 各マッチの前に表示する行
   */
  '-B'?: number;
  /**
   * 各マッチの後に表示する行
   */
  '-A'?: number;
  /**
   * 各マッチの前後に表示する行
   */
  '-C'?: number;
  /**
   * 出力を最初のN行/エントリに制限
   */
  head_limit?: number;
  /**
   * マルチラインモードを有効にする
   */
  multiline?: boolean;
}
```

ripgrepに基づいた正規表現サポート付きの強力な検索ツール。

### KillBash

**ツール名：** `KillBash`

```typescript
interface KillShellInput {
  /**
   * 終了するバックグラウンドシェルのID
   */
  shell_id: string;
}
```

IDでバックグラウンドbashシェルを実行中のものを終了します。

### NotebookEdit

**ツール名：** `NotebookEdit`

```typescript
interface NotebookEditInput {
  /**
   * Jupyterノートブックファイルの絶対パス
   */
  notebook_path: string;
  /**
   * 編集するセルのID
   */
  cell_id?: string;
  /**
   * セルの新しいソース
   */
  new_source: string;
  /**
   * セルのタイプ（codeまたはmarkdown）
   */
  cell_type?: 'code' | 'markdown';
  /**
   * 編集のタイプ（replace、insert、delete）
   */
  edit_mode?: 'replace' | 'insert' | 'delete';
}
```

Jupyterノートブックファイル内のセルを編集します。

### WebFetch

**ツール名：** `WebFetch`

```typescript
interface WebFetchInput {
  /**
   * コンテンツを取得するURL
   */
  url: string;
  /**
   * 取得したコンテンツで実行するプロンプト
   */
  prompt: string;
}
```

URLからコンテンツを取得し、AIモデルで処理します。

### WebSearch

**ツール名：** `WebSearch`

```typescript
interface WebSearchInput {
  /**
   * 使用する検索クエリ
   */
  query: string;
  /**
   * これらのドメインからのみ結果を含める
   */
  allowed_domains?: string[];
  /**
   * これらのドメインからの結果を決して含めない
   */
  blocked_domains?: string[];
}
```

ウェブを検索し、フォーマットされた結果を返します。

### TodoWrite

**ツール名：** `TodoWrite`

```typescript
interface TodoWriteInput {
  /**
   * 更新されたtodoリスト
   */
  todos: Array<{
    /**
     * タスクの説明
     */
    content: string;
    /**
     * タスクのステータス
     */
    status: 'pending' | 'in_progress' | 'completed';
    /**
     * タスク説明の能動形
     */
    activeForm: string;
  }>;
}
```

進捗を追跡するための構造化されたタスクリストを作成および管理します。

### ExitPlanMode

**ツール名：** `ExitPlanMode`

```typescript
interface ExitPlanModeInput {
  /**
   * ユーザーの承認のために実行する計画
   */
  plan: string;
}
```

計画モードを終了し、ユーザーに計画の承認を促します。

### ListMcpResources

**ツール名：** `ListMcpResources`

```typescript
interface ListMcpResourcesInput {
  /**
   * リソースをフィルタリングするオプションのサーバー名
   */
  server?: string;
}
```

接続されたサーバーから利用可能なMCPリソースをリストします。

### ReadMcpResource

**ツール名：** `ReadMcpResource`

```typescript
interface ReadMcpResourceInput {
  /**
   * MCPサーバー名
   */
  server: string;
  /**
   * 読み込むリソースURI
   */
  uri: string;
}
```

サーバーから特定のMCPリソースを読み込みます。

## ツール出力型

すべての組み込みClaude Codeツールの出力スキーマのドキュメント。これらの型は各ツールによって返される実際の応答データを表します。

### `ToolOutput`

**注：** これはわかりやすくするためのドキュメント専用型です。すべてのツール出力型の共用体を表します。

```typescript
type ToolOutput =
  | TaskOutput
  | AskUserQuestionOutput
  | BashOutput
  | BashOutputToolOutput
  | EditOutput
  | ReadOutput
  | WriteOutput
  | GlobOutput
  | GrepOutput
  | KillBashOutput
  | NotebookEditOutput
  | WebFetchOutput
  | WebSearchOutput
  | TodoWriteOutput
  | ExitPlanModeOutput
  | ListMcpResourcesOutput
  | ReadMcpResourceOutput;
```

### Task

**ツール名：** `Task`

```typescript
interface TaskOutput {
  /**
   * サブエージェントからの最終結果メッセージ
   */
  result: string;
  /**
   * トークン使用統計
   */
  usage?: {
    input_tokens: number;
    output_tokens: number;
    cache_creation_input_tokens?: number;
    cache_read_input_tokens?: number;
  };
  /**
   * USDでの総コスト
   */
  total_cost_usd?: number;
  /**
   * ミリ秒単位の実行期間
   */
  duration_ms?: number;
}
```

委任されたタスクを完了した後、サブエージェントからの最終結果を返します。

### AskUserQuestion

**ツール名：** `AskUserQuestion`

```typescript
interface AskUserQuestionOutput {
  /**
   * 尋ねられた質問
   */
  questions: Array<{
    question: string;
    header: string;
    options: Array<{
      label: string;
      description: string;
    }>;
    multiSelect: boolean;
  }>;
  /**
   * ユーザーが提供した回答。
   * 質問テキストを回答文字列にマップします。
   * マルチセレクト回答はカンマで区切られます。
   */
  answers: Record<string, string>;
}
```

尋ねられた質問とユーザーの回答を返します。

### Bash

**ツール名：** `Bash`

```typescript
interface BashOutput {
  /**
   * 標準出力と標準エラーの結合出力
   */
  output: string;
  /**
   * コマンドの終了コード
   */
  exitCode: number;
  /**
   * タイムアウトのためにコマンドが終了されたかどうか
   */
  killed?: boolean;
  /**
   * バックグラウンドプロセスのシェルID
   */
  shellId?: string;
}
```

コマンド出力と終了ステータスを返します。バックグラウンドコマンドはshellIdで即座に返されます。

### BashOutput

**ツール名：** `BashOutput`

```typescript
interface BashOutputToolOutput {
  /**
   * 最後のチェック以降の新しい出力
   */
  output: string;
  /**
   * 現在のシェルステータス
   */
  status: 'running' | 'completed' | 'failed';
  /**
   * 終了コード（完了時）
   */
  exitCode?: number;
}
```

バックグラウンドシェルからの増分出力を返します。

### Edit

**ツール名：** `Edit`

```typescript
interface EditOutput {
  /**
   * 確認メッセージ
   */
  message: string;
  /**
   * 行われた置換の数
   */
  replacements: number;
  /**
   * 編集されたファイルパス
   */
  file_path: string;
}
```

成功した編集の確認と置換数を返します。

### Read

**ツール名:** `Read`

```typescript
type ReadOutput = 
  | TextFileOutput
  | ImageFileOutput
  | PDFFileOutput
  | NotebookFileOutput;

interface TextFileOutput {
  /**
   * 行番号付きのファイル内容
   */
  content: string;
  /**
   * ファイルの総行数
   */
  total_lines: number;
  /**
   * 実際に返された行数
   */
  lines_returned: number;
}

interface ImageFileOutput {
  /**
   * Base64エンコードされた画像データ
   */
  image: string;
  /**
   * 画像のMIMEタイプ
   */
  mime_type: string;
  /**
   * ファイルサイズ（バイト単位）
   */
  file_size: number;
}

interface PDFFileOutput {
  /**
   * ページ内容の配列
   */
  pages: Array<{
    page_number: number;
    text?: string;
    images?: Array<{
      image: string;
      mime_type: string;
    }>;
  }>;
  /**
   * 総ページ数
   */
  total_pages: number;
}

interface NotebookFileOutput {
  /**
   * Jupyterノートブックのセル
   */
  cells: Array<{
    cell_type: 'code' | 'markdown';
    source: string;
    outputs?: any[];
    execution_count?: number;
  }>;
  /**
   * ノートブックのメタデータ
   */
  metadata?: Record<string, any>;
}
```

ファイルタイプに適切な形式でファイル内容を返します。

### Write

**ツール名:** `Write`

```typescript
interface WriteOutput {
  /**
   * 成功メッセージ
   */
  message: string;
  /**
   * 書き込まれたバイト数
   */
  bytes_written: number;
  /**
   * 書き込まれたファイルパス
   */
  file_path: string;
}
```

ファイルの書き込みに成功した後、確認を返します。

### Glob

**ツール名:** `Glob`

```typescript
interface GlobOutput {
  /**
   * マッチしたファイルパスの配列
   */
  matches: string[];
  /**
   * 見つかったマッチ数
   */
  count: number;
  /**
   * 使用された検索ディレクトリ
   */
  search_path: string;
}
```

globパターンにマッチするファイルパスを、変更時刻でソートして返します。

### Grep

**ツール名:** `Grep`

```typescript
type GrepOutput = 
  | GrepContentOutput
  | GrepFilesOutput
  | GrepCountOutput;

interface GrepContentOutput {
  /**
   * コンテキスト付きのマッチ行
   */
  matches: Array<{
    file: string;
    line_number?: number;
    line: string;
    before_context?: string[];
    after_context?: string[];
  }>;
  /**
   * マッチの総数
   */
  total_matches: number;
}

interface GrepFilesOutput {
  /**
   * マッチを含むファイル
   */
  files: string[];
  /**
   * マッチを含むファイル数
   */
  count: number;
}

interface GrepCountOutput {
  /**
   * ファイルごとのマッチ数
   */
  counts: Array<{
    file: string;
    count: number;
  }>;
  /**
   * すべてのファイル全体でのマッチの総数
   */
  total: number;
}
```

output_modeで指定された形式で検索結果を返します。

### KillBash

**ツール名:** `KillBash`

```typescript
interface KillBashOutput {
  /**
   * 成功メッセージ
   */
  message: string;
  /**
   * 終了されたシェルのID
   */
  shell_id: string;
}
```

バックグラウンドシェルの終了後、確認を返します。

### NotebookEdit

**ツール名:** `NotebookEdit`

```typescript
interface NotebookEditOutput {
  /**
   * 成功メッセージ
   */
  message: string;
  /**
   * 実行された編集のタイプ
   */
  edit_type: 'replaced' | 'inserted' | 'deleted';
  /**
   * 影響を受けたセルのID
   */
  cell_id?: string;
  /**
   * 編集後のノートブックの総セル数
   */
  total_cells: number;
}
```

Jupyterノートブックの変更後、確認を返します。

### WebFetch

**ツール名:** `WebFetch`

```typescript
interface WebFetchOutput {
  /**
   * プロンプトに対するAIモデルの応答
   */
  response: string;
  /**
   * フェッチされたURL
   */
  url: string;
  /**
   * リダイレクト後の最終URL
   */
  final_url?: string;
  /**
   * HTTPステータスコード
   */
  status_code?: number;
}
```

フェッチされたウェブコンテンツのAIの分析を返します。

### WebSearch

**ツール名:** `WebSearch`

```typescript
interface WebSearchOutput {
  /**
   * 検索結果
   */
  results: Array<{
    title: string;
    url: string;
    snippet: string;
    /**
     * 利用可能な場合は追加メタデータ
     */
    metadata?: Record<string, any>;
  }>;
  /**
   * 結果の総数
   */
  total_results: number;
  /**
   * 検索されたクエリ
   */
  query: string;
}
```

ウェブからのフォーマットされた検索結果を返します。

### TodoWrite

**ツール名:** `TodoWrite`

```typescript
interface TodoWriteOutput {
  /**
   * 成功メッセージ
   */
  message: string;
  /**
   * 現在のタスク統計
   */
  stats: {
    total: number;
    pending: number;
    in_progress: number;
    completed: number;
  };
}
```

現在のタスク統計と共に確認を返します。

### ExitPlanMode

**ツール名:** `ExitPlanMode`

```typescript
interface ExitPlanModeOutput {
  /**
   * 確認メッセージ
   */
  message: string;
  /**
   * ユーザーがプランを承認したかどうか
   */
  approved?: boolean;
}
```

プランモード終了後、確認を返します。

### ListMcpResources

**ツール名:** `ListMcpResources`

```typescript
interface ListMcpResourcesOutput {
  /**
   * 利用可能なリソース
   */
  resources: Array<{
    uri: string;
    name: string;
    description?: string;
    mimeType?: string;
    server: string;
  }>;
  /**
   * リソースの総数
   */
  total: number;
}
```

利用可能なMCPリソースのリストを返します。

### ReadMcpResource

**ツール名:** `ReadMcpResource`

```typescript
interface ReadMcpResourceOutput {
  /**
   * リソースの内容
   */
  contents: Array<{
    uri: string;
    mimeType?: string;
    text?: string;
    blob?: string;
  }>;
  /**
   * リソースを提供したサーバー
   */
  server: string;
}
```

要求されたMCPリソースの内容を返します。

## パーミッションタイプ

### `PermissionUpdate`

パーミッションを更新するための操作。

```typescript
type PermissionUpdate = 
  | {
      type: 'addRules';
      rules: PermissionRuleValue[];
      behavior: PermissionBehavior;
      destination: PermissionUpdateDestination;
    }
  | {
      type: 'replaceRules';
      rules: PermissionRuleValue[];
      behavior: PermissionBehavior;
      destination: PermissionUpdateDestination;
    }
  | {
      type: 'removeRules';
      rules: PermissionRuleValue[];
      behavior: PermissionBehavior;
      destination: PermissionUpdateDestination;
    }
  | {
      type: 'setMode';
      mode: PermissionMode;
      destination: PermissionUpdateDestination;
    }
  | {
      type: 'addDirectories';
      directories: string[];
      destination: PermissionUpdateDestination;
    }
  | {
      type: 'removeDirectories';
      directories: string[];
      destination: PermissionUpdateDestination;
    }
```

### `PermissionBehavior`

```typescript
type PermissionBehavior = 'allow' | 'deny' | 'ask';
```

### `PermissionUpdateDestination`

```typescript
type PermissionUpdateDestination = 
  | 'userSettings'     // グローバルユーザー設定
  | 'projectSettings'  // ディレクトリごとのプロジェクト設定
  | 'localSettings'    // Gitignoreされたローカル設定
  | 'session'          // 現在のセッションのみ
```

### `PermissionRuleValue`

```typescript
type PermissionRuleValue = {
  toolName: string;
  ruleContent?: string;
}
```

## その他のタイプ

### `ApiKeySource`

```typescript
type ApiKeySource = 'user' | 'project' | 'org' | 'temporary';
```

### `SdkBeta`

`betas`オプションを使用して有効にできる利用可能なベータ機能。詳細については[ベータヘッダー](/docs/ja/api/beta-headers)を参照してください。

```typescript
type SdkBeta = 'context-1m-2025-08-07';
```

| 値 | 説明 | 互換性のあるモデル |
|:------|:------------|:------------------|
| `'context-1m-2025-08-07'` | 100万トークンの[コンテキストウィンドウ](/docs/ja/build-with-claude/context-windows)を有効にします | Claude Sonnet 4、Claude Sonnet 4.5 |

### `SlashCommand`

利用可能なスラッシュコマンドに関する情報。

```typescript
type SlashCommand = {
  name: string;
  description: string;
  argumentHint: string;
}
```

### `ModelInfo`

利用可能なモデルに関する情報。

```typescript
type ModelInfo = {
  value: string;
  displayName: string;
  description: string;
}
```

### `McpServerStatus`

接続されたMCPサーバーのステータス。

```typescript
type McpServerStatus = {
  name: string;
  status: 'connected' | 'failed' | 'needs-auth' | 'pending';
  serverInfo?: {
    name: string;
    version: string;
  };
}
```

### `AccountInfo`

認証されたユーザーのアカウント情報。

```typescript
type AccountInfo = {
  email?: string;
  organization?: string;
  subscriptionType?: string;
  tokenSource?: string;
  apiKeySource?: string;
}
```

### `ModelUsage`

結果メッセージで返されるモデルごとの使用統計。

```typescript
type ModelUsage = {
  inputTokens: number;
  outputTokens: number;
  cacheReadInputTokens: number;
  cacheCreationInputTokens: number;
  webSearchRequests: number;
  costUSD: number;
  contextWindow: number;
}
```

### `ConfigScope`

```typescript
type ConfigScope = 'local' | 'user' | 'project';
```

### `NonNullableUsage`

すべてのnullable フィールドがnon-nullableになった[`Usage`](#usage)のバージョン。

```typescript
type NonNullableUsage = {
  [K in keyof Usage]: NonNullable<Usage[K]>;
}
```

### `Usage`

トークン使用統計（`@anthropic-ai/sdk`から）。

```typescript
type Usage = {
  input_tokens: number | null;
  output_tokens: number | null;
  cache_creation_input_tokens?: number | null;
  cache_read_input_tokens?: number | null;
}
```

### `CallToolResult`

MCPツール結果タイプ（`@modelcontextprotocol/sdk/types.js`から）。

```typescript
type CallToolResult = {
  content: Array<{
    type: 'text' | 'image' | 'resource';
    // 追加フィールドはタイプによって異なります
  }>;
  isError?: boolean;
}
```

### `AbortError`

中止操作用のカスタムエラークラス。

```typescript
class AbortError extends Error {}
```

## サンドボックス設定

### `SandboxSettings`

サンドボックス動作の設定。これを使用して、コマンドサンドボックスを有効にし、ネットワーク制限をプログラムで設定します。

```typescript
type SandboxSettings = {
  enabled?: boolean;
  autoAllowBashIfSandboxed?: boolean;
  excludedCommands?: string[];
  allowUnsandboxedCommands?: boolean;
  network?: NetworkSandboxSettings;
  ignoreViolations?: SandboxIgnoreViolations;
  enableWeakerNestedSandbox?: boolean;
}
```

| プロパティ | タイプ | デフォルト | 説明 |
| :------- | :--- | :------ | :---------- |
| `enabled` | `boolean` | `false` | コマンド実行のサンドボックスモードを有効にします |
| `autoAllowBashIfSandboxed` | `boolean` | `false` | サンドボックスが有効な場合、bashコマンドを自動承認します |
| `excludedCommands` | `string[]` | `[]` | サンドボックス制限を常にバイパスするコマンド（例：`['docker']`）。これらはモデルの関与なしに自動的にサンドボックスなしで実行されます |
| `allowUnsandboxedCommands` | `boolean` | `false` | モデルがサンドボックス外でコマンドを実行するようにリクエストすることを許可します。`true`の場合、モデルはツール入力で`dangerouslyDisableSandbox`を設定でき、これは[パーミッションシステム](#permissions-fallback-for-unsandboxed-commands)にフォールバックします |
| `network` | [`NetworkSandboxSettings`](#networksandboxsettings) | `undefined` | ネットワーク固有のサンドボックス設定 |
| `ignoreViolations` | [`SandboxIgnoreViolations`](#sandboxignoreviolations) | `undefined` | 無視するサンドボックス違反を設定します |
| `enableWeakerNestedSandbox` | `boolean` | `false` | 互換性のためにより弱いネストされたサンドボックスを有効にします |

<Note>
**ファイルシステムとネットワークアクセス制限**はサンドボックス設定を使用して設定されません。代わりに、[パーミッションルール](https://code.claude.com/docs/ja/settings#permission-settings)から派生します：

- **ファイルシステム読み取り制限**：読み取り拒否ルール
- **ファイルシステム書き込み制限**：編集許可/拒否ルール
- **ネットワーク制限**：WebFetch許可/拒否ルール

コマンド実行サンドボックスにはサンドボックス設定を使用し、ファイルシステムとネットワークアクセス制御にはパーミッションルールを使用します。
</Note>

#### 使用例

```typescript
import { query } from "@anthropic-ai/claude-agent-sdk";

const result = await query({
  prompt: "Build and test my project",
  options: {
    sandbox: {
      enabled: true,
      autoAllowBashIfSandboxed: true,
      excludedCommands: ["docker"],
      network: {
        allowLocalBinding: true,
        allowUnixSockets: ["/var/run/docker.sock"]
      }
    }
  }
});
```

### `NetworkSandboxSettings`

サンドボックスモード用のネットワーク固有の設定。

```typescript
type NetworkSandboxSettings = {
  allowLocalBinding?: boolean;
  allowUnixSockets?: string[];
  allowAllUnixSockets?: boolean;
  httpProxyPort?: number;
  socksProxyPort?: number;
}
```

| プロパティ | タイプ | デフォルト | 説明 |
| :------- | :--- | :------ | :---------- |
| `allowLocalBinding` | `boolean` | `false` | プロセスがローカルポートにバインドすることを許可します（例：開発サーバー用） |
| `allowUnixSockets` | `string[]` | `[]` | プロセスがアクセスできるUnixソケットパス（例：Dockerソケット） |
| `allowAllUnixSockets` | `boolean` | `false` | すべてのUnixソケットへのアクセスを許可します |
| `httpProxyPort` | `number` | `undefined` | ネットワークリクエスト用のHTTPプロキシポート |
| `socksProxyPort` | `number` | `undefined` | ネットワークリクエスト用のSOCKSプロキシポート |

### `SandboxIgnoreViolations`

特定のサンドボックス違反を無視するための設定。

```typescript
type SandboxIgnoreViolations = {
  file?: string[];
  network?: string[];
}
```

| プロパティ | タイプ | デフォルト | 説明 |
| :------- | :--- | :------ | :---------- |
| `file` | `string[]` | `[]` | 違反を無視するファイルパスパターン |
| `network` | `string[]` | `[]` | 違反を無視するネットワークパターン |

### サンドボックスなしコマンドのパーミッションフォールバック

`allowUnsandboxedCommands`が有効な場合、モデルはツール入力で`dangerouslyDisableSandbox: true`を設定することで、サンドボックス外でコマンドを実行するようにリクエストできます。これらのリクエストは既存のパーミッションシステムにフォールバックします。つまり、`canUseTool`ハンドラーが呼び出され、カスタム認可ロジックを実装できます。

<Note>
**`excludedCommands` vs `allowUnsandboxedCommands`：**
- `excludedCommands`：サンドボックスを常に自動的にバイパスするコマンドの静的リスト（例：`['docker']`）。モデルはこれを制御できません。
- `allowUnsandboxedCommands`：モデルが実行時にツール入力で`dangerouslyDisableSandbox: true`を設定することで、サンドボックスなし実行をリクエストするかどうかを決定できます。
</Note>

```typescript
import { query } from "@anthropic-ai/claude-agent-sdk";

const result = await query({
  prompt: "Deploy my application",
  options: {
    sandbox: {
      enabled: true,
      allowUnsandboxedCommands: true  // モデルはサンドボックスなし実行をリクエストできます
    },
    permissionMode: "default",
    canUseTool: async (tool, input) => {
      // モデルがサンドボックスをバイパスするようにリクエストしているかチェックします
      if (tool === "Bash" && input.dangerouslyDisableSandbox) {
        // モデルはこのコマンドをサンドボックス外で実行したいと考えています
        console.log(`Unsandboxed command requested: ${input.command}`);

        // 許可する場合はtrueを返し、拒否する場合はfalseを返します
        return isCommandAuthorized(input.command);
      }
      return true;
    }
  }
});
```

このパターンにより、以下が可能になります：

- **モデルリクエストの監査**：モデルがサンドボックスなし実行をリクエストするときをログに記録します
- **許可リストの実装**：特定のコマンドのみがサンドボックスなしで実行されることを許可します
- **承認ワークフローの追加**：特権操作に明示的な認可を要求します

<Warning>
`dangerouslyDisableSandbox: true`で実行されるコマンドは完全なシステムアクセスを持ちます。`canUseTool`ハンドラーがこれらのリクエストを慎重に検証することを確認してください。
</Warning>

## 関連項目

- [SDKの概要](/docs/ja/agent-sdk/overview) - 一般的なSDKの概念
- [Python SDKリファレンス](/docs/ja/agent-sdk/python) - Python SDKドキュメント
- [CLIリファレンス](https://code.claude.com/docs/ja/cli-reference) - コマンドラインインターフェース
- [一般的なワークフロー](https://code.claude.com/docs/ja/common-workflows) - ステップバイステップガイド