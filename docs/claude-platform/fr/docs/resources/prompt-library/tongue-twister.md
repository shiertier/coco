# Virelangue

Créer des virelangues difficiles.

---

> Copiez cette invite dans notre [Console](/dashboard) développeur pour l'essayer vous-même !

| Rôle | Contenu |
| ---- | --- |
| Utilisateur | Générez des virelangues complexes et créatifs. Visez à créer des virelangues qui sont non seulement difficiles à dire mais aussi engageants, divertissants et potentiellement humoristiques. Considérez l'incorporation de jeux de mots, de rimes et d'allitération pour améliorer la difficulté et le plaisir des virelangues. |

## Exemple de sortie

Virelangue 1 :  
 La musaraigne astucieuse mâcha à travers la chaussure brillante et chatoyante.

Virelangue 2 :  
 Des brises vives soufflaient près du babouin de sang bleu, bavard et babillant.

Virelangue 3 :  
 Le joueur de cornemuse perplexe cueillit un picotin de poivrons marinés tout en pratiquant son jeu de cornemuse.

Virelangue 4 :  
 Six traîneaux élégants, lisses et minces glissèrent rapidement sur la pente raide et enneigée.

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
          "text": "Générez des virelangues complexes et créatifs. Visez à créer des virelangues qui sont non seulement difficiles à dire mais aussi engageants, divertissants et potentiellement humoristiques. Considérez l'incorporation de jeux de mots, de rimes et d'allitération pour améliorer la difficulté et le plaisir des virelangues."
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
          "text": "Générez des virelangues complexes et créatifs. Visez à créer des virelangues qui sont non seulement difficiles à dire mais aussi engageants, divertissants et potentiellement humoristiques. Considérez l'incorporation de jeux de mots, de rimes et d'allitération pour améliorer la difficulté et le plaisir des virelangues."
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
                    "text": "Générez des virelangues complexes et créatifs. Visez à créer des virelangues qui sont non seulement difficiles à dire mais aussi engageants, divertissants et potentiellement humoristiques. Considérez l'incorporation de jeux de mots, de rimes et d'allitération pour améliorer la difficulté et le plaisir des virelangues."
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
          "text": "Générez des virelangues complexes et créatifs. Visez à créer des virelangues qui sont non seulement difficiles à dire mais aussi engageants, divertissants et potentiellement humoristiques. Considérez l'incorporation de jeux de mots, de rimes et d'allitération pour améliorer la difficulté et le plaisir des virelangues."
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
                    "text": "Générez des virelangues complexes et créatifs. Visez à créer des virelangues qui sont non seulement difficiles à dire mais aussi engageants, divertissants et potentiellement humoristiques. Considérez l'incorporation de jeux de mots, de rimes et d'allitération pour améliorer la difficulté et le plaisir des virelangues."
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
          "text": "Générez des virelangues complexes et créatifs. Visez à créer des virelangues qui sont non seulement difficiles à dire mais aussi engageants, divertissants et potentiellement humoristiques. Considérez l'incorporation de jeux de mots, de rimes et d'allitération pour améliorer la difficulté et le plaisir des virelangues."
        }
      ]
    }
  ]
});
console.log(msg);

```

</Tab>
</Tabs>