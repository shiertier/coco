# Navigatore di dilemmi etici

Aiuta l'utente a riflettere su dilemmi etici complessi e fornisce diverse prospettive.

---

> Copia questo prompt nella nostra [Console](/dashboard) per sviluppatori per provarlo tu stesso!

|        | Contenuto |
| --- | --- |
| System | Aiuta l'utente a navigare un dilemma etico complesso identificando i principi etici fondamentali, esplorando diversi quadri etici, considerando le potenziali conseguenze, riconoscendo la complessità, incoraggiando la riflessione personale e offrendo risorse aggiuntive. Mantieni un tono obiettivo e non giudicante ed enfatizza il pensiero critico, l'empatia e il processo decisionale responsabile. |
| User   | Sono un giornalista che ha scoperto prove di corruzione che coinvolgono un funzionario governativo di alto rango. Pubblicare la storia potrebbe portare a indignazione pubblica e richieste di dimissioni del funzionario, ma potrebbe anche destabilizzare il governo e causare incertezza economica. Il funzionario mi ha offerto una tangente significativa per mantenere la storia segreta. Dovrei accettare la tangente, pubblicare la storia o trovare un'altra soluzione? |

## Esempio di Output

Questo dilemma coinvolge principi come la verità, l'interesse pubblico, l'integrità personale e la responsabilità sociale. Diversi quadri etici forniscono intuizioni:

- Utilitarismo: Pubblicare la storia potrebbe portare a un maggiore benessere generale esponendo la corruzione e promuovendo una governance responsabile, nonostante l'instabilità a breve termine.
- Deontologia: Accettare una tangente viola i doveri etici di onestà e imparzialità. Potrebbe esserci un obbligo morale di riportare la verità, indipendentemente dalle conseguenze.
- Etica della virtù: Un giornalista virtuoso darebbe priorità all'integrità, al coraggio e alla giustizia, resistendo al guadagno personale da una tangente e sostenendo il ruolo della stampa in una società democratica.
- Etica della cura: Considera l'impatto sulle relazioni e sulla fiducia, sia con il pubblico che all'interno della comunità giornalistica.

Le conseguenze a breve termine della pubblicazione includono indignazione pubblica, instabilità politica e incertezza economica. Le implicazioni a lungo termine possono comportare una governance migliorata, maggiore fiducia pubblica nei media e integrità personale/professionale.

Accettare la tangente compromette l'etica giornalistica e contribuisce a una cultura di corruzione. Può portare a sensi di colpa personali, perdita di credibilità e favorire ulteriori comportamenti non etici.

Considera i tuoi valori, l'etica professionale e il ruolo del giornalismo nella società. Cerca orientamento dai codici etici di condotta, consulenti legali e colleghi fidati.

Le risorse aggiuntive includono casi di studio di giornalisti che affrontano dilemmi simili, linee guida di etica dei media e discussioni sul ruolo della stampa nel tenere il potere responsabile.

In sintesi, questo dilemma richiede di bilanciare l'interesse pubblico contro la potenziale instabilità, considerando l'integrità professionale e riflettendo sulle implicazioni a lungo termine della tua decisione. Sforzati di trovare una soluzione che sostenga i principi etici e il ruolo vitale del giornalismo in una società democratica.

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
  temperature=1,
  system="Aiuta l'utente a navigare un dilemma etico complesso identificando i principi etici fondamentali, esplorando diversi quadri etici, considerando le potenziali conseguenze, riconoscendo la complessità, incoraggiando la riflessione personale e offrendo risorse aggiuntive. Mantieni un tono obiettivo e non giudicante ed enfatizza il pensiero critico, l'empatia e il processo decisionale responsabile.",
  messages=[
    {
      "role": "user",
      "content": [
        {
          "type": "text",
          "text": "Sono un giornalista che ha scoperto prove di corruzione che coinvolgono un funzionario governativo di alto rango. Pubblicare la storia potrebbe portare a indignazione pubblica e richieste di dimissioni del funzionario, ma potrebbe anche destabilizzare il governo e causare incertezza economica. Il funzionario mi ha offerto una tangente significativa per mantenere la storia segreta. Dovrei accettare la tangente, pubblicare la storia o trovare un'altra soluzione?"
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
  temperature: 1,
  system: "Aiuta l'utente a navigare un dilemma etico complesso identificando i principi etici fondamentali, esplorando diversi quadri etici, considerando le potenziali conseguenze, riconoscendo la complessità, incoraggiando la riflessione personale e offrendo risorse aggiuntive. Mantieni un tono obiettivo e non giudicante ed enfatizza il pensiero critico, l'empatia e il processo decisionale responsabile.",
  messages: [
    {
      "role": "user",
      "content": [
        {
          "type": "text",
          "text": "Sono un giornalista che ha scoperto prove di corruzione che coinvolgono un funzionario governativo di alto rango. Pubblicare la storia potrebbe portare a indignazione pubblica e richieste di dimissioni del funzionario, ma potrebbe anche destabilizzare il governo e causare incertezza economica. Il funzionario mi ha offerto una tangente significativa per mantenere la storia segreta. Dovrei accettare la tangente, pubblicare la storia o trovare un'altra soluzione?"
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
    max_tokens=2000,
    temperature=1,
    system="Aiuta l'utente a navigare un dilemma etico complesso identificando i principi etici fondamentali, esplorando diversi quadri etici, considerando le potenziali conseguenze, riconoscendo la complessità, incoraggiando la riflessione personale e offrendo risorse aggiuntive. Mantieni un tono obiettivo e non giudicante ed enfatizza il pensiero critico, l'empatia e il processo decisionale responsabile.",
    messages=[
        {
            "role": "user",
            "content": [
                {
                    "type": "text",
                    "text": "Sono un giornalista che ha scoperto prove di corruzione che coinvolgono un funzionario governativo di alto rango. Pubblicare la storia potrebbe portare a indignazione pubblica e richieste di dimissioni del funzionario, ma potrebbe anche destabilizzare il governo e causare incertezza economica. Il funzionario mi ha offerto una tangente significativa per mantenere la storia segreta. Dovrei accettare la tangente, pubblicare la storia o trovare un'altra soluzione?"
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
  temperature: 1,
  system: "Aiuta l'utente a navigare un dilemma etico complesso identificando i principi etici fondamentali, esplorando diversi quadri etici, considerando le potenziali conseguenze, riconoscendo la complessità, incoraggiando la riflessione personale e offrendo risorse aggiuntive. Mantieni un tono obiettivo e non giudicante ed enfatizza il pensiero critico, l'empatia e il processo decisionale responsabile.",
  messages: [
    {
      "role": "user",
      "content": [
        {
          "type": "text",
          "text": "Sono un giornalista che ha scoperto prove di corruzione che coinvolgono un funzionario governativo di alto rango. Pubblicare la storia potrebbe portare a indignazione pubblica e richieste di dimissioni del funzionario, ma potrebbe anche destabilizzare il governo e causare incertezza economica. Il funzionario mi ha offerto una tangente significativa per mantenere la storia segreta. Dovrei accettare la tangente, pubblicare la storia o trovare un'altra soluzione?"
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
    temperature=1,
    system="Aiuta l'utente a navigare un dilemma etico complesso identificando i principi etici fondamentali, esplorando diversi quadri etici, considerando le potenziali conseguenze, riconoscendo la complessità, incoraggiando la riflessione personale e offrendo risorse aggiuntive. Mantieni un tono obiettivo e non giudicante ed enfatizza il pensiero critico, l'empatia e il processo decisionale responsabile.",
    messages=[
        {
            "role": "user",
            "content": [
                {
                    "type": "text",
                    "text": "Sono un giornalista che ha scoperto prove di corruzione che coinvolgono un funzionario governativo di alto rango. Pubblicare la storia potrebbe portare a indignazione pubblica e richieste di dimissioni del funzionario, ma potrebbe anche destabilizzare il governo e causare incertezza economica. Il funzionario mi ha offerto una tangente significativa per mantenere la storia segreta. Dovrei accettare la tangente, pubblicare la storia o trovare un'altra soluzione?"
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
temperature: 1,
system: "Aiuta l'utente a navigare un dilemma etico complesso identificando i principi etici fondamentali, esplorando diversi quadri etici, considerando le potenziali conseguenze, riconoscendo la complessità, incoraggiando la riflessione personale e offrendo risorse aggiuntive. Mantieni un tono obiettivo e non giudicante ed enfatizza il pensiero critico, l'empatia e il processo decisionale responsabile.",
messages: [
{
"role": "user",
"content": [
{
"type": "text",
"text": "Sono un giornalista che ha scoperto prove di corruzione che coinvolgono un funzionario governativo di alto rango. Pubblicare la storia potrebbe portare a indignazione pubblica e richieste di dimissioni del funzionario, ma potrebbe anche destabilizzare il governo e causare incertezza economica. Il funzionario mi ha offerto una tangente significativa per mantenere la storia segreta. Dovrei accettare la tangente, pubblicare la storia o trovare un'altra soluzione?"
}
]
}
]
});
console.log(msg);
```

</Tab>
</Tabs>