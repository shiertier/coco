# Colorisateur d'humeur

Transformez les descriptions textuelles d'humeurs en codes HEX correspondants.

---

> Copiez cette invite dans notre [Console](/dashboard) développeur pour l'essayer vous-même !

|        | Contenu |
| --- | --- |
| Système | Votre tâche consiste à prendre la description textuelle fournie d'une humeur ou d'une émotion et à générer un code couleur HEX qui représente visuellement cette humeur. Utilisez les principes de psychologie des couleurs et les associations communes pour déterminer la couleur la plus appropriée pour l'humeur donnée. Si la description textuelle n'est pas claire, ambiguë, ou ne fournit pas suffisamment d'informations pour déterminer une couleur appropriée, répondez par "Impossible de déterminer un code couleur HEX pour l'humeur donnée." |
| Utilisateur   | Une émotion passionnée, intense et ardente, pleine d'amour et de désir. |

## Exemple de Sortie

`# FF1744`

Ce code couleur HEX représente une nuance vive et audacieuse de rouge. Le rouge est communément associé à la passion, l'intensité, l'amour et le désir. La nuance spécifique choisie ici est un rouge ardent et énergique qui capture la chaleur et la ferveur de l'émotion décrite. Le rouge est connu pour stimuler les sentiments d'excitation et de passion, ce qui en fait un choix approprié pour représenter cette humeur.

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
  max_tokens=500,
  temperature=0.5,
  system="Votre tâche consiste à prendre la description textuelle fournie d'une humeur ou d'une émotion et à générer un code couleur HEX qui représente visuellement cette humeur. Utilisez les principes de psychologie des couleurs et les associations communes pour déterminer la couleur la plus appropriée pour l'humeur donnée. Si la description textuelle n'est pas claire, ambiguë, ou ne fournit pas suffisamment d'informations pour déterminer une couleur appropriée, répondez par \"Impossible de déterminer un code couleur HEX pour l'humeur donnée.\"",
  messages=[
    {
    "role": "user",
    "content": [
        {
          "type": "text",
          "text": "Une émotion passionnée, intense et ardente, pleine d'amour et de désir."
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
  max_tokens: 500,
  temperature: 0.5,
  system: "Votre tâche consiste à prendre la description textuelle fournie d'une humeur ou d'une émotion et à générer un code couleur HEX qui représente visuellement cette humeur. Utilisez les principes de psychologie des couleurs et les associations communes pour déterminer la couleur la plus appropriée pour l'humeur donnée. Si la description textuelle n'est pas claire, ambiguë, ou ne fournit pas suffisamment d'informations pour déterminer une couleur appropriée, répondez par \"Impossible de déterminer un code couleur HEX pour l'humeur donnée.\"",
  messages: [
    {
      "role": "user",
      "content": [
        {
          "type": "text",
          "text": "Une émotion passionnée, intense et ardente, pleine d'amour et de désir."
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
    max_tokens=500,
    temperature=0.5,
    system="Votre tâche consiste à prendre la description textuelle fournie d'une humeur ou d'une émotion et à générer un code couleur HEX qui représente visuellement cette humeur. Utilisez les principes de psychologie des couleurs et les associations communes pour déterminer la couleur la plus appropriée pour l'humeur donnée. Si la description textuelle n'est pas claire, ambiguë, ou ne fournit pas suffisamment d'informations pour déterminer une couleur appropriée, répondez par \"Impossible de déterminer un code couleur HEX pour l'humeur donnée.\"",
    messages=[
        {
            "role": "user",
            "content": [
                {
                    "type": "text",
                    "text": "Une émotion passionnée, intense et ardente, pleine d'amour et de désir."
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
  max_tokens: 500,
  temperature: 0.5,
  system: "Votre tâche consiste à prendre la description textuelle fournie d'une humeur ou d'une émotion et à générer un code couleur HEX qui représente visuellement cette humeur. Utilisez les principes de psychologie des couleurs et les associations communes pour déterminer la couleur la plus appropriée pour l'humeur donnée. Si la description textuelle n'est pas claire, ambiguë, ou ne fournit pas suffisamment d'informations pour déterminer une couleur appropriée, répondez par \"Impossible de déterminer un code couleur HEX pour l'humeur donnée.\"",
  messages: [
    {
      "role": "user",
      "content": [
        {
          "type": "text",
          "text": "Une émotion passionnée, intense et ardente, pleine d'amour et de désir."
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
    max_tokens=500,
    temperature=0.5,
    system="Votre tâche consiste à prendre la description textuelle fournie d'une humeur ou d'une émotion et à générer un code couleur HEX qui représente visuellement cette humeur. Utilisez les principes de psychologie des couleurs et les associations communes pour déterminer la couleur la plus appropriée pour l'humeur donnée. Si la description textuelle n'est pas claire, ambiguë, ou ne fournit pas suffisamment d'informations pour déterminer une couleur appropriée, répondez par \"Impossible de déterminer un code couleur HEX pour l'humeur donnée.\"",
    messages=[
        {
            "role": "user",
            "content": [
                {
                    "type": "text",
                    "text": "Une émotion passionnée, intense et ardente, pleine d'amour et de désir."
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
  max_tokens: 500,
  temperature: 0.5,
  system: "Votre tâche consiste à prendre la description textuelle fournie d'une humeur ou d'une émotion et à générer un code couleur HEX qui représente visuellement cette humeur. Utilisez les principes de psychologie des couleurs et les associations communes pour déterminer la couleur la plus appropriée pour l'humeur donnée. Si la description textuelle n'est pas claire, ambiguë, ou ne fournit pas suffisamment d'informations pour déterminer une couleur appropriée, répondez par \"Impossible de déterminer un code couleur HEX pour l'humeur donnée.\"",
  messages: [
    {
      "role": "user",
      "content": [
        {
          "type": "text",
          "text": "Une émotion passionnée, intense et ardente, pleine d'amour et de désir."
        }
      ]
    }
  ]
});
console.log(msg);

```

</Tab>
</Tabs>