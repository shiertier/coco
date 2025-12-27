# Semplificatore per seconda elementare

Rendi i testi complessi facili da capire per i giovani studenti.

---

> Copia questo prompt nella nostra [Console](/dashboard) per sviluppatori per provarlo tu stesso!

|        | Contenuto |
| --- | --- |
| System | Il tuo compito è prendere il testo fornito e riscriverlo in modo che sia facile da leggere e comprendere per i giovani studenti delle classi 3-5. Semplifica il vocabolario avanzato, spezza le frasi lunghe, spiega i concetti difficili in linguaggio semplice e presenta le informazioni in modo chiaro e coinvolgente. Il breve testo riscritto dovrebbe trasmettere le idee principali del testo originale in modo appropriato per l'età. |
| User   | I mitocondri, spesso chiamati le centrali elettriche della cellula, sono organelli essenziali che svolgono un ruolo cruciale nella produzione di adenosina trifosfato (ATP) attraverso il processo di respirazione cellulare. L'ATP è la principale valuta energetica della cellula, che consente il verificarsi di varie funzioni cellulari e reazioni biochimiche. |

## Esempio di Output

I mitocondri sono piccole parti all'interno delle cellule che funzionano come piccole centrali elettriche. Producono una molecola speciale chiamata ATP, che è come il denaro energetico della cellula. L'ATP aiuta la cellula a svolgere lavori importanti e la mantiene in funzione senza problemi, proprio come il denaro aiuta le persone a comprare le cose di cui hanno bisogno. I mitocondri sono molto importanti perché senza di loro, le cellule non avrebbero l'energia per funzionare correttamente.

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
  temperature=0,
  system="Il tuo compito è prendere il testo fornito e riscriverlo in modo che sia facile da leggere e comprendere per i giovani studenti delle classi 3-5. Semplifica il vocabolario avanzato, spezza le frasi lunghe, spiega i concetti difficili in linguaggio semplice e presenta le informazioni in modo chiaro e coinvolgente. Il breve testo riscritto dovrebbe trasmettere le idee principali del testo originale in modo appropriato per l'età.",
  messages=[
    {
      "role": "user",
      "content": [
        {
          "type": "text",
          "text": "I mitocondri, spesso chiamati le centrali elettriche della cellula, sono organelli essenziali che svolgono un ruolo cruciale nella produzione di adenosina trifosfato (ATP) attraverso il processo di respirazione cellulare. L'ATP è la principale valuta energetica della cellula, che consente il verificarsi di varie funzioni cellulari e reazioni biochimiche."
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
  temperature: 0,
  system: "Il tuo compito è prendere il testo fornito e riscriverlo in modo che sia facile da leggere e comprendere per i giovani studenti delle classi 3-5. Semplifica il vocabolario avanzato, spezza le frasi lunghe, spiega i concetti difficili in linguaggio semplice e presenta le informazioni in modo chiaro e coinvolgente. Il breve testo riscritto dovrebbe trasmettere le idee principali del testo originale in modo appropriato per l'età.",
  messages: [
    {
      "role": "user",
      "content": [
        {
          "type": "text",
          "text": "I mitocondri, spesso chiamati le centrali elettriche della cellula, sono organelli essenziali che svolgono un ruolo cruciale nella produzione di adenosina trifosfato (ATP) attraverso il processo di respirazione cellulare. L'ATP è la principale valuta energetica della cellula, che consente il verificarsi di varie funzioni cellulari e reazioni biochimiche."
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
    system="Il tuo compito è prendere il testo fornito e riscriverlo in modo che sia facile da leggere e comprendere per i giovani studenti delle classi 3-5. Semplifica il vocabolario avanzato, spezza le frasi lunghe, spiega i concetti difficili in linguaggio semplice e presenta le informazioni in modo chiaro e coinvolgente. Il breve testo riscritto dovrebbe trasmettere le idee principali del testo originale in modo appropriato per l'età.",
    messages=[
        {
            "role": "user",
            "content": [
                {
                    "type": "text",
                    "text": "I mitocondri, spesso chiamati le centrali elettriche della cellula, sono organelli essenziali che svolgono un ruolo cruciale nella produzione di adenosina trifosfato (ATP) attraverso il processo di respirazione cellulare. L'ATP è la principale valuta energetica della cellula, che consente il verificarsi di varie funzioni cellulari e reazioni biochimiche."
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
  system: "Il tuo compito è prendere il testo fornito e riscriverlo in modo che sia facile da leggere e comprendere per i giovani studenti delle classi 3-5. Semplifica il vocabolario avanzato, spezza le frasi lunghe, spiega i concetti difficili in linguaggio semplice e presenta le informazioni in modo chiaro e coinvolgente. Il breve testo riscritto dovrebbe trasmettere le idee principali del testo originale in modo appropriato per l'età.",
  messages: [
    {
      "role": "user",
      "content": [
        {
          "type": "text",
          "text": "I mitocondri, spesso chiamati le centrali elettriche della cellula, sono organelli essenziali che svolgono un ruolo cruciale nella produzione di adenosina trifosfato (ATP) attraverso il processo di respirazione cellulare. L'ATP è la principale valuta energetica della cellula, che consente il verificarsi di varie funzioni cellulari e reazioni biochimiche."
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
system="Il tuo compito è prendere il testo fornito e riscriverlo in modo che sia facile da leggere e comprendere per i giovani studenti delle classi 3-5. Semplifica il vocabolario avanzato, spezza le frasi lunghe, spiega i concetti difficili in linguaggio semplice e presenta le informazioni in modo chiaro e coinvolgente. Il breve testo riscritto dovrebbe trasmettere le idee principali del testo originale in modo appropriato per l'età.",
messages=[
{
"role": "user",
"content": [
{
"type": "text",
"text": "I mitocondri, spesso chiamati le centrali elettriche della cellula, sono organelli essenziali che svolgono un ruolo cruciale nella produzione di adenosina trifosfato (ATP) attraverso il processo di respirazione cellulare. L'ATP è la principale valuta energetica della cellula, che consente il verificarsi di varie funzioni cellulari e reazioni biochimiche."
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
  system: "Il tuo compito è prendere il testo fornito e riscriverlo in modo che sia facile da leggere e comprendere per i giovani studenti delle classi 3-5. Semplifica il vocabolario avanzato, spezza le frasi lunghe, spiega i concetti difficili in linguaggio semplice e presenta le informazioni in modo chiaro e coinvolgente. Il breve testo riscritto dovrebbe trasmettere le idee principali del testo originale in modo appropriato per l'età.",
  messages: [
    {
      "role": "user",
      "content": [
        {
          "type": "text",
          "text": "I mitocondri, spesso chiamati le centrali elettriche della cellula, sono organelli essenziali che svolgono un ruolo cruciale nella produzione di adenosina trifosfato (ATP) attraverso il processo di respirazione cellulare. L'ATP è la principale valuta energetica della cellula, che consente il verificarsi di varie funzioni cellulari e reazioni biochimiche."
        }
      ]
    }
  ]
});
console.log(msg);

````

</Tab>
</Tabs>