# SDK 中的結構化輸出

從代理工作流程獲取驗證的 JSON 結果

---

從代理工作流程獲取結構化、驗證的 JSON。Agent SDK 通過 JSON Schemas 支持結構化輸出，確保您的代理以您需要的確切格式返回數據。

<Note>
**何時使用結構化輸出**

當您需要在代理完成包含工具的多輪工作流程（文件搜索、命令執行、網絡研究等）後獲得驗證的 JSON 時，請使用結構化輸出。

對於沒有工具使用的單個 API 調用，請參閱 [API 結構化輸出](/docs/zh-TW/build-with-claude/structured-outputs)。
</Note>

## 為什麼使用結構化輸出

結構化輸出為您的應用程序提供可靠的、類型安全的集成：

- **驗證的結構**：始終接收與您的架構匹配的有效 JSON
- **簡化的集成**：無需解析或驗證代碼
- **類型安全**：與 TypeScript 或 Python 類型提示一起使用以實現端到端安全
- **清晰的分離**：將輸出要求與任務指令分開定義
- **工具自主性**：代理選擇使用哪些工具，同時保證輸出格式

<Tabs>
<Tab title="TypeScript">

## 快速開始

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

## 使用 Zod 定義架構

對於 TypeScript 項目，使用 Zod 進行類型安全的架構定義和驗證：

```typescript
import { z } from 'zod'
import { zodToJsonSchema } from 'zod-to-json-schema'

// 使用 Zod 定義架構
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

// 轉換為 JSON Schema
const schema = zodToJsonSchema(AnalysisResult, { $refStrategy: 'root' })

// 在查詢中使用
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
    // 驗證並獲取完全類型化的結果
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

**Zod 的優點：**
- 完整的 TypeScript 類型推斷
- 使用 `safeParse()` 進行運行時驗證
- 更好的錯誤消息
- 可組合的架構

</Tab>
<Tab title="Python">

## 快速開始

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

## 使用 Pydantic 定義架構

對於 Python 項目，使用 Pydantic 進行類型安全的架構定義和驗證：

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

# 在查詢中使用
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
        # 驗證並獲取完全類型化的結果
        result = AnalysisResult.model_validate(message.structured_output)
        print(f"Score: {result.score}")
        print(f"Found {len(result.issues)} issues")
        for issue in result.issues:
            print(f"[{issue.severity}] {issue.file}: {issue.description}")
```

**Pydantic 的優點：**
- 完整的 Python 類型提示
- 使用 `model_validate()` 進行運行時驗證
- 更好的錯誤消息
- 數據類功能

</Tab>
</Tabs>

## 結構化輸出如何工作

<Steps>
  <Step title="定義您的 JSON 架構">
    創建一個 JSON Schema，描述您希望代理返回的結構。該架構使用標準 JSON Schema 格式。
  </Step>
  <Step title="添加 outputFormat 參數">
    在您的查詢選項中包含 `outputFormat` 參數，其中 `type: "json_schema"` 和您的架構定義。
  </Step>
  <Step title="運行您的查詢">
    代理使用完成任務所需的任何工具（文件操作、命令、網絡搜索等）。
  </Step>
  <Step title="訪問驗證的輸出">
    代理的最終結果將是與您的架構匹配的有效 JSON，可在 `message.structured_output` 中獲得。
  </Step>
</Steps>

## 支持的 JSON Schema 功能

Agent SDK 支持與 [API 結構化輸出](/docs/zh-TW/build-with-claude/structured-outputs#json-schema-limitations) 相同的 JSON Schema 功能和限制。

關鍵支持的功能：
- 所有基本類型：object、array、string、integer、number、boolean、null
- `enum`、`const`、`required`、`additionalProperties`（必須為 `false`）
- 字符串格式：`date-time`、`date`、`email`、`uri`、`uuid` 等
- `$ref`、`$def` 和 `definitions`

有關支持的功能、限制和正則表達式模式支持的完整詳細信息，請參閱 API 文檔中的 [JSON Schema 限制](/docs/zh-TW/build-with-claude/structured-outputs#json-schema-limitations)。

## 示例：TODO 跟蹤代理

以下是一個完整示例，展示了一個搜索代碼中的 TODO 並提取 git blame 信息的代理：

<CodeGroup>

```typescript TypeScript
import { query } from '@anthropic-ai/claude-agent-sdk'

// 定義 TODO 提取的結構
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

// 代理使用 Grep 查找 TODO，使用 Bash 獲取 git blame 信息
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

# 定義 TODO 提取的結構
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

# 代理使用 Grep 查找 TODO，使用 Bash 獲取 git blame 信息
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

代理自主使用正確的工具（Grep、Bash）來收集信息並返回驗證的數據。

## 錯誤處理

如果代理無法生成與您的架構匹配的有效輸出，您將收到錯誤結果：

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

## 相關資源

- [JSON Schema 文檔](https://json-schema.org/)
- [API 結構化輸出](/docs/zh-TW/build-with-claude/structured-outputs) - 用於單個 API 調用
- [自定義工具](/docs/zh-TW/agent-sdk/custom-tools) - 為您的代理定義工具
- [TypeScript SDK 參考](/docs/zh-TW/agent-sdk/typescript) - 完整的 TypeScript API
- [Python SDK 參考](/docs/zh-TW/agent-sdk/python) - 完整的 Python API