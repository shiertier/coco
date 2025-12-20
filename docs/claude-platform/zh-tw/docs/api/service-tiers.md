# 服務層級

不同的服務層級允許您根據應用程式的需求在可用性、效能和可預測成本之間取得平衡。

---

我們提供三個服務層級：
- **優先層級：** 最適合在生產環境中部署的工作流程，其中時間、可用性和可預測定價很重要
- **標準層級：** 用於試驗和擴展日常使用案例的預設層級
- **批次層級：** 最適合可以等待或受益於在正常容量之外運行的非同步工作流程

## 標準層級

標準層級是所有 API 請求的預設服務層級。此層級中的請求與所有其他請求一起優先處理，並遵循盡力而為的可用性。

## 優先層級

此層級中的請求優先於所有其他對 Anthropic 的請求。此優先順序有助於最小化 ["伺服器超載" 錯誤](/docs/zh-TW/api/errors#http-errors)，即使在尖峰時段也是如此。

如需更多資訊，請參閱 [開始使用優先層級](#get-started-with-priority-tier)

## 請求如何被分配層級

在處理請求時，Anthropic 在以下情況下決定將請求分配給優先層級：
- 您的組織具有足夠的優先層級容量 **input** 每分鐘令牌數
- 您的組織具有足夠的優先層級容量 **output** 每分鐘令牌數

Anthropic 針對優先層級容量計算使用情況如下：

**輸入令牌**
- 快取讀取為每個從快取讀取的令牌 0.1 個令牌
- 快取寫入為每個寫入快取的令牌 1.25 個令牌，TTL 為 5 分鐘
- 快取寫入為每個寫入快取的令牌 2.00 個令牌，TTL 為 1 小時
- 對於 [長上下文](/docs/zh-TW/build-with-claude/context-windows)（>200k 輸入令牌）請求，輸入令牌為每個令牌 2 個令牌
- 所有其他輸入令牌為每個令牌 1 個令牌

**輸出令牌**
- 對於 [長上下文](/docs/zh-TW/build-with-claude/context-windows)（>200k 輸入令牌）請求，輸出令牌為每個令牌 1.5 個令牌
- 所有其他輸出令牌為每個令牌 1 個令牌

否則，請求以標準層級進行。

<Note>
分配給優先層級的請求從優先層級容量和常規速率限制中提取。
如果服務請求會超過速率限制，則請求被拒絕。
</Note>

## 使用服務層級

您可以通過設定 `service_tier` 參數來控制哪些服務層級可用於請求：

```python
message = client.messages.create(
    model="claude-sonnet-4-5",
    max_tokens=1024,
    messages=[{"role": "user", "content": "Hello, Claude!"}],
    service_tier="auto"  # Automatically use Priority Tier when available, fallback to standard
)
```

`service_tier` 參數接受以下值：

- `"auto"`（預設）- 如果可用，使用優先層級容量，否則回退到您的其他容量
- `"standard_only"` - 僅使用標準層級容量，如果您不想使用優先層級容量，這很有用

回應 `usage` 物件也包括分配給請求的服務層級：

```json
{
  "usage": {
    "input_tokens": 410,
    "cache_creation_input_tokens": 0,
    "cache_read_input_tokens": 0,
    "output_tokens": 585,
    "service_tier": "priority"
  }
}
```
這允許您確定哪個服務層級被分配給了請求。

當使用具有優先層級承諾的模型請求 `service_tier="auto"` 時，這些回應標頭提供見解：
```
anthropic-priority-input-tokens-limit: 10000
anthropic-priority-input-tokens-remaining: 9618
anthropic-priority-input-tokens-reset: 2025-01-12T23:11:59Z
anthropic-priority-output-tokens-limit: 10000
anthropic-priority-output-tokens-remaining: 6000
anthropic-priority-output-tokens-reset: 2025-01-12T23:12:21Z
```
您可以使用這些標頭的存在來檢測您的請求是否符合優先層級的條件，即使它超過了限制。

## 開始使用優先層級

如果您對以下方面感興趣，您可能想要承諾優先層級容量：
- **更高的可用性**：目標 99.5% 的正常運行時間，具有優先計算資源
- **成本控制**：可預測的支出和更長承諾期的折扣
- **靈活溢出**：當您超過承諾容量時自動回退到標準層級

承諾優先層級將涉及決定：
- 每分鐘輸入令牌數
- 每分鐘輸出令牌數
- 承諾期限（1、3、6 或 12 個月）
- 特定的模型版本

<Note>
您購買的輸入令牌與輸出令牌的比率很重要。調整您的優先層級容量以符合您的實際流量模式有助於您最大化購買令牌的利用率。
</Note>

### 支援的模型

優先層級支援：

- Claude Opus 4.5
- Claude Sonnet 4.5
- Claude Haiku 4.5
- Claude Opus 4.1
- Claude Opus 4
- Claude Sonnet 4
- Claude Sonnet 3.7（[已棄用](/docs/zh-TW/about-claude/model-deprecations)）
- Claude Haiku 3.5（[已棄用](/docs/zh-TW/about-claude/model-deprecations)）

查看 [模型概覽頁面](/docs/zh-TW/about-claude/models/overview) 以了解有關我們模型的更多詳細資訊。

### 如何存取優先層級

要開始使用優先層級：

1. [聯絡銷售](https://claude.com/contact-sales/priority-tier) 以完成佈建
2. （可選）更新您的 API 請求以選擇性地將 `service_tier` 參數設定為 `auto`
3. 通過回應標頭和 Claude 控制台監控您的使用情況