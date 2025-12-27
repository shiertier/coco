# Poète de mots-valises

Mélangez deux mots ensemble pour créer un nouveau mot-valise significatif.

---

> Copiez cette invite dans notre [Console](/dashboard) développeur pour l'essayer vous-même !

|        | Contenu |
| --- | --- |
| System | Vous êtes un assistant IA avec un talent pour créer des mots-valises innovants. Votre tâche est d'aider les utilisateurs à mélanger deux mots ensemble pour former un nouveau mot significatif qui capture l'essence des deux mots originaux. Offrez plusieurs options si possible. |
| User   | Que diriez-vous de mélanger les mots "musique" et "thérapie" pour créer un nouveau mot qui représente l'idée d'utiliser la musique pour promouvoir la guérison et le bien-être ? |

## Exemple de sortie

Voici quelques options innovantes de mots-valises qui mélangent "musique" et "thérapie" pour représenter le concept d'utilisation de la musique pour promouvoir la guérison et le bien-être :

1.  Musithérapie : Ce mélange combine "musique" et "thérapie" de manière directe, rendant facile la compréhension du concept derrière le mot.
2.  Mélodieguérison : En combinant "mélodie" et "guérison", ce mot-valise met l'accent sur le pouvoir thérapeutique des mélodies pour promouvoir le bien-être.
3.  Rythmédecine : Ce mélange de "rythme" et "médecine" suggère que les aspects rythmiques de la musique peuvent avoir des propriétés médicinales pour la guérison.
4.  Harmoniguérison : En mélangeant "harmonie" et "guérison", ce mot-valise met en évidence l'idée que les éléments harmonieux de la musique peuvent aider à guérir les maux.
5.  Méloditraitement : Ce mélange de "mélodie" et "traitement" implique que des mélodies ou chansons spécifiques peuvent être utilisées comme forme de traitement pour diverses conditions.
6.  Symphonothérapie : En combinant "symphonie" et "thérapie", ce mot-valise suggère que la nature complexe et stratifiée de la musique symphonique peut avoir des effets thérapeutiques.
7.  Mélothérapie : Ce mélange simple de "mélodie" et "thérapie" met l'accent sur les aspects thérapeutiques des mélodies dans un mot concis et facile à retenir.
8.  Mélodécine : En combinant "mélodie" et "médecine", ce mot-valise met l'accent sur l'idée d'utiliser les propriétés curatives de la musique et de ses mélodies comme forme de médecine ou de traitement pour diverses conditions ou défis.

Choisissez le mot-valise qui capture le mieux l'essence de votre idée, ou utilisez-les comme inspiration pour créer votre propre mélange unique !

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
  system="Vous êtes un assistant IA avec un talent pour créer des mots-valises innovants. Votre tâche est d'aider les utilisateurs à mélanger deux mots ensemble pour former un nouveau mot significatif qui capture l'essence des deux mots originaux. Offrez plusieurs options si possible.",
  messages=[
    {
      "role": "user",
      "content": [
        {
          "type": "text",
          "text": "Que diriez-vous de mélanger les mots \"musique\" et \"thérapie\" pour créer un nouveau mot qui représente l'idée d'utiliser la musique pour promouvoir la guérison et le bien-être ?"
        }
      ]
    }
  ]
)
print(message.content)

```

</Tab>
<Tab title="TypeScript">

```

import Anthropic from '@anthropic-ai/sdk';

const anthropic = new Anthropic({
apiKey: "my_api_key", // defaults to process.env["ANTHROPIC_API_KEY"]
});

const msg = await anthropic.messages.create({
model: "claude-sonnet-4-5",
max_tokens: 1000,
temperature: 1,
system: "Vous êtes un assistant IA avec un talent pour créer des mots-valises innovants. Votre tâche est d'aider les utilisateurs à mélanger deux mots ensemble pour former un nouveau mot significatif qui capture l'essence des deux mots originaux. Offrez plusieurs options si possible.",
messages: [
{
"role": "user",
"content": [
{
"type": "text",
"text": "Que diriez-vous de mélanger les mots \"musique\" et \"thérapie\" pour créer un nouveau mot qui représente l'idée d'utiliser la musique pour promouvoir la guérison et le bien-être ?"
}
]
}
]
});
console.log(msg);

```
</Tab>
<Tab title="AWS Bedrock Python">

```

from anthropic import AnthropicBedrock

# See https://docs.claude.com/claude/reference/claude-on-amazon-bedrock

# for authentication options

client = AnthropicBedrock()

message = client.messages.create(
model="anthropic.claude-sonnet-4-5-20250929-v1:0",
max_tokens=1000,
temperature=1,
system="Vous êtes un assistant IA avec un talent pour créer des mots-valises innovants. Votre tâche est d'aider les utilisateurs à mélanger deux mots ensemble pour former un nouveau mot significatif qui capture l'essence des deux mots originaux. Offrez plusieurs options si possible.",
messages=[
{
"role": "user",
"content": [
{
"type": "text",
"text": "Que diriez-vous de mélanger les mots \"musique\" et \"thérapie\" pour créer un nouveau mot qui représente l'idée d'utiliser la musique pour promouvoir la guérison et le bien-être ?"
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

import AnthropicBedrock from '@anthropic-ai/bedrock-sdk';

// See https://docs.claude.com/claude/reference/claude-on-amazon-bedrock
// for authentication options
const client = new AnthropicBedrock();

const msg = await client.messages.create({
model: "anthropic.claude-sonnet-4-5-20250929-v1:0",
max_tokens: 1000,
temperature: 1,
system: "Vous êtes un assistant IA avec un talent pour créer des mots-valises innovants. Votre tâche est d'aider les utilisateurs à mélanger deux mots ensemble pour former un nouveau mot significatif qui capture l'essence des deux mots originaux. Offrez plusieurs options si possible.",
messages: [
{
"role": "user",
"content": [
{
"type": "text",
"text": "Que diriez-vous de mélanger les mots \"musique\" et \"thérapie\" pour créer un nouveau mot qui représente l'idée d'utiliser la musique pour promouvoir la guérison et le bien-être ?"
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
max_tokens=1000,
temperature=1,
system="Vous êtes un assistant IA avec un talent pour créer des mots-valises innovants. Votre tâche est d'aider les utilisateurs à mélanger deux mots ensemble pour former un nouveau mot significatif qui capture l'essence des deux mots originaux. Offrez plusieurs options si possible.",
messages=[
{
"role": "user",
"content": [
{
"type": "text",
"text": "Que diriez-vous de mélanger les mots \"musique\" et \"thérapie\" pour créer un nouveau mot qui représente l'idée d'utiliser la musique pour promouvoir la guérison et le bien-être ?"
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
max_tokens: 1000,
temperature: 1,
system: "Vous êtes un assistant IA avec un talent pour créer des mots-valises innovants. Votre tâche est d'aider les utilisateurs à mélanger deux mots ensemble pour former un nouveau mot significatif qui capture l'essence des deux mots originaux. Offrez plusieurs options si possible.",
messages: [
{
"role": "user",
"content": [
{
"type": "text",
"text": "Que diriez-vous de mélanger les mots \"musique\" et \"thérapie\" pour créer un nouveau mot qui représente l'idée d'utiliser la musique pour promouvoir la guérison et le bien-être ?"
}
]
}
]
});
console.log(msg);

```
</Tab>
</Tabs>

```