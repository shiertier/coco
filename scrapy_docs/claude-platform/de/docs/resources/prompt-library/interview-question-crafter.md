# Interview-Fragen-Ersteller

Generiere Fragen für Interviews.

---

> Kopiere diesen Prompt in unsere Entwickler-[Konsole](/dashboard), um es selbst auszuprobieren!

|        | Inhalt |
| --- | --- |
| System | Deine Aufgabe ist es, eine Reihe durchdachter, offener Fragen für ein Interview basierend auf dem gegebenen Kontext zu generieren. Die Fragen sollten darauf ausgelegt sein, aufschlussreiche und detaillierte Antworten vom Befragten zu erhalten, die es ihm ermöglichen, sein Wissen, seine Erfahrung und seine kritischen Denkfähigkeiten zu zeigen. Vermeide Ja/Nein-Fragen oder solche mit offensichtlichen Antworten. Konzentriere dich stattdessen auf Fragen, die zur Reflexion, Selbsteinschätzung und zum Teilen spezifischer Beispiele oder Anekdoten ermutigen. |
| User   | Du führst ein Interview mit einem Kandidaten für eine Marketing-Manager-Position in einem etablierten E-Commerce-Unternehmen. Der ideale Kandidat sollte Erfahrung in der Entwicklung und Umsetzung von Multi-Channel-Marketing-Kampagnen haben, starke analytische Fähigkeiten besitzen und die Fähigkeit haben, effektiv mit funktionsübergreifenden Teams zusammenzuarbeiten. Er sollte auch leidenschaftlich daran interessiert sein, über die neuesten Marketing-Trends und -Technologien auf dem Laufenden zu bleiben. |

## Beispiel-Ausgabe

1.  Können Sie mich durch eine erfolgreiche Multi-Channel-Marketing-Kampagne führen, die Sie in Ihrer vorherigen Rolle entwickelt und umgesetzt haben? Welche Herausforderungen haben Sie dabei erlebt, und wie haben Sie diese überwunden?
2.  Wie messen Sie den Erfolg Ihrer Marketing-Kampagnen, und auf welche Metriken konzentrieren Sie sich? Können Sie ein Beispiel dafür geben, wie Sie Daten und Analysen verwendet haben, um die Leistung einer Kampagne zu optimieren?
3.  Was sind Ihrer Erfahrung nach die Schlüsselelemente einer erfolgreichen E-Commerce-Marketing-Strategie? Wie stellen Sie sicher, dass Ihre Strategien mit den übergeordneten Geschäftszielen des Unternehmens übereinstimmen?
4.  Können Sie ein Beispiel für eine Zeit teilen, in der Sie mit einem funktionsübergreifenden Team (z.B. Produkt, Vertrieb oder Kundenservice) zusammenarbeiten mussten, um ein Marketing-Ziel zu erreichen? Welche Herausforderungen sind Sie dabei begegnet, und wie haben Sie eine effektive Zusammenarbeit gefördert?
5.  Wie bleiben Sie über die neuesten Marketing-Trends und -Technologien informiert, und wie haben Sie dieses Wissen angewendet, um Ihre Marketing-Strategien oder -Kampagnen zu verbessern?
6.  Können Sie eine Zeit beschreiben, in der eine Marketing-Kampagne, an der Sie beteiligt waren, nicht wie erwartet funktioniert hat? Was haben Sie aus dieser Erfahrung gelernt, und wie haben Sie diese Lektionen auf zukünftige Kampagnen angewendet?
7.  Unser Unternehmen schätzt Innovation und kontinuierliche Verbesserung. Können Sie ein Beispiel für eine Zeit teilen, in der Sie einen neuen Marketing-Ansatz, ein Tool oder eine Technologie eingeführt haben, die die Leistung oder Ergebnisse Ihres Teams erheblich beeinflusst hat?

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
  temperature=0.5,
  system="Deine Aufgabe ist es, eine Reihe durchdachter, offener Fragen für ein Interview basierend auf dem gegebenen Kontext zu generieren. Die Fragen sollten darauf ausgelegt sein, aufschlussreiche und detaillierte Antworten vom Befragten zu erhalten, die es ihm ermöglichen, sein Wissen, seine Erfahrung und seine kritischen Denkfähigkeiten zu zeigen. Vermeide Ja/Nein-Fragen oder solche mit offensichtlichen Antworten. Konzentriere dich stattdessen auf Fragen, die zur Reflexion, Selbsteinschätzung und zum Teilen spezifischer Beispiele oder Anekdoten ermutigen.",
  messages=[
    {
      "role": "user",
      "content": [
        {
          "type": "text",
          "text": "Du führst ein Interview mit einem Kandidaten für eine Marketing-Manager-Position in einem etablierten E-Commerce-Unternehmen. Der ideale Kandidat sollte Erfahrung in der Entwicklung und Umsetzung von Multi-Channel-Marketing-Kampagnen haben, starke analytische Fähigkeiten besitzen und die Fähigkeit haben, effektiv mit funktionsübergreifenden Teams zusammenzuarbeiten. Er sollte auch leidenschaftlich daran interessiert sein, über die neuesten Marketing-Trends und -Technologien auf dem Laufenden zu bleiben."
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
  system: "Deine Aufgabe ist es, eine Reihe durchdachter, offener Fragen für ein Interview basierend auf dem gegebenen Kontext zu generieren. Die Fragen sollten darauf ausgelegt sein, aufschlussreiche und detaillierte Antworten vom Befragten zu erhalten, die es ihm ermöglichen, sein Wissen, seine Erfahrung und seine kritischen Denkfähigkeiten zu zeigen. Vermeide Ja/Nein-Fragen oder solche mit offensichtlichen Antworten. Konzentriere dich stattdessen auf Fragen, die zur Reflexion, Selbsteinschätzung und zum Teilen spezifischer Beispiele oder Anekdoten ermutigen.",
  messages: [
    {
      "role": "user",
      "content": [
        {
          "type": "text",
          "text": "Du führst ein Interview mit einem Kandidaten für eine Marketing-Manager-Position in einem etablierten E-Commerce-Unternehmen. Der ideale Kandidat sollte Erfahrung in der Entwicklung und Umsetzung von Multi-Channel-Marketing-Kampagnen haben, starke analytische Fähigkeiten besitzen und die Fähigkeit haben, effektiv mit funktionsübergreifenden Teams zusammenzuarbeiten. Er sollte auch leidenschaftlich daran interessiert sein, über die neuesten Marketing-Trends und -Technologien auf dem Laufenden zu bleiben."
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
    system="Deine Aufgabe ist es, eine Reihe durchdachter, offener Fragen für ein Interview basierend auf dem gegebenen Kontext zu generieren. Die Fragen sollten darauf ausgelegt sein, aufschlussreiche und detaillierte Antworten vom Befragten zu erhalten, die es ihm ermöglichen, sein Wissen, seine Erfahrung und seine kritischen Denkfähigkeiten zu zeigen. Vermeide Ja/Nein-Fragen oder solche mit offensichtlichen Antworten. Konzentriere dich stattdessen auf Fragen, die zur Reflexion, Selbsteinschätzung und zum Teilen spezifischer Beispiele oder Anekdoten ermutigen.",
    messages=[
        {
            "role": "user",
            "content": [
                {
                    "type": "text",
                    "text": "Du führst ein Interview mit einem Kandidaten für eine Marketing-Manager-Position in einem etablierten E-Commerce-Unternehmen. Der ideale Kandidat sollte Erfahrung in der Entwicklung und Umsetzung von Multi-Channel-Marketing-Kampagnen haben, starke analytische Fähigkeiten besitzen und die Fähigkeit haben, effektiv mit funktionsübergreifenden Teams zusammenzuarbeiten. Er sollte auch leidenschaftlich daran interessiert sein, über die neuesten Marketing-Trends und -Technologien auf dem Laufenden zu bleiben."
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
system: "Deine Aufgabe ist es, eine Reihe durchdachter, offener Fragen für ein Interview basierend auf dem gegebenen Kontext zu generieren. Die Fragen sollten darauf ausgelegt sein, aufschlussreiche und detaillierte Antworten vom Befragten zu erhalten, die es ihm ermöglichen, sein Wissen, seine Erfahrung und seine kritischen Denkfähigkeiten zu zeigen. Vermeide Ja/Nein-Fragen oder solche mit offensichtlichen Antworten. Konzentriere dich stattdessen auf Fragen, die zur Reflexion, Selbsteinschätzung und zum Teilen spezifischer Beispiele oder Anekdoten ermutigen.",
messages: [
{
"role": "user",
"content": [
{
"type": "text",
"text": "Du führst ein Interview mit einem Kandidaten für eine Marketing-Manager-Position in einem etablierten E-Commerce-Unternehmen. Der ideale Kandidat sollte Erfahrung in der Entwicklung und Umsetzung von Multi-Channel-Marketing-Kampagnen haben, starke analytische Fähigkeiten besitzen und die Fähigkeit haben, effektiv mit funktionsübergreifenden Teams zusammenzuarbeiten. Er sollte auch leidenschaftlich daran interessiert sein, über die neuesten Marketing-Trends und -Technologien auf dem Laufenden zu bleiben."
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
    system="Deine Aufgabe ist es, eine Reihe durchdachter, offener Fragen für ein Interview basierend auf dem gegebenen Kontext zu generieren. Die Fragen sollten darauf ausgelegt sein, aufschlussreiche und detaillierte Antworten vom Befragten zu erhalten, die es ihm ermöglichen, sein Wissen, seine Erfahrung und seine kritischen Denkfähigkeiten zu zeigen. Vermeide Ja/Nein-Fragen oder solche mit offensichtlichen Antworten. Konzentriere dich stattdessen auf Fragen, die zur Reflexion, Selbsteinschätzung und zum Teilen spezifischer Beispiele oder Anekdoten ermutigen.",
    messages=[
        {
            "role": "user",
            "content": [
                {
                    "type": "text",
                    "text": "Du führst ein Interview mit einem Kandidaten für eine Marketing-Manager-Position in einem etablierten E-Commerce-Unternehmen. Der ideale Kandidat sollte Erfahrung in der Entwicklung und Umsetzung von Multi-Channel-Marketing-Kampagnen haben, starke analytische Fähigkeiten besitzen und die Fähigkeit haben, effektiv mit funktionsübergreifenden Teams zusammenzuarbeiten. Er sollte auch leidenschaftlich daran interessiert sein, über die neuesten Marketing-Trends und -Technologien auf dem Laufenden zu bleiben."
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
  system: "Deine Aufgabe ist es, eine Reihe durchdachter, offener Fragen für ein Interview basierend auf dem gegebenen Kontext zu generieren. Die Fragen sollten darauf ausgelegt sein, aufschlussreiche und detaillierte Antworten vom Befragten zu erhalten, die es ihm ermöglichen, sein Wissen, seine Erfahrung und seine kritischen Denkfähigkeiten zu zeigen. Vermeide Ja/Nein-Fragen oder solche mit offensichtlichen Antworten. Konzentriere dich stattdessen auf Fragen, die zur Reflexion, Selbsteinschätzung und zum Teilen spezifischer Beispiele oder Anekdoten ermutigen.",
  messages: [
    {
      "role": "user",
      "content": [
        {
          "type": "text",
          "text": "Du führst ein Interview mit einem Kandidaten für eine Marketing-Manager-Position in einem etablierten E-Commerce-Unternehmen. Der ideale Kandidat sollte Erfahrung in der Entwicklung und Umsetzung von Multi-Channel-Marketing-Kampagnen haben, starke analytische Fähigkeiten besitzen und die Fähigkeit haben, effektiv mit funktionsübergreifenden Teams zusammenzuarbeiten. Er sollte auch leidenschaftlich daran interessiert sein, über die neuesten Marketing-Trends und -Technologien auf dem Laufenden zu bleiben."
        }
      ]
    }
  ]
});
console.log(msg);

```

</Tab>
</Tabs>