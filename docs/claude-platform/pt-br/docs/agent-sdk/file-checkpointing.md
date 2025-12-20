# Desfazer alterações de arquivo com checkpointing

Rastreie alterações de arquivo durante sessões de agente e restaure arquivos para qualquer estado anterior

---

O checkpointing de arquivo rastreia modificações de arquivo feitas através das ferramentas Write, Edit e NotebookEdit durante uma sessão de agente, permitindo que você desfaça arquivos para qualquer estado anterior. Quer experimentar? Vá para o [exemplo interativo](#try-it-out).

Com checkpointing, você pode:

- **Desfazer alterações indesejadas** restaurando arquivos para um estado conhecido como bom
- **Explorar alternativas** restaurando para um checkpoint e tentando uma abordagem diferente
- **Recuperar de erros** quando o agente faz modificações incorretas

<Warning>
Apenas alterações feitas através das ferramentas Write, Edit e NotebookEdit são rastreadas. Alterações feitas através de comandos Bash (como `echo > file.txt` ou `sed -i`) não são capturadas pelo sistema de checkpoint.
</Warning>

## Como o checkpointing funciona

Quando você ativa o checkpointing de arquivo, o SDK cria backups de arquivos antes de modificá-los através das ferramentas Write, Edit ou NotebookEdit. Mensagens de usuário no fluxo de resposta incluem um UUID de checkpoint que você pode usar como ponto de restauração.

O checkpoint funciona com essas ferramentas integradas que o agente usa para modificar arquivos:

| Ferramenta | Descrição |
|------|-------------|
| Write | Cria um novo arquivo ou sobrescreve um arquivo existente com novo conteúdo |
| Edit | Faz edições direcionadas em partes específicas de um arquivo existente |
| NotebookEdit | Modifica células em notebooks Jupyter (arquivos `.ipynb`) |

<Note>
A restauração de arquivo restaura arquivos no disco para um estado anterior. Não desfaz a conversa em si. O histórico de conversa e contexto permanecem intactos após chamar `rewindFiles()` (TypeScript) ou `rewind_files()` (Python).
</Note>

O sistema de checkpoint rastreia:

- Arquivos criados durante a sessão
- Arquivos modificados durante a sessão
- O conteúdo original de arquivos modificados

Quando você volta para um checkpoint, arquivos criados são deletados e arquivos modificados são restaurados para seu conteúdo naquele ponto.

## Implementar checkpointing

Para usar checkpointing de arquivo, ative-o em suas opções, capture UUIDs de checkpoint do fluxo de resposta, então chame `rewindFiles()` (TypeScript) ou `rewind_files()` (Python) quando precisar restaurar.

O exemplo a seguir mostra o fluxo completo: ative checkpointing, capture o UUID de checkpoint e ID de sessão do fluxo de resposta, então retome a sessão mais tarde para desfazer arquivos. Cada passo é explicado em detalhes abaixo.

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

<Step title="Defina a variável de ambiente">

O checkpointing de arquivo requer a variável de ambiente `CLAUDE_CODE_ENABLE_SDK_FILE_CHECKPOINTING`. Você pode defini-la via linha de comando antes de executar seu script, ou diretamente nas opções do SDK.

**Opção 1: Defina via linha de comando**

```bash Python
export CLAUDE_CODE_ENABLE_SDK_FILE_CHECKPOINTING=1
```

**Opção 2: Defina nas opções do SDK**

Passe a variável de ambiente através da opção `env` ao configurar o SDK:

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

<Step title="Ative o checkpointing">

Configure suas opções de SDK para ativar checkpointing e receber UUIDs de checkpoint:

| Opção | Python | TypeScript | Descrição |
|--------|--------|------------|-------------|
| Ativar checkpointing | `enable_file_checkpointing=True` | `enableFileCheckpointing: true` | Rastreia alterações de arquivo para desfazer |
| Receber UUIDs de checkpoint | `extra_args={"replay-user-messages": None}` | `extraArgs: { 'replay-user-messages': null }` | Necessário para obter UUIDs de mensagem de usuário no fluxo |

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

<Step title="Capture UUID de checkpoint e ID de sessão">

Com a opção `replay-user-messages` definida (mostrada acima), cada mensagem de usuário no fluxo de resposta tem um UUID que serve como um checkpoint.

Para a maioria dos casos de uso, capture o UUID da primeira mensagem de usuário (`message.uuid`); desfazer para ele restaura todos os arquivos para seu estado original. Para armazenar múltiplos checkpoints e desfazer para estados intermediários, veja [Múltiplos pontos de restauração](#multiple-restore-points).

Capturar o ID de sessão (`message.session_id`) é opcional; você só precisa dele se quiser desfazer mais tarde, após o fluxo ser concluído. Se você estiver chamando `rewindFiles()` imediatamente enquanto ainda processa mensagens (como o exemplo em [Checkpoint antes de operações arriscadas](#checkpoint-before-risky-operations) faz), você pode pular a captura do ID de sessão.

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

<Step title="Desfaça arquivos">

Para desfazer após o fluxo ser concluído, retome a sessão com um prompt vazio e chame `rewind_files()` (Python) ou `rewindFiles()` (TypeScript) com seu UUID de checkpoint. Você também pode desfazer durante o fluxo; veja [Checkpoint antes de operações arriscadas](#checkpoint-before-risky-operations) para esse padrão.

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

Se você capturar o ID de sessão e ID de checkpoint, você também pode desfazer a partir da CLI:

```bash
claude --resume <session-id> --rewind-files <checkpoint-uuid>
```

</Step>

</Steps>

## Padrões comuns

Esses padrões mostram diferentes maneiras de capturar e usar UUIDs de checkpoint dependendo do seu caso de uso.

### Checkpoint antes de operações arriscadas

Este padrão mantém apenas o UUID de checkpoint mais recente, atualizando-o antes de cada turno do agente. Se algo der errado durante o processamento, você pode imediatamente desfazer para o último estado seguro e sair do loop.

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

### Múltiplos pontos de restauração

Se Claude fizer alterações em múltiplos turnos, você pode querer desfazer para um ponto específico em vez de voltar completamente. Por exemplo, se Claude refatorar um arquivo no turno um e adicionar testes no turno dois, você pode querer manter a refatoração mas desfazer os testes.

Este padrão armazena todos os UUIDs de checkpoint em um array com metadados. Após a sessão ser concluída, você pode desfazer para qualquer checkpoint anterior:

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

## Experimente

Este exemplo completo cria um pequeno arquivo utilitário, faz o agente adicionar comentários de documentação, mostra as alterações, então pergunta se você quer desfazer.

Antes de começar, certifique-se de ter o [Claude Agent SDK instalado](/docs/pt-BR/agent-sdk/quickstart).

<Steps>

<Step title="Crie um arquivo de teste">

Crie um novo arquivo chamado `utils.py` (Python) ou `utils.ts` (TypeScript) e cole o seguinte código:

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

<Step title="Execute o exemplo interativo">

Crie um novo arquivo chamado `try_checkpointing.py` (Python) ou `try_checkpointing.ts` (TypeScript) no mesmo diretório que seu arquivo utilitário, e cole o seguinte código.

Este script pede ao Claude para adicionar comentários de documentação ao seu arquivo utilitário, então oferece a opção de desfazer e restaurar o original.

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

Este exemplo demonstra o fluxo de trabalho completo de checkpointing:

1. **Ative checkpointing**: configure o SDK com `enable_file_checkpointing=True` e `permission_mode="acceptEdits"` para aprovar automaticamente edições de arquivo
2. **Capture dados de checkpoint**: conforme o agente é executado, armazene o UUID da primeira mensagem de usuário (seu ponto de restauração) e o ID de sessão
3. **Solicite desfazer**: após o agente terminar, verifique seu arquivo utilitário para ver os comentários de documentação, então decida se você quer desfazer as alterações
4. **Retome e desfaça**: se sim, retome a sessão com um prompt vazio e chame `rewind_files()` para restaurar o arquivo original

</Step>

<Step title="Execute o exemplo">

Defina a variável de ambiente e execute o script do mesmo diretório que seu arquivo utilitário.

<Tip>
Abra seu arquivo utilitário (`utils.py` ou `utils.ts`) em seu IDE ou editor antes de executar o script. Você verá o arquivo atualizar em tempo real conforme o agente adiciona comentários de documentação, então reverter para o original quando você escolher desfazer.
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

Você verá o agente adicionar comentários de documentação, então um prompt perguntando se você quer desfazer. Se você escolher sim, o arquivo é restaurado para seu estado original.

</Step>

</Steps>

## Limitações

O checkpointing de arquivo tem as seguintes limitações:

| Limitação | Descrição |
|------------|-------------|
| Apenas ferramentas Write/Edit/NotebookEdit | Alterações feitas através de comandos Bash não são rastreadas |
| Mesma sessão | Checkpoints estão vinculados à sessão que os criou |
| Apenas conteúdo de arquivo | Criar, mover ou deletar diretórios não é desfeito por desfazer |
| Arquivos locais | Arquivos remotos ou de rede não são rastreados |

## Solução de problemas

### Opções de checkpointing não reconhecidas

Se `enableFileCheckpointing` ou `rewindFiles()` não estiver disponível, você pode estar em uma versão mais antiga do SDK.

**Solução**: Atualize para a versão mais recente do SDK:
- **Python**: `pip install --upgrade claude-agent-sdk`
- **TypeScript**: `npm install @anthropic-ai/claude-agent-sdk@latest`

### Mensagens de usuário não têm UUIDs

Se `message.uuid` for `undefined` ou estiver faltando, você não está recebendo UUIDs de checkpoint.

**Causa**: A opção `replay-user-messages` não está definida.

**Solução**: Adicione `extra_args={"replay-user-messages": None}` (Python) ou `extraArgs: { 'replay-user-messages': null }` (TypeScript) às suas opções.

### Erro "No file checkpoint found for message"

Este erro ocorre quando os dados de checkpoint não existem para o UUID de mensagem de usuário especificado.

**Causas comuns**:
- A variável de ambiente `CLAUDE_CODE_ENABLE_SDK_FILE_CHECKPOINTING` não está definida
- A sessão não foi adequadamente concluída antes de tentar retomar e desfazer

**Solução**: Certifique-se de ter definido a variável de ambiente (veja [Defina a variável de ambiente](#set-the-environment-variable)), então use o padrão mostrado nos exemplos: capture o UUID da primeira mensagem de usuário, conclua a sessão completamente, então retome com um prompt vazio e chame `rewindFiles()` uma vez.

### Erro "ProcessTransport is not ready for writing"

Este erro ocorre quando você chama `rewindFiles()` ou `rewind_files()` após ter terminado de iterar através da resposta. A conexão com o processo CLI fecha quando o loop é concluído.

**Solução**: Retome a sessão com um prompt vazio, então desfaça na nova consulta:

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

## Próximos passos

- **[Sessões](/docs/pt-BR/agent-sdk/sessions)**: aprenda como retomar sessões, que é necessário para desfazer após o fluxo ser concluído. Cobre IDs de sessão, retomada de conversas e bifurcação de sessão.
- **[Permissões](/docs/pt-BR/agent-sdk/permissions)**: configure quais ferramentas Claude pode usar e como modificações de arquivo são aprovadas. Útil se você quiser mais controle sobre quando edições acontecem.
- **[Referência do SDK TypeScript](/docs/pt-BR/agent-sdk/typescript)**: referência completa da API incluindo todas as opções para `query()` e o método `rewindFiles()`.
- **[Referência do SDK Python](/docs/pt-BR/agent-sdk/python)**: referência completa da API incluindo todas as opções para `ClaudeAgentOptions` e o método `rewind_files()`.