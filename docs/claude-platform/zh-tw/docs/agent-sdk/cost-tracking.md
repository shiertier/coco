# 追蹤成本和使用量

了解並追蹤 Claude Agent SDK 中用於計費的代幣使用量

---

# SDK 成本追蹤

Claude Agent SDK 為每次與 Claude 的互動提供詳細的代幣使用資訊。本指南說明如何正確追蹤成本並了解使用量報告，特別是在處理並行工具使用和多步驟對話時。

如需完整的 API 文件，請參閱 [TypeScript SDK 參考](https://code.claude.com/docs/typescript-sdk-reference)。

## 了解代幣使用量

當 Claude 處理請求時，它會在訊息層級報告代幣使用量。這些使用量資料對於追蹤成本和適當地向用戶計費至關重要。

### 關鍵概念

1. **步驟**：步驟是您的應用程式與 Claude 之間的單一請求/回應對
2. **訊息**：步驟內的個別訊息（文字、工具使用、工具結果）
3. **使用量**：附加到助理訊息的代幣消耗資料

## 使用量報告結構

### 單一與並行工具使用

當 Claude 執行工具時，使用量報告會根據工具是順序執行還是並行執行而有所不同：

<CodeGroup>

```typescript TypeScript
import { query } from "@anthropic-ai/claude-agent-sdk";

// 範例：在對話中追蹤使用量
const result = await query({
  prompt: "分析這個程式碼庫並執行測試",
  options: {
    onMessage: (message) => {
      if (message.type === 'assistant' && message.usage) {
        console.log(`訊息 ID: ${message.id}`);
        console.log(`使用量:`, message.usage);
      }
    }
  }
});
```

```python Python
from claude_agent_sdk import query, ClaudeAgentOptions, AssistantMessage
import asyncio

# 範例：在對話中追蹤使用量
async def track_usage():
    # 處理到達的訊息
    async for message in query(
        prompt="分析這個程式碼庫並執行測試"
    ):
        if isinstance(message, AssistantMessage) and hasattr(message, 'usage'):
            print(f"訊息 ID: {message.id}")
            print(f"使用量: {message.usage}")

asyncio.run(track_usage())
```

</CodeGroup>

### 訊息流程範例

以下是典型多步驟對話中訊息和使用量的報告方式：

```
<!-- 步驟 1：使用並行工具的初始請求 -->
assistant (text)      { id: "msg_1", usage: { output_tokens: 100, ... } }
assistant (tool_use)  { id: "msg_1", usage: { output_tokens: 100, ... } }
assistant (tool_use)  { id: "msg_1", usage: { output_tokens: 100, ... } }
assistant (tool_use)  { id: "msg_1", usage: { output_tokens: 100, ... } }
user (tool_result)
user (tool_result)
user (tool_result)

<!-- 步驟 2：後續回應 -->
assistant (text)      { id: "msg_2", usage: { output_tokens: 98, ... } }
```

## 重要使用量規則

### 1. 相同 ID = 相同使用量

**所有具有相同 `id` 欄位的訊息報告相同的使用量**。當 Claude 在同一輪中發送多個訊息（例如，文字 + 工具使用）時，它們共享相同的訊息 ID 和使用量資料。

```typescript
// 所有這些訊息都有相同的 ID 和使用量
const messages = [
  { type: 'assistant', id: 'msg_123', usage: { output_tokens: 100 } },
  { type: 'assistant', id: 'msg_123', usage: { output_tokens: 100 } },
  { type: 'assistant', id: 'msg_123', usage: { output_tokens: 100 } }
];

// 每個唯一訊息 ID 只收費一次
const uniqueUsage = messages[0].usage; // 對於具有此 ID 的所有訊息都相同
```

### 2. 每步驟收費一次

**您應該只對每個步驟向用戶收費一次**，而不是對每個個別訊息收費。當您看到具有相同 ID 的多個助理訊息時，使用其中任何一個的使用量。

### 3. 結果訊息包含累積使用量

最終的 `result` 訊息包含對話中所有步驟的總累積使用量：

```typescript
// 最終結果包含總使用量
const result = await query({
  prompt: "多步驟任務",
  options: { /* ... */ }
});

console.log("總使用量:", result.usage);
console.log("總成本:", result.usage.total_cost_usd);
```

## 實作：成本追蹤系統

以下是實作成本追蹤系統的完整範例：

<CodeGroup>

```typescript TypeScript
import { query } from "@anthropic-ai/claude-agent-sdk";

class CostTracker {
  private processedMessageIds = new Set<string>();
  private stepUsages: Array<any> = [];
  
  async trackConversation(prompt: string) {
    const result = await query({
      prompt,
      options: {
        onMessage: (message) => {
          this.processMessage(message);
        }
      }
    });
    
    return {
      result,
      stepUsages: this.stepUsages,
      totalCost: result.usage?.total_cost_usd || 0
    };
  }
  
  private processMessage(message: any) {
    // 只處理具有使用量的助理訊息
    if (message.type !== 'assistant' || !message.usage) {
      return;
    }
    
    // 如果已經處理過此訊息 ID，則跳過
    if (this.processedMessageIds.has(message.id)) {
      return;
    }
    
    // 標記為已處理並記錄使用量
    this.processedMessageIds.add(message.id);
    this.stepUsages.push({
      messageId: message.id,
      timestamp: new Date().toISOString(),
      usage: message.usage,
      costUSD: this.calculateCost(message.usage)
    });
  }
  
  private calculateCost(usage: any): number {
    // 在此處實作您的定價計算
    // 這是一個簡化的範例
    const inputCost = usage.input_tokens * 0.00003;
    const outputCost = usage.output_tokens * 0.00015;
    const cacheReadCost = (usage.cache_read_input_tokens || 0) * 0.0000075;
    
    return inputCost + outputCost + cacheReadCost;
  }
}

// 使用方式
const tracker = new CostTracker();
const { result, stepUsages, totalCost } = await tracker.trackConversation(
  "分析並重構這個程式碼"
);

console.log(`處理的步驟數: ${stepUsages.length}`);
console.log(`總成本: $${totalCost.toFixed(4)}`);
```

```python Python
from claude_agent_sdk import query, AssistantMessage, ResultMessage
from datetime import datetime
import asyncio

class CostTracker:
    def __init__(self):
        self.processed_message_ids = set()
        self.step_usages = []

    async def track_conversation(self, prompt):
        result = None

        # 處理到達的訊息
        async for message in query(prompt=prompt):
            self.process_message(message)

            # 捕獲最終結果訊息
            if isinstance(message, ResultMessage):
                result = message

        return {
            "result": result,
            "step_usages": self.step_usages,
            "total_cost": result.total_cost_usd if result else 0
        }

    def process_message(self, message):
        # 只處理具有使用量的助理訊息
        if not isinstance(message, AssistantMessage) or not hasattr(message, 'usage'):
            return

        # 如果已經處理過此訊息 ID，則跳過
        message_id = getattr(message, 'id', None)
        if not message_id or message_id in self.processed_message_ids:
            return

        # 標記為已處理並記錄使用量
        self.processed_message_ids.add(message_id)
        self.step_usages.append({
            "message_id": message_id,
            "timestamp": datetime.now().isoformat(),
            "usage": message.usage,
            "cost_usd": self.calculate_cost(message.usage)
        })

    def calculate_cost(self, usage):
        # 實作您的定價計算
        input_cost = usage.get("input_tokens", 0) * 0.00003
        output_cost = usage.get("output_tokens", 0) * 0.00015
        cache_read_cost = usage.get("cache_read_input_tokens", 0) * 0.0000075

        return input_cost + output_cost + cache_read_cost

# 使用方式
async def main():
    tracker = CostTracker()
    result = await tracker.track_conversation("分析並重構這個程式碼")

    print(f"處理的步驟數: {len(result['step_usages'])}")
    print(f"總成本: ${result['total_cost']:.4f}")

asyncio.run(main())
```

</CodeGroup>

## 處理邊緣情況

### 輸出代幣差異

在極少數情況下，您可能會觀察到具有相同 ID 的訊息有不同的 `output_tokens` 值。當發生這種情況時：

1. **使用最高值** - 群組中的最後一個訊息通常包含準確的總計
2. **對照總成本驗證** - 結果訊息中的 `total_cost_usd` 是權威的
3. **報告不一致** - 在 [Claude Code GitHub 儲存庫](https://github.com/anthropics/claude-code/issues)提交問題

### 快取代幣追蹤

使用提示快取時，請分別追蹤這些代幣類型：

```typescript
interface CacheUsage {
  cache_creation_input_tokens: number;
  cache_read_input_tokens: number;
  cache_creation: {
    ephemeral_5m_input_tokens: number;
    ephemeral_1h_input_tokens: number;
  };
}
```

## 最佳實務

1. **使用訊息 ID 進行去重複**：始終追蹤已處理的訊息 ID 以避免重複收費
2. **監控結果訊息**：最終結果包含權威的累積使用量
3. **實作日誌記錄**：記錄所有使用量資料以進行審計和除錯
4. **優雅地處理失敗**：即使對話失敗也要追蹤部分使用量
5. **考慮串流**：對於串流回應，在訊息到達時累積使用量

## 使用量欄位參考

每個使用量物件包含：

- `input_tokens`：處理的基本輸入代幣
- `output_tokens`：在回應中生成的代幣
- `cache_creation_input_tokens`：用於建立快取項目的代幣
- `cache_read_input_tokens`：從快取讀取的代幣
- `service_tier`：使用的服務層級（例如，"standard"）
- `total_cost_usd`：以美元計算的總成本（僅在結果訊息中）

## 範例：建立計費儀表板

以下是如何為計費儀表板聚合使用量資料：

```typescript
class BillingAggregator {
  private userUsage = new Map<string, {
    totalTokens: number;
    totalCost: number;
    conversations: number;
  }>();
  
  async processUserRequest(userId: string, prompt: string) {
    const tracker = new CostTracker();
    const { result, stepUsages, totalCost } = await tracker.trackConversation(prompt);
    
    // 更新用戶總計
    const current = this.userUsage.get(userId) || {
      totalTokens: 0,
      totalCost: 0,
      conversations: 0
    };
    
    const totalTokens = stepUsages.reduce((sum, step) => 
      sum + step.usage.input_tokens + step.usage.output_tokens, 0
    );
    
    this.userUsage.set(userId, {
      totalTokens: current.totalTokens + totalTokens,
      totalCost: current.totalCost + totalCost,
      conversations: current.conversations + 1
    });
    
    return result;
  }
  
  getUserBilling(userId: string) {
    return this.userUsage.get(userId) || {
      totalTokens: 0,
      totalCost: 0,
      conversations: 0
    };
  }
}
```

## 相關文件

- [TypeScript SDK 參考](https://code.claude.com/docs/typescript-sdk-reference) - 完整的 API 文件
- [SDK 概述](/docs/zh-TW/agent-sdk/overview) - SDK 入門指南
- [SDK 權限](/docs/zh-TW/agent-sdk/permissions) - 管理工具權限