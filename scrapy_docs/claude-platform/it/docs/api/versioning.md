# Versioni

Quando effettui richieste API, devi inviare un header di richiesta `anthropic-version`. Ad esempio, `anthropic-version: 2023-06-01`. Se stai utilizzando i nostri [SDK client](/docs/it/api/client-sdks), questo viene gestito automaticamente per te.

---

Per qualsiasi versione API data, preserveremo:

* Parametri di input esistenti
* Parametri di output esistenti

Tuttavia, potremmo fare quanto segue:

* Aggiungere input opzionali aggiuntivi
* Aggiungere valori aggiuntivi all'output
* Modificare le condizioni per tipi di errore specifici
* Aggiungere nuove varianti ai valori di output simili a enum (ad esempio, tipi di eventi di streaming)

In generale, se stai utilizzando l'API come documentato in questo riferimento, non interromperemo il tuo utilizzo.

## Cronologia delle versioni

Raccomandiamo sempre di utilizzare l'ultima versione API quando possibile. Le versioni precedenti sono considerate deprecate e potrebbero non essere disponibili per i nuovi utenti.

* `2023-06-01`  
   * Nuovo formato per gli eventi server-sent (SSE) di [streaming](/docs/it/api/streaming):  
         * I completamenti sono incrementali. Ad esempio, `" Ciao"`, `" il"`, `" mio"`, `" nome"`, `" è"`, `" Claude."` invece di `" Ciao"`, `" Ciao il"`, `" Ciao il mio"`, `" Ciao il mio nome"`, `" Ciao il mio nome è"`, `" Ciao il mio nome è Claude."`.  
         * Tutti gli eventi sono [eventi nominati](https://developer.mozilla.org/en-US/Web/API/Server-sent%5Fevents/Using%5Fserver-sent%5Fevents#named%5Fevents), piuttosto che [eventi solo dati](https://developer.mozilla.org/en-US/Web/API/Server-sent%5Fevents/Using%5Fserver-sent%5Fevents#data-only%5Fmessages).  
         * Rimosso l'evento `data: [DONE]` non necessario.  
   * Rimossi i valori legacy `exception` e `truncated` nelle risposte.
* `2023-01-01`: Rilascio iniziale.