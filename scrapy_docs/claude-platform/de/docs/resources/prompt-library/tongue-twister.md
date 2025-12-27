# Zungenbrecher

Erstelle herausfordernde Zungenbrecher.

---

> Kopiere diesen Prompt in unsere Entwickler-[Konsole](/dashboard), um es selbst auszuprobieren!

| Rolle | Inhalt |
| ---- | --- |
| Benutzer | Generiere komplexe und kreative Zungenbrecher. Ziele darauf ab, Zungenbrecher zu erstellen, die nicht nur schwer auszusprechen sind, sondern auch ansprechend, unterhaltsam und möglicherweise humorvoll. Berücksichtige Wortspiele, Reime und Alliterationen, um die Schwierigkeit und den Spaß der Zungenbrecher zu erhöhen. |

## Beispielausgabe

Zungenbrecher 1:  
 Die schlaue Spitzmaus kaute durch den glänzenden, schimmernden Schuh.

Zungenbrecher 2:  
 Frische Brisen bliesen am blaublütigen, schwatzenden, plappernden Pavian vorbei.

Zungenbrecher 3:  
 Der verwirrte Pfeifer pflückte einen Scheffel eingelegter Paprika, während er sein Pfeifen übte.

Zungenbrecher 4:  
 Sechs schlanke, glatte, schmale Schlitten glitten schnell den steilen, schneereichen Hang hinunter.

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
          "text": "Generiere komplexe und kreative Zungenbrecher. Ziele darauf ab, Zungenbrecher zu erstellen, die nicht nur schwer auszusprechen sind, sondern auch ansprechend, unterhaltsam und möglicherweise humorvoll. Berücksichtige Wortspiele, Reime und Alliterationen, um die Schwierigkeit und den Spaß der Zungenbrecher zu erhöhen."
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
          "text": "Generiere komplexe und kreative Zungenbrecher. Ziele darauf ab, Zungenbrecher zu erstellen, die nicht nur schwer auszusprechen sind, sondern auch ansprechend, unterhaltsam und möglicherweise humorvoll. Berücksichtige Wortspiele, Reime und Alliterationen, um die Schwierigkeit und den Spaß der Zungenbrecher zu erhöhen."
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
                    "text": "Generiere komplexe und kreative Zungenbrecher. Ziele darauf ab, Zungenbrecher zu erstellen, die nicht nur schwer auszusprechen sind, sondern auch ansprechend, unterhaltsam und möglicherweise humorvoll. Berücksichtige Wortspiele, Reime und Alliterationen, um die Schwierigkeit und den Spaß der Zungenbrecher zu erhöhen."
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
          "text": "Generiere komplexe und kreative Zungenbrecher. Ziele darauf ab, Zungenbrecher zu erstellen, die nicht nur schwer auszusprechen sind, sondern auch ansprechend, unterhaltsam und möglicherweise humorvoll. Berücksichtige Wortspiele, Reime und Alliterationen, um die Schwierigkeit und den Spaß der Zungenbrecher zu erhöhen."
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
                    "text": "Generiere komplexe und kreative Zungenbrecher. Ziele darauf ab, Zungenbrecher zu erstellen, die nicht nur schwer auszusprechen sind, sondern auch ansprechend, unterhaltsam und möglicherweise humorvoll. Berücksichtige Wortspiele, Reime und Alliterationen, um die Schwierigkeit und den Spaß der Zungenbrecher zu erhöhen."
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
          "text": "Generiere komplexe und kreative Zungenbrecher. Ziele darauf ab, Zungenbrecher zu erstellen, die nicht nur schwer auszusprechen sind, sondern auch ansprechend, unterhaltsam und möglicherweise humorvoll. Berücksichtige Wortspiele, Reime und Alliterationen, um die Schwierigkeit und den Spaß der Zungenbrecher zu erhöhen."
        }
      ]
    }
  ]
});
console.log(msg);

```

</Tab>
</Tabs>