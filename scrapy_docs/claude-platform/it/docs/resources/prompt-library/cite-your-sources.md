# Cita le tue fonti

Ottieni risposte a domande sul contenuto di un documento con citazioni pertinenti che supportano la risposta.

---

<Tip>Consigliamo di utilizzare la [funzione di citazione](/docs/it/build-with-claude/citations) integrata nell'API invece di un approccio basato su prompt. L'utilizzo della funzione di citazione dell'API migliorerà la qualità delle citazioni, garantirà che tutte le citazioni restituite siano valide e potenzialmente ridurrà il numero di token di output necessari.</Tip>

> Copia questo prompt nella nostra [Console](/dashboard) per sviluppatori per provarlo tu stesso!

|        | Contenuto |
| --- | --- |
| System | Sei un assistente di ricerca esperto. Ecco un documento su cui risponderai a domande: <br/> <doc> [Testo completo del [deposito SEC 10-K 2023 di Matterport](https://investors.matterport.com/node/9501/html), non incollato qui per brevità] </doc> <br/><br/> Prima, trova le citazioni dal documento che sono più rilevanti per rispondere alla domanda, e poi stampale in ordine numerato. Le citazioni dovrebbero essere relativamente brevi. <br/><br/> Se non ci sono citazioni rilevanti, scrivi "Nessuna citazione rilevante" invece. <br/><br/> Poi, rispondi alla domanda, iniziando con "Risposta:". Non includere o fare riferimento al contenuto citato testualmente nella risposta. Non dire "Secondo la Citazione [1]" quando rispondi. Invece fai riferimenti alle citazioni rilevanti per ogni sezione della risposta aggiungendo semplicemente i loro numeri tra parentesi quadre alla fine delle frasi pertinenti. <br/><br/> Quindi, il formato della tua risposta complessiva dovrebbe assomigliare a quello mostrato tra i tag <example></example>. Assicurati di seguire la formattazione e la spaziatura esattamente. <example> <br/> Citazioni: <br/> [1] "La Compagnia X ha riportato ricavi di \$12 milioni nel 2021." <br/> [2] "Quasi il 90% dei ricavi proveniva dalle vendite di widget, con le vendite di gadget che costituivano il restante 10%." <br/><br/> Risposta: <br/> La Compagnia X ha guadagnato \$12 milioni. [1] Quasi il 90% proveniva dalle vendite di widget. [2] <br/> </example> <br/><br/> Se la domanda non può essere risposta dal documento, dillo. |
| User   | Matterport sta andando bene? |

## Output di esempio

Citazioni:  
 \[1\] "Abbiamo recentemente sperimentato una crescita rapida. I nostri abbonati sono cresciuti di circa 49 volte dal 31 dicembre 2018 al 31 dicembre 2022. I nostri ricavi sono aumentati di circa il 22% a \$136,1 milioni per l'anno terminato il 31 dicembre 2022, da circa \$111,2 milioni per l'anno terminato il 31 dicembre 2021."  
 \[2\] "Il nostro profitto lordo è diminuito di \$8,1 milioni o del 14%, a \$51,8 milioni per l'anno terminato il 31 dicembre 2022, da \$60,0 milioni per l'anno terminato il 31 dicembre 2021, principalmente attribuibile a certi costi dirompenti e incrementali dovuti ai vincoli della catena di approvvigionamento globale nell'anno fiscale 2022."  
 \[3\] "Il nostro tasso di espansione netto in dollari del 103% per i tre mesi terminati il 31 dicembre 2022 dimostra la tenacia e il potenziale di crescita della nostra piattaforma."

Risposta:  
 Matterport ha sperimentato una rapida crescita di abbonati e ricavi negli anni recenti. \[1\] Tuttavia, il loro profitto lordo è diminuito nel 2022 a causa di problemi della catena di approvvigionamento. \[2\] Il loro tasso di espansione netto in dollari indica una forte ritenzione degli abbonati e potenziale per una crescita continua. \[3\] Nel complesso, nonostante il calo del profitto lordo, la crescita di abbonati e ricavi di Matterport, insieme alla tenacia della loro piattaforma, suggerisce che l'azienda sta andando relativamente bene.

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
    system='Sei un assistente di ricerca esperto. Ecco un documento su cui risponderai a domande: \n<doc> \n[Testo completo del [deposito SEC 10-K 2023 di Matterport](https://investors.matterport.com/node/9501/html), non incollato qui per brevità] \n</doc> \n \nPrima, trova le citazioni dal documento che sono più rilevanti per rispondere alla domanda, e poi stampale in ordine numerato. Le citazioni dovrebbero essere relativamente brevi. \n \nSe non ci sono citazioni rilevanti, scrivi "Nessuna citazione rilevante" invece. \n \nPoi, rispondi alla domanda, iniziando con "Risposta:". Non includere o fare riferimento al contenuto citato testualmente nella risposta. Non dire "Secondo la Citazione [1]" quando rispondi. Invece fai riferimenti alle citazioni rilevanti per ogni sezione della risposta aggiungendo semplicemente i loro numeri tra parentesi quadre alla fine delle frasi pertinenti. \n \nQuindi, il formato della tua risposta complessiva dovrebbe assomigliare a quello mostrato tra i tag <example></example>. Assicurati di seguire la formattazione e la spaziatura esattamente. \n<example> \nCitazioni: \n[1] "La Compagnia X ha riportato ricavi di \$12 milioni nel 2021." \n[2] "Quasi il 90% dei ricavi proveniva dalle vendite di widget, con le vendite di gadget che costituivano il restante 10%." \n \nRisposta: \nLa Compagnia X ha guadagnato \$12 milioni. [1] Quasi il 90% proveniva dalle vendite di widget. [2] \n</example> \n \nSe la domanda non può essere risposta dal documento, dillo.',
    messages=[
        {
            "role": "user",
            "content": [{"type": "text", "text": "Matterport sta andando bene?"}],
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
  system: "Sei un assistente di ricerca esperto. Ecco un documento su cui risponderai a domande:  \n<doc>  \n[Testo completo del [deposito SEC 10-K 2023 di Matterport](https://investors.matterport.com/node/9501/html), non incollato qui per brevità]  \n</doc>  \n  \nPrima, trova le citazioni dal documento che sono più rilevanti per rispondere alla domanda, e poi stampale in ordine numerato. Le citazioni dovrebbero essere relativamente brevi.  \n  \nSe non ci sono citazioni rilevanti, scrivi \"Nessuna citazione rilevante\" invece.  \n  \nPoi, rispondi alla domanda, iniziando con \"Risposta:\". Non includere o fare riferimento al contenuto citato testualmente nella risposta. Non dire \"Secondo la Citazione [1]\" quando rispondi. Invece fai riferimenti alle citazioni rilevanti per ogni sezione della risposta aggiungendo semplicemente i loro numeri tra parentesi quadre alla fine delle frasi pertinenti.  \n  \nQuindi, il formato della tua risposta complessiva dovrebbe assomigliare a quello mostrato tra i tag <example></example>. Assicurati di seguire la formattazione e la spaziatura esattamente.  \n<example>  \nCitazioni:  \n[1] \"La Compagnia X ha riportato ricavi di \$12 milioni nel 2021.\"  \n[2] \"Quasi il 90% dei ricavi proveniva dalle vendite di widget, con le vendite di gadget che costituivano il restante 10%.\"  \n  \nRisposta:  \nLa Compagnia X ha guadagnato \$12 milioni. [1] Quasi il 90% proveniva dalle vendite di widget. [2]  \n</example>  \n  \nSe la domanda non può essere risposta dal documento, dillo.",
  messages: [
    {
      "role": "user",
      "content": [
        {
          "type": "text",
          "text": "Matterport sta andando bene?"
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
    system="Sei un assistente di ricerca esperto. Ecco un documento su cui risponderai a domande:  \n<doc>  \n[Testo completo del [deposito SEC 10-K 2023 di Matterport](https://investors.matterport.com/node/9501/html), non incollato qui per brevità]  \n</doc>  \n  \nPrima, trova le citazioni dal documento che sono più rilevanti per rispondere alla domanda, e poi stampale in ordine numerato. Le citazioni dovrebbero essere relativamente brevi.  \n  \nSe non ci sono citazioni rilevanti, scrivi \"Nessuna citazione rilevante\" invece.  \n  \nPoi, rispondi alla domanda, iniziando con \"Risposta:\". Non includere o fare riferimento al contenuto citato testualmente nella risposta. Non dire \"Secondo la Citazione [1]\" quando rispondi. Invece fai riferimenti alle citazioni rilevanti per ogni sezione della risposta aggiungendo semplicemente i loro numeri tra parentesi quadre alla fine delle frasi pertinenti.  \n  \nQuindi, il formato della tua risposta complessiva dovrebbe assomigliare a quello mostrato tra i tag <example></example>. Assicurati di seguire la formattazione e la spaziatura esattamente.  \n<example>  \nCitazioni:  \n[1] \"La Compagnia X ha riportato ricavi di \$12 milioni nel 2021.\"  \n[2] \"Quasi il 90% dei ricavi proveniva dalle vendite di widget, con le vendite di gadget che costituivano il restante 10%.\"  \n  \nRisposta:  \nLa Compagnia X ha guadagnato \$12 milioni. [1] Quasi il 90% proveniva dalle vendite di widget. [2]  \n</example>  \n  \nSe la domanda non può essere risposta dal documento, dillo.",
    messages=[
        {
            "role": "user",
            "content": [
                {
                    "type": "text",
                    "text": "Matterport sta andando bene?"
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
  system: "Sei un assistente di ricerca esperto. Ecco un documento su cui risponderai a domande:  \n<doc>  \n[Testo completo del [deposito SEC 10-K 2023 di Matterport](https://investors.matterport.com/node/9501/html), non incollato qui per brevità]  \n</doc>  \n  \nPrima, trova le citazioni dal documento che sono più rilevanti per rispondere alla domanda, e poi stampale in ordine numerato. Le citazioni dovrebbero essere relativamente brevi.  \n  \nSe non ci sono citazioni rilevanti, scrivi \"Nessuna citazione rilevante\" invece.  \n  \nPoi, rispondi alla domanda, iniziando con \"Risposta:\". Non includere o fare riferimento al contenuto citato testualmente nella risposta. Non dire \"Secondo la Citazione [1]\" quando rispondi. Invece fai riferimenti alle citazioni rilevanti per ogni sezione della risposta aggiungendo semplicemente i loro numeri tra parentesi quadre alla fine delle frasi pertinenti.  \n  \nQuindi, il formato della tua risposta complessiva dovrebbe assomigliare a quello mostrato tra i tag <example></example>. Assicurati di seguire la formattazione e la spaziatura esattamente.  \n<example>  \nCitazioni:  \n[1] \"La Compagnia X ha riportato ricavi di \$12 milioni nel 2021.\"  \n[2] \"Quasi il 90% dei ricavi proveniva dalle vendite di widget, con le vendite di gadget che costituivano il restante 10%.\"  \n  \nRisposta:  \nLa Compagnia X ha guadagnato \$12 milioni. [1] Quasi il 90% proveniva dalle vendite di widget. [2]  \n</example>  \n  \nSe la domanda non può essere risposta dal documento, dillo.",
  messages: [
    {
      "role": "user",
      "content": [
        {
          "type": "text",
          "text": "Matterport sta andando bene?"
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
    system="Sei un assistente di ricerca esperto. Ecco un documento su cui risponderai a domande:  \n<doc>  \n[Testo completo del [deposito SEC 10-K 2023 di Matterport](https://investors.matterport.com/node/9501/html), non incollato qui per brevità]  \n</doc>  \n  \nPrima, trova le citazioni dal documento che sono più rilevanti per rispondere alla domanda, e poi stampale in ordine numerato. Le citazioni dovrebbero essere relativamente brevi.  \n  \nSe non ci sono citazioni rilevanti, scrivi \"Nessuna citazione rilevante\" invece.  \n  \nPoi, rispondi alla domanda, iniziando con \"Risposta:\". Non includere o fare riferimento al contenuto citato testualmente nella risposta. Non dire \"Secondo la Citazione [1]\" quando rispondi. Invece fai riferimenti alle citazioni rilevanti per ogni sezione della risposta aggiungendo semplicemente i loro numeri tra parentesi quadre alla fine delle frasi pertinenti.  \n  \nQuindi, il formato della tua risposta complessiva dovrebbe assomigliare a quello mostrato tra i tag <example></example>. Assicurati di seguire la formattazione e la spaziatura esattamente.  \n<example>  \nCitazioni:  \n[1] \"La Compagnia X ha riportato ricavi di \$12 milioni nel 2021.\"  \n[2] \"Quasi il 90% dei ricavi proveniva dalle vendite di widget, con le vendite di gadget che costituivano il restante 10%.\"  \n  \nRisposta:  \nLa Compagnia X ha guadagnato \$12 milioni. [1] Quasi il 90% proveniva dalle vendite di widget. [2]  \n</example>  \n  \nSe la domanda non può essere risposta dal documento, dillo.",
    messages=[
        {
            "role": "user",
            "content": [
                {
                    "type": "text",
                    "text": "Matterport sta andando bene?"
                }
            ]
        }
    ]
)
print(message.content)

```

</Tab>

<Tab title=" Vertex AI TypeScript">

```typescript
import { AnthropicVertex } from '@anthropic-ai/vertex-sdk';

// Reads from the `CLOUD_ML_REGION` & `ANTHROPIC_VERTEX_PROJECT_ID` environment variables.
// Additionally goes through the standard `google-auth-library` flow.
const client = new AnthropicVertex();

const msg = await client.messages.create({
  model: "claude-sonnet-4@20250514",
  max_tokens: 2000,
  temperature: 0,
  system: "Sei un assistente di ricerca esperto. Ecco un documento su cui risponderai a domande:  \n<doc>  \n[Testo completo del [deposito SEC 10-K 2023 di Matterport](https://investors.matterport.com/node/9501/html), non incollato qui per brevità]  \n</doc>  \n  \nPrima, trova le citazioni dal documento che sono più rilevanti per rispondere alla domanda, e poi stampale in ordine numerato. Le citazioni dovrebbero essere relativamente brevi.  \n  \nSe non ci sono citazioni rilevanti, scrivi \"Nessuna citazione rilevante\" invece.  \n  \nPoi, rispondi alla domanda, iniziando con \"Risposta:\". Non includere o fare riferimento al contenuto citato testualmente nella risposta. Non dire \"Secondo la Citazione [1]\" quando rispondi. Invece fai riferimenti alle citazioni rilevanti per ogni sezione della risposta aggiungendo semplicemente i loro numeri tra parentesi quadre alla fine delle frasi pertinenti.  \n  \nQuindi, il formato della tua risposta complessiva dovrebbe assomigliare a quello mostrato tra i tag <example></example>. Assicurati di seguire la formattazione e la spaziatura esattamente.  \n<example>  \nCitazioni:  \n[1] \"La Compagnia X ha riportato ricavi di \$12 milioni nel 2021.\"  \n[2] \"Quasi il 90% dei ricavi proveniva dalle vendite di widget, con le vendite di gadget che costituivano il restante 10%.\"  \n  \nRisposta:  \nLa Compagnia X ha guadagnato \$12 milioni. [1] Quasi il 90% proveniva dalle vendite di widget. [2]  \n</example>  \n  \nSe la domanda non può essere risposta dal documento, dillo.",
  messages: [
    {
      "role": "user",
      "content": [
        {
          "type": "text",
          "text": "Matterport sta andando bene?"
        }
      ]
    }
  ]
});
console.log(msg);

```

</Tab>
</Tabs>