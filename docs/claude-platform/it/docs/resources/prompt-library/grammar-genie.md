# Genio della grammatica

Trasforma frasi grammaticalmente scorrette in inglese corretto.

---

> Copia questo prompt nella nostra [Console](/dashboard) per sviluppatori per provarlo tu stesso!

|        | Contenuto |
| --- | --- |
| System | Il tuo compito è prendere il testo fornito e riscriverlo in una versione chiara e grammaticalmente corretta preservando il significato originale il più fedelmente possibile. Correggi eventuali errori di ortografia, errori di punteggiatura, problemi di tempi verbali, problemi di scelta delle parole e altri errori grammaticali. |
| User   | I can haz cheeseburger? |

## Output di Esempio

May I have a cheeseburger?

---

## Richiesta API

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
    temperature=0,
    system="Il tuo compito è prendere il testo fornito e riscriverlo in una versione chiara e grammaticalmente corretta preservando il significato originale il più fedelmente possibile. Correggi eventuali errori di ortografia, errori di punteggiatura, problemi di tempi verbali, problemi di scelta delle parole e altri errori grammaticali.",
    messages=[
        {
            "role": "user",
            "content": [{"type": "text", "text": "I can haz cheeseburger?"}],
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
  max_tokens: 1000,
  temperature: 0,
  system: "Il tuo compito è prendere il testo fornito e riscriverlo in una versione chiara e grammaticalmente corretta preservando il significato originale il più fedelmente possibile. Correggi eventuali errori di ortografia, errori di punteggiatura, problemi di tempi verbali, problemi di scelta delle parole e altri errori grammaticali.",
  messages: [
    {
      "role": "user",
      "content": [
        {
          "type": "text",
          "text": "I can haz cheeseburger?"
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
    system="Il tuo compito è prendere il testo fornito e riscriverlo in una versione chiara e grammaticalmente corretta preservando il significato originale il più fedelmente possibile. Correggi eventuali errori di ortografia, errori di punteggiatura, problemi di tempi verbali, problemi di scelta delle parole e altri errori grammaticali.",
    messages=[
        {
            "role": "user",
            "content": [
                {
                    "type": "text",
                    "text": "I can haz cheeseburger?"
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
system: "Il tuo compito è prendere il testo fornito e riscriverlo in una versione chiara e grammaticalmente corretta preservando il significato originale il più fedelmente possibile. Correggi eventuali errori di ortografia, errori di punteggiatura, problemi di tempi verbali, problemi di scelta delle parole e altri errori grammaticali.",
messages: [
{
"role": "user",
"content": [
{
"type": "text",
"text": "I can haz cheeseburger?"
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
    temperature=0,
    system="Il tuo compito è prendere il testo fornito e riscriverlo in una versione chiara e grammaticalmente corretta preservando il significato originale il più fedelmente possibile. Correggi eventuali errori di ortografia, errori di punteggiatura, problemi di tempi verbali, problemi di scelta delle parole e altri errori grammaticali.",
    messages=[
        {
            "role": "user",
            "content": [
                {
                    "type": "text",
                    "text": "I can haz cheeseburger?"
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
  system: "Il tuo compito è prendere il testo fornito e riscriverlo in una versione chiara e grammaticalmente corretta preservando il significato originale il più fedelmente possibile. Correggi eventuali errori di ortografia, errori di punteggiatura, problemi di tempi verbali, problemi di scelta delle parole e altri errori grammaticali.",
  messages: [
    {
      "role": "user",
      "content": [
        {
          "type": "text",
          "text": "I can haz cheeseburger?"
        }
      ]
    }
  ]
});
console.log(msg);

```

</Tab>
</Tabs>