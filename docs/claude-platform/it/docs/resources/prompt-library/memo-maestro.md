# Memo maestro

Componi memo aziendali completi basati su punti chiave.

---

> Copia questo prompt nella nostra [Console](/dashboard) per sviluppatori per provarlo tu stesso!

|        | Contenuto |
| --- | --- |
| System | Il tuo compito è comporre un memo aziendale completo basato sui punti chiave forniti. Il memo dovrebbe essere scritto in tono professionale, affrontando tutte le informazioni rilevanti in modo chiaro e conciso. Usa la formattazione appropriata, come titoli, sottotitoli e punti elenco, per organizzare il contenuto in modo efficace. Assicurati che il memo sia ben strutturato, coerente e facile da comprendere per il pubblico di destinazione. |
| User   | - Annuncio di un nuovo programma di benessere per i dipendenti: "Fit4Success" <br/> - Obiettivi del programma: promuovere il benessere fisico e mentale, ridurre lo stress e aumentare la produttività <br/> - Componenti: lezioni di fitness in sede, opzioni di pasti salutari, risorse per la salute mentale e workshop sul benessere <br/> - Partnership con palestre locali e centri benessere per abbonamenti scontati <br/> - Incentivi per la partecipazione: fitness tracker, punti benessere e estrazioni di premi <br/> - Data di lancio del programma: 1 giugno 2023 <br/> - Processo di iscrizione: registrazione online tramite intranet aziendale <br/> - Coordinatori del programma: team Risorse Umane e Coinvolgimento Dipendenti |

## Esempio di Output

Oggetto: Presentazione di Fit4Success - Il Nostro Nuovo Programma di Benessere per i Dipendenti

Ciao a tutti,

Sono entusiasta di annunciare il lancio del nostro nuovo programma di benessere per i dipendenti, Fit4Success, progettato per promuovere il benessere fisico e mentale, ridurre lo stress e aumentare la produttività tra i membri del nostro team. Il programma inizierà ufficialmente il 1 giugno 2023.

Obiettivi del Programma:

- Incoraggiare l'attività fisica regolare e scelte di vita salutari
- Fornire risorse e supporto per la salute mentale e la gestione dello stress
- Promuovere un ambiente di lavoro positivo e coinvolgente che dia priorità al benessere dei dipendenti
- Migliorare la produttività complessiva e la soddisfazione lavorativa

Componenti del Programma:

- Lezioni di fitness in sede: sessioni di yoga, Pilates e allenamento a intervalli ad alta intensità (HIIT)
- Opzioni di pasti salutari: snack e pasti nutrienti disponibili nella mensa aziendale
- Risorse per la salute mentale: accesso a servizi di consulenza, app di meditazione e workshop per alleviare lo stress
- Workshop sul benessere: seminari su argomenti come nutrizione, igiene del sonno e equilibrio vita-lavoro

Partnership e Sconti:  
 Per supportare ulteriormente il vostro percorso di benessere, abbiamo stretto partnership con palestre locali e centri benessere per offrire abbonamenti scontati esclusivamente per i nostri dipendenti. Approfittate di queste tariffe speciali per accedere a un'ampia gamma di strutture fitness e lezioni.

Incentivi per la Partecipazione:  
 Per incoraggiare la partecipazione attiva al programma Fit4Success, offriremo i seguenti incentivi:

- Fitness tracker per tutti i partecipanti per monitorare i loro progressi e stabilire obiettivi personali
- Punti benessere guadagnati attraverso la partecipazione al programma, riscattabili per ricompense aggiuntive
- Estrazioni mensili di premi per i partecipanti che raggiungono specifici traguardi di benessere

Processo di Iscrizione:  
 Per iscriversi al programma Fit4Success, visitate la nostra intranet aziendale e completate il modulo di registrazione online. La scadenza per l'iscrizione è il 25 maggio 2023. Se avete domande o dubbi sul processo di iscrizione, contattate il nostro team Risorse Umane.

Coordinamento del Programma:  
 Il programma Fit4Success sarà coordinato congiuntamente dai nostri team Risorse Umane e Coinvolgimento Dipendenti. Saranno disponibili per fornire guida, supporto e risorse durante tutto il vostro percorso di benessere.

Crediamo fermamente che investire nel benessere dei nostri dipendenti sia cruciale per il successo e la crescita della nostra organizzazione. Vi incoraggiamo ad approfittare di questa entusiasmante opportunità e a unirvi a noi nel creare un ambiente di lavoro più sano, più felice e più produttivo.

Abbracciamo il programma Fit4Success e lavoriamo insieme verso un futuro più sano!

Cordiali saluti,  
 \[Il Vostro Nome\]  
 \[Il Vostro Titolo\]

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
    system="Il tuo compito è comporre un memo aziendale completo basato sui punti chiave forniti. Il memo dovrebbe essere scritto in tono professionale, affrontando tutte le informazioni rilevanti in modo chiaro e conciso. Usa la formattazione appropriata, come titoli, sottotitoli e punti elenco, per organizzare il contenuto in modo efficace. Assicurati che il memo sia ben strutturato, coerente e facile da comprendere per il pubblico di destinazione.",
    messages=[
        {
            "role": "user",
            "content": [
                {
                    "type": "text",
                    "text": "- Annuncio di un nuovo programma di benessere per i dipendenti: \"Fit4Success\"  \n- Obiettivi del programma: promuovere il benessere fisico e mentale, ridurre lo stress e aumentare la produttività  \n- Componenti: lezioni di fitness in sede, opzioni di pasti salutari, risorse per la salute mentale e workshop sul benessere  \n- Partnership con palestre locali e centri benessere per abbonamenti scontati  \n- Incentivi per la partecipazione: fitness tracker, punti benessere e estrazioni di premi  \n- Data di lancio del programma: 1 giugno 2023  \n- Processo di iscrizione: registrazione online tramite intranet aziendale  \n- Coordinatori del programma: team Risorse Umane e Coinvolgimento Dipendenti"
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
  system: "Il tuo compito è comporre un memo aziendale completo basato sui punti chiave forniti. Il memo dovrebbe essere scritto in tono professionale, affrontando tutte le informazioni rilevanti in modo chiaro e conciso. Usa la formattazione appropriata, come titoli, sottotitoli e punti elenco, per organizzare il contenuto in modo efficace. Assicurati che il memo sia ben strutturato, coerente e facile da comprendere per il pubblico di destinazione.",
  messages: [
    {
      "role": "user",
      "content": [
        {
          "type": "text",
          "text": "- Annuncio di un nuovo programma di benessere per i dipendenti: \"Fit4Success\"  \n- Obiettivi del programma: promuovere il benessere fisico e mentale, ridurre lo stress e aumentare la produttività  \n- Componenti: lezioni di fitness in sede, opzioni di pasti salutari, risorse per la salute mentale e workshop sul benessere  \n- Partnership con palestre locali e centri benessere per abbonamenti scontati  \n- Incentivi per la partecipazione: fitness tracker, punti benessere e estrazioni di premi  \n- Data di lancio del programma: 1 giugno 2023  \n- Processo di iscrizione: registrazione online tramite intranet aziendale  \n- Coordinatori del programma: team Risorse Umane e Coinvolgimento Dipendenti"
        }
      ]
    }
  ]
});
console.log(msg);

```

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
system="Il tuo compito è comporre un memo aziendale completo basato sui punti chiave forniti. Il memo dovrebbe essere scritto in tono professionale, affrontando tutte le informazioni rilevanti in modo chiaro e conciso. Usa la formattazione appropriata, come titoli, sottotitoli e punti elenco, per organizzare il contenuto in modo efficace. Assicurati che il memo sia ben strutturato, coerente e facile da comprendere per il pubblico di destinazione.",
messages=[
{
"role": "user",
"content": [
{
"type": "text",
"text": "- Annuncio di un nuovo programma di benessere per i dipendenti: \"Fit4Success\" \n- Obiettivi del programma: promuovere il benessere fisico e mentale, ridurre lo stress e aumentare la produttività \n- Componenti: lezioni di fitness in sede, opzioni di pasti salutari, risorse per la salute mentale e workshop sul benessere \n- Partnership con palestre locali e centri benessere per abbonamenti scontati \n- Incentivi per la partecipazione: fitness tracker, punti benessere e estrazioni di premi \n- Data di lancio del programma: 1 giugno 2023 \n- Processo di iscrizione: registrazione online tramite intranet aziendale \n- Coordinatori del programma: team Risorse Umane e Coinvolgimento Dipendenti"
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
  system: "Il tuo compito è comporre un memo aziendale completo basato sui punti chiave forniti. Il memo dovrebbe essere scritto in tono professionale, affrontando tutte le informazioni rilevanti in modo chiaro e conciso. Usa la formattazione appropriata, come titoli, sottotitoli e punti elenco, per organizzare il contenuto in modo efficace. Assicurati che il memo sia ben strutturato, coerente e facile da comprendere per il pubblico di destinazione.",
  messages: [
    {
      "role": "user",
      "content": [
        {
          "type": "text",
          "text": "- Annuncio di un nuovo programma di benessere per i dipendenti: \"Fit4Success\"  \n- Obiettivi del programma: promuovere il benessere fisico e mentale, ridurre lo stress e aumentare la produttività  \n- Componenti: lezioni di fitness in sede, opzioni di pasti salutari, risorse per la salute mentale e workshop sul benessere  \n- Partnership con palestre locali e centri benessere per abbonamenti scontati  \n- Incentivi per la partecipazione: fitness tracker, punti benessere e estrazioni di premi  \n- Data di lancio del programma: 1 giugno 2023  \n- Processo di iscrizione: registrazione online tramite intranet aziendale  \n- Coordinatori del programma: team Risorse Umane e Coinvolgimento Dipendenti"
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
    system="Il tuo compito è comporre un memo aziendale completo basato sui punti chiave forniti. Il memo dovrebbe essere scritto in tono professionale, affrontando tutte le informazioni rilevanti in modo chiaro e conciso. Usa la formattazione appropriata, come titoli, sottotitoli e punti elenco, per organizzare il contenuto in modo efficace. Assicurati che il memo sia ben strutturato, coerente e facile da comprendere per il pubblico di destinazione.",
    messages=[
        {
            "role": "user",
            "content": [
                {
                    "type": "text",
                    "text": "- Annuncio di un nuovo programma di benessere per i dipendenti: \"Fit4Success\"  \n- Obiettivi del programma: promuovere il benessere fisico e mentale, ridurre lo stress e aumentare la produttività  \n- Componenti: lezioni di fitness in sede, opzioni di pasti salutari, risorse per la salute mentale e workshop sul benessere  \n- Partnership con palestre locali e centri benessere per abbonamenti scontati  \n- Incentivi per la partecipazione: fitness tracker, punti benessere e estrazioni di premi  \n- Data di lancio del programma: 1 giugno 2023  \n- Processo di iscrizione: registrazione online tramite intranet aziendale  \n- Coordinatori del programma: team Risorse Umane e Coinvolgimento Dipendenti"
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
  system: "Il tuo compito è comporre un memo aziendale completo basato sui punti chiave forniti. Il memo dovrebbe essere scritto in tono professionale, affrontando tutte le informazioni rilevanti in modo chiaro e conciso. Usa la formattazione appropriata, come titoli, sottotitoli e punti elenco, per organizzare il contenuto in modo efficace. Assicurati che il memo sia ben strutturato, coerente e facile da comprendere per il pubblico di destinazione.",
  messages: [
    {
      "role": "user",
      "content": [
        {
          "type": "text",
          "text": "- Annuncio di un nuovo programma di benessere per i dipendenti: \"Fit4Success\"  \n- Obiettivi del programma: promuovere il benessere fisico e mentale, ridurre lo stress e aumentare la produttività  \n- Componenti: lezioni di fitness in sede, opzioni di pasti salutari, risorse per la salute mentale e workshop sul benessere  \n- Partnership con palestre locali e centri benessere per abbonamenti scontati  \n- Incentivi per la partecipazione: fitness tracker, punti benessere e estrazioni di premi  \n- Data di lancio del programma: 1 giugno 2023  \n- Processo di iscrizione: registrazione online tramite intranet aziendale  \n- Coordinatori del programma: team Risorse Umane e Coinvolgimento Dipendenti"
        }
      ]
    }
  ]
});
console.log(msg);

```

</Tab>
</Tabs>