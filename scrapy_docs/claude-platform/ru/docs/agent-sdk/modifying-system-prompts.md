# Изменение системных промптов

Узнайте, как настроить поведение Claude, изменяя системные промпты с помощью трех подходов - стили вывода, systemPrompt с append и пользовательские системные промпты.

---

Системные промпты определяют поведение Claude, возможности и стиль ответов. Claude Agent SDK предоставляет три способа настройки системных промптов: использование стилей вывода (постоянные конфигурации на основе файлов), добавление к промпту Claude Code или использование полностью пользовательского промпта.

## Понимание системных промптов

Системный промпт - это начальный набор инструкций, который формирует поведение Claude на протяжении всего разговора.

<Note>
**Поведение по умолчанию:** Agent SDK использует **пустой системный промпт** по умолчанию для максимальной гибкости. Чтобы использовать системный промпт Claude Code (инструкции по инструментам, руководящие принципы кода и т.д.), укажите `systemPrompt: { preset: "claude_code" }` в TypeScript или `system_prompt="claude_code"` в Python.
</Note>

Системный промпт Claude Code включает:

- Инструкции по использованию инструментов и доступные инструменты
- Руководящие принципы стиля и форматирования кода
- Настройки тона ответа и многословности
- Инструкции по безопасности и защите
- Контекст о текущем рабочем каталоге и окружении

## Методы изменения

### Метод 1: Файлы CLAUDE.md (инструкции на уровне проекта)

Файлы CLAUDE.md предоставляют специфичный для проекта контекст и инструкции, которые автоматически читаются Agent SDK при запуске в каталоге. Они служат постоянной "памятью" для вашего проекта.

#### Как CLAUDE.md работает с SDK

**Расположение и обнаружение:**

- **Уровень проекта:** `CLAUDE.md` или `.claude/CLAUDE.md` в вашем рабочем каталоге
- **Уровень пользователя:** `~/.claude/CLAUDE.md` для глобальных инструкций во всех проектах

**ВАЖНО:** SDK читает файлы CLAUDE.md только когда вы явно настраиваете `settingSources` (TypeScript) или `setting_sources` (Python):

- Включите `'project'` для загрузки CLAUDE.md на уровне проекта
- Включите `'user'` для загрузки CLAUDE.md на уровне пользователя (`~/.claude/CLAUDE.md`)

Пресет системного промпта `claude_code` НЕ загружает автоматически CLAUDE.md - вы также должны указать источники настроек.

**Формат содержимого:**
Файлы CLAUDE.md используют обычный markdown и могут содержать:

- Руководящие принципы и стандарты кодирования
- Специфичный для проекта контекст
- Общие команды или рабочие процессы
- Соглашения API
- Требования к тестированию

#### Пример CLAUDE.md

```markdown
# Руководящие принципы проекта

## Стиль кода

- Используйте строгий режим TypeScript
- Предпочитайте функциональные компоненты в React
- Всегда включайте комментарии JSDoc для публичных API

## Тестирование

- Запускайте `npm test` перед коммитом
- Поддерживайте >80% покрытия кода
- Используйте jest для модульных тестов, playwright для E2E

## Команды

- Сборка: `npm run build`
- Dev сервер: `npm run dev`
- Проверка типов: `npm run typecheck`
```

#### Использование CLAUDE.md с SDK

<CodeGroup>

```typescript TypeScript
import { query } from "@anthropic-ai/claude-agent-sdk";

// ВАЖНО: Вы должны указать settingSources для загрузки CLAUDE.md
// Пресет claude_code сам по себе НЕ загружает файлы CLAUDE.md
const messages = [];

for await (const message of query({
  prompt: "Добавь новый React компонент для профилей пользователей",
  options: {
    systemPrompt: {
      type: "preset",
      preset: "claude_code", // Используй системный промпт Claude Code
    },
    settingSources: ["project"], // Требуется для загрузки CLAUDE.md из проекта
  },
})) {
  messages.push(message);
}

// Теперь Claude имеет доступ к руководящим принципам вашего проекта из CLAUDE.md
```

```python Python
from claude_agent_sdk import query, ClaudeAgentOptions

# ВАЖНО: Вы должны указать setting_sources для загрузки CLAUDE.md
# Пресет claude_code сам по себе НЕ загружает файлы CLAUDE.md
messages = []

async for message in query(
    prompt="Добавь новый React компонент для профилей пользователей",
    options=ClaudeAgentOptions(
        system_prompt={
            "type": "preset",
            "preset": "claude_code"  # Используй системный промпт Claude Code
        },
        setting_sources=["project"]  # Требуется для загрузки CLAUDE.md из проекта
    )
):
    messages.append(message)

# Теперь Claude имеет доступ к руководящим принципам вашего проекта из CLAUDE.md
```

</CodeGroup>

#### Когда использовать CLAUDE.md

**Лучше всего для:**

- **Общий для команды контекст** - Руководящие принципы, которым должны следовать все
- **Соглашения проекта** - Стандарты кодирования, структура файлов, шаблоны именования
- **Общие команды** - Команды сборки, тестирования, развертывания, специфичные для вашего проекта
- **Долгосрочная память** - Контекст, который должен сохраняться во всех сессиях
- **Инструкции под контролем версий** - Коммит в git, чтобы команда оставалась синхронизированной

**Ключевые характеристики:**

- ✅ Постоянство во всех сессиях в проекте
- ✅ Совместное использование с командой через git
- ✅ Автоматическое обнаружение (не требуется изменений кода)
- ⚠️ Требует загрузки настроек через `settingSources`

### Метод 2: Стили вывода (постоянные конфигурации)

Стили вывода - это сохраненные конфигурации, которые изменяют системный промпт Claude. Они хранятся как markdown файлы и могут быть повторно использованы в разных сессиях и проектах.

#### Создание стиля вывода

<CodeGroup>

```typescript TypeScript
import { writeFile, mkdir } from "fs/promises";
import { join } from "path";
import { homedir } from "os";

async function createOutputStyle(
  name: string,
  description: string,
  prompt: string
) {
  // Уровень пользователя: ~/.claude/output-styles
  // Уровень проекта: .claude/output-styles
  const outputStylesDir = join(homedir(), ".claude", "output-styles");

  await mkdir(outputStylesDir, { recursive: true });

  const content = `---
name: ${name}
description: ${description}
---

${prompt}`;

  const filePath = join(
    outputStylesDir,
    `${name.toLowerCase().replace(/\s+/g, "-")}.md`
  );
  await writeFile(filePath, content, "utf-8");
}

// Пример: Создать специалиста по обзору кода
await createOutputStyle(
  "Code Reviewer",
  "Тщательный помощник по обзору кода",
  `Ты эксперт по обзору кода.

Для каждой отправки кода:
1. Проверь на ошибки и проблемы безопасности
2. Оцени производительность
3. Предложи улучшения
4. Оцени качество кода (1-10)`
);
```

```python Python
from pathlib import Path

async def create_output_style(name: str, description: str, prompt: str):
    # Уровень пользователя: ~/.claude/output-styles
    # Уровень проекта: .claude/output-styles
    output_styles_dir = Path.home() / '.claude' / 'output-styles'

    output_styles_dir.mkdir(parents=True, exist_ok=True)

    content = f"""---
name: {name}
description: {description}
---

{prompt}"""

    file_name = name.lower().replace(' ', '-') + '.md'
    file_path = output_styles_dir / file_name
    file_path.write_text(content, encoding='utf-8')

# Пример: Создать специалиста по обзору кода
await create_output_style(
    'Code Reviewer',
    'Тщательный помощник по обзору кода',
    """Ты эксперт по обзору кода.

Для каждой отправки кода:
1. Проверь на ошибки и проблемы безопасности
2. Оцени производительность
3. Предложи улучшения
4. Оцени качество кода (1-10)"""
)
```

</CodeGroup>

#### Использование стилей вывода

После создания активируйте стили вывода через:

- **CLI**: `/output-style [имя-стиля]`
- **Настройки**: `.claude/settings.local.json`
- **Создать новый**: `/output-style:new [описание]`

**Примечание для пользователей SDK:** Стили вывода загружаются, когда вы включаете `settingSources: ['user']` или `settingSources: ['project']` (TypeScript) / `setting_sources=["user"]` или `setting_sources=["project"]` (Python) в ваших опциях.

### Метод 3: Использование `systemPrompt` с append

Вы можете использовать пресет Claude Code со свойством `append` для добавления ваших пользовательских инструкций, сохраняя при этом всю встроенную функциональность.

<CodeGroup>

```typescript TypeScript
import { query } from "@anthropic-ai/claude-agent-sdk";

const messages = [];

for await (const message of query({
  prompt: "Помоги мне написать Python функцию для вычисления чисел Фибоначчи",
  options: {
    systemPrompt: {
      type: "preset",
      preset: "claude_code",
      append:
        "Всегда включай подробные docstrings и подсказки типов в Python коде.",
    },
  },
})) {
  messages.push(message);
  if (message.type === "assistant") {
    console.log(message.message.content);
  }
}
```

```python Python
from claude_agent_sdk import query, ClaudeAgentOptions

messages = []

async for message in query(
    prompt="Помоги мне написать Python функцию для вычисления чисел Фибоначчи",
    options=ClaudeAgentOptions(
        system_prompt={
            "type": "preset",
            "preset": "claude_code",
            "append": "Всегда включай подробные docstrings и подсказки типов в Python коде."
        }
    )
):
    messages.append(message)
    if message.type == 'assistant':
        print(message.message.content)
```

</CodeGroup>

### Метод 4: Пользовательские системные промпты

Вы можете предоставить пользовательскую строку как `systemPrompt`, чтобы полностью заменить значение по умолчанию своими собственными инструкциями.

<CodeGroup>

```typescript TypeScript
import { query } from "@anthropic-ai/claude-agent-sdk";

const customPrompt = `Ты специалист по Python кодированию.
Следуй этим руководящим принципам:
- Пиши чистый, хорошо документированный код
- Используй подсказки типов для всех функций
- Включай исчерпывающие docstrings
- Предпочитай шаблоны функционального программирования, когда это уместно
- Всегда объясняй свой выбор кода`;

const messages = [];

for await (const message of query({
  prompt: "Создай конвейер обработки данных",
  options: {
    systemPrompt: customPrompt,
  },
})) {
  messages.push(message);
  if (message.type === "assistant") {
    console.log(message.message.content);
  }
}
```

```python Python
from claude_agent_sdk import query, ClaudeAgentOptions

custom_prompt = """Ты специалист по Python кодированию.
Следуй этим руководящим принципам:
- Пиши чистый, хорошо документированный код
- Используй подсказки типов для всех функций
- Включай исчерпывающие docstrings
- Предпочитай шаблоны функционального программирования, когда это уместно
- Всегда объясняй свой выбор кода"""

messages = []

async for message in query(
    prompt="Создай конвейер обработки данных",
    options=ClaudeAgentOptions(
        system_prompt=custom_prompt
    )
):
    messages.append(message)
    if message.type == 'assistant':
        print(message.message.content)
```

</CodeGroup>

## Сравнение всех четырех подходов

| Функция                 | CLAUDE.md           | Стили вывода      | `systemPrompt` с append | Пользовательский `systemPrompt`     |
| --- | --- | --- | --- | --- |
| **Постоянство**         | Файл на проект | Сохранены как файлы  | Только сессия            | Только сессия           |
| **Повторное использование**         | На проект      | Между проектами | Дублирование кода        | Дублирование кода       |
| **Управление**          | В файловой системе    | CLI + файлы     | В коде                 | В коде                |
| **Инструменты по умолчанию**       | Сохранены        | Сохранены       | Сохранены               | Потеряны (если не включены) |
| **Встроенная безопасность**     | Поддерживается       | Поддерживается      | Поддерживается              | Должна быть добавлена          |
| **Контекст окружения** | Автоматический        | Автоматический       | Автоматический               | Должен быть предоставлен       |
| **Уровень настройки** | Только дополнения   | Заменить по умолчанию | Только дополнения          | Полный контроль       |
| **Контроль версий**     | С проектом     | Да             | С кодом               | С кодом              |
| **Область**               | Специфично для проекта | Пользователь или проект | Сессия кода            | Сессия кода           |

**Примечание:** "С append" означает использование `systemPrompt: { type: "preset", preset: "claude_code", append: "..." }` в TypeScript или `system_prompt={"type": "preset", "preset": "claude_code", "append": "..."}` в Python.

## Случаи использования и лучшие практики

### Когда использовать CLAUDE.md

**Лучше всего для:**

- Специфичных для проекта стандартов и соглашений кодирования
- Документирования структуры и архитектуры проекта
- Перечисления общих команд (сборка, тест, развертывание)
- Общего для команды контекста, который должен быть под контролем версий
- Инструкций, которые применяются ко всему использованию SDK в проекте

**Примеры:**

- "Все API конечные точки должны использовать шаблоны async/await"
- "Запускай `npm run lint:fix` перед коммитом"
- "Миграции базы данных находятся в каталоге `migrations/`"

**Важно:** Чтобы загрузить файлы CLAUDE.md, вы должны явно установить `settingSources: ['project']` (TypeScript) или `setting_sources=["project"]` (Python). Пресет системного промпта `claude_code` НЕ загружает автоматически CLAUDE.md без этой настройки.

### Когда использовать стили вывода

**Лучше всего для:**

- Постоянных изменений поведения между сессиями
- Конфигураций, общих для команды
- Специализированных помощников (обзорщик кода, специалист по данным, DevOps)
- Сложных изменений промпта, которые нуждаются в версионировании

**Примеры:**

- Создание специализированного помощника по оптимизации SQL
- Построение обзорщика кода, ориентированного на безопасность
- Разработка помощника по обучению с конкретной педагогикой

### Когда использовать `systemPrompt` с append

**Лучше всего для:**

- Добавления конкретных стандартов или предпочтений кодирования
- Настройки форматирования вывода
- Добавления знаний, специфичных для домена
- Изменения многословности ответа
- Улучшения поведения Claude Code по умолчанию без потери инструкций по инструментам

### Когда использовать пользовательский `systemPrompt`

**Лучше всего для:**

- Полного контроля над поведением Claude
- Специализированных задач одной сессии
- Тестирования новых стратегий промпта
- Ситуаций, где инструменты по умолчанию не нужны
- Построения специализированных агентов с уникальным поведением

## Комбинирование подходов

Вы можете комбинировать эти методы для максимальной гибкости:

### Пример: Стиль вывода с дополнениями, специфичными для сессии

<CodeGroup>

```typescript TypeScript
import { query } from "@anthropic-ai/claude-agent-sdk";

// Предполагая, что стиль вывода "Code Reviewer" активен (через /output-style)
// Добавить области фокуса, специфичные для сессии
const messages = [];

for await (const message of query({
  prompt: "Просмотри этот модуль аутентификации",
  options: {
    systemPrompt: {
      type: "preset",
      preset: "claude_code",
      append: `
        Для этого обзора приоритизируй:
        - Соответствие OAuth 2.0
        - Безопасность хранения токенов
        - Управление сессиями
      `,
    },
  },
})) {
  messages.push(message);
}
```

```python Python
from claude_agent_sdk import query, ClaudeAgentOptions

# Предполагая, что стиль вывода "Code Reviewer" активен (через /output-style)
# Добавить области фокуса, специфичные для сессии
messages = []

async for message in query(
    prompt="Просмотри этот модуль аутентификации",
    options=ClaudeAgentOptions(
        system_prompt={
            "type": "preset",
            "preset": "claude_code",
            "append": """
            Для этого обзора приоритизируй:
            - Соответствие OAuth 2.0
            - Безопасность хранения токенов
            - Управление сессиями
            """
        }
    )
):
    messages.append(message)
```

</CodeGroup>

## См. также

- [Стили вывода](https://code.claude.com/docs/output-styles) - Полная документация по стилям вывода
- [Руководство по TypeScript SDK](/docs/ru/agent-sdk/typescript) - Полное руководство по использованию SDK
- [Справочник по TypeScript SDK](https://code.claude.com/docs/typescript-sdk-reference) - Полная документация API
- [Руководство по конфигурации](https://code.claude.com/docs/configuration) - Общие опции конфигурации