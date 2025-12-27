# Claude su Amazon Bedrock

I modelli Claude di Anthropic sono ora generalmente disponibili tramite Amazon Bedrock.

---

Chiamare Claude tramite Bedrock differisce leggermente da come chiameresti Claude quando usi gli SDK client di Anthropic. Questa guida ti guiderà attraverso il processo di completamento di una chiamata API a Claude su Bedrock in Python o TypeScript.

Nota che questa guida presuppone che tu abbia già registrato un [account AWS](https://portal.aws.amazon.com/billing/signup) e configurato l'accesso programmatico.

## Installare e configurare l'AWS CLI

1. [Installa una versione dell'AWS CLI](https://docs.aws.amazon.com/cli/latest/userguide/cli-chap-welcome.html) alla versione `2.13.23` o successiva
2. Configura le tue credenziali AWS usando il comando AWS configure (vedi [Configurare l'AWS CLI](https://docs.aws.amazon.com/cli/latest/userguide/cli-chap-configure.html)) oppure trova le tue credenziali navigando a "Command line or programmatic access" all'interno della tua dashboard AWS e seguendo le indicazioni nel modal popup.
3. Verifica che le tue credenziali funzionino:

```bash Shell
aws sts get-caller-identity
```

## Installare un SDK per accedere a Bedrock

Gli [SDK client](/docs/it/api/client-sdks) di Anthropic supportano Bedrock. Puoi anche usare direttamente un SDK AWS come `boto3`.

<CodeGroup>
  ```python Python
  pip install -U "anthropic[bedrock]"
  ```

  ```typescript TypeScript
  npm install @anthropic-ai/bedrock-sdk
  ```

  ```python Boto3 (Python)
  pip install boto3>=1.28.59
  ```
</CodeGroup>

## Accesso a Bedrock

### Sottoscrivere i modelli Anthropic

Vai a [AWS Console > Bedrock > Model Access](https://console.aws.amazon.com/bedrock/home?region=us-west-2#/modelaccess) e richiedi l'accesso ai modelli Anthropic. Nota che la disponibilità dei modelli Anthropic varia per regione. Vedi la [documentazione AWS](https://docs.aws.amazon.com/bedrock/latest/userguide/models-regions.html) per le informazioni più recenti.

#### ID modello API

| Modello | ID modello Bedrock base | `global` | `us` | `eu` | `jp` | `apac` |
| :---- | :---- | :---- | :---- | :---- | :---- | :---- |
| Claude Sonnet 4.5 | anthropic.claude-sonnet-4-5-20250929-v1:0 | Sì | Sì | Sì | Sì | No |
| Claude Sonnet 4 | anthropic.claude-sonnet-4-20250514-v1:0 | Sì | Sì | Sì | No | Sì |
| Claude Sonnet 3.7 <Tooltip tooltipContent="Deprecato a partire dal 28 ottobre 2025.">⚠️</Tooltip> | anthropic.claude-3-7-sonnet-20250219-v1:0 | No | Sì | Sì | No | Sì |
| Claude Opus 4.5 | anthropic.claude-opus-4-5-20251101-v1:0 | Sì | Sì | Sì | No | No |
| Claude Opus 4.1 | anthropic.claude-opus-4-1-20250805-v1:0 | No | Sì | No | No | No |
| Claude Opus 4 | anthropic.claude-opus-4-20250514-v1:0 | No | Sì | No | No | No |
| Claude Opus 3 <Tooltip tooltipContent="Deprecato a partire dal 30 giugno 2025.">⚠️</Tooltip> | anthropic.claude-3-opus-20240229-v1:0 | No | Sì | No | No | No |
| Claude Haiku 4.5 | anthropic.claude-haiku-4-5-20251001-v1:0 | Sì | Sì | Sì | No | No |
| Claude Haiku 3.5 <Tooltip tooltipContent="Deprecato a partire dal 19 dicembre 2025.">⚠️</Tooltip> | anthropic.claude-3-5-haiku-20241022-v1:0 | No | Sì | No | No | No |
| Claude Haiku 3 | anthropic.claude-3-haiku-20240307-v1:0 | No | Sì | Sì | No | Sì |

Per ulteriori informazioni sugli ID modello regionali rispetto a quelli globali, vedi la sezione [Endpoint globali vs regionali](#global-vs-regional-endpoints) di seguito.

### Elencare i modelli disponibili

I seguenti esempi mostrano come stampare un elenco di tutti i modelli Claude disponibili tramite Bedrock:

<CodeGroup>
  ```bash AWS CLI
  aws bedrock list-foundation-models --region=us-west-2 --by-provider anthropic --query "modelSummaries[*].modelId"
  ```

  ```python Boto3 (Python)
  import boto3

  bedrock = boto3.client(service_name="bedrock")
  response = bedrock.list_foundation_models(byProvider="anthropic")

  for summary in response["modelSummaries"]:
      print(summary["modelId"])
  ```
</CodeGroup>

### Effettuare richieste

I seguenti esempi mostrano come generare testo da Claude su Bedrock:

<CodeGroup>
  ```python Python
  from anthropic import AnthropicBedrock

  client = AnthropicBedrock(
      # Autentica fornendo le chiavi di seguito oppure usa i provider di credenziali AWS predefiniti, come
      # usando ~/.aws/credentials o le variabili di ambiente "AWS_SECRET_ACCESS_KEY" e "AWS_ACCESS_KEY_ID".
      aws_access_key="<access key>",
      aws_secret_key="<secret key>",
      # Le credenziali temporanee possono essere utilizzate con aws_session_token.
      # Leggi di più su https://docs.aws.amazon.com/IAM/latest/UserGuide/id_credentials_temp.html.
      aws_session_token="<session_token>",
      # aws_region cambia la regione aws a cui viene effettuata la richiesta. Per impostazione predefinita, leggiamo AWS_REGION,
      # e se non è presente, usiamo per impostazione predefinita us-east-1. Nota che non leggiamo ~/.aws/config per la regione.
      aws_region="us-west-2",
  )

  message = client.messages.create(
      model="global.anthropic.claude-sonnet-4-5-20250929-v1:0",
      max_tokens=256,
      messages=[{"role": "user", "content": "Hello, world"}]
  )
  print(message.content)
  ```

  ```typescript TypeScript
  import AnthropicBedrock from '@anthropic-ai/bedrock-sdk';

  const client = new AnthropicBedrock({
    // Autentica fornendo le chiavi di seguito oppure usa i provider di credenziali AWS predefiniti, come
    // usando ~/.aws/credentials o le variabili di ambiente "AWS_SECRET_ACCESS_KEY" e "AWS_ACCESS_KEY_ID".
    awsAccessKey: '<access key>',
    awsSecretKey: '<secret key>',

    // Le credenziali temporanee possono essere utilizzate con awsSessionToken.
    // Leggi di più su https://docs.aws.amazon.com/IAM/latest/UserGuide/id_credentials_temp.html.
    awsSessionToken: '<session_token>',

    // awsRegion cambia la regione aws a cui viene effettuata la richiesta. Per impostazione predefinita, leggiamo AWS_REGION,
    // e se non è presente, usiamo per impostazione predefinita us-east-1. Nota che non leggiamo ~/.aws/config per la regione.
    awsRegion: 'us-west-2',
  });

  async function main() {
    const message = await client.messages.create({
      model: 'global.anthropic.claude-sonnet-4-5-20250929-v1:0',
      max_tokens: 256,
      messages: [{"role": "user", "content": "Hello, world"}]
    });
    console.log(message);
  }
  main().catch(console.error);
  ```

  ```python Boto3 (Python)
  import boto3
  import json

  bedrock = boto3.client(service_name="bedrock-runtime")
  body = json.dumps({
    "max_tokens": 256,
    "messages": [{"role": "user", "content": "Hello, world"}],
    "anthropic_version": "bedrock-2023-05-31"
  })

  response = bedrock.invoke_model(body=body, modelId="global.anthropic.claude-sonnet-4-5-20250929-v1:0")

  response_body = json.loads(response.get("body").read())
  print(response_body.get("content"))
  ```
</CodeGroup>

Vedi i nostri [SDK client](/docs/it/api/client-sdks) per ulteriori dettagli, e la documentazione ufficiale di Bedrock [qui](https://docs.aws.amazon.com/bedrock/).

## Registrazione dell'attività

Bedrock fornisce un [servizio di registrazione dell'invocazione](https://docs.aws.amazon.com/bedrock/latest/userguide/model-invocation-logging.html) che consente ai clienti di registrare i prompt e i completamenti associati al tuo utilizzo.

Anthropic consiglia di registrare la tua attività su almeno una base mobile di 30 giorni per comprendere la tua attività e investigare qualsiasi potenziale abuso.

<Note>
L'attivazione di questo servizio non dà ad AWS o Anthropic alcun accesso ai tuoi contenuti.
</Note>

## Supporto delle funzionalità

Puoi trovare tutte le funzionalità attualmente supportate su Bedrock [qui](/docs/it/api/overview).

### Supporto PDF su Bedrock

Il supporto PDF è disponibile su Amazon Bedrock tramite sia l'API Converse che l'API InvokeModel. Per informazioni dettagliate sulle capacità e limitazioni dell'elaborazione PDF, vedi la [documentazione del supporto PDF](/docs/it/build-with-claude/pdf-support#amazon-bedrock-pdf-support).

**Considerazioni importanti per gli utenti dell'API Converse:**
- L'analisi visiva dei PDF (grafici, immagini, layout) richiede l'abilitazione delle citazioni
- Senza citazioni, è disponibile solo l'estrazione di testo di base
- Per il controllo completo senza citazioni forzate, usa l'API InvokeModel

Per ulteriori dettagli sulle due modalità di elaborazione dei documenti e le loro limitazioni, fai riferimento alla [guida del supporto PDF](/docs/it/build-with-claude/pdf-support#amazon-bedrock-pdf-support).

### Finestra di contesto da 1M token

Claude Sonnet 4 e 4.5 supportano la [finestra di contesto da 1M token](/docs/it/build-with-claude/context-windows#1m-token-context-window) su Amazon Bedrock.

<Note>
La finestra di contesto da 1M token è attualmente in beta. Per utilizzare la finestra di contesto estesa, includi l'header beta `context-1m-2025-08-07` nelle tue [richieste API Bedrock](https://docs.aws.amazon.com/bedrock/latest/userguide/model-parameters-anthropic-claude-messages-request-response.html).
</Note>

## Endpoint globali vs regionali

A partire da **Claude Sonnet 4.5 e tutti i modelli futuri**, Amazon Bedrock offre due tipi di endpoint:

- **Endpoint globali**: Instradamento dinamico per la massima disponibilità
- **Endpoint regionali**: Instradamento dati garantito attraverso regioni geografiche specifiche

Gli endpoint regionali includono un premio di prezzo del 10% rispetto agli endpoint globali.

<Note>
Questo si applica solo a Claude Sonnet 4.5 e ai modelli futuri. I modelli più vecchi (Claude Sonnet 4, Opus 4 e precedenti) mantengono le loro strutture di prezzo esistenti.
</Note>

### Quando usare ogni opzione

**Endpoint globali (consigliato):**
- Forniscono la massima disponibilità e tempo di attività
- Instradano dinamicamente le richieste alle regioni con capacità disponibile
- Nessun premio di prezzo
- Migliore per le applicazioni in cui la residenza dei dati è flessibile

**Endpoint regionali (CRIS):**
- Instradano il traffico attraverso regioni geografiche specifiche
- Richiesti per i requisiti di residenza dei dati e conformità
- Disponibili per Stati Uniti, UE, Giappone e Australia
- Il premio di prezzo del 10% riflette i costi dell'infrastruttura per la capacità regionale dedicata

### Implementazione

**Utilizzo di endpoint globali (predefinito per Sonnet 4.5 e 4):**

Gli ID modello per Claude Sonnet 4.5 e 4 includono già il prefisso `global.`:

<CodeGroup>
```python Python
from anthropic import AnthropicBedrock

client = AnthropicBedrock(aws_region="us-west-2")

message = client.messages.create(
    model="global.anthropic.claude-sonnet-4-5-20250929-v1:0",
    max_tokens=256,
    messages=[{"role": "user", "content": "Hello, world"}]
)
```

```typescript TypeScript
import AnthropicBedrock from '@anthropic-ai/bedrock-sdk';

const client = new AnthropicBedrock({
  awsRegion: 'us-west-2',
});

const message = await client.messages.create({
  model: 'global.anthropic.claude-sonnet-4-5-20250929-v1:0',
  max_tokens: 256,
  messages: [{role: "user", content: "Hello, world"}]
});
```
</CodeGroup>

**Utilizzo di endpoint regionali (CRIS):**

Per usare gli endpoint regionali, rimuovi il prefisso `global.` dall'ID modello:

<CodeGroup>
```python Python
from anthropic import AnthropicBedrock

client = AnthropicBedrock(aws_region="us-west-2")

# Utilizzo dell'endpoint regionale US (CRIS)
message = client.messages.create(
    model="anthropic.claude-sonnet-4-5-20250929-v1:0",  # Nessun prefisso global.
    max_tokens=256,
    messages=[{"role": "user", "content": "Hello, world"}]
)
```

```typescript TypeScript
import AnthropicBedrock from '@anthropic-ai/bedrock-sdk';

const client = new AnthropicBedrock({
  awsRegion: 'us-west-2',
});

// Utilizzo dell'endpoint regionale US (CRIS)
const message = await client.messages.create({
  model: 'anthropic.claude-sonnet-4-5-20250929-v1:0',  // Nessun prefisso global.
  max_tokens: 256,
  messages: [{role: "user", content: "Hello, world"}]
});
```
</CodeGroup>

### Risorse aggiuntive

- **Prezzi AWS Bedrock:** [aws.amazon.com/bedrock/pricing](https://aws.amazon.com/bedrock/pricing/)
- **Documentazione dei prezzi AWS:** [Guida ai prezzi di Bedrock](https://docs.aws.amazon.com/bedrock/latest/userguide/bedrock-pricing.html)
- **Post del blog AWS:** [Introduzione a Claude Sonnet 4.5 in Amazon Bedrock](https://aws.amazon.com/blogs/aws/introducing-claude-sonnet-4-5-in-amazon-bedrock-anthropics-most-intelligent-model-best-for-coding-and-complex-agents/)
- **Dettagli sui prezzi di Anthropic:** [Documentazione sui prezzi](/docs/it/about-claude/pricing#third-party-platform-pricing)