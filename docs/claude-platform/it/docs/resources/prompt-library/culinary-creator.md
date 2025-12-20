# Creatore culinario

Suggerisci idee per ricette basate sugli ingredienti disponibili dell'utente e le preferenze dietetiche.

---

> Copia questo prompt nella nostra [Console](/dashboard) per sviluppatori per provarlo tu stesso!

|        | Contenuto |
| --- | --- |
| System | Il tuo compito è generare idee per ricette personalizzate basate sull'input dell'utente degli ingredienti disponibili e delle preferenze dietetiche. Usa queste informazioni per suggerire una varietà di ricette creative e deliziose che possono essere preparate usando gli ingredienti forniti mentre si adattano alle esigenze dietetiche dell'utente, se ne vengono menzionate. Per ogni ricetta, fornisci una breve descrizione, un elenco degli ingredienti richiesti e un semplice set di istruzioni. Assicurati che le ricette siano facili da seguire, nutrienti e possano essere preparate con ingredienti o attrezzature aggiuntive minime. |
| User   | Ingredienti disponibili: petto di pollo, broccoli, carote, cipolla, aglio, olio d'oliva, riso, salsa di soia senza glutine, miele <br/> Preferenze dietetiche: senza glutine |

## Output di Esempio

Ricette suggerite:

Saltato di Pollo all'Aglio e Miele

Descrizione: Un saltato veloce e saporito con tenero petto di pollo, verdure croccanti e una salsa dolce e salata all'aglio e miele.  
 Ingredienti:

- 2 petti di pollo, tagliati a pezzetti
- 2 tazze di cimette di broccoli
- 1 tazza di carote a fette
- 1 cipolla, tagliata a dadini
- 3 spicchi d'aglio, tritati
- 2 cucchiai di olio d'oliva
- 1/4 tazza di salsa di soia (senza glutine)
- 2 cucchiai di miele

Istruzioni:

1.  In una grande padella o wok, scalda l'olio d'oliva a fuoco medio-alto.
2.  Aggiungi la cipolla tagliata a dadini e l'aglio tritato, e rosola fino a quando diventa profumato, circa 2 minuti.
3.  Aggiungi i pezzi di pollo e cuoci fino a quando non sono più rosa, circa 5-7 minuti.
4.  Aggiungi i broccoli e le carote, e saltali fino a quando le verdure sono tenere ma croccanti, circa 5 minuti.
5.  In una piccola ciotola, sbatti insieme la salsa di soia e il miele.
6.  Versa la salsa sul pollo e le verdure, e mescola per ricoprire uniformemente.
7.  Cuoci per altri 2-3 minuti, o fino a quando la salsa si è addensata leggermente.
8.  Servi caldo con riso al vapore.

Ciotole di Riso con Pollo e Verdure

Descrizione: Una ciotola di riso sostanziosa e nutriente condita con pollo condito, verdure arrostite e una spruzzata di salsa di soia.

Ingredienti:

- 2 petti di pollo, tagliati a pezzetti
- 2 tazze di cimette di broccoli
- 1 tazza di carote a fette
- 1 cipolla, tagliata a dadini
- 2 spicchi d'aglio, tritati
- 2 cucchiai di olio d'oliva
- 1/4 tazza di salsa di soia (senza glutine)
- 2 tazze di riso cotto

Istruzioni:

1.  Preriscalda il forno a 400°F (200°C).
2.  In una grande ciotola, combina i pezzi di pollo, i broccoli, le carote, la cipolla e l'aglio.
3.  Condisci con olio d'oliva e metà della salsa di soia, e mescola per ricoprire uniformemente.
4.  Distribuisci il pollo e le verdure su una teglia e arrostisci nel forno preriscaldato per 20-25 minuti, o fino a quando il pollo è cotto completamente e le verdure sono tenere.
5.  Dividi il riso cotto tra le ciotole da portata.
6.  Guarnisci ogni ciotola con il pollo e le verdure arrostite.
7.  Spruzza la salsa di soia rimanente sopra ogni ciotola.
8.  Servi caldo e buon appetito!

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
  system="Il tuo compito è generare idee per ricette personalizzate basate sull'input dell'utente degli ingredienti disponibili e delle preferenze dietetiche. Usa queste informazioni per suggerire una varietà di ricette creative e deliziose che possono essere preparate usando gli ingredienti forniti mentre si adattano alle esigenze dietetiche dell'utente, se ne vengono menzionate. Per ogni ricetta, fornisci una breve descrizione, un elenco degli ingredienti richiesti e un semplice set di istruzioni. Assicurati che le ricette siano facili da seguire, nutrienti e possano essere preparate con ingredienti o attrezzature aggiuntive minime.",
  messages=[
    {
    "role": "user",
    "content": [
        {
          "type": "text",
          "text": "Ingredienti disponibili: petto di pollo, broccoli, carote, cipolla, aglio, olio d'oliva, riso, salsa di soia senza glutine, miele \nPreferenze dietetiche: senza glutine"
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
  system: "Il tuo compito è generare idee per ricette personalizzate basate sull'input dell'utente degli ingredienti disponibili e delle preferenze dietetiche. Usa queste informazioni per suggerire una varietà di ricette creative e deliziose che possono essere preparate usando gli ingredienti forniti mentre si adattano alle esigenze dietetiche dell'utente, se ne vengono menzionate. Per ogni ricetta, fornisci una breve descrizione, un elenco degli ingredienti richiesti e un semplice set di istruzioni. Assicurati che le ricette siano facili da seguire, nutrienti e possano essere preparate con ingredienti o attrezzature aggiuntive minime.",
  messages: [
    {
      "role": "user",
      "content": [
        {
          "type": "text",
          "text": "Ingredienti disponibili: petto di pollo, broccoli, carote, cipolla, aglio, olio d'oliva, riso, salsa di soia senza glutine, miele  \nPreferenze dietetiche: senza glutine"
        }
      ]
    }
  ]
});
console.log(msg);

````

</Tab>
<Tab title="AWS Bedrock Python">

```
from anthropic import AnthropicBedrock

# See https://docs.claude.com/claude/reference/claude-on-amazon-bedrock
# for authentication options
client = AnthropicBedrock()

message = client.messages.create(
    model="anthropic.claude-sonnet-4-5-20250929-v1:0",
    max_tokens=2000,
    temperature=0.5,
    system="Il tuo compito è generare idee per ricette personalizzate basate sull'input dell'utente degli ingredienti disponibili e delle preferenze dietetiche. Usa queste informazioni per suggerire una varietà di ricette creative e deliziose che possono essere preparate usando gli ingredienti forniti mentre si adattano alle esigenze dietetiche dell'utente, se ne vengono menzionate. Per ogni ricetta, fornisci una breve descrizione, un elenco degli ingredienti richiesti e un semplice set di istruzioni. Assicurati che le ricette siano facili da seguire, nutrienti e possano essere preparate con ingredienti o attrezzature aggiuntive minime.",
    messages=[
        {
            "role": "user",
            "content": [
                {
                    "type": "text",
                    "text": "Ingredienti disponibili: petto di pollo, broccoli, carote, cipolla, aglio, olio d'oliva, riso, salsa di soia senza glutine, miele  \nPreferenze dietetiche: senza glutine"
                }
            ]
        }
    ]
)
print(message.content)

```

</Tab>
<Tab title="AWS Bedrock TypeScript">

```
import AnthropicBedrock from "@anthropic-ai/bedrock-sdk";

// See https://docs.claude.com/claude/reference/claude-on-amazon-bedrock
// for authentication options
const client = new AnthropicBedrock();

const msg = await client.messages.create({
  model: "anthropic.claude-sonnet-4-5-20250929-v1:0",
  max_tokens: 2000,
  temperature: 0.5,
  system: "Il tuo compito è generare idee per ricette personalizzate basate sull'input dell'utente degli ingredienti disponibili e delle preferenze dietetiche. Usa queste informazioni per suggerire una varietà di ricette creative e deliziose che possono essere preparate usando gli ingredienti forniti mentre si adattano alle esigenze dietetiche dell'utente, se ne vengono menzionate. Per ogni ricetta, fornisci una breve descrizione, un elenco degli ingredienti richiesti e un semplice set di istruzioni. Assicurati che le ricette siano facili da seguire, nutrienti e possano essere preparate con ingredienti o attrezzature aggiuntive minime.",
  messages: [
    {
      "role": "user",
      "content": [
        {
          "type": "text",
          "text": "Ingredienti disponibili: petto di pollo, broccoli, carote, cipolla, aglio, olio d'oliva, riso, salsa di soia senza glutine, miele  \nPreferenze dietetiche: senza glutine"
        }
      ]
    }
  ]
});
console.log(msg);

```

</Tab>
<Tab title="Vertex AI Python">

```
from anthropic import AnthropicVertex

client = AnthropicVertex()

message = client.messages.create(
    model="claude-sonnet-4@20250514",
    max_tokens=2000,
    temperature=0.5,
    system="Il tuo compito è generare idee per ricette personalizzate basate sull'input dell'utente degli ingredienti disponibili e delle preferenze dietetiche. Usa queste informazioni per suggerire una varietà di ricette creative e deliziose che possono essere preparate usando gli ingredienti forniti mentre si adattano alle esigenze dietetiche dell'utente, se ne vengono menzionate. Per ogni ricetta, fornisci una breve descrizione, un elenco degli ingredienti richiesti e un semplice set di istruzioni. Assicurati che le ricette siano facili da seguire, nutrienti e possano essere preparate con ingredienti o attrezzature aggiuntive minime.",
    messages=[
        {
            "role": "user",
            "content": [
                {
                    "type": "text",
                    "text": "Ingredienti disponibili: petto di pollo, broccoli, carote, cipolla, aglio, olio d'oliva, riso, salsa di soia senza glutine, miele  \nPreferenze dietetiche: senza glutine"
                }
            ]
        }
    ]
)
print(message.content)

```

</Tab>
<Tab title="Vertex AI TypeScript">

```
import { AnthropicVertex } from '@anthropic-ai/vertex-sdk';

// Reads from the `CLOUD_ML_REGION` & `ANTHROPIC_VERTEX_PROJECT_ID` environment variables.
// Additionally goes through the standard `google-auth-library` flow.
const client = new AnthropicVertex();

const msg = await client.messages.create({
  model: "claude-sonnet-4@20250514",
  max_tokens: 2000,
  temperature: 0.5,
  system: "Il tuo compito è generare idee per ricette personalizzate basate sull'input dell'utente degli ingredienti disponibili e delle preferenze dietetiche. Usa queste informazioni per suggerire una varietà di ricette creative e deliziose che possono essere preparate usando gli ingredienti forniti mentre si adattano alle esigenze dietetiche dell'utente, se ne vengono menzionate. Per ogni ricetta, fornisci una breve descrizione, un elenco degli ingredienti richiesti e un semplice set di istruzioni. Assicurati che le ricette siano facili da seguire, nutrienti e possano essere preparate con ingredienti o attrezzature aggiuntive minime.",
  messages: [
    {
      "role": "user",
      "content": [
        {
          "type": "text",
          "text": "Ingredienti disponibili: petto di pollo, broccoli, carote, cipolla, aglio, olio d'oliva, riso, salsa di soia senza glutine, miele  \nPreferenze dietetiche: senza glutine"
        }
      ]
    }
  ]
});
console.log(msg);

```

</Tab>
</Tabs>