# 託管 Agent SDK

在生產環境中部署和託管 Claude Agent SDK

---

Claude Agent SDK 與傳統的無狀態 LLM API 不同，它維護對話狀態並在持久環境中執行命令。本指南涵蓋了 SDK 型代理在生產環境中部署的架構、託管考慮因素和最佳實踐。

<Info>
如需超越基本沙箱的安全強化——包括網絡控制、憑證管理和隔離選項——請參閱 [安全部署](/docs/zh-TW/agent-sdk/secure-deployment)。
</Info>

## 託管要求

### 基於容器的沙箱

為了安全性和隔離，SDK 應在沙箱容器環境中運行。這提供了進程隔離、資源限制、網絡控制和臨時文件系統。

SDK 還支持 [程序化沙箱配置](/docs/zh-TW/agent-sdk/typescript#sandbox-settings) 用於命令執行。

### 系統要求

每個 SDK 實例需要：

- **運行時依賴項**
  - Python 3.10+（用於 Python SDK）或 Node.js 18+（用於 TypeScript SDK）
  - Node.js（由 Claude Code CLI 要求）
  - Claude Code CLI：`npm install -g @anthropic-ai/claude-code`

- **資源分配**
  - 建議：1GiB RAM、5GiB 磁盤和 1 個 CPU（根據您的任務需要調整）

- **網絡訪問**
  - 出站 HTTPS 到 `api.anthropic.com`
  - 可選：訪問 MCP 服務器或外部工具

## 理解 SDK 架構

與無狀態 API 調用不同，Claude Agent SDK 作為 **長期運行的進程** 運行，該進程：
- **在持久 shell 環境中執行命令**
- **在工作目錄內管理文件操作**
- **使用來自先前交互的上下文處理工具執行**

## 沙箱提供商選項

多個提供商專門提供用於 AI 代碼執行的安全容器環境：

- **[Cloudflare Sandboxes](https://github.com/cloudflare/sandbox-sdk)**
- **[Modal Sandboxes](https://modal.com/docs/guide/sandbox)**
- **[Daytona](https://www.daytona.io/)**
- **[E2B](https://e2b.dev/)**
- **[Fly Machines](https://fly.io/docs/machines/)**
- **[Vercel Sandbox](https://vercel.com/docs/functions/sandbox)**

有關自託管選項（Docker、gVisor、Firecracker）和詳細隔離配置，請參閱 [隔離技術](/docs/zh-TW/agent-sdk/secure-deployment#isolation-technologies)。

## 生產部署模式

### 模式 1：臨時會話

為每個用戶任務創建一個新容器，然後在完成時銷毀它。

最適合一次性任務，用戶可能在任務完成時仍與 AI 交互，但一旦完成，容器就會被銷毀。

**示例：**
- 錯誤調查和修復：使用相關上下文調試和解決特定問題
- 發票處理：從收據/發票中提取和結構化數據用於會計系統
- 翻譯任務：在語言之間翻譯文檔或內容批次
- 圖像/視頻處理：對媒體文件應用轉換、優化或提取元數據

### 模式 2：長期運行會話

為長期運行的任務維護持久容器實例。通常在容器內根據需求運行 **多個** Claude Agent 進程。

最適合主動代理，它們在沒有用戶輸入的情況下採取行動，提供內容的代理或處理大量消息的代理。

**示例：**
- 電子郵件代理：監控傳入電子郵件並根據內容自主進行分類、回復或採取行動
- 網站構建器：為每個用戶託管自定義網站，具有通過容器端口提供的實時編輯功能
- 高頻聊天機器人：處理來自 Slack 等平台的連續消息流，其中快速響應時間至關重要

### 模式 3：混合會話

臨時容器，使用歷史和狀態進行補充，可能來自數據庫或 SDK 的會話恢復功能。

最適合與用戶進行間歇性交互的容器，啟動工作並在工作完成時關閉，但可以繼續進行。

**示例：**
- 個人項目經理：幫助管理進行中的項目，進行間歇性檢查，維護任務、決策和進度的上下文
- 深度研究：進行多小時的研究任務，保存發現並在用戶返回時恢復調查
- 客戶支持代理：處理跨越多個交互的支持票證，加載票證歷史和客戶上下文

### 模式 4：單個容器

在一個全局容器中運行多個 Claude Agent SDK 進程。

最適合必須緊密協作的代理。這可能是最不受歡迎的模式，因為您必須防止代理相互覆蓋。

**示例：**
- **模擬**：在模擬（如視頻遊戲）中相互交互的代理。

# 常見問題

### 我如何與我的沙箱通信？
在容器中託管時，公開端口以與您的 SDK 實例通信。您的應用程序可以為外部客戶端公開 HTTP/WebSocket 端點，而 SDK 在容器內部運行。

### 託管容器的成本是多少？
我們發現服務代理的主要成本是令牌，容器根據您配置的內容而異，但最低成本大約是每小時運行 5 美分。

### 何時應該關閉空閒容器與保持它們溫暖？
這可能取決於提供商，不同的沙箱提供商將允許您為空閒超時設置不同的條件，之後沙箱可能會關閉。
您需要根據您認為用戶響應可能的頻率來調整此超時。

### 我應該多久更新一次 Claude Code CLI？
Claude Code CLI 使用 semver 進行版本控制，因此任何破壞性更改都將被版本化。

### 我如何監控容器健康和代理性能？
由於容器只是服務器，您用於後端的相同日誌記錄基礎設施將適用於容器。

### 代理會話在超時前可以運行多長時間？
代理會話不會超時，但我們建議設置 'maxTurns' 屬性以防止 Claude 陷入循環。

## 後續步驟

- [安全部署](/docs/zh-TW/agent-sdk/secure-deployment) - 網絡控制、憑證管理和隔離強化
- [TypeScript SDK - 沙箱設置](/docs/zh-TW/agent-sdk/typescript#sandbox-settings) - 以程序方式配置沙箱
- [會話指南](/docs/zh-TW/agent-sdk/sessions) - 了解會話管理
- [權限](/docs/zh-TW/agent-sdk/permissions) - 配置工具權限
- [成本追蹤](/docs/zh-TW/agent-sdk/cost-tracking) - 監控 API 使用情況
- [MCP 集成](/docs/zh-TW/agent-sdk/mcp) - 使用自定義工具進行擴展