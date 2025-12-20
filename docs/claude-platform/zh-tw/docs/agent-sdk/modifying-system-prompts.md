# 修改系統提示

學習如何透過三種方法修改系統提示來自訂 Claude 的行為 - 輸出樣式、systemPrompt 附加，以及自訂系統提示。

---

系統提示定義了 Claude 的行為、能力和回應風格。Claude Agent SDK 提供三種自訂系統提示的方法：使用輸出樣式（持久性、基於檔案的配置）、附加到 Claude Code 的提示，或使用完全自訂的提示。

## 理解系統提示

系統提示是塑造 Claude 在整個對話過程中行為的初始指令集。

<Note>
**預設行為：** Agent SDK 預設使用**空的系統提示**以獲得最大靈活性。要使用 Claude Code 的系統提示（工具指令、程式碼指南等），請在 TypeScript 中指定 `systemPrompt: { preset: "claude_code" }` 或在 Python 中指定 `system_prompt="claude_code"`。
</Note>

Claude Code 的系統提示包括：

- 工具使用指令和可用工具
- 程式碼風格和格式指南
- 回應語調和詳細程度設定
- 安全和保護指令
- 關於當前工作目錄和環境的上下文

## 修改方法

### 方法 1：CLAUDE.md 檔案（專案層級指令）

CLAUDE.md 檔案提供專案特定的上下文和指令，當 Agent SDK 在目錄中執行時會自動讀取。它們作為您專案的持久性「記憶」。

#### CLAUDE.md 如何與 SDK 配合使用

**位置和發現：**

- **專案層級：** 您工作目錄中的 `CLAUDE.md` 或 `.claude/CLAUDE.md`
- **使用者層級：** `~/.claude/CLAUDE.md` 用於所有專案的全域指令

**重要：** SDK 只有在您明確配置 `settingSources`（TypeScript）或 `setting_sources`（Python）時才會讀取 CLAUDE.md 檔案：

- 包含 `'project'` 以載入專案層級的 CLAUDE.md
- 包含 `'user'` 以載入使用者層級的 CLAUDE.md（`~/.claude/CLAUDE.md`）

`claude_code` 系統提示預設不會自動載入 CLAUDE.md - 您還必須指定設定來源。

**內容格式：**
CLAUDE.md 檔案使用純 markdown 格式，可以包含：

- 編碼指南和標準
- 專案特定上下文
- 常用命令或工作流程
- API 慣例
- 測試需求

#### CLAUDE.md 範例

```markdown
# 專案指南

## 程式碼風格

- 使用 TypeScript 嚴格模式
- 在 React 中偏好函數式元件
- 總是為公共 API 包含 JSDoc 註解

## 測試

- 提交前執行 `npm test`
- 維持 >80% 程式碼覆蓋率
- 使用 jest 進行單元測試，playwright 進行 E2E

## 命令

- 建置：`npm run build`
- 開發伺服器：`npm run dev`
- 類型檢查：`npm run typecheck`
```

#### 在 SDK 中使用 CLAUDE.md

<CodeGroup>

```typescript TypeScript
import { query } from "@anthropic-ai/claude-agent-sdk";

// 重要：您必須指定 settingSources 來載入 CLAUDE.md
// 僅使用 claude_code 預設不會載入 CLAUDE.md 檔案
const messages = [];

for await (const message of query({
  prompt: "為使用者個人檔案新增一個新的 React 元件",
  options: {
    systemPrompt: {
      type: "preset",
      preset: "claude_code", // 使用 Claude Code 的系統提示
    },
    settingSources: ["project"], // 從專案載入 CLAUDE.md 所需
  },
})) {
  messages.push(message);
}

// 現在 Claude 可以存取您專案中 CLAUDE.md 的指南
```

```python Python
from claude_agent_sdk import query, ClaudeAgentOptions

# 重要：您必須指定 setting_sources 來載入 CLAUDE.md
# 僅使用 claude_code 預設不會載入 CLAUDE.md 檔案
messages = []

async for message in query(
    prompt="為使用者個人檔案新增一個新的 React 元件",
    options=ClaudeAgentOptions(
        system_prompt={
            "type": "preset",
            "preset": "claude_code"  # 使用 Claude Code 的系統提示
        },
        setting_sources=["project"]  # 從專案載入 CLAUDE.md 所需
    )
):
    messages.append(message)

# 現在 Claude 可以存取您專案中 CLAUDE.md 的指南
```

</CodeGroup>

#### 何時使用 CLAUDE.md

**最適合：**

- **團隊共享上下文** - 每個人都應該遵循的指南
- **專案慣例** - 編碼標準、檔案結構、命名模式
- **常用命令** - 特定於您專案的建置、測試、部署命令
- **長期記憶** - 應該在所有會話中持續存在的上下文
- **版本控制指令** - 提交到 git 以便團隊保持同步

**主要特徵：**

- ✅ 在專案的所有會話中持續存在
- ✅ 透過 git 與團隊共享
- ✅ 自動發現（不需要程式碼變更）
- ⚠️ 需要透過 `settingSources` 載入設定

### 方法 2：輸出樣式（持久性配置）

輸出樣式是修改 Claude 系統提示的已儲存配置。它們以 markdown 檔案形式儲存，可以在會話和專案之間重複使用。

#### 建立輸出樣式

<CodeGroup>

```typescript TypeScript
import { writeFile, mkdir } from "fs/promises";
import { join } from "path";
import { homedir } from "os";

async function createOutputStyle(
  name: string,
  description: string,
  prompt: string
) {
  // 使用者層級：~/.claude/output-styles
  // 專案層級：.claude/output-styles
  const outputStylesDir = join(homedir(), ".claude", "output-styles");

  await mkdir(outputStylesDir, { recursive: true });

  const content = `---
name: ${name}
description: ${description}
---

${prompt}`;

  const filePath = join(
    outputStylesDir,
    `${name.toLowerCase().replace(/\s+/g, "-")}.md`
  );
  await writeFile(filePath, content, "utf-8");
}

// 範例：建立程式碼審查專家
await createOutputStyle(
  "Code Reviewer",
  "徹底的程式碼審查助手",
  `您是一位專業的程式碼審查員。

對於每個程式碼提交：
1. 檢查錯誤和安全問題
2. 評估效能
3. 建議改進
4. 評估程式碼品質（1-10）`
);
```

```python Python
from pathlib import Path

async def create_output_style(name: str, description: str, prompt: str):
    # 使用者層級：~/.claude/output-styles
    # 專案層級：.claude/output-styles
    output_styles_dir = Path.home() / '.claude' / 'output-styles'

    output_styles_dir.mkdir(parents=True, exist_ok=True)

    content = f"""---
name: {name}
description: {description}
---

{prompt}"""

    file_name = name.lower().replace(' ', '-') + '.md'
    file_path = output_styles_dir / file_name
    file_path.write_text(content, encoding='utf-8')

# 範例：建立程式碼審查專家
await create_output_style(
    'Code Reviewer',
    '徹底的程式碼審查助手',
    """您是一位專業的程式碼審查員。

對於每個程式碼提交：
1. 檢查錯誤和安全問題
2. 評估效能
3. 建議改進
4. 評估程式碼品質（1-10）"""
)
```

</CodeGroup>

#### 使用輸出樣式

建立後，透過以下方式啟用輸出樣式：

- **CLI**：`/output-style [style-name]`
- **設定**：`.claude/settings.local.json`
- **建立新的**：`/output-style:new [description]`

**SDK 使用者注意：** 當您在選項中包含 `settingSources: ['user']` 或 `settingSources: ['project']`（TypeScript）/ `setting_sources=["user"]` 或 `setting_sources=["project"]`（Python）時，會載入輸出樣式。

### 方法 3：使用 `systemPrompt` 附加

您可以使用 Claude Code 預設與 `append` 屬性來新增您的自訂指令，同時保留所有內建功能。

<CodeGroup>

```typescript TypeScript
import { query } from "@anthropic-ai/claude-agent-sdk";

const messages = [];

for await (const message of query({
  prompt: "幫我寫一個計算費波那契數列的 Python 函數",
  options: {
    systemPrompt: {
      type: "preset",
      preset: "claude_code",
      append:
        "在 Python 程式碼中總是包含詳細的文件字串和類型提示。",
    },
  },
})) {
  messages.push(message);
  if (message.type === "assistant") {
    console.log(message.message.content);
  }
}
```

```python Python
from claude_agent_sdk import query, ClaudeAgentOptions

messages = []

async for message in query(
    prompt="幫我寫一個計算費波那契數列的 Python 函數",
    options=ClaudeAgentOptions(
        system_prompt={
            "type": "preset",
            "preset": "claude_code",
            "append": "在 Python 程式碼中總是包含詳細的文件字串和類型提示。"
        }
    )
):
    messages.append(message)
    if message.type == 'assistant':
        print(message.message.content)
```

</CodeGroup>

### 方法 4：自訂系統提示

您可以提供自訂字串作為 `systemPrompt`，完全用您自己的指令替換預設值。

<CodeGroup>

```typescript TypeScript
import { query } from "@anthropic-ai/claude-agent-sdk";

const customPrompt = `您是一位 Python 編碼專家。
遵循這些指南：
- 撰寫乾淨、有良好文件的程式碼
- 為所有函數使用類型提示
- 包含全面的文件字串
- 適當時偏好函數式程式設計模式
- 總是解釋您的程式碼選擇`;

const messages = [];

for await (const message of query({
  prompt: "建立一個資料處理管線",
  options: {
    systemPrompt: customPrompt,
  },
})) {
  messages.push(message);
  if (message.type === "assistant") {
    console.log(message.message.content);
  }
}
```

```python Python
from claude_agent_sdk import query, ClaudeAgentOptions

custom_prompt = """您是一位 Python 編碼專家。
遵循這些指南：
- 撰寫乾淨、有良好文件的程式碼
- 為所有函數使用類型提示
- 包含全面的文件字串
- 適當時偏好函數式程式設計模式
- 總是解釋您的程式碼選擇"""

messages = []

async for message in query(
    prompt="建立一個資料處理管線",
    options=ClaudeAgentOptions(
        system_prompt=custom_prompt
    )
):
    messages.append(message)
    if message.type == 'assistant':
        print(message.message.content)
```

</CodeGroup>

## 四種方法的比較

| 功能                 | CLAUDE.md           | 輸出樣式      | `systemPrompt` 附加 | 自訂 `systemPrompt`     |
| --- | --- | --- | --- | --- |
| **持久性**         | 每個專案檔案 | 儲存為檔案  | 僅限會話            | 僅限會話           |
| **重複使用性**         | 每個專案      | 跨專案 | 程式碼重複        | 程式碼重複       |
| **管理**          | 在檔案系統    | CLI + 檔案     | 在程式碼中                 | 在程式碼中                |
| **預設工具**       | 保留        | 保留       | 保留               | 遺失（除非包含） |
| **內建安全性**     | 維持       | 維持      | 維持              | 必須新增          |
| **環境上下文** | 自動        | 自動       | 自動               | 必須提供       |
| **自訂層級** | 僅新增   | 替換預設 | 僅新增          | 完全控制       |
| **版本控制**     | 與專案一起     | 是             | 與程式碼一起               | 與程式碼一起              |
| **範圍**               | 專案特定 | 使用者或專案 | 程式碼會話            | 程式碼會話           |

**注意：** "附加" 意指在 TypeScript 中使用 `systemPrompt: { type: "preset", preset: "claude_code", append: "..." }` 或在 Python 中使用 `system_prompt={"type": "preset", "preset": "claude_code", "append": "..."}`。

## 使用案例和最佳實踐

### 何時使用 CLAUDE.md

**最適合：**

- 專案特定的編碼標準和慣例
- 記錄專案結構和架構
- 列出常用命令（建置、測試、部署）
- 應該進行版本控制的團隊共享上下文
- 適用於專案中所有 SDK 使用的指令

**範例：**

- "所有 API 端點都應該使用 async/await 模式"
- "提交前執行 `npm run lint:fix`"
- "資料庫遷移在 `migrations/` 目錄中"

**重要：** 要載入 CLAUDE.md 檔案，您必須明確設定 `settingSources: ['project']`（TypeScript）或 `setting_sources=["project"]`（Python）。沒有此設定，`claude_code` 系統提示預設不會自動載入 CLAUDE.md。

### 何時使用輸出樣式

**最適合：**

- 跨會話的持久性行為變更
- 團隊共享配置
- 專業助手（程式碼審查員、資料科學家、DevOps）
- 需要版本控制的複雜提示修改

**範例：**

- 建立專門的 SQL 最佳化助手
- 建立專注於安全性的程式碼審查員
- 開發具有特定教學法的教學助手

### 何時使用 `systemPrompt` 附加

**最適合：**

- 新增特定的編碼標準或偏好
- 自訂輸出格式
- 新增領域特定知識
- 修改回應詳細程度
- 在不失去工具指令的情況下增強 Claude Code 的預設行為

### 何時使用自訂 `systemPrompt`

**最適合：**

- 完全控制 Claude 的行為
- 專業的單一會話任務
- 測試新的提示策略
- 不需要預設工具的情況
- 建立具有獨特行為的專業代理

## 結合方法

您可以結合這些方法以獲得最大靈活性：

### 範例：輸出樣式與會話特定新增

<CodeGroup>

```typescript TypeScript
import { query } from "@anthropic-ai/claude-agent-sdk";

// 假設 "Code Reviewer" 輸出樣式已啟用（透過 /output-style）
// 新增會話特定的重點領域
const messages = [];

for await (const message of query({
  prompt: "審查這個身份驗證模組",
  options: {
    systemPrompt: {
      type: "preset",
      preset: "claude_code",
      append: `
        對於這次審查，優先考慮：
        - OAuth 2.0 合規性
        - 令牌儲存安全性
        - 會話管理
      `,
    },
  },
})) {
  messages.push(message);
}
```

```python Python
from claude_agent_sdk import query, ClaudeAgentOptions

# 假設 "Code Reviewer" 輸出樣式已啟用（透過 /output-style）
# 新增會話特定的重點領域
messages = []

async for message in query(
    prompt="審查這個身份驗證模組",
    options=ClaudeAgentOptions(
        system_prompt={
            "type": "preset",
            "preset": "claude_code",
            "append": """
            對於這次審查，優先考慮：
            - OAuth 2.0 合規性
            - 令牌儲存安全性
            - 會話管理
            """
        }
    )
):
    messages.append(message)
```

</CodeGroup>

## 另請參閱

- [輸出樣式](https://code.claude.com/docs/output-styles) - 完整的輸出樣式文件
- [TypeScript SDK 指南](/docs/zh-TW/agent-sdk/typescript) - 完整的 SDK 使用指南
- [TypeScript SDK 參考](https://code.claude.com/docs/typescript-sdk-reference) - 完整的 API 文件
- [配置指南](https://code.claude.com/docs/configuration) - 一般配置選項