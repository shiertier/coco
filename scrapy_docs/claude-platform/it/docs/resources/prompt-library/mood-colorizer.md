# Colorizzatore di stati d'animo

Trasforma le descrizioni testuali degli stati d'animo in codici HEX corrispondenti.

---

> Copia questo prompt nella nostra [Console](/dashboard) per sviluppatori per provarlo tu stesso!

|        | Contenuto |
| --- | --- |
| System | Il tuo compito è prendere la descrizione testuale fornita di uno stato d'animo o emozione e generare un codice colore HEX che rappresenti visivamente quello stato d'animo. Usa i principi della psicologia del colore e le associazioni comuni per determinare il colore più appropriato per lo stato d'animo dato. Se la descrizione testuale non è chiara, ambigua, o non fornisce abbastanza informazioni per determinare un colore adatto, rispondi con "Impossibile determinare un codice colore HEX per lo stato d'animo dato." |
| User   | Un'emozione appassionata, intensa e ardente, piena di amore e desiderio. |

## Output di Esempio

`# FF1744`

Questo codice colore HEX rappresenta una tonalità vivida e audace di rosso. Il rosso è comunemente associato alla passione, intensità, amore e desiderio. La tonalità specifica scelta qui è un rosso ardente ed energico che cattura il calore e il fervore dell'emozione descritta. Il rosso è noto per stimolare sentimenti di eccitazione e passione, rendendolo una scelta appropriata per rappresentare questo stato d'animo.

---

## Richiesta API

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
  system="Il tuo compito è prendere la descrizione testuale fornita di uno stato d'animo o emozione e generare un codice colore HEX che rappresenti visivamente quello stato d'animo. Usa i principi della psicologia del colore e le associazioni comuni per determinare il colore più appropriato per lo stato d'animo dato. Se la descrizione testuale non è chiara, ambigua, o non fornisce abbastanza informazioni per determinare un colore adatto, rispondi con \"Impossibile determinare un codice colore HEX per lo stato d'animo dato.\"",
  messages=[
    {
    "role": "user",
    "content": [
        {
          "type": "text",
          "text": "Un'emozione appassionata, intensa e ardente, piena di amore e desiderio."
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
  system: "Il tuo compito è prendere la descrizione testuale fornita di uno stato d'animo o emozione e generare un codice colore HEX che rappresenti visivamente quello stato d'animo. Usa i principi della psicologia del colore e le associazioni comuni per determinare il colore più appropriato per lo stato d'animo dato. Se la descrizione testuale non è chiara, ambigua, o non fornisce abbastanza informazioni per determinare un colore adatto, rispondi con \"Impossibile determinare un codice colore HEX per lo stato d'animo dato.\"",
  messages: [
    {
      "role": "user",
      "content": [
        {
          "type": "text",
          "text": "Un'emozione appassionata, intensa e ardente, piena di amore e desiderio."
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
    system="Il tuo compito è prendere la descrizione testuale fornita di uno stato d'animo o emozione e generare un codice colore HEX che rappresenti visivamente quello stato d'animo. Usa i principi della psicologia del colore e le associazioni comuni per determinare il colore più appropriato per lo stato d'animo dato. Se la descrizione testuale non è chiara, ambigua, o non fornisce abbastanza informazioni per determinare un colore adatto, rispondi con \"Impossibile determinare un codice colore HEX per lo stato d'animo dato.\"",
    messages=[
        {
            "role": "user",
            "content": [
                {
                    "type": "text",
                    "text": "Un'emozione appassionata, intensa e ardente, piena di amore e desiderio."
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
  system: "Il tuo compito è prendere la descrizione testuale fornita di uno stato d'animo o emozione e generare un codice colore HEX che rappresenti visivamente quello stato d'animo. Usa i principi della psicologia del colore e le associazioni comuni per determinare il colore più appropriato per lo stato d'animo dato. Se la descrizione testuale non è chiara, ambigua, o non fornisce abbastanza informazioni per determinare un colore adatto, rispondi con \"Impossibile determinare un codice colore HEX per lo stato d'animo dato.\"",
  messages: [
    {
      "role": "user",
      "content": [
        {
          "type": "text",
          "text": "Un'emozione appassionata, intensa e ardente, piena di amore e desiderio."
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
    system="Il tuo compito è prendere la descrizione testuale fornita di uno stato d'animo o emozione e generare un codice colore HEX che rappresenti visivamente quello stato d'animo. Usa i principi della psicologia del colore e le associazioni comuni per determinare il colore più appropriato per lo stato d'animo dato. Se la descrizione testuale non è chiara, ambigua, o non fornisce abbastanza informazioni per determinare un colore adatto, rispondi con \"Impossibile determinare un codice colore HEX per lo stato d'animo dato.\"",
    messages=[
        {
            "role": "user",
            "content": [
                {
                    "type": "text",
                    "text": "Un'emozione appassionata, intensa e ardente, piena di amore e desiderio."
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
  system: "Il tuo compito è prendere la descrizione testuale fornita di uno stato d'animo o emozione e generare un codice colore HEX che rappresenti visivamente quello stato d'animo. Usa i principi della psicologia del colore e le associazioni comuni per determinare il colore più appropriato per lo stato d'animo dato. Se la descrizione testuale non è chiara, ambigua, o non fornisce abbastanza informazioni per determinare un colore adatto, rispondi con \"Impossibile determinare un codice colore HEX per lo stato d'animo dato.\"",
  messages: [
    {
      "role": "user",
      "content": [
        {
          "type": "text",
          "text": "Un'emozione appassionata, intensa e ardente, piena di amore e desiderio."
        }
      ]
    }
  ]
});
console.log(msg);

```

</Tab>
</Tabs>