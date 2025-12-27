# 세션 관리

Claude Agent SDK가 세션 및 세션 재개를 처리하는 방식 이해

---

# 세션 관리

Claude Agent SDK는 대화 상태 및 재개를 처리하기 위한 세션 관리 기능을 제공합니다. 세션을 통해 전체 컨텍스트를 유지하면서 여러 상호작용에 걸쳐 대화를 계속할 수 있습니다.

## 세션 작동 방식

새로운 쿼리를 시작하면 SDK는 자동으로 세션을 생성하고 초기 시스템 메시지에서 세션 ID를 반환합니다. 이 ID를 캡처하여 나중에 세션을 재개할 수 있습니다.

### 세션 ID 가져오기

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

## 세션 재개

SDK는 이전 대화 상태에서 세션을 재개하는 것을 지원하여 지속적인 개발 워크플로우를 가능하게 합니다. 세션 ID와 함께 `resume` 옵션을 사용하여 이전 대화를 계속하세요.

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

SDK는 세션을 재개할 때 대화 기록 및 컨텍스트 로드를 자동으로 처리하므로 Claude가 정확히 중단한 지점에서 계속할 수 있습니다.

<Tip>
세션 전체에서 파일 변경 사항을 추적하고 되돌리려면 [파일 체크포인팅](/docs/ko/agent-sdk/file-checkpointing)을 참조하세요.
</Tip>

## 세션 포킹

세션을 재개할 때 원본 세션을 계속하거나 새로운 분기로 포크할 수 있습니다. 기본적으로 세션을 재개하면 원본 세션이 계속됩니다. `forkSession` 옵션(TypeScript) 또는 `fork_session` 옵션(Python)을 사용하여 재개된 상태에서 시작하는 새로운 세션 ID를 생성하세요.

### 세션을 포크해야 할 때

포킹은 다음과 같은 경우에 유용합니다:
- 동일한 시작점에서 다양한 접근 방식을 탐색하려는 경우
- 원본을 수정하지 않고 여러 대화 분기를 생성하려는 경우
- 원본 세션 기록에 영향을 주지 않고 변경 사항을 테스트하려는 경우
- 다양한 실험을 위해 별도의 대화 경로를 유지하려는 경우

### 포킹 vs 계속

| 동작 | `forkSession: false` (기본값) | `forkSession: true` |
|----------|-------------------------------|---------------------|
| **세션 ID** | 원본과 동일 | 새로운 세션 ID 생성 |
| **기록** | 원본 세션에 추가 | 재개 지점에서 새로운 분기 생성 |
| **원본 세션** | 수정됨 | 변경되지 않은 상태로 유지 |
| **사용 사례** | 선형 대화 계속 | 대안 탐색을 위한 분기 |

### 예제: 세션 포킹

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