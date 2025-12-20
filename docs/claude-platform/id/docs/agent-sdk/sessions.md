# Manajemen Sesi

Memahami bagaimana Claude Agent SDK menangani sesi dan pemulihan sesi

---

# Manajemen Sesi

Claude Agent SDK menyediakan kemampuan manajemen sesi untuk menangani status percakapan dan pemulihan. Sesi memungkinkan Anda untuk melanjutkan percakapan di berbagai interaksi sambil mempertahankan konteks penuh.

## Cara Kerja Sesi

Ketika Anda memulai kueri baru, SDK secara otomatis membuat sesi dan mengembalikan ID sesi dalam pesan sistem awal. Anda dapat menangkap ID ini untuk melanjutkan sesi nanti.

### Mendapatkan ID Sesi

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

## Melanjutkan Sesi

SDK mendukung pemulihan sesi dari status percakapan sebelumnya, memungkinkan alur kerja pengembangan berkelanjutan. Gunakan opsi `resume` dengan ID sesi untuk melanjutkan percakapan sebelumnya.

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

SDK secara otomatis menangani pemuatan riwayat percakapan dan konteks ketika Anda melanjutkan sesi, memungkinkan Claude untuk melanjutkan tepat di mana ia berhenti.

<Tip>
Untuk melacak dan mengembalikan perubahan file di seluruh sesi, lihat [File Checkpointing](/docs/id/agent-sdk/file-checkpointing).
</Tip>

## Memisahkan Sesi

Ketika melanjutkan sesi, Anda dapat memilih untuk melanjutkan sesi asli atau memisahkannya menjadi cabang baru. Secara default, melanjutkan akan terus dengan sesi asli. Gunakan opsi `forkSession` (TypeScript) atau `fork_session` (Python) untuk membuat ID sesi baru yang dimulai dari status yang dilanjutkan.

### Kapan Memisahkan Sesi

Pemisahan berguna ketika Anda ingin:
- Menjelajahi pendekatan berbeda dari titik awal yang sama
- Membuat beberapa cabang percakapan tanpa mengubah yang asli
- Menguji perubahan tanpa mempengaruhi riwayat sesi asli
- Mempertahankan jalur percakapan terpisah untuk eksperimen berbeda

### Memisahkan vs Melanjutkan

| Perilaku | `forkSession: false` (default) | `forkSession: true` |
|----------|-------------------------------|---------------------|
| **ID Sesi** | Sama dengan asli | ID sesi baru dihasilkan |
| **Riwayat** | Menambahkan ke sesi asli | Membuat cabang baru dari titik lanjutan |
| **Sesi Asli** | Dimodifikasi | Dipertahankan tidak berubah |
| **Kasus Penggunaan** | Lanjutkan percakapan linear | Cabang untuk menjelajahi alternatif |

### Contoh: Memisahkan Sesi

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