# Слэш-команды в SDK

Узнайте, как использовать слэш-команды для управления сессиями Claude Code через SDK

---

Слэш-команды предоставляют способ управления сессиями Claude Code с помощью специальных команд, которые начинаются с `/`. Эти команды могут быть отправлены через SDK для выполнения действий, таких как очистка истории разговора, сжатие сообщений или получение справки.

## Обнаружение доступных слэш-команд

Claude Agent SDK предоставляет информацию о доступных слэш-командах в системном сообщении инициализации. Получите доступ к этой информации при запуске вашей сессии:

<CodeGroup>

```typescript TypeScript
import { query } from "@anthropic-ai/claude-agent-sdk";

for await (const message of query({
  prompt: "Hello Claude",
  options: { maxTurns: 1 }
})) {
  if (message.type === "system" && message.subtype === "init") {
    console.log("Available slash commands:", message.slash_commands);
    // Example output: ["/compact", "/clear", "/help"]
  }
}
```

```python Python
import asyncio
from claude_agent_sdk import query

async def main():
    async for message in query(
        prompt="Hello Claude",
        options={"max_turns": 1}
    ):
        if message.type == "system" and message.subtype == "init":
            print("Available slash commands:", message.slash_commands)
            # Example output: ["/compact", "/clear", "/help"]

asyncio.run(main())
```

</CodeGroup>

## Отправка слэш-команд

Отправляйте слэш-команды, включив их в строку вашего запроса, как обычный текст:

<CodeGroup>

```typescript TypeScript
import { query } from "@anthropic-ai/claude-agent-sdk";

// Send a slash command
for await (const message of query({
  prompt: "/compact",
  options: { maxTurns: 1 }
})) {
  if (message.type === "result") {
    console.log("Command executed:", message.result);
  }
}
```

```python Python
import asyncio
from claude_agent_sdk import query

async def main():
    # Send a slash command
    async for message in query(
        prompt="/compact",
        options={"max_turns": 1}
    ):
        if message.type == "result":
            print("Command executed:", message.result)

asyncio.run(main())
```

</CodeGroup>

## Общие слэш-команды

### `/compact` - Сжатие истории разговора

Команда `/compact` уменьшает размер истории вашего разговора, суммируя старые сообщения, сохраняя при этом важный контекст:

<CodeGroup>

```typescript TypeScript
import { query } from "@anthropic-ai/claude-agent-sdk";

for await (const message of query({
  prompt: "/compact",
  options: { maxTurns: 1 }
})) {
  if (message.type === "system" && message.subtype === "compact_boundary") {
    console.log("Compaction completed");
    console.log("Pre-compaction tokens:", message.compact_metadata.pre_tokens);
    console.log("Trigger:", message.compact_metadata.trigger);
  }
}
```

```python Python
import asyncio
from claude_agent_sdk import query

async def main():
    async for message in query(
        prompt="/compact",
        options={"max_turns": 1}
    ):
        if (message.type == "system" and 
            message.subtype == "compact_boundary"):
            print("Compaction completed")
            print("Pre-compaction tokens:", 
                  message.compact_metadata.pre_tokens)
            print("Trigger:", message.compact_metadata.trigger)

asyncio.run(main())
```

</CodeGroup>

### `/clear` - Очистка разговора

Команда `/clear` начинает новый разговор, очищая всю предыдущую историю:

<CodeGroup>

```typescript TypeScript
import { query } from "@anthropic-ai/claude-agent-sdk";

// Clear conversation and start fresh
for await (const message of query({
  prompt: "/clear",
  options: { maxTurns: 1 }
})) {
  if (message.type === "system" && message.subtype === "init") {
    console.log("Conversation cleared, new session started");
    console.log("Session ID:", message.session_id);
  }
}
```

```python Python
import asyncio
from claude_agent_sdk import query

async def main():
    # Clear conversation and start fresh
    async for message in query(
        prompt="/clear",
        options={"max_turns": 1}
    ):
        if message.type == "system" and message.subtype == "init":
            print("Conversation cleared, new session started")
            print("Session ID:", message.session_id)

asyncio.run(main())
```

</CodeGroup>

## Создание пользовательских слэш-команд

В дополнение к использованию встроенных слэш-команд, вы можете создавать свои собственные пользовательские команды, которые доступны через SDK. Пользовательские команды определяются как markdown файлы в специальных каталогах, аналогично тому, как настраиваются субагенты.

### Расположение файлов

Пользовательские слэш-команды хранятся в специально предназначенных каталогах в зависимости от их области действия:

- **Команды проекта**: `.claude/commands/` - Доступны только в текущем проекте
- **Личные команды**: `~/.claude/commands/` - Доступны во всех ваших проектах

### Формат файла

Каждая пользовательская команда представляет собой markdown файл, где:
- Имя файла (без расширения `.md`) становится именем команды
- Содержимое файла определяет, что делает команда
- Необязательный YAML frontmatter предоставляет конфигурацию

#### Базовый пример

Создайте `.claude/commands/refactor.md`:

```markdown
Refactor the selected code to improve readability and maintainability.
Focus on clean code principles and best practices.
```

Это создает команду `/refactor`, которую вы можете использовать через SDK.

#### С Frontmatter

Создайте `.claude/commands/security-check.md`:

```markdown
---
allowed-tools: Read, Grep, Glob
description: Run security vulnerability scan
model: claude-3-5-sonnet-20241022
---

Analyze the codebase for security vulnerabilities including:
- SQL injection risks
- XSS vulnerabilities
- Exposed credentials
- Insecure configurations
```

### Использование пользовательских команд в SDK

После определения в файловой системе пользовательские команды автоматически становятся доступными через SDK:

<CodeGroup>

```typescript TypeScript
import { query } from "@anthropic-ai/claude-agent-sdk";

// Use a custom command
for await (const message of query({
  prompt: "/refactor src/auth/login.ts",
  options: { maxTurns: 3 }
})) {
  if (message.type === "assistant") {
    console.log("Refactoring suggestions:", message.message);
  }
}

// Custom commands appear in the slash_commands list
for await (const message of query({
  prompt: "Hello",
  options: { maxTurns: 1 }
})) {
  if (message.type === "system" && message.subtype === "init") {
    // Will include both built-in and custom commands
    console.log("Available commands:", message.slash_commands);
    // Example: ["/compact", "/clear", "/help", "/refactor", "/security-check"]
  }
}
```

```python Python
import asyncio
from claude_agent_sdk import query

async def main():
    # Use a custom command
    async for message in query(
        prompt="/refactor src/auth/login.py",
        options={"max_turns": 3}
    ):
        if message.type == "assistant":
            print("Refactoring suggestions:", message.message)
    
    # Custom commands appear in the slash_commands list
    async for message in query(
        prompt="Hello",
        options={"max_turns": 1}
    ):
        if message.type == "system" and message.subtype == "init":
            # Will include both built-in and custom commands
            print("Available commands:", message.slash_commands)
            # Example: ["/compact", "/clear", "/help", "/refactor", "/security-check"]

asyncio.run(main())
```

</CodeGroup>

### Расширенные возможности

#### Аргументы и заполнители

Пользовательские команды поддерживают динамические аргументы с использованием заполнителей:

Создайте `.claude/commands/fix-issue.md`:

```markdown
---
argument-hint: [issue-number] [priority]
description: Fix a GitHub issue
---

Fix issue #$1 with priority $2.
Check the issue description and implement the necessary changes.
```

Используйте в SDK:

<CodeGroup>

```typescript TypeScript
import { query } from "@anthropic-ai/claude-agent-sdk";

// Pass arguments to custom command
for await (const message of query({
  prompt: "/fix-issue 123 high",
  options: { maxTurns: 5 }
})) {
  // Command will process with $1="123" and $2="high"
  if (message.type === "result") {
    console.log("Issue fixed:", message.result);
  }
}
```

```python Python
import asyncio
from claude_agent_sdk import query

async def main():
    # Pass arguments to custom command
    async for message in query(
        prompt="/fix-issue 123 high",
        options={"max_turns": 5}
    ):
        # Command will process with $1="123" and $2="high"
        if message.type == "result":
            print("Issue fixed:", message.result)

asyncio.run(main())
```

</CodeGroup>

#### Выполнение Bash команд

Пользовательские команды могут выполнять bash команды и включать их вывод:

Создайте `.claude/commands/git-commit.md`:

```markdown
---
allowed-tools: Bash(git add:*), Bash(git status:*), Bash(git commit:*)
description: Create a git commit
---

## Context

- Current status: !`git status`
- Current diff: !`git diff HEAD`

## Task

Create a git commit with appropriate message based on the changes.
```

#### Ссылки на файлы

Включайте содержимое файлов, используя префикс `@`:

Создайте `.claude/commands/review-config.md`:

```markdown
---
description: Review configuration files
---

Review the following configuration files for issues:
- Package config: @package.json
- TypeScript config: @tsconfig.json
- Environment config: @.env

Check for security issues, outdated dependencies, and misconfigurations.
```

### Организация с пространствами имен

Организуйте команды в подкаталогах для лучшей структуры:

```bash
.claude/commands/
├── frontend/
│   ├── component.md      # Creates /component (project:frontend)
│   └── style-check.md     # Creates /style-check (project:frontend)
├── backend/
│   ├── api-test.md        # Creates /api-test (project:backend)
│   └── db-migrate.md      # Creates /db-migrate (project:backend)
└── review.md              # Creates /review (project)
```

Подкаталог появляется в описании команды, но не влияет на само имя команды.

### Практические примеры

#### Команда обзора кода

Создайте `.claude/commands/code-review.md`:

```markdown
---
allowed-tools: Read, Grep, Glob, Bash(git diff:*)
description: Comprehensive code review
---

## Changed Files
!`git diff --name-only HEAD~1`

## Detailed Changes
!`git diff HEAD~1`

## Review Checklist

Review the above changes for:
1. Code quality and readability
2. Security vulnerabilities
3. Performance implications
4. Test coverage
5. Documentation completeness

Provide specific, actionable feedback organized by priority.
```

#### Команда запуска тестов

Создайте `.claude/commands/test.md`:

```markdown
---
allowed-tools: Bash, Read, Edit
argument-hint: [test-pattern]
description: Run tests with optional pattern
---

Run tests matching pattern: $ARGUMENTS

1. Detect the test framework (Jest, pytest, etc.)
2. Run tests with the provided pattern
3. If tests fail, analyze and fix them
4. Re-run to verify fixes
```

Используйте эти команды через SDK:

<CodeGroup>

```typescript TypeScript
import { query } from "@anthropic-ai/claude-agent-sdk";

// Run code review
for await (const message of query({
  prompt: "/code-review",
  options: { maxTurns: 3 }
})) {
  // Process review feedback
}

// Run specific tests
for await (const message of query({
  prompt: "/test auth",
  options: { maxTurns: 5 }
})) {
  // Handle test results
}
```

```python Python
import asyncio
from claude_agent_sdk import query

async def main():
    # Run code review
    async for message in query(
        prompt="/code-review",
        options={"max_turns": 3}
    ):
        # Process review feedback
        pass
    
    # Run specific tests
    async for message in query(
        prompt="/test auth",
        options={"max_turns": 5}
    ):
        # Handle test results
        pass

asyncio.run(main())
```

</CodeGroup>

## См. также

- [Слэш-команды](https://code.claude.com/docs/slash-commands) - Полная документация по слэш-командам
- [Субагенты в SDK](/docs/ru/agent-sdk/subagents) - Аналогичная конфигурация на основе файловой системы для субагентов
- [Справочник TypeScript SDK](https://code.claude.com/docs/typescript-sdk-reference) - Полная документация API
- [Обзор SDK](/docs/ru/agent-sdk/overview) - Общие концепции SDK
- [Справочник CLI](https://code.claude.com/docs/cli-reference) - Интерфейс командной строки