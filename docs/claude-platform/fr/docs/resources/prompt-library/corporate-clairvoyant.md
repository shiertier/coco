# Voyant d'entreprise

Extraire des insights, identifier les risques et distiller les informations clés de longs rapports d'entreprise en un seul mémo.

---

> Copiez ce prompt dans notre [Console](/dashboard) développeur pour l'essayer vous-même !

|      | Contenu |
| ---- | --- |
| Utilisateur | Votre tâche est d'analyser le rapport suivant : <br /> <br /> <report> [Texte complet du [dépôt SEC 10-K 2023 de Matterport](https://investors.matterport.com/node/9501/html), non collé ici par souci de brièveté] </report> <br /> <br /> Résumez ce rapport annuel de manière concise et claire, et identifiez les tendances clés du marché et les points à retenir. Présentez vos conclusions sous forme d'un court mémo que je peux envoyer à mon équipe. L'objectif du mémo est de s'assurer que mon équipe reste informée de la situation des institutions financières et de prévoir qualitativement et d'identifier s'il y a des risques opérationnels et de revenus à prévoir pour le trimestre à venir. Assurez-vous d'inclure tous les détails pertinents dans votre résumé et votre analyse. |

## Exemple de Sortie

> À : Équipe d'Investissement  
> De : \[Votre Nom\]  
> Date : 4 mars 2024  
> Objet : Points saillants du 10-K 2022 de Matterport et Perspectives Financières
>
> Bonjour l'équipe,
>
> J'ai identifié quelques points clés à retenir du rapport annuel 2022 de Matterport :
>
> Performance Financière :
>
> - Les revenus ont augmenté de 22% à 136,1 M$ en 2022, portés par la croissance des revenus d'abonnement et de services
> - Cependant, la marge brute a diminué de 54% à 38% en raison de l'augmentation des coûts liés aux revenus de produits et des défis de la chaîne d'approvisionnement
> - Les pertes opérationnelles se sont creusées à 275,5 M$ alors que l'entreprise investissait massivement en R&D et en ventes et marketing pour stimuler la croissance
> - A terminé 2022 avec 476,9 M$ en liquidités et investissements pour financer les opérations
>
> Points Saillants de l'Activité :
>
> - La base d'abonnés a augmenté de 39% à plus de 701 000 à la fin de 2022. Les espaces sous gestion ont atteint 9,2 millions
> - Expansion de la plateforme avec le lancement du support motorisé Matterport Axis, de la caméra Pro3, et des mises à jour majeures du logiciel cloud
> - Acquisitions complétées d'Enview (IA pour données spatiales 3D) et VHT Studios (marketing immobilier) pour améliorer les offres
> - Partenariats notables signés avec Burns & McDonnell et CGS pour livrer les solutions de Matterport à leurs clients entreprise
>
> Perspectives :
>
> - Matterport voit une opportunité de marché massive dans la numérisation des bâtiments et espaces du monde (estime un TAM de 240 Md$+)
> - Concentré sur l'adoption par de gros clients entreprise dans des secteurs clés comme l'immobilier, l'AEC, la gestion d'installations, et plus
> - Continuera d'investir en R&D et expansion internationale pour pénétrer le marché mondial des bâtiments largement inexploité
> - La croissance et les marges à court terme peuvent être impactées par les vents contraires macroéconomiques et les problèmes de chaîne d'approvisionnement
> - Cependant, une position de liquidité saine et une base de clients relativement fidèle offrent une protection contre les risques de baisse
>
> En résumé, Matterport a livré une forte croissance du chiffre d'affaires en 2022 alors qu'elle progressait dans la pénétration d'un énorme marché mondial. Bien que la rentabilité reste difficile alors que l'entreprise investit massivement pour la croissance, son bilan semble solide. Mis à part quelques problèmes persistants de chaîne d'approvisionnement, l'élan commercial sous-jacent semble intact basé sur les métriques opérationnelles clés. Sauf récession économique sévère, Matterport semble bien positionnée pour continuer à gagner des parts dans l'espace naissant de la numérisation des bâtiments.

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
    temperature=0,
    messages=[
        {
            "role": "user",
            "content": [
                {
                    "type": "text",
                    "text": "Votre tâche est d'analyser le rapport suivant :  \n<report>  \n[Texte complet du [dépôt SEC 10-K 2023 de Matterport](https://investors.matterport.com/node/9501/html), non collé ici par souci de brièveté]  \n</report>  \n  \nRésumez ce rapport annuel de manière concise et claire, et identifiez les tendances clés du marché et les points à retenir. Présentez vos conclusions sous forme d'un court mémo que je peux envoyer à mon équipe. L'objectif du mémo est de s'assurer que mon équipe reste informée de la situation des institutions financières et de prévoir qualitativement et d'identifier s'il y a des risques opérationnels et de revenus à prévoir pour le trimestre à venir. Assurez-vous d'inclure tous les détails pertinents dans votre résumé et votre analyse."
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
  temperature: 0,
  messages: [
    {
      "role": "user",
      "content": [
        {
          "type": "text",
          "text": "Votre tâche est d'analyser le rapport suivant :  \n<report>  \n[Texte complet du [dépôt SEC 10-K 2023 de Matterport](https://investors.matterport.com/node/9501/html), non collé ici par souci de brièveté]  \n</report>  \n  \nRésumez ce rapport annuel de manière concise et claire, et identifiez les tendances clés du marché et les points à retenir. Présentez vos conclusions sous forme d'un court mémo que je peux envoyer à mon équipe. L'objectif du mémo est de s'assurer que mon équipe reste informée de la situation des institutions financières et de prévoir qualitativement et d'identifier s'il y a des risques opérationnels et de revenus à prévoir pour le trimestre à venir. Assurez-vous d'inclure tous les détails pertinents dans votre résumé et votre analyse."
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
"text": "Votre tâche est d'analyser le rapport suivant : \n<report> \n[Texte complet du [dépôt SEC 10-K 2023 de Matterport](https://investors.matterport.com/node/9501/html), non collé ici par souci de brièveté] \n</report> \n \nRésumez ce rapport annuel de manière concise et claire, et identifiez les tendances clés du marché et les points à retenir. Présentez vos conclusions sous forme d'un court mémo que je peux envoyer à mon équipe. L'objectif du mémo est de s'assurer que mon équipe reste informée de la situation des institutions financières et de prévoir qualitativement et d'identifier s'il y a des risques opérationnels et de revenus à prévoir pour le trimestre à venir. Assurez-vous d'inclure tous les détails pertinents dans votre résumé et votre analyse."
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
  temperature: 0,
  messages: [
    {
      "role": "user",
      "content": [
        {
          "type": "text",
          "text": "Votre tâche est d'analyser le rapport suivant :  \n<report>  \n[Texte complet du [dépôt SEC 10-K 2023 de Matterport](https://investors.matterport.com/node/9501/html), non collé ici par souci de brièveté]  \n</report>  \n  \nRésumez ce rapport annuel de manière concise et claire, et identifiez les tendances clés du marché et les points à retenir. Présentez vos conclusions sous forme d'un court mémo que je peux envoyer à mon équipe. L'objectif du mémo est de s'assurer que mon équipe reste informée de la situation des institutions financières et de prévoir qualitativement et d'identifier s'il y a des risques opérationnels et de revenus à prévoir pour le trimestre à venir. Assurez-vous d'inclure tous les détails pertinents dans votre résumé et votre analyse."
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
    temperature=0,
    messages=[
        {
            "role": "user",
            "content": [
                {
                    "type": "text",
                    "text": "Votre tâche est d'analyser le rapport suivant :  \n<report>  \n[Texte complet du [dépôt SEC 10-K 2023 de Matterport](https://investors.matterport.com/node/9501/html), non collé ici par souci de brièveté]  \n</report>  \n  \nRésumez ce rapport annuel de manière concise et claire, et identifiez les tendances clés du marché et les points à retenir. Présentez vos conclusions sous forme d'un court mémo que je peux envoyer à mon équipe. L'objectif du mémo est de s'assurer que mon équipe reste informée de la situation des institutions financières et de prévoir qualitativement et d'identifier s'il y a des risques opérationnels et de revenus à prévoir pour le trimestre à venir. Assurez-vous d'inclure tous les détails pertinents dans votre résumé et votre analyse."
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
          "text": "Votre tâche est d'analyser le rapport suivant :  \n<report>  \n[Texte complet du [dépôt SEC 10-K 2023 de Matterport](https://investors.matterport.com/node/9501/html), non collé ici par souci de brièveté]  \n</report>  \n  \nRésumez ce rapport annuel de manière concise et claire, et identifiez les tendances clés du marché et les points à retenir. Présentez vos conclusions sous forme d'un court mémo que je peux envoyer à mon équipe. L'objectif du mémo est de s'assurer que mon équipe reste informée de la situation des institutions financières et de prévoir qualitativement et d'identifier s'il y a des risques opérationnels et de revenus à prévoir pour le trimestre à venir. Assurez-vous d'inclure tous les détails pertinents dans votre résumé et votre analyse."
        }
      ]
    }
  ]
});
console.log(msg);

```

  </Tab>
</Tabs>