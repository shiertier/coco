# Claude su Vertex AI

I modelli Claude di Anthropic sono ora generalmente disponibili tramite [Vertex AI](https://cloud.google.com/vertex-ai).

---

L'API Vertex per accedere a Claude è quasi identica all'[API Messages](/docs/it/api/messages) e supporta tutte le stesse opzioni, con due differenze chiave:

* In Vertex, `model` non viene passato nel corpo della richiesta. Invece, viene specificato nell'URL dell'endpoint di Google Cloud.
* In Vertex, `anthropic_version` viene passato nel corpo della richiesta (piuttosto che come intestazione) e deve essere impostato al valore `vertex-2023-10-16`.

Vertex è supportato anche dagli [SDK client](/docs/it/api/client-sdks) ufficiali di Anthropic. Questa guida ti guiderà attraverso il processo di effettuare una richiesta a Claude su Vertex AI in Python o TypeScript.

Nota che questa guida presuppone che tu abbia già un progetto GCP in grado di utilizzare Vertex AI. Vedi [utilizzo dei modelli Claude 3 di Anthropic](https://cloud.google.com/vertex-ai/generative-ai/docs/partner-models/use-claude) per ulteriori informazioni sulla configurazione richiesta, nonché una procedura dettagliata completa.

## Installa un SDK per accedere a Vertex AI

Per prima cosa, installa l'[SDK client](/docs/it/api/client-sdks) di Anthropic per il linguaggio di tua scelta.

<CodeGroup>
  ```python Python
  pip install -U google-cloud-aiplatform "anthropic[vertex]"
  ```

  ```typescript TypeScript
  npm install @anthropic-ai/vertex-sdk
  ```
</CodeGroup>

## Accesso a Vertex AI

### Disponibilità del modello

Nota che la disponibilità dei modelli Anthropic varia in base alla regione. Cerca "Claude" in [Vertex AI Model Garden](https://cloud.google.com/model-garden) o vai a [Usa Claude 3](https://cloud.google.com/vertex-ai/generative-ai/docs/partner-models/use-claude) per le informazioni più recenti.

#### ID modello API

| Modello                          | ID modello API Vertex AI |
| ------------------------------ | ------------------------ |
| Claude Sonnet 4.5              | claude-sonnet-4-5@20250929 |
| Claude Sonnet 4                | claude-sonnet-4@20250514 |
| Claude Sonnet 3.7 <Tooltip tooltipContent="Deprecato a partire dal 28 ottobre 2025.">⚠️</Tooltip> | claude-3-7-sonnet@20250219 |
| Claude Opus 4.5                | claude-opus-4-5@20251101 |
| Claude Opus 4.1                | claude-opus-4-1@20250805 |
| Claude Opus 4                  | claude-opus-4@20250514   |
| Claude Opus 3 <Tooltip tooltipContent="Deprecato a partire dal 30 giugno 2025.">⚠️</Tooltip> | claude-3-opus@20240229   |
| Claude Haiku 4.5               | claude-haiku-4-5@20251001 |
| Claude Haiku 3.5 <Tooltip tooltipContent="Deprecato a partire dal 19 dicembre 2025.">⚠️</Tooltip> | claude-3-5-haiku@20241022 |
| Claude Haiku 3                 | claude-3-haiku@20240307  |

### Effettuare richieste

Prima di eseguire le richieste, potrebbe essere necessario eseguire `gcloud auth application-default login` per autenticarsi con GCP.

I seguenti esempi mostrano come generare testo da Claude su Vertex AI:
<CodeGroup>

  ```python Python
  from anthropic import AnthropicVertex

  project_id = "MY_PROJECT_ID"
  region = "global"

  client = AnthropicVertex(project_id=project_id, region=region)

  message = client.messages.create(
      model="claude-sonnet-4-5@20250929",
      max_tokens=100,
      messages=[
          {
              "role": "user",
              "content": "Hey Claude!",
          }
      ],
  )
  print(message)
  ```

  ```typescript TypeScript
  import { AnthropicVertex } from '@anthropic-ai/vertex-sdk';

  const projectId = 'MY_PROJECT_ID';
  const region = 'global';

  // Goes through the standard `google-auth-library` flow.
  const client = new AnthropicVertex({
    projectId,
    region,
  });

  async function main() {
    const result = await client.messages.create({
      model: 'claude-sonnet-4-5@20250929',
      max_tokens: 100,
      messages: [
        {
          role: 'user',
          content: 'Hey Claude!',
        },
      ],
    });
    console.log(JSON.stringify(result, null, 2));
  }

  main();
  ```

  ```bash Shell
  MODEL_ID=claude-sonnet-4-5@20250929
  LOCATION=global
  PROJECT_ID=MY_PROJECT_ID

  curl \
  -X POST \
  -H "Authorization: Bearer $(gcloud auth print-access-token)" \
  -H "Content-Type: application/json" \
  https://$LOCATION-aiplatform.googleapis.com/v1/projects/${PROJECT_ID}/locations/${LOCATION}/publishers/anthropic/models/${MODEL_ID}:streamRawPredict -d \
  '{
    "anthropic_version": "vertex-2023-10-16",
    "messages": [{
      "role": "user",
      "content": "Hey Claude!"
    }],
    "max_tokens": 100,
  }'
  ```
</CodeGroup>

Vedi i nostri [SDK client](/docs/it/api/client-sdks) e la [documentazione ufficiale di Vertex AI](https://cloud.google.com/vertex-ai/docs) per ulteriori dettagli.

## Registrazione dell'attività

Vertex fornisce un [servizio di registrazione delle richieste-risposte](https://cloud.google.com/vertex-ai/generative-ai/docs/multimodal/request-response-logging) che consente ai clienti di registrare i prompt e i completamenti associati al tuo utilizzo.

Anthropic consiglia di registrare la tua attività almeno su base mobile di 30 giorni per comprendere la tua attività e indagare su eventuali usi impropri.

<Note>
L'attivazione di questo servizio non dà a Google o Anthropic alcun accesso ai tuoi contenuti.
</Note>

## Supporto delle funzionalità
Puoi trovare tutte le funzionalità attualmente supportate su Vertex [qui](/docs/it/api/overview).

## Endpoint globali e regionali

A partire da **Claude Sonnet 4.5 e tutti i modelli futuri**, Google Vertex AI offre due tipi di endpoint:

- **Endpoint globali**: Instradamento dinamico per la massima disponibilità
- **Endpoint regionali**: Instradamento dati garantito attraverso regioni geografiche specifiche

Gli endpoint regionali includono un premio di prezzo del 10% rispetto agli endpoint globali.

<Note>
Questo si applica solo a Claude Sonnet 4.5 e ai modelli futuri. I modelli più vecchi (Claude Sonnet 4, Opus 4 e versioni precedenti) mantengono le loro strutture di prezzo esistenti.
</Note>

### Quando utilizzare ogni opzione

**Endpoint globali (consigliato):**
- Forniscono la massima disponibilità e tempo di attività
- Instradano dinamicamente le richieste alle regioni con capacità disponibile
- Nessun premio di prezzo
- Ideale per applicazioni in cui la residenza dei dati è flessibile
- Supporta solo il traffico pay-as-you-go (la velocità effettiva fornita richiede endpoint regionali)

**Endpoint regionali:**
- Instradano il traffico attraverso regioni geografiche specifiche
- Richiesti per i requisiti di residenza dei dati e conformità
- Supportano sia il traffico pay-as-you-go che la velocità effettiva fornita
- Il premio di prezzo del 10% riflette i costi dell'infrastruttura per la capacità regionale dedicata

### Implementazione

**Utilizzo di endpoint globali (consigliato):**

Imposta il parametro `region` su `"global"` quando inizializzi il client:

<CodeGroup>
```python Python
from anthropic import AnthropicVertex

project_id = "MY_PROJECT_ID"
region = "global"

client = AnthropicVertex(project_id=project_id, region=region)

message = client.messages.create(
    model="claude-sonnet-4-5@20250929",
    max_tokens=100,
    messages=[
        {
            "role": "user",
            "content": "Hey Claude!",
        }
    ],
)
print(message)
```

```typescript TypeScript
import { AnthropicVertex } from '@anthropic-ai/vertex-sdk';

const projectId = 'MY_PROJECT_ID';
const region = 'global';

const client = new AnthropicVertex({
  projectId,
  region,
});

const result = await client.messages.create({
  model: 'claude-sonnet-4-5@20250929',
  max_tokens: 100,
  messages: [
    {
      role: 'user',
      content: 'Hey Claude!',
    },
  ],
});
```
</CodeGroup>

**Utilizzo di endpoint regionali:**

Specifica una regione specifica come `"us-east1"` o `"europe-west1"`:

<CodeGroup>
```python Python
from anthropic import AnthropicVertex

project_id = "MY_PROJECT_ID"
region = "us-east1"  # Specify a specific region

client = AnthropicVertex(project_id=project_id, region=region)

message = client.messages.create(
    model="claude-sonnet-4-5@20250929",
    max_tokens=100,
    messages=[
        {
            "role": "user",
            "content": "Hey Claude!",
        }
    ],
)
print(message)
```

```typescript TypeScript
import { AnthropicVertex } from '@anthropic-ai/vertex-sdk';

const projectId = 'MY_PROJECT_ID';
const region = 'us-east1';  // Specify a specific region

const client = new AnthropicVertex({
  projectId,
  region,
});

const result = await client.messages.create({
  model: 'claude-sonnet-4-5@20250929',
  max_tokens: 100,
  messages: [
    {
      role: 'user',
      content: 'Hey Claude!',
    },
  ],
});
```
</CodeGroup>

### Risorse aggiuntive

- **Prezzi di Google Vertex AI:** [cloud.google.com/vertex-ai/generative-ai/pricing](https://cloud.google.com/vertex-ai/generative-ai/pricing)
- **Documentazione dei modelli Claude:** [Claude su Vertex AI](https://cloud.google.com/vertex-ai/generative-ai/docs/partner-models/claude)
- **Post del blog di Google:** [Endpoint globale per i modelli Claude](https://cloud.google.com/blog/products/ai-machine-learning/global-endpoint-for-claude-models-generally-available-on-vertex-ai)
- **Dettagli sui prezzi di Anthropic:** [Documentazione sui prezzi](/docs/it/about-claude/pricing#third-party-platform-pricing)