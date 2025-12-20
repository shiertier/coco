# Отслеживание затрат и использования

Понимание и отслеживание использования токенов для выставления счетов в Claude Agent SDK

---

# Отслеживание затрат SDK

Claude Agent SDK предоставляет подробную информацию об использовании токенов для каждого взаимодействия с Claude. Это руководство объясняет, как правильно отслеживать затраты и понимать отчеты об использовании, особенно при работе с параллельным использованием инструментов и многошаговыми разговорами.

Для полной документации API см. [справочник TypeScript SDK](https://code.claude.com/docs/typescript-sdk-reference).

## Понимание использования токенов

Когда Claude обрабатывает запросы, он сообщает об использовании токенов на уровне сообщений. Эти данные об использовании необходимы для отслеживания затрат и правильного выставления счетов пользователям.

### Ключевые концепции

1. **Шаги**: Шаг - это одна пара запрос/ответ между вашим приложением и Claude
2. **Сообщения**: Отдельные сообщения в рамках шага (текст, использование инструментов, результаты инструментов)
3. **Использование**: Данные о потреблении токенов, прикрепленные к сообщениям ассистента

## Структура отчетности об использовании

### Одиночное против параллельного использования инструментов

Когда Claude выполняет инструменты, отчетность об использовании различается в зависимости от того, выполняются ли инструменты последовательно или параллельно:

<CodeGroup>

```typescript TypeScript
import { query } from "@anthropic-ai/claude-agent-sdk";

// Пример: Отслеживание использования в разговоре
const result = await query({
  prompt: "Проанализируй эту кодовую базу и запусти тесты",
  options: {
    onMessage: (message) => {
      if (message.type === 'assistant' && message.usage) {
        console.log(`ID сообщения: ${message.id}`);
        console.log(`Использование:`, message.usage);
      }
    }
  }
});
```

```python Python
from claude_agent_sdk import query, ClaudeAgentOptions, AssistantMessage
import asyncio

# Пример: Отслеживание использования в разговоре
async def track_usage():
    # Обрабатываем сообщения по мере их поступления
    async for message in query(
        prompt="Проанализируй эту кодовую базу и запусти тесты"
    ):
        if isinstance(message, AssistantMessage) and hasattr(message, 'usage'):
            print(f"ID сообщения: {message.id}")
            print(f"Использование: {message.usage}")

asyncio.run(track_usage())
```

</CodeGroup>

### Пример потока сообщений

Вот как сообщения и использование сообщаются в типичном многошаговом разговоре:

```
<!-- Шаг 1: Начальный запрос с параллельным использованием инструментов -->
assistant (text)      { id: "msg_1", usage: { output_tokens: 100, ... } }
assistant (tool_use)  { id: "msg_1", usage: { output_tokens: 100, ... } }
assistant (tool_use)  { id: "msg_1", usage: { output_tokens: 100, ... } }
assistant (tool_use)  { id: "msg_1", usage: { output_tokens: 100, ... } }
user (tool_result)
user (tool_result)
user (tool_result)

<!-- Шаг 2: Последующий ответ -->
assistant (text)      { id: "msg_2", usage: { output_tokens: 98, ... } }
```

## Важные правила использования

### 1. Одинаковый ID = Одинаковое использование

**Все сообщения с одинаковым полем `id` сообщают об идентичном использовании**. Когда Claude отправляет несколько сообщений в одном ходе (например, текст + использование инструментов), они имеют один и тот же ID сообщения и данные об использовании.

```typescript
// Все эти сообщения имеют одинаковый ID и использование
const messages = [
  { type: 'assistant', id: 'msg_123', usage: { output_tokens: 100 } },
  { type: 'assistant', id: 'msg_123', usage: { output_tokens: 100 } },
  { type: 'assistant', id: 'msg_123', usage: { output_tokens: 100 } }
];

// Взимайте плату только один раз за уникальный ID сообщения
const uniqueUsage = messages[0].usage; // Одинаково для всех сообщений с этим ID
```

### 2. Взимайте плату один раз за шаг

**Вы должны взимать плату с пользователей только один раз за шаг**, а не за каждое отдельное сообщение. Когда вы видите несколько сообщений ассистента с одинаковым ID, используйте использование из любого одного из них.

### 3. Сообщение результата содержит кумулятивное использование

Финальное сообщение `result` содержит общее кумулятивное использование из всех шагов в разговоре:

```typescript
// Финальный результат включает общее использование
const result = await query({
  prompt: "Многошаговая задача",
  options: { /* ... */ }
});

console.log("Общее использование:", result.usage);
console.log("Общая стоимость:", result.usage.total_cost_usd);
```

## Реализация: Система отслеживания затрат

Вот полный пример реализации системы отслеживания затрат:

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
    // Обрабатываем только сообщения ассистента с использованием
    if (message.type !== 'assistant' || !message.usage) {
      return;
    }
    
    // Пропускаем, если мы уже обработали этот ID сообщения
    if (this.processedMessageIds.has(message.id)) {
      return;
    }
    
    // Отмечаем как обработанное и записываем использование
    this.processedMessageIds.add(message.id);
    this.stepUsages.push({
      messageId: message.id,
      timestamp: new Date().toISOString(),
      usage: message.usage,
      costUSD: this.calculateCost(message.usage)
    });
  }
  
  private calculateCost(usage: any): number {
    // Реализуйте здесь расчет цены
    // Это упрощенный пример
    const inputCost = usage.input_tokens * 0.00003;
    const outputCost = usage.output_tokens * 0.00015;
    const cacheReadCost = (usage.cache_read_input_tokens || 0) * 0.0000075;
    
    return inputCost + outputCost + cacheReadCost;
  }
}

// Использование
const tracker = new CostTracker();
const { result, stepUsages, totalCost } = await tracker.trackConversation(
  "Проанализируй и рефактори этот код"
);

console.log(`Обработано шагов: ${stepUsages.length}`);
console.log(`Общая стоимость: $${totalCost.toFixed(4)}`);
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

        # Обрабатываем сообщения по мере их поступления
        async for message in query(prompt=prompt):
            self.process_message(message)

            # Захватываем финальное сообщение результата
            if isinstance(message, ResultMessage):
                result = message

        return {
            "result": result,
            "step_usages": self.step_usages,
            "total_cost": result.total_cost_usd if result else 0
        }

    def process_message(self, message):
        # Обрабатываем только сообщения ассистента с использованием
        if not isinstance(message, AssistantMessage) or not hasattr(message, 'usage'):
            return

        # Пропускаем, если уже обработали этот ID сообщения
        message_id = getattr(message, 'id', None)
        if not message_id or message_id in self.processed_message_ids:
            return

        # Отмечаем как обработанное и записываем использование
        self.processed_message_ids.add(message_id)
        self.step_usages.append({
            "message_id": message_id,
            "timestamp": datetime.now().isoformat(),
            "usage": message.usage,
            "cost_usd": self.calculate_cost(message.usage)
        })

    def calculate_cost(self, usage):
        # Реализуйте расчет цены
        input_cost = usage.get("input_tokens", 0) * 0.00003
        output_cost = usage.get("output_tokens", 0) * 0.00015
        cache_read_cost = usage.get("cache_read_input_tokens", 0) * 0.0000075

        return input_cost + output_cost + cache_read_cost

# Использование
async def main():
    tracker = CostTracker()
    result = await tracker.track_conversation("Проанализируй и рефактори этот код")

    print(f"Обработано шагов: {len(result['step_usages'])}")
    print(f"Общая стоимость: ${result['total_cost']:.4f}")

asyncio.run(main())
```

</CodeGroup>

## Обработка крайних случаев

### Расхождения в выходных токенах

В редких случаях вы можете наблюдать разные значения `output_tokens` для сообщений с одинаковым ID. Когда это происходит:

1. **Используйте наибольшее значение** - Финальное сообщение в группе обычно содержит точную сумму
2. **Проверяйте против общей стоимости** - `total_cost_usd` в сообщении результата является авторитетным
3. **Сообщайте о несоответствиях** - Подавайте проблемы в [репозиторий Claude Code GitHub](https://github.com/anthropics/claude-code/issues)

### Отслеживание токенов кэша

При использовании кэширования промптов отслеживайте эти типы токенов отдельно:

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

## Лучшие практики

1. **Используйте ID сообщений для дедупликации**: Всегда отслеживайте обработанные ID сообщений, чтобы избежать двойного взимания платы
2. **Мониторьте сообщение результата**: Финальный результат содержит авторитетное кумулятивное использование
3. **Реализуйте логирование**: Логируйте все данные об использовании для аудита и отладки
4. **Обрабатывайте сбои грациозно**: Отслеживайте частичное использование даже если разговор завершается неудачей
5. **Рассмотрите стриминг**: Для потоковых ответов накапливайте использование по мере поступления сообщений

## Справочник полей использования

Каждый объект использования содержит:

- `input_tokens`: Базовые входные токены, обработанные
- `output_tokens`: Токены, сгенерированные в ответе
- `cache_creation_input_tokens`: Токены, используемые для создания записей кэша
- `cache_read_input_tokens`: Токены, прочитанные из кэша
- `service_tier`: Используемый уровень сервиса (например, "standard")
- `total_cost_usd`: Общая стоимость в долларах США (только в сообщении результата)

## Пример: Создание панели выставления счетов

Вот как агрегировать данные об использовании для панели выставления счетов:

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
    
    // Обновляем итоги пользователя
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

## Связанная документация

- [Справочник TypeScript SDK](https://code.claude.com/docs/typescript-sdk-reference) - Полная документация API
- [Обзор SDK](/docs/ru/agent-sdk/overview) - Начало работы с SDK
- [Разрешения SDK](/docs/ru/agent-sdk/permissions) - Управление разрешениями инструментов