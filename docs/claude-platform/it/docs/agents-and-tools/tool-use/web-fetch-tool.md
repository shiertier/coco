# Strumento di recupero web

Lo strumento di recupero web consente a Claude di recuperare il contenuto completo da pagine web e documenti PDF specificati.

---

Lo strumento di recupero web consente a Claude di recuperare il contenuto completo da pagine web e documenti PDF specificati.

<Note>
Lo strumento di recupero web è attualmente in versione beta. Per abilitarlo, utilizza l'intestazione beta `web-fetch-2025-09-10` nelle tue richieste API.

Utilizza [questo modulo](https://forms.gle/NhWcgmkcvPCMmPE86) per fornire feedback sulla qualità delle risposte del modello, sull'API stessa o sulla qualità della documentazione.
</Note>

<Warning>
L'abilitazione dello strumento di recupero web in ambienti in cui Claude elabora input non attendibili insieme a dati sensibili comporta rischi di esfiltrazione dei dati. Ti consigliamo di utilizzare questo strumento solo in ambienti attendibili o quando gestisci dati non sensibili.

Per ridurre al minimo i rischi di esfiltrazione, a Claude non è consentito costruire dinamicamente gli URL. Claude può recuperare solo gli URL che sono stati esplicitamente forniti dall'utente o che provengono da risultati di ricerca web o recupero web precedenti. Tuttavia, esiste ancora un rischio residuo che dovrebbe essere attentamente considerato quando si utilizza questo strumento.

Se l'esfiltrazione dei dati è una preoccupazione, considera:
- Disabilitare completamente lo strumento di recupero web
- Utilizzare il parametro `max_uses` per limitare il numero di richieste
- Utilizzare il parametro `allowed_domains` per limitare a domini noti e sicuri
</Warning>

## Modelli supportati

Il recupero web è disponibile su:

- Claude Sonnet 4.5 (`claude-sonnet-4-5-20250929`)
- Claude Sonnet 4 (`claude-sonnet-4-20250514`)
- Claude Sonnet 3.7 ([deprecato](/docs/it/about-claude/model-deprecations)) (`claude-3-7-sonnet-20250219`)
- Claude Haiku 4.5 (`claude-haiku-4-5-20251001`)
- Claude Haiku 3.5 ([deprecato](/docs/it/about-claude/model-deprecations)) (`claude-3-5-haiku-latest`)
- Claude Opus 4.5 (`claude-opus-4-5-20251101`)
- Claude Opus 4.1 (`claude-opus-4-1-20250805`)
- Claude Opus 4 (`claude-opus-4-20250514`)

## Come funziona il recupero web

Quando aggiungi lo strumento di recupero web alla tua richiesta API:

1. Claude decide quando recuperare il contenuto in base al prompt e agli URL disponibili.
2. L'API recupera il contenuto di testo completo dall'URL specificato.
3. Per i PDF, viene eseguita l'estrazione automatica del testo.
4. Claude analizza il contenuto recuperato e fornisce una risposta con citazioni opzionali.

<Note>
Lo strumento di recupero web attualmente non supporta siti web renderizzati dinamicamente tramite Javascript.
</Note>

## Come utilizzare il recupero web

Fornisci lo strumento di recupero web nella tua richiesta API:

<CodeGroup>
```bash Shell
curl https://api.anthropic.com/v1/messages \
    --header "x-api-key: $ANTHROPIC_API_KEY" \
    --header "anthropic-version: 2023-06-01" \
    --header "anthropic-beta: web-fetch-2025-09-10" \
    --header "content-type: application/json" \
    --data '{
        "model": "claude-sonnet-4-5",
        "max_tokens": 1024,
        "messages": [
            {
                "role": "user",
                "content": "Please analyze the content at https://example.com/article"
            }
        ],
        "tools": [{
            "type": "web_fetch_20250910",
            "name": "web_fetch",
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
            "content": "Please analyze the content at https://example.com/article"
        }
    ],
    tools=[{
        "type": "web_fetch_20250910",
        "name": "web_fetch",
        "max_uses": 5
    }],
    extra_headers={
        "anthropic-beta": "web-fetch-2025-09-10"
    }
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
        content: "Please analyze the content at https://example.com/article"
      }
    ],
    tools: [{
      type: "web_fetch_20250910",
      name: "web_fetch",
      max_uses: 5
    }],
    headers: {
      "anthropic-beta": "web-fetch-2025-09-10"
    }
  });

  console.log(response);
}

main().catch(console.error);
```
</CodeGroup>

### Definizione dello strumento

Lo strumento di recupero web supporta i seguenti parametri:

```json JSON
{
  "type": "web_fetch_20250910",
  "name": "web_fetch",

  // Optional: Limit the number of fetches per request
  "max_uses": 10,

  // Optional: Only fetch from these domains
  "allowed_domains": ["example.com", "docs.example.com"],

  // Optional: Never fetch from these domains
  "blocked_domains": ["private.example.com"],

  // Optional: Enable citations for fetched content
  "citations": {
    "enabled": true
  },

  // Optional: Maximum content length in tokens
  "max_content_tokens": 100000
}
```

#### Max uses

Il parametro `max_uses` limita il numero di recuperi web eseguiti. Se Claude tenta più recuperi di quelli consentiti, il `web_fetch_tool_result` sarà un errore con il codice di errore `max_uses_exceeded`. Attualmente non esiste un limite predefinito.

#### Filtro dei domini

Quando si utilizzano i filtri dei domini:

- I domini non devono includere lo schema HTTP/HTTPS (utilizza `example.com` invece di `https://example.com`)
- I sottodomini sono automaticamente inclusi (`example.com` copre `docs.example.com`)
- I sottopercorsi sono supportati (`example.com/blog`)
- Puoi utilizzare `allowed_domains` o `blocked_domains`, ma non entrambi nella stessa richiesta.

<Warning>
Tieni presente che i caratteri Unicode nei nomi di dominio possono creare vulnerabilità di sicurezza attraverso attacchi di omografi, in cui caratteri visivamente simili da script diversi possono aggirare i filtri dei domini. Ad esempio, `аmazon.com` (utilizzando la 'а' cirillica) potrebbe sembrare identico a `amazon.com` ma rappresenta un dominio diverso.

Quando configuri elenchi di consentimento/blocco dei domini:
- Utilizza nomi di dominio solo ASCII quando possibile
- Considera che i parser di URL possono gestire la normalizzazione Unicode diversamente
- Testa i tuoi filtri dei domini con potenziali variazioni di omografi
- Controlla regolarmente le tue configurazioni di dominio per caratteri Unicode sospetti
</Warning>

#### Limiti di contenuto

Il parametro `max_content_tokens` limita la quantità di contenuto che sarà incluso nel contesto. Se il contenuto recuperato supera questo limite, verrà troncato. Questo aiuta a controllare l'utilizzo dei token quando si recuperano documenti di grandi dimensioni.

<Note>
Il limite del parametro `max_content_tokens` è approssimativo. Il numero effettivo di token di input utilizzati può variare leggermente.
</Note>

#### Citazioni

A differenza della ricerca web in cui le citazioni sono sempre abilitate, le citazioni sono opzionali per il recupero web. Imposta `"citations": {"enabled": true}` per consentire a Claude di citare passaggi specifici dai documenti recuperati.

<Note>
Quando visualizzi gli output dell'API direttamente agli utenti finali, le citazioni devono essere incluse alla fonte originale. Se stai apportando modifiche agli output dell'API, incluso il rielaborazione e/o la combinazione con il tuo materiale prima di visualizzarli agli utenti finali, visualizza le citazioni come appropriato in base alla consultazione con il tuo team legale.
</Note>

### Risposta

Ecco una struttura di risposta di esempio:

```json
{
  "role": "assistant",
  "content": [
    // 1. Claude's decision to fetch
    {
      "type": "text",
      "text": "I'll fetch the content from the article to analyze it."
    },
    // 2. The fetch request
    {
      "type": "server_tool_use",
      "id": "srvtoolu_01234567890abcdef",
      "name": "web_fetch",
      "input": {
        "url": "https://example.com/article"
      }
    },
    // 3. Fetch results
    {
      "type": "web_fetch_tool_result",
      "tool_use_id": "srvtoolu_01234567890abcdef",
      "content": {
        "type": "web_fetch_result",
        "url": "https://example.com/article",
        "content": {
          "type": "document",
          "source": {
            "type": "text",
            "media_type": "text/plain",
            "data": "Full text content of the article..."
          },
          "title": "Article Title",
          "citations": {"enabled": true}
        },
        "retrieved_at": "2025-08-25T10:30:00Z"
      }
    },
    // 4. Claude's analysis with citations (if enabled)
    {
      "text": "Based on the article, ",
      "type": "text"
    },
    {
      "text": "the main argument presented is that artificial intelligence will transform healthcare",
      "type": "text",
      "citations": [
        {
          "type": "char_location",
          "document_index": 0,
          "document_title": "Article Title",
          "start_char_index": 1234,
          "end_char_index": 1456,
          "cited_text": "Artificial intelligence is poised to revolutionize healthcare delivery..."
        }
      ]
    }
  ],
  "id": "msg_a930390d3a",
  "usage": {
    "input_tokens": 25039,
    "output_tokens": 931,
    "server_tool_use": {
      "web_fetch_requests": 1
    }
  },
  "stop_reason": "end_turn"
}
```

#### Risultati del recupero

I risultati del recupero includono:

- `url`: L'URL che è stato recuperato
- `content`: Un blocco di documento contenente il contenuto recuperato
- `retrieved_at`: Timestamp di quando il contenuto è stato recuperato

<Note>
Lo strumento di recupero web memorizza nella cache i risultati per migliorare le prestazioni e ridurre le richieste ridondanti. Ciò significa che il contenuto restituito potrebbe non essere sempre la versione più recente disponibile all'URL. Il comportamento della cache è gestito automaticamente e potrebbe cambiare nel tempo per ottimizzare diversi tipi di contenuto e modelli di utilizzo.
</Note>

Per i documenti PDF, il contenuto sarà restituito come dati codificati in base64:

```json
{
  "type": "web_fetch_tool_result",
  "tool_use_id": "srvtoolu_02",
  "content": {
    "type": "web_fetch_result",
    "url": "https://example.com/paper.pdf",
    "content": {
      "type": "document",
      "source": {
        "type": "base64",
        "media_type": "application/pdf",
        "data": "JVBERi0xLjQKJcOkw7zDtsOfCjIgMCBvYmo..."
      },
      "citations": {"enabled": true}
    },
    "retrieved_at": "2025-08-25T10:30:02Z"
  }
}
```

#### Errori

Quando lo strumento di recupero web incontra un errore, l'API Claude restituisce una risposta 200 (successo) con l'errore rappresentato nel corpo della risposta:

```json
{
  "type": "web_fetch_tool_result",
  "tool_use_id": "srvtoolu_a93jad",
  "content": {
    "type": "web_fetch_tool_error",
    "error_code": "url_not_accessible"
  }
}
```

Questi sono i possibili codici di errore:

- `invalid_input`: Formato URL non valido
- `url_too_long`: L'URL supera la lunghezza massima (250 caratteri)
- `url_not_allowed`: URL bloccato dalle regole di filtro dei domini e dalle restrizioni del modello
- `url_not_accessible`: Impossibile recuperare il contenuto (errore HTTP)
- `too_many_requests`: Limite di velocità superato
- `unsupported_content_type`: Tipo di contenuto non supportato (solo testo e PDF)
- `max_uses_exceeded`: Utilizzi massimi dello strumento di recupero web superati
- `unavailable`: Si è verificato un errore interno

## Convalida dell'URL

Per motivi di sicurezza, lo strumento di recupero web può recuperare solo gli URL che sono stati precedentemente visualizzati nel contesto della conversazione. Questo include:

- URL nei messaggi dell'utente
- URL nei risultati degli strumenti lato client
- URL dai risultati di ricerca web o recupero web precedenti

Lo strumento non può recuperare URL arbitrari generati da Claude o URL da strumenti server basati su container (Code Execution, Bash, ecc.).

## Ricerca e recupero combinati

Il recupero web funziona perfettamente con la ricerca web per una raccolta di informazioni completa:

```python
import anthropic

client = anthropic.Anthropic()

response = client.messages.create(
    model="claude-sonnet-4-5",
    max_tokens=4096,
    messages=[
        {
            "role": "user",
            "content": "Find recent articles about quantum computing and analyze the most relevant one in detail"
        }
    ],
    tools=[
        {
            "type": "web_search_20250305",
            "name": "web_search",
            "max_uses": 3
        },
        {
            "type": "web_fetch_20250910",
            "name": "web_fetch",
            "max_uses": 5,
            "citations": {"enabled": True}
        }
    ],
    extra_headers={
        "anthropic-beta": "web-fetch-2025-09-10"
    }
)
```

In questo flusso di lavoro, Claude:
1. Utilizzerà la ricerca web per trovare articoli rilevanti
2. Selezionerà i risultati più promettenti
3. Utilizzerà il recupero web per recuperare il contenuto completo
4. Fornirà un'analisi dettagliata con citazioni

## Memorizzazione nella cache dei prompt

Il recupero web funziona con la [memorizzazione nella cache dei prompt](/docs/it/build-with-claude/prompt-caching). Per abilitare la memorizzazione nella cache dei prompt, aggiungi punti di interruzione `cache_control` nella tua richiesta. I risultati del recupero memorizzati nella cache possono essere riutilizzati nei turni di conversazione.

```python
import anthropic

client = anthropic.Anthropic()

# First request with web fetch
messages = [
    {
        "role": "user",
        "content": "Analyze this research paper: https://arxiv.org/abs/2024.12345"
    }
]

response1 = client.messages.create(
    model="claude-sonnet-4-5",
    max_tokens=1024,
    messages=messages,
    tools=[{
        "type": "web_fetch_20250910",
        "name": "web_fetch"
    }],
    extra_headers={
        "anthropic-beta": "web-fetch-2025-09-10"
    }
)

# Add Claude's response to conversation
messages.append({
    "role": "assistant",
    "content": response1.content
})

# Second request with cache breakpoint
messages.append({
    "role": "user",
    "content": "What methodology does the paper use?",
    "cache_control": {"type": "ephemeral"}
})

response2 = client.messages.create(
    model="claude-sonnet-4-5",
    max_tokens=1024,
    messages=messages,
    tools=[{
        "type": "web_fetch_20250910",
        "name": "web_fetch"
    }],
    extra_headers={
        "anthropic-beta": "web-fetch-2025-09-10"
    }
)

# The second response benefits from cached fetch results
print(f"Cache read tokens: {response2.usage.get('cache_read_input_tokens', 0)}")
```

## Streaming

Con lo streaming abilitato, gli eventi di recupero fanno parte dello stream con una pausa durante il recupero del contenuto:

```javascript
event: message_start
data: {"type": "message_start", "message": {"id": "msg_abc123", "type": "message"}}

event: content_block_start
data: {"type": "content_block_start", "index": 0, "content_block": {"type": "text", "text": ""}}

// Claude's decision to fetch

event: content_block_start
data: {"type": "content_block_start", "index": 1, "content_block": {"type": "server_tool_use", "id": "srvtoolu_xyz789", "name": "web_fetch"}}

// Fetch URL streamed
event: content_block_delta
data: {"type": "content_block_delta", "index": 1, "delta": {"type": "input_json_delta", "partial_json": "{\"url\":\"https://example.com/article\"}"}}

// Pause while fetch executes

// Fetch results streamed
event: content_block_start
data: {"type": "content_block_start", "index": 2, "content_block": {"type": "web_fetch_tool_result", "tool_use_id": "srvtoolu_xyz789", "content": {"type": "web_fetch_result", "url": "https://example.com/article", "content": {"type": "document", "source": {"type": "text", "media_type": "text/plain", "data": "Article content..."}}}}}

// Claude's response continues...
```

## Richieste batch

Puoi includere lo strumento di recupero web nell'[API Messages Batches](/docs/it/build-with-claude/batch-processing). Le chiamate dello strumento di recupero web tramite l'API Messages Batches hanno lo stesso prezzo di quelle nelle richieste dell'API Messages regolari.

## Utilizzo e prezzi

Web fetch usage has **no additional charges** beyond standard token costs:

```json
"usage": {
  "input_tokens": 25039,
  "output_tokens": 931,
  "cache_read_input_tokens": 0,
  "cache_creation_input_tokens": 0,
  "server_tool_use": {
    "web_fetch_requests": 1
  }
}
```

The web fetch tool is available on the Claude API at **no additional cost**. You only pay standard token costs for the fetched content that becomes part of your conversation context.

To protect against inadvertently fetching large content that would consume excessive tokens, use the `max_content_tokens` parameter to set appropriate limits based on your use case and budget considerations.

Example token usage for typical content:
- Average web page (10KB): ~2,500 tokens
- Large documentation page (100KB): ~25,000 tokens  
- Research paper PDF (500KB): ~125,000 tokens