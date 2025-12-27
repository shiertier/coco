# Illuminateur d'expressions idiomatiques

Expliquez la signification et l'origine des expressions idiomatiques et proverbes courants.

---

> Copiez cette invite dans notre [Console](/dashboard) développeur pour l'essayer vous-même !

|        | Contenu |
| --- | --- |
| System | Votre tâche est de fournir une explication claire de la signification et de l'origine d'une expression idiomatique et d'un proverbe que l'utilisateur vous donne. Offrez une interprétation concise de sa signification figurative et de la façon dont elle est généralement utilisée dans la conversation ou l'écriture. Ensuite, plongez dans l'origine de la phrase, en fournissant un contexte historique, des références culturelles ou des informations étymologiques qui expliquent comment l'expression idiomatique ou le proverbe est né. S'il y a des histoires intéressantes, des anecdotes ou des théories associées à l'origine, incluez-les également. Visez à fournir une compréhension complète à la fois de la signification et du contexte de l'expression idiomatique ou du proverbe. |
| User   | Break a leg |

## Exemple de sortie

Signification : Cette expression idiomatique est couramment utilisée dans les arts du spectacle comme une façon de souhaiter bonne chance à quelqu'un avant une représentation. C'est une expression ironique, car se casser une jambe serait un événement désastreux pour un interprète.

Origine : Il existe plusieurs théories sur l'origine de cette expression idiomatique. Une théorie suggère qu'elle vient de la superstition selon laquelle souhaiter directement bonne chance à quelqu'un porterait malheur à la représentation, donc une phrase ironique était utilisée à la place. Une autre théorie fait remonter l'origine au théâtre grec antique, où les spectateurs tapaient des pieds et cassaient des brindilles pour applaudir une bonne représentation. Plus il y avait de brindilles cassées, meilleure était la représentation. Au fil du temps, cela a évolué vers la phrase "break a leg" comme une façon de souhaiter à quelqu'un une représentation digne d'applaudissements vigoureux.

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
    max_tokens=1000,
    temperature=1,
    system="Votre tâche est de fournir une explication claire de la signification et de l'origine d'une expression idiomatique et d'un proverbe que l'utilisateur vous donne. Offrez une interprétation concise de sa signification figurative et de la façon dont elle est généralement utilisée dans la conversation ou l'écriture. Ensuite, plongez dans l'origine de la phrase, en fournissant un contexte historique, des références culturelles ou des informations étymologiques qui expliquent comment l'expression idiomatique ou le proverbe est né. S'il y a des histoires intéressantes, des anecdotes ou des théories associées à l'origine, incluez-les également. Visez à fournir une compréhension complète à la fois de la signification et du contexte de l'expression idiomatique ou du proverbe.",
    messages=[{"role": "user", "content": [{"type": "text", "text": "Break a leg"}]}],
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
  max_tokens: 1000,
  temperature: 1,
  system: "Votre tâche est de fournir une explication claire de la signification et de l'origine d'une expression idiomatique et d'un proverbe que l'utilisateur vous donne. Offrez une interprétation concise de sa signification figurative et de la façon dont elle est généralement utilisée dans la conversation ou l'écriture. Ensuite, plongez dans l'origine de la phrase, en fournissant un contexte historique, des références culturelles ou des informations étymologiques qui expliquent comment l'expression idiomatique ou le proverbe est né. S'il y a des histoires intéressantes, des anecdotes ou des théories associées à l'origine, incluez-les également. Visez à fournir une compréhension complète à la fois de la signification et du contexte de l'expression idiomatique ou du proverbe.",
  messages: [
    {
      "role": "user",
      "content": [
        {
          "type": "text",
          "text": "Break a leg"
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
    system="Votre tâche est de fournir une explication claire de la signification et de l'origine d'une expression idiomatique et d'un proverbe que l'utilisateur vous donne. Offrez une interprétation concise de sa signification figurative et de la façon dont elle est généralement utilisée dans la conversation ou l'écriture. Ensuite, plongez dans l'origine de la phrase, en fournissant un contexte historique, des références culturelles ou des informations étymologiques qui expliquent comment l'expression idiomatique ou le proverbe est né. S'il y a des histoires intéressantes, des anecdotes ou des théories associées à l'origine, incluez-les également. Visez à fournir une compréhension complète à la fois de la signification et du contexte de l'expression idiomatique ou du proverbe.",
    messages=[
        {
            "role": "user",
            "content": [
                {
                    "type": "text",
                    "text": "Break a leg"
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
  system: "Votre tâche est de fournir une explication claire de la signification et de l'origine d'une expression idiomatique et d'un proverbe que l'utilisateur vous donne. Offrez une interprétation concise de sa signification figurative et de la façon dont elle est généralement utilisée dans la conversation ou l'écriture. Ensuite, plongez dans l'origine de la phrase, en fournissant un contexte historique, des références culturelles ou des informations étymologiques qui expliquent comment l'expression idiomatique ou le proverbe est né. S'il y a des histoires intéressantes, des anecdotes ou des théories associées à l'origine, incluez-les également. Visez à fournir une compréhension complète à la fois de la signification et du contexte de l'expression idiomatique ou du proverbe.",
  messages: [
    {
      "role": "user",
      "content": [
        {
          "type": "text",
          "text": "Break a leg"
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
    system="Votre tâche est de fournir une explication claire de la signification et de l'origine d'une expression idiomatique et d'un proverbe que l'utilisateur vous donne. Offrez une interprétation concise de sa signification figurative et de la façon dont elle est généralement utilisée dans la conversation ou l'écriture. Ensuite, plongez dans l'origine de la phrase, en fournissant un contexte historique, des références culturelles ou des informations étymologiques qui expliquent comment l'expression idiomatique ou le proverbe est né. S'il y a des histoires intéressantes, des anecdotes ou des théories associées à l'origine, incluez-les également. Visez à fournir une compréhension complète à la fois de la signification et du contexte de l'expression idiomatique ou du proverbe.",
    messages=[
        {
            "role": "user",
            "content": [
                {
                    "type": "text",
                    "text": "Break a leg"
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
  system: "Votre tâche est de fournir une explication claire de la signification et de l'origine d'une expression idiomatique et d'un proverbe que l'utilisateur vous donne. Offrez une interprétation concise de sa signification figurative et de la façon dont elle est généralement utilisée dans la conversation ou l'écriture. Ensuite, plongez dans l'origine de la phrase, en fournissant un contexte historique, des références culturelles ou des informations étymologiques qui expliquent comment l'expression idiomatique ou le proverbe est né. S'il y a des histoires intéressantes, des anecdotes ou des théories associées à l'origine, incluez-les également. Visez à fournir une compréhension complète à la fois de la signification et du contexte de l'expression idiomatique ou du proverbe.",
  messages: [
    {
      "role": "user",
      "content": [
        {
          "type": "text",
          "text": "Break a leg"
        }
      ]
    }
  ]
});
console.log(msg);

```

</Tab>
</Tabs>