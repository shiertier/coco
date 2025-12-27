# Innovateur de fitness VR

Générez des idées créatives pour des jeux de fitness en réalité virtuelle.

---

> Copiez cette invite dans notre [Console](/dashboard) développeur pour l'essayer vous-même !

| Rôle | Contenu |
| ---- | --- |
| Utilisateur | Votre tâche est de générer une liste d'idées innovantes et engageantes pour des jeux de fitness en réalité virtuelle (VR). Considérez divers genres de jeux, des mécaniques de gameplay uniques, des environnements immersifs, et des façons créatives d'incorporer des exercices physiques et des mouvements. Les idées devraient être attrayantes pour un large éventail d'amateurs de fitness et de joueurs, les encourageant à rester actifs et à s'amuser tout en faisant de l'exercice en VR. Pour chaque idée, fournissez une brève description du concept de jeu, des caractéristiques clés, et comment il favorise le fitness. |

## Exemple de Sortie

1.  Idée : Aventure Parkour VR  
    Description : Les joueurs naviguent à travers un environnement urbain dynamique, courant, sautant, grimpant, et effectuant des cascades de parkour pour surmonter les obstacles et atteindre les objectifs. Le jeu s'adapte aux mouvements du joueur, fournissant un entraînement intense pour tout le corps tout en explorant la ville virtuelle.  
    Caractéristiques clés : Mécaniques de parkour réalistes, environnements générés procéduralement, courses multijoueurs, et défis.  
    Aspects fitness : Cardio, agilité, coordination, et force du bas du corps.
2.  Idée : Boxe Rythmée  
    Description : Un jeu de boxe haute énergie calé sur le rythme de morceaux de musique populaires. Les joueurs doivent frapper, esquiver, et bloquer en synchronisation avec le rythme pour marquer des points et progresser à travers des niveaux de plus en plus difficiles. Le jeu offre une variété de styles de boxe et de modes d'entraînement.  
    Caractéristiques clés : Musique sous licence, boxeurs personnalisables, multijoueur en ligne, et une gamme diverse de techniques de boxe.  
    Aspects fitness : Cardio, force du haut du corps, réflexes, et endurance.
3.  Idée : RPG Fitness VR  
    Description : Un jeu de rôle immersif où les joueurs créent leur propre personnage et se lancent dans une quête pour sauver un monde fantastique. Le jeu combine des éléments RPG traditionnels avec des défis de fitness, exigeant des joueurs qu'ils effectuent des exercices physiques pour lancer des sorts, vaincre des ennemis, et faire monter de niveau leur personnage.  
    Caractéristiques clés : Personnalisation de personnage, arbres de compétences, batailles de boss épiques, et un mélange d'exercices de force, cardio, et flexibilité.  
    Aspects fitness : Entraînements pour tout le corps, musculation, cardio, et flexibilité.

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
          "text": "Votre tâche est de générer une liste d'idées innovantes et engageantes pour des jeux de fitness en réalité virtuelle (VR). Considérez divers genres de jeux, des mécaniques de gameplay uniques, des environnements immersifs, et des façons créatives d'incorporer des exercices physiques et des mouvements. Les idées devraient être attrayantes pour un large éventail d'amateurs de fitness et de joueurs, les encourageant à rester actifs et à s'amuser tout en faisant de l'exercice en VR. Pour chaque idée, fournissez une brève description du concept de jeu, des caractéristiques clés, et comment il favorise le fitness."
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
          "text": "Votre tâche est de générer une liste d'idées innovantes et engageantes pour des jeux de fitness en réalité virtuelle (VR). Considérez divers genres de jeux, des mécaniques de gameplay uniques, des environnements immersifs, et des façons créatives d'incorporer des exercices physiques et des mouvements. Les idées devraient être attrayantes pour un large éventail d'amateurs de fitness et de joueurs, les encourageant à rester actifs et à s'amuser tout en faisant de l'exercice en VR. Pour chaque idée, fournissez une brève description du concept de jeu, des caractéristiques clés, et comment il favorise le fitness."
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
                    "text": "Votre tâche est de générer une liste d'idées innovantes et engageantes pour des jeux de fitness en réalité virtuelle (VR). Considérez divers genres de jeux, des mécaniques de gameplay uniques, des environnements immersifs, et des façons créatives d'incorporer des exercices physiques et des mouvements. Les idées devraient être attrayantes pour un large éventail d'amateurs de fitness et de joueurs, les encourageant à rester actifs et à s'amuser tout en faisant de l'exercice en VR. Pour chaque idée, fournissez une brève description du concept de jeu, des caractéristiques clés, et comment il favorise le fitness."
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
          "text": "Votre tâche est de générer une liste d'idées innovantes et engageantes pour des jeux de fitness en réalité virtuelle (VR). Considérez divers genres de jeux, des mécaniques de gameplay uniques, des environnements immersifs, et des façons créatives d'incorporer des exercices physiques et des mouvements. Les idées devraient être attrayantes pour un large éventail d'amateurs de fitness et de joueurs, les encourageant à rester actifs et à s'amuser tout en faisant de l'exercice en VR. Pour chaque idée, fournissez une brève description du concept de jeu, des caractéristiques clés, et comment il favorise le fitness."
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
                    "text": "Votre tâche est de générer une liste d'idées innovantes et engageantes pour des jeux de fitness en réalité virtuelle (VR). Considérez divers genres de jeux, des mécaniques de gameplay uniques, des environnements immersifs, et des façons créatives d'incorporer des exercices physiques et des mouvements. Les idées devraient être attrayantes pour un large éventail d'amateurs de fitness et de joueurs, les encourageant à rester actifs et à s'amuser tout en faisant de l'exercice en VR. Pour chaque idée, fournissez une brève description du concept de jeu, des caractéristiques clés, et comment il favorise le fitness."
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
          "text": "Votre tâche est de générer une liste d'idées innovantes et engageantes pour des jeux de fitness en réalité virtuelle (VR). Considérez divers genres de jeux, des mécaniques de gameplay uniques, des environnements immersifs, et des façons créatives d'incorporer des exercices physiques et des mouvements. Les idées devraient être attrayantes pour un large éventail d'amateurs de fitness et de joueurs, les encourageant à rester actifs et à s'amuser tout en faisant de l'exercice en VR. Pour chaque idée, fournissez une brève description du concept de jeu, des caractéristiques clés, et comment il favorise le fitness."
        }
      ]
    }
  ]
});
console.log(msg);

```

</Tab>
</Tabs>