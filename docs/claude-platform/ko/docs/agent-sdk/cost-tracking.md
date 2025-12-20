# 비용 및 사용량 추적

Claude Agent SDK에서 청구를 위한 토큰 사용량을 이해하고 추적하기

---

# SDK 비용 추적

Claude Agent SDK는 Claude와의 각 상호작용에 대한 상세한 토큰 사용량 정보를 제공합니다. 이 가이드는 특히 병렬 도구 사용 및 다단계 대화를 다룰 때 비용을 적절히 추적하고 사용량 보고를 이해하는 방법을 설명합니다.

완전한 API 문서는 [TypeScript SDK 참조](https://code.claude.com/docs/typescript-sdk-reference)를 참조하세요.

## 토큰 사용량 이해

Claude가 요청을 처리할 때, 메시지 수준에서 토큰 사용량을 보고합니다. 이 사용량 데이터는 비용을 추적하고 사용자에게 적절히 청구하는 데 필수적입니다.

### 핵심 개념

1. **단계**: 단계는 애플리케이션과 Claude 간의 단일 요청/응답 쌍입니다
2. **메시지**: 단계 내의 개별 메시지(텍스트, 도구 사용, 도구 결과)
3. **사용량**: 어시스턴트 메시지에 첨부된 토큰 소비 데이터

## 사용량 보고 구조

### 단일 vs 병렬 도구 사용

Claude가 도구를 실행할 때, 도구가 순차적으로 실행되는지 병렬로 실행되는지에 따라 사용량 보고가 달라집니다:

<CodeGroup>

```typescript TypeScript
import { query } from "@anthropic-ai/claude-agent-sdk";

// 예제: 대화에서 사용량 추적
const result = await query({
  prompt: "이 코드베이스를 분석하고 테스트를 실행해",
  options: {
    onMessage: (message) => {
      if (message.type === 'assistant' && message.usage) {
        console.log(`메시지 ID: ${message.id}`);
        console.log(`사용량:`, message.usage);
      }
    }
  }
});
```

```python Python
from claude_agent_sdk import query, ClaudeAgentOptions, AssistantMessage
import asyncio

# 예제: 대화에서 사용량 추적
async def track_usage():
    # 메시지가 도착하는 대로 처리
    async for message in query(
        prompt="이 코드베이스를 분석하고 테스트를 실행해"
    ):
        if isinstance(message, AssistantMessage) and hasattr(message, 'usage'):
            print(f"메시지 ID: {message.id}")
            print(f"사용량: {message.usage}")

asyncio.run(track_usage())
```

</CodeGroup>

### 메시지 흐름 예제

다음은 일반적인 다단계 대화에서 메시지와 사용량이 보고되는 방식입니다:

```
<!-- 단계 1: 병렬 도구 사용이 포함된 초기 요청 -->
assistant (text)      { id: "msg_1", usage: { output_tokens: 100, ... } }
assistant (tool_use)  { id: "msg_1", usage: { output_tokens: 100, ... } }
assistant (tool_use)  { id: "msg_1", usage: { output_tokens: 100, ... } }
assistant (tool_use)  { id: "msg_1", usage: { output_tokens: 100, ... } }
user (tool_result)
user (tool_result)
user (tool_result)

<!-- 단계 2: 후속 응답 -->
assistant (text)      { id: "msg_2", usage: { output_tokens: 98, ... } }
```

## 중요한 사용량 규칙

### 1. 동일한 ID = 동일한 사용량

**동일한 `id` 필드를 가진 모든 메시지는 동일한 사용량을 보고합니다**. Claude가 동일한 턴에서 여러 메시지를 보낼 때(예: 텍스트 + 도구 사용), 이들은 동일한 메시지 ID와 사용량 데이터를 공유합니다.

```typescript
// 이 모든 메시지는 동일한 ID와 사용량을 가집니다
const messages = [
  { type: 'assistant', id: 'msg_123', usage: { output_tokens: 100 } },
  { type: 'assistant', id: 'msg_123', usage: { output_tokens: 100 } },
  { type: 'assistant', id: 'msg_123', usage: { output_tokens: 100 } }
];

// 고유한 메시지 ID당 한 번만 청구
const uniqueUsage = messages[0].usage; // 이 ID를 가진 모든 메시지에 대해 동일
```

### 2. 단계당 한 번 청구

**사용자에게는 단계당 한 번만 청구해야 하며**, 개별 메시지마다 청구하지 않습니다. 동일한 ID를 가진 여러 어시스턴트 메시지를 볼 때, 그 중 하나의 사용량을 사용하세요.

### 3. 결과 메시지에는 누적 사용량 포함

최종 `result` 메시지에는 대화의 모든 단계에서의 총 누적 사용량이 포함됩니다:

```typescript
// 최종 결과에는 총 사용량이 포함됩니다
const result = await query({
  prompt: "다단계 작업",
  options: { /* ... */ }
});

console.log("총 사용량:", result.usage);
console.log("총 비용:", result.usage.total_cost_usd);
```

## 구현: 비용 추적 시스템

다음은 비용 추적 시스템을 구현하는 완전한 예제입니다:

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
    // 사용량이 있는 어시스턴트 메시지만 처리
    if (message.type !== 'assistant' || !message.usage) {
      return;
    }
    
    // 이미 처리한 메시지 ID인 경우 건너뛰기
    if (this.processedMessageIds.has(message.id)) {
      return;
    }
    
    // 처리됨으로 표시하고 사용량 기록
    this.processedMessageIds.add(message.id);
    this.stepUsages.push({
      messageId: message.id,
      timestamp: new Date().toISOString(),
      usage: message.usage,
      costUSD: this.calculateCost(message.usage)
    });
  }
  
  private calculateCost(usage: any): number {
    // 여기에 가격 계산을 구현하세요
    // 이것은 단순화된 예제입니다
    const inputCost = usage.input_tokens * 0.00003;
    const outputCost = usage.output_tokens * 0.00015;
    const cacheReadCost = (usage.cache_read_input_tokens || 0) * 0.0000075;
    
    return inputCost + outputCost + cacheReadCost;
  }
}

// 사용법
const tracker = new CostTracker();
const { result, stepUsages, totalCost } = await tracker.trackConversation(
  "이 코드를 분석하고 리팩토링해"
);

console.log(`처리된 단계: ${stepUsages.length}`);
console.log(`총 비용: $${totalCost.toFixed(4)}`);
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

        # 메시지가 도착하는 대로 처리
        async for message in query(prompt=prompt):
            self.process_message(message)

            # 최종 결과 메시지 캡처
            if isinstance(message, ResultMessage):
                result = message

        return {
            "result": result,
            "step_usages": self.step_usages,
            "total_cost": result.total_cost_usd if result else 0
        }

    def process_message(self, message):
        # 사용량이 있는 어시스턴트 메시지만 처리
        if not isinstance(message, AssistantMessage) or not hasattr(message, 'usage'):
            return

        # 이미 처리한 메시지 ID인 경우 건너뛰기
        message_id = getattr(message, 'id', None)
        if not message_id or message_id in self.processed_message_ids:
            return

        # 처리됨으로 표시하고 사용량 기록
        self.processed_message_ids.add(message_id)
        self.step_usages.append({
            "message_id": message_id,
            "timestamp": datetime.now().isoformat(),
            "usage": message.usage,
            "cost_usd": self.calculate_cost(message.usage)
        })

    def calculate_cost(self, usage):
        # 가격 계산 구현
        input_cost = usage.get("input_tokens", 0) * 0.00003
        output_cost = usage.get("output_tokens", 0) * 0.00015
        cache_read_cost = usage.get("cache_read_input_tokens", 0) * 0.0000075

        return input_cost + output_cost + cache_read_cost

# 사용법
async def main():
    tracker = CostTracker()
    result = await tracker.track_conversation("이 코드를 분석하고 리팩토링해")

    print(f"처리된 단계: {len(result['step_usages'])}")
    print(f"총 비용: ${result['total_cost']:.4f}")

asyncio.run(main())
```

</CodeGroup>

## 예외 상황 처리

### 출력 토큰 불일치

드문 경우지만, 동일한 ID를 가진 메시지에 대해 다른 `output_tokens` 값을 관찰할 수 있습니다. 이런 경우:

1. **가장 높은 값 사용** - 그룹의 마지막 메시지가 일반적으로 정확한 총계를 포함합니다
2. **총 비용과 대조 확인** - 결과 메시지의 `total_cost_usd`가 권위 있는 값입니다
3. **불일치 보고** - [Claude Code GitHub 저장소](https://github.com/anthropics/claude-code/issues)에 이슈를 제출하세요

### 캐시 토큰 추적

프롬프트 캐싱을 사용할 때, 이러한 토큰 유형을 별도로 추적하세요:

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

## 모범 사례

1. **중복 제거를 위해 메시지 ID 사용**: 이중 청구를 피하기 위해 항상 처리된 메시지 ID를 추적하세요
2. **결과 메시지 모니터링**: 최종 결과에는 권위 있는 누적 사용량이 포함됩니다
3. **로깅 구현**: 감사 및 디버깅을 위해 모든 사용량 데이터를 로그하세요
4. **실패를 우아하게 처리**: 대화가 실패하더라도 부분 사용량을 추적하세요
5. **스트리밍 고려**: 스트리밍 응답의 경우, 메시지가 도착하는 대로 사용량을 누적하세요

## 사용량 필드 참조

각 사용량 객체에는 다음이 포함됩니다:

- `input_tokens`: 처리된 기본 입력 토큰
- `output_tokens`: 응답에서 생성된 토큰
- `cache_creation_input_tokens`: 캐시 항목을 생성하는 데 사용된 토큰
- `cache_read_input_tokens`: 캐시에서 읽은 토큰
- `service_tier`: 사용된 서비스 계층(예: "standard")
- `total_cost_usd`: USD 총 비용(결과 메시지에만 포함)

## 예제: 청구 대시보드 구축

청구 대시보드를 위해 사용량 데이터를 집계하는 방법입니다:

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
    
    // 사용자 총계 업데이트
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

## 관련 문서

- [TypeScript SDK 참조](https://code.claude.com/docs/typescript-sdk-reference) - 완전한 API 문서
- [SDK 개요](/docs/ko/agent-sdk/overview) - SDK 시작하기
- [SDK 권한](/docs/ko/agent-sdk/permissions) - 도구 권한 관리