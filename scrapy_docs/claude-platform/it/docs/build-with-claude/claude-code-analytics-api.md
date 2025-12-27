# Claude Code Analytics API

Accedi programmaticamente alle analitiche di utilizzo di Claude Code della tua organizzazione e alle metriche di produttività con l'API Admin di Claude Code Analytics.

---

<Tip>
**The Admin API is unavailable for individual accounts.** To collaborate with teammates and add members, set up your organization in **Console → Settings → Organization**.
</Tip>

L'API Admin di Claude Code Analytics fornisce accesso programmatico alle metriche di utilizzo giornaliere aggregate per gli utenti di Claude Code, consentendo alle organizzazioni di analizzare la produttività degli sviluppatori e creare dashboard personalizzate. Questa API colma il divario tra la nostra [dashboard Analytics](/claude-code) di base e l'integrazione OpenTelemetry complessa.

Questa API ti consente di monitorare, analizzare e ottimizzare meglio l'adozione di Claude Code:

* **Analisi della produttività degli sviluppatori:** Traccia sessioni, righe di codice aggiunte/rimosse, commit e pull request create utilizzando Claude Code
* **Metriche di utilizzo degli strumenti:** Monitora i tassi di accettazione e rifiuto per diversi strumenti di Claude Code (Edit, Write, NotebookEdit)
* **Analisi dei costi:** Visualizza i costi stimati e l'utilizzo dei token suddivisi per modello Claude
* **Reporting personalizzato:** Esporta i dati per creare dashboard esecutivi e report per i team di gestione
* **Giustificazione dell'utilizzo:** Fornisci metriche per giustificare ed espandere l'adozione di Claude Code internamente

<Check>
  **Chiave API Admin richiesta**
  
  Questa API fa parte dell'[Admin API](/docs/it/build-with-claude/administration-api). Questi endpoint richiedono una chiave API Admin (che inizia con `sk-ant-admin...`) che differisce dalle chiavi API standard. Solo i membri dell'organizzazione con il ruolo di amministratore possono fornire chiavi API Admin tramite la [Claude Console](/settings/admin-keys).
</Check>

## Avvio rapido

Ottieni le analitiche di Claude Code della tua organizzazione per un giorno specifico:

```bash
curl "https://api.anthropic.com/v1/organizations/usage_report/claude_code?\
starting_at=2025-09-08&\
limit=20" \
  --header "anthropic-version: 2023-06-01" \
  --header "x-api-key: $ADMIN_API_KEY"
```

<Tip>
  **Imposta un header User-Agent per le integrazioni**
  
  Se stai creando un'integrazione, imposta il tuo header User-Agent per aiutarci a comprendere i modelli di utilizzo:
  ```
  User-Agent: YourApp/1.0.0 (https://yourapp.com)
  ```
</Tip>

## Claude Code Analytics API

Traccia l'utilizzo di Claude Code, le metriche di produttività e l'attività degli sviluppatori in tutta la tua organizzazione con l'endpoint `/v1/organizations/usage_report/claude_code`.

### Concetti chiave

- **Aggregazione giornaliera**: Restituisce metriche per un singolo giorno specificato dal parametro `starting_at`
- **Dati a livello di utente**: Ogni record rappresenta l'attività di un utente per il giorno specificato
- **Metriche di produttività**: Traccia sessioni, righe di codice, commit, pull request e utilizzo degli strumenti
- **Dati di token e costi**: Monitora l'utilizzo e i costi stimati suddivisi per modello Claude
- **Paginazione basata su cursore**: Gestisci grandi set di dati con paginazione stabile utilizzando cursori opachi
- **Freschezza dei dati**: Le metriche sono disponibili con un ritardo fino a 1 ora per coerenza

Per i dettagli completi dei parametri e gli schemi di risposta, consulta il [riferimento dell'API Claude Code Analytics](/docs/it/api/admin-api/claude-code/get-claude-code-usage-report).

### Esempi di base

#### Ottieni analitiche per un giorno specifico

```bash
curl "https://api.anthropic.com/v1/organizations/usage_report/claude_code?\
starting_at=2025-09-08" \
  --header "anthropic-version: 2023-06-01" \
  --header "x-api-key: $ADMIN_API_KEY"
```

#### Ottieni analitiche con paginazione

```bash
# Prima richiesta
curl "https://api.anthropic.com/v1/organizations/usage_report/claude_code?\
starting_at=2025-09-08&\
limit=20" \
  --header "anthropic-version: 2023-06-01" \
  --header "x-api-key: $ADMIN_API_KEY"

# Richiesta successiva utilizzando il cursore dalla risposta
curl "https://api.anthropic.com/v1/organizations/usage_report/claude_code?\
starting_at=2025-09-08&\
page=page_MjAyNS0wNS0xNFQwMDowMDowMFo=" \
  --header "anthropic-version: 2023-06-01" \
  --header "x-api-key: $ADMIN_API_KEY"
```

### Parametri della richiesta

| Parametro | Tipo | Obbligatorio | Descrizione |
|-----------|------|----------|-------------|
| `starting_at` | string | Sì | Data UTC nel formato YYYY-MM-DD. Restituisce metriche solo per questo singolo giorno |
| `limit` | integer | No | Numero di record per pagina (predefinito: 20, massimo: 1000) |
| `page` | string | No | Token cursore opaco dal campo `next_page` della risposta precedente |

### Metriche disponibili

Ogni record di risposta contiene le seguenti metriche per un singolo utente in un singolo giorno:

#### Dimensioni
- **date**: Data nel formato RFC 3339 (timestamp UTC)
- **actor**: L'utente o la chiave API che ha eseguito le azioni di Claude Code (sia `user_actor` con `email_address` che `api_actor` con `api_key_name`)
- **organization_id**: UUID dell'organizzazione
- **customer_type**: Tipo di account cliente (`api` per clienti API, `subscription` per clienti Pro/Team)
- **terminal_type**: Tipo di terminale o ambiente in cui è stato utilizzato Claude Code (ad es. `vscode`, `iTerm.app`, `tmux`)

#### Metriche principali
- **num_sessions**: Numero di sessioni distinte di Claude Code avviate da questo attore
- **lines_of_code.added**: Numero totale di righe di codice aggiunte in tutti i file da Claude Code
- **lines_of_code.removed**: Numero totale di righe di codice rimosse in tutti i file da Claude Code
- **commits_by_claude_code**: Numero di commit git creati tramite la funzionalità di commit di Claude Code
- **pull_requests_by_claude_code**: Numero di pull request create tramite la funzionalità PR di Claude Code

#### Metriche delle azioni degli strumenti
Suddivisione dei tassi di accettazione e rifiuto delle azioni degli strumenti per tipo di strumento:
- **edit_tool.accepted/rejected**: Numero di proposte dello strumento Edit accettate/rifiutate dall'utente
- **write_tool.accepted/rejected**: Numero di proposte dello strumento Write accettate/rifiutate dall'utente
- **notebook_edit_tool.accepted/rejected**: Numero di proposte dello strumento NotebookEdit accettate/rifiutate dall'utente

#### Suddivisione per modello
Per ogni modello Claude utilizzato:
- **model**: Identificatore del modello Claude (ad es. `claude-sonnet-4-5-20250929`)
- **tokens.input/output**: Conteggi dei token di input e output per questo modello
- **tokens.cache_read/cache_creation**: Utilizzo dei token correlati alla cache per questo modello
- **estimated_cost.amount**: Costo stimato in centesimi USD per questo modello
- **estimated_cost.currency**: Codice valuta per l'importo del costo (attualmente sempre `USD`)

### Struttura della risposta

L'API restituisce i dati nel seguente formato:

```json
{
  "data": [
    {
      "date": "2025-09-01T00:00:00Z",
      "actor": {
        "type": "user_actor",
        "email_address": "developer@company.com"
      },
      "organization_id": "dc9f6c26-b22c-4831-8d01-0446bada88f1",
      "customer_type": "api",
      "terminal_type": "vscode",
      "core_metrics": {
        "num_sessions": 5,
        "lines_of_code": {
          "added": 1543,
          "removed": 892
        },
        "commits_by_claude_code": 12,
        "pull_requests_by_claude_code": 2
      },
      "tool_actions": {
        "edit_tool": {
          "accepted": 45,
          "rejected": 5
        },
        "multi_edit_tool": {
          "accepted": 12,
          "rejected": 2
        },
        "write_tool": {
          "accepted": 8,
          "rejected": 1
        },
        "notebook_edit_tool": {
          "accepted": 3,
          "rejected": 0
        }
      },
      "model_breakdown": [
        {
          "model": "claude-sonnet-4-5-20250929",
          "tokens": {
            "input": 100000,
            "output": 35000,
            "cache_read": 10000,
            "cache_creation": 5000
          },
          "estimated_cost": {
            "currency": "USD",
            "amount": 1025
          }
        }
      ]
    }
  ],
  "has_more": false,
  "next_page": null
}
```

## Paginazione

L'API supporta la paginazione basata su cursore per le organizzazioni con un gran numero di utenti:

1. Effettua la tua richiesta iniziale con il parametro `limit` opzionale
2. Se `has_more` è `true` nella risposta, utilizza il valore `next_page` nella tua prossima richiesta
3. Continua fino a quando `has_more` non è `false`

Il cursore codifica la posizione dell'ultimo record e garantisce una paginazione stabile anche quando arrivano nuovi dati. Ogni sessione di paginazione mantiene un confine dati coerente per assicurarsi di non perdere o duplicare record.

## Casi d'uso comuni

- **Dashboard esecutivi**: Crea report di alto livello che mostrano l'impatto di Claude Code sulla velocità di sviluppo
- **Confronto degli strumenti AI**: Esporta metriche per confrontare Claude Code con altri strumenti di codifica AI come Copilot e Cursor
- **Analisi della produttività degli sviluppatori**: Traccia le metriche di produttività individuali e di team nel tempo
- **Tracciamento e allocazione dei costi**: Monitora i modelli di spesa e alloca i costi per team o progetto
- **Monitoraggio dell'adozione**: Identifica quali team e utenti traggono il massimo valore da Claude Code
- **Giustificazione del ROI**: Fornisci metriche concrete per giustificare ed espandere l'adozione di Claude Code internamente

## Domande frequenti

### Quanto sono freschi i dati delle analitiche?
I dati delle analitiche di Claude Code in genere appaiono entro 1 ora dal completamento dell'attività dell'utente. Per garantire risultati di paginazione coerenti, solo i dati più vecchi di 1 ora sono inclusi nelle risposte.

### Posso ottenere metriche in tempo reale?
No, questa API fornisce solo metriche giornaliere aggregate. Per il monitoraggio in tempo reale, considera l'utilizzo dell'[integrazione OpenTelemetry](https://code.claude.com/docs/en/monitoring-usage).

### Come vengono identificati gli utenti nei dati?
Gli utenti vengono identificati tramite il campo `actor` in due modi:
- **`user_actor`**: Contiene `email_address` per gli utenti che si autenticano tramite OAuth (più comune)
- **`api_actor`**: Contiene `api_key_name` per gli utenti che si autenticano tramite chiave API

Il campo `customer_type` indica se l'utilizzo proviene da clienti `api` (API PAYG) o clienti `subscription` (piani Pro/Team).

### Qual è il periodo di conservazione dei dati?
I dati storici delle analitiche di Claude Code vengono conservati e sono accessibili tramite l'API. Non esiste un periodo di eliminazione specificato per questi dati.

### Quali distribuzioni di Claude Code sono supportate?
Questa API traccia solo l'utilizzo di Claude Code su Claude API (1° parte). L'utilizzo su Amazon Bedrock, Google Vertex AI o altre piattaforme di terze parti non è incluso.

### Quanto costa utilizzare questa API?
L'API Claude Code Analytics è gratuita per tutte le organizzazioni con accesso all'Admin API.

### Come calcolo i tassi di accettazione degli strumenti?
Tasso di accettazione dello strumento = `accepted / (accepted + rejected)` per ogni tipo di strumento. Ad esempio, se lo strumento di modifica mostra 45 accettati e 5 rifiutati, il tasso di accettazione è del 90%.

### Quale fuso orario viene utilizzato per il parametro della data?
Tutte le date sono in UTC. Il parametro `starting_at` deve essere nel formato YYYY-MM-DD e rappresenta la mezzanotte UTC per quel giorno.

## Vedi anche

L'API Claude Code Analytics ti aiuta a comprendere e ottimizzare il flusso di lavoro di sviluppo del tuo team. Scopri di più sulle funzionalità correlate:

- [Panoramica dell'Admin API](/docs/it/build-with-claude/administration-api)
- [Riferimento dell'Admin API](/docs/it/api/admin)
- [Dashboard Claude Code Analytics](/claude-code)
- [API di utilizzo e costi](/docs/it/build-with-claude/usage-cost-api) - Traccia l'utilizzo dell'API in tutti i servizi Anthropic
- [Gestione dell'identità e dell'accesso](https://code.claude.com/docs/en/iam)
- [Monitoraggio dell'utilizzo con OpenTelemetry](https://code.claude.com/docs/en/monitoring-usage) per metriche personalizzate e avvisi