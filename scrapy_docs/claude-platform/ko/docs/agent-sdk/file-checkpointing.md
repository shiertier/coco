# 체크포인팅으로 파일 변경사항 되돌리기

에이전트 세션 중 파일 변경사항을 추적하고 파일을 이전의 모든 상태로 복원

---

파일 체크포인팅은 에이전트 세션 중 Write, Edit, NotebookEdit 도구를 통해 이루어진 파일 수정사항을 추적하여 파일을 이전의 모든 상태로 되돌릴 수 있게 합니다. 직접 시도해보고 싶으신가요? [대화형 예제](#try-it-out)로 이동하세요.

체크포인팅을 사용하면 다음을 할 수 있습니다:

- **원치 않는 변경사항 취소** - 파일을 알려진 좋은 상태로 복원
- **대안 탐색** - 체크포인트로 복원한 후 다른 접근 방식 시도
- **오류 복구** - 에이전트가 잘못된 수정을 했을 때 복구

<Warning>
Write, Edit, NotebookEdit 도구를 통해 이루어진 변경사항만 추적됩니다. Bash 명령어(예: `echo > file.txt` 또는 `sed -i`)를 통해 이루어진 변경사항은 체크포인트 시스템에서 캡처되지 않습니다.
</Warning>

## 체크포인팅 작동 방식

파일 체크포인팅을 활성화하면 SDK는 Write, Edit 또는 NotebookEdit 도구를 통해 파일을 수정하기 전에 파일의 백업을 생성합니다. 응답 스트림의 사용자 메시지에는 복원 지점으로 사용할 수 있는 체크포인트 UUID가 포함됩니다.

체크포인팅은 에이전트가 파일을 수정하는 데 사용하는 다음의 기본 제공 도구와 함께 작동합니다:

| 도구 | 설명 |
|------|-------------|
| Write | 새 파일을 생성하거나 기존 파일을 새 내용으로 덮어쓰기 |
| Edit | 기존 파일의 특정 부분에 대한 대상 편집 수행 |
| NotebookEdit | Jupyter 노트북(`.ipynb` 파일)의 셀 수정 |

<Note>
파일 되돌리기는 디스크의 파일을 이전 상태로 복원합니다. 대화 자체를 되돌리지는 않습니다. `rewindFiles()`(TypeScript) 또는 `rewind_files()`(Python)를 호출한 후에도 대화 기록과 컨텍스트는 그대로 유지됩니다.
</Note>

체크포인트 시스템은 다음을 추적합니다:

- 세션 중에 생성된 파일
- 세션 중에 수정된 파일
- 수정된 파일의 원본 내용

체크포인트로 되돌리면 생성된 파일은 삭제되고 수정된 파일은 해당 시점의 내용으로 복원됩니다.

## 체크포인팅 구현

파일 체크포인팅을 사용하려면 옵션에서 활성화하고, 응답 스트림에서 체크포인트 UUID를 캡처한 다음, 복원이 필요할 때 `rewindFiles()`(TypeScript) 또는 `rewind_files()`(Python)를 호출합니다.

다음 예제는 완전한 흐름을 보여줍니다: 체크포인팅을 활성화하고, 응답 스트림에서 체크포인트 UUID와 세션 ID를 캡처한 다음, 나중에 세션을 재개하여 파일을 되돌립니다. 각 단계는 아래에서 자세히 설명됩니다.

<CodeGroup>

```python Python
import asyncio
import os
from claude_agent_sdk import ClaudeSDKClient, ClaudeAgentOptions, UserMessage, ResultMessage

async def main():
    # 단계 1: 체크포인팅 활성화
    options = ClaudeAgentOptions(
        enable_file_checkpointing=True,
        permission_mode="acceptEdits",  # 프롬프트 없이 파일 편집 자동 수락
        extra_args={"replay-user-messages": None},  # 응답 스트림에서 체크포인트 UUID를 받기 위해 필수
        env={**os.environ, "CLAUDE_CODE_ENABLE_SDK_FILE_CHECKPOINTING": "1"}
    )

    checkpoint_id = None
    session_id = None

    # 쿼리를 실행하고 체크포인트 UUID와 세션 ID를 캡처
    async with ClaudeSDKClient(options) as client:
        await client.query("Refactor the authentication module")

        # 단계 2: 첫 번째 사용자 메시지에서 체크포인트 UUID 캡처
        async for message in client.receive_response():
            if isinstance(message, UserMessage) and message.uuid and not checkpoint_id:
                checkpoint_id = message.uuid
            if isinstance(message, ResultMessage) and not session_id:
                session_id = message.session_id

    # 단계 3: 나중에 빈 프롬프트로 세션을 재개하여 되돌리기
    if checkpoint_id and session_id:
        async with ClaudeSDKClient(ClaudeAgentOptions(
            enable_file_checkpointing=True,
            resume=session_id
        )) as client:
            await client.query("")  # 연결을 열기 위한 빈 프롬프트
            async for message in client.receive_response():
                await client.rewind_files(checkpoint_id)
                break
        print(f"Rewound to checkpoint: {checkpoint_id}")

asyncio.run(main())
```

```typescript TypeScript
import { query } from "@anthropic-ai/claude-agent-sdk";

async function main() {
  // 단계 1: 체크포인팅 활성화
  const opts = {
    enableFileCheckpointing: true,
    permissionMode: "acceptEdits" as const,  // 프롬프트 없이 파일 편집 자동 수락
    extraArgs: { 'replay-user-messages': null },  // 응답 스트림에서 체크포인트 UUID를 받기 위해 필수
    env: { ...process.env, CLAUDE_CODE_ENABLE_SDK_FILE_CHECKPOINTING: '1' }
  };

  const response = query({
    prompt: "Refactor the authentication module",
    options: opts
  });

  let checkpointId: string | undefined;
  let sessionId: string | undefined;

  // 단계 2: 첫 번째 사용자 메시지에서 체크포인트 UUID 캡처
  for await (const message of response) {
    if (message.type === 'user' && message.uuid && !checkpointId) {
      checkpointId = message.uuid;
    }
    if ('session_id' in message && !sessionId) {
      sessionId = message.session_id;
    }
  }

  // 단계 3: 나중에 빈 프롬프트로 세션을 재개하여 되돌리기
  if (checkpointId && sessionId) {
    const rewindQuery = query({
      prompt: "",  // 연결을 열기 위한 빈 프롬프트
      options: { ...opts, resume: sessionId }
    });

    for await (const msg of rewindQuery) {
      await rewindQuery.rewindFiles(checkpointId);
      break;
    }
    console.log(`Rewound to checkpoint: ${checkpointId}`);
  }
}

main();
```

</CodeGroup>

<Steps>

<Step title="환경 변수 설정">

파일 체크포인팅에는 `CLAUDE_CODE_ENABLE_SDK_FILE_CHECKPOINTING` 환경 변수가 필요합니다. 스크립트를 실행하기 전에 명령줄을 통해 설정하거나 SDK 옵션에서 직접 설정할 수 있습니다.

**옵션 1: 명령줄을 통해 설정**

```bash Python
export CLAUDE_CODE_ENABLE_SDK_FILE_CHECKPOINTING=1
```

**옵션 2: SDK 옵션에서 설정**

SDK를 구성할 때 `env` 옵션을 통해 환경 변수를 전달합니다:

<CodeGroup>

```python Python
import os

options = ClaudeAgentOptions(
    enable_file_checkpointing=True,
    env={**os.environ, "CLAUDE_CODE_ENABLE_SDK_FILE_CHECKPOINTING": "1"}
)
```

```typescript TypeScript
const opts = {
  enableFileCheckpointing: true,
  env: { ...process.env, CLAUDE_CODE_ENABLE_SDK_FILE_CHECKPOINTING: '1' }
};
```

</CodeGroup>

</Step>

<Step title="체크포인팅 활성화">

체크포인팅을 활성화하고 체크포인트 UUID를 받도록 SDK 옵션을 구성합니다:

| 옵션 | Python | TypeScript | 설명 |
|--------|--------|------------|-------------|
| 체크포인팅 활성화 | `enable_file_checkpointing=True` | `enableFileCheckpointing: true` | 되돌리기 위해 파일 변경사항 추적 |
| 체크포인트 UUID 받기 | `extra_args={"replay-user-messages": None}` | `extraArgs: { 'replay-user-messages': null }` | 스트림에서 사용자 메시지 UUID를 받기 위해 필수 |

<CodeGroup>

```python Python
options = ClaudeAgentOptions(
    enable_file_checkpointing=True,
    permission_mode="acceptEdits",
    extra_args={"replay-user-messages": None}
)

async with ClaudeSDKClient(options) as client:
    await client.query("Refactor the authentication module")
```

```typescript TypeScript
const response = query({
  prompt: "Refactor the authentication module",
  options: {
    enableFileCheckpointing: true,
    permissionMode: "acceptEdits" as const,
    extraArgs: { 'replay-user-messages': null }
  }
});
```

</CodeGroup>

</Step>

<Step title="체크포인트 UUID와 세션 ID 캡처">

`replay-user-messages` 옵션이 설정되면(위 참조), 응답 스트림의 각 사용자 메시지에는 체크포인트로 사용되는 UUID가 있습니다.

대부분의 사용 사례에서 첫 번째 사용자 메시지 UUID(`message.uuid`)를 캡처합니다. 이것으로 되돌리면 모든 파일이 원본 상태로 복원됩니다. 여러 체크포인트를 저장하고 중간 상태로 되돌리려면 [여러 복원 지점](#multiple-restore-points)을 참조하세요.

세션 ID(`message.session_id`) 캡처는 선택 사항입니다. 스트림이 완료된 후 나중에 되돌리려는 경우에만 필요합니다. [위험한 작업 전 체크포인팅](#checkpoint-before-risky-operations)의 예제처럼 메시지를 처리하는 동안 즉시 `rewindFiles()`를 호출하는 경우 세션 ID 캡처를 건너뛸 수 있습니다.

<CodeGroup>

```python Python
checkpoint_id = None
session_id = None

async for message in client.receive_response():
    # 각 사용자 메시지에서 체크포인트 업데이트(최신 유지)
    if isinstance(message, UserMessage) and message.uuid:
        checkpoint_id = message.uuid
    # 결과 메시지에서 세션 ID 캡처
    if isinstance(message, ResultMessage):
        session_id = message.session_id
```

```typescript TypeScript
let checkpointId: string | undefined;
let sessionId: string | undefined;

for await (const message of response) {
  // 각 사용자 메시지에서 체크포인트 업데이트(최신 유지)
  if (message.type === 'user' && message.uuid) {
    checkpointId = message.uuid;
  }
  // 세션 ID를 가진 모든 메시지에서 세션 ID 캡처
  if ('session_id' in message) {
    sessionId = message.session_id;
  }
}
```

</CodeGroup>

</Step>

<Step title="파일 되돌리기">

스트림이 완료된 후 되돌리려면 빈 프롬프트로 세션을 재개하고 체크포인트 UUID와 함께 `rewind_files()`(Python) 또는 `rewindFiles()`(TypeScript)를 호출합니다. 스트림 중에도 되돌릴 수 있습니다. [위험한 작업 전 체크포인팅](#checkpoint-before-risky-operations)에서 해당 패턴을 참조하세요.

<CodeGroup>

```python Python
async with ClaudeSDKClient(ClaudeAgentOptions(
    enable_file_checkpointing=True,
    resume=session_id
)) as client:
    await client.query("")  # 연결을 열기 위한 빈 프롬프트
    async for message in client.receive_response():
        await client.rewind_files(checkpoint_id)
        break
```

```typescript TypeScript
const rewindQuery = query({
  prompt: "",  // 연결을 열기 위한 빈 프롬프트
  options: { ...opts, resume: sessionId }
});

for await (const msg of rewindQuery) {
  await rewindQuery.rewindFiles(checkpointId);
  break;
}
```

</CodeGroup>

세션 ID와 체크포인트 ID를 캡처한 경우 CLI에서도 되돌릴 수 있습니다:

```bash
claude --resume <session-id> --rewind-files <checkpoint-uuid>
```

</Step>

</Steps>

## 일반적인 패턴

이 패턴들은 사용 사례에 따라 체크포인트 UUID를 캡처하고 사용하는 다양한 방법을 보여줍니다.

### 위험한 작업 전 체크포인팅

이 패턴은 가장 최근의 체크포인트 UUID만 유지하며, 각 에이전트 턴 전에 업데이트합니다. 처리 중에 문제가 발생하면 즉시 마지막 안전한 상태로 되돌리고 루프를 빠져나올 수 있습니다.

<CodeGroup>

```python Python
import asyncio
import os
from claude_agent_sdk import ClaudeSDKClient, ClaudeAgentOptions, UserMessage

async def main():
    options = ClaudeAgentOptions(
        enable_file_checkpointing=True,
        permission_mode="acceptEdits",
        extra_args={"replay-user-messages": None},
        env={**os.environ, "CLAUDE_CODE_ENABLE_SDK_FILE_CHECKPOINTING": "1"}
    )

    safe_checkpoint = None

    async with ClaudeSDKClient(options) as client:
        await client.query("Refactor the authentication module")

        async for message in client.receive_response():
            # 각 에이전트 턴이 시작되기 전에 체크포인트 업데이트
            # 이전 체크포인트를 덮어씁니다. 최신만 유지
            if isinstance(message, UserMessage) and message.uuid:
                safe_checkpoint = message.uuid

            # 자신의 로직에 따라 되돌릴 시기 결정
            # 예: 오류 감지, 검증 실패 또는 사용자 입력
            if your_revert_condition and safe_checkpoint:
                await client.rewind_files(safe_checkpoint)
                # 되돌린 후 루프 종료, 파일이 복원됨
                break

asyncio.run(main())
```

```typescript TypeScript
import { query } from "@anthropic-ai/claude-agent-sdk";

async function main() {
  const response = query({
    prompt: "Refactor the authentication module",
    options: {
      enableFileCheckpointing: true,
      permissionMode: "acceptEdits" as const,
      extraArgs: { 'replay-user-messages': null },
      env: { ...process.env, CLAUDE_CODE_ENABLE_SDK_FILE_CHECKPOINTING: '1' }
    }
  });

  let safeCheckpoint: string | undefined;

  for await (const message of response) {
    // 각 에이전트 턴이 시작되기 전에 체크포인트 업데이트
    // 이전 체크포인트를 덮어씁니다. 최신만 유지
    if (message.type === 'user' && message.uuid) {
      safeCheckpoint = message.uuid;
    }

    // 자신의 로직에 따라 되돌릴 시기 결정
    // 예: 오류 감지, 검증 실패 또는 사용자 입력
    if (yourRevertCondition && safeCheckpoint) {
      await response.rewindFiles(safeCheckpoint);
      // 되돌린 후 루프 종료, 파일이 복원됨
      break;
    }
  }
}

main();
```

</CodeGroup>

### 여러 복원 지점

Claude가 여러 턴에 걸쳐 변경사항을 만드는 경우 모든 방식으로 되돌리기보다는 특정 지점으로 되돌리고 싶을 수 있습니다. 예를 들어 Claude가 첫 번째 턴에서 파일을 리팩토링하고 두 번째 턴에서 테스트를 추가한 경우 리팩토링은 유지하되 테스트는 취소하고 싶을 수 있습니다.

이 패턴은 메타데이터와 함께 배열에 모든 체크포인트 UUID를 저장합니다. 세션이 완료된 후 이전의 모든 체크포인트로 되돌릴 수 있습니다:

<CodeGroup>

```python Python
import asyncio
import os
from dataclasses import dataclass
from datetime import datetime
from claude_agent_sdk import ClaudeSDKClient, ClaudeAgentOptions, UserMessage, ResultMessage

# 더 나은 추적을 위해 체크포인트 메타데이터 저장
@dataclass
class Checkpoint:
    id: str
    description: str
    timestamp: datetime

async def main():
    options = ClaudeAgentOptions(
        enable_file_checkpointing=True,
        permission_mode="acceptEdits",
        extra_args={"replay-user-messages": None},
        env={**os.environ, "CLAUDE_CODE_ENABLE_SDK_FILE_CHECKPOINTING": "1"}
    )

    checkpoints = []
    session_id = None

    async with ClaudeSDKClient(options) as client:
        await client.query("Refactor the authentication module")

        async for message in client.receive_response():
            if isinstance(message, UserMessage) and message.uuid:
                checkpoints.append(Checkpoint(
                    id=message.uuid,
                    description=f"After turn {len(checkpoints) + 1}",
                    timestamp=datetime.now()
                ))
            if isinstance(message, ResultMessage) and not session_id:
                session_id = message.session_id

    # 나중에: 세션을 재개하여 모든 체크포인트로 되돌리기
    if checkpoints and session_id:
        target = checkpoints[0]  # 모든 체크포인트 선택
        async with ClaudeSDKClient(ClaudeAgentOptions(
            enable_file_checkpointing=True,
            resume=session_id
        )) as client:
            await client.query("")  # 연결을 열기 위한 빈 프롬프트
            async for message in client.receive_response():
                await client.rewind_files(target.id)
                break
        print(f"Rewound to: {target.description}")

asyncio.run(main())
```

```typescript TypeScript
import { query } from "@anthropic-ai/claude-agent-sdk";

// 더 나은 추적을 위해 체크포인트 메타데이터 저장
interface Checkpoint {
  id: string;
  description: string;
  timestamp: Date;
}

async function main() {
  const opts = {
    enableFileCheckpointing: true,
    permissionMode: "acceptEdits" as const,
    extraArgs: { 'replay-user-messages': null },
    env: { ...process.env, CLAUDE_CODE_ENABLE_SDK_FILE_CHECKPOINTING: '1' }
  };

  const response = query({
    prompt: "Refactor the authentication module",
    options: opts
  });

  const checkpoints: Checkpoint[] = [];
  let sessionId: string | undefined;

  for await (const message of response) {
    if (message.type === 'user' && message.uuid) {
      checkpoints.push({
        id: message.uuid,
        description: `After turn ${checkpoints.length + 1}`,
        timestamp: new Date()
      });
    }
    if ('session_id' in message && !sessionId) {
      sessionId = message.session_id;
    }
  }

  // 나중에: 세션을 재개하여 모든 체크포인트로 되돌리기
  if (checkpoints.length > 0 && sessionId) {
    const target = checkpoints[0];  // 모든 체크포인트 선택
    const rewindQuery = query({
      prompt: "",  // 연결을 열기 위한 빈 프롬프트
      options: { ...opts, resume: sessionId }
    });

    for await (const msg of rewindQuery) {
      await rewindQuery.rewindFiles(target.id);
      break;
    }
    console.log(`Rewound to: ${target.description}`);
  }
}

main();
```

</CodeGroup>

## 직접 시도해보기

이 완전한 예제는 작은 유틸리티 파일을 생성하고, 에이전트가 문서 주석을 추가하도록 하고, 변경사항을 표시한 다음, 되돌릴지 여부를 묻습니다.

시작하기 전에 [Claude Agent SDK가 설치](/docs/ko/agent-sdk/quickstart)되어 있는지 확인하세요.

<Steps>

<Step title="테스트 파일 생성">

`utils.py`(Python) 또는 `utils.ts`(TypeScript)라는 새 파일을 생성하고 다음 코드를 붙여넣습니다:

<CodeGroup>

```python utils.py
def add(a, b):
    return a + b

def subtract(a, b):
    return a - b

def multiply(a, b):
    return a * b

def divide(a, b):
    if b == 0:
        raise ValueError("Cannot divide by zero")
    return a / b
```

```typescript utils.ts
export function add(a: number, b: number): number {
  return a + b;
}

export function subtract(a: number, b: number): number {
  return a - b;
}

export function multiply(a: number, b: number): number {
  return a * b;
}

export function divide(a: number, b: number): number {
  if (b === 0) {
    throw new Error("Cannot divide by zero");
  }
  return a / b;
}
```

</CodeGroup>

</Step>

<Step title="대화형 예제 실행">

유틸리티 파일과 같은 디렉토리에 `try_checkpointing.py`(Python) 또는 `try_checkpointing.ts`(TypeScript)라는 새 파일을 생성하고 다음 코드를 붙여넣습니다.

이 스크립트는 Claude에게 유틸리티 파일에 문서 주석을 추가하도록 요청한 다음 원본으로 되돌릴 수 있는 옵션을 제공합니다.

<CodeGroup>

```python try_checkpointing.py
import asyncio
from claude_agent_sdk import ClaudeSDKClient, ClaudeAgentOptions, UserMessage, ResultMessage

async def main():
    # 체크포인팅이 활성화된 SDK 구성
    # - enable_file_checkpointing: 되돌리기 위해 파일 변경사항 추적
    # - permission_mode: 프롬프트 없이 파일 편집 자동 수락
    # - extra_args: 스트림에서 사용자 메시지 UUID를 받기 위해 필수
    options = ClaudeAgentOptions(
        enable_file_checkpointing=True,
        permission_mode="acceptEdits",
        extra_args={"replay-user-messages": None}
    )

    checkpoint_id = None  # 되돌리기 위해 사용자 메시지 UUID 저장
    session_id = None     # 나중에 재개하기 위해 세션 ID 저장

    print("Running agent to add doc comments to utils.py...\n")

    # 에이전트를 실행하고 응답 스트림에서 체크포인트 데이터 캡처
    async with ClaudeSDKClient(options) as client:
        await client.query("Add doc comments to utils.py")

        async for message in client.receive_response():
            # 첫 번째 사용자 메시지 UUID 캡처 - 이것이 복원 지점
            if isinstance(message, UserMessage) and message.uuid and not checkpoint_id:
                checkpoint_id = message.uuid
            # 나중에 재개할 수 있도록 세션 ID 캡처
            if isinstance(message, ResultMessage):
                session_id = message.session_id

    print("Done! Open utils.py to see the added doc comments.\n")

    # 사용자에게 변경사항을 되돌릴지 여부 묻기
    if checkpoint_id and session_id:
        response = input("Rewind to remove the doc comments? (y/n): ")

        if response.lower() == "y":
            # 빈 프롬프트로 세션을 재개한 다음 되돌리기
            async with ClaudeSDKClient(ClaudeAgentOptions(
                enable_file_checkpointing=True,
                resume=session_id
            )) as client:
                await client.query("")  # 빈 프롬프트가 연결을 열기
                async for message in client.receive_response():
                    await client.rewind_files(checkpoint_id)  # 파일 복원
                    break

            print("\n✓ File restored! Open utils.py to verify the doc comments are gone.")
        else:
            print("\nKept the modified file.")

asyncio.run(main())
```

```typescript try_checkpointing.ts
import { query } from "@anthropic-ai/claude-agent-sdk";
import * as readline from "readline";

async function main() {
  // 체크포인팅이 활성화된 SDK 구성
  // - enableFileCheckpointing: 되돌리기 위해 파일 변경사항 추적
  // - permissionMode: 프롬프트 없이 파일 편집 자동 수락
  // - extraArgs: 스트림에서 사용자 메시지 UUID를 받기 위해 필수
  const opts = {
    enableFileCheckpointing: true,
    permissionMode: "acceptEdits" as const,
    extraArgs: { 'replay-user-messages': null }
  };

  let sessionId: string | undefined;    // 나중에 재개하기 위해 세션 ID 저장
  let checkpointId: string | undefined; // 되돌리기 위해 사용자 메시지 UUID 저장

  console.log("Running agent to add doc comments to utils.ts...\n");

  // 에이전트를 실행하고 응답 스트림에서 체크포인트 데이터 캡처
  const response = query({
    prompt: "Add doc comments to utils.ts",
    options: opts
  });

  for await (const message of response) {
    // 첫 번째 사용자 메시지 UUID 캡처 - 이것이 복원 지점
    if (message.type === "user" && message.uuid && !checkpointId) {
      checkpointId = message.uuid;
    }
    // 나중에 재개할 수 있도록 세션 ID 캡처
    if ("session_id" in message) {
      sessionId = message.session_id;
    }
  }

  console.log("Done! Open utils.ts to see the added doc comments.\n");

  // 사용자에게 변경사항을 되돌릴지 여부 묻기
  if (checkpointId && sessionId) {
    const rl = readline.createInterface({
      input: process.stdin,
      output: process.stdout
    });

    const answer = await new Promise<string>((resolve) => {
      rl.question("Rewind to remove the doc comments? (y/n): ", resolve);
    });
    rl.close();

    if (answer.toLowerCase() === "y") {
      // 빈 프롬프트로 세션을 재개한 다음 되돌리기
      const rewindQuery = query({
        prompt: "",  // 빈 프롬프트가 연결을 열기
        options: { ...opts, resume: sessionId }
      });

      for await (const msg of rewindQuery) {
        await rewindQuery.rewindFiles(checkpointId);  // 파일 복원
        break;
      }

      console.log("\n✓ File restored! Open utils.ts to verify the doc comments are gone.");
    } else {
      console.log("\nKept the modified file.");
    }
  }
}

main();
```

</CodeGroup>

이 예제는 완전한 체크포인팅 워크플로우를 보여줍니다:

1. **체크포인팅 활성화**: `enable_file_checkpointing=True`와 `permission_mode="acceptEdits"`로 SDK를 구성하여 파일 편집 자동 승인
2. **체크포인트 데이터 캡처**: 에이전트가 실행되는 동안 첫 번째 사용자 메시지 UUID(복원 지점)와 세션 ID 저장
3. **되돌리기 프롬프트**: 에이전트가 완료된 후 유틸리티 파일을 확인하여 문서 주석을 보고 변경사항을 취소할지 결정
4. **재개 및 되돌리기**: 예인 경우 빈 프롬프트로 세션을 재개하고 `rewind_files()`를 호출하여 원본 파일 복원

</Step>

<Step title="예제 실행">

유틸리티 파일과 같은 디렉토리에서 환경 변수를 설정하고 스크립트를 실행합니다.

<Tip>
스크립트를 실행하기 전에 유틸리티 파일(`utils.py` 또는 `utils.ts`)을 IDE 또는 편집기에서 엽니다. 에이전트가 문서 주석을 추가할 때 파일이 실시간으로 업데이트되고, 되돌리기를 선택하면 원본으로 되돌아갑니다.
</Tip>

<CodeGroup>

```bash Python
export CLAUDE_CODE_ENABLE_SDK_FILE_CHECKPOINTING=1
python try_checkpointing.py
```

```bash TypeScript
export CLAUDE_CODE_ENABLE_SDK_FILE_CHECKPOINTING=1
npx tsx try_checkpointing.ts
```

</CodeGroup>

에이전트가 문서 주석을 추가한 다음 되돌릴지 여부를 묻는 프롬프트가 표시됩니다. 예를 선택하면 파일이 원본 상태로 복원됩니다.

</Step>

</Steps>

## 제한사항

파일 체크포인팅에는 다음과 같은 제한사항이 있습니다:

| 제한사항 | 설명 |
|------------|-------------|
| Write/Edit/NotebookEdit 도구만 | Bash 명령어를 통해 이루어진 변경사항은 추적되지 않음 |
| 동일 세션 | 체크포인트는 생성한 세션에 연결됨 |
| 파일 내용만 | 디렉토리 생성, 이동 또는 삭제는 되돌리기로 취소되지 않음 |
| 로컬 파일 | 원격 또는 네트워크 파일은 추적되지 않음 |

## 문제 해결

### 체크포인팅 옵션이 인식되지 않음

`enableFileCheckpointing` 또는 `rewindFiles()`를 사용할 수 없으면 이전 SDK 버전을 사용 중일 수 있습니다.

**해결책**: 최신 SDK 버전으로 업데이트합니다:
- **Python**: `pip install --upgrade claude-agent-sdk`
- **TypeScript**: `npm install @anthropic-ai/claude-agent-sdk@latest`

### 사용자 메시지에 UUID가 없음

`message.uuid`가 `undefined`이거나 누락된 경우 체크포인트 UUID를 받지 못하고 있습니다.

**원인**: `replay-user-messages` 옵션이 설정되지 않았습니다.

**해결책**: 옵션에 `extra_args={"replay-user-messages": None}`(Python) 또는 `extraArgs: { 'replay-user-messages': null }`(TypeScript)를 추가합니다.

### "No file checkpoint found for message" 오류

이 오류는 지정된 사용자 메시지 UUID에 대한 체크포인트 데이터가 없을 때 발생합니다.

**일반적인 원인**:
- `CLAUDE_CODE_ENABLE_SDK_FILE_CHECKPOINTING` 환경 변수가 설정되지 않음
- 재개 및 되돌리기를 시도하기 전에 세션이 제대로 완료되지 않음

**해결책**: 환경 변수가 설정되었는지 확인하고([환경 변수 설정](#set-the-environment-variable) 참조), 예제에 표시된 패턴을 사용합니다: 첫 번째 사용자 메시지 UUID를 캡처하고, 세션을 완전히 완료한 다음, 빈 프롬프트로 재개하고 `rewindFiles()`를 한 번 호출합니다.

### "ProcessTransport is not ready for writing" 오류

이 오류는 응답을 반복 처리한 후 `rewindFiles()` 또는 `rewind_files()`를 호출할 때 발생합니다. 루프가 완료되면 CLI 프로세스에 대한 연결이 닫힙니다.

**해결책**: 빈 프롬프트로 세션을 재개한 다음 새 쿼리에서 되돌립니다:

<CodeGroup>

```python Python
# 빈 프롬프트로 세션을 재개한 다음 되돌리기
async with ClaudeSDKClient(ClaudeAgentOptions(
    enable_file_checkpointing=True,
    resume=session_id
)) as client:
    await client.query("")
    async for message in client.receive_response():
        await client.rewind_files(checkpoint_id)
        break
```

```typescript TypeScript
// 빈 프롬프트로 세션을 재개한 다음 되돌리기
const rewindQuery = query({
  prompt: "",
  options: { ...opts, resume: sessionId }
});

for await (const msg of rewindQuery) {
  await rewindQuery.rewindFiles(checkpointId);
  break;
}
```

</CodeGroup>

## 다음 단계

- **[세션](/docs/ko/agent-sdk/sessions)**: 세션을 재개하는 방법을 알아봅니다. 스트림이 완료된 후 되돌리기에 필요합니다. 세션 ID, 대화 재개 및 세션 포킹을 다룹니다.
- **[권한](/docs/ko/agent-sdk/permissions)**: Claude가 사용할 수 있는 도구와 파일 수정 승인 방식을 구성합니다. 편집이 발생하는 시기를 더 많이 제어하려는 경우 유용합니다.
- **[TypeScript SDK 참조](/docs/ko/agent-sdk/typescript)**: `query()`의 모든 옵션과 `rewindFiles()` 메서드를 포함한 완전한 API 참조입니다.
- **[Python SDK 참조](/docs/ko/agent-sdk/python)**: `ClaudeAgentOptions`의 모든 옵션과 `rewind_files()` 메서드를 포함한 완전한 API 참조입니다.