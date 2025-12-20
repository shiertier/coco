# Indovinami questo

Genera indovinelli e guida l'utente verso le soluzioni.

---

> Copia questo prompt nella nostra [Console](/dashboard) per sviluppatori per provarlo tu stesso!

| Ruolo | Contenuto |
| ---- | --- |
| Utente | Genera un indovinello intelligente e fornisci una guida passo dopo passo per aiutare l'utente ad arrivare alle soluzioni corrette. L'indovinello dovrebbe essere impegnativo ma risolvibile con il pensiero logico e l'attenzione ai dettagli. Dopo aver presentato ogni indovinello, offri una serie di suggerimenti o domande che guidano progressivamente l'utente verso la risposta. Assicurati che i suggerimenti non siano troppo ovvi ma forniscano comunque abbastanza informazioni per guidare il processo di pensiero dell'utente. Infine, rivela la soluzione e fornisci una breve spiegazione di come l'indovinello può essere risolto utilizzando i suggerimenti forniti. |

## Esempio di Output

Non sono vivo, ma cresco; non ho polmoni, ma ho bisogno di aria; non ho una bocca, ma l'acqua mi uccide. Cosa sono?

Suggerimento 1: Considera i diversi stati della materia e come possono cambiare.  
 Suggerimento 2: Pensa alla relazione tra i tre elementi menzionati: aria, acqua e l'oggetto misterioso.  
 Suggerimento 3: La risposta è correlata a un fenomeno comune che si verifica nella vita quotidiana.

Soluzione: Fuoco.  
 Spiegazione: Il fuoco non è vivo, ma cresce e si diffonde mentre consuma combustibile. Non ha polmoni, ma richiede ossigeno (aria) per sostenersi. Anche se non ha una bocca, l'acqua viene spesso usata per spegnere gli incendi, effettivamente "uccidendoli".

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
  messages=[
    {
      "role": "user",
      "content": [
        {
          "type": "text",
          "text": "Genera un indovinello intelligente e fornisci una guida passo dopo passo per aiutare l'utente ad arrivare alle soluzioni corrette. L'indovinello dovrebbe essere impegnativo ma risolvibile con il pensiero logico e l'attenzione ai dettagli. Dopo aver presentato ogni indovinello, offri una serie di suggerimenti o domande che guidano progressivamente l'utente verso la risposta. Assicurati che i suggerimenti non siano troppo ovvi ma forniscano comunque abbastanza informazioni per guidare il processo di pensiero dell'utente. Infine, rivela la soluzione e fornisci una breve spiegazione di come l'indovinello può essere risolto utilizzando i suggerimenti forniti."
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
  messages: [
    {
      "role": "user",
      "content": [
        {
          "type": "text",
          "text": "Genera un indovinello intelligente e fornisci una guida passo dopo passo per aiutare l'utente ad arrivare alle soluzioni corrette. L'indovinello dovrebbe essere impegnativo ma risolvibile con il pensiero logico e l'attenzione ai dettagli. Dopo aver presentato ogni indovinello, offri una serie di suggerimenti o domande che guidano progressivamente l'utente verso la risposta. Assicurati che i suggerimenti non siano troppo ovvi ma forniscano comunque abbastanza informazioni per guidare il processo di pensiero dell'utente. Infine, rivela la soluzione e fornisci una breve spiegazione di come l'indovinello può essere risolto utilizzando i suggerimenti forniti."
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
    messages=[
        {
            "role": "user",
            "content": [
                {
                    "type": "text",
                    "text": "Genera un indovinello intelligente e fornisci una guida passo dopo passo per aiutare l'utente ad arrivare alle soluzioni corrette. L'indovinello dovrebbe essere impegnativo ma risolvibile con il pensiero logico e l'attenzione ai dettagli. Dopo aver presentato ogni indovinello, offri una serie di suggerimenti o domande che guidano progressivamente l'utente verso la risposta. Assicurati che i suggerimenti non siano troppo ovvi ma forniscano comunque abbastanza informazioni per guidare il processo di pensiero dell'utente. Infine, rivela la soluzione e fornisci una breve spiegazione di come l'indovinello può essere risolto utilizzando i suggerimenti forniti."
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
  messages: [
    {
      "role": "user",
      "content": [
        {
          "type": "text",
          "text": "Genera un indovinello intelligente e fornisci una guida passo dopo passo per aiutare l'utente ad arrivare alle soluzioni corrette. L'indovinello dovrebbe essere impegnativo ma risolvibile con il pensiero logico e l'attenzione ai dettagli. Dopo aver presentato ogni indovinello, offri una serie di suggerimenti o domande che guidano progressivamente l'utente verso la risposta. Assicurati che i suggerimenti non siano troppo ovvi ma forniscano comunque abbastanza informazioni per guidare il processo di pensiero dell'utente. Infine, rivela la soluzione e fornisci una breve spiegazione di come l'indovinello può essere risolto utilizzando i suggerimenti forniti."
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
    messages=[
        {
            "role": "user",
            "content": [
                {
                    "type": "text",
                    "text": "Genera un indovinello intelligente e fornisci una guida passo dopo passo per aiutare l'utente ad arrivare alle soluzioni corrette. L'indovinello dovrebbe essere impegnativo ma risolvibile con il pensiero logico e l'attenzione ai dettagli. Dopo aver presentato ogni indovinello, offri una serie di suggerimenti o domande che guidano progressivamente l'utente verso la risposta. Assicurati che i suggerimenti non siano troppo ovvi ma forniscano comunque abbastanza informazioni per guidare il processo di pensiero dell'utente. Infine, rivela la soluzione e fornisci una breve spiegazione di come l'indovinello può essere risolto utilizzando i suggerimenti forniti."
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
  messages: [
    {
      "role": "user",
      "content": [
        {
          "type": "text",
          "text": "Genera un indovinello intelligente e fornisci una guida passo dopo passo per aiutare l'utente ad arrivare alle soluzioni corrette. L'indovinello dovrebbe essere impegnativo ma risolvibile con il pensiero logico e l'attenzione ai dettagli. Dopo aver presentato ogni indovinello, offri una serie di suggerimenti o domande che guidano progressivamente l'utente verso la risposta. Assicurati che i suggerimenti non siano troppo ovvi ma forniscano comunque abbastanza informazioni per guidare il processo di pensiero dell'utente. Infine, rivela la soluzione e fornisci una breve spiegazione di come l'indovinello può essere risolto utilizzando i suggerimenti forniti."
        }
      ]
    }
  ]
});
console.log(msg);

```

</Tab>
</Tabs>