# Navigateur de dilemmes éthiques

Aidez l'utilisateur à réfléchir à des dilemmes éthiques complexes et fournissez différentes perspectives.

---

> Copiez cette invite dans notre [Console](/dashboard) développeur pour l'essayer vous-même !

|        | Contenu |
| --- | --- |
| Système | Aidez l'utilisateur à naviguer dans un dilemme éthique complexe en identifiant les principes éthiques fondamentaux, en explorant différents cadres éthiques, en considérant les conséquences potentielles, en reconnaissant la complexité, en encourageant la réflexion personnelle et en offrant des ressources supplémentaires. Maintenez un ton objectif et sans jugement et mettez l'accent sur la pensée critique, l'empathie et la prise de décision responsable. |
| Utilisateur   | Je suis un journaliste qui a découvert des preuves de corruption impliquant un haut fonctionnaire du gouvernement. Publier l'histoire pourrait conduire à l'indignation publique et aux appels à la démission du fonctionnaire, mais cela pourrait aussi déstabiliser le gouvernement et causer une incertitude économique. Le fonctionnaire m'a offert un pot-de-vin important pour garder l'histoire secrète. Dois-je accepter le pot-de-vin, publier l'histoire, ou trouver une autre solution ? |

## Exemple de sortie

Ce dilemme implique des principes tels que la vérité, l'intérêt public, l'intégrité personnelle et la responsabilité sociale. Différents cadres éthiques fournissent des perspectives :

- Utilitarisme : Publier l'histoire pourrait conduire à un bien-être général plus grand en exposant la corruption et en promouvant une gouvernance responsable, malgré l'instabilité à court terme.
- Déontologie : Accepter un pot-de-vin viole les devoirs éthiques d'honnêteté et d'impartialité. Il peut y avoir une obligation morale de rapporter la vérité, indépendamment des conséquences.
- Éthique de la vertu : Un journaliste vertueux donnerait la priorité à l'intégrité, au courage et à la justice, résistant au gain personnel d'un pot-de-vin et défendant le rôle de la presse dans une société démocratique.
- Éthique du care : Considérez l'impact sur les relations et la confiance, tant avec le public qu'au sein de la communauté journalistique.

Les conséquences à court terme de la publication incluent l'indignation publique, l'instabilité politique et l'incertitude économique. Les implications à long terme peuvent impliquer une gouvernance améliorée, une confiance publique accrue dans les médias et l'intégrité personnelle/professionnelle.

Accepter le pot-de-vin compromet l'éthique journalistique et contribue à une culture de corruption. Cela peut conduire à la culpabilité personnelle, à la perte de crédibilité et à permettre d'autres comportements non éthiques.

Considérez vos valeurs, l'éthique professionnelle et le rôle du journalisme dans la société. Cherchez des conseils auprès des codes de conduite éthiques, des conseillers juridiques et des collègues de confiance.

Les ressources supplémentaires incluent des études de cas de journalistes confrontés à des dilemmes similaires, des directives d'éthique des médias et des discussions sur le rôle de la presse dans la responsabilisation du pouvoir.

En résumé, ce dilemme nécessite de peser l'intérêt public contre l'instabilité potentielle, de considérer l'intégrité professionnelle et de réfléchir aux implications à long terme de votre décision. Efforcez-vous de trouver une solution qui respecte les principes éthiques et le rôle vital du journalisme dans une société démocratique.

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
  temperature=1,
  system="Aidez l'utilisateur à naviguer dans un dilemme éthique complexe en identifiant les principes éthiques fondamentaux, en explorant différents cadres éthiques, en considérant les conséquences potentielles, en reconnaissant la complexité, en encourageant la réflexion personnelle et en offrant des ressources supplémentaires. Maintenez un ton objectif et sans jugement et mettez l'accent sur la pensée critique, l'empathie et la prise de décision responsable.",
  messages=[
    {
      "role": "user",
      "content": [
        {
          "type": "text",
          "text": "Je suis un journaliste qui a découvert des preuves de corruption impliquant un haut fonctionnaire du gouvernement. Publier l'histoire pourrait conduire à l'indignation publique et aux appels à la démission du fonctionnaire, mais cela pourrait aussi déstabiliser le gouvernement et causer une incertitude économique. Le fonctionnaire m'a offert un pot-de-vin important pour garder l'histoire secrète. Dois-je accepter le pot-de-vin, publier l'histoire, ou trouver une autre solution ?"
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
  temperature: 1,
  system: "Aidez l'utilisateur à naviguer dans un dilemme éthique complexe en identifiant les principes éthiques fondamentaux, en explorant différents cadres éthiques, en considérant les conséquences potentielles, en reconnaissant la complexité, en encourageant la réflexion personnelle et en offrant des ressources supplémentaires. Maintenez un ton objectif et sans jugement et mettez l'accent sur la pensée critique, l'empathie et la prise de décision responsable.",
  messages: [
    {
      "role": "user",
      "content": [
        {
          "type": "text",
          "text": "Je suis un journaliste qui a découvert des preuves de corruption impliquant un haut fonctionnaire du gouvernement. Publier l'histoire pourrait conduire à l'indignation publique et aux appels à la démission du fonctionnaire, mais cela pourrait aussi déstabiliser le gouvernement et causer une incertitude économique. Le fonctionnaire m'a offert un pot-de-vin important pour garder l'histoire secrète. Dois-je accepter le pot-de-vin, publier l'histoire, ou trouver une autre solution ?"
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
    max_tokens=2000,
    temperature=1,
    system="Aidez l'utilisateur à naviguer dans un dilemme éthique complexe en identifiant les principes éthiques fondamentaux, en explorant différents cadres éthiques, en considérant les conséquences potentielles, en reconnaissant la complexité, en encourageant la réflexion personnelle et en offrant des ressources supplémentaires. Maintenez un ton objectif et sans jugement et mettez l'accent sur la pensée critique, l'empathie et la prise de décision responsable.",
    messages=[
        {
            "role": "user",
            "content": [
                {
                    "type": "text",
                    "text": "Je suis un journaliste qui a découvert des preuves de corruption impliquant un haut fonctionnaire du gouvernement. Publier l'histoire pourrait conduire à l'indignation publique et aux appels à la démission du fonctionnaire, mais cela pourrait aussi déstabiliser le gouvernement et causer une incertitude économique. Le fonctionnaire m'a offert un pot-de-vin important pour garder l'histoire secrète. Dois-je accepter le pot-de-vin, publier l'histoire, ou trouver une autre solution ?"
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
  temperature: 1,
  system: "Aidez l'utilisateur à naviguer dans un dilemme éthique complexe en identifiant les principes éthiques fondamentaux, en explorant différents cadres éthiques, en considérant les conséquences potentielles, en reconnaissant la complexité, en encourageant la réflexion personnelle et en offrant des ressources supplémentaires. Maintenez un ton objectif et sans jugement et mettez l'accent sur la pensée critique, l'empathie et la prise de décision responsable.",
  messages: [
    {
      "role": "user",
      "content": [
        {
          "type": "text",
          "text": "Je suis un journaliste qui a découvert des preuves de corruption impliquant un haut fonctionnaire du gouvernement. Publier l'histoire pourrait conduire à l'indignation publique et aux appels à la démission du fonctionnaire, mais cela pourrait aussi déstabiliser le gouvernement et causer une incertitude économique. Le fonctionnaire m'a offert un pot-de-vin important pour garder l'histoire secrète. Dois-je accepter le pot-de-vin, publier l'histoire, ou trouver une autre solution ?"
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
    temperature=1,
    system="Aidez l'utilisateur à naviguer dans un dilemme éthique complexe en identifiant les principes éthiques fondamentaux, en explorant différents cadres éthiques, en considérant les conséquences potentielles, en reconnaissant la complexité, en encourageant la réflexion personnelle et en offrant des ressources supplémentaires. Maintenez un ton objectif et sans jugement et mettez l'accent sur la pensée critique, l'empathie et la prise de décision responsable.",
    messages=[
        {
            "role": "user",
            "content": [
                {
                    "type": "text",
                    "text": "Je suis un journaliste qui a découvert des preuves de corruption impliquant un haut fonctionnaire du gouvernement. Publier l'histoire pourrait conduire à l'indignation publique et aux appels à la démission du fonctionnaire, mais cela pourrait aussi déstabiliser le gouvernement et causer une incertitude économique. Le fonctionnaire m'a offert un pot-de-vin important pour garder l'histoire secrète. Dois-je accepter le pot-de-vin, publier l'histoire, ou trouver une autre solution ?"
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
temperature: 1,
system: "Aidez l'utilisateur à naviguer dans un dilemme éthique complexe en identifiant les principes éthiques fondamentaux, en explorant différents cadres éthiques, en considérant les conséquences potentielles, en reconnaissant la complexité, en encourageant la réflexion personnelle et en offrant des ressources supplémentaires. Maintenez un ton objectif et sans jugement et mettez l'accent sur la pensée critique, l'empathie et la prise de décision responsable.",
messages: [
{
"role": "user",
"content": [
{
"type": "text",
"text": "Je suis un journaliste qui a découvert des preuves de corruption impliquant un haut fonctionnaire du gouvernement. Publier l'histoire pourrait conduire à l'indignation publique et aux appels à la démission du fonctionnaire, mais cela pourrait aussi déstabiliser le gouvernement et causer une incertitude économique. Le fonctionnaire m'a offert un pot-de-vin important pour garder l'histoire secrète. Dois-je accepter le pot-de-vin, publier l'histoire, ou trouver une autre solution ?"
}
]
}
]
});
console.log(msg);
```

</Tab>
</Tabs>