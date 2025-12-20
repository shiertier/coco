# Devine un peu

Générer des énigmes et guider l'utilisateur vers les solutions.

---

> Copiez cette invite dans notre [Console](/dashboard) développeur pour l'essayer vous-même !

| Rôle | Contenu |
| ---- | --- |
| Utilisateur | Générez une énigme astucieuse et fournissez un guide étape par étape pour aider l'utilisateur à arriver aux bonnes solutions. L'énigme doit être difficile mais résolvable avec une réflexion logique et une attention aux détails. Après avoir présenté chaque énigme, offrez un ensemble d'indices ou de questions qui mènent progressivement l'utilisateur vers la réponse. Assurez-vous que les indices ne sont pas trop évidents mais fournissent encore suffisamment d'informations pour guider le processus de réflexion de l'utilisateur. Enfin, révélez la solution et fournissez une brève explication de comment l'énigme peut être résolue en utilisant les indices donnés. |

## Exemple de sortie

Je ne suis pas vivant, mais je grandis ; je n'ai pas de poumons, mais j'ai besoin d'air ; je n'ai pas de bouche, mais l'eau me tue. Que suis-je ?

Indice 1 : Considérez les différents états de la matière et comment ils peuvent changer.  
Indice 2 : Pensez à la relation entre les trois éléments mentionnés : l'air, l'eau, et l'objet mystérieux.  
Indice 3 : La réponse est liée à un phénomène commun qui se produit dans la vie quotidienne.

Solution : Le feu.  
Explication : Le feu n'est pas vivant, mais il grandit et se propage en consommant du combustible. Il n'a pas de poumons, mais il nécessite de l'oxygène (air) pour se maintenir. Bien qu'il n'ait pas de bouche, l'eau est souvent utilisée pour éteindre les feux, les "tuant" effectivement.

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
  messages=[
    {
      "role": "user",
      "content": [
        {
          "type": "text",
          "text": "Générez une énigme astucieuse et fournissez un guide étape par étape pour aider l'utilisateur à arriver aux bonnes solutions. L'énigme doit être difficile mais résolvable avec une réflexion logique et une attention aux détails. Après avoir présenté chaque énigme, offrez un ensemble d'indices ou de questions qui mènent progressivement l'utilisateur vers la réponse. Assurez-vous que les indices ne sont pas trop évidents mais fournissent encore suffisamment d'informations pour guider le processus de réflexion de l'utilisateur. Enfin, révélez la solution et fournissez une brève explication de comment l'énigme peut être résolue en utilisant les indices donnés."
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
  messages: [
    {
      "role": "user",
      "content": [
        {
          "type": "text",
          "text": "Générez une énigme astucieuse et fournissez un guide étape par étape pour aider l'utilisateur à arriver aux bonnes solutions. L'énigme doit être difficile mais résolvable avec une réflexion logique et une attention aux détails. Après avoir présenté chaque énigme, offrez un ensemble d'indices ou de questions qui mènent progressivement l'utilisateur vers la réponse. Assurez-vous que les indices ne sont pas trop évidents mais fournissent encore suffisamment d'informations pour guider le processus de réflexion de l'utilisateur. Enfin, révélez la solution et fournissez une brève explication de comment l'énigme peut être résolue en utilisant les indices donnés."
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
    messages=[
        {
            "role": "user",
            "content": [
                {
                    "type": "text",
                    "text": "Générez une énigme astucieuse et fournissez un guide étape par étape pour aider l'utilisateur à arriver aux bonnes solutions. L'énigme doit être difficile mais résolvable avec une réflexion logique et une attention aux détails. Après avoir présenté chaque énigme, offrez un ensemble d'indices ou de questions qui mènent progressivement l'utilisateur vers la réponse. Assurez-vous que les indices ne sont pas trop évidents mais fournissent encore suffisamment d'informations pour guider le processus de réflexion de l'utilisateur. Enfin, révélez la solution et fournissez une brève explication de comment l'énigme peut être résolue en utilisant les indices donnés."
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
  messages: [
    {
      "role": "user",
      "content": [
        {
          "type": "text",
          "text": "Générez une énigme astucieuse et fournissez un guide étape par étape pour aider l'utilisateur à arriver aux bonnes solutions. L'énigme doit être difficile mais résolvable avec une réflexion logique et une attention aux détails. Après avoir présenté chaque énigme, offrez un ensemble d'indices ou de questions qui mènent progressivement l'utilisateur vers la réponse. Assurez-vous que les indices ne sont pas trop évidents mais fournissent encore suffisamment d'informations pour guider le processus de réflexion de l'utilisateur. Enfin, révélez la solution et fournissez une brève explication de comment l'énigme peut être résolue en utilisant les indices donnés."
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
    messages=[
        {
            "role": "user",
            "content": [
                {
                    "type": "text",
                    "text": "Générez une énigme astucieuse et fournissez un guide étape par étape pour aider l'utilisateur à arriver aux bonnes solutions. L'énigme doit être difficile mais résolvable avec une réflexion logique et une attention aux détails. Après avoir présenté chaque énigme, offrez un ensemble d'indices ou de questions qui mènent progressivement l'utilisateur vers la réponse. Assurez-vous que les indices ne sont pas trop évidents mais fournissent encore suffisamment d'informations pour guider le processus de réflexion de l'utilisateur. Enfin, révélez la solution et fournissez une brève explication de comment l'énigme peut être résolue en utilisant les indices donnés."
                }
            ]
        }
    ]
)
print(message.content)

```

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
  messages: [
    {
      "role": "user",
      "content": [
        {
          "type": "text",
          "text": "Générez une énigme astucieuse et fournissez un guide étape par étape pour aider l'utilisateur à arriver aux bonnes solutions. L'énigme doit être difficile mais résolvable avec une réflexion logique et une attention aux détails. Après avoir présenté chaque énigme, offrez un ensemble d'indices ou de questions qui mènent progressivement l'utilisateur vers la réponse. Assurez-vous que les indices ne sont pas trop évidents mais fournissent encore suffisamment d'informations pour guider le processus de réflexion de l'utilisateur. Enfin, révélez la solution et fournissez une brève explication de comment l'énigme peut être résolue en utilisant les indices donnés."
        }
      ]
    }
  ]
});
console.log(msg);

```

</Tab>
</Tabs>