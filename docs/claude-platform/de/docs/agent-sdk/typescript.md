# Agent SDK reference - TypeScript

Vollständige API-Referenz für das TypeScript Agent SDK, einschließlich aller Funktionen, Typen und Schnittstellen.

---

<script src="/components/typescript-sdk-type-links.js" defer />

<Note>
**Probieren Sie die neue V2-Schnittstelle (Vorschau):** Eine vereinfachte Schnittstelle mit `send()`- und `receive()`-Mustern ist jetzt verfügbar und macht mehrteilige Gespräche einfacher. [Weitere Informationen](/docs/de/agent-sdk/typescript-v2-preview)
</Note>

## Installation

```bash
npm install @anthropic-ai/claude-agent-sdk
```

## Funktionen

### `query()`

Die primäre Funktion für die Interaktion mit Claude Code. Erstellt einen asynchronen Generator, der Nachrichten streamt, wenn sie ankommen.

```typescript
function query({
  prompt,
  options
}: {
  prompt: string | AsyncIterable<SDKUserMessage>;
  options?: Options;
}): Query
```

#### Parameter

| Parameter | Typ | Beschreibung |
| :-------- | :--- | :---------- |
| `prompt` | `string \| AsyncIterable<`[`SDKUserMessage`](#sdkusermessage)`>` | Die Eingabeaufforderung als Zeichenkette oder asynchrones Iterable für den Streaming-Modus |
| `options` | [`Options`](#options) | Optionales Konfigurationsobjekt (siehe Options-Typ unten) |

#### Rückgabewert

Gibt ein [`Query`](#query-1)-Objekt zurück, das `AsyncGenerator<`[`SDKMessage`](#sdkmessage)`, void>` mit zusätzlichen Methoden erweitert.

### `tool()`

Erstellt eine typsichere MCP-Tool-Definition zur Verwendung mit SDK MCP-Servern.

```typescript
function tool<Schema extends ZodRawShape>(
  name: string,
  description: string,
  inputSchema: Schema,
  handler: (args: z.infer<ZodObject<Schema>>, extra: unknown) => Promise<CallToolResult>
): SdkMcpToolDefinition<Schema>
```

#### Parameter

| Parameter | Typ | Beschreibung |
| :-------- | :--- | :---------- |
| `name` | `string` | Der Name des Tools |
| `description` | `string` | Eine Beschreibung, was das Tool tut |
| `inputSchema` | `Schema extends ZodRawShape` | Zod-Schema, das die Eingabeparameter des Tools definiert |
| `handler` | `(args, extra) => Promise<`[`CallToolResult`](#calltoolresult)`>` | Asynchrone Funktion, die die Tool-Logik ausführt |

### `createSdkMcpServer()`

Erstellt eine MCP-Server-Instanz, die im selben Prozess wie Ihre Anwendung ausgeführt wird.

```typescript
function createSdkMcpServer(options: {
  name: string;
  version?: string;
  tools?: Array<SdkMcpToolDefinition<any>>;
}): McpSdkServerConfigWithInstance
```

#### Parameter

| Parameter | Typ | Beschreibung |
| :-------- | :--- | :---------- |
| `options.name` | `string` | Der Name des MCP-Servers |
| `options.version` | `string` | Optionale Versionszeichenkette |
| `options.tools` | `Array<SdkMcpToolDefinition>` | Array von Tool-Definitionen, die mit [`tool()`](#tool) erstellt wurden |

## Typen

### `Options`

Konfigurationsobjekt für die `query()`-Funktion.

| Eigenschaft | Typ | Standard | Beschreibung |
| :------- | :--- | :------ | :---------- |
| `abortController` | `AbortController` | `new AbortController()` | Controller zum Abbrechen von Operationen |
| `additionalDirectories` | `string[]` | `[]` | Zusätzliche Verzeichnisse, auf die Claude zugreifen kann |
| `agents` | `Record<string, [`AgentDefinition`](#agentdefinition)>` | `undefined` | Programmgesteuerte Definition von Subagenten |
| `allowDangerouslySkipPermissions` | `boolean` | `false` | Ermöglichen Sie das Umgehen von Berechtigungen. Erforderlich bei Verwendung von `permissionMode: 'bypassPermissions'` |
| `allowedTools` | `string[]` | Alle Tools | Liste der zulässigen Tool-Namen |
| `betas` | [`SdkBeta`](#sdkbeta)`[]` | `[]` | Beta-Funktionen aktivieren (z. B. `['context-1m-2025-08-07']`) |
| `canUseTool` | [`CanUseTool`](#canusetool) | `undefined` | Benutzerdefinierte Berechtigungsfunktion für die Tool-Nutzung |
| `continue` | `boolean` | `false` | Setzen Sie das neueste Gespräch fort |
| `cwd` | `string` | `process.cwd()` | Aktuelles Arbeitsverzeichnis |
| `disallowedTools` | `string[]` | `[]` | Liste der nicht zulässigen Tool-Namen |
| `enableFileCheckpointing` | `boolean` | `false` | Aktivieren Sie die Verfolgung von Dateiänderungen zum Zurückspulen. Siehe [Datei-Checkpointing](/docs/de/agent-sdk/file-checkpointing) |
| `env` | `Dict<string>` | `process.env` | Umgebungsvariablen |
| `executable` | `'bun' \| 'deno' \| 'node'` | Automatisch erkannt | Zu verwendende JavaScript-Laufzeit |
| `executableArgs` | `string[]` | `[]` | Argumente, die an die ausführbare Datei übergeben werden |
| `extraArgs` | `Record<string, string \| null>` | `{}` | Zusätzliche Argumente |
| `fallbackModel` | `string` | `undefined` | Modell, das verwendet werden soll, wenn das primäre fehlschlägt |
| `forkSession` | `boolean` | `false` | Beim Fortsetzen mit `resume` zu einer neuen Sitzungs-ID verzweigen, anstatt die ursprüngliche Sitzung fortzusetzen |
| `hooks` | `Partial<Record<`[`HookEvent`](#hookevent)`, `[`HookCallbackMatcher`](#hookcallbackmatcher)`[]>>` | `{}` | Hook-Rückrufe für Ereignisse |
| `includePartialMessages` | `boolean` | `false` | Teilweise Nachrichtenereignisse einschließen |
| `maxBudgetUsd` | `number` | `undefined` | Maximales Budget in USD für die Abfrage |
| `maxThinkingTokens` | `number` | `undefined` | Maximale Token für den Denkprozess |
| `maxTurns` | `number` | `undefined` | Maximale Gesprächsrunden |
| `mcpServers` | `Record<string, [`McpServerConfig`](#mcpserverconfig)>` | `{}` | MCP-Server-Konfigurationen |
| `model` | `string` | Standard aus CLI | Claude-Modell, das verwendet werden soll |
| `outputFormat` | `{ type: 'json_schema', schema: JSONSchema }` | `undefined` | Definieren Sie das Ausgabeformat für Agent-Ergebnisse. Siehe [Strukturierte Ausgaben](/docs/de/agent-sdk/structured-outputs) für Details |
| `pathToClaudeCodeExecutable` | `string` | Verwendet integrierte ausführbare Datei | Pfad zur Claude Code-ausführbaren Datei |
| `permissionMode` | [`PermissionMode`](#permissionmode) | `'default'` | Berechtigungsmodus für die Sitzung |
| `permissionPromptToolName` | `string` | `undefined` | MCP-Tool-Name für Berechtigungsaufforderungen |
| `plugins` | [`SdkPluginConfig`](#sdkpluginconfig)`[]` | `[]` | Laden Sie benutzerdefinierte Plugins aus lokalen Pfaden. Siehe [Plugins](/docs/de/agent-sdk/plugins) für Details |
| `resume` | `string` | `undefined` | Sitzungs-ID zum Fortsetzen |
| `resumeSessionAt` | `string` | `undefined` | Sitzung bei einer bestimmten Nachrichten-UUID fortsetzen |
| `sandbox` | [`SandboxSettings`](#sandboxsettings) | `undefined` | Konfigurieren Sie das Sandbox-Verhalten programmgesteuert. Siehe [Sandbox-Einstellungen](#sandboxsettings) für Details |
| `settingSources` | [`SettingSource`](#settingsource)`[]` | `[]` (keine Einstellungen) | Steuern Sie, welche Dateisystem-Einstellungen geladen werden. Wenn weggelassen, werden keine Einstellungen geladen. **Hinweis:** Muss `'project'` enthalten, um CLAUDE.md-Dateien zu laden |
| `stderr` | `(data: string) => void` | `undefined` | Rückruf für stderr-Ausgabe |
| `strictMcpConfig` | `boolean` | `false` | Erzwingen Sie strikte MCP-Validierung |
| `systemPrompt` | `string \| { type: 'preset'; preset: 'claude_code'; append?: string }` | `undefined` (leere Aufforderung) | Systemanforderungskonfiguration. Übergeben Sie eine Zeichenkette für eine benutzerdefinierte Aufforderung oder `{ type: 'preset', preset: 'claude_code' }`, um die Systemaufforderung von Claude Code zu verwenden. Bei Verwendung der Preset-Objektform fügen Sie `append` hinzu, um die Systemaufforderung mit zusätzlichen Anweisungen zu erweitern |
| `tools` | `string[] \| { type: 'preset'; preset: 'claude_code' }` | `undefined` | Tool-Konfiguration. Übergeben Sie ein Array von Tool-Namen oder verwenden Sie die Voreinstellung, um die Standard-Tools von Claude Code zu erhalten |

### `Query`

Schnittstelle, die von der `query()`-Funktion zurückgegeben wird.

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

#### Methoden

| Methode | Beschreibung |
| :----- | :---------- |
| `interrupt()` | Unterbricht die Abfrage (nur im Streaming-Eingabemodus verfügbar) |
| `rewindFiles(userMessageUuid)` | Stellt Dateien in ihren Zustand bei der angegebenen Benutzernachricht wieder her. Erfordert `enableFileCheckpointing: true`. Siehe [Datei-Checkpointing](/docs/de/agent-sdk/file-checkpointing) |
| `setPermissionMode()` | Ändert den Berechtigungsmodus (nur im Streaming-Eingabemodus verfügbar) |
| `setModel()` | Ändert das Modell (nur im Streaming-Eingabemodus verfügbar) |
| `setMaxThinkingTokens()` | Ändert die maximalen Denktoken (nur im Streaming-Eingabemodus verfügbar) |
| `supportedCommands()` | Gibt verfügbare Schrägstrich-Befehle zurück |
| `supportedModels()` | Gibt verfügbare Modelle mit Anzeigeinformationen zurück |
| `mcpServerStatus()` | Gibt den Status verbundener MCP-Server zurück |
| `accountInfo()` | Gibt Kontoinformationen zurück |

### `AgentDefinition`

Konfiguration für einen programmgesteuert definierten Subagenten.

```typescript
type AgentDefinition = {
  description: string;
  tools?: string[];
  prompt: string;
  model?: 'sonnet' | 'opus' | 'haiku' | 'inherit';
}
```

| Feld | Erforderlich | Beschreibung |
|:------|:---------|:------------|
| `description` | Ja | Natürlichsprachige Beschreibung, wann dieser Agent verwendet werden soll |
| `tools` | Nein | Array von zulässigen Tool-Namen. Wenn weggelassen, erbt alle Tools |
| `prompt` | Ja | Die Systemaufforderung des Agenten |
| `model` | Nein | Modellüberschreibung für diesen Agenten. Wenn weggelassen, verwendet das Hauptmodell |

### `SettingSource`

Steuert, welche dateisystembasierte Konfigurationsquellen das SDK Einstellungen aus lädt.

```typescript
type SettingSource = 'user' | 'project' | 'local';
```

| Wert | Beschreibung | Standort |
|:------|:------------|:---------|
| `'user'` | Globale Benutzereinstellungen | `~/.claude/settings.json` |
| `'project'` | Gemeinsame Projekteinstellungen (versionskontrolliert) | `.claude/settings.json` |
| `'local'` | Lokale Projekteinstellungen (gitignoriert) | `.claude/settings.local.json` |

#### Standardverhalten

Wenn `settingSources` **weggelassen** oder **undefined** ist, lädt das SDK **keine** Dateisystem-Einstellungen. Dies bietet Isolation für SDK-Anwendungen.

#### Warum settingSources verwenden?

**Laden Sie alle Dateisystem-Einstellungen (Legacy-Verhalten):**
```typescript
// Laden Sie alle Einstellungen wie SDK v0.0.x
const result = query({
  prompt: "Analyze this code",
  options: {
    settingSources: ['user', 'project', 'local']  // Laden Sie alle Einstellungen
  }
});
```

**Laden Sie nur bestimmte Einstellungsquellen:**
```typescript
// Laden Sie nur Projekteinstellungen, ignorieren Sie Benutzer und lokal
const result = query({
  prompt: "Run CI checks",
  options: {
    settingSources: ['project']  // Nur .claude/settings.json
  }
});
```

**Test- und CI-Umgebungen:**
```typescript
// Stellen Sie konsistentes Verhalten in CI sicher, indem Sie lokale Einstellungen ausschließen
const result = query({
  prompt: "Run tests",
  options: {
    settingSources: ['project'],  // Nur teamweit gemeinsame Einstellungen
    permissionMode: 'bypassPermissions'
  }
});
```

**SDK-only-Anwendungen:**
```typescript
// Definieren Sie alles programmgesteuert (Standardverhalten)
// Keine Dateisystem-Abhängigkeiten - settingSources standardmäßig auf []
const result = query({
  prompt: "Review this PR",
  options: {
    // settingSources: [] ist der Standard, keine Angabe erforderlich
    agents: { /* ... */ },
    mcpServers: { /* ... */ },
    allowedTools: ['Read', 'Grep', 'Glob']
  }
});
```

**Laden von CLAUDE.md-Projektanweisungen:**
```typescript
// Laden Sie Projekteinstellungen, um CLAUDE.md-Dateien einzuschließen
const result = query({
  prompt: "Add a new feature following project conventions",
  options: {
    systemPrompt: {
      type: 'preset',
      preset: 'claude_code'  // Erforderlich zur Verwendung von CLAUDE.md
    },
    settingSources: ['project'],  // Lädt CLAUDE.md aus dem Projektverzeichnis
    allowedTools: ['Read', 'Write', 'Edit']
  }
});
```

#### Einstellungspriorität

Wenn mehrere Quellen geladen werden, werden Einstellungen mit dieser Priorität zusammengeführt (höchste bis niedrigste):
1. Lokale Einstellungen (`.claude/settings.local.json`)
2. Projekteinstellungen (`.claude/settings.json`)
3. Benutzereinstellungen (`~/.claude/settings.json`)

Programmgesteuerte Optionen (wie `agents`, `allowedTools`) überschreiben immer Dateisystem-Einstellungen.

### `PermissionMode`

```typescript
type PermissionMode =
  | 'default'           // Standardberechtigungsverhalten
  | 'acceptEdits'       // Dateibearbeitungen automatisch akzeptieren
  | 'bypassPermissions' // Alle Berechtigungsprüfungen umgehen
  | 'plan'              // Planungsmodus - keine Ausführung
```

### `CanUseTool`

Benutzerdefinierter Berechtigungsfunktionstyp zur Steuerung der Tool-Nutzung.

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

Ergebnis einer Berechtigungsprüfung.

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

Konfiguration für MCP-Server.

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

Konfiguration zum Laden von Plugins im SDK.

```typescript
type SdkPluginConfig = {
  type: 'local';
  path: string;
}
```

| Feld | Typ | Beschreibung |
|:------|:-----|:------------|
| `type` | `'local'` | Muss `'local'` sein (derzeit nur lokale Plugins unterstützt) |
| `path` | `string` | Absoluter oder relativer Pfad zum Plugin-Verzeichnis |

**Beispiel:**
```typescript
plugins: [
  { type: 'local', path: './my-plugin' },
  { type: 'local', path: '/absolute/path/to/plugin' }
]
```

Vollständige Informationen zum Erstellen und Verwenden von Plugins finden Sie unter [Plugins](/docs/de/agent-sdk/plugins).

## Nachrichtentypen

### `SDKMessage`

Union-Typ aller möglichen Nachrichten, die von der Abfrage zurückgegeben werden.

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

Antwortnachricht des Assistenten.

```typescript
type SDKAssistantMessage = {
  type: 'assistant';
  uuid: UUID;
  session_id: string;
  message: APIAssistantMessage; // Aus Anthropic SDK
  parent_tool_use_id: string | null;
}
```

### `SDKUserMessage`

Benutzereingabenachricht.

```typescript
type SDKUserMessage = {
  type: 'user';
  uuid?: UUID;
  session_id: string;
  message: APIUserMessage; // Aus Anthropic SDK
  parent_tool_use_id: string | null;
}
```

### `SDKUserMessageReplay`

Wiedergegebene Benutzernachricht mit erforderlicher UUID.

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

Endgültige Ergebnisnachricht.

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

Systeminitalisierungsnachricht.

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

Streaming-Teilnachricht (nur wenn `includePartialMessages` wahr ist).

```typescript
type SDKPartialAssistantMessage = {
  type: 'stream_event';
  event: RawMessageStreamEvent; // Aus Anthropic SDK
  parent_tool_use_id: string | null;
  uuid: UUID;
  session_id: string;
}
```

### `SDKCompactBoundaryMessage`

Nachricht, die eine Gesprächskomprimierungsgrenze anzeigt.

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

Informationen über eine verweigerte Tool-Nutzung.

```typescript
type SDKPermissionDenial = {
  tool_name: string;
  tool_use_id: string;
  tool_input: ToolInput;
}
```

## Hook-Typen

Eine umfassende Anleitung zur Verwendung von Hooks mit Beispielen und häufigen Mustern finden Sie im [Hooks-Leitfaden](/docs/de/agent-sdk/hooks).

### `HookEvent`

Verfügbare Hook-Ereignisse.

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

Hook-Rückruffunktionstyp.

```typescript
type HookCallback = (
  input: HookInput, // Union aller Hook-Eingabetypen
  toolUseID: string | undefined,
  options: { signal: AbortSignal }
) => Promise<HookJSONOutput>;
```

### `HookCallbackMatcher`

Hook-Konfiguration mit optionalem Matcher.

```typescript
interface HookCallbackMatcher {
  matcher?: string;
  hooks: HookCallback[];
}
```

### `HookInput`

Union-Typ aller Hook-Eingabetypen.

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

Basisschnittstelle, die alle Hook-Eingabetypen erweitern.

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
  reason: ExitReason;  // Zeichenkette aus EXIT_REASONS-Array
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

Hook-Rückgabewert.

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

## Tool-Eingabetypen

Dokumentation von Eingabeschemas für alle integrierten Claude Code-Tools. Diese Typen werden aus `@anthropic-ai/claude-agent-sdk` exportiert und können für typsichere Tool-Interaktionen verwendet werden.

### `ToolInput`

**Hinweis:** Dies ist ein reiner Dokumentationstyp zur Verdeutlichung. Er stellt die Union aller Tool-Eingabetypen dar.

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

**Tool-Name:** `Task`

```typescript
interface AgentInput {
  /**
   * Eine kurze (3-5 Wörter) Beschreibung der Aufgabe
   */
  description: string;
  /**
   * Die Aufgabe, die der Agent ausführen soll
   */
  prompt: string;
  /**
   * Der Typ des spezialisierten Agenten, der für diese Aufgabe verwendet werden soll
   */
  subagent_type: string;
}
```

Startet einen neuen Agenten, um komplexe, mehrstufige Aufgaben autonom zu bearbeiten.

### AskUserQuestion

**Tool-Name:** `AskUserQuestion`

```typescript
interface AskUserQuestionInput {
  /**
   * Fragen, die dem Benutzer gestellt werden sollen (1-4 Fragen)
   */
  questions: Array<{
    /**
     * Die vollständige Frage, die dem Benutzer gestellt werden soll. Sollte klar und spezifisch sein
     * und mit einem Fragezeichen enden.
     */
    question: string;
    /**
     * Sehr kurzes Label, das als Chip/Tag angezeigt wird (max. 12 Zeichen).
     * Beispiele: "Auth method", "Library", "Approach"
     */
    header: string;
    /**
     * Die verfügbaren Auswahlmöglichkeiten (2-4 Optionen). Eine "Other"-Option wird
     * automatisch bereitgestellt.
     */
    options: Array<{
      /**
       * Anzeigetext für diese Option (1-5 Wörter)
       */
      label: string;
      /**
       * Erklärung, was diese Option bedeutet
       */
      description: string;
    }>;
    /**
     * Setzen Sie auf true, um mehrere Auswahlmöglichkeiten zu ermöglichen
     */
    multiSelect: boolean;
  }>;
  /**
   * Benutzerantworten, die vom Berechtigungssystem ausgefüllt werden.
   * Ordnet Fragetext ausgewählten Optionslabeln zu.
   * Mehrfachauswahlantworte sind durch Kommas getrennt.
   */
  answers?: Record<string, string>;
}
```

Stellt dem Benutzer während der Ausführung Klärungsfragen. Siehe [Umgang mit dem AskUserQuestion-Tool](/docs/de/agent-sdk/permissions#handling-the-askuserquestion-tool) für Nutzungsdetails.

### Bash

**Tool-Name:** `Bash`

```typescript
interface BashInput {
  /**
   * Der auszuführende Befehl
   */
  command: string;
  /**
   * Optionales Timeout in Millisekunden (max. 600000)
   */
  timeout?: number;
  /**
   * Klare, prägnante Beschreibung, was dieser Befehl in 5-10 Wörtern tut
   */
  description?: string;
  /**
   * Setzen Sie auf true, um diesen Befehl im Hintergrund auszuführen
   */
  run_in_background?: boolean;
}
```

Führt Bash-Befehle in einer persistenten Shell-Sitzung mit optionalem Timeout und Hintergrundausführung aus.

### BashOutput

**Tool-Name:** `BashOutput`

```typescript
interface BashOutputInput {
  /**
   * Die ID der Hintergrund-Shell, von der die Ausgabe abgerufen werden soll
   */
  bash_id: string;
  /**
   * Optionaler Regex zum Filtern von Ausgabezeilen
   */
  filter?: string;
}
```

Ruft die Ausgabe von einer laufenden oder abgeschlossenen Hintergrund-Bash-Shell ab.

### Edit

**Tool-Name:** `Edit`

```typescript
interface FileEditInput {
  /**
   * Der absolute Pfad zur zu ändernden Datei
   */
  file_path: string;
  /**
   * Der zu ersetzende Text
   */
  old_string: string;
  /**
   * Der Text, durch den er ersetzt werden soll (muss sich von old_string unterscheiden)
   */
  new_string: string;
  /**
   * Ersetzen Sie alle Vorkommen von old_string (Standard false)
   */
  replace_all?: boolean;
}
```

Führt exakte Zeichenkettenersetzungen in Dateien durch.

### Read

**Tool-Name:** `Read`

```typescript
interface FileReadInput {
  /**
   * Der absolute Pfad zur zu lesenden Datei
   */
  file_path: string;
  /**
   * Die Zeilennummer, ab der gelesen werden soll
   */
  offset?: number;
  /**
   * Die Anzahl der zu lesenden Zeilen
   */
  limit?: number;
}
```

Liest Dateien aus dem lokalen Dateisystem, einschließlich Text, Bilder, PDFs und Jupyter-Notebooks.

### Write

**Tool-Name:** `Write`

```typescript
interface FileWriteInput {
  /**
   * Der absolute Pfad zur zu schreibenden Datei
   */
  file_path: string;
  /**
   * Der in die Datei zu schreibende Inhalt
   */
  content: string;
}
```

Schreibt eine Datei in das lokale Dateisystem und überschreibt sie, falls vorhanden.

### Glob

**Tool-Name:** `Glob`

```typescript
interface GlobInput {
  /**
   * Das Glob-Muster, das mit Dateien abgeglichen werden soll
   */
  pattern: string;
  /**
   * Das Verzeichnis, in dem gesucht werden soll (Standard cwd)
   */
  path?: string;
}
```

Schnelle Dateimusterabstimmung, die mit jeder Codebasis-Größe funktioniert.

### Grep

**Tool-Name:** `Grep`

```typescript
interface GrepInput {
  /**
   * Das reguläre Ausdrucksmuster, nach dem gesucht werden soll
   */
  pattern: string;
  /**
   * Datei oder Verzeichnis, in dem gesucht werden soll (Standard cwd)
   */
  path?: string;
  /**
   * Glob-Muster zum Filtern von Dateien (z. B. "*.js")
   */
  glob?: string;
  /**
   * Dateityp, in dem gesucht werden soll (z. B. "js", "py", "rust")
   */
  type?: string;
  /**
   * Ausgabemodus: "content", "files_with_matches" oder "count"
   */
  output_mode?: 'content' | 'files_with_matches' | 'count';
  /**
   * Suche ohne Berücksichtigung der Groß-/Kleinschreibung
   */
  '-i'?: boolean;
  /**
   * Zeilennummern anzeigen (für Content-Modus)
   */
  '-n'?: boolean;
  /**
   * Zeilen vor jedem Match anzeigen
   */
  '-B'?: number;
  /**
   * Zeilen nach jedem Match anzeigen
   */
  '-A'?: number;
  /**
   * Zeilen vor und nach jedem Match anzeigen
   */
  '-C'?: number;
  /**
   * Ausgabe auf erste N Zeilen/Einträge begrenzen
   */
  head_limit?: number;
  /**
   * Mehrzeilenmodus aktivieren
   */
  multiline?: boolean;
}
```

Leistungsstarkes Suchtool basierend auf ripgrep mit Regex-Unterstützung.

### KillBash

**Tool-Name:** `KillBash`

```typescript
interface KillShellInput {
  /**
   * Die ID der zu beendenden Hintergrund-Shell
   */
  shell_id: string;
}
```

Beendet eine laufende Hintergrund-Bash-Shell nach ihrer ID.

### NotebookEdit

**Tool-Name:** `NotebookEdit`

```typescript
interface NotebookEditInput {
  /**
   * Der absolute Pfad zur Jupyter-Notebook-Datei
   */
  notebook_path: string;
  /**
   * Die ID der zu bearbeitenden Zelle
   */
  cell_id?: string;
  /**
   * Die neue Quelle für die Zelle
   */
  new_source: string;
  /**
   * Der Typ der Zelle (Code oder Markdown)
   */
  cell_type?: 'code' | 'markdown';
  /**
   * Der Typ der Bearbeitung (Ersetzen, Einfügen, Löschen)
   */
  edit_mode?: 'replace' | 'insert' | 'delete';
}
```

Bearbeitet Zellen in Jupyter-Notebook-Dateien.

### WebFetch

**Tool-Name:** `WebFetch`

```typescript
interface WebFetchInput {
  /**
   * Die URL, von der Inhalte abgerufen werden sollen
   */
  url: string;
  /**
   * Die Aufforderung, die auf den abgerufenen Inhalt angewendet werden soll
   */
  prompt: string;
}
```

Ruft Inhalte von einer URL ab und verarbeitet sie mit einem KI-Modell.

### WebSearch

**Tool-Name:** `WebSearch`

```typescript
interface WebSearchInput {
  /**
   * Die zu verwendende Suchabfrage
   */
  query: string;
  /**
   * Nur Ergebnisse von diesen Domänen einschließen
   */
  allowed_domains?: string[];
  /**
   * Niemals Ergebnisse von diesen Domänen einschließen
   */
  blocked_domains?: string[];
}
```

Durchsucht das Web und gibt formatierte Ergebnisse zurück.

### TodoWrite

**Tool-Name:** `TodoWrite`

```typescript
interface TodoWriteInput {
  /**
   * Die aktualisierte Todo-Liste
   */
  todos: Array<{
    /**
     * Die Aufgabenbeschreibung
     */
    content: string;
    /**
     * Der Aufgabenstatus
     */
    status: 'pending' | 'in_progress' | 'completed';
    /**
     * Aktive Form der Aufgabenbeschreibung
     */
    activeForm: string;
  }>;
}
```

Erstellt und verwaltet eine strukturierte Aufgabenliste zur Verfolgung des Fortschritts.

### ExitPlanMode

**Tool-Name:** `ExitPlanMode`

```typescript
interface ExitPlanModeInput {
  /**
   * Der Plan, der vom Benutzer zur Genehmigung ausgeführt werden soll
   */
  plan: string;
}
```

Beendet den Planungsmodus und fordert den Benutzer auf, den Plan zu genehmigen.

### ListMcpResources

**Tool-Name:** `ListMcpResources`

```typescript
interface ListMcpResourcesInput {
  /**
   * Optionaler Servername zum Filtern von Ressourcen
   */
  server?: string;
}
```

Listet verfügbare MCP-Ressourcen von verbundenen Servern auf.

### ReadMcpResource

**Tool-Name:** `ReadMcpResource`

```typescript
interface ReadMcpResourceInput {
  /**
   * Der MCP-Servername
   */
  server: string;
  /**
   * Die zu lesende Ressourcen-URI
   */
  uri: string;
}
```

Liest eine bestimmte MCP-Ressource von einem Server.

## Tool-Ausgabetypen

Dokumentation von Ausgabeschemas für alle integrierten Claude Code-Tools. Diese Typen stellen die tatsächlichen Antwortdaten dar, die von jedem Tool zurückgegeben werden.

### `ToolOutput`

**Hinweis:** Dies ist ein reiner Dokumentationstyp zur Verdeutlichung. Er stellt die Union aller Tool-Ausgabetypen dar.

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

**Tool-Name:** `Task`

```typescript
interface TaskOutput {
  /**
   * Endgültige Ergebnisnachricht vom Subagenten
   */
  result: string;
  /**
   * Token-Nutzungsstatistiken
   */
  usage?: {
    input_tokens: number;
    output_tokens: number;
    cache_creation_input_tokens?: number;
    cache_read_input_tokens?: number;
  };
  /**
   * Gesamtkosten in USD
   */
  total_cost_usd?: number;
  /**
   * Ausführungsdauer in Millisekunden
   */
  duration_ms?: number;
}
```

Gibt das endgültige Ergebnis vom Subagenten nach Abschluss der delegierten Aufgabe zurück.

### AskUserQuestion

**Tool-Name:** `AskUserQuestion`

```typescript
interface AskUserQuestionOutput {
  /**
   * Die gestellten Fragen
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
   * Die vom Benutzer bereitgestellten Antworten.
   * Ordnet Fragetext Antwortzeichenkette zu.
   * Mehrfachauswahlantworte sind durch Kommas getrennt.
   */
  answers: Record<string, string>;
}
```

Gibt die gestellten Fragen und die Antworten des Benutzers zurück.

### Bash

**Tool-Name:** `Bash`

```typescript
interface BashOutput {
  /**
   * Kombinierte stdout- und stderr-Ausgabe
   */
  output: string;
  /**
   * Exitcode des Befehls
   */
  exitCode: number;
  /**
   * Ob der Befehl aufgrund eines Timeouts beendet wurde
   */
  killed?: boolean;
  /**
   * Shell-ID für Hintergrundprozesse
   */
  shellId?: string;
}
```

Gibt Befehlsausgabe mit Exitstatus zurück. Hintergundbefehle geben sofort mit einer shellId zurück.

### BashOutput

**Tool-Name:** `BashOutput`

```typescript
interface BashOutputToolOutput {
  /**
   * Neue Ausgabe seit der letzten Überprüfung
   */
  output: string;
  /**
   * Aktueller Shell-Status
   */
  status: 'running' | 'completed' | 'failed';
  /**
   * Exitcode (wenn abgeschlossen)
   */
  exitCode?: number;
}
```

Gibt inkrementelle Ausgabe von Hintergrund-Shells zurück.

### Edit

**Tool-Name:** `Edit`

```typescript
interface EditOutput {
  /**
   * Bestätigungsnachricht
   */
  message: string;
  /**
   * Anzahl der durchgeführten Ersetzungen
   */
  replacements: number;
  /**
   * Dateipfad, der bearbeitet wurde
   */
  file_path: string;
}
```

Gibt Bestätigung erfolgreicher Bearbeitungen mit Ersetzungsanzahl zurück.

### Read

**Tool name:** `Read`

```typescript
type ReadOutput = 
  | TextFileOutput
  | ImageFileOutput
  | PDFFileOutput
  | NotebookFileOutput;

interface TextFileOutput {
  /**
   * File contents with line numbers
   */
  content: string;
  /**
   * Total number of lines in file
   */
  total_lines: number;
  /**
   * Lines actually returned
   */
  lines_returned: number;
}

interface ImageFileOutput {
  /**
   * Base64 encoded image data
   */
  image: string;
  /**
   * Image MIME type
   */
  mime_type: string;
  /**
   * File size in bytes
   */
  file_size: number;
}

interface PDFFileOutput {
  /**
   * Array of page contents
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
   * Total number of pages
   */
  total_pages: number;
}

interface NotebookFileOutput {
  /**
   * Jupyter notebook cells
   */
  cells: Array<{
    cell_type: 'code' | 'markdown';
    source: string;
    outputs?: any[];
    execution_count?: number;
  }>;
  /**
   * Notebook metadata
   */
  metadata?: Record<string, any>;
}
```

Gibt Dateiinhalte in einem für den Dateityp geeigneten Format zurück.

### Write

**Tool name:** `Write`

```typescript
interface WriteOutput {
  /**
   * Success message
   */
  message: string;
  /**
   * Number of bytes written
   */
  bytes_written: number;
  /**
   * File path that was written
   */
  file_path: string;
}
```

Gibt eine Bestätigung nach erfolgreichem Schreiben der Datei zurück.

### Glob

**Tool name:** `Glob`

```typescript
interface GlobOutput {
  /**
   * Array of matching file paths
   */
  matches: string[];
  /**
   * Number of matches found
   */
  count: number;
  /**
   * Search directory used
   */
  search_path: string;
}
```

Gibt Dateipfade zurück, die dem Glob-Muster entsprechen, sortiert nach Änderungszeit.

### Grep

**Tool name:** `Grep`

```typescript
type GrepOutput = 
  | GrepContentOutput
  | GrepFilesOutput
  | GrepCountOutput;

interface GrepContentOutput {
  /**
   * Matching lines with context
   */
  matches: Array<{
    file: string;
    line_number?: number;
    line: string;
    before_context?: string[];
    after_context?: string[];
  }>;
  /**
   * Total number of matches
   */
  total_matches: number;
}

interface GrepFilesOutput {
  /**
   * Files containing matches
   */
  files: string[];
  /**
   * Number of files with matches
   */
  count: number;
}

interface GrepCountOutput {
  /**
   * Match counts per file
   */
  counts: Array<{
    file: string;
    count: number;
  }>;
  /**
   * Total matches across all files
   */
  total: number;
}
```

Gibt Suchergebnisse in dem durch output_mode angegebenen Format zurück.

### KillBash

**Tool name:** `KillBash`

```typescript
interface KillBashOutput {
  /**
   * Success message
   */
  message: string;
  /**
   * ID of the killed shell
   */
  shell_id: string;
}
```

Gibt eine Bestätigung nach Beendigung der Hintergrund-Shell zurück.

### NotebookEdit

**Tool name:** `NotebookEdit`

```typescript
interface NotebookEditOutput {
  /**
   * Success message
   */
  message: string;
  /**
   * Type of edit performed
   */
  edit_type: 'replaced' | 'inserted' | 'deleted';
  /**
   * Cell ID that was affected
   */
  cell_id?: string;
  /**
   * Total cells in notebook after edit
   */
  total_cells: number;
}
```

Gibt eine Bestätigung nach Änderung des Jupyter-Notebooks zurück.

### WebFetch

**Tool name:** `WebFetch`

```typescript
interface WebFetchOutput {
  /**
   * AI model's response to the prompt
   */
  response: string;
  /**
   * URL that was fetched
   */
  url: string;
  /**
   * Final URL after redirects
   */
  final_url?: string;
  /**
   * HTTP status code
   */
  status_code?: number;
}
```

Gibt die Analyse des KI-Modells des abgerufenen Webinhalts zurück.

### WebSearch

**Tool name:** `WebSearch`

```typescript
interface WebSearchOutput {
  /**
   * Search results
   */
  results: Array<{
    title: string;
    url: string;
    snippet: string;
    /**
     * Additional metadata if available
     */
    metadata?: Record<string, any>;
  }>;
  /**
   * Total number of results
   */
  total_results: number;
  /**
   * The query that was searched
   */
  query: string;
}
```

Gibt formatierte Suchergebnisse aus dem Web zurück.

### TodoWrite

**Tool name:** `TodoWrite`

```typescript
interface TodoWriteOutput {
  /**
   * Success message
   */
  message: string;
  /**
   * Current todo statistics
   */
  stats: {
    total: number;
    pending: number;
    in_progress: number;
    completed: number;
  };
}
```

Gibt eine Bestätigung mit aktuellen Aufgabenstatistiken zurück.

### ExitPlanMode

**Tool name:** `ExitPlanMode`

```typescript
interface ExitPlanModeOutput {
  /**
   * Confirmation message
   */
  message: string;
  /**
   * Whether user approved the plan
   */
  approved?: boolean;
}
```

Gibt eine Bestätigung nach Beendigung des Plan-Modus zurück.

### ListMcpResources

**Tool name:** `ListMcpResources`

```typescript
interface ListMcpResourcesOutput {
  /**
   * Available resources
   */
  resources: Array<{
    uri: string;
    name: string;
    description?: string;
    mimeType?: string;
    server: string;
  }>;
  /**
   * Total number of resources
   */
  total: number;
}
```

Gibt eine Liste der verfügbaren MCP-Ressourcen zurück.

### ReadMcpResource

**Tool name:** `ReadMcpResource`

```typescript
interface ReadMcpResourceOutput {
  /**
   * Resource contents
   */
  contents: Array<{
    uri: string;
    mimeType?: string;
    text?: string;
    blob?: string;
  }>;
  /**
   * Server that provided the resource
   */
  server: string;
}
```

Gibt den Inhalt der angeforderten MCP-Ressource zurück.

## Permission Types

### `PermissionUpdate`

Operationen zum Aktualisieren von Berechtigungen.

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
  | 'userSettings'     // Global user settings
  | 'projectSettings'  // Per-directory project settings
  | 'localSettings'    // Gitignored local settings
  | 'session'          // Current session only
```

### `PermissionRuleValue`

```typescript
type PermissionRuleValue = {
  toolName: string;
  ruleContent?: string;
}
```

## Other Types

### `ApiKeySource`

```typescript
type ApiKeySource = 'user' | 'project' | 'org' | 'temporary';
```

### `SdkBeta`

Verfügbare Beta-Funktionen, die über die Option `betas` aktiviert werden können. Weitere Informationen finden Sie unter [Beta-Header](/docs/de/api/beta-headers).

```typescript
type SdkBeta = 'context-1m-2025-08-07';
```

| Value | Description | Compatible Models |
|:------|:------------|:------------------|
| `'context-1m-2025-08-07'` | Aktiviert ein [Kontextfenster](/docs/de/build-with-claude/context-windows) von 1 Million Token | Claude Sonnet 4, Claude Sonnet 4.5 |

### `SlashCommand`

Informationen über einen verfügbaren Schrägstrich-Befehl.

```typescript
type SlashCommand = {
  name: string;
  description: string;
  argumentHint: string;
}
```

### `ModelInfo`

Informationen über ein verfügbares Modell.

```typescript
type ModelInfo = {
  value: string;
  displayName: string;
  description: string;
}
```

### `McpServerStatus`

Status eines verbundenen MCP-Servers.

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

Kontoinformationen für den authentifizierten Benutzer.

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

Pro-Modell-Nutzungsstatistiken, die in Ergebnismeldungen zurückgegeben werden.

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

Eine Version von [`Usage`](#usage) mit allen nullable Feldern, die nicht nullable gemacht wurden.

```typescript
type NonNullableUsage = {
  [K in keyof Usage]: NonNullable<Usage[K]>;
}
```

### `Usage`

Token-Nutzungsstatistiken (von `@anthropic-ai/sdk`).

```typescript
type Usage = {
  input_tokens: number | null;
  output_tokens: number | null;
  cache_creation_input_tokens?: number | null;
  cache_read_input_tokens?: number | null;
}
```

### `CallToolResult`

MCP-Tool-Ergebnistyp (von `@modelcontextprotocol/sdk/types.js`).

```typescript
type CallToolResult = {
  content: Array<{
    type: 'text' | 'image' | 'resource';
    // Additional fields vary by type
  }>;
  isError?: boolean;
}
```

### `AbortError`

Benutzerdefinierte Fehlerklasse für Abbruchvorgänge.

```typescript
class AbortError extends Error {}
```

## Sandbox-Konfiguration

### `SandboxSettings`

Konfiguration für das Sandbox-Verhalten. Verwenden Sie dies, um Befehls-Sandboxing zu aktivieren und Netzwerkbeschränkungen programmgesteuert zu konfigurieren.

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

| Property | Type | Default | Description |
| :------- | :--- | :------ | :---------- |
| `enabled` | `boolean` | `false` | Aktivieren Sie den Sandbox-Modus für die Befehlsausführung |
| `autoAllowBashIfSandboxed` | `boolean` | `false` | Bash-Befehle automatisch genehmigen, wenn die Sandbox aktiviert ist |
| `excludedCommands` | `string[]` | `[]` | Befehle, die Sandbox-Beschränkungen immer umgehen (z. B. `['docker']`). Diese werden automatisch ohne Modellbeteiligung unsandboxed ausgeführt |
| `allowUnsandboxedCommands` | `boolean` | `false` | Ermöglichen Sie dem Modell, die Ausführung von Befehlen außerhalb der Sandbox anzufordern. Wenn `true`, kann das Modell `dangerouslyDisableSandbox` in der Tool-Eingabe setzen, was auf das [Berechtigungssystem](#permissions-fallback-for-unsandboxed-commands) zurückfällt |
| `network` | [`NetworkSandboxSettings`](#networksandboxsettings) | `undefined` | Netzwerkspezifische Sandbox-Konfiguration |
| `ignoreViolations` | [`SandboxIgnoreViolations`](#sandboxignoreviolations) | `undefined` | Konfigurieren Sie, welche Sandbox-Verstöße ignoriert werden sollen |
| `enableWeakerNestedSandbox` | `boolean` | `false` | Aktivieren Sie eine schwächere verschachtelte Sandbox für Kompatibilität |

<Note>
**Dateisystem- und Netzwerkzugriffsbeschränkungen** werden NICHT über Sandbox-Einstellungen konfiguriert. Stattdessen werden sie von [Berechtigungsregeln](https://code.claude.com/docs/de/settings#permission-settings) abgeleitet:

- **Dateisystem-Lesebeschränkungen**: Regeln zum Ablehnen von Lesevorgängen
- **Dateisystem-Schreibbeschränkungen**: Regeln zum Zulassen/Ablehnen von Bearbeitungen
- **Netzwerkbeschränkungen**: WebFetch-Regeln zum Zulassen/Ablehnen

Verwenden Sie Sandbox-Einstellungen für Befehls-Ausführungs-Sandboxing und Berechtigungsregeln für Dateisystem- und Netzwerkzugriffskontrolle.
</Note>

#### Beispielverwendung

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

Netzwerkspezifische Konfiguration für den Sandbox-Modus.

```typescript
type NetworkSandboxSettings = {
  allowLocalBinding?: boolean;
  allowUnixSockets?: string[];
  allowAllUnixSockets?: boolean;
  httpProxyPort?: number;
  socksProxyPort?: number;
}
```

| Property | Type | Default | Description |
| :------- | :--- | :------ | :---------- |
| `allowLocalBinding` | `boolean` | `false` | Ermöglichen Sie Prozessen, sich an lokale Ports zu binden (z. B. für Dev-Server) |
| `allowUnixSockets` | `string[]` | `[]` | Unix-Socket-Pfade, auf die Prozesse zugreifen können (z. B. Docker-Socket) |
| `allowAllUnixSockets` | `boolean` | `false` | Ermöglichen Sie den Zugriff auf alle Unix-Sockets |
| `httpProxyPort` | `number` | `undefined` | HTTP-Proxy-Port für Netzwerkanfragen |
| `socksProxyPort` | `number` | `undefined` | SOCKS-Proxy-Port für Netzwerkanfragen |

### `SandboxIgnoreViolations`

Konfiguration zum Ignorieren spezifischer Sandbox-Verstöße.

```typescript
type SandboxIgnoreViolations = {
  file?: string[];
  network?: string[];
}
```

| Property | Type | Default | Description |
| :------- | :--- | :------ | :---------- |
| `file` | `string[]` | `[]` | Dateipfad-Muster, für die Verstöße ignoriert werden sollen |
| `network` | `string[]` | `[]` | Netzwerkmuster, für die Verstöße ignoriert werden sollen |

### Berechtigungen-Fallback für Unsandboxed-Befehle

Wenn `allowUnsandboxedCommands` aktiviert ist, kann das Modell anfordern, Befehle außerhalb der Sandbox auszuführen, indem es `dangerouslyDisableSandbox: true` in der Tool-Eingabe setzt. Diese Anfragen fallen auf das bestehende Berechtigungssystem zurück, was bedeutet, dass Ihr `canUseTool`-Handler aufgerufen wird, sodass Sie benutzerdefinierte Autorisierungslogik implementieren können.

<Note>
**`excludedCommands` vs `allowUnsandboxedCommands`:**
- `excludedCommands`: Eine statische Liste von Befehlen, die die Sandbox immer automatisch umgehen (z. B. `['docker']`). Das Modell hat keine Kontrolle darüber.
- `allowUnsandboxedCommands`: Ermöglicht dem Modell, zur Laufzeit zu entscheiden, ob es die Ausführung außerhalb der Sandbox anfordert, indem es `dangerouslyDisableSandbox: true` in der Tool-Eingabe setzt.
</Note>

```typescript
import { query } from "@anthropic-ai/claude-agent-sdk";

const result = await query({
  prompt: "Deploy my application",
  options: {
    sandbox: {
      enabled: true,
      allowUnsandboxedCommands: true  // Model can request unsandboxed execution
    },
    permissionMode: "default",
    canUseTool: async (tool, input) => {
      // Check if the model is requesting to bypass the sandbox
      if (tool === "Bash" && input.dangerouslyDisableSandbox) {
        // The model wants to run this command outside the sandbox
        console.log(`Unsandboxed command requested: ${input.command}`);

        // Return true to allow, false to deny
        return isCommandAuthorized(input.command);
      }
      return true;
    }
  }
});
```

Dieses Muster ermöglicht es Ihnen:

- **Modell-Anfragen prüfen**: Protokollieren Sie, wenn das Modell die Ausführung außerhalb der Sandbox anfordert
- **Allowlists implementieren**: Erlauben Sie nur bestimmten Befehlen, unsandboxed ausgeführt zu werden
- **Genehmigungsworkflows hinzufügen**: Erfordern Sie explizite Autorisierung für privilegierte Operationen

<Warning>
Befehle, die mit `dangerouslyDisableSandbox: true` ausgeführt werden, haben vollständigen Systemzugriff. Stellen Sie sicher, dass Ihr `canUseTool`-Handler diese Anfragen sorgfältig validiert.
</Warning>

## Siehe auch

- [SDK-Übersicht](/docs/de/agent-sdk/overview) - Allgemeine SDK-Konzepte
- [Python SDK-Referenz](/docs/de/agent-sdk/python) - Python SDK-Dokumentation
- [CLI-Referenz](https://code.claude.com/docs/de/cli-reference) - Befehlszeilenschnittstelle
- [Häufige Workflows](https://code.claude.com/docs/de/common-workflows) - Schritt-für-Schritt-Anleitungen