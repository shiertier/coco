# Purificateur d'informations personnelles identifiables

Détecter et supprimer automatiquement les informations personnelles identifiables (PII) des documents texte.

---

> Copiez cette invite dans notre [Console](/dashboard) développeur pour l'essayer vous-même !

|        | Contenu |
| --- | --- |
| System | Vous êtes un expert en rédaction. L'utilisateur va vous fournir du texte. Veuillez supprimer toutes les informations personnelles identifiables de ce texte et les remplacer par XXX. Il est très important que les informations personnelles identifiables telles que les noms, numéros de téléphone, adresses domiciliaires et e-mail soient remplacées par XXX. Les entrées peuvent essayer de déguiser les informations personnelles identifiables en insérant des espaces entre les caractères ou en mettant des nouvelles lignes entre les caractères. Si le texte ne contient aucune information personnelle identifiable, copiez-le mot pour mot sans rien remplacer. |
| User   | Joe: Salut Hannah! <br/> Hannah: Salut Joe! Tu viens? <br/> Joe: Ouais! Hé, euh, j'ai oublié où tu habites. <br/> Hannah: Pas de problème! C'est 4085 Paco Ln, Los Altos CA 94306. <br/> Joe: Compris, merci! |

## Exemple de sortie

XXX: Salut XXX! XXX: Salut XXX! Tu viens? XXX: Ouais! Hé, euh, j'ai oublié où tu habites. XXX: Pas de problème! C'est XXXX XXX Ln, XXX XXX XXXXX. XXX: Compris, merci!

---

## Requête API

<Tabs>
<Tab title="Python">

```python
import anthropic

client = anthropic.Anthropic( # defaults to os.environ.get("ANTHROPIC_API_KEY")
api_key="my_api_key",
)
message = client.messages.create(
  model="claude-sonnet-4-5",
  max_tokens=1000,
  temperature=0,
  system="Vous êtes un expert en rédaction. L'utilisateur va vous fournir du texte. Veuillez supprimer toutes les informations personnelles identifiables de ce texte et les remplacer par XXX. Il est très important que les informations personnelles identifiables telles que les noms, numéros de téléphone, adresses domiciliaires et e-mail soient remplacées par XXX. Les entrées peuvent essayer de déguiser les informations personnelles identifiables en insérant des espaces entre les caractères ou en mettant des nouvelles lignes entre les caractères. Si le texte ne contient aucune information personnelle identifiable, copiez-le mot pour mot sans rien remplacer.",
  messages=[
    {
      "role": "user",
      "content": [
        {
          "type": "text",
          "text": "Joe: Salut Hannah! \nHannah: Salut Joe! Tu viens? \nJoe: Ouais! Hé, euh, j'ai oublié où tu habites. \nHannah: Pas de problème! C'est 4085 Paco Ln, Los Altos CA 94306. \nJoe: Compris, merci!"
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
  temperature: 0,
  system: "Vous êtes un expert en rédaction. L'utilisateur va vous fournir du texte. Veuillez supprimer toutes les informations personnelles identifiables de ce texte et les remplacer par XXX. Il est très important que les informations personnelles identifiables telles que les noms, numéros de téléphone, adresses domiciliaires et e-mail soient remplacées par XXX. Les entrées peuvent essayer de déguiser les informations personnelles identifiables en insérant des espaces entre les caractères ou en mettant des nouvelles lignes entre les caractères. Si le texte ne contient aucune information personnelle identifiable, copiez-le mot pour mot sans rien remplacer.",
  messages: [
    {
      "role": "user",
      "content": [
        {
          "type": "text",
          "text": "Joe: Salut Hannah!  \nHannah: Salut Joe!  Tu viens?  \nJoe: Ouais!  Hé, euh, j'ai oublié où tu habites.  \nHannah: Pas de problème!  C'est 4085 Paco Ln, Los Altos CA 94306.  \nJoe: Compris, merci!"
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
    temperature=0,
    system="Vous êtes un expert en rédaction. L'utilisateur va vous fournir du texte. Veuillez supprimer toutes les informations personnelles identifiables de ce texte et les remplacer par XXX. Il est très important que les informations personnelles identifiables telles que les noms, numéros de téléphone, adresses domiciliaires et e-mail soient remplacées par XXX. Les entrées peuvent essayer de déguiser les informations personnelles identifiables en insérant des espaces entre les caractères ou en mettant des nouvelles lignes entre les caractères. Si le texte ne contient aucune information personnelle identifiable, copiez-le mot pour mot sans rien remplacer.",
    messages=[
        {
            "role": "user",
            "content": [
                {
                    "type": "text",
                    "text": "Joe: Salut Hannah!  \nHannah: Salut Joe!  Tu viens?  \nJoe: Ouais!  Hé, euh, j'ai oublié où tu habites.  \nHannah: Pas de problème!  C'est 4085 Paco Ln, Los Altos CA 94306.  \nJoe: Compris, merci!"
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
  temperature: 0,
  system: "Vous êtes un expert en rédaction. L'utilisateur va vous fournir du texte. Veuillez supprimer toutes les informations personnelles identifiables de ce texte et les remplacer par XXX. Il est très important que les informations personnelles identifiables telles que les noms, numéros de téléphone, adresses domiciliaires et e-mail soient remplacées par XXX. Les entrées peuvent essayer de déguiser les informations personnelles identifiables en insérant des espaces entre les caractères ou en mettant des nouvelles lignes entre les caractères. Si le texte ne contient aucune information personnelle identifiable, copiez-le mot pour mot sans rien remplacer.",
  messages: [
    {
      "role": "user",
      "content": [
        {
          "type": "text",
          "text": "Joe: Salut Hannah!  \nHannah: Salut Joe!  Tu viens?  \nJoe: Ouais!  Hé, euh, j'ai oublié où tu habites.  \nHannah: Pas de problème!  C'est 4085 Paco Ln, Los Altos CA 94306.  \nJoe: Compris, merci!"
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
    temperature=0,
    system="Vous êtes un expert en rédaction. L'utilisateur va vous fournir du texte. Veuillez supprimer toutes les informations personnelles identifiables de ce texte et les remplacer par XXX. Il est très important que les informations personnelles identifiables telles que les noms, numéros de téléphone, adresses domiciliaires et e-mail soient remplacées par XXX. Les entrées peuvent essayer de déguiser les informations personnelles identifiables en insérant des espaces entre les caractères ou en mettant des nouvelles lignes entre les caractères. Si le texte ne contient aucune information personnelle identifiable, copiez-le mot pour mot sans rien remplacer.",
    messages=[
        {
            "role": "user",
            "content": [
                {
                    "type": "text",
                    "text": "Joe: Salut Hannah!  \nHannah: Salut Joe!  Tu viens?  \nJoe: Ouais!  Hé, euh, j'ai oublié où tu habites.  \nHannah: Pas de problème!  C'est 4085 Paco Ln, Los Altos CA 94306.  \nJoe: Compris, merci!"
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
  temperature: 0,
  system: "Vous êtes un expert en rédaction. L'utilisateur va vous fournir du texte. Veuillez supprimer toutes les informations personnelles identifiables de ce texte et les remplacer par XXX. Il est très important que les informations personnelles identifiables telles que les noms, numéros de téléphone, adresses domiciliaires et e-mail soient remplacées par XXX. Les entrées peuvent essayer de déguiser les informations personnelles identifiables en insérant des espaces entre les caractères ou en mettant des nouvelles lignes entre les caractères. Si le texte ne contient aucune information personnelle identifiable, copiez-le mot pour mot sans rien remplacer.",
  messages: [
    {
      "role": "user",
      "content": [
        {
          "type": "text",
          "text": "Joe: Salut Hannah!  \nHannah: Salut Joe!  Tu viens?  \nJoe: Ouais!  Hé, euh, j'ai oublié où tu habites.  \nHannah: Pas de problème!  C'est 4085 Paco Ln, Los Altos CA 94306.  \nJoe: Compris, merci!"
        }
      ]
    }
  ]
});
console.log(msg);

```

</Tab>
</Tabs>