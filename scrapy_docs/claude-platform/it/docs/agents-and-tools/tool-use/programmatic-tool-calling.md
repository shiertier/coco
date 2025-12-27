# Invocazione programmatica di strumenti

Consenti a Claude di scrivere codice che chiama i tuoi strumenti in modo programmatico all'interno di un contenitore di esecuzione del codice

---

L'invocazione programmatica di strumenti consente a Claude di scrivere codice che chiama i tuoi strumenti in modo programmatico all'interno di un [contenitore di esecuzione del codice](/docs/it/agents-and-tools/tool-use/code-execution-tool), piuttosto che richiedere round trip attraverso il modello per ogni invocazione di strumento. Questo riduce la latenza per i flussi di lavoro multi-strumento e diminuisce il consumo di token consentendo a Claude di filtrare o elaborare i dati prima che raggiungano la finestra di contesto del modello.

<Note>
L'invocazione programmatica di strumenti è attualmente in beta pubblica.

Per utilizzare questa funzione, aggiungi l'[intestazione beta](/docs/it/api/beta-headers) `"advanced-tool-use-2025-11-20"` alle tue richieste API.

Questa funzione richiede che lo strumento di esecuzione del codice sia abilitato.
</Note>

## Compatibilità del modello

L'invocazione programmatica di strumenti è disponibile sui seguenti modelli:

| Modello | Versione dello strumento |
|-------|--------------|
| Claude Opus 4.5 (`claude-opus-4-5-20251101`) | `code_execution_20250825` |
| Claude Sonnet 4.5 (`claude-sonnet-4-5-20250929`) | `code_execution_20250825` |

<Warning>
L'invocazione programmatica di strumenti è disponibile tramite l'API Claude e Microsoft Foundry.
</Warning>

## Avvio rapido

Ecco un semplice esempio in cui Claude interroga programmaticamente un database più volte e aggrega i risultati:

<CodeGroup>
```bash Shell
curl https://api.anthropic.com/v1/messages \
    --header "x-api-key: $ANTHROPIC_API_KEY" \
    --header "anthropic-version: 2023-06-01" \
    --header "anthropic-beta: advanced-tool-use-2025-11-20" \
    --header "content-type: application/json" \
    --data '{
        "model": "claude-sonnet-4-5",
        "max_tokens": 4096,
        "messages": [
            {
                "role": "user",
                "content": "Query sales data for the West, East, and Central regions, then tell me which region had the highest revenue"
            }
        ],
        "tools": [
            {
                "type": "code_execution_20250825",
                "name": "code_execution"
            },
            {
                "name": "query_database",
                "description": "Execute a SQL query against the sales database. Returns a list of rows as JSON objects.",
                "input_schema": {
                    "type": "object",
                    "properties": {
                        "sql": {
                            "type": "string",
                            "description": "SQL query to execute"
                        }
                    },
                    "required": ["sql"]
                },
                "allowed_callers": ["code_execution_20250825"]
            }
        ]
    }'
```

```python Python
import anthropic

client = anthropic.Anthropic()

response = client.beta.messages.create(
    model="claude-sonnet-4-5",
    betas=["advanced-tool-use-2025-11-20"],
    max_tokens=4096,
    messages=[{
        "role": "user",
        "content": "Query sales data for the West, East, and Central regions, then tell me which region had the highest revenue"
    }],
    tools=[
        {
            "type": "code_execution_20250825",
            "name": "code_execution"
        },
        {
            "name": "query_database",
            "description": "Execute a SQL query against the sales database. Returns a list of rows as JSON objects.",
            "input_schema": {
                "type": "object",
                "properties": {
                    "sql": {
                        "type": "string",
                        "description": "SQL query to execute"
                    }
                },
                "required": ["sql"]
            },
            "allowed_callers": ["code_execution_20250825"]
        }
    ]
)

print(response)
```

```typescript TypeScript
import { Anthropic } from '@anthropic-ai/sdk';

const anthropic = new Anthropic();

async function main() {
  const response = await anthropic.beta.messages.create({
    model: "claude-sonnet-4-5",
    betas: ["advanced-tool-use-2025-11-20"],
    max_tokens: 4096,
    messages: [
      {
        role: "user",
        content: "Query sales data for the West, East, and Central regions, then tell me which region had the highest revenue"
      }
    ],
    tools: [
      {
        type: "code_execution_20250825",
        name: "code_execution"
      },
      {
        name: "query_database",
        description: "Execute a SQL query against the sales database. Returns a list of rows as JSON objects.",
        input_schema: {
          type: "object",
          properties: {
            sql: {
              type: "string",
              description: "SQL query to execute"
            }
          },
          required: ["sql"]
        },
        allowed_callers: ["code_execution_20250825"]
      }
    ]
  });

  console.log(response);
}

main().catch(console.error);
```
</CodeGroup>

## Come funziona l'invocazione programmatica di strumenti

Quando configuri uno strumento per essere richiamabile dall'esecuzione del codice e Claude decide di utilizzare quello strumento:

1. Claude scrive codice Python che invoca lo strumento come una funzione, potenzialmente includendo più chiamate di strumenti e logica di pre/post-elaborazione
2. Claude esegue questo codice in un contenitore sandbox tramite l'esecuzione del codice
3. Quando viene chiamata una funzione dello strumento, l'esecuzione del codice si interrompe e l'API restituisce un blocco `tool_use`
4. Fornisci il risultato dello strumento e l'esecuzione del codice continua (i risultati intermedi non vengono caricati nella finestra di contesto di Claude)
5. Una volta completata l'esecuzione del codice, Claude riceve l'output finale e continua a lavorare sul compito

Questo approccio è particolarmente utile per:
- **Elaborazione di grandi dati**: Filtra o aggrega i risultati degli strumenti prima che raggiungano il contesto di Claude
- **Flussi di lavoro multi-step**: Risparmia token e latenza chiamando gli strumenti in serie o in un ciclo senza campionare Claude tra le chiamate degli strumenti
- **Logica condizionale**: Prendi decisioni in base ai risultati intermedi degli strumenti

<Note>
Gli strumenti personalizzati vengono convertiti in funzioni Python asincrone per supportare le chiamate parallele degli strumenti. Quando Claude scrive codice che chiama i tuoi strumenti, utilizza `await` (ad es., `result = await query_database("<sql>")`) e include automaticamente la funzione wrapper asincrona appropriata.

Il wrapper asincrono è omesso dagli esempi di codice in questa documentazione per chiarezza.
</Note>

## Concetti fondamentali

### Il campo `allowed_callers`

Il campo `allowed_callers` specifica quali contesti possono invocare uno strumento:

```json
{
  "name": "query_database",
  "description": "Execute a SQL query against the database",
  "input_schema": {...},
  "allowed_callers": ["code_execution_20250825"]
}
```

**Valori possibili:**
- `["direct"]` - Solo Claude può chiamare questo strumento direttamente (predefinito se omesso)
- `["code_execution_20250825"]` - Richiamabile solo dall'interno dell'esecuzione del codice
- `["direct", "code_execution_20250825"]` - Richiamabile sia direttamente che dall'esecuzione del codice

<Tip>
Consigliamo di scegliere `["direct"]` o `["code_execution_20250825"]` per ogni strumento piuttosto che abilitare entrambi, in quanto ciò fornisce una guida più chiara a Claude su come utilizzare al meglio lo strumento.
</Tip>

### Il campo `caller` nelle risposte

Ogni blocco di utilizzo dello strumento include un campo `caller` che indica come è stato invocato:

**Invocazione diretta (utilizzo tradizionale dello strumento):**
```json
{
  "type": "tool_use",
  "id": "toolu_abc123",
  "name": "query_database",
  "input": {"sql": "<sql>"},
  "caller": {"type": "direct"}
}
```

**Invocazione programmatica:**
```json
{
  "type": "tool_use",
  "id": "toolu_xyz789",
  "name": "query_database",
  "input": {"sql": "<sql>"},
  "caller": {
    "type": "code_execution_20250825",
    "tool_id": "srvtoolu_abc123"
  }
}
```

Il `tool_id` fa riferimento allo strumento di esecuzione del codice che ha effettuato la chiamata programmatica.

### Ciclo di vita del contenitore

L'invocazione programmatica di strumenti utilizza gli stessi contenitori dell'esecuzione del codice:

- **Creazione del contenitore**: Un nuovo contenitore viene creato per ogni sessione a meno che non riutilizzi uno esistente
- **Scadenza**: I contenitori scadono dopo circa 4,5 minuti di inattività (soggetto a modifiche)
- **ID del contenitore**: Restituito nelle risposte tramite il campo `container`
- **Riutilizzo**: Passa l'ID del contenitore per mantenere lo stato tra le richieste

<Warning>
Quando uno strumento viene chiamato programmaticamente e il contenitore è in attesa del risultato dello strumento, devi rispondere prima che il contenitore scada. Monitora il campo `expires_at`. Se il contenitore scade, Claude potrebbe trattare la chiamata dello strumento come scaduta e ritentarla.
</Warning>

## Flusso di lavoro di esempio

Ecco come funziona un flusso di invocazione programmatica di strumenti completo:

### Passaggio 1: Richiesta iniziale

Invia una richiesta con esecuzione del codice e uno strumento che consente la chiamata programmatica. Per abilitare la chiamata programmatica, aggiungi il campo `allowed_callers` alla definizione dello strumento.

<Note>
Fornisci descrizioni dettagliate del formato di output dello strumento nella descrizione dello strumento. Se specifichi che lo strumento restituisce JSON, Claude tenterà di deserializzare ed elaborare il risultato nel codice. Più dettagli fornisci sullo schema di output, meglio Claude può gestire la risposta in modo programmatico.
</Note>

<CodeGroup>
```python Python
response = client.beta.messages.create(
    model="claude-sonnet-4-5",
    betas=["advanced-tool-use-2025-11-20"],
    max_tokens=4096,
    messages=[{
        "role": "user",
        "content": "Query customer purchase history from the last quarter and identify our top 5 customers by revenue"
    }],
    tools=[
        {
            "type": "code_execution_20250825",
            "name": "code_execution"
        },
        {
            "name": "query_database",
            "description": "Execute a SQL query against the sales database. Returns a list of rows as JSON objects.",
            "input_schema": {...},
            "allowed_callers": ["code_execution_20250825"]
        }
    ]
)
```

```typescript TypeScript
const response = await anthropic.beta.messages.create({
  model: "claude-sonnet-4-5",
  betas: ["advanced-tool-use-2025-11-20"],
  max_tokens: 4096,
  messages: [{
    role: "user",
    content: "Query customer purchase history from the last quarter and identify our top 5 customers by revenue"
  }],
  tools: [
    {
      type: "code_execution_20250825",
      name: "code_execution"
    },
    {
      name: "query_database",
      description: "Execute a SQL query against the sales database. Returns a list of rows as JSON objects.",
      input_schema: {...},
      allowed_callers: ["code_execution_20250825"]
    }
  ]
});
```
</CodeGroup>

### Passaggio 2: Risposta API con chiamata dello strumento

Claude scrive codice che chiama il tuo strumento. L'API si interrompe e restituisce:

```json
{
  "role": "assistant",
  "content": [
    {
      "type": "text",
      "text": "I'll query the purchase history and analyze the results."
    },
    {
      "type": "server_tool_use",
      "id": "srvtoolu_abc123",
      "name": "code_execution",
      "input": {
        "code": "results = await query_database('<sql>')\ntop_customers = sorted(results, key=lambda x: x['revenue'], reverse=True)[:5]\nprint(f'Top 5 customers: {top_customers}')"
      }
    },
    {
      "type": "tool_use",
      "id": "toolu_def456",
      "name": "query_database",
      "input": {"sql": "<sql>"},
      "caller": {
        "type": "code_execution_20250825",
        "tool_id": "srvtoolu_abc123"
      }
    }
  ],
  "container": {
    "id": "container_xyz789",
    "expires_at": "2025-01-15T14:30:00Z"
  },
  "stop_reason": "tool_use"
}
```

### Passaggio 3: Fornisci il risultato dello strumento

Includi la cronologia completa della conversazione più il risultato dello strumento:

<CodeGroup>
```python Python
response = client.beta.messages.create(
    model="claude-sonnet-4-5",
    betas=["advanced-tool-use-2025-11-20"],
    max_tokens=4096,
    container="container_xyz789",  # Riutilizza il contenitore
    messages=[
        {"role": "user", "content": "Query customer purchase history from the last quarter and identify our top 5 customers by revenue"},
        {
            "role": "assistant",
            "content": [
                {"type": "text", "text": "I'll query the purchase history and analyze the results."},
                {
                    "type": "server_tool_use",
                    "id": "srvtoolu_abc123",
                    "name": "code_execution",
                    "input": {"code": "..."}
                },
                {
                    "type": "tool_use",
                    "id": "toolu_def456",
                    "name": "query_database",
                    "input": {"sql": "<sql>"},
                    "caller": {
                        "type": "code_execution_20250825",
                        "tool_id": "srvtoolu_abc123"
                    }
                }
            ]
        },
        {
            "role": "user",
            "content": [
                {
                    "type": "tool_result",
                    "tool_use_id": "toolu_def456",
                    "content": "[{\"customer_id\": \"C1\", \"revenue\": 45000}, {\"customer_id\": \"C2\", \"revenue\": 38000}, ...]"
                }
            ]
        }
    ],
    tools=[...]
)
```

```typescript TypeScript
const response = await anthropic.beta.messages.create({
  model: "claude-sonnet-4-5",
  betas: ["advanced-tool-use-2025-11-20"],
  max_tokens: 4096,
  container: "container_xyz789",  // Riutilizza il contenitore
  messages: [
    { role: "user", content: "Query customer purchase history from the last quarter and identify our top 5 customers by revenue" },
    {
      role: "assistant",
      content: [
        { type: "text", text: "I'll query the purchase history and analyze the results." },
        {
          type: "server_tool_use",
          id: "srvtoolu_abc123",
          name: "code_execution",
          input: { code: "..." }
        },
        {
          type: "tool_use",
          id: "toolu_def456",
          name: "query_database",
          input: { sql: "<sql>" },
          caller: {
            type: "code_execution_20250825",
            tool_id: "srvtoolu_abc123"
          }
        }
      ]
    },
    {
      role: "user",
      content: [
        {
          type: "tool_result",
          tool_use_id: "toolu_def456",
          content: "[{\"customer_id\": \"C1\", \"revenue\": 45000}, {\"customer_id\": \"C2\", \"revenue\": 38000}, ...]"
        }
      ]
    }
  ],
  tools: [...]
});
```
</CodeGroup>

### Passaggio 4: Prossima chiamata dello strumento o completamento

L'esecuzione del codice continua ed elabora i risultati. Se sono necessarie ulteriori chiamate di strumenti, ripeti il Passaggio 3 fino a quando tutte le chiamate di strumenti non sono soddisfatte.

### Passaggio 5: Risposta finale

Una volta completata l'esecuzione del codice, Claude fornisce la risposta finale:

```json
{
  "content": [
    {
      "type": "code_execution_tool_result",
      "tool_use_id": "srvtoolu_abc123",
      "content": {
        "type": "code_execution_result",
        "stdout": "Top 5 customers by revenue:\n1. Customer C1: $45,000\n2. Customer C2: $38,000\n3. Customer C5: $32,000\n4. Customer C8: $28,500\n5. Customer C3: $24,000",
        "stderr": "",
        "return_code": 0,
        "content": []
      }
    },
    {
      "type": "text",
      "text": "I've analyzed the purchase history from last quarter. Your top 5 customers generated $167,500 in total revenue, with Customer C1 leading at $45,000."
    }
  ],
  "stop_reason": "end_turn"
}
```

## Modelli avanzati

### Elaborazione in batch con cicli

Claude può scrivere codice che elabora più elementi in modo efficiente:

```python
# async wrapper omitted for clarity
regions = ["West", "East", "Central", "North", "South"]
results = {}
for region in regions:
    data = await query_database(f"<sql for {region}>")
    results[region] = sum(row["revenue"] for row in data)

# Process results programmatically
top_region = max(results.items(), key=lambda x: x[1])
print(f"Top region: {top_region[0]} with ${top_region[1]:,} in revenue")
```

Questo modello:
- Riduce i round trip del modello da N (uno per regione) a 1
- Elabora grandi set di risultati in modo programmatico prima di tornare a Claude
- Risparmia token restituendo solo conclusioni aggregate invece di dati grezzi

### Terminazione anticipata

Claude può interrompere l'elaborazione non appena vengono soddisfatti i criteri di successo:

```python
# async wrapper omitted for clarity
endpoints = ["us-east", "eu-west", "apac"]
for endpoint in endpoints:
    status = await check_health(endpoint)
    if status == "healthy":
        print(f"Found healthy endpoint: {endpoint}")
        break  # Stop early, don't check remaining
```

### Selezione condizionale dello strumento

```python
# async wrapper omitted for clarity
file_info = await get_file_info(path)
if file_info["size"] < 10000:
    content = await read_full_file(path)
else:
    content = await read_file_summary(path)
print(content)
```

### Filtraggio dei dati

```python
# async wrapper omitted for clarity
logs = await fetch_logs(server_id)
errors = [log for log in logs if "ERROR" in log]
print(f"Found {len(errors)} errors")
for error in errors[-10:]:  # Only return last 10 errors
    print(error)
```

## Formato della risposta

### Chiamata dello strumento programmatico

Quando l'esecuzione del codice chiama uno strumento:

```json
{
  "type": "tool_use",
  "id": "toolu_abc123",
  "name": "query_database",
  "input": {"sql": "<sql>"},
  "caller": {
    "type": "code_execution_20250825",
    "tool_id": "srvtoolu_xyz789"
  }
}
```

### Gestione del risultato dello strumento

Il risultato dello strumento viene passato al codice in esecuzione:

```json
{
  "role": "user",
  "content": [
    {
      "type": "tool_result",
      "tool_use_id": "toolu_abc123",
      "content": "[{\"customer_id\": \"C1\", \"revenue\": 45000, \"orders\": 23}, {\"customer_id\": \"C2\", \"revenue\": 38000, \"orders\": 18}, ...]"
    }
  ]
}
```

### Completamento dell'esecuzione del codice

Quando tutte le chiamate di strumenti sono soddisfatte e il codice si completa:

```json
{
  "type": "code_execution_tool_result",
  "tool_use_id": "srvtoolu_xyz789",
  "content": {
    "type": "code_execution_result",
    "stdout": "Analysis complete. Top 5 customers identified from 847 total records.",
    "stderr": "",
    "return_code": 0,
    "content": []
  }
}
```

## Gestione degli errori

### Errori comuni

| Errore | Descrizione | Soluzione |
|-------|-------------|----------|
| `invalid_tool_input` | L'input dello strumento non corrisponde allo schema | Convalida il input_schema dello strumento |
| `tool_not_allowed` | Lo strumento non consente il tipo di chiamante richiesto | Verifica che `allowed_callers` includa i contesti giusti |
| `missing_beta_header` | Intestazione beta PTC non fornita | Aggiungi entrambe le intestazioni beta alla tua richiesta |

### Scadenza del contenitore durante la chiamata dello strumento

Se il tuo strumento impiega troppo tempo per rispondere, l'esecuzione del codice riceverà un `TimeoutError`. Claude lo vede in stderr e di solito ritenterà:

```json
{
  "type": "code_execution_tool_result",
  "tool_use_id": "srvtoolu_abc123",
  "content": {
    "type": "code_execution_result",
    "stdout": "",
    "stderr": "TimeoutError: Calling tool ['query_database'] timed out.",
    "return_code": 0,
    "content": []
  }
}
```

Per prevenire i timeout:
- Monitora il campo `expires_at` nelle risposte
- Implementa timeout per l'esecuzione dello strumento
- Considera di suddividere le operazioni lunghe in blocchi più piccoli

### Errori di esecuzione dello strumento

Se il tuo strumento restituisce un errore:

```python
# Fornisci informazioni sull'errore nel risultato dello strumento
{
    "type": "tool_result",
    "tool_use_id": "toolu_abc123",
    "content": "Error: Query timeout - table lock exceeded 30 seconds"
}
```

Il codice di Claude riceverà questo errore e può gestirlo in modo appropriato.

## Vincoli e limitazioni

### Incompatibilità delle funzioni

- **Output strutturati**: Gli strumenti con `strict: true` non sono supportati con la chiamata programmatica
- **Scelta dello strumento**: Non puoi forzare la chiamata programmatica di uno strumento specifico tramite `tool_choice`
- **Utilizzo parallelo dello strumento**: `disable_parallel_tool_use: true` non è supportato con la chiamata programmatica

### Restrizioni degli strumenti

I seguenti strumenti attualmente non possono essere chiamati programmaticamente, ma il supporto potrebbe essere aggiunto nelle versioni future:

- Ricerca web
- Recupero web
- Strumenti forniti da un [connettore MCP](/docs/it/agents-and-tools/mcp-connector)

### Restrizioni sulla formattazione dei messaggi

Quando rispondi alle chiamate programmatiche degli strumenti, ci sono rigorosi requisiti di formattazione:

**Risposte solo con risultati dello strumento**: Se ci sono chiamate di strumenti programmatiche in sospeso in attesa di risultati, il tuo messaggio di risposta deve contenere **solo** blocchi `tool_result`. Non puoi includere alcun contenuto di testo, nemmeno dopo i risultati dello strumento.

```json
// ❌ INVALID - Cannot include text when responding to programmatic tool calls
{
  "role": "user",
  "content": [
    {"type": "tool_result", "tool_use_id": "toolu_01", "content": "[{\"customer_id\": \"C1\", \"revenue\": 45000}]"},
    {"type": "text", "text": "What should I do next?"}  // This will cause an error
  ]
}

// ✅ VALID - Only tool results when responding to programmatic tool calls
{
  "role": "user",
  "content": [
    {"type": "tool_result", "tool_use_id": "toolu_01", "content": "[{\"customer_id\": \"C1\", \"revenue\": 45000}]"}
  ]
}
```

Questa restrizione si applica solo quando rispondi alle chiamate di strumenti programmatiche (esecuzione del codice). Per le normali chiamate di strumenti lato client, puoi includere contenuto di testo dopo i risultati dello strumento.

### Limiti di velocità

Le chiamate programmatiche degli strumenti sono soggette agli stessi limiti di velocità delle normali chiamate di strumenti. Ogni chiamata di strumento dall'esecuzione del codice conta come una separata invocazione.

### Convalida i risultati dello strumento prima dell'uso

Quando implementi strumenti personalizzati che verranno chiamati programmaticamente:

- **I risultati dello strumento vengono restituiti come stringhe**: Possono contenere qualsiasi contenuto, inclusi frammenti di codice o comandi eseguibili che potrebbero essere elaborati dall'ambiente di esecuzione.
- **Convalida i risultati degli strumenti esterni**: Se il tuo strumento restituisce dati da fonti esterne o accetta input dell'utente, sii consapevole dei rischi di code injection se l'output verrà interpretato o eseguito come codice.

## Efficienza dei token

L'invocazione programmatica di strumenti può ridurre significativamente il consumo di token:

- **I risultati dello strumento dalle chiamate programmatiche non vengono aggiunti al contesto di Claude** - solo l'output finale del codice
- **L'elaborazione intermedia avviene nel codice** - filtraggio, aggregazione, ecc. non consumano token del modello
- **Più chiamate di strumenti in un'esecuzione di codice** - riduce il sovraccarico rispetto ai turni di modello separati

Ad esempio, chiamare 10 strumenti direttamente utilizza ~10 volte i token di chiamarli programmaticamente e restituire un riepilogo.

## Utilizzo e prezzi

L'invocazione programmatica di strumenti utilizza gli stessi prezzi dell'esecuzione del codice. Vedi i [prezzi dell'esecuzione del codice](/docs/it/agents-and-tools/tool-use/code-execution-tool#usage-and-pricing) per i dettagli.

<Note>
Conteggio dei token per le chiamate programmatiche degli strumenti: I risultati dello strumento dalle invocazioni programmatiche non contano verso l'utilizzo dei token di input/output. Solo il risultato finale dell'esecuzione del codice e la risposta di Claude contano.
</Note>

## Migliori pratiche

### Progettazione dello strumento

- **Fornisci descrizioni dettagliate dell'output**: Poiché Claude deserializza i risultati dello strumento nel codice, documenta chiaramente il formato (struttura JSON, tipi di campo, ecc.)
- **Restituisci dati strutturati**: I formati JSON o altri facilmente analizzabili funzionano meglio per l'elaborazione programmatica
- **Mantieni le risposte concise**: Restituisci solo i dati necessari per ridurre al minimo il sovraccarico di elaborazione

### Quando utilizzare la chiamata programmatica

**Buoni casi d'uso:**
- Elaborazione di grandi set di dati dove hai bisogno solo di aggregati o riepiloghi
- Flussi di lavoro multi-step con 3+ chiamate di strumenti dipendenti
- Operazioni che richiedono filtraggio, ordinamento o trasformazione dei risultati dello strumento
- Compiti in cui i dati intermedi non dovrebbero influenzare il ragionamento di Claude
- Operazioni parallele su molti elementi (ad es., verifica di 50 endpoint)

**Casi d'uso meno ideali:**
- Singole chiamate di strumenti con risposte semplici
- Strumenti che necessitano di feedback immediato dell'utente
- Operazioni molto veloci in cui il sovraccarico dell'esecuzione del codice supererebbe il vantaggio

### Ottimizzazione delle prestazioni

- **Riutilizza i contenitori** quando effettui più richieste correlate per mantenere lo stato
- **Raggruppa operazioni simili** in un'unica esecuzione di codice quando possibile

## Risoluzione dei problemi

### Problemi comuni

**Errore "Tool not allowed"**
- Verifica che la definizione dello strumento includa `"allowed_callers": ["code_execution_20250825"]`
- Controlla che stai utilizzando le intestazioni beta corrette

**Scadenza del contenitore**
- Assicurati di rispondere alle chiamate di strumenti entro la durata del contenitore (~4,5 minuti)
- Monitora il campo `expires_at` nelle risposte
- Considera di implementare un'esecuzione dello strumento più veloce

**Problemi con l'intestazione beta**
- Hai bisogno dell'intestazione: `"advanced-tool-use-2025-11-20"`

**Il risultato dello strumento non viene analizzato correttamente**
- Assicurati che il tuo strumento restituisca dati di stringa che Claude possa deserializzare
- Fornisci una chiara documentazione del formato di output nella descrizione dello strumento

### Suggerimenti per il debug

1. **Registra tutte le chiamate di strumenti e i risultati** per tracciare il flusso
2. **Controlla il campo `caller`** per confermare l'invocazione programmatica
3. **Monitora gli ID dei contenitori** per assicurare il corretto riutilizzo
4. **Testa gli strumenti indipendentemente** prima di abilitare la chiamata programmatica

## Perché funziona l'invocazione programmatica di strumenti

L'addestramento di Claude include un'ampia esposizione al codice, rendendolo efficace nel ragionare attraverso e concatenare le chiamate di funzioni. Quando gli strumenti vengono presentati come funzioni richiamabili all'interno di un ambiente di esecuzione del codice, Claude può sfruttare questa forza per:

- **Ragionare naturalmente sulla composizione dello strumento**: Concatenare operazioni e gestire le dipendenze naturalmente come scrivere qualsiasi codice Python
- **Elaborare grandi risultati in modo efficiente**: Filtrare i grandi output dello strumento, estrarre solo i dati rilevanti o scrivere i risultati intermedi su file prima di restituire riepiloghi alla finestra di contesto
- **Ridurre significativamente la latenza**: Eliminare il sovraccarico del ricampionamento di Claude tra ogni chiamata di strumento nei flussi di lavoro multi-step

Questo approccio abilita flussi di lavoro che sarebbero impraticabili con l'utilizzo tradizionale dello strumento, come l'elaborazione di file superiori a 1M token, consentendo a Claude di lavorare con i dati in modo programmatico piuttosto che caricare tutto nel contesto della conversazione.

## Implementazioni alternative

L'invocazione programmatica di strumenti è un modello generalizzabile che può essere implementato al di fuori dell'esecuzione del codice gestito di Anthropic. Ecco una panoramica degli approcci:

### Esecuzione diretta lato client

Fornisci a Claude uno strumento di esecuzione del codice e descrivi quali funzioni sono disponibili in quell'ambiente. Quando Claude invoca lo strumento con il codice, la tua applicazione lo esegue localmente dove quelle funzioni sono definite.

**Vantaggi:**
- Semplice da implementare con una riarchiettazione minima
- Controllo completo sull'ambiente e sulle istruzioni

**Svantaggi:**
- Esegue codice non attendibile al di fuori di una sandbox
- Le invocazioni dello strumento possono essere vettori per code injection

**Usa quando:** La tua applicazione può eseguire in modo sicuro codice arbitrario, desideri una soluzione semplice e l'offerta gestita di Anthropic non si adatta alle tue esigenze.

### Esecuzione sandbox auto-gestita

Lo stesso approccio dal punto di vista di Claude, ma il codice viene eseguito in un contenitore sandbox con restrizioni di sicurezza (ad es., nessun egresso di rete). Se i tuoi strumenti richiedono risorse esterne, avrai bisogno di un protocollo per eseguire le chiamate di strumenti al di fuori della sandbox.

**Vantaggi:**
- Chiamata programmatica sicura dello strumento sulla tua infrastruttura
- Controllo completo sull'ambiente di esecuzione

**Svantaggi:**
- Complesso da costruire e mantenere
- Richiede la gestione sia dell'infrastruttura che della comunicazione inter-processo

**Usa quando:** La sicurezza è critica e la soluzione gestita di Anthropic non si adatta ai tuoi requisiti.

### Esecuzione gestita da Anthropic

L'invocazione programmatica di strumenti di Anthropic è una versione gestita dell'esecuzione sandbox con un ambiente Python orientato che è sintonizzato per Claude. Anthropic gestisce la gestione dei contenitori, l'esecuzione del codice e la comunicazione sicura dell'invocazione dello strumento.

**Vantaggi:**
- Sicuro e protetto per impostazione predefinita
- Facile da abilitare con una configurazione minima
- Ambiente e istruzioni ottimizzati per Claude

Consigliamo di utilizzare la soluzione gestita di Anthropic se stai utilizzando l'API Claude.

## Funzioni correlate

<CardGroup cols={2}>
  <Card title="Code Execution Tool" icon="code" href="/docs/it/agents-and-tools/tool-use/code-execution-tool">
    Scopri la capacità di esecuzione del codice sottostante che alimenta l'invocazione programmatica di strumenti.
  </Card>
  <Card title="Tool Use Overview" icon="wrench" href="/docs/it/agents-and-tools/tool-use/overview">
    Comprendi i fondamenti dell'utilizzo dello strumento con Claude.
  </Card>
  <Card title="Implement Tool Use" icon="hammer" href="/docs/it/agents-and-tools/tool-use/implement-tool-use">
    Guida passo dopo passo per implementare gli strumenti.
  </Card>
</CardGroup>