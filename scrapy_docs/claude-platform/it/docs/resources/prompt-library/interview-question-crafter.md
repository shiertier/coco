# Creatore di domande per colloqui

Genera domande per colloqui.

---

> Copia questo prompt nella nostra [Console](/dashboard) per sviluppatori per provarlo tu stesso!

|        | Contenuto |
| --- | --- |
| System | Il tuo compito è generare una serie di domande ponderate e aperte per un colloquio basate sul contesto fornito. Le domande dovrebbero essere progettate per suscitare risposte perspicaci e dettagliate dall'intervistato, permettendogli di mostrare le proprie conoscenze, esperienza e capacità di pensiero critico. Evita domande sì/no o quelle con risposte ovvie. Invece, concentrati su domande che incoraggiano la riflessione, l'autovalutazione e la condivisione di esempi specifici o aneddoti. |
| User   | Stai intervistando un candidato per una posizione di marketing manager presso un'azienda di e-commerce ben consolidata. Il candidato ideale dovrebbe avere esperienza nello sviluppo e nell'esecuzione di campagne di marketing multi-canale, forti competenze analitiche e la capacità di collaborare efficacemente con team interfunzionali. Dovrebbe anche essere appassionato di rimanere aggiornato con le ultime tendenze e tecnologie di marketing. |

## Output di Esempio

1.  Puoi guidarmi attraverso una campagna di marketing multi-canale di successo che hai sviluppato ed eseguito nel tuo ruolo precedente? Quali sono state le sfide che hai affrontato e come le hai superate?
2.  Come misuri il successo delle tue campagne di marketing e su quali metriche ti concentri? Puoi fornire un esempio di come hai utilizzato dati e analisi per ottimizzare le prestazioni di una campagna?
3.  Nella tua esperienza, quali sono gli elementi chiave di una strategia di marketing e-commerce di successo? Come ti assicuri che le tue strategie si allineino con gli obiettivi aziendali complessivi dell'azienda?
4.  Puoi condividere un esempio di quando hai dovuto collaborare con un team interfunzionale (ad esempio, prodotto, vendite o servizio clienti) per raggiungere un obiettivo di marketing? Quali sfide hai incontrato e come hai favorito una collaborazione efficace?
5.  Come rimani informato sulle ultime tendenze e tecnologie di marketing e come hai applicato questa conoscenza per migliorare le tue strategie o campagne di marketing?
6.  Puoi descrivere un momento in cui una campagna di marketing a cui hai partecipato non ha performato come previsto? Cosa hai imparato dall'esperienza e come hai applicato quelle lezioni alle campagne future?
7.  La nostra azienda valorizza l'innovazione e il miglioramento continuo. Puoi condividere un esempio di quando hai introdotto un nuovo approccio di marketing, strumento o tecnologia che ha impattato significativamente le prestazioni o i risultati del tuo team?

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
  temperature=0.5,
  system="Il tuo compito è generare una serie di domande ponderate e aperte per un colloquio basate sul contesto fornito. Le domande dovrebbero essere progettate per suscitare risposte perspicaci e dettagliate dall'intervistato, permettendogli di mostrare le proprie conoscenze, esperienza e capacità di pensiero critico. Evita domande sì/no o quelle con risposte ovvie. Invece, concentrati su domande che incoraggiano la riflessione, l'autovalutazione e la condivisione di esempi specifici o aneddoti.",
  messages=[
    {
      "role": "user",
      "content": [
        {
          "type": "text",
          "text": "Stai intervistando un candidato per una posizione di marketing manager presso un'azienda di e-commerce ben consolidata. Il candidato ideale dovrebbe avere esperienza nello sviluppo e nell'esecuzione di campagne di marketing multi-canale, forti competenze analitiche e la capacità di collaborare efficacemente con team interfunzionali. Dovrebbe anche essere appassionato di rimanere aggiornato con le ultime tendenze e tecnologie di marketing."
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
  temperature: 0.5,
  system: "Il tuo compito è generare una serie di domande ponderate e aperte per un colloquio basate sul contesto fornito. Le domande dovrebbero essere progettate per suscitare risposte perspicaci e dettagliate dall'intervistato, permettendogli di mostrare le proprie conoscenze, esperienza e capacità di pensiero critico. Evita domande sì/no o quelle con risposte ovvie. Invece, concentrati su domande che incoraggiano la riflessione, l'autovalutazione e la condivisione di esempi specifici o aneddoti.",
  messages: [
    {
      "role": "user",
      "content": [
        {
          "type": "text",
          "text": "Stai intervistando un candidato per una posizione di marketing manager presso un'azienda di e-commerce ben consolidata. Il candidato ideale dovrebbe avere esperienza nello sviluppo e nell'esecuzione di campagne di marketing multi-canale, forti competenze analitiche e la capacità di collaborare efficacemente con team interfunzionali. Dovrebbe anche essere appassionato di rimanere aggiornato con le ultime tendenze e tecnologie di marketing."
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
    temperature=0.5,
    system="Il tuo compito è generare una serie di domande ponderate e aperte per un colloquio basate sul contesto fornito. Le domande dovrebbero essere progettate per suscitare risposte perspicaci e dettagliate dall'intervistato, permettendogli di mostrare le proprie conoscenze, esperienza e capacità di pensiero critico. Evita domande sì/no o quelle con risposte ovvie. Invece, concentrati su domande che incoraggiano la riflessione, l'autovalutazione e la condivisione di esempi specifici o aneddoti.",
    messages=[
        {
            "role": "user",
            "content": [
                {
                    "type": "text",
                    "text": "Stai intervistando un candidato per una posizione di marketing manager presso un'azienda di e-commerce ben consolidata. Il candidato ideale dovrebbe avere esperienza nello sviluppo e nell'esecuzione di campagne di marketing multi-canale, forti competenze analitiche e la capacità di collaborare efficacemente con team interfunzionali. Dovrebbe anche essere appassionato di rimanere aggiornato con le ultime tendenze e tecnologie di marketing."
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
temperature: 0.5,
system: "Il tuo compito è generare una serie di domande ponderate e aperte per un colloquio basate sul contesto fornito. Le domande dovrebbero essere progettate per suscitare risposte perspicaci e dettagliate dall'intervistato, permettendogli di mostrare le proprie conoscenze, esperienza e capacità di pensiero critico. Evita domande sì/no o quelle con risposte ovvie. Invece, concentrati su domande che incoraggiano la riflessione, l'autovalutazione e la condivisione di esempi specifici o aneddoti.",
messages: [
{
"role": "user",
"content": [
{
"type": "text",
"text": "Stai intervistando un candidato per una posizione di marketing manager presso un'azienda di e-commerce ben consolidata. Il candidato ideale dovrebbe avere esperienza nello sviluppo e nell'esecuzione di campagne di marketing multi-canale, forti competenze analitiche e la capacità di collaborare efficacemente con team interfunzionali. Dovrebbe anche essere appassionato di rimanere aggiornato con le ultime tendenze e tecnologie di marketing."
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
    temperature=0.5,
    system="Il tuo compito è generare una serie di domande ponderate e aperte per un colloquio basate sul contesto fornito. Le domande dovrebbero essere progettate per suscitare risposte perspicaci e dettagliate dall'intervistato, permettendogli di mostrare le proprie conoscenze, esperienza e capacità di pensiero critico. Evita domande sì/no o quelle con risposte ovvie. Invece, concentrati su domande che incoraggiano la riflessione, l'autovalutazione e la condivisione di esempi specifici o aneddoti.",
    messages=[
        {
            "role": "user",
            "content": [
                {
                    "type": "text",
                    "text": "Stai intervistando un candidato per una posizione di marketing manager presso un'azienda di e-commerce ben consolidata. Il candidato ideale dovrebbe avere esperienza nello sviluppo e nell'esecuzione di campagne di marketing multi-canale, forti competenze analitiche e la capacità di collaborare efficacemente con team interfunzionali. Dovrebbe anche essere appassionato di rimanere aggiornato con le ultime tendenze e tecnologie di marketing."
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
  temperature: 0.5,
  system: "Il tuo compito è generare una serie di domande ponderate e aperte per un colloquio basate sul contesto fornito. Le domande dovrebbero essere progettate per suscitare risposte perspicaci e dettagliate dall'intervistato, permettendogli di mostrare le proprie conoscenze, esperienza e capacità di pensiero critico. Evita domande sì/no o quelle con risposte ovvie. Invece, concentrati su domande che incoraggiano la riflessione, l'autovalutazione e la condivisione di esempi specifici o aneddoti.",
  messages: [
    {
      "role": "user",
      "content": [
        {
          "type": "text",
          "text": "Stai intervistando un candidato per una posizione di marketing manager presso un'azienda di e-commerce ben consolidata. Il candidato ideale dovrebbe avere esperienza nello sviluppo e nell'esecuzione di campagne di marketing multi-canale, forti competenze analitiche e la capacità di collaborare efficacemente con team interfunzionali. Dovrebbe anche essere appassionato di rimanere aggiornato con le ultime tendenze e tecnologie di marketing."
        }
      ]
    }
  ]
});
console.log(msg);

```

</Tab>
</Tabs>