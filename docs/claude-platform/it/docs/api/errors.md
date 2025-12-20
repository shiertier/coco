# Errori

---

## Errori HTTP

La nostra API segue un formato prevedibile di codici di errore HTTP:

* 400 - `invalid_request_error`: C'è stato un problema con il formato o il contenuto della tua richiesta. Potremmo anche utilizzare questo tipo di errore per altri codici di stato 4XX non elencati di seguito.
* 401 - `authentication_error`: C'è un problema con la tua chiave API.
* 403 - `permission_error`: La tua chiave API non ha il permesso di utilizzare la risorsa specificata.
* 404 - `not_found_error`: La risorsa richiesta non è stata trovata.
* 413 - `request_too_large`: La richiesta supera il numero massimo consentito di byte. La dimensione massima della richiesta è di 32 MB per gli endpoint API standard.
* 429 - `rate_limit_error`: Il tuo account ha raggiunto un limite di velocità.
* 500 - `api_error`: Si è verificato un errore imprevisto interno ai sistemi di Anthropic.
* 529 - `overloaded_error`: L'API è temporaneamente sovraccarica.

  <Warning>
  Gli errori 529 possono verificarsi quando le API sperimentano traffico elevato tra tutti gli utenti.
  
  In rari casi, se la tua organizzazione ha un forte aumento nell'utilizzo, potresti vedere errori 429 a causa dei limiti di accelerazione sull'API. Per evitare di raggiungere i limiti di accelerazione, aumenta gradualmente il tuo traffico e mantieni modelli di utilizzo coerenti.
  </Warning>

Quando si riceve una risposta in [streaming](/docs/it/build-with-claude/streaming) tramite SSE, è possibile che si verifichi un errore dopo aver restituito una risposta 200, nel qual caso la gestione degli errori non seguirebbe questi meccanismi standard.

## Limiti di dimensione delle richieste

L'API applica limiti di dimensione delle richieste per garantire prestazioni ottimali:

| Tipo di Endpoint | Dimensione Massima della Richiesta |
|:---|:---|
| API Messages | 32 MB |
| API Token Counting | 32 MB |
| [API Batch](/docs/it/build-with-claude/batch-processing) | 256 MB |
| [API Files](/docs/it/build-with-claude/files) | 500 MB |

Se superi questi limiti, riceverai un errore 413 `request_too_large`. L'errore viene restituito da Cloudflare prima che la richiesta raggiunga i nostri server API.

## Forme degli errori

Gli errori vengono sempre restituiti come JSON, con un oggetto `error` di livello superiore che include sempre un valore `type` e `message`. La risposta include anche un campo `request_id` per un tracciamento e debug più facili. Per esempio:

```json JSON
{
  "type": "error",
  "error": {
    "type": "not_found_error",
    "message": "The requested resource could not be found."
  },
  "request_id": "req_011CSHoEeqs5C35K2UUqR7Fy"
}
```

In conformità con la nostra politica di [versioning](/docs/it/api/versioning), potremmo espandere i valori all'interno di questi oggetti, ed è possibile che i valori `type` crescano nel tempo.

## ID della richiesta

Ogni risposta API include un header univoco `request-id`. Questo header contiene un valore come `req_018EeWyXxfu5pfWkrYcMdjWG`. Quando contatti il supporto riguardo a una richiesta specifica, includi questo ID per aiutarci a risolvere rapidamente il tuo problema.

I nostri SDK ufficiali forniscono questo valore come proprietà sugli oggetti di risposta di livello superiore, contenente il valore dell'header `request-id`:

<CodeGroup>
  ```python Python
  import anthropic

  client = anthropic.Anthropic()

  message = client.messages.create(
      model="claude-sonnet-4-5",
      max_tokens=1024,
      messages=[
          {"role": "user", "content": "Hello, Claude"}
      ]
  )
  print(f"Request ID: {message._request_id}")
  ```

  ```typescript TypeScript
  import Anthropic from '@anthropic-ai/sdk';

  const client = new Anthropic();

  const message = await client.messages.create({
    model: 'claude-sonnet-4-5',
    max_tokens: 1024,
    messages: [
      {"role": "user", "content": "Hello, Claude"}
    ]
  });
  console.log('Request ID:', message._request_id);
  ```
</CodeGroup>

## Richieste lunghe

<Warning>
 Incoraggiamo vivamente l'uso dell'[API Messages in streaming](/docs/it/build-with-claude/streaming) o dell'[API Message Batches](/docs/it/api/creating-message-batches) per richieste di lunga durata, specialmente quelle oltre i 10 minuti.
</Warning>

Non raccomandiamo di impostare valori `max_tokens` elevati senza utilizzare la nostra [API Messages in streaming](/docs/it/build-with-claude/streaming)
o l'[API Message Batches](/docs/it/api/creating-message-batches):

- Alcune reti potrebbero interrompere le connessioni inattive dopo un periodo di tempo variabile, il che
può causare il fallimento della richiesta o il timeout senza ricevere una risposta da Anthropic.
- Le reti differiscono in affidabilità; la nostra [API Message Batches](/docs/it/api/creating-message-batches) può aiutarti a
gestire il rischio di problemi di rete permettendoti di fare polling per i risultati piuttosto che richiedere una connessione di rete ininterrotta.

Se stai costruendo un'integrazione API diretta, dovresti essere consapevole che impostare un [TCP socket keep-alive](https://tldp.org/HOWTO/TCP-Keepalive-HOWTO/programming.html) può ridurre l'impatto dei timeout di connessione inattiva su alcune reti.

I nostri [SDK](/docs/it/api/client-sdks) valideranno che le tue richieste API Messages non in streaming non dovrebbero superare un timeout di 10 minuti e
imposteranno anche un'opzione socket per il TCP keep-alive.