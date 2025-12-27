# Chiaroveggente aziendale

Estrai intuizioni, identifica rischi e distilla informazioni chiave da lunghi report aziendali in un singolo memo.

---

> Copia questo prompt nella nostra [Console](/dashboard) per sviluppatori per provarlo tu stesso!

|      | Contenuto |
| ---- | --- |
| Utente | Il tuo compito è analizzare il seguente report: <br /> <br /> <report> [Testo completo del [deposito SEC 10-K 2023 di Matterport](https://investors.matterport.com/node/9501/html), non incollato qui per brevità] </report> <br /> <br /> Riassumi questo report annuale in modo conciso e chiaro, e identifica le tendenze di mercato chiave e i punti salienti. Presenta i tuoi risultati come un breve memo che posso inviare al mio team. L'obiettivo del memo è assicurarsi che il mio team rimanga aggiornato su come stanno andando le istituzioni finanziarie e prevedere qualitativamente e identificare se ci sono rischi operativi e di ricavi da aspettarsi nel prossimo trimestre. Assicurati di includere tutti i dettagli rilevanti nel tuo riassunto e analisi. |

## Output di Esempio

> A: Team di Investimento  
> Da: \[Il Tuo Nome\]  
> Data: 4 marzo 2024  
> Oggetto: Punti Salienti del 10-K 2022 di Matterport e Prospettive Finanziarie
>
> Ciao team,
>
> Ho identificato alcuni punti chiave dal report annuale 2022 di Matterport:
>
> Performance Finanziaria:
>
> - I ricavi sono aumentati del 22% a \$136,1M nel 2022, guidati dalla crescita dei ricavi da abbonamenti e servizi
> - Tuttavia, il margine lordo è diminuito dal 54% al 38% a causa dell'aumento dei costi dai ricavi dei prodotti e delle sfide della catena di approvvigionamento
> - Le perdite operative si sono ampliate a \$275,5M mentre l'azienda ha investito pesantemente in R&S e vendite e marketing per guidare la crescita
> - Ha chiuso il 2022 con \$476,9M in contanti e investimenti per finanziare le operazioni
>
> Punti Salienti del Business:
>
> - La base di abbonati è cresciuta del 39% a oltre 701.000 alla fine del 2022. Gli spazi sotto gestione hanno raggiunto 9,2 milioni
> - Ha espanso la piattaforma con il lancio del supporto motorizzato Matterport Axis, della camera Pro3 e dei principali aggiornamenti al software cloud
> - Ha completato le acquisizioni di Enview (AI per dati spaziali 3D) e VHT Studios (marketing immobiliare) per migliorare le offerte
> - Ha firmato partnership notevoli con Burns & McDonnell e CGS per fornire le soluzioni di Matterport ai loro clienti enterprise
>
> Prospettive:
>
> - Matterport vede un'opportunità di mercato massiccia nella digitalizzazione degli edifici e spazi del mondo (stima TAM di \$240B+)
> - Concentrata sulla promozione dell'adozione con grandi clienti enterprise attraverso settori verticali chiave come immobiliare, AEC, gestione delle strutture e altro
> - Continuerà a investire in R&S ed espansione internazionale per penetrare il mercato globale degli edifici largamente inesplorato
> - La crescita e i margini a breve termine potrebbero essere impattati da venti contrari macroeconomici e problemi della catena di approvvigionamento
> - Tuttavia, la posizione di liquidità sana e la base clienti relativamente fedele forniscono protezione al ribasso
>
> In sintesi, Matterport ha consegnato una forte crescita dei ricavi nel 2022 mentre ha fatto progressi nella penetrazione di un enorme mercato globale. Mentre la redditività rimane sfidante mentre l'azienda investe pesantemente per la crescita, il suo bilancio sembra solido. A parte alcuni problemi continui della catena di approvvigionamento, lo slancio del business sottostante appare intatto basato sulle metriche operative chiave. Salvo una grave recessione economica, Matterport sembra ben posizionata per continuare a guadagnare quote nello spazio nascente della digitalizzazione degli edifici.

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
    temperature=0,
    messages=[
        {
            "role": "user",
            "content": [
                {
                    "type": "text",
                    "text": "Il tuo compito è analizzare il seguente report:  \n<report>  \n[Testo completo del [deposito SEC 10-K 2023 di Matterport](https://investors.matterport.com/node/9501/html), non incollato qui per brevità]  \n</report>  \n  \nRiassumi questo report annuale in modo conciso e chiaro, e identifica le tendenze di mercato chiave e i punti salienti. Presenta i tuoi risultati come un breve memo che posso inviare al mio team. L'obiettivo del memo è assicurarsi che il mio team rimanga aggiornato su come stanno andando le istituzioni finanziarie e prevedere qualitativamente e identificare se ci sono rischi operativi e di ricavi da aspettarsi nel prossimo trimestre. Assicurati di includere tutti i dettagli rilevanti nel tuo riassunto e analisi."
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
  temperature: 0,
  messages: [
    {
      "role": "user",
      "content": [
        {
          "type": "text",
          "text": "Il tuo compito è analizzare il seguente report:  \n<report>  \n[Testo completo del [deposito SEC 10-K 2023 di Matterport](https://investors.matterport.com/node/9501/html), non incollato qui per brevità]  \n</report>  \n  \nRiassumi questo report annuale in modo conciso e chiaro, e identifica le tendenze di mercato chiave e i punti salienti. Presenta i tuoi risultati come un breve memo che posso inviare al mio team. L'obiettivo del memo è assicurarsi che il mio team rimanga aggiornato su come stanno andando le istituzioni finanziarie e prevedere qualitativamente e identificare se ci sono rischi operativi e di ricavi da aspettarsi nel prossimo trimestre. Assicurati di includere tutti i dettagli rilevanti nel tuo riassunto e analisi."
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

# Vedi https://docs.claude.com/claude/reference/claude-on-amazon-bedrock

# per le opzioni di autenticazione

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
"text": "Il tuo compito è analizzare il seguente report: \n<report> \n[Testo completo del [deposito SEC 10-K 2023 di Matterport](https://investors.matterport.com/node/9501/html), non incollato qui per brevità] \n</report> \n \nRiassumi questo report annuale in modo conciso e chiaro, e identifica le tendenze di mercato chiave e i punti salienti. Presenta i tuoi risultati come un breve memo che posso inviare al mio team. L'obiettivo del memo è assicurarsi che il mio team rimanga aggiornato su come stanno andando le istituzioni finanziarie e prevedere qualitativamente e identificare se ci sono rischi operativi e di ricavi da aspettarsi nel prossimo trimestre. Assicurati di includere tutti i dettagli rilevanti nel tuo riassunto e analisi."
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

// Vedi https://docs.claude.com/claude/reference/claude-on-amazon-bedrock
// per le opzioni di autenticazione
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
          "text": "Il tuo compito è analizzare il seguente report:  \n<report>  \n[Testo completo del [deposito SEC 10-K 2023 di Matterport](https://investors.matterport.com/node/9501/html), non incollato qui per brevità]  \n</report>  \n  \nRiassumi questo report annuale in modo conciso e chiaro, e identifica le tendenze di mercato chiave e i punti salienti. Presenta i tuoi risultati come un breve memo che posso inviare al mio team. L'obiettivo del memo è assicurarsi che il mio team rimanga aggiornato su come stanno andando le istituzioni finanziarie e prevedere qualitativamente e identificare se ci sono rischi operativi e di ricavi da aspettarsi nel prossimo trimestre. Assicurati di includere tutti i dettagli rilevanti nel tuo riassunto e analisi."
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
    temperature=0,
    messages=[
        {
            "role": "user",
            "content": [
                {
                    "type": "text",
                    "text": "Il tuo compito è analizzare il seguente report:  \n<report>  \n[Testo completo del [deposito SEC 10-K 2023 di Matterport](https://investors.matterport.com/node/9501/html), non incollato qui per brevità]  \n</report>  \n  \nRiassumi questo report annuale in modo conciso e chiaro, e identifica le tendenze di mercato chiave e i punti salienti. Presenta i tuoi risultati come un breve memo che posso inviare al mio team. L'obiettivo del memo è assicurarsi che il mio team rimanga aggiornato su come stanno andando le istituzioni finanziarie e prevedere qualitativamente e identificare se ci sono rischi operativi e di ricavi da aspettarsi nel prossimo trimestre. Assicurati di includere tutti i dettagli rilevanti nel tuo riassunto e analisi."
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

// Legge dalle variabili d'ambiente `CLOUD_ML_REGION` & `ANTHROPIC_VERTEX_PROJECT_ID`.
// Inoltre passa attraverso il flusso standard `google-auth-library`.
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
          "text": "Il tuo compito è analizzare il seguente report:  \n<report>  \n[Testo completo del [deposito SEC 10-K 2023 di Matterport](https://investors.matterport.com/node/9501/html), non incollato qui per brevità]  \n</report>  \n  \nRiassumi questo report annuale in modo conciso e chiaro, e identifica le tendenze di mercato chiave e i punti salienti. Presenta i tuoi risultati come un breve memo che posso inviare al mio team. L'obiettivo del memo è assicurarsi che il mio team rimanga aggiornato su come stanno andando le istituzioni finanziarie e prevedere qualitativamente e identificare se ci sono rischi operativi e di ricavi da aspettarsi nel prossimo trimestre. Assicurati di includere tutti i dettagli rilevanti nel tuo riassunto e analisi."
        }
      ]
    }
  ]
});
console.log(msg);

```

  </Tab>
</Tabs>