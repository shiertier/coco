# Interceptar e controlar o comportamento do agente com hooks

Interceptar e personalizar o comportamento do agente em pontos-chave de execução com hooks

---

Hooks permitem que você intercepte a execução do agente em pontos-chave para adicionar validação, logging, controles de segurança ou lógica personalizada. Com hooks, você pode:

- **Bloquear operações perigosas** antes de serem executadas, como comandos shell destrutivos ou acesso a arquivos não autorizado
- **Registrar e auditar** cada chamada de ferramenta para conformidade, depuração ou análise
- **Transformar entradas e saídas** para sanitizar dados, injetar credenciais ou redirecionar caminhos de arquivo
- **Exigir aprovação humana** para ações sensíveis como gravações em banco de dados ou chamadas de API
- **Rastrear ciclo de vida da sessão** para gerenciar estado, limpar recursos ou enviar notificações

Um hook tem duas partes:

1. **A função de callback**: a lógica que é executada quando o hook dispara
2. **A configuração do hook**: informa ao SDK qual evento conectar (como `PreToolUse`) e quais ferramentas corresponder

O exemplo a seguir bloqueia o agente de modificar arquivos `.env`. Primeiro, defina um callback que verifica o caminho do arquivo, depois passe-o para `query()` para executar antes de qualquer chamada de ferramenta Write ou Edit:

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

Este é um hook `PreToolUse`. Ele é executado antes da ferramenta ser executada e pode bloquear ou permitir operações com base em sua lógica. O restante deste guia cobre todos os hooks disponíveis, suas opções de configuração e padrões para casos de uso comuns.

## Hooks disponíveis

O SDK fornece hooks para diferentes estágios de execução do agente. Alguns hooks estão disponíveis em ambos os SDKs, enquanto outros são apenas para TypeScript porque o SDK Python não os suporta.

| Hook Event | Python SDK | TypeScript SDK | O que dispara | Caso de uso de exemplo |
|------------|------------|----------------|------------------|------------------|
| `PreToolUse` | Sim | Sim | Solicitação de chamada de ferramenta (pode bloquear ou modificar) | Bloquear comandos shell perigosos |
| `PostToolUse` | Sim | Sim | Resultado da execução da ferramenta | Registrar todas as alterações de arquivo na trilha de auditoria |
| `PostToolUseFailure` | Não | Sim | Falha na execução da ferramenta | Lidar ou registrar erros de ferramenta |
| `UserPromptSubmit` | Sim | Sim | Envio de prompt do usuário | Injetar contexto adicional em prompts |
| `Stop` | Sim | Sim | Parada de execução do agente | Salvar estado da sessão antes de sair |
| `SubagentStart` | Não | Sim | Inicialização de subagentte | Rastrear geração de tarefas paralelas |
| `SubagentStop` | Sim | Sim | Conclusão de subagente | Agregar resultados de tarefas paralelas |
| `PreCompact` | Sim | Sim | Solicitação de compactação de conversa | Arquivar transcrição completa antes de resumir |
| `PermissionRequest` | Não | Sim | Diálogo de permissão seria exibido | Manipulação de permissão personalizada |
| `SessionStart` | Não | Sim | Inicialização de sessão | Inicializar logging e telemetria |
| `SessionEnd` | Não | Sim | Encerramento de sessão | Limpar recursos temporários |
| `Notification` | Não | Sim | Mensagens de status do agente | Enviar atualizações de status do agente para Slack ou PagerDuty |

## Casos de uso comuns

Hooks são flexíveis o suficiente para lidar com muitos cenários diferentes. Aqui estão alguns dos padrões mais comuns organizados por categoria.

<Tabs>
  <Tab title="Segurança">
    - Bloquear comandos perigosos (como `rm -rf /`, SQL destrutivo)
    - Validar caminhos de arquivo antes de operações de gravação
    - Aplicar listas de permissão/bloqueio para uso de ferramentas
  </Tab>
  <Tab title="Logging">
    - Criar trilhas de auditoria de todas as ações do agente
    - Rastrear métricas de execução e desempenho
    - Depurar comportamento do agente em desenvolvimento
  </Tab>
  <Tab title="Interceptação de ferramenta">
    - Redirecionar operações de arquivo para diretórios em sandbox
    - Injetar variáveis de ambiente ou credenciais
    - Transformar entradas ou saídas de ferramentas
  </Tab>
  <Tab title="Autorização">
    - Implementar controle de acesso baseado em função
    - Exigir aprovação humana para operações sensíveis
    - Limitar taxa de uso específico de ferramenta
  </Tab>
</Tabs>

## Configurar hooks

Para configurar um hook para seu agente, passe o hook no parâmetro `options.hooks` ao chamar `query()`:

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

A opção `hooks` é um dicionário (Python) ou objeto (TypeScript) onde:
- **Chaves** são [nomes de eventos de hook](#available-hooks) (por exemplo, `'PreToolUse'`, `'PostToolUse'`, `'Stop'`)
- **Valores** são arrays de [matchers](#matchers), cada um contendo um padrão de filtro opcional e suas [funções de callback](#callback-function-inputs)

Suas funções de callback de hook recebem [dados de entrada](#input-data) sobre o evento e retornam uma [resposta](#callback-outputs) para que o agente saiba permitir, bloquear ou modificar a operação.

### Matchers

Use matchers para filtrar quais ferramentas disparam seus callbacks:

| Opção | Tipo | Padrão | Descrição |
|--------|------|---------|-------------|
| `matcher` | `string` | `undefined` | Padrão regex para corresponder nomes de ferramentas. As ferramentas integradas incluem `Bash`, `Read`, `Write`, `Edit`, `Glob`, `Grep`, `WebFetch`, `Task` e outras. As ferramentas MCP usam o padrão `mcp__<server>__<action>`. |
| `hooks` | `HookCallback[]` | - | Obrigatório. Array de funções de callback para executar quando o padrão corresponder |
| `timeout` | `number` | `60` | Timeout em segundos; aumente para hooks que fazem chamadas de API externas |

Use o padrão `matcher` para direcionar ferramentas específicas sempre que possível. Um matcher com `'Bash'` é executado apenas para comandos Bash, enquanto omitir o padrão executa seus callbacks para cada chamada de ferramenta. Observe que matchers apenas filtram por **nome da ferramenta**, não por caminhos de arquivo ou outros argumentos—para filtrar por caminho de arquivo, verifique `tool_input.file_path` dentro de seu callback.

Matchers se aplicam apenas a hooks baseados em ferramentas (`PreToolUse`, `PostToolUse`, `PostToolUseFailure`, `PermissionRequest`). Para hooks de ciclo de vida como `Stop`, `SessionStart` e `Notification`, matchers são ignorados e o hook dispara para todos os eventos desse tipo.

<Tip>
**Descobrindo nomes de ferramentas:** Verifique o array `tools` na mensagem do sistema inicial quando sua sessão começar, ou adicione um hook sem um matcher para registrar todas as chamadas de ferramenta.

**Nomenclatura de ferramentas MCP:** As ferramentas MCP sempre começam com `mcp__` seguidas pelo nome do servidor e ação: `mcp__<server>__<action>`. Por exemplo, se você configurar um servidor chamado `playwright`, suas ferramentas serão nomeadas `mcp__playwright__browser_screenshot`, `mcp__playwright__browser_click`, etc. O nome do servidor vem da chave que você usa na configuração `mcpServers`.
</Tip>

Este exemplo usa um matcher para executar um hook apenas para ferramentas que modificam arquivos quando o evento `PreToolUse` dispara:

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

### Entradas de função de callback

Cada callback de hook recebe três argumentos:

1. **Dados de entrada** (`dict` / `HookInput`): Detalhes do evento. Veja [dados de entrada](#input-data) para campos
2. **ID de uso de ferramenta** (`str | None` / `string | null`): Correlacionar eventos `PreToolUse` e `PostToolUse`
3. **Contexto** (`HookContext`): Em TypeScript, contém uma propriedade `signal` (`AbortSignal`) para cancelamento. Passe isso para operações assíncronas como `fetch()` para que elas se cancelem automaticamente se o hook expirar. Em Python, este argumento é reservado para uso futuro.

### Dados de entrada

O primeiro argumento para seu callback de hook contém informações sobre o evento. Os nomes dos campos são idênticos entre SDKs (ambos usam snake_case).

**Campos comuns** presentes em todos os tipos de hook:

| Campo | Tipo | Descrição |
|-------|------|-------------|
| `hook_event_name` | `string` | O tipo de hook (`PreToolUse`, `PostToolUse`, etc.) |
| `session_id` | `string` | Identificador de sessão atual |
| `transcript_path` | `string` | Caminho para a transcrição da conversa |
| `cwd` | `string` | Diretório de trabalho atual |

**Campos específicos do hook** variam por tipo de hook. Itens marcados <sup>TS</sup> estão disponíveis apenas no SDK TypeScript:

| Campo | Tipo | Descrição | Hooks |
|-------|------|-------------|-------|
| `tool_name` | `string` | Nome da ferramenta sendo chamada | PreToolUse, PostToolUse, PostToolUseFailure<sup>TS</sup>, PermissionRequest<sup>TS</sup> |
| `tool_input` | `object` | Argumentos passados para a ferramenta | PreToolUse, PostToolUse, PostToolUseFailure<sup>TS</sup>, PermissionRequest<sup>TS</sup> |
| `tool_response` | `any` | Resultado retornado da execução da ferramenta | PostToolUse |
| `error` | `string` | Mensagem de erro da falha na execução da ferramenta | PostToolUseFailure<sup>TS</sup> |
| `is_interrupt` | `boolean` | Se a falha foi causada por uma interrupção | PostToolUseFailure<sup>TS</sup> |
| `prompt` | `string` | O texto do prompt do usuário | UserPromptSubmit |
| `stop_hook_active` | `boolean` | Se um hook de parada está sendo processado | Stop, SubagentStop |
| `agent_id` | `string` | Identificador único para o subagente | SubagentStart<sup>TS</sup>, SubagentStop<sup>TS</sup> |
| `agent_type` | `string` | Tipo/função do subagente | SubagentStart<sup>TS</sup> |
| `agent_transcript_path` | `string` | Caminho para a transcrição da conversa do subagente | SubagentStop<sup>TS</sup> |
| `trigger` | `string` | O que disparou a compactação: `manual` ou `auto` | PreCompact |
| `custom_instructions` | `string` | Instruções personalizadas fornecidas para compactação | PreCompact |
| `permission_suggestions` | `array` | Atualizações de permissão sugeridas para a ferramenta | PermissionRequest<sup>TS</sup> |
| `source` | `string` | Como a sessão começou: `startup`, `resume`, `clear` ou `compact` | SessionStart<sup>TS</sup> |
| `reason` | `string` | Por que a sessão terminou: `clear`, `logout`, `prompt_input_exit`, `bypass_permissions_disabled` ou `other` | SessionEnd<sup>TS</sup> |
| `message` | `string` | Mensagem de status do agente | Notification<sup>TS</sup> |
| `notification_type` | `string` | Tipo de notificação: `permission_prompt`, `idle_prompt`, `auth_success` ou `elicitation_dialog` | Notification<sup>TS</sup> |
| `title` | `string` | Título opcional definido pelo agente | Notification<sup>TS</sup> |

O código abaixo define um callback de hook que usa `tool_name` e `tool_input` para registrar detalhes sobre cada chamada de ferramenta:

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

### Saídas de callback

Sua função de callback retorna um objeto que informa ao SDK como proceder. Retorne um objeto vazio `{}` para permitir a operação sem alterações. Para bloquear, modificar ou adicionar contexto à operação, retorne um objeto com um campo `hookSpecificOutput` contendo sua decisão.

**Campos de nível superior** (fora de `hookSpecificOutput`):

| Campo | Tipo | Descrição |
|-------|------|-------------|
| `continue` | `boolean` | Se o agente deve continuar após este hook (padrão: `true`) |
| `stopReason` | `string` | Mensagem mostrada quando `continue` é `false` |
| `suppressOutput` | `boolean` | Ocultar stdout da transcrição (padrão: `false`) |
| `systemMessage` | `string` | Mensagem injetada na conversa para Claude ver |

**Campos dentro de `hookSpecificOutput`**:

| Campo | Tipo | Hooks | Descrição |
|-------|------|-------|-------------|
| `hookEventName` | `string` | Todos | Obrigatório. Use `input.hook_event_name` para corresponder ao evento atual |
| `permissionDecision` | `'allow'` \| `'deny'` \| `'ask'` | PreToolUse | Controla se a ferramenta é executada |
| `permissionDecisionReason` | `string` | PreToolUse | Explicação mostrada a Claude para a decisão |
| `updatedInput` | `object` | PreToolUse | Entrada de ferramenta modificada (requer `permissionDecision: 'allow'`) |
| `additionalContext` | `string` | PostToolUse, UserPromptSubmit, SessionStart<sup>TS</sup>, SubagentStart<sup>TS</sup> | Contexto adicionado à conversa |

Este exemplo bloqueia operações de gravação no diretório `/etc` enquanto injeta uma mensagem do sistema para lembrar Claude sobre práticas seguras de arquivo:

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

#### Fluxo de decisão de permissão

Quando múltiplos hooks ou regras de permissão se aplicam, o SDK os avalia nesta ordem:

1. **Regras de negação** são verificadas primeiro (qualquer correspondência = negação imediata).
2. **Regras de pergunta** são verificadas em segundo lugar.
3. **Regras de permissão** são verificadas em terceiro lugar.
4. **Padrão para pergunta** se nada corresponder.

Se qualquer hook retornar `deny`, a operação é bloqueada—outros hooks retornando `allow` não a substituirão.

#### Bloquear uma ferramenta

Retorne uma decisão de negação para impedir a execução da ferramenta:

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

#### Modificar entrada de ferramenta

Retorne entrada atualizada para alterar o que a ferramenta recebe:

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
Ao usar `updatedInput`, você também deve incluir `permissionDecision`. Sempre retorne um novo objeto em vez de mutar o `tool_input` original.
</Note>

#### Adicionar uma mensagem do sistema

Injetar contexto na conversa:

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

#### Aprovar automaticamente ferramentas específicas

Contornar prompts de permissão para ferramentas confiáveis. Isso é útil quando você quer que certas operações sejam executadas sem confirmação do usuário:

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
O campo `permissionDecision` aceita três valores: `'allow'` (aprovar automaticamente), `'deny'` (bloquear) ou `'ask'` (solicitar confirmação).
</Note>

## Lidar com cenários avançados

Esses padrões ajudam você a construir sistemas de hook mais sofisticados para casos de uso complexos.

### Encadeando múltiplos hooks

Hooks são executados na ordem em que aparecem no array. Mantenha cada hook focado em uma única responsabilidade e encadeie múltiplos hooks para lógica complexa. Este exemplo executa todos os quatro hooks para cada chamada de ferramenta (nenhum matcher especificado):

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

### Matchers de ferramentas específicas com regex

Use padrões regex para corresponder múltiplas ferramentas:

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
Matchers apenas correspondem **nomes de ferramentas**, não caminhos de arquivo ou outros argumentos. Para filtrar por caminho de arquivo, verifique `tool_input.file_path` dentro de seu callback de hook.
</Note>

### Rastreando atividade de subagente

Use hooks `SubagentStop` para monitorar a conclusão de subagente. O `tool_use_id` ajuda a correlacionar chamadas de agente pai com seus subagentes:

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

### Operações assíncronas em hooks

Hooks podem realizar operações assíncronas como requisições HTTP. Lidar com erros graciosamente capturando exceções em vez de lançá-las. Em TypeScript, passe o `signal` para `fetch()` para que a requisição seja cancelada se o hook expirar:

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

### Enviando notificações (apenas TypeScript)

Use hooks `Notification` para receber atualizações de status do agente e encaminhá-las para serviços externos como Slack ou painéis de monitoramento:

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

## Corrigir problemas comuns

Esta seção cobre problemas comuns e como resolvê-los.

### Hook não dispara

- Verifique se o nome do evento do hook está correto e sensível a maiúsculas (`PreToolUse`, não `preToolUse`)
- Verifique se seu padrão de matcher corresponde ao nome da ferramenta exatamente
- Certifique-se de que o hook está sob o tipo de evento correto em `options.hooks`
- Para hooks `SubagentStop`, `Stop`, `SessionStart`, `SessionEnd` e `Notification`, matchers são ignorados. Esses hooks disparam para todos os eventos desse tipo.
- Hooks podem não disparar quando o agente atinge o limite [`max_turns`](/docs/pt-BR/agent-sdk/python#configuration-options) porque a sessão termina antes dos hooks poderem ser executados

### Matcher não filtrando conforme esperado

Matchers apenas correspondem **nomes de ferramentas**, não caminhos de arquivo ou outros argumentos. Para filtrar por caminho de arquivo, verifique `tool_input.file_path` dentro de seu hook:

```typescript
const myHook: HookCallback = async (input, toolUseID, { signal }) => {
  const preInput = input as PreToolUseHookInput;
  const filePath = preInput.tool_input?.file_path as string;
  if (!filePath?.endsWith('.md')) return {};  // Skip non-markdown files
  // Process markdown files...
};
```

### Hook timeout

- Aumente o valor `timeout` na configuração `HookMatcher`
- Use o `AbortSignal` do terceiro argumento de callback para lidar com cancelamento graciosamente em TypeScript

### Ferramenta bloqueada inesperadamente

- Verifique todos os hooks `PreToolUse` para retornos `permissionDecision: 'deny'`
- Adicione logging aos seus hooks para ver qual `permissionDecisionReason` eles estão retornando
- Verifique se os padrões de matcher não são muito amplos (um matcher vazio corresponde a todas as ferramentas)

### Entrada modificada não aplicada

- Certifique-se de que `updatedInput` está dentro de `hookSpecificOutput`, não no nível superior:

  ```typescript
  return {
    hookSpecificOutput: {
      hookEventName: input.hook_event_name,
      permissionDecision: 'allow',
      updatedInput: { command: 'new command' }
    }
  };
  ```

- Você também deve retornar `permissionDecision: 'allow'` para que a modificação de entrada tenha efeito
- Inclua `hookEventName` em `hookSpecificOutput` para identificar qual tipo de hook a saída é

### Hooks de sessão não disponíveis

Hooks `SessionStart`, `SessionEnd` e `Notification` estão disponíveis apenas no SDK TypeScript. O SDK Python não suporta esses eventos devido a limitações de configuração.

### Prompts de permissão de subagente se multiplicando

Ao gerar múltiplos subagentes, cada um pode solicitar permissões separadamente. Subagentes não herdam automaticamente permissões de agente pai. Para evitar prompts repetidos, use hooks `PreToolUse` para aprovar automaticamente ferramentas específicas, ou configure regras de permissão que se apliquem a sessões de subagente.

### Loops recursivos de hook com subagentes

Um hook `UserPromptSubmit` que gera subagentes pode criar loops infinitos se esses subagentes disparem o mesmo hook. Para evitar isso:

- Verifique um indicador de subagente na entrada do hook antes de gerar
- Use o campo `parent_tool_use_id` para detectar se você já está em um contexto de subagente
- Escope hooks para executar apenas para a sessão de agente de nível superior

### systemMessage não aparecendo na saída

O campo `systemMessage` adiciona contexto à conversa que o modelo vê, mas pode não aparecer em todos os modos de saída do SDK. Se você precisar exibir decisões de hook para sua aplicação, registre-as separadamente ou use um canal de saída dedicado.

## Saiba mais

- [Permissões](/docs/pt-BR/agent-sdk/permissions): controlar o que seu agente pode fazer
- [Ferramentas Personalizadas](/docs/pt-BR/agent-sdk/custom-tools): construir ferramentas para estender capacidades do agente
- [Referência do SDK TypeScript](/docs/pt-BR/agent-sdk/typescript)
- [Referência do SDK Python](/docs/pt-BR/agent-sdk/python)