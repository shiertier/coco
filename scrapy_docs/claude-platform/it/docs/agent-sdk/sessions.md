# Gestione delle Sessioni

Comprensione di come Claude Agent SDK gestisce le sessioni e la ripresa delle sessioni

---

# Gestione delle Sessioni

Claude Agent SDK fornisce capacità di gestione delle sessioni per gestire lo stato della conversazione e la ripresa. Le sessioni ti permettono di continuare le conversazioni attraverso più interazioni mantenendo il contesto completo.

## Come Funzionano le Sessioni

Quando avvii una nuova query, l'SDK crea automaticamente una sessione e restituisce un ID di sessione nel messaggio di sistema iniziale. Puoi acquisire questo ID per riprendere la sessione in seguito.

### Ottenere l'ID della Sessione

<CodeGroup>

```typescript TypeScript
import { query } from "@anthropic-ai/claude-agent-sdk"

let sessionId: string | undefined

const response = query({
  prompt: "Help me build a web application",
  options: {
    model: "claude-sonnet-4-5"
  }
})

for await (const message of response) {
  // The first message is a system init message with the session ID
  if (message.type === 'system' && message.subtype === 'init') {
    sessionId = message.session_id
    console.log(`Session started with ID: ${sessionId}`)
    // You can save this ID for later resumption
  }

  // Process other messages...
  console.log(message)
}

// Later, you can use the saved sessionId to resume
if (sessionId) {
  const resumedResponse = query({
    prompt: "Continue where we left off",
    options: {
      resume: sessionId
    }
  })
}
```

```python Python
from claude_agent_sdk import query, ClaudeAgentOptions

session_id = None

async for message in query(
    prompt="Help me build a web application",
    options=ClaudeAgentOptions(
        model="claude-sonnet-4-5"
    )
):
    # The first message is a system init message with the session ID
    if hasattr(message, 'subtype') and message.subtype == 'init':
        session_id = message.data.get('session_id')
        print(f"Session started with ID: {session_id}")
        # You can save this ID for later resumption

    # Process other messages...
    print(message)

# Later, you can use the saved session_id to resume
if session_id:
    async for message in query(
        prompt="Continue where we left off",
        options=ClaudeAgentOptions(
            resume=session_id
        )
    ):
        print(message)
```

</CodeGroup>

## Ripresa delle Sessioni

L'SDK supporta la ripresa delle sessioni da stati di conversazione precedenti, abilitando flussi di lavoro di sviluppo continuo. Utilizza l'opzione `resume` con un ID di sessione per continuare una conversazione precedente.

<CodeGroup>

```typescript TypeScript
import { query } from "@anthropic-ai/claude-agent-sdk"

// Resume a previous session using its ID
const response = query({
  prompt: "Continue implementing the authentication system from where we left off",
  options: {
    resume: "session-xyz", // Session ID from previous conversation
    model: "claude-sonnet-4-5",
    allowedTools: ["Read", "Edit", "Write", "Glob", "Grep", "Bash"]
  }
})

// The conversation continues with full context from the previous session
for await (const message of response) {
  console.log(message)
}
```

```python Python
from claude_agent_sdk import query, ClaudeAgentOptions

# Resume a previous session using its ID
async for message in query(
    prompt="Continue implementing the authentication system from where we left off",
    options=ClaudeAgentOptions(
        resume="session-xyz",  # Session ID from previous conversation
        model="claude-sonnet-4-5",
        allowed_tools=["Read", "Edit", "Write", "Glob", "Grep", "Bash"]
    )
):
    print(message)

# The conversation continues with full context from the previous session
```

</CodeGroup>

L'SDK gestisce automaticamente il caricamento della cronologia della conversazione e del contesto quando riprendi una sessione, permettendo a Claude di continuare esattamente da dove si era fermato.

<Tip>
Per tracciare e ripristinare i cambiamenti dei file tra le sessioni, vedi [File Checkpointing](/docs/it/agent-sdk/file-checkpointing).
</Tip>

## Creazione di Rami di Sessioni

Quando riprendi una sessione, puoi scegliere di continuare la sessione originale o creare un ramo in una nuova sessione. Per impostazione predefinita, la ripresa continua la sessione originale. Utilizza l'opzione `forkSession` (TypeScript) o `fork_session` (Python) per creare un nuovo ID di sessione che inizia dallo stato ripreso.

### Quando Creare un Ramo di una Sessione

La creazione di un ramo è utile quando vuoi:
- Esplorare approcci diversi dallo stesso punto di partenza
- Creare più rami di conversazione senza modificare l'originale
- Testare i cambiamenti senza influenzare la cronologia della sessione originale
- Mantenere percorsi di conversazione separati per diversi esperimenti

### Creazione di Rami vs Continuazione

| Comportamento | `forkSession: false` (predefinito) | `forkSession: true` |
|----------|-------------------------------|---------------------|
| **ID Sessione** | Uguale all'originale | Nuovo ID di sessione generato |
| **Cronologia** | Aggiunge alla sessione originale | Crea un nuovo ramo dal punto di ripresa |
| **Sessione Originale** | Modificata | Preservata invariata |
| **Caso d'Uso** | Continua conversazione lineare | Ramo per esplorare alternative |

### Esempio: Creazione di un Ramo di una Sessione

<CodeGroup>

```typescript TypeScript
import { query } from "@anthropic-ai/claude-agent-sdk"

// First, capture the session ID
let sessionId: string | undefined

const response = query({
  prompt: "Help me design a REST API",
  options: { model: "claude-sonnet-4-5" }
})

for await (const message of response) {
  if (message.type === 'system' && message.subtype === 'init') {
    sessionId = message.session_id
    console.log(`Original session: ${sessionId}`)
  }
}

// Fork the session to try a different approach
const forkedResponse = query({
  prompt: "Now let's redesign this as a GraphQL API instead",
  options: {
    resume: sessionId,
    forkSession: true,  // Creates a new session ID
    model: "claude-sonnet-4-5"
  }
})

for await (const message of forkedResponse) {
  if (message.type === 'system' && message.subtype === 'init') {
    console.log(`Forked session: ${message.session_id}`)
    // This will be a different session ID
  }
}

// The original session remains unchanged and can still be resumed
const originalContinued = query({
  prompt: "Add authentication to the REST API",
  options: {
    resume: sessionId,
    forkSession: false,  // Continue original session (default)
    model: "claude-sonnet-4-5"
  }
})
```

```python Python
from claude_agent_sdk import query, ClaudeAgentOptions

# First, capture the session ID
session_id = None

async for message in query(
    prompt="Help me design a REST API",
    options=ClaudeAgentOptions(model="claude-sonnet-4-5")
):
    if hasattr(message, 'subtype') and message.subtype == 'init':
        session_id = message.data.get('session_id')
        print(f"Original session: {session_id}")

# Fork the session to try a different approach
async for message in query(
    prompt="Now let's redesign this as a GraphQL API instead",
    options=ClaudeAgentOptions(
        resume=session_id,
        fork_session=True,  # Creates a new session ID
        model="claude-sonnet-4-5"
    )
):
    if hasattr(message, 'subtype') and message.subtype == 'init':
        forked_id = message.data.get('session_id')
        print(f"Forked session: {forked_id}")
        # This will be a different session ID

# The original session remains unchanged and can still be resumed
async for message in query(
    prompt="Add authentication to the REST API",
    options=ClaudeAgentOptions(
        resume=session_id,
        fork_session=False,  # Continue original session (default)
        model="claude-sonnet-4-5"
    )
):
    print(message)
```

</CodeGroup>