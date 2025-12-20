# Strumento di ricerca web

Lo strumento di ricerca web consente a Claude di accedere direttamente ai contenuti web in tempo reale, permettendogli di rispondere alle domande con informazioni aggiornate oltre il suo limite di conoscenza.

---

Lo strumento di ricerca web consente a Claude di accedere direttamente ai contenuti web in tempo reale, permettendogli di rispondere alle domande con informazioni aggiornate oltre il suo limite di conoscenza. Claude cita automaticamente le fonti dai risultati della ricerca come parte della sua risposta.

<Note>
Contattaci tramite il nostro [modulo di feedback](https://forms.gle/sWjBtsrNEY2oKGuE8) per condividere la tua esperienza con lo strumento di ricerca web.
</Note>

## Modelli supportati

La ricerca web è disponibile su:

- Claude Sonnet 4.5 (`claude-sonnet-4-5-20250929`)
- Claude Sonnet 4 (`claude-sonnet-4-20250514`)
- Claude Sonnet 3.7 ([deprecato](/docs/it/about-claude/model-deprecations)) (`claude-3-7-sonnet-20250219`)
- Claude Haiku 4.5 (`claude-haiku-4-5-20251001`)
- Claude Haiku 3.5 ([deprecato](/docs/it/about-claude/model-deprecations)) (`claude-3-5-haiku-latest`)
- Claude Opus 4.5 (`claude-opus-4-5-20251101`)
- Claude Opus 4.1 (`claude-opus-4-1-20250805`)
- Claude Opus 4 (`claude-opus-4-20250514`)

## Come funziona la ricerca web

Quando aggiungi lo strumento di ricerca web alla tua richiesta API:

1. Claude decide quando cercare in base al prompt.
2. L'API esegue le ricerche e fornisce a Claude i risultati. Questo processo può ripetersi più volte durante una singola richiesta.
3. Alla fine del suo turno, Claude fornisce una risposta finale con fonti citate.

## Come utilizzare la ricerca web

<Note>
L'amministratore della tua organizzazione deve abilitare la ricerca web in [Console](/settings/privacy).
</Note>

Fornisci lo strumento di ricerca web nella tua richiesta API:

<CodeGroup>
```bash Shell
curl https://api.anthropic.com/v1/messages \
    --header "x-api-key: $ANTHROPIC_API_KEY" \
    --header "anthropic-version: 2023-06-01" \
    --header "content-type: application/json" \
    --data '{
        "model": "claude-sonnet-4-5",
        "max_tokens": 1024,
        "messages": [
            {
                "role": "user",
                "content": "What is the weather in NYC?"
            }
        ],
        "tools": [{
            "type": "web_search_20250305",
            "name": "web_search",
            "max_uses": 5
        }]
    }'
```

```python Python
import anthropic

client = anthropic.Anthropic()

response = client.messages.create(
    model="claude-sonnet-4-5",
    max_tokens=1024,
    messages=[
        {
            "role": "user",
            "content": "What's the weather in NYC?"
        }
    ],
    tools=[{
        "type": "web_search_20250305",
        "name": "web_search",
        "max_uses": 5
    }]
)
print(response)
```

```typescript TypeScript
import { Anthropic } from '@anthropic-ai/sdk';

const anthropic = new Anthropic();

async function main() {
  const response = await anthropic.messages.create({
    model: "claude-sonnet-4-5",
    max_tokens: 1024,
    messages: [
      {
        role: "user",
        content: "What's the weather in NYC?"
      }
    ],
    tools: [{
      type: "web_search_20250305",
      name: "web_search",
      max_uses: 5
    }]
  });

  console.log(response);
}

main().catch(console.error);
```
</CodeGroup>

### Definizione dello strumento

Lo strumento di ricerca web supporta i seguenti parametri:

```json JSON
{
  "type": "web_search_20250305",
  "name": "web_search",

  // Facoltativo: Limita il numero di ricerche per richiesta
  "max_uses": 5,

  // Facoltativo: Includi solo risultati da questi domini
  "allowed_domains": ["example.com", "trusteddomain.org"],

  // Facoltativo: Non includere mai risultati da questi domini
  "blocked_domains": ["untrustedsource.com"],

  // Facoltativo: Localizza i risultati della ricerca
  "user_location": {
    "type": "approximate",
    "city": "San Francisco",
    "region": "California",
    "country": "US",
    "timezone": "America/Los_Angeles"
  }
}
```

#### Max uses

Il parametro `max_uses` limita il numero di ricerche eseguite. Se Claude tenta più ricerche di quelle consentite, il `web_search_tool_result` sarà un errore con il codice di errore `max_uses_exceeded`.

#### Filtro dei domini

Quando utilizzi i filtri dei domini:

- I domini non devono includere lo schema HTTP/HTTPS (usa `example.com` invece di `https://example.com`)
- I sottodomini sono inclusi automaticamente (`example.com` copre `docs.example.com`)
- I sottodomini specifici limitano i risultati solo a quel sottodominio (`docs.example.com` restituisce solo risultati da quel sottodominio, non da `example.com` o `api.example.com`)
- I percorsi secondari sono supportati e corrispondono a qualsiasi cosa dopo il percorso (`example.com/blog` corrisponde a `example.com/blog/post-1`)
- Puoi usare `allowed_domains` o `blocked_domains`, ma non entrambi nella stessa richiesta.

**Supporto dei caratteri jolly:**

- È consentito un solo carattere jolly (`*`) per voce di dominio e deve apparire dopo la parte del dominio (nel percorso)
- Valido: `example.com/*`, `example.com/*/articles`
- Non valido: `*.example.com`, `ex*.com`, `example.com/*/news/*`

I formati di dominio non validi restituiranno un errore dello strumento `invalid_tool_input`.

<Note>
Le restrizioni dei domini a livello di richiesta devono essere compatibili con le restrizioni dei domini a livello di organizzazione configurate nella Console. I domini a livello di richiesta possono solo limitare ulteriormente i domini, non ignorare o espandere oltre l'elenco a livello di organizzazione. Se la tua richiesta include domini che entrano in conflitto con le impostazioni dell'organizzazione, l'API restituirà un errore di convalida.
</Note>

#### Localizzazione

Il parametro `user_location` consente di localizzare i risultati della ricerca in base alla posizione di un utente.

- `type`: Il tipo di posizione (deve essere `approximate`)
- `city`: Il nome della città
- `region`: La regione o lo stato
- `country`: Il paese
- `timezone`: L'[ID del fuso orario IANA](https://en.wikipedia.org/wiki/List_of_tz_database_time_zones).

### Risposta

Ecco una struttura di risposta di esempio:

```json
{
  "role": "assistant",
  "content": [
    // 1. La decisione di Claude di cercare
    {
      "type": "text",
      "text": "I'll search for when Claude Shannon was born."
    },
    // 2. La query di ricerca utilizzata
    {
      "type": "server_tool_use",
      "id": "srvtoolu_01WYG3ziw53XMcoyKL4XcZmE",
      "name": "web_search",
      "input": {
        "query": "claude shannon birth date"
      }
    },
    // 3. Risultati della ricerca
    {
      "type": "web_search_tool_result",
      "tool_use_id": "srvtoolu_01WYG3ziw53XMcoyKL4XcZmE",
      "content": [
        {
          "type": "web_search_result",
          "url": "https://en.wikipedia.org/wiki/Claude_Shannon",
          "title": "Claude Shannon - Wikipedia",
          "encrypted_content": "EqgfCioIARgBIiQ3YTAwMjY1Mi1mZjM5LTQ1NGUtODgxNC1kNjNjNTk1ZWI3Y...",
          "page_age": "April 30, 2025"
        }
      ]
    },
    {
      "text": "Based on the search results, ",
      "type": "text"
    },
    // 4. La risposta di Claude con citazioni
    {
      "text": "Claude Shannon was born on April 30, 1916, in Petoskey, Michigan",
      "type": "text",
      "citations": [
        {
          "type": "web_search_result_location",
          "url": "https://en.wikipedia.org/wiki/Claude_Shannon",
          "title": "Claude Shannon - Wikipedia",
          "encrypted_index": "Eo8BCioIAhgBIiQyYjQ0OWJmZi1lNm..",
          "cited_text": "Claude Elwood Shannon (April 30, 1916 – February 24, 2001) was an American mathematician, electrical engineer, computer scientist, cryptographer and i..."
        }
      ]
    }
  ],
  "id": "msg_a930390d3a",
  "usage": {
    "input_tokens": 6039,
    "output_tokens": 931,
    "server_tool_use": {
      "web_search_requests": 1
    }
  },
  "stop_reason": "end_turn"
}
```

#### Risultati della ricerca

I risultati della ricerca includono:

- `url`: L'URL della pagina di origine
- `title`: Il titolo della pagina di origine
- `page_age`: Quando il sito è stato aggiornato l'ultima volta
- `encrypted_content`: Contenuto crittografato che deve essere passato indietro nelle conversazioni multi-turno per le citazioni

#### Citazioni

Le citazioni sono sempre abilitate per la ricerca web e ogni `web_search_result_location` include:

- `url`: L'URL della fonte citata
- `title`: Il titolo della fonte citata
- `encrypted_index`: Un riferimento che deve essere passato indietro per le conversazioni multi-turno.
- `cited_text`: Fino a 150 caratteri del contenuto citato

I campi di citazione della ricerca web `cited_text`, `title` e `url` non contano verso l'utilizzo dei token di input o output.

<Note>
  Quando visualizzi gli output dell'API direttamente agli utenti finali, le citazioni devono essere incluse alla fonte originale. Se stai apportando modifiche agli output dell'API, incluso rielaborandoli e/o combinandoli con il tuo materiale prima di visualizzarli agli utenti finali, visualizza le citazioni come appropriato in base alla consultazione con il tuo team legale.
</Note>

#### Errori

Quando lo strumento di ricerca web incontra un errore (come il raggiungimento dei limiti di velocità), l'API Claude restituisce comunque una risposta 200 (successo). L'errore è rappresentato nel corpo della risposta utilizzando la seguente struttura:

```json
{
  "type": "web_search_tool_result",
  "tool_use_id": "servertoolu_a93jad",
  "content": {
    "type": "web_search_tool_result_error",
    "error_code": "max_uses_exceeded"
  }
}
```

Questi sono i possibili codici di errore:

- `too_many_requests`: Limite di velocità superato
- `invalid_input`: Parametro di query di ricerca non valido
- `max_uses_exceeded`: Utilizzi massimi dello strumento di ricerca web superati
- `query_too_long`: La query supera la lunghezza massima
- `unavailable`: Si è verificato un errore interno

#### Motivo di arresto `pause_turn`

La risposta può includere un motivo di arresto `pause_turn`, che indica che l'API ha messo in pausa un turno di lunga durata. Puoi fornire la risposta così com'è in una richiesta successiva per consentire a Claude di continuare il suo turno, oppure modificare il contenuto se desideri interrompere la conversazione.

## Memorizzazione nella cache dei prompt

La ricerca web funziona con la [memorizzazione nella cache dei prompt](/docs/it/build-with-claude/prompt-caching). Per abilitare la memorizzazione nella cache dei prompt, aggiungi almeno un punto di interruzione `cache_control` nella tua richiesta. Il sistema memorizzerà automaticamente nella cache fino all'ultimo blocco `web_search_tool_result` durante l'esecuzione dello strumento.

Per le conversazioni multi-turno, imposta un punto di interruzione `cache_control` su o dopo l'ultimo blocco `web_search_tool_result` per riutilizzare il contenuto memorizzato nella cache.

Ad esempio, per utilizzare la memorizzazione nella cache dei prompt con la ricerca web per una conversazione multi-turno:

<CodeGroup>
```python
import anthropic

client = anthropic.Anthropic()

# Prima richiesta con ricerca web e punto di interruzione della cache
messages = [
    {
        "role": "user",
        "content": "What's the current weather in San Francisco today?"
    }
]

response1 = client.messages.create(
    model="claude-sonnet-4-5",
    max_tokens=1024,
    messages=messages,
    tools=[{
        "type": "web_search_20250305",
        "name": "web_search",
        "user_location": {
            "type": "approximate",
            "city": "San Francisco",
            "region": "California",
            "country": "US",
            "timezone": "America/Los_Angeles"
        }
    }]
)

# Aggiungi la risposta di Claude alla conversazione
messages.append({
    "role": "assistant",
    "content": response1.content
})

# Seconda richiesta con punto di interruzione della cache dopo i risultati della ricerca
messages.append({
    "role": "user",
    "content": "Should I expect rain later this week?",
    "cache_control": {"type": "ephemeral"}  # Memorizza nella cache fino a questo punto
})

response2 = client.messages.create(
    model="claude-sonnet-4-5",
    max_tokens=1024,
    messages=messages,
    tools=[{
        "type": "web_search_20250305",
        "name": "web_search",
        "user_location": {
            "type": "approximate",
            "city": "San Francisco",
            "region": "California",
            "country": "US",
            "timezone": "America/Los_Angeles"
        }
    }]
)
# La seconda risposta trarrà vantaggio dai risultati della ricerca memorizzati nella cache
# pur essendo in grado di eseguire nuove ricerche se necessario
print(f"Cache read tokens: {response2.usage.get('cache_read_input_tokens', 0)}")
```

</CodeGroup>

## Streaming

Con lo streaming abilitato, riceverai gli eventi di ricerca come parte dello stream. Ci sarà una pausa mentre la ricerca viene eseguita:

```javascript
event: message_start
data: {"type": "message_start", "message": {"id": "msg_abc123", "type": "message"}}

event: content_block_start
data: {"type": "content_block_start", "index": 0, "content_block": {"type": "text", "text": ""}}

// La decisione di Claude di cercare

event: content_block_start
data: {"type": "content_block_start", "index": 1, "content_block": {"type": "server_tool_use", "id": "srvtoolu_xyz789", "name": "web_search"}}

// Query di ricerca trasmessa in streaming
event: content_block_delta
data: {"type": "content_block_delta", "index": 1, "delta": {"type": "input_json_delta", "partial_json": "{\"query\":\"latest quantum computing breakthroughs 2025\"}"}}

// Pausa mentre la ricerca viene eseguita

// Risultati della ricerca trasmessi in streaming
event: content_block_start
data: {"type": "content_block_start", "index": 2, "content_block": {"type": "web_search_tool_result", "tool_use_id": "srvtoolu_xyz789", "content": [{"type": "web_search_result", "title": "Quantum Computing Breakthroughs in 2025", "url": "https://example.com"}]}}

// La risposta di Claude con citazioni (omessa in questo esempio)
```

## Richieste batch

Puoi includere lo strumento di ricerca web nell'[API Messages Batches](/docs/it/build-with-claude/batch-processing). Le chiamate dello strumento di ricerca web tramite l'API Messages Batches hanno lo stesso prezzo di quelle nelle richieste dell'API Messages regolari.

## Utilizzo e prezzi

Web search usage is charged in addition to token usage:

```json
"usage": {
  "input_tokens": 105,
  "output_tokens": 6039,
  "cache_read_input_tokens": 7123,
  "cache_creation_input_tokens": 7345,
  "server_tool_use": {
    "web_search_requests": 1
  }
}
```

Web search is available on the Claude API for **$10 per 1,000 searches**, plus standard token costs for search-generated content. Web search results retrieved throughout a conversation are counted as input tokens, in search iterations executed during a single turn and in subsequent conversation turns.

Each web search counts as one use, regardless of the number of results returned. If an error occurs during web search, the web search will not be billed.