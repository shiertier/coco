# Expert en comparaisons

Générer des comparaisons à partir de descriptions de base.

---

> Copiez cette invite dans notre [Console](/dashboard) développeur pour l'essayer vous-même !

| Rôle | Contenu |
| ---- | --- |
| Utilisateur | Aidez-moi à créer quelques comparaisons pour décrire le rire d'une personne qui est joyeux et contagieux ? |

## Exemple de sortie

1.  Son rire bouillonnait comme une bouteille de champagne fraîchement ouverte, effervescent et plein de vie.
2.  Leur rire était aussi chaleureux et accueillant qu'une cheminée confortable par une soirée fraîche, attirant les gens pour partager la gaieté.

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
          "text": "Aidez-moi à créer quelques comparaisons pour décrire le rire d'une personne qui est joyeux et contagieux ?"
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
          "text": "Aidez-moi à créer quelques comparaisons pour décrire le rire d'une personne qui est joyeux et contagieux ?"
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
                    "text": "Aidez-moi à créer quelques comparaisons pour décrire le rire d'une personne qui est joyeux et contagieux ?"
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
          "text": "Aidez-moi à créer quelques comparaisons pour décrire le rire d'une personne qui est joyeux et contagieux ?"
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
                    "text": "Aidez-moi à créer quelques comparaisons pour décrire le rire d'une personne qui est joyeux et contagieux ?"
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
          "text": "Aidez-moi à créer quelques comparaisons pour décrire le rire d'une personne qui est joyeux et contagieux ?"
        }
      ]
    }
  ]
});
console.log(msg);

```

</Tab>
</Tabs>