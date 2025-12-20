# Encodeur d'emoji

Convertir du texte brut en messages emoji amusants et expressifs.

---

> Copiez cette invite dans notre [Console](/dashboard) développeur pour l'essayer vous-même !

|        | Contenu |
| --- | --- |
| System | Votre tâche est de prendre le message en texte brut fourni et de le convertir en un message expressif et riche en emojis qui transmet le même sens et la même intention. Remplacez les mots-clés et les phrases par des emojis pertinents le cas échéant pour ajouter un intérêt visuel et de l'émotion. Utilisez les emojis de manière créative mais assurez-vous que le message reste clair et facile à comprendre. Ne changez pas le message principal ou n'ajoutez pas de nouvelles informations. |
| User   | Le monde entier est une scène, et tous les hommes et femmes ne sont que des acteurs. Ils ont leurs sorties et leurs entrées ; Et un homme dans son temps joue plusieurs rôles. |

## Exemple de sortie

Le 🌍 entier est une 🎭, et tous les 👨 et 👩 ne sont que des 🎭🎬. Ils ont leurs 🚪🚶‍♂️ et leurs 🚶‍♀️🚪 ; Et un 👨 dans son ⌛ joue plusieurs 🎭.

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
    system="Votre tâche est de prendre le message en texte brut fourni et de le convertir en un message expressif et riche en emojis qui transmet le même sens et la même intention. Remplacez les mots-clés et les phrases par des emojis pertinents le cas échéant pour ajouter un intérêt visuel et de l'émotion. Utilisez les emojis de manière créative mais assurez-vous que le message reste clair et facile à comprendre. Ne changez pas le message principal ou n'ajoutez pas de nouvelles informations.",
    messages=[
        {
            "role": "user",
            "content": [
                {
                    "type": "text",
                    "text": "Le monde entier est une scène, et tous les hommes et femmes ne sont que des acteurs. Ils ont leurs sorties et leurs entrées ; Et un homme dans son temps joue plusieurs rôles.",
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
  system: "Votre tâche est de prendre le message en texte brut fourni et de le convertir en un message expressif et riche en emojis qui transmet le même sens et la même intention. Remplacez les mots-clés et les phrases par des emojis pertinents le cas échéant pour ajouter un intérêt visuel et de l'émotion. Utilisez les emojis de manière créative mais assurez-vous que le message reste clair et facile à comprendre. Ne changez pas le message principal ou n'ajoutez pas de nouvelles informations.",
  messages: [
    {
      "role": "user",
      "content": [
        {
          "type": "text",
          "text": "Le monde entier est une scène, et tous les hommes et femmes ne sont que des acteurs. Ils ont leurs sorties et leurs entrées ; Et un homme dans son temps joue plusieurs rôles."
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
    system="Votre tâche est de prendre le message en texte brut fourni et de le convertir en un message expressif et riche en emojis qui transmet le même sens et la même intention. Remplacez les mots-clés et les phrases par des emojis pertinents le cas échéant pour ajouter un intérêt visuel et de l'émotion. Utilisez les emojis de manière créative mais assurez-vous que le message reste clair et facile à comprendre. Ne changez pas le message principal ou n'ajoutez pas de nouvelles informations.",
    messages=[
        {
            "role": "user",
            "content": [
                {
                    "type": "text",
                    "text": "Le monde entier est une scène, et tous les hommes et femmes ne sont que des acteurs. Ils ont leurs sorties et leurs entrées ; Et un homme dans son temps joue plusieurs rôles."
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
  system: "Votre tâche est de prendre le message en texte brut fourni et de le convertir en un message expressif et riche en emojis qui transmet le même sens et la même intention. Remplacez les mots-clés et les phrases par des emojis pertinents le cas échéant pour ajouter un intérêt visuel et de l'émotion. Utilisez les emojis de manière créative mais assurez-vous que le message reste clair et facile à comprendre. Ne changez pas le message principal ou n'ajoutez pas de nouvelles informations.",
  messages: [
    {
      "role": "user",
      "content": [
        {
          "type": "text",
          "text": "Le monde entier est une scène, et tous les hommes et femmes ne sont que des acteurs. Ils ont leurs sorties et leurs entrées ; Et un homme dans son temps joue plusieurs rôles."
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
    system="Votre tâche est de prendre le message en texte brut fourni et de le convertir en un message expressif et riche en emojis qui transmet le même sens et la même intention. Remplacez les mots-clés et les phrases par des emojis pertinents le cas échéant pour ajouter un intérêt visuel et de l'émotion. Utilisez les emojis de manière créative mais assurez-vous que le message reste clair et facile à comprendre. Ne changez pas le message principal ou n'ajoutez pas de nouvelles informations.",
    messages=[
        {
            "role": "user",
            "content": [
                {
                    "type": "text",
                    "text": "Le monde entier est une scène, et tous les hommes et femmes ne sont que des acteurs. Ils ont leurs sorties et leurs entrées ; Et un homme dans son temps joue plusieurs rôles."
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
  system: "Votre tâche est de prendre le message en texte brut fourni et de le convertir en un message expressif et riche en emojis qui transmet le même sens et la même intention. Remplacez les mots-clés et les phrases par des emojis pertinents le cas échéant pour ajouter un intérêt visuel et de l'émotion. Utilisez les emojis de manière créative mais assurez-vous que le message reste clair et facile à comprendre. Ne changez pas le message principal ou n'ajoutez pas de nouvelles informations.",
  messages: [
    {
      "role": "user",
      "content": [
        {
          "type": "text",
          "text": "Le monde entier est une scène, et tous les hommes et femmes ne sont que des acteurs. Ils ont leurs sorties et leurs entrées ; Et un homme dans son temps joue plusieurs rôles."
        }
      ]
    }
  ]
});
console.log(msg);

```

</Tab>
</Tabs>