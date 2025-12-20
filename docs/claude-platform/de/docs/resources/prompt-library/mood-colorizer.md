# Stimmungs-Farbgeber

Verwandelt Textbeschreibungen von Stimmungen in entsprechende HEX-Codes.

---

> Kopiere diesen Prompt in unsere Entwickler-[Konsole](/dashboard), um es selbst auszuprobieren!

|        | Inhalt |
| --- | --- |
| System | Deine Aufgabe ist es, die bereitgestellte Textbeschreibung einer Stimmung oder Emotion zu nehmen und einen HEX-Farbcode zu generieren, der diese Stimmung visuell repräsentiert. Verwende Farbpsychologie-Prinzipien und gängige Assoziationen, um die am besten geeignete Farbe für die gegebene Stimmung zu bestimmen. Wenn die Textbeschreibung unklar, mehrdeutig ist oder nicht genügend Informationen liefert, um eine geeignete Farbe zu bestimmen, antworte mit "Kann keinen HEX-Farbcode für die gegebene Stimmung bestimmen." |
| User   | Eine leidenschaftliche, intensive und feurige Emotion, voller Liebe und Verlangen. |

## Beispielausgabe

`# FF1744`

Dieser HEX-Farbcode repräsentiert einen lebendigen, kräftigen Rotton. Rot wird häufig mit Leidenschaft, Intensität, Liebe und Verlangen assoziiert. Der hier gewählte spezifische Farbton ist ein feuriges, energiegeladenes Rot, das die Hitze und den Eifer der beschriebenen Emotion einfängt. Rot ist dafür bekannt, Gefühle von Aufregung und Leidenschaft zu stimulieren, was es zu einer angemessenen Wahl für die Repräsentation dieser Stimmung macht.

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
  max_tokens=500,
  temperature=0.5,
  system="Deine Aufgabe ist es, die bereitgestellte Textbeschreibung einer Stimmung oder Emotion zu nehmen und einen HEX-Farbcode zu generieren, der diese Stimmung visuell repräsentiert. Verwende Farbpsychologie-Prinzipien und gängige Assoziationen, um die am besten geeignete Farbe für die gegebene Stimmung zu bestimmen. Wenn die Textbeschreibung unklar, mehrdeutig ist oder nicht genügend Informationen liefert, um eine geeignete Farbe zu bestimmen, antworte mit \"Kann keinen HEX-Farbcode für die gegebene Stimmung bestimmen.\"",
  messages=[
    {
    "role": "user",
    "content": [
        {
          "type": "text",
          "text": "Eine leidenschaftliche, intensive und feurige Emotion, voller Liebe und Verlangen."
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
  max_tokens: 500,
  temperature: 0.5,
  system: "Deine Aufgabe ist es, die bereitgestellte Textbeschreibung einer Stimmung oder Emotion zu nehmen und einen HEX-Farbcode zu generieren, der diese Stimmung visuell repräsentiert. Verwende Farbpsychologie-Prinzipien und gängige Assoziationen, um die am besten geeignete Farbe für die gegebene Stimmung zu bestimmen. Wenn die Textbeschreibung unklar, mehrdeutig ist oder nicht genügend Informationen liefert, um eine geeignete Farbe zu bestimmen, antworte mit \"Kann keinen HEX-Farbcode für die gegebene Stimmung bestimmen.\"",
  messages: [
    {
      "role": "user",
      "content": [
        {
          "type": "text",
          "text": "Eine leidenschaftliche, intensive und feurige Emotion, voller Liebe und Verlangen."
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
    max_tokens=500,
    temperature=0.5,
    system="Deine Aufgabe ist es, die bereitgestellte Textbeschreibung einer Stimmung oder Emotion zu nehmen und einen HEX-Farbcode zu generieren, der diese Stimmung visuell repräsentiert. Verwende Farbpsychologie-Prinzipien und gängige Assoziationen, um die am besten geeignete Farbe für die gegebene Stimmung zu bestimmen. Wenn die Textbeschreibung unklar, mehrdeutig ist oder nicht genügend Informationen liefert, um eine geeignete Farbe zu bestimmen, antworte mit \"Kann keinen HEX-Farbcode für die gegebene Stimmung bestimmen.\"",
    messages=[
        {
            "role": "user",
            "content": [
                {
                    "type": "text",
                    "text": "Eine leidenschaftliche, intensive und feurige Emotion, voller Liebe und Verlangen."
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
  max_tokens: 500,
  temperature: 0.5,
  system: "Deine Aufgabe ist es, die bereitgestellte Textbeschreibung einer Stimmung oder Emotion zu nehmen und einen HEX-Farbcode zu generieren, der diese Stimmung visuell repräsentiert. Verwende Farbpsychologie-Prinzipien und gängige Assoziationen, um die am besten geeignete Farbe für die gegebene Stimmung zu bestimmen. Wenn die Textbeschreibung unklar, mehrdeutig ist oder nicht genügend Informationen liefert, um eine geeignete Farbe zu bestimmen, antworte mit \"Kann keinen HEX-Farbcode für die gegebene Stimmung bestimmen.\"",
  messages: [
    {
      "role": "user",
      "content": [
        {
          "type": "text",
          "text": "Eine leidenschaftliche, intensive und feurige Emotion, voller Liebe und Verlangen."
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
    max_tokens=500,
    temperature=0.5,
    system="Deine Aufgabe ist es, die bereitgestellte Textbeschreibung einer Stimmung oder Emotion zu nehmen und einen HEX-Farbcode zu generieren, der diese Stimmung visuell repräsentiert. Verwende Farbpsychologie-Prinzipien und gängige Assoziationen, um die am besten geeignete Farbe für die gegebene Stimmung zu bestimmen. Wenn die Textbeschreibung unklar, mehrdeutig ist oder nicht genügend Informationen liefert, um eine geeignete Farbe zu bestimmen, antworte mit \"Kann keinen HEX-Farbcode für die gegebene Stimmung bestimmen.\"",
    messages=[
        {
            "role": "user",
            "content": [
                {
                    "type": "text",
                    "text": "Eine leidenschaftliche, intensive und feurige Emotion, voller Liebe und Verlangen."
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
  max_tokens: 500,
  temperature: 0.5,
  system: "Deine Aufgabe ist es, die bereitgestellte Textbeschreibung einer Stimmung oder Emotion zu nehmen und einen HEX-Farbcode zu generieren, der diese Stimmung visuell repräsentiert. Verwende Farbpsychologie-Prinzipien und gängige Assoziationen, um die am besten geeignete Farbe für die gegebene Stimmung zu bestimmen. Wenn die Textbeschreibung unklar, mehrdeutig ist oder nicht genügend Informationen liefert, um eine geeignete Farbe zu bestimmen, antworte mit \"Kann keinen HEX-Farbcode für die gegebene Stimmung bestimmen.\"",
  messages: [
    {
      "role": "user",
      "content": [
        {
          "type": "text",
          "text": "Eine leidenschaftliche, intensive und feurige Emotion, voller Liebe und Verlangen."
        }
      ]
    }
  ]
});
console.log(msg);

```

</Tab>
</Tabs>