# Scioglilingua

Crea scioglilingua impegnativi.

---

> Copia questo prompt nella nostra [Console](/dashboard) per sviluppatori per provarlo tu stesso!

| Ruolo | Contenuto |
| ---- | --- |
| Utente | Genera scioglilingua complessi e creativi. Mira a creare scioglilingua che non siano solo difficili da pronunciare ma anche coinvolgenti, divertenti e potenzialmente umoristici. Considera di incorporare giochi di parole, rime e allitterazioni per aumentare la difficoltà e il divertimento degli scioglilingua. |

## Output di Esempio

Scioglilingua 1:  
 L'astuto toporagno masticò attraverso la scarpa lucida e scintillante.

Scioglilingua 2:  
 Brezze vivaci soffiarono vicino al babbuino di sangue blu, chiacchierone e balbettante.

Scioglilingua 3:  
 Il pifferaio perplesso raccolse un beccata di peperoni sottaceto mentre praticava il suo suonare.

Scioglilingua 4:  
 Sei slitte eleganti, lisce, snelle scivolarono rapidamente giù per il pendio ripido e innevato.

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
  max_tokens=1000,
  temperature=1,
  messages=[
    {
      "role": "user",
      "content": [
        {
          "type": "text",
          "text": "Genera scioglilingua complessi e creativi. Mira a creare scioglilingua che non siano solo difficili da pronunciare ma anche coinvolgenti, divertenti e potenzialmente umoristici. Considera di incorporare giochi di parole, rime e allitterazioni per aumentare la difficoltà e il divertimento degli scioglilingua."
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
  max_tokens: 1000,
  temperature: 1,
  messages: [
    {
      "role": "user",
      "content": [
        {
          "type": "text",
          "text": "Genera scioglilingua complessi e creativi. Mira a creare scioglilingua che non siano solo difficili da pronunciare ma anche coinvolgenti, divertenti e potenzialmente umoristici. Considera di incorporare giochi di parole, rime e allitterazioni per aumentare la difficoltà e il divertimento degli scioglilingua."
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
    messages=[
        {
            "role": "user",
            "content": [
                {
                    "type": "text",
                    "text": "Genera scioglilingua complessi e creativi. Mira a creare scioglilingua che non siano solo difficili da pronunciare ma anche coinvolgenti, divertenti e potenzialmente umoristici. Considera di incorporare giochi di parole, rime e allitterazioni per aumentare la difficoltà e il divertimento degli scioglilingua."
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
  messages: [
    {
      "role": "user",
      "content": [
        {
          "type": "text",
          "text": "Genera scioglilingua complessi e creativi. Mira a creare scioglilingua che non siano solo difficili da pronunciare ma anche coinvolgenti, divertenti e potenzialmente umoristici. Considera di incorporare giochi di parole, rime e allitterazioni per aumentare la difficoltà e il divertimento degli scioglilingua."
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
    messages=[
        {
            "role": "user",
            "content": [
                {
                    "type": "text",
                    "text": "Genera scioglilingua complessi e creativi. Mira a creare scioglilingua che non siano solo difficili da pronunciare ma anche coinvolgenti, divertenti e potenzialmente umoristici. Considera di incorporare giochi di parole, rime e allitterazioni per aumentare la difficoltà e il divertimento degli scioglilingua."
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
  messages: [
    {
      "role": "user",
      "content": [
        {
          "type": "text",
          "text": "Genera scioglilingua complessi e creativi. Mira a creare scioglilingua che non siano solo difficili da pronunciare ma anche coinvolgenti, divertenti e potenzialmente umoristici. Considera di incorporare giochi di parole, rime e allitterazioni per aumentare la difficoltà e il divertimento degli scioglilingua."
        }
      ]
    }
  ]
});
console.log(msg);

```

</Tab>
</Tabs>