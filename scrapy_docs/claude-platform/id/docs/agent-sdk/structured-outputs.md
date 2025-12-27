# Output terstruktur dalam SDK

Dapatkan hasil JSON yang divalidasi dari alur kerja agen

---

Dapatkan JSON terstruktur dan tervalidasi dari alur kerja agen. Agent SDK mendukung output terstruktur melalui JSON Schemas, memastikan agen Anda mengembalikan data dalam format yang tepat sesuai kebutuhan Anda.

<Note>
**Kapan menggunakan output terstruktur**

Gunakan output terstruktur ketika Anda memerlukan JSON yang divalidasi setelah agen menyelesaikan alur kerja multi-turn dengan tools (pencarian file, eksekusi perintah, penelusuran web, dll.).

Untuk panggilan API tunggal tanpa penggunaan tool, lihat [API Structured Outputs](/docs/id/build-with-claude/structured-outputs).
</Note>

## Mengapa menggunakan output terstruktur

Output terstruktur menyediakan integrasi yang andal dan type-safe dengan aplikasi Anda:

- **Struktur yang divalidasi**: Selalu terima JSON yang valid sesuai dengan skema Anda
- **Integrasi yang disederhanakan**: Tidak perlu kode parsing atau validasi
- **Keamanan tipe**: Gunakan dengan petunjuk tipe TypeScript atau Python untuk keamanan end-to-end
- **Pemisahan yang bersih**: Tentukan persyaratan output secara terpisah dari instruksi tugas
- **Otonomi tool**: Agen memilih tool mana yang akan digunakan sambil menjamin format output

<Tabs>
<Tab title="TypeScript">

## Mulai cepat

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

## Mendefinisikan skema dengan Zod

Untuk proyek TypeScript, gunakan Zod untuk definisi skema yang type-safe dan validasi:

```typescript
import { z } from 'zod'
import { zodToJsonSchema } from 'zod-to-json-schema'

// Define schema with Zod
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

// Convert to JSON Schema
const schema = zodToJsonSchema(AnalysisResult, { $refStrategy: 'root' })

// Use in query
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
    // Validate and get fully typed result
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

**Manfaat Zod:**
- Inferensi tipe TypeScript penuh
- Validasi runtime dengan `safeParse()`
- Pesan kesalahan yang lebih baik
- Skema yang dapat dikomposisi

</Tab>
<Tab title="Python">

## Mulai cepat

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

## Mendefinisikan skema dengan Pydantic

Untuk proyek Python, gunakan Pydantic untuk definisi skema yang type-safe dan validasi:

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

# Use in query
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
        # Validate and get fully typed result
        result = AnalysisResult.model_validate(message.structured_output)
        print(f"Score: {result.score}")
        print(f"Found {len(result.issues)} issues")
        for issue in result.issues:
            print(f"[{issue.severity}] {issue.file}: {issue.description}")
```

**Manfaat Pydantic:**
- Petunjuk tipe Python penuh
- Validasi runtime dengan `model_validate()`
- Pesan kesalahan yang lebih baik
- Fungsionalitas data class

</Tab>
</Tabs>

## Cara kerja output terstruktur

<Steps>
  <Step title="Tentukan skema JSON Anda">
    Buat JSON Schema yang mendeskripsikan struktur yang ingin dikembalikan oleh agen. Skema menggunakan format JSON Schema standar.
  </Step>
  <Step title="Tambahkan parameter outputFormat">
    Sertakan parameter `outputFormat` dalam opsi query Anda dengan `type: "json_schema"` dan definisi skema Anda.
  </Step>
  <Step title="Jalankan query Anda">
    Agen menggunakan tool apa pun yang diperlukan untuk menyelesaikan tugas (operasi file, perintah, pencarian web, dll.).
  </Step>
  <Step title="Akses output yang divalidasi">
    Hasil akhir agen akan berupa JSON yang valid sesuai dengan skema Anda, tersedia di `message.structured_output`.
  </Step>
</Steps>

## Fitur JSON Schema yang didukung

Agent SDK mendukung fitur dan batasan JSON Schema yang sama seperti [API Structured Outputs](/docs/id/build-with-claude/structured-outputs#json-schema-limitations).

Fitur utama yang didukung:
- Semua tipe dasar: object, array, string, integer, number, boolean, null
- `enum`, `const`, `required`, `additionalProperties` (harus `false`)
- Format string: `date-time`, `date`, `email`, `uri`, `uuid`, dll.
- `$ref`, `$def`, dan `definitions`

Untuk detail lengkap tentang fitur yang didukung, batasan, dan dukungan pola regex, lihat [JSON Schema limitations](/docs/id/build-with-claude/structured-outputs#json-schema-limitations) dalam dokumentasi API.

## Contoh: Agen pelacakan TODO

Berikut adalah contoh lengkap yang menunjukkan agen yang mencari TODO dalam kode dan mengekstrak informasi git blame:

<CodeGroup>

```typescript TypeScript
import { query } from '@anthropic-ai/claude-agent-sdk'

// Define structure for TODO extraction
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

// Agent uses Grep to find TODOs, Bash to get git blame info
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

# Define structure for TODO extraction
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

# Agent uses Grep to find TODOs, Bash to get git blame info
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

Agen secara otonomi menggunakan tool yang tepat (Grep, Bash) untuk mengumpulkan informasi dan mengembalikan data yang divalidasi.

## Penanganan kesalahan

Jika agen tidak dapat menghasilkan output yang valid sesuai dengan skema Anda, Anda akan menerima hasil kesalahan:

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

## Sumber daya terkait

- [JSON Schema documentation](https://json-schema.org/)
- [API Structured Outputs](/docs/id/build-with-claude/structured-outputs) - Untuk panggilan API tunggal
- [Custom tools](/docs/id/agent-sdk/custom-tools) - Tentukan tool untuk agen Anda
- [TypeScript SDK reference](/docs/id/agent-sdk/typescript) - Referensi API TypeScript lengkap
- [Python SDK reference](/docs/id/agent-sdk/python) - Referensi API Python lengkap