# SDK 中的斜線命令

學習如何使用斜線命令透過 SDK 控制 Claude Code 會話

---

斜線命令提供了一種使用以 `/` 開頭的特殊命令來控制 Claude Code 會話的方法。這些命令可以透過 SDK 發送，以執行清除對話歷史、壓縮訊息或獲取幫助等操作。

## 發現可用的斜線命令

Claude Agent SDK 在系統初始化訊息中提供有關可用斜線命令的資訊。在會話開始時存取此資訊：

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

## 發送斜線命令

透過在提示字串中包含斜線命令來發送它們，就像普通文字一樣：

<CodeGroup>

```typescript TypeScript
import { query } from "@anthropic-ai/claude-agent-sdk";

// Send a slash command
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
    # Send a slash command
    async for message in query(
        prompt="/compact",
        options={"max_turns": 1}
    ):
        if message.type == "result":
            print("Command executed:", message.result)

asyncio.run(main())
```

</CodeGroup>

## 常見的斜線命令

### `/compact` - 壓縮對話歷史

`/compact` 命令透過總結較舊的訊息同時保留重要上下文來減少對話歷史的大小：

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

### `/clear` - 清除對話

`/clear` 命令透過清除所有先前的歷史來開始新的對話：

<CodeGroup>

```typescript TypeScript
import { query } from "@anthropic-ai/claude-agent-sdk";

// Clear conversation and start fresh
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
    # Clear conversation and start fresh
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

## 建立自訂斜線命令

除了使用內建的斜線命令外，您還可以建立自己的自訂命令，這些命令可透過 SDK 使用。自訂命令定義為特定目錄中的 markdown 檔案，類似於子代理的配置方式。

### 檔案位置

自訂斜線命令根據其範圍儲存在指定的目錄中：

- **專案命令**：`.claude/commands/` - 僅在當前專案中可用
- **個人命令**：`~/.claude/commands/` - 在您的所有專案中可用

### 檔案格式

每個自訂命令都是一個 markdown 檔案，其中：
- 檔案名稱（不含 `.md` 副檔名）成為命令名稱
- 檔案內容定義命令的功能
- 可選的 YAML 前置資料提供配置

#### 基本範例

建立 `.claude/commands/refactor.md`：

```markdown
Refactor the selected code to improve readability and maintainability.
Focus on clean code principles and best practices.
```

這會建立 `/refactor` 命令，您可以透過 SDK 使用。

#### 使用前置資料

建立 `.claude/commands/security-check.md`：

```markdown
---
allowed-tools: Read, Grep, Glob
description: Run security vulnerability scan
model: claude-3-5-sonnet-20241022
---

Analyze the codebase for security vulnerabilities including:
- SQL injection risks
- XSS vulnerabilities
- Exposed credentials
- Insecure configurations
```

### 在 SDK 中使用自訂命令

一旦在檔案系統中定義，自訂命令就會自動透過 SDK 可用：

<CodeGroup>

```typescript TypeScript
import { query } from "@anthropic-ai/claude-agent-sdk";

// Use a custom command
for await (const message of query({
  prompt: "/refactor src/auth/login.ts",
  options: { maxTurns: 3 }
})) {
  if (message.type === "assistant") {
    console.log("Refactoring suggestions:", message.message);
  }
}

// Custom commands appear in the slash_commands list
for await (const message of query({
  prompt: "Hello",
  options: { maxTurns: 1 }
})) {
  if (message.type === "system" && message.subtype === "init") {
    // Will include both built-in and custom commands
    console.log("Available commands:", message.slash_commands);
    // Example: ["/compact", "/clear", "/help", "/refactor", "/security-check"]
  }
}
```

```python Python
import asyncio
from claude_agent_sdk import query

async def main():
    # Use a custom command
    async for message in query(
        prompt="/refactor src/auth/login.py",
        options={"max_turns": 3}
    ):
        if message.type == "assistant":
            print("Refactoring suggestions:", message.message)
    
    # Custom commands appear in the slash_commands list
    async for message in query(
        prompt="Hello",
        options={"max_turns": 1}
    ):
        if message.type == "system" and message.subtype == "init":
            # Will include both built-in and custom commands
            print("Available commands:", message.slash_commands)
            # Example: ["/compact", "/clear", "/help", "/refactor", "/security-check"]

asyncio.run(main())
```

</CodeGroup>

### 進階功能

#### 參數和佔位符

自訂命令支援使用佔位符的動態參數：

建立 `.claude/commands/fix-issue.md`：

```markdown
---
argument-hint: [issue-number] [priority]
description: Fix a GitHub issue
---

Fix issue #$1 with priority $2.
Check the issue description and implement the necessary changes.
```

在 SDK 中使用：

<CodeGroup>

```typescript TypeScript
import { query } from "@anthropic-ai/claude-agent-sdk";

// Pass arguments to custom command
for await (const message of query({
  prompt: "/fix-issue 123 high",
  options: { maxTurns: 5 }
})) {
  // Command will process with $1="123" and $2="high"
  if (message.type === "result") {
    console.log("Issue fixed:", message.result);
  }
}
```

```python Python
import asyncio
from claude_agent_sdk import query

async def main():
    # Pass arguments to custom command
    async for message in query(
        prompt="/fix-issue 123 high",
        options={"max_turns": 5}
    ):
        # Command will process with $1="123" and $2="high"
        if message.type == "result":
            print("Issue fixed:", message.result)

asyncio.run(main())
```

</CodeGroup>

#### Bash 命令執行

自訂命令可以執行 bash 命令並包含其輸出：

建立 `.claude/commands/git-commit.md`：

```markdown
---
allowed-tools: Bash(git add:*), Bash(git status:*), Bash(git commit:*)
description: Create a git commit
---

## Context

- Current status: !`git status`
- Current diff: !`git diff HEAD`

## Task

Create a git commit with appropriate message based on the changes.
```

#### 檔案參考

使用 `@` 前綴包含檔案內容：

建立 `.claude/commands/review-config.md`：

```markdown
---
description: Review configuration files
---

Review the following configuration files for issues:
- Package config: @package.json
- TypeScript config: @tsconfig.json
- Environment config: @.env

Check for security issues, outdated dependencies, and misconfigurations.
```

### 使用命名空間進行組織

在子目錄中組織命令以獲得更好的結構：

```bash
.claude/commands/
├── frontend/
│   ├── component.md      # Creates /component (project:frontend)
│   └── style-check.md     # Creates /style-check (project:frontend)
├── backend/
│   ├── api-test.md        # Creates /api-test (project:backend)
│   └── db-migrate.md      # Creates /db-migrate (project:backend)
└── review.md              # Creates /review (project)
```

子目錄出現在命令描述中，但不會影響命令名稱本身。

### 實用範例

#### 程式碼審查命令

建立 `.claude/commands/code-review.md`：

```markdown
---
allowed-tools: Read, Grep, Glob, Bash(git diff:*)
description: Comprehensive code review
---

## Changed Files
!`git diff --name-only HEAD~1`

## Detailed Changes
!`git diff HEAD~1`

## Review Checklist

Review the above changes for:
1. Code quality and readability
2. Security vulnerabilities
3. Performance implications
4. Test coverage
5. Documentation completeness

Provide specific, actionable feedback organized by priority.
```

#### 測試執行器命令

建立 `.claude/commands/test.md`：

```markdown
---
allowed-tools: Bash, Read, Edit
argument-hint: [test-pattern]
description: Run tests with optional pattern
---

Run tests matching pattern: $ARGUMENTS

1. Detect the test framework (Jest, pytest, etc.)
2. Run tests with the provided pattern
3. If tests fail, analyze and fix them
4. Re-run to verify fixes
```

透過 SDK 使用這些命令：

<CodeGroup>

```typescript TypeScript
import { query } from "@anthropic-ai/claude-agent-sdk";

// Run code review
for await (const message of query({
  prompt: "/code-review",
  options: { maxTurns: 3 }
})) {
  // Process review feedback
}

// Run specific tests
for await (const message of query({
  prompt: "/test auth",
  options: { maxTurns: 5 }
})) {
  // Handle test results
}
```

```python Python
import asyncio
from claude_agent_sdk import query

async def main():
    # Run code review
    async for message in query(
        prompt="/code-review",
        options={"max_turns": 3}
    ):
        # Process review feedback
        pass
    
    # Run specific tests
    async for message in query(
        prompt="/test auth",
        options={"max_turns": 5}
    ):
        # Handle test results
        pass

asyncio.run(main())
```

</CodeGroup>

## 另請參閱

- [斜線命令](https://code.claude.com/docs/slash-commands) - 完整的斜線命令文件
- [SDK 中的子代理](/docs/zh-TW/agent-sdk/subagents) - 類似的基於檔案系統的子代理配置
- [TypeScript SDK 參考](https://code.claude.com/docs/typescript-sdk-reference) - 完整的 API 文件
- [SDK 概述](/docs/zh-TW/agent-sdk/overview) - 一般 SDK 概念
- [CLI 參考](https://code.claude.com/docs/cli-reference) - 命令列介面