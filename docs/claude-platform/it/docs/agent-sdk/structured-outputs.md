# Output strutturati nell'SDK

Ottieni risultati JSON convalidati dai flussi di lavoro degli agenti

---

Ottieni JSON strutturati e convalidati dai flussi di lavoro degli agenti. L'Agent SDK supporta output strutturati attraverso JSON Schemas, assicurando che i tuoi agenti restituiscano dati esattamente nel formato di cui hai bisogno.

<Note>
**Quando utilizzare output strutturati**

Utilizza output strutturati quando hai bisogno di JSON convalidato dopo che un agente completa un flusso di lavoro multi-turno con strumenti (ricerche di file, esecuzione di comandi, ricerca web, ecc.).

Per singole chiamate API senza utilizzo di strumenti, vedi [Output strutturati API](/docs/it/build-with-claude/structured-outputs).
</Note>

## Perché utilizzare output strutturati

Gli output strutturati forniscono un'integrazione affidabile e type-safe con le tue applicazioni:

- **Struttura convalidata**: Ricevi sempre JSON valido corrispondente al tuo schema
- **Integrazione semplificata**: Non è necessario codice di parsing o validazione
- **Type safety**: Utilizza con suggerimenti di tipo TypeScript o Python per la sicurezza end-to-end
- **Separazione pulita**: Definisci i requisiti di output separatamente dalle istruzioni del compito
- **Autonomia dello strumento**: L'agente sceglie quali strumenti utilizzare garantendo il formato di output

<Tabs>
<Tab title="TypeScript">

## Avvio rapido

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

## Definire schemi con Zod

Per i progetti TypeScript, utilizza Zod per la definizione dello schema type-safe e la validazione:

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

**Vantaggi di Zod:**
- Inferenza completa del tipo TypeScript
- Validazione runtime con `safeParse()`
- Messaggi di errore migliori
- Schemi componibili

</Tab>
<Tab title="Python">

## Avvio rapido

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

## Definire schemi con Pydantic

Per i progetti Python, utilizza Pydantic per la definizione dello schema type-safe e la validazione:

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

**Vantaggi di Pydantic:**
- Suggerimenti di tipo Python completi
- Validazione runtime con `model_validate()`
- Messaggi di errore migliori
- Funzionalità della classe di dati

</Tab>
</Tabs>

## Come funzionano gli output strutturati

<Steps>
  <Step title="Definisci il tuo schema JSON">
    Crea uno schema JSON che descriva la struttura che desideri che l'agente restituisca. Lo schema utilizza il formato standard JSON Schema.
  </Step>
  <Step title="Aggiungi il parametro outputFormat">
    Includi il parametro `outputFormat` nelle opzioni della query con `type: "json_schema"` e la definizione dello schema.
  </Step>
  <Step title="Esegui la query">
    L'agente utilizza gli strumenti di cui ha bisogno per completare il compito (operazioni su file, comandi, ricerca web, ecc.).
  </Step>
  <Step title="Accedi all'output convalidato">
    Il risultato finale dell'agente sarà JSON valido corrispondente al tuo schema, disponibile in `message.structured_output`.
  </Step>
</Steps>

## Funzionalità JSON Schema supportate

L'Agent SDK supporta le stesse funzionalità e limitazioni di JSON Schema di [API Structured Outputs](/docs/it/build-with-claude/structured-outputs#json-schema-limitations).

Funzionalità chiave supportate:
- Tutti i tipi di base: object, array, string, integer, number, boolean, null
- `enum`, `const`, `required`, `additionalProperties` (deve essere `false`)
- Formati di stringa: `date-time`, `date`, `email`, `uri`, `uuid`, ecc.
- `$ref`, `$def`, e `definitions`

Per i dettagli completi sulle funzionalità supportate, limitazioni e supporto dei pattern regex, vedi [Limitazioni JSON Schema](/docs/it/build-with-claude/structured-outputs#json-schema-limitations) nella documentazione dell'API.

## Esempio: agente di tracciamento TODO

Ecco un esempio completo che mostra un agente che cerca TODO nel codice ed estrae informazioni di git blame:

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

L'agente utilizza autonomamente gli strumenti giusti (Grep, Bash) per raccogliere informazioni e restituisce dati convalidati.

## Gestione degli errori

Se l'agente non riesce a produrre un output valido corrispondente al tuo schema, riceverai un risultato di errore:

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

## Risorse correlate

- [Documentazione JSON Schema](https://json-schema.org/)
- [API Structured Outputs](/docs/it/build-with-claude/structured-outputs) - Per singole chiamate API
- [Strumenti personalizzati](/docs/it/agent-sdk/custom-tools) - Definisci strumenti per i tuoi agenti
- [Riferimento SDK TypeScript](/docs/it/agent-sdk/typescript) - API TypeScript completa
- [Riferimento SDK Python](/docs/it/agent-sdk/python) - API Python completa