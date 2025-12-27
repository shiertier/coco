# Управление сеансами

Понимание того, как Claude Agent SDK обрабатывает сеансы и возобновление сеансов

---

# Управление сеансами

Claude Agent SDK предоставляет возможности управления сеансами для обработки состояния разговора и возобновления. Сеансы позволяют вам продолжать разговоры в нескольких взаимодействиях, сохраняя полный контекст.

## Как работают сеансы

Когда вы начинаете новый запрос, SDK автоматически создает сеанс и возвращает ID сеанса в начальном системном сообщении. Вы можете захватить этот ID, чтобы возобновить сеанс позже.

### Получение ID сеанса

<CodeGroup>

```typescript TypeScript
import { query } from "@anthropic-ai/claude-agent-sdk"

let sessionId: string | undefined

const response = query({
  prompt: "Help me build a web application",
  options: {
    model: "claude-sonnet-4-5"
  }
})

for await (const message of response) {
  // The first message is a system init message with the session ID
  if (message.type === 'system' && message.subtype === 'init') {
    sessionId = message.session_id
    console.log(`Session started with ID: ${sessionId}`)
    // You can save this ID for later resumption
  }

  // Process other messages...
  console.log(message)
}

// Later, you can use the saved sessionId to resume
if (sessionId) {
  const resumedResponse = query({
    prompt: "Continue where we left off",
    options: {
      resume: sessionId
    }
  })
}
```

```python Python
from claude_agent_sdk import query, ClaudeAgentOptions

session_id = None

async for message in query(
    prompt="Help me build a web application",
    options=ClaudeAgentOptions(
        model="claude-sonnet-4-5"
    )
):
    # The first message is a system init message with the session ID
    if hasattr(message, 'subtype') and message.subtype == 'init':
        session_id = message.data.get('session_id')
        print(f"Session started with ID: {session_id}")
        # You can save this ID for later resumption

    # Process other messages...
    print(message)

# Later, you can use the saved session_id to resume
if session_id:
    async for message in query(
        prompt="Continue where we left off",
        options=ClaudeAgentOptions(
            resume=session_id
        )
    ):
        print(message)
```

</CodeGroup>

## Возобновление сеансов

SDK поддерживает возобновление сеансов из предыдущих состояний разговора, обеспечивая непрерывные рабочие процессы разработки. Используйте опцию `resume` с ID сеанса, чтобы продолжить предыдущий разговор.

<CodeGroup>

```typescript TypeScript
import { query } from "@anthropic-ai/claude-agent-sdk"

// Resume a previous session using its ID
const response = query({
  prompt: "Continue implementing the authentication system from where we left off",
  options: {
    resume: "session-xyz", // Session ID from previous conversation
    model: "claude-sonnet-4-5",
    allowedTools: ["Read", "Edit", "Write", "Glob", "Grep", "Bash"]
  }
})

// The conversation continues with full context from the previous session
for await (const message of response) {
  console.log(message)
}
```

```python Python
from claude_agent_sdk import query, ClaudeAgentOptions

# Resume a previous session using its ID
async for message in query(
    prompt="Continue implementing the authentication system from where we left off",
    options=ClaudeAgentOptions(
        resume="session-xyz",  # Session ID from previous conversation
        model="claude-sonnet-4-5",
        allowed_tools=["Read", "Edit", "Write", "Glob", "Grep", "Bash"]
    )
):
    print(message)

# The conversation continues with full context from the previous session
```

</CodeGroup>

SDK автоматически обрабатывает загрузку истории разговора и контекста при возобновлении сеанса, позволяя Claude продолжить ровно с того места, где он остановился.

<Tip>
Для отслеживания и отката изменений файлов в сеансах см. [File Checkpointing](/docs/ru/agent-sdk/file-checkpointing).
</Tip>

## Разветвление сеансов

При возобновлении сеанса вы можете выбрать либо продолжить исходный сеанс, либо разветвить его в новую ветвь. По умолчанию возобновление продолжает исходный сеанс. Используйте опцию `forkSession` (TypeScript) или `fork_session` (Python) для создания нового ID сеанса, который начинается с возобновленного состояния.

### Когда разветвлять сеанс

Разветвление полезно, когда вы хотите:
- Исследовать различные подходы с одной и той же начальной точки
- Создать несколько ветвей разговора без изменения оригинала
- Протестировать изменения без влияния на исходную историю сеанса
- Поддерживать отдельные пути разговора для различных экспериментов

### Разветвление в сравнении с продолжением

| Поведение | `forkSession: false` (по умолчанию) | `forkSession: true` |
|----------|-------------------------------|---------------------|
| **ID сеанса** | Такой же, как оригинальный | Генерируется новый ID сеанса |
| **История** | Добавляется к исходному сеансу | Создает новую ветвь с точки возобновления |
| **Исходный сеанс** | Изменен | Сохранен без изменений |
| **Вариант использования** | Продолжить линейный разговор | Разветвить для исследования альтернатив |

### Пример: Разветвление сеанса

<CodeGroup>

```typescript TypeScript
import { query } from "@anthropic-ai/claude-agent-sdk"

// First, capture the session ID
let sessionId: string | undefined

const response = query({
  prompt: "Help me design a REST API",
  options: { model: "claude-sonnet-4-5" }
})

for await (const message of response) {
  if (message.type === 'system' && message.subtype === 'init') {
    sessionId = message.session_id
    console.log(`Original session: ${sessionId}`)
  }
}

// Fork the session to try a different approach
const forkedResponse = query({
  prompt: "Now let's redesign this as a GraphQL API instead",
  options: {
    resume: sessionId,
    forkSession: true,  // Creates a new session ID
    model: "claude-sonnet-4-5"
  }
})

for await (const message of forkedResponse) {
  if (message.type === 'system' && message.subtype === 'init') {
    console.log(`Forked session: ${message.session_id}`)
    // This will be a different session ID
  }
}

// The original session remains unchanged and can still be resumed
const originalContinued = query({
  prompt: "Add authentication to the REST API",
  options: {
    resume: sessionId,
    forkSession: false,  // Continue original session (default)
    model: "claude-sonnet-4-5"
  }
})
```

```python Python
from claude_agent_sdk import query, ClaudeAgentOptions

# First, capture the session ID
session_id = None

async for message in query(
    prompt="Help me design a REST API",
    options=ClaudeAgentOptions(model="claude-sonnet-4-5")
):
    if hasattr(message, 'subtype') and message.subtype == 'init':
        session_id = message.data.get('session_id')
        print(f"Original session: {session_id}")

# Fork the session to try a different approach
async for message in query(
    prompt="Now let's redesign this as a GraphQL API instead",
    options=ClaudeAgentOptions(
        resume=session_id,
        fork_session=True,  # Creates a new session ID
        model="claude-sonnet-4-5"
    )
):
    if hasattr(message, 'subtype') and message.subtype == 'init':
        forked_id = message.data.get('session_id')
        print(f"Forked session: {forked_id}")
        # This will be a different session ID

# The original session remains unchanged and can still be resumed
async for message in query(
    prompt="Add authentication to the REST API",
    options=ClaudeAgentOptions(
        resume=session_id,
        fork_session=False,  # Continue original session (default)
        model="claude-sonnet-4-5"
    )
):
    print(message)
```

</CodeGroup>