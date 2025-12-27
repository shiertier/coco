# Структурированные выходные данные в SDK

Получайте проверенные результаты JSON из рабочих процессов агентов

---

Получайте структурированные, проверенные JSON из рабочих процессов агентов. Agent SDK поддерживает структурированные выходные данные через JSON Schemas, гарантируя, что ваши агенты возвращают данные в точном формате, который вам нужен.

<Note>
**Когда использовать структурированные выходные данные**

Используйте структурированные выходные данные, когда вам нужен проверенный JSON после того, как агент завершит многоходовый рабочий процесс с инструментами (поиск файлов, выполнение команд, веб-исследование и т. д.).

Для одиночных вызовов API без использования инструментов см. [API Structured Outputs](/docs/ru/build-with-claude/structured-outputs).
</Note>

## Почему использовать структурированные выходные данные

Структурированные выходные данные обеспечивают надежную интеграцию, безопасную по типам, с вашими приложениями:

- **Проверенная структура**: Всегда получайте корректный JSON, соответствующий вашей схеме
- **Упрощенная интеграция**: Не требуется код для парсинга или валидации
- **Безопасность типов**: Используйте с подсказками типов TypeScript или Python для полной безопасности
- **Чистое разделение**: Определяйте требования к выходным данным отдельно от инструкций задачи
- **Автономия инструментов**: Агент выбирает, какие инструменты использовать, гарантируя формат выходных данных

<Tabs>
<Tab title="TypeScript">

## Быстрый старт

```typescript
import { query } from '@anthropic-ai/claude-agent-sdk'

const schema = {
  type: 'object',
  properties: {
    company_name: { type: 'string' },
    founded_year: { type: 'number' },
    headquarters: { type: 'string' }
  },
  required: ['company_name']
}

for await (const message of query({
  prompt: 'Research Anthropic and provide key company information',
  options: {
    outputFormat: {
      type: 'json_schema',
      schema: schema
    }
  }
})) {
  if (message.type === 'result' && message.structured_output) {
    console.log(message.structured_output)
    // { company_name: "Anthropic", founded_year: 2021, headquarters: "San Francisco, CA" }
  }
}
```

## Определение схем с помощью Zod

Для проектов TypeScript используйте Zod для определения схемы, безопасного по типам, и валидации:

```typescript
import { z } from 'zod'
import { zodToJsonSchema } from 'zod-to-json-schema'

// Define schema with Zod
const AnalysisResult = z.object({
  summary: z.string(),
  issues: z.array(z.object({
    severity: z.enum(['low', 'medium', 'high']),
    description: z.string(),
    file: z.string()
  })),
  score: z.number().min(0).max(100)
})

type AnalysisResult = z.infer<typeof AnalysisResult>

// Convert to JSON Schema
const schema = zodToJsonSchema(AnalysisResult, { $refStrategy: 'root' })

// Use in query
for await (const message of query({
  prompt: 'Analyze the codebase for security issues',
  options: {
    outputFormat: {
      type: 'json_schema',
      schema: schema
    }
  }
})) {
  if (message.type === 'result' && message.structured_output) {
    // Validate and get fully typed result
    const parsed = AnalysisResult.safeParse(message.structured_output)
    if (parsed.success) {
      const data: AnalysisResult = parsed.data
      console.log(`Score: ${data.score}`)
      console.log(`Found ${data.issues.length} issues`)
      data.issues.forEach(issue => {
        console.log(`[${issue.severity}] ${issue.file}: ${issue.description}`)
      })
    }
  }
}
```

**Преимущества Zod:**
- Полный вывод типов TypeScript
- Валидация во время выполнения с `safeParse()`
- Лучшие сообщения об ошибках
- Компонуемые схемы

</Tab>
<Tab title="Python">

## Быстрый старт

```python
from claude_agent_sdk import query

schema = {
    "type": "object",
    "properties": {
        "company_name": {"type": "string"},
        "founded_year": {"type": "number"},
        "headquarters": {"type": "string"}
    },
    "required": ["company_name"]
}

async for message in query(
    prompt="Research Anthropic and provide key company information",
    options={
        "output_format": {
            "type": "json_schema",
            "schema": schema
        }
    }
):
    if hasattr(message, 'structured_output'):
        print(message.structured_output)
        # {'company_name': 'Anthropic', 'founded_year': 2021, 'headquarters': 'San Francisco, CA'}
```

## Определение схем с помощью Pydantic

Для проектов Python используйте Pydantic для определения схемы, безопасного по типам, и валидации:

```python
from pydantic import BaseModel
from claude_agent_sdk import query

class Issue(BaseModel):
    severity: str  # 'low', 'medium', 'high'
    description: str
    file: str

class AnalysisResult(BaseModel):
    summary: str
    issues: list[Issue]
    score: int

# Use in query
async for message in query(
    prompt="Analyze the codebase for security issues",
    options={
        "output_format": {
            "type": "json_schema",
            "schema": AnalysisResult.model_json_schema()
        }
    }
):
    if hasattr(message, 'structured_output'):
        # Validate and get fully typed result
        result = AnalysisResult.model_validate(message.structured_output)
        print(f"Score: {result.score}")
        print(f"Found {len(result.issues)} issues")
        for issue in result.issues:
            print(f"[{issue.severity}] {issue.file}: {issue.description}")
```

**Преимущества Pydantic:**
- Полные подсказки типов Python
- Валидация во время выполнения с `model_validate()`
- Лучшие сообщения об ошибках
- Функциональность класса данных

</Tab>
</Tabs>

## Как работают структурированные выходные данные

<Steps>
  <Step title="Определите вашу JSON схему">
    Создайте JSON Schema, которая описывает структуру, которую должен вернуть агент. Схема использует стандартный формат JSON Schema.
  </Step>
  <Step title="Добавьте параметр outputFormat">
    Включите параметр `outputFormat` в параметры запроса с `type: "json_schema"` и определением вашей схемы.
  </Step>
  <Step title="Запустите ваш запрос">
    Агент использует любые инструменты, необходимые для выполнения задачи (операции с файлами, команды, веб-поиск и т. д.).
  </Step>
  <Step title="Получите доступ к проверенному выходу">
    Окончательный результат агента будет корректным JSON, соответствующим вашей схеме, доступным в `message.structured_output`.
  </Step>
</Steps>

## Поддерживаемые функции JSON Schema

Agent SDK поддерживает те же функции и ограничения JSON Schema, что и [API Structured Outputs](/docs/ru/build-with-claude/structured-outputs#json-schema-limitations).

Ключевые поддерживаемые функции:
- Все базовые типы: object, array, string, integer, number, boolean, null
- `enum`, `const`, `required`, `additionalProperties` (должно быть `false`)
- Форматы строк: `date-time`, `date`, `email`, `uri`, `uuid` и т. д.
- `$ref`, `$def` и `definitions`

Для полной информации о поддерживаемых функциях, ограничениях и поддержке регулярных выражений см. [JSON Schema limitations](/docs/ru/build-with-claude/structured-outputs#json-schema-limitations) в документации API.

## Пример: агент отслеживания TODO

Вот полный пример, показывающий агента, который ищет TODO в коде и извлекает информацию о git blame:

<CodeGroup>

```typescript TypeScript
import { query } from '@anthropic-ai/claude-agent-sdk'

// Define structure for TODO extraction
const todoSchema = {
  type: 'object',
  properties: {
    todos: {
      type: 'array',
      items: {
        type: 'object',
        properties: {
          text: { type: 'string' },
          file: { type: 'string' },
          line: { type: 'number' },
          author: { type: 'string' },
          date: { type: 'string' }
        },
        required: ['text', 'file', 'line']
      }
    },
    total_count: { type: 'number' }
  },
  required: ['todos', 'total_count']
}

// Agent uses Grep to find TODOs, Bash to get git blame info
for await (const message of query({
  prompt: 'Find all TODO comments in src/ and identify who added them',
  options: {
    outputFormat: {
      type: 'json_schema',
      schema: todoSchema
    }
  }
})) {
  if (message.type === 'result' && message.structured_output) {
    const data = message.structured_output
    console.log(`Found ${data.total_count} TODOs`)
    data.todos.forEach(todo => {
      console.log(`${todo.file}:${todo.line} - ${todo.text}`)
      if (todo.author) {
        console.log(`  Added by ${todo.author} on ${todo.date}`)
      }
    })
  }
}
```

```python Python
from claude_agent_sdk import query

# Define structure for TODO extraction
todo_schema = {
    "type": "object",
    "properties": {
        "todos": {
            "type": "array",
            "items": {
                "type": "object",
                "properties": {
                    "text": {"type": "string"},
                    "file": {"type": "string"},
                    "line": {"type": "number"},
                    "author": {"type": "string"},
                    "date": {"type": "string"}
                },
                "required": ["text", "file", "line"]
            }
        },
        "total_count": {"type": "number"}
    },
    "required": ["todos", "total_count"]
}

# Agent uses Grep to find TODOs, Bash to get git blame info
async for message in query(
    prompt="Find all TODO comments in src/ and identify who added them",
    options={
        "output_format": {
            "type": "json_schema",
            "schema": todo_schema
        }
    }
):
    if hasattr(message, 'structured_output'):
        data = message.structured_output
        print(f"Found {data['total_count']} TODOs")
        for todo in data['todos']:
            print(f"{todo['file']}:{todo['line']} - {todo['text']}")
            if 'author' in todo:
                print(f"  Added by {todo['author']} on {todo['date']}")
```

</CodeGroup>

Агент автономно использует правильные инструменты (Grep, Bash) для сбора информации и возвращает проверенные данные.

## Обработка ошибок

Если агент не может создать корректный выход, соответствующий вашей схеме, вы получите результат ошибки:

```typescript
for await (const msg of query({
  prompt: 'Analyze the data',
  options: {
    outputFormat: {
      type: 'json_schema',
      schema: mySchema
    }
  }
})) {
  if (msg.type === 'result') {
    if (msg.subtype === 'success' && msg.structured_output) {
      console.log(msg.structured_output)
    } else if (msg.subtype === 'error_max_structured_output_retries') {
      console.error('Could not produce valid output')
    }
  }
}
```

## Связанные ресурсы

- [JSON Schema documentation](https://json-schema.org/)
- [API Structured Outputs](/docs/ru/build-with-claude/structured-outputs) - Для одиночных вызовов API
- [Custom tools](/docs/ru/agent-sdk/custom-tools) - Определите инструменты для ваших агентов
- [TypeScript SDK reference](/docs/ru/agent-sdk/typescript) - Полный API TypeScript
- [Python SDK reference](/docs/ru/agent-sdk/python) - Полный API Python