# Gestión de Sesiones

Entender cómo el SDK del Agente Claude maneja sesiones y reanudación de sesiones

---

# Gestión de Sesiones

El SDK del Agente Claude proporciona capacidades de gestión de sesiones para manejar el estado de la conversación y la reanudación. Las sesiones te permiten continuar conversaciones a través de múltiples interacciones mientras mantienes el contexto completo.

## Cómo Funcionan las Sesiones

Cuando inicias una nueva consulta, el SDK crea automáticamente una sesión y devuelve un ID de sesión en el mensaje del sistema inicial. Puedes capturar este ID para reanudar la sesión más tarde.

### Obtener el ID de Sesión

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

## Reanudación de Sesiones

El SDK admite la reanudación de sesiones desde estados de conversación anteriores, lo que permite flujos de trabajo de desarrollo continuo. Usa la opción `resume` con un ID de sesión para continuar una conversación anterior.

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

El SDK maneja automáticamente la carga del historial de conversación y el contexto cuando reanudas una sesión, permitiendo que Claude continúe exactamente donde lo dejó.

<Tip>
Para rastrear y revertir cambios de archivos entre sesiones, consulta [Puntos de Control de Archivos](/docs/es/agent-sdk/file-checkpointing).
</Tip>

## Bifurcación de Sesiones

Cuando reanudas una sesión, puedes elegir entre continuar la sesión original o bifurcarla en una nueva rama. Por defecto, reanudar continúa la sesión original. Usa la opción `forkSession` (TypeScript) u opción `fork_session` (Python) para crear un nuevo ID de sesión que comience desde el estado reanudado.

### Cuándo Bifurcar una Sesión

La bifurcación es útil cuando quieres:
- Explorar diferentes enfoques desde el mismo punto de partida
- Crear múltiples ramas de conversación sin modificar la original
- Probar cambios sin afectar el historial de sesión original
- Mantener rutas de conversación separadas para diferentes experimentos

### Bifurcación vs Continuación

| Comportamiento | `forkSession: false` (predeterminado) | `forkSession: true` |
|----------|-------------------------------|---------------------|
| **ID de Sesión** | Igual que el original | Nuevo ID de sesión generado |
| **Historial** | Se añade a la sesión original | Crea nueva rama desde el punto de reanudación |
| **Sesión Original** | Modificada | Preservada sin cambios |
| **Caso de Uso** | Continuar conversación lineal | Bifurcar para explorar alternativas |

### Ejemplo: Bifurcación de una Sesión

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