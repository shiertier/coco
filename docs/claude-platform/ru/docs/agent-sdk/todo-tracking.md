# Списки задач

Отслеживайте и отображайте задачи с помощью Claude Agent SDK для организованного управления задачами

---

Отслеживание задач предоставляет структурированный способ управления задачами и отображения прогресса пользователям. Claude Agent SDK включает встроенную функциональность задач, которая помогает организовать сложные рабочие процессы и держать пользователей в курсе прогресса выполнения задач.

### Жизненный цикл задач

Задачи следуют предсказуемому жизненному циклу:
1. **Создаются** как `pending`, когда задачи идентифицированы
2. **Активируются** в `in_progress`, когда начинается работа
3. **Завершаются**, когда задача успешно завершается
4. **Удаляются**, когда все задачи в группе завершены

### Когда используются задачи

SDK автоматически создает задачи для:
- **Сложных многошаговых задач**, требующих 3 или более отдельных действий
- **Предоставленных пользователем списков задач**, когда упоминается несколько элементов
- **Нетривиальных операций**, которые выигрывают от отслеживания прогресса
- **Явных запросов**, когда пользователи просят организацию задач

## Примеры

### Мониторинг изменений задач

<CodeGroup>

```typescript TypeScript
import { query } from "@anthropic-ai/claude-agent-sdk";

for await (const message of query({
  prompt: "Оптимизируй производительность моего React приложения и отслеживай прогресс с задачами",
  options: { maxTurns: 15 }
})) {
  // Обновления задач отражаются в потоке сообщений
  if (message.type === "tool_use" && message.name === "TodoWrite") {
    const todos = message.input.todos;
    
    console.log("Обновление статуса задач:");
    todos.forEach((todo, index) => {
      const status = todo.status === "completed" ? "✅" : 
                    todo.status === "in_progress" ? "🔧" : "❌";
      console.log(`${index + 1}. ${status} ${todo.content}`);
    });
  }
}
```

```python Python
from claude_agent_sdk import query

async for message in query(
    prompt="Оптимизируй производительность моего React приложения и отслеживай прогресс с задачами",
    options={"max_turns": 15}
):
    # Обновления задач отражаются в потоке сообщений
    if message.get("type") == "tool_use" and message.get("name") == "TodoWrite":
        todos = message["input"]["todos"]
        
        print("Обновление статуса задач:")
        for i, todo in enumerate(todos):
            status = "✅" if todo["status"] == "completed" else \
                    "🔧" if todo["status"] == "in_progress" else "❌"
            print(f"{i + 1}. {status} {todo['content']}")
```

</CodeGroup>

### Отображение прогресса в реальном времени

<CodeGroup>

```typescript TypeScript
import { query } from "@anthropic-ai/claude-agent-sdk";

class TodoTracker {
  private todos: any[] = [];
  
  displayProgress() {
    if (this.todos.length === 0) return;
    
    const completed = this.todos.filter(t => t.status === "completed").length;
    const inProgress = this.todos.filter(t => t.status === "in_progress").length;
    const total = this.todos.length;
    
    console.log(`\nПрогресс: ${completed}/${total} завершено`);
    console.log(`Сейчас работаем над: ${inProgress} задач(ей)\n`);
    
    this.todos.forEach((todo, index) => {
      const icon = todo.status === "completed" ? "✅" : 
                  todo.status === "in_progress" ? "🔧" : "❌";
      const text = todo.status === "in_progress" ? todo.activeForm : todo.content;
      console.log(`${index + 1}. ${icon} ${text}`);
    });
  }
  
  async trackQuery(prompt: string) {
    for await (const message of query({
      prompt,
      options: { maxTurns: 20 }
    })) {
      if (message.type === "tool_use" && message.name === "TodoWrite") {
        this.todos = message.input.todos;
        this.displayProgress();
      }
    }
  }
}

// Использование
const tracker = new TodoTracker();
await tracker.trackQuery("Создай полную систему аутентификации с задачами");
```

```python Python
from claude_agent_sdk import query
from typing import List, Dict

class TodoTracker:
    def __init__(self):
        self.todos: List[Dict] = []
    
    def display_progress(self):
        if not self.todos:
            return
        
        completed = len([t for t in self.todos if t["status"] == "completed"])
        in_progress = len([t for t in self.todos if t["status"] == "in_progress"])
        total = len(self.todos)
        
        print(f"\nПрогресс: {completed}/{total} завершено")
        print(f"Сейчас работаем над: {in_progress} задач(ей)\n")
        
        for i, todo in enumerate(self.todos):
            icon = "✅" if todo["status"] == "completed" else \
                  "🔧" if todo["status"] == "in_progress" else "❌"
            text = todo["activeForm"] if todo["status"] == "in_progress" else todo["content"]
            print(f"{i + 1}. {icon} {text}")
    
    async def track_query(self, prompt: str):
        async for message in query(
            prompt=prompt,
            options={"max_turns": 20}
        ):
            if message.get("type") == "tool_use" and message.get("name") == "TodoWrite":
                self.todos = message["input"]["todos"]
                self.display_progress()

# Использование
tracker = TodoTracker()
await tracker.track_query("Создай полную систему аутентификации с задачами")
```

</CodeGroup>

## Связанная документация

- [Справочник TypeScript SDK](/docs/ru/agent-sdk/typescript)
- [Справочник Python SDK](/docs/ru/agent-sdk/python) 
- [Потоковый режим против одиночного режима](/docs/ru/agent-sdk/streaming-vs-single-mode)
- [Пользовательские инструменты](/docs/ru/agent-sdk/custom-tools)