# Agent-Verhalten mit Hooks abfangen und steuern

Agent-Verhalten an wichtigen Ausführungspunkten mit Hooks abfangen und anpassen

---

Hooks ermöglichen es Ihnen, die Agent-Ausführung an wichtigen Punkten abzufangen, um Validierung, Protokollierung, Sicherheitskontrollen oder benutzerdefinierte Logik hinzuzufügen. Mit Hooks können Sie:

- **Gefährliche Operationen blockieren**, bevor sie ausgeführt werden, wie destruktive Shell-Befehle oder nicht autorisierter Dateizugriff
- **Protokollieren und überprüfen** jeden Tool-Aufruf für Compliance, Debugging oder Analytik
- **Eingaben und Ausgaben transformieren**, um Daten zu bereinigen, Anmeldedaten einzufügen oder Dateipfade umzuleiten
- **Menschliche Genehmigung erfordern** für sensible Aktionen wie Datenbankschreibvorgänge oder API-Aufrufe
- **Sitzungslebenszyklus verfolgen**, um den Status zu verwalten, Ressourcen freizugeben oder Benachrichtigungen zu senden

Ein Hook hat zwei Teile:

1. **Die Callback-Funktion**: die Logik, die ausgeführt wird, wenn der Hook ausgelöst wird
2. **Die Hook-Konfiguration**: teilt dem SDK mit, welches Ereignis abgefangen werden soll (wie `PreToolUse`) und welche Tools abgeglichen werden sollen

Das folgende Beispiel blockiert den Agent daran, `.env`-Dateien zu ändern. Definieren Sie zunächst einen Callback, der den Dateipfad überprüft, und übergeben Sie ihn dann an `query()`, um ihn vor jedem Write- oder Edit-Tool-Aufruf auszuführen:

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

Dies ist ein `PreToolUse`-Hook. Er wird vor der Ausführung des Tools ausgeführt und kann Operationen basierend auf Ihrer Logik blockieren oder zulassen. Der Rest dieses Leitfadens behandelt alle verfügbaren Hooks, ihre Konfigurationsoptionen und Muster für häufige Anwendungsfälle.

## Verfügbare Hooks

Das SDK bietet Hooks für verschiedene Phasen der Agent-Ausführung. Einige Hooks sind in beiden SDKs verfügbar, während andere nur für TypeScript verfügbar sind, da das Python SDK sie nicht unterstützt.

| Hook-Ereignis | Python SDK | TypeScript SDK | Was löst es aus | Beispiel-Anwendungsfall |
|------------|------------|----------------|------------------|------------------|
| `PreToolUse` | Ja | Ja | Tool-Aufrufanforderung (kann blockiert oder geändert werden) | Gefährliche Shell-Befehle blockieren |
| `PostToolUse` | Ja | Ja | Tool-Ausführungsergebnis | Alle Dateiänderungen im Audit-Trail protokollieren |
| `PostToolUseFailure` | Nein | Ja | Tool-Ausführungsfehler | Tool-Fehler behandeln oder protokollieren |
| `UserPromptSubmit` | Ja | Ja | Benutzer-Prompt-Einreichung | Zusätzlichen Kontext in Prompts einfügen |
| `Stop` | Ja | Ja | Agent-Ausführung stoppen | Sitzungsstatus vor dem Beenden speichern |
| `SubagentStart` | Nein | Ja | Subagent-Initialisierung | Parallele Task-Erzeugung verfolgen |
| `SubagentStop` | Ja | Ja | Subagent-Fertigstellung | Ergebnisse aus parallelen Tasks aggregieren |
| `PreCompact` | Ja | Ja | Anforderung zur Gesprächskomprimierung | Vollständiges Transkript vor der Zusammenfassung archivieren |
| `PermissionRequest` | Nein | Ja | Berechtigungsdialog würde angezeigt | Benutzerdefinierte Berechtigungsbehandlung |
| `SessionStart` | Nein | Ja | Sitzungsinitialisierung | Protokollierung und Telemetrie initialisieren |
| `SessionEnd` | Nein | Ja | Sitzungsbeendigung | Temporäre Ressourcen bereinigen |
| `Notification` | Nein | Ja | Agent-Statusmeldungen | Agent-Status-Updates an Slack oder PagerDuty senden |

## Häufige Anwendungsfälle

Hooks sind flexibel genug, um viele verschiedene Szenarien zu handhaben. Hier sind einige der häufigsten Muster, organisiert nach Kategorie.

<Tabs>
  <Tab title="Sicherheit">
    - Gefährliche Befehle blockieren (wie `rm -rf /`, destruktive SQL)
    - Dateipfade vor Schreibvorgängen validieren
    - Allowlists/Blocklists für Tool-Nutzung erzwingen
  </Tab>
  <Tab title="Protokollierung">
    - Audit-Trails aller Agent-Aktionen erstellen
    - Ausführungsmetriken und Leistung verfolgen
    - Agent-Verhalten in der Entwicklung debuggen
  </Tab>
  <Tab title="Tool-Abfangen">
    - Dateivorgänge in Sandbox-Verzeichnisse umleiten
    - Umgebungsvariablen oder Anmeldedaten einfügen
    - Tool-Eingaben oder -Ausgaben transformieren
  </Tab>
  <Tab title="Autorisierung">
    - Rollenbasierte Zugriffskontrolle implementieren
    - Menschliche Genehmigung für sensible Operationen erfordern
    - Spezifische Tool-Nutzung begrenzen
  </Tab>
</Tabs>

## Hooks konfigurieren

Um einen Hook für Ihren Agent zu konfigurieren, übergeben Sie den Hook im Parameter `options.hooks` beim Aufrufen von `query()`:

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

Die `hooks`-Option ist ein Wörterbuch (Python) oder Objekt (TypeScript), wobei:
- **Schlüssel** [Hook-Ereignisnamen](#available-hooks) sind (z. B. `'PreToolUse'`, `'PostToolUse'`, `'Stop'`)
- **Werte** Arrays von [Matchern](#matchers) sind, die jeweils ein optionales Filtermuster und Ihre [Callback-Funktionen](#callback-function-inputs) enthalten

Ihre Hook-Callback-Funktionen erhalten [Eingabedaten](#input-data) über das Ereignis und geben eine [Antwort](#callback-outputs) zurück, damit der Agent weiß, ob die Operation zulässig, blockiert oder geändert werden soll.

### Matcher

Verwenden Sie Matcher, um zu filtern, welche Tools Ihre Callbacks auslösen:

| Option | Typ | Standard | Beschreibung |
|--------|------|---------|-------------|
| `matcher` | `string` | `undefined` | Regex-Muster zum Abgleich von Tool-Namen. Integrierte Tools umfassen `Bash`, `Read`, `Write`, `Edit`, `Glob`, `Grep`, `WebFetch`, `Task` und andere. MCP-Tools verwenden das Muster `mcp__<server>__<action>`. |
| `hooks` | `HookCallback[]` | - | Erforderlich. Array von Callback-Funktionen, die ausgeführt werden, wenn das Muster übereinstimmt |
| `timeout` | `number` | `60` | Timeout in Sekunden; erhöhen Sie es für Hooks, die externe API-Aufrufe tätigen |

Verwenden Sie das `matcher`-Muster, um wenn möglich spezifische Tools anzusteuern. Ein Matcher mit `'Bash'` wird nur für Bash-Befehle ausgeführt, während das Weglassen des Musters Ihre Callbacks für jeden Tool-Aufruf ausführt. Beachten Sie, dass Matcher nur nach **Tool-Namen** filtern, nicht nach Dateipfaden oder anderen Argumenten – um nach Dateipfad zu filtern, überprüfen Sie `tool_input.file_path` in Ihrem Callback.

Matcher gelten nur für Tool-basierte Hooks (`PreToolUse`, `PostToolUse`, `PostToolUseFailure`, `PermissionRequest`). Für Lifecycle-Hooks wie `Stop`, `SessionStart` und `Notification` werden Matcher ignoriert und der Hook wird für alle Ereignisse dieses Typs ausgelöst.

<Tip>
**Tool-Namen entdecken:** Überprüfen Sie das `tools`-Array in der anfänglichen Systemnachricht, wenn Ihre Sitzung startet, oder fügen Sie einen Hook ohne Matcher hinzu, um alle Tool-Aufrufe zu protokollieren.

**MCP-Tool-Benennung:** MCP-Tools beginnen immer mit `mcp__` gefolgt vom Servernamen und der Aktion: `mcp__<server>__<action>`. Wenn Sie beispielsweise einen Server namens `playwright` konfigurieren, werden seine Tools `mcp__playwright__browser_screenshot`, `mcp__playwright__browser_click` usw. benannt. Der Servername stammt aus dem Schlüssel, den Sie in der `mcpServers`-Konfiguration verwenden.
</Tip>

Dieses Beispiel verwendet einen Matcher, um einen Hook nur für dateiändernde Tools auszuführen, wenn das `PreToolUse`-Ereignis ausgelöst wird:

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

### Callback-Funktionseingaben

Jeder Hook-Callback erhält drei Argumente:

1. **Eingabedaten** (`dict` / `HookInput`): Ereignisdetails. Siehe [Eingabedaten](#input-data) für Felder
2. **Tool-Verwendungs-ID** (`str | None` / `string | null`): Korrelieren Sie `PreToolUse`- und `PostToolUse`-Ereignisse
3. **Kontext** (`HookContext`): In TypeScript enthält eine `signal`-Eigenschaft (`AbortSignal`) zur Stornierung. Übergeben Sie dies an asynchrone Operationen wie `fetch()`, damit sie automatisch storniert werden, wenn der Hook ein Timeout hat. In Python ist dieses Argument für zukünftige Verwendung reserviert.

### Eingabedaten

Das erste Argument für Ihren Hook-Callback enthält Informationen über das Ereignis. Feldnamen sind über SDKs identisch (beide verwenden snake_case).

**Gemeinsame Felder** in allen Hook-Typen:

| Feld | Typ | Beschreibung |
|-------|------|-------------|
| `hook_event_name` | `string` | Der Hook-Typ (`PreToolUse`, `PostToolUse` usw.) |
| `session_id` | `string` | Aktuelle Sitzungskennung |
| `transcript_path` | `string` | Pfad zum Gesprächstranskript |
| `cwd` | `string` | Aktuelles Arbeitsverzeichnis |

**Hook-spezifische Felder** variieren je nach Hook-Typ. Elemente, die mit <sup>TS</sup> gekennzeichnet sind, sind nur im TypeScript SDK verfügbar:

| Feld | Typ | Beschreibung | Hooks |
|-------|------|-------------|-------|
| `tool_name` | `string` | Name des aufgerufenen Tools | PreToolUse, PostToolUse, PostToolUseFailure<sup>TS</sup>, PermissionRequest<sup>TS</sup> |
| `tool_input` | `object` | An das Tool übergebene Argumente | PreToolUse, PostToolUse, PostToolUseFailure<sup>TS</sup>, PermissionRequest<sup>TS</sup> |
| `tool_response` | `any` | Von der Tool-Ausführung zurückgegebenes Ergebnis | PostToolUse |
| `error` | `string` | Fehlermeldung aus Tool-Ausführungsfehler | PostToolUseFailure<sup>TS</sup> |
| `is_interrupt` | `boolean` | Ob der Fehler durch eine Unterbrechung verursacht wurde | PostToolUseFailure<sup>TS</sup> |
| `prompt` | `string` | Der Text des Benutzer-Prompts | UserPromptSubmit |
| `stop_hook_active` | `boolean` | Ob ein Stop-Hook gerade verarbeitet wird | Stop, SubagentStop |
| `agent_id` | `string` | Eindeutige Kennung für den Subagent | SubagentStart<sup>TS</sup>, SubagentStop<sup>TS</sup> |
| `agent_type` | `string` | Typ/Rolle des Subagents | SubagentStart<sup>TS</sup> |
| `agent_transcript_path` | `string` | Pfad zum Gesprächstranskript des Subagents | SubagentStop<sup>TS</sup> |
| `trigger` | `string` | Was die Komprimierung ausgelöst hat: `manual` oder `auto` | PreCompact |
| `custom_instructions` | `string` | Benutzerdefinierte Anweisungen für die Komprimierung | PreCompact |
| `permission_suggestions` | `array` | Vorgeschlagene Berechtigungsaktualisierungen für das Tool | PermissionRequest<sup>TS</sup> |
| `source` | `string` | Wie die Sitzung gestartet wurde: `startup`, `resume`, `clear` oder `compact` | SessionStart<sup>TS</sup> |
| `reason` | `string` | Warum die Sitzung endete: `clear`, `logout`, `prompt_input_exit`, `bypass_permissions_disabled` oder `other` | SessionEnd<sup>TS</sup> |
| `message` | `string` | Statusmeldung vom Agent | Notification<sup>TS</sup> |
| `notification_type` | `string` | Art der Benachrichtigung: `permission_prompt`, `idle_prompt`, `auth_success` oder `elicitation_dialog` | Notification<sup>TS</sup> |
| `title` | `string` | Optionaler Titel, der vom Agent gesetzt wird | Notification<sup>TS</sup> |

Der folgende Code definiert einen Hook-Callback, der `tool_name` und `tool_input` verwendet, um Details zu jedem Tool-Aufruf zu protokollieren:

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

### Callback-Ausgaben

Ihre Callback-Funktion gibt ein Objekt zurück, das dem SDK mitteilt, wie es fortfahren soll. Geben Sie ein leeres Objekt `{}` zurück, um die Operation ohne Änderungen zuzulassen. Um die Operation zu blockieren, zu ändern oder Kontext hinzuzufügen, geben Sie ein Objekt mit einem `hookSpecificOutput`-Feld zurück, das Ihre Entscheidung enthält.

**Top-Level-Felder** (außerhalb von `hookSpecificOutput`):

| Feld | Typ | Beschreibung |
|-------|------|-------------|
| `continue` | `boolean` | Ob der Agent nach diesem Hook fortfahren soll (Standard: `true`) |
| `stopReason` | `string` | Meldung, die angezeigt wird, wenn `continue` `false` ist |
| `suppressOutput` | `boolean` | Stdout aus dem Transkript ausblenden (Standard: `false`) |
| `systemMessage` | `string` | Meldung, die in das Gespräch für Claude eingefügt wird |

**Felder in `hookSpecificOutput`**:

| Feld | Typ | Hooks | Beschreibung |
|-------|------|-------|-------------|
| `hookEventName` | `string` | Alle | Erforderlich. Verwenden Sie `input.hook_event_name`, um das aktuelle Ereignis abzugleichen |
| `permissionDecision` | `'allow'` \| `'deny'` \| `'ask'` | PreToolUse | Steuert, ob das Tool ausgeführt wird |
| `permissionDecisionReason` | `string` | PreToolUse | Erklärung, die Claude für die Entscheidung angezeigt wird |
| `updatedInput` | `object` | PreToolUse | Geänderte Tool-Eingabe (erfordert `permissionDecision: 'allow'`) |
| `additionalContext` | `string` | PostToolUse, UserPromptSubmit, SessionStart<sup>TS</sup>, SubagentStart<sup>TS</sup> | Kontext, der zum Gespräch hinzugefügt wird |

Dieses Beispiel blockiert Schreibvorgänge in das `/etc`-Verzeichnis und fügt gleichzeitig eine Systemnachricht ein, um Claude an sichere Dateipraktiken zu erinnern:

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

#### Berechtigungsentscheidungsfluss

Wenn mehrere Hooks oder Berechtigungsregeln gelten, wertet das SDK sie in dieser Reihenfolge aus:

1. **Deny**-Regeln werden zuerst überprüft (jede Übereinstimmung = sofortige Ablehnung).
2. **Ask**-Regeln werden als zweite überprüft.
3. **Allow**-Regeln werden als dritte überprüft.
4. **Standard auf Ask**, wenn nichts übereinstimmt.

Wenn ein Hook `deny` zurückgibt, wird die Operation blockiert – andere Hooks, die `allow` zurückgeben, können dies nicht überschreiben.

#### Ein Tool blockieren

Geben Sie eine Deny-Entscheidung zurück, um die Tool-Ausführung zu verhindern:

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

#### Tool-Eingabe ändern

Geben Sie aktualisierte Eingabe zurück, um zu ändern, was das Tool erhält:

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
Wenn Sie `updatedInput` verwenden, müssen Sie auch `permissionDecision` einschließen. Geben Sie immer ein neues Objekt zurück, anstatt das ursprüngliche `tool_input` zu mutieren.
</Note>

#### Eine Systemnachricht hinzufügen

Fügen Sie Kontext in das Gespräch ein:

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

#### Spezifische Tools automatisch genehmigen

Umgehen Sie Berechtigungsaufforderungen für vertrauenswürdige Tools. Dies ist nützlich, wenn Sie möchten, dass bestimmte Operationen ohne Benutzerbestätigung ausgeführt werden:

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
Das Feld `permissionDecision` akzeptiert drei Werte: `'allow'` (automatisch genehmigen), `'deny'` (blockieren) oder `'ask'` (zur Bestätigung auffordern).
</Note>

## Erweiterte Szenarien handhaben

Diese Muster helfen Ihnen, anspruchsvollere Hook-Systeme für komplexe Anwendungsfälle zu erstellen.

### Mehrere Hooks verketten

Hooks werden in der Reihenfolge ausgeführt, in der sie im Array erscheinen. Halten Sie jeden Hook auf eine einzelne Verantwortung konzentriert und verketten Sie mehrere Hooks für komplexe Logik. Dieses Beispiel führt alle vier Hooks für jeden Tool-Aufruf aus (kein Matcher angegeben):

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

### Tool-spezifische Matcher mit Regex

Verwenden Sie Regex-Muster, um mehrere Tools abzugleichen:

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
Matcher gleichen nur **Tool-Namen** ab, nicht Dateipfade oder andere Argumente. Um nach Dateipfad zu filtern, überprüfen Sie `tool_input.file_path` in Ihrem Hook-Callback.
</Note>

### Subagent-Aktivität verfolgen

Verwenden Sie `SubagentStop`-Hooks, um die Subagent-Fertigstellung zu überwachen. Die `tool_use_id` hilft, Parent-Agent-Aufrufe mit ihren Subagents zu korrelieren:

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

### Asynchrone Operationen in Hooks

Hooks können asynchrone Operationen wie HTTP-Anfragen ausführen. Behandeln Sie Fehler elegant, indem Sie Ausnahmen abfangen, anstatt sie zu werfen. In TypeScript übergeben Sie `signal` an `fetch()`, damit die Anfrage storniert wird, wenn der Hook ein Timeout hat:

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

### Benachrichtigungen senden (nur TypeScript)

Verwenden Sie `Notification`-Hooks, um Status-Updates vom Agent zu erhalten und diese an externe Dienste wie Slack oder Monitoring-Dashboards weiterzuleiten:

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

## Häufige Probleme beheben

Dieser Abschnitt behandelt häufige Probleme und deren Lösungen.

### Hook wird nicht ausgelöst

- Überprüfen Sie, ob der Hook-Ereignisname korrekt und case-sensitiv ist (`PreToolUse`, nicht `preToolUse`)
- Überprüfen Sie, ob Ihr Matcher-Muster genau mit dem Tool-Namen übereinstimmt
- Stellen Sie sicher, dass der Hook unter dem richtigen Ereignistyp in `options.hooks` ist
- Für `SubagentStop`, `Stop`, `SessionStart`, `SessionEnd` und `Notification`-Hooks werden Matcher ignoriert. Diese Hooks werden für alle Ereignisse dieses Typs ausgelöst.
- Hooks werden möglicherweise nicht ausgelöst, wenn der Agent das [`max_turns`](/docs/de/agent-sdk/python#configuration-options)-Limit erreicht, da die Sitzung endet, bevor Hooks ausgeführt werden können

### Matcher filtert nicht wie erwartet

Matcher gleichen nur **Tool-Namen** ab, nicht Dateipfade oder andere Argumente. Um nach Dateipfad zu filtern, überprüfen Sie `tool_input.file_path` in Ihrem Hook:

```typescript
const myHook: HookCallback = async (input, toolUseID, { signal }) => {
  const preInput = input as PreToolUseHookInput;
  const filePath = preInput.tool_input?.file_path as string;
  if (!filePath?.endsWith('.md')) return {};  // Skip non-markdown files
  // Process markdown files...
};
```

### Hook-Timeout

- Erhöhen Sie den `timeout`-Wert in der `HookMatcher`-Konfiguration
- Verwenden Sie das `AbortSignal` aus dem dritten Callback-Argument, um die Stornierung elegant in TypeScript zu handhaben

### Tool unerwartet blockiert

- Überprüfen Sie alle `PreToolUse`-Hooks auf `permissionDecision: 'deny'`-Rückgaben
- Fügen Sie Protokollierung zu Ihren Hooks hinzu, um zu sehen, welche `permissionDecisionReason` sie zurückgeben
- Überprüfen Sie, ob Matcher-Muster nicht zu breit sind (ein leerer Matcher passt zu allen Tools)

### Geänderte Eingabe nicht angewendet

- Stellen Sie sicher, dass `updatedInput` in `hookSpecificOutput` ist, nicht auf der obersten Ebene:

  ```typescript
  return {
    hookSpecificOutput: {
      hookEventName: input.hook_event_name,
      permissionDecision: 'allow',
      updatedInput: { command: 'new command' }
    }
  };
  ```

- Sie müssen auch `permissionDecision: 'allow'` zurückgeben, damit die Eingabeänderung wirksam wird
- Schließen Sie `hookEventName` in `hookSpecificOutput` ein, um zu identifizieren, welcher Hook-Typ die Ausgabe ist

### Sitzungs-Hooks nicht verfügbar

`SessionStart`, `SessionEnd` und `Notification`-Hooks sind nur im TypeScript SDK verfügbar. Das Python SDK unterstützt diese Ereignisse aufgrund von Setup-Einschränkungen nicht.

### Subagent-Berechtigungsaufforderungen vervielfachen sich

Beim Erzeugen mehrerer Subagents kann jeder einzelne Berechtigungen separat anfordern. Subagents erben nicht automatisch Parent-Agent-Berechtigungen. Um wiederholte Aufforderungen zu vermeiden, verwenden Sie `PreToolUse`-Hooks, um spezifische Tools automatisch zu genehmigen, oder konfigurieren Sie Berechtigungsregeln, die für Subagent-Sitzungen gelten.

### Rekursive Hook-Schleifen mit Subagents

Ein `UserPromptSubmit`-Hook, der Subagents erzeugt, kann unendliche Schleifen erstellen, wenn diese Subagents denselben Hook auslösen. Um dies zu verhindern:

- Überprüfen Sie auf einen Subagent-Indikator in der Hook-Eingabe, bevor Sie Subagents erzeugen
- Verwenden Sie das Feld `parent_tool_use_id`, um zu erkennen, ob Sie sich bereits in einem Subagent-Kontext befinden
- Beschränken Sie Hooks so, dass sie nur für die Top-Level-Agent-Sitzung ausgeführt werden

### systemMessage erscheint nicht in der Ausgabe

Das Feld `systemMessage` fügt Kontext zum Gespräch hinzu, das das Modell sieht, aber es wird möglicherweise nicht in allen SDK-Ausgabemodi angezeigt. Wenn Sie Hook-Entscheidungen für Ihre Anwendung anzeigen müssen, protokollieren Sie sie separat oder verwenden Sie einen dedizierten Ausgabekanal.

## Weitere Informationen

- [Berechtigungen](/docs/de/agent-sdk/permissions): Steuern Sie, was Ihr Agent tun kann
- [Benutzerdefinierte Tools](/docs/de/agent-sdk/custom-tools): Erstellen Sie Tools, um die Agent-Funktionen zu erweitern
- [TypeScript SDK-Referenz](/docs/de/agent-sdk/typescript)
- [Python SDK-Referenz](/docs/de/agent-sdk/python)