# Salidas estructuradas en el SDK

Obtén resultados JSON validados de flujos de trabajo de agentes

---

Obtén JSON estructurado y validado de flujos de trabajo de agentes. El SDK del Agente admite salidas estructuradas a través de Esquemas JSON, asegurando que tus agentes devuelvan datos exactamente en el formato que necesitas.

<Note>
**Cuándo usar salidas estructuradas**

Usa salidas estructuradas cuando necesites JSON validado después de que un agente complete un flujo de trabajo de múltiples turnos con herramientas (búsquedas de archivos, ejecución de comandos, investigación web, etc.).

Para llamadas de API únicas sin uso de herramientas, consulta [Salidas Estructuradas de API](/docs/es/build-with-claude/structured-outputs).
</Note>

## Por qué usar salidas estructuradas

Las salidas estructuradas proporcionan integración confiable y segura en tipos con tus aplicaciones:

- **Estructura validada**: Siempre recibe JSON válido que coincida con tu esquema
- **Integración simplificada**: No se necesita código de análisis o validación
- **Seguridad de tipos**: Úsalo con sugerencias de tipos de TypeScript o Python para seguridad de extremo a extremo
- **Separación limpia**: Define requisitos de salida separados de las instrucciones de tareas
- **Autonomía de herramientas**: El agente elige qué herramientas usar mientras garantiza el formato de salida

<Tabs>
<Tab title="TypeScript">

## Inicio rápido

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

## Definición de esquemas con Zod

Para proyectos de TypeScript, usa Zod para definición de esquema segura en tipos y validación:

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

**Beneficios de Zod:**
- Inferencia de tipos completa de TypeScript
- Validación en tiempo de ejecución con `safeParse()`
- Mejores mensajes de error
- Esquemas componibles

</Tab>
<Tab title="Python">

## Inicio rápido

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

## Definición de esquemas con Pydantic

Para proyectos de Python, usa Pydantic para definición de esquema segura en tipos y validación:

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

**Beneficios de Pydantic:**
- Sugerencias de tipos completas de Python
- Validación en tiempo de ejecución con `model_validate()`
- Mejores mensajes de error
- Funcionalidad de clase de datos

</Tab>
</Tabs>

## Cómo funcionan las salidas estructuradas

<Steps>
  <Step title="Define tu esquema JSON">
    Crea un Esquema JSON que describa la estructura que deseas que devuelva el agente. El esquema utiliza el formato estándar de Esquema JSON.
  </Step>
  <Step title="Añade el parámetro outputFormat">
    Incluye el parámetro `outputFormat` en tus opciones de consulta con `type: "json_schema"` y tu definición de esquema.
  </Step>
  <Step title="Ejecuta tu consulta">
    El agente utiliza cualquier herramienta que necesite para completar la tarea (operaciones de archivos, comandos, búsqueda web, etc.).
  </Step>
  <Step title="Accede a la salida validada">
    El resultado final del agente será JSON válido que coincida con tu esquema, disponible en `message.structured_output`.
  </Step>
</Steps>

## Características de Esquema JSON admitidas

El SDK del Agente admite las mismas características y limitaciones de Esquema JSON que [Salidas Estructuradas de API](/docs/es/build-with-claude/structured-outputs#json-schema-limitations).

Características clave admitidas:
- Todos los tipos básicos: object, array, string, integer, number, boolean, null
- `enum`, `const`, `required`, `additionalProperties` (debe ser `false`)
- Formatos de cadena: `date-time`, `date`, `email`, `uri`, `uuid`, etc.
- `$ref`, `$def`, y `definitions`

Para detalles completos sobre características admitidas, limitaciones y soporte de patrones regex, consulta [Limitaciones de Esquema JSON](/docs/es/build-with-claude/structured-outputs#json-schema-limitations) en la documentación de API.

## Ejemplo: Agente de seguimiento de TODO

Aquí hay un ejemplo completo que muestra un agente que busca TODOs en código y extrae información de git blame:

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

El agente utiliza autónomamente las herramientas correctas (Grep, Bash) para recopilar información y devuelve datos validados.

## Manejo de errores

Si el agente no puede producir una salida válida que coincida con tu esquema, recibirás un resultado de error:

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

- [Documentación de Esquema JSON](https://json-schema.org/)
- [Salidas Estructuradas de API](/docs/es/build-with-claude/structured-outputs) - Para llamadas de API únicas
- [Herramientas personalizadas](/docs/es/agent-sdk/custom-tools) - Define herramientas para tus agentes
- [Referencia del SDK de TypeScript](/docs/es/agent-sdk/typescript) - API completa de TypeScript
- [Referencia del SDK de Python](/docs/es/agent-sdk/python) - API completa de Python