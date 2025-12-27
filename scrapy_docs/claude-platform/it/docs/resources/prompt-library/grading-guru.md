# Guru della valutazione

Confronta e valuta la qualità di testi scritti basandosi su criteri e standard definiti dall'utente.

---

> Copia questo prompt nella nostra [Console](/dashboard) per sviluppatori per provarlo tu stesso!

| Ruolo | Contenuto |
| ---- | --- |
| Utente | Valuta i seguenti due testi basandoti sui criteri forniti: <br/> <br/> Testo 1: <br/> Il sole stava tramontando dietro le montagne, proiettando un caldo bagliore dorato attraverso la valle. Gli uccelli cinguettavano le loro canzoni serali mentre la brezza gentile frusciava tra le foglie. Era un momento pacifico e sereno, uno che riempiva il cuore di gioia e contentezza. <br/><br/> Testo 2: <br/> Il tramonto era carino. Gli uccelli stavano cantando e il vento stava soffiando. Mi ha reso felice. <br/><br/> Criteri: <br/> 1. Linguaggio descrittivo e immaginario <br/> 2. Struttura delle frasi e varietà <br/> 3. Impatto emotivo e coinvolgimento <br/> 4. Grammatica e punteggiatura |

## Esempio di Output

Basandomi sui criteri forniti, ecco la mia valutazione dei due testi:

Testo 1:

1.  Linguaggio descrittivo e immaginario: Il testo utilizza un linguaggio vivido ed evocativo per dipingere un'immagine dell'ambientazione. Frasi come "caldo bagliore dorato," "gli uccelli cinguettavano le loro canzoni serali," e "la brezza gentile frusciava tra le foglie" creano un'immaginario visivo e uditivo forte, immergendo il lettore nella scena.
2.  Struttura delle frasi e varietà: Il testo dimostra un buon mix di strutture di frasi, con sia frasi semplici che composte. Questa varietà aiuta a mantenere un flusso scorrevole e mantiene il lettore coinvolto.
3.  Impatto emotivo e coinvolgimento: Il testo trasmette efficacemente un senso di pace, gioia e contentezza, invitando il lettore a condividere l'esperienza emotiva del momento. La descrizione dell'ambientazione e l'uso di linguaggio positivo creano una forte connessione emotiva.
4.  Grammatica e punteggiatura: Il testo sembra essere grammaticalmente corretto e ben punteggiato, senza errori o problemi evidenti.

Testo 2:

1.  Linguaggio descrittivo e immaginario: Il testo manca di linguaggio descrittivo dettagliato e immaginario. Mentre menziona il tramonto, gli uccelli e il vento, non fornisce dettagli vividi o evocativi per aiutare il lettore a visualizzare la scena.
2.  Struttura delle frasi e varietà: Il testo consiste di frasi brevi e semplici con poca variazione nella struttura. Questa mancanza di varietà può rendere la scrittura monotona e meno coinvolgente.
3.  Impatto emotivo e coinvolgimento: Il testo trasmette un'emozione positiva affermando "Mi ha reso felice," ma non fornisce una forte connessione emotiva o coinvolgimento a causa della mancanza di linguaggio descrittivo e immaginario.
4.  Grammatica e punteggiatura: Il testo sembra essere grammaticalmente corretto e ben punteggiato, senza errori o problemi evidenti.

Analisi Comparativa:  
 Basandomi sulla valutazione, il Testo 1 performa significativamente meglio del Testo 2 in termini di linguaggio descrittivo, immaginario, struttura delle frasi, varietà e impatto emotivo. Il Testo 1 crea un'esperienza vivida e immersiva per il lettore e dimostra una qualità di scrittura superiore basata sui criteri forniti, con il suo forte linguaggio descrittivo, strutture di frasi variate e coinvolgimento emotivo efficace. Il Testo 2, pur essendo grammaticalmente corretto, manca della ricchezza descrittiva e dell'impatto emotivo presente nel Testo 1.

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
    max_tokens=2000,
    temperature=0,
    messages=[
        {
            "role": "user",
            "content": [
                {
                    "type": "text",
                    "text": "Valuta i seguenti due testi basandoti sui criteri forniti: \n \nTesto 1: \nIl sole stava tramontando dietro le montagne, proiettando un caldo bagliore dorato attraverso la valle. Gli uccelli cinguettavano le loro canzoni serali mentre la brezza gentile frusciava tra le foglie. Era un momento pacifico e sereno, uno che riempiva il cuore di gioia e contentezza. \n \nTesto 2: \nIl tramonto era carino. Gli uccelli stavano cantando e il vento stava soffiando. Mi ha reso felice. \n \nCriteri: \n1. Linguaggio descrittivo e immaginario \n2. Struttura delle frasi e varietà \n3. Impatto emotivo e coinvolgimento \n4. Grammatica e punteggiatura",
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
  max_tokens: 2000,
  temperature: 0,
  messages: [
    {
      "role": "user",
      "content": [
        {
          "type": "text",
          "text": "Valuta i seguenti due testi basandoti sui criteri forniti:  \n  \nTesto 1:  \nIl sole stava tramontando dietro le montagne, proiettando un caldo bagliore dorato attraverso la valle. Gli uccelli cinguettavano le loro canzoni serali mentre la brezza gentile frusciava tra le foglie. Era un momento pacifico e sereno, uno che riempiva il cuore di gioia e contentezza.  \n  \nTesto 2:  \nIl tramonto era carino. Gli uccelli stavano cantando e il vento stava soffiando. Mi ha reso felice.  \n  \nCriteri:  \n1. Linguaggio descrittivo e immaginario  \n2. Struttura delle frasi e varietà  \n3. Impatto emotivo e coinvolgimento  \n4. Grammatica e punteggiatura"
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
    temperature=0,
    messages=[
        {
            "role": "user",
            "content": [
                {
                    "type": "text",
                    "text": "Valuta i seguenti due testi basandoti sui criteri forniti:  \n  \nTesto 1:  \nIl sole stava tramontando dietro le montagne, proiettando un caldo bagliore dorato attraverso la valle. Gli uccelli cinguettavano le loro canzoni serali mentre la brezza gentile frusciava tra le foglie. Era un momento pacifico e sereno, uno che riempiva il cuore di gioia e contentezza.  \n  \nTesto 2:  \nIl tramonto era carino. Gli uccelli stavano cantando e il vento stava soffiando. Mi ha reso felice.  \n  \nCriteri:  \n1. Linguaggio descrittivo e immaginario  \n2. Struttura delle frasi e varietà  \n3. Impatto emotivo e coinvolgimento  \n4. Grammatica e punteggiatura"
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
  max_tokens: 2000,
  temperature: 0,
  messages: [
    {
      "role": "user",
      "content": [
        {
          "type": "text",
          "text": "Valuta i seguenti due testi basandoti sui criteri forniti:  \n  \nTesto 1:  \nIl sole stava tramontando dietro le montagne, proiettando un caldo bagliore dorato attraverso la valle. Gli uccelli cinguettavano le loro canzoni serali mentre la brezza gentile frusciava tra le foglie. Era un momento pacifico e sereno, uno che riempiva il cuore di gioia e contentezza.  \n  \nTesto 2:  \nIl tramonto era carino. Gli uccelli stavano cantando e il vento stava soffiando. Mi ha reso felice.  \n  \nCriteri:  \n1. Linguaggio descrittivo e immaginario  \n2. Struttura delle frasi e varietà  \n3. Impatto emotivo e coinvolgimento  \n4. Grammatica e punteggiatura"
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
    max_tokens=2000,
    temperature=0,
    messages=[
        {
            "role": "user",
            "content": [
                {
                    "type": "text",
                    "text": "Valuta i seguenti due testi basandoti sui criteri forniti:  \n  \nTesto 1:  \nIl sole stava tramontando dietro le montagne, proiettando un caldo bagliore dorato attraverso la valle. Gli uccelli cinguettavano le loro canzoni serali mentre la brezza gentile frusciava tra le foglie. Era un momento pacifico e sereno, uno che riempiva il cuore di gioia e contentezza.  \n  \nTesto 2:  \nIl tramonto era carino. Gli uccelli stavano cantando e il vento stava soffiando. Mi ha reso felice.  \n  \nCriteri:  \n1. Linguaggio descrittivo e immaginario  \n2. Struttura delle frasi e varietà  \n3. Impatto emotivo e coinvolgimento  \n4. Grammatica e punteggiatura"
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
  temperature: 0,
  messages: [
    {
      "role": "user",
      "content": [
        {
          "type": "text",
          "text": "Valuta i seguenti due testi basandoti sui criteri forniti:  \n  \nTesto 1:  \nIl sole stava tramontando dietro le montagne, proiettando un caldo bagliore dorato attraverso la valle. Gli uccelli cinguettavano le loro canzoni serali mentre la brezza gentile frusciava tra le foglie. Era un momento pacifico e sereno, uno che riempiva il cuore di gioia e contentezza.  \n  \nTesto 2:  \nIl tramonto era carino. Gli uccelli stavano cantando e il vento stava soffiando. Mi ha reso felice.  \n  \nCriteri:  \n1. Linguaggio descrittivo e immaginario  \n2. Struttura delle frasi e varietà  \n3. Impatto emotivo e coinvolgimento  \n4. Grammatica e punteggiatura"
        }
      ]
    }
  ]
});
console.log(msg);

```

</Tab>
</Tabs>