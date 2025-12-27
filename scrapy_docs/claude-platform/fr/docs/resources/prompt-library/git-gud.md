# Git gud

Générer des commandes Git appropriées basées sur les actions de contrôle de version décrites par l'utilisateur.

---

> Copiez cette invite dans notre [Console](/dashboard) développeur pour l'essayer vous-même !

| Rôle | Contenu |
| ---- | --- |
| Utilisateur | J'ai apporté quelques modifications à mes fichiers locaux et je veux les sauvegarder dans mon dépôt Git local. Quelle commande Git dois-je utiliser ? |

## Exemple de Sortie

Pour sauvegarder les modifications que vous avez apportées à vos fichiers locaux dans votre dépôt Git local, vous devriez utiliser les commandes Git suivantes :

1.  `git add .` ou `git add <fichier>`

    - Cette commande met en scène les modifications que vous avez apportées, les préparant à être commitées. Le `.` ajoute toutes les modifications dans le répertoire courant et ses sous-répertoires. Si vous voulez seulement mettre en scène des fichiers spécifiques, remplacez `.` par le nom ou le chemin du fichier.

2.  `git commit -m "Votre message de commit"`

    - Cette commande commite les modifications mises en scène dans votre dépôt Git local. Le drapeau `-m` vous permet de fournir un message de commit décrivant les modifications que vous avez apportées. Remplacez `"Votre message de commit"` par un résumé bref et descriptif de vos modifications.

Par exemple, si vous avez apporté des modifications à un fichier nommé `example.txt`, vous exécuteriez :

```
git add example.txt
git commit -m "Mettre à jour example.txt avec du nouveau contenu"

```

Ces commandes sauvegarderont vos modifications dans votre dépôt Git local, créant un nouveau commit avec le message fourni. Vous pouvez plus tard pousser ces modifications vers un dépôt distant ou continuer à travailler sur votre branche locale.

N'oubliez pas d'exécuter ces commandes depuis le répertoire de votre dépôt Git dans votre terminal ou invite de commande.

---

## Requête API

<Tabs>
<Tab title="Python">
```python
import anthropic

client = anthropic.Anthropic(  # defaults to os.environ.get("ANTHROPIC_API_KEY")
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
                    "text": "J'ai apporté quelques modifications à mes fichiers locaux et je veux les sauvegarder dans mon dépôt Git local. Quelle commande Git dois-je utiliser ?",
                }
            ],
        }
    ],
)
print(message.content)


````
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
          "text": "J'ai apporté quelques modifications à mes fichiers locaux et je veux les sauvegarder dans mon dépôt Git local. Quelle commande Git dois-je utiliser ?"
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
                    "text": "J'ai apporté quelques modifications à mes fichiers locaux et je veux les sauvegarder dans mon dépôt Git local. Quelle commande Git dois-je utiliser ?"
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
          "text": "J'ai apporté quelques modifications à mes fichiers locaux et je veux les sauvegarder dans mon dépôt Git local. Quelle commande Git dois-je utiliser ?"
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
                    "text": "J'ai apporté quelques modifications à mes fichiers locaux et je veux les sauvegarder dans mon dépôt Git local. Quelle commande Git dois-je utiliser ?"
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
          "text": "J'ai apporté quelques modifications à mes fichiers locaux et je veux les sauvegarder dans mon dépôt Git local. Quelle commande Git dois-je utiliser ?"
        }
      ]
    }
  ]
});
console.log(msg);

```

</Tab>
</Tabs>