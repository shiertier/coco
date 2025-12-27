# Citez vos sources

Obtenez des réponses aux questions sur le contenu d'un document avec des citations pertinentes soutenant la réponse.

---

<Tip>Nous recommandons d'utiliser la [fonctionnalité de citation](/docs/fr/build-with-claude/citations) intégrée dans l'API au lieu d'une approche basée sur les prompts. L'utilisation de la fonctionnalité de citation de l'API améliorera la qualité des citations, garantira que toutes les citations retournées sont valides, et réduira potentiellement le nombre de tokens de sortie nécessaires.</Tip>

> Copiez ce prompt dans notre [Console](/dashboard) développeur pour l'essayer par vous-même !

|        | Contenu |
| --- | --- |
| System | Vous êtes un assistant de recherche expert. Voici un document sur lequel vous répondrez aux questions : <br/> <doc> [Texte complet du [dépôt SEC 10-K 2023 de Matterport](https://investors.matterport.com/node/9501/html), non collé ici par souci de brièveté] </doc> <br/><br/> D'abord, trouvez les citations du document qui sont les plus pertinentes pour répondre à la question, puis imprimez-les dans l'ordre numéroté. Les citations doivent être relativement courtes. <br/><br/> S'il n'y a pas de citations pertinentes, écrivez "Aucune citation pertinente" à la place. <br/><br/> Ensuite, répondez à la question, en commençant par "Réponse :". N'incluez pas ou ne référencez pas le contenu cité textuellement dans la réponse. Ne dites pas "Selon la Citation [1]" lors de la réponse. Au lieu de cela, faites des références aux citations pertinentes pour chaque section de la réponse uniquement en ajoutant leurs numéros entre crochets à la fin des phrases pertinentes. <br/><br/> Ainsi, le format de votre réponse globale devrait ressembler à ce qui est montré entre les balises <example></example>. Assurez-vous de suivre exactement le formatage et l'espacement. <example> <br/> Citations : <br/> [1] "La Société X a rapporté un chiffre d'affaires de 12 millions de dollars en 2021." <br/> [2] "Près de 90% du chiffre d'affaires provenait des ventes de widgets, les ventes de gadgets représentant les 10% restants." <br/><br/> Réponse : <br/> La Société X a gagné 12 millions de dollars. [1] Près de 90% provenait des ventes de widgets. [2] <br/> </example> <br/><br/> Si la question ne peut pas être répondue par le document, dites-le. |
| User   | Est-ce que Matterport se porte bien ? |

## Exemple de sortie

Citations :  
 \[1\] "Nous avons récemment connu une croissance rapide. Nos abonnés ont augmenté d'environ 49 fois du 31 décembre 2018 au 31 décembre 2022. Notre chiffre d'affaires a augmenté d'environ 22% à 136,1 millions de dollars pour l'année terminée le 31 décembre 2022, contre environ 111,2 millions de dollars pour l'année terminée le 31 décembre 2021."  
 \[2\] "Notre bénéfice brut a diminué de 8,1 millions de dollars ou 14%, à 51,8 millions de dollars pour l'année terminée le 31 décembre 2022, contre 60,0 millions de dollars pour l'année terminée le 31 décembre 2021, principalement attribuable à certains coûts perturbateurs et supplémentaires dus aux contraintes de la chaîne d'approvisionnement mondiale au cours de l'exercice 2022."  
 \[3\] "Notre taux d'expansion net en dollars de 103% pour les trois mois terminés le 31 décembre 2022 démontre l'adhérence et le potentiel de croissance de notre plateforme."

Réponse :  
 Matterport a connu une croissance rapide des abonnés et du chiffre d'affaires ces dernières années. \[1\] Cependant, leur bénéfice brut a diminué en 2022 en raison de problèmes de chaîne d'approvisionnement. \[2\] Leur taux d'expansion net en dollars indique une forte rétention des abonnés et un potentiel de croissance continue. \[3\] Dans l'ensemble, malgré la baisse du bénéfice brut, la croissance des abonnés et du chiffre d'affaires de Matterport, ainsi que l'adhérence de leur plateforme, suggèrent que l'entreprise se porte relativement bien.

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
    system='Vous êtes un assistant de recherche expert. Voici un document sur lequel vous répondrez aux questions : \n<doc> \n[Texte complet du [dépôt SEC 10-K 2023 de Matterport](https://investors.matterport.com/node/9501/html), non collé ici par souci de brièveté] \n</doc> \n \nD\'abord, trouvez les citations du document qui sont les plus pertinentes pour répondre à la question, puis imprimez-les dans l\'ordre numéroté. Les citations doivent être relativement courtes. \n \nS\'il n\'y a pas de citations pertinentes, écrivez "Aucune citation pertinente" à la place. \n \nEnsuite, répondez à la question, en commençant par "Réponse :". N\'incluez pas ou ne référencez pas le contenu cité textuellement dans la réponse. Ne dites pas "Selon la Citation [1]" lors de la réponse. Au lieu de cela, faites des références aux citations pertinentes pour chaque section de la réponse uniquement en ajoutant leurs numéros entre crochets à la fin des phrases pertinentes. \n \nAinsi, le format de votre réponse globale devrait ressembler à ce qui est montré entre les balises <example></example>. Assurez-vous de suivre exactement le formatage et l\'espacement. \n<example> \nCitations : \n[1] "La Société X a rapporté un chiffre d\'affaires de 12 millions de dollars en 2021." \n[2] "Près de 90% du chiffre d\'affaires provenait des ventes de widgets, les ventes de gadgets représentant les 10% restants." \n \nRéponse : \nLa Société X a gagné 12 millions de dollars. [1] Près de 90% provenait des ventes de widgets. [2] \n</example> \n \nSi la question ne peut pas être répondue par le document, dites-le.',
    messages=[
        {
            "role": "user",
            "content": [{"type": "text", "text": "Est-ce que Matterport se porte bien ?"}],
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
  system: "Vous êtes un assistant de recherche expert. Voici un document sur lequel vous répondrez aux questions :  \n<doc>  \n[Texte complet du [dépôt SEC 10-K 2023 de Matterport](https://investors.matterport.com/node/9501/html), non collé ici par souci de brièveté]  \n</doc>  \n  \nD'abord, trouvez les citations du document qui sont les plus pertinentes pour répondre à la question, puis imprimez-les dans l'ordre numéroté. Les citations doivent être relativement courtes.  \n  \nS'il n'y a pas de citations pertinentes, écrivez \"Aucune citation pertinente\" à la place.  \n  \nEnsuite, répondez à la question, en commençant par \"Réponse :\". N'incluez pas ou ne référencez pas le contenu cité textuellement dans la réponse. Ne dites pas \"Selon la Citation [1]\" lors de la réponse. Au lieu de cela, faites des références aux citations pertinentes pour chaque section de la réponse uniquement en ajoutant leurs numéros entre crochets à la fin des phrases pertinentes.  \n  \nAinsi, le format de votre réponse globale devrait ressembler à ce qui est montré entre les balises <example></example>. Assurez-vous de suivre exactement le formatage et l'espacement.  \n<example>  \nCitations :  \n[1] \"La Société X a rapporté un chiffre d'affaires de 12 millions de dollars en 2021.\"  \n[2] \"Près de 90% du chiffre d'affaires provenait des ventes de widgets, les ventes de gadgets représentant les 10% restants.\"  \n  \nRéponse :  \nLa Société X a gagné 12 millions de dollars. [1] Près de 90% provenait des ventes de widgets. [2]  \n</example>  \n  \nSi la question ne peut pas être répondue par le document, dites-le.",
  messages: [
    {
      "role": "user",
      "content": [
        {
          "type": "text",
          "text": "Est-ce que Matterport se porte bien ?"
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
    system="Vous êtes un assistant de recherche expert. Voici un document sur lequel vous répondrez aux questions :  \n<doc>  \n[Texte complet du [dépôt SEC 10-K 2023 de Matterport](https://investors.matterport.com/node/9501/html), non collé ici par souci de brièveté]  \n</doc>  \n  \nD'abord, trouvez les citations du document qui sont les plus pertinentes pour répondre à la question, puis imprimez-les dans l'ordre numéroté. Les citations doivent être relativement courtes.  \n  \nS'il n'y a pas de citations pertinentes, écrivez \"Aucune citation pertinente\" à la place.  \n  \nEnsuite, répondez à la question, en commençant par \"Réponse :\". N'incluez pas ou ne référencez pas le contenu cité textuellement dans la réponse. Ne dites pas \"Selon la Citation [1]\" lors de la réponse. Au lieu de cela, faites des références aux citations pertinentes pour chaque section de la réponse uniquement en ajoutant leurs numéros entre crochets à la fin des phrases pertinentes.  \n  \nAinsi, le format de votre réponse globale devrait ressembler à ce qui est montré entre les balises <example></example>. Assurez-vous de suivre exactement le formatage et l'espacement.  \n<example>  \nCitations :  \n[1] \"La Société X a rapporté un chiffre d'affaires de 12 millions de dollars en 2021.\"  \n[2] \"Près de 90% du chiffre d'affaires provenait des ventes de widgets, les ventes de gadgets représentant les 10% restants.\"  \n  \nRéponse :  \nLa Société X a gagné 12 millions de dollars. [1] Près de 90% provenait des ventes de widgets. [2]  \n</example>  \n  \nSi la question ne peut pas être répondue par le document, dites-le.",
    messages=[
        {
            "role": "user",
            "content": [
                {
                    "type": "text",
                    "text": "Est-ce que Matterport se porte bien ?"
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
  system: "Vous êtes un assistant de recherche expert. Voici un document sur lequel vous répondrez aux questions :  \n<doc>  \n[Texte complet du [dépôt SEC 10-K 2023 de Matterport](https://investors.matterport.com/node/9501/html), non collé ici par souci de brièveté]  \n</doc>  \n  \nD'abord, trouvez les citations du document qui sont les plus pertinentes pour répondre à la question, puis imprimez-les dans l'ordre numéroté. Les citations doivent être relativement courtes.  \n  \nS'il n'y a pas de citations pertinentes, écrivez \"Aucune citation pertinente\" à la place.  \n  \nEnsuite, répondez à la question, en commençant par \"Réponse :\". N'incluez pas ou ne référencez pas le contenu cité textuellement dans la réponse. Ne dites pas \"Selon la Citation [1]\" lors de la réponse. Au lieu de cela, faites des références aux citations pertinentes pour chaque section de la réponse uniquement en ajoutant leurs numéros entre crochets à la fin des phrases pertinentes.  \n  \nAinsi, le format de votre réponse globale devrait ressembler à ce qui est montré entre les balises <example></example>. Assurez-vous de suivre exactement le formatage et l'espacement.  \n<example>  \nCitations :  \n[1] \"La Société X a rapporté un chiffre d'affaires de 12 millions de dollars en 2021.\"  \n[2] \"Près de 90% du chiffre d'affaires provenait des ventes de widgets, les ventes de gadgets représentant les 10% restants.\"  \n  \nRéponse :  \nLa Société X a gagné 12 millions de dollars. [1] Près de 90% provenait des ventes de widgets. [2]  \n</example>  \n  \nSi la question ne peut pas être répondue par le document, dites-le.",
  messages: [
    {
      "role": "user",
      "content": [
        {
          "type": "text",
          "text": "Est-ce que Matterport se porte bien ?"
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
    system="Vous êtes un assistant de recherche expert. Voici un document sur lequel vous répondrez aux questions :  \n<doc>  \n[Texte complet du [dépôt SEC 10-K 2023 de Matterport](https://investors.matterport.com/node/9501/html), non collé ici par souci de brièveté]  \n</doc>  \n  \nD'abord, trouvez les citations du document qui sont les plus pertinentes pour répondre à la question, puis imprimez-les dans l'ordre numéroté. Les citations doivent être relativement courtes.  \n  \nS'il n'y a pas de citations pertinentes, écrivez \"Aucune citation pertinente\" à la place.  \n  \nEnsuite, répondez à la question, en commençant par \"Réponse :\". N'incluez pas ou ne référencez pas le contenu cité textuellement dans la réponse. Ne dites pas \"Selon la Citation [1]\" lors de la réponse. Au lieu de cela, faites des références aux citations pertinentes pour chaque section de la réponse uniquement en ajoutant leurs numéros entre crochets à la fin des phrases pertinentes.  \n  \nAinsi, le format de votre réponse globale devrait ressembler à ce qui est montré entre les balises <example></example>. Assurez-vous de suivre exactement le formatage et l'espacement.  \n<example>  \nCitations :  \n[1] \"La Société X a rapporté un chiffre d'affaires de 12 millions de dollars en 2021.\"  \n[2] \"Près de 90% du chiffre d'affaires provenait des ventes de widgets, les ventes de gadgets représentant les 10% restants.\"  \n  \nRéponse :  \nLa Société X a gagné 12 millions de dollars. [1] Près de 90% provenait des ventes de widgets. [2]  \n</example>  \n  \nSi la question ne peut pas être répondue par le document, dites-le.",
    messages=[
        {
            "role": "user",
            "content": [
                {
                    "type": "text",
                    "text": "Est-ce que Matterport se porte bien ?"
                }
            ]
        }
    ]
)
print(message.content)

```

</Tab>

<Tab title=" Vertex AI TypeScript">

```typescript
import { AnthropicVertex } from '@anthropic-ai/vertex-sdk';

// Reads from the `CLOUD_ML_REGION` & `ANTHROPIC_VERTEX_PROJECT_ID` environment variables.
// Additionally goes through the standard `google-auth-library` flow.
const client = new AnthropicVertex();

const msg = await client.messages.create({
  model: "claude-sonnet-4@20250514",
  max_tokens: 2000,
  temperature: 0,
  system: "Vous êtes un assistant de recherche expert. Voici un document sur lequel vous répondrez aux questions :  \n<doc>  \n[Texte complet du [dépôt SEC 10-K 2023 de Matterport](https://investors.matterport.com/node/9501/html), non collé ici par souci de brièveté]  \n</doc>  \n  \nD'abord, trouvez les citations du document qui sont les plus pertinentes pour répondre à la question, puis imprimez-les dans l'ordre numéroté. Les citations doivent être relativement courtes.  \n  \nS'il n'y a pas de citations pertinentes, écrivez \"Aucune citation pertinente\" à la place.  \n  \nEnsuite, répondez à la question, en commençant par \"Réponse :\". N'incluez pas ou ne référencez pas le contenu cité textuellement dans la réponse. Ne dites pas \"Selon la Citation [1]\" lors de la réponse. Au lieu de cela, faites des références aux citations pertinentes pour chaque section de la réponse uniquement en ajoutant leurs numéros entre crochets à la fin des phrases pertinentes.  \n  \nAinsi, le format de votre réponse globale devrait ressembler à ce qui est montré entre les balises <example></example>. Assurez-vous de suivre exactement le formatage et l'espacement.  \n<example>  \nCitations :  \n[1] \"La Société X a rapporté un chiffre d'affaires de 12 millions de dollars en 2021.\"  \n[2] \"Près de 90% du chiffre d'affaires provenait des ventes de widgets, les ventes de gadgets représentant les 10% restants.\"  \n  \nRéponse :  \nLa Société X a gagné 12 millions de dollars. [1] Près de 90% provenait des ventes de widgets. [2]  \n</example>  \n  \nSi la question ne peut pas être répondue par le document, dites-le.",
  messages: [
    {
      "role": "user",
      "content": [
        {
          "type": "text",
          "text": "Est-ce que Matterport se porte bien ?"
        }
      ]
    }
  ]
});
console.log(msg);

```

</Tab>
</Tabs>