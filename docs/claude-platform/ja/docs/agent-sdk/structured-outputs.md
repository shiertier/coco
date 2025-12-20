# SDK での構造化出力

エージェント ワークフローから検証済みの JSON 結果を取得する

---

エージェント ワークフローから構造化された検証済みの JSON を取得します。Agent SDK は JSON スキーマを通じて構造化出力をサポートしており、エージェントが必要な形式で正確にデータを返すことを保証します。

<Note>
**構造化出力を使用する場合**

エージェントがツール（ファイル検索、コマンド実行、ウェブ検索など）を使用した複数ターンのワークフローを完了した後に検証済みの JSON が必要な場合は、構造化出力を使用します。

ツール使用なしの単一の API 呼び出しについては、[API 構造化出力](/docs/ja/build-with-claude/structured-outputs)を参照してください。
</Note>

## 構造化出力を使用する理由

構造化出力は、アプリケーションとの信頼性の高い、型安全な統合を提供します。

- **検証済みの構造**: スキーマに一致する有効な JSON を常に受け取ります
- **統合の簡素化**: 解析または検証コードは不要です
- **型安全性**: TypeScript または Python 型ヒントでエンドツーエンドの安全性を使用します
- **クリーンな分離**: タスク指示とは別に出力要件を定義します
- **ツール自律性**: エージェントは使用するツールを選択しながら、出力形式を保証します

<Tabs>
<Tab title="TypeScript">

## クイックスタート

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

## Zod を使用したスキーマの定義

TypeScript プロジェクトの場合、Zod を使用して型安全なスキーマ定義と検証を行います。

```typescript
import { z } from 'zod'
import { zodToJsonSchema } from 'zod-to-json-schema'

// Zod でスキーマを定義
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

// JSON スキーマに変換
const schema = zodToJsonSchema(AnalysisResult, { $refStrategy: 'root' })

// クエリで使用
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
    // 検証して完全に型付けされた結果を取得
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

**Zod の利点:**
- 完全な TypeScript 型推論
- `safeParse()` による実行時検証
- より良いエラーメッセージ
- 構成可能なスキーマ

</Tab>
<Tab title="Python">

## クイックスタート

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

## Pydantic を使用したスキーマの定義

Python プロジェクトの場合、Pydantic を使用して型安全なスキーマ定義と検証を行います。

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

# クエリで使用
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
        # 検証して完全に型付けされた結果を取得
        result = AnalysisResult.model_validate(message.structured_output)
        print(f"Score: {result.score}")
        print(f"Found {len(result.issues)} issues")
        for issue in result.issues:
            print(f"[{issue.severity}] {issue.file}: {issue.description}")
```

**Pydantic の利点:**
- 完全な Python 型ヒント
- `model_validate()` による実行時検証
- より良いエラーメッセージ
- データクラス機能

</Tab>
</Tabs>

## 構造化出力の仕組み

<Steps>
  <Step title="JSON スキーマを定義する">
    エージェントが返すべき構造を説明する JSON スキーマを作成します。スキーマは標準的な JSON スキーマ形式を使用します。
  </Step>
  <Step title="outputFormat パラメータを追加する">
    クエリオプションに `outputFormat` パラメータを含め、`type: "json_schema"` とスキーマ定義を指定します。
  </Step>
  <Step title="クエリを実行する">
    エージェントはタスクを完了するために必要なツール（ファイル操作、コマンド、ウェブ検索など）を使用します。
  </Step>
  <Step title="検証済み出力にアクセスする">
    エージェントの最終結果は、スキーマに一致する有効な JSON となり、`message.structured_output` で利用可能になります。
  </Step>
</Steps>

## サポートされている JSON スキーマ機能

Agent SDK は、[API 構造化出力](/docs/ja/build-with-claude/structured-outputs#json-schema-limitations)と同じ JSON スキーマ機能と制限をサポートしています。

主にサポートされている機能:
- すべての基本型: object、array、string、integer、number、boolean、null
- `enum`、`const`、`required`、`additionalProperties`（`false` である必要があります）
- 文字列形式: `date-time`、`date`、`email`、`uri`、`uuid` など
- `$ref`、`$def`、および `definitions`

サポートされている機能、制限、および正規表現パターンサポートの完全な詳細については、API ドキュメントの [JSON スキーマの制限](/docs/ja/build-with-claude/structured-outputs#json-schema-limitations)を参照してください。

## 例: TODO 追跡エージェント

コードから TODO を検索し、git blame 情報を抽出するエージェントを示す完全な例を以下に示します。

<CodeGroup>

```typescript TypeScript
import { query } from '@anthropic-ai/claude-agent-sdk'

// TODO 抽出用の構造を定義
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

// エージェントは Grep を使用して TODO を検索し、Bash を使用して git blame 情報を取得
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

# TODO 抽出用の構造を定義
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

# エージェントは Grep を使用して TODO を検索し、Bash を使用して git blame 情報を取得
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

エージェントは自律的に適切なツール（Grep、Bash）を使用して情報を収集し、検証済みデータを返します。

## エラーハンドリング

エージェントがスキーマに一致する有効な出力を生成できない場合、エラー結果が表示されます。

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

## 関連リソース

- [JSON スキーマドキュメント](https://json-schema.org/)
- [API 構造化出力](/docs/ja/build-with-claude/structured-outputs) - 単一の API 呼び出し用
- [カスタムツール](/docs/ja/agent-sdk/custom-tools) - エージェント用のツールを定義
- [TypeScript SDK リファレンス](/docs/ja/agent-sdk/typescript) - 完全な TypeScript API
- [Python SDK リファレンス](/docs/ja/agent-sdk/python) - 完全な Python API