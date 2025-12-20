# SDK의 서브에이전트

Claude Agent SDK에서 서브에이전트 작업하기

---

Claude Agent SDK의 서브에이전트는 메인 에이전트에 의해 조율되는 전문화된 AI입니다.
컨텍스트 관리와 병렬화를 위해 서브에이전트를 사용하세요.

이 가이드는 `agents` 매개변수를 사용하여 SDK에서 서브에이전트를 정의하고 사용하는 방법을 설명합니다.

## 개요

SDK를 사용할 때 서브에이전트는 두 가지 방법으로 정의할 수 있습니다:

1. **프로그래밍 방식** - `query()` 옵션에서 `agents` 매개변수 사용 (SDK 애플리케이션에 권장)
2. **파일시스템 기반** - 지정된 디렉토리(`.claude/agents/`)에 YAML 프론트매터가 있는 마크다운 파일 배치

이 가이드는 주로 `agents` 매개변수를 사용하는 프로그래밍 방식에 중점을 두며, 이는 SDK 애플리케이션에 더 통합된 개발 경험을 제공합니다.

## 서브에이전트 사용의 이점

### 컨텍스트 관리
서브에이전트는 메인 에이전트와 별도의 컨텍스트를 유지하여 정보 과부하를 방지하고 상호작용을 집중적으로 유지합니다. 이러한 격리는 전문화된 작업이 관련 없는 세부사항으로 메인 대화 컨텍스트를 오염시키지 않도록 보장합니다.

**예시**: `research-assistant` 서브에이전트는 수십 개의 파일과 문서 페이지를 탐색할 수 있으면서도 모든 중간 검색 결과로 메인 대화를 어수선하게 만들지 않고 관련 발견사항만 반환할 수 있습니다.

### 병렬화
여러 서브에이전트가 동시에 실행되어 복잡한 워크플로우를 극적으로 가속화할 수 있습니다.

**예시**: 코드 리뷰 중에 `style-checker`, `security-scanner`, `test-coverage` 서브에이전트를 동시에 실행하여 리뷰 시간을 몇 분에서 몇 초로 단축할 수 있습니다.

### 전문화된 지시사항과 지식
각 서브에이전트는 특정 전문 지식, 모범 사례, 제약 조건을 가진 맞춤형 시스템 프롬프트를 가질 수 있습니다.

**예시**: `database-migration` 서브에이전트는 SQL 모범 사례, 롤백 전략, 데이터 무결성 검사에 대한 상세한 지식을 가질 수 있으며, 이는 메인 에이전트의 지시사항에서는 불필요한 노이즈가 될 것입니다.

### 도구 제한
서브에이전트는 특정 도구로 제한되어 의도하지 않은 작업의 위험을 줄일 수 있습니다.

**예시**: `doc-reviewer` 서브에이전트는 Read와 Grep 도구에만 액세스할 수 있어 분석은 할 수 있지만 문서 파일을 실수로 수정하지 않도록 보장할 수 있습니다.

## 서브에이전트 생성

### 프로그래밍 정의 (권장)

`agents` 매개변수를 사용하여 코드에서 직접 서브에이전트를 정의하세요:

```typescript
import { query } from '@anthropic-ai/claude-agent-sdk';

const result = query({
  prompt: "인증 모듈의 보안 문제를 검토하세요",
  options: {
    agents: {
      'code-reviewer': {
        description: '전문 코드 리뷰 전문가. 품질, 보안, 유지보수성 리뷰에 사용하세요.',
        prompt: `당신은 보안, 성능, 모범 사례에 전문 지식을 가진 코드 리뷰 전문가입니다.

코드를 리뷰할 때:
- 보안 취약점 식별
- 성능 문제 확인
- 코딩 표준 준수 검증
- 구체적인 개선사항 제안

피드백은 철저하되 간결하게 제공하세요.`,
        tools: ['Read', 'Grep', 'Glob'],
        model: 'sonnet'
      },
      'test-runner': {
        description: '테스트 스위트를 실행하고 분석합니다. 테스트 실행과 커버리지 분석에 사용하세요.',
        prompt: `당신은 테스트 실행 전문가입니다. 테스트를 실행하고 결과에 대한 명확한 분석을 제공하세요.

다음에 집중하세요:
- 테스트 명령 실행
- 테스트 출력 분석
- 실패한 테스트 식별
- 실패에 대한 수정 제안`,
        tools: ['Bash', 'Read', 'Grep'],
      }
    }
  }
});

for await (const message of result) {
  console.log(message);
}
```

### AgentDefinition 구성

| 필드 | 타입 | 필수 | 설명 |
|:---|:---|:---|:---|
| `description` | `string` | 예 | 이 에이전트를 언제 사용할지에 대한 자연어 설명 |
| `prompt` | `string` | 예 | 에이전트의 역할과 행동을 정의하는 시스템 프롬프트 |
| `tools` | `string[]` | 아니오 | 허용된 도구 이름 배열. 생략하면 모든 도구를 상속 |
| `model` | `'sonnet' \| 'opus' \| 'haiku' \| 'inherit'` | 아니오 | 이 에이전트의 모델 재정의. 생략하면 메인 모델로 기본값 설정 |

### 파일시스템 기반 정의 (대안)

특정 디렉토리에 마크다운 파일로 서브에이전트를 정의할 수도 있습니다:

- **프로젝트 수준**: `.claude/agents/*.md` - 현재 프로젝트에서만 사용 가능
- **사용자 수준**: `~/.claude/agents/*.md` - 모든 프로젝트에서 사용 가능

각 서브에이전트는 YAML 프론트매터가 있는 마크다운 파일입니다:

```markdown
---
name: code-reviewer
description: 전문 코드 리뷰 전문가. 품질, 보안, 유지보수성 리뷰에 사용하세요.
tools: Read, Grep, Glob, Bash
---

서브에이전트의 시스템 프롬프트가 여기에 들어갑니다. 이것은 서브에이전트의
역할, 능력, 문제 해결 접근 방식을 정의합니다.
```

**참고:** 프로그래밍 방식으로 정의된 에이전트(`agents` 매개변수를 통해)는 같은 이름의 파일시스템 기반 에이전트보다 우선합니다.

## SDK가 서브에이전트를 사용하는 방법

Claude Agent SDK를 사용할 때 서브에이전트는 프로그래밍 방식으로 정의되거나 파일시스템에서 로드될 수 있습니다. Claude는:

1. **프로그래밍 에이전트 로드** - 옵션의 `agents` 매개변수에서
2. **파일시스템 에이전트 자동 감지** - `.claude/agents/` 디렉토리에서 (재정의되지 않은 경우)
3. **자동으로 호출** - 작업 매칭과 에이전트의 `description`을 기반으로
4. **전문화된 프롬프트 사용** 및 도구 제한
5. **별도 컨텍스트 유지** - 각 서브에이전트 호출에 대해

프로그래밍 방식으로 정의된 에이전트(`agents` 매개변수를 통해)는 같은 이름의 파일시스템 기반 에이전트보다 우선합니다.

## 예시 서브에이전트

코드 리뷰어, 테스트 러너, 디버거, 보안 감사자를 포함한 서브에이전트의 포괄적인 예시는 [메인 서브에이전트 가이드](https://code.claude.com/docs/sub-agents#example-subagents)를 참조하세요. 이 가이드에는 효과적인 서브에이전트 생성을 위한 상세한 구성과 모범 사례가 포함되어 있습니다.

## SDK 통합 패턴

### 자동 호출

SDK는 작업 컨텍스트를 기반으로 적절한 서브에이전트를 자동으로 호출합니다. 에이전트의 `description` 필드가 언제 사용되어야 하는지 명확하게 나타내도록 하세요:

```typescript
const result = query({
  prompt: "API 레이어의 데이터베이스 쿼리를 최적화하세요",
  options: {
    agents: {
      'performance-optimizer': {
        description: '코드 변경이 성능에 영향을 줄 수 있을 때 적극적으로 사용하세요. 최적화 작업에는 반드시 사용되어야 합니다.',
        prompt: '당신은 성능 최적화 전문가입니다...',
        tools: ['Read', 'Edit', 'Bash', 'Grep'],
        model: 'sonnet'
      }
    }
  }
});
```

### 명시적 호출

사용자는 프롬프트에서 특정 서브에이전트를 요청할 수 있습니다:

```typescript
const result = query({
  prompt: "code-reviewer 에이전트를 사용하여 인증 모듈을 확인하세요",
  options: {
    agents: {
      'code-reviewer': {
        description: '전문 코드 리뷰 전문가',
        prompt: '당신은 보안에 중점을 둔 코드 리뷰어입니다...',
        tools: ['Read', 'Grep', 'Glob']
      }
    }
  }
});
```

### 동적 에이전트 구성

애플리케이션의 필요에 따라 에이전트를 동적으로 구성할 수 있습니다:

```typescript
import { query, type AgentDefinition } from '@anthropic-ai/claude-agent-sdk';

function createSecurityAgent(securityLevel: 'basic' | 'strict'): AgentDefinition {
  return {
    description: '보안 코드 리뷰어',
    prompt: `당신은 ${securityLevel === 'strict' ? '엄격한' : '균형잡힌'} 보안 리뷰어입니다...`,
    tools: ['Read', 'Grep', 'Glob'],
    model: securityLevel === 'strict' ? 'opus' : 'sonnet'
  };
}

const result = query({
  prompt: "이 PR의 보안 문제를 검토하세요",
  options: {
    agents: {
      'security-reviewer': createSecurityAgent('strict')
    }
  }
});
```

## 도구 제한

서브에이전트는 `tools` 필드를 통해 제한된 도구 액세스를 가질 수 있습니다:

- **필드 생략** - 에이전트가 사용 가능한 모든 도구를 상속 (기본값)
- **도구 지정** - 에이전트가 나열된 도구만 사용 가능

읽기 전용 분석 에이전트의 예시:

```typescript
const result = query({
  prompt: "이 코드베이스의 아키텍처를 분석하세요",
  options: {
    agents: {
      'code-analyzer': {
        description: '정적 코드 분석 및 아키텍처 검토',
        prompt: `당신은 코드 아키텍처 분석가입니다. 코드 구조를 분석하고,
패턴을 식별하며, 변경 없이 개선사항을 제안하세요.`,
        tools: ['Read', 'Grep', 'Glob']  // 쓰기나 실행 권한 없음
      }
    }
  }
});
```

### 일반적인 도구 조합

**읽기 전용 에이전트** (분석, 검토):
```typescript
tools: ['Read', 'Grep', 'Glob']
```

**테스트 실행 에이전트**:
```typescript
tools: ['Bash', 'Read', 'Grep']
```

**코드 수정 에이전트**:
```typescript
tools: ['Read', 'Edit', 'Write', 'Grep', 'Glob']
```

## 관련 문서

- [메인 서브에이전트 가이드](https://code.claude.com/docs/sub-agents) - 포괄적인 서브에이전트 문서
- [SDK 개요](/docs/ko/agent-sdk/overview) - Claude Agent SDK 개요
- [설정](https://code.claude.com/docs/settings) - 구성 파일 참조
- [슬래시 명령](https://code.claude.com/docs/slash-commands) - 사용자 정의 명령 생성