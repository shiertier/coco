# Innovatore del fitness VR

Genera idee creative per giochi di fitness in realtà virtuale.

---

> Copia questo prompt nella nostra [Console](/dashboard) per sviluppatori per provarlo tu stesso!

| Ruolo | Contenuto |
| ---- | --- |
| Utente | Il tuo compito è generare una lista di idee innovative e coinvolgenti per giochi di fitness in realtà virtuale (VR). Considera vari generi di giochi, meccaniche di gioco uniche, ambienti immersivi e modi creativi per incorporare esercizi fisici e movimenti. Le idee dovrebbero essere attraenti per una vasta gamma di appassionati di fitness e gamer, incoraggiandoli a rimanere attivi e divertirsi mentre si esercitano in VR. Per ogni idea, fornisci una breve descrizione del concetto di gioco, caratteristiche chiave e come promuove il fitness. |

## Output di Esempio

1.  Idea: Avventura Parkour VR  
    Descrizione: I giocatori navigano attraverso un ambiente urbano dinamico, correndo, saltando, arrampicandosi ed eseguendo acrobazie di parkour per superare ostacoli e raggiungere obiettivi. Il gioco si adatta ai movimenti del giocatore, fornendo un intenso allenamento per tutto il corpo mentre esplora la città virtuale.  
    Caratteristiche chiave: Meccaniche di parkour realistiche, ambienti generati proceduralmente, gare multiplayer e sfide.  
    Aspetti fitness: Cardio, agilità, coordinazione e forza della parte inferiore del corpo.
2.  Idea: Boxe Ritmica  
    Descrizione: Un gioco di boxe ad alta energia ambientato al ritmo di brani musicali popolari. I giocatori devono colpire, schivare e bloccare in sincronia con il ritmo per segnare punti e progredire attraverso livelli sempre più impegnativi. Il gioco offre una varietà di stili di boxe e modalità di allenamento.  
    Caratteristiche chiave: Musica con licenza, pugili personalizzabili, multiplayer online e una gamma diversificata di tecniche di boxe.  
    Aspetti fitness: Cardio, forza della parte superiore del corpo, riflessi e resistenza.
3.  Idea: RPG Fitness VR  
    Descrizione: Un gioco di ruolo immersivo dove i giocatori creano il proprio personaggio e intraprendono una missione per salvare un mondo fantasy. Il gioco combina elementi RPG tradizionali con sfide di fitness, richiedendo ai giocatori di eseguire esercizi fisici per lanciare incantesimi, sconfiggere nemici e far salire di livello il proprio personaggio.  
    Caratteristiche chiave: Personalizzazione del personaggio, alberi delle abilità, battaglie epiche contro boss e un mix di esercizi di forza, cardio e flessibilità.  
    Aspetti fitness: Allenamenti per tutto il corpo, allenamento della forza, cardio e flessibilità.

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
          "text": "Il tuo compito è generare una lista di idee innovative e coinvolgenti per giochi di fitness in realtà virtuale (VR). Considera vari generi di giochi, meccaniche di gioco uniche, ambienti immersivi e modi creativi per incorporare esercizi fisici e movimenti. Le idee dovrebbero essere attraenti per una vasta gamma di appassionati di fitness e gamer, incoraggiandoli a rimanere attivi e divertirsi mentre si esercitano in VR. Per ogni idea, fornisci una breve descrizione del concetto di gioco, caratteristiche chiave e come promuove il fitness."
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
          "text": "Il tuo compito è generare una lista di idee innovative e coinvolgenti per giochi di fitness in realtà virtuale (VR). Considera vari generi di giochi, meccaniche di gioco uniche, ambienti immersivi e modi creativi per incorporare esercizi fisici e movimenti. Le idee dovrebbero essere attraenti per una vasta gamma di appassionati di fitness e gamer, incoraggiandoli a rimanere attivi e divertirsi mentre si esercitano in VR. Per ogni idea, fornisci una breve descrizione del concetto di gioco, caratteristiche chiave e come promuove il fitness."
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
                    "text": "Il tuo compito è generare una lista di idee innovative e coinvolgenti per giochi di fitness in realtà virtuale (VR). Considera vari generi di giochi, meccaniche di gioco uniche, ambienti immersivi e modi creativi per incorporare esercizi fisici e movimenti. Le idee dovrebbero essere attraenti per una vasta gamma di appassionati di fitness e gamer, incoraggiandoli a rimanere attivi e divertirsi mentre si esercitano in VR. Per ogni idea, fornisci una breve descrizione del concetto di gioco, caratteristiche chiave e come promuove il fitness."
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
          "text": "Il tuo compito è generare una lista di idee innovative e coinvolgenti per giochi di fitness in realtà virtuale (VR). Considera vari generi di giochi, meccaniche di gioco uniche, ambienti immersivi e modi creativi per incorporare esercizi fisici e movimenti. Le idee dovrebbero essere attraenti per una vasta gamma di appassionati di fitness e gamer, incoraggiandoli a rimanere attivi e divertirsi mentre si esercitano in VR. Per ogni idea, fornisci una breve descrizione del concetto di gioco, caratteristiche chiave e come promuove il fitness."
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
                    "text": "Il tuo compito è generare una lista di idee innovative e coinvolgenti per giochi di fitness in realtà virtuale (VR). Considera vari generi di giochi, meccaniche di gioco uniche, ambienti immersivi e modi creativi per incorporare esercizi fisici e movimenti. Le idee dovrebbero essere attraenti per una vasta gamma di appassionati di fitness e gamer, incoraggiandoli a rimanere attivi e divertirsi mentre si esercitano in VR. Per ogni idea, fornisci una breve descrizione del concetto di gioco, caratteristiche chiave e come promuove il fitness."
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
          "text": "Il tuo compito è generare una lista di idee innovative e coinvolgenti per giochi di fitness in realtà virtuale (VR). Considera vari generi di giochi, meccaniche di gioco uniche, ambienti immersivi e modi creativi per incorporare esercizi fisici e movimenti. Le idee dovrebbero essere attraenti per una vasta gamma di appassionati di fitness e gamer, incoraggiandoli a rimanere attivi e divertirsi mentre si esercitano in VR. Per ogni idea, fornisci una breve descrizione del concetto di gioco, caratteristiche chiave e come promuove il fitness."
        }
      ]
    }
  ]
});
console.log(msg);

```

</Tab>
</Tabs>