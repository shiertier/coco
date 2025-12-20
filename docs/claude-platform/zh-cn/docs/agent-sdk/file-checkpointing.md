# 使用检查点重新调整文件更改

在代理会话期间跟踪文件更改并将文件恢复到任何先前状态

---

文件检查点跟踪在代理会话期间通过 Write、Edit 和 NotebookEdit 工具所做的文件修改，允许您将文件回退到任何先前状态。想要尝试一下？跳转到[交互式示例](#try-it-out)。

使用检查点，您可以：

- **撤销不需要的更改**，通过将文件恢复到已知的良好状态
- **探索替代方案**，通过恢复到检查点并尝试不同的方法
- **从错误中恢复**，当代理进行不正确的修改时

<Warning>
只有通过 Write、Edit 和 NotebookEdit 工具所做的更改才会被跟踪。通过 Bash 命令（如 `echo > file.txt` 或 `sed -i`）所做的更改不会被检查点系统捕获。
</Warning>

## 检查点的工作原理

启用文件检查点后，SDK 会在通过 Write、Edit 或 NotebookEdit 工具修改文件之前创建文件备份。响应流中的用户消息包含一个检查点 UUID，您可以将其用作恢复点。

检查点适用于代理用来修改文件的这些内置工具：

| 工具 | 描述 |
|------|-------------|
| Write | 创建新文件或用新内容覆盖现有文件 |
| Edit | 对现有文件的特定部分进行有针对性的编辑 |
| NotebookEdit | 修改 Jupyter 笔记本（`.ipynb` 文件）中的单元格 |

<Note>
文件回退将磁盘上的文件恢复到先前的状态。它不会回退对话本身。调用 `rewindFiles()`（TypeScript）或 `rewind_files()`（Python）后，对话历史和上下文保持不变。
</Note>

检查点系统跟踪：

- 会话期间创建的文件
- 会话期间修改的文件
- 修改文件的原始内容

当您回退到检查点时，创建的文件将被删除，修改的文件将恢复到该时间点的内容。

## 实现检查点

要使用文件检查点，请在选项中启用它，从响应流中捕获检查点 UUID，然后在需要恢复时调用 `rewindFiles()`（TypeScript）或 `rewind_files()`（Python）。

以下示例显示完整流程：启用检查点，从响应流中捕获检查点 UUID 和会话 ID，然后稍后恢复会话以回退文件。下面详细解释了每个步骤。

<CodeGroup>

```python Python
import asyncio
import os
from claude_agent_sdk import ClaudeSDKClient, ClaudeAgentOptions, UserMessage, ResultMessage

async def main():
    # Step 1: Enable checkpointing
    options = ClaudeAgentOptions(
        enable_file_checkpointing=True,
        permission_mode="acceptEdits",  # Auto-accept file edits without prompting
        extra_args={"replay-user-messages": None},  # Required to receive checkpoint UUIDs in the response stream
        env={**os.environ, "CLAUDE_CODE_ENABLE_SDK_FILE_CHECKPOINTING": "1"}
    )

    checkpoint_id = None
    session_id = None

    # Run the query and capture checkpoint UUID and session ID
    async with ClaudeSDKClient(options) as client:
        await client.query("Refactor the authentication module")

        # Step 2: Capture checkpoint UUID from the first user message
        async for message in client.receive_response():
            if isinstance(message, UserMessage) and message.uuid and not checkpoint_id:
                checkpoint_id = message.uuid
            if isinstance(message, ResultMessage) and not session_id:
                session_id = message.session_id

    # Step 3: Later, rewind by resuming the session with an empty prompt
    if checkpoint_id and session_id:
        async with ClaudeSDKClient(ClaudeAgentOptions(
            enable_file_checkpointing=True,
            resume=session_id
        )) as client:
            await client.query("")  # Empty prompt to open the connection
            async for message in client.receive_response():
                await client.rewind_files(checkpoint_id)
                break
        print(f"Rewound to checkpoint: {checkpoint_id}")

asyncio.run(main())
```

```typescript TypeScript
import { query } from "@anthropic-ai/claude-agent-sdk";

async function main() {
  // Step 1: Enable checkpointing
  const opts = {
    enableFileCheckpointing: true,
    permissionMode: "acceptEdits" as const,  // Auto-accept file edits without prompting
    extraArgs: { 'replay-user-messages': null },  // Required to receive checkpoint UUIDs in the response stream
    env: { ...process.env, CLAUDE_CODE_ENABLE_SDK_FILE_CHECKPOINTING: '1' }
  };

  const response = query({
    prompt: "Refactor the authentication module",
    options: opts
  });

  let checkpointId: string | undefined;
  let sessionId: string | undefined;

  // Step 2: Capture checkpoint UUID from the first user message
  for await (const message of response) {
    if (message.type === 'user' && message.uuid && !checkpointId) {
      checkpointId = message.uuid;
    }
    if ('session_id' in message && !sessionId) {
      sessionId = message.session_id;
    }
  }

  // Step 3: Later, rewind by resuming the session with an empty prompt
  if (checkpointId && sessionId) {
    const rewindQuery = query({
      prompt: "",  // Empty prompt to open the connection
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

<Step title="设置环境变量">

文件检查点需要 `CLAUDE_CODE_ENABLE_SDK_FILE_CHECKPOINTING` 环境变量。您可以在运行脚本之前通过命令行设置它，或直接在 SDK 选项中设置。

**选项 1：通过命令行设置**

```bash Python
export CLAUDE_CODE_ENABLE_SDK_FILE_CHECKPOINTING=1
```

**选项 2：在 SDK 选项中设置**

在配置 SDK 时通过 `env` 选项传递环境变量：

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

<Step title="启用检查点">

配置您的 SDK 选项以启用检查点并接收检查点 UUID：

| 选项 | Python | TypeScript | 描述 |
|--------|--------|------------|-------------|
| 启用检查点 | `enable_file_checkpointing=True` | `enableFileCheckpointing: true` | 跟踪文件更改以便回退 |
| 接收检查点 UUID | `extra_args={"replay-user-messages": None}` | `extraArgs: { 'replay-user-messages': null }` | 需要在流中获取用户消息 UUID |

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

<Step title="捕获检查点 UUID 和会话 ID">

设置 `replay-user-messages` 选项后（如上所示），响应流中的每条用户消息都有一个 UUID，用作检查点。

对于大多数用例，捕获第一条用户消息 UUID（`message.uuid`）；回退到它会将所有文件恢复到原始状态。要存储多个检查点并回退到中间状态，请参阅[多个恢复点](#multiple-restore-points)。

捕获会话 ID（`message.session_id`）是可选的；只有在您想稍后回退（在流完成后）时才需要它。如果您在仍处理消息时立即调用 `rewindFiles()`（如[检查点前的危险操作](#checkpoint-before-risky-operations)中的示例所做的那样），您可以跳过捕获会话 ID。

<CodeGroup>

```python Python
checkpoint_id = None
session_id = None

async for message in client.receive_response():
    # Update checkpoint on each user message (keeps the latest)
    if isinstance(message, UserMessage) and message.uuid:
        checkpoint_id = message.uuid
    # Capture session ID from the result message
    if isinstance(message, ResultMessage):
        session_id = message.session_id
```

```typescript TypeScript
let checkpointId: string | undefined;
let sessionId: string | undefined;

for await (const message of response) {
  // Update checkpoint on each user message (keeps the latest)
  if (message.type === 'user' && message.uuid) {
    checkpointId = message.uuid;
  }
  // Capture session ID from any message that has it
  if ('session_id' in message) {
    sessionId = message.session_id;
  }
}
```

</CodeGroup>

</Step>

<Step title="回退文件">

要在流完成后回退，请使用空提示恢复会话，并使用您的检查点 UUID 调用 `rewind_files()`（Python）或 `rewindFiles()`（TypeScript）。您也可以在流期间回退；有关该模式，请参阅[检查点前的危险操作](#checkpoint-before-risky-operations)。

<CodeGroup>

```python Python
async with ClaudeSDKClient(ClaudeAgentOptions(
    enable_file_checkpointing=True,
    resume=session_id
)) as client:
    await client.query("")  # Empty prompt to open the connection
    async for message in client.receive_response():
        await client.rewind_files(checkpoint_id)
        break
```

```typescript TypeScript
const rewindQuery = query({
  prompt: "",  // Empty prompt to open the connection
  options: { ...opts, resume: sessionId }
});

for await (const msg of rewindQuery) {
  await rewindQuery.rewindFiles(checkpointId);
  break;
}
```

</CodeGroup>

如果您捕获了会话 ID 和检查点 ID，您也可以从 CLI 回退：

```bash
claude --resume <session-id> --rewind-files <checkpoint-uuid>
```

</Step>

</Steps>

## 常见模式

这些模式显示了根据您的用例捕获和使用检查点 UUID 的不同方式。

### 检查点前的危险操作

此模式仅保留最新的检查点 UUID，在每个代理轮次之前更新它。如果处理过程中出现问题，您可以立即回退到最后的安全状态并跳出循环。

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
            # Update checkpoint before each agent turn starts
            # This overwrites the previous checkpoint. Only keep the latest
            if isinstance(message, UserMessage) and message.uuid:
                safe_checkpoint = message.uuid

            # Decide when to revert based on your own logic
            # For example: error detection, validation failure, or user input
            if your_revert_condition and safe_checkpoint:
                await client.rewind_files(safe_checkpoint)
                # Exit the loop after rewinding, files are restored
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
    // Update checkpoint before each agent turn starts
    // This overwrites the previous checkpoint. Only keep the latest
    if (message.type === 'user' && message.uuid) {
      safeCheckpoint = message.uuid;
    }

    // Decide when to revert based on your own logic
    // For example: error detection, validation failure, or user input
    if (yourRevertCondition && safeCheckpoint) {
      await response.rewindFiles(safeCheckpoint);
      // Exit the loop after rewinding, files are restored
      break;
    }
  }
}

main();
```

</CodeGroup>

### 多个恢复点

如果 Claude 在多个轮次中进行更改，您可能想回退到特定点而不是一直回退。例如，如果 Claude 在第一轮重构文件，在第二轮添加测试，您可能想保留重构但撤销测试。

此模式将所有检查点 UUID 存储在带有元数据的数组中。会话完成后，您可以回退到任何先前的检查点：

<CodeGroup>

```python Python
import asyncio
import os
from dataclasses import dataclass
from datetime import datetime
from claude_agent_sdk import ClaudeSDKClient, ClaudeAgentOptions, UserMessage, ResultMessage

# Store checkpoint metadata for better tracking
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

    # Later: rewind to any checkpoint by resuming the session
    if checkpoints and session_id:
        target = checkpoints[0]  # Pick any checkpoint
        async with ClaudeSDKClient(ClaudeAgentOptions(
            enable_file_checkpointing=True,
            resume=session_id
        )) as client:
            await client.query("")  # Empty prompt to open the connection
            async for message in client.receive_response():
                await client.rewind_files(target.id)
                break
        print(f"Rewound to: {target.description}")

asyncio.run(main())
```

```typescript TypeScript
import { query } from "@anthropic-ai/claude-agent-sdk";

// Store checkpoint metadata for better tracking
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

  // Later: rewind to any checkpoint by resuming the session
  if (checkpoints.length > 0 && sessionId) {
    const target = checkpoints[0];  // Pick any checkpoint
    const rewindQuery = query({
      prompt: "",  // Empty prompt to open the connection
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

## 尝试一下

这个完整的示例创建一个小实用程序文件，让代理添加文档注释，显示更改，然后询问您是否想回退。

在开始之前，请确保您已[安装 Claude Agent SDK](/docs/zh-CN/agent-sdk/quickstart)。

<Steps>

<Step title="创建测试文件">

创建一个名为 `utils.py`（Python）或 `utils.ts`（TypeScript）的新文件，并粘贴以下代码：

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

<Step title="运行交互式示例">

在与实用程序文件相同的目录中创建一个名为 `try_checkpointing.py`（Python）或 `try_checkpointing.ts`（TypeScript）的新文件，并粘贴以下代码。

此脚本要求 Claude 向您的实用程序文件添加文档注释，然后为您提供回退和恢复原始文件的选项。

<CodeGroup>

```python try_checkpointing.py
import asyncio
from claude_agent_sdk import ClaudeSDKClient, ClaudeAgentOptions, UserMessage, ResultMessage

async def main():
    # Configure the SDK with checkpointing enabled
    # - enable_file_checkpointing: Track file changes for rewinding
    # - permission_mode: Auto-accept file edits without prompting
    # - extra_args: Required to receive user message UUIDs in the stream
    options = ClaudeAgentOptions(
        enable_file_checkpointing=True,
        permission_mode="acceptEdits",
        extra_args={"replay-user-messages": None}
    )

    checkpoint_id = None  # Store the user message UUID for rewinding
    session_id = None     # Store the session ID for resuming

    print("Running agent to add doc comments to utils.py...\n")

    # Run the agent and capture checkpoint data from the response stream
    async with ClaudeSDKClient(options) as client:
        await client.query("Add doc comments to utils.py")

        async for message in client.receive_response():
            # Capture the first user message UUID - this is our restore point
            if isinstance(message, UserMessage) and message.uuid and not checkpoint_id:
                checkpoint_id = message.uuid
            # Capture the session ID so we can resume later
            if isinstance(message, ResultMessage):
                session_id = message.session_id

    print("Done! Open utils.py to see the added doc comments.\n")

    # Ask the user if they want to rewind the changes
    if checkpoint_id and session_id:
        response = input("Rewind to remove the doc comments? (y/n): ")

        if response.lower() == "y":
            # Resume the session with an empty prompt, then rewind
            async with ClaudeSDKClient(ClaudeAgentOptions(
                enable_file_checkpointing=True,
                resume=session_id
            )) as client:
                await client.query("")  # Empty prompt opens the connection
                async for message in client.receive_response():
                    await client.rewind_files(checkpoint_id)  # Restore files
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
  // Configure the SDK with checkpointing enabled
  // - enableFileCheckpointing: Track file changes for rewinding
  // - permissionMode: Auto-accept file edits without prompting
  // - extraArgs: Required to receive user message UUIDs in the stream
  const opts = {
    enableFileCheckpointing: true,
    permissionMode: "acceptEdits" as const,
    extraArgs: { 'replay-user-messages': null }
  };

  let sessionId: string | undefined;    // Store the session ID for resuming
  let checkpointId: string | undefined; // Store the user message UUID for rewinding

  console.log("Running agent to add doc comments to utils.ts...\n");

  // Run the agent and capture checkpoint data from the response stream
  const response = query({
    prompt: "Add doc comments to utils.ts",
    options: opts
  });

  for await (const message of response) {
    // Capture the first user message UUID - this is our restore point
    if (message.type === "user" && message.uuid && !checkpointId) {
      checkpointId = message.uuid;
    }
    // Capture the session ID so we can resume later
    if ("session_id" in message) {
      sessionId = message.session_id;
    }
  }

  console.log("Done! Open utils.ts to see the added doc comments.\n");

  // Ask the user if they want to rewind the changes
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
      // Resume the session with an empty prompt, then rewind
      const rewindQuery = query({
        prompt: "",  // Empty prompt opens the connection
        options: { ...opts, resume: sessionId }
      });

      for await (const msg of rewindQuery) {
        await rewindQuery.rewindFiles(checkpointId);  // Restore files
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

此示例演示了完整的检查点工作流程：

1. **启用检查点**：使用 `enable_file_checkpointing=True` 和 `permission_mode="acceptEdits"` 配置 SDK 以自动批准文件编辑
2. **捕获检查点数据**：当代理运行时，存储第一条用户消息 UUID（您的恢复点）和会话 ID
3. **提示回退**：代理完成后，检查您的实用程序文件以查看文档注释，然后决定是否要撤销更改
4. **恢复和回退**：如果是，使用空提示恢复会话并调用 `rewind_files()` 以恢复原始文件

</Step>

<Step title="运行示例">

设置环境变量并从与实用程序文件相同的目录运行脚本。

<Tip>
在运行脚本之前，在您的 IDE 或编辑器中打开您的实用程序文件（`utils.py` 或 `utils.ts`）。当代理添加文档注释时，您将看到文件实时更新，然后当您选择回退时恢复到原始状态。
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

您将看到代理添加文档注释，然后出现一个提示询问您是否想回退。如果您选择是，文件将恢复到其原始状态。

</Step>

</Steps>

## 限制

文件检查点具有以下限制：

| 限制 | 描述 |
|------------|-------------|
| 仅 Write/Edit/NotebookEdit 工具 | 通过 Bash 命令所做的更改不被跟踪 |
| 相同会话 | 检查点与创建它们的会话相关联 |
| 仅文件内容 | 创建、移动或删除目录不会通过回退撤销 |
| 本地文件 | 远程或网络文件不被跟踪 |

## 故障排除

### 检查点选项未被识别

如果 `enableFileCheckpointing` 或 `rewindFiles()` 不可用，您可能使用的是较旧的 SDK 版本。

**解决方案**：更新到最新的 SDK 版本：
- **Python**：`pip install --upgrade claude-agent-sdk`
- **TypeScript**：`npm install @anthropic-ai/claude-agent-sdk@latest`

### 用户消息没有 UUID

如果 `message.uuid` 是 `undefined` 或缺失，您没有接收检查点 UUID。

**原因**：未设置 `replay-user-messages` 选项。

**解决方案**：将 `extra_args={"replay-user-messages": None}`（Python）或 `extraArgs: { 'replay-user-messages': null }`（TypeScript）添加到您的选项中。

### "No file checkpoint found for message" 错误

当指定的用户消息 UUID 的检查点数据不存在时，会发生此错误。

**常见原因**：
- 未设置 `CLAUDE_CODE_ENABLE_SDK_FILE_CHECKPOINTING` 环境变量
- 在尝试恢复和回退之前，会话未正确完成

**解决方案**：确保您已设置环境变量（请参阅[设置环境变量](#set-the-environment-variable)），然后使用示例中显示的模式：捕获第一条用户消息 UUID，完全完成会话，然后使用空提示恢复并调用 `rewindFiles()` 一次。

### "ProcessTransport is not ready for writing" 错误

当您在完成响应迭代后调用 `rewindFiles()` 或 `rewind_files()` 时，会发生此错误。当循环完成时，与 CLI 进程的连接关闭。

**解决方案**：使用空提示恢复会话，然后在新查询上调用回退：

<CodeGroup>

```python Python
# Resume session with empty prompt, then rewind
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
// Resume session with empty prompt, then rewind
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

## 后续步骤

- **[会话](/docs/zh-CN/agent-sdk/sessions)**：了解如何恢复会话，这是在流完成后回退所必需的。涵盖会话 ID、恢复对话和会话分叉。
- **[权限](/docs/zh-CN/agent-sdk/permissions)**：配置 Claude 可以使用哪些工具以及如何批准文件修改。如果您想更好地控制何时进行编辑，这很有用。
- **[TypeScript SDK 参考](/docs/zh-CN/agent-sdk/typescript)**：完整的 API 参考，包括 `query()` 的所有选项和 `rewindFiles()` 方法。
- **[Python SDK 参考](/docs/zh-CN/agent-sdk/python)**：完整的 API 参考，包括 `ClaudeAgentOptions` 的所有选项和 `rewind_files()` 方法。