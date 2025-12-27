# Trivia-Generator

Erstelle Trivia-Fragen zu einer Vielzahl von Themen und biete Hinweise bei Bedarf.

---

> Kopiere diesen Prompt in unsere Entwickler-[Konsole](/dashboard), um es selbst auszuprobieren!

| Rolle | Inhalt |
| ---- | --- |
| Benutzer | Erstelle Trivia-Fragen zu verschiedenen Themen und biete Hinweise, um Benutzern zu helfen, die richtige Antwort zu finden. Wähle aus einer vielfältigen Auswahl von Kategorien und erstelle Fragen, die das Wissen oder die Denkfähigkeiten des Benutzers testen. Biete eine Reihe von zunehmend spezifischen Hinweisen, um Benutzer zur Lösung zu führen. Stelle sicher, dass die Fragen herausfordernd sind und dass die Hinweise genügend Informationen bieten, um dem Benutzer zu helfen, ohne die Antwort zu leicht preiszugeben. |

## Beispielausgabe

Kategorie: Geographie

Frage:  
 Was ist das einzige Land der Welt, das sowohl an Frankreich als auch an Spanien grenzt?

Hinweis 1: Dieses Land liegt in den Pyrenäen zwischen Frankreich und Spanien.  
 Hinweis 2: Es ist bekannt für seine atemberaubenden Berglandschaften, Skigebiete und steuerfreies Einkaufen.  
 Hinweis 3: Die Hauptstadt des Landes trägt den Namen des Landes selbst.

Antwort: Andorra

Kategorie: Wissenschaft

Frage:  
 Wie heißt der Prozess, bei dem Pflanzen Sonnenlicht, Wasser und Kohlendioxid in Energie und Sauerstoff umwandeln?

Hinweis 1: Dieser Prozess ist essentiell für das Leben auf der Erde, da er sowohl Nahrung als auch Sauerstoff liefert.  
 Hinweis 2: Er findet hauptsächlich in den Blättern der Pflanzen statt.  
 Hinweis 3: Der Prozess beinhaltet Chlorophyll, einen grünen Farbstoff, der Sonnenlicht absorbiert.

Antwort: Photosynthese

Kategorie: Literatur

Frage:  
 Wer schrieb den klassischen Roman "Stolz und Vorurteil", der erstmals 1813 veröffentlicht wurde?

Hinweis 1: Die Autorin ist eine britische Frau, die von 1775 bis 1817 lebte.  
 Hinweis 2: Sie ist bekannt für ihre witzigen, einfühlsamen Darstellungen der englischen Gesellschaft und Sitten.  
 Hinweis 3: Ihre anderen berühmten Werke umfassen "Verstand und Gefühl" und "Emma".

Antwort: Jane Austen

Kategorie: Geschichte

Frage:  
 In welchem Jahr begann der Abriss der Berliner Mauer, die Ost- und Westdeutschland teilte?

Hinweis 1: Das Ereignis fand im späten 20. Jahrhundert statt.  
 Hinweis 2: Es markierte einen bedeutsamen Moment im Ende des Kalten Krieges.  
 Hinweis 3: Das Jahr wird oft mit dem Ausdruck "Fall des Kommunismus" in Europa assoziiert.

Antwort: 1989

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
  messages=[
    {
    "role": "user",
    "content": [
        {
          "type": "text",
          "text": "Erstelle Trivia-Fragen zu verschiedenen Themen und biete Hinweise, um Benutzern zu helfen, die richtige Antwort zu finden. Wähle aus einer vielfältigen Auswahl von Kategorien und erstelle Fragen, die das Wissen oder die Denkfähigkeiten des Benutzers testen. Biete eine Reihe von zunehmend spezifischen Hinweisen, um Benutzer zur Lösung zu führen. Stelle sicher, dass die Fragen herausfordernd sind und dass die Hinweise genügend Informationen bieten, um dem Benutzer zu helfen, ohne die Antwort zu leicht preiszugeben."
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
  messages: [
    {
      "role": "user",
      "content": [
        {
          "type": "text",
          "text": "Erstelle Trivia-Fragen zu verschiedenen Themen und biete Hinweise, um Benutzern zu helfen, die richtige Antwort zu finden. Wähle aus einer vielfältigen Auswahl von Kategorien und erstelle Fragen, die das Wissen oder die Denkfähigkeiten des Benutzers testen. Biete eine Reihe von zunehmend spezifischen Hinweisen, um Benutzer zur Lösung zu führen. Stelle sicher, dass die Fragen herausfordernd sind und dass die Hinweise genügend Informationen bieten, um dem Benutzer zu helfen, ohne die Antwort zu leicht preiszugeben."
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
temperature=0.5,
messages=[
{
"role": "user",
"content": [
{
"type": "text",
"text": "Erstelle Trivia-Fragen zu verschiedenen Themen und biete Hinweise, um Benutzern zu helfen, die richtige Antwort zu finden. Wähle aus einer vielfältigen Auswahl von Kategorien und erstelle Fragen, die das Wissen oder die Denkfähigkeiten des Benutzers testen. Biete eine Reihe von zunehmend spezifischen Hinweisen, um Benutzer zur Lösung zu führen. Stelle sicher, dass die Fragen herausfordernd sind und dass die Hinweise genügend Informationen bieten, um dem Benutzer zu helfen, ohne die Antwort zu leicht preiszugeben."
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
  temperature: 0.5,
  messages: [
    {
      "role": "user",
      "content": [
        {
          "type": "text",
          "text": "Erstelle Trivia-Fragen zu verschiedenen Themen und biete Hinweise, um Benutzern zu helfen, die richtige Antwort zu finden. Wähle aus einer vielfältigen Auswahl von Kategorien und erstelle Fragen, die das Wissen oder die Denkfähigkeiten des Benutzers testen. Biete eine Reihe von zunehmend spezifischen Hinweisen, um Benutzer zur Lösung zu führen. Stelle sicher, dass die Fragen herausfordernd sind und dass die Hinweise genügend Informationen bieten, um dem Benutzer zu helfen, ohne die Antwort zu leicht preiszugeben."
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
    temperature=0.5,
    messages=[
        {
            "role": "user",
            "content": [
                {
                    "type": "text",
                    "text": "Erstelle Trivia-Fragen zu verschiedenen Themen und biete Hinweise, um Benutzern zu helfen, die richtige Antwort zu finden. Wähle aus einer vielfältigen Auswahl von Kategorien und erstelle Fragen, die das Wissen oder die Denkfähigkeiten des Benutzers testen. Biete eine Reihe von zunehmend spezifischen Hinweisen, um Benutzer zur Lösung zu führen. Stelle sicher, dass die Fragen herausfordernd sind und dass die Hinweise genügend Informationen bieten, um dem Benutzer zu helfen, ohne die Antwort zu leicht preiszugeben."
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
  temperature: 0.5,
  messages: [
    {
      "role": "user",
      "content": [
        {
          "type": "text",
          "text": "Erstelle Trivia-Fragen zu verschiedenen Themen und biete Hinweise, um Benutzern zu helfen, die richtige Antwort zu finden. Wähle aus einer vielfältigen Auswahl von Kategorien und erstelle Fragen, die das Wissen oder die Denkfähigkeiten des Benutzers testen. Biete eine Reihe von zunehmend spezifischen Hinweisen, um Benutzer zur Lösung zu führen. Stelle sicher, dass die Fragen herausfordernd sind und dass die Hinweise genügend Informationen bieten, um dem Benutzer zu helfen, ohne die Antwort zu leicht preiszugeben."
        }
      ]
    }
  ]
});
console.log(msg);

```

</Tab>
</Tabs>