# 시스템 프롬프트 수정하기

출력 스타일, append를 사용한 systemPrompt, 사용자 정의 시스템 프롬프트의 세 가지 접근 방식을 사용하여 시스템 프롬프트를 수정해 Claude의 동작을 사용자 정의하는 방법을 알아보세요.

---

시스템 프롬프트는 Claude의 동작, 기능 및 응답 스타일을 정의합니다. Claude Agent SDK는 시스템 프롬프트를 사용자 정의하는 세 가지 방법을 제공합니다: 출력 스타일 사용(지속적인 파일 기반 구성), Claude Code의 프롬프트에 추가, 또는 완전히 사용자 정의된 프롬프트 사용.

## 시스템 프롬프트 이해하기

시스템 프롬프트는 대화 전반에 걸쳐 Claude가 어떻게 행동할지를 형성하는 초기 명령어 세트입니다.

<Note>
**기본 동작:** Agent SDK는 최대한의 유연성을 위해 기본적으로 **빈 시스템 프롬프트**를 사용합니다. Claude Code의 시스템 프롬프트(도구 지침, 코드 가이드라인 등)를 사용하려면 TypeScript에서 `systemPrompt: { preset: "claude_code" }`를 지정하거나 Python에서 `system_prompt="claude_code"`를 지정하세요.
</Note>

Claude Code의 시스템 프롬프트에는 다음이 포함됩니다:

- 도구 사용 지침 및 사용 가능한 도구
- 코드 스타일 및 형식 지침
- 응답 톤 및 상세도 설정
- 보안 및 안전 지침
- 현재 작업 디렉토리 및 환경에 대한 컨텍스트

## 수정 방법

### 방법 1: CLAUDE.md 파일 (프로젝트 수준 지침)

CLAUDE.md 파일은 Agent SDK가 디렉토리에서 실행될 때 자동으로 읽는 프로젝트별 컨텍스트와 지침을 제공합니다. 이들은 프로젝트의 지속적인 "메모리" 역할을 합니다.

#### CLAUDE.md가 SDK와 작동하는 방식

**위치 및 발견:**

- **프로젝트 수준:** 작업 디렉토리의 `CLAUDE.md` 또는 `.claude/CLAUDE.md`
- **사용자 수준:** 모든 프로젝트에 걸친 전역 지침을 위한 `~/.claude/CLAUDE.md`

**중요:** SDK는 `settingSources` (TypeScript) 또는 `setting_sources` (Python)를 명시적으로 구성할 때만 CLAUDE.md 파일을 읽습니다:

- 프로젝트 수준 CLAUDE.md를 로드하려면 `'project'`를 포함하세요
- 사용자 수준 CLAUDE.md (`~/.claude/CLAUDE.md`)를 로드하려면 `'user'`를 포함하세요

`claude_code` 시스템 프롬프트 프리셋은 CLAUDE.md를 자동으로 로드하지 않습니다 - 설정 소스도 지정해야 합니다.

**콘텐츠 형식:**
CLAUDE.md 파일은 일반 마크다운을 사용하며 다음을 포함할 수 있습니다:

- 코딩 가이드라인 및 표준
- 프로젝트별 컨텍스트
- 일반적인 명령어 또는 워크플로우
- API 규칙
- 테스트 요구사항

#### CLAUDE.md 예시

```markdown
# 프로젝트 가이드라인

## 코드 스타일

- TypeScript strict 모드 사용
- React에서 함수형 컴포넌트 선호
- 공개 API에 대해 항상 JSDoc 주석 포함

## 테스트

- 커밋 전에 `npm test` 실행
- 80% 이상의 코드 커버리지 유지
- 단위 테스트는 jest, E2E는 playwright 사용

## 명령어

- 빌드: `npm run build`
- 개발 서버: `npm run dev`
- 타입 체크: `npm run typecheck`
```

#### SDK와 함께 CLAUDE.md 사용하기

<CodeGroup>

```typescript TypeScript
import { query } from "@anthropic-ai/claude-agent-sdk";

// 중요: CLAUDE.md를 로드하려면 settingSources를 지정해야 합니다
// claude_code 프리셋만으로는 CLAUDE.md 파일을 로드하지 않습니다
const messages = [];

for await (const message of query({
  prompt: "사용자 프로필을 위한 새로운 React 컴포넌트를 추가하세요",
  options: {
    systemPrompt: {
      type: "preset",
      preset: "claude_code", // Claude Code의 시스템 프롬프트 사용
    },
    settingSources: ["project"], // 프로젝트에서 CLAUDE.md를 로드하는 데 필요
  },
})) {
  messages.push(message);
}

// 이제 Claude는 CLAUDE.md의 프로젝트 가이드라인에 액세스할 수 있습니다
```

```python Python
from claude_agent_sdk import query, ClaudeAgentOptions

# 중요: CLAUDE.md를 로드하려면 setting_sources를 지정해야 합니다
# claude_code 프리셋만으로는 CLAUDE.md 파일을 로드하지 않습니다
messages = []

async for message in query(
    prompt="사용자 프로필을 위한 새로운 React 컴포넌트를 추가하세요",
    options=ClaudeAgentOptions(
        system_prompt={
            "type": "preset",
            "preset": "claude_code"  # Claude Code의 시스템 프롬프트 사용
        },
        setting_sources=["project"]  # 프로젝트에서 CLAUDE.md를 로드하는 데 필요
    )
):
    messages.append(message)

# 이제 Claude는 CLAUDE.md의 프로젝트 가이드라인에 액세스할 수 있습니다
```

</CodeGroup>

#### CLAUDE.md를 언제 사용할지

**다음에 가장 적합:**

- **팀 공유 컨텍스트** - 모든 사람이 따라야 할 가이드라인
- **프로젝트 규칙** - 코딩 표준, 파일 구조, 명명 패턴
- **일반적인 명령어** - 프로젝트별 빌드, 테스트, 배포 명령어
- **장기 메모리** - 모든 세션에서 지속되어야 하는 컨텍스트
- **버전 관리된 지침** - 팀이 동기화를 유지할 수 있도록 git에 커밋

**주요 특징:**

- ✅ 프로젝트의 모든 세션에서 지속
- ✅ git을 통해 팀과 공유
- ✅ 자동 발견 (코드 변경 불필요)
- ⚠️ `settingSources`를 통한 설정 로딩 필요

### 방법 2: 출력 스타일 (지속적인 구성)

출력 스타일은 Claude의 시스템 프롬프트를 수정하는 저장된 구성입니다. 마크다운 파일로 저장되며 세션과 프로젝트 전반에 걸쳐 재사용할 수 있습니다.

#### 출력 스타일 생성하기

<CodeGroup>

```typescript TypeScript
import { writeFile, mkdir } from "fs/promises";
import { join } from "path";
import { homedir } from "os";

async function createOutputStyle(
  name: string,
  description: string,
  prompt: string
) {
  // 사용자 수준: ~/.claude/output-styles
  // 프로젝트 수준: .claude/output-styles
  const outputStylesDir = join(homedir(), ".claude", "output-styles");

  await mkdir(outputStylesDir, { recursive: true });

  const content = `---
name: ${name}
description: ${description}
---

${prompt}`;

  const filePath = join(
    outputStylesDir,
    `${name.toLowerCase().replace(/\s+/g, "-")}.md`
  );
  await writeFile(filePath, content, "utf-8");
}

// 예시: 코드 리뷰 전문가 생성
await createOutputStyle(
  "Code Reviewer",
  "철저한 코드 리뷰 어시스턴트",
  `당신은 전문 코드 리뷰어입니다.

모든 코드 제출에 대해:
1. 버그 및 보안 문제 확인
2. 성능 평가
3. 개선 사항 제안
4. 코드 품질 평가 (1-10)`
);
```

```python Python
from pathlib import Path

async def create_output_style(name: str, description: str, prompt: str):
    # 사용자 수준: ~/.claude/output-styles
    # 프로젝트 수준: .claude/output-styles
    output_styles_dir = Path.home() / '.claude' / 'output-styles'

    output_styles_dir.mkdir(parents=True, exist_ok=True)

    content = f"""---
name: {name}
description: {description}
---

{prompt}"""

    file_name = name.lower().replace(' ', '-') + '.md'
    file_path = output_styles_dir / file_name
    file_path.write_text(content, encoding='utf-8')

# 예시: 코드 리뷰 전문가 생성
await create_output_style(
    'Code Reviewer',
    '철저한 코드 리뷰 어시스턴트',
    """당신은 전문 코드 리뷰어입니다.

모든 코드 제출에 대해:
1. 버그 및 보안 문제 확인
2. 성능 평가
3. 개선 사항 제안
4. 코드 품질 평가 (1-10)"""
)
```

</CodeGroup>

#### 출력 스타일 사용하기

생성된 후, 다음을 통해 출력 스타일을 활성화하세요:

- **CLI**: `/output-style [style-name]`
- **설정**: `.claude/settings.local.json`
- **새로 생성**: `/output-style:new [description]`

**SDK 사용자를 위한 참고사항:** 출력 스타일은 옵션에서 `settingSources: ['user']` 또는 `settingSources: ['project']` (TypeScript) / `setting_sources=["user"]` 또는 `setting_sources=["project"]` (Python)를 포함할 때 로드됩니다.

### 방법 3: append와 함께 `systemPrompt` 사용하기

모든 내장 기능을 보존하면서 사용자 정의 지침을 추가하기 위해 `append` 속성과 함께 Claude Code 프리셋을 사용할 수 있습니다.

<CodeGroup>

```typescript TypeScript
import { query } from "@anthropic-ai/claude-agent-sdk";

const messages = [];

for await (const message of query({
  prompt: "피보나치 수를 계산하는 Python 함수를 작성하는 데 도움을 주세요",
  options: {
    systemPrompt: {
      type: "preset",
      preset: "claude_code",
      append:
        "Python 코드에 항상 상세한 docstring과 타입 힌트를 포함하세요.",
    },
  },
})) {
  messages.push(message);
  if (message.type === "assistant") {
    console.log(message.message.content);
  }
}
```

```python Python
from claude_agent_sdk import query, ClaudeAgentOptions

messages = []

async for message in query(
    prompt="피보나치 수를 계산하는 Python 함수를 작성하는 데 도움을 주세요",
    options=ClaudeAgentOptions(
        system_prompt={
            "type": "preset",
            "preset": "claude_code",
            "append": "Python 코드에 항상 상세한 docstring과 타입 힌트를 포함하세요."
        }
    )
):
    messages.append(message)
    if message.type == 'assistant':
        print(message.message.content)
```

</CodeGroup>

### 방법 4: 사용자 정의 시스템 프롬프트

`systemPrompt`에 사용자 정의 문자열을 제공하여 기본값을 완전히 자신만의 지침으로 대체할 수 있습니다.

<CodeGroup>

```typescript TypeScript
import { query } from "@anthropic-ai/claude-agent-sdk";

const customPrompt = `당신은 Python 코딩 전문가입니다.
다음 가이드라인을 따르세요:
- 깔끔하고 잘 문서화된 코드 작성
- 모든 함수에 타입 힌트 사용
- 포괄적인 docstring 포함
- 적절한 경우 함수형 프로그래밍 패턴 선호
- 항상 코드 선택에 대해 설명`;

const messages = [];

for await (const message of query({
  prompt: "데이터 처리 파이프라인을 생성하세요",
  options: {
    systemPrompt: customPrompt,
  },
})) {
  messages.push(message);
  if (message.type === "assistant") {
    console.log(message.message.content);
  }
}
```

```python Python
from claude_agent_sdk import query, ClaudeAgentOptions

custom_prompt = """당신은 Python 코딩 전문가입니다.
다음 가이드라인을 따르세요:
- 깔끔하고 잘 문서화된 코드 작성
- 모든 함수에 타입 힌트 사용
- 포괄적인 docstring 포함
- 적절한 경우 함수형 프로그래밍 패턴 선호
- 항상 코드 선택에 대해 설명"""

messages = []

async for message in query(
    prompt="데이터 처리 파이프라인을 생성하세요",
    options=ClaudeAgentOptions(
        system_prompt=custom_prompt
    )
):
    messages.append(message)
    if message.type == 'assistant':
        print(message.message.content)
```

</CodeGroup>

## 네 가지 접근 방식 모두 비교

| 기능                 | CLAUDE.md           | 출력 스타일      | append와 함께 `systemPrompt` | 사용자 정의 `systemPrompt`     |
| --- | --- | --- | --- | --- |
| **지속성**         | 프로젝트별 파일 | 파일로 저장  | 세션만            | 세션만           |
| **재사용성**         | 프로젝트별      | 프로젝트 전반 | 코드 중복        | 코드 중복       |
| **관리**          | 파일시스템에서    | CLI + 파일     | 코드에서                 | 코드에서                |
| **기본 도구**       | 보존        | 보존       | 보존               | 손실 (포함하지 않는 한) |
| **내장 안전성**     | 유지       | 유지      | 유지              | 추가해야 함          |
| **환경 컨텍스트** | 자동        | 자동       | 자동               | 제공해야 함       |
| **사용자 정의 수준** | 추가만   | 기본값 대체 | 추가만          | 완전한 제어       |
| **버전 관리**     | 프로젝트와 함께     | 예             | 코드와 함께               | 코드와 함께              |
| **범위**               | 프로젝트별 | 사용자 또는 프로젝트 | 코드 세션            | 코드 세션           |

**참고:** "append와 함께"는 TypeScript에서 `systemPrompt: { type: "preset", preset: "claude_code", append: "..." }`를 사용하거나 Python에서 `system_prompt={"type": "preset", "preset": "claude_code", "append": "..."}`를 사용하는 것을 의미합니다.

## 사용 사례 및 모범 사례

### CLAUDE.md를 언제 사용할지

**다음에 가장 적합:**

- 프로젝트별 코딩 표준 및 규칙
- 프로젝트 구조 및 아키텍처 문서화
- 일반적인 명령어 나열 (빌드, 테스트, 배포)
- 버전 관리되어야 하는 팀 공유 컨텍스트
- 프로젝트의 모든 SDK 사용에 적용되는 지침

**예시:**

- "모든 API 엔드포인트는 async/await 패턴을 사용해야 합니다"
- "커밋 전에 `npm run lint:fix`를 실행하세요"
- "데이터베이스 마이그레이션은 `migrations/` 디렉토리에 있습니다"

**중요:** CLAUDE.md 파일을 로드하려면 `settingSources: ['project']` (TypeScript) 또는 `setting_sources=["project"]` (Python)를 명시적으로 설정해야 합니다. `claude_code` 시스템 프롬프트 프리셋은 이 설정 없이는 CLAUDE.md를 자동으로 로드하지 않습니다.

### 출력 스타일을 언제 사용할지

**다음에 가장 적합:**

- 세션 전반에 걸친 지속적인 동작 변경
- 팀 공유 구성
- 전문화된 어시스턴트 (코드 리뷰어, 데이터 사이언티스트, DevOps)
- 버전 관리가 필요한 복잡한 프롬프트 수정

**예시:**

- 전용 SQL 최적화 어시스턴트 생성
- 보안 중심 코드 리뷰어 구축
- 특정 교육법을 가진 교육 어시스턴트 개발

### append와 함께 `systemPrompt`를 언제 사용할지

**다음에 가장 적합:**

- 특정 코딩 표준 또는 선호도 추가
- 출력 형식 사용자 정의
- 도메인별 지식 추가
- 응답 상세도 수정
- 도구 지침을 잃지 않고 Claude Code의 기본 동작 향상

### 사용자 정의 `systemPrompt`를 언제 사용할지

**다음에 가장 적합:**

- Claude의 동작에 대한 완전한 제어
- 전문화된 단일 세션 작업
- 새로운 프롬프트 전략 테스트
- 기본 도구가 필요하지 않은 상황
- 고유한 동작을 가진 전문화된 에이전트 구축

## 접근 방식 결합하기

최대한의 유연성을 위해 이러한 방법들을 결합할 수 있습니다:

### 예시: 세션별 추가 사항이 있는 출력 스타일

<CodeGroup>

```typescript TypeScript
import { query } from "@anthropic-ai/claude-agent-sdk";

// "Code Reviewer" 출력 스타일이 활성화되어 있다고 가정 (/output-style을 통해)
// 세션별 중점 영역 추가
const messages = [];

for await (const message of query({
  prompt: "이 인증 모듈을 검토하세요",
  options: {
    systemPrompt: {
      type: "preset",
      preset: "claude_code",
      append: `
        이 검토에서 다음을 우선시하세요:
        - OAuth 2.0 준수
        - 토큰 저장 보안
        - 세션 관리
      `,
    },
  },
})) {
  messages.push(message);
}
```

```python Python
from claude_agent_sdk import query, ClaudeAgentOptions

# "Code Reviewer" 출력 스타일이 활성화되어 있다고 가정 (/output-style을 통해)
# 세션별 중점 영역 추가
messages = []

async for message in query(
    prompt="이 인증 모듈을 검토하세요",
    options=ClaudeAgentOptions(
        system_prompt={
            "type": "preset",
            "preset": "claude_code",
            "append": """
            이 검토에서 다음을 우선시하세요:
            - OAuth 2.0 준수
            - 토큰 저장 보안
            - 세션 관리
            """
        }
    )
):
    messages.append(message)
```

</CodeGroup>

## 참고 자료

- [출력 스타일](https://code.claude.com/docs/output-styles) - 완전한 출력 스타일 문서
- [TypeScript SDK 가이드](/docs/ko/agent-sdk/typescript) - 완전한 SDK 사용 가이드
- [TypeScript SDK 참조](https://code.claude.com/docs/typescript-sdk-reference) - 전체 API 문서
- [구성 가이드](https://code.claude.com/docs/configuration) - 일반 구성 옵션