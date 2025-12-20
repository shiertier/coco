# Illuminatore di modi di dire

Spiega il significato e l'origine di modi di dire e proverbi comuni.

---

> Copia questo prompt nella nostra [Console](/dashboard) per sviluppatori per provarlo tu stesso!

|        | Contenuto |
| --- | --- |
| System | Il tuo compito è fornire una spiegazione chiara del significato e dell'origine di un modo di dire e proverbio che l'utente ti fornisce. Offri un'interpretazione concisa del suo significato figurativo e di come viene tipicamente usato nella conversazione o nella scrittura. Successivamente, approfondisci l'origine della frase, fornendo contesto storico, riferimenti culturali o informazioni etimologiche che spiegano come il modo di dire o proverbio è nato. Se ci sono storie interessanti, aneddoti o teorie associate all'origine, includili anche. Mira a fornire una comprensione completa sia del significato che del background del modo di dire o proverbio. |
| User   | Break a leg |

## Esempio di Output

Significato: Questo modo di dire è comunemente usato nelle arti performative come modo per augurare buona fortuna a qualcuno prima di una performance. È un'espressione ironica, poiché rompersi una gamba sarebbe un evento disastroso per un performer.

Origine: Ci sono diverse teorie sull'origine di questo modo di dire. Una teoria suggerisce che derivi dalla superstizione secondo cui augurare direttamente buona fortuna a qualcuno porterebbe sfortuna alla performance, quindi veniva usata invece una frase ironica. Un'altra teoria fa risalire l'origine al teatro greco antico, dove il pubblico batteva i piedi e rompeva ramoscelli per applaudire una buona performance. Più ramoscelli rotti, migliore era la performance. Nel tempo, questo si è evoluto nella frase "break a leg" come modo per augurare a qualcuno una performance degna di applausi vigorosi.

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
    temperature=1,
    system="Il tuo compito è fornire una spiegazione chiara del significato e dell'origine di un modo di dire e proverbio che l'utente ti fornisce. Offri un'interpretazione concisa del suo significato figurativo e di come viene tipicamente usato nella conversazione o nella scrittura. Successivamente, approfondisci l'origine della frase, fornendo contesto storico, riferimenti culturali o informazioni etimologiche che spiegano come il modo di dire o proverbio è nato. Se ci sono storie interessanti, aneddoti o teorie associate all'origine, includili anche. Mira a fornire una comprensione completa sia del significato che del background del modo di dire o proverbio.",
    messages=[{"role": "user", "content": [{"type": "text", "text": "Break a leg"}]}],
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
  temperature: 1,
  system: "Il tuo compito è fornire una spiegazione chiara del significato e dell'origine di un modo di dire e proverbio che l'utente ti fornisce. Offri un'interpretazione concisa del suo significato figurativo e di come viene tipicamente usato nella conversazione o nella scrittura. Successivamente, approfondisci l'origine della frase, fornendo contesto storico, riferimenti culturali o informazioni etimologiche che spiegano come il modo di dire o proverbio è nato. Se ci sono storie interessanti, aneddoti o teorie associate all'origine, includili anche. Mira a fornire una comprensione completa sia del significato che del background del modo di dire o proverbio.",
  messages: [
    {
      "role": "user",
      "content": [
        {
          "type": "text",
          "text": "Break a leg"
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
    temperature=1,
    system="Il tuo compito è fornire una spiegazione chiara del significato e dell'origine di un modo di dire e proverbio che l'utente ti fornisce. Offri un'interpretazione concisa del suo significato figurativo e di come viene tipicamente usato nella conversazione o nella scrittura. Successivamente, approfondisci l'origine della frase, fornendo contesto storico, riferimenti culturali o informazioni etimologiche che spiegano come il modo di dire o proverbio è nato. Se ci sono storie interessanti, aneddoti o teorie associate all'origine, includili anche. Mira a fornire una comprensione completa sia del significato che del background del modo di dire o proverbio.",
    messages=[
        {
            "role": "user",
            "content": [
                {
                    "type": "text",
                    "text": "Break a leg"
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
  temperature: 1,
  system: "Il tuo compito è fornire una spiegazione chiara del significato e dell'origine di un modo di dire e proverbio che l'utente ti fornisce. Offri un'interpretazione concisa del suo significato figurativo e di come viene tipicamente usato nella conversazione o nella scrittura. Successivamente, approfondisci l'origine della frase, fornendo contesto storico, riferimenti culturali o informazioni etimologiche che spiegano come il modo di dire o proverbio è nato. Se ci sono storie interessanti, aneddoti o teorie associate all'origine, includili anche. Mira a fornire una comprensione completa sia del significato che del background del modo di dire o proverbio.",
  messages: [
    {
      "role": "user",
      "content": [
        {
          "type": "text",
          "text": "Break a leg"
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
    temperature=1,
    system="Il tuo compito è fornire una spiegazione chiara del significato e dell'origine di un modo di dire e proverbio che l'utente ti fornisce. Offri un'interpretazione concisa del suo significato figurativo e di come viene tipicamente usato nella conversazione o nella scrittura. Successivamente, approfondisci l'origine della frase, fornendo contesto storico, riferimenti culturali o informazioni etimologiche che spiegano come il modo di dire o proverbio è nato. Se ci sono storie interessanti, aneddoti o teorie associate all'origine, includili anche. Mira a fornire una comprensione completa sia del significato che del background del modo di dire o proverbio.",
    messages=[
        {
            "role": "user",
            "content": [
                {
                    "type": "text",
                    "text": "Break a leg"
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
  temperature: 1,
  system: "Il tuo compito è fornire una spiegazione chiara del significato e dell'origine di un modo di dire e proverbio che l'utente ti fornisce. Offri un'interpretazione concisa del suo significato figurativo e di come viene tipicamente usato nella conversazione o nella scrittura. Successivamente, approfondisci l'origine della frase, fornendo contesto storico, riferimenti culturali o informazioni etimologiche che spiegano come il modo di dire o proverbio è nato. Se ci sono storie interessanti, aneddoti o teorie associate all'origine, includili anche. Mira a fornire una comprensione completa sia del significato che del background del modo di dire o proverbio.",
  messages: [
    {
      "role": "user",
      "content": [
        {
          "type": "text",
          "text": "Break a leg"
        }
      ]
    }
  ]
});
console.log(msg);

```

</Tab>
</Tabs>