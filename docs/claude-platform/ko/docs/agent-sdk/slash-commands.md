# SDK의 슬래시 명령어

SDK를 통해 Claude Code 세션을 제어하기 위한 슬래시 명령어 사용법을 알아보세요

---

슬래시 명령어는 `/`로 시작하는 특별한 명령어로 Claude Code 세션을 제어하는 방법을 제공합니다. 이러한 명령어는 SDK를 통해 전송되어 대화 기록 지우기, 메시지 압축, 도움말 가져오기 등의 작업을 수행할 수 있습니다.

## 사용 가능한 슬래시 명령어 찾기

Claude Agent SDK는 시스템 초기화 메시지에서 사용 가능한 슬래시 명령어에 대한 정보를 제공합니다. 세션이 시작될 때 이 정보에 액세스하세요:

<CodeGroup>

```typescript TypeScript
import { query } from "@anthropic-ai/claude-agent-sdk";

for await (const message of query({
  prompt: "Hello Claude",
  options: { maxTurns: 1 }
})) {
  if (message.type === "system" && message.subtype === "init") {
    console.log("Available slash commands:", message.slash_commands);
    // 예시 출력: ["/compact", "/clear", "/help"]
  }
}
```

```python Python
import asyncio
from claude_agent_sdk import query

async def main():
    async for message in query(
        prompt="Hello Claude",
        options={"max_turns": 1}
    ):
        if message.type == "system" and message.subtype == "init":
            print("Available slash commands:", message.slash_commands)
            # 예시 출력: ["/compact", "/clear", "/help"]

asyncio.run(main())
```

</CodeGroup>

## 슬래시 명령어 전송

일반 텍스트와 마찬가지로 프롬프트 문자열에 슬래시 명령어를 포함하여 전송하세요:

<CodeGroup>

```typescript TypeScript
import { query } from "@anthropic-ai/claude-agent-sdk";

// 슬래시 명령어 전송
for await (const message of query({
  prompt: "/compact",
  options: { maxTurns: 1 }
})) {
  if (message.type === "result") {
    console.log("Command executed:", message.result);
  }
}
```

```python Python
import asyncio
from claude_agent_sdk import query

async def main():
    # 슬래시 명령어 전송
    async for message in query(
        prompt="/compact",
        options={"max_turns": 1}
    ):
        if message.type == "result":
            print("Command executed:", message.result)

asyncio.run(main())
```

</CodeGroup>

## 일반적인 슬래시 명령어

### `/compact` - 대화 기록 압축

`/compact` 명령어는 중요한 컨텍스트를 보존하면서 오래된 메시지를 요약하여 대화 기록의 크기를 줄입니다:

<CodeGroup>

```typescript TypeScript
import { query } from "@anthropic-ai/claude-agent-sdk";

for await (const message of query({
  prompt: "/compact",
  options: { maxTurns: 1 }
})) {
  if (message.type === "system" && message.subtype === "compact_boundary") {
    console.log("Compaction completed");
    console.log("Pre-compaction tokens:", message.compact_metadata.pre_tokens);
    console.log("Trigger:", message.compact_metadata.trigger);
  }
}
```

```python Python
import asyncio
from claude_agent_sdk import query

async def main():
    async for message in query(
        prompt="/compact",
        options={"max_turns": 1}
    ):
        if (message.type == "system" and 
            message.subtype == "compact_boundary"):
            print("Compaction completed")
            print("Pre-compaction tokens:", 
                  message.compact_metadata.pre_tokens)
            print("Trigger:", message.compact_metadata.trigger)

asyncio.run(main())
```

</CodeGroup>

### `/clear` - 대화 지우기

`/clear` 명령어는 모든 이전 기록을 지워서 새로운 대화를 시작합니다:

<CodeGroup>

```typescript TypeScript
import { query } from "@anthropic-ai/claude-agent-sdk";

// 대화 지우기 및 새로 시작
for await (const message of query({
  prompt: "/clear",
  options: { maxTurns: 1 }
})) {
  if (message.type === "system" && message.subtype === "init") {
    console.log("Conversation cleared, new session started");
    console.log("Session ID:", message.session_id);
  }
}
```

```python Python
import asyncio
from claude_agent_sdk import query

async def main():
    # 대화 지우기 및 새로 시작
    async for message in query(
        prompt="/clear",
        options={"max_turns": 1}
    ):
        if message.type == "system" and message.subtype == "init":
            print("Conversation cleared, new session started")
            print("Session ID:", message.session_id)

asyncio.run(main())
```

</CodeGroup>

## 사용자 정의 슬래시 명령어 생성

내장 슬래시 명령어를 사용하는 것 외에도 SDK를 통해 사용할 수 있는 자신만의 사용자 정의 명령어를 생성할 수 있습니다. 사용자 정의 명령어는 서브에이전트가 구성되는 방식과 유사하게 특정 디렉토리의 마크다운 파일로 정의됩니다.

### 파일 위치

사용자 정의 슬래시 명령어는 범위에 따라 지정된 디렉토리에 저장됩니다:

- **프로젝트 명령어**: `.claude/commands/` - 현재 프로젝트에서만 사용 가능
- **개인 명령어**: `~/.claude/commands/` - 모든 프로젝트에서 사용 가능

### 파일 형식

각 사용자 정의 명령어는 마크다운 파일입니다:
- 파일명(`.md` 확장자 제외)이 명령어 이름이 됩니다
- 파일 내용이 명령어가 수행하는 작업을 정의합니다
- 선택적 YAML 프론트매터가 구성을 제공합니다

#### 기본 예시

`.claude/commands/refactor.md` 생성:

```markdown
선택된 코드를 리팩토링하여 가독성과 유지보수성을 개선합니다.
클린 코드 원칙과 모범 사례에 중점을 둡니다.
```

이렇게 하면 SDK를 통해 사용할 수 있는 `/refactor` 명령어가 생성됩니다.

#### 프론트매터 포함

`.claude/commands/security-check.md` 생성:

```markdown
---
allowed-tools: Read, Grep, Glob
description: Run security vulnerability scan
model: claude-3-5-sonnet-20241022
---

다음을 포함한 보안 취약점에 대해 코드베이스를 분석합니다:
- SQL 인젝션 위험
- XSS 취약점
- 노출된 자격 증명
- 안전하지 않은 구성
```

### SDK에서 사용자 정의 명령어 사용

파일 시스템에 정의되면 사용자 정의 명령어는 SDK를 통해 자동으로 사용할 수 있습니다:

<CodeGroup>

```typescript TypeScript
import { query } from "@anthropic-ai/claude-agent-sdk";

// 사용자 정의 명령어 사용
for await (const message of query({
  prompt: "/refactor src/auth/login.ts",
  options: { maxTurns: 3 }
})) {
  if (message.type === "assistant") {
    console.log("Refactoring suggestions:", message.message);
  }
}

// 사용자 정의 명령어가 slash_commands 목록에 나타남
for await (const message of query({
  prompt: "Hello",
  options: { maxTurns: 1 }
})) {
  if (message.type === "system" && message.subtype === "init") {
    // 내장 명령어와 사용자 정의 명령어 모두 포함
    console.log("Available commands:", message.slash_commands);
    // 예시: ["/compact", "/clear", "/help", "/refactor", "/security-check"]
  }
}
```

```python Python
import asyncio
from claude_agent_sdk import query

async def main():
    # 사용자 정의 명령어 사용
    async for message in query(
        prompt="/refactor src/auth/login.py",
        options={"max_turns": 3}
    ):
        if message.type == "assistant":
            print("Refactoring suggestions:", message.message)
    
    # 사용자 정의 명령어가 slash_commands 목록에 나타남
    async for message in query(
        prompt="Hello",
        options={"max_turns": 1}
    ):
        if message.type == "system" and message.subtype == "init":
            # 내장 명령어와 사용자 정의 명령어 모두 포함
            print("Available commands:", message.slash_commands)
            # 예시: ["/compact", "/clear", "/help", "/refactor", "/security-check"]

asyncio.run(main())
```

</CodeGroup>

### 고급 기능

#### 인수 및 플레이스홀더

사용자 정의 명령어는 플레이스홀더를 사용하여 동적 인수를 지원합니다:

`.claude/commands/fix-issue.md` 생성:

```markdown
---
argument-hint: [issue-number] [priority]
description: Fix a GitHub issue
---

우선순위 $2로 이슈 #$1을 수정합니다.
이슈 설명을 확인하고 필요한 변경사항을 구현합니다.
```

SDK에서 사용:

<CodeGroup>

```typescript TypeScript
import { query } from "@anthropic-ai/claude-agent-sdk";

// 사용자 정의 명령어에 인수 전달
for await (const message of query({
  prompt: "/fix-issue 123 high",
  options: { maxTurns: 5 }
})) {
  // 명령어는 $1="123" 및 $2="high"로 처리됩니다
  if (message.type === "result") {
    console.log("Issue fixed:", message.result);
  }
}
```

```python Python
import asyncio
from claude_agent_sdk import query

async def main():
    # 사용자 정의 명령어에 인수 전달
    async for message in query(
        prompt="/fix-issue 123 high",
        options={"max_turns": 5}
    ):
        # 명령어는 $1="123" 및 $2="high"로 처리됩니다
        if message.type == "result":
            print("Issue fixed:", message.result)

asyncio.run(main())
```

</CodeGroup>

#### Bash 명령어 실행

사용자 정의 명령어는 bash 명령어를 실행하고 출력을 포함할 수 있습니다:

`.claude/commands/git-commit.md` 생성:

```markdown
---
allowed-tools: Bash(git add:*), Bash(git status:*), Bash(git commit:*)
description: Create a git commit
---

## 컨텍스트

- 현재 상태: !`git status`
- 현재 차이점: !`git diff HEAD`

## 작업

변경사항을 기반으로 적절한 메시지로 git 커밋을 생성합니다.
```

#### 파일 참조

`@` 접두사를 사용하여 파일 내용을 포함합니다:

`.claude/commands/review-config.md` 생성:

```markdown
---
description: Review configuration files
---

다음 구성 파일의 문제점을 검토합니다:
- 패키지 구성: @package.json
- TypeScript 구성: @tsconfig.json
- 환경 구성: @.env

보안 문제, 오래된 종속성, 잘못된 구성을 확인합니다.
```

### 네임스페이싱을 통한 구성

더 나은 구조를 위해 하위 디렉토리에서 명령어를 구성합니다:

```bash
.claude/commands/
├── frontend/
│   ├── component.md      # /component (project:frontend) 생성
│   └── style-check.md     # /style-check (project:frontend) 생성
├── backend/
│   ├── api-test.md        # /api-test (project:backend) 생성
│   └── db-migrate.md      # /db-migrate (project:backend) 생성
└── review.md              # /review (project) 생성
```

하위 디렉토리는 명령어 설명에 나타나지만 명령어 이름 자체에는 영향을 주지 않습니다.

### 실용적인 예시

#### 코드 리뷰 명령어

`.claude/commands/code-review.md` 생성:

```markdown
---
allowed-tools: Read, Grep, Glob, Bash(git diff:*)
description: Comprehensive code review
---

## 변경된 파일
!`git diff --name-only HEAD~1`

## 상세 변경사항
!`git diff HEAD~1`

## 리뷰 체크리스트

위의 변경사항을 다음 항목에 대해 검토합니다:
1. 코드 품질 및 가독성
2. 보안 취약점
3. 성능 영향
4. 테스트 커버리지
5. 문서 완성도

우선순위별로 구성된 구체적이고 실행 가능한 피드백을 제공합니다.
```

#### 테스트 실행 명령어

`.claude/commands/test.md` 생성:

```markdown
---
allowed-tools: Bash, Read, Edit
argument-hint: [test-pattern]
description: Run tests with optional pattern
---

패턴과 일치하는 테스트 실행: $ARGUMENTS

1. 테스트 프레임워크 감지 (Jest, pytest 등)
2. 제공된 패턴으로 테스트 실행
3. 테스트가 실패하면 분석하고 수정
4. 수정 사항을 확인하기 위해 재실행
```

SDK를 통해 이러한 명령어를 사용합니다:

<CodeGroup>

```typescript TypeScript
import { query } from "@anthropic-ai/claude-agent-sdk";

// 코드 리뷰 실행
for await (const message of query({
  prompt: "/code-review",
  options: { maxTurns: 3 }
})) {
  // 리뷰 피드백 처리
}

// 특정 테스트 실행
for await (const message of query({
  prompt: "/test auth",
  options: { maxTurns: 5 }
})) {
  // 테스트 결과 처리
}
```

```python Python
import asyncio
from claude_agent_sdk import query

async def main():
    # 코드 리뷰 실행
    async for message in query(
        prompt="/code-review",
        options={"max_turns": 3}
    ):
        # 리뷰 피드백 처리
        pass
    
    # 특정 테스트 실행
    async for message in query(
        prompt="/test auth",
        options={"max_turns": 5}
    ):
        # 테스트 결과 처리
        pass

asyncio.run(main())
```

</CodeGroup>

## 참고 자료

- [슬래시 명령어](https://code.claude.com/docs/slash-commands) - 완전한 슬래시 명령어 문서
- [SDK의 서브에이전트](/docs/ko/agent-sdk/subagents) - 서브에이전트를 위한 유사한 파일 시스템 기반 구성
- [TypeScript SDK 참조](https://code.claude.com/docs/typescript-sdk-reference) - 완전한 API 문서
- [SDK 개요](/docs/ko/agent-sdk/overview) - 일반적인 SDK 개념
- [CLI 참조](https://code.claude.com/docs/cli-reference) - 명령줄 인터페이스