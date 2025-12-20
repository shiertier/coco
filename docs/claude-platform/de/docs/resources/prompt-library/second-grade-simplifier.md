# Zweitklässler-Vereinfacher

Machen Sie komplexe Texte für junge Lernende leicht verständlich.

---

> Kopieren Sie diesen Prompt in unsere Entwickler-[Konsole](/dashboard), um es selbst auszuprobieren!

|        | Inhalt |
| --- | --- |
| System | Ihre Aufgabe ist es, den bereitgestellten Text zu nehmen und ihn so umzuschreiben, dass er für junge Lernende in den Klassen 3-5 leicht zu lesen und zu verstehen ist. Vereinfachen Sie fortgeschrittenes Vokabular, zerlegen Sie lange Sätze, erklären Sie schwierige Konzepte in einfacher Sprache und präsentieren Sie die Informationen auf eine klare, ansprechende Weise. Der kurze umgeschriebene Text sollte die Kernideen des ursprünglichen Textes auf altersgerechte Weise vermitteln. |
| User   | Die Mitochondrien, oft als die Kraftwerke der Zelle bezeichnet, sind wesentliche Organellen, die eine entscheidende Rolle bei der Produktion von Adenosintriphosphat (ATP) durch den Prozess der Zellatmung spielen. ATP ist die primäre Energiewährung der Zelle und ermöglicht es, dass verschiedene zelluläre Funktionen und biochemische Reaktionen auftreten. |

## Beispielausgabe

Mitochondrien sind winzige Teile in Zellen, die wie kleine Kraftwerke funktionieren. Sie stellen ein spezielles Molekül namens ATP her, das wie das Energiegeld der Zelle ist. ATP hilft der Zelle, wichtige Aufgaben zu erledigen und hält sie reibungslos am Laufen, genau wie Geld Menschen dabei hilft, Dinge zu kaufen, die sie brauchen. Mitochondrien sind sehr wichtig, denn ohne sie hätten Zellen nicht die Energie, um richtig zu funktionieren.

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
  system="Ihre Aufgabe ist es, den bereitgestellten Text zu nehmen und ihn so umzuschreiben, dass er für junge Lernende in den Klassen 3-5 leicht zu lesen und zu verstehen ist. Vereinfachen Sie fortgeschrittenes Vokabular, zerlegen Sie lange Sätze, erklären Sie schwierige Konzepte in einfacher Sprache und präsentieren Sie die Informationen auf eine klare, ansprechende Weise. Der kurze umgeschriebene Text sollte die Kernideen des ursprünglichen Textes auf altersgerechte Weise vermitteln.",
  messages=[
    {
      "role": "user",
      "content": [
        {
          "type": "text",
          "text": "Die Mitochondrien, oft als die Kraftwerke der Zelle bezeichnet, sind wesentliche Organellen, die eine entscheidende Rolle bei der Produktion von Adenosintriphosphat (ATP) durch den Prozess der Zellatmung spielen. ATP ist die primäre Energiewährung der Zelle und ermöglicht es, dass verschiedene zelluläre Funktionen und biochemische Reaktionen auftreten."
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
  system: "Ihre Aufgabe ist es, den bereitgestellten Text zu nehmen und ihn so umzuschreiben, dass er für junge Lernende in den Klassen 3-5 leicht zu lesen und zu verstehen ist. Vereinfachen Sie fortgeschrittenes Vokabular, zerlegen Sie lange Sätze, erklären Sie schwierige Konzepte in einfacher Sprache und präsentieren Sie die Informationen auf eine klare, ansprechende Weise. Der kurze umgeschriebene Text sollte die Kernideen des ursprünglichen Textes auf altersgerechte Weise vermitteln.",
  messages: [
    {
      "role": "user",
      "content": [
        {
          "type": "text",
          "text": "Die Mitochondrien, oft als die Kraftwerke der Zelle bezeichnet, sind wesentliche Organellen, die eine entscheidende Rolle bei der Produktion von Adenosintriphosphat (ATP) durch den Prozess der Zellatmung spielen. ATP ist die primäre Energiewährung der Zelle und ermöglicht es, dass verschiedene zelluläre Funktionen und biochemische Reaktionen auftreten."
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
    system="Ihre Aufgabe ist es, den bereitgestellten Text zu nehmen und ihn so umzuschreiben, dass er für junge Lernende in den Klassen 3-5 leicht zu lesen und zu verstehen ist. Vereinfachen Sie fortgeschrittenes Vokabular, zerlegen Sie lange Sätze, erklären Sie schwierige Konzepte in einfacher Sprache und präsentieren Sie die Informationen auf eine klare, ansprechende Weise. Der kurze umgeschriebene Text sollte die Kernideen des ursprünglichen Textes auf altersgerechte Weise vermitteln.",
    messages=[
        {
            "role": "user",
            "content": [
                {
                    "type": "text",
                    "text": "Die Mitochondrien, oft als die Kraftwerke der Zelle bezeichnet, sind wesentliche Organellen, die eine entscheidende Rolle bei der Produktion von Adenosintriphosphat (ATP) durch den Prozess der Zellatmung spielen. ATP ist die primäre Energiewährung der Zelle und ermöglicht es, dass verschiedene zelluläre Funktionen und biochemische Reaktionen auftreten."
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
  system: "Ihre Aufgabe ist es, den bereitgestellten Text zu nehmen und ihn so umzuschreiben, dass er für junge Lernende in den Klassen 3-5 leicht zu lesen und zu verstehen ist. Vereinfachen Sie fortgeschrittenes Vokabular, zerlegen Sie lange Sätze, erklären Sie schwierige Konzepte in einfacher Sprache und präsentieren Sie die Informationen auf eine klare, ansprechende Weise. Der kurze umgeschriebene Text sollte die Kernideen des ursprünglichen Textes auf altersgerechte Weise vermitteln.",
  messages: [
    {
      "role": "user",
      "content": [
        {
          "type": "text",
          "text": "Die Mitochondrien, oft als die Kraftwerke der Zelle bezeichnet, sind wesentliche Organellen, die eine entscheidende Rolle bei der Produktion von Adenosintriphosphat (ATP) durch den Prozess der Zellatmung spielen. ATP ist die primäre Energiewährung der Zelle und ermöglicht es, dass verschiedene zelluläre Funktionen und biochemische Reaktionen auftreten."
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
system="Ihre Aufgabe ist es, den bereitgestellten Text zu nehmen und ihn so umzuschreiben, dass er für junge Lernende in den Klassen 3-5 leicht zu lesen und zu verstehen ist. Vereinfachen Sie fortgeschrittenes Vokabular, zerlegen Sie lange Sätze, erklären Sie schwierige Konzepte in einfacher Sprache und präsentieren Sie die Informationen auf eine klare, ansprechende Weise. Der kurze umgeschriebene Text sollte die Kernideen des ursprünglichen Textes auf altersgerechte Weise vermitteln.",
messages=[
{
"role": "user",
"content": [
{
"type": "text",
"text": "Die Mitochondrien, oft als die Kraftwerke der Zelle bezeichnet, sind wesentliche Organellen, die eine entscheidende Rolle bei der Produktion von Adenosintriphosphat (ATP) durch den Prozess der Zellatmung spielen. ATP ist die primäre Energiewährung der Zelle und ermöglicht es, dass verschiedene zelluläre Funktionen und biochemische Reaktionen auftreten."
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
  temperature: 0,
  system: "Ihre Aufgabe ist es, den bereitgestellten Text zu nehmen und ihn so umzuschreiben, dass er für junge Lernende in den Klassen 3-5 leicht zu lesen und zu verstehen ist. Vereinfachen Sie fortgeschrittenes Vokabular, zerlegen Sie lange Sätze, erklären Sie schwierige Konzepte in einfacher Sprache und präsentieren Sie die Informationen auf eine klare, ansprechende Weise. Der kurze umgeschriebene Text sollte die Kernideen des ursprünglichen Textes auf altersgerechte Weise vermitteln.",
  messages: [
    {
      "role": "user",
      "content": [
        {
          "type": "text",
          "text": "Die Mitochondrien, oft als die Kraftwerke der Zelle bezeichnet, sind wesentliche Organellen, die eine entscheidende Rolle bei der Produktion von Adenosintriphosphat (ATP) durch den Prozess der Zellatmung spielen. ATP ist die primäre Energiewährung der Zelle und ermöglicht es, dass verschiedene zelluläre Funktionen und biochemische Reaktionen auftreten."
        }
      ]
    }
  ]
});
console.log(msg);

````

</Tab>
</Tabs>