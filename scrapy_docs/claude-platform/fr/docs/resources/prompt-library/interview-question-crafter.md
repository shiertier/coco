# Créateur de questions d'entretien

Générer des questions pour les entretiens.

---

> Copiez cette invite dans notre [Console](/dashboard) développeur pour l'essayer vous-même !

|        | Contenu |
| --- | --- |
| System | Votre tâche est de générer une série de questions réfléchies et ouvertes pour un entretien basé sur le contexte donné. Les questions doivent être conçues pour susciter des réponses perspicaces et détaillées de la part de la personne interrogée, lui permettant de mettre en valeur ses connaissances, son expérience et ses compétences de pensée critique. Évitez les questions oui/non ou celles avec des réponses évidentes. Concentrez-vous plutôt sur des questions qui encouragent la réflexion, l'auto-évaluation et le partage d'exemples ou d'anecdotes spécifiques. |
| User   | Vous menez un entretien avec un candidat pour un poste de responsable marketing dans une entreprise de commerce électronique bien établie. Le candidat idéal devrait avoir de l'expérience dans le développement et l'exécution de campagnes marketing multi-canaux, de solides compétences analytiques et la capacité de collaborer efficacement avec des équipes transversales. Il devrait également être passionné par le fait de rester à jour avec les dernières tendances et technologies marketing. |

## Exemple de sortie

1.  Pouvez-vous me présenter une campagne marketing multi-canaux réussie que vous avez développée et exécutée dans votre rôle précédent ? Quels ont été les défis auxquels vous avez fait face, et comment les avez-vous surmontés ?
2.  Comment mesurez-vous le succès de vos campagnes marketing, et sur quelles métriques vous concentrez-vous ? Pouvez-vous fournir un exemple de la façon dont vous avez utilisé les données et l'analyse pour optimiser les performances d'une campagne ?
3.  D'après votre expérience, quels sont les éléments clés d'une stratégie marketing e-commerce réussie ? Comment vous assurez-vous que vos stratégies s'alignent avec les objectifs commerciaux globaux de l'entreprise ?
4.  Pouvez-vous partager un exemple d'une fois où vous avez dû collaborer avec une équipe transversale (par exemple, produit, ventes ou service client) pour atteindre un objectif marketing ? Quels défis avez-vous rencontrés, et comment avez-vous favorisé une collaboration efficace ?
5.  Comment restez-vous informé des dernières tendances et technologies marketing, et comment avez-vous appliqué ces connaissances pour améliorer vos stratégies ou campagnes marketing ?
6.  Pouvez-vous décrire une fois où une campagne marketing à laquelle vous avez participé n'a pas performé comme prévu ? Qu'avez-vous appris de cette expérience, et comment avez-vous appliqué ces leçons aux campagnes futures ?
7.  Notre entreprise valorise l'innovation et l'amélioration continue. Pouvez-vous partager un exemple d'une fois où vous avez introduit une nouvelle approche marketing, un outil ou une technologie qui a significativement impacté les performances ou les résultats de votre équipe ?

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
  temperature=0.5,
  system="Votre tâche est de générer une série de questions réfléchies et ouvertes pour un entretien basé sur le contexte donné. Les questions doivent être conçues pour susciter des réponses perspicaces et détaillées de la part de la personne interrogée, lui permettant de mettre en valeur ses connaissances, son expérience et ses compétences de pensée critique. Évitez les questions oui/non ou celles avec des réponses évidentes. Concentrez-vous plutôt sur des questions qui encouragent la réflexion, l'auto-évaluation et le partage d'exemples ou d'anecdotes spécifiques.",
  messages=[
    {
      "role": "user",
      "content": [
        {
          "type": "text",
          "text": "Vous menez un entretien avec un candidat pour un poste de responsable marketing dans une entreprise de commerce électronique bien établie. Le candidat idéal devrait avoir de l'expérience dans le développement et l'exécution de campagnes marketing multi-canaux, de solides compétences analytiques et la capacité de collaborer efficacement avec des équipes transversales. Il devrait également être passionné par le fait de rester à jour avec les dernières tendances et technologies marketing."
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
  temperature: 0.5,
  system: "Votre tâche est de générer une série de questions réfléchies et ouvertes pour un entretien basé sur le contexte donné. Les questions doivent être conçues pour susciter des réponses perspicaces et détaillées de la part de la personne interrogée, lui permettant de mettre en valeur ses connaissances, son expérience et ses compétences de pensée critique. Évitez les questions oui/non ou celles avec des réponses évidentes. Concentrez-vous plutôt sur des questions qui encouragent la réflexion, l'auto-évaluation et le partage d'exemples ou d'anecdotes spécifiques.",
  messages: [
    {
      "role": "user",
      "content": [
        {
          "type": "text",
          "text": "Vous menez un entretien avec un candidat pour un poste de responsable marketing dans une entreprise de commerce électronique bien établie. Le candidat idéal devrait avoir de l'expérience dans le développement et l'exécution de campagnes marketing multi-canaux, de solides compétences analytiques et la capacité de collaborer efficacement avec des équipes transversales. Il devrait également être passionné par le fait de rester à jour avec les dernières tendances et technologies marketing."
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
    temperature=0.5,
    system="Votre tâche est de générer une série de questions réfléchies et ouvertes pour un entretien basé sur le contexte donné. Les questions doivent être conçues pour susciter des réponses perspicaces et détaillées de la part de la personne interrogée, lui permettant de mettre en valeur ses connaissances, son expérience et ses compétences de pensée critique. Évitez les questions oui/non ou celles avec des réponses évidentes. Concentrez-vous plutôt sur des questions qui encouragent la réflexion, l'auto-évaluation et le partage d'exemples ou d'anecdotes spécifiques.",
    messages=[
        {
            "role": "user",
            "content": [
                {
                    "type": "text",
                    "text": "Vous menez un entretien avec un candidat pour un poste de responsable marketing dans une entreprise de commerce électronique bien établie. Le candidat idéal devrait avoir de l'expérience dans le développement et l'exécution de campagnes marketing multi-canaux, de solides compétences analytiques et la capacité de collaborer efficacement avec des équipes transversales. Il devrait également être passionné par le fait de rester à jour avec les dernières tendances et technologies marketing."
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
temperature: 0.5,
system: "Votre tâche est de générer une série de questions réfléchies et ouvertes pour un entretien basé sur le contexte donné. Les questions doivent être conçues pour susciter des réponses perspicaces et détaillées de la part de la personne interrogée, lui permettant de mettre en valeur ses connaissances, son expérience et ses compétences de pensée critique. Évitez les questions oui/non ou celles avec des réponses évidentes. Concentrez-vous plutôt sur des questions qui encouragent la réflexion, l'auto-évaluation et le partage d'exemples ou d'anecdotes spécifiques.",
messages: [
{
"role": "user",
"content": [
{
"type": "text",
"text": "Vous menez un entretien avec un candidat pour un poste de responsable marketing dans une entreprise de commerce électronique bien établie. Le candidat idéal devrait avoir de l'expérience dans le développement et l'exécution de campagnes marketing multi-canaux, de solides compétences analytiques et la capacité de collaborer efficacement avec des équipes transversales. Il devrait également être passionné par le fait de rester à jour avec les dernières tendances et technologies marketing."
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
    max_tokens=1000,
    temperature=0.5,
    system="Votre tâche est de générer une série de questions réfléchies et ouvertes pour un entretien basé sur le contexte donné. Les questions doivent être conçues pour susciter des réponses perspicaces et détaillées de la part de la personne interrogée, lui permettant de mettre en valeur ses connaissances, son expérience et ses compétences de pensée critique. Évitez les questions oui/non ou celles avec des réponses évidentes. Concentrez-vous plutôt sur des questions qui encouragent la réflexion, l'auto-évaluation et le partage d'exemples ou d'anecdotes spécifiques.",
    messages=[
        {
            "role": "user",
            "content": [
                {
                    "type": "text",
                    "text": "Vous menez un entretien avec un candidat pour un poste de responsable marketing dans une entreprise de commerce électronique bien établie. Le candidat idéal devrait avoir de l'expérience dans le développement et l'exécution de campagnes marketing multi-canaux, de solides compétences analytiques et la capacité de collaborer efficacement avec des équipes transversales. Il devrait également être passionné par le fait de rester à jour avec les dernières tendances et technologies marketing."
                }
            ]
        }
    ]
)
print(message.content)

````

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
  temperature: 0.5,
  system: "Votre tâche est de générer une série de questions réfléchies et ouvertes pour un entretien basé sur le contexte donné. Les questions doivent être conçues pour susciter des réponses perspicaces et détaillées de la part de la personne interrogée, lui permettant de mettre en valeur ses connaissances, son expérience et ses compétences de pensée critique. Évitez les questions oui/non ou celles avec des réponses évidentes. Concentrez-vous plutôt sur des questions qui encouragent la réflexion, l'auto-évaluation et le partage d'exemples ou d'anecdotes spécifiques.",
  messages: [
    {
      "role": "user",
      "content": [
        {
          "type": "text",
          "text": "Vous menez un entretien avec un candidat pour un poste de responsable marketing dans une entreprise de commerce électronique bien établie. Le candidat idéal devrait avoir de l'expérience dans le développement et l'exécution de campagnes marketing multi-canaux, de solides compétences analytiques et la capacité de collaborer efficacement avec des équipes transversales. Il devrait également être passionné par le fait de rester à jour avec les dernières tendances et technologies marketing."
        }
      ]
    }
  ]
});
console.log(msg);

```

</Tab>
</Tabs>