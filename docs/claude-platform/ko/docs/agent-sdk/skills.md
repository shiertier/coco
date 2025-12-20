# SDK의 에이전트 스킬

Claude Agent SDK에서 전문화된 기능을 사용하여 Claude를 확장하세요

---

## 개요

에이전트 스킬은 Claude를 전문화된 기능으로 확장하며, Claude가 관련성이 있을 때 자율적으로 호출합니다. 스킬은 지침, 설명 및 선택적 지원 리소스를 포함하는 `SKILL.md` 파일로 패키징됩니다.

스킬의 이점, 아키텍처 및 작성 지침을 포함한 포괄적인 정보는 [에이전트 스킬 개요](/docs/ko/agents-and-tools/agent-skills/overview)를 참조하세요.

## SDK에서 스킬이 작동하는 방식

Claude Agent SDK를 사용할 때 스킬은:

1. **파일시스템 아티팩트로 정의됨**: 특정 디렉토리(`.claude/skills/`)에 `SKILL.md` 파일로 생성됨
2. **파일시스템에서 로드됨**: 스킬은 구성된 파일시스템 위치에서 로드됩니다. 파일시스템에서 스킬을 로드하려면 `settingSources`(TypeScript) 또는 `setting_sources`(Python)를 지정해야 합니다
3. **자동으로 발견됨**: 파일시스템 설정이 로드되면 스킬 메타데이터는 시작 시 사용자 및 프로젝트 디렉토리에서 발견되며, 트리거될 때 전체 콘텐츠가 로드됩니다
4. **모델 호출됨**: Claude는 컨텍스트를 기반으로 사용할 시기를 자율적으로 선택합니다
5. **allowed_tools를 통해 활성화됨**: `allowed_tools`에 `"Skill"`을 추가하여 스킬을 활성화합니다

서브에이전트(프로그래밍 방식으로 정의할 수 있음)와 달리 스킬은 파일시스템 아티팩트로 생성되어야 합니다. SDK는 스킬을 등록하기 위한 프로그래밍 API를 제공하지 않습니다.

<Note>
**기본 동작**: 기본적으로 SDK는 파일시스템 설정을 로드하지 않습니다. 스킬을 사용하려면 옵션에서 `settingSources: ['user', 'project']`(TypeScript) 또는 `setting_sources=["user", "project"]`(Python)을 명시적으로 구성해야 합니다.
</Note>

## SDK에서 스킬 사용

SDK에서 스킬을 사용하려면 다음을 수행해야 합니다:

1. `allowed_tools` 구성에 `"Skill"`을 포함합니다
2. 파일시스템에서 스킬을 로드하도록 `settingSources`/`setting_sources`를 구성합니다

구성되면 Claude는 지정된 디렉토리에서 스킬을 자동으로 발견하고 사용자의 요청과 관련이 있을 때 호출합니다.

<CodeGroup>

```python Python
import asyncio
from claude_agent_sdk import query, ClaudeAgentOptions

async def main():
    options = ClaudeAgentOptions(
        cwd="/path/to/project",  # Project with .claude/skills/
        setting_sources=["user", "project"],  # Load Skills from filesystem
        allowed_tools=["Skill", "Read", "Write", "Bash"]  # Enable Skill tool
    )

    async for message in query(
        prompt="Help me process this PDF document",
        options=options
    ):
        print(message)

asyncio.run(main())
```

```typescript TypeScript
import { query } from "@anthropic-ai/claude-agent-sdk";

for await (const message of query({
  prompt: "Help me process this PDF document",
  options: {
    cwd: "/path/to/project",  // Project with .claude/skills/
    settingSources: ["user", "project"],  // Load Skills from filesystem
    allowedTools: ["Skill", "Read", "Write", "Bash"]  // Enable Skill tool
  }
})) {
  console.log(message);
}
```

</CodeGroup>

## 스킬 위치

스킬은 `settingSources`/`setting_sources` 구성을 기반으로 파일시스템 디렉토리에서 로드됩니다:

- **프로젝트 스킬** (`.claude/skills/`): git을 통해 팀과 공유됨 - `setting_sources`에 `"project"`가 포함될 때 로드됨
- **사용자 스킬** (`~/.claude/skills/`): 모든 프로젝트의 개인 스킬 - `setting_sources`에 `"user"`가 포함될 때 로드됨
- **플러그인 스킬**: 설치된 Claude Code 플러그인과 함께 번들됨

## 스킬 생성

스킬은 YAML 프론트매터와 마크다운 콘텐츠가 포함된 `SKILL.md` 파일을 포함하는 디렉토리로 정의됩니다. `description` 필드는 Claude가 스킬을 호출하는 시기를 결정합니다.

**예제 디렉토리 구조**:
```bash
.claude/skills/processing-pdfs/
└── SKILL.md
```

스킬 생성에 대한 완전한 지침(SKILL.md 구조, 다중 파일 스킬 및 예제 포함)은 다음을 참조하세요:
- [Claude Code의 에이전트 스킬](https://code.claude.com/docs/skills): 예제가 포함된 완전한 가이드
- [에이전트 스킬 모범 사례](/docs/ko/agents-and-tools/agent-skills/best-practices): 작성 지침 및 명명 규칙

## 도구 제한

<Note>
SKILL.md의 `allowed-tools` 프론트매터 필드는 Claude Code CLI를 직접 사용할 때만 지원됩니다. **SDK를 통해 스킬을 사용할 때는 적용되지 않습니다**.

SDK를 사용할 때는 쿼리 구성의 주 `allowedTools` 옵션을 통해 도구 액세스를 제어합니다.
</Note>

SDK 애플리케이션에서 스킬에 대한 도구를 제한하려면 `allowedTools` 옵션을 사용합니다:

<Note>
첫 번째 예제의 import 문이 다음 코드 스니펫에서 가정됩니다.
</Note>

<CodeGroup>

```python Python
options = ClaudeAgentOptions(
    setting_sources=["user", "project"],  # Load Skills from filesystem
    allowed_tools=["Skill", "Read", "Grep", "Glob"]  # Restricted toolset
)

async for message in query(
    prompt="Analyze the codebase structure",
    options=options
):
    print(message)
```

```typescript TypeScript
// Skills can only use Read, Grep, and Glob tools
for await (const message of query({
  prompt: "Analyze the codebase structure",
  options: {
    settingSources: ["user", "project"],  // Load Skills from filesystem
    allowedTools: ["Skill", "Read", "Grep", "Glob"]  // Restricted toolset
  }
})) {
  console.log(message);
}
```

</CodeGroup>

## 사용 가능한 스킬 발견

SDK 애플리케이션에서 사용 가능한 스킬을 확인하려면 Claude에게 간단히 물어보세요:

<CodeGroup>

```python Python
options = ClaudeAgentOptions(
    setting_sources=["user", "project"],  # Load Skills from filesystem
    allowed_tools=["Skill"]
)

async for message in query(
    prompt="What Skills are available?",
    options=options
):
    print(message)
```

```typescript TypeScript
for await (const message of query({
  prompt: "What Skills are available?",
  options: {
    settingSources: ["user", "project"],  // Load Skills from filesystem
    allowedTools: ["Skill"]
  }
})) {
  console.log(message);
}
```

</CodeGroup>

Claude는 현재 작업 디렉토리 및 설치된 플러그인을 기반으로 사용 가능한 스킬을 나열합니다.

## 스킬 테스트

설명과 일치하는 질문을 하여 스킬을 테스트합니다:

<CodeGroup>

```python Python
options = ClaudeAgentOptions(
    cwd="/path/to/project",
    setting_sources=["user", "project"],  # Load Skills from filesystem
    allowed_tools=["Skill", "Read", "Bash"]
)

async for message in query(
    prompt="Extract text from invoice.pdf",
    options=options
):
    print(message)
```

```typescript TypeScript
for await (const message of query({
  prompt: "Extract text from invoice.pdf",
  options: {
    cwd: "/path/to/project",
    settingSources: ["user", "project"],  // Load Skills from filesystem
    allowedTools: ["Skill", "Read", "Bash"]
  }
})) {
  console.log(message);
}
```

</CodeGroup>

Claude는 설명이 요청과 일치하면 관련 스킬을 자동으로 호출합니다.

## 문제 해결

### 스킬을 찾을 수 없음

**settingSources 구성 확인**: 스킬은 `settingSources`/`setting_sources`를 명시적으로 구성할 때만 로드됩니다. 이것이 가장 일반적인 문제입니다:

<CodeGroup>

```python Python
# 잘못됨 - 스킬이 로드되지 않음
options = ClaudeAgentOptions(
    allowed_tools=["Skill"]
)

# 올바름 - 스킬이 로드됨
options = ClaudeAgentOptions(
    setting_sources=["user", "project"],  # Required to load Skills
    allowed_tools=["Skill"]
)
```

```typescript TypeScript
// 잘못됨 - 스킬이 로드되지 않음
const options = {
  allowedTools: ["Skill"]
};

// 올바름 - 스킬이 로드됨
const options = {
  settingSources: ["user", "project"],  // Required to load Skills
  allowedTools: ["Skill"]
};
```

</CodeGroup>

`settingSources`/`setting_sources`에 대한 자세한 내용은 [TypeScript SDK 참조](/docs/ko/agent-sdk/typescript#settingsource) 또는 [Python SDK 참조](/docs/ko/agent-sdk/python#settingsource)를 참조하세요.

**작업 디렉토리 확인**: SDK는 `cwd` 옵션을 기준으로 스킬을 로드합니다. `.claude/skills/`를 포함하는 디렉토리를 가리키는지 확인하세요:

<CodeGroup>

```python Python
# cwd가 .claude/skills/를 포함하는 디렉토리를 가리키는지 확인하세요
options = ClaudeAgentOptions(
    cwd="/path/to/project",  # Must contain .claude/skills/
    setting_sources=["user", "project"],  # Required to load Skills
    allowed_tools=["Skill"]
)
```

```typescript TypeScript
// cwd가 .claude/skills/를 포함하는 디렉토리를 가리키는지 확인하세요
const options = {
  cwd: "/path/to/project",  // Must contain .claude/skills/
  settingSources: ["user", "project"],  // Required to load Skills
  allowedTools: ["Skill"]
};
```

</CodeGroup>

위의 "SDK에서 스킬 사용" 섹션에서 완전한 패턴을 참조하세요.

**파일시스템 위치 확인**:
```bash
# 프로젝트 스킬 확인
ls .claude/skills/*/SKILL.md

# 개인 스킬 확인
ls ~/.claude/skills/*/SKILL.md
```

### 스킬이 사용되지 않음

**스킬 도구가 활성화되었는지 확인**: `allowedTools`에 `"Skill"`이 있는지 확인하세요.

**설명 확인**: 구체적이고 관련 키워드를 포함하는지 확인하세요. 효과적인 설명 작성에 대한 지침은 [에이전트 스킬 모범 사례](/docs/ko/agents-and-tools/agent-skills/best-practices#writing-effective-descriptions)를 참조하세요.

### 추가 문제 해결

일반 스킬 문제 해결(YAML 구문, 디버깅 등)은 [Claude Code 스킬 문제 해결 섹션](https://code.claude.com/docs/skills#troubleshooting)을 참조하세요.

## 관련 문서

### 스킬 가이드
- [Claude Code의 에이전트 스킬](https://code.claude.com/docs/skills): 생성, 예제 및 문제 해결이 포함된 완전한 스킬 가이드
- [에이전트 스킬 개요](/docs/ko/agents-and-tools/agent-skills/overview): 개념적 개요, 이점 및 아키텍처
- [에이전트 스킬 모범 사례](/docs/ko/agents-and-tools/agent-skills/best-practices): 효과적인 스킬을 위한 작성 지침
- [에이전트 스킬 쿡북](https://github.com/anthropics/claude-cookbooks/tree/main/skills): 예제 스킬 및 템플릿

### SDK 리소스
- [SDK의 서브에이전트](/docs/ko/agent-sdk/subagents): 프로그래밍 옵션이 있는 유사한 파일시스템 기반 에이전트
- [SDK의 슬래시 명령](/docs/ko/agent-sdk/slash-commands): 사용자 호출 명령
- [SDK 개요](/docs/ko/agent-sdk/overview): 일반 SDK 개념
- [TypeScript SDK 참조](/docs/ko/agent-sdk/typescript): 완전한 API 문서
- [Python SDK 참조](/docs/ko/agent-sdk/python): 완전한 API 문서