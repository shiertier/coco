# Intercettare e controllare il comportamento dell'agente con hook

Intercettare e personalizzare il comportamento dell'agente nei punti chiave di esecuzione con hook

---

Gli hook ti permettono di intercettare l'esecuzione dell'agente nei punti chiave per aggiungere validazione, logging, controlli di sicurezza o logica personalizzata. Con gli hook, puoi:

- **Bloccare operazioni pericolose** prima che vengano eseguite, come comandi shell distruttivi o accesso a file non autorizzato
- **Registrare e controllare** ogni chiamata di strumento per conformità, debug o analitiche
- **Trasformare input e output** per sanificare i dati, iniettare credenziali o reindirizzare percorsi di file
- **Richiedere approvazione umana** per azioni sensibili come scritture su database o chiamate API
- **Tracciare il ciclo di vita della sessione** per gestire lo stato, pulire risorse o inviare notifiche

Un hook ha due parti:

1. **La funzione di callback**: la logica che viene eseguita quando l'hook si attiva
2. **La configurazione dell'hook**: dice all'SDK quale evento agganciare (come `PreToolUse`) e quali strumenti abbinare

L'esempio seguente blocca l'agente dal modificare i file `.env`. Per prima cosa, definisci un callback che controlla il percorso del file, quindi passalo a `query()` per eseguirlo prima di qualsiasi chiamata a uno strumento Write o Edit:

<CodeGroup>

```python Python
import asyncio
from claude_agent_sdk import query, ClaudeAgentOptions, HookMatcher

# Definisci un callback hook che riceve i dettagli della chiamata dello strumento
async def protect_env_files(input_data, tool_use_id, context):
    # Estrai il percorso del file dagli argomenti di input dello strumento
    file_path = input_data['tool_input'].get('file_path', '')
    file_name = file_path.split('/')[-1]

    # Blocca l'operazione se è destinata a un file .env
    if file_name == '.env':
        return {
            'hookSpecificOutput': {
                'hookEventName': input_data['hook_event_name'],
                'permissionDecision': 'deny',
                'permissionDecisionReason': 'Cannot modify .env files'
            }
        }

    # Restituisci un oggetto vuoto per consentire l'operazione
    return {}

async def main():
    async for message in query(
        prompt="Update the database configuration",
        options=ClaudeAgentOptions(
            hooks={
                # Registra l'hook per gli eventi PreToolUse
                # Il matcher filtra solo le chiamate agli strumenti Write e Edit
                'PreToolUse': [HookMatcher(matcher='Write|Edit', hooks=[protect_env_files])]
            }
        )
    ):
        print(message)

asyncio.run(main())
```

```typescript TypeScript
import { query, HookCallback, PreToolUseHookInput } from "@anthropic-ai/claude-agent-sdk";

// Definisci un callback hook con il tipo HookCallback
const protectEnvFiles: HookCallback = async (input, toolUseID, { signal }) => {
  // Esegui il cast dell'input al tipo di hook specifico per la sicurezza dei tipi
  const preInput = input as PreToolUseHookInput;

  // Estrai il percorso del file dagli argomenti di input dello strumento
  const filePath = preInput.tool_input?.file_path as string;
  const fileName = filePath?.split('/').pop();

  // Blocca l'operazione se è destinata a un file .env
  if (fileName === '.env') {
    return {
      hookSpecificOutput: {
        hookEventName: input.hook_event_name,
        permissionDecision: 'deny',
        permissionDecisionReason: 'Cannot modify .env files'
      }
    };
  }

  // Restituisci un oggetto vuoto per consentire l'operazione
  return {};
};

for await (const message of query({
  prompt: "Update the database configuration",
  options: {
    hooks: {
      // Registra l'hook per gli eventi PreToolUse
      // Il matcher filtra solo le chiamate agli strumenti Write e Edit
      PreToolUse: [{ matcher: 'Write|Edit', hooks: [protectEnvFiles] }]
    }
  }
})) {
  console.log(message);
}
```

</CodeGroup>

Questo è un hook `PreToolUse`. Viene eseguito prima che lo strumento si esegua e può bloccare o consentire operazioni in base alla tua logica. Il resto di questa guida copre tutti gli hook disponibili, le loro opzioni di configurazione e i modelli per i casi d'uso comuni.

## Hook disponibili

L'SDK fornisce hook per diverse fasi dell'esecuzione dell'agente. Alcuni hook sono disponibili in entrambi gli SDK, mentre altri sono solo TypeScript perché l'SDK Python non li supporta.

| Hook Event | Python SDK | TypeScript SDK | Cosa lo attiva | Caso d'uso di esempio |
|------------|------------|----------------|------------------|------------------|
| `PreToolUse` | Sì | Sì | Richiesta di chiamata dello strumento (può bloccare o modificare) | Bloccare comandi shell pericolosi |
| `PostToolUse` | Sì | Sì | Risultato dell'esecuzione dello strumento | Registrare tutti i cambiamenti ai file nel registro di controllo |
| `PostToolUseFailure` | No | Sì | Errore nell'esecuzione dello strumento | Gestire o registrare errori dello strumento |
| `UserPromptSubmit` | Sì | Sì | Invio del prompt dell'utente | Iniettare contesto aggiuntivo nei prompt |
| `Stop` | Sì | Sì | Arresto dell'esecuzione dell'agente | Salvare lo stato della sessione prima dell'uscita |
| `SubagentStart` | No | Sì | Inizializzazione del subagente | Tracciare l'avvio di attività parallele |
| `SubagentStop` | Sì | Sì | Completamento del subagente | Aggregare i risultati dalle attività parallele |
| `PreCompact` | Sì | Sì | Richiesta di compattazione della conversazione | Archiviare la trascrizione completa prima del riepilogo |
| `PermissionRequest` | No | Sì | La finestra di dialogo delle autorizzazioni verrebbe visualizzata | Gestione personalizzata delle autorizzazioni |
| `SessionStart` | No | Sì | Inizializzazione della sessione | Inizializzare logging e telemetria |
| `SessionEnd` | No | Sì | Terminazione della sessione | Pulire le risorse temporanee |
| `Notification` | No | Sì | Messaggi di stato dell'agente | Inviare aggiornamenti dello stato dell'agente a Slack o PagerDuty |

## Casi d'uso comuni

Gli hook sono abbastanza flessibili da gestire molti scenari diversi. Ecco alcuni dei modelli più comuni organizzati per categoria.

<Tabs>
  <Tab title="Sicurezza">
    - Bloccare comandi pericolosi (come `rm -rf /`, SQL distruttivo)
    - Convalidare i percorsi dei file prima delle operazioni di scrittura
    - Applicare allowlist/blocklist per l'utilizzo degli strumenti
  </Tab>
  <Tab title="Logging">
    - Creare audit trail di tutte le azioni dell'agente
    - Tracciare metriche di esecuzione e prestazioni
    - Eseguire il debug del comportamento dell'agente nello sviluppo
  </Tab>
  <Tab title="Intercettazione degli strumenti">
    - Reindirizzare le operazioni sui file a directory sandbox
    - Iniettare variabili di ambiente o credenziali
    - Trasformare input o output degli strumenti
  </Tab>
  <Tab title="Autorizzazione">
    - Implementare il controllo degli accessi basato sui ruoli
    - Richiedere approvazione umana per operazioni sensibili
    - Limitare la velocità di utilizzo di strumenti specifici
  </Tab>
</Tabs>

## Configurare gli hook

Per configurare un hook per il tuo agente, passa l'hook nel parametro `options.hooks` quando chiami `query()`:

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

L'opzione `hooks` è un dizionario (Python) o un oggetto (TypeScript) dove:
- **Le chiavi** sono [nomi degli eventi hook](#available-hooks) (ad es. `'PreToolUse'`, `'PostToolUse'`, `'Stop'`)
- **I valori** sono array di [matcher](#matchers), ognuno contenente un modello di filtro opzionale e le tue [funzioni di callback](#callback-function-inputs)

Le tue funzioni di callback hook ricevono [dati di input](#input-data) sull'evento e restituiscono una [risposta](#callback-outputs) in modo che l'agente sappia se consentire, bloccare o modificare l'operazione.

### Matcher

Usa i matcher per filtrare quali strumenti attivano i tuoi callback:

| Opzione | Tipo | Predefinito | Descrizione |
|--------|------|---------|-------------|
| `matcher` | `string` | `undefined` | Modello regex per abbinare i nomi degli strumenti. Gli strumenti integrati includono `Bash`, `Read`, `Write`, `Edit`, `Glob`, `Grep`, `WebFetch`, `Task` e altri. Gli strumenti MCP utilizzano il modello `mcp__<server>__<action>`. |
| `hooks` | `HookCallback[]` | - | Obbligatorio. Array di funzioni di callback da eseguire quando il modello corrisponde |
| `timeout` | `number` | `60` | Timeout in secondi; aumenta per gli hook che effettuano chiamate API esterne |

Usa il modello `matcher` per indirizzare strumenti specifici quando possibile. Un matcher con `'Bash'` viene eseguito solo per i comandi Bash, mentre omettere il modello esegue i tuoi callback per ogni chiamata di strumento. Nota che i matcher filtrano solo per **nome dello strumento**, non per percorsi di file o altri argomenti—per filtrare per percorso di file, controlla `tool_input.file_path` all'interno del tuo callback.

I matcher si applicano solo agli hook basati su strumenti (`PreToolUse`, `PostToolUse`, `PostToolUseFailure`, `PermissionRequest`). Per gli hook del ciclo di vita come `Stop`, `SessionStart` e `Notification`, i matcher vengono ignorati e l'hook si attiva per tutti gli eventi di quel tipo.

<Tip>
**Scoprire i nomi degli strumenti:** Controlla l'array `tools` nel messaggio di sistema iniziale quando la tua sessione inizia, oppure aggiungi un hook senza un matcher per registrare tutte le chiamate agli strumenti.

**Denominazione degli strumenti MCP:** Gli strumenti MCP iniziano sempre con `mcp__` seguito dal nome del server e dall'azione: `mcp__<server>__<action>`. Ad esempio, se configuri un server denominato `playwright`, i suoi strumenti saranno denominati `mcp__playwright__browser_screenshot`, `mcp__playwright__browser_click`, ecc. Il nome del server proviene dalla chiave che usi nella configurazione `mcpServers`.
</Tip>

Questo esempio utilizza un matcher per eseguire un hook solo per gli strumenti che modificano i file quando si attiva l'evento `PreToolUse`:

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

### Input della funzione di callback

Ogni callback hook riceve tre argomenti:

1. **Dati di input** (`dict` / `HookInput`): Dettagli dell'evento. Vedi [dati di input](#input-data) per i campi
2. **ID di utilizzo dello strumento** (`str | None` / `string | null`): Correla gli eventi `PreToolUse` e `PostToolUse`
3. **Contesto** (`HookContext`): In TypeScript, contiene una proprietà `signal` (`AbortSignal`) per l'annullamento. Passalo alle operazioni asincrone come `fetch()` in modo che si annullino automaticamente se l'hook scade. In Python, questo argomento è riservato per uso futuro.

### Dati di input

Il primo argomento del tuo callback hook contiene informazioni sull'evento. I nomi dei campi sono identici tra gli SDK (entrambi usano snake_case).

**Campi comuni** presenti in tutti i tipi di hook:

| Campo | Tipo | Descrizione |
|-------|------|-------------|
| `hook_event_name` | `string` | Il tipo di hook (`PreToolUse`, `PostToolUse`, ecc.) |
| `session_id` | `string` | Identificatore della sessione corrente |
| `transcript_path` | `string` | Percorso della trascrizione della conversazione |
| `cwd` | `string` | Directory di lavoro corrente |

**Campi specifici dell'hook** variano in base al tipo di hook. Gli elementi contrassegnati <sup>TS</sup> sono disponibili solo nell'SDK TypeScript:

| Campo | Tipo | Descrizione | Hook |
|-------|------|-------------|-------|
| `tool_name` | `string` | Nome dello strumento in fase di chiamata | PreToolUse, PostToolUse, PostToolUseFailure<sup>TS</sup>, PermissionRequest<sup>TS</sup> |
| `tool_input` | `object` | Argomenti passati allo strumento | PreToolUse, PostToolUse, PostToolUseFailure<sup>TS</sup>, PermissionRequest<sup>TS</sup> |
| `tool_response` | `any` | Risultato restituito dall'esecuzione dello strumento | PostToolUse |
| `error` | `string` | Messaggio di errore dall'errore di esecuzione dello strumento | PostToolUseFailure<sup>TS</sup> |
| `is_interrupt` | `boolean` | Se l'errore è stato causato da un'interruzione | PostToolUseFailure<sup>TS</sup> |
| `prompt` | `string` | Il testo del prompt dell'utente | UserPromptSubmit |
| `stop_hook_active` | `boolean` | Se un hook di arresto è attualmente in elaborazione | Stop, SubagentStop |
| `agent_id` | `string` | Identificatore univoco per il subagente | SubagentStart<sup>TS</sup>, SubagentStop<sup>TS</sup> |
| `agent_type` | `string` | Tipo/ruolo del subagente | SubagentStart<sup>TS</sup> |
| `agent_transcript_path` | `string` | Percorso della trascrizione della conversazione del subagente | SubagentStop<sup>TS</sup> |
| `trigger` | `string` | Cosa ha attivato la compattazione: `manual` o `auto` | PreCompact |
| `custom_instructions` | `string` | Istruzioni personalizzate fornite per la compattazione | PreCompact |
| `permission_suggestions` | `array` | Aggiornamenti delle autorizzazioni suggeriti per lo strumento | PermissionRequest<sup>TS</sup> |
| `source` | `string` | Come è iniziata la sessione: `startup`, `resume`, `clear` o `compact` | SessionStart<sup>TS</sup> |
| `reason` | `string` | Perché è terminata la sessione: `clear`, `logout`, `prompt_input_exit`, `bypass_permissions_disabled` o `other` | SessionEnd<sup>TS</sup> |
| `message` | `string` | Messaggio di stato dall'agente | Notification<sup>TS</sup> |
| `notification_type` | `string` | Tipo di notifica: `permission_prompt`, `idle_prompt`, `auth_success` o `elicitation_dialog` | Notification<sup>TS</sup> |
| `title` | `string` | Titolo opzionale impostato dall'agente | Notification<sup>TS</sup> |

Il codice seguente definisce un callback hook che utilizza `tool_name` e `tool_input` per registrare i dettagli di ogni chiamata di strumento:

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

### Output del callback

La tua funzione di callback restituisce un oggetto che dice all'SDK come procedere. Restituisci un oggetto vuoto `{}` per consentire l'operazione senza modifiche. Per bloccare, modificare o aggiungere contesto all'operazione, restituisci un oggetto con un campo `hookSpecificOutput` contenente la tua decisione.

**Campi di livello superiore** (al di fuori di `hookSpecificOutput`):

| Campo | Tipo | Descrizione |
|-------|------|-------------|
| `continue` | `boolean` | Se l'agente deve continuare dopo questo hook (predefinito: `true`) |
| `stopReason` | `string` | Messaggio mostrato quando `continue` è `false` |
| `suppressOutput` | `boolean` | Nascondere stdout dalla trascrizione (predefinito: `false`) |
| `systemMessage` | `string` | Messaggio iniettato nella conversazione affinché Claude lo veda |

**Campi all'interno di `hookSpecificOutput`**:

| Campo | Tipo | Hook | Descrizione |
|-------|------|-------|-------------|
| `hookEventName` | `string` | Tutti | Obbligatorio. Usa `input.hook_event_name` per abbinare l'evento corrente |
| `permissionDecision` | `'allow'` \| `'deny'` \| `'ask'` | PreToolUse | Controlla se lo strumento viene eseguito |
| `permissionDecisionReason` | `string` | PreToolUse | Spiegazione mostrata a Claude per la decisione |
| `updatedInput` | `object` | PreToolUse | Input dello strumento modificato (richiede `permissionDecision: 'allow'`) |
| `additionalContext` | `string` | PostToolUse, UserPromptSubmit, SessionStart<sup>TS</sup>, SubagentStart<sup>TS</sup> | Contesto aggiunto alla conversazione |

Questo esempio blocca le operazioni di scrittura nella directory `/etc` mentre inietta un messaggio di sistema per ricordare a Claude le pratiche di file sicure:

<CodeGroup>

```python Python
async def block_etc_writes(input_data, tool_use_id, context):
    file_path = input_data['tool_input'].get('file_path', '')

    if file_path.startswith('/etc'):
        return {
            # Campo di livello superiore: inietta guida nella conversazione
            'systemMessage': 'Remember: system directories like /etc are protected.',
            # hookSpecificOutput: blocca l'operazione
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
      // Campo di livello superiore: inietta guida nella conversazione
      systemMessage: 'Remember: system directories like /etc are protected.',
      // hookSpecificOutput: blocca l'operazione
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

#### Flusso di decisione delle autorizzazioni

Quando si applicano più hook o regole di autorizzazione, l'SDK le valuta in questo ordine:

1. Le regole **Deny** vengono controllate per prime (qualsiasi corrispondenza = negazione immediata).
2. Le regole **Ask** vengono controllate per seconde.
3. Le regole **Allow** vengono controllate per terze.
4. **Predefinito su Ask** se nulla corrisponde.

Se un hook restituisce `deny`, l'operazione viene bloccata—altri hook che restituiscono `allow` non la sovrascriveranno.

#### Bloccare uno strumento

Restituisci una decisione di negazione per impedire l'esecuzione dello strumento:

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

#### Modificare l'input dello strumento

Restituisci l'input aggiornato per cambiare ciò che lo strumento riceve:

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
Quando usi `updatedInput`, devi anche includere `permissionDecision`. Restituisci sempre un nuovo oggetto piuttosto che mutare il `tool_input` originale.
</Note>

#### Aggiungere un messaggio di sistema

Inietta contesto nella conversazione:

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

#### Approvare automaticamente strumenti specifici

Ignora i prompt di autorizzazione per gli strumenti affidabili. Questo è utile quando vuoi che determinate operazioni vengano eseguite senza conferma dell'utente:

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
Il campo `permissionDecision` accetta tre valori: `'allow'` (approva automaticamente), `'deny'` (blocca) o `'ask'` (richiedi conferma).
</Note>

## Gestire scenari avanzati

Questi modelli ti aiutano a costruire sistemi di hook più sofisticati per casi d'uso complessi.

### Concatenare più hook

Gli hook vengono eseguiti nell'ordine in cui appaiono nell'array. Mantieni ogni hook focalizzato su una singola responsabilità e concatena più hook per logica complessa. Questo esempio esegue tutti e quattro gli hook per ogni chiamata di strumento (nessun matcher specificato):

<CodeGroup>

```python Python
options = ClaudeAgentOptions(
    hooks={
        'PreToolUse': [
            HookMatcher(hooks=[rate_limiter]),        # Primo: controlla i limiti di velocità
            HookMatcher(hooks=[authorization_check]), # Secondo: verifica le autorizzazioni
            HookMatcher(hooks=[input_sanitizer]),     # Terzo: sanifica gli input
            HookMatcher(hooks=[audit_logger])         # Ultimo: registra l'azione
        ]
    }
)
```

```typescript TypeScript
const options = {
  hooks: {
    'PreToolUse': [
      { hooks: [rateLimiter] },        // Primo: controlla i limiti di velocità
      { hooks: [authorizationCheck] }, // Secondo: verifica le autorizzazioni
      { hooks: [inputSanitizer] },     // Terzo: sanifica gli input
      { hooks: [auditLogger] }         // Ultimo: registra l'azione
    ]
  }
};
```

</CodeGroup>

### Matcher specifici per strumenti con regex

Usa modelli regex per abbinare più strumenti:

<CodeGroup>

```python Python
options = ClaudeAgentOptions(
    hooks={
        'PreToolUse': [
            # Abbina gli strumenti di modifica dei file
            HookMatcher(matcher='Write|Edit|Delete', hooks=[file_security_hook]),

            # Abbina tutti gli strumenti MCP
            HookMatcher(matcher='^mcp__', hooks=[mcp_audit_hook]),

            # Abbina tutto (nessun matcher)
            HookMatcher(hooks=[global_logger])
        ]
    }
)
```

```typescript TypeScript
const options = {
  hooks: {
    'PreToolUse': [
      // Abbina gli strumenti di modifica dei file
      { matcher: 'Write|Edit|Delete', hooks: [fileSecurityHook] },

      // Abbina tutti gli strumenti MCP
      { matcher: '^mcp__', hooks: [mcpAuditHook] },

      // Abbina tutto (nessun matcher)
      { hooks: [globalLogger] }
    ]
  }
};
```

</CodeGroup>

<Note>
I matcher abbinano solo i **nomi degli strumenti**, non i percorsi dei file o altri argomenti. Per filtrare per percorso di file, controlla `tool_input.file_path` all'interno del tuo callback hook.
</Note>

### Tracciare l'attività del subagente

Usa gli hook `SubagentStop` per monitorare il completamento del subagente. L'`tool_use_id` aiuta a correlare le chiamate dell'agente genitore con i loro subagenti:

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

### Operazioni asincrone negli hook

Gli hook possono eseguire operazioni asincrone come richieste HTTP. Gestisci gli errori con grazia catturando le eccezioni invece di lanciarle. In TypeScript, passa il `signal` a `fetch()` in modo che la richiesta si annulli se l'hook scade:

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
    // Passa il signal per il corretto annullamento
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

### Inviare notifiche (solo TypeScript)

Usa gli hook `Notification` per ricevere aggiornamenti di stato dall'agente e inoltrarli a servizi esterni come Slack o dashboard di monitoraggio:

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

## Risolvere i problemi comuni

Questa sezione copre i problemi comuni e come risolverli.

### Hook non si attiva

- Verifica che il nome dell'evento hook sia corretto e sensibile alle maiuscole (`PreToolUse`, non `preToolUse`)
- Controlla che il tuo modello di matcher corrisponda esattamente al nome dello strumento
- Assicurati che l'hook sia sotto il tipo di evento corretto in `options.hooks`
- Per gli hook `SubagentStop`, `Stop`, `SessionStart`, `SessionEnd` e `Notification`, i matcher vengono ignorati. Questi hook si attivano per tutti gli eventi di quel tipo.
- Gli hook potrebbero non attivarsi quando l'agente raggiunge il limite [`max_turns`](/docs/it/agent-sdk/python#configuration-options) perché la sessione termina prima che gli hook possano essere eseguiti

### Matcher non filtra come previsto

I matcher abbinano solo i **nomi degli strumenti**, non i percorsi dei file o altri argomenti. Per filtrare per percorso di file, controlla `tool_input.file_path` all'interno del tuo hook:

```typescript
const myHook: HookCallback = async (input, toolUseID, { signal }) => {
  const preInput = input as PreToolUseHookInput;
  const filePath = preInput.tool_input?.file_path as string;
  if (!filePath?.endsWith('.md')) return {};  // Salta i file non markdown
  // Elabora i file markdown...
};
```

### Timeout dell'hook

- Aumenta il valore `timeout` nella configurazione `HookMatcher`
- Usa l'`AbortSignal` dal terzo argomento del callback per gestire l'annullamento con grazia in TypeScript

### Strumento bloccato inaspettatamente

- Controlla tutti gli hook `PreToolUse` per i ritorni `permissionDecision: 'deny'`
- Aggiungi logging ai tuoi hook per vedere quale `permissionDecisionReason` stanno restituendo
- Verifica che i modelli di matcher non siano troppo ampi (un matcher vuoto abbina tutti gli strumenti)

### Input modificato non applicato

- Assicurati che `updatedInput` sia all'interno di `hookSpecificOutput`, non al livello superiore:

  ```typescript
  return {
    hookSpecificOutput: {
      hookEventName: input.hook_event_name,
      permissionDecision: 'allow',
      updatedInput: { command: 'new command' }
    }
  };
  ```

- Devi anche restituire `permissionDecision: 'allow'` affinché la modifica dell'input abbia effetto
- Includi `hookEventName` in `hookSpecificOutput` per identificare quale tipo di hook è l'output

### Hook di sessione non disponibili

Gli hook `SessionStart`, `SessionEnd` e `Notification` sono disponibili solo nell'SDK TypeScript. L'SDK Python non supporta questi eventi a causa di limitazioni di configurazione.

### Prompt di autorizzazione del subagente moltiplicati

Quando si generano più subagenti, ognuno potrebbe richiedere autorizzazioni separatamente. I subagenti non ereditano automaticamente le autorizzazioni dell'agente genitore. Per evitare prompt ripetuti, usa gli hook `PreToolUse` per approvare automaticamente strumenti specifici, o configura regole di autorizzazione che si applicano alle sessioni dei subagenti.

### Loop ricorsivi di hook con subagenti

Un hook `UserPromptSubmit` che genera subagenti può creare loop infiniti se quei subagenti attivano lo stesso hook. Per prevenire questo:

- Controlla un indicatore di subagente nell'input dell'hook prima di generare
- Usa il campo `parent_tool_use_id` per rilevare se sei già in un contesto di subagente
- Limita gli hook per eseguire solo per la sessione dell'agente di livello superiore

### systemMessage non appare nell'output

Il campo `systemMessage` aggiunge contesto alla conversazione che il modello vede, ma potrebbe non apparire in tutte le modalità di output dell'SDK. Se hai bisogno di esporre le decisioni degli hook alla tua applicazione, registrale separatamente o usa un canale di output dedicato.

## Scopri di più

- [Autorizzazioni](/docs/it/agent-sdk/permissions): controlla cosa può fare il tuo agente
- [Strumenti personalizzati](/docs/it/agent-sdk/custom-tools): costruisci strumenti per estendere le capacità dell'agente
- [Riferimento SDK TypeScript](/docs/it/agent-sdk/typescript)
- [Riferimento SDK Python](/docs/it/agent-sdk/python)