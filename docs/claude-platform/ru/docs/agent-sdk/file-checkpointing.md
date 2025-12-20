# Отмотка изменений файлов с помощью контрольных точек

Отслеживайте изменения файлов во время сеансов агента и восстанавливайте файлы в любое предыдущее состояние

---

Контрольные точки файлов отслеживают изменения файлов, внесённые через инструменты Write, Edit и NotebookEdit во время сеанса агента, позволяя вам отмотать файлы в любое предыдущее состояние. Хотите попробовать? Перейдите к [интерактивному примеру](#try-it-out).

С помощью контрольных точек вы можете:

- **Отменить нежелательные изменения**, восстановив файлы в известное хорошее состояние
- **Исследовать альтернативы**, восстановив контрольную точку и попробовав другой подход
- **Восстановиться после ошибок**, когда агент вносит неправильные изменения

<Warning>
Отслеживаются только изменения, внесённые через инструменты Write, Edit и NotebookEdit. Изменения, внесённые через команды Bash (например, `echo > file.txt` или `sed -i`), не фиксируются системой контрольных точек.
</Warning>

## Как работают контрольные точки

Когда вы включаете контрольные точки файлов, SDK создаёт резервные копии файлов перед их изменением через инструменты Write, Edit или NotebookEdit. Пользовательские сообщения в потоке ответов включают UUID контрольной точки, который вы можете использовать как точку восстановления.

Контрольные точки работают с этими встроенными инструментами, которые агент использует для изменения файлов:

| Инструмент | Описание |
|------|-------------|
| Write | Создаёт новый файл или перезаписывает существующий файл новым содержимым |
| Edit | Вносит целевые правки в определённые части существующего файла |
| NotebookEdit | Изменяет ячейки в Jupyter ноутбуках (файлы `.ipynb`) |

<Note>
Отмотка файлов восстанавливает файлы на диске в предыдущее состояние. Она не отматывает саму беседу. История беседы и контекст остаются неизменными после вызова `rewindFiles()` (TypeScript) или `rewind_files()` (Python).
</Note>

Система контрольных точек отслеживает:

- Файлы, созданные во время сеанса
- Файлы, изменённые во время сеанса
- Исходное содержимое изменённых файлов

Когда вы отматываете к контрольной точке, созданные файлы удаляются, а изменённые файлы восстанавливаются до их содержимого в этот момент.

## Реализация контрольных точек

Чтобы использовать контрольные точки файлов, включите их в ваши параметры, захватите UUID контрольных точек из потока ответов, затем вызовите `rewindFiles()` (TypeScript) или `rewind_files()` (Python), когда вам нужно восстановить.

Следующий пример показывает полный процесс: включение контрольных точек, захват UUID контрольной точки и ID сеанса из потока ответов, затем возобновление сеанса позже для отмотки файлов. Каждый шаг подробно объясняется ниже.

<CodeGroup>

```python Python
import asyncio
import os
from claude_agent_sdk import ClaudeSDKClient, ClaudeAgentOptions, UserMessage, ResultMessage

async def main():
    # Step 1: Enable checkpointing
    options = ClaudeAgentOptions(
        enable_file_checkpointing=True,
        permission_mode="acceptEdits",  # Auto-accept file edits without prompting
        extra_args={"replay-user-messages": None},  # Required to receive checkpoint UUIDs in the response stream
        env={**os.environ, "CLAUDE_CODE_ENABLE_SDK_FILE_CHECKPOINTING": "1"}
    )

    checkpoint_id = None
    session_id = None

    # Run the query and capture checkpoint UUID and session ID
    async with ClaudeSDKClient(options) as client:
        await client.query("Refactor the authentication module")

        # Step 2: Capture checkpoint UUID from the first user message
        async for message in client.receive_response():
            if isinstance(message, UserMessage) and message.uuid and not checkpoint_id:
                checkpoint_id = message.uuid
            if isinstance(message, ResultMessage) and not session_id:
                session_id = message.session_id

    # Step 3: Later, rewind by resuming the session with an empty prompt
    if checkpoint_id and session_id:
        async with ClaudeSDKClient(ClaudeAgentOptions(
            enable_file_checkpointing=True,
            resume=session_id
        )) as client:
            await client.query("")  # Empty prompt to open the connection
            async for message in client.receive_response():
                await client.rewind_files(checkpoint_id)
                break
        print(f"Rewound to checkpoint: {checkpoint_id}")

asyncio.run(main())
```

```typescript TypeScript
import { query } from "@anthropic-ai/claude-agent-sdk";

async function main() {
  // Step 1: Enable checkpointing
  const opts = {
    enableFileCheckpointing: true,
    permissionMode: "acceptEdits" as const,  // Auto-accept file edits without prompting
    extraArgs: { 'replay-user-messages': null },  // Required to receive checkpoint UUIDs in the response stream
    env: { ...process.env, CLAUDE_CODE_ENABLE_SDK_FILE_CHECKPOINTING: '1' }
  };

  const response = query({
    prompt: "Refactor the authentication module",
    options: opts
  });

  let checkpointId: string | undefined;
  let sessionId: string | undefined;

  // Step 2: Capture checkpoint UUID from the first user message
  for await (const message of response) {
    if (message.type === 'user' && message.uuid && !checkpointId) {
      checkpointId = message.uuid;
    }
    if ('session_id' in message && !sessionId) {
      sessionId = message.session_id;
    }
  }

  // Step 3: Later, rewind by resuming the session with an empty prompt
  if (checkpointId && sessionId) {
    const rewindQuery = query({
      prompt: "",  // Empty prompt to open the connection
      options: { ...opts, resume: sessionId }
    });

    for await (const msg of rewindQuery) {
      await rewindQuery.rewindFiles(checkpointId);
      break;
    }
    console.log(`Rewound to checkpoint: ${checkpointId}`);
  }
}

main();
```

</CodeGroup>

<Steps>

<Step title="Установите переменную окружения">

Контрольные точки файлов требуют переменную окружения `CLAUDE_CODE_ENABLE_SDK_FILE_CHECKPOINTING`. Вы можете установить её либо через командную строку перед запуском вашего скрипта, либо непосредственно в параметры SDK.

**Вариант 1: Установка через командную строку**

```bash Python
export CLAUDE_CODE_ENABLE_SDK_FILE_CHECKPOINTING=1
```

**Вариант 2: Установка в параметры SDK**

Передайте переменную окружения через параметр `env` при настройке SDK:

<CodeGroup>

```python Python
import os

options = ClaudeAgentOptions(
    enable_file_checkpointing=True,
    env={**os.environ, "CLAUDE_CODE_ENABLE_SDK_FILE_CHECKPOINTING": "1"}
)
```

```typescript TypeScript
const opts = {
  enableFileCheckpointing: true,
  env: { ...process.env, CLAUDE_CODE_ENABLE_SDK_FILE_CHECKPOINTING: '1' }
};
```

</CodeGroup>

</Step>

<Step title="Включите контрольные точки">

Настройте параметры SDK для включения контрольных точек и получения UUID контрольных точек:

| Параметр | Python | TypeScript | Описание |
|--------|--------|------------|-------------|
| Включить контрольные точки | `enable_file_checkpointing=True` | `enableFileCheckpointing: true` | Отслеживает изменения файлов для отмотки |
| Получить UUID контрольных точек | `extra_args={"replay-user-messages": None}` | `extraArgs: { 'replay-user-messages': null }` | Требуется для получения UUID пользовательских сообщений в потоке |

<CodeGroup>

```python Python
options = ClaudeAgentOptions(
    enable_file_checkpointing=True,
    permission_mode="acceptEdits",
    extra_args={"replay-user-messages": None}
)

async with ClaudeSDKClient(options) as client:
    await client.query("Refactor the authentication module")
```

```typescript TypeScript
const response = query({
  prompt: "Refactor the authentication module",
  options: {
    enableFileCheckpointing: true,
    permissionMode: "acceptEdits" as const,
    extraArgs: { 'replay-user-messages': null }
  }
});
```

</CodeGroup>

</Step>

<Step title="Захватите UUID контрольной точки и ID сеанса">

С установленным параметром `replay-user-messages` (показано выше), каждое пользовательское сообщение в потоке ответов имеет UUID, который служит контрольной точкой.

Для большинства случаев захватите UUID первого пользовательского сообщения (`message.uuid`); отмотка к нему восстанавливает все файлы в их исходное состояние. Чтобы сохранить несколько контрольных точек и отмотать к промежуточным состояниям, см. [Несколько точек восстановления](#multiple-restore-points).

Захват ID сеанса (`message.session_id`) является необязательным; вам он нужен только если вы хотите отмотать позже, после завершения потока. Если вы вызываете `rewindFiles()` немедленно, пока всё ещё обрабатываете сообщения (как это делает пример в [Контрольная точка перед рискованными операциями](#checkpoint-before-risky-operations)), вы можете пропустить захват ID сеанса.

<CodeGroup>

```python Python
checkpoint_id = None
session_id = None

async for message in client.receive_response():
    # Update checkpoint on each user message (keeps the latest)
    if isinstance(message, UserMessage) and message.uuid:
        checkpoint_id = message.uuid
    # Capture session ID from the result message
    if isinstance(message, ResultMessage):
        session_id = message.session_id
```

```typescript TypeScript
let checkpointId: string | undefined;
let sessionId: string | undefined;

for await (const message of response) {
  // Update checkpoint on each user message (keeps the latest)
  if (message.type === 'user' && message.uuid) {
    checkpointId = message.uuid;
  }
  // Capture session ID from any message that has it
  if ('session_id' in message) {
    sessionId = message.session_id;
  }
}
```

</CodeGroup>

</Step>

<Step title="Отмотайте файлы">

Чтобы отмотать после завершения потока, возобновите сеанс с пустым приглашением и вызовите `rewind_files()` (Python) или `rewindFiles()` (TypeScript) с вашим UUID контрольной точки. Вы также можете отмотать во время потока; см. [Контрольная точка перед рискованными операциями](#checkpoint-before-risky-operations) для этого паттерна.

<CodeGroup>

```python Python
async with ClaudeSDKClient(ClaudeAgentOptions(
    enable_file_checkpointing=True,
    resume=session_id
)) as client:
    await client.query("")  # Empty prompt to open the connection
    async for message in client.receive_response():
        await client.rewind_files(checkpoint_id)
        break
```

```typescript TypeScript
const rewindQuery = query({
  prompt: "",  // Empty prompt to open the connection
  options: { ...opts, resume: sessionId }
});

for await (const msg of rewindQuery) {
  await rewindQuery.rewindFiles(checkpointId);
  break;
}
```

</CodeGroup>

Если вы захватили ID сеанса и UUID контрольной точки, вы также можете отмотать из CLI:

```bash
claude --resume <session-id> --rewind-files <checkpoint-uuid>
```

</Step>

</Steps>

## Общие паттерны

Эти паттерны показывают различные способы захвата и использования UUID контрольных точек в зависимости от вашего случая использования.

### Контрольная точка перед рискованными операциями

Этот паттерн сохраняет только самый последний UUID контрольной точки, обновляя его перед каждым ходом агента. Если что-то пойдёт не так во время обработки, вы можете немедленно отмотать к последнему безопасному состоянию и выйти из цикла.

<CodeGroup>

```python Python
import asyncio
import os
from claude_agent_sdk import ClaudeSDKClient, ClaudeAgentOptions, UserMessage

async def main():
    options = ClaudeAgentOptions(
        enable_file_checkpointing=True,
        permission_mode="acceptEdits",
        extra_args={"replay-user-messages": None},
        env={**os.environ, "CLAUDE_CODE_ENABLE_SDK_FILE_CHECKPOINTING": "1"}
    )

    safe_checkpoint = None

    async with ClaudeSDKClient(options) as client:
        await client.query("Refactor the authentication module")

        async for message in client.receive_response():
            # Update checkpoint before each agent turn starts
            # This overwrites the previous checkpoint. Only keep the latest
            if isinstance(message, UserMessage) and message.uuid:
                safe_checkpoint = message.uuid

            # Decide when to revert based on your own logic
            # For example: error detection, validation failure, or user input
            if your_revert_condition and safe_checkpoint:
                await client.rewind_files(safe_checkpoint)
                # Exit the loop after rewinding, files are restored
                break

asyncio.run(main())
```

```typescript TypeScript
import { query } from "@anthropic-ai/claude-agent-sdk";

async function main() {
  const response = query({
    prompt: "Refactor the authentication module",
    options: {
      enableFileCheckpointing: true,
      permissionMode: "acceptEdits" as const,
      extraArgs: { 'replay-user-messages': null },
      env: { ...process.env, CLAUDE_CODE_ENABLE_SDK_FILE_CHECKPOINTING: '1' }
    }
  });

  let safeCheckpoint: string | undefined;

  for await (const message of response) {
    // Update checkpoint before each agent turn starts
    // This overwrites the previous checkpoint. Only keep the latest
    if (message.type === 'user' && message.uuid) {
      safeCheckpoint = message.uuid;
    }

    // Decide when to revert based on your own logic
    // For example: error detection, validation failure, or user input
    if (yourRevertCondition && safeCheckpoint) {
      await response.rewindFiles(safeCheckpoint);
      // Exit the loop after rewinding, files are restored
      break;
    }
  }
}

main();
```

</CodeGroup>

### Несколько точек восстановления

Если Claude вносит изменения в несколько ходов, вы можете захотеть отмотать к определённой точке, а не полностью назад. Например, если Claude рефакторит файл в первом ходе и добавляет тесты во втором ходе, вы можете захотеть сохранить рефакторинг, но отменить тесты.

Этот паттерн сохраняет все UUID контрольных точек в массиве с метаданными. После завершения сеанса вы можете отмотать к любой предыдущей контрольной точке:

<CodeGroup>

```python Python
import asyncio
import os
from dataclasses import dataclass
from datetime import datetime
from claude_agent_sdk import ClaudeSDKClient, ClaudeAgentOptions, UserMessage, ResultMessage

# Store checkpoint metadata for better tracking
@dataclass
class Checkpoint:
    id: str
    description: str
    timestamp: datetime

async def main():
    options = ClaudeAgentOptions(
        enable_file_checkpointing=True,
        permission_mode="acceptEdits",
        extra_args={"replay-user-messages": None},
        env={**os.environ, "CLAUDE_CODE_ENABLE_SDK_FILE_CHECKPOINTING": "1"}
    )

    checkpoints = []
    session_id = None

    async with ClaudeSDKClient(options) as client:
        await client.query("Refactor the authentication module")

        async for message in client.receive_response():
            if isinstance(message, UserMessage) and message.uuid:
                checkpoints.append(Checkpoint(
                    id=message.uuid,
                    description=f"After turn {len(checkpoints) + 1}",
                    timestamp=datetime.now()
                ))
            if isinstance(message, ResultMessage) and not session_id:
                session_id = message.session_id

    # Later: rewind to any checkpoint by resuming the session
    if checkpoints and session_id:
        target = checkpoints[0]  # Pick any checkpoint
        async with ClaudeSDKClient(ClaudeAgentOptions(
            enable_file_checkpointing=True,
            resume=session_id
        )) as client:
            await client.query("")  # Empty prompt to open the connection
            async for message in client.receive_response():
                await client.rewind_files(target.id)
                break
        print(f"Rewound to: {target.description}")

asyncio.run(main())
```

```typescript TypeScript
import { query } from "@anthropic-ai/claude-agent-sdk";

// Store checkpoint metadata for better tracking
interface Checkpoint {
  id: string;
  description: string;
  timestamp: Date;
}

async function main() {
  const opts = {
    enableFileCheckpointing: true,
    permissionMode: "acceptEdits" as const,
    extraArgs: { 'replay-user-messages': null },
    env: { ...process.env, CLAUDE_CODE_ENABLE_SDK_FILE_CHECKPOINTING: '1' }
  };

  const response = query({
    prompt: "Refactor the authentication module",
    options: opts
  });

  const checkpoints: Checkpoint[] = [];
  let sessionId: string | undefined;

  for await (const message of response) {
    if (message.type === 'user' && message.uuid) {
      checkpoints.push({
        id: message.uuid,
        description: `After turn ${checkpoints.length + 1}`,
        timestamp: new Date()
      });
    }
    if ('session_id' in message && !sessionId) {
      sessionId = message.session_id;
    }
  }

  // Later: rewind to any checkpoint by resuming the session
  if (checkpoints.length > 0 && sessionId) {
    const target = checkpoints[0];  // Pick any checkpoint
    const rewindQuery = query({
      prompt: "",  // Empty prompt to open the connection
      options: { ...opts, resume: sessionId }
    });

    for await (const msg of rewindQuery) {
      await rewindQuery.rewindFiles(target.id);
      break;
    }
    console.log(`Rewound to: ${target.description}`);
  }
}

main();
```

</CodeGroup>

## Попробуйте

Этот полный пример создаёт небольшой служебный файл, просит агента добавить комментарии к документации, показывает вам изменения, затем спрашивает, хотите ли вы отмотать.

Перед началом убедитесь, что у вас установлен [Claude Agent SDK](/docs/ru/agent-sdk/quickstart).

<Steps>

<Step title="Создайте тестовый файл">

Создайте новый файл с именем `utils.py` (Python) или `utils.ts` (TypeScript) и вставьте следующий код:

<CodeGroup>

```python utils.py
def add(a, b):
    return a + b

def subtract(a, b):
    return a - b

def multiply(a, b):
    return a * b

def divide(a, b):
    if b == 0:
        raise ValueError("Cannot divide by zero")
    return a / b
```

```typescript utils.ts
export function add(a: number, b: number): number {
  return a + b;
}

export function subtract(a: number, b: number): number {
  return a - b;
}

export function multiply(a: number, b: number): number {
  return a * b;
}

export function divide(a: number, b: number): number {
  if (b === 0) {
    throw new Error("Cannot divide by zero");
  }
  return a / b;
}
```

</CodeGroup>

</Step>

<Step title="Запустите интерактивный пример">

Создайте новый файл с именем `try_checkpointing.py` (Python) или `try_checkpointing.ts` (TypeScript) в том же каталоге, что и ваш служебный файл, и вставьте следующий код.

Этот скрипт просит Claude добавить комментарии к документации в ваш служебный файл, затем даёт вам возможность отмотать и восстановить оригинал.

<CodeGroup>

```python try_checkpointing.py
import asyncio
from claude_agent_sdk import ClaudeSDKClient, ClaudeAgentOptions, UserMessage, ResultMessage

async def main():
    # Configure the SDK with checkpointing enabled
    # - enable_file_checkpointing: Track file changes for rewinding
    # - permission_mode: Auto-accept file edits without prompting
    # - extra_args: Required to receive user message UUIDs in the stream
    options = ClaudeAgentOptions(
        enable_file_checkpointing=True,
        permission_mode="acceptEdits",
        extra_args={"replay-user-messages": None}
    )

    checkpoint_id = None  # Store the user message UUID for rewinding
    session_id = None     # Store the session ID for resuming

    print("Running agent to add doc comments to utils.py...\n")

    # Run the agent and capture checkpoint data from the response stream
    async with ClaudeSDKClient(options) as client:
        await client.query("Add doc comments to utils.py")

        async for message in client.receive_response():
            # Capture the first user message UUID - this is our restore point
            if isinstance(message, UserMessage) and message.uuid and not checkpoint_id:
                checkpoint_id = message.uuid
            # Capture the session ID so we can resume later
            if isinstance(message, ResultMessage):
                session_id = message.session_id

    print("Done! Open utils.py to see the added doc comments.\n")

    # Ask the user if they want to rewind the changes
    if checkpoint_id and session_id:
        response = input("Rewind to remove the doc comments? (y/n): ")

        if response.lower() == "y":
            # Resume the session with an empty prompt, then rewind
            async with ClaudeSDKClient(ClaudeAgentOptions(
                enable_file_checkpointing=True,
                resume=session_id
            )) as client:
                await client.query("")  # Empty prompt opens the connection
                async for message in client.receive_response():
                    await client.rewind_files(checkpoint_id)  # Restore files
                    break

            print("\n✓ File restored! Open utils.py to verify the doc comments are gone.")
        else:
            print("\nKept the modified file.")

asyncio.run(main())
```

```typescript try_checkpointing.ts
import { query } from "@anthropic-ai/claude-agent-sdk";
import * as readline from "readline";

async function main() {
  // Configure the SDK with checkpointing enabled
  // - enableFileCheckpointing: Track file changes for rewinding
  // - permissionMode: Auto-accept file edits without prompting
  // - extraArgs: Required to receive user message UUIDs in the stream
  const opts = {
    enableFileCheckpointing: true,
    permissionMode: "acceptEdits" as const,
    extraArgs: { 'replay-user-messages': null }
  };

  let sessionId: string | undefined;    // Store the session ID for resuming
  let checkpointId: string | undefined; // Store the user message UUID for rewinding

  console.log("Running agent to add doc comments to utils.ts...\n");

  // Run the agent and capture checkpoint data from the response stream
  const response = query({
    prompt: "Add doc comments to utils.ts",
    options: opts
  });

  for await (const message of response) {
    // Capture the first user message UUID - this is our restore point
    if (message.type === "user" && message.uuid && !checkpointId) {
      checkpointId = message.uuid;
    }
    // Capture the session ID so we can resume later
    if ("session_id" in message) {
      sessionId = message.session_id;
    }
  }

  console.log("Done! Open utils.ts to see the added doc comments.\n");

  // Ask the user if they want to rewind the changes
  if (checkpointId && sessionId) {
    const rl = readline.createInterface({
      input: process.stdin,
      output: process.stdout
    });

    const answer = await new Promise<string>((resolve) => {
      rl.question("Rewind to remove the doc comments? (y/n): ", resolve);
    });
    rl.close();

    if (answer.toLowerCase() === "y") {
      // Resume the session with an empty prompt, then rewind
      const rewindQuery = query({
        prompt: "",  // Empty prompt opens the connection
        options: { ...opts, resume: sessionId }
      });

      for await (const msg of rewindQuery) {
        await rewindQuery.rewindFiles(checkpointId);  // Restore files
        break;
      }

      console.log("\n✓ File restored! Open utils.ts to verify the doc comments are gone.");
    } else {
      console.log("\nKept the modified file.");
    }
  }
}

main();
```

</CodeGroup>

Этот пример демонстрирует полный рабочий процесс контрольных точек:

1. **Включите контрольные точки**: настройте SDK с `enable_file_checkpointing=True` и `permission_mode="acceptEdits"` для автоматического одобрения редактирования файлов
2. **Захватите данные контрольной точки**: по мере запуска агента сохраняйте UUID первого пользовательского сообщения (вашу точку восстановления) и ID сеанса
3. **Запросите отмотку**: после завершения работы агента проверьте ваш служебный файл, чтобы увидеть комментарии к документации, затем решите, хотите ли вы отменить изменения
4. **Возобновите и отмотайте**: если да, возобновите сеанс с пустым приглашением и вызовите `rewind_files()` для восстановления исходного файла

</Step>

<Step title="Запустите пример">

Установите переменную окружения и запустите скрипт из того же каталога, что и ваш служебный файл.

<Tip>
Откройте ваш служебный файл (`utils.py` или `utils.ts`) в вашей IDE или редакторе перед запуском скрипта. Вы увидите, как файл обновляется в реальном времени, когда агент добавляет комментарии к документации, затем вернётся в исходное состояние, когда вы выберете отмотку.
</Tip>

<CodeGroup>

```bash Python
export CLAUDE_CODE_ENABLE_SDK_FILE_CHECKPOINTING=1
python try_checkpointing.py
```

```bash TypeScript
export CLAUDE_CODE_ENABLE_SDK_FILE_CHECKPOINTING=1
npx tsx try_checkpointing.ts
```

</CodeGroup>

Вы увидите, как агент добавляет комментарии к документации, затем появится приглашение, спрашивающее, хотите ли вы отмотать. Если вы выберете да, файл будет восстановлен в его исходное состояние.

</Step>

</Steps>

## Ограничения

Контрольные точки файлов имеют следующие ограничения:

| Ограничение | Описание |
|------------|-------------|
| Только инструменты Write/Edit/NotebookEdit | Изменения, внесённые через команды Bash, не отслеживаются |
| Один сеанс | Контрольные точки привязаны к сеансу, который их создал |
| Только содержимое файла | Создание, перемещение или удаление каталогов не отменяется отмоткой |
| Локальные файлы | Удалённые или сетевые файлы не отслеживаются |

## Устранение неполадок

### Параметры контрольных точек не распознаны

Если `enableFileCheckpointing` или `rewindFiles()` недоступны, вы можете использовать старую версию SDK.

**Решение**: Обновитесь до последней версии SDK:
- **Python**: `pip install --upgrade claude-agent-sdk`
- **TypeScript**: `npm install @anthropic-ai/claude-agent-sdk@latest`

### Пользовательские сообщения не имеют UUID

Если `message.uuid` имеет значение `undefined` или отсутствует, вы не получаете UUID контрольных точек.

**Причина**: Параметр `replay-user-messages` не установлен.

**Решение**: Добавьте `extra_args={"replay-user-messages": None}` (Python) или `extraArgs: { 'replay-user-messages': null }` (TypeScript) в ваши параметры.

### Ошибка "No file checkpoint found for message"

Эта ошибка возникает, когда данные контрольной точки не существуют для указанного UUID пользовательского сообщения.

**Частые причины**:
- Переменная окружения `CLAUDE_CODE_ENABLE_SDK_FILE_CHECKPOINTING` не установлена
- Сеанс не был должным образом завершён перед попыткой возобновления и отмотки

**Решение**: Убедитесь, что вы установили переменную окружения (см. [Установите переменную окружения](#set-the-environment-variable)), затем используйте паттерн, показанный в примерах: захватите UUID первого пользовательского сообщения, полностью завершите сеанс, затем возобновите с пустым приглашением и вызовите `rewindFiles()` один раз.

### Ошибка "ProcessTransport is not ready for writing"

Эта ошибка возникает, когда вы вызываете `rewindFiles()` или `rewind_files()` после завершения итерации по ответу. Соединение с процессом CLI закрывается при завершении цикла.

**Решение**: Возобновите сеанс с пустым приглашением, затем отмотайте в новом запросе:

<CodeGroup>

```python Python
# Resume session with empty prompt, then rewind
async with ClaudeSDKClient(ClaudeAgentOptions(
    enable_file_checkpointing=True,
    resume=session_id
)) as client:
    await client.query("")
    async for message in client.receive_response():
        await client.rewind_files(checkpoint_id)
        break
```

```typescript TypeScript
// Resume session with empty prompt, then rewind
const rewindQuery = query({
  prompt: "",
  options: { ...opts, resume: sessionId }
});

for await (const msg of rewindQuery) {
  await rewindQuery.rewindFiles(checkpointId);
  break;
}
```

</CodeGroup>

## Следующие шаги

- **[Сеансы](/docs/ru/agent-sdk/sessions)**: узнайте, как возобновлять сеансы, что требуется для отмотки после завершения потока. Охватывает ID сеансов, возобновление бесед и разветвление сеансов.
- **[Разрешения](/docs/ru/agent-sdk/permissions)**: настройте, какие инструменты может использовать Claude и как одобряются изменения файлов. Полезно, если вы хотите больше контроля над тем, когда происходят правки.
- **[Справочник TypeScript SDK](/docs/ru/agent-sdk/typescript)**: полный справочник API, включая все параметры для `query()` и метода `rewindFiles()`.
- **[Справочник Python SDK](/docs/ru/agent-sdk/python)**: полный справочник API, включая все параметры для `ClaudeAgentOptions` и метода `rewind_files()`.