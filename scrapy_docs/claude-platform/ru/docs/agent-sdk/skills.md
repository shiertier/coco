# Навыки агента в SDK

Расширьте Claude специализированными возможностями, используя Agent Skills в Claude Agent SDK

---

## Обзор

Agent Skills расширяют Claude специализированными возможностями, которые Claude автономно вызывает при необходимости. Навыки упакованы как файлы `SKILL.md`, содержащие инструкции, описания и дополнительные вспомогательные ресурсы.

Для получения полной информации о Skills, включая преимущества, архитектуру и рекомендации по созданию, см. [обзор Agent Skills](/docs/ru/agents-and-tools/agent-skills/overview).

## Как Skills работают с SDK

При использовании Claude Agent SDK Skills:

1. **Определены как артефакты файловой системы**: Созданы как файлы `SKILL.md` в определённых каталогах (`.claude/skills/`)
2. **Загружены из файловой системы**: Skills загружаются из настроенных мест в файловой системе. Вы должны указать `settingSources` (TypeScript) или `setting_sources` (Python) для загрузки Skills из файловой системы
3. **Автоматически обнаружены**: После загрузки параметров файловой системы метаданные Skill обнаруживаются при запуске из пользовательских и проектных каталогов; полное содержимое загружается при срабатывании
4. **Вызваны моделью**: Claude автономно выбирает, когда их использовать, на основе контекста
5. **Включены через allowed_tools**: Добавьте `"Skill"` в ваш `allowed_tools` для включения Skills

В отличие от подагентов (которые могут быть определены программно), Skills должны быть созданы как артефакты файловой системы. SDK не предоставляет программный API для регистрации Skills.

<Note>
**Поведение по умолчанию**: По умолчанию SDK не загружает никакие параметры файловой системы. Для использования Skills вы должны явно настроить `settingSources: ['user', 'project']` (TypeScript) или `setting_sources=["user", "project"]` (Python) в ваших параметрах.
</Note>

## Использование Skills с SDK

Для использования Skills с SDK вам необходимо:

1. Включить `"Skill"` в конфигурацию `allowed_tools`
2. Настроить `settingSources`/`setting_sources` для загрузки Skills из файловой системы

После настройки Claude автоматически обнаруживает Skills из указанных каталогов и вызывает их при необходимости для запроса пользователя.

<CodeGroup>

```python Python
import asyncio
from claude_agent_sdk import query, ClaudeAgentOptions

async def main():
    options = ClaudeAgentOptions(
        cwd="/path/to/project",  # Project with .claude/skills/
        setting_sources=["user", "project"],  # Load Skills from filesystem
        allowed_tools=["Skill", "Read", "Write", "Bash"]  # Enable Skill tool
    )

    async for message in query(
        prompt="Help me process this PDF document",
        options=options
    ):
        print(message)

asyncio.run(main())
```

```typescript TypeScript
import { query } from "@anthropic-ai/claude-agent-sdk";

for await (const message of query({
  prompt: "Help me process this PDF document",
  options: {
    cwd: "/path/to/project",  // Project with .claude/skills/
    settingSources: ["user", "project"],  // Load Skills from filesystem
    allowedTools: ["Skill", "Read", "Write", "Bash"]  // Enable Skill tool
  }
})) {
  console.log(message);
}
```

</CodeGroup>

## Расположение Skills

Skills загружаются из каталогов файловой системы на основе вашей конфигурации `settingSources`/`setting_sources`:

- **Project Skills** (`.claude/skills/`): Общие с вашей командой через git - загружаются, когда `setting_sources` включает `"project"`
- **User Skills** (`~/.claude/skills/`): Личные Skills для всех проектов - загружаются, когда `setting_sources` включает `"user"`
- **Plugin Skills**: Поставляются с установленными плагинами Claude Code

## Создание Skills

Skills определены как каталоги, содержащие файл `SKILL.md` с YAML frontmatter и содержимым Markdown. Поле `description` определяет, когда Claude вызывает ваш Skill.

**Пример структуры каталога**:
```bash
.claude/skills/processing-pdfs/
└── SKILL.md
```

Для полного руководства по созданию Skills, включая структуру SKILL.md, многофайловые Skills и примеры, см.:
- [Agent Skills в Claude Code](https://code.claude.com/docs/skills): Полное руководство с примерами
- [Agent Skills Best Practices](/docs/ru/agents-and-tools/agent-skills/best-practices): Рекомендации по созданию и соглашения об именовании

## Ограничения инструментов

<Note>
Поле frontmatter `allowed-tools` в SKILL.md поддерживается только при прямом использовании Claude Code CLI. **Оно не применяется при использовании Skills через SDK**.

При использовании SDK контролируйте доступ к инструментам через основной параметр `allowedTools` в конфигурации вашего запроса.
</Note>

Для ограничения инструментов для Skills в приложениях SDK используйте параметр `allowedTools`:

<Note>
Предполагается, что операторы импорта из первого примера используются в следующих фрагментах кода.
</Note>

<CodeGroup>

```python Python
options = ClaudeAgentOptions(
    setting_sources=["user", "project"],  # Load Skills from filesystem
    allowed_tools=["Skill", "Read", "Grep", "Glob"]  # Restricted toolset
)

async for message in query(
    prompt="Analyze the codebase structure",
    options=options
):
    print(message)
```

```typescript TypeScript
// Skills can only use Read, Grep, and Glob tools
for await (const message of query({
  prompt: "Analyze the codebase structure",
  options: {
    settingSources: ["user", "project"],  // Load Skills from filesystem
    allowedTools: ["Skill", "Read", "Grep", "Glob"]  // Restricted toolset
  }
})) {
  console.log(message);
}
```

</CodeGroup>

## Обнаружение доступных Skills

Чтобы узнать, какие Skills доступны в вашем приложении SDK, просто спросите Claude:

<CodeGroup>

```python Python
options = ClaudeAgentOptions(
    setting_sources=["user", "project"],  # Load Skills from filesystem
    allowed_tools=["Skill"]
)

async for message in query(
    prompt="What Skills are available?",
    options=options
):
    print(message)
```

```typescript TypeScript
for await (const message of query({
  prompt: "What Skills are available?",
  options: {
    settingSources: ["user", "project"],  // Load Skills from filesystem
    allowedTools: ["Skill"]
  }
})) {
  console.log(message);
}
```

</CodeGroup>

Claude выведет список доступных Skills на основе вашего текущего рабочего каталога и установленных плагинов.

## Тестирование Skills

Протестируйте Skills, задав вопросы, соответствующие их описаниям:

<CodeGroup>

```python Python
options = ClaudeAgentOptions(
    cwd="/path/to/project",
    setting_sources=["user", "project"],  # Load Skills from filesystem
    allowed_tools=["Skill", "Read", "Bash"]
)

async for message in query(
    prompt="Extract text from invoice.pdf",
    options=options
):
    print(message)
```

```typescript TypeScript
for await (const message of query({
  prompt: "Extract text from invoice.pdf",
  options: {
    cwd: "/path/to/project",
    settingSources: ["user", "project"],  // Load Skills from filesystem
    allowedTools: ["Skill", "Read", "Bash"]
  }
})) {
  console.log(message);
}
```

</CodeGroup>

Claude автоматически вызывает соответствующий Skill, если описание совпадает с вашим запросом.

## Устранение неполадок

### Skills не найдены

**Проверьте конфигурацию settingSources**: Skills загружаются только при явной настройке `settingSources`/`setting_sources`. Это наиболее распространённая проблема:

<CodeGroup>

```python Python
# Wrong - Skills won't be loaded
options = ClaudeAgentOptions(
    allowed_tools=["Skill"]
)

# Correct - Skills will be loaded
options = ClaudeAgentOptions(
    setting_sources=["user", "project"],  # Required to load Skills
    allowed_tools=["Skill"]
)
```

```typescript TypeScript
// Wrong - Skills won't be loaded
const options = {
  allowedTools: ["Skill"]
};

// Correct - Skills will be loaded
const options = {
  settingSources: ["user", "project"],  // Required to load Skills
  allowedTools: ["Skill"]
};
```

</CodeGroup>

Для получения дополнительной информации о `settingSources`/`setting_sources` см. [справочник TypeScript SDK](/docs/ru/agent-sdk/typescript#settingsource) или [справочник Python SDK](/docs/ru/agent-sdk/python#settingsource).

**Проверьте рабочий каталог**: SDK загружает Skills относительно параметра `cwd`. Убедитесь, что он указывает на каталог, содержащий `.claude/skills/`:

<CodeGroup>

```python Python
# Ensure your cwd points to the directory containing .claude/skills/
options = ClaudeAgentOptions(
    cwd="/path/to/project",  # Must contain .claude/skills/
    setting_sources=["user", "project"],  # Required to load Skills
    allowed_tools=["Skill"]
)
```

```typescript TypeScript
// Ensure your cwd points to the directory containing .claude/skills/
const options = {
  cwd: "/path/to/project",  // Must contain .claude/skills/
  settingSources: ["user", "project"],  // Required to load Skills
  allowedTools: ["Skill"]
};
```

</CodeGroup>

Полный шаблон см. в разделе "Использование Skills с SDK" выше.

**Проверьте расположение файловой системы**:
```bash
# Check project Skills
ls .claude/skills/*/SKILL.md

# Check personal Skills
ls ~/.claude/skills/*/SKILL.md
```

### Skill не используется

**Проверьте, что инструмент Skill включен**: Убедитесь, что `"Skill"` находится в вашем `allowedTools`.

**Проверьте описание**: Убедитесь, что оно конкретно и включает соответствующие ключевые слова. См. [Agent Skills Best Practices](/docs/ru/agents-and-tools/agent-skills/best-practices#writing-effective-descriptions) для руководства по написанию эффективных описаний.

### Дополнительное устранение неполадок

Для общего устранения неполадок Skills (синтаксис YAML, отладка и т.д.) см. [раздел устранения неполадок Claude Code Skills](https://code.claude.com/docs/skills#troubleshooting).

## Связанная документация

### Руководства Skills
- [Agent Skills в Claude Code](https://code.claude.com/docs/skills): Полное руководство Skills с созданием, примерами и устранением неполадок
- [Agent Skills Overview](/docs/ru/agents-and-tools/agent-skills/overview): Концептуальный обзор, преимущества и архитектура
- [Agent Skills Best Practices](/docs/ru/agents-and-tools/agent-skills/best-practices): Рекомендации по созданию эффективных Skills
- [Agent Skills Cookbook](https://github.com/anthropics/claude-cookbooks/tree/main/skills): Примеры Skills и шаблоны

### Ресурсы SDK
- [Subagents in the SDK](/docs/ru/agent-sdk/subagents): Аналогичные агенты на основе файловой системы с программными параметрами
- [Slash Commands in the SDK](/docs/ru/agent-sdk/slash-commands): Команды, вызываемые пользователем
- [SDK Overview](/docs/ru/agent-sdk/overview): Общие концепции SDK
- [TypeScript SDK Reference](/docs/ru/agent-sdk/typescript): Полная документация API
- [Python SDK Reference](/docs/ru/agent-sdk/python): Полная документация API