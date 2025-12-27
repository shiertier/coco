# Chasseur de bugs Python

Détecter et corriger les bugs dans le code Python.

---

> Copiez cette invite dans notre [Console](/dashboard) développeur pour l'essayer vous-même !

|        | Contenu |
| --- | --- |
| System | Votre tâche consiste à analyser l'extrait de code Python fourni, identifier tous les bugs ou erreurs présents, et fournir une version corrigée du code qui résout ces problèmes. Expliquez les problèmes que vous avez trouvés dans le code original et comment vos corrections les résolvent. Le code corrigé doit être fonctionnel, efficace et respecter les meilleures pratiques de programmation Python. |
| User   | def calculate_average(nums): <br/> sum = 0 <br/> for num in nums: <br/> sum += num <br/> average = sum / len(nums) <br/> return average <br/><br/> numbers = [10, 20, 30, 40, 50] <br/> result = calculate_average(numbers) <br/> print("The average is:", results) |

## Exemple de sortie

Code Python corrigé :

```python Python
def calculate_average(nums):
    total = 0
    for num in nums:
        total += num
    average = total / len(nums)
    return average

numbers = [10, 20, 30, 40, 50]
result = calculate_average(numbers)
print("The average is:", result)
```

Explication :

    1. Changé le nom de variable "sum" en "total" pour éviter d'utiliser la fonction Python intégrée "sum()".
    2. Corrigé la faute de frappe dans l'instruction print, en changeant "results" en "result" pour référencer correctement la variable.

## Requête API

<Tabs>
<Tab title="Python">

```python
import anthropic

client = anthropic.Anthropic(
  # defaults to os.environ.get("ANTHROPIC_API_KEY")
  api_key="my_api_key",
)
message = client.messages.create(
  model="claude-sonnet-4-5",
  max_tokens=1000,
  temperature=0,
  system="Votre tâche consiste à analyser l'extrait de code Python fourni, identifier tous les bugs ou erreurs présents, et fournir une version corrigée du code qui résout ces problèmes. Expliquez les problèmes que vous avez trouvés dans le code original et comment vos corrections les résolvent. Le code corrigé doit être fonctionnel, efficace et respecter les meilleures pratiques de programmation Python.",
  messages=[
    {
    "role": "user",
    "content": [
        {
          "type": "text",
          "text": "def calculate_average(nums):\n sum = 0\n for num in nums:\n sum += num\n average = sum / len(nums)\n return average\n\nnumbers = [10, 20, 30, 40, 50]\nresult = calculate_average(numbers)\nprint(\"The average is:\", results)"
        }
      ]
    }
  ]
)
print(message.content)

```

</Tab>
<Tab title="TypeScript">

```typescript
import Anthropic from "@anthropic-ai/sdk";

const anthropic = new Anthropic({
  apiKey: "my_api_key", // defaults to process.env["ANTHROPIC_API_KEY"]
});

const msg = await anthropic.messages.create({
  model: "claude-sonnet-4-5",
  max_tokens: 1000,
  temperature: 0,
  system: "Votre tâche consiste à analyser l'extrait de code Python fourni, identifier tous les bugs ou erreurs présents, et fournir une version corrigée du code qui résout ces problèmes. Expliquez les problèmes que vous avez trouvés dans le code original et comment vos corrections les résolvent. Le code corrigé doit être fonctionnel, efficace et respecter les meilleures pratiques de programmation Python.",
  messages: [
    {
      "role": "user",
      "content": [
        {
          "type": "text",
          "text": "def calculate_average(nums):\n   sum = 0\n   for num in nums:\n      sum += num\n   average = sum / len(nums)\n   return average\n\nnumbers = [10, 20, 30, 40, 50]\nresult = calculate_average(numbers)\nprint(\"The average is:\", results)"
        }
      ]
    }
  ]
});
console.log(msg);
````

  </Tab>
  <Tab title="AWS Bedrock Python">
```python
from anthropic import AnthropicBedrock

# See https://docs.claude.com/claude/reference/claude-on-amazon-bedrock

# for authentication options

client = AnthropicBedrock()

message = client.messages.create(
model="anthropic.claude-sonnet-4-5-20250929-v1:0",
max_tokens=1000,
temperature=0,
system="Votre tâche consiste à analyser l'extrait de code Python fourni, identifier tous les bugs ou erreurs présents, et fournir une version corrigée du code qui résout ces problèmes. Expliquez les problèmes que vous avez trouvés dans le code original et comment vos corrections les résolvent. Le code corrigé doit être fonctionnel, efficace et respecter les meilleures pratiques de programmation Python.",
messages=[
{
"role": "user",
"content": [
{
"type": "text",
"text": "def calculate_average(nums):\n sum = 0\n for num in nums:\n sum += num\n average = sum / len(nums)\n return average\n\nnumbers = [10, 20, 30, 40, 50]\nresult = calculate_average(numbers)\nprint(\"The average is:\", results)"
}
]
}
]
)
print(message.content)

````
  </Tab>
    <Tab title="AWS Bedrock TypeScript">
```typescript
import AnthropicBedrock from "@anthropic-ai/bedrock-sdk";

// See https://docs.claude.com/claude/reference/claude-on-amazon-bedrock
// for authentication options
const client = new AnthropicBedrock();

const msg = await client.messages.create({
  model: "anthropic.claude-sonnet-4-5-20250929-v1:0",
  max_tokens: 1000,
  temperature: 0,
  system: "Votre tâche consiste à analyser l'extrait de code Python fourni, identifier tous les bugs ou erreurs présents, et fournir une version corrigée du code qui résout ces problèmes. Expliquez les problèmes que vous avez trouvés dans le code original et comment vos corrections les résolvent. Le code corrigé doit être fonctionnel, efficace et respecter les meilleures pratiques de programmation Python.",
  messages: [
    {
      "role": "user",
      "content": [
        {
          "type": "text",
          "text": "def calculate_average(nums):\n   sum = 0\n   for num in nums:\n      sum += num\n   average = sum / len(nums)\n   return average\n\nnumbers = [10, 20, 30, 40, 50]\nresult = calculate_average(numbers)\nprint(\"The average is:\", results)"
        }
      ]
    }
  ]
});
console.log(msg);
````

  </Tab>
    <Tab title="Vertex AI Python">
```python
from anthropic import AnthropicVertex

client = AnthropicVertex()

message = client.messages.create(
model="claude-sonnet-4@20250514",
max_tokens=1000,
temperature=0,
system="Votre tâche consiste à analyser l'extrait de code Python fourni, identifier tous les bugs ou erreurs présents, et fournir une version corrigée du code qui résout ces problèmes. Expliquez les problèmes que vous avez trouvés dans le code original et comment vos corrections les résolvent. Le code corrigé doit être fonctionnel, efficace et respecter les meilleures pratiques de programmation Python.",
messages=[
{
"role": "user",
"content": [
{
"type": "text",
"text": "def calculate_average(nums):\n sum = 0\n for num in nums:\n sum += num\n average = sum / len(nums)\n return average\n\nnumbers = [10, 20, 30, 40, 50]\nresult = calculate_average(numbers)\nprint(\"The average is:\", results)"
}
]
}
]
)
print(message.content)

````
  </Tab>
    <Tab title="Vertex AI TypeScript">
```typescript
import { AnthropicVertex } from '@anthropic-ai/vertex-sdk';

// Reads from the `CLOUD_ML_REGION` & `ANTHROPIC_VERTEX_PROJECT_ID` environment variables.
// Additionally goes through the standard `google-auth-library` flow.
const client = new AnthropicVertex();

const msg = await client.messages.create({
  model: "claude-sonnet-4@20250514",
  max_tokens: 1000,
  temperature: 0,
  system: "Votre tâche consiste à analyser l'extrait de code Python fourni, identifier tous les bugs ou erreurs présents, et fournir une version corrigée du code qui résout ces problèmes. Expliquez les problèmes que vous avez trouvés dans le code original et comment vos corrections les résolvent. Le code corrigé doit être fonctionnel, efficace et respecter les meilleures pratiques de programmation Python.",
  messages: [
    {
      "role": "user",
      "content": [
        {
          "type": "text",
          "text": "def calculate_average(nums):\n   sum = 0\n   for num in nums:\n      sum += num\n   average = sum / len(nums)\n   return average\n\nnumbers = [10, 20, 30, 40, 50]\nresult = calculate_average(numbers)\nprint(\"The average is:\", results)"
        }
      ]
    }
  ]
});
console.log(msg);
````

  </Tab>
</Tabs>