# Generatore di trivia

Genera domande di trivia su una vasta gamma di argomenti e fornisce suggerimenti quando necessario.

---

> Copia questo prompt nella nostra [Console](/dashboard) per sviluppatori per provarlo tu stesso!

| Ruolo | Contenuto |
| ---- | --- |
| Utente | Genera domande di trivia su vari argomenti e fornisci suggerimenti per aiutare gli utenti ad arrivare alla risposta corretta. Seleziona da un insieme diversificato di categorie e crea domande che testino le conoscenze o le capacità di ragionamento dell'utente. Offri una serie di suggerimenti sempre più specifici per guidare gli utenti verso la soluzione. Assicurati che le domande siano impegnative e che i suggerimenti forniscano informazioni sufficienti per aiutare l'utente senza rivelare la risposta troppo facilmente. |

## Output di Esempio

Categoria: Geografia

Domanda:  
 Qual è l'unico paese al mondo che confina sia con la Francia che con la Spagna?

Suggerimento 1: Questo paese si trova sui monti Pirenei tra Francia e Spagna.  
 Suggerimento 2: È noto per i suoi paesaggi montani mozzafiato, le stazioni sciistiche e lo shopping duty-free.  
 Suggerimento 3: La capitale del paese presenta il nome del paese stesso.

Risposta: Andorra

Categoria: Scienza

Domanda:  
 Qual è il processo attraverso il quale le piante convertono la luce solare, l'acqua e l'anidride carbonica in energia e ossigeno?

Suggerimento 1: Questo processo è essenziale per la vita sulla Terra, poiché fornisce sia cibo che ossigeno.  
 Suggerimento 2: Avviene principalmente nelle foglie delle piante.  
 Suggerimento 3: Il processo coinvolge la clorofilla, un pigmento verde che assorbe la luce solare.

Risposta: Fotosintesi

Categoria: Letteratura

Domanda:  
 Chi ha scritto il romanzo classico "Orgoglio e Pregiudizio", pubblicato per la prima volta nel 1813?

Suggerimento 1: L'autrice è una donna britannica che visse dal 1775 al 1817.  
 Suggerimento 2: È nota per i suoi ritratti arguti e perspicaci della società e dei costumi inglesi.  
 Suggerimento 3: Le sue altre opere famose includono "Senno e Sensibilità" ed "Emma".

Risposta: Jane Austen

Categoria: Storia

Domanda:  
 In che anno iniziò ad essere abbattuto il Muro di Berlino, che divideva la Germania Est e Ovest?

Suggerimento 1: L'evento si verificò alla fine del XX secolo.  
 Suggerimento 2: Segnò un momento significativo nella fine della Guerra Fredda.  
 Suggerimento 3: L'anno è spesso associato alla frase "caduta del comunismo" in Europa.

Risposta: 1989

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
  max_tokens=2000,
  temperature=0.5,
  messages=[
    {
    "role": "user",
    "content": [
        {
          "type": "text",
          "text": "Genera domande di trivia su vari argomenti e fornisci suggerimenti per aiutare gli utenti ad arrivare alla risposta corretta. Seleziona da un insieme diversificato di categorie e crea domande che testino le conoscenze o le capacità di ragionamento dell'utente. Offri una serie di suggerimenti sempre più specifici per guidare gli utenti verso la soluzione. Assicurati che le domande siano impegnative e che i suggerimenti forniscano informazioni sufficienti per aiutare l'utente senza rivelare la risposta troppo facilmente."
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
  max_tokens: 2000,
  temperature: 0.5,
  messages: [
    {
      "role": "user",
      "content": [
        {
          "type": "text",
          "text": "Genera domande di trivia su vari argomenti e fornisci suggerimenti per aiutare gli utenti ad arrivare alla risposta corretta. Seleziona da un insieme diversificato di categorie e crea domande che testino le conoscenze o le capacità di ragionamento dell'utente. Offri una serie di suggerimenti sempre più specifici per guidare gli utenti verso la soluzione. Assicurati che le domande siano impegnative e che i suggerimenti forniscano informazioni sufficienti per aiutare l'utente senza rivelare la risposta troppo facilmente."
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
max_tokens=2000,
temperature=0.5,
messages=[
{
"role": "user",
"content": [
{
"type": "text",
"text": "Genera domande di trivia su vari argomenti e fornisci suggerimenti per aiutare gli utenti ad arrivare alla risposta corretta. Seleziona da un insieme diversificato di categorie e crea domande che testino le conoscenze o le capacità di ragionamento dell'utente. Offri una serie di suggerimenti sempre più specifici per guidare gli utenti verso la soluzione. Assicurati che le domande siano impegnative e che i suggerimenti forniscano informazioni sufficienti per aiutare l'utente senza rivelare la risposta troppo facilmente."
}
]
}
]
)
print(message.content)

````
</Tab>
<Tab title="AWS Bedrock TypeScript">

```typescript
import AnthropicBedrock from "@anthropic-ai/bedrock-sdk";

// See https://docs.claude.com/claude/reference/claude-on-amazon-bedrock
// for authentication options
const client = new AnthropicBedrock();

const msg = await client.messages.create({
  model: "anthropic.claude-sonnet-4-5-20250929-v1:0",
  max_tokens: 2000,
  temperature: 0.5,
  messages: [
    {
      "role": "user",
      "content": [
        {
          "type": "text",
          "text": "Genera domande di trivia su vari argomenti e fornisci suggerimenti per aiutare gli utenti ad arrivare alla risposta corretta. Seleziona da un insieme diversificato di categorie e crea domande che testino le conoscenze o le capacità di ragionamento dell'utente. Offri una serie di suggerimenti sempre più specifici per guidare gli utenti verso la soluzione. Assicurati che le domande siano impegnative e che i suggerimenti forniscano informazioni sufficienti per aiutare l'utente senza rivelare la risposta troppo facilmente."
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
    max_tokens=2000,
    temperature=0.5,
    messages=[
        {
            "role": "user",
            "content": [
                {
                    "type": "text",
                    "text": "Genera domande di trivia su vari argomenti e fornisci suggerimenti per aiutare gli utenti ad arrivare alla risposta corretta. Seleziona da un insieme diversificato di categorie e crea domande che testino le conoscenze o le capacità di ragionamento dell'utente. Offri una serie di suggerimenti sempre più specifici per guidare gli utenti verso la soluzione. Assicurati che le domande siano impegnative e che i suggerimenti forniscano informazioni sufficienti per aiutare l'utente senza rivelare la risposta troppo facilmente."
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
  max_tokens: 2000,
  temperature: 0.5,
  messages: [
    {
      "role": "user",
      "content": [
        {
          "type": "text",
          "text": "Genera domande di trivia su vari argomenti e fornisci suggerimenti per aiutare gli utenti ad arrivare alla risposta corretta. Seleziona da un insieme diversificato di categorie e crea domande che testino le conoscenze o le capacità di ragionamento dell'utente. Offri una serie di suggerimenti sempre più specifici per guidare gli utenti verso la soluzione. Assicurati che le domande siano impegnative e che i suggerimenti forniscano informazioni sufficienti per aiutare l'utente senza rivelare la risposta troppo facilmente."
        }
      ]
    }
  ]
});
console.log(msg);

```

</Tab>
</Tabs>