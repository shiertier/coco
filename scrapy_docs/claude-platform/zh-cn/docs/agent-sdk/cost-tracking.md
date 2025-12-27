# 跟踪成本和使用情况

了解并跟踪Claude Agent SDK中用于计费的令牌使用情况

---

# SDK成本跟踪

Claude Agent SDK为与Claude的每次交互提供详细的令牌使用信息。本指南解释了如何正确跟踪成本和理解使用情况报告，特别是在处理并行工具使用和多步骤对话时。

有关完整的API文档，请参阅[TypeScript SDK参考](https://code.claude.com/docs/typescript-sdk-reference)。

## 理解令牌使用情况

当Claude处理请求时，它在消息级别报告令牌使用情况。这些使用数据对于跟踪成本和适当地向用户计费至关重要。

### 关键概念

1. **步骤**：步骤是您的应用程序与Claude之间的单个请求/响应对
2. **消息**：步骤内的单个消息（文本、工具使用、工具结果）
3. **使用情况**：附加到助手消息的令牌消耗数据

## 使用情况报告结构

### 单个与并行工具使用

当Claude执行工具时，使用情况报告根据工具是顺序执行还是并行执行而有所不同：

<CodeGroup>

```typescript TypeScript
import { query } from "@anthropic-ai/claude-agent-sdk";

// 示例：在对话中跟踪使用情况
const result = await query({
  prompt: "分析这个代码库并运行测试",
  options: {
    onMessage: (message) => {
      if (message.type === 'assistant' && message.usage) {
        console.log(`消息ID: ${message.id}`);
        console.log(`使用情况:`, message.usage);
      }
    }
  }
});
```

```python Python
from claude_agent_sdk import query, ClaudeAgentOptions, AssistantMessage
import asyncio

# 示例：在对话中跟踪使用情况
async def track_usage():
    # 处理到达的消息
    async for message in query(
        prompt="分析这个代码库并运行测试"
    ):
        if isinstance(message, AssistantMessage) and hasattr(message, 'usage'):
            print(f"消息ID: {message.id}")
            print(f"使用情况: {message.usage}")

asyncio.run(track_usage())
```

</CodeGroup>

### 消息流示例

以下是典型多步骤对话中消息和使用情况的报告方式：

```
<!-- 步骤1：带有并行工具使用的初始请求 -->
assistant (text)      { id: "msg_1", usage: { output_tokens: 100, ... } }
assistant (tool_use)  { id: "msg_1", usage: { output_tokens: 100, ... } }
assistant (tool_use)  { id: "msg_1", usage: { output_tokens: 100, ... } }
assistant (tool_use)  { id: "msg_1", usage: { output_tokens: 100, ... } }
user (tool_result)
user (tool_result)
user (tool_result)

<!-- 步骤2：后续响应 -->
assistant (text)      { id: "msg_2", usage: { output_tokens: 98, ... } }
```

## 重要使用规则

### 1. 相同ID = 相同使用情况

**所有具有相同`id`字段的消息报告相同的使用情况**。当Claude在同一轮中发送多条消息时（例如，文本+工具使用），它们共享相同的消息ID和使用数据。

```typescript
// 所有这些消息都有相同的ID和使用情况
const messages = [
  { type: 'assistant', id: 'msg_123', usage: { output_tokens: 100 } },
  { type: 'assistant', id: 'msg_123', usage: { output_tokens: 100 } },
  { type: 'assistant', id: 'msg_123', usage: { output_tokens: 100 } }
];

// 每个唯一消息ID只收费一次
const uniqueUsage = messages[0].usage; // 对于具有此ID的所有消息都相同
```

### 2. 每步骤收费一次

**您应该只对每个步骤向用户收费一次**，而不是对每个单独的消息收费。当您看到具有相同ID的多条助手消息时，使用其中任何一条的使用情况。

### 3. 结果消息包含累积使用情况

最终的`result`消息包含对话中所有步骤的总累积使用情况：

```typescript
// 最终结果包括总使用情况
const result = await query({
  prompt: "多步骤任务",
  options: { /* ... */ }
});

console.log("总使用情况:", result.usage);
console.log("总成本:", result.usage.total_cost_usd);
```

## 实现：成本跟踪系统

以下是实现成本跟踪系统的完整示例：

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
    // 只处理带有使用情况的助手消息
    if (message.type !== 'assistant' || !message.usage) {
      return;
    }
    
    // 如果我们已经处理过这个消息ID，则跳过
    if (this.processedMessageIds.has(message.id)) {
      return;
    }
    
    // 标记为已处理并记录使用情况
    this.processedMessageIds.add(message.id);
    this.stepUsages.push({
      messageId: message.id,
      timestamp: new Date().toISOString(),
      usage: message.usage,
      costUSD: this.calculateCost(message.usage)
    });
  }
  
  private calculateCost(usage: any): number {
    // 在这里实现您的定价计算
    // 这是一个简化的示例
    const inputCost = usage.input_tokens * 0.00003;
    const outputCost = usage.output_tokens * 0.00015;
    const cacheReadCost = (usage.cache_read_input_tokens || 0) * 0.0000075;
    
    return inputCost + outputCost + cacheReadCost;
  }
}

// 使用方法
const tracker = new CostTracker();
const { result, stepUsages, totalCost } = await tracker.trackConversation(
  "分析并重构这段代码"
);

console.log(`处理的步骤: ${stepUsages.length}`);
console.log(`总成本: $${totalCost.toFixed(4)}`);
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

        # 处理到达的消息
        async for message in query(prompt=prompt):
            self.process_message(message)

            # 捕获最终结果消息
            if isinstance(message, ResultMessage):
                result = message

        return {
            "result": result,
            "step_usages": self.step_usages,
            "total_cost": result.total_cost_usd if result else 0
        }

    def process_message(self, message):
        # 只处理带有使用情况的助手消息
        if not isinstance(message, AssistantMessage) or not hasattr(message, 'usage'):
            return

        # 如果已经处理过这个消息ID，则跳过
        message_id = getattr(message, 'id', None)
        if not message_id or message_id in self.processed_message_ids:
            return

        # 标记为已处理并记录使用情况
        self.processed_message_ids.add(message_id)
        self.step_usages.append({
            "message_id": message_id,
            "timestamp": datetime.now().isoformat(),
            "usage": message.usage,
            "cost_usd": self.calculate_cost(message.usage)
        })

    def calculate_cost(self, usage):
        # 实现您的定价计算
        input_cost = usage.get("input_tokens", 0) * 0.00003
        output_cost = usage.get("output_tokens", 0) * 0.00015
        cache_read_cost = usage.get("cache_read_input_tokens", 0) * 0.0000075

        return input_cost + output_cost + cache_read_cost

# 使用方法
async def main():
    tracker = CostTracker()
    result = await tracker.track_conversation("分析并重构这段代码")

    print(f"处理的步骤: {len(result['step_usages'])}")
    print(f"总成本: ${result['total_cost']:.4f}")

asyncio.run(main())
```

</CodeGroup>

## 处理边缘情况

### 输出令牌差异

在极少数情况下，您可能会观察到具有相同ID的消息的`output_tokens`值不同。当出现这种情况时：

1. **使用最高值** - 组中的最后一条消息通常包含准确的总数
2. **根据总成本验证** - 结果消息中的`total_cost_usd`是权威的
3. **报告不一致** - 在[Claude Code GitHub存储库](https://github.com/anthropics/claude-code/issues)提交问题

### 缓存令牌跟踪

使用提示缓存时，请分别跟踪这些令牌类型：

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

## 最佳实践

1. **使用消息ID进行去重**：始终跟踪已处理的消息ID以避免重复收费
2. **监控结果消息**：最终结果包含权威的累积使用情况
3. **实现日志记录**：记录所有使用数据以进行审计和调试
4. **优雅地处理失败**：即使对话失败也要跟踪部分使用情况
5. **考虑流式传输**：对于流式响应，在消息到达时累积使用情况

## 使用字段参考

每个使用对象包含：

- `input_tokens`：处理的基础输入令牌
- `output_tokens`：响应中生成的令牌
- `cache_creation_input_tokens`：用于创建缓存条目的令牌
- `cache_read_input_tokens`：从缓存读取的令牌
- `service_tier`：使用的服务层级（例如，"standard"）
- `total_cost_usd`：以美元为单位的总成本（仅在结果消息中）

## 示例：构建计费仪表板

以下是如何为计费仪表板聚合使用数据：

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
    
    // 更新用户总计
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

## 相关文档

- [TypeScript SDK参考](https://code.claude.com/docs/typescript-sdk-reference) - 完整的API文档
- [SDK概述](/docs/zh-CN/agent-sdk/overview) - SDK入门
- [SDK权限](/docs/zh-CN/agent-sdk/permissions) - 管理工具权限