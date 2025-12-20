# Rätsel mir das

Generiere Rätsel und führe den Benutzer zu den Lösungen.

---

> Kopiere diesen Prompt in unsere Entwickler-[Konsole](/dashboard), um es selbst auszuprobieren!

| Rolle | Inhalt |
| ---- | --- |
| Benutzer | Generiere ein cleveres Rätsel und biete eine schrittweise Anleitung, um dem Benutzer zu helfen, die richtigen Lösungen zu finden. Das Rätsel sollte herausfordernd, aber mit logischem Denken und Aufmerksamkeit für Details lösbar sein. Nach der Präsentation jedes Rätsels biete eine Reihe von Hinweisen oder Fragen an, die den Benutzer progressiv zur Antwort führen. Stelle sicher, dass die Hinweise nicht zu offensichtlich sind, aber dennoch genügend Informationen bieten, um den Denkprozess des Benutzers zu leiten. Enthülle schließlich die Lösung und biete eine kurze Erklärung, wie das Rätsel mit den gegebenen Hinweisen gelöst werden kann. |

## Beispielausgabe

Ich bin nicht lebendig, aber ich wachse; ich habe keine Lungen, aber ich brauche Luft; ich habe keinen Mund, aber Wasser tötet mich. Was bin ich?

Hinweis 1: Betrachte die verschiedenen Aggregatzustände und wie sie sich verändern können.  
Hinweis 2: Denke über die Beziehung zwischen den drei erwähnten Elementen nach: Luft, Wasser und dem mysteriösen Objekt.  
Hinweis 3: Die Antwort bezieht sich auf ein häufiges Phänomen, das im Alltag auftritt.

Lösung: Feuer.  
Erklärung: Feuer ist nicht lebendig, aber es wächst und breitet sich aus, während es Brennstoff verbraucht. Es hat keine Lungen, aber es benötigt Sauerstoff (Luft), um sich zu erhalten. Obwohl es keinen Mund hat, wird Wasser oft verwendet, um Feuer zu löschen und es effektiv zu "töten".

---

## API-Anfrage

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
  temperature=0,
  messages=[
    {
      "role": "user",
      "content": [
        {
          "type": "text",
          "text": "Generiere ein cleveres Rätsel und biete eine schrittweise Anleitung, um dem Benutzer zu helfen, die richtigen Lösungen zu finden. Das Rätsel sollte herausfordernd, aber mit logischem Denken und Aufmerksamkeit für Details lösbar sein. Nach der Präsentation jedes Rätsels biete eine Reihe von Hinweisen oder Fragen an, die den Benutzer progressiv zur Antwort führen. Stelle sicher, dass die Hinweise nicht zu offensichtlich sind, aber dennoch genügend Informationen bieten, um den Denkprozess des Benutzers zu leiten. Enthülle schließlich die Lösung und biete eine kurze Erklärung, wie das Rätsel mit den gegebenen Hinweisen gelöst werden kann."
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
  messages: [
    {
      "role": "user",
      "content": [
        {
          "type": "text",
          "text": "Generiere ein cleveres Rätsel und biete eine schrittweise Anleitung, um dem Benutzer zu helfen, die richtigen Lösungen zu finden. Das Rätsel sollte herausfordernd, aber mit logischem Denken und Aufmerksamkeit für Details lösbar sein. Nach der Präsentation jedes Rätsels biete eine Reihe von Hinweisen oder Fragen an, die den Benutzer progressiv zur Antwort führen. Stelle sicher, dass die Hinweise nicht zu offensichtlich sind, aber dennoch genügend Informationen bieten, um den Denkprozess des Benutzers zu leiten. Enthülle schließlich die Lösung und biete eine kurze Erklärung, wie das Rätsel mit den gegebenen Hinweisen gelöst werden kann."
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
    messages=[
        {
            "role": "user",
            "content": [
                {
                    "type": "text",
                    "text": "Generiere ein cleveres Rätsel und biete eine schrittweise Anleitung, um dem Benutzer zu helfen, die richtigen Lösungen zu finden. Das Rätsel sollte herausfordernd, aber mit logischem Denken und Aufmerksamkeit für Details lösbar sein. Nach der Präsentation jedes Rätsels biete eine Reihe von Hinweisen oder Fragen an, die den Benutzer progressiv zur Antwort führen. Stelle sicher, dass die Hinweise nicht zu offensichtlich sind, aber dennoch genügend Informationen bieten, um den Denkprozess des Benutzers zu leiten. Enthülle schließlich die Lösung und biete eine kurze Erklärung, wie das Rätsel mit den gegebenen Hinweisen gelöst werden kann."
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
  messages: [
    {
      "role": "user",
      "content": [
        {
          "type": "text",
          "text": "Generiere ein cleveres Rätsel und biete eine schrittweise Anleitung, um dem Benutzer zu helfen, die richtigen Lösungen zu finden. Das Rätsel sollte herausfordernd, aber mit logischem Denken und Aufmerksamkeit für Details lösbar sein. Nach der Präsentation jedes Rätsels biete eine Reihe von Hinweisen oder Fragen an, die den Benutzer progressiv zur Antwort führen. Stelle sicher, dass die Hinweise nicht zu offensichtlich sind, aber dennoch genügend Informationen bieten, um den Denkprozess des Benutzers zu leiten. Enthülle schließlich die Lösung und biete eine kurze Erklärung, wie das Rätsel mit den gegebenen Hinweisen gelöst werden kann."
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
    messages=[
        {
            "role": "user",
            "content": [
                {
                    "type": "text",
                    "text": "Generiere ein cleveres Rätsel und biete eine schrittweise Anleitung, um dem Benutzer zu helfen, die richtigen Lösungen zu finden. Das Rätsel sollte herausfordernd, aber mit logischem Denken und Aufmerksamkeit für Details lösbar sein. Nach der Präsentation jedes Rätsels biete eine Reihe von Hinweisen oder Fragen an, die den Benutzer progressiv zur Antwort führen. Stelle sicher, dass die Hinweise nicht zu offensichtlich sind, aber dennoch genügend Informationen bieten, um den Denkprozess des Benutzers zu leiten. Enthülle schließlich die Lösung und biete eine kurze Erklärung, wie das Rätsel mit den gegebenen Hinweisen gelöst werden kann."
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
  messages: [
    {
      "role": "user",
      "content": [
        {
          "type": "text",
          "text": "Generiere ein cleveres Rätsel und biete eine schrittweise Anleitung, um dem Benutzer zu helfen, die richtigen Lösungen zu finden. Das Rätsel sollte herausfordernd, aber mit logischem Denken und Aufmerksamkeit für Details lösbar sein. Nach der Präsentation jedes Rätsels biete eine Reihe von Hinweisen oder Fragen an, die den Benutzer progressiv zur Antwort führen. Stelle sicher, dass die Hinweise nicht zu offensichtlich sind, aber dennoch genügend Informationen bieten, um den Denkprozess des Benutzers zu leiten. Enthülle schließlich die Lösung und biete eine kurze Erklärung, wie das Rätsel mit den gegebenen Hinweisen gelöst werden kann."
        }
      ]
    }
  ]
});
console.log(msg);

```

</Tab>
</Tabs>