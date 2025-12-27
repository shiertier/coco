# 使用鉤子攔截和控制代理行為

在關鍵執行點使用鉤子攔截和自訂代理行為

---

鉤子讓您在關鍵點攔截代理執行，以新增驗證、日誌記錄、安全控制或自訂邏輯。使用鉤子，您可以：

- **阻止危險操作**在執行前，例如破壞性 shell 命令或未授權的檔案存取
- **記錄和審計**每個工具呼叫，用於合規性、除錯或分析
- **轉換輸入和輸出**以清理資料、注入認證或重定向檔案路徑
- **要求人工批准**敏感操作，例如資料庫寫入或 API 呼叫
- **追蹤會話生命週期**以管理狀態、清理資源或傳送通知

鉤子有兩個部分：

1. **回呼函式**：鉤子觸發時執行的邏輯
2. **鉤子配置**：告訴 SDK 要鉤入哪個事件（例如 `PreToolUse`）以及要匹配哪些工具

以下範例阻止代理修改 `.env` 檔案。首先定義一個檢查檔案路徑的回呼，然後將其傳遞給 `query()` 以在任何 Write 或 Edit 工具呼叫前執行：

<CodeGroup>

```python Python
import asyncio
from claude_agent_sdk import query, ClaudeAgentOptions, HookMatcher

# 定義一個接收工具呼叫詳細資訊的鉤子回呼
async def protect_env_files(input_data, tool_use_id, context):
    # 從工具的輸入引數中提取檔案路徑
    file_path = input_data['tool_input'].get('file_path', '')
    file_name = file_path.split('/')[-1]

    # 如果目標是 .env 檔案，則阻止操作
    if file_name == '.env':
        return {
            'hookSpecificOutput': {
                'hookEventName': input_data['hook_event_name'],
                'permissionDecision': 'deny',
                'permissionDecisionReason': 'Cannot modify .env files'
            }
        }

    # 返回空物件以允許操作
    return {}

async def main():
    async for message in query(
        prompt="Update the database configuration",
        options=ClaudeAgentOptions(
            hooks={
                # 為 PreToolUse 事件註冊鉤子
                # 匹配器只篩選 Write 和 Edit 工具呼叫
                'PreToolUse': [HookMatcher(matcher='Write|Edit', hooks=[protect_env_files])]
            }
        )
    ):
        print(message)

asyncio.run(main())
```

```typescript TypeScript
import { query, HookCallback, PreToolUseHookInput } from "@anthropic-ai/claude-agent-sdk";

// 使用 HookCallback 型別定義鉤子回呼
const protectEnvFiles: HookCallback = async (input, toolUseID, { signal }) => {
  // 將輸入轉換為特定鉤子型別以確保型別安全
  const preInput = input as PreToolUseHookInput;

  // 從工具的輸入引數中提取檔案路徑
  const filePath = preInput.tool_input?.file_path as string;
  const fileName = filePath?.split('/').pop();

  // 如果目標是 .env 檔案，則阻止操作
  if (fileName === '.env') {
    return {
      hookSpecificOutput: {
        hookEventName: input.hook_event_name,
        permissionDecision: 'deny',
        permissionDecisionReason: 'Cannot modify .env files'
      }
    };
  }

  // 返回空物件以允許操作
  return {};
};

for await (const message of query({
  prompt: "Update the database configuration",
  options: {
    hooks: {
      // 為 PreToolUse 事件註冊鉤子
      // 匹配器只篩選 Write 和 Edit 工具呼叫
      PreToolUse: [{ matcher: 'Write|Edit', hooks: [protectEnvFiles] }]
    }
  }
})) {
  console.log(message);
}
```

</CodeGroup>

這是一個 `PreToolUse` 鉤子。它在工具執行前執行，可以根據您的邏輯阻止或允許操作。本指南的其餘部分涵蓋所有可用的鉤子、它們的配置選項以及常見用例的模式。

## 可用的鉤子

SDK 為代理執行的不同階段提供鉤子。某些鉤子在兩個 SDK 中都可用，而其他鉤子僅在 TypeScript 中可用，因為 Python SDK 不支援它們。

| 鉤子事件 | Python SDK | TypeScript SDK | 觸發條件 | 範例用例 |
|------------|------------|----------------|------------------|------------------|
| `PreToolUse` | 是 | 是 | 工具呼叫請求（可以阻止或修改） | 阻止危險的 shell 命令 |
| `PostToolUse` | 是 | 是 | 工具執行結果 | 將所有檔案變更記錄到審計軌跡 |
| `PostToolUseFailure` | 否 | 是 | 工具執行失敗 | 處理或記錄工具錯誤 |
| `UserPromptSubmit` | 是 | 是 | 使用者提示提交 | 將額外內容注入提示 |
| `Stop` | 是 | 是 | 代理執行停止 | 在退出前儲存會話狀態 |
| `SubagentStart` | 否 | 是 | 子代理初始化 | 追蹤平行任務生成 |
| `SubagentStop` | 是 | 是 | 子代理完成 | 聚合來自平行任務的結果 |
| `PreCompact` | 是 | 是 | 對話壓縮請求 | 在摘要前存檔完整記錄 |
| `PermissionRequest` | 否 | 是 | 將顯示權限對話 | 自訂權限處理 |
| `SessionStart` | 否 | 是 | 會話初始化 | 初始化日誌記錄和遙測 |
| `SessionEnd` | 否 | 是 | 會話終止 | 清理臨時資源 |
| `Notification` | 否 | 是 | 代理狀態訊息 | 將代理狀態更新傳送到 Slack 或 PagerDuty |

## 常見用例

鉤子足夠靈活，可以處理許多不同的場景。以下是按類別組織的一些最常見的模式。

<Tabs>
  <Tab title="安全">
    - 阻止危險命令（例如 `rm -rf /`、破壞性 SQL）
    - 在寫入操作前驗證檔案路徑
    - 強制執行工具使用的允許清單/封鎖清單
  </Tab>
  <Tab title="日誌記錄">
    - 建立所有代理操作的審計軌跡
    - 追蹤執行指標和效能
    - 在開發中除錯代理行為
  </Tab>
  <Tab title="工具攔截">
    - 將檔案操作重定向到沙箱目錄
    - 注入環境變數或認證
    - 轉換工具輸入或輸出
  </Tab>
  <Tab title="授權">
    - 實施基於角色的存取控制
    - 要求對敏感操作進行人工批准
    - 限制特定工具使用的速率
  </Tab>
</Tabs>

## 配置鉤子

要為您的代理配置鉤子，在呼叫 `query()` 時在 `options.hooks` 參數中傳遞鉤子：

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

`hooks` 選項是一個字典（Python）或物件（TypeScript），其中：
- **鍵**是[鉤子事件名稱](#available-hooks)（例如 `'PreToolUse'`、`'PostToolUse'`、`'Stop'`）
- **值**是[匹配器](#matchers)的陣列，每個都包含一個可選的篩選模式和您的[回呼函式](#callback-function-inputs)

您的鉤子回呼函式接收[輸入資料](#input-data)關於事件的資訊，並返回[回應](#callback-outputs)，以便代理知道是否允許、阻止或修改操作。

### 匹配器

使用匹配器篩選哪些工具觸發您的回呼：

| 選項 | 型別 | 預設值 | 描述 |
|--------|------|---------|-------------|
| `matcher` | `string` | `undefined` | 用於匹配工具名稱的正規表達式模式。內建工具包括 `Bash`、`Read`、`Write`、`Edit`、`Glob`、`Grep`、`WebFetch`、`Task` 等。MCP 工具使用模式 `mcp__<server>__<action>`。 |
| `hooks` | `HookCallback[]` | - | 必需。當模式匹配時執行的回呼函式陣列 |
| `timeout` | `number` | `60` | 超時時間（秒）；對於進行外部 API 呼叫的鉤子，請增加此值 |

盡可能使用 `matcher` 模式來定位特定工具。具有 `'Bash'` 的匹配器只對 Bash 命令執行，而省略模式會對每個工具呼叫執行您的回呼。請注意，匹配器只按**工具名稱**篩選，不按檔案路徑或其他引數篩選——要按檔案路徑篩選，請在回呼內檢查 `tool_input.file_path`。

匹配器只適用於工具型鉤子（`PreToolUse`、`PostToolUse`、`PostToolUseFailure`、`PermissionRequest`）。對於生命週期鉤子（如 `Stop`、`SessionStart` 和 `Notification`），匹配器被忽略，鉤子對該型別的所有事件都會觸發。

<Tip>
**發現工具名稱：**檢查會話開始時初始系統訊息中的 `tools` 陣列，或新增不帶匹配器的鉤子以記錄所有工具呼叫。

**MCP 工具命名：**MCP 工具始終以 `mcp__` 開頭，後跟伺服器名稱和操作：`mcp__<server>__<action>`。例如，如果您配置一個名為 `playwright` 的伺服器，其工具將被命名為 `mcp__playwright__browser_screenshot`、`mcp__playwright__browser_click` 等。伺服器名稱來自您在 `mcpServers` 配置中使用的鍵。
</Tip>

此範例使用匹配器在 `PreToolUse` 事件觸發時只對檔案修改工具執行鉤子：

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

### 回呼函式輸入

每個鉤子回呼接收三個引數：

1. **輸入資料**（`dict` / `HookInput`）：事件詳細資訊。請參閱[輸入資料](#input-data)以了解欄位
2. **工具使用 ID**（`str | None` / `string | null`）：關聯 `PreToolUse` 和 `PostToolUse` 事件
3. **上下文**（`HookContext`）：在 TypeScript 中，包含 `signal` 屬性（`AbortSignal`）用於取消。將此傳遞給非同步操作（如 `fetch()`），以便在鉤子超時時自動取消。在 Python 中，此引數保留供將來使用。

### 輸入資料

您的鉤子回呼的第一個引數包含有關事件的資訊。欄位名稱在 SDK 中相同（都使用 snake_case）。

**所有鉤子型別中存在的常見欄位**：

| 欄位 | 型別 | 描述 |
|-------|------|-------------|
| `hook_event_name` | `string` | 鉤子型別（`PreToolUse`、`PostToolUse` 等） |
| `session_id` | `string` | 目前會話識別碼 |
| `transcript_path` | `string` | 對話記錄的路徑 |
| `cwd` | `string` | 目前工作目錄 |

**鉤子特定欄位**因鉤子型別而異。標記為 <sup>TS</sup> 的項目僅在 TypeScript SDK 中可用：

| 欄位 | 型別 | 描述 | 鉤子 |
|-------|------|-------------|-------|
| `tool_name` | `string` | 被呼叫的工具名稱 | PreToolUse、PostToolUse、PostToolUseFailure<sup>TS</sup>、PermissionRequest<sup>TS</sup> |
| `tool_input` | `object` | 傳遞給工具的引數 | PreToolUse、PostToolUse、PostToolUseFailure<sup>TS</sup>、PermissionRequest<sup>TS</sup> |
| `tool_response` | `any` | 工具執行返回的結果 | PostToolUse |
| `error` | `string` | 工具執行失敗的錯誤訊息 | PostToolUseFailure<sup>TS</sup> |
| `is_interrupt` | `boolean` | 失敗是否由中斷引起 | PostToolUseFailure<sup>TS</sup> |
| `prompt` | `string` | 使用者的提示文字 | UserPromptSubmit |
| `stop_hook_active` | `boolean` | 停止鉤子是否正在處理 | Stop、SubagentStop |
| `agent_id` | `string` | 子代理的唯一識別碼 | SubagentStart<sup>TS</sup>、SubagentStop<sup>TS</sup> |
| `agent_type` | `string` | 子代理的型別/角色 | SubagentStart<sup>TS</sup> |
| `agent_transcript_path` | `string` | 子代理的對話記錄路徑 | SubagentStop<sup>TS</sup> |
| `trigger` | `string` | 觸發壓縮的原因：`manual` 或 `auto` | PreCompact |
| `custom_instructions` | `string` | 為壓縮提供的自訂指示 | PreCompact |
| `permission_suggestions` | `array` | 工具的建議權限更新 | PermissionRequest<sup>TS</sup> |
| `source` | `string` | 會話如何開始：`startup`、`resume`、`clear` 或 `compact` | SessionStart<sup>TS</sup> |
| `reason` | `string` | 會話結束的原因：`clear`、`logout`、`prompt_input_exit`、`bypass_permissions_disabled` 或 `other` | SessionEnd<sup>TS</sup> |
| `message` | `string` | 來自代理的狀態訊息 | Notification<sup>TS</sup> |
| `notification_type` | `string` | 通知型別：`permission_prompt`、`idle_prompt`、`auth_success` 或 `elicitation_dialog` | Notification<sup>TS</sup> |
| `title` | `string` | 代理設定的可選標題 | Notification<sup>TS</sup> |

下面的程式碼定義了一個鉤子回呼，使用 `tool_name` 和 `tool_input` 記錄每個工具呼叫的詳細資訊：

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

### 回呼輸出

您的回呼函式返回一個物件，告訴 SDK 如何繼續。返回空物件 `{}` 以允許操作而不進行更改。要阻止、修改或新增操作的上下文，請返回包含 `hookSpecificOutput` 欄位的物件，其中包含您的決定。

**頂級欄位**（在 `hookSpecificOutput` 外）：

| 欄位 | 型別 | 描述 |
|-------|------|-------------|
| `continue` | `boolean` | 此鉤子後代理是否應繼續（預設：`true`） |
| `stopReason` | `string` | 當 `continue` 為 `false` 時顯示的訊息 |
| `suppressOutput` | `boolean` | 隱藏記錄中的 stdout（預設：`false`） |
| `systemMessage` | `string` | 注入對話中供 Claude 查看的訊息 |

**`hookSpecificOutput` 內的欄位**：

| 欄位 | 型別 | 鉤子 | 描述 |
|-------|------|-------|-------------|
| `hookEventName` | `string` | 全部 | 必需。使用 `input.hook_event_name` 匹配目前事件 |
| `permissionDecision` | `'allow'` \| `'deny'` \| `'ask'` | PreToolUse | 控制工具是否執行 |
| `permissionDecisionReason` | `string` | PreToolUse | 為決定向 Claude 顯示的說明 |
| `updatedInput` | `object` | PreToolUse | 修改的工具輸入（需要 `permissionDecision: 'allow'`） |
| `additionalContext` | `string` | PostToolUse、UserPromptSubmit、SessionStart<sup>TS</sup>、SubagentStart<sup>TS</sup> | 新增到對話的上下文 |

此範例阻止對 `/etc` 目錄的寫入操作，同時注入系統訊息以提醒 Claude 關於安全檔案實踐：

<CodeGroup>

```python Python
async def block_etc_writes(input_data, tool_use_id, context):
    file_path = input_data['tool_input'].get('file_path', '')

    if file_path.startswith('/etc'):
        return {
            # 頂級欄位：將指導注入對話
            'systemMessage': 'Remember: system directories like /etc are protected.',
            # hookSpecificOutput：阻止操作
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
      // 頂級欄位：將指導注入對話
      systemMessage: 'Remember: system directories like /etc are protected.',
      // hookSpecificOutput：阻止操作
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

#### 權限決定流程

當多個鉤子或權限規則適用時，SDK 按此順序評估它們：

1. **拒絕**規則首先被檢查（任何匹配 = 立即拒絕）。
2. **詢問**規則其次被檢查。
3. **允許**規則第三被檢查。
4. **預設為詢問**如果沒有匹配。

如果任何鉤子返回 `deny`，操作被阻止——返回 `allow` 的其他鉤子不會覆蓋它。

#### 阻止工具

返回拒絕決定以防止工具執行：

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

#### 修改工具輸入

返回更新的輸入以更改工具接收的內容：

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
使用 `updatedInput` 時，您也必須包含 `permissionDecision`。始終返回新物件而不是變更原始 `tool_input`。
</Note>

#### 新增系統訊息

將上下文注入對話：

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

#### 自動批准特定工具

繞過受信任工具的權限提示。當您希望某些操作在沒有使用者確認的情況下執行時，這很有用：

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
`permissionDecision` 欄位接受三個值：`'allow'`（自動批准）、`'deny'`（阻止）或 `'ask'`（提示確認）。
</Note>

## 處理進階場景

這些模式幫助您為複雜用例構建更複雜的鉤子系統。

### 鏈接多個鉤子

鉤子按它們在陣列中出現的順序執行。保持每個鉤子專注於單一責任，並為複雜邏輯鏈接多個鉤子。此範例為每個工具呼叫執行所有四個鉤子（未指定匹配器）：

<CodeGroup>

```python Python
options = ClaudeAgentOptions(
    hooks={
        'PreToolUse': [
            HookMatcher(hooks=[rate_limiter]),        # 首先：檢查速率限制
            HookMatcher(hooks=[authorization_check]), # 其次：驗證權限
            HookMatcher(hooks=[input_sanitizer]),     # 第三：清理輸入
            HookMatcher(hooks=[audit_logger])         # 最後：記錄操作
        ]
    }
)
```

```typescript TypeScript
const options = {
  hooks: {
    'PreToolUse': [
      { hooks: [rateLimiter] },        // 首先：檢查速率限制
      { hooks: [authorizationCheck] }, // 其次：驗證權限
      { hooks: [inputSanitizer] },     // 第三：清理輸入
      { hooks: [auditLogger] }         // 最後：記錄操作
    ]
  }
};
```

</CodeGroup>

### 使用正規表達式的工具特定匹配器

使用正規表達式模式匹配多個工具：

<CodeGroup>

```python Python
options = ClaudeAgentOptions(
    hooks={
        'PreToolUse': [
            # 匹配檔案修改工具
            HookMatcher(matcher='Write|Edit|Delete', hooks=[file_security_hook]),

            # 匹配所有 MCP 工具
            HookMatcher(matcher='^mcp__', hooks=[mcp_audit_hook]),

            # 匹配所有內容（無匹配器）
            HookMatcher(hooks=[global_logger])
        ]
    }
)
```

```typescript TypeScript
const options = {
  hooks: {
    'PreToolUse': [
      // 匹配檔案修改工具
      { matcher: 'Write|Edit|Delete', hooks: [fileSecurityHook] },

      // 匹配所有 MCP 工具
      { matcher: '^mcp__', hooks: [mcpAuditHook] },

      // 匹配所有內容（無匹配器）
      { hooks: [globalLogger] }
    ]
  }
};
```

</CodeGroup>

<Note>
匹配器只匹配**工具名稱**，不匹配檔案路徑或其他引數。要按檔案路徑篩選，請在鉤子回呼內檢查 `tool_input.file_path`。
</Note>

### 追蹤子代理活動

使用 `SubagentStop` 鉤子監視子代理完成。`tool_use_id` 幫助關聯父代理呼叫與其子代理：

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

### 鉤子中的非同步操作

鉤子可以執行非同步操作，例如 HTTP 請求。通過捕獲異常而不是拋出異常來優雅地處理錯誤。在 TypeScript 中，將 `signal` 傳遞給 `fetch()`，以便在鉤子超時時請求取消：

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
    // 傳遞 signal 以進行適當的取消
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

### 傳送通知（僅限 TypeScript）

使用 `Notification` 鉤子接收來自代理的狀態更新，並將其轉發到外部服務，如 Slack 或監視儀表板：

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

## 修復常見問題

本節涵蓋常見問題及其解決方法。

### 鉤子未觸發

- 驗證鉤子事件名稱正確且區分大小寫（`PreToolUse`，不是 `preToolUse`）
- 檢查您的匹配器模式是否與工具名稱完全匹配
- 確保鉤子在 `options.hooks` 中的正確事件型別下
- 對於 `SubagentStop`、`Stop`、`SessionStart`、`SessionEnd` 和 `Notification` 鉤子，匹配器被忽略。這些鉤子對該型別的所有事件都會觸發。
- 當代理達到 [`max_turns`](/docs/zh-TW/agent-sdk/python#configuration-options) 限制時，鉤子可能不會觸發，因為會話在鉤子可以執行前結束

### 匹配器未按預期篩選

匹配器只匹配**工具名稱**，不匹配檔案路徑或其他引數。要按檔案路徑篩選，請在鉤子內檢查 `tool_input.file_path`：

```typescript
const myHook: HookCallback = async (input, toolUseID, { signal }) => {
  const preInput = input as PreToolUseHookInput;
  const filePath = preInput.tool_input?.file_path as string;
  if (!filePath?.endsWith('.md')) return {};  // 跳過非 markdown 檔案
  // 處理 markdown 檔案...
};
```

### 鉤子超時

- 增加 `HookMatcher` 配置中的 `timeout` 值
- 在 TypeScript 中使用第三個回呼引數中的 `AbortSignal` 以優雅地處理取消

### 工具意外被阻止

- 檢查所有 `PreToolUse` 鉤子是否返回 `permissionDecision: 'deny'`
- 新增日誌記錄到您的鉤子以查看它們返回的 `permissionDecisionReason`
- 驗證匹配器模式不是太寬泛（空匹配器匹配所有工具）

### 修改的輸入未應用

- 確保 `updatedInput` 在 `hookSpecificOutput` 內，而不是在頂級：

  ```typescript
  return {
    hookSpecificOutput: {
      hookEventName: input.hook_event_name,
      permissionDecision: 'allow',
      updatedInput: { command: 'new command' }
    }
  };
  ```

- 您還必須返回 `permissionDecision: 'allow'` 以使輸入修改生效
- 在 `hookSpecificOutput` 中包含 `hookEventName` 以識別輸出適用於哪個鉤子型別

### 會話鉤子不可用

`SessionStart`、`SessionEnd` 和 `Notification` 鉤子僅在 TypeScript SDK 中可用。由於設定限制，Python SDK 不支援這些事件。

### 子代理權限提示倍增

生成多個子代理時，每個子代理可能會單獨請求權限。子代理不會自動繼承父代理權限。要避免重複提示，請使用 `PreToolUse` 鉤子自動批准特定工具，或配置適用於子代理會話的權限規則。

### 子代理的遞迴鉤子迴圈

生成子代理的 `UserPromptSubmit` 鉤子如果這些子代理觸發相同鉤子，可能會建立無限迴圈。要防止這種情況：

- 在生成子代理前檢查鉤子輸入中的子代理指示符
- 使用 `parent_tool_use_id` 欄位檢測您是否已在子代理上下文中
- 將鉤子範圍限制為僅對頂級代理會話執行

### systemMessage 未出現在輸出中

`systemMessage` 欄位將上下文新增到模型看到的對話中，但它可能不會出現在所有 SDK 輸出模式中。如果您需要將鉤子決定呈現給您的應用程式，請單獨記錄它們或使用專用輸出通道。

## 了解更多

- [權限](/docs/zh-TW/agent-sdk/permissions)：控制您的代理可以做什麼
- [自訂工具](/docs/zh-TW/agent-sdk/custom-tools)：構建工具以擴展代理功能
- [TypeScript SDK 參考](/docs/zh-TW/agent-sdk/typescript)
- [Python SDK 參考](/docs/zh-TW/agent-sdk/python)