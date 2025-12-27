# Créateur de néologismes

Inventez de nouveaux mots et fournissez leurs définitions basées sur des concepts ou idées fournis par l'utilisateur.

---

> Copiez cette invite dans notre [Console](/dashboard) développeur pour l'essayer vous-même !

|      | Contenu |
| ---- | --- |
| Utilisateur | Pouvez-vous m'aider à créer un nouveau mot pour l'acte de prétendre comprendre quelque chose afin d'éviter de paraître ignorant ou mal informé ? |

## Exemple de sortie

Que diriez-vous du néologisme "fauxpréhension" pour décrire ce concept ?

Fauxpréhension (fəʊprɪˈhɛnʃ(ə)n) - nom  
 Définition : L'acte de feindre la compréhension ou la compréhension d'un sujet, concept, ou situation afin d'éviter de paraître ignorant, mal informé, ou déconnecté. C'est la prétention de connaissance ou de maîtrise d'un sujet, souvent employée dans des contextes sociaux ou professionnels pour maintenir son image ou son statut.

Étymologie : Le mot "fauxpréhension" est un mélange de "faux," qui vient du mot français signifiant faux ou factice, et "compréhension," qui signifie l'action ou la capacité de comprendre quelque chose.

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
  temperature=1,
  messages=[
    {
      "role": "user",
      "content": [
        {
          "type": "text",
          "text": "Pouvez-vous m'aider à créer un nouveau mot pour l'acte de prétendre comprendre quelque chose afin d'éviter de paraître ignorant ou mal informé ?"
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
  temperature: 1,
  messages: [
    {
      "role": "user",
      "content": [
        {
          "type": "text",
          "text": "Pouvez-vous m'aider à créer un nouveau mot pour l'acte de prétendre comprendre quelque chose afin d'éviter de paraître ignorant ou mal informé ?"
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
    temperature=1,
    messages=[
        {
            "role": "user",
            "content": [
                {
                    "type": "text",
                    "text": "Pouvez-vous m'aider à créer un nouveau mot pour l'acte de prétendre comprendre quelque chose afin d'éviter de paraître ignorant ou mal informé ?"
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
  temperature: 1,
  messages: [
    {
      "role": "user",
      "content": [
        {
          "type": "text",
          "text": "Pouvez-vous m'aider à créer un nouveau mot pour l'acte de prétendre comprendre quelque chose afin d'éviter de paraître ignorant ou mal informé ?"
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
    temperature=1,
    messages=[
        {
            "role": "user",
            "content": [
                {
                    "type": "text",
                    "text": "Pouvez-vous m'aider à créer un nouveau mot pour l'acte de prétendre comprendre quelque chose afin d'éviter de paraître ignorant ou mal informé ?"
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
  temperature: 1,
  messages: [
    {
      "role": "user",
      "content": [
        {
          "type": "text",
          "text": "Pouvez-vous m'aider à créer un nouveau mot pour l'acte de prétendre comprendre quelque chose afin d'éviter de paraître ignorant ou mal informé ?"
        }
      ]
    }
  ]
});
console.log(msg);

```

</Tab>
</Tabs>