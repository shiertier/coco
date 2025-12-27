# Simplificateur de niveau CE1

Rendre les textes complexes faciles à comprendre pour les jeunes apprenants.

---

> Copiez cette invite dans notre [Console](/dashboard) développeur pour l'essayer vous-même !

|        | Contenu |
| --- | --- |
| System | Votre tâche est de prendre le texte fourni et de le réécrire d'une manière qui soit facile à lire et à comprendre pour les jeunes apprenants du CE2 au CM2. Simplifiez le vocabulaire avancé, décomposez les phrases longues, expliquez les concepts difficiles en langage simple, et présentez l'information de manière claire et engageante. Le texte réécrit court doit transmettre les idées principales du texte original de manière appropriée à l'âge. |
| User   | Les mitochondries, souvent appelées les centrales électriques de la cellule, sont des organelles essentielles qui jouent un rôle crucial dans la production d'adénosine triphosphate (ATP) par le processus de respiration cellulaire. L'ATP est la principale monnaie énergétique de la cellule, permettant à diverses fonctions cellulaires et réactions biochimiques de se produire. |

## Exemple de sortie

Les mitochondries sont de minuscules parties à l'intérieur des cellules qui fonctionnent comme de petites centrales électriques. Elles fabriquent une molécule spéciale appelée ATP, qui est comme l'argent énergétique de la cellule. L'ATP aide la cellule à faire des travaux importants et la maintient en bon fonctionnement, tout comme l'argent aide les gens à acheter les choses dont ils ont besoin. Les mitochondries sont très importantes car sans elles, les cellules n'auraient pas l'énergie pour fonctionner correctement.

---

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
  system="Votre tâche est de prendre le texte fourni et de le réécrire d'une manière qui soit facile à lire et à comprendre pour les jeunes apprenants du CE2 au CM2. Simplifiez le vocabulaire avancé, décomposez les phrases longues, expliquez les concepts difficiles en langage simple, et présentez l'information de manière claire et engageante. Le texte réécrit court doit transmettre les idées principales du texte original de manière appropriée à l'âge.",
  messages=[
    {
      "role": "user",
      "content": [
        {
          "type": "text",
          "text": "Les mitochondries, souvent appelées les centrales électriques de la cellule, sont des organelles essentielles qui jouent un rôle crucial dans la production d'adénosine triphosphate (ATP) par le processus de respiration cellulaire. L'ATP est la principale monnaie énergétique de la cellule, permettant à diverses fonctions cellulaires et réactions biochimiques de se produire."
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
  system: "Votre tâche est de prendre le texte fourni et de le réécrire d'une manière qui soit facile à lire et à comprendre pour les jeunes apprenants du CE2 au CM2. Simplifiez le vocabulaire avancé, décomposez les phrases longues, expliquez les concepts difficiles en langage simple, et présentez l'information de manière claire et engageante. Le texte réécrit court doit transmettre les idées principales du texte original de manière appropriée à l'âge.",
  messages: [
    {
      "role": "user",
      "content": [
        {
          "type": "text",
          "text": "Les mitochondries, souvent appelées les centrales électriques de la cellule, sont des organelles essentielles qui jouent un rôle crucial dans la production d'adénosine triphosphate (ATP) par le processus de respiration cellulaire. L'ATP est la principale monnaie énergétique de la cellule, permettant à diverses fonctions cellulaires et réactions biochimiques de se produire."
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
    system="Votre tâche est de prendre le texte fourni et de le réécrire d'une manière qui soit facile à lire et à comprendre pour les jeunes apprenants du CE2 au CM2. Simplifiez le vocabulaire avancé, décomposez les phrases longues, expliquez les concepts difficiles en langage simple, et présentez l'information de manière claire et engageante. Le texte réécrit court doit transmettre les idées principales du texte original de manière appropriée à l'âge.",
    messages=[
        {
            "role": "user",
            "content": [
                {
                    "type": "text",
                    "text": "Les mitochondries, souvent appelées les centrales électriques de la cellule, sont des organelles essentielles qui jouent un rôle crucial dans la production d'adénosine triphosphate (ATP) par le processus de respiration cellulaire. L'ATP est la principale monnaie énergétique de la cellule, permettant à diverses fonctions cellulaires et réactions biochimiques de se produire."
                }
            ]
        }
    ]
)
print(message.content)

```

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
  system: "Votre tâche est de prendre le texte fourni et de le réécrire d'une manière qui soit facile à lire et à comprendre pour les jeunes apprenants du CE2 au CM2. Simplifiez le vocabulaire avancé, décomposez les phrases longues, expliquez les concepts difficiles en langage simple, et présentez l'information de manière claire et engageante. Le texte réécrit court doit transmettre les idées principales du texte original de manière appropriée à l'âge.",
  messages: [
    {
      "role": "user",
      "content": [
        {
          "type": "text",
          "text": "Les mitochondries, souvent appelées les centrales électriques de la cellule, sont des organelles essentielles qui jouent un rôle crucial dans la production d'adénosine triphosphate (ATP) par le processus de respiration cellulaire. L'ATP est la principale monnaie énergétique de la cellule, permettant à diverses fonctions cellulaires et réactions biochimiques de se produire."
        }
      ]
    }
  ]
});
console.log(msg);

```

</Tab>
<Tab title="Vertex AI Python">
```python
from anthropic import AnthropicVertex

client = AnthropicVertex()

message = client.messages.create(
model="claude-sonnet-4@20250514",
max_tokens=1000,
temperature=0,
system="Votre tâche est de prendre le texte fourni et de le réécrire d'une manière qui soit facile à lire et à comprendre pour les jeunes apprenants du CE2 au CM2. Simplifiez le vocabulaire avancé, décomposez les phrases longues, expliquez les concepts difficiles en langage simple, et présentez l'information de manière claire et engageante. Le texte réécrit court doit transmettre les idées principales du texte original de manière appropriée à l'âge.",
messages=[
{
"role": "user",
"content": [
{
"type": "text",
"text": "Les mitochondries, souvent appelées les centrales électriques de la cellule, sont des organelles essentielles qui jouent un rôle crucial dans la production d'adénosine triphosphate (ATP) par le processus de respiration cellulaire. L'ATP est la principale monnaie énergétique de la cellule, permettant à diverses fonctions cellulaires et réactions biochimiques de se produire."
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
  system: "Votre tâche est de prendre le texte fourni et de le réécrire d'une manière qui soit facile à lire et à comprendre pour les jeunes apprenants du CE2 au CM2. Simplifiez le vocabulaire avancé, décomposez les phrases longues, expliquez les concepts difficiles en langage simple, et présentez l'information de manière claire et engageante. Le texte réécrit court doit transmettre les idées principales du texte original de manière appropriée à l'âge.",
  messages: [
    {
      "role": "user",
      "content": [
        {
          "type": "text",
          "text": "Les mitochondries, souvent appelées les centrales électriques de la cellule, sont des organelles essentielles qui jouent un rôle crucial dans la production d'adénosine triphosphate (ATP) par le processus de respiration cellulaire. L'ATP est la principale monnaie énergétique de la cellule, permettant à diverses fonctions cellulaires et réactions biochimiques de se produire."
        }
      ]
    }
  ]
});
console.log(msg);

````

</Tab>
</Tabs>