# Sitzungsverwaltung

Verständnis für die Handhabung von Sitzungen und Sitzungswiederaufnahme durch das Claude Agent SDK

---

# Sitzungsverwaltung

Das Claude Agent SDK bietet Sitzungsverwaltungsfunktionen zur Handhabung des Gesprächszustands und der Wiederaufnahme. Sitzungen ermöglichen es Ihnen, Gespräche über mehrere Interaktionen hinweg fortzusetzen und dabei den vollständigen Kontext beizubehalten.

## Wie Sitzungen funktionieren

Wenn Sie eine neue Abfrage starten, erstellt das SDK automatisch eine Sitzung und gibt eine Sitzungs-ID in der anfänglichen Systemmeldung zurück. Sie können diese ID erfassen, um die Sitzung später fortzusetzen.

### Abrufen der Sitzungs-ID

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

## Sitzungen fortsetzen

Das SDK unterstützt die Wiederaufnahme von Sitzungen aus vorherigen Gesprächszuständen und ermöglicht kontinuierliche Entwicklungs-Workflows. Verwenden Sie die Option `resume` mit einer Sitzungs-ID, um ein vorheriges Gespräch fortzusetzen.

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

Das SDK verwaltet automatisch das Laden des Gesprächsverlaufs und des Kontexts, wenn Sie eine Sitzung fortsetzen, sodass Claude genau dort weitermachen kann, wo es aufgehört hat.

<Tip>
Um Dateiänderungen über Sitzungen hinweg zu verfolgen und rückgängig zu machen, siehe [Datei-Checkpointing](/docs/de/agent-sdk/file-checkpointing).
</Tip>

## Sitzungen verzweigen

Beim Fortsetzen einer Sitzung können Sie wählen, ob Sie die ursprüngliche Sitzung fortsetzen oder sie in einen neuen Zweig verzweigen möchten. Standardmäßig setzt das Fortsetzen die ursprüngliche Sitzung fort. Verwenden Sie die Option `forkSession` (TypeScript) oder `fork_session` (Python), um eine neue Sitzungs-ID zu erstellen, die vom fortgesetzten Zustand aus beginnt.

### Wann sollte man eine Sitzung verzweigen

Verzweigung ist nützlich, wenn Sie:
- Verschiedene Ansätze vom gleichen Ausgangspunkt erkunden möchten
- Mehrere Gesprächszweige erstellen möchten, ohne das Original zu ändern
- Änderungen testen möchten, ohne den ursprünglichen Sitzungsverlauf zu beeinflussen
- Separate Gesprächspfade für verschiedene Experimente beibehalten möchten

### Verzweigung vs. Fortsetzung

| Verhalten | `forkSession: false` (Standard) | `forkSession: true` |
|----------|-------------------------------|---------------------|
| **Sitzungs-ID** | Gleich wie Original | Neue Sitzungs-ID generiert |
| **Verlauf** | Wird an ursprüngliche Sitzung angehängt | Erstellt neuen Zweig vom Wiederaufnahmepunkt |
| **Ursprüngliche Sitzung** | Geändert | Unverändert erhalten |
| **Anwendungsfall** | Lineares Gespräch fortsetzen | Verzweigung zum Erkunden von Alternativen |

### Beispiel: Verzweigung einer Sitzung

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