# Saídas estruturadas no SDK

Obtenha resultados JSON validados de fluxos de trabalho de agentes

---

Obtenha JSON estruturado e validado de fluxos de trabalho de agentes. O Agent SDK suporta saídas estruturadas através de JSON Schemas, garantindo que seus agentes retornem dados exatamente no formato que você precisa.

<Note>
**Quando usar saídas estruturadas**

Use saídas estruturadas quando você precisar de JSON validado após um agente completar um fluxo de trabalho multi-turno com ferramentas (buscas de arquivo, execução de comando, pesquisa na web, etc.).

Para chamadas de API única sem uso de ferramentas, consulte [Saídas Estruturadas de API](/docs/pt-BR/build-with-claude/structured-outputs).
</Note>

## Por que usar saídas estruturadas

Saídas estruturadas fornecem integração confiável e type-safe com suas aplicações:

- **Estrutura validada**: Sempre receba JSON válido correspondendo ao seu schema
- **Integração simplificada**: Nenhum código de análise ou validação necessário
- **Segurança de tipo**: Use com dicas de tipo TypeScript ou Python para segurança end-to-end
- **Separação limpa**: Defina requisitos de saída separadamente das instruções de tarefa
- **Autonomia de ferramentas**: O agente escolhe quais ferramentas usar enquanto garante o formato de saída

<Tabs>
<Tab title="TypeScript">

## Início rápido

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

## Definindo schemas com Zod

Para projetos TypeScript, use Zod para definição de schema type-safe e validação:

```typescript
import { z } from 'zod'
import { zodToJsonSchema } from 'zod-to-json-schema'

// Define schema com Zod
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

// Converta para JSON Schema
const schema = zodToJsonSchema(AnalysisResult, { $refStrategy: 'root' })

// Use em query
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
    // Valide e obtenha resultado totalmente tipado
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

**Benefícios do Zod:**
- Inferência de tipo TypeScript completa
- Validação em tempo de execução com `safeParse()`
- Mensagens de erro melhores
- Schemas compostos

</Tab>
<Tab title="Python">

## Início rápido

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

## Definindo schemas com Pydantic

Para projetos Python, use Pydantic para definição de schema type-safe e validação:

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

# Use em query
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
        # Valide e obtenha resultado totalmente tipado
        result = AnalysisResult.model_validate(message.structured_output)
        print(f"Score: {result.score}")
        print(f"Found {len(result.issues)} issues")
        for issue in result.issues:
            print(f"[{issue.severity}] {issue.file}: {issue.description}")
```

**Benefícios do Pydantic:**
- Dicas de tipo Python completas
- Validação em tempo de execução com `model_validate()`
- Mensagens de erro melhores
- Funcionalidade de classe de dados

</Tab>
</Tabs>

## Como funcionam as saídas estruturadas

<Steps>
  <Step title="Defina seu JSON schema">
    Crie um JSON Schema que descreva a estrutura que você quer que o agente retorne. O schema usa o formato padrão de JSON Schema.
  </Step>
  <Step title="Adicione o parâmetro outputFormat">
    Inclua o parâmetro `outputFormat` nas opções de sua query com `type: "json_schema"` e sua definição de schema.
  </Step>
  <Step title="Execute sua query">
    O agente usa qualquer ferramenta que precisar para completar a tarefa (operações de arquivo, comandos, pesquisa na web, etc.).
  </Step>
  <Step title="Acesse a saída validada">
    O resultado final do agente será JSON válido correspondendo ao seu schema, disponível em `message.structured_output`.
  </Step>
</Steps>

## Recursos de JSON Schema suportados

O Agent SDK suporta os mesmos recursos e limitações de JSON Schema que [Saídas Estruturadas de API](/docs/pt-BR/build-with-claude/structured-outputs#json-schema-limitations).

Recursos principais suportados:
- Todos os tipos básicos: object, array, string, integer, number, boolean, null
- `enum`, `const`, `required`, `additionalProperties` (deve ser `false`)
- Formatos de string: `date-time`, `date`, `email`, `uri`, `uuid`, etc.
- `$ref`, `$def`, e `definitions`

Para detalhes completos sobre recursos suportados, limitações e suporte a padrões regex, consulte [Limitações de JSON Schema](/docs/pt-BR/build-with-claude/structured-outputs#json-schema-limitations) na documentação da API.

## Exemplo: agente de rastreamento de TODO

Aqui está um exemplo completo mostrando um agente que pesquisa código para TODOs e extrai informações de git blame:

<CodeGroup>

```typescript TypeScript
import { query } from '@anthropic-ai/claude-agent-sdk'

// Defina estrutura para extração de TODO
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

// O agente usa Grep para encontrar TODOs, Bash para obter informações de git blame
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

# Defina estrutura para extração de TODO
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

# O agente usa Grep para encontrar TODOs, Bash para obter informações de git blame
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

O agente usa autonomamente as ferramentas certas (Grep, Bash) para reunir informações e retorna dados validados.

## Tratamento de erros

Se o agente não conseguir produzir saída válida correspondendo ao seu schema, você receberá um resultado de erro:

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

## Recursos relacionados

- [Documentação de JSON Schema](https://json-schema.org/)
- [Saídas Estruturadas de API](/docs/pt-BR/build-with-claude/structured-outputs) - Para chamadas de API única
- [Ferramentas personalizadas](/docs/pt-BR/agent-sdk/custom-tools) - Defina ferramentas para seus agentes
- [Referência do SDK TypeScript](/docs/pt-BR/agent-sdk/typescript) - API TypeScript completa
- [Referência do SDK Python](/docs/pt-BR/agent-sdk/python) - API Python completa