# 使用檢查點重新開始檔案變更

追蹤代理程式工作階段期間的檔案變更，並將檔案還原到任何先前的狀態

---

檔案檢查點追蹤在代理程式工作階段期間透過 Write、Edit 和 NotebookEdit 工具進行的檔案修改，允許您將檔案還原到任何先前的狀態。想要試試看嗎？跳到[互動範例](#try-it-out)。

使用檢查點，您可以：

- **撤銷不需要的變更**，透過將檔案還原到已知的良好狀態
- **探索替代方案**，透過還原到檢查點並嘗試不同的方法
- **從錯誤中恢復**，當代理程式進行不正確的修改時

<Warning>
只有透過 Write、Edit 和 NotebookEdit 工具進行的變更才會被追蹤。透過 Bash 命令進行的變更（如 `echo > file.txt` 或 `sed -i`）不會被檢查點系統捕獲。
</Warning>

## 檢查點的運作方式

當您啟用檔案檢查點時，SDK 會在透過 Write、Edit 或 NotebookEdit 工具修改檔案之前建立檔案備份。回應串流中的使用者訊息包含一個檢查點 UUID，您可以將其用作還原點。

檢查點適用於代理程式用來修改檔案的這些內建工具：

| 工具 | 描述 |
|------|-------------|
| Write | 建立新檔案或用新內容覆寫現有檔案 |
| Edit | 對現有檔案的特定部分進行有針對性的編輯 |
| NotebookEdit | 修改 Jupyter 筆記本（`.ipynb` 檔案）中的儲存格 |

<Note>
檔案回退將磁碟上的檔案還原到先前的狀態。它不會回退對話本身。呼叫 `rewindFiles()`（TypeScript）或 `rewind_files()`（Python）後，對話歷史記錄和上下文保持不變。
</Note>

檢查點系統追蹤：

- 在工作階段期間建立的檔案
- 在工作階段期間修改的檔案
- 修改檔案的原始內容

當您還原到檢查點時，建立的檔案會被刪除，修改的檔案會還原到該時間點的內容。

## 實現檢查點

要使用檔案檢查點，請在您的選項中啟用它，從回應串流中捕獲檢查點 UUID，然後在需要還原時呼叫 `rewindFiles()`（TypeScript）或 `rewind_files()`（Python）。

以下範例顯示完整流程：啟用檢查點、從回應串流中捕獲檢查點 UUID 和工作階段 ID，然後稍後恢復工作階段以回退檔案。下面詳細說明每個步驟。

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

<Step title="設定環境變數">

檔案檢查點需要 `CLAUDE_CODE_ENABLE_SDK_FILE_CHECKPOINTING` 環境變數。您可以在執行指令碼之前透過命令列設定它，或直接在 SDK 選項中設定。

**選項 1：透過命令列設定**

```bash Python
export CLAUDE_CODE_ENABLE_SDK_FILE_CHECKPOINTING=1
```

**選項 2：在 SDK 選項中設定**

在設定 SDK 時透過 `env` 選項傳遞環境變數：

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

<Step title="啟用檢查點">

設定您的 SDK 選項以啟用檢查點並接收檢查點 UUID：

| 選項 | Python | TypeScript | 描述 |
|--------|--------|------------|-------------|
| 啟用檢查點 | `enable_file_checkpointing=True` | `enableFileCheckpointing: true` | 追蹤檔案變更以便回退 |
| 接收檢查點 UUID | `extra_args={"replay-user-messages": None}` | `extraArgs: { 'replay-user-messages': null }` | 需要在串流中取得使用者訊息 UUID |

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

<Step title="捕獲檢查點 UUID 和工作階段 ID">

設定 `replay-user-messages` 選項後（如上所示），回應串流中的每個使用者訊息都有一個 UUID，用作檢查點。

對於大多數使用情況，捕獲第一個使用者訊息 UUID（`message.uuid`）；還原到它會將所有檔案還原到其原始狀態。要儲存多個檢查點並還原到中間狀態，請參閱[多個還原點](#multiple-restore-points)。

捕獲工作階段 ID（`message.session_id`）是可選的；只有在您想在串流完成後稍後回退時才需要它。如果您在仍在處理訊息時立即呼叫 `rewindFiles()`（如[檢查點在風險操作之前](#checkpoint-before-risky-operations)中的範例所做的那樣），您可以跳過捕獲工作階段 ID。

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

<Step title="回退檔案">

要在串流完成後回退，請使用空提示恢復工作階段，並使用您的檢查點 UUID 呼叫 `rewind_files()`（Python）或 `rewindFiles()`（TypeScript）。您也可以在串流期間回退；請參閱[檢查點在風險操作之前](#checkpoint-before-risky-operations)以了解該模式。

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

如果您捕獲了工作階段 ID 和檢查點 ID，您也可以從 CLI 回退：

```bash
claude --resume <session-id> --rewind-files <checkpoint-uuid>
```

</Step>

</Steps>

## 常見模式

這些模式顯示根據您的使用情況捕獲和使用檢查點 UUID 的不同方式。

### 檢查點在風險操作之前

此模式僅保留最新的檢查點 UUID，在每個代理程式轉向之前更新它。如果在處理過程中出現問題，您可以立即回退到最後的安全狀態並跳出迴圈。

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

### 多個還原點

如果 Claude 在多個轉向中進行變更，您可能想要還原到特定點而不是一直回到最開始。例如，如果 Claude 在第一個轉向中重構檔案，在第二個轉向中新增測試，您可能想要保留重構但撤銷測試。

此模式將所有檢查點 UUID 與中繼資料一起儲存在陣列中。工作階段完成後，您可以還原到任何先前的檢查點：

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

## 試試看

此完整範例建立一個小型公用程式檔案，讓代理程式新增文件註解，向您顯示變更，然後詢問您是否要回退。

在開始之前，請確保您已[安裝 Claude Agent SDK](/docs/zh-TW/agent-sdk/quickstart)。

<Steps>

<Step title="建立測試檔案">

建立一個名為 `utils.py`（Python）或 `utils.ts`（TypeScript）的新檔案，並貼上以下程式碼：

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

<Step title="執行互動範例">

在與您的公用程式檔案相同的目錄中建立一個名為 `try_checkpointing.py`（Python）或 `try_checkpointing.ts`（TypeScript）的新檔案，並貼上以下程式碼。

此指令碼要求 Claude 將文件註解新增到您的公用程式檔案，然後給您選擇回退並還原原始檔案的選項。

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

此範例演示完整的檢查點工作流程：

1. **啟用檢查點**：使用 `enable_file_checkpointing=True` 和 `permission_mode="acceptEdits"` 設定 SDK 以自動批准檔案編輯
2. **捕獲檢查點資料**：當代理程式執行時，儲存第一個使用者訊息 UUID（您的還原點）和工作階段 ID
3. **提示回退**：代理程式完成後，檢查您的公用程式檔案以查看文件註解，然後決定是否要撤銷變更
4. **恢復並回退**：如果是，使用空提示恢復工作階段並呼叫 `rewind_files()` 以還原原始檔案

</Step>

<Step title="執行範例">

設定環境變數並從與您的公用程式檔案相同的目錄執行指令碼。

<Tip>
在執行指令碼之前，在您的 IDE 或編輯器中開啟您的公用程式檔案（`utils.py` 或 `utils.ts`）。當代理程式新增文件註解時，您會看到檔案實時更新，然後當您選擇回退時還原到原始檔案。
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

您會看到代理程式新增文件註解，然後出現一個提示，詢問您是否要回退。如果您選擇是，檔案會還原到其原始狀態。

</Step>

</Steps>

## 限制

檔案檢查點有以下限制：

| 限制 | 描述 |
|------------|-------------|
| 僅限 Write/Edit/NotebookEdit 工具 | 透過 Bash 命令進行的變更不會被追蹤 |
| 相同工作階段 | 檢查點與建立它們的工作階段相關聯 |
| 僅限檔案內容 | 建立、移動或刪除目錄不會透過回退來撤銷 |
| 本機檔案 | 遠端或網路檔案不會被追蹤 |

## 故障排除

### 檢查點選項未被識別

如果 `enableFileCheckpointing` 或 `rewindFiles()` 不可用，您可能使用的是較舊的 SDK 版本。

**解決方案**：更新到最新的 SDK 版本：
- **Python**：`pip install --upgrade claude-agent-sdk`
- **TypeScript**：`npm install @anthropic-ai/claude-agent-sdk@latest`

### 使用者訊息沒有 UUID

如果 `message.uuid` 是 `undefined` 或遺失，您沒有接收檢查點 UUID。

**原因**：未設定 `replay-user-messages` 選項。

**解決方案**：將 `extra_args={"replay-user-messages": None}`（Python）或 `extraArgs: { 'replay-user-messages': null }`（TypeScript）新增到您的選項。

### "No file checkpoint found for message" 錯誤

當指定的使用者訊息 UUID 的檢查點資料不存在時，會發生此錯誤。

**常見原因**：
- 未設定 `CLAUDE_CODE_ENABLE_SDK_FILE_CHECKPOINTING` 環境變數
- 在嘗試恢復和回退之前，工作階段未正確完成

**解決方案**：確保您已設定環境變數（請參閱[設定環境變數](#set-the-environment-variable)），然後使用範例中顯示的模式：捕獲第一個使用者訊息 UUID，完全完成工作階段，然後使用空提示恢復並呼叫 `rewindFiles()` 一次。

### "ProcessTransport is not ready for writing" 錯誤

當您在完成回應迭代後呼叫 `rewindFiles()` 或 `rewind_files()` 時，會發生此錯誤。當迴圈完成時，與 CLI 程序的連線會關閉。

**解決方案**：使用空提示恢復工作階段，然後在新查詢上呼叫回退：

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

## 後續步驟

- **[工作階段](/docs/zh-TW/agent-sdk/sessions)**：了解如何恢復工作階段，這是在串流完成後回退所需的。涵蓋工作階段 ID、恢復對話和工作階段分叉。
- **[權限](/docs/zh-TW/agent-sdk/permissions)**：設定 Claude 可以使用哪些工具以及如何批准檔案修改。如果您想更好地控制何時進行編輯，這很有用。
- **[TypeScript SDK 參考](/docs/zh-TW/agent-sdk/typescript)**：完整的 API 參考，包括 `query()` 的所有選項和 `rewindFiles()` 方法。
- **[Python SDK 參考](/docs/zh-TW/agent-sdk/python)**：完整的 API 參考，包括 `ClaudeAgentOptions` 的所有選項和 `rewind_files()` 方法。