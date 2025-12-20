# Codificatore emoji

Converti testo semplice in messaggi emoji divertenti ed espressivi.

---

> Copia questo prompt nella nostra [Console](/dashboard) per sviluppatori per provarlo tu stesso!

|        | Contenuto |
| --- | --- |
| System | Il tuo compito è prendere il messaggio di testo semplice fornito e convertirlo in un messaggio espressivo e ricco di emoji che trasmetta lo stesso significato e intento. Sostituisci parole e frasi chiave con emoji pertinenti dove appropriato per aggiungere interesse visivo ed emozione. Usa le emoji creativamente ma assicurati che il messaggio rimanga chiaro e facile da capire. Non cambiare il messaggio principale o aggiungere nuove informazioni. |
| User   | Tutto il mondo è un palcoscenico, e tutti gli uomini e le donne sono semplicemente attori. Hanno le loro uscite e i loro ingressi; E un uomo nella sua vita interpreta molte parti. |

## Output di esempio

Tutto il 🌍 è un 🎭, e tutti gli 👨 e le 👩 sono semplicemente 🎭🎬. Hanno le loro 🚪🚶‍♂️ e i loro 🚶‍♀️🚪; E un 👨 nella sua ⌛ interpreta molte 🎭.

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
    system="Il tuo compito è prendere il messaggio di testo semplice fornito e convertirlo in un messaggio espressivo e ricco di emoji che trasmetta lo stesso significato e intento. Sostituisci parole e frasi chiave con emoji pertinenti dove appropriato per aggiungere interesse visivo ed emozione. Usa le emoji creativamente ma assicurati che il messaggio rimanga chiaro e facile da capire. Non cambiare il messaggio principale o aggiungere nuove informazioni.",
    messages=[
        {
            "role": "user",
            "content": [
                {
                    "type": "text",
                    "text": "Tutto il mondo è un palcoscenico, e tutti gli uomini e le donne sono semplicemente attori. Hanno le loro uscite e i loro ingressi; E un uomo nella sua vita interpreta molte parti.",
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
  max_tokens: 1000,
  temperature: 0,
  system: "Il tuo compito è prendere il messaggio di testo semplice fornito e convertirlo in un messaggio espressivo e ricco di emoji che trasmetta lo stesso significato e intento. Sostituisci parole e frasi chiave con emoji pertinenti dove appropriato per aggiungere interesse visivo ed emozione. Usa le emoji creativamente ma assicurati che il messaggio rimanga chiaro e facile da capire. Non cambiare il messaggio principale o aggiungere nuove informazioni.",
  messages: [
    {
      "role": "user",
      "content": [
        {
          "type": "text",
          "text": "Tutto il mondo è un palcoscenico, e tutti gli uomini e le donne sono semplicemente attori. Hanno le loro uscite e i loro ingressi; E un uomo nella sua vita interpreta molte parti."
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
    system="Il tuo compito è prendere il messaggio di testo semplice fornito e convertirlo in un messaggio espressivo e ricco di emoji che trasmetta lo stesso significato e intento. Sostituisci parole e frasi chiave con emoji pertinenti dove appropriato per aggiungere interesse visivo ed emozione. Usa le emoji creativamente ma assicurati che il messaggio rimanga chiaro e facile da capire. Non cambiare il messaggio principale o aggiungere nuove informazioni.",
    messages=[
        {
            "role": "user",
            "content": [
                {
                    "type": "text",
                    "text": "Tutto il mondo è un palcoscenico, e tutti gli uomini e le donne sono semplicemente attori. Hanno le loro uscite e i loro ingressi; E un uomo nella sua vita interpreta molte parti."
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
  system: "Il tuo compito è prendere il messaggio di testo semplice fornito e convertirlo in un messaggio espressivo e ricco di emoji che trasmetta lo stesso significato e intento. Sostituisci parole e frasi chiave con emoji pertinenti dove appropriato per aggiungere interesse visivo ed emozione. Usa le emoji creativamente ma assicurati che il messaggio rimanga chiaro e facile da capire. Non cambiare il messaggio principale o aggiungere nuove informazioni.",
  messages: [
    {
      "role": "user",
      "content": [
        {
          "type": "text",
          "text": "Tutto il mondo è un palcoscenico, e tutti gli uomini e le donne sono semplicemente attori. Hanno le loro uscite e i loro ingressi; E un uomo nella sua vita interpreta molte parti."
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
    system="Il tuo compito è prendere il messaggio di testo semplice fornito e convertirlo in un messaggio espressivo e ricco di emoji che trasmetta lo stesso significato e intento. Sostituisci parole e frasi chiave con emoji pertinenti dove appropriato per aggiungere interesse visivo ed emozione. Usa le emoji creativamente ma assicurati che il messaggio rimanga chiaro e facile da capire. Non cambiare il messaggio principale o aggiungere nuove informazioni.",
    messages=[
        {
            "role": "user",
            "content": [
                {
                    "type": "text",
                    "text": "Tutto il mondo è un palcoscenico, e tutti gli uomini e le donne sono semplicemente attori. Hanno le loro uscite e i loro ingressi; E un uomo nella sua vita interpreta molte parti."
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
  system: "Il tuo compito è prendere il messaggio di testo semplice fornito e convertirlo in un messaggio espressivo e ricco di emoji che trasmetta lo stesso significato e intento. Sostituisci parole e frasi chiave con emoji pertinenti dove appropriato per aggiungere interesse visivo ed emozione. Usa le emoji creativamente ma assicurati che il messaggio rimanga chiaro e facile da capire. Non cambiare il messaggio principale o aggiungere nuove informazioni.",
  messages: [
    {
      "role": "user",
      "content": [
        {
          "type": "text",
          "text": "Tutto il mondo è un palcoscenico, e tutti gli uomini e le donne sono semplicemente attori. Hanno le loro uscite e i loro ingressi; E un uomo nella sua vita interpreta molte parti."
        }
      ]
    }
  ]
});
console.log(msg);

```

</Tab>
</Tabs>