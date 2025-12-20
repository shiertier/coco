# Bewertungsguru

Vergleichen und bewerten Sie die Qualität schriftlicher Texte basierend auf benutzerdefinierten Kriterien und Standards.

---

> Kopieren Sie diesen Prompt in unsere Entwickler-[Konsole](/dashboard), um ihn selbst auszuprobieren!

| Rolle | Inhalt |
| ---- | --- |
| Benutzer | Bewerten Sie die folgenden beiden Texte basierend auf den gegebenen Kriterien: <br/> <br/> Text 1: <br/> Die Sonne ging hinter den Bergen unter und warf einen warmen goldenen Schein über das Tal. Die Vögel zwitscherten ihre Abendlieder, während die sanfte Brise durch die Blätter raschelte. Es war ein friedlicher und ruhiger Moment, einer, der das Herz mit Freude und Zufriedenheit erfüllte. <br/><br/> Text 2: <br/> Der Sonnenuntergang war schön. Die Vögel sangen und der Wind wehte. Es machte mich glücklich. <br/><br/> Kriterien: <br/> 1. Beschreibende Sprache und Bildsprache <br/> 2. Satzstruktur und Vielfalt <br/> 3. Emotionale Wirkung und Engagement <br/> 4. Grammatik und Zeichensetzung |

## Beispielausgabe

Basierend auf den bereitgestellten Kriterien ist hier meine Bewertung der beiden Texte:

Text 1:

1.  Beschreibende Sprache und Bildsprache: Der Text verwendet lebendige und eindrucksvolle Sprache, um ein Bild der Szenerie zu malen. Phrasen wie "warmer goldener Schein", "Vögel zwitscherten ihre Abendlieder" und "sanfte Brise raschelte durch die Blätter" schaffen starke visuelle und auditive Bilder und tauchen den Leser in die Szene ein.
2.  Satzstruktur und Vielfalt: Der Text zeigt eine gute Mischung aus Satzstrukturen mit sowohl einfachen als auch zusammengesetzten Sätzen. Diese Vielfalt hilft dabei, einen fließenden Ablauf aufrechtzuerhalten und hält den Leser engagiert.
3.  Emotionale Wirkung und Engagement: Der Text vermittelt effektiv ein Gefühl von Frieden, Freude und Zufriedenheit und lädt den Leser ein, an der emotionalen Erfahrung des Moments teilzuhaben. Die Beschreibung der Szenerie und die Verwendung positiver Sprache schaffen eine starke emotionale Verbindung.
4.  Grammatik und Zeichensetzung: Der Text scheint grammatikalisch korrekt und gut interpunktiert zu sein, ohne offensichtliche Fehler oder Probleme.

Text 2:

1.  Beschreibende Sprache und Bildsprache: Dem Text fehlt detaillierte beschreibende Sprache und Bildsprache. Obwohl er den Sonnenuntergang, die Vögel und den Wind erwähnt, bietet er keine lebendigen oder eindrucksvollen Details, die dem Leser helfen würden, die Szene zu visualisieren.
2.  Satzstruktur und Vielfalt: Der Text besteht aus kurzen, einfachen Sätzen mit wenig Variation in der Struktur. Dieser Mangel an Vielfalt kann das Schreiben monoton und weniger ansprechend wirken lassen.
3.  Emotionale Wirkung und Engagement: Der Text vermittelt eine positive Emotion, indem er "Es machte mich glücklich" sagt, aber er bietet aufgrund des Mangels an beschreibender Sprache und Bildsprache keine starke emotionale Verbindung oder Engagement.
4.  Grammatik und Zeichensetzung: Der Text scheint grammatikalisch korrekt und gut interpunktiert zu sein, ohne offensichtliche Fehler oder Probleme.

Vergleichende Analyse:  
 Basierend auf der Bewertung schneidet Text 1 deutlich besser ab als Text 2 in Bezug auf beschreibende Sprache, Bildsprache, Satzstruktur, Vielfalt und emotionale Wirkung. Text 1 schafft eine lebendige und immersive Erfahrung für den Leser und zeigt eine höhere Schreibqualität basierend auf den gegebenen Kriterien, mit seiner starken beschreibenden Sprache, vielfältigen Satzstrukturen und effektivem emotionalem Engagement. Text 2 ist zwar grammatikalisch korrekt, aber ihm fehlt der beschreibende Reichtum und die emotionale Wirkung, die in Text 1 vorhanden sind.

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
    messages=[
        {
            "role": "user",
            "content": [
                {
                    "type": "text",
                    "text": "Bewerten Sie die folgenden beiden Texte basierend auf den gegebenen Kriterien: \n \nText 1: \nDie Sonne ging hinter den Bergen unter und warf einen warmen goldenen Schein über das Tal. Die Vögel zwitscherten ihre Abendlieder, während die sanfte Brise durch die Blätter raschelte. Es war ein friedlicher und ruhiger Moment, einer, der das Herz mit Freude und Zufriedenheit erfüllte. \n \nText 2: \nDer Sonnenuntergang war schön. Die Vögel sangen und der Wind wehte. Es machte mich glücklich. \n \nKriterien: \n1. Beschreibende Sprache und Bildsprache \n2. Satzstruktur und Vielfalt \n3. Emotionale Wirkung und Engagement \n4. Grammatik und Zeichensetzung",
                }
            ],
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
  messages: [
    {
      "role": "user",
      "content": [
        {
          "type": "text",
          "text": "Bewerten Sie die folgenden beiden Texte basierend auf den gegebenen Kriterien:  \n  \nText 1:  \nDie Sonne ging hinter den Bergen unter und warf einen warmen goldenen Schein über das Tal. Die Vögel zwitscherten ihre Abendlieder, während die sanfte Brise durch die Blätter raschelte. Es war ein friedlicher und ruhiger Moment, einer, der das Herz mit Freude und Zufriedenheit erfüllte.  \n  \nText 2:  \nDer Sonnenuntergang war schön. Die Vögel sangen und der Wind wehte. Es machte mich glücklich.  \n  \nKriterien:  \n1. Beschreibende Sprache und Bildsprache  \n2. Satzstruktur und Vielfalt  \n3. Emotionale Wirkung und Engagement  \n4. Grammatik und Zeichensetzung"
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
                    "text": "Bewerten Sie die folgenden beiden Texte basierend auf den gegebenen Kriterien:  \n  \nText 1:  \nDie Sonne ging hinter den Bergen unter und warf einen warmen goldenen Schein über das Tal. Die Vögel zwitscherten ihre Abendlieder, während die sanfte Brise durch die Blätter raschelte. Es war ein friedlicher und ruhiger Moment, einer, der das Herz mit Freude und Zufriedenheit erfüllte.  \n  \nText 2:  \nDer Sonnenuntergang war schön. Die Vögel sangen und der Wind wehte. Es machte mich glücklich.  \n  \nKriterien:  \n1. Beschreibende Sprache und Bildsprache  \n2. Satzstruktur und Vielfalt  \n3. Emotionale Wirkung und Engagement  \n4. Grammatik und Zeichensetzung"
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
  messages: [
    {
      "role": "user",
      "content": [
        {
          "type": "text",
          "text": "Bewerten Sie die folgenden beiden Texte basierend auf den gegebenen Kriterien:  \n  \nText 1:  \nDie Sonne ging hinter den Bergen unter und warf einen warmen goldenen Schein über das Tal. Die Vögel zwitscherten ihre Abendlieder, während die sanfte Brise durch die Blätter raschelte. Es war ein friedlicher und ruhiger Moment, einer, der das Herz mit Freude und Zufriedenheit erfüllte.  \n  \nText 2:  \nDer Sonnenuntergang war schön. Die Vögel sangen und der Wind wehte. Es machte mich glücklich.  \n  \nKriterien:  \n1. Beschreibende Sprache und Bildsprache  \n2. Satzstruktur und Vielfalt  \n3. Emotionale Wirkung und Engagement  \n4. Grammatik und Zeichensetzung"
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
    messages=[
        {
            "role": "user",
            "content": [
                {
                    "type": "text",
                    "text": "Bewerten Sie die folgenden beiden Texte basierend auf den gegebenen Kriterien:  \n  \nText 1:  \nDie Sonne ging hinter den Bergen unter und warf einen warmen goldenen Schein über das Tal. Die Vögel zwitscherten ihre Abendlieder, während die sanfte Brise durch die Blätter raschelte. Es war ein friedlicher und ruhiger Moment, einer, der das Herz mit Freude und Zufriedenheit erfüllte.  \n  \nText 2:  \nDer Sonnenuntergang war schön. Die Vögel sangen und der Wind wehte. Es machte mich glücklich.  \n  \nKriterien:  \n1. Beschreibende Sprache und Bildsprache  \n2. Satzstruktur und Vielfalt  \n3. Emotionale Wirkung und Engagement  \n4. Grammatik und Zeichensetzung"
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
          "text": "Bewerten Sie die folgenden beiden Texte basierend auf den gegebenen Kriterien:  \n  \nText 1:  \nDie Sonne ging hinter den Bergen unter und warf einen warmen goldenen Schein über das Tal. Die Vögel zwitscherten ihre Abendlieder, während die sanfte Brise durch die Blätter raschelte. Es war ein friedlicher und ruhiger Moment, einer, der das Herz mit Freude und Zufriedenheit erfüllte.  \n  \nText 2:  \nDer Sonnenuntergang war schön. Die Vögel sangen und der Wind wehte. Es machte mich glücklich.  \n  \nKriterien:  \n1. Beschreibende Sprache und Bildsprache  \n2. Satzstruktur und Vielfalt  \n3. Emotionale Wirkung und Engagement  \n4. Grammatik und Zeichensetzung"
        }
      ]
    }
  ]
});
console.log(msg);

```

</Tab>
</Tabs>