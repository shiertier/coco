# Agent SDK reference - TypeScript

Riferimento API completo per l'Agent SDK TypeScript, incluse tutte le funzioni, i tipi e le interfacce.

---

<script src="/components/typescript-sdk-type-links.js" defer />

<Note>
**Prova la nuova interfaccia V2 (anteprima):** Un'interfaccia semplificata con pattern `send()` e `receive()` è ora disponibile, rendendo le conversazioni multi-turno più facili. [Scopri di più](/docs/it/agent-sdk/typescript-v2-preview)
</Note>

## Installazione

```bash
npm install @anthropic-ai/claude-agent-sdk
```

## Funzioni

### `query()`

La funzione principale per interagire con Claude Code. Crea un generatore asincrono che trasmette i messaggi man mano che arrivano.

```typescript
function query({
  prompt,
  options
}: {
  prompt: string | AsyncIterable<SDKUserMessage>;
  options?: Options;
}): Query
```

#### Parametri

| Parametro | Tipo | Descrizione |
| :-------- | :--- | :---------- |
| `prompt` | `string \| AsyncIterable<`[`SDKUserMessage`](#sdkusermessage)`>` | Il prompt di input come stringa o iterabile asincrono per la modalità streaming |
| `options` | [`Options`](#options) | Oggetto di configurazione facoltativo (vedi il tipo Options di seguito) |

#### Restituisce

Restituisce un oggetto [`Query`](#query-1) che estende `AsyncGenerator<`[`SDKMessage`](#sdkmessage)`, void>` con metodi aggiuntivi.

### `tool()`

Crea una definizione di strumento MCP type-safe per l'uso con i server MCP dell'SDK.

```typescript
function tool<Schema extends ZodRawShape>(
  name: string,
  description: string,
  inputSchema: Schema,
  handler: (args: z.infer<ZodObject<Schema>>, extra: unknown) => Promise<CallToolResult>
): SdkMcpToolDefinition<Schema>
```

#### Parametri

| Parametro | Tipo | Descrizione |
| :-------- | :--- | :---------- |
| `name` | `string` | Il nome dello strumento |
| `description` | `string` | Una descrizione di cosa fa lo strumento |
| `inputSchema` | `Schema extends ZodRawShape` | Schema Zod che definisce i parametri di input dello strumento |
| `handler` | `(args, extra) => Promise<`[`CallToolResult`](#calltoolresult)`>` | Funzione asincrona che esegue la logica dello strumento |

### `createSdkMcpServer()`

Crea un'istanza di server MCP che viene eseguita nello stesso processo della tua applicazione.

```typescript
function createSdkMcpServer(options: {
  name: string;
  version?: string;
  tools?: Array<SdkMcpToolDefinition<any>>;
}): McpSdkServerConfigWithInstance
```

#### Parametri

| Parametro | Tipo | Descrizione |
| :-------- | :--- | :---------- |
| `options.name` | `string` | Il nome del server MCP |
| `options.version` | `string` | Stringa di versione facoltativa |
| `options.tools` | `Array<SdkMcpToolDefinition>` | Array di definizioni di strumenti creati con [`tool()`](#tool) |

## Tipi

### `Options`

Oggetto di configurazione per la funzione `query()`.

| Proprietà | Tipo | Predefinito | Descrizione |
| :------- | :--- | :------ | :---------- |
| `abortController` | `AbortController` | `new AbortController()` | Controller per annullare le operazioni |
| `additionalDirectories` | `string[]` | `[]` | Directory aggiuntive a cui Claude può accedere |
| `agents` | `Record<string, [`AgentDefinition`](#agentdefinition)>` | `undefined` | Definisci i subagenti a livello di programmazione |
| `allowDangerouslySkipPermissions` | `boolean` | `false` | Abilita il bypass dei permessi. Richiesto quando si usa `permissionMode: 'bypassPermissions'` |
| `allowedTools` | `string[]` | Tutti gli strumenti | Elenco dei nomi degli strumenti consentiti |
| `betas` | [`SdkBeta`](#sdkbeta)`[]` | `[]` | Abilita le funzioni beta (ad es. `['context-1m-2025-08-07']`) |
| `canUseTool` | [`CanUseTool`](#canusetool) | `undefined` | Funzione di permesso personalizzata per l'uso dello strumento |
| `continue` | `boolean` | `false` | Continua la conversazione più recente |
| `cwd` | `string` | `process.cwd()` | Directory di lavoro corrente |
| `disallowedTools` | `string[]` | `[]` | Elenco dei nomi degli strumenti non consentiti |
| `enableFileCheckpointing` | `boolean` | `false` | Abilita il tracciamento dei cambiamenti di file per il riavvolgimento. Vedi [File checkpointing](/docs/it/agent-sdk/file-checkpointing) |
| `env` | `Dict<string>` | `process.env` | Variabili di ambiente |
| `executable` | `'bun' \| 'deno' \| 'node'` | Rilevamento automatico | Runtime JavaScript da utilizzare |
| `executableArgs` | `string[]` | `[]` | Argomenti da passare all'eseguibile |
| `extraArgs` | `Record<string, string \| null>` | `{}` | Argomenti aggiuntivi |
| `fallbackModel` | `string` | `undefined` | Modello da utilizzare se il primario fallisce |
| `forkSession` | `boolean` | `false` | Quando si riprende con `resume`, esegui il fork a un nuovo ID di sessione invece di continuare la sessione originale |
| `hooks` | `Partial<Record<`[`HookEvent`](#hookevent)`, `[`HookCallbackMatcher`](#hookcallbackmatcher)`[]>>` | `{}` | Callback hook per gli eventi |
| `includePartialMessages` | `boolean` | `false` | Includi gli eventi di messaggi parziali |
| `maxBudgetUsd` | `number` | `undefined` | Budget massimo in USD per la query |
| `maxThinkingTokens` | `number` | `undefined` | Token massimi per il processo di pensiero |
| `maxTurns` | `number` | `undefined` | Turni di conversazione massimi |
| `mcpServers` | `Record<string, [`McpServerConfig`](#mcpserverconfig)>` | `{}` | Configurazioni del server MCP |
| `model` | `string` | Predefinito da CLI | Modello Claude da utilizzare |
| `outputFormat` | `{ type: 'json_schema', schema: JSONSchema }` | `undefined` | Definisci il formato di output per i risultati dell'agente. Vedi [Output strutturati](/docs/it/agent-sdk/structured-outputs) per i dettagli |
| `pathToClaudeCodeExecutable` | `string` | Usa l'eseguibile integrato | Percorso all'eseguibile Claude Code |
| `permissionMode` | [`PermissionMode`](#permissionmode) | `'default'` | Modalità di permesso per la sessione |
| `permissionPromptToolName` | `string` | `undefined` | Nome dello strumento MCP per i prompt di permesso |
| `plugins` | [`SdkPluginConfig`](#sdkpluginconfig)`[]` | `[]` | Carica plugin personalizzati da percorsi locali. Vedi [Plugin](/docs/it/agent-sdk/plugins) per i dettagli |
| `resume` | `string` | `undefined` | ID della sessione da riprendere |
| `resumeSessionAt` | `string` | `undefined` | Riprendi la sessione a un UUID di messaggio specifico |
| `sandbox` | [`SandboxSettings`](#sandboxsettings) | `undefined` | Configura il comportamento della sandbox a livello di programmazione. Vedi [Impostazioni sandbox](#sandboxsettings) per i dettagli |
| `settingSources` | [`SettingSource`](#settingsource)`[]` | `[]` (nessuna impostazione) | Controlla quali impostazioni basate su filesystem caricare. Se omesso, nessuna impostazione viene caricata. **Nota:** Deve includere `'project'` per caricare i file CLAUDE.md |
| `stderr` | `(data: string) => void` | `undefined` | Callback per l'output stderr |
| `strictMcpConfig` | `boolean` | `false` | Applica la convalida MCP rigorosa |
| `systemPrompt` | `string \| { type: 'preset'; preset: 'claude_code'; append?: string }` | `undefined` (prompt vuoto) | Configurazione del prompt di sistema. Passa una stringa per un prompt personalizzato, o `{ type: 'preset', preset: 'claude_code' }` per usare il prompt di sistema di Claude Code. Quando si usa la forma dell'oggetto preset, aggiungi `append` per estendere il prompt di sistema con istruzioni aggiuntive |
| `tools` | `string[] \| { type: 'preset'; preset: 'claude_code' }` | `undefined` | Configurazione dello strumento. Passa un array di nomi di strumenti o usa il preset per ottenere gli strumenti predefiniti di Claude Code |

### `Query`

Interfaccia restituita dalla funzione `query()`.

```typescript
interface Query extends AsyncGenerator<SDKMessage, void> {
  interrupt(): Promise<void>;
  rewindFiles(userMessageUuid: string): Promise<void>;
  setPermissionMode(mode: PermissionMode): Promise<void>;
  setModel(model?: string): Promise<void>;
  setMaxThinkingTokens(maxThinkingTokens: number | null): Promise<void>;
  supportedCommands(): Promise<SlashCommand[]>;
  supportedModels(): Promise<ModelInfo[]>;
  mcpServerStatus(): Promise<McpServerStatus[]>;
  accountInfo(): Promise<AccountInfo>;
}
```

#### Metodi

| Metodo | Descrizione |
| :----- | :---------- |
| `interrupt()` | Interrompe la query (disponibile solo in modalità input streaming) |
| `rewindFiles(userMessageUuid)` | Ripristina i file al loro stato al messaggio utente specificato. Richiede `enableFileCheckpointing: true`. Vedi [File checkpointing](/docs/it/agent-sdk/file-checkpointing) |
| `setPermissionMode()` | Cambia la modalità di permesso (disponibile solo in modalità input streaming) |
| `setModel()` | Cambia il modello (disponibile solo in modalità input streaming) |
| `setMaxThinkingTokens()` | Cambia i token di pensiero massimi (disponibile solo in modalità input streaming) |
| `supportedCommands()` | Restituisce i comandi slash disponibili |
| `supportedModels()` | Restituisce i modelli disponibili con informazioni di visualizzazione |
| `mcpServerStatus()` | Restituisce lo stato dei server MCP connessi |
| `accountInfo()` | Restituisce le informazioni dell'account |

### `AgentDefinition`

Configurazione per un subagente definito a livello di programmazione.

```typescript
type AgentDefinition = {
  description: string;
  tools?: string[];
  prompt: string;
  model?: 'sonnet' | 'opus' | 'haiku' | 'inherit';
}
```

| Campo | Obbligatorio | Descrizione |
|:------|:---------|:------------|
| `description` | Sì | Descrizione in linguaggio naturale di quando usare questo agente |
| `tools` | No | Array di nomi di strumenti consentiti. Se omesso, eredita tutti gli strumenti |
| `prompt` | Sì | Il prompt di sistema dell'agente |
| `model` | No | Override del modello per questo agente. Se omesso, usa il modello principale |

### `SettingSource`

Controlla quali fonti di configurazione basate su filesystem l'SDK carica le impostazioni da.

```typescript
type SettingSource = 'user' | 'project' | 'local';
```

| Valore | Descrizione | Posizione |
|:------|:------------|:---------|
| `'user'` | Impostazioni globali dell'utente | `~/.claude/settings.json` |
| `'project'` | Impostazioni di progetto condivise (controllate dalla versione) | `.claude/settings.json` |
| `'local'` | Impostazioni di progetto locali (gitignorate) | `.claude/settings.local.json` |

#### Comportamento predefinito

Quando `settingSources` è **omesso** o **undefined**, l'SDK **non** carica alcuna impostazione del filesystem. Questo fornisce isolamento per le applicazioni SDK.

#### Perché usare settingSources?

**Carica tutte le impostazioni del filesystem (comportamento legacy):**
```typescript
// Carica tutte le impostazioni come faceva l'SDK v0.0.x
const result = query({
  prompt: "Analizza questo codice",
  options: {
    settingSources: ['user', 'project', 'local']  // Carica tutte le impostazioni
  }
});
```

**Carica solo fonti di impostazioni specifiche:**
```typescript
// Carica solo le impostazioni di progetto, ignora quelle utente e locali
const result = query({
  prompt: "Esegui i controlli CI",
  options: {
    settingSources: ['project']  // Solo .claude/settings.json
  }
});
```

**Ambienti di test e CI:**
```typescript
// Assicura un comportamento coerente in CI escludendo le impostazioni locali
const result = query({
  prompt: "Esegui i test",
  options: {
    settingSources: ['project'],  // Solo impostazioni condivise dal team
    permissionMode: 'bypassPermissions'
  }
});
```

**Applicazioni solo SDK:**
```typescript
// Definisci tutto a livello di programmazione (comportamento predefinito)
// Nessuna dipendenza dal filesystem - settingSources predefinito è []
const result = query({
  prompt: "Rivedi questo PR",
  options: {
    // settingSources: [] è il predefinito, non è necessario specificare
    agents: { /* ... */ },
    mcpServers: { /* ... */ },
    allowedTools: ['Read', 'Grep', 'Glob']
  }
});
```

**Caricamento delle istruzioni di progetto CLAUDE.md:**
```typescript
// Carica le impostazioni di progetto per includere i file CLAUDE.md
const result = query({
  prompt: "Aggiungi una nuova funzione seguendo le convenzioni del progetto",
  options: {
    systemPrompt: {
      type: 'preset',
      preset: 'claude_code'  // Richiesto per usare CLAUDE.md
    },
    settingSources: ['project'],  // Carica CLAUDE.md dalla directory del progetto
    allowedTools: ['Read', 'Write', 'Edit']
  }
});
```

#### Precedenza delle impostazioni

Quando più fonti vengono caricate, le impostazioni vengono unite con questa precedenza (da più alta a più bassa):
1. Impostazioni locali (`.claude/settings.local.json`)
2. Impostazioni di progetto (`.claude/settings.json`)
3. Impostazioni utente (`~/.claude/settings.json`)

Le opzioni programmatiche (come `agents`, `allowedTools`) sovrascrivono sempre le impostazioni del filesystem.

### `PermissionMode`

```typescript
type PermissionMode =
  | 'default'           // Comportamento di permesso standard
  | 'acceptEdits'       // Auto-accetta le modifiche ai file
  | 'bypassPermissions' // Bypass di tutti i controlli di permesso
  | 'plan'              // Modalità di pianificazione - nessuna esecuzione
```

### `CanUseTool`

Tipo di funzione di permesso personalizzato per controllare l'uso dello strumento.

```typescript
type CanUseTool = (
  toolName: string,
  input: ToolInput,
  options: {
    signal: AbortSignal;
    suggestions?: PermissionUpdate[];
  }
) => Promise<PermissionResult>;
```

### `PermissionResult`

Risultato di un controllo di permesso.

```typescript
type PermissionResult = 
  | {
      behavior: 'allow';
      updatedInput: ToolInput;
      updatedPermissions?: PermissionUpdate[];
    }
  | {
      behavior: 'deny';
      message: string;
      interrupt?: boolean;
    }
```

### `McpServerConfig`

Configurazione per i server MCP.

```typescript
type McpServerConfig = 
  | McpStdioServerConfig
  | McpSSEServerConfig
  | McpHttpServerConfig
  | McpSdkServerConfigWithInstance;
```

#### `McpStdioServerConfig`

```typescript
type McpStdioServerConfig = {
  type?: 'stdio';
  command: string;
  args?: string[];
  env?: Record<string, string>;
}
```

#### `McpSSEServerConfig`

```typescript
type McpSSEServerConfig = {
  type: 'sse';
  url: string;
  headers?: Record<string, string>;
}
```

#### `McpHttpServerConfig`

```typescript
type McpHttpServerConfig = {
  type: 'http';
  url: string;
  headers?: Record<string, string>;
}
```

#### `McpSdkServerConfigWithInstance`

```typescript
type McpSdkServerConfigWithInstance = {
  type: 'sdk';
  name: string;
  instance: McpServer;
}
```

### `SdkPluginConfig`

Configurazione per il caricamento dei plugin nell'SDK.

```typescript
type SdkPluginConfig = {
  type: 'local';
  path: string;
}
```

| Campo | Tipo | Descrizione |
|:------|:-----|:------------|
| `type` | `'local'` | Deve essere `'local'` (attualmente sono supportati solo i plugin locali) |
| `path` | `string` | Percorso assoluto o relativo alla directory del plugin |

**Esempio:**
```typescript
plugins: [
  { type: 'local', path: './my-plugin' },
  { type: 'local', path: '/absolute/path/to/plugin' }
]
```

Per informazioni complete sulla creazione e l'uso dei plugin, vedi [Plugin](/docs/it/agent-sdk/plugins).

## Tipi di messaggio

### `SDKMessage`

Tipo di unione di tutti i possibili messaggi restituiti dalla query.

```typescript
type SDKMessage = 
  | SDKAssistantMessage
  | SDKUserMessage
  | SDKUserMessageReplay
  | SDKResultMessage
  | SDKSystemMessage
  | SDKPartialAssistantMessage
  | SDKCompactBoundaryMessage;
```

### `SDKAssistantMessage`

Messaggio di risposta dell'assistente.

```typescript
type SDKAssistantMessage = {
  type: 'assistant';
  uuid: UUID;
  session_id: string;
  message: APIAssistantMessage; // Da Anthropic SDK
  parent_tool_use_id: string | null;
}
```

### `SDKUserMessage`

Messaggio di input dell'utente.

```typescript
type SDKUserMessage = {
  type: 'user';
  uuid?: UUID;
  session_id: string;
  message: APIUserMessage; // Da Anthropic SDK
  parent_tool_use_id: string | null;
}
```

### `SDKUserMessageReplay`

Messaggio utente riprodotto con UUID richiesto.

```typescript
type SDKUserMessageReplay = {
  type: 'user';
  uuid: UUID;
  session_id: string;
  message: APIUserMessage;
  parent_tool_use_id: string | null;
}
```

### `SDKResultMessage`

Messaggio di risultato finale.

```typescript
type SDKResultMessage =
  | {
      type: 'result';
      subtype: 'success';
      uuid: UUID;
      session_id: string;
      duration_ms: number;
      duration_api_ms: number;
      is_error: boolean;
      num_turns: number;
      result: string;
      total_cost_usd: number;
      usage: NonNullableUsage;
      modelUsage: { [modelName: string]: ModelUsage };
      permission_denials: SDKPermissionDenial[];
      structured_output?: unknown;
    }
  | {
      type: 'result';
      subtype:
        | 'error_max_turns'
        | 'error_during_execution'
        | 'error_max_budget_usd'
        | 'error_max_structured_output_retries';
      uuid: UUID;
      session_id: string;
      duration_ms: number;
      duration_api_ms: number;
      is_error: boolean;
      num_turns: number;
      total_cost_usd: number;
      usage: NonNullableUsage;
      modelUsage: { [modelName: string]: ModelUsage };
      permission_denials: SDKPermissionDenial[];
      errors: string[];
    }
```

### `SDKSystemMessage`

Messaggio di inizializzazione del sistema.

```typescript
type SDKSystemMessage = {
  type: 'system';
  subtype: 'init';
  uuid: UUID;
  session_id: string;
  apiKeySource: ApiKeySource;
  cwd: string;
  tools: string[];
  mcp_servers: {
    name: string;
    status: string;
  }[];
  model: string;
  permissionMode: PermissionMode;
  slash_commands: string[];
  output_style: string;
}
```

### `SDKPartialAssistantMessage`

Messaggio parziale di streaming (solo quando `includePartialMessages` è true).

```typescript
type SDKPartialAssistantMessage = {
  type: 'stream_event';
  event: RawMessageStreamEvent; // Da Anthropic SDK
  parent_tool_use_id: string | null;
  uuid: UUID;
  session_id: string;
}
```

### `SDKCompactBoundaryMessage`

Messaggio che indica un limite di compattazione della conversazione.

```typescript
type SDKCompactBoundaryMessage = {
  type: 'system';
  subtype: 'compact_boundary';
  uuid: UUID;
  session_id: string;
  compact_metadata: {
    trigger: 'manual' | 'auto';
    pre_tokens: number;
  };
}
```

### `SDKPermissionDenial`

Informazioni su un uso dello strumento negato.

```typescript
type SDKPermissionDenial = {
  tool_name: string;
  tool_use_id: string;
  tool_input: ToolInput;
}
```

## Tipi di hook

Per una guida completa sull'uso degli hook con esempi e pattern comuni, vedi la [guida agli Hook](/docs/it/agent-sdk/hooks).

### `HookEvent`

Eventi hook disponibili.

```typescript
type HookEvent =
  | 'PreToolUse'
  | 'PostToolUse'
  | 'PostToolUseFailure'
  | 'Notification'
  | 'UserPromptSubmit'
  | 'SessionStart'
  | 'SessionEnd'
  | 'Stop'
  | 'SubagentStart'
  | 'SubagentStop'
  | 'PreCompact'
  | 'PermissionRequest';
```

### `HookCallback`

Tipo di funzione callback hook.

```typescript
type HookCallback = (
  input: HookInput, // Unione di tutti i tipi di input hook
  toolUseID: string | undefined,
  options: { signal: AbortSignal }
) => Promise<HookJSONOutput>;
```

### `HookCallbackMatcher`

Configurazione hook con matcher facoltativo.

```typescript
interface HookCallbackMatcher {
  matcher?: string;
  hooks: HookCallback[];
}
```

### `HookInput`

Tipo di unione di tutti i tipi di input hook.

```typescript
type HookInput =
  | PreToolUseHookInput
  | PostToolUseHookInput
  | PostToolUseFailureHookInput
  | NotificationHookInput
  | UserPromptSubmitHookInput
  | SessionStartHookInput
  | SessionEndHookInput
  | StopHookInput
  | SubagentStartHookInput
  | SubagentStopHookInput
  | PreCompactHookInput
  | PermissionRequestHookInput;
```

### `BaseHookInput`

Interfaccia base che tutti i tipi di input hook estendono.

```typescript
type BaseHookInput = {
  session_id: string;
  transcript_path: string;
  cwd: string;
  permission_mode?: string;
}
```

#### `PreToolUseHookInput`

```typescript
type PreToolUseHookInput = BaseHookInput & {
  hook_event_name: 'PreToolUse';
  tool_name: string;
  tool_input: unknown;
}
```

#### `PostToolUseHookInput`

```typescript
type PostToolUseHookInput = BaseHookInput & {
  hook_event_name: 'PostToolUse';
  tool_name: string;
  tool_input: unknown;
  tool_response: unknown;
}
```

#### `PostToolUseFailureHookInput`

```typescript
type PostToolUseFailureHookInput = BaseHookInput & {
  hook_event_name: 'PostToolUseFailure';
  tool_name: string;
  tool_input: unknown;
  error: string;
  is_interrupt?: boolean;
}
```

#### `NotificationHookInput`

```typescript
type NotificationHookInput = BaseHookInput & {
  hook_event_name: 'Notification';
  message: string;
  title?: string;
}
```

#### `UserPromptSubmitHookInput`

```typescript
type UserPromptSubmitHookInput = BaseHookInput & {
  hook_event_name: 'UserPromptSubmit';
  prompt: string;
}
```

#### `SessionStartHookInput`

```typescript
type SessionStartHookInput = BaseHookInput & {
  hook_event_name: 'SessionStart';
  source: 'startup' | 'resume' | 'clear' | 'compact';
}
```

#### `SessionEndHookInput`

```typescript
type SessionEndHookInput = BaseHookInput & {
  hook_event_name: 'SessionEnd';
  reason: ExitReason;  // Stringa dall'array EXIT_REASONS
}
```

#### `StopHookInput`

```typescript
type StopHookInput = BaseHookInput & {
  hook_event_name: 'Stop';
  stop_hook_active: boolean;
}
```

#### `SubagentStartHookInput`

```typescript
type SubagentStartHookInput = BaseHookInput & {
  hook_event_name: 'SubagentStart';
  agent_id: string;
  agent_type: string;
}
```

#### `SubagentStopHookInput`

```typescript
type SubagentStopHookInput = BaseHookInput & {
  hook_event_name: 'SubagentStop';
  stop_hook_active: boolean;
}
```

#### `PreCompactHookInput`

```typescript
type PreCompactHookInput = BaseHookInput & {
  hook_event_name: 'PreCompact';
  trigger: 'manual' | 'auto';
  custom_instructions: string | null;
}
```

#### `PermissionRequestHookInput`

```typescript
type PermissionRequestHookInput = BaseHookInput & {
  hook_event_name: 'PermissionRequest';
  tool_name: string;
  tool_input: unknown;
  permission_suggestions?: PermissionUpdate[];
}
```

### `HookJSONOutput`

Valore di ritorno hook.

```typescript
type HookJSONOutput = AsyncHookJSONOutput | SyncHookJSONOutput;
```

#### `AsyncHookJSONOutput`

```typescript
type AsyncHookJSONOutput = {
  async: true;
  asyncTimeout?: number;
}
```

#### `SyncHookJSONOutput`

```typescript
type SyncHookJSONOutput = {
  continue?: boolean;
  suppressOutput?: boolean;
  stopReason?: string;
  decision?: 'approve' | 'block';
  systemMessage?: string;
  reason?: string;
  hookSpecificOutput?:
    | {
        hookEventName: 'PreToolUse';
        permissionDecision?: 'allow' | 'deny' | 'ask';
        permissionDecisionReason?: string;
        updatedInput?: Record<string, unknown>;
      }
    | {
        hookEventName: 'UserPromptSubmit';
        additionalContext?: string;
      }
    | {
        hookEventName: 'SessionStart';
        additionalContext?: string;
      }
    | {
        hookEventName: 'PostToolUse';
        additionalContext?: string;
      };
}
```

## Tipi di input dello strumento

Documentazione degli schemi di input per tutti gli strumenti Claude Code integrati. Questi tipi vengono esportati da `@anthropic-ai/claude-agent-sdk` e possono essere utilizzati per interazioni di strumenti type-safe.

### `ToolInput`

**Nota:** Questo è un tipo solo per la documentazione per chiarezza. Rappresenta l'unione di tutti i tipi di input dello strumento.

```typescript
type ToolInput =
  | AgentInput
  | AskUserQuestionInput
  | BashInput
  | BashOutputInput
  | FileEditInput
  | FileReadInput
  | FileWriteInput
  | GlobInput
  | GrepInput
  | KillShellInput
  | NotebookEditInput
  | WebFetchInput
  | WebSearchInput
  | TodoWriteInput
  | ExitPlanModeInput
  | ListMcpResourcesInput
  | ReadMcpResourceInput;
```

### Task

**Nome dello strumento:** `Task`

```typescript
interface AgentInput {
  /**
   * Una breve descrizione del compito (3-5 parole)
   */
  description: string;
  /**
   * Il compito che l'agente deve eseguire
   */
  prompt: string;
  /**
   * Il tipo di agente specializzato da utilizzare per questo compito
   */
  subagent_type: string;
}
```

Avvia un nuovo agente per gestire compiti complessi e multi-step in modo autonomo.

### AskUserQuestion

**Nome dello strumento:** `AskUserQuestion`

```typescript
interface AskUserQuestionInput {
  /**
   * Domande da porre all'utente (1-4 domande)
   */
  questions: Array<{
    /**
     * La domanda completa da porre all'utente. Deve essere chiara, specifica,
     * e terminare con un punto interrogativo.
     */
    question: string;
    /**
     * Etichetta molto breve visualizzata come chip/tag (max 12 caratteri).
     * Esempi: "Auth method", "Library", "Approach"
     */
    header: string;
    /**
     * Le scelte disponibili (2-4 opzioni). Un'opzione "Other" viene
     * fornita automaticamente.
     */
    options: Array<{
      /**
       * Testo di visualizzazione per questa opzione (1-5 parole)
       */
      label: string;
      /**
       * Spiegazione di cosa significa questa opzione
       */
      description: string;
    }>;
    /**
     * Impostare su true per consentire selezioni multiple
     */
    multiSelect: boolean;
  }>;
  /**
   * Risposte dell'utente compilate dal sistema di permessi.
   * Mappa il testo della domanda all'etichetta dell'opzione selezionata.
   * Le risposte a selezione multipla sono separate da virgole.
   */
  answers?: Record<string, string>;
}
```

Pone domande di chiarimento all'utente durante l'esecuzione. Vedi [Gestione dello strumento AskUserQuestion](/docs/it/agent-sdk/permissions#handling-the-askuserquestion-tool) per i dettagli di utilizzo.

### Bash

**Nome dello strumento:** `Bash`

```typescript
interface BashInput {
  /**
   * Il comando da eseguire
   */
  command: string;
  /**
   * Timeout facoltativo in millisecondi (max 600000)
   */
  timeout?: number;
  /**
   * Descrizione chiara e concisa di cosa fa questo comando in 5-10 parole
   */
  description?: string;
  /**
   * Impostare su true per eseguire questo comando in background
   */
  run_in_background?: boolean;
}
```

Esegue comandi bash in una sessione shell persistente con timeout facoltativo ed esecuzione in background.

### BashOutput

**Nome dello strumento:** `BashOutput`

```typescript
interface BashOutputInput {
  /**
   * L'ID della shell in background da cui recuperare l'output
   */
  bash_id: string;
  /**
   * Regex facoltativo per filtrare le righe di output
   */
  filter?: string;
}
```

Recupera l'output da una shell bash in esecuzione o completata in background.

### Edit

**Nome dello strumento:** `Edit`

```typescript
interface FileEditInput {
  /**
   * Il percorso assoluto del file da modificare
   */
  file_path: string;
  /**
   * Il testo da sostituire
   */
  old_string: string;
  /**
   * Il testo con cui sostituirlo (deve essere diverso da old_string)
   */
  new_string: string;
  /**
   * Sostituisci tutte le occorrenze di old_string (predefinito false)
   */
  replace_all?: boolean;
}
```

Esegue sostituzioni di stringhe esatte nei file.

### Read

**Nome dello strumento:** `Read`

```typescript
interface FileReadInput {
  /**
   * Il percorso assoluto del file da leggere
   */
  file_path: string;
  /**
   * Il numero di riga da cui iniziare a leggere
   */
  offset?: number;
  /**
   * Il numero di righe da leggere
   */
  limit?: number;
}
```

Legge i file dal filesystem locale, inclusi testo, immagini, PDF e notebook Jupyter.

### Write

**Nome dello strumento:** `Write`

```typescript
interface FileWriteInput {
  /**
   * Il percorso assoluto del file da scrivere
   */
  file_path: string;
  /**
   * Il contenuto da scrivere nel file
   */
  content: string;
}
```

Scrive un file nel filesystem locale, sovrascrivendo se esiste.

### Glob

**Nome dello strumento:** `Glob`

```typescript
interface GlobInput {
  /**
   * Il pattern glob da abbinare ai file
   */
  pattern: string;
  /**
   * La directory in cui cercare (predefinito cwd)
   */
  path?: string;
}
```

Corrispondenza di pattern di file veloce che funziona con qualsiasi dimensione di codebase.

### Grep

**Nome dello strumento:** `Grep`

```typescript
interface GrepInput {
  /**
   * Il pattern di espressione regolare da cercare
   */
  pattern: string;
  /**
   * File o directory in cui cercare (predefinito cwd)
   */
  path?: string;
  /**
   * Pattern glob per filtrare i file (ad es. "*.js")
   */
  glob?: string;
  /**
   * Tipo di file da cercare (ad es. "js", "py", "rust")
   */
  type?: string;
  /**
   * Modalità di output: "content", "files_with_matches", o "count"
   */
  output_mode?: 'content' | 'files_with_matches' | 'count';
  /**
   * Ricerca senza distinzione tra maiuscole e minuscole
   */
  '-i'?: boolean;
  /**
   * Mostra i numeri di riga (per la modalità content)
   */
  '-n'?: boolean;
  /**
   * Righe da mostrare prima di ogni corrispondenza
   */
  '-B'?: number;
  /**
   * Righe da mostrare dopo ogni corrispondenza
   */
  '-A'?: number;
  /**
   * Righe da mostrare prima e dopo ogni corrispondenza
   */
  '-C'?: number;
  /**
   * Limita l'output alle prime N righe/voci
   */
  head_limit?: number;
  /**
   * Abilita la modalità multilinea
   */
  multiline?: boolean;
}
```

Potente strumento di ricerca basato su ripgrep con supporto regex.

### KillBash

**Nome dello strumento:** `KillBash`

```typescript
interface KillShellInput {
  /**
   * L'ID della shell in background da terminare
   */
  shell_id: string;
}
```

Termina una shell bash in esecuzione in background dal suo ID.

### NotebookEdit

**Nome dello strumento:** `NotebookEdit`

```typescript
interface NotebookEditInput {
  /**
   * Il percorso assoluto del file notebook Jupyter
   */
  notebook_path: string;
  /**
   * L'ID della cella da modificare
   */
  cell_id?: string;
  /**
   * La nuova sorgente per la cella
   */
  new_source: string;
  /**
   * Il tipo della cella (code o markdown)
   */
  cell_type?: 'code' | 'markdown';
  /**
   * Il tipo di modifica (replace, insert, delete)
   */
  edit_mode?: 'replace' | 'insert' | 'delete';
}
```

Modifica le celle nei file notebook Jupyter.

### WebFetch

**Nome dello strumento:** `WebFetch`

```typescript
interface WebFetchInput {
  /**
   * L'URL da cui recuperare il contenuto
   */
  url: string;
  /**
   * Il prompt da eseguire sul contenuto recuperato
   */
  prompt: string;
}
```

Recupera il contenuto da un URL e lo elabora con un modello AI.

### WebSearch

**Nome dello strumento:** `WebSearch`

```typescript
interface WebSearchInput {
  /**
   * La query di ricerca da utilizzare
   */
  query: string;
  /**
   * Includi solo i risultati da questi domini
   */
  allowed_domains?: string[];
  /**
   * Non includere mai i risultati da questi domini
   */
  blocked_domains?: string[];
}
```

Cerca il web e restituisce i risultati formattati.

### TodoWrite

**Nome dello strumento:** `TodoWrite`

```typescript
interface TodoWriteInput {
  /**
   * L'elenco di cose da fare aggiornato
   */
  todos: Array<{
    /**
     * La descrizione del compito
     */
    content: string;
    /**
     * Lo stato del compito
     */
    status: 'pending' | 'in_progress' | 'completed';
    /**
     * Forma attiva della descrizione del compito
     */
    activeForm: string;
  }>;
}
```

Crea e gestisce un elenco di compiti strutturato per il tracciamento dei progressi.

### ExitPlanMode

**Nome dello strumento:** `ExitPlanMode`

```typescript
interface ExitPlanModeInput {
  /**
   * Il piano da eseguire dall'utente per l'approvazione
   */
  plan: string;
}
```

Esce dalla modalità di pianificazione e chiede all'utente di approvare il piano.

### ListMcpResources

**Nome dello strumento:** `ListMcpResources`

```typescript
interface ListMcpResourcesInput {
  /**
   * Nome del server facoltativo per filtrare le risorse per
   */
  server?: string;
}
```

Elenca le risorse MCP disponibili dai server connessi.

### ReadMcpResource

**Nome dello strumento:** `ReadMcpResource`

```typescript
interface ReadMcpResourceInput {
  /**
   * Il nome del server MCP
   */
  server: string;
  /**
   * L'URI della risorsa da leggere
   */
  uri: string;
}
```

Legge una risorsa MCP specifica da un server.

## Tipi di output dello strumento

Documentazione degli schemi di output per tutti gli strumenti Claude Code integrati. Questi tipi rappresentano i dati di risposta effettivi restituiti da ogni strumento.

### `ToolOutput`

**Nota:** Questo è un tipo solo per la documentazione per chiarezza. Rappresenta l'unione di tutti i tipi di output dello strumento.

```typescript
type ToolOutput =
  | TaskOutput
  | AskUserQuestionOutput
  | BashOutput
  | BashOutputToolOutput
  | EditOutput
  | ReadOutput
  | WriteOutput
  | GlobOutput
  | GrepOutput
  | KillBashOutput
  | NotebookEditOutput
  | WebFetchOutput
  | WebSearchOutput
  | TodoWriteOutput
  | ExitPlanModeOutput
  | ListMcpResourcesOutput
  | ReadMcpResourceOutput;
```

### Task

**Nome dello strumento:** `Task`

```typescript
interface TaskOutput {
  /**
   * Messaggio di risultato finale dal subagente
   */
  result: string;
  /**
   * Statistiche di utilizzo dei token
   */
  usage?: {
    input_tokens: number;
    output_tokens: number;
    cache_creation_input_tokens?: number;
    cache_read_input_tokens?: number;
  };
  /**
   * Costo totale in USD
   */
  total_cost_usd?: number;
  /**
   * Durata dell'esecuzione in millisecondi
   */
  duration_ms?: number;
}
```

Restituisce il risultato finale dal subagente dopo aver completato il compito delegato.

### AskUserQuestion

**Nome dello strumento:** `AskUserQuestion`

```typescript
interface AskUserQuestionOutput {
  /**
   * Le domande che sono state poste
   */
  questions: Array<{
    question: string;
    header: string;
    options: Array<{
      label: string;
      description: string;
    }>;
    multiSelect: boolean;
  }>;
  /**
   * Le risposte fornite dall'utente.
   * Mappa il testo della domanda alla stringa di risposta.
   * Le risposte a selezione multipla sono separate da virgole.
   */
  answers: Record<string, string>;
}
```

Restituisce le domande poste e le risposte dell'utente.

### Bash

**Nome dello strumento:** `Bash`

```typescript
interface BashOutput {
  /**
   * Output combinato di stdout e stderr
   */
  output: string;
  /**
   * Codice di uscita del comando
   */
  exitCode: number;
  /**
   * Se il comando è stato terminato a causa del timeout
   */
  killed?: boolean;
  /**
   * ID della shell per i processi in background
   */
  shellId?: string;
}
```

Restituisce l'output del comando con lo stato di uscita. I comandi in background restituiscono immediatamente con uno shellId.

### BashOutput

**Nome dello strumento:** `BashOutput`

```typescript
interface BashOutputToolOutput {
  /**
   * Nuovo output dall'ultimo controllo
   */
  output: string;
  /**
   * Stato della shell corrente
   */
  status: 'running' | 'completed' | 'failed';
  /**
   * Codice di uscita (quando completato)
   */
  exitCode?: number;
}
```

Restituisce l'output incrementale dalle shell in background.

### Edit

**Nome dello strumento:** `Edit`

```typescript
interface EditOutput {
  /**
   * Messaggio di conferma
   */
  message: string;
  /**
   * Numero di sostituzioni effettuate
   */
  replacements: number;
  /**
   * Percorso del file che è stato modificato
   */
  file_path: string;
}
```

Restituisce la conferma delle modifiche riuscite con il conteggio delle sostituzioni.

### Leggi

**Nome dello strumento:** `Read`

```typescript
type ReadOutput = 
  | TextFileOutput
  | ImageFileOutput
  | PDFFileOutput
  | NotebookFileOutput;

interface TextFileOutput {
  /**
   * Contenuto del file con numeri di riga
   */
  content: string;
  /**
   * Numero totale di righe nel file
   */
  total_lines: number;
  /**
   * Righe effettivamente restituite
   */
  lines_returned: number;
}

interface ImageFileOutput {
  /**
   * Dati immagine codificati in Base64
   */
  image: string;
  /**
   * Tipo MIME dell'immagine
   */
  mime_type: string;
  /**
   * Dimensione del file in byte
   */
  file_size: number;
}

interface PDFFileOutput {
  /**
   * Array dei contenuti delle pagine
   */
  pages: Array<{
    page_number: number;
    text?: string;
    images?: Array<{
      image: string;
      mime_type: string;
    }>;
  }>;
  /**
   * Numero totale di pagine
   */
  total_pages: number;
}

interface NotebookFileOutput {
  /**
   * Celle del notebook Jupyter
   */
  cells: Array<{
    cell_type: 'code' | 'markdown';
    source: string;
    outputs?: any[];
    execution_count?: number;
  }>;
  /**
   * Metadati del notebook
   */
  metadata?: Record<string, any>;
}
```

Restituisce il contenuto del file nel formato appropriato al tipo di file.

### Scrivi

**Nome dello strumento:** `Write`

```typescript
interface WriteOutput {
  /**
   * Messaggio di successo
   */
  message: string;
  /**
   * Numero di byte scritti
   */
  bytes_written: number;
  /**
   * Percorso del file che è stato scritto
   */
  file_path: string;
}
```

Restituisce una conferma dopo aver scritto con successo il file.

### Glob

**Nome dello strumento:** `Glob`

```typescript
interface GlobOutput {
  /**
   * Array dei percorsi dei file corrispondenti
   */
  matches: string[];
  /**
   * Numero di corrispondenze trovate
   */
  count: number;
  /**
   * Directory di ricerca utilizzata
   */
  search_path: string;
}
```

Restituisce i percorsi dei file che corrispondono al pattern glob, ordinati per data di modifica.

### Grep

**Nome dello strumento:** `Grep`

```typescript
type GrepOutput = 
  | GrepContentOutput
  | GrepFilesOutput
  | GrepCountOutput;

interface GrepContentOutput {
  /**
   * Righe corrispondenti con contesto
   */
  matches: Array<{
    file: string;
    line_number?: number;
    line: string;
    before_context?: string[];
    after_context?: string[];
  }>;
  /**
   * Numero totale di corrispondenze
   */
  total_matches: number;
}

interface GrepFilesOutput {
  /**
   * File che contengono corrispondenze
   */
  files: string[];
  /**
   * Numero di file con corrispondenze
   */
  count: number;
}

interface GrepCountOutput {
  /**
   * Conteggi delle corrispondenze per file
   */
  counts: Array<{
    file: string;
    count: number;
  }>;
  /**
   * Corrispondenze totali in tutti i file
   */
  total: number;
}
```

Restituisce i risultati della ricerca nel formato specificato da output_mode.

### KillBash

**Nome dello strumento:** `KillBash`

```typescript
interface KillBashOutput {
  /**
   * Messaggio di successo
   */
  message: string;
  /**
   * ID della shell terminata
   */
  shell_id: string;
}
```

Restituisce una conferma dopo aver terminato la shell di background.

### NotebookEdit

**Nome dello strumento:** `NotebookEdit`

```typescript
interface NotebookEditOutput {
  /**
   * Messaggio di successo
   */
  message: string;
  /**
   * Tipo di modifica eseguita
   */
  edit_type: 'replaced' | 'inserted' | 'deleted';
  /**
   * ID della cella interessata
   */
  cell_id?: string;
  /**
   * Numero totale di celle nel notebook dopo la modifica
   */
  total_cells: number;
}
```

Restituisce una conferma dopo aver modificato il notebook Jupyter.

### WebFetch

**Nome dello strumento:** `WebFetch`

```typescript
interface WebFetchOutput {
  /**
   * Risposta del modello AI al prompt
   */
  response: string;
  /**
   * URL che è stato recuperato
   */
  url: string;
  /**
   * URL finale dopo i reindirizzamenti
   */
  final_url?: string;
  /**
   * Codice di stato HTTP
   */
  status_code?: number;
}
```

Restituisce l'analisi dell'IA del contenuto web recuperato.

### WebSearch

**Nome dello strumento:** `WebSearch`

```typescript
interface WebSearchOutput {
  /**
   * Risultati della ricerca
   */
  results: Array<{
    title: string;
    url: string;
    snippet: string;
    /**
     * Metadati aggiuntivi se disponibili
     */
    metadata?: Record<string, any>;
  }>;
  /**
   * Numero totale di risultati
   */
  total_results: number;
  /**
   * La query che è stata cercata
   */
  query: string;
}
```

Restituisce i risultati della ricerca formattati dal web.

### TodoWrite

**Nome dello strumento:** `TodoWrite`

```typescript
interface TodoWriteOutput {
  /**
   * Messaggio di successo
   */
  message: string;
  /**
   * Statistiche attuali delle attività
   */
  stats: {
    total: number;
    pending: number;
    in_progress: number;
    completed: number;
  };
}
```

Restituisce una conferma con le statistiche attuali delle attività.

### ExitPlanMode

**Nome dello strumento:** `ExitPlanMode`

```typescript
interface ExitPlanModeOutput {
  /**
   * Messaggio di conferma
   */
  message: string;
  /**
   * Se l'utente ha approvato il piano
   */
  approved?: boolean;
}
```

Restituisce una conferma dopo aver uscito dalla modalità piano.

### ListMcpResources

**Nome dello strumento:** `ListMcpResources`

```typescript
interface ListMcpResourcesOutput {
  /**
   * Risorse disponibili
   */
  resources: Array<{
    uri: string;
    name: string;
    description?: string;
    mimeType?: string;
    server: string;
  }>;
  /**
   * Numero totale di risorse
   */
  total: number;
}
```

Restituisce l'elenco delle risorse MCP disponibili.

### ReadMcpResource

**Nome dello strumento:** `ReadMcpResource`

```typescript
interface ReadMcpResourceOutput {
  /**
   * Contenuti della risorsa
   */
  contents: Array<{
    uri: string;
    mimeType?: string;
    text?: string;
    blob?: string;
  }>;
  /**
   * Server che ha fornito la risorsa
   */
  server: string;
}
```

Restituisce il contenuto della risorsa MCP richiesta.

## Tipi di Permesso

### `PermissionUpdate`

Operazioni per l'aggiornamento dei permessi.

```typescript
type PermissionUpdate = 
  | {
      type: 'addRules';
      rules: PermissionRuleValue[];
      behavior: PermissionBehavior;
      destination: PermissionUpdateDestination;
    }
  | {
      type: 'replaceRules';
      rules: PermissionRuleValue[];
      behavior: PermissionBehavior;
      destination: PermissionUpdateDestination;
    }
  | {
      type: 'removeRules';
      rules: PermissionRuleValue[];
      behavior: PermissionBehavior;
      destination: PermissionUpdateDestination;
    }
  | {
      type: 'setMode';
      mode: PermissionMode;
      destination: PermissionUpdateDestination;
    }
  | {
      type: 'addDirectories';
      directories: string[];
      destination: PermissionUpdateDestination;
    }
  | {
      type: 'removeDirectories';
      directories: string[];
      destination: PermissionUpdateDestination;
    }
```

### `PermissionBehavior`

```typescript
type PermissionBehavior = 'allow' | 'deny' | 'ask';
```

### `PermissionUpdateDestination`

```typescript
type PermissionUpdateDestination = 
  | 'userSettings'     // Impostazioni globali dell'utente
  | 'projectSettings'  // Impostazioni del progetto per directory
  | 'localSettings'    // Impostazioni locali ignorate da Git
  | 'session'          // Solo sessione corrente
```

### `PermissionRuleValue`

```typescript
type PermissionRuleValue = {
  toolName: string;
  ruleContent?: string;
}
```

## Altri Tipi

### `ApiKeySource`

```typescript
type ApiKeySource = 'user' | 'project' | 'org' | 'temporary';
```

### `SdkBeta`

Funzioni beta disponibili che possono essere abilitate tramite l'opzione `betas`. Vedi [Intestazioni beta](/docs/it/api/beta-headers) per ulteriori informazioni.

```typescript
type SdkBeta = 'context-1m-2025-08-07';
```

| Valore | Descrizione | Modelli Compatibili |
|:------|:------------|:------------------|
| `'context-1m-2025-08-07'` | Abilita la [finestra di contesto](/docs/it/build-with-claude/context-windows) di 1 milione di token | Claude Sonnet 4, Claude Sonnet 4.5 |

### `SlashCommand`

Informazioni su un comando slash disponibile.

```typescript
type SlashCommand = {
  name: string;
  description: string;
  argumentHint: string;
}
```

### `ModelInfo`

Informazioni su un modello disponibile.

```typescript
type ModelInfo = {
  value: string;
  displayName: string;
  description: string;
}
```

### `McpServerStatus`

Stato di un server MCP connesso.

```typescript
type McpServerStatus = {
  name: string;
  status: 'connected' | 'failed' | 'needs-auth' | 'pending';
  serverInfo?: {
    name: string;
    version: string;
  };
}
```

### `AccountInfo`

Informazioni sull'account per l'utente autenticato.

```typescript
type AccountInfo = {
  email?: string;
  organization?: string;
  subscriptionType?: string;
  tokenSource?: string;
  apiKeySource?: string;
}
```

### `ModelUsage`

Statistiche di utilizzo per modello restituite nei messaggi di risultato.

```typescript
type ModelUsage = {
  inputTokens: number;
  outputTokens: number;
  cacheReadInputTokens: number;
  cacheCreationInputTokens: number;
  webSearchRequests: number;
  costUSD: number;
  contextWindow: number;
}
```

### `ConfigScope`

```typescript
type ConfigScope = 'local' | 'user' | 'project';
```

### `NonNullableUsage`

Una versione di [`Usage`](#usage) con tutti i campi nullable resi non-nullable.

```typescript
type NonNullableUsage = {
  [K in keyof Usage]: NonNullable<Usage[K]>;
}
```

### `Usage`

Statistiche di utilizzo dei token (da `@anthropic-ai/sdk`).

```typescript
type Usage = {
  input_tokens: number | null;
  output_tokens: number | null;
  cache_creation_input_tokens?: number | null;
  cache_read_input_tokens?: number | null;
}
```

### `CallToolResult`

Tipo di risultato dello strumento MCP (da `@modelcontextprotocol/sdk/types.js`).

```typescript
type CallToolResult = {
  content: Array<{
    type: 'text' | 'image' | 'resource';
    // I campi aggiuntivi variano in base al tipo
  }>;
  isError?: boolean;
}
```

### `AbortError`

Classe di errore personalizzata per le operazioni di interruzione.

```typescript
class AbortError extends Error {}
```

## Configurazione della Sandbox

### `SandboxSettings`

Configurazione per il comportamento della sandbox. Utilizzare questa opzione per abilitare il sandboxing dei comandi e configurare le restrizioni di rete a livello di programmazione.

```typescript
type SandboxSettings = {
  enabled?: boolean;
  autoAllowBashIfSandboxed?: boolean;
  excludedCommands?: string[];
  allowUnsandboxedCommands?: boolean;
  network?: NetworkSandboxSettings;
  ignoreViolations?: SandboxIgnoreViolations;
  enableWeakerNestedSandbox?: boolean;
}
```

| Proprietà | Tipo | Predefinito | Descrizione |
| :------- | :--- | :------ | :---------- |
| `enabled` | `boolean` | `false` | Abilita la modalità sandbox per l'esecuzione dei comandi |
| `autoAllowBashIfSandboxed` | `boolean` | `false` | Approva automaticamente i comandi bash quando la sandbox è abilitata |
| `excludedCommands` | `string[]` | `[]` | Comandi che bypassano sempre le restrizioni della sandbox (ad es. `['docker']`). Questi vengono eseguiti senza sandbox automaticamente senza il coinvolgimento del modello |
| `allowUnsandboxedCommands` | `boolean` | `false` | Consenti al modello di richiedere l'esecuzione di comandi al di fuori della sandbox. Quando `true`, il modello può impostare `dangerouslyDisableSandbox` nell'input dello strumento, che ricade nel [sistema di permessi](#permissions-fallback-for-unsandboxed-commands) |
| `network` | [`NetworkSandboxSettings`](#networksandboxsettings) | `undefined` | Configurazione della sandbox specifica della rete |
| `ignoreViolations` | [`SandboxIgnoreViolations`](#sandboxignoreviolations) | `undefined` | Configura quali violazioni della sandbox ignorare |
| `enableWeakerNestedSandbox` | `boolean` | `false` | Abilita una sandbox nidificata più debole per compatibilità |

<Note>
**Le restrizioni di accesso al file system e alla rete** NON sono configurate tramite le impostazioni della sandbox. Invece, sono derivate dalle [regole di permesso](https://code.claude.com/docs/it/settings#permission-settings):

- **Restrizioni di lettura del file system**: Regole di negazione della lettura
- **Restrizioni di scrittura del file system**: Regole di consentimento/negazione della modifica
- **Restrizioni di rete**: Regole di consentimento/negazione di WebFetch

Utilizza le impostazioni della sandbox per il sandboxing dell'esecuzione dei comandi e le regole di permesso per il controllo dell'accesso al file system e alla rete.
</Note>

#### Esempio di utilizzo

```typescript
import { query } from "@anthropic-ai/claude-agent-sdk";

const result = await query({
  prompt: "Build and test my project",
  options: {
    sandbox: {
      enabled: true,
      autoAllowBashIfSandboxed: true,
      excludedCommands: ["docker"],
      network: {
        allowLocalBinding: true,
        allowUnixSockets: ["/var/run/docker.sock"]
      }
    }
  }
});
```

### `NetworkSandboxSettings`

Configurazione specifica della rete per la modalità sandbox.

```typescript
type NetworkSandboxSettings = {
  allowLocalBinding?: boolean;
  allowUnixSockets?: string[];
  allowAllUnixSockets?: boolean;
  httpProxyPort?: number;
  socksProxyPort?: number;
}
```

| Proprietà | Tipo | Predefinito | Descrizione |
| :------- | :--- | :------ | :---------- |
| `allowLocalBinding` | `boolean` | `false` | Consenti ai processi di associarsi alle porte locali (ad es. per server di sviluppo) |
| `allowUnixSockets` | `string[]` | `[]` | Percorsi dei socket Unix a cui i processi possono accedere (ad es. socket Docker) |
| `allowAllUnixSockets` | `boolean` | `false` | Consenti l'accesso a tutti i socket Unix |
| `httpProxyPort` | `number` | `undefined` | Porta proxy HTTP per le richieste di rete |
| `socksProxyPort` | `number` | `undefined` | Porta proxy SOCKS per le richieste di rete |

### `SandboxIgnoreViolations`

Configurazione per ignorare violazioni specifiche della sandbox.

```typescript
type SandboxIgnoreViolations = {
  file?: string[];
  network?: string[];
}
```

| Proprietà | Tipo | Predefinito | Descrizione |
| :------- | :--- | :------ | :---------- |
| `file` | `string[]` | `[]` | Pattern di percorsi di file per cui ignorare le violazioni |
| `network` | `string[]` | `[]` | Pattern di rete per cui ignorare le violazioni |

### Fallback dei Permessi per Comandi Non Sandboxati

Quando `allowUnsandboxedCommands` è abilitato, il modello può richiedere di eseguire comandi al di fuori della sandbox impostando `dangerouslyDisableSandbox: true` nell'input dello strumento. Queste richieste ricadono nel sistema di permessi esistente, il che significa che il tuo gestore `canUseTool` verrà invocato, permettendoti di implementare logica di autorizzazione personalizzata.

<Note>
**`excludedCommands` vs `allowUnsandboxedCommands`:**
- `excludedCommands`: Un elenco statico di comandi che bypassano sempre la sandbox automaticamente (ad es. `['docker']`). Il modello non ha controllo su questo.
- `allowUnsandboxedCommands`: Consente al modello di decidere a runtime se richiedere l'esecuzione non sandboxata impostando `dangerouslyDisableSandbox: true` nell'input dello strumento.
</Note>

```typescript
import { query } from "@anthropic-ai/claude-agent-sdk";

const result = await query({
  prompt: "Deploy my application",
  options: {
    sandbox: {
      enabled: true,
      allowUnsandboxedCommands: true  // Il modello può richiedere l'esecuzione non sandboxata
    },
    permissionMode: "default",
    canUseTool: async (tool, input) => {
      // Controlla se il modello sta richiedendo di bypassare la sandbox
      if (tool === "Bash" && input.dangerouslyDisableSandbox) {
        // Il modello vuole eseguire questo comando al di fuori della sandbox
        console.log(`Unsandboxed command requested: ${input.command}`);

        // Restituisci true per consentire, false per negare
        return isCommandAuthorized(input.command);
      }
      return true;
    }
  }
});
```

Questo pattern ti consente di:

- **Controllare le richieste del modello**: Registra quando il modello richiede l'esecuzione non sandboxata
- **Implementare elenchi di consentimento**: Consenti solo a comandi specifici di essere eseguiti non sandboxati
- **Aggiungere flussi di lavoro di approvazione**: Richiedi autorizzazione esplicita per operazioni privilegiate

<Warning>
I comandi in esecuzione con `dangerouslyDisableSandbox: true` hanno accesso completo al sistema. Assicurati che il tuo gestore `canUseTool` convalidi queste richieste con attenzione.
</Warning>

## Vedi anche

- [Panoramica dell'SDK](/docs/it/agent-sdk/overview) - Concetti generali dell'SDK
- [Riferimento dell'SDK Python](/docs/it/agent-sdk/python) - Documentazione dell'SDK Python
- [Riferimento CLI](https://code.claude.com/docs/it/cli-reference) - Interfaccia della riga di comando
- [Flussi di lavoro comuni](https://code.claude.com/docs/it/common-workflows) - Guide passo dopo passo