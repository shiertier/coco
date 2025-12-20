# SDK의 구조화된 출력

에이전트 워크플로우에서 검증된 JSON 결과 얻기

---

에이전트 워크플로우에서 구조화되고 검증된 JSON을 얻습니다. Agent SDK는 JSON 스키마를 통해 구조화된 출력을 지원하므로 에이전트가 정확히 필요한 형식으로 데이터를 반환합니다.

<Note>
**구조화된 출력을 사용하는 경우**

에이전트가 도구(파일 검색, 명령 실행, 웹 연구 등)를 사용하여 다중 턴 워크플로우를 완료한 후 검증된 JSON이 필요할 때 구조화된 출력을 사용합니다.

도구 사용 없이 단일 API 호출의 경우 [API 구조화된 출력](/docs/ko/build-with-claude/structured-outputs)을 참조하세요.
</Note>

## 구조화된 출력을 사용하는 이유

구조화된 출력은 애플리케이션과의 안정적이고 타입 안전한 통합을 제공합니다:

- **검증된 구조**: 항상 스키마와 일치하는 유효한 JSON을 수신합니다
- **간소화된 통합**: 파싱 또는 검증 코드가 필요하지 않습니다
- **타입 안전성**: TypeScript 또는 Python 타입 힌트와 함께 사용하여 엔드투엔드 안전성을 확보합니다
- **깔끔한 분리**: 작업 지침과 별도로 출력 요구사항을 정의합니다
- **도구 자율성**: 에이전트가 사용할 도구를 선택하면서 출력 형식을 보장합니다

<Tabs>
<Tab title="TypeScript">

## 빠른 시작

```typescript
import { query } from '@anthropic-ai/claude-agent-sdk'

const schema = {
  type: 'object',
  properties: {
    company_name: { type: 'string' },
    founded_year: { type: 'number' },
    headquarters: { type: 'string' }
  },
  required: ['company_name']
}

for await (const message of query({
  prompt: 'Research Anthropic and provide key company information',
  options: {
    outputFormat: {
      type: 'json_schema',
      schema: schema
    }
  }
})) {
  if (message.type === 'result' && message.structured_output) {
    console.log(message.structured_output)
    // { company_name: "Anthropic", founded_year: 2021, headquarters: "San Francisco, CA" }
  }
}
```

## Zod를 사용한 스키마 정의

TypeScript 프로젝트의 경우 Zod를 사용하여 타입 안전 스키마 정의 및 검증을 수행합니다:

```typescript
import { z } from 'zod'
import { zodToJsonSchema } from 'zod-to-json-schema'

// Zod를 사용한 스키마 정의
const AnalysisResult = z.object({
  summary: z.string(),
  issues: z.array(z.object({
    severity: z.enum(['low', 'medium', 'high']),
    description: z.string(),
    file: z.string()
  })),
  score: z.number().min(0).max(100)
})

type AnalysisResult = z.infer<typeof AnalysisResult>

// JSON 스키마로 변환
const schema = zodToJsonSchema(AnalysisResult, { $refStrategy: 'root' })

// 쿼리에서 사용
for await (const message of query({
  prompt: 'Analyze the codebase for security issues',
  options: {
    outputFormat: {
      type: 'json_schema',
      schema: schema
    }
  }
})) {
  if (message.type === 'result' && message.structured_output) {
    // 검증 및 완전히 타입된 결과 얻기
    const parsed = AnalysisResult.safeParse(message.structured_output)
    if (parsed.success) {
      const data: AnalysisResult = parsed.data
      console.log(`Score: ${data.score}`)
      console.log(`Found ${data.issues.length} issues`)
      data.issues.forEach(issue => {
        console.log(`[${issue.severity}] ${issue.file}: ${issue.description}`)
      })
    }
  }
}
```

**Zod의 장점:**
- 완전한 TypeScript 타입 추론
- `safeParse()`를 사용한 런타임 검증
- 더 나은 오류 메시지
- 조합 가능한 스키마

</Tab>
<Tab title="Python">

## 빠른 시작

```python
from claude_agent_sdk import query

schema = {
    "type": "object",
    "properties": {
        "company_name": {"type": "string"},
        "founded_year": {"type": "number"},
        "headquarters": {"type": "string"}
    },
    "required": ["company_name"]
}

async for message in query(
    prompt="Research Anthropic and provide key company information",
    options={
        "output_format": {
            "type": "json_schema",
            "schema": schema
        }
    }
):
    if hasattr(message, 'structured_output'):
        print(message.structured_output)
        # {'company_name': 'Anthropic', 'founded_year': 2021, 'headquarters': 'San Francisco, CA'}
```

## Pydantic을 사용한 스키마 정의

Python 프로젝트의 경우 Pydantic을 사용하여 타입 안전 스키마 정의 및 검증을 수행합니다:

```python
from pydantic import BaseModel
from claude_agent_sdk import query

class Issue(BaseModel):
    severity: str  # 'low', 'medium', 'high'
    description: str
    file: str

class AnalysisResult(BaseModel):
    summary: str
    issues: list[Issue]
    score: int

# 쿼리에서 사용
async for message in query(
    prompt="Analyze the codebase for security issues",
    options={
        "output_format": {
            "type": "json_schema",
            "schema": AnalysisResult.model_json_schema()
        }
    }
):
    if hasattr(message, 'structured_output'):
        # 검증 및 완전히 타입된 결과 얻기
        result = AnalysisResult.model_validate(message.structured_output)
        print(f"Score: {result.score}")
        print(f"Found {len(result.issues)} issues")
        for issue in result.issues:
            print(f"[{issue.severity}] {issue.file}: {issue.description}")
```

**Pydantic의 장점:**
- 완전한 Python 타입 힌트
- `model_validate()`를 사용한 런타임 검증
- 더 나은 오류 메시지
- 데이터 클래스 기능

</Tab>
</Tabs>

## 구조화된 출력의 작동 방식

<Steps>
  <Step title="JSON 스키마 정의">
    에이전트가 반환할 구조를 설명하는 JSON 스키마를 만듭니다. 스키마는 표준 JSON 스키마 형식을 사용합니다.
  </Step>
  <Step title="outputFormat 매개변수 추가">
    쿼리 옵션에 `outputFormat` 매개변수를 포함하고 `type: "json_schema"`와 스키마 정의를 지정합니다.
  </Step>
  <Step title="쿼리 실행">
    에이전트는 작업을 완료하기 위해 필요한 도구(파일 작업, 명령, 웹 검색 등)를 사용합니다.
  </Step>
  <Step title="검증된 출력 액세스">
    에이전트의 최종 결과는 스키마와 일치하는 유효한 JSON이며 `message.structured_output`에서 사용할 수 있습니다.
  </Step>
</Steps>

## 지원되는 JSON 스키마 기능

Agent SDK는 [API 구조화된 출력](/docs/ko/build-with-claude/structured-outputs#json-schema-limitations)과 동일한 JSON 스키마 기능 및 제한사항을 지원합니다.

주요 지원 기능:
- 모든 기본 타입: object, array, string, integer, number, boolean, null
- `enum`, `const`, `required`, `additionalProperties` (`false`여야 함)
- 문자열 형식: `date-time`, `date`, `email`, `uri`, `uuid` 등
- `$ref`, `$def` 및 `definitions`

지원되는 기능, 제한사항 및 정규식 패턴 지원에 대한 완전한 세부사항은 API 문서의 [JSON 스키마 제한사항](/docs/ko/build-with-claude/structured-outputs#json-schema-limitations)을 참조하세요.

## 예제: TODO 추적 에이전트

코드에서 TODO를 검색하고 git blame 정보를 추출하는 에이전트를 보여주는 완전한 예제입니다:

<CodeGroup>

```typescript TypeScript
import { query } from '@anthropic-ai/claude-agent-sdk'

// TODO 추출을 위한 구조 정의
const todoSchema = {
  type: 'object',
  properties: {
    todos: {
      type: 'array',
      items: {
        type: 'object',
        properties: {
          text: { type: 'string' },
          file: { type: 'string' },
          line: { type: 'number' },
          author: { type: 'string' },
          date: { type: 'string' }
        },
        required: ['text', 'file', 'line']
      }
    },
    total_count: { type: 'number' }
  },
  required: ['todos', 'total_count']
}

// 에이전트는 Grep을 사용하여 TODO를 찾고 Bash를 사용하여 git blame 정보를 얻습니다
for await (const message of query({
  prompt: 'Find all TODO comments in src/ and identify who added them',
  options: {
    outputFormat: {
      type: 'json_schema',
      schema: todoSchema
    }
  }
})) {
  if (message.type === 'result' && message.structured_output) {
    const data = message.structured_output
    console.log(`Found ${data.total_count} TODOs`)
    data.todos.forEach(todo => {
      console.log(`${todo.file}:${todo.line} - ${todo.text}`)
      if (todo.author) {
        console.log(`  Added by ${todo.author} on ${todo.date}`)
      }
    })
  }
}
```

```python Python
from claude_agent_sdk import query

# TODO 추출을 위한 구조 정의
todo_schema = {
    "type": "object",
    "properties": {
        "todos": {
            "type": "array",
            "items": {
                "type": "object",
                "properties": {
                    "text": {"type": "string"},
                    "file": {"type": "string"},
                    "line": {"type": "number"},
                    "author": {"type": "string"},
                    "date": {"type": "string"}
                },
                "required": ["text", "file", "line"]
            }
        },
        "total_count": {"type": "number"}
    },
    "required": ["todos", "total_count"]
}

# 에이전트는 Grep을 사용하여 TODO를 찾고 Bash를 사용하여 git blame 정보를 얻습니다
async for message in query(
    prompt="Find all TODO comments in src/ and identify who added them",
    options={
        "output_format": {
            "type": "json_schema",
            "schema": todo_schema
        }
    }
):
    if hasattr(message, 'structured_output'):
        data = message.structured_output
        print(f"Found {data['total_count']} TODOs")
        for todo in data['todos']:
            print(f"{todo['file']}:{todo['line']} - {todo['text']}")
            if 'author' in todo:
                print(f"  Added by {todo['author']} on {todo['date']}")
```

</CodeGroup>

에이전트는 자율적으로 올바른 도구(Grep, Bash)를 사용하여 정보를 수집하고 검증된 데이터를 반환합니다.

## 오류 처리

에이전트가 스키마와 일치하는 유효한 출력을 생성할 수 없으면 오류 결과를 받게 됩니다:

```typescript
for await (const msg of query({
  prompt: 'Analyze the data',
  options: {
    outputFormat: {
      type: 'json_schema',
      schema: mySchema
    }
  }
})) {
  if (msg.type === 'result') {
    if (msg.subtype === 'success' && msg.structured_output) {
      console.log(msg.structured_output)
    } else if (msg.subtype === 'error_max_structured_output_retries') {
      console.error('Could not produce valid output')
    }
  }
}
```

## 관련 리소스

- [JSON 스키마 문서](https://json-schema.org/)
- [API 구조화된 출력](/docs/ko/build-with-claude/structured-outputs) - 단일 API 호출용
- [사용자 정의 도구](/docs/ko/agent-sdk/custom-tools) - 에이전트용 도구 정의
- [TypeScript SDK 참조](/docs/ko/agent-sdk/typescript) - 전체 TypeScript API
- [Python SDK 참조](/docs/ko/agent-sdk/python) - 전체 Python API