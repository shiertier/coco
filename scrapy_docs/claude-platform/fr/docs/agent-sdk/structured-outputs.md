# Sorties structurées dans le SDK

Obtenez des résultats JSON validés à partir de workflows d'agents

---

Obtenez du JSON structuré et validé à partir de workflows d'agents. Le SDK Agent prend en charge les sorties structurées via des schémas JSON, garantissant que vos agents retournent des données exactement au format dont vous avez besoin.

<Note>
**Quand utiliser les sorties structurées**

Utilisez les sorties structurées lorsque vous avez besoin de JSON validé après qu'un agent ait complété un workflow multi-tour avec des outils (recherches de fichiers, exécution de commandes, recherche web, etc.).

Pour les appels API uniques sans utilisation d'outils, voir [Sorties structurées API](/docs/fr/build-with-claude/structured-outputs).
</Note>

## Pourquoi utiliser les sorties structurées

Les sorties structurées offrent une intégration fiable et type-safe avec vos applications :

- **Structure validée** : Recevez toujours du JSON valide correspondant à votre schéma
- **Intégration simplifiée** : Aucun code d'analyse ou de validation nécessaire
- **Sécurité des types** : Utilisez avec les indices de type TypeScript ou Python pour une sécurité de bout en bout
- **Séparation claire** : Définissez les exigences de sortie séparément des instructions de tâche
- **Autonomie des outils** : L'agent choisit les outils à utiliser tout en garantissant le format de sortie

<Tabs>
<Tab title="TypeScript">

## Démarrage rapide

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

## Définir des schémas avec Zod

Pour les projets TypeScript, utilisez Zod pour une définition et une validation de schéma type-safe :

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

**Avantages de Zod :**
- Inférence de type TypeScript complète
- Validation à l'exécution avec `safeParse()`
- Meilleurs messages d'erreur
- Schémas composables

</Tab>
<Tab title="Python">

## Démarrage rapide

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

## Définir des schémas avec Pydantic

Pour les projets Python, utilisez Pydantic pour une définition et une validation de schéma type-safe :

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

**Avantages de Pydantic :**
- Indices de type Python complets
- Validation à l'exécution avec `model_validate()`
- Meilleurs messages d'erreur
- Fonctionnalité de classe de données

</Tab>
</Tabs>

## Comment fonctionnent les sorties structurées

<Steps>
  <Step title="Définir votre schéma JSON">
    Créez un schéma JSON qui décrit la structure que vous voulez que l'agent retourne. Le schéma utilise le format standard JSON Schema.
  </Step>
  <Step title="Ajouter le paramètre outputFormat">
    Incluez le paramètre `outputFormat` dans vos options de requête avec `type: "json_schema"` et votre définition de schéma.
  </Step>
  <Step title="Exécuter votre requête">
    L'agent utilise tous les outils dont il a besoin pour compléter la tâche (opérations sur fichiers, commandes, recherche web, etc.).
  </Step>
  <Step title="Accéder à la sortie validée">
    Le résultat final de l'agent sera du JSON valide correspondant à votre schéma, disponible dans `message.structured_output`.
  </Step>
</Steps>

## Fonctionnalités JSON Schema prises en charge

Le SDK Agent prend en charge les mêmes fonctionnalités et limitations de JSON Schema que [Sorties structurées API](/docs/fr/build-with-claude/structured-outputs#json-schema-limitations).

Fonctionnalités clés prises en charge :
- Tous les types de base : object, array, string, integer, number, boolean, null
- `enum`, `const`, `required`, `additionalProperties` (doit être `false`)
- Formats de chaîne : `date-time`, `date`, `email`, `uri`, `uuid`, etc.
- `$ref`, `$def`, et `definitions`

Pour des détails complets sur les fonctionnalités prises en charge, les limitations et le support des motifs regex, voir [Limitations JSON Schema](/docs/fr/build-with-claude/structured-outputs#json-schema-limitations) dans la documentation de l'API.

## Exemple : Agent de suivi des TODOs

Voici un exemple complet montrant un agent qui recherche des TODOs dans le code et extrait les informations de git blame :

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

L'agent utilise de manière autonome les bons outils (Grep, Bash) pour rassembler les informations et retourne des données validées.

## Gestion des erreurs

Si l'agent ne peut pas produire une sortie valide correspondant à votre schéma, vous recevrez un résultat d'erreur :

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

## Ressources connexes

- [Documentation JSON Schema](https://json-schema.org/)
- [Sorties structurées API](/docs/fr/build-with-claude/structured-outputs) - Pour les appels API uniques
- [Outils personnalisés](/docs/fr/agent-sdk/custom-tools) - Définir des outils pour vos agents
- [Référence SDK TypeScript](/docs/fr/agent-sdk/typescript) - API TypeScript complète
- [Référence SDK Python](/docs/fr/agent-sdk/python) - API Python complète