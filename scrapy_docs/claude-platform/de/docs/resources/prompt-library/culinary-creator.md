# Kulinarischer Schöpfer

Schlagen Sie Rezeptideen basierend auf den verfügbaren Zutaten und Ernährungsvorlieben des Benutzers vor.

---

> Kopieren Sie diesen Prompt in unsere Entwickler-[Konsole](/dashboard), um es selbst auszuprobieren!

|        | Inhalt |
| --- | --- |
| System | Ihre Aufgabe ist es, personalisierte Rezeptideen basierend auf den vom Benutzer eingegebenen verfügbaren Zutaten und Ernährungsvorlieben zu generieren. Verwenden Sie diese Informationen, um eine Vielzahl kreativer und köstlicher Rezepte vorzuschlagen, die mit den gegebenen Zutaten zubereitet werden können, während sie die Ernährungsbedürfnisse des Benutzers berücksichtigen, falls welche erwähnt werden. Stellen Sie für jedes Rezept eine kurze Beschreibung, eine Liste der erforderlichen Zutaten und eine einfache Anleitung bereit. Stellen Sie sicher, dass die Rezepte leicht zu befolgen, nahrhaft sind und mit minimalen zusätzlichen Zutaten oder Geräten zubereitet werden können. |
| User   | Verfügbare Zutaten: Hähnchenbrust, Brokkoli, Karotten, Zwiebel, Knoblauch, Olivenöl, Reis, glutenfreie Sojasauce, Honig <br/> Ernährungsvorlieben: glutenfrei |

## Beispielausgabe

Vorgeschlagene Rezepte:

Honig-Knoblauch-Hähnchen-Pfannengericht

Beschreibung: Ein schnelles und geschmackvolles Pfannengericht mit zartem Hähnchenfleisch, knackigem Gemüse und einer süßen und herzhaften Honig-Knoblauch-Sauce.  
 Zutaten:

- 2 Hähnchenbrüste, in mundgerechte Stücke geschnitten
- 2 Tassen Brokkoliröschen
- 1 Tasse geschnittene Karotten
- 1 Zwiebel, gewürfelt
- 3 Knoblauchzehen, gehackt
- 2 EL Olivenöl
- 1/4 Tasse Sojasauce (glutenfrei)
- 2 EL Honig

Anleitung:

1.  In einer großen Pfanne oder einem Wok das Olivenöl bei mittlerer bis hoher Hitze erhitzen.
2.  Gewürfelte Zwiebel und gehackten Knoblauch hinzufügen und anbraten, bis sie duften, etwa 2 Minuten.
3.  Hähnchenstücke hinzufügen und kochen, bis sie nicht mehr rosa sind, etwa 5-7 Minuten.
4.  Brokkoli und Karotten hinzufügen und pfannenrühren, bis das Gemüse bissfest ist, etwa 5 Minuten.
5.  In einer kleinen Schüssel Sojasauce und Honig zusammenrühren.
6.  Die Sauce über das Hähnchen und Gemüse gießen und gleichmäßig vermengen.
7.  Weitere 2-3 Minuten kochen, oder bis die Sauce leicht eingedickt ist.
8.  Heiß mit gedämpftem Reis servieren.

Hähnchen- und Gemüse-Reis-Schalen

Beschreibung: Eine herzhafte und nahrhafte Reisschale, belegt mit gewürztem Hähnchen, geröstetem Gemüse und einem Spritzer Sojasauce.

Zutaten:

- 2 Hähnchenbrüste, in mundgerechte Stücke geschnitten
- 2 Tassen Brokkoliröschen
- 1 Tasse geschnittene Karotten
- 1 Zwiebel, gewürfelt
- 2 Knoblauchzehen, gehackt
- 2 EL Olivenöl
- 1/4 Tasse Sojasauce (glutenfrei)
- 2 Tassen gekochter Reis

Anleitung:

1.  Den Ofen auf 200°C (400°F) vorheizen.
2.  In einer großen Schüssel Hähnchenstücke, Brokkoli, Karotten, Zwiebel und Knoblauch vermengen.
3.  Mit Olivenöl und der Hälfte der Sojasauce beträufeln und gleichmäßig vermengen.
4.  Das Hähnchen und Gemüse auf einem Backblech verteilen und im vorgeheizten Ofen 20-25 Minuten rösten, oder bis das Hähnchen durchgegart und das Gemüse zart ist.
5.  Gekochten Reis auf Servierschalen verteilen.
6.  Jede Schale mit dem gerösteten Hähnchen und Gemüse belegen.
7.  Die restliche Sojasauce über jede Schale träufeln.
8.  Heiß servieren und genießen!

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
  temperature=0.5,
  system="Ihre Aufgabe ist es, personalisierte Rezeptideen basierend auf den vom Benutzer eingegebenen verfügbaren Zutaten und Ernährungsvorlieben zu generieren. Verwenden Sie diese Informationen, um eine Vielzahl kreativer und köstlicher Rezepte vorzuschlagen, die mit den gegebenen Zutaten zubereitet werden können, während sie die Ernährungsbedürfnisse des Benutzers berücksichtigen, falls welche erwähnt werden. Stellen Sie für jedes Rezept eine kurze Beschreibung, eine Liste der erforderlichen Zutaten und eine einfache Anleitung bereit. Stellen Sie sicher, dass die Rezepte leicht zu befolgen, nahrhaft sind und mit minimalen zusätzlichen Zutaten oder Geräten zubereitet werden können.",
  messages=[
    {
    "role": "user",
    "content": [
        {
          "type": "text",
          "text": "Verfügbare Zutaten: Hähnchenbrust, Brokkoli, Karotten, Zwiebel, Knoblauch, Olivenöl, Reis, glutenfreie Sojasauce, Honig \nErnährungsvorlieben: glutenfrei"
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
  temperature: 0.5,
  system: "Ihre Aufgabe ist es, personalisierte Rezeptideen basierend auf den vom Benutzer eingegebenen verfügbaren Zutaten und Ernährungsvorlieben zu generieren. Verwenden Sie diese Informationen, um eine Vielzahl kreativer und köstlicher Rezepte vorzuschlagen, die mit den gegebenen Zutaten zubereitet werden können, während sie die Ernährungsbedürfnisse des Benutzers berücksichtigen, falls welche erwähnt werden. Stellen Sie für jedes Rezept eine kurze Beschreibung, eine Liste der erforderlichen Zutaten und eine einfache Anleitung bereit. Stellen Sie sicher, dass die Rezepte leicht zu befolgen, nahrhaft sind und mit minimalen zusätzlichen Zutaten oder Geräten zubereitet werden können.",
  messages: [
    {
      "role": "user",
      "content": [
        {
          "type": "text",
          "text": "Verfügbare Zutaten: Hähnchenbrust, Brokkoli, Karotten, Zwiebel, Knoblauch, Olivenöl, Reis, glutenfreie Sojasauce, Honig  \nErnährungsvorlieben: glutenfrei"
        }
      ]
    }
  ]
});
console.log(msg);

````

</Tab>
<Tab title="AWS Bedrock Python">

```
from anthropic import AnthropicBedrock

# See https://docs.claude.com/claude/reference/claude-on-amazon-bedrock
# for authentication options
client = AnthropicBedrock()

message = client.messages.create(
    model="anthropic.claude-sonnet-4-5-20250929-v1:0",
    max_tokens=2000,
    temperature=0.5,
    system="Ihre Aufgabe ist es, personalisierte Rezeptideen basierend auf den vom Benutzer eingegebenen verfügbaren Zutaten und Ernährungsvorlieben zu generieren. Verwenden Sie diese Informationen, um eine Vielzahl kreativer und köstlicher Rezepte vorzuschlagen, die mit den gegebenen Zutaten zubereitet werden können, während sie die Ernährungsbedürfnisse des Benutzers berücksichtigen, falls welche erwähnt werden. Stellen Sie für jedes Rezept eine kurze Beschreibung, eine Liste der erforderlichen Zutaten und eine einfache Anleitung bereit. Stellen Sie sicher, dass die Rezepte leicht zu befolgen, nahrhaft sind und mit minimalen zusätzlichen Zutaten oder Geräten zubereitet werden können.",
    messages=[
        {
            "role": "user",
            "content": [
                {
                    "type": "text",
                    "text": "Verfügbare Zutaten: Hähnchenbrust, Brokkoli, Karotten, Zwiebel, Knoblauch, Olivenöl, Reis, glutenfreie Sojasauce, Honig  \nErnährungsvorlieben: glutenfrei"
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
import AnthropicBedrock from "@anthropic-ai/bedrock-sdk";

// See https://docs.claude.com/claude/reference/claude-on-amazon-bedrock
// for authentication options
const client = new AnthropicBedrock();

const msg = await client.messages.create({
  model: "anthropic.claude-sonnet-4-5-20250929-v1:0",
  max_tokens: 2000,
  temperature: 0.5,
  system: "Ihre Aufgabe ist es, personalisierte Rezeptideen basierend auf den vom Benutzer eingegebenen verfügbaren Zutaten und Ernährungsvorlieben zu generieren. Verwenden Sie diese Informationen, um eine Vielzahl kreativer und köstlicher Rezepte vorzuschlagen, die mit den gegebenen Zutaten zubereitet werden können, während sie die Ernährungsbedürfnisse des Benutzers berücksichtigen, falls welche erwähnt werden. Stellen Sie für jedes Rezept eine kurze Beschreibung, eine Liste der erforderlichen Zutaten und eine einfache Anleitung bereit. Stellen Sie sicher, dass die Rezepte leicht zu befolgen, nahrhaft sind und mit minimalen zusätzlichen Zutaten oder Geräten zubereitet werden können.",
  messages: [
    {
      "role": "user",
      "content": [
        {
          "type": "text",
          "text": "Verfügbare Zutaten: Hähnchenbrust, Brokkoli, Karotten, Zwiebel, Knoblauch, Olivenöl, Reis, glutenfreie Sojasauce, Honig  \nErnährungsvorlieben: glutenfrei"
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
    max_tokens=2000,
    temperature=0.5,
    system="Ihre Aufgabe ist es, personalisierte Rezeptideen basierend auf den vom Benutzer eingegebenen verfügbaren Zutaten und Ernährungsvorlieben zu generieren. Verwenden Sie diese Informationen, um eine Vielzahl kreativer und köstlicher Rezepte vorzuschlagen, die mit den gegebenen Zutaten zubereitet werden können, während sie die Ernährungsbedürfnisse des Benutzers berücksichtigen, falls welche erwähnt werden. Stellen Sie für jedes Rezept eine kurze Beschreibung, eine Liste der erforderlichen Zutaten und eine einfache Anleitung bereit. Stellen Sie sicher, dass die Rezepte leicht zu befolgen, nahrhaft sind und mit minimalen zusätzlichen Zutaten oder Geräten zubereitet werden können.",
    messages=[
        {
            "role": "user",
            "content": [
                {
                    "type": "text",
                    "text": "Verfügbare Zutaten: Hähnchenbrust, Brokkoli, Karotten, Zwiebel, Knoblauch, Olivenöl, Reis, glutenfreie Sojasauce, Honig  \nErnährungsvorlieben: glutenfrei"
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
  max_tokens: 2000,
  temperature: 0.5,
  system: "Ihre Aufgabe ist es, personalisierte Rezeptideen basierend auf den vom Benutzer eingegebenen verfügbaren Zutaten und Ernährungsvorlieben zu generieren. Verwenden Sie diese Informationen, um eine Vielzahl kreativer und köstlicher Rezepte vorzuschlagen, die mit den gegebenen Zutaten zubereitet werden können, während sie die Ernährungsbedürfnisse des Benutzers berücksichtigen, falls welche erwähnt werden. Stellen Sie für jedes Rezept eine kurze Beschreibung, eine Liste der erforderlichen Zutaten und eine einfache Anleitung bereit. Stellen Sie sicher, dass die Rezepte leicht zu befolgen, nahrhaft sind und mit minimalen zusätzlichen Zutaten oder Geräten zubereitet werden können.",
  messages: [
    {
      "role": "user",
      "content": [
        {
          "type": "text",
          "text": "Verfügbare Zutaten: Hähnchenbrust, Brokkoli, Karotten, Zwiebel, Knoblauch, Olivenöl, Reis, glutenfreie Sojasauce, Honig  \nErnährungsvorlieben: glutenfrei"
        }
      ]
    }
  ]
});
console.log(msg);

```

</Tab>
</Tabs>