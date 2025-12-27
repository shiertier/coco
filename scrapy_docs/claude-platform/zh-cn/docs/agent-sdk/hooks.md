# 使用钩子拦截和控制代理行为

在关键执行点使用钩子拦截和自定义代理行为

---

钩子让你在关键点拦截代理执行，以添加验证、日志记录、安全控制或自定义逻辑。使用钩子，你可以：

- **阻止危险操作**在执行前，如破坏性 shell 命令或未授权的文件访问
- **记录和审计**每个工具调用，用于合规性、调试或分析
- **转换输入和输出**以清理数据、注入凭证或重定向文件路径
- **要求人工批准**敏感操作，如数据库写入或 API 调用
- **跟踪会话生命周期**以管理状态、清理资源或发送通知

钩子有两个部分：

1. **回调函数**：钩子触发时运行的逻辑
2. **钩子配置**：告诉 SDK 要钩入哪个事件（如 `PreToolUse`）以及要匹配哪些工具

以下示例阻止代理修改 `.env` 文件。首先，定义一个检查文件路径的回调，然后将其传递给 `query()` 以在任何 Write 或 Edit 工具调用前运行：

<CodeGroup>

```python Python
import asyncio
from claude_agent_sdk import query, ClaudeAgentOptions, HookMatcher

# 定义一个接收工具调用详情的钩子回调
async def protect_env_files(input_data, tool_use_id, context):
    # 从工具的输入参数中提取文件路径
    file_path = input_data['tool_input'].get('file_path', '')
    file_name = file_path.split('/')[-1]

    # 如果目标是 .env 文件，则阻止操作
    if file_name == '.env':
        return {
            'hookSpecificOutput': {
                'hookEventName': input_data['hook_event_name'],
                'permissionDecision': 'deny',
                'permissionDecisionReason': 'Cannot modify .env files'
            }
        }

    # 返回空对象以允许操作
    return {}

async def main():
    async for message in query(
        prompt="Update the database configuration",
        options=ClaudeAgentOptions(
            hooks={
                # 为 PreToolUse 事件注册钩子
                # 匹配器仅过滤 Write 和 Edit 工具调用
                'PreToolUse': [HookMatcher(matcher='Write|Edit', hooks=[protect_env_files])]
            }
        )
    ):
        print(message)

asyncio.run(main())
```

```typescript TypeScript
import { query, HookCallback, PreToolUseHookInput } from "@anthropic-ai/claude-agent-sdk";

// 使用 HookCallback 类型定义钩子回调
const protectEnvFiles: HookCallback = async (input, toolUseID, { signal }) => {
  // 将输入转换为特定钩子类型以获得类型安全
  const preInput = input as PreToolUseHookInput;

  // 从工具的输入参数中提取文件路径
  const filePath = preInput.tool_input?.file_path as string;
  const fileName = filePath?.split('/').pop();

  // 如果目标是 .env 文件，则阻止操作
  if (fileName === '.env') {
    return {
      hookSpecificOutput: {
        hookEventName: input.hook_event_name,
        permissionDecision: 'deny',
        permissionDecisionReason: 'Cannot modify .env files'
      }
    };
  }

  // 返回空对象以允许操作
  return {};
};

for await (const message of query({
  prompt: "Update the database configuration",
  options: {
    hooks: {
      // 为 PreToolUse 事件注册钩子
      // 匹配器仅过滤 Write 和 Edit 工具调用
      PreToolUse: [{ matcher: 'Write|Edit', hooks: [protectEnvFiles] }]
    }
  }
})) {
  console.log(message);
}
```

</CodeGroup>

这是一个 `PreToolUse` 钩子。它在工具执行前运行，可以根据你的逻辑阻止或允许操作。本指南的其余部分涵盖所有可用的钩子、它们的配置选项以及常见用例的模式。

## 可用的钩子

SDK 为代理执行的不同阶段提供钩子。某些钩子在两个 SDK 中都可用，而其他钩子仅在 TypeScript 中可用，因为 Python SDK 不支持它们。

| 钩子事件 | Python SDK | TypeScript SDK | 触发条件 | 示例用例 |
|------------|------------|----------------|------------------|------------------|
| `PreToolUse` | 是 | 是 | 工具调用请求（可以阻止或修改） | 阻止危险的 shell 命令 |
| `PostToolUse` | 是 | 是 | 工具执行结果 | 将所有文件更改记录到审计跟踪 |
| `PostToolUseFailure` | 否 | 是 | 工具执行失败 | 处理或记录工具错误 |
| `UserPromptSubmit` | 是 | 是 | 用户提示提交 | 将额外上下文注入提示 |
| `Stop` | 是 | 是 | 代理执行停止 | 在退出前保存会话状态 |
| `SubagentStart` | 否 | 是 | 子代理初始化 | 跟踪并行任务生成 |
| `SubagentStop` | 是 | 是 | 子代理完成 | 聚合来自并行任务的结果 |
| `PreCompact` | 是 | 是 | 会话压缩请求 | 在总结前存档完整记录 |
| `PermissionRequest` | 否 | 是 | 权限对话将显示 | 自定义权限处理 |
| `SessionStart` | 否 | 是 | 会话初始化 | 初始化日志记录和遥测 |
| `SessionEnd` | 否 | 是 | 会话终止 | 清理临时资源 |
| `Notification` | 否 | 是 | 代理状态消息 | 将代理状态更新发送到 Slack 或 PagerDuty |

## 常见用例

钩子足够灵活，可以处理许多不同的场景。以下是按类别组织的一些最常见的模式。

<Tabs>
  <Tab title="安全">
    - 阻止危险命令（如 `rm -rf /`、破坏性 SQL）
    - 在写入操作前验证文件路径
    - 为工具使用强制执行允许列表/阻止列表
  </Tab>
  <Tab title="日志记录">
    - 创建所有代理操作的审计跟踪
    - 跟踪执行指标和性能
    - 在开发中调试代理行为
  </Tab>
  <Tab title="工具拦截">
    - 将文件操作重定向到沙箱目录
    - 注入环境变量或凭证
    - 转换工具输入或输出
  </Tab>
  <Tab title="授权">
    - 实现基于角色的访问控制
    - 要求对敏感操作进行人工批准
    - 限制特定工具使用的速率
  </Tab>
</Tabs>

## 配置钩子

要为代理配置钩子，在调用 `query()` 时在 `options.hooks` 参数中传递钩子：

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

`hooks` 选项是一个字典（Python）或对象（TypeScript），其中：
- **键**是[钩子事件名称](#available-hooks)（例如 `'PreToolUse'`、`'PostToolUse'`、`'Stop'`）
- **值**是[匹配器](#matchers)数组，每个包含可选的过滤模式和你的[回调函数](#callback-function-inputs)

你的钩子回调函数接收关于事件的[输入数据](#input-data)，并返回一个[响应](#callback-outputs)，以便代理知道是否允许、阻止或修改操作。

### 匹配器

使用匹配器过滤哪些工具触发你的回调：

| 选项 | 类型 | 默认值 | 描述 |
|--------|------|---------|-------------|
| `matcher` | `string` | `undefined` | 用于匹配工具名称的正则表达式模式。内置工具包括 `Bash`、`Read`、`Write`、`Edit`、`Glob`、`Grep`、`WebFetch`、`Task` 等。MCP 工具使用模式 `mcp__<server>__<action>`。 |
| `hooks` | `HookCallback[]` | - | 必需。当模式匹配时执行的回调函数数组 |
| `timeout` | `number` | `60` | 超时时间（秒）；对于进行外部 API 调用的钩子，请增加此值 |

尽可能使用 `matcher` 模式来针对特定工具。匹配器为 `'Bash'` 仅对 Bash 命令运行，而省略模式则对每个工具调用运行你的回调。请注意，匹配器仅按**工具名称**过滤，不按文件路径或其他参数过滤——要按文件路径过滤，请在回调中检查 `tool_input.file_path`。

匹配器仅适用于基于工具的钩子（`PreToolUse`、`PostToolUse`、`PostToolUseFailure`、`PermissionRequest`）。对于生命周期钩子如 `Stop`、`SessionStart` 和 `Notification`，匹配器被忽略，钩子对该类型的所有事件触发。

<Tip>
**发现工具名称：**检查会话启动时初始系统消息中的 `tools` 数组，或添加没有匹配器的钩子来记录所有工具调用。

**MCP 工具命名：**MCP 工具始终以 `mcp__` 开头，后跟服务器名称和操作：`mcp__<server>__<action>`。例如，如果你配置了一个名为 `playwright` 的服务器，其工具将被命名为 `mcp__playwright__browser_screenshot`、`mcp__playwright__browser_click` 等。服务器名称来自你在 `mcpServers` 配置中使用的键。
</Tip>

此示例使用匹配器在 `PreToolUse` 事件触发时仅对文件修改工具运行钩子：

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

### 回调函数输入

每个钩子回调接收三个参数：

1. **输入数据**（`dict` / `HookInput`）：事件详情。参见[输入数据](#input-data)了解字段
2. **工具使用 ID**（`str | None` / `string | null`）：关联 `PreToolUse` 和 `PostToolUse` 事件
3. **上下文**（`HookContext`）：在 TypeScript 中，包含 `signal` 属性（`AbortSignal`）用于取消。将其传递给异步操作如 `fetch()`，以便在钩子超时时自动取消。在 Python 中，此参数保留供将来使用。

### 输入数据

你的钩子回调的第一个参数包含关于事件的信息。字段名称在 SDK 中相同（都使用 snake_case）。

**所有钩子类型中存在的常见字段**：

| 字段 | 类型 | 描述 |
|-------|------|-------------|
| `hook_event_name` | `string` | 钩子类型（`PreToolUse`、`PostToolUse` 等） |
| `session_id` | `string` | 当前会话标识符 |
| `transcript_path` | `string` | 对话记录的路径 |
| `cwd` | `string` | 当前工作目录 |

**特定于钩子的字段**因钩子类型而异。标记为 <sup>TS</sup> 的项仅在 TypeScript SDK 中可用：

| 字段 | 类型 | 描述 | 钩子 |
|-------|------|-------------|-------|
| `tool_name` | `string` | 被调用的工具名称 | PreToolUse、PostToolUse、PostToolUseFailure<sup>TS</sup>、PermissionRequest<sup>TS</sup> |
| `tool_input` | `object` | 传递给工具的参数 | PreToolUse、PostToolUse、PostToolUseFailure<sup>TS</sup>、PermissionRequest<sup>TS</sup> |
| `tool_response` | `any` | 从工具执行返回的结果 | PostToolUse |
| `error` | `string` | 工具执行失败的错误消息 | PostToolUseFailure<sup>TS</sup> |
| `is_interrupt` | `boolean` | 失败是否由中断引起 | PostToolUseFailure<sup>TS</sup> |
| `prompt` | `string` | 用户的提示文本 | UserPromptSubmit |
| `stop_hook_active` | `boolean` | 停止钩子是否正在处理 | Stop、SubagentStop |
| `agent_id` | `string` | 子代理的唯一标识符 | SubagentStart<sup>TS</sup>、SubagentStop<sup>TS</sup> |
| `agent_type` | `string` | 子代理的类型/角色 | SubagentStart<sup>TS</sup> |
| `agent_transcript_path` | `string` | 子代理的对话记录路径 | SubagentStop<sup>TS</sup> |
| `trigger` | `string` | 触发压缩的原因：`manual` 或 `auto` | PreCompact |
| `custom_instructions` | `string` | 为压缩提供的自定义指令 | PreCompact |
| `permission_suggestions` | `array` | 工具的建议权限更新 | PermissionRequest<sup>TS</sup> |
| `source` | `string` | 会话如何启动：`startup`、`resume`、`clear` 或 `compact` | SessionStart<sup>TS</sup> |
| `reason` | `string` | 会话为何结束：`clear`、`logout`、`prompt_input_exit`、`bypass_permissions_disabled` 或 `other` | SessionEnd<sup>TS</sup> |
| `message` | `string` | 来自代理的状态消息 | Notification<sup>TS</sup> |
| `notification_type` | `string` | 通知类型：`permission_prompt`、`idle_prompt`、`auth_success` 或 `elicitation_dialog` | Notification<sup>TS</sup> |
| `title` | `string` | 代理设置的可选标题 | Notification<sup>TS</sup> |

下面的代码定义了一个钩子回调，使用 `tool_name` 和 `tool_input` 来记录每个工具调用的详情：

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

### 回调输出

你的回调函数返回一个对象，告诉 SDK 如何继续。返回空对象 `{}` 以允许操作而不做任何更改。要阻止、修改或向操作添加上下文，返回一个包含 `hookSpecificOutput` 字段的对象，其中包含你的决定。

**顶级字段**（在 `hookSpecificOutput` 外）：

| 字段 | 类型 | 描述 |
|-------|------|-------------|
| `continue` | `boolean` | 代理是否应在此钩子后继续（默认：`true`） |
| `stopReason` | `string` | 当 `continue` 为 `false` 时显示的消息 |
| `suppressOutput` | `boolean` | 从记录中隐藏 stdout（默认：`false`） |
| `systemMessage` | `string` | 注入到对话中供 Claude 查看的消息 |

**`hookSpecificOutput` 内的字段**：

| 字段 | 类型 | 钩子 | 描述 |
|-------|------|-------|-------------|
| `hookEventName` | `string` | 全部 | 必需。使用 `input.hook_event_name` 来匹配当前事件 |
| `permissionDecision` | `'allow'` \| `'deny'` \| `'ask'` | PreToolUse | 控制工具是否执行 |
| `permissionDecisionReason` | `string` | PreToolUse | 为决定向 Claude 显示的解释 |
| `updatedInput` | `object` | PreToolUse | 修改的工具输入（需要 `permissionDecision: 'allow'`） |
| `additionalContext` | `string` | PostToolUse、UserPromptSubmit、SessionStart<sup>TS</sup>、SubagentStart<sup>TS</sup> | 添加到对话的上下文 |

此示例阻止对 `/etc` 目录的写入操作，同时注入系统消息以提醒 Claude 关于安全文件实践：

<CodeGroup>

```python Python
async def block_etc_writes(input_data, tool_use_id, context):
    file_path = input_data['tool_input'].get('file_path', '')

    if file_path.startswith('/etc'):
        return {
            # 顶级字段：将指导注入对话
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
      // 顶级字段：将指导注入对话
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

#### 权限决定流程

当多个钩子或权限规则适用时，SDK 按以下顺序评估它们：

1. **拒绝**规则首先被检查（任何匹配 = 立即拒绝）。
2. **询问**规则其次被检查。
3. **允许**规则第三被检查。
4. **默认为询问**如果没有匹配。

如果任何钩子返回 `deny`，操作被阻止——其他返回 `allow` 的钩子不会覆盖它。

#### 阻止工具

返回拒绝决定以防止工具执行：

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

#### 修改工具输入

返回更新的输入以改变工具接收的内容：

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
使用 `updatedInput` 时，你还必须包含 `permissionDecision`。始终返回新对象而不是改变原始 `tool_input`。
</Note>

#### 添加系统消息

将上下文注入对话：

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

#### 自动批准特定工具

绕过受信任工具的权限提示。当你希望某些操作在没有用户确认的情况下运行时，这很有用：

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
`permissionDecision` 字段接受三个值：`'allow'`（自动批准）、`'deny'`（阻止）或 `'ask'`（提示确认）。
</Note>

## 处理高级场景

这些模式帮助你为复杂用例构建更复杂的钩子系统。

### 链接多个钩子

钩子按它们在数组中出现的顺序执行。保持每个钩子专注于单一职责，并链接多个钩子以实现复杂逻辑。此示例为每个工具调用运行所有四个钩子（未指定匹配器）：

<CodeGroup>

```python Python
options = ClaudeAgentOptions(
    hooks={
        'PreToolUse': [
            HookMatcher(hooks=[rate_limiter]),        # 首先：检查速率限制
            HookMatcher(hooks=[authorization_check]), # 其次：验证权限
            HookMatcher(hooks=[input_sanitizer]),     # 第三：清理输入
            HookMatcher(hooks=[audit_logger])         # 最后：记录操作
        ]
    }
)
```

```typescript TypeScript
const options = {
  hooks: {
    'PreToolUse': [
      { hooks: [rateLimiter] },        // 首先：检查速率限制
      { hooks: [authorizationCheck] }, // 其次：验证权限
      { hooks: [inputSanitizer] },     // 第三：清理输入
      { hooks: [auditLogger] }         // 最后：记录操作
    ]
  }
};
```

</CodeGroup>

### 使用正则表达式的工具特定匹配器

使用正则表达式模式匹配多个工具：

<CodeGroup>

```python Python
options = ClaudeAgentOptions(
    hooks={
        'PreToolUse': [
            # 匹配文件修改工具
            HookMatcher(matcher='Write|Edit|Delete', hooks=[file_security_hook]),

            # 匹配所有 MCP 工具
            HookMatcher(matcher='^mcp__', hooks=[mcp_audit_hook]),

            # 匹配所有内容（无匹配器）
            HookMatcher(hooks=[global_logger])
        ]
    }
)
```

```typescript TypeScript
const options = {
  hooks: {
    'PreToolUse': [
      // 匹配文件修改工具
      { matcher: 'Write|Edit|Delete', hooks: [fileSecurityHook] },

      // 匹配所有 MCP 工具
      { matcher: '^mcp__', hooks: [mcpAuditHook] },

      // 匹配所有内容（无匹配器）
      { hooks: [globalLogger] }
    ]
  }
};
```

</CodeGroup>

<Note>
匹配器仅匹配**工具名称**，不匹配文件路径或其他参数。要按文件路径过滤，请在钩子回调中检查 `tool_input.file_path`。
</Note>

### 跟踪子代理活动

使用 `SubagentStop` 钩子监控子代理完成。`tool_use_id` 帮助关联父代理调用与其子代理：

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

### 钩子中的异步操作

钩子可以执行异步操作，如 HTTP 请求。通过捕获异常而不是抛出异常来优雅地处理错误。在 TypeScript 中，将 `signal` 传递给 `fetch()`，以便在钩子超时时请求取消：

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
    // 传递 signal 以进行适当的取消
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

### 发送通知（仅 TypeScript）

使用 `Notification` 钩子接收来自代理的状态更新，并将其转发到外部服务，如 Slack 或监控仪表板：

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

## 修复常见问题

本部分涵盖常见问题及其解决方法。

### 钩子未触发

- 验证钩子事件名称正确且区分大小写（`PreToolUse`，而不是 `preToolUse`）
- 检查你的匹配器模式是否与工具名称完全匹配
- 确保钩子在 `options.hooks` 中的正确事件类型下
- 对于 `SubagentStop`、`Stop`、`SessionStart`、`SessionEnd` 和 `Notification` 钩子，匹配器被忽略。这些钩子对该类型的所有事件触发。
- 当代理达到 [`max_turns`](/docs/zh-CN/agent-sdk/python#configuration-options) 限制时，钩子可能不会触发，因为会话在钩子执行前结束

### 匹配器未按预期过滤

匹配器仅匹配**工具名称**，不匹配文件路径或其他参数。要按文件路径过滤，请在钩子中检查 `tool_input.file_path`：

```typescript
const myHook: HookCallback = async (input, toolUseID, { signal }) => {
  const preInput = input as PreToolUseHookInput;
  const filePath = preInput.tool_input?.file_path as string;
  if (!filePath?.endsWith('.md')) return {};  // 跳过非 markdown 文件
  // 处理 markdown 文件...
};
```

### 钩子超时

- 在 `HookMatcher` 配置中增加 `timeout` 值
- 在 TypeScript 中使用第三个回调参数中的 `AbortSignal` 来优雅地处理取消

### 工具意外被阻止

- 检查所有 `PreToolUse` 钩子是否返回 `permissionDecision: 'deny'`
- 向你的钩子添加日志记录以查看它们返回的 `permissionDecisionReason`
- 验证匹配器模式不会太宽泛（空匹配器匹配所有工具）

### 修改的输入未应用

- 确保 `updatedInput` 在 `hookSpecificOutput` 内，而不是在顶级：

  ```typescript
  return {
    hookSpecificOutput: {
      hookEventName: input.hook_event_name,
      permissionDecision: 'allow',
      updatedInput: { command: 'new command' }
    }
  };
  ```

- 你还必须返回 `permissionDecision: 'allow'` 以使输入修改生效
- 在 `hookSpecificOutput` 中包含 `hookEventName` 以识别输出适用于哪个钩子类型

### 会话钩子不可用

`SessionStart`、`SessionEnd` 和 `Notification` 钩子仅在 TypeScript SDK 中可用。由于设置限制，Python SDK 不支持这些事件。

### 子代理权限提示倍增

生成多个子代理时，每个子代理可能会单独请求权限。子代理不会自动继承父代理权限。要避免重复提示，使用 `PreToolUse` 钩子自动批准特定工具，或配置适用于子代理会话的权限规则。

### 子代理的递归钩子循环

生成子代理的 `UserPromptSubmit` 钩子如果这些子代理触发相同钩子，可能会创建无限循环。要防止这种情况：

- 在生成子代理前检查钩子输入中的子代理指示符
- 使用 `parent_tool_use_id` 字段检测你是否已在子代理上下文中
- 将钩子范围限制为仅在顶级代理会话中运行

### systemMessage 未出现在输出中

`systemMessage` 字段将上下文添加到模型看到的对话中，但它可能不会出现在所有 SDK 输出模式中。如果你需要将钩子决定呈现给你的应用程序，请单独记录它们或使用专用输出通道。

## 了解更多

- [权限](/docs/zh-CN/agent-sdk/permissions)：控制你的代理可以做什么
- [自定义工具](/docs/zh-CN/agent-sdk/custom-tools)：构建工具以扩展代理功能
- [TypeScript SDK 参考](/docs/zh-CN/agent-sdk/typescript)
- [Python SDK 参考](/docs/zh-CN/agent-sdk/python)