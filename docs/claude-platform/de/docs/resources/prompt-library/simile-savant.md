# Gleichnis-Gelehrter

Generiere Gleichnisse aus einfachen Beschreibungen.

---

> Kopiere diesen Prompt in unsere Entwickler-[Konsole](/dashboard), um es selbst auszuprobieren!

| Rolle | Inhalt |
| ---- | --- |
| Benutzer | Hilf mir dabei, einige Gleichnisse zu erstellen, um das Lachen einer Person zu beschreiben, das fröhlich und ansteckend ist? |

## Beispielausgabe

1.  Sein Lachen sprudelte hervor wie eine frisch geöffnete Flasche Champagner, sprudelnd und voller Leben.
2.  Ihr Lachen war so warm und einladend wie ein gemütlicher Kamin an einem kühlen Abend und zog die Menschen dazu ein, an der Fröhlichkeit teilzuhaben.

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
  max_tokens=1000,
  temperature=1,
  messages=[
    {
      "role": "user",
      "content": [
        {
          "type": "text",
          "text": "Hilf mir dabei, einige Gleichnisse zu erstellen, um das Lachen einer Person zu beschreiben, das fröhlich und ansteckend ist?"
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
          "text": "Hilf mir dabei, einige Gleichnisse zu erstellen, um das Lachen einer Person zu beschreiben, das fröhlich und ansteckend ist?"
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
                    "text": "Hilf mir dabei, einige Gleichnisse zu erstellen, um das Lachen einer Person zu beschreiben, das fröhlich und ansteckend ist?"
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
          "text": "Hilf mir dabei, einige Gleichnisse zu erstellen, um das Lachen einer Person zu beschreiben, das fröhlich und ansteckend ist?"
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
                    "text": "Hilf mir dabei, einige Gleichnisse zu erstellen, um das Lachen einer Person zu beschreiben, das fröhlich und ansteckend ist?"
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
          "text": "Hilf mir dabei, einige Gleichnisse zu erstellen, um das Lachen einer Person zu beschreiben, das fröhlich und ansteckend ist?"
        }
      ]
    }
  ]
});
console.log(msg);

```

</Tab>
</Tabs>