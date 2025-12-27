# Перехват и управление поведением агента с помощью hooks

Перехватывайте и настраивайте поведение агента в ключевых точках выполнения с помощью hooks

---

Hooks позволяют вам перехватывать выполнение агента в ключевых точках для добавления валидации, логирования, элементов управления безопасностью или пользовательской логики. С помощью hooks вы можете:

- **Блокировать опасные операции** перед их выполнением, такие как деструктивные команды shell или несанкционированный доступ к файлам
- **Логировать и аудировать** каждый вызов инструмента для соответствия, отладки или аналитики
- **Преобразовывать входные и выходные данные** для санитизации данных, внедрения учетных данных или перенаправления путей файлов
- **Требовать одобрение человека** для чувствительных действий, таких как запись в базу данных или вызовы API
- **Отслеживать жизненный цикл сеанса** для управления состоянием, очистки ресурсов или отправки уведомлений

Hook состоит из двух частей:

1. **Функция обратного вызова**: логика, которая выполняется при срабатывании hook
2. **Конфигурация hook**: указывает SDK, какое событие перехватывать (например, `PreToolUse`) и какие инструменты сопоставлять

Следующий пример блокирует агента от изменения файлов `.env`. Сначала определите обратный вызов, который проверяет путь файла, затем передайте его в `query()` для запуска перед любым вызовом инструмента Write или Edit:

<CodeGroup>

```python Python
import asyncio
from claude_agent_sdk import query, ClaudeAgentOptions, HookMatcher

# Define a hook callback that receives tool call details
async def protect_env_files(input_data, tool_use_id, context):
    # Extract the file path from the tool's input arguments
    file_path = input_data['tool_input'].get('file_path', '')
    file_name = file_path.split('/')[-1]

    # Block the operation if targeting a .env file
    if file_name == '.env':
        return {
            'hookSpecificOutput': {
                'hookEventName': input_data['hook_event_name'],
                'permissionDecision': 'deny',
                'permissionDecisionReason': 'Cannot modify .env files'
            }
        }

    # Return empty object to allow the operation
    return {}

async def main():
    async for message in query(
        prompt="Update the database configuration",
        options=ClaudeAgentOptions(
            hooks={
                # Register the hook for PreToolUse events
                # The matcher filters to only Write and Edit tool calls
                'PreToolUse': [HookMatcher(matcher='Write|Edit', hooks=[protect_env_files])]
            }
        )
    ):
        print(message)

asyncio.run(main())
```

```typescript TypeScript
import { query, HookCallback, PreToolUseHookInput } from "@anthropic-ai/claude-agent-sdk";

// Define a hook callback with the HookCallback type
const protectEnvFiles: HookCallback = async (input, toolUseID, { signal }) => {
  // Cast input to the specific hook type for type safety
  const preInput = input as PreToolUseHookInput;

  // Extract the file path from the tool's input arguments
  const filePath = preInput.tool_input?.file_path as string;
  const fileName = filePath?.split('/').pop();

  // Block the operation if targeting a .env file
  if (fileName === '.env') {
    return {
      hookSpecificOutput: {
        hookEventName: input.hook_event_name,
        permissionDecision: 'deny',
        permissionDecisionReason: 'Cannot modify .env files'
      }
    };
  }

  // Return empty object to allow the operation
  return {};
};

for await (const message of query({
  prompt: "Update the database configuration",
  options: {
    hooks: {
      // Register the hook for PreToolUse events
      // The matcher filters to only Write and Edit tool calls
      PreToolUse: [{ matcher: 'Write|Edit', hooks: [protectEnvFiles] }]
    }
  }
})) {
  console.log(message);
}
```

</CodeGroup>

Это hook `PreToolUse`. Он выполняется перед выполнением инструмента и может блокировать или разрешать операции на основе вашей логики. Остальная часть этого руководства охватывает все доступные hooks, их параметры конфигурации и шаблоны для распространенных случаев использования.

## Доступные hooks

SDK предоставляет hooks для различных этапов выполнения агента. Некоторые hooks доступны в обоих SDK, в то время как другие доступны только в TypeScript, потому что Python SDK их не поддерживает.

| Hook Event | Python SDK | TypeScript SDK | Что его запускает | Пример использования |
|------------|------------|----------------|------------------|------------------|
| `PreToolUse` | Да | Да | Запрос вызова инструмента (может блокировать или изменять) | Блокировать опасные команды shell |
| `PostToolUse` | Да | Да | Результат выполнения инструмента | Логировать все изменения файлов в журнал аудита |
| `PostToolUseFailure` | Нет | Да | Ошибка выполнения инструмента | Обработать или логировать ошибки инструмента |
| `UserPromptSubmit` | Да | Да | Отправка пользовательского запроса | Внедрить дополнительный контекст в запросы |
| `Stop` | Да | Да | Остановка выполнения агента | Сохранить состояние сеанса перед выходом |
| `SubagentStart` | Нет | Да | Инициализация подагента | Отслеживать порождение параллельных задач |
| `SubagentStop` | Да | Да | Завершение подагента | Агрегировать результаты из параллельных задач |
| `PreCompact` | Да | Да | Запрос компактирования разговора | Архивировать полную стенограмму перед суммированием |
| `PermissionRequest` | Нет | Да | Диалог разрешения будет отображен | Пользовательская обработка разрешений |
| `SessionStart` | Нет | Да | Инициализация сеанса | Инициализировать логирование и телеметрию |
| `SessionEnd` | Нет | Да | Завершение сеанса | Очистить временные ресурсы |
| `Notification` | Нет | Да | Сообщения о статусе агента | Отправить обновления статуса агента в Slack или PagerDuty |

## Распространенные случаи использования

Hooks достаточно гибкие для обработки многих различных сценариев. Вот некоторые из наиболее распространенных шаблонов, организованные по категориям.

<Tabs>
  <Tab title="Безопасность">
    - Блокировать опасные команды (такие как `rm -rf /`, деструктивный SQL)
    - Проверять пути файлов перед операциями записи
    - Применять списки разрешений/запретов для использования инструментов
  </Tab>
  <Tab title="Логирование">
    - Создавать журналы аудита всех действий агента
    - Отслеживать метрики выполнения и производительность
    - Отлаживать поведение агента в разработке
  </Tab>
  <Tab title="Перехват инструментов">
    - Перенаправлять операции с файлами в изолированные каталоги
    - Внедрять переменные окружения или учетные данные
    - Преобразовывать входные или выходные данные инструментов
  </Tab>
  <Tab title="Авторизация">
    - Реализовать управление доступом на основе ролей
    - Требовать одобрение человека для чувствительных операций
    - Ограничивать частоту использования конкретных инструментов
  </Tab>
</Tabs>

## Настройка hooks

Чтобы настроить hook для вашего агента, передайте hook в параметр `options.hooks` при вызове `query()`:

<CodeGroup>

```python Python
async for message in query(
    prompt="Your prompt",
    options=ClaudeAgentOptions(
        hooks={
            'PreToolUse': [HookMatcher(matcher='Bash', hooks=[my_callback])]
        }
    )
):
    print(message)
```

```typescript TypeScript
for await (const message of query({
  prompt: "Your prompt",
  options: {
    hooks: {
      PreToolUse: [{ matcher: 'Bash', hooks: [myCallback] }]
    }
  }
})) {
  console.log(message);
}
```

</CodeGroup>

Опция `hooks` — это словарь (Python) или объект (TypeScript), где:
- **Ключи** — это [имена событий hook](#available-hooks) (например, `'PreToolUse'`, `'PostToolUse'`, `'Stop'`)
- **Значения** — это массивы [сопоставителей](#matchers), каждый содержащий необязательный шаблон фильтра и ваши [функции обратного вызова](#callback-function-inputs)

Ваши функции обратного вызова hook получают [входные данные](#input-data) о событии и возвращают [ответ](#callback-outputs), чтобы агент знал, разрешить, блокировать или изменить операцию.

### Сопоставители

Используйте сопоставители для фильтрации, какие инструменты запускают ваши обратные вызовы:

| Опция | Тип | По умолчанию | Описание |
|--------|------|---------|-------------|
| `matcher` | `string` | `undefined` | Шаблон регулярного выражения для сопоставления имен инструментов. Встроенные инструменты включают `Bash`, `Read`, `Write`, `Edit`, `Glob`, `Grep`, `WebFetch`, `Task` и другие. Инструменты MCP используют шаблон `mcp__<server>__<action>`. |
| `hooks` | `HookCallback[]` | - | Обязательно. Массив функций обратного вызова для выполнения при совпадении шаблона |
| `timeout` | `number` | `60` | Тайм-аут в секундах; увеличьте для hooks, которые выполняют внешние вызовы API |

Используйте шаблон `matcher` для нацеливания на конкретные инструменты, когда это возможно. Сопоставитель с `'Bash'` выполняется только для команд Bash, в то время как опущение шаблона запускает ваши обратные вызовы для каждого вызова инструмента. Обратите внимание, что сопоставители фильтруют только по **имени инструмента**, а не по путям файлов или другим аргументам — для фильтрации по пути файла проверьте `tool_input.file_path` внутри вашего обратного вызова.

Сопоставители применяются только к hooks на основе инструментов (`PreToolUse`, `PostToolUse`, `PostToolUseFailure`, `PermissionRequest`). Для hooks жизненного цикла, таких как `Stop`, `SessionStart` и `Notification`, сопоставители игнорируются и hook срабатывает для всех событий этого типа.

<Tip>
**Обнаружение имен инструментов:** Проверьте массив `tools` в начальном системном сообщении при запуске вашего сеанса или добавьте hook без сопоставителя для логирования всех вызовов инструментов.

**Именование инструментов MCP:** Инструменты MCP всегда начинаются с `mcp__`, за которым следует имя сервера и действие: `mcp__<server>__<action>`. Например, если вы настроите сервер с именем `playwright`, его инструменты будут названы `mcp__playwright__browser_screenshot`, `mcp__playwright__browser_click` и т. д. Имя сервера берется из ключа, который вы используете в конфигурации `mcpServers`.
</Tip>

Этот пример использует сопоставитель для запуска hook только для инструментов, изменяющих файлы, при срабатывании события `PreToolUse`:

<CodeGroup>

```python Python
options = ClaudeAgentOptions(
    hooks={
        'PreToolUse': [
            HookMatcher(matcher='Write|Edit', hooks=[validate_file_path])
        ]
    }
)
```

```typescript TypeScript
const options = {
  hooks: {
    PreToolUse: [
      { matcher: 'Write|Edit', hooks: [validateFilePath] }
    ]
  }
};
```

</CodeGroup>

### Входные данные функции обратного вызова

Каждый hook обратного вызова получает три аргумента:

1. **Входные данные** (`dict` / `HookInput`): Детали события. См. [входные данные](#input-data) для полей
2. **ID использования инструмента** (`str | None` / `string | null`): Коррелировать события `PreToolUse` и `PostToolUse`
3. **Контекст** (`HookContext`): В TypeScript содержит свойство `signal` (`AbortSignal`) для отмены. Передайте это асинхронным операциям, таким как `fetch()`, чтобы они автоматически отменялись, если hook истечет. В Python этот аргумент зарезервирован для будущего использования.

### Входные данные

Первый аргумент вашего hook обратного вызова содержит информацию о событии. Имена полей идентичны в SDK (оба используют snake_case).

**Общие поля**, присутствующие во всех типах hooks:

| Поле | Тип | Описание |
|-------|------|-------------|
| `hook_event_name` | `string` | Тип hook (`PreToolUse`, `PostToolUse` и т. д.) |
| `session_id` | `string` | Текущий идентификатор сеанса |
| `transcript_path` | `string` | Путь к стенограмме разговора |
| `cwd` | `string` | Текущий рабочий каталог |

**Поля, специфичные для hook**, варьируются в зависимости от типа hook. Элементы, отмеченные <sup>TS</sup>, доступны только в TypeScript SDK:

| Поле | Тип | Описание | Hooks |
|-------|------|-------------|-------|
| `tool_name` | `string` | Имя вызываемого инструмента | PreToolUse, PostToolUse, PostToolUseFailure<sup>TS</sup>, PermissionRequest<sup>TS</sup> |
| `tool_input` | `object` | Аргументы, передаваемые инструменту | PreToolUse, PostToolUse, PostToolUseFailure<sup>TS</sup>, PermissionRequest<sup>TS</sup> |
| `tool_response` | `any` | Результат, возвращенный из выполнения инструмента | PostToolUse |
| `error` | `string` | Сообщение об ошибке из ошибки выполнения инструмента | PostToolUseFailure<sup>TS</sup> |
| `is_interrupt` | `boolean` | Была ли ошибка вызвана прерыванием | PostToolUseFailure<sup>TS</sup> |
| `prompt` | `string` | Текст запроса пользователя | UserPromptSubmit |
| `stop_hook_active` | `boolean` | Обрабатывается ли в данный момент stop hook | Stop, SubagentStop |
| `agent_id` | `string` | Уникальный идентификатор подагента | SubagentStart<sup>TS</sup>, SubagentStop<sup>TS</sup> |
| `agent_type` | `string` | Тип/роль подагента | SubagentStart<sup>TS</sup> |
| `agent_transcript_path` | `string` | Путь к стенограмме разговора подагента | SubagentStop<sup>TS</sup> |
| `trigger` | `string` | Что вызвало компактирование: `manual` или `auto` | PreCompact |
| `custom_instructions` | `string` | Пользовательские инструкции, предоставленные для компактирования | PreCompact |
| `permission_suggestions` | `array` | Предлагаемые обновления разрешений для инструмента | PermissionRequest<sup>TS</sup> |
| `source` | `string` | Как был запущен сеанс: `startup`, `resume`, `clear` или `compact` | SessionStart<sup>TS</sup> |
| `reason` | `string` | Почему сеанс завершился: `clear`, `logout`, `prompt_input_exit`, `bypass_permissions_disabled` или `other` | SessionEnd<sup>TS</sup> |
| `message` | `string` | Сообщение о статусе от агента | Notification<sup>TS</sup> |
| `notification_type` | `string` | Тип уведомления: `permission_prompt`, `idle_prompt`, `auth_success` или `elicitation_dialog` | Notification<sup>TS</sup> |
| `title` | `string` | Необязательный заголовок, установленный агентом | Notification<sup>TS</sup> |

Код ниже определяет hook обратного вызова, который использует `tool_name` и `tool_input` для логирования деталей каждого вызова инструмента:

<CodeGroup>

```python Python
async def log_tool_calls(input_data, tool_use_id, context):
    if input_data['hook_event_name'] == 'PreToolUse':
        print(f"Tool: {input_data['tool_name']}")
        print(f"Input: {input_data['tool_input']}")
    return {}
```

```typescript TypeScript
const logToolCalls: HookCallback = async (input, toolUseID, { signal }) => {
  if (input.hook_event_name === 'PreToolUse') {
    const preInput = input as PreToolUseHookInput;
    console.log(`Tool: ${preInput.tool_name}`);
    console.log(`Input:`, preInput.tool_input);
  }
  return {};
};
```

</CodeGroup>

### Выходные данные обратного вызова

Ваша функция обратного вызова возвращает объект, который сообщает SDK, как продолжить. Верните пустой объект `{}` для разрешения операции без изменений. Чтобы блокировать, изменять или добавлять контекст к операции, верните объект с полем `hookSpecificOutput`, содержащим ваше решение.

**Поля верхнего уровня** (вне `hookSpecificOutput`):

| Поле | Тип | Описание |
|-------|------|-------------|
| `continue` | `boolean` | Должен ли агент продолжить после этого hook (по умолчанию: `true`) |
| `stopReason` | `string` | Сообщение, показываемое при `continue` равном `false` |
| `suppressOutput` | `boolean` | Скрыть stdout из стенограммы (по умолчанию: `false`) |
| `systemMessage` | `string` | Сообщение, внедренное в разговор для Claude |

**Поля внутри `hookSpecificOutput`**:

| Поле | Тип | Hooks | Описание |
|-------|------|-------|-------------|
| `hookEventName` | `string` | Все | Обязательно. Используйте `input.hook_event_name` для сопоставления текущего события |
| `permissionDecision` | `'allow'` \| `'deny'` \| `'ask'` | PreToolUse | Контролирует, выполняется ли инструмент |
| `permissionDecisionReason` | `string` | PreToolUse | Объяснение, показываемое Claude для решения |
| `updatedInput` | `object` | PreToolUse | Измененный входной сигнал инструмента (требует `permissionDecision: 'allow'`) |
| `additionalContext` | `string` | PostToolUse, UserPromptSubmit, SessionStart<sup>TS</sup>, SubagentStart<sup>TS</sup> | Контекст, добавленный в разговор |

Этот пример блокирует операции записи в каталог `/etc` при внедрении системного сообщения для напоминания Claude о безопасных практиках работы с файлами:

<CodeGroup>

```python Python
async def block_etc_writes(input_data, tool_use_id, context):
    file_path = input_data['tool_input'].get('file_path', '')

    if file_path.startswith('/etc'):
        return {
            # Top-level field: inject guidance into the conversation
            'systemMessage': 'Remember: system directories like /etc are protected.',
            # hookSpecificOutput: block the operation
            'hookSpecificOutput': {
                'hookEventName': input_data['hook_event_name'],
                'permissionDecision': 'deny',
                'permissionDecisionReason': 'Writing to /etc is not allowed'
            }
        }
    return {}
```

```typescript TypeScript
const blockEtcWrites: HookCallback = async (input, toolUseID, { signal }) => {
  const filePath = (input as PreToolUseHookInput).tool_input?.file_path as string;

  if (filePath?.startsWith('/etc')) {
    return {
      // Top-level field: inject guidance into the conversation
      systemMessage: 'Remember: system directories like /etc are protected.',
      // hookSpecificOutput: block the operation
      hookSpecificOutput: {
        hookEventName: input.hook_event_name,
        permissionDecision: 'deny',
        permissionDecisionReason: 'Writing to /etc is not allowed'
      }
    };
  }
  return {};
};
```

</CodeGroup>

#### Поток принятия решения о разрешении

Когда применяются несколько hooks или правил разрешений, SDK оценивает их в этом порядке:

1. **Правила Deny** проверяются первыми (любое совпадение = немедленный отказ).
2. **Правила Ask** проверяются вторыми.
3. **Правила Allow** проверяются третьими.
4. **По умолчанию Ask**, если ничего не совпадает.

Если какой-либо hook возвращает `deny`, операция блокируется — другие hooks, возвращающие `allow`, не переопределят это.

#### Блокировать инструмент

Верните решение deny для предотвращения выполнения инструмента:

<CodeGroup>

```python Python
async def block_dangerous_commands(input_data, tool_use_id, context):
    if input_data['hook_event_name'] != 'PreToolUse':
        return {}

    command = input_data['tool_input'].get('command', '')

    if 'rm -rf /' in command:
        return {
            'hookSpecificOutput': {
                'hookEventName': input_data['hook_event_name'],
                'permissionDecision': 'deny',
                'permissionDecisionReason': 'Dangerous command blocked: rm -rf /'
            }
        }
    return {}
```

```typescript TypeScript
const blockDangerousCommands: HookCallback = async (input, toolUseID, { signal }) => {
  if (input.hook_event_name !== 'PreToolUse') return {};

  const command = (input as PreToolUseHookInput).tool_input.command as string;

  if (command?.includes('rm -rf /')) {
    return {
      hookSpecificOutput: {
        hookEventName: input.hook_event_name,
        permissionDecision: 'deny',
        permissionDecisionReason: 'Dangerous command blocked: rm -rf /'
      }
    };
  }
  return {};
};
```

</CodeGroup>

#### Изменить входной сигнал инструмента

Верните обновленный входной сигнал для изменения того, что получает инструмент:

<CodeGroup>

```python Python
async def redirect_to_sandbox(input_data, tool_use_id, context):
    if input_data['hook_event_name'] != 'PreToolUse':
        return {}

    if input_data['tool_name'] == 'Write':
        original_path = input_data['tool_input'].get('file_path', '')
        return {
            'hookSpecificOutput': {
                'hookEventName': input_data['hook_event_name'],
                'permissionDecision': 'allow',
                'updatedInput': {
                    **input_data['tool_input'],
                    'file_path': f'/sandbox{original_path}'
                }
            }
        }
    return {}
```

```typescript TypeScript
const redirectToSandbox: HookCallback = async (input, toolUseID, { signal }) => {
  if (input.hook_event_name !== 'PreToolUse') return {};

  const preInput = input as PreToolUseHookInput;
  if (preInput.tool_name === 'Write') {
    const originalPath = preInput.tool_input.file_path as string;
    return {
      hookSpecificOutput: {
        hookEventName: input.hook_event_name,
        permissionDecision: 'allow',
        updatedInput: {
          ...preInput.tool_input,
          file_path: `/sandbox${originalPath}`
        }
      }
    };
  }
  return {};
};
```

</CodeGroup>

<Note>
При использовании `updatedInput` вы также должны включить `permissionDecision`. Всегда возвращайте новый объект вместо мутирования исходного `tool_input`.
</Note>

#### Добавить системное сообщение

Внедрите контекст в разговор:

<CodeGroup>

```python Python
async def add_security_reminder(input_data, tool_use_id, context):
    return {
        'systemMessage': 'Remember to follow security best practices.'
    }
```

```typescript TypeScript
const addSecurityReminder: HookCallback = async (input, toolUseID, { signal }) => {
  return {
    systemMessage: 'Remember to follow security best practices.'
  };
};
```

</CodeGroup>

#### Автоматически одобрять конкретные инструменты

Обойти запросы разрешений для доверенных инструментов. Это полезно, когда вы хотите, чтобы определенные операции выполнялись без подтверждения пользователя:

<CodeGroup>

```python Python
async def auto_approve_read_only(input_data, tool_use_id, context):
    if input_data['hook_event_name'] != 'PreToolUse':
        return {}

    read_only_tools = ['Read', 'Glob', 'Grep', 'LS']
    if input_data['tool_name'] in read_only_tools:
        return {
            'hookSpecificOutput': {
                'hookEventName': input_data['hook_event_name'],
                'permissionDecision': 'allow',
                'permissionDecisionReason': 'Read-only tool auto-approved'
            }
        }
    return {}
```

```typescript TypeScript
const autoApproveReadOnly: HookCallback = async (input, toolUseID, { signal }) => {
  if (input.hook_event_name !== 'PreToolUse') return {};

  const preInput = input as PreToolUseHookInput;
  const readOnlyTools = ['Read', 'Glob', 'Grep', 'LS'];
  if (readOnlyTools.includes(preInput.tool_name)) {
    return {
      hookSpecificOutput: {
        hookEventName: input.hook_event_name,
        permissionDecision: 'allow',
        permissionDecisionReason: 'Read-only tool auto-approved'
      }
    };
  }
  return {};
};
```

</CodeGroup>

<Note>
Поле `permissionDecision` принимает три значения: `'allow'` (автоматическое одобрение), `'deny'` (блокировка) или `'ask'` (запрос подтверждения).
</Note>

## Обработка продвинутых сценариев

Эти шаблоны помогут вам создавать более сложные системы hooks для сложных случаев использования.

### Цепочка нескольких hooks

Hooks выполняются в порядке их появления в массиве. Сосредоточьте каждый hook на одной ответственности и свяжите несколько hooks для сложной логики. Этот пример запускает все четыре hook для каждого вызова инструмента (сопоставитель не указан):

<CodeGroup>

```python Python
options = ClaudeAgentOptions(
    hooks={
        'PreToolUse': [
            HookMatcher(hooks=[rate_limiter]),        # First: check rate limits
            HookMatcher(hooks=[authorization_check]), # Second: verify permissions
            HookMatcher(hooks=[input_sanitizer]),     # Third: sanitize inputs
            HookMatcher(hooks=[audit_logger])         # Last: log the action
        ]
    }
)
```

```typescript TypeScript
const options = {
  hooks: {
    'PreToolUse': [
      { hooks: [rateLimiter] },        // First: check rate limits
      { hooks: [authorizationCheck] }, // Second: verify permissions
      { hooks: [inputSanitizer] },     // Third: sanitize inputs
      { hooks: [auditLogger] }         // Last: log the action
    ]
  }
};
```

</CodeGroup>

### Сопоставители, специфичные для инструмента, с регулярными выражениями

Используйте шаблоны регулярных выражений для сопоставления нескольких инструментов:

<CodeGroup>

```python Python
options = ClaudeAgentOptions(
    hooks={
        'PreToolUse': [
            # Match file modification tools
            HookMatcher(matcher='Write|Edit|Delete', hooks=[file_security_hook]),

            # Match all MCP tools
            HookMatcher(matcher='^mcp__', hooks=[mcp_audit_hook]),

            # Match everything (no matcher)
            HookMatcher(hooks=[global_logger])
        ]
    }
)
```

```typescript TypeScript
const options = {
  hooks: {
    'PreToolUse': [
      // Match file modification tools
      { matcher: 'Write|Edit|Delete', hooks: [fileSecurityHook] },

      // Match all MCP tools
      { matcher: '^mcp__', hooks: [mcpAuditHook] },

      // Match everything (no matcher)
      { hooks: [globalLogger] }
    ]
  }
};
```

</CodeGroup>

<Note>
Сопоставители сопоставляют только **имена инструментов**, а не пути файлов или другие аргументы. Для фильтрации по пути файла проверьте `tool_input.file_path` внутри вашего hook обратного вызова.
</Note>

### Отслеживание активности подагента

Используйте hooks `SubagentStop` для мониторинга завершения подагента. `tool_use_id` помогает коррелировать вызовы родительского агента с их подагентами:

<CodeGroup>

```python Python
async def subagent_tracker(input_data, tool_use_id, context):
    if input_data['hook_event_name'] == 'SubagentStop':
        print(f"[SUBAGENT] Completed")
        print(f"  Tool use ID: {tool_use_id}")
        print(f"  Stop hook active: {input_data.get('stop_hook_active')}")
    return {}

options = ClaudeAgentOptions(
    hooks={
        'SubagentStop': [HookMatcher(hooks=[subagent_tracker])]
    }
)
```

```typescript TypeScript
const subagentTracker: HookCallback = async (input, toolUseID, { signal }) => {
  if (input.hook_event_name === 'SubagentStop') {
    console.log(`[SUBAGENT] Completed`);
    console.log(`  Tool use ID: ${toolUseID}`);
    console.log(`  Stop hook active: ${input.stop_hook_active}`);
  }
  return {};
};

const options = {
  hooks: {
    SubagentStop: [{ hooks: [subagentTracker] }]
  }
};
```

</CodeGroup>

### Асинхронные операции в hooks

Hooks могут выполнять асинхронные операции, такие как HTTP-запросы. Обрабатывайте ошибки корректно, перехватывая исключения вместо их выброса. В TypeScript передайте `signal` в `fetch()`, чтобы запрос отменялся, если hook истечет:

<CodeGroup>

```python Python
import aiohttp
from datetime import datetime

async def webhook_notifier(input_data, tool_use_id, context):
    if input_data['hook_event_name'] != 'PostToolUse':
        return {}

    try:
        async with aiohttp.ClientSession() as session:
            await session.post(
                'https://api.example.com/webhook',
                json={
                    'tool': input_data['tool_name'],
                    'timestamp': datetime.now().isoformat()
                }
            )
    except Exception as e:
        print(f'Webhook request failed: {e}')

    return {}
```

```typescript TypeScript
const webhookNotifier: HookCallback = async (input, toolUseID, { signal }) => {
  if (input.hook_event_name !== 'PostToolUse') return {};

  try {
    // Pass signal for proper cancellation
    await fetch('https://api.example.com/webhook', {
      method: 'POST',
      body: JSON.stringify({
        tool: (input as PostToolUseHookInput).tool_name,
        timestamp: new Date().toISOString()
      }),
      signal
    });
  } catch (error) {
    if (error instanceof Error && error.name === 'AbortError') {
      console.log('Webhook request cancelled');
    }
  }

  return {};
};
```

</CodeGroup>

### Отправка уведомлений (только TypeScript)

Используйте hooks `Notification` для получения обновлений статуса от агента и их перенаправления во внешние сервисы, такие как Slack или панели мониторинга:

```typescript TypeScript
import { query, HookCallback, NotificationHookInput } from "@anthropic-ai/claude-agent-sdk";

const notificationHandler: HookCallback = async (input, toolUseID, { signal }) => {
  const notification = input as NotificationHookInput;

  await fetch('https://hooks.slack.com/services/YOUR/WEBHOOK/URL', {
    method: 'POST',
    body: JSON.stringify({
      text: `Agent status: ${notification.message}`
    }),
    signal
  });

  return {};
};

for await (const message of query({
  prompt: "Analyze this codebase",
  options: {
    hooks: {
      Notification: [{ hooks: [notificationHandler] }]
    }
  }
})) {
  console.log(message);
}
```

## Исправление распространенных проблем

Этот раздел охватывает распространенные проблемы и способы их решения.

### Hook не срабатывает

- Проверьте, что имя события hook правильное и чувствительно к регистру (`PreToolUse`, а не `preToolUse`)
- Проверьте, что ваш шаблон сопоставителя точно совпадает с именем инструмента
- Убедитесь, что hook находится под правильным типом события в `options.hooks`
- Для hooks `SubagentStop`, `Stop`, `SessionStart`, `SessionEnd` и `Notification` сопоставители игнорируются. Эти hooks срабатывают для всех событий этого типа.
- Hooks могут не срабатывать, когда агент достигает лимита [`max_turns`](/docs/ru/agent-sdk/python#configuration-options), потому что сеанс завершается перед выполнением hooks

### Сопоставитель не фильтрует как ожидается

Сопоставители сопоставляют только **имена инструментов**, а не пути файлов или другие аргументы. Для фильтрации по пути файла проверьте `tool_input.file_path` внутри вашего hook:

```typescript
const myHook: HookCallback = async (input, toolUseID, { signal }) => {
  const preInput = input as PreToolUseHookInput;
  const filePath = preInput.tool_input?.file_path as string;
  if (!filePath?.endsWith('.md')) return {};  // Skip non-markdown files
  // Process markdown files...
};
```

### Тайм-аут hook

- Увеличьте значение `timeout` в конфигурации `HookMatcher`
- Используйте `AbortSignal` из третьего аргумента обратного вызова для корректной обработки отмены в TypeScript

### Инструмент заблокирован неожиданно

- Проверьте все hooks `PreToolUse` на возвращение `permissionDecision: 'deny'`
- Добавьте логирование в ваши hooks, чтобы увидеть, какие `permissionDecisionReason` они возвращают
- Проверьте, что шаблоны сопоставителей не слишком широкие (пустой сопоставитель совпадает со всеми инструментами)

### Измененный входной сигнал не применяется

- Убедитесь, что `updatedInput` находится внутри `hookSpecificOutput`, а не на верхнем уровне:

  ```typescript
  return {
    hookSpecificOutput: {
      hookEventName: input.hook_event_name,
      permissionDecision: 'allow',
      updatedInput: { command: 'new command' }
    }
  };
  ```

- Вы также должны вернуть `permissionDecision: 'allow'` для применения изменения входного сигнала
- Включите `hookEventName` в `hookSpecificOutput` для идентификации типа hook для выходных данных

### Hooks сеанса недоступны

Hooks `SessionStart`, `SessionEnd` и `Notification` доступны только в TypeScript SDK. Python SDK не поддерживает эти события из-за ограничений установки.

### Запросы разрешений подагента умножаются

При порождении нескольких подагентов каждый может запросить разрешения отдельно. Подагенты не наследуют автоматически разрешения родительского агента. Чтобы избежать повторяющихся запросов, используйте hooks `PreToolUse` для автоматического одобрения конкретных инструментов или настройте правила разрешений, которые применяются к сеансам подагентов.

### Рекурсивные циклы hook с подагентами

Hook `UserPromptSubmit`, который порождает подагентов, может создать бесконечные циклы, если эти подагенты запускают тот же hook. Чтобы предотвратить это:

- Проверьте наличие индикатора подагента во входных данных hook перед порождением
- Используйте поле `parent_tool_use_id` для обнаружения, находитесь ли вы уже в контексте подагента
- Ограничьте hooks для запуска только для сеанса агента верхнего уровня

### systemMessage не отображается в выходных данных

Поле `systemMessage` добавляет контекст в разговор, который видит модель, но оно может не отображаться во всех режимах вывода SDK. Если вам нужно отобразить решения hook вашему приложению, логируйте их отдельно или используйте выделенный канал вывода.

## Узнать больше

- [Разрешения](/docs/ru/agent-sdk/permissions): контролируйте, что может делать ваш агент
- [Пользовательские инструменты](/docs/ru/agent-sdk/custom-tools): создавайте инструменты для расширения возможностей агента
- [Справочник TypeScript SDK](/docs/ru/agent-sdk/typescript)
- [Справочник Python SDK](/docs/ru/agent-sdk/python)