# SDK 中的子代理

在 Claude Agent SDK 中使用子代理

---

Claude Agent SDK 中的子代理是由主代理編排的專門化 AI。
使用子代理進行上下文管理和並行化。

本指南說明如何使用 `agents` 參數在 SDK 中定義和使用子代理。

## 概述

在使用 SDK 時，子代理可以通過兩種方式定義：

1. **程式化** - 在您的 `query()` 選項中使用 `agents` 參數（推薦用於 SDK 應用程式）
2. **基於檔案系統** - 將帶有 YAML 前置資料的 markdown 檔案放置在指定目錄中（`.claude/agents/`）

本指南主要專注於使用 `agents` 參數的程式化方法，這為 SDK 應用程式提供了更整合的開發體驗。

## 使用子代理的好處

### 上下文管理
子代理與主代理保持獨立的上下文，防止資訊過載並保持互動的專注性。這種隔離確保專門化任務不會用無關細節污染主對話上下文。

**範例**：`research-assistant` 子代理可以探索數十個檔案和文件頁面，而不會用所有中間搜尋結果混亂主對話 - 只返回相關發現。

### 並行化
多個子代理可以同時運行，大幅加速複雜工作流程。

**範例**：在程式碼審查期間，您可以同時運行 `style-checker`、`security-scanner` 和 `test-coverage` 子代理，將審查時間從幾分鐘縮短到幾秒鐘。

### 專門化指令和知識
每個子代理都可以有量身定制的系統提示，具有特定的專業知識、最佳實踐和約束。

**範例**：`database-migration` 子代理可以擁有關於 SQL 最佳實踐、回滾策略和資料完整性檢查的詳細知識，這些在主代理的指令中會是不必要的雜訊。

### 工具限制
子代理可以被限制為特定工具，降低意外操作的風險。

**範例**：`doc-reviewer` 子代理可能只能存取 Read 和 Grep 工具，確保它可以分析但永遠不會意外修改您的文件檔案。

## 建立子代理

### 程式化定義（推薦）

使用 `agents` 參數直接在您的程式碼中定義子代理：

```typescript
import { query } from '@anthropic-ai/claude-agent-sdk';

const result = query({
  prompt: "Review the authentication module for security issues",
  options: {
    agents: {
      'code-reviewer': {
        description: 'Expert code review specialist. Use for quality, security, and maintainability reviews.',
        prompt: `You are a code review specialist with expertise in security, performance, and best practices.

When reviewing code:
- Identify security vulnerabilities
- Check for performance issues
- Verify adherence to coding standards
- Suggest specific improvements

Be thorough but concise in your feedback.`,
        tools: ['Read', 'Grep', 'Glob'],
        model: 'sonnet'
      },
      'test-runner': {
        description: 'Runs and analyzes test suites. Use for test execution and coverage analysis.',
        prompt: `You are a test execution specialist. Run tests and provide clear analysis of results.

Focus on:
- Running test commands
- Analyzing test output
- Identifying failing tests
- Suggesting fixes for failures`,
        tools: ['Bash', 'Read', 'Grep'],
      }
    }
  }
});

for await (const message of result) {
  console.log(message);
}
```

### AgentDefinition 配置

| 欄位 | 類型 | 必需 | 描述 |
|:---|:---|:---|:---|
| `description` | `string` | 是 | 何時使用此代理的自然語言描述 |
| `prompt` | `string` | 是 | 定義代理角色和行為的系統提示 |
| `tools` | `string[]` | 否 | 允許的工具名稱陣列。如果省略，繼承所有工具 |
| `model` | `'sonnet' \| 'opus' \| 'haiku' \| 'inherit'` | 否 | 此代理的模型覆蓋。如果省略，預設為主模型 |

### 基於檔案系統的定義（替代方案）

您也可以將子代理定義為特定目錄中的 markdown 檔案：

- **專案級別**：`.claude/agents/*.md` - 僅在當前專案中可用
- **使用者級別**：`~/.claude/agents/*.md` - 在所有專案中可用

每個子代理都是帶有 YAML 前置資料的 markdown 檔案：

```markdown
---
name: code-reviewer
description: Expert code review specialist. Use for quality, security, and maintainability reviews.
tools: Read, Grep, Glob, Bash
---

Your subagent's system prompt goes here. This defines the subagent's
role, capabilities, and approach to solving problems.
```

**注意**：程式化定義的代理（通過 `agents` 參數）優先於同名的基於檔案系統的代理。

## SDK 如何使用子代理

使用 Claude Agent SDK 時，子代理可以程式化定義或從檔案系統載入。Claude 將：

1. **載入程式化代理** 從您選項中的 `agents` 參數
2. **自動檢測檔案系統代理** 從 `.claude/agents/` 目錄（如果未被覆蓋）
3. **自動調用它們** 基於任務匹配和代理的 `description`
4. **使用它們的專門化提示** 和工具限制
5. **為每個子代理調用維護獨立上下文**

程式化定義的代理（通過 `agents` 參數）優先於同名的基於檔案系統的代理。

## 範例子代理

有關子代理的全面範例，包括程式碼審查員、測試執行器、除錯器和安全稽核員，請參閱[主要子代理指南](https://code.claude.com/docs/sub-agents#example-subagents)。該指南包括詳細的配置和建立有效子代理的最佳實踐。

## SDK 整合模式

### 自動調用

SDK 將根據任務上下文自動調用適當的子代理。確保您代理的 `description` 欄位清楚地指示何時應該使用它：

```typescript
const result = query({
  prompt: "Optimize the database queries in the API layer",
  options: {
    agents: {
      'performance-optimizer': {
        description: 'Use PROACTIVELY when code changes might impact performance. MUST BE USED for optimization tasks.',
        prompt: 'You are a performance optimization specialist...',
        tools: ['Read', 'Edit', 'Bash', 'Grep'],
        model: 'sonnet'
      }
    }
  }
});
```

### 明確調用

使用者可以在他們的提示中請求特定的子代理：

```typescript
const result = query({
  prompt: "Use the code-reviewer agent to check the authentication module",
  options: {
    agents: {
      'code-reviewer': {
        description: 'Expert code review specialist',
        prompt: 'You are a security-focused code reviewer...',
        tools: ['Read', 'Grep', 'Glob']
      }
    }
  }
});
```

### 動態代理配置

您可以根據應用程式的需求動態配置代理：

```typescript
import { query, type AgentDefinition } from '@anthropic-ai/claude-agent-sdk';

function createSecurityAgent(securityLevel: 'basic' | 'strict'): AgentDefinition {
  return {
    description: 'Security code reviewer',
    prompt: `You are a ${securityLevel === 'strict' ? 'strict' : 'balanced'} security reviewer...`,
    tools: ['Read', 'Grep', 'Glob'],
    model: securityLevel === 'strict' ? 'opus' : 'sonnet'
  };
}

const result = query({
  prompt: "Review this PR for security issues",
  options: {
    agents: {
      'security-reviewer': createSecurityAgent('strict')
    }
  }
});
```

## 工具限制

子代理可以通過 `tools` 欄位限制工具存取：

- **省略該欄位** - 代理繼承所有可用工具（預設）
- **指定工具** - 代理只能使用列出的工具

唯讀分析代理的範例：

```typescript
const result = query({
  prompt: "Analyze the architecture of this codebase",
  options: {
    agents: {
      'code-analyzer': {
        description: 'Static code analysis and architecture review',
        prompt: `You are a code architecture analyst. Analyze code structure,
identify patterns, and suggest improvements without making changes.`,
        tools: ['Read', 'Grep', 'Glob']  // No write or execute permissions
      }
    }
  }
});
```

### 常見工具組合

**唯讀代理**（分析、審查）：
```typescript
tools: ['Read', 'Grep', 'Glob']
```

**測試執行代理**：
```typescript
tools: ['Bash', 'Read', 'Grep']
```

**程式碼修改代理**：
```typescript
tools: ['Read', 'Edit', 'Write', 'Grep', 'Glob']
```

## 相關文件

- [主要子代理指南](https://code.claude.com/docs/sub-agents) - 全面的子代理文件
- [SDK 概述](/docs/zh-TW/agent-sdk/overview) - Claude Agent SDK 概述
- [設定](https://code.claude.com/docs/settings) - 配置檔案參考
- [斜線命令](https://code.claude.com/docs/slash-commands) - 自訂命令建立