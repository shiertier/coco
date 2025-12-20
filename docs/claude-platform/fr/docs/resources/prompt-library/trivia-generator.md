# Générateur de quiz

Générez des questions de quiz sur une large gamme de sujets et fournissez des indices lorsque nécessaire.

---

> Copiez cette invite dans notre [Console](/dashboard) développeur pour l'essayer vous-même !

| Rôle | Contenu |
| ---- | --- |
| Utilisateur | Générez des questions de quiz sur divers sujets et fournissez des indices pour aider les utilisateurs à arriver à la bonne réponse. Sélectionnez parmi un ensemble diversifié de catégories et créez des questions qui testent les connaissances ou les compétences de raisonnement de l'utilisateur. Offrez une série d'indices de plus en plus spécifiques pour guider les utilisateurs vers la solution. Assurez-vous que les questions sont stimulantes et que les indices fournissent suffisamment d'informations pour aider l'utilisateur sans révéler la réponse trop facilement. |

## Exemple de sortie

Catégorie : Géographie

Question :  
 Quel est le seul pays au monde qui borde à la fois la France et l'Espagne ?

Indice 1 : Ce pays est situé dans les montagnes des Pyrénées entre la France et l'Espagne.  
 Indice 2 : Il est connu pour ses paysages montagneux époustouflants, ses stations de ski et ses achats hors taxes.  
 Indice 3 : La capitale du pays porte le nom du pays lui-même.

Réponse : Andorre

Catégorie : Science

Question :  
 Quel est le processus par lequel les plantes convertissent la lumière du soleil, l'eau et le dioxyde de carbone en énergie et en oxygène ?

Indice 1 : Ce processus est essentiel à la vie sur Terre, car il fournit à la fois de la nourriture et de l'oxygène.  
 Indice 2 : Il se déroule principalement dans les feuilles des plantes.  
 Indice 3 : Le processus implique la chlorophylle, un pigment vert qui absorbe la lumière du soleil.

Réponse : Photosynthèse

Catégorie : Littérature

Question :  
 Qui a écrit le roman classique "Orgueil et Préjugés", qui fut publié pour la première fois en 1813 ?

Indice 1 : L'auteur est une femme britannique qui a vécu de 1775 à 1817.  
 Indice 2 : Elle est connue pour ses portraits spirituels et perspicaces de la société et des mœurs anglaises.  
 Indice 3 : Ses autres œuvres célèbres incluent "Raison et Sentiments" et "Emma".

Réponse : Jane Austen

Catégorie : Histoire

Question :  
 En quelle année le mur de Berlin, qui divisait l'Allemagne de l'Est et de l'Ouest, a-t-il commencé à être démoli ?

Indice 1 : L'événement s'est produit à la fin du 20e siècle.  
 Indice 2 : Il a marqué un moment significatif dans la fin de la Guerre froide.  
 Indice 3 : L'année est souvent associée à l'expression "chute du communisme" en Europe.

Réponse : 1989

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
  max_tokens=2000,
  temperature=0.5,
  messages=[
    {
    "role": "user",
    "content": [
        {
          "type": "text",
          "text": "Générez des questions de quiz sur divers sujets et fournissez des indices pour aider les utilisateurs à arriver à la bonne réponse. Sélectionnez parmi un ensemble diversifié de catégories et créez des questions qui testent les connaissances ou les compétences de raisonnement de l'utilisateur. Offrez une série d'indices de plus en plus spécifiques pour guider les utilisateurs vers la solution. Assurez-vous que les questions sont stimulantes et que les indices fournissent suffisamment d'informations pour aider l'utilisateur sans révéler la réponse trop facilement."
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
  max_tokens: 2000,
  temperature: 0.5,
  messages: [
    {
      "role": "user",
      "content": [
        {
          "type": "text",
          "text": "Générez des questions de quiz sur divers sujets et fournissez des indices pour aider les utilisateurs à arriver à la bonne réponse. Sélectionnez parmi un ensemble diversifié de catégories et créez des questions qui testent les connaissances ou les compétences de raisonnement de l'utilisateur. Offrez une série d'indices de plus en plus spécifiques pour guider les utilisateurs vers la solution. Assurez-vous que les questions sont stimulantes et que les indices fournissent suffisamment d'informations pour aider l'utilisateur sans révéler la réponse trop facilement."
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
max_tokens=2000,
temperature=0.5,
messages=[
{
"role": "user",
"content": [
{
"type": "text",
"text": "Générez des questions de quiz sur divers sujets et fournissez des indices pour aider les utilisateurs à arriver à la bonne réponse. Sélectionnez parmi un ensemble diversifié de catégories et créez des questions qui testent les connaissances ou les compétences de raisonnement de l'utilisateur. Offrez une série d'indices de plus en plus spécifiques pour guider les utilisateurs vers la solution. Assurez-vous que les questions sont stimulantes et que les indices fournissent suffisamment d'informations pour aider l'utilisateur sans révéler la réponse trop facilement."
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
  max_tokens: 2000,
  temperature: 0.5,
  messages: [
    {
      "role": "user",
      "content": [
        {
          "type": "text",
          "text": "Générez des questions de quiz sur divers sujets et fournissez des indices pour aider les utilisateurs à arriver à la bonne réponse. Sélectionnez parmi un ensemble diversifié de catégories et créez des questions qui testent les connaissances ou les compétences de raisonnement de l'utilisateur. Offrez une série d'indices de plus en plus spécifiques pour guider les utilisateurs vers la solution. Assurez-vous que les questions sont stimulantes et que les indices fournissent suffisamment d'informations pour aider l'utilisateur sans révéler la réponse trop facilement."
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
    max_tokens=2000,
    temperature=0.5,
    messages=[
        {
            "role": "user",
            "content": [
                {
                    "type": "text",
                    "text": "Générez des questions de quiz sur divers sujets et fournissez des indices pour aider les utilisateurs à arriver à la bonne réponse. Sélectionnez parmi un ensemble diversifié de catégories et créez des questions qui testent les connaissances ou les compétences de raisonnement de l'utilisateur. Offrez une série d'indices de plus en plus spécifiques pour guider les utilisateurs vers la solution. Assurez-vous que les questions sont stimulantes et que les indices fournissent suffisamment d'informations pour aider l'utilisateur sans révéler la réponse trop facilement."
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
  max_tokens: 2000,
  temperature: 0.5,
  messages: [
    {
      "role": "user",
      "content": [
        {
          "type": "text",
          "text": "Générez des questions de quiz sur divers sujets et fournissez des indices pour aider les utilisateurs à arriver à la bonne réponse. Sélectionnez parmi un ensemble diversifié de catégories et créez des questions qui testent les connaissances ou les compétences de raisonnement de l'utilisateur. Offrez une série d'indices de plus en plus spécifiques pour guider les utilisateurs vers la solution. Assurez-vous que les questions sont stimulantes et que les indices fournissent suffisamment d'informations pour aider l'utilisateur sans révéler la réponse trop facilement."
        }
      ]
    }
  ]
});
console.log(msg);

```

</Tab>
</Tabs>