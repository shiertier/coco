# Strumento di ricerca degli strumenti

Abilita Claude a lavorare con centinaia o migliaia di strumenti scoprendo e caricando dinamicamente quelli necessari su richiesta.

---

Lo strumento di ricerca degli strumenti consente a Claude di lavorare con centinaia o migliaia di strumenti scoprendo e caricando dinamicamente quelli necessari su richiesta. Invece di caricare tutte le definizioni degli strumenti nella finestra di contesto in anticipo, Claude cerca il tuo catalogo di strumenti—inclusi nomi degli strumenti, descrizioni, nomi degli argomenti e descrizioni degli argomenti—e carica solo gli strumenti di cui ha bisogno.

Questo approccio risolve due sfide critiche man mano che le librerie di strumenti si espandono:

- **Efficienza del contesto**: Le definizioni degli strumenti possono consumare porzioni massicce della tua finestra di contesto (50 strumenti ≈ 10-20K token), lasciando meno spazio per il lavoro effettivo
- **Accuratezza della selezione degli strumenti**: La capacità di Claude di selezionare correttamente gli strumenti si degrada significativamente con più di 30-50 strumenti disponibili convencionalmente

Sebbene questo sia fornito come uno strumento lato server, puoi anche implementare la tua funzionalità di ricerca degli strumenti lato client. Vedi [Implementazione personalizzata della ricerca degli strumenti](#custom-tool-search-implementation) per i dettagli.

<Note>
Lo strumento di ricerca degli strumenti è attualmente in beta pubblica. Includi l'appropriato [header beta](/docs/it/api/beta-headers) per il tuo provider:

| Provider                 | Header beta                    | Modelli supportati                     |
| ------------------------ | ------------------------------ | -------------------------------------- |
| Claude API<br/>Microsoft Foundry  | `advanced-tool-use-2025-11-20` | Claude Opus 4.5<br />Claude Sonnet 4.5 |
| Google Cloud's Vertex AI | `tool-search-tool-2025-10-19`  | Claude Opus 4.5<br />Claude Sonnet 4.5 |
| Amazon Bedrock           | `tool-search-tool-2025-10-19`  | Claude Opus 4.5                        |
</Note>

<Warning>
  Su Amazon Bedrock, la ricerca degli strumenti lato server è disponibile solo tramite l'[API invoke](https://docs.aws.amazon.com/bedrock/latest/userguide/bedrock-runtime_example_bedrock-runtime_InvokeModel_AnthropicClaude_section.html),
  non l'API converse.
</Warning>

Puoi anche implementare [ricerca degli strumenti lato client](#custom-tool-search-implementation) restituendo blocchi `tool_reference` dalla tua implementazione di ricerca personalizzata.

## Come funziona la ricerca degli strumenti

Ci sono due varianti di ricerca degli strumenti:

- **Regex** (`tool_search_tool_regex_20251119`): Claude costruisce pattern regex per cercare gli strumenti
- **BM25** (`tool_search_tool_bm25_20251119`): Claude utilizza query in linguaggio naturale per cercare gli strumenti

Quando abiliti lo strumento di ricerca degli strumenti:

1. Includi uno strumento di ricerca degli strumenti (ad es. `tool_search_tool_regex_20251119` o `tool_search_tool_bm25_20251119`) nella tua lista di strumenti
2. Fornisci tutte le definizioni degli strumenti con `defer_loading: true` per gli strumenti che non dovrebbero essere caricati immediatamente
3. Claude vede solo lo strumento di ricerca degli strumenti e gli strumenti non differiti inizialmente
4. Quando Claude ha bisogno di strumenti aggiuntivi, cerca utilizzando uno strumento di ricerca degli strumenti
5. L'API restituisce 3-5 blocchi `tool_reference` più rilevanti
6. Questi riferimenti vengono automaticamente espansi in definizioni complete degli strumenti
7. Claude seleziona dagli strumenti scoperti e li invoca

Questo mantiene la tua finestra di contesto efficiente mantenendo un'elevata accuratezza nella selezione degli strumenti.

## Avvio rapido

Ecco un semplice esempio con strumenti differiti:

<CodeGroup>
```bash Shell
curl https://api.anthropic.com/v1/messages \
    --header "x-api-key: $ANTHROPIC_API_KEY" \
    --header "anthropic-version: 2023-06-01" \
    --header "anthropic-beta: advanced-tool-use-2025-11-20" \
    --header "content-type: application/json" \
    --data '{
        "model": "claude-sonnet-4-5-20250929",
        "max_tokens": 2048,
        "messages": [
            {
                "role": "user",
                "content": "What is the weather in San Francisco?"
            }
        ],
        "tools": [
            {
                "type": "tool_search_tool_regex_20251119",
                "name": "tool_search_tool_regex"
            },
            {
                "name": "get_weather",
                "description": "Get the weather at a specific location",
                "input_schema": {
                    "type": "object",
                    "properties": {
                        "location": {"type": "string"},
                        "unit": {
                            "type": "string",
                            "enum": ["celsius", "fahrenheit"]
                        }
                    },
                    "required": ["location"]
                },
                "defer_loading": true
            },
            {
                "name": "search_files",
                "description": "Search through files in the workspace",
                "input_schema": {
                    "type": "object",
                    "properties": {
                        "query": {"type": "string"},
                        "file_types": {
                            "type": "array",
                            "items": {"type": "string"}
                        }
                    },
                    "required": ["query"]
                },
                "defer_loading": true
            }
        ]
    }'
```

```python Python
import anthropic

client = anthropic.Anthropic()

response = client.beta.messages.create(
    model="claude-sonnet-4-5-20250929",
    betas=["advanced-tool-use-2025-11-20"],
    max_tokens=2048,
    messages=[
        {
            "role": "user",
            "content": "What is the weather in San Francisco?"
        }
    ],
    tools=[
        {
            "type": "tool_search_tool_regex_20251119",
            "name": "tool_search_tool_regex"
        },
        {
            "name": "get_weather",
            "description": "Get the weather at a specific location",
            "input_schema": {
                "type": "object",
                "properties": {
                    "location": {"type": "string"},
                    "unit": {
                        "type": "string",
                        "enum": ["celsius", "fahrenheit"]
                    }
                },
                "required": ["location"]
            },
            "defer_loading": True
        },
        {
            "name": "search_files",
            "description": "Search through files in the workspace",
            "input_schema": {
                "type": "object",
                "properties": {
                    "query": {"type": "string"},
                    "file_types": {
                        "type": "array",
                        "items": {"type": "string"}
                    }
                },
                "required": ["query"]
            },
            "defer_loading": True
        }
    ]
)

print(response)
```

```typescript TypeScript
import Anthropic from "@anthropic-ai/sdk";

const client = new Anthropic();

async function main() {
  const response = await client.beta.messages.create({
    model: "claude-sonnet-4-5-20250929",
    betas: ["advanced-tool-use-2025-11-20"],
    max_tokens: 2048,
    messages: [
      {
        role: "user",
        content: "What is the weather in San Francisco?",
      },
    ],
    tools: [
      {
        type: "tool_search_tool_regex_20251119",
        name: "tool_search_tool_regex",
      },
      {
        name: "get_weather",
        description: "Get the weather at a specific location",
        input_schema: {
          type: "object",
          properties: {
            location: { type: "string" },
            unit: {
              type: "string",
              enum: ["celsius", "fahrenheit"],
            },
          },
          required: ["location"],
        },
        defer_loading: true,
      },
      {
        name: "search_files",
        description: "Search through files in the workspace",
        input_schema: {
          type: "object",
          properties: {
            query: { type: "string" },
            file_types: {
              type: "array",
              items: { type: "string" },
            },
          },
          required: ["query"],
        },
        defer_loading: true,
      },
    ],
  });

  console.log(JSON.stringify(response, null, 2));
}

main();
```

</CodeGroup>

## Definizione dello strumento

Lo strumento di ricerca degli strumenti ha due varianti:

```json JSON
{
  "type": "tool_search_tool_regex_20251119",
  "name": "tool_search_tool_regex"
}
```

```json JSON
{
  "type": "tool_search_tool_bm25_20251119",
  "name": "tool_search_tool_bm25"
}
```

<Warning>
**Formato della query della variante Regex: Regex Python, NON linguaggio naturale**

Quando si utilizza `tool_search_tool_regex_20251119`, Claude costruisce pattern regex utilizzando la sintassi `re.search()` di Python, non query in linguaggio naturale. Pattern comuni:

- `"weather"` - corrisponde ai nomi/descrizioni degli strumenti contenenti "weather"
- `"get_.*_data"` - corrisponde a strumenti come `get_user_data`, `get_weather_data`
- `"database.*query|query.*database"` - pattern OR per flessibilità
- `"(?i)slack"` - ricerca insensibile alle maiuscole

Lunghezza massima della query: 200 caratteri

</Warning>

<Note>
**Formato della query della variante BM25: Linguaggio naturale**

Quando si utilizza `tool_search_tool_bm25_20251119`, Claude utilizza query in linguaggio naturale per cercare gli strumenti.

</Note>

### Caricamento differito degli strumenti

Contrassegna gli strumenti per il caricamento su richiesta aggiungendo `defer_loading: true`:

```json JSON
{
  "name": "get_weather",
  "description": "Get current weather for a location",
  "input_schema": {
    "type": "object",
    "properties": {
      "location": { "type": "string" },
      "unit": { "type": "string", "enum": ["celsius", "fahrenheit"] }
    },
    "required": ["location"]
  },
  "defer_loading": true
}
```

**Punti chiave:**

- Gli strumenti senza `defer_loading` vengono caricati nel contesto immediatamente
- Gli strumenti con `defer_loading: true` vengono caricati solo quando Claude li scopre tramite ricerca
- Lo strumento di ricerca degli strumenti stesso non dovrebbe **mai** avere `defer_loading: true`
- Mantieni i tuoi 3-5 strumenti più utilizzati frequentemente come non differiti per prestazioni ottimali

Entrambe le varianti di ricerca degli strumenti (`regex` e `bm25`) cercano nomi degli strumenti, descrizioni, nomi degli argomenti e descrizioni degli argomenti.

## Formato della risposta

Quando Claude utilizza lo strumento di ricerca degli strumenti, la risposta include nuovi tipi di blocco:

```json JSON
{
  "role": "assistant",
  "content": [
    {
      "type": "text",
      "text": "I'll search for tools to help with the weather information."
    },
    {
      "type": "server_tool_use",
      "id": "srvtoolu_01ABC123",
      "name": "tool_search_tool_regex",
      "input": {
        "query": "weather"
      }
    },
    {
      "type": "tool_search_tool_result",
      "tool_use_id": "srvtoolu_01ABC123",
      "content": {
        "type": "tool_search_tool_search_result",
        "tool_references": [{ "type": "tool_reference", "tool_name": "get_weather" }]
      }
    },
    {
      "type": "text",
      "text": "I found a weather tool. Let me get the weather for San Francisco."
    },
    {
      "type": "tool_use",
      "id": "toolu_01XYZ789",
      "name": "get_weather",
      "input": { "location": "San Francisco", "unit": "fahrenheit" }
    }
  ],
  "stop_reason": "tool_use"
}
```

### Comprensione della risposta

- **`server_tool_use`**: Indica che Claude sta invocando lo strumento di ricerca degli strumenti
- **`tool_search_tool_result`**: Contiene i risultati della ricerca con un oggetto `tool_search_tool_search_result` annidato
- **`tool_references`**: Array di oggetti `tool_reference` che puntano agli strumenti scoperti
- **`tool_use`**: Claude che invoca lo strumento scoperto

I blocchi `tool_reference` vengono automaticamente espansi in definizioni complete degli strumenti prima di essere mostrati a Claude. Non è necessario gestire questa espansione da solo. Accade automaticamente nell'API purché fornisci tutte le definizioni degli strumenti corrispondenti nel parametro `tools`.

## Integrazione MCP

Lo strumento di ricerca degli strumenti funziona con i [server MCP](/docs/it/agents-and-tools/mcp-connector). Aggiungi l'[header beta](/docs/it/api/beta-headers) `"mcp-client-2025-11-20"` alla tua richiesta API, e quindi utilizza `mcp_toolset` con `default_config` per differire il caricamento degli strumenti MCP:

<CodeGroup>
```bash Shell
curl https://api.anthropic.com/v1/messages \
  --header "x-api-key: $ANTHROPIC_API_KEY" \
  --header "anthropic-version: 2023-06-01" \
  --header "anthropic-beta: advanced-tool-use-2025-11-20,mcp-client-2025-11-20" \
  --header "content-type: application/json" \
  --data '{
    "model": "claude-sonnet-4-5-20250929",
    "max_tokens": 2048,
    "mcp_servers": [
      {
        "type": "url",
        "name": "database-server",
        "url": "https://mcp-db.example.com"
      }
    ],
    "tools": [
      {
        "type": "tool_search_tool_regex_20251119",
        "name": "tool_search_tool_regex"
      },
      {
        "type": "mcp_toolset",
        "mcp_server_name": "database-server",
        "default_config": {
          "defer_loading": true
        },
        "configs": {
          "search_events": {
            "defer_loading": false
          }
        }
      }
    ],
    "messages": [
      {
        "role": "user",
        "content": "What events are in my database?"
      }
    ]
  }'
```

```python Python
import anthropic

client = anthropic.Anthropic()

response = client.beta.messages.create(
    model="claude-sonnet-4-5-20250929",
    betas=["advanced-tool-use-2025-11-20", "mcp-client-2025-11-20"],
    max_tokens=2048,
    mcp_servers=[
        {
            "type": "url",
            "name": "database-server",
            "url": "https://mcp-db.example.com"
        }
    ],
    tools=[
        {
            "type": "tool_search_tool_regex_20251119",
            "name": "tool_search_tool_regex"
        },
        {
            "type": "mcp_toolset",
            "mcp_server_name": "database-server",
            "default_config": {
                "defer_loading": True
            },
            "configs": {
                "search_events": {
                    "defer_loading": False
                }
            }
        }
    ],
    messages=[
        {
            "role": "user",
            "content": "What events are in my database?"
        }
    ]
)

print(response)
```

```typescript TypeScript
import Anthropic from "@anthropic-ai/sdk";

const client = new Anthropic();

async function main() {
  const response = await client.beta.messages.create({
    model: "claude-sonnet-4-5-20250929",
    betas: ["advanced-tool-use-2025-11-20", "mcp-client-2025-11-20"],
    max_tokens: 2048,
    mcp_servers: [
      {
        type: "url",
        name: "database-server",
        url: "https://mcp-db.example.com",
      },
    ],
    tools: [
      {
        type: "tool_search_tool_regex_20251119",
        name: "tool_search_tool_regex",
      },
      {
        type: "mcp_toolset",
        mcp_server_name: "database-server",
        default_config: {
          defer_loading: true,
        },
        configs: {
          search_events: {
            defer_loading: false,
          },
        },
      },
    ],
    messages: [
      {
        role: "user",
        content: "What events are in my database?",
      },
    ],
  });

  console.log(JSON.stringify(response, null, 2));
}

main();
```

</CodeGroup>

**Opzioni di configurazione MCP:**

- `default_config.defer_loading`: Imposta il valore predefinito per tutti gli strumenti dal server MCP
- `configs`: Sostituisci i valori predefiniti per strumenti specifici per nome
- Combina più server MCP con la ricerca degli strumenti per librerie di strumenti massicce

## Implementazione personalizzata della ricerca degli strumenti

Puoi implementare la tua logica di ricerca degli strumenti (ad es. utilizzando embeddings o ricerca semantica) restituendo blocchi `tool_reference` da uno strumento personalizzato:

```json JSON
{
  "type": "tool_search_tool_result",
  "tool_use_id": "toolu_custom_search",
  "content": {
    "type": "tool_search_tool_search_result",
    "tool_references": [{ "type": "tool_reference", "tool_name": "discovered_tool_name" }]
  }
}
```

Ogni strumento referenziato deve avere una definizione dello strumento corrispondente nel parametro `tools` di livello superiore con `defer_loading: true`. Questo approccio ti consente di utilizzare algoritmi di ricerca più sofisticati mantenendo la compatibilità con il sistema di ricerca degli strumenti.

Per un esempio completo utilizzando embeddings, vedi il nostro [cookbook di ricerca degli strumenti con embeddings](https://github.com/anthropics/anthropic-cookbook).

## Gestione degli errori

<Note>
  Lo strumento di ricerca degli strumenti non è compatibile con gli [esempi di utilizzo degli strumenti](/docs/it/agents-and-tools/tool-use/implement-tool-use#providing-tool-use-examples).
  Se hai bisogno di fornire esempi di utilizzo degli strumenti, utilizza la chiamata degli strumenti standard
  senza ricerca degli strumenti.
</Note>

### Errori HTTP (stato 400)

Questi errori impediscono l'elaborazione della richiesta:

**Tutti gli strumenti differiti:**

```json
{
  "type": "error",
  "error": {
    "type": "invalid_request_error",
    "message": "All tools have defer_loading set. At least one tool must be non-deferred."
  }
}
```

**Definizione dello strumento mancante:**

```json
{
  "type": "error",
  "error": {
    "type": "invalid_request_error",
    "message": "Tool reference 'unknown_tool' has no corresponding tool definition"
  }
}
```

### Errori dei risultati degli strumenti (stato 200)

Gli errori durante l'esecuzione dello strumento restituiscono una risposta 200 con informazioni di errore nel corpo:

```json JSON
{
  "type": "tool_result",
  "tool_use_id": "srvtoolu_01ABC123",
  "content": {
    "type": "tool_search_tool_result_error",
    "error_code": "invalid_pattern"
  }
}
```

**Codici di errore:**

- `too_many_requests`: Limite di velocità superato per le operazioni di ricerca degli strumenti
- `invalid_pattern`: Pattern regex malformato
- `pattern_too_long`: Il pattern supera il limite di 200 caratteri
- `unavailable`: Servizio di ricerca degli strumenti temporaneamente non disponibile

### Errori comuni

<section title="Errore 400: Tutti gli strumenti sono differiti">

**Causa**: Hai impostato `defer_loading: true` su TUTTI gli strumenti incluso lo strumento di ricerca

**Soluzione**: Rimuovi `defer_loading` dallo strumento di ricerca degli strumenti:

```json
{
  "type": "tool_search_tool_regex_20251119", // Nessun defer_loading qui
  "name": "tool_search_tool_regex"
}
```

</section>

<section title="Errore 400: Definizione dello strumento mancante">

**Causa**: Un `tool_reference` punta a uno strumento non nel tuo array `tools`

**Soluzione**: Assicurati che ogni strumento che potrebbe essere scoperto abbia una definizione completa:

```json
{
  "name": "my_tool",
  "description": "Full description here",
  "input_schema": {
    /* complete schema */
  },
  "defer_loading": true
}
```

</section>

<section title="Claude non trova gli strumenti previsti">

**Causa**: I nomi o le descrizioni degli strumenti non corrispondono al pattern regex

**Passaggi di debug:**

1. Controlla il nome e la descrizione dello strumento—Claude cerca in ENTRAMBI i campi
2. Testa il tuo pattern: `import re; re.search(r"your_pattern", "tool_name")`
3. Ricorda che le ricerche sono sensibili alle maiuscole per impostazione predefinita (usa `(?i)` per insensibile alle maiuscole)
4. Claude utilizza pattern ampi come `".*weather.*"` non corrispondenze esatte

**Suggerimento**: Aggiungi parole chiave comuni alle descrizioni degli strumenti per migliorare la scopribilità

</section>

## Caching dei prompt

La ricerca degli strumenti funziona con il [caching dei prompt](/docs/it/build-with-claude/prompt-caching). Aggiungi punti di interruzione `cache_control` per ottimizzare le conversazioni multi-turno:

<CodeGroup>
```python Python
import anthropic

client = anthropic.Anthropic()

# First request with tool search
messages = [
    {
        "role": "user",
        "content": "What's the weather in Seattle?"
    }
]

response1 = client.beta.messages.create(
    model="claude-sonnet-4-5-20250929",
    betas=["advanced-tool-use-2025-11-20"],
    max_tokens=2048,
    messages=messages,
    tools=[
        {
            "type": "tool_search_tool_regex_20251119",
            "name": "tool_search_tool_regex"
        },
        {
            "name": "get_weather",
            "description": "Get weather for a location",
            "input_schema": {
                "type": "object",
                "properties": {
                    "location": {"type": "string"}
                },
                "required": ["location"]
            },
            "defer_loading": True
        }
    ]
)

# Add Claude's response to conversation
messages.append({
    "role": "assistant",
    "content": response1.content
})

# Second request with cache breakpoint
messages.append({
    "role": "user",
    "content": "What about New York?",
    "cache_control": {"type": "ephemeral"}
})

response2 = client.beta.messages.create(
    model="claude-sonnet-4-5-20250929",
    betas=["advanced-tool-use-2025-11-20"],
    max_tokens=2048,
    messages=messages,
    tools=[
        {
            "type": "tool_search_tool_regex_20251119",
            "name": "tool_search_tool_regex"
        },
        {
            "name": "get_weather",
            "description": "Get weather for a location",
            "input_schema": {
                "type": "object",
                "properties": {
                    "location": {"type": "string"}
                },
                "required": ["location"]
            },
            "defer_loading": True
        }
    ]
)

print(f"Cache read tokens: {response2.usage.get('cache_read_input_tokens', 0)}")
```
</CodeGroup>

Il sistema espande automaticamente i blocchi tool_reference in tutta la cronologia della conversazione, quindi Claude può riutilizzare gli strumenti scoperti nei turni successivi senza ricercare di nuovo.

## Streaming

Con lo streaming abilitato, riceverai gli eventi di ricerca degli strumenti come parte dello stream:

```javascript
event: content_block_start
data: {"type": "content_block_start", "index": 1, "content_block": {"type": "server_tool_use", "id": "srvtoolu_xyz789", "name": "tool_search_tool_regex"}}

// Search query streamed
event: content_block_delta
data: {"type": "content_block_delta", "index": 1, "delta": {"type": "input_json_delta", "partial_json": "{\"query\":\"weather\"}"}}

// Pause while search executes

// Search results streamed
event: content_block_start
data: {"type": "content_block_start", "index": 2, "content_block": {"type": "tool_search_tool_result", "tool_use_id": "srvtoolu_xyz789", "content": {"type": "tool_search_tool_search_result", "tool_references": [{"type": "tool_reference", "tool_name": "get_weather"}]}}}

// Claude continues with discovered tools
```

## Richieste batch

Puoi includere lo strumento di ricerca degli strumenti nell'[API Messages Batches](/docs/it/build-with-claude/batch-processing). Le operazioni di ricerca degli strumenti tramite l'API Messages Batches hanno lo stesso prezzo di quelle nelle richieste dell'API Messages regolari.

## Limiti e best practice

### Limiti

- **Strumenti massimi**: 10.000 strumenti nel tuo catalogo
- **Risultati della ricerca**: Restituisce 3-5 strumenti più rilevanti per ricerca
- **Lunghezza del pattern**: Massimo 200 caratteri per i pattern regex
- **Supporto del modello**: Solo Sonnet 4.0+, Opus 4.0+ (no Haiku)

### Quando utilizzare la ricerca degli strumenti

**Buoni casi d'uso:**

- 10+ strumenti disponibili nel tuo sistema
- Le definizioni degli strumenti consumano >10K token
- Problemi di accuratezza nella selezione degli strumenti con grandi set di strumenti
- Costruzione di sistemi basati su MCP con più server (200+ strumenti)
- Libreria di strumenti in crescita nel tempo

**Quando la chiamata degli strumenti tradizionale potrebbe essere migliore:**

- Meno di 10 strumenti totali
- Tutti gli strumenti sono utilizzati frequentemente in ogni richiesta
- Definizioni degli strumenti molto piccole (\<100 token totali)

### Suggerimenti di ottimizzazione

- Mantieni i tuoi 3-5 strumenti più utilizzati frequentemente come non differiti
- Scrivi nomi e descrizioni degli strumenti chiari e descrittivi
- Utilizza parole chiave semantiche nelle descrizioni che corrispondono a come gli utenti descrivono i compiti
- Aggiungi una sezione di prompt di sistema che descrive le categorie di strumenti disponibili: "Puoi cercare strumenti per interagire con Slack, GitHub e Jira"
- Monitora quali strumenti Claude scopre per affinare le descrizioni

## Utilizzo

L'utilizzo dello strumento di ricerca degli strumenti viene tracciato nell'oggetto di utilizzo della risposta:

```json JSON
{
  "usage": {
    "input_tokens": 1024,
    "output_tokens": 256,
    "server_tool_use": {
      "tool_search_requests": 2
    }
  }
}
```