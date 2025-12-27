# Gestion des sessions

Comprendre comment le Claude Agent SDK gère les sessions et la reprise de session

---

# Gestion des sessions

Le Claude Agent SDK fournit des capacités de gestion de session pour gérer l'état de la conversation et la reprise. Les sessions vous permettent de continuer les conversations sur plusieurs interactions tout en maintenant le contexte complet.

## Comment fonctionnent les sessions

Lorsque vous démarrez une nouvelle requête, le SDK crée automatiquement une session et retourne un ID de session dans le message système initial. Vous pouvez capturer cet ID pour reprendre la session plus tard.

### Obtenir l'ID de session

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

## Reprendre les sessions

Le SDK supporte la reprise de sessions à partir d'états de conversation précédents, permettant des flux de travail de développement continu. Utilisez l'option `resume` avec un ID de session pour continuer une conversation précédente.

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

Le SDK gère automatiquement le chargement de l'historique de conversation et du contexte lorsque vous reprenez une session, permettant à Claude de continuer exactement où il s'était arrêté.

<Tip>
Pour suivre et annuler les modifications de fichiers entre les sessions, voir [File Checkpointing](/docs/fr/agent-sdk/file-checkpointing).
</Tip>

## Forker les sessions

Lors de la reprise d'une session, vous pouvez choisir de continuer la session originale ou de la forker dans une nouvelle branche. Par défaut, la reprise continue la session originale. Utilisez l'option `forkSession` (TypeScript) ou `fork_session` (Python) pour créer un nouvel ID de session qui commence à partir de l'état repris.

### Quand forker une session

Le forking est utile lorsque vous voulez :
- Explorer différentes approches à partir du même point de départ
- Créer plusieurs branches de conversation sans modifier l'original
- Tester des modifications sans affecter l'historique de session original
- Maintenir des chemins de conversation séparés pour différentes expériences

### Forker vs Continuer

| Comportement | `forkSession: false` (par défaut) | `forkSession: true` |
|----------|-------------------------------|---------------------|
| **ID de session** | Identique à l'original | Nouvel ID de session généré |
| **Historique** | Ajoute à la session originale | Crée une nouvelle branche à partir du point de reprise |
| **Session originale** | Modifiée | Préservée inchangée |
| **Cas d'utilisation** | Continuer la conversation linéaire | Brancher pour explorer les alternatives |

### Exemple : Forker une session

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