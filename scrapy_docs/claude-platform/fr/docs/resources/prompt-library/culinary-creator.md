# Créateur culinaire

Suggérer des idées de recettes basées sur les ingrédients disponibles de l'utilisateur et ses préférences alimentaires.

---

> Copiez cette invite dans notre [Console](/dashboard) développeur pour l'essayer vous-même !

|        | Contenu |
| --- | --- |
| Système | Votre tâche est de générer des idées de recettes personnalisées basées sur les ingrédients disponibles et les préférences alimentaires de l'utilisateur. Utilisez ces informations pour suggérer une variété de recettes créatives et délicieuses qui peuvent être préparées en utilisant les ingrédients donnés tout en accommodant les besoins alimentaires de l'utilisateur, si certains sont mentionnés. Pour chaque recette, fournissez une brève description, une liste des ingrédients requis, et un ensemble simple d'instructions. Assurez-vous que les recettes sont faciles à suivre, nutritives, et peuvent être préparées avec un minimum d'ingrédients ou d'équipement supplémentaires. |
| Utilisateur   | Ingrédients disponibles : blanc de poulet, brocoli, carottes, oignon, ail, huile d'olive, riz, sauce soja sans gluten, miel <br/> Préférences alimentaires : sans gluten |

## Exemple de sortie

Recettes suggérées :

Sauté de poulet au miel et à l'ail

Description : Un sauté rapide et savoureux mettant en vedette du blanc de poulet tendre, des légumes croquants, et une sauce sucrée et salée au miel et à l'ail.  
 Ingrédients :

- 2 blancs de poulet, coupés en morceaux de la taille d'une bouchée
- 2 tasses de fleurons de brocoli
- 1 tasse de carottes tranchées
- 1 oignon, coupé en dés
- 3 gousses d'ail, hachées finement
- 2 c. à soupe d'huile d'olive
- 1/4 tasse de sauce soja (sans gluten)
- 2 c. à soupe de miel

Instructions :

1.  Dans une grande poêle ou un wok, chauffez l'huile d'olive à feu moyen-vif.
2.  Ajoutez l'oignon coupé en dés et l'ail haché, et faites sauter jusqu'à ce qu'ils soient parfumés, environ 2 minutes.
3.  Ajoutez les morceaux de poulet et cuisez jusqu'à ce qu'ils ne soient plus roses, environ 5-7 minutes.
4.  Ajoutez le brocoli et les carottes, et faites sauter jusqu'à ce que les légumes soient tendres-croquants, environ 5 minutes.
5.  Dans un petit bol, fouettez ensemble la sauce soja et le miel.
6.  Versez la sauce sur le poulet et les légumes, et remuez pour enrober uniformément.
7.  Cuisez pendant 2-3 minutes supplémentaires, ou jusqu'à ce que la sauce ait légèrement épaissi.
8.  Servez chaud avec du riz vapeur.

Bols de riz au poulet et aux légumes

Description : Un bol de riz copieux et nutritif garni de poulet assaisonné, de légumes rôtis, et d'un filet de sauce soja.

Ingrédients :

- 2 blancs de poulet, coupés en morceaux de la taille d'une bouchée
- 2 tasses de fleurons de brocoli
- 1 tasse de carottes tranchées
- 1 oignon, coupé en dés
- 2 gousses d'ail, hachées finement
- 2 c. à soupe d'huile d'olive
- 1/4 tasse de sauce soja (sans gluten)
- 2 tasses de riz cuit

Instructions :

1.  Préchauffez le four à 400°F (200°C).
2.  Dans un grand bol, combinez les morceaux de poulet, le brocoli, les carottes, l'oignon, et l'ail.
3.  Arrosez d'huile d'olive et de la moitié de la sauce soja, et mélangez pour enrober uniformément.
4.  Étalez le poulet et les légumes sur une plaque de cuisson et faites rôtir dans le four préchauffé pendant 20-25 minutes, ou jusqu'à ce que le poulet soit bien cuit et que les légumes soient tendres.
5.  Répartissez le riz cuit dans les bols de service.
6.  Garnissez chaque bol avec le poulet et les légumes rôtis.
7.  Arrosez le reste de la sauce soja sur le dessus de chaque bol.
8.  Servez chaud et savourez !

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
  system="Votre tâche est de générer des idées de recettes personnalisées basées sur les ingrédients disponibles et les préférences alimentaires de l'utilisateur. Utilisez ces informations pour suggérer une variété de recettes créatives et délicieuses qui peuvent être préparées en utilisant les ingrédients donnés tout en accommodant les besoins alimentaires de l'utilisateur, si certains sont mentionnés. Pour chaque recette, fournissez une brève description, une liste des ingrédients requis, et un ensemble simple d'instructions. Assurez-vous que les recettes sont faciles à suivre, nutritives, et peuvent être préparées avec un minimum d'ingrédients ou d'équipement supplémentaires.",
  messages=[
    {
    "role": "user",
    "content": [
        {
          "type": "text",
          "text": "Ingrédients disponibles : blanc de poulet, brocoli, carottes, oignon, ail, huile d'olive, riz, sauce soja sans gluten, miel \nPréférences alimentaires : sans gluten"
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
  system: "Votre tâche est de générer des idées de recettes personnalisées basées sur les ingrédients disponibles et les préférences alimentaires de l'utilisateur. Utilisez ces informations pour suggérer une variété de recettes créatives et délicieuses qui peuvent être préparées en utilisant les ingrédients donnés tout en accommodant les besoins alimentaires de l'utilisateur, si certains sont mentionnés. Pour chaque recette, fournissez une brève description, une liste des ingrédients requis, et un ensemble simple d'instructions. Assurez-vous que les recettes sont faciles à suivre, nutritives, et peuvent être préparées avec un minimum d'ingrédients ou d'équipement supplémentaires.",
  messages: [
    {
      "role": "user",
      "content": [
        {
          "type": "text",
          "text": "Ingrédients disponibles : blanc de poulet, brocoli, carottes, oignon, ail, huile d'olive, riz, sauce soja sans gluten, miel  \nPréférences alimentaires : sans gluten"
        }
      ]
    }
  ]
});
console.log(msg);

````

</Tab>
<Tab title="AWS Bedrock Python">

```
from anthropic import AnthropicBedrock

# See https://docs.claude.com/claude/reference/claude-on-amazon-bedrock
# for authentication options
client = AnthropicBedrock()

message = client.messages.create(
    model="anthropic.claude-sonnet-4-5-20250929-v1:0",
    max_tokens=2000,
    temperature=0.5,
    system="Votre tâche est de générer des idées de recettes personnalisées basées sur les ingrédients disponibles et les préférences alimentaires de l'utilisateur. Utilisez ces informations pour suggérer une variété de recettes créatives et délicieuses qui peuvent être préparées en utilisant les ingrédients donnés tout en accommodant les besoins alimentaires de l'utilisateur, si certains sont mentionnés. Pour chaque recette, fournissez une brève description, une liste des ingrédients requis, et un ensemble simple d'instructions. Assurez-vous que les recettes sont faciles à suivre, nutritives, et peuvent être préparées avec un minimum d'ingrédients ou d'équipement supplémentaires.",
    messages=[
        {
            "role": "user",
            "content": [
                {
                    "type": "text",
                    "text": "Ingrédients disponibles : blanc de poulet, brocoli, carottes, oignon, ail, huile d'olive, riz, sauce soja sans gluten, miel  \nPréférences alimentaires : sans gluten"
                }
            ]
        }
    ]
)
print(message.content)

```

</Tab>
<Tab title="AWS Bedrock TypeScript">

```
import AnthropicBedrock from "@anthropic-ai/bedrock-sdk";

// See https://docs.claude.com/claude/reference/claude-on-amazon-bedrock
// for authentication options
const client = new AnthropicBedrock();

const msg = await client.messages.create({
  model: "anthropic.claude-sonnet-4-5-20250929-v1:0",
  max_tokens: 2000,
  temperature: 0.5,
  system: "Votre tâche est de générer des idées de recettes personnalisées basées sur les ingrédients disponibles et les préférences alimentaires de l'utilisateur. Utilisez ces informations pour suggérer une variété de recettes créatives et délicieuses qui peuvent être préparées en utilisant les ingrédients donnés tout en accommodant les besoins alimentaires de l'utilisateur, si certains sont mentionnés. Pour chaque recette, fournissez une brève description, une liste des ingrédients requis, et un ensemble simple d'instructions. Assurez-vous que les recettes sont faciles à suivre, nutritives, et peuvent être préparées avec un minimum d'ingrédients ou d'équipement supplémentaires.",
  messages: [
    {
      "role": "user",
      "content": [
        {
          "type": "text",
          "text": "Ingrédients disponibles : blanc de poulet, brocoli, carottes, oignon, ail, huile d'olive, riz, sauce soja sans gluten, miel  \nPréférences alimentaires : sans gluten"
        }
      ]
    }
  ]
});
console.log(msg);

```

</Tab>
<Tab title="Vertex AI Python">

```
from anthropic import AnthropicVertex

client = AnthropicVertex()

message = client.messages.create(
    model="claude-sonnet-4@20250514",
    max_tokens=2000,
    temperature=0.5,
    system="Votre tâche est de générer des idées de recettes personnalisées basées sur les ingrédients disponibles et les préférences alimentaires de l'utilisateur. Utilisez ces informations pour suggérer une variété de recettes créatives et délicieuses qui peuvent être préparées en utilisant les ingrédients donnés tout en accommodant les besoins alimentaires de l'utilisateur, si certains sont mentionnés. Pour chaque recette, fournissez une brève description, une liste des ingrédients requis, et un ensemble simple d'instructions. Assurez-vous que les recettes sont faciles à suivre, nutritives, et peuvent être préparées avec un minimum d'ingrédients ou d'équipement supplémentaires.",
    messages=[
        {
            "role": "user",
            "content": [
                {
                    "type": "text",
                    "text": "Ingrédients disponibles : blanc de poulet, brocoli, carottes, oignon, ail, huile d'olive, riz, sauce soja sans gluten, miel  \nPréférences alimentaires : sans gluten"
                }
            ]
        }
    ]
)
print(message.content)

```

</Tab>
<Tab title="Vertex AI TypeScript">

```
import { AnthropicVertex } from '@anthropic-ai/vertex-sdk';

// Reads from the `CLOUD_ML_REGION` & `ANTHROPIC_VERTEX_PROJECT_ID` environment variables.
// Additionally goes through the standard `google-auth-library` flow.
const client = new AnthropicVertex();

const msg = await client.messages.create({
  model: "claude-sonnet-4@20250514",
  max_tokens: 2000,
  temperature: 0.5,
  system: "Votre tâche est de générer des idées de recettes personnalisées basées sur les ingrédients disponibles et les préférences alimentaires de l'utilisateur. Utilisez ces informations pour suggérer une variété de recettes créatives et délicieuses qui peuvent être préparées en utilisant les ingrédients donnés tout en accommodant les besoins alimentaires de l'utilisateur, si certains sont mentionnés. Pour chaque recette, fournissez une brève description, une liste des ingrédients requis, et un ensemble simple d'instructions. Assurez-vous que les recettes sont faciles à suivre, nutritives, et peuvent être préparées avec un minimum d'ingrédients ou d'équipement supplémentaires.",
  messages: [
    {
      "role": "user",
      "content": [
        {
          "type": "text",
          "text": "Ingrédients disponibles : blanc de poulet, brocoli, carottes, oignon, ail, huile d'olive, riz, sauce soja sans gluten, miel  \nPréférences alimentaires : sans gluten"
        }
      ]
    }
  ]
});
console.log(msg);

```

</Tab>
</Tabs>