# Unternehmens-Hellseher

Extrahieren Sie Erkenntnisse, identifizieren Sie Risiken und destillieren Sie wichtige Informationen aus langen Unternehmensberichten in ein einziges Memo.

---

> Kopieren Sie diesen Prompt in unsere Entwickler-[Konsole](/dashboard), um es selbst auszuprobieren!

|      | Inhalt |
| ---- | --- |
| Benutzer | Ihre Aufgabe ist es, den folgenden Bericht zu analysieren: <br /> <br /> <report> [Volltext der [Matterport SEC-Einreichung 10-K 2023](https://investors.matterport.com/node/9501/html), hier aus Gründen der Kürze nicht eingefügt] </report> <br /> <br /> Fassen Sie diesen Jahresbericht prägnant und klar zusammen und identifizieren Sie wichtige Markttrends und Erkenntnisse. Geben Sie Ihre Ergebnisse als kurzes Memo aus, das ich an mein Team senden kann. Das Ziel des Memos ist es, sicherzustellen, dass mein Team über die Lage der Finanzinstitute auf dem Laufenden bleibt und qualitativ prognostiziert und identifiziert, ob im kommenden Quartal operative und Umsatzrisiken zu erwarten sind. Stellen Sie sicher, dass Sie alle relevanten Details in Ihre Zusammenfassung und Analyse einbeziehen. |

## Beispielausgabe

> An: Investment-Team  
> Von: \[Ihr Name\]  
> Datum: 4. März 2024  
> Betreff: Matterport 2022 10-K Highlights und Finanzausblick
>
> Hallo Team,
>
> Ich habe einige wichtige Erkenntnisse aus Matterports Jahresbericht 2022 identifiziert:
>
> Finanzielle Leistung:
>
> - Der Umsatz stieg um 22% auf 136,1 Mio. $ im Jahr 2022, angetrieben durch das Wachstum bei Abonnement- und Serviceumsätzen
> - Die Bruttomarge sank jedoch von 54% auf 38% aufgrund erhöhter Kosten durch Produktumsätze und Lieferkettenherausforderungen
> - Die Betriebsverluste weiteten sich auf 275,5 Mio. $ aus, da das Unternehmen stark in F&E sowie Vertrieb und Marketing investierte, um das Wachstum voranzutreiben
> - Beendete 2022 mit 476,9 Mio. $ in Bargeld und Investitionen zur Finanzierung der Geschäftstätigkeit
>
> Geschäftshöhepunkte:
>
> - Die Abonnentenbasis wuchs um 39% auf über 701.000 Ende 2022. Die verwalteten Räume erreichten 9,2 Millionen
> - Erweiterte Plattform mit der Einführung der Matterport Axis motorisierten Halterung, Pro3-Kamera und wichtigen Updates der Cloud-Software
> - Abgeschlossene Akquisitionen von Enview (KI für 3D-Raumdaten) und VHT Studios (Immobilienmarketing) zur Verbesserung der Angebote
> - Unterzeichnete bemerkenswerte Partnerschaften mit Burns & McDonnell und CGS, um Matterports Lösungen an ihre Unternehmenskunden zu liefern
>
> Ausblick:
>
> - Matterport sieht eine massive Marktchance bei der Digitalisierung der Gebäude und Räume der Welt (schätzt 240+ Mrd. $ TAM)
> - Fokussiert auf die Förderung der Akzeptanz bei großen Unternehmenskunden in wichtigen Branchen wie Immobilien, AEC, Facility Management und mehr
> - Wird weiterhin in F&E und internationale Expansion investieren, um den weitgehend ungenutzten globalen Gebäudemarkt zu durchdringen
> - Kurzfristiges Wachstum und Margen könnten durch makroökonomische Gegenwinds und Lieferkettenprobleme beeinträchtigt werden
> - Die gesunde Liquiditätsposition und relativ treue Kundenbasis bieten jedoch Schutz nach unten
>
> Zusammenfassend lieferte Matterport 2022 starkes Umsatzwachstum, während es Fortschritte bei der Durchdringung eines riesigen globalen Marktes machte. Während die Rentabilität herausgefordert bleibt, da das Unternehmen stark für das Wachstum investiert, sieht seine Bilanz solide aus. Abgesehen von einigen anhaltenden Lieferkettenproblemen scheint die zugrunde liegende Geschäftsdynamik basierend auf wichtigen Betriebskennzahlen intakt zu sein. Abgesehen von einem schweren wirtschaftlichen Abschwung scheint Matterport gut positioniert zu sein, um weiterhin Marktanteile im entstehenden Gebäudedigitalisierungsbereich zu gewinnen.

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
    max_tokens=2000,
    temperature=0,
    messages=[
        {
            "role": "user",
            "content": [
                {
                    "type": "text",
                    "text": "Ihre Aufgabe ist es, den folgenden Bericht zu analysieren:  \n<report>  \n[Volltext der [Matterport SEC-Einreichung 10-K 2023](https://investors.matterport.com/node/9501/html), hier aus Gründen der Kürze nicht eingefügt]  \n</report>  \n  \nFassen Sie diesen Jahresbericht prägnant und klar zusammen und identifizieren Sie wichtige Markttrends und Erkenntnisse. Geben Sie Ihre Ergebnisse als kurzes Memo aus, das ich an mein Team senden kann. Das Ziel des Memos ist es, sicherzustellen, dass mein Team über die Lage der Finanzinstitute auf dem Laufenden bleibt und qualitativ prognostiziert und identifiziert, ob im kommenden Quartal operative und Umsatzrisiken zu erwarten sind. Stellen Sie sicher, dass Sie alle relevanten Details in Ihre Zusammenfassung und Analyse einbeziehen."
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
  temperature: 0,
  messages: [
    {
      "role": "user",
      "content": [
        {
          "type": "text",
          "text": "Ihre Aufgabe ist es, den folgenden Bericht zu analysieren:  \n<report>  \n[Volltext der [Matterport SEC-Einreichung 10-K 2023](https://investors.matterport.com/node/9501/html), hier aus Gründen der Kürze nicht eingefügt]  \n</report>  \n  \nFassen Sie diesen Jahresbericht prägnant und klar zusammen und identifizieren Sie wichtige Markttrends und Erkenntnisse. Geben Sie Ihre Ergebnisse als kurzes Memo aus, das ich an mein Team senden kann. Das Ziel des Memos ist es, sicherzustellen, dass mein Team über die Lage der Finanzinstitute auf dem Laufenden bleibt und qualitativ prognostiziert und identifiziert, ob im kommenden Quartal operative und Umsatzrisiken zu erwarten sind. Stellen Sie sicher, dass Sie alle relevanten Details in Ihre Zusammenfassung und Analyse einbeziehen."
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
messages=[
{
"role": "user",
"content": [
{
"type": "text",
"text": "Ihre Aufgabe ist es, den folgenden Bericht zu analysieren: \n<report> \n[Volltext der [Matterport SEC-Einreichung 10-K 2023](https://investors.matterport.com/node/9501/html), hier aus Gründen der Kürze nicht eingefügt] \n</report> \n \nFassen Sie diesen Jahresbericht prägnant und klar zusammen und identifizieren Sie wichtige Markttrends und Erkenntnisse. Geben Sie Ihre Ergebnisse als kurzes Memo aus, das ich an mein Team senden kann. Das Ziel des Memos ist es, sicherzustellen, dass mein Team über die Lage der Finanzinstitute auf dem Laufenden bleibt und qualitativ prognostiziert und identifiziert, ob im kommenden Quartal operative und Umsatzrisiken zu erwarten sind. Stellen Sie sicher, dass Sie alle relevanten Details in Ihre Zusammenfassung und Analyse einbeziehen."
}
]
}
]
)
print(message.content)

````
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
  messages: [
    {
      "role": "user",
      "content": [
        {
          "type": "text",
          "text": "Ihre Aufgabe ist es, den folgenden Bericht zu analysieren:  \n<report>  \n[Volltext der [Matterport SEC-Einreichung 10-K 2023](https://investors.matterport.com/node/9501/html), hier aus Gründen der Kürze nicht eingefügt]  \n</report>  \n  \nFassen Sie diesen Jahresbericht prägnant und klar zusammen und identifizieren Sie wichtige Markttrends und Erkenntnisse. Geben Sie Ihre Ergebnisse als kurzes Memo aus, das ich an mein Team senden kann. Das Ziel des Memos ist es, sicherzustellen, dass mein Team über die Lage der Finanzinstitute auf dem Laufenden bleibt und qualitativ prognostiziert und identifiziert, ob im kommenden Quartal operative und Umsatzrisiken zu erwarten sind. Stellen Sie sicher, dass Sie alle relevanten Details in Ihre Zusammenfassung und Analyse einbeziehen."
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
    max_tokens=2000,
    temperature=0,
    messages=[
        {
            "role": "user",
            "content": [
                {
                    "type": "text",
                    "text": "Ihre Aufgabe ist es, den folgenden Bericht zu analysieren:  \n<report>  \n[Volltext der [Matterport SEC-Einreichung 10-K 2023](https://investors.matterport.com/node/9501/html), hier aus Gründen der Kürze nicht eingefügt]  \n</report>  \n  \nFassen Sie diesen Jahresbericht prägnant und klar zusammen und identifizieren Sie wichtige Markttrends und Erkenntnisse. Geben Sie Ihre Ergebnisse als kurzes Memo aus, das ich an mein Team senden kann. Das Ziel des Memos ist es, sicherzustellen, dass mein Team über die Lage der Finanzinstitute auf dem Laufenden bleibt und qualitativ prognostiziert und identifiziert, ob im kommenden Quartal operative und Umsatzrisiken zu erwarten sind. Stellen Sie sicher, dass Sie alle relevanten Details in Ihre Zusammenfassung und Analyse einbeziehen."
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
  temperature: 0,
  messages: [
    {
      "role": "user",
      "content": [
        {
          "type": "text",
          "text": "Ihre Aufgabe ist es, den folgenden Bericht zu analysieren:  \n<report>  \n[Volltext der [Matterport SEC-Einreichung 10-K 2023](https://investors.matterport.com/node/9501/html), hier aus Gründen der Kürze nicht eingefügt]  \n</report>  \n  \nFassen Sie diesen Jahresbericht prägnant und klar zusammen und identifizieren Sie wichtige Markttrends und Erkenntnisse. Geben Sie Ihre Ergebnisse als kurzes Memo aus, das ich an mein Team senden kann. Das Ziel des Memos ist es, sicherzustellen, dass mein Team über die Lage der Finanzinstitute auf dem Laufenden bleibt und qualitativ prognostiziert und identifiziert, ob im kommenden Quartal operative und Umsatzrisiken zu erwarten sind. Stellen Sie sicher, dass Sie alle relevanten Details in Ihre Zusammenfassung und Analyse einbeziehen."
        }
      ]
    }
  ]
});
console.log(msg);

```

  </Tab>
</Tabs>