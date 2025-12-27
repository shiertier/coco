# SDK 中的代理技能

使用 Claude 代理 SDK 中的代理技能擴展 Claude 的專業功能

---

## 概述

代理技能使用專業功能擴展 Claude，Claude 會在相關時自動調用這些技能。技能被打包為 `SKILL.md` 文件，包含說明、描述和可選的支持資源。

有關技能的全面信息，包括優勢、架構和編寫指南，請參閱[代理技能概述](/docs/zh-TW/agents-and-tools/agent-skills/overview)。

## 技能如何與 SDK 配合使用

使用 Claude 代理 SDK 時，技能具有以下特點：

1. **定義為文件系統工件**：在特定目錄（`.claude/skills/`）中創建為 `SKILL.md` 文件
2. **從文件系統加載**：技能從配置的文件系統位置加載。您必須指定 `settingSources`（TypeScript）或 `setting_sources`（Python）以從文件系統加載技能
3. **自動發現**：加載文件系統設置後，技能元數據在啟動時從用戶和項目目錄中發現；觸發時加載完整內容
4. **由模型調用**：Claude 根據上下文自主選擇何時使用它們
5. **通過 allowed_tools 啟用**：將 `"Skill"` 添加到您的 `allowed_tools` 以啟用技能

與子代理（可以以編程方式定義）不同，技能必須創建為文件系統工件。SDK 不提供用於註冊技能的編程 API。

<Note>
**默認行為**：默認情況下，SDK 不加載任何文件系統設置。要使用技能，您必須在選項中明確配置 `settingSources: ['user', 'project']`（TypeScript）或 `setting_sources=["user", "project"]`（Python）。
</Note>

## 在 SDK 中使用技能

要在 SDK 中使用技能，您需要：

1. 在 `allowed_tools` 配置中包含 `"Skill"`
2. 配置 `settingSources`/`setting_sources` 以從文件系統加載技能

配置完成後，Claude 會自動從指定目錄發現技能，並在與用戶請求相關時調用它們。

<CodeGroup>

```python Python
import asyncio
from claude_agent_sdk import query, ClaudeAgentOptions

async def main():
    options = ClaudeAgentOptions(
        cwd="/path/to/project",  # Project with .claude/skills/
        setting_sources=["user", "project"],  # Load Skills from filesystem
        allowed_tools=["Skill", "Read", "Write", "Bash"]  # Enable Skill tool
    )

    async for message in query(
        prompt="Help me process this PDF document",
        options=options
    ):
        print(message)

asyncio.run(main())
```

```typescript TypeScript
import { query } from "@anthropic-ai/claude-agent-sdk";

for await (const message of query({
  prompt: "Help me process this PDF document",
  options: {
    cwd: "/path/to/project",  // Project with .claude/skills/
    settingSources: ["user", "project"],  // Load Skills from filesystem
    allowedTools: ["Skill", "Read", "Write", "Bash"]  // Enable Skill tool
  }
})) {
  console.log(message);
}
```

</CodeGroup>

## 技能位置

根據您的 `settingSources`/`setting_sources` 配置，技能從文件系統目錄加載：

- **項目技能**（`.claude/skills/`）：通過 git 與您的團隊共享 - 當 `setting_sources` 包含 `"project"` 時加載
- **用戶技能**（`~/.claude/skills/`）：跨所有項目的個人技能 - 當 `setting_sources` 包含 `"user"` 時加載
- **插件技能**：與已安裝的 Claude Code 插件捆綁

## 創建技能

技能定義為包含具有 YAML 前置事項和 Markdown 內容的 `SKILL.md` 文件的目錄。`description` 字段確定 Claude 何時調用您的技能。

**示例目錄結構**：
```bash
.claude/skills/processing-pdfs/
└── SKILL.md
```

有關創建技能的完整指導，包括 SKILL.md 結構、多文件技能和示例，請參閱：
- [Claude Code 中的代理技能](https://code.claude.com/docs/skills)：包含示例的完整指南
- [代理技能最佳實踐](/docs/zh-TW/agents-and-tools/agent-skills/best-practices)：編寫指南和命名約定

## 工具限制

<Note>
SKILL.md 中的 `allowed-tools` 前置事項字段僅在直接使用 Claude Code CLI 時受支持。**通過 SDK 使用技能時不適用**。

使用 SDK 時，通過查詢配置中的主 `allowedTools` 選項控制工具訪問。
</Note>

要在 SDK 應用程序中限制技能的工具，請使用 `allowedTools` 選項：

<Note>
假設第一個示例中的導入語句在以下代碼片段中。
</Note>

<CodeGroup>

```python Python
options = ClaudeAgentOptions(
    setting_sources=["user", "project"],  # Load Skills from filesystem
    allowed_tools=["Skill", "Read", "Grep", "Glob"]  # Restricted toolset
)

async for message in query(
    prompt="Analyze the codebase structure",
    options=options
):
    print(message)
```

```typescript TypeScript
// Skills can only use Read, Grep, and Glob tools
for await (const message of query({
  prompt: "Analyze the codebase structure",
  options: {
    settingSources: ["user", "project"],  // Load Skills from filesystem
    allowedTools: ["Skill", "Read", "Grep", "Glob"]  // Restricted toolset
  }
})) {
  console.log(message);
}
```

</CodeGroup>

## 發現可用技能

要查看 SDK 應用程序中可用的技能，只需詢問 Claude：

<CodeGroup>

```python Python
options = ClaudeAgentOptions(
    setting_sources=["user", "project"],  # Load Skills from filesystem
    allowed_tools=["Skill"]
)

async for message in query(
    prompt="What Skills are available?",
    options=options
):
    print(message)
```

```typescript TypeScript
for await (const message of query({
  prompt: "What Skills are available?",
  options: {
    settingSources: ["user", "project"],  // Load Skills from filesystem
    allowedTools: ["Skill"]
  }
})) {
  console.log(message);
}
```

</CodeGroup>

Claude 將根據您當前的工作目錄和已安裝的插件列出可用的技能。

## 測試技能

通過提出與技能描述相匹配的問題來測試技能：

<CodeGroup>

```python Python
options = ClaudeAgentOptions(
    cwd="/path/to/project",
    setting_sources=["user", "project"],  # Load Skills from filesystem
    allowed_tools=["Skill", "Read", "Bash"]
)

async for message in query(
    prompt="Extract text from invoice.pdf",
    options=options
):
    print(message)
```

```typescript TypeScript
for await (const message of query({
  prompt: "Extract text from invoice.pdf",
  options: {
    cwd: "/path/to/project",
    settingSources: ["user", "project"],  // Load Skills from filesystem
    allowedTools: ["Skill", "Read", "Bash"]
  }
})) {
  console.log(message);
}
```

</CodeGroup>

如果描述與您的請求相匹配，Claude 會自動調用相關技能。

## 故障排除

### 找不到技能

**檢查 settingSources 配置**：只有在明確配置 `settingSources`/`setting_sources` 時才會加載技能。這是最常見的問題：

<CodeGroup>

```python Python
# Wrong - Skills won't be loaded
options = ClaudeAgentOptions(
    allowed_tools=["Skill"]
)

# Correct - Skills will be loaded
options = ClaudeAgentOptions(
    setting_sources=["user", "project"],  # Required to load Skills
    allowed_tools=["Skill"]
)
```

```typescript TypeScript
// Wrong - Skills won't be loaded
const options = {
  allowedTools: ["Skill"]
};

// Correct - Skills will be loaded
const options = {
  settingSources: ["user", "project"],  // Required to load Skills
  allowedTools: ["Skill"]
};
```

</CodeGroup>

有關 `settingSources`/`setting_sources` 的更多詳細信息，請參閱 [TypeScript SDK 參考](/docs/zh-TW/agent-sdk/typescript#settingsource)或 [Python SDK 參考](/docs/zh-TW/agent-sdk/python#settingsource)。

**檢查工作目錄**：SDK 相對於 `cwd` 選項加載技能。確保它指向包含 `.claude/skills/` 的目錄：

<CodeGroup>

```python Python
# Ensure your cwd points to the directory containing .claude/skills/
options = ClaudeAgentOptions(
    cwd="/path/to/project",  # Must contain .claude/skills/
    setting_sources=["user", "project"],  # Required to load Skills
    allowed_tools=["Skill"]
)
```

```typescript TypeScript
// Ensure your cwd points to the directory containing .claude/skills/
const options = {
  cwd: "/path/to/project",  // Must contain .claude/skills/
  settingSources: ["user", "project"],  // Required to load Skills
  allowedTools: ["Skill"]
};
```

</CodeGroup>

有關完整模式，請參閱上面的「在 SDK 中使用技能」部分。

**驗證文件系統位置**：
```bash
# Check project Skills
ls .claude/skills/*/SKILL.md

# Check personal Skills
ls ~/.claude/skills/*/SKILL.md
```

### 技能未被使用

**檢查技能工具是否已啟用**：確認 `"Skill"` 在您的 `allowedTools` 中。

**檢查描述**：確保它具體且包含相關關鍵字。有關編寫有效描述的指導，請參閱[代理技能最佳實踐](/docs/zh-TW/agents-and-tools/agent-skills/best-practices#writing-effective-descriptions)。

### 其他故障排除

有關一般技能故障排除（YAML 語法、調試等），請參閱 [Claude Code 技能故障排除部分](https://code.claude.com/docs/skills#troubleshooting)。

## 相關文檔

### 技能指南
- [Claude Code 中的代理技能](https://code.claude.com/docs/skills)：包含創建、示例和故障排除的完整技能指南
- [代理技能概述](/docs/zh-TW/agents-and-tools/agent-skills/overview)：概念概述、優勢和架構
- [代理技能最佳實踐](/docs/zh-TW/agents-and-tools/agent-skills/best-practices)：有效技能的編寫指南
- [代理技能食譜](https://github.com/anthropics/claude-cookbooks/tree/main/skills)：示例技能和模板

### SDK 資源
- [SDK 中的子代理](/docs/zh-TW/agent-sdk/subagents)：類似的基於文件系統的代理，具有編程選項
- [SDK 中的斜杠命令](/docs/zh-TW/agent-sdk/slash-commands)：用戶調用的命令
- [SDK 概述](/docs/zh-TW/agent-sdk/overview)：一般 SDK 概念
- [TypeScript SDK 參考](/docs/zh-TW/agent-sdk/typescript)：完整 API 文檔
- [Python SDK 參考](/docs/zh-TW/agent-sdk/python)：完整 API 文檔