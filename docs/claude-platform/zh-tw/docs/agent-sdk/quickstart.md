# 快速開始

使用 Python 或 TypeScript Agent SDK 開始構建能夠自主工作的 AI 代理

---

使用 Agent SDK 構建一個 AI 代理，它可以讀取你的代碼、找到錯誤並修復它們，完全無需手動干預。

**你將做什麼：**
1. 使用 Agent SDK 設置一個項目
2. 創建一個包含一些有缺陷代碼的文件
3. 運行一個代理，自動找到並修復錯誤

## 前置條件

- **Node.js 18+** 或 **Python 3.10+**
- 一個 **Anthropic 帳戶**（[在此註冊](https://console.anthropic.com/)）

## 設置

<Steps>
  <Step title="安裝 Claude Code">
    Agent SDK 使用 Claude Code 作為其運行時。為你的平台安裝它：

    <Tabs>
      <Tab title="macOS/Linux/WSL">
        ```bash
        curl -fsSL https://claude.ai/install.sh | bash
        ```
      </Tab>
      <Tab title="Homebrew">
        ```bash
        brew install --cask claude-code
        ```
      </Tab>
      <Tab title="npm">
        ```bash
        npm install -g @anthropic-ai/claude-code
        ```
      </Tab>
    </Tabs>

    在你的機器上安裝 Claude Code 後，在終端中運行 `claude` 並按照提示進行身份驗證。SDK 將自動使用此身份驗證。

    <Tip>
    有關 Claude Code 安裝的更多信息，請參閱 [Claude Code 設置](https://docs.anthropic.com/en/docs/claude-code/setup)。
    </Tip>
  </Step>

  <Step title="創建項目文件夾">
    為此快速開始創建一個新目錄：

    ```bash
    mkdir my-agent && cd my-agent
    ```

    對於你自己的項目，你可以從任何文件夾運行 SDK；它默認將有權訪問該目錄及其子目錄中的文件。
  </Step>

  <Step title="安裝 SDK">
    為你的語言安裝 Agent SDK 包：

    <Tabs>
      <Tab title="TypeScript">
        ```bash
        npm install @anthropic-ai/claude-agent-sdk
        ```
      </Tab>
      <Tab title="Python (uv)">
        [uv Python 包管理器](https://docs.astral.sh/uv/) 是一個快速的 Python 包管理器，可以自動處理虛擬環境：
        ```bash
        uv init && uv add claude-agent-sdk
        ```
      </Tab>
      <Tab title="Python (pip)">
        首先創建一個虛擬環境，然後安裝：
        ```bash
        python3 -m venv .venv && source .venv/bin/activate
        pip3 install claude-agent-sdk
        ```
      </Tab>
    </Tabs>
  </Step>

  <Step title="設置你的 API 密鑰">
    如果你已經驗證了 Claude Code（通過在終端中運行 `claude`），SDK 將自動使用該身份驗證。

    否則，你需要一個 API 密鑰，你可以從 [Claude 控制台](https://console.anthropic.com/) 獲取。

    在你的項目目錄中創建一個 `.env` 文件並將 API 密鑰存儲在那裡：

    ```bash
    ANTHROPIC_API_KEY=your-api-key
    ```

    <Note>
    **使用 Amazon Bedrock、Google Vertex AI 或 Microsoft Azure？** 請參閱 [Bedrock](https://code.claude.com/docs/en/amazon-bedrock)、[Vertex AI](https://code.claude.com/docs/en/google-vertex-ai) 或 [Azure AI Foundry](https://code.claude.com/docs/en/azure-ai-foundry) 的設置指南。

    除非事先獲得批准，否則 Anthropic 不允許第三方開發人員為其產品提供 claude.ai 登錄或速率限制，包括基於 Claude Agent SDK 構建的代理。請改用本文檔中描述的 API 密鑰身份驗證方法。
    </Note>
  </Step>
</Steps>

## 創建有缺陷的文件

此快速開始將引導你構建一個可以找到並修復代碼中的錯誤的代理。首先，你需要一個包含一些有意錯誤的文件供代理修復。在 `my-agent` 目錄中創建 `utils.py` 並粘貼以下代碼：

```python
def calculate_average(numbers):
    total = 0
    for num in numbers:
        total += num
    return total / len(numbers)

def get_user_name(user):
    return user["name"].upper()
```

此代碼有兩個錯誤：
1. `calculate_average([])` 會因除以零而崩潰
2. `get_user_name(None)` 會因 TypeError 而崩潰

## 構建一個找到並修復錯誤的代理

如果你使用 Python SDK，請創建 `agent.py`，或者如果使用 TypeScript，請創建 `agent.ts`：

<CodeGroup>
```python Python
import asyncio
from claude_agent_sdk import query, ClaudeAgentOptions, AssistantMessage, ResultMessage

async def main():
    # Agentic loop: streams messages as Claude works
    async for message in query(
        prompt="Review utils.py for bugs that would cause crashes. Fix any issues you find.",
        options=ClaudeAgentOptions(
            allowed_tools=["Read", "Edit", "Glob"],  # Tools Claude can use
            permission_mode="acceptEdits"            # Auto-approve file edits
        )
    ):
        # Print human-readable output
        if isinstance(message, AssistantMessage):
            for block in message.content:
                if hasattr(block, "text"):
                    print(block.text)              # Claude's reasoning
                elif hasattr(block, "name"):
                    print(f"Tool: {block.name}")   # Tool being called
        elif isinstance(message, ResultMessage):
            print(f"Done: {message.subtype}")      # Final result

asyncio.run(main())
```

```typescript TypeScript
import { query } from "@anthropic-ai/claude-agent-sdk";

// Agentic loop: streams messages as Claude works
for await (const message of query({
  prompt: "Review utils.py for bugs that would cause crashes. Fix any issues you find.",
  options: {
    allowedTools: ["Read", "Edit", "Glob"],  // Tools Claude can use
    permissionMode: "acceptEdits"            // Auto-approve file edits
  }
})) {
  // Print human-readable output
  if (message.type === "assistant" && message.message?.content) {
    for (const block of message.message.content) {
      if ("text" in block) {
        console.log(block.text);             // Claude's reasoning
      } else if ("name" in block) {
        console.log(`Tool: ${block.name}`);  // Tool being called
      }
    }
  } else if (message.type === "result") {
    console.log(`Done: ${message.subtype}`); // Final result
  }
}
```
</CodeGroup>

此代碼有三個主要部分：

1. **`query`**：創建 agentic 循環的主要入口點。它返回一個異步迭代器，所以你使用 `async for` 來流式傳輸 Claude 工作時的消息。查看 [Python](/docs/zh-TW/agent-sdk/python#query) 或 [TypeScript](/docs/zh-TW/agent-sdk/typescript#query) SDK 參考中的完整 API。

2. **`prompt`**：你想讓 Claude 做什麼。Claude 根據任務確定要使用哪些工具。

3. **`options`**：代理的配置。此示例使用 `allowedTools` 將 Claude 限制為 `Read`、`Edit` 和 `Glob`，並使用 `permissionMode: "acceptEdits"` 自動批准文件更改。其他選項包括 `systemPrompt`、`mcpServers` 等。查看 [Python](/docs/zh-TW/agent-sdk/python#claudeagentoptions) 或 [TypeScript](/docs/zh-TW/agent-sdk/typescript#claudeagentoptions) 的所有選項。

`async for` 循環在 Claude 思考、調用工具、觀察結果並決定下一步做什麼時持續運行。每次迭代都會產生一條消息：Claude 的推理、工具調用、工具結果或最終結果。SDK 處理編排（工具執行、上下文管理、重試），所以你只需使用流。當 Claude 完成任務或遇到錯誤時，循環結束。

循環內的消息處理過濾人類可讀的輸出。如果沒有過濾，你會看到原始消息對象，包括系統初始化和內部狀態，這對調試很有用，但通常很冗長。

<Note>
此示例使用流式傳輸來實時顯示進度。如果你不需要實時輸出（例如，對於後台作業或 CI 管道），你可以一次性收集所有消息。有關詳細信息，請參閱 [流式傳輸與單輪模式](/docs/zh-TW/agent-sdk/streaming-vs-single-mode)。
</Note>

### 運行你的代理

你的代理已準備好。使用以下命令運行它：

<Tabs>
  <Tab title="Python">
    ```bash
    python3 agent.py
    ```
  </Tab>
  <Tab title="TypeScript">
    ```bash
    npx tsx agent.ts
    ```
  </Tab>
</Tabs>

運行後，檢查 `utils.py`。你會看到防禦性代碼處理空列表和空用戶。你的代理自主地：

1. **讀取** `utils.py` 以理解代碼
2. **分析** 邏輯並識別會導致崩潰的邊界情況
3. **編輯** 文件以添加適當的錯誤處理

這就是 Agent SDK 的不同之處：Claude 直接執行工具，而不是要求你實現它們。

<Note>
如果你看到「Claude Code not found」，請 [安裝 Claude Code](#安裝-claude-code) 並重新啟動終端。對於「API key not found」，請 [設置你的 API 密鑰](#設置你的-api-密鑰)。有關更多幫助，請參閱 [完整故障排除指南](https://docs.anthropic.com/en/docs/claude-code/troubleshooting)。
</Note>

### 嘗試其他提示

現在你的代理已設置好，嘗試一些不同的提示：

- `"Add docstrings to all functions in utils.py"`
- `"Add type hints to all functions in utils.py"`
- `"Create a README.md documenting the functions in utils.py"`

### 自定義你的代理

你可以通過更改選項來修改代理的行為。以下是一些示例：

**添加網絡搜索功能：**

<CodeGroup>
```python Python
options=ClaudeAgentOptions(
    allowed_tools=["Read", "Edit", "Glob", "WebSearch"],
    permission_mode="acceptEdits"
)
```

```typescript TypeScript
options: {
  allowedTools: ["Read", "Edit", "Glob", "WebSearch"],
  permissionMode: "acceptEdits"
}
```
</CodeGroup>

**給 Claude 一個自定義系統提示：**

<CodeGroup>
```python Python
options=ClaudeAgentOptions(
    allowed_tools=["Read", "Edit", "Glob"],
    permission_mode="acceptEdits",
    system_prompt="You are a senior Python developer. Always follow PEP 8 style guidelines."
)
```

```typescript TypeScript
options: {
  allowedTools: ["Read", "Edit", "Glob"],
  permissionMode: "acceptEdits",
  systemPrompt: "You are a senior Python developer. Always follow PEP 8 style guidelines."
}
```
</CodeGroup>

**在終端中運行命令：**

<CodeGroup>
```python Python
options=ClaudeAgentOptions(
    allowed_tools=["Read", "Edit", "Glob", "Bash"],
    permission_mode="acceptEdits"
)
```

```typescript TypeScript
options: {
  allowedTools: ["Read", "Edit", "Glob", "Bash"],
  permissionMode: "acceptEdits"
}
```
</CodeGroup>

啟用 `Bash` 後，嘗試：`"Write unit tests for utils.py, run them, and fix any failures"`

## 關鍵概念

**工具** 控制你的代理可以做什麼：

| 工具 | 代理可以做什麼 |
|-------|----------------------|
| `Read`、`Glob`、`Grep` | 只讀分析 |
| `Read`、`Edit`、`Glob` | 分析和修改代碼 |
| `Read`、`Edit`、`Bash`、`Glob`、`Grep` | 完全自動化 |

**權限模式** 控制你想要多少人工監督：

| 模式 | 行為 | 用例 |
|------|----------|----------|
| `acceptEdits` | 自動批准文件編輯，詢問其他操作 | 受信任的開發工作流 |
| `bypassPermissions` | 無提示運行 | CI/CD 管道、自動化 |
| `default` | 需要 `canUseTool` 回調來處理批准 | 自定義批准流程 |

上面的示例使用 `acceptEdits` 模式，它自動批准文件操作，以便代理可以無需交互提示運行。如果你想提示用戶批准，請使用 `default` 模式並提供一個 [`canUseTool` 回調](/docs/zh-TW/agent-sdk/permissions#canusetool) 來收集用戶輸入。有關更多控制，請參閱 [權限](/docs/zh-TW/agent-sdk/permissions)。

## 後續步驟

現在你已經創建了你的第一個代理，學習如何擴展其功能並根據你的用例進行定制：

- **[權限](/docs/zh-TW/agent-sdk/permissions)**：控制你的代理可以做什麼以及何時需要批准
- **[鉤子](/docs/zh-TW/agent-sdk/hooks)**：在工具調用之前或之後運行自定義代碼
- **[會話](/docs/zh-TW/agent-sdk/sessions)**：構建維護上下文的多輪代理
- **[MCP 服務器](/docs/zh-TW/agent-sdk/mcp)**：連接到數據庫、瀏覽器、API 和其他外部系統
- **[託管](/docs/zh-TW/agent-sdk/hosting)**：將代理部署到 Docker、雲和 CI/CD
- **[示例代理](https://github.com/anthropics/claude-agent-sdk-demos)**：查看完整示例：電子郵件助手、研究代理等