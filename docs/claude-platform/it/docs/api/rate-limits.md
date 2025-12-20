# Limiti di velocità

Per mitigare gli abusi e gestire la capacità della nostra API, abbiamo implementato limiti su quanto un'organizzazione può utilizzare l'API Claude.

---

Abbiamo due tipi di limiti:

1. **Limiti di spesa** impostano un costo mensile massimo che un'organizzazione può sostenere per l'utilizzo dell'API.
2. **Limiti di velocità** impostano il numero massimo di richieste API che un'organizzazione può effettuare in un periodo di tempo definito.

Applichiamo i limiti configurati dal servizio a livello di organizzazione, ma puoi anche impostare limiti configurabili dall'utente per i workspace della tua organizzazione.

Questi limiti si applicano sia all'utilizzo del Tier Standard che del Tier Priority. Per ulteriori informazioni su Priority Tier, che offre livelli di servizio migliorati in cambio di spesa impegnata, vedi [Service Tiers](/docs/it/api/service-tiers).

## Informazioni sui nostri limiti

* I limiti sono progettati per prevenire gli abusi dell'API, riducendo al minimo l'impatto sui comuni modelli di utilizzo dei clienti.
* I limiti sono definiti per **tier di utilizzo**, dove ogni tier è associato a un diverso insieme di limiti di spesa e velocità.
* La tua organizzazione aumenterà automaticamente i tier man mano che raggiungi determinati soglie durante l'utilizzo dell'API.
  I limiti sono impostati a livello di organizzazione. Puoi vedere i limiti della tua organizzazione nella [pagina Limiti](/settings/limits) nella [Claude Console](/).
* Potresti raggiungere i limiti di velocità su intervalli di tempo più brevi. Ad esempio, una velocità di 60 richieste al minuto (RPM) potrebbe essere applicata come 1 richiesta al secondo. Brevi raffiche di richieste ad alto volume possono superare il limite di velocità e causare errori di limite di velocità.
* I limiti descritti di seguito sono i nostri limiti del tier standard. Se stai cercando limiti più alti e personalizzati o Priority Tier per livelli di servizio migliorati, contatta il team di vendita tramite la [Claude Console](/settings/limits).
* Utilizziamo l'[algoritmo token bucket](https://en.wikipedia.org/wiki/Token_bucket) per fare rate limiting. Ciò significa che la tua capacità viene continuamente reintegrata fino al tuo limite massimo, piuttosto che essere ripristinata a intervalli fissi.
* Tutti i limiti descritti qui rappresentano l'utilizzo massimo consentito, non i minimi garantiti. Questi limiti sono destinati a ridurre la spesa eccessiva involontaria e garantire una distribuzione equa delle risorse tra gli utenti.

## Limiti di spesa

Ogni tier di utilizzo ha un limite su quanto puoi spendere per l'API ogni mese di calendario. Una volta raggiunto il limite di spesa del tuo tier, fino a quando non ti qualifichi per il tier successivo, dovrai aspettare fino al mese successivo per poter utilizzare di nuovo l'API.

Per qualificarti per il tier successivo, devi soddisfare un requisito di deposito. Per ridurre al minimo il rischio di sovrafinanziamento del tuo account, non puoi depositare più del tuo limite di spesa mensile.

### Requisiti per avanzare di tier
<table>
  <thead>
    <tr>
      <th>Tier di utilizzo</th>
      <th>Acquisto di crediti</th>
      <th>Acquisto massimo di crediti</th>
    </tr>
  </thead>
  <tbody>
    <tr>
      <td>Tier 1</td>
      <td>\$5</td>
      <td>\$100</td>
    </tr>
    <tr>
      <td>Tier 2</td>
      <td>\$40</td>
      <td>\$500</td>
    </tr>
    <tr>
      <td>Tier 3</td>
      <td>\$200</td>
      <td>\$1,000</td>
    </tr>
    <tr>
      <td>Tier 4</td>
      <td>\$400</td>
      <td>\$5,000</td>
    </tr>
    <tr>
      <td>Fatturazione mensile</td>
      <td>N/A</td>
      <td>N/A</td>
    </tr>
  </tbody>
</table>

<Note>
**Acquisto di crediti** mostra gli acquisti di crediti cumulativi (escluse le tasse) richiesti per avanzare a quel tier. Avanzi immediatamente al raggiungimento della soglia.

**Acquisto massimo di crediti** limita l'importo massimo che puoi aggiungere al tuo account in una singola transazione per prevenire il sovrafinanziamento dell'account.
</Note>

## Limiti di velocità

I nostri limiti di velocità per l'API Messages sono misurati in richieste al minuto (RPM), token di input al minuto (ITPM) e token di output al minuto (OTPM) per ogni classe di modello.
Se superi uno qualsiasi dei limiti di velocità, riceverai un [errore 429](/docs/it/api/errors) che descrive quale limite di velocità è stato superato, insieme a un'intestazione `retry-after` che indica quanto tempo aspettare.

<Note>
Potresti anche incontrare errori 429 a causa di limiti di accelerazione sull'API se la tua organizzazione ha un aumento netto dell'utilizzo. Per evitare di raggiungere i limiti di accelerazione, aumenta il tuo traffico gradualmente e mantieni modelli di utilizzo coerenti.
</Note>

### ITPM consapevole della cache

Molti provider di API utilizzano un limite combinato di "token al minuto" (TPM) che potrebbe includere tutti i token, sia quelli memorizzati in cache che quelli non memorizzati, input e output. **Per la maggior parte dei modelli Claude, solo i token di input non memorizzati in cache contano verso i tuoi limiti di velocità ITPM.** Questo è un vantaggio chiave che rende i nostri limiti di velocità effettivamente più alti di quanto potrebbero sembrare inizialmente.

I limiti di velocità ITPM sono stimati all'inizio di ogni richiesta e la stima viene regolata durante la richiesta per riflettere il numero effettivo di token di input utilizzati.

Ecco cosa conta verso ITPM:
- `input_tokens` (token dopo l'ultimo punto di interruzione della cache) ✓ **Contano verso ITPM**
- `cache_creation_input_tokens` (token scritti nella cache) ✓ **Contano verso ITPM**
- `cache_read_input_tokens` (token letti dalla cache) ✗ **NON contano verso ITPM** per la maggior parte dei modelli

<Note>
Il campo `input_tokens` rappresenta solo i token che appaiono **dopo l'ultimo punto di interruzione della cache**, non tutti i token di input nella tua richiesta. Per calcolare il totale dei token di input:

```
total_input_tokens = cache_read_input_tokens + cache_creation_input_tokens + input_tokens
```

Ciò significa che quando hai contenuto memorizzato in cache, `input_tokens` sarà tipicamente molto più piccolo del tuo input totale. Ad esempio, con un documento memorizzato in cache di 200K token e una domanda dell'utente di 50 token, vedrai `input_tokens: 50` anche se l'input totale è di 200.050 token.

Per scopi di limite di velocità sulla maggior parte dei modelli, solo `input_tokens` + `cache_creation_input_tokens` contano verso il tuo limite ITPM, rendendo [prompt caching](/docs/it/build-with-claude/prompt-caching) un modo efficace per aumentare il tuo throughput effettivo.
</Note>

**Esempio**: Con un limite ITPM di 2.000.000 e un tasso di hit della cache dell'80%, potresti elaborare efficacemente 10.000.000 token di input totali al minuto (2M non memorizzati in cache + 8M memorizzati in cache), poiché i token memorizzati in cache non contano verso il tuo limite di velocità.

<Note>
Alcuni modelli più vecchi (contrassegnati con † nelle tabelle dei limiti di velocità di seguito) contano anche `cache_read_input_tokens` verso i limiti di velocità ITPM.

Per tutti i modelli senza il marcatore †, i token di input memorizzati in cache non contano verso i limiti di velocità e vengono fatturati a una tariffa ridotta (10% del prezzo del token di input di base). Ciò significa che puoi ottenere un throughput effettivo significativamente più alto utilizzando [prompt caching](/docs/it/build-with-claude/prompt-caching).
</Note>

<Tip>
**Massimizza i tuoi limiti di velocità con prompt caching**

Per ottenere il massimo dai tuoi limiti di velocità, utilizza [prompt caching](/docs/it/build-with-claude/prompt-caching) per contenuti ripetuti come:
- Istruzioni di sistema e prompt
- Documenti di contesto di grandi dimensioni
- Definizioni di strumenti
- Cronologia della conversazione

Con caching efficace, puoi aumentare drammaticamente il tuo throughput effettivo senza aumentare i tuoi limiti di velocità. Monitora il tuo tasso di hit della cache nella [pagina Utilizzo](/settings/usage) per ottimizzare la tua strategia di caching.
</Tip>

I limiti di velocità OTPM sono stimati in base a `max_tokens` all'inizio di ogni richiesta e la stima viene regolata alla fine della richiesta per riflettere il numero effettivo di token di output utilizzati.
Se stai raggiungendo i limiti OTPM prima del previsto, prova a ridurre `max_tokens` per approssimare meglio la dimensione dei tuoi completamenti.

I limiti di velocità vengono applicati separatamente per ogni modello; pertanto puoi utilizzare modelli diversi fino ai rispettivi limiti contemporaneamente.
Puoi controllare i tuoi limiti di velocità attuali e il comportamento nella [Claude Console](/settings/limits).

<Note>
Per richieste di contesto lungo (>200K token) quando si utilizza l'intestazione beta `context-1m-2025-08-07` con Claude Sonnet 4.x, si applicano limiti di velocità separati. Vedi [Limiti di velocità per contesto lungo](#long-context-rate-limits) di seguito.
</Note>

<Tabs>
<Tab title="Tier 1">
| Modello                                                                                      | Richieste massime al minuto (RPM) | Token di input massimi al minuto (ITPM) | Token di output massimi al minuto (OTPM) |
| -------------------------------------------------------------------------------------------- | --------------------------------- | -------------------------------------- | --------------------------------------- |
| Claude Sonnet 4.x<sup>**</sup>                                                               | 50                                | 30.000                                 | 8.000                                   |
| Claude Sonnet 3.7 ([deprecato](/docs/it/about-claude/model-deprecations))                    | 50                                | 20.000                                 | 8.000                                   |
| Claude Haiku 4.5                                                                             | 50                                | 50.000                                 | 10.000                                  |
| Claude Haiku 3.5 ([deprecato](/docs/it/about-claude/model-deprecations))                     | 50                                | 50.000<sup>†</sup>                     | 10.000                                  |
| Claude Haiku 3                                                                               | 50                                | 50.000<sup>†</sup>                     | 10.000                                  |
| Claude Opus 4.x<sup>*</sup>                                                                  | 50                                | 30.000                                 | 8.000                                   |
| Claude Opus 3 ([deprecato](/docs/it/about-claude/model-deprecations))                       | 50                                | 20.000<sup>†</sup>                     | 4.000                                   |

</Tab>
<Tab title="Tier 2">
| Modello                                                                                      | Richieste massime al minuto (RPM) | Token di input massimi al minuto (ITPM) | Token di output massimi al minuto (OTPM) |
| -------------------------------------------------------------------------------------------- | --------------------------------- | -------------------------------------- | --------------------------------------- |
| Claude Sonnet 4.x<sup>**</sup>                                                               | 1.000                             | 450.000                                | 90.000                                  |
| Claude Sonnet 3.7 ([deprecato](/docs/it/about-claude/model-deprecations))                    | 1.000                             | 40.000                                 | 16.000                                  |
| Claude Haiku 4.5                                                                             | 1.000                             | 450.000                                | 90.000                                  |
| Claude Haiku 3.5 ([deprecato](/docs/it/about-claude/model-deprecations))                     | 1.000                             | 100.000<sup>†</sup>                    | 20.000                                  |
| Claude Haiku 3                                                                               | 1.000                             | 100.000<sup>†</sup>                    | 20.000                                  |
| Claude Opus 4.x<sup>*</sup>                                                                  | 1.000                             | 450.000                                | 90.000                                  |
| Claude Opus 3 ([deprecato](/docs/it/about-claude/model-deprecations))                       | 1.000                             | 40.000<sup>†</sup>                     | 8.000                                   |

</Tab>
<Tab title="Tier 3">
| Modello                                                                                      | Richieste massime al minuto (RPM) | Token di input massimi al minuto (ITPM) | Token di output massimi al minuto (OTPM) |
| -------------------------------------------------------------------------------------------- | --------------------------------- | -------------------------------------- | --------------------------------------- |
| Claude Sonnet 4.x<sup>**</sup>                                                               | 2.000                             | 800.000                                | 160.000                                 |
| Claude Sonnet 3.7 ([deprecato](/docs/it/about-claude/model-deprecations))                    | 2.000                             | 80.000                                 | 32.000                                  |
| Claude Haiku 4.5                                                                             | 2.000                             | 1.000.000                              | 200.000                                 |
| Claude Haiku 3.5 ([deprecato](/docs/it/about-claude/model-deprecations))                     | 2.000                             | 200.000<sup>†</sup>                    | 40.000                                  |
| Claude Haiku 3                                                                               | 2.000                             | 200.000<sup>†</sup>                    | 40.000                                  |
| Claude Opus 4.x<sup>*</sup>                                                                  | 2.000                             | 800.000                                | 160.000                                 |
| Claude Opus 3 ([deprecato](/docs/it/about-claude/model-deprecations))                       | 2.000                             | 80.000<sup>†</sup>                     | 16.000                                  |

</Tab>
<Tab title="Tier 4">
| Modello                                                                                      | Richieste massime al minuto (RPM) | Token di input massimi al minuto (ITPM) | Token di output massimi al minuto (OTPM) |
| -------------------------------------------------------------------------------------------- | --------------------------------- | -------------------------------------- | --------------------------------------- |
| Claude Sonnet 4.x<sup>**</sup>                                                               | 4.000                             | 2.000.000                              | 400.000                                 |
| Claude Sonnet 3.7 ([deprecato](/docs/it/about-claude/model-deprecations))                    | 4.000                             | 200.000                                | 80.000                                  |
| Claude Haiku 4.5                                                                             | 4.000                             | 4.000.000                              | 800.000                                 |
| Claude Haiku 3.5 ([deprecato](/docs/it/about-claude/model-deprecations))                     | 4.000                             | 400.000<sup>†</sup>                    | 80.000                                  |
| Claude Haiku 3                                                                               | 4.000                             | 400.000<sup>†</sup>                    | 80.000                                  |
| Claude Opus 4.x<sup>*</sup>                                                                  | 4.000                             | 2.000.000                              | 400.000                                 |
| Claude Opus 3 ([deprecato](/docs/it/about-claude/model-deprecations))                       | 4.000                             | 400.000<sup>†</sup>                    | 80.000                                  |

</Tab>
<Tab title="Personalizzato">
Se stai cercando limiti più alti per un caso d'uso Enterprise, contatta il team di vendita tramite la [Claude Console](/settings/limits).
</Tab>
</Tabs>

_<sup>* - Il limite di velocità Opus 4.x è un limite totale che si applica al traffico combinato tra Opus 4, Opus 4.1 e Opus 4.5.</sup>_

_<sup>** - Il limite di velocità Sonnet 4.x è un limite totale che si applica al traffico combinato tra Sonnet 4 e Sonnet 4.5.</sup>_

_<sup>† - Il limite conta `cache_read_input_tokens` verso l'utilizzo ITPM.</sup>_

### API Message Batches

L'API Message Batches ha il suo proprio insieme di limiti di velocità che sono condivisi tra tutti i modelli. Questi includono un limite di richieste al minuto (RPM) a tutti gli endpoint API e un limite sul numero di richieste batch che possono trovarsi nella coda di elaborazione contemporaneamente. Una "richiesta batch" qui si riferisce a parte di un Message Batch. Puoi creare un Message Batch contenente migliaia di richieste batch, ognuna delle quali conta verso questo limite. Una richiesta batch è considerata parte della coda di elaborazione quando deve ancora essere elaborata con successo dal modello.

<Tabs>
<Tab title="Tier 1">
| Richieste massime al minuto (RPM) | Richieste batch massime nella coda di elaborazione | Richieste batch massime per batch |
| --------------------------------- | ------------------------------------------ | -------------------------------- |
| 50                                | 100.000                                    | 100.000                          |
</Tab>
<Tab title="Tier 2">
| Richieste massime al minuto (RPM) | Richieste batch massime nella coda di elaborazione | Richieste batch massime per batch |
| --------------------------------- | ------------------------------------------ | -------------------------------- |
| 1.000                             | 200.000                                    | 100.000                          |
</Tab>
<Tab title="Tier 3">
| Richieste massime al minuto (RPM) | Richieste batch massime nella coda di elaborazione | Richieste batch massime per batch |
| --------------------------------- | ------------------------------------------ | -------------------------------- |
| 2.000                             | 300.000                                    | 100.000                          |
</Tab>
<Tab title="Tier 4">
| Richieste massime al minuto (RPM) | Richieste batch massime nella coda di elaborazione | Richieste batch massime per batch |
| --------------------------------- | ------------------------------------------ | -------------------------------- |
| 4.000                             | 500.000                                    | 100.000                          |
</Tab>
<Tab title="Personalizzato">
Se stai cercando limiti più alti per un caso d'uso Enterprise, contatta il team di vendita tramite la [Claude Console](/settings/limits).
</Tab>
</Tabs>

### Limiti di velocità per contesto lungo

Quando si utilizza Claude Sonnet 4 e Sonnet 4.5 con la [finestra di contesto di 1M token abilitata](/docs/it/build-with-claude/context-windows#1m-token-context-window), i seguenti limiti di velocità dedicati si applicano alle richieste che superano 200K token.

<Note>
La finestra di contesto di 1M token è attualmente in beta per le organizzazioni nel tier di utilizzo 4 e le organizzazioni con limiti di velocità personalizzati. La finestra di contesto di 1M token è disponibile solo per Claude Sonnet 4 e Sonnet 4.5.
</Note>

<Tabs>
<Tab title="Tier 4">
| Token di input massimi al minuto (ITPM) | Token di output massimi al minuto (OTPM) |
| -------------------------------------- | --------------------------------------- |
| 1.000.000                              | 200.000                                 |
</Tab>
<Tab title="Personalizzato">
Per limiti di velocità personalizzati per contesto lungo per casi d'uso enterprise, contatta il team di vendita tramite la [Claude Console](/settings/limits).
</Tab>
</Tabs>

<Tip>
Per ottenere il massimo dalla finestra di contesto di 1M token con limiti di velocità, utilizza [prompt caching](/docs/it/build-with-claude/prompt-caching).
</Tip>

### Monitoraggio dei tuoi limiti di velocità nella Console

Puoi monitorare l'utilizzo dei tuoi limiti di velocità nella pagina [Utilizzo](/settings/usage) della [Claude Console](/). 

Oltre a fornire grafici di token e richieste, la pagina Utilizzo fornisce due grafici separati dei limiti di velocità. Utilizza questi grafici per vedere quanto spazio hai per crescere, quando potresti raggiungere il picco di utilizzo, comprendere meglio quali limiti di velocità richiedere, o come puoi migliorare i tuoi tassi di caching. I grafici visualizzano una serie di metriche per un determinato limite di velocità (ad es. per modello):

- Il grafico **Rate Limit - Input Tokens** include:
  - Massimo orario di token di input non memorizzati in cache al minuto
  - Il tuo attuale limite di velocità di token di input al minuto
  - Il tasso di cache per i tuoi token di input (cioè la percentuale di token di input letti dalla cache)
- Il grafico **Rate Limit - Output Tokens** include:
  - Massimo orario di token di output al minuto
  - Il tuo attuale limite di velocità di token di output al minuto

## Impostazione di limiti inferiori per i Workspace

Per proteggere i Workspace nella tua Organizzazione da un potenziale utilizzo eccessivo, puoi impostare limiti di spesa e velocità personalizzati per Workspace.

Esempio: Se il limite della tua Organizzazione è 40.000 token di input al minuto e 8.000 token di output al minuto, potresti limitare un Workspace a 30.000 token totali al minuto. Questo protegge gli altri Workspace da un potenziale utilizzo eccessivo e garantisce una distribuzione più equa delle risorse tra la tua Organizzazione. I token al minuto inutilizzati rimanenti (o più, se quel Workspace non utilizza il limite) sono quindi disponibili per altri Workspace da utilizzare.

Nota:
- Non puoi impostare limiti sul Workspace predefinito.
- Se non impostato, i limiti del Workspace corrispondono al limite dell'Organizzazione.
- I limiti a livello di organizzazione si applicano sempre, anche se i limiti del Workspace si sommano a più.
- Il supporto per i limiti di token di input e output verrà aggiunto ai Workspace in futuro.

## Intestazioni di risposta

La risposta API include intestazioni che ti mostrano il limite di velocità applicato, l'utilizzo attuale e quando il limite verrà ripristinato.

Le seguenti intestazioni vengono restituite:

| Intestazione                                  | Descrizione                                                                                                                                     |
| --------------------------------------------- | ----------------------------------------------------------------------------------------------------------------------------------------------- |
| `retry-after`                                 | Il numero di secondi da aspettare prima di poter riprovare la richiesta. I tentativi precedenti falliranno.                                    |
| `anthropic-ratelimit-requests-limit`          | Il numero massimo di richieste consentite entro qualsiasi periodo di limite di velocità.                                                       |
| `anthropic-ratelimit-requests-remaining`      | Il numero di richieste rimanenti prima di essere limitato dalla velocità.                                                                      |
| `anthropic-ratelimit-requests-reset`          | L'ora in cui il limite di velocità della richiesta sarà completamente reintegrato, fornito in formato RFC 3339.                                |
| `anthropic-ratelimit-tokens-limit`            | Il numero massimo di token consentiti entro qualsiasi periodo di limite di velocità.                                                           |
| `anthropic-ratelimit-tokens-remaining`        | Il numero di token rimanenti (arrotondato al migliaio più vicino) prima di essere limitato dalla velocità.                                     |
| `anthropic-ratelimit-tokens-reset`            | L'ora in cui il limite di velocità del token sarà completamente reintegrato, fornito in formato RFC 3339.                                      |
| `anthropic-ratelimit-input-tokens-limit`      | Il numero massimo di token di input consentiti entro qualsiasi periodo di limite di velocità.                                                  |
| `anthropic-ratelimit-input-tokens-remaining`  | Il numero di token di input rimanenti (arrotondato al migliaio più vicino) prima di essere limitato dalla velocità.                            |
| `anthropic-ratelimit-input-tokens-reset`      | L'ora in cui il limite di velocità del token di input sarà completamente reintegrato, fornito in formato RFC 3339.                             |
| `anthropic-ratelimit-output-tokens-limit`     | Il numero massimo di token di output consentiti entro qualsiasi periodo di limite di velocità.                                                 |
| `anthropic-ratelimit-output-tokens-remaining` | Il numero di token di output rimanenti (arrotondato al migliaio più vicino) prima di essere limitato dalla velocità.                           |
| `anthropic-ratelimit-output-tokens-reset`     | L'ora in cui il limite di velocità del token di output sarà completamente reintegrato, fornito in formato RFC 3339.                            |
| `anthropic-priority-input-tokens-limit`       | Il numero massimo di token di input Priority Tier consentiti entro qualsiasi periodo di limite di velocità. (Solo Priority Tier)               |
| `anthropic-priority-input-tokens-remaining`   | Il numero di token di input Priority Tier rimanenti (arrotondato al migliaio più vicino) prima di essere limitato dalla velocità. (Solo Priority Tier) |
| `anthropic-priority-input-tokens-reset`       | L'ora in cui il limite di velocità del token di input Priority Tier sarà completamente reintegrato, fornito in formato RFC 3339. (Solo Priority Tier) |
| `anthropic-priority-output-tokens-limit`      | Il numero massimo di token di output Priority Tier consentiti entro qualsiasi periodo di limite di velocità. (Solo Priority Tier)              |
| `anthropic-priority-output-tokens-remaining`  | Il numero di token di output Priority Tier rimanenti (arrotondato al migliaio più vicino) prima di essere limitato dalla velocità. (Solo Priority Tier) |
| `anthropic-priority-output-tokens-reset`      | L'ora in cui il limite di velocità del token di output Priority Tier sarà completamente reintegrato, fornito in formato RFC 3339. (Solo Priority Tier) |

Le intestazioni `anthropic-ratelimit-tokens-*` visualizzano i valori per il limite più restrittivo attualmente in vigore. Ad esempio, se hai superato il limite di token al minuto per Workspace, le intestazioni conterranno i valori del limite di velocità di token al minuto per Workspace. Se i limiti del Workspace non si applicano, le intestazioni restituiranno il totale dei token rimanenti, dove il totale è la somma dei token di input e output. Questo approccio garantisce che tu abbia visibilità nel vincolo più rilevante sul tuo utilizzo attuale dell'API.