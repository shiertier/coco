# Interceptar y controlar el comportamiento del agente con hooks

Interceptar y personalizar el comportamiento del agente en puntos clave de ejecución con hooks

---

Los hooks te permiten interceptar la ejecución del agente en puntos clave para agregar validación, registro, controles de seguridad o lógica personalizada. Con hooks, puedes:

- **Bloquear operaciones peligrosas** antes de que se ejecuten, como comandos de shell destructivos o acceso a archivos no autorizado
- **Registrar y auditar** cada llamada de herramienta para cumplimiento, depuración o análisis
- **Transformar entradas y salidas** para desinfectar datos, inyectar credenciales o redirigir rutas de archivos
- **Requerir aprobación humana** para acciones sensibles como escrituras en bases de datos o llamadas a API
- **Rastrear el ciclo de vida de la sesión** para gestionar estado, limpiar recursos o enviar notificaciones

Un hook tiene dos partes:

1. **La función de devolución de llamada**: la lógica que se ejecuta cuando se dispara el hook
2. **La configuración del hook**: le dice al SDK qué evento enganchar (como `PreToolUse`) y qué herramientas coinciden

El siguiente ejemplo bloquea al agente de modificar archivos `.env`. Primero, define una devolución de llamada que verifica la ruta del archivo, luego pásala a `query()` para ejecutarla antes de cualquier llamada de herramienta Write o Edit:

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

Este es un hook `PreToolUse`. Se ejecuta antes de que la herramienta se ejecute y puede bloquear u permitir operaciones basadas en tu lógica. El resto de esta guía cubre todos los hooks disponibles, sus opciones de configuración y patrones para casos de uso comunes.

## Hooks disponibles

El SDK proporciona hooks para diferentes etapas de la ejecución del agente. Algunos hooks están disponibles en ambos SDK, mientras que otros son solo de TypeScript porque el SDK de Python no los admite.

| Hook Event | Python SDK | TypeScript SDK | What triggers it | Example use case |
|------------|------------|----------------|------------------|------------------|
| `PreToolUse` | Yes | Yes | Tool call request (can block or modify) | Block dangerous shell commands |
| `PostToolUse` | Yes | Yes | Tool execution result | Log all file changes to audit trail |
| `PostToolUseFailure` | No | Yes | Tool execution failure | Handle or log tool errors |
| `UserPromptSubmit` | Yes | Yes | User prompt submission | Inject additional context into prompts |
| `Stop` | Yes | Yes | Agent execution stop | Save session state before exit |
| `SubagentStart` | No | Yes | Subagent initialization | Track parallel task spawning |
| `SubagentStop` | Yes | Yes | Subagent completion | Aggregate results from parallel tasks |
| `PreCompact` | Yes | Yes | Conversation compaction request | Archive full transcript before summarizing |
| `PermissionRequest` | No | Yes | Permission dialog would be displayed | Custom permission handling |
| `SessionStart` | No | Yes | Session initialization | Initialize logging and telemetry |
| `SessionEnd` | No | Yes | Session termination | Clean up temporary resources |
| `Notification` | No | Yes | Agent status messages | Send agent status updates to Slack or PagerDuty |

## Casos de uso comunes

Los hooks son lo suficientemente flexibles para manejar muchos escenarios diferentes. Aquí hay algunos de los patrones más comunes organizados por categoría.

<Tabs>
  <Tab title="Seguridad">
    - Bloquear comandos peligrosos (como `rm -rf /`, SQL destructivo)
    - Validar rutas de archivo antes de operaciones de escritura
    - Aplicar listas de permitidos/bloqueados para el uso de herramientas
  </Tab>
  <Tab title="Registro">
    - Crear registros de auditoría de todas las acciones del agente
    - Rastrear métricas de ejecución y rendimiento
    - Depurar el comportamiento del agente en desarrollo
  </Tab>
  <Tab title="Intercepción de herramientas">
    - Redirigir operaciones de archivo a directorios aislados
    - Inyectar variables de entorno o credenciales
    - Transformar entradas o salidas de herramientas
  </Tab>
  <Tab title="Autorización">
    - Implementar control de acceso basado en roles
    - Requerir aprobación humana para operaciones sensibles
    - Limitar la velocidad de uso de herramientas específicas
  </Tab>
</Tabs>

## Configurar hooks

Para configurar un hook para tu agente, pasa el hook en el parámetro `options.hooks` al llamar a `query()`:

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

La opción `hooks` es un diccionario (Python) u objeto (TypeScript) donde:
- **Las claves** son [nombres de eventos de hook](#available-hooks) (por ejemplo, `'PreToolUse'`, `'PostToolUse'`, `'Stop'`)
- **Los valores** son matrices de [matchers](#matchers), cada una conteniendo un patrón de filtro opcional y tus [funciones de devolución de llamada](#callback-function-inputs)

Tus funciones de devolución de llamada de hook reciben [datos de entrada](#input-data) sobre el evento y devuelven una [respuesta](#callback-outputs) para que el agente sepa si permitir, bloquear o modificar la operación.

### Matchers

Usa matchers para filtrar qué herramientas disparan tus devoluciones de llamada:

| Option | Type | Default | Description |
|--------|------|---------|-------------|
| `matcher` | `string` | `undefined` | Regex pattern to match tool names. Built-in tools include `Bash`, `Read`, `Write`, `Edit`, `Glob`, `Grep`, `WebFetch`, `Task`, and others. MCP tools use the pattern `mcp__<server>__<action>`. |
| `hooks` | `HookCallback[]` | - | Required. Array of callback functions to execute when the pattern matches |
| `timeout` | `number` | `60` | Timeout in seconds; increase for hooks that make external API calls |

Usa el patrón `matcher` para dirigirse a herramientas específicas siempre que sea posible. Un matcher con `'Bash'` solo se ejecuta para comandos Bash, mientras que omitir el patrón ejecuta tus devoluciones de llamada para cada llamada de herramienta. Ten en cuenta que los matchers solo filtran por **nombre de herramienta**, no por rutas de archivo u otros argumentos—para filtrar por ruta de archivo, verifica `tool_input.file_path` dentro de tu devolución de llamada.

Los matchers solo se aplican a hooks basados en herramientas (`PreToolUse`, `PostToolUse`, `PostToolUseFailure`, `PermissionRequest`). Para hooks de ciclo de vida como `Stop`, `SessionStart` y `Notification`, los matchers se ignoran y el hook se dispara para todos los eventos de ese tipo.

<Tip>
**Descubriendo nombres de herramientas:** Verifica la matriz `tools` en el mensaje del sistema inicial cuando comienza tu sesión, o agrega un hook sin un matcher para registrar todas las llamadas de herramientas.

**Nomenclatura de herramientas MCP:** Las herramientas MCP siempre comienzan con `mcp__` seguidas del nombre del servidor y la acción: `mcp__<server>__<action>`. Por ejemplo, si configuras un servidor llamado `playwright`, sus herramientas se nombrarán `mcp__playwright__browser_screenshot`, `mcp__playwright__browser_click`, etc. El nombre del servidor proviene de la clave que usas en la configuración de `mcpServers`.
</Tip>

Este ejemplo usa un matcher para ejecutar un hook solo para herramientas que modifican archivos cuando se dispara el evento `PreToolUse`:

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

### Entradas de función de devolución de llamada

Cada devolución de llamada de hook recibe tres argumentos:

1. **Datos de entrada** (`dict` / `HookInput`): Detalles del evento. Consulta [datos de entrada](#input-data) para campos
2. **ID de uso de herramienta** (`str | None` / `string | null`): Correlacionar eventos `PreToolUse` y `PostToolUse`
3. **Contexto** (`HookContext`): En TypeScript, contiene una propiedad `signal` (`AbortSignal`) para cancelación. Pasa esto a operaciones asincrónicas como `fetch()` para que se cancelen automáticamente si el hook agota el tiempo. En Python, este argumento está reservado para uso futuro.

### Datos de entrada

El primer argumento de tu devolución de llamada de hook contiene información sobre el evento. Los nombres de campo son idénticos en todos los SDK (ambos usan snake_case).

**Campos comunes** presentes en todos los tipos de hook:

| Field | Type | Description |
|-------|------|-------------|
| `hook_event_name` | `string` | The hook type (`PreToolUse`, `PostToolUse`, etc.) |
| `session_id` | `string` | Current session identifier |
| `transcript_path` | `string` | Path to the conversation transcript |
| `cwd` | `string` | Current working directory |

**Campos específicos del hook** varían según el tipo de hook. Los elementos marcados <sup>TS</sup> solo están disponibles en el SDK de TypeScript:

| Field | Type | Description | Hooks |
|-------|------|-------------|-------|
| `tool_name` | `string` | Name of the tool being called | PreToolUse, PostToolUse, PostToolUseFailure<sup>TS</sup>, PermissionRequest<sup>TS</sup> |
| `tool_input` | `object` | Arguments passed to the tool | PreToolUse, PostToolUse, PostToolUseFailure<sup>TS</sup>, PermissionRequest<sup>TS</sup> |
| `tool_response` | `any` | Result returned from tool execution | PostToolUse |
| `error` | `string` | Error message from tool execution failure | PostToolUseFailure<sup>TS</sup> |
| `is_interrupt` | `boolean` | Whether the failure was caused by an interrupt | PostToolUseFailure<sup>TS</sup> |
| `prompt` | `string` | The user's prompt text | UserPromptSubmit |
| `stop_hook_active` | `boolean` | Whether a stop hook is currently processing | Stop, SubagentStop |
| `agent_id` | `string` | Unique identifier for the subagent | SubagentStart<sup>TS</sup>, SubagentStop<sup>TS</sup> |
| `agent_type` | `string` | Type/role of the subagent | SubagentStart<sup>TS</sup> |
| `agent_transcript_path` | `string` | Path to the subagent's conversation transcript | SubagentStop<sup>TS</sup> |
| `trigger` | `string` | What triggered compaction: `manual` or `auto` | PreCompact |
| `custom_instructions` | `string` | Custom instructions provided for compaction | PreCompact |
| `permission_suggestions` | `array` | Suggested permission updates for the tool | PermissionRequest<sup>TS</sup> |
| `source` | `string` | How the session started: `startup`, `resume`, `clear`, or `compact` | SessionStart<sup>TS</sup> |
| `reason` | `string` | Why the session ended: `clear`, `logout`, `prompt_input_exit`, `bypass_permissions_disabled`, or `other` | SessionEnd<sup>TS</sup> |
| `message` | `string` | Status message from the agent | Notification<sup>TS</sup> |
| `notification_type` | `string` | Type of notification: `permission_prompt`, `idle_prompt`, `auth_success`, or `elicitation_dialog` | Notification<sup>TS</sup> |
| `title` | `string` | Optional title set by the agent | Notification<sup>TS</sup> |

El código a continuación define una devolución de llamada de hook que usa `tool_name` y `tool_input` para registrar detalles sobre cada llamada de herramienta:

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

### Salidas de devolución de llamada

Tu función de devolución de llamada devuelve un objeto que le dice al SDK cómo proceder. Devuelve un objeto vacío `{}` para permitir la operación sin cambios. Para bloquear, modificar o agregar contexto a la operación, devuelve un objeto con un campo `hookSpecificOutput` que contiene tu decisión.

**Campos de nivel superior** (fuera de `hookSpecificOutput`):

| Field | Type | Description |
|-------|------|-------------|
| `continue` | `boolean` | Whether the agent should continue after this hook (default: `true`) |
| `stopReason` | `string` | Message shown when `continue` is `false` |
| `suppressOutput` | `boolean` | Hide stdout from the transcript (default: `false`) |
| `systemMessage` | `string` | Message injected into the conversation for Claude to see |

**Campos dentro de `hookSpecificOutput`**:

| Field | Type | Hooks | Description |
|-------|------|-------|-------------|
| `hookEventName` | `string` | All | Required. Use `input.hook_event_name` to match the current event |
| `permissionDecision` | `'allow'` \| `'deny'` \| `'ask'` | PreToolUse | Controls whether the tool executes |
| `permissionDecisionReason` | `string` | PreToolUse | Explanation shown to Claude for the decision |
| `updatedInput` | `object` | PreToolUse | Modified tool input (requires `permissionDecision: 'allow'`) |
| `additionalContext` | `string` | PostToolUse, UserPromptSubmit, SessionStart<sup>TS</sup>, SubagentStart<sup>TS</sup> | Context added to the conversation |

Este ejemplo bloquea operaciones de escritura en el directorio `/etc` mientras inyecta un mensaje del sistema para recordarle a Claude sobre prácticas seguras de archivos:

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

#### Flujo de decisión de permisos

Cuando se aplican múltiples hooks o reglas de permisos, el SDK los evalúa en este orden:

1. **Las reglas de negación** se verifican primero (cualquier coincidencia = negación inmediata).
2. **Las reglas de pregunta** se verifican segundo.
3. **Las reglas de permitir** se verifican tercero.
4. **Por defecto a Preguntar** si nada coincide.

Si algún hook devuelve `deny`, la operación se bloquea—otros hooks que devuelven `allow` no lo anularán.

#### Bloquear una herramienta

Devuelve una decisión de negación para evitar la ejecución de la herramienta:

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

#### Modificar entrada de herramienta

Devuelve entrada actualizada para cambiar lo que recibe la herramienta:

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
Cuando uses `updatedInput`, también debes incluir `permissionDecision`. Siempre devuelve un nuevo objeto en lugar de mutar el `tool_input` original.
</Note>

#### Agregar un mensaje del sistema

Inyecta contexto en la conversación:

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

#### Aprobar automáticamente herramientas específicas

Omite solicitudes de permisos para herramientas de confianza. Esto es útil cuando deseas que ciertas operaciones se ejecuten sin confirmación del usuario:

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
El campo `permissionDecision` acepta tres valores: `'allow'` (aprobar automáticamente), `'deny'` (bloquear) o `'ask'` (solicitar confirmación).
</Note>

## Manejar escenarios avanzados

Estos patrones te ayudan a construir sistemas de hooks más sofisticados para casos de uso complejos.

### Encadenar múltiples hooks

Los hooks se ejecutan en el orden en que aparecen en la matriz. Mantén cada hook enfocado en una única responsabilidad y encadena múltiples hooks para lógica compleja. Este ejemplo ejecuta los cuatro hooks para cada llamada de herramienta (sin matcher especificado):

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

### Matchers específicos de herramientas con regex

Usa patrones regex para coincidir con múltiples herramientas:

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
Los matchers solo coinciden con **nombres de herramientas**, no con rutas de archivo u otros argumentos. Para filtrar por ruta de archivo, verifica `tool_input.file_path` dentro de tu devolución de llamada de hook.
</Note>

### Rastrear actividad de subagentes

Usa hooks `SubagentStop` para monitorear la finalización de subagentes. El `tool_use_id` ayuda a correlacionar llamadas de agente padre con sus subagentes:

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

### Operaciones asincrónicas en hooks

Los hooks pueden realizar operaciones asincrónicas como solicitudes HTTP. Maneja errores correctamente capturando excepciones en lugar de lanzarlas. En TypeScript, pasa el `signal` a `fetch()` para que la solicitud se cancele si el hook agota el tiempo:

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

### Enviar notificaciones (solo TypeScript)

Usa hooks `Notification` para recibir actualizaciones de estado del agente y reenviarlas a servicios externos como Slack o paneles de monitoreo:

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

## Solucionar problemas comunes

Esta sección cubre problemas comunes y cómo resolverlos.

### Hook no se dispara

- Verifica que el nombre del evento del hook sea correcto y sensible a mayúsculas (`PreToolUse`, no `preToolUse`)
- Comprueba que tu patrón de matcher coincida exactamente con el nombre de la herramienta
- Asegúrate de que el hook esté bajo el tipo de evento correcto en `options.hooks`
- Para hooks `SubagentStop`, `Stop`, `SessionStart`, `SessionEnd` y `Notification`, los matchers se ignoran. Estos hooks se disparan para todos los eventos de ese tipo.
- Los hooks pueden no dispararse cuando el agente alcanza el límite de [`max_turns`](/docs/es/agent-sdk/python#configuration-options) porque la sesión termina antes de que los hooks puedan ejecutarse

### Matcher no filtra como se esperaba

Los matchers solo coinciden con **nombres de herramientas**, no con rutas de archivo u otros argumentos. Para filtrar por ruta de archivo, verifica `tool_input.file_path` dentro de tu hook:

```typescript
const myHook: HookCallback = async (input, toolUseID, { signal }) => {
  const preInput = input as PreToolUseHookInput;
  const filePath = preInput.tool_input?.file_path as string;
  if (!filePath?.endsWith('.md')) return {};  // Skip non-markdown files
  // Process markdown files...
};
```

### Tiempo de espera del hook

- Aumenta el valor `timeout` en la configuración de `HookMatcher`
- Usa el `AbortSignal` del tercer argumento de devolución de llamada para manejar la cancelación correctamente en TypeScript

### Herramienta bloqueada inesperadamente

- Verifica todos los hooks `PreToolUse` para devoluciones `permissionDecision: 'deny'`
- Agrega registro a tus hooks para ver qué `permissionDecisionReason` están devolviendo
- Verifica que los patrones de matcher no sean demasiado amplios (un matcher vacío coincide con todas las herramientas)

### Entrada modificada no aplicada

- Asegúrate de que `updatedInput` esté dentro de `hookSpecificOutput`, no en el nivel superior:

  ```typescript
  return {
    hookSpecificOutput: {
      hookEventName: input.hook_event_name,
      permissionDecision: 'allow',
      updatedInput: { command: 'new command' }
    }
  };
  ```

- También debes devolver `permissionDecision: 'allow'` para que la modificación de entrada surta efecto
- Incluye `hookEventName` en `hookSpecificOutput` para identificar qué tipo de hook es la salida

### Hooks de sesión no disponibles

Los hooks `SessionStart`, `SessionEnd` y `Notification` solo están disponibles en el SDK de TypeScript. El SDK de Python no admite estos eventos debido a limitaciones de configuración.

### Solicitudes de permisos de subagentes multiplicándose

Al generar múltiples subagentes, cada uno puede solicitar permisos por separado. Los subagentes no heredan automáticamente los permisos del agente padre. Para evitar solicitudes repetidas, usa hooks `PreToolUse` para aprobar automáticamente herramientas específicas, o configura reglas de permisos que se apliquen a sesiones de subagentes.

### Bucles recursivos de hooks con subagentes

Un hook `UserPromptSubmit` que genera subagentes puede crear bucles infinitos si esos subagentes disparan el mismo hook. Para evitar esto:

- Verifica un indicador de subagente en la entrada del hook antes de generar
- Usa el campo `parent_tool_use_id` para detectar si ya estás en un contexto de subagente
- Limita los hooks para que solo se ejecuten en la sesión del agente de nivel superior

### systemMessage no aparece en la salida

El campo `systemMessage` agrega contexto a la conversación que el modelo ve, pero puede no aparecer en todos los modos de salida del SDK. Si necesitas mostrar decisiones de hooks a tu aplicación, regístralas por separado o usa un canal de salida dedicado.

## Aprende más

- [Permisos](/docs/es/agent-sdk/permissions): controla qué puede hacer tu agente
- [Herramientas personalizadas](/docs/es/agent-sdk/custom-tools): construye herramientas para extender las capacidades del agente
- [Referencia del SDK de TypeScript](/docs/es/agent-sdk/typescript)
- [Referencia del SDK de Python](/docs/es/agent-sdk/python)