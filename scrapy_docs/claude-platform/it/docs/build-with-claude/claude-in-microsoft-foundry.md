# Claude in Microsoft Foundry

Accedi ai modelli Claude tramite Microsoft Foundry con endpoint nativi di Azure e autenticazione.

---

Questa guida ti guiderà attraverso il processo di configurazione e di effettuazione di chiamate API a Claude in Foundry in Python, TypeScript, o utilizzando richieste HTTP dirette. Quando potrai accedere a Claude in Foundry, verrai fatturato per l'utilizzo di Claude nel Microsoft Marketplace con il tuo abbonamento Azure, permettendoti di accedere alle ultime capacità di Claude mentre gestisci i costi attraverso il tuo abbonamento Azure.

Disponibilità regionale: Al lancio, Claude è disponibile come tipo di distribuzione Global Standard nelle risorse Foundry con US DataZone in arrivo a breve. I prezzi per Claude nel Microsoft Marketplace utilizzano i prezzi API standard di Anthropic. Visita la nostra [pagina dei prezzi](https://claude.com/pricing#api) per i dettagli.

## Anteprima

In questa integrazione della piattaforma di anteprima, i modelli Claude vengono eseguiti sull'infrastruttura di Anthropic. Si tratta di un'integrazione commerciale per la fatturazione e l'accesso tramite Azure. Come processore indipendente per Microsoft, i clienti che utilizzano Claude tramite Microsoft Foundry sono soggetti ai termini di utilizzo dei dati di Anthropic. Anthropic continua a fornire i suoi impegni di sicurezza e dati leader del settore, inclusa la disponibilità di zero conservazione dei dati.

## Prerequisiti

Prima di iniziare, assicurati di avere:

- Un abbonamento Azure attivo
- Accesso a [Foundry](https://ai.azure.com/)
- [Azure CLI](https://docs.microsoft.com/en-us/cli/azure/install-azure-cli) installato (facoltativo, per la gestione delle risorse)

## Installa un SDK

Gli [SDK client](/docs/it/api/client-sdks) di Anthropic supportano Foundry attraverso pacchetti specifici della piattaforma.

```bash
# Python
pip install -U "anthropic"

# Typescript
npm install @anthropic-ai/foundry-sdk
```

## Provisioning

Foundry utilizza una gerarchia a due livelli: le **risorse** contengono la tua configurazione di sicurezza e fatturazione, mentre le **distribuzioni** sono le istanze del modello che chiami tramite API. Per prima cosa creerai una risorsa Foundry, quindi creerai una o più distribuzioni Claude al suo interno.

### Provisioning delle risorse Foundry

Crea una risorsa Foundry, che è necessaria per utilizzare e gestire i servizi in Azure. Puoi seguire queste istruzioni per creare una [risorsa Foundry](https://learn.microsoft.com/en-us/azure/ai-services/multi-service-resource?pivots=azportal#create-a-new-azure-ai-foundry-resource). In alternativa, puoi iniziare creando un [progetto Foundry](https://learn.microsoft.com/en-us/azure/ai-foundry/how-to/create-projects?tabs=ai-foundry), che comporta la creazione di una risorsa Foundry.

Per eseguire il provisioning della tua risorsa:

1. Accedi al [portale Foundry](https://ai.azure.com/)
2. Crea una nuova risorsa Foundry o selezionane una esistente
3. Configura la gestione dell'accesso utilizzando chiavi API emesse da Azure o Entra ID per il controllo dell'accesso basato sui ruoli
4. Facoltativamente, configura la risorsa per far parte di una rete privata (Azure Virtual Network) per una sicurezza migliorata
5. Annota il nome della tua risorsa: lo utilizzerai come `{resource}` negli endpoint API (ad es., `https://{resource}.services.ai.azure.com/anthropic/v1/*`)

### Creazione di distribuzioni Foundry

Dopo aver creato la tua risorsa, distribuisci un modello Claude per renderlo disponibile per le chiamate API:

1. Nel portale Foundry, accedi alla tua risorsa
2. Vai a **Models + endpoints** e seleziona **+ Deploy model** > **Deploy base model**
3. Cerca e seleziona un modello Claude (ad es., `claude-sonnet-4-5`)
4. Configura le impostazioni di distribuzione:
   - **Deployment name**: Per impostazione predefinita corrisponde all'ID del modello, ma puoi personalizzarlo (ad es., `my-claude-deployment`). Il nome della distribuzione non può essere modificato dopo la creazione.
   - **Deployment type**: Seleziona Global Standard (consigliato per Claude)
5. Seleziona **Deploy** e attendi il completamento del provisioning
6. Una volta distribuito, puoi trovare l'URL dell'endpoint e le chiavi sotto **Keys and Endpoint**

<Note>
  Il nome della distribuzione che scegli diventa il valore che passi nel parametro `model` delle tue richieste API. Puoi creare più distribuzioni dello stesso modello con nomi diversi per gestire configurazioni separate o limiti di velocità.
</Note>

## Autenticazione

Claude su Foundry supporta due metodi di autenticazione: chiavi API e token Entra ID. Entrambi i metodi utilizzano endpoint ospitati su Azure nel formato `https://{resource}.services.ai.azure.com/anthropic/v1/*`.

### Autenticazione con chiave API

Dopo aver eseguito il provisioning della tua risorsa Claude Foundry, puoi ottenere una chiave API dal portale Foundry:

1. Accedi alla tua risorsa nel portale Foundry
2. Vai alla sezione **Keys and Endpoint**
3. Copia una delle chiavi API fornite
4. Utilizza l'intestazione `api-key` o `x-api-key` nelle tue richieste, oppure forniscila all'SDK

Gli SDK Python e TypeScript richiedono una chiave API e un nome di risorsa o un URL di base. Gli SDK leggeranno automaticamente questi valori dalle seguenti variabili di ambiente se definite:

- `ANTHROPIC_FOUNDRY_API_KEY` - La tua chiave API
- `ANTHROPIC_FOUNDRY_RESOURCE` - Il nome della tua risorsa (ad es., `example-resource`)
- `ANTHROPIC_FOUNDRY_BASE_URL` - Alternativa al nome della risorsa; l'URL di base completo (ad es., `https://example-resource.services.ai.azure.com/anthropic/`)

<Note>
I parametri `resource` e `base_url` si escludono a vicenda. Fornisci il nome della risorsa (che l'SDK utilizza per costruire l'URL come `https://{resource}.services.ai.azure.com/anthropic/`) o l'URL di base completo direttamente.
</Note>

**Esempio utilizzando la chiave API:**

<CodeGroup>
```python Python
import os
from anthropic import AnthropicFoundry

client = AnthropicFoundry(
    api_key=os.environ.get("ANTHROPIC_FOUNDRY_API_KEY"),
    resource='example-resource', # your resource name
)

message = client.messages.create(
    model="claude-sonnet-4-5",
    max_tokens=1024,
    messages=[{"role": "user", "content": "Hello!"}]
)
print(message.content)
```

```typescript TypeScript
import AnthropicFoundry from "@anthropic-ai/foundry-sdk";

const client = new AnthropicFoundry({
  apiKey: process.env.ANTHROPIC_FOUNDRY_API_KEY,
  resource: 'example-resource', // your resource name
});

const message = await client.messages.create({
  model: "claude-sonnet-4-5",
  max_tokens: 1024,
  messages: [{ role: "user", content: "Hello!" }],
});
console.log(message.content);
```

```bash Shell
curl https://{resource}.services.ai.azure.com/anthropic/v1/messages \
  -H "content-type: application/json" \
  -H "api-key: YOUR_AZURE_API_KEY" \
  -H "anthropic-version: 2023-06-01" \
  -d '{
    "model": "claude-sonnet-4-5",
    "max_tokens": 1024,
    "messages": [
      {"role": "user", "content": "Hello!"}
    ]
  }'
```
</CodeGroup>

<Warning>
Mantieni le tue chiavi API al sicuro. Non eseguire mai il commit nel controllo di versione o condividerle pubblicamente. Chiunque abbia accesso alla tua chiave API può effettuare richieste a Claude tramite la tua risorsa Foundry.
</Warning>

## Autenticazione Microsoft Entra

Per una sicurezza migliorata e una gestione centralizzata dell'accesso, puoi utilizzare token Entra ID (precedentemente Azure Active Directory):

1. Abilita l'autenticazione Entra per la tua risorsa Foundry
2. Ottieni un token di accesso da Entra ID
3. Utilizza il token nell'intestazione `Authorization: Bearer {TOKEN}`

**Esempio utilizzando Entra ID:**

<CodeGroup>
```python Python
import os
from anthropic import AnthropicFoundry
from azure.identity import DefaultAzureCredential, get_bearer_token_provider

# Get Azure Entra ID token using token provider pattern
token_provider = get_bearer_token_provider(
    DefaultAzureCredential(),
    "https://cognitiveservices.azure.com/.default"
)

# Create client with Entra ID authentication
client = AnthropicFoundry(
    resource='example-resource', # your resource name
    azure_ad_token_provider=token_provider  # Use token provider for Entra ID auth
)

# Make request
message = client.messages.create(
    model="claude-sonnet-4-5",
    max_tokens=1024,
    messages=[{"role": "user", "content": "Hello!"}]
)
print(message.content)
```

```typescript TypeScript
import AnthropicFoundry from "@anthropic-ai/foundry-sdk";
import {
  DefaultAzureCredential,
  getBearerTokenProvider,
} from "@azure/identity";

// Get Entra ID token using token provider pattern
const credential = new DefaultAzureCredential();
const tokenProvider = getBearerTokenProvider(
  credential,
  "https://cognitiveservices.azure.com/.default"
);

// Create client with Entra ID authentication
const client = new AnthropicFoundry({
  resource: 'example-resource', // your resource name
  azureADTokenProvider: tokenProvider, // Use token provider for Entra ID auth
});

// Make request
const message = await client.messages.create({
  model: "claude-sonnet-4-5",
  max_tokens: 1024,
  messages: [{ role: "user", content: "Hello!" }],
});
console.log(message.content);
```

```bash Shell
# Get Azure Entra ID token
ACCESS_TOKEN=$(az account get-access-token --resource https://cognitiveservices.azure.com --query accessToken -o tsv)

# Make request with token. Replace {resource} with your resource name
curl https://{resource}.services.ai.azure.com/anthropic/v1/messages \
  -H "content-type: application/json" \
  -H "Authorization: Bearer $ACCESS_TOKEN" \
  -H "anthropic-version: 2023-06-01" \
  -d '{
    "model": "claude-sonnet-4-5",
    "max_tokens": 1024,
    "messages": [
      {"role": "user", "content": "Hello!"}
    ]
  }'
```
</CodeGroup>

<Note>
L'autenticazione Azure Entra ID ti consente di gestire l'accesso utilizzando Azure RBAC, integrarti con la gestione dell'identità della tua organizzazione ed evitare di gestire manualmente le chiavi API.
</Note>

## ID di richiesta di correlazione

Foundry include identificatori di richiesta negli header di risposta HTTP per il debug e la traccia. Quando contatti il supporto, fornisci sia i valori `request-id` che `apim-request-id` per aiutare i team a localizzare e investigare rapidamente la tua richiesta su entrambi i sistemi Anthropic e Azure.

## Funzionalità supportate

Claude su Foundry supporta la maggior parte delle potenti funzionalità di Claude. Puoi trovare tutte le funzionalità attualmente supportate [qui](/docs/it/build-with-claude/overview).

### Funzionalità non supportate

- Admin API (endpoint `/v1/organizations/*`)
- Models API (`/v1/models`)
- Message Batch API (`/v1/messages/batches`)

## Risposte API

Le risposte API da Claude su Foundry seguono il [formato di risposta API Anthropic](/docs/it/api/messages) standard. Questo include l'oggetto `usage` nei corpi delle risposte, che fornisce informazioni dettagliate sul consumo di token per le tue richieste. L'oggetto `usage` è coerente su tutte le piattaforme (API di prima parte, Foundry, Amazon Bedrock e Google Vertex AI).

Per i dettagli sugli header di risposta specifici di Foundry, consulta la [sezione ID di richiesta di correlazione](#correlation-request-ids).

## ID modello API e distribuzioni

I seguenti modelli Claude sono disponibili tramite Foundry. I modelli della generazione più recente (Sonnet 4.5, Opus 4.1 e Haiku 4.5) offrono le capacità più avanzate:

| Model             | Default Deployment Name     |
| :---------------- | :-------------------------- |
| Claude Opus 4.5   | `claude-opus-4-5`  |
| Claude Sonnet 4.5 | `claude-sonnet-4-5`         |
| Claude Opus 4.1   | `claude-opus-4-1`           |
| Claude Haiku 4.5  | `claude-haiku-4-5`          |

Per impostazione predefinita, i nomi delle distribuzioni corrispondono agli ID dei modelli mostrati sopra. Tuttavia, puoi creare distribuzioni personalizzate con nomi diversi nel portale Foundry per gestire configurazioni, versioni o limiti di velocità diversi. Utilizza il nome della distribuzione (non necessariamente l'ID del modello) nelle tue richieste API.

## Monitoraggio e registrazione

Azure fornisce capacità di monitoraggio e registrazione complete per l'utilizzo di Claude attraverso modelli Azure standard:

- **Azure Monitor**: Traccia l'utilizzo dell'API, la latenza e i tassi di errore
- **Azure Log Analytics**: Interroga e analizza i log di richiesta/risposta
- **Cost Management**: Monitora e prevedi i costi associati all'utilizzo di Claude

Anthropic consiglia di registrare la tua attività su almeno una base mobile di 30 giorni per comprendere i modelli di utilizzo e investigare eventuali problemi.

<Note>
I servizi di registrazione di Azure sono configurati all'interno del tuo abbonamento Azure. L'abilitazione della registrazione non fornisce a Microsoft o Anthropic accesso ai tuoi contenuti oltre a quanto necessario per la fatturazione e il funzionamento del servizio.
</Note>

## Risoluzione dei problemi

### Errori di autenticazione

**Errore**: `401 Unauthorized` o `Invalid API key`

- **Soluzione**: Verifica che la tua chiave API sia corretta. Puoi ottenere una nuova chiave API dal portale Azure sotto **Keys and Endpoint** per la tua risorsa Claude.
- **Soluzione**: Se utilizzi Azure Entra ID, assicurati che il tuo token di accesso sia valido e non sia scaduto. I token in genere scadono dopo 1 ora.

**Errore**: `403 Forbidden`

- **Soluzione**: Il tuo account Azure potrebbe non disporre delle autorizzazioni necessarie. Assicurati di avere il ruolo Azure RBAC appropriato assegnato (ad es., "Cognitive Services OpenAI User").

### Limitazione della velocità

**Errore**: `429 Too Many Requests`

- **Soluzione**: Hai superato il tuo limite di velocità. Implementa la logica di backoff esponenziale e retry nella tua applicazione.
- **Soluzione**: Considera di richiedere aumenti del limite di velocità tramite il portale Azure o il supporto Azure.

#### Header del limite di velocità

Foundry non include gli header del limite di velocità standard di Anthropic (`anthropic-ratelimit-tokens-limit`, `anthropic-ratelimit-tokens-remaining`, `anthropic-ratelimit-tokens-reset`, `anthropic-ratelimit-input-tokens-limit`, `anthropic-ratelimit-input-tokens-remaining`, `anthropic-ratelimit-input-tokens-reset`, `anthropic-ratelimit-output-tokens-limit`, `anthropic-ratelimit-output-tokens-remaining` e `anthropic-ratelimit-output-tokens-reset`) nelle risposte. Gestisci la limitazione della velocità attraverso gli strumenti di monitoraggio di Azure.

### Errori di modello e distribuzione

**Errore**: `Model not found` o `Deployment not found`

- **Soluzione**: Verifica di utilizzare il nome della distribuzione corretto. Se non hai creato una distribuzione personalizzata, utilizza l'ID del modello predefinito (ad es., `claude-sonnet-4-5`).
- **Soluzione**: Assicurati che il modello/distribuzione sia disponibile nella tua regione Azure.

**Errore**: `Invalid model parameter`

- **Soluzione**: Il parametro del modello deve contenere il nome della tua distribuzione, che può essere personalizzato nel portale Foundry. Verifica che la distribuzione esista e sia configurata correttamente.

## Risorse aggiuntive

- **Documentazione Foundry**: [ai.azure.com/catalog](https://ai.azure.com/catalog/publishers/anthropic)
- **Prezzi Azure**: [azure.microsoft.com/en-us/pricing](https://azure.microsoft.com/en-us/pricing/)
- **Dettagli sui prezzi di Anthropic**: [Documentazione sui prezzi](/docs/it/about-claude/pricing#third-party-platform-pricing)
- **Guida all'autenticazione**: Consulta la [sezione autenticazione](#authentication) sopra
- **Portale Azure**: [portal.azure.com](https://portal.azure.com/)