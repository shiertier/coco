# Gourou de la notation

Comparez et évaluez la qualité de textes écrits basés sur des critères et standards définis par l'utilisateur.

---

> Copiez cette invite dans notre [Console](/dashboard) développeur pour l'essayer vous-même !

| Rôle | Contenu |
| ---- | --- |
| Utilisateur | Évaluez les deux textes suivants basés sur les critères donnés : <br/> <br/> Texte 1 : <br/> Le soleil se couchait derrière les montagnes, projetant une lueur dorée et chaleureuse à travers la vallée. Les oiseaux chantaient leurs chansons du soir tandis que la brise douce bruissait à travers les feuilles. C'était un moment paisible et serein, qui remplissait le cœur de joie et de contentement. <br/><br/> Texte 2 : <br/> Le coucher de soleil était joli. Les oiseaux chantaient et le vent soufflait. Cela m'a rendu heureux. <br/><br/> Critères : <br/> 1. Langage descriptif et imagerie <br/> 2. Structure et variété des phrases <br/> 3. Impact émotionnel et engagement <br/> 4. Grammaire et ponctuation |

## Exemple de sortie

Basé sur les critères fournis, voici mon évaluation des deux textes :

Texte 1 :

1.  Langage descriptif et imagerie : Le texte utilise un langage vif et évocateur pour peindre une image du décor. Des phrases comme "lueur dorée et chaleureuse", "les oiseaux chantaient leurs chansons du soir", et "la brise douce bruissait à travers les feuilles" créent une imagerie visuelle et auditive forte, immergeant le lecteur dans la scène.
2.  Structure et variété des phrases : Le texte démontre un bon mélange de structures de phrases, avec des phrases simples et composées. Cette variété aide à maintenir un flux fluide et garde le lecteur engagé.
3.  Impact émotionnel et engagement : Le texte transmet efficacement un sentiment de paix, de joie et de contentement, invitant le lecteur à partager l'expérience émotionnelle du moment. La description du décor et l'utilisation d'un langage positif créent une connexion émotionnelle forte.
4.  Grammaire et ponctuation : Le texte semble être grammaticalement correct et bien ponctué, sans erreurs ou problèmes évidents.

Texte 2 :

1.  Langage descriptif et imagerie : Le texte manque de langage descriptif détaillé et d'imagerie. Bien qu'il mentionne le coucher de soleil, les oiseaux et le vent, il ne fournit aucun détail vif ou évocateur pour aider le lecteur à visualiser la scène.
2.  Structure et variété des phrases : Le texte consiste en phrases courtes et simples avec peu de variation dans la structure. Ce manque de variété peut rendre l'écriture monotone et moins engageante.
3.  Impact émotionnel et engagement : Le texte transmet une émotion positive en déclarant "Cela m'a rendu heureux", mais il ne fournit pas une connexion émotionnelle forte ou un engagement en raison du manque de langage descriptif et d'imagerie.
4.  Grammaire et ponctuation : Le texte semble être grammaticalement correct et bien ponctué, sans erreurs ou problèmes évidents.

Analyse comparative :  
 Basé sur l'évaluation, le Texte 1 performe significativement mieux que le Texte 2 en termes de langage descriptif, d'imagerie, de structure de phrases, de variété et d'impact émotionnel. Le Texte 1 crée une expérience vive et immersive pour le lecteur et démontre une qualité d'écriture supérieure basée sur les critères donnés, avec son langage descriptif fort, ses structures de phrases variées et son engagement émotionnel efficace. Le Texte 2, bien que grammaticalement correct, manque de la richesse descriptive et de l'impact émotionnel présents dans le Texte 1.

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
    max_tokens=2000,
    temperature=0,
    messages=[
        {
            "role": "user",
            "content": [
                {
                    "type": "text",
                    "text": "Évaluez les deux textes suivants basés sur les critères donnés : \n \nTexte 1 : \nLe soleil se couchait derrière les montagnes, projetant une lueur dorée et chaleureuse à travers la vallée. Les oiseaux chantaient leurs chansons du soir tandis que la brise douce bruissait à travers les feuilles. C'était un moment paisible et serein, qui remplissait le cœur de joie et de contentement. \n \nTexte 2 : \nLe coucher de soleil était joli. Les oiseaux chantaient et le vent soufflait. Cela m'a rendu heureux. \n \nCritères : \n1. Langage descriptif et imagerie \n2. Structure et variété des phrases \n3. Impact émotionnel et engagement \n4. Grammaire et ponctuation",
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
  max_tokens: 2000,
  temperature: 0,
  messages: [
    {
      "role": "user",
      "content": [
        {
          "type": "text",
          "text": "Évaluez les deux textes suivants basés sur les critères donnés :  \n  \nTexte 1 :  \nLe soleil se couchait derrière les montagnes, projetant une lueur dorée et chaleureuse à travers la vallée. Les oiseaux chantaient leurs chansons du soir tandis que la brise douce bruissait à travers les feuilles. C'était un moment paisible et serein, qui remplissait le cœur de joie et de contentement.  \n  \nTexte 2 :  \nLe coucher de soleil était joli. Les oiseaux chantaient et le vent soufflait. Cela m'a rendu heureux.  \n  \nCritères :  \n1. Langage descriptif et imagerie  \n2. Structure et variété des phrases  \n3. Impact émotionnel et engagement  \n4. Grammaire et ponctuation"
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
    temperature=0,
    messages=[
        {
            "role": "user",
            "content": [
                {
                    "type": "text",
                    "text": "Évaluez les deux textes suivants basés sur les critères donnés :  \n  \nTexte 1 :  \nLe soleil se couchait derrière les montagnes, projetant une lueur dorée et chaleureuse à travers la vallée. Les oiseaux chantaient leurs chansons du soir tandis que la brise douce bruissait à travers les feuilles. C'était un moment paisible et serein, qui remplissait le cœur de joie et de contentement.  \n  \nTexte 2 :  \nLe coucher de soleil était joli. Les oiseaux chantaient et le vent soufflait. Cela m'a rendu heureux.  \n  \nCritères :  \n1. Langage descriptif et imagerie  \n2. Structure et variété des phrases  \n3. Impact émotionnel et engagement  \n4. Grammaire et ponctuation"
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
  max_tokens: 2000,
  temperature: 0,
  messages: [
    {
      "role": "user",
      "content": [
        {
          "type": "text",
          "text": "Évaluez les deux textes suivants basés sur les critères donnés :  \n  \nTexte 1 :  \nLe soleil se couchait derrière les montagnes, projetant une lueur dorée et chaleureuse à travers la vallée. Les oiseaux chantaient leurs chansons du soir tandis que la brise douce bruissait à travers les feuilles. C'était un moment paisible et serein, qui remplissait le cœur de joie et de contentement.  \n  \nTexte 2 :  \nLe coucher de soleil était joli. Les oiseaux chantaient et le vent soufflait. Cela m'a rendu heureux.  \n  \nCritères :  \n1. Langage descriptif et imagerie  \n2. Structure et variété des phrases  \n3. Impact émotionnel et engagement  \n4. Grammaire et ponctuation"
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
    max_tokens=2000,
    temperature=0,
    messages=[
        {
            "role": "user",
            "content": [
                {
                    "type": "text",
                    "text": "Évaluez les deux textes suivants basés sur les critères donnés :  \n  \nTexte 1 :  \nLe soleil se couchait derrière les montagnes, projetant une lueur dorée et chaleureuse à travers la vallée. Les oiseaux chantaient leurs chansons du soir tandis que la brise douce bruissait à travers les feuilles. C'était un moment paisible et serein, qui remplissait le cœur de joie et de contentement.  \n  \nTexte 2 :  \nLe coucher de soleil était joli. Les oiseaux chantaient et le vent soufflait. Cela m'a rendu heureux.  \n  \nCritères :  \n1. Langage descriptif et imagerie  \n2. Structure et variété des phrases  \n3. Impact émotionnel et engagement  \n4. Grammaire et ponctuation"
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
  temperature: 0,
  messages: [
    {
      "role": "user",
      "content": [
        {
          "type": "text",
          "text": "Évaluez les deux textes suivants basés sur les critères donnés :  \n  \nTexte 1 :  \nLe soleil se couchait derrière les montagnes, projetant une lueur dorée et chaleureuse à travers la vallée. Les oiseaux chantaient leurs chansons du soir tandis que la brise douce bruissait à travers les feuilles. C'était un moment paisible et serein, qui remplissait le cœur de joie et de contentement.  \n  \nTexte 2 :  \nLe coucher de soleil était joli. Les oiseaux chantaient et le vent soufflait. Cela m'a rendu heureux.  \n  \nCritères :  \n1. Langage descriptif et imagerie  \n2. Structure et variété des phrases  \n3. Impact émotionnel et engagement  \n4. Grammaire et ponctuation"
        }
      ]
    }
  ]
});
console.log(msg);

```

</Tab>
</Tabs>