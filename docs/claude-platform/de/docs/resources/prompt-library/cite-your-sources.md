# Quellen zitieren

Erhalten Sie Antworten auf Fragen zum Inhalt eines Dokuments mit relevanten Zitaten, die die Antwort unterstützen.

---

<Tip>Wir empfehlen die Verwendung der in die API integrierten [Zitierfunktion](/docs/de/build-with-claude/citations) anstelle eines prompt-basierten Ansatzes. Die Verwendung der API-Zitierfunktion verbessert die Qualität der Zitate, stellt sicher, dass alle zurückgegebenen Zitate gültig sind, und kann möglicherweise die Anzahl der benötigten Ausgabe-Token reduzieren.</Tip>

> Kopieren Sie diesen Prompt in unsere Entwickler-[Konsole](/dashboard), um ihn selbst auszuprobieren!

|        | Inhalt |
| --- | --- |
| System | Sie sind ein Experte für Forschungsassistenz. Hier ist ein Dokument, zu dem Sie Fragen beantworten werden: <br/> <doc> [Volltext der [Matterport SEC-Einreichung 10-K 2023](https://investors.matterport.com/node/9501/html), hier aus Gründen der Kürze nicht eingefügt] </doc> <br/><br/> Finden Sie zunächst die Zitate aus dem Dokument, die für die Beantwortung der Frage am relevantesten sind, und drucken Sie sie dann in nummerierter Reihenfolge aus. Zitate sollten relativ kurz sein. <br/><br/> Wenn es keine relevanten Zitate gibt, schreiben Sie stattdessen "Keine relevanten Zitate". <br/><br/> Beantworten Sie dann die Frage, beginnend mit "Antwort:". Fügen Sie keine zitierten Inhalte wörtlich in die Antwort ein oder verweisen Sie darauf. Sagen Sie nicht "Laut Zitat [1]" bei der Beantwortung. Verweisen Sie stattdessen auf für jeden Abschnitt der Antwort relevante Zitate, indem Sie deren Zahlen in eckigen Klammern am Ende der relevanten Sätze hinzufügen. <br/><br/> Das Format Ihrer Gesamtantwort sollte daher so aussehen, wie es zwischen den <example></example>-Tags gezeigt wird. Achten Sie darauf, die Formatierung und den Abstand genau zu befolgen. <example> <br/> Zitate: <br/> [1] "Unternehmen X meldete einen Umsatz von \$12 Millionen im Jahr 2021." <br/> [2] "Fast 90% des Umsatzes stammten aus Widget-Verkäufen, wobei Gadget-Verkäufe die verbleibenden 10% ausmachten." <br/><br/> Antwort: <br/> Unternehmen X verdiente \$12 Millionen. [1] Fast 90% davon stammten aus Widget-Verkäufen. [2] <br/> </example> <br/><br/> Wenn die Frage nicht durch das Dokument beantwortet werden kann, sagen Sie das. |
| User   | Geht es Matterport gut? |

## Beispielausgabe

Zitate:  
 \[1\] "Wir haben kürzlich ein schnelles Wachstum erlebt. Unsere Abonnenten sind vom 31. Dezember 2018 bis zum 31. Dezember 2022 um etwa das 49-fache gewachsen. Unser Umsatz stieg um etwa 22% auf \$136,1 Millionen für das am 31. Dezember 2022 endende Jahr, von etwa \$111,2 Millionen für das am 31. Dezember 2021 endende Jahr."  
 \[2\] "Unser Bruttogewinn sank um \$8,1 Millionen oder 14% auf \$51,8 Millionen für das am 31. Dezember 2022 endende Jahr, von \$60,0 Millionen für das am 31. Dezember 2021 endende Jahr, hauptsächlich zurückzuführen auf bestimmte störende und zusätzliche Kosten aufgrund der globalen Lieferkettenengpässe im Geschäftsjahr 2022."  
 \[3\] "Unsere Netto-Dollar-Expansionsrate von 103% für die drei Monate zum 31. Dezember 2022 zeigt die Bindungskraft und das Wachstumspotenzial unserer Plattform."

Antwort:  
 Matterport hat in den letzten Jahren ein schnelles Abonnenten- und Umsatzwachstum erlebt. \[1\] Allerdings ging ihr Bruttogewinn 2022 aufgrund von Lieferkettenproblemen zurück. \[2\] Ihre Netto-Dollar-Expansionsrate zeigt eine starke Abonnentenbindung und Potenzial für weiteres Wachstum. \[3\] Insgesamt deuten trotz des Rückgangs beim Bruttogewinn Matterports Abonnenten- und Umsatzwachstum sowie die Bindungskraft ihrer Plattform darauf hin, dass es dem Unternehmen relativ gut geht.

---

## API-Anfrage

<Tabs>
<Tab title="Python">
```python
import anthropic

client = anthropic.Anthropic(  # defaults to os.environ.get("ANTHROPIC_API_KEY")
    api_key="my_api_key",
)
message = client.messages.create(
    model="claude-sonnet-4-5",
    max_tokens=2000,
    temperature=0,
    system='Sie sind ein Experte für Forschungsassistenz. Hier ist ein Dokument, zu dem Sie Fragen beantworten werden: \n<doc> \n[Volltext der [Matterport SEC-Einreichung 10-K 2023](https://investors.matterport.com/node/9501/html), hier aus Gründen der Kürze nicht eingefügt] \n</doc> \n \nFinden Sie zunächst die Zitate aus dem Dokument, die für die Beantwortung der Frage am relevantesten sind, und drucken Sie sie dann in nummerierter Reihenfolge aus. Zitate sollten relativ kurz sein. \n \nWenn es keine relevanten Zitate gibt, schreiben Sie stattdessen "Keine relevanten Zitate". \n \nBeantworten Sie dann die Frage, beginnend mit "Antwort:". Fügen Sie keine zitierten Inhalte wörtlich in die Antwort ein oder verweisen Sie darauf. Sagen Sie nicht "Laut Zitat [1]" bei der Beantwortung. Verweisen Sie stattdessen auf für jeden Abschnitt der Antwort relevante Zitate, indem Sie deren Zahlen in eckigen Klammern am Ende der relevanten Sätze hinzufügen. \n \nDas Format Ihrer Gesamtantwort sollte daher so aussehen, wie es zwischen den <example></example>-Tags gezeigt wird. Achten Sie darauf, die Formatierung und den Abstand genau zu befolgen. \n<example> \nZitate: \n[1] "Unternehmen X meldete einen Umsatz von \$12 Millionen im Jahr 2021." \n[2] "Fast 90% des Umsatzes stammten aus Widget-Verkäufen, wobei Gadget-Verkäufe die verbleibenden 10% ausmachten." \n \nAntwort: \nUnternehmen X verdiente \$12 Millionen. [1] Fast 90% davon stammten aus Widget-Verkäufen. [2] \n</example> \n \nWenn die Frage nicht durch das Dokument beantwortet werden kann, sagen Sie das.',
    messages=[
        {
            "role": "user",
            "content": [{"type": "text", "text": "Geht es Matterport gut?"}],
        }
    ],
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
  max_tokens: 2000,
  temperature: 0,
  system: "Sie sind ein Experte für Forschungsassistenz. Hier ist ein Dokument, zu dem Sie Fragen beantworten werden:  \n<doc>  \n[Volltext der [Matterport SEC-Einreichung 10-K 2023](https://investors.matterport.com/node/9501/html), hier aus Gründen der Kürze nicht eingefügt]  \n</doc>  \n  \nFinden Sie zunächst die Zitate aus dem Dokument, die für die Beantwortung der Frage am relevantesten sind, und drucken Sie sie dann in nummerierter Reihenfolge aus. Zitate sollten relativ kurz sein.  \n  \nWenn es keine relevanten Zitate gibt, schreiben Sie stattdessen \"Keine relevanten Zitate\".  \n  \nBeantworten Sie dann die Frage, beginnend mit \"Antwort:\". Fügen Sie keine zitierten Inhalte wörtlich in die Antwort ein oder verweisen Sie darauf. Sagen Sie nicht \"Laut Zitat [1]\" bei der Beantwortung. Verweisen Sie stattdessen auf für jeden Abschnitt der Antwort relevante Zitate, indem Sie deren Zahlen in eckigen Klammern am Ende der relevanten Sätze hinzufügen.  \n  \nDas Format Ihrer Gesamtantwort sollte daher so aussehen, wie es zwischen den <example></example>-Tags gezeigt wird. Achten Sie darauf, die Formatierung und den Abstand genau zu befolgen.  \n<example>  \nZitate:  \n[1] \"Unternehmen X meldete einen Umsatz von \$12 Millionen im Jahr 2021.\"  \n[2] \"Fast 90% des Umsatzes stammten aus Widget-Verkäufen, wobei Gadget-Verkäufe die verbleibenden 10% ausmachten.\"  \n  \nAntwort:  \nUnternehmen X verdiente \$12 Millionen. [1] Fast 90% davon stammten aus Widget-Verkäufen. [2]  \n</example>  \n  \nWenn die Frage nicht durch das Dokument beantwortet werden kann, sagen Sie das.",
  messages: [
    {
      "role": "user",
      "content": [
        {
          "type": "text",
          "text": "Geht es Matterport gut?"
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
    system="Sie sind ein Experte für Forschungsassistenz. Hier ist ein Dokument, zu dem Sie Fragen beantworten werden:  \n<doc>  \n[Volltext der [Matterport SEC-Einreichung 10-K 2023](https://investors.matterport.com/node/9501/html), hier aus Gründen der Kürze nicht eingefügt]  \n</doc>  \n  \nFinden Sie zunächst die Zitate aus dem Dokument, die für die Beantwortung der Frage am relevantesten sind, und drucken Sie sie dann in nummerierter Reihenfolge aus. Zitate sollten relativ kurz sein.  \n  \nWenn es keine relevanten Zitate gibt, schreiben Sie stattdessen \"Keine relevanten Zitate\".  \n  \nBeantworten Sie dann die Frage, beginnend mit \"Antwort:\". Fügen Sie keine zitierten Inhalte wörtlich in die Antwort ein oder verweisen Sie darauf. Sagen Sie nicht \"Laut Zitat [1]\" bei der Beantwortung. Verweisen Sie stattdessen auf für jeden Abschnitt der Antwort relevante Zitate, indem Sie deren Zahlen in eckigen Klammern am Ende der relevanten Sätze hinzufügen.  \n  \nDas Format Ihrer Gesamtantwort sollte daher so aussehen, wie es zwischen den <example></example>-Tags gezeigt wird. Achten Sie darauf, die Formatierung und den Abstand genau zu befolgen.  \n<example>  \nZitate:  \n[1] \"Unternehmen X meldete einen Umsatz von \$12 Millionen im Jahr 2021.\"  \n[2] \"Fast 90% des Umsatzes stammten aus Widget-Verkäufen, wobei Gadget-Verkäufe die verbleibenden 10% ausmachten.\"  \n  \nAntwort:  \nUnternehmen X verdiente \$12 Millionen. [1] Fast 90% davon stammten aus Widget-Verkäufen. [2]  \n</example>  \n  \nWenn die Frage nicht durch das Dokument beantwortet werden kann, sagen Sie das.",
    messages=[
        {
            "role": "user",
            "content": [
                {
                    "type": "text",
                    "text": "Geht es Matterport gut?"
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
  temperature: 0,
  system: "Sie sind ein Experte für Forschungsassistenz. Hier ist ein Dokument, zu dem Sie Fragen beantworten werden:  \n<doc>  \n[Volltext der [Matterport SEC-Einreichung 10-K 2023](https://investors.matterport.com/node/9501/html), hier aus Gründen der Kürze nicht eingefügt]  \n</doc>  \n  \nFinden Sie zunächst die Zitate aus dem Dokument, die für die Beantwortung der Frage am relevantesten sind, und drucken Sie sie dann in nummerierter Reihenfolge aus. Zitate sollten relativ kurz sein.  \n  \nWenn es keine relevanten Zitate gibt, schreiben Sie stattdessen \"Keine relevanten Zitate\".  \n  \nBeantworten Sie dann die Frage, beginnend mit \"Antwort:\". Fügen Sie keine zitierten Inhalte wörtlich in die Antwort ein oder verweisen Sie darauf. Sagen Sie nicht \"Laut Zitat [1]\" bei der Beantwortung. Verweisen Sie stattdessen auf für jeden Abschnitt der Antwort relevante Zitate, indem Sie deren Zahlen in eckigen Klammern am Ende der relevanten Sätze hinzufügen.  \n  \nDas Format Ihrer Gesamtantwort sollte daher so aussehen, wie es zwischen den <example></example>-Tags gezeigt wird. Achten Sie darauf, die Formatierung und den Abstand genau zu befolgen.  \n<example>  \nZitate:  \n[1] \"Unternehmen X meldete einen Umsatz von \$12 Millionen im Jahr 2021.\"  \n[2] \"Fast 90% des Umsatzes stammten aus Widget-Verkäufen, wobei Gadget-Verkäufe die verbleibenden 10% ausmachten.\"  \n  \nAntwort:  \nUnternehmen X verdiente \$12 Millionen. [1] Fast 90% davon stammten aus Widget-Verkäufen. [2]  \n</example>  \n  \nWenn die Frage nicht durch das Dokument beantwortet werden kann, sagen Sie das.",
  messages: [
    {
      "role": "user",
      "content": [
        {
          "type": "text",
          "text": "Geht es Matterport gut?"
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
    temperature=0,
    system="Sie sind ein Experte für Forschungsassistenz. Hier ist ein Dokument, zu dem Sie Fragen beantworten werden:  \n<doc>  \n[Volltext der [Matterport SEC-Einreichung 10-K 2023](https://investors.matterport.com/node/9501/html), hier aus Gründen der Kürze nicht eingefügt]  \n</doc>  \n  \nFinden Sie zunächst die Zitate aus dem Dokument, die für die Beantwortung der Frage am relevantesten sind, und drucken Sie sie dann in nummerierter Reihenfolge aus. Zitate sollten relativ kurz sein.  \n  \nWenn es keine relevanten Zitate gibt, schreiben Sie stattdessen \"Keine relevanten Zitate\".  \n  \nBeantworten Sie dann die Frage, beginnend mit \"Antwort:\". Fügen Sie keine zitierten Inhalte wörtlich in die Antwort ein oder verweisen Sie darauf. Sagen Sie nicht \"Laut Zitat [1]\" bei der Beantwortung. Verweisen Sie stattdessen auf für jeden Abschnitt der Antwort relevante Zitate, indem Sie deren Zahlen in eckigen Klammern am Ende der relevanten Sätze hinzufügen.  \n  \nDas Format Ihrer Gesamtantwort sollte daher so aussehen, wie es zwischen den <example></example>-Tags gezeigt wird. Achten Sie darauf, die Formatierung und den Abstand genau zu befolgen.  \n<example>  \nZitate:  \n[1] \"Unternehmen X meldete einen Umsatz von \$12 Millionen im Jahr 2021.\"  \n[2] \"Fast 90% des Umsatzes stammten aus Widget-Verkäufen, wobei Gadget-Verkäufe die verbleibenden 10% ausmachten.\"  \n  \nAntwort:  \nUnternehmen X verdiente \$12 Millionen. [1] Fast 90% davon stammten aus Widget-Verkäufen. [2]  \n</example>  \n  \nWenn die Frage nicht durch das Dokument beantwortet werden kann, sagen Sie das.",
    messages=[
        {
            "role": "user",
            "content": [
                {
                    "type": "text",
                    "text": "Geht es Matterport gut?"
                }
            ]
        }
    ]
)
print(message.content)

```

</Tab>

<Tab title=" Vertex AI TypeScript">

```typescript
import { AnthropicVertex } from '@anthropic-ai/vertex-sdk';

// Reads from the `CLOUD_ML_REGION` & `ANTHROPIC_VERTEX_PROJECT_ID` environment variables.
// Additionally goes through the standard `google-auth-library` flow.
const client = new AnthropicVertex();

const msg = await client.messages.create({
  model: "claude-sonnet-4@20250514",
  max_tokens: 2000,
  temperature: 0,
  system: "Sie sind ein Experte für Forschungsassistenz. Hier ist ein Dokument, zu dem Sie Fragen beantworten werden:  \n<doc>  \n[Volltext der [Matterport SEC-Einreichung 10-K 2023](https://investors.matterport.com/node/9501/html), hier aus Gründen der Kürze nicht eingefügt]  \n</doc>  \n  \nFinden Sie zunächst die Zitate aus dem Dokument, die für die Beantwortung der Frage am relevantesten sind, und drucken Sie sie dann in nummerierter Reihenfolge aus. Zitate sollten relativ kurz sein.  \n  \nWenn es keine relevanten Zitate gibt, schreiben Sie stattdessen \"Keine relevanten Zitate\".  \n  \nBeantworten Sie dann die Frage, beginnend mit \"Antwort:\". Fügen Sie keine zitierten Inhalte wörtlich in die Antwort ein oder verweisen Sie darauf. Sagen Sie nicht \"Laut Zitat [1]\" bei der Beantwortung. Verweisen Sie stattdessen auf für jeden Abschnitt der Antwort relevante Zitate, indem Sie deren Zahlen in eckigen Klammern am Ende der relevanten Sätze hinzufügen.  \n  \nDas Format Ihrer Gesamtantwort sollte daher so aussehen, wie es zwischen den <example></example>-Tags gezeigt wird. Achten Sie darauf, die Formatierung und den Abstand genau zu befolgen.  \n<example>  \nZitate:  \n[1] \"Unternehmen X meldete einen Umsatz von \$12 Millionen im Jahr 2021.\"  \n[2] \"Fast 90% des Umsatzes stammten aus Widget-Verkäufen, wobei Gadget-Verkäufe die verbleibenden 10% ausmachten.\"  \n  \nAntwort:  \nUnternehmen X verdiente \$12 Millionen. [1] Fast 90% davon stammten aus Widget-Verkäufen. [2]  \n</example>  \n  \nWenn die Frage nicht durch das Dokument beantwortet werden kann, sagen Sie das.",
  messages: [
    {
      "role": "user",
      "content": [
        {
          "type": "text",
          "text": "Geht es Matterport gut?"
        }
      ]
    }
  ]
});
console.log(msg);

```

</Tab>
</Tabs>