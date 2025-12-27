# Gerenciamento de Sessões

Entendendo como o Claude Agent SDK lida com sessões e retomada de sessões

---

# Gerenciamento de Sessões

O Claude Agent SDK fornece recursos de gerenciamento de sessões para lidar com o estado da conversa e retomada. As sessões permitem que você continue conversas em múltiplas interações mantendo contexto completo.

## Como as Sessões Funcionam

Quando você inicia uma nova consulta, o SDK cria automaticamente uma sessão e retorna um ID de sessão na mensagem de sistema inicial. Você pode capturar este ID para retomar a sessão mais tarde.

### Obtendo o ID da Sessão

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

## Retomando Sessões

O SDK suporta retomada de sessões de estados de conversa anteriores, permitindo fluxos de trabalho de desenvolvimento contínuo. Use a opção `resume` com um ID de sessão para continuar uma conversa anterior.

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

O SDK lida automaticamente com o carregamento do histórico de conversa e contexto quando você retoma uma sessão, permitindo que Claude continue exatamente de onde parou.

<Tip>
Para rastrear e reverter alterações de arquivo em sessões, consulte [Ponto de Verificação de Arquivo](/docs/pt-BR/agent-sdk/file-checkpointing).
</Tip>

## Bifurcando Sessões

Ao retomar uma sessão, você pode escolher continuar a sessão original ou bifurcá-la em um novo ramo. Por padrão, retomar continua a sessão original. Use a opção `forkSession` (TypeScript) ou `fork_session` (Python) para criar um novo ID de sessão que comece a partir do estado retomado.

### Quando Bifurcar uma Sessão

Bifurcar é útil quando você quer:
- Explorar diferentes abordagens a partir do mesmo ponto de partida
- Criar múltiplos ramos de conversa sem modificar o original
- Testar alterações sem afetar o histórico de sessão original
- Manter caminhos de conversa separados para diferentes experimentos

### Bifurcar vs Continuar

| Comportamento | `forkSession: false` (padrão) | `forkSession: true` |
|----------|-------------------------------|---------------------|
| **ID da Sessão** | Mesmo do original | Novo ID de sessão gerado |
| **Histórico** | Acrescenta à sessão original | Cria novo ramo a partir do ponto de retomada |
| **Sessão Original** | Modificada | Preservada inalterada |
| **Caso de Uso** | Continuar conversa linear | Bifurcar para explorar alternativas |

### Exemplo: Bifurcando uma Sessão

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