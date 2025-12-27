# Maître des mémos

Rédigez des mémos d'entreprise complets basés sur des points clés.

---

> Copiez cette invite dans notre [Console](/dashboard) développeur pour l'essayer vous-même !

|        | Contenu |
| --- | --- |
| System | Votre tâche est de rédiger un mémo d'entreprise complet basé sur les points clés fournis. Le mémo doit être écrit dans un ton professionnel, abordant toutes les informations pertinentes de manière claire et concise. Utilisez un formatage approprié, tel que des titres, des sous-titres et des puces, pour organiser efficacement le contenu. Assurez-vous que le mémo est bien structuré, cohérent et facile à comprendre pour le public visé. |
| User   | - Annonce d'un nouveau programme de bien-être des employés : "Fit4Success" <br/> - Objectifs du programme : promouvoir le bien-être physique et mental, réduire le stress et augmenter la productivité <br/> - Composants : cours de fitness sur site, options de repas sains, ressources de santé mentale et ateliers de bien-être <br/> - Partenariat avec des salles de sport et centres de bien-être locaux pour des adhésions à prix réduit <br/> - Incitations à la participation : trackers de fitness, points de bien-être et tirages au sort de prix <br/> - Date de lancement du programme : 1er juin 2023 <br/> - Processus d'inscription : inscription en ligne via l'intranet de l'entreprise <br/> - Coordinateurs du programme : équipes des Ressources Humaines et de l'Engagement des Employés |

## Exemple de sortie

Objet : Présentation de Fit4Success - Notre nouveau programme de bien-être des employés

Bonjour à tous,

Je suis ravi d'annoncer le lancement de notre nouveau programme de bien-être des employés, Fit4Success, conçu pour promouvoir le bien-être physique et mental, réduire le stress et augmenter la productivité parmi les membres de notre équipe. Le programme commencera officiellement le 1er juin 2023.

Objectifs du programme :

- Encourager l'activité physique régulière et les choix de mode de vie sain
- Fournir des ressources et un soutien pour la santé mentale et la gestion du stress
- Favoriser un environnement de travail positif et engageant qui priorise le bien-être des employés
- Améliorer la productivité globale et la satisfaction au travail

Composants du programme :

- Cours de fitness sur site : séances de yoga, Pilates et entraînement par intervalles à haute intensité (HIIT)
- Options de repas sains : collations et repas nutritifs disponibles dans la cafétéria de l'entreprise
- Ressources de santé mentale : accès aux services de conseil, applications de méditation et ateliers de soulagement du stress
- Ateliers de bien-être : séminaires sur des sujets tels que la nutrition, l'hygiène du sommeil et l'équilibre travail-vie personnelle

Partenariats et réductions :  
 Pour soutenir davantage votre parcours de bien-être, nous nous sommes associés avec des salles de sport et centres de bien-être locaux pour offrir des adhésions à prix réduit exclusivement pour nos employés. Profitez de ces tarifs spéciaux pour accéder à une large gamme d'installations de fitness et de cours.

Incitations à la participation :  
 Pour encourager la participation active au programme Fit4Success, nous offrirons les incitations suivantes :

- Trackers de fitness pour tous les participants pour surveiller leurs progrès et fixer des objectifs personnels
- Points de bien-être gagnés grâce à la participation au programme, échangeables contre des récompenses supplémentaires
- Tirages au sort mensuels de prix pour les participants qui atteignent des jalons de bien-être spécifiques

Processus d'inscription :  
 Pour vous inscrire au programme Fit4Success, veuillez visiter notre intranet d'entreprise et compléter le formulaire d'inscription en ligne. La date limite d'inscription est le 25 mai 2023. Si vous avez des questions ou des préoccupations concernant le processus d'inscription, veuillez contacter notre équipe des Ressources Humaines.

Coordination du programme :  
 Le programme Fit4Success sera coordonné conjointement par nos équipes des Ressources Humaines et de l'Engagement des Employés. Elles seront disponibles pour fournir des conseils, un soutien et des ressources tout au long de votre parcours de bien-être.

Nous croyons fermement qu'investir dans le bien-être de nos employés est crucial pour le succès et la croissance de notre organisation. Nous vous encourageons à profiter de cette opportunité passionnante et à nous rejoindre pour créer un lieu de travail plus sain, plus heureux et plus productif.

Embrassons le programme Fit4Success et travaillons ensemble vers un avenir plus sain !

Meilleures salutations,  
 \[Votre nom\]  
 \[Votre titre\]

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
    system="Votre tâche est de rédiger un mémo d'entreprise complet basé sur les points clés fournis. Le mémo doit être écrit dans un ton professionnel, abordant toutes les informations pertinentes de manière claire et concise. Utilisez un formatage approprié, tel que des titres, des sous-titres et des puces, pour organiser efficacement le contenu. Assurez-vous que le mémo est bien structuré, cohérent et facile à comprendre pour le public visé.",
    messages=[
        {
            "role": "user",
            "content": [
                {
                    "type": "text",
                    "text": "- Annonce d'un nouveau programme de bien-être des employés : \"Fit4Success\"  \n- Objectifs du programme : promouvoir le bien-être physique et mental, réduire le stress et augmenter la productivité  \n- Composants : cours de fitness sur site, options de repas sains, ressources de santé mentale et ateliers de bien-être  \n- Partenariat avec des salles de sport et centres de bien-être locaux pour des adhésions à prix réduit  \n- Incitations à la participation : trackers de fitness, points de bien-être et tirages au sort de prix  \n- Date de lancement du programme : 1er juin 2023  \n- Processus d'inscription : inscription en ligne via l'intranet de l'entreprise  \n- Coordinateurs du programme : équipes des Ressources Humaines et de l'Engagement des Employés"
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
  system: "Votre tâche est de rédiger un mémo d'entreprise complet basé sur les points clés fournis. Le mémo doit être écrit dans un ton professionnel, abordant toutes les informations pertinentes de manière claire et concise. Utilisez un formatage approprié, tel que des titres, des sous-titres et des puces, pour organiser efficacement le contenu. Assurez-vous que le mémo est bien structuré, cohérent et facile à comprendre pour le public visé.",
  messages: [
    {
      "role": "user",
      "content": [
        {
          "type": "text",
          "text": "- Annonce d'un nouveau programme de bien-être des employés : \"Fit4Success\"  \n- Objectifs du programme : promouvoir le bien-être physique et mental, réduire le stress et augmenter la productivité  \n- Composants : cours de fitness sur site, options de repas sains, ressources de santé mentale et ateliers de bien-être  \n- Partenariat avec des salles de sport et centres de bien-être locaux pour des adhésions à prix réduit  \n- Incitations à la participation : trackers de fitness, points de bien-être et tirages au sort de prix  \n- Date de lancement du programme : 1er juin 2023  \n- Processus d'inscription : inscription en ligne via l'intranet de l'entreprise  \n- Coordinateurs du programme : équipes des Ressources Humaines et de l'Engagement des Employés"
        }
      ]
    }
  ]
});
console.log(msg);

```

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
system="Votre tâche est de rédiger un mémo d'entreprise complet basé sur les points clés fournis. Le mémo doit être écrit dans un ton professionnel, abordant toutes les informations pertinentes de manière claire et concise. Utilisez un formatage approprié, tel que des titres, des sous-titres et des puces, pour organiser efficacement le contenu. Assurez-vous que le mémo est bien structuré, cohérent et facile à comprendre pour le public visé.",
messages=[
{
"role": "user",
"content": [
{
"type": "text",
"text": "- Annonce d'un nouveau programme de bien-être des employés : \"Fit4Success\" \n- Objectifs du programme : promouvoir le bien-être physique et mental, réduire le stress et augmenter la productivité \n- Composants : cours de fitness sur site, options de repas sains, ressources de santé mentale et ateliers de bien-être \n- Partenariat avec des salles de sport et centres de bien-être locaux pour des adhésions à prix réduit \n- Incitations à la participation : trackers de fitness, points de bien-être et tirages au sort de prix \n- Date de lancement du programme : 1er juin 2023 \n- Processus d'inscription : inscription en ligne via l'intranet de l'entreprise \n- Coordinateurs du programme : équipes des Ressources Humaines et de l'Engagement des Employés"
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
  system: "Votre tâche est de rédiger un mémo d'entreprise complet basé sur les points clés fournis. Le mémo doit être écrit dans un ton professionnel, abordant toutes les informations pertinentes de manière claire et concise. Utilisez un formatage approprié, tel que des titres, des sous-titres et des puces, pour organiser efficacement le contenu. Assurez-vous que le mémo est bien structuré, cohérent et facile à comprendre pour le public visé.",
  messages: [
    {
      "role": "user",
      "content": [
        {
          "type": "text",
          "text": "- Annonce d'un nouveau programme de bien-être des employés : \"Fit4Success\"  \n- Objectifs du programme : promouvoir le bien-être physique et mental, réduire le stress et augmenter la productivité  \n- Composants : cours de fitness sur site, options de repas sains, ressources de santé mentale et ateliers de bien-être  \n- Partenariat avec des salles de sport et centres de bien-être locaux pour des adhésions à prix réduit  \n- Incitations à la participation : trackers de fitness, points de bien-être et tirages au sort de prix  \n- Date de lancement du programme : 1er juin 2023  \n- Processus d'inscription : inscription en ligne via l'intranet de l'entreprise  \n- Coordinateurs du programme : équipes des Ressources Humaines et de l'Engagement des Employés"
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
    system="Votre tâche est de rédiger un mémo d'entreprise complet basé sur les points clés fournis. Le mémo doit être écrit dans un ton professionnel, abordant toutes les informations pertinentes de manière claire et concise. Utilisez un formatage approprié, tel que des titres, des sous-titres et des puces, pour organiser efficacement le contenu. Assurez-vous que le mémo est bien structuré, cohérent et facile à comprendre pour le public visé.",
    messages=[
        {
            "role": "user",
            "content": [
                {
                    "type": "text",
                    "text": "- Annonce d'un nouveau programme de bien-être des employés : \"Fit4Success\"  \n- Objectifs du programme : promouvoir le bien-être physique et mental, réduire le stress et augmenter la productivité  \n- Composants : cours de fitness sur site, options de repas sains, ressources de santé mentale et ateliers de bien-être  \n- Partenariat avec des salles de sport et centres de bien-être locaux pour des adhésions à prix réduit  \n- Incitations à la participation : trackers de fitness, points de bien-être et tirages au sort de prix  \n- Date de lancement du programme : 1er juin 2023  \n- Processus d'inscription : inscription en ligne via l'intranet de l'entreprise  \n- Coordinateurs du programme : équipes des Ressources Humaines et de l'Engagement des Employés"
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
  system: "Votre tâche est de rédiger un mémo d'entreprise complet basé sur les points clés fournis. Le mémo doit être écrit dans un ton professionnel, abordant toutes les informations pertinentes de manière claire et concise. Utilisez un formatage approprié, tel que des titres, des sous-titres et des puces, pour organiser efficacement le contenu. Assurez-vous que le mémo est bien structuré, cohérent et facile à comprendre pour le public visé.",
  messages: [
    {
      "role": "user",
      "content": [
        {
          "type": "text",
          "text": "- Annonce d'un nouveau programme de bien-être des employés : \"Fit4Success\"  \n- Objectifs du programme : promouvoir le bien-être physique et mental, réduire le stress et augmenter la productivité  \n- Composants : cours de fitness sur site, options de repas sains, ressources de santé mentale et ateliers de bien-être  \n- Partenariat avec des salles de sport et centres de bien-être locaux pour des adhésions à prix réduit  \n- Incitations à la participation : trackers de fitness, points de bien-être et tirages au sort de prix  \n- Date de lancement du programme : 1er juin 2023  \n- Processus d'inscription : inscription en ligne via l'intranet de l'entreprise  \n- Coordinateurs du programme : équipes des Ressources Humaines et de l'Engagement des Employés"
        }
      ]
    }
  ]
});
console.log(msg);

```

</Tab>
</Tabs>