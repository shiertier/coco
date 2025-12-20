# Piattaforma Claude Developer

Aggiornamenti alla Piattaforma Claude Developer, inclusi l'API Claude, gli SDK client e la Console Claude.

---

<Tip>
Per le note sulla versione di Claude Apps, consulta le [Note sulla versione di Claude Apps nel Centro assistenza Claude](https://support.claude.com/en/articles/12138966-release-notes).

Per gli aggiornamenti a Claude Code, consulta il [CHANGELOG.md completo](https://github.com/anthropics/claude-code/blob/main/CHANGELOG.md) nel repository `claude-code`.
</Tip>

### 19 dicembre 2025
- Abbiamo annunciato l'obsolescenza del modello Claude Haiku 3.5. Leggi di più nella [nostra documentazione](/docs/it/about-claude/model-deprecations).

### 4 dicembre 2025
- Gli [output strutturati](/docs/it/build-with-claude/structured-outputs) ora supportano Claude Haiku 4.5.

### 24 novembre 2025
- Abbiamo lanciato [Claude Opus 4.5](https://www.anthropic.com/news/claude-opus-4-5), il nostro modello più intelligente che combina la massima capacità con prestazioni pratiche. Ideale per compiti specializzati complessi, ingegneria del software professionale e agenti avanzati. Presenta miglioramenti significativi nella visione, nella codifica e nell'uso del computer a un prezzo più accessibile rispetto ai modelli Opus precedenti. Scopri di più nella nostra [documentazione Modelli e Prezzi](/docs/it/about-claude/models).
- Abbiamo lanciato [la chiamata di strumenti programmatica](/docs/it/agents-and-tools/tool-use/programmatic-tool-calling) in beta pubblico, consentendo a Claude di chiamare strumenti dall'interno dell'esecuzione del codice per ridurre la latenza e l'utilizzo di token nei flussi di lavoro multi-strumento.
- Abbiamo lanciato lo [strumento di ricerca di strumenti](/docs/it/agents-and-tools/tool-use/tool-search-tool) in beta pubblico, consentendo a Claude di scoprire e caricare dinamicamente gli strumenti su richiesta da cataloghi di strumenti di grandi dimensioni.
- Abbiamo lanciato il [parametro effort](/docs/it/build-with-claude/effort) in beta pubblico per Claude Opus 4.5, consentendoti di controllare l'utilizzo dei token scambiando tra la completezza della risposta e l'efficienza.
- Abbiamo aggiunto la [compattazione lato client](/docs/it/build-with-claude/context-editing#client-side-compaction-sdk) ai nostri SDK Python e TypeScript, gestendo automaticamente il contesto della conversazione attraverso il riepilogo quando si utilizza `tool_runner`.

### 21 novembre 2025
- I blocchi di contenuto dei risultati di ricerca sono ora generalmente disponibili su Amazon Bedrock. Scopri di più nella nostra [documentazione sui risultati di ricerca](/docs/it/build-with-claude/search-results).

### 19 novembre 2025
- Abbiamo lanciato una **nuova piattaforma di documentazione** su [platform.claude.com/docs](https://platform.claude.com/docs). La nostra documentazione ora vive fianco a fianco con la Console Claude, fornendo un'esperienza sviluppatore unificata. Il sito di documentazione precedente su docs.claude.com reindirizzerà alla nuova posizione.

### 18 novembre 2025
- Abbiamo lanciato **Claude in Microsoft Foundry**, portando i modelli Claude ai clienti Azure con fatturazione Azure e autenticazione OAuth. Accedi all'API Messages completa incluso il pensiero esteso, la memorizzazione nella cache dei prompt (5 minuti e 1 ora), il supporto PDF, l'API Files, Agent Skills e l'uso degli strumenti. Scopri di più nella nostra [documentazione Microsoft Foundry](/docs/it/build-with-claude/claude-in-microsoft-foundry).

### 14 novembre 2025
- Abbiamo lanciato gli [output strutturati](/docs/it/build-with-claude/structured-outputs) in beta pubblico, fornendo conformità dello schema garantita per le risposte di Claude. Utilizza output JSON per risposte di dati strutturati o uso rigoroso degli strumenti per input degli strumenti convalidati. Disponibile per Claude Sonnet 4.5 e Claude Opus 4.1. Per abilitare, utilizza l'intestazione beta `structured-outputs-2025-11-13`.

### 28 ottobre 2025
- Abbiamo annunciato l'obsolescenza del modello Claude Sonnet 3.7. Leggi di più nella [nostra documentazione](/docs/it/about-claude/model-deprecations).
- Abbiamo ritirato i modelli Claude Sonnet 3.5. Tutte le richieste a questi modelli ora restituiranno un errore.
- Abbiamo ampliato la modifica del contesto con la cancellazione dei blocchi di pensiero (`clear_thinking_20251015`), consentendo la gestione automatica dei blocchi di pensiero. Scopri di più nella nostra [documentazione sulla modifica del contesto](/docs/it/build-with-claude/context-editing).

### 16 ottobre 2025
- Abbiamo lanciato [Agent Skills](https://www.anthropic.com/engineering/equipping-agents-for-the-real-world-with-agent-skills) (beta `skills-2025-10-02`), un nuovo modo per estendere le capacità di Claude. Le Skills sono cartelle organizzate di istruzioni, script e risorse che Claude carica dinamicamente per eseguire compiti specializzati. La versione iniziale include:
  - **Skills gestite da Anthropic**: Skills pre-costruite per lavorare con file PowerPoint (.pptx), Excel (.xlsx), Word (.docx) e PDF
  - **Skills personalizzate**: Carica le tue Skills tramite l'API Skills (endpoint `/v1/skills`) per confezionare competenze di dominio e flussi di lavoro organizzativi
  - Le Skills richiedono l'abilitazione dello [strumento di esecuzione del codice](/docs/it/agents-and-tools/tool-use/code-execution-tool)
  - Scopri di più nella nostra [documentazione Agent Skills](/docs/it/agents-and-tools/agent-skills/overview) e [riferimento API](/docs/it/api/skills/create-skill)

### 15 ottobre 2025
- Abbiamo lanciato [Claude Haiku 4.5](https://www.anthropic.com/news/claude-haiku-4-5), il nostro modello Haiku più veloce e intelligente con prestazioni quasi di frontiera. Ideale per applicazioni in tempo reale, elaborazione ad alto volume e distribuzioni sensibili ai costi che richiedono un ragionamento forte. Scopri di più nella nostra [documentazione Modelli e Prezzi](/docs/it/about-claude/models).

### 29 settembre 2025
- Abbiamo lanciato [Claude Sonnet 4.5](https://www.anthropic.com/news/claude-sonnet-4-5), il nostro miglior modello per agenti complessi e codifica, con l'intelligenza più alta nella maggior parte dei compiti. Scopri di più in [Novità in Claude 4.5](/docs/it/about-claude/models/whats-new-claude-4-5).
- Abbiamo introdotto i [prezzi degli endpoint globali](/docs/it/about-claude/pricing#third-party-platform-pricing) per AWS Bedrock e Google Vertex AI. I prezzi dell'API Claude (1P) non sono interessati.
- Abbiamo introdotto un nuovo motivo di arresto `model_context_window_exceeded` che ti consente di richiedere il massimo numero possibile di token senza calcolare la dimensione dell'input. Scopri di più nella nostra [documentazione sulla gestione dei motivi di arresto](/docs/it/build-with-claude/handling-stop-reasons).
- Abbiamo lanciato lo strumento di memoria in beta, consentendo a Claude di archiviare e consultare informazioni tra le conversazioni. Scopri di più nella nostra [documentazione dello strumento di memoria](/docs/it/agents-and-tools/tool-use/memory-tool).
- Abbiamo lanciato la modifica del contesto in beta, fornendo strategie per gestire automaticamente il contesto della conversazione. La versione iniziale supporta la cancellazione dei risultati e delle chiamate degli strumenti più vecchi quando ci si avvicina ai limiti dei token. Scopri di più nella nostra [documentazione sulla modifica del contesto](/docs/it/build-with-claude/context-editing).

### 17 settembre 2025
- Abbiamo lanciato gli helper degli strumenti in beta per gli SDK Python e TypeScript, semplificando la creazione e l'esecuzione degli strumenti con convalida dell'input type-safe e un tool runner per la gestione automatica degli strumenti nelle conversazioni. Per i dettagli, consulta la documentazione per [l'SDK Python](https://github.com/anthropics/anthropic-sdk-python/blob/main/tools.md) e [l'SDK TypeScript](https://github.com/anthropics/anthropic-sdk-typescript/blob/main/helpers.md#tool-helpers).

### 16 settembre 2025
- Abbiamo unificato le nostre offerte per sviluppatori sotto il marchio Claude. Dovresti vedere nomi e URL aggiornati su tutta la nostra piattaforma e documentazione, ma **le nostre interfacce per sviluppatori rimarranno le stesse**. Ecco alcuni cambiamenti notevoli:
  - Anthropic Console ([console.anthropic.com](https://console.anthropic.com)) → Claude Console ([platform.claude.com](https://platform.claude.com)). La console sarà disponibile su entrambi gli URL fino al 16 dicembre 2025. Dopo tale data, [console.anthropic.com](https://console.anthropic.com) reindirizzerà automaticamente a [platform.claude.com](https://platform.claude.com).
  - Anthropic Docs ([docs.claude.com](https://docs.claude.com)) → Claude Docs ([docs.claude.com](https://docs.claude.com))
  - Anthropic Help Center ([support.claude.com](https://support.claude.com)) → Claude Help Center ([support.claude.com](https://support.claude.com))
  - Gli endpoint API, le intestazioni, le variabili di ambiente e gli SDK rimangono gli stessi. Le tue integrazioni esistenti continueranno a funzionare senza alcun cambiamento.

### 10 settembre 2025
- Abbiamo lanciato lo strumento di recupero web in beta, consentendo a Claude di recuperare il contenuto completo da pagine web e documenti PDF specificati. Scopri di più nella nostra [documentazione dello strumento di recupero web](/docs/it/agents-and-tools/tool-use/web-fetch-tool).
- Abbiamo lanciato l'[API Claude Code Analytics](/docs/it/build-with-claude/claude-code-analytics-api), consentendo alle organizzazioni di accedere programmaticamente alle metriche di utilizzo aggregate giornaliere per Claude Code, incluse metriche di produttività, statistiche di utilizzo degli strumenti e dati sui costi.

### 8 settembre 2025
- Abbiamo lanciato una versione beta dell'[SDK C#](https://github.com/anthropics/anthropic-sdk-csharp).

### 5 settembre 2025
- Abbiamo lanciato i [grafici dei limiti di velocità](/docs/it/api/rate-limits#monitoring-your-rate-limits-in-the-console) nella pagina [Utilizzo](https://console.anthropic.com/settings/usage) della Console, consentendoti di monitorare l'utilizzo dei limiti di velocità dell'API e i tassi di memorizzazione nella cache nel tempo.

### 3 settembre 2025
- Abbiamo lanciato il supporto per documenti citabili nei risultati degli strumenti lato client. Scopri di più nella nostra [documentazione sull'uso degli strumenti](/docs/it/agents-and-tools/tool-use/implement-tool-use).

### 2 settembre 2025
- Abbiamo lanciato la v2 dello [Strumento di esecuzione del codice](/docs/it/agents-and-tools/tool-use/code-execution-tool) in beta pubblico, sostituendo lo strumento originale solo Python con l'esecuzione di comandi Bash e le capacità di manipolazione diretta dei file, inclusa la scrittura di codice in altri linguaggi.

### 27 agosto 2025
- Abbiamo lanciato una versione beta dell'[SDK PHP](https://github.com/anthropics/anthropic-sdk-php).

### 26 agosto 2025
- Abbiamo aumentato i limiti di velocità sulla [finestra di contesto di 1M token](/docs/it/build-with-claude/context-windows#1m-token-context-window) per Claude Sonnet 4 sull'API Claude. Per ulteriori informazioni, consulta [Limiti di velocità del contesto lungo](/docs/it/api/rate-limits#long-context-rate-limits).
- La finestra di contesto di 1m token è ora disponibile su Google Cloud's Vertex AI. Per ulteriori informazioni, consulta [Claude su Vertex AI](/docs/it/build-with-claude/claude-on-vertex-ai).

### 19 agosto 2025
- Gli ID richiesta sono ora inclusi direttamente nei corpi delle risposte di errore insieme all'intestazione `request-id` esistente. Scopri di più nella nostra [documentazione degli errori](/docs/it/api/errors#error-shapes).

### 18 agosto 2025
- Abbiamo rilasciato l'[API Utilizzo e Costi](/docs/it/build-with-claude/usage-cost-api), consentendo agli amministratori di monitorare programmaticamente i dati di utilizzo e costi della loro organizzazione.
- Abbiamo aggiunto un nuovo endpoint all'API Admin per il recupero delle informazioni sull'organizzazione. Per i dettagli, consulta il [riferimento API Admin per le informazioni sull'organizzazione](/docs/it/api/admin-api/organization/get-me).

### 13 agosto 2025
- Abbiamo annunciato l'obsolescenza dei modelli Claude Sonnet 3.5 (`claude-3-5-sonnet-20240620` e `claude-3-5-sonnet-20241022`). Questi modelli verranno ritirati il 28 ottobre 2025. Ti consigliamo di migrare a Claude Sonnet 4.5 (`claude-sonnet-4-5-20250929`) per prestazioni e capacità migliorate. Leggi di più nella [documentazione Deprecazioni modelli](/docs/it/about-claude/model-deprecations).
- La durata della cache di 1 ora per la memorizzazione nella cache dei prompt è ora generalmente disponibile. Puoi ora utilizzare il TTL della cache estesa senza un'intestazione beta. Scopri di più nella nostra [documentazione sulla memorizzazione nella cache dei prompt](/docs/it/build-with-claude/prompt-caching#1-hour-cache-duration).

### 12 agosto 2025
- Abbiamo lanciato il supporto beta per una [finestra di contesto di 1M token](/docs/it/build-with-claude/context-windows#1m-token-context-window) in Claude Sonnet 4 sull'API Claude e Amazon Bedrock.

### 11 agosto 2025
- Alcuni clienti potrebbero riscontrare errori 429 (`rate_limit_error`) [errori](/docs/it/api/errors) a seguito di un aumento acuto dell'utilizzo dell'API dovuto ai limiti di accelerazione sull'API. In precedenza, gli errori 529 (`overloaded_error`) si verificherebbero in scenari simili.

### 8 agosto 2025
- I blocchi di contenuto dei risultati di ricerca sono ora generalmente disponibili sull'API Claude e su Google Cloud's Vertex AI. Questa funzione consente citazioni naturali per le applicazioni RAG con corretta attribuzione della fonte. L'intestazione beta `search-results-2025-06-09` non è più richiesta. Scopri di più nella nostra [documentazione sui risultati di ricerca](/docs/it/build-with-claude/search-results).

### 5 agosto 2025
- Abbiamo lanciato [Claude Opus 4.1](https://www.anthropic.com/news/claude-opus-4-1), un aggiornamento incrementale a Claude Opus 4 con capacità migliorate e miglioramenti delle prestazioni.<sup>*</sup> Scopri di più nella nostra [documentazione Modelli e Prezzi](/docs/it/about-claude/models).

_<sup>* - Opus 4.1 non consente di specificare sia i parametri `temperature` che `top_p`. Utilizza solo uno. </sup>_

### 28 luglio 2025
- Abbiamo rilasciato `text_editor_20250728`, uno strumento editor di testo aggiornato che corregge alcuni problemi dalle versioni precedenti e aggiunge un parametro `max_characters` facoltativo che ti consente di controllare la lunghezza del troncamento quando visualizzi file di grandi dimensioni.

### 24 luglio 2025
- Abbiamo aumentato i [limiti di velocità](/docs/it/api/rate-limits) per Claude Opus 4 sull'API Claude per darti più capacità di costruire e scalare con Claude. Per i clienti con [limiti di velocità del livello di utilizzo 1-4](/docs/it/api/rate-limits#rate-limits), questi cambiamenti si applicano immediatamente al tuo account - nessuna azione necessaria.

### 21 luglio 2025
- Abbiamo ritirato i modelli Claude 2.0, Claude 2.1 e Claude Sonnet 3. Tutte le richieste a questi modelli ora restituiranno un errore. Leggi di più nella [nostra documentazione](/docs/it/about-claude/model-deprecations).

### 17 luglio 2025
- Abbiamo aumentato i [limiti di velocità](/docs/it/api/rate-limits) per Claude Sonnet 4 sull'API Claude per darti più capacità di costruire e scalare con Claude. Per i clienti con [limiti di velocità del livello di utilizzo 1-4](/docs/it/api/rate-limits#rate-limits), questi cambiamenti si applicano immediatamente al tuo account - nessuna azione necessaria.

### 3 luglio 2025
- Abbiamo lanciato i blocchi di contenuto dei risultati di ricerca in beta, consentendo citazioni naturali per le applicazioni RAG. Gli strumenti possono ora restituire risultati di ricerca con corretta attribuzione della fonte, e Claude citerà automaticamente queste fonti nelle sue risposte - corrispondendo alla qualità delle citazioni della ricerca web. Questo elimina la necessità di soluzioni alternative ai documenti nelle applicazioni della base di conoscenza personalizzate. Scopri di più nella nostra [documentazione sui risultati di ricerca](/docs/it/build-with-claude/search-results). Per abilitare questa funzione, utilizza l'intestazione beta `search-results-2025-06-09`.

### 30 giugno 2025
- Abbiamo annunciato l'obsolescenza del modello Claude Opus 3. Leggi di più nella [nostra documentazione](/docs/it/about-claude/model-deprecations).

### 23 giugno 2025
- Gli utenti della Console con il ruolo Developer possono ora accedere alla pagina [Costo](https://console.anthropic.com/settings/cost). In precedenza, il ruolo Developer consentiva l'accesso alla pagina [Utilizzo](https://console.anthropic.com/settings/usage), ma non alla pagina Costo.

### 11 giugno 2025
- Abbiamo lanciato lo [streaming degli strumenti a grana fine](/docs/it/agents-and-tools/tool-use/fine-grained-tool-streaming) in beta pubblico, una funzione che consente a Claude di trasmettere i parametri di utilizzo degli strumenti senza buffering / convalida JSON. Per abilitare lo streaming degli strumenti a grana fine, utilizza l'[intestazione beta](/docs/it/api/beta-headers) `fine-grained-tool-streaming-2025-05-14`.

### 22 maggio 2025
- Abbiamo lanciato [Claude Opus 4 e Claude Sonnet 4](http://www.anthropic.com/news/claude-4), i nostri ultimi modelli con capacità di pensiero esteso. Scopri di più nella nostra [documentazione Modelli e Prezzi](/docs/it/about-claude/models).
- Il comportamento predefinito del [pensiero esteso](/docs/it/build-with-claude/extended-thinking) nei modelli Claude 4 restituisce un riepilogo del processo di pensiero completo di Claude, con il pensiero completo crittografato e restituito nel campo `signature` dell'output del blocco `thinking`.
- Abbiamo lanciato il [pensiero intercalato](/docs/it/build-with-claude/extended-thinking#interleaved-thinking) in beta pubblico, una funzione che consente a Claude di pensare tra le chiamate degli strumenti. Per abilitare il pensiero intercalato, utilizza l'[intestazione beta](/docs/it/api/beta-headers) `interleaved-thinking-2025-05-14`.
- Abbiamo lanciato l'[API Files](/docs/it/build-with-claude/files) in beta pubblico, consentendoti di caricare file e farvi riferimento nell'API Messages e nello strumento di esecuzione del codice.
- Abbiamo lanciato lo [strumento di esecuzione del codice](/docs/it/agents-and-tools/tool-use/code-execution-tool) in beta pubblico, uno strumento che consente a Claude di eseguire codice Python in un ambiente sandbox sicuro.
- Abbiamo lanciato il [connettore MCP](/docs/it/agents-and-tools/mcp-connector) in beta pubblico, una funzione che ti consente di connetterti ai server MCP remoti direttamente dall'API Messages. 
- Per aumentare la qualità delle risposte e diminuire gli errori degli strumenti, abbiamo modificato il valore predefinito del parametro `top_p` [campionamento nucleus](https://en.wikipedia.org/wiki/Top-p_sampling) nell'API Messages da 0.999 a 0.99 per tutti i modelli. Per ripristinare questo cambiamento, imposta `top_p` a 0.999. 
    Inoltre, quando il pensiero esteso è abilitato, puoi ora impostare `top_p` su valori tra 0.95 e 1.
- Abbiamo spostato il nostro [SDK Go](https://github.com/anthropics/anthropic-sdk-go) da beta a GA.
- Abbiamo incluso granularità a livello di minuto e ora nella pagina [Utilizzo](https://console.anthropic.com/settings/usage) della Console insieme ai tassi di errore 429 nella pagina Utilizzo.

### 21 maggio 2025
- Abbiamo spostato il nostro [SDK Ruby](https://github.com/anthropics/anthropic-sdk-ruby) da beta a GA.

### 7 maggio 2025
- Abbiamo lanciato uno strumento di ricerca web nell'API, consentendo a Claude di accedere a informazioni aggiornate dal web. Scopri di più nella nostra [documentazione dello strumento di ricerca web](/docs/it/agents-and-tools/tool-use/web-search-tool).

### 1 maggio 2025
- Il controllo della cache deve ora essere specificato direttamente nel blocco `content` padre di `tool_result` e `document.source`. Per la compatibilità all'indietro, se il controllo della cache viene rilevato nell'ultimo blocco in `tool_result.content` o `document.source.content`, verrà automaticamente applicato al blocco padre invece. Il controllo della cache su qualsiasi altro blocco all'interno di `tool_result.content` e `document.source.content` comporterà un errore di convalida.

### 9 aprile 2025
- Abbiamo lanciato una versione beta dell'[SDK Ruby](https://github.com/anthropics/anthropic-sdk-ruby)

### 31 marzo 2025
- Abbiamo spostato il nostro [SDK Java](https://github.com/anthropics/anthropic-sdk-java) da beta a GA.
- Abbiamo spostato il nostro [SDK Go](https://github.com/anthropics/anthropic-sdk-go) da alfa a beta.

### 27 febbraio 2025
- Abbiamo aggiunto blocchi di origine URL per immagini e PDF nell'API Messages. Puoi ora fare riferimento a immagini e PDF direttamente tramite URL invece di doverli codificare in base64. Scopri di più nella nostra [documentazione sulla visione](/docs/it/build-with-claude/vision) e [documentazione sul supporto PDF](/docs/it/build-with-claude/pdf-support).
- Abbiamo aggiunto il supporto per un'opzione `none` al parametro `tool_choice` nell'API Messages che impedisce a Claude di chiamare qualsiasi strumento. Inoltre, non sei più obbligato a fornire alcun `tools` quando includi blocchi `tool_use` e `tool_result`.
- Abbiamo lanciato un endpoint API compatibile con OpenAI, consentendoti di testare i modelli Claude cambiando solo la tua chiave API, l'URL di base e il nome del modello nelle integrazioni OpenAI esistenti. Questo livello di compatibilità supporta la funzionalità di completamenti di chat di base. Scopri di più nella nostra [documentazione sulla compatibilità dell'SDK OpenAI](/docs/it/api/openai-sdk).

### 24 febbraio 2025
- Abbiamo lanciato [Claude Sonnet 3.7](http://www.anthropic.com/news/claude-3-7-sonnet), il nostro modello più intelligente finora. Claude Sonnet 3.7 può produrre risposte quasi istantanee o mostrare il suo pensiero esteso passo dopo passo. Un modello, due modi di pensare. Scopri di più su tutti i modelli Claude nella nostra [documentazione Modelli e Prezzi](/docs/it/about-claude/models).
- Abbiamo aggiunto il supporto della visione a Claude Haiku 3.5, consentendo al modello di analizzare e comprendere le immagini.
- Abbiamo rilasciato un'implementazione dell'uso degli strumenti efficiente in termini di token, migliorando le prestazioni complessive quando si utilizzano strumenti con Claude. Scopri di più nella nostra [documentazione sull'uso degli strumenti](/docs/it/agents-and-tools/tool-use/overview).
- Abbiamo modificato la temperatura predefinita nella [Console](https://console.anthropic.com/workbench) per i nuovi prompt da 0 a 1 per coerenza con la temperatura predefinita nell'API. I prompt salvati esistenti rimangono invariati.
- Abbiamo rilasciato versioni aggiornate dei nostri strumenti che disaccoppiano gli strumenti di modifica del testo e bash dal prompt di sistema di utilizzo del computer:
  - `bash_20250124`: Stessa funzionalità della versione precedente ma indipendente dall'uso del computer. Non richiede un'intestazione beta.
  - `text_editor_20250124`: Stessa funzionalità della versione precedente ma indipendente dall'uso del computer. Non richiede un'intestazione beta.
  - `computer_20250124`: Strumento di utilizzo del computer aggiornato con nuove opzioni di comando incluse "hold_key", "left_mouse_down", "left_mouse_up", "scroll", "triple_click" e "wait". Questo strumento richiede l'intestazione anthropic-beta "computer-use-2025-01-24".
  Scopri di più nella nostra [documentazione sull'uso degli strumenti](/docs/it/agents-and-tools/tool-use/overview).

### 10 febbraio 2025
- Abbiamo aggiunto l'intestazione della risposta `anthropic-organization-id` a tutte le risposte dell'API. Questa intestazione fornisce l'ID dell'organizzazione associato alla chiave API utilizzata nella richiesta.

### 31 gennaio 2025

- Abbiamo spostato il nostro [SDK Java](https://github.com/anthropics/anthropic-sdk-java) da alfa a beta.

### 23 gennaio 2025

- Abbiamo lanciato la capacità di citazioni nell'API, consentendo a Claude di fornire l'attribuzione della fonte per le informazioni. Scopri di più nella nostra [documentazione sulle citazioni](/docs/it/build-with-claude/citations).
- Abbiamo aggiunto il supporto per documenti di testo semplice e documenti di contenuto personalizzato nell'API Messages.

### 21 gennaio 2025

- Abbiamo annunciato l'obsolescenza dei modelli Claude 2, Claude 2.1 e Claude Sonnet 3. Leggi di più nella [nostra documentazione](/docs/it/about-claude/model-deprecations).

### 15 gennaio 2025

- Abbiamo aggiornato la [memorizzazione nella cache dei prompt](/docs/it/build-with-claude/prompt-caching) per essere più facile da usare. Ora, quando imposti un punto di interruzione della cache, leggeremo automaticamente dal tuo prefisso precedentemente memorizzato nella cache più lungo.
- Puoi ora mettere parole nella bocca di Claude quando utilizzi gli strumenti.

### 10 gennaio 2025

- Abbiamo ottimizzato il supporto per la [memorizzazione nella cache dei prompt nell'API Message Batches](/docs/it/build-with-claude/batch-processing#using-prompt-caching-with-message-batches) per migliorare il tasso di hit della cache.

### 19 dicembre 2024

- Abbiamo aggiunto il supporto per un [endpoint di eliminazione](/docs/it/api/deleting-message-batches) nell'API Message Batches

### 17 dicembre 2024
Le seguenti funzioni sono ora generalmente disponibili nell'API Claude:

- [API Modelli](/docs/it/api/models-list): Interroga i modelli disponibili, convalida gli ID dei modelli e risolvi gli [alias dei modelli](/docs/it/about-claude/models#model-names) ai loro ID canonici dei modelli.
- [API Message Batches](/docs/it/build-with-claude/batch-processing): Elabora grandi batch di messaggi in modo asincrono al 50% del costo API standard.
- [API Token Counting](/docs/it/build-with-claude/token-counting): Calcola i conteggi dei token per i Messages prima di inviarli a Claude.
- [Memorizzazione nella cache dei prompt](/docs/it/build-with-claude/prompt-caching): Riduci i costi fino al 90% e la latenza fino all'80% memorizzando nella cache e riutilizzando il contenuto dei prompt.
- [Supporto PDF](/docs/it/build-with-claude/pdf-support): Elabora i PDF per analizzare sia il contenuto testuale che visivo all'interno dei documenti.

Abbiamo anche rilasciato nuovi SDK ufficiali:
- [SDK Java](https://github.com/anthropics/anthropic-sdk-java) (alfa)
- [SDK Go](https://github.com/anthropics/anthropic-sdk-go) (alfa)

### 4 dicembre 2024

- Abbiamo aggiunto la possibilità di raggruppare per chiave API alle pagine [Utilizzo](https://console.anthropic.com/settings/usage) e [Costo](https://console.anthropic.com/settings/cost) della [Console Developer](https://console.anthropic.com)
- Abbiamo aggiunto due nuove colonne **Ultimo utilizzo** e **Costo** e la possibilità di ordinare per qualsiasi colonna nella pagina [Chiavi API](https://console.anthropic.com/settings/keys) della [Console Developer](https://console.anthropic.com)

### 21 novembre 2024

- Abbiamo rilasciato l'[API Admin](/docs/it/build-with-claude/administration-api), consentendo agli utenti di gestire programmaticamente le risorse della loro organizzazione.

### 20 novembre 2024

- Abbiamo aggiornato i nostri limiti di velocità per l'API Messages. Abbiamo sostituito il limite di velocità dei token al minuto con nuovi limiti di velocità dei token di input e output al minuto. Leggi di più nella nostra [documentazione](/docs/it/api/rate-limits).
- Abbiamo aggiunto il supporto per l'[uso degli strumenti](/docs/it/agents-and-tools/tool-use/overview) nel [Workbench](https://console.anthropic.com/workbench).

### 13 novembre 2024

- Abbiamo aggiunto il supporto PDF per tutti i modelli Claude Sonnet 3.5. Leggi di più nella nostra [documentazione](/docs/it/build-with-claude/pdf-support).

### 6 novembre 2024

- Abbiamo ritirato i modelli Claude 1 e Instant. Leggi di più nella [nostra documentazione](/docs/it/about-claude/model-deprecations).

### 4 novembre 2024

- [Claude Haiku 3.5](https://www.anthropic.com/claude/haiku) è ora disponibile sull'API Claude come modello solo testo.

### 1 novembre 2024

- Abbiamo aggiunto il supporto PDF per l'uso con il nuovo Claude Sonnet 3.5. Leggi di più nella nostra [documentazione](/docs/it/build-with-claude/pdf-support).
- Abbiamo anche aggiunto il conteggio dei token, che ti consente di determinare il numero totale di token in un Message, prima di inviarlo a Claude. Leggi di più nella nostra [documentazione](/docs/it/build-with-claude/token-counting).

### 22 ottobre 2024

- Abbiamo aggiunto gli strumenti di utilizzo del computer definiti da Anthropic alla nostra API per l'uso con il nuovo Claude Sonnet 3.5. Leggi di più nella nostra [documentazione](/docs/it/agents-and-tools/tool-use/computer-use-tool).
- Claude Sonnet 3.5, il nostro modello più intelligente finora, ha appena ricevuto un aggiornamento ed è ora disponibile sull'API Claude. Leggi di più [qui](https://www.anthropic.com/claude/sonnet).

### 8 ottobre 2024

- L'API Message Batches è ora disponibile in beta. Elabora grandi batch di query in modo asincrono nell'API Claude per il 50% in meno di costo. Leggi di più nella nostra [documentazione](/docs/it/build-with-claude/batch-processing).
- Abbiamo allentato le restrizioni sull'ordinamento dei turni `user`/`assistant` nella nostra API Messages. I messaggi `user`/`assistant` consecutivi verranno combinati in un singolo messaggio invece di generare un errore, e non richiediamo più che il primo messaggio di input sia un messaggio `user`.
- Abbiamo deprecato i piani Build e Scale a favore di una suite di funzioni standard (precedentemente denominata Build), insieme a funzioni aggiuntive disponibili tramite vendite. Leggi di più [qui](https://claude.com/platform/api).

### 3 ottobre 2024

- Abbiamo aggiunto la possibilità di disabilitare l'uso parallelo degli strumenti nell'API. Imposta `disable_parallel_tool_use: true` nel campo `tool_choice` per assicurarti che Claude utilizzi al massimo uno strumento. Leggi di più nella nostra [documentazione](/docs/it/agents-and-tools/tool-use/implement-tool-use#parallel-tool-use).

### 10 settembre 2024

- Abbiamo aggiunto Workspaces alla [Console Developer](https://console.anthropic.com). I Workspaces ti consentono di impostare limiti di spesa o velocità personalizzati, raggruppare le chiavi API, tracciare l'utilizzo per progetto e controllare l'accesso con ruoli utente. Leggi di più nel nostro [post del blog](https://www.anthropic.com/news/workspaces).

### 4 settembre 2024

- Abbiamo annunciato l'obsolescenza dei modelli Claude 1. Leggi di più nella [nostra documentazione](/docs/it/about-claude/model-deprecations).

### 22 agosto 2024

- Abbiamo aggiunto il supporto per l'utilizzo dell'SDK nei browser restituendo intestazioni CORS nelle risposte dell'API. Imposta `dangerouslyAllowBrowser: true` nell'istanziazione dell'SDK per abilitare questa funzione.

### 19 agosto 2024

- Abbiamo spostato gli output di 8.192 token da beta a disponibilità generale per Claude Sonnet 3.5.

### 14 agosto 2024

- La [memorizzazione nella cache dei prompt](/docs/it/build-with-claude/prompt-caching) è ora disponibile come funzione beta nell'API Claude. Memorizza nella cache e riutilizza i prompt per ridurre la latenza fino all'80% e i costi fino al 90%.

### 15 luglio 2024

- Genera output fino a 8.192 token di lunghezza da Claude Sonnet 3.5 con la nuova intestazione `anthropic-beta: max-tokens-3-5-sonnet-2024-07-15`.

### 9 luglio 2024

- Genera automaticamente casi di test per i tuoi prompt utilizzando Claude nella [Console Developer](https://console.anthropic.com).
- Confronta gli output da diversi prompt fianco a fianco nella nuova modalità di confronto degli output nella [Console Developer](https://console.anthropic.com).

### 27 giugno 2024

- Visualizza l'utilizzo dell'API e la fatturazione suddivisi per importo in dollari, conteggio dei token e chiavi API nelle nuove schede [Utilizzo](https://console.anthropic.com/settings/usage) e [Costo](https://console.anthropic.com/settings/cost) nella [Console Developer](https://console.anthropic.com).
- Visualizza i tuoi attuali limiti di velocità dell'API nella nuova scheda [Limiti di velocità](https://console.anthropic.com/settings/limits) nella [Console Developer](https://console.anthropic.com).

### 20 giugno 2024

- [Claude Sonnet 3.5](http://anthropic.com/news/claude-3-5-sonnet), il nostro modello più intelligente finora, è ora generalmente disponibile su tutta l'API Claude, Amazon Bedrock e Google Vertex AI.

### 30 maggio 2024

- L'[uso degli strumenti](/docs/it/agents-and-tools/tool-use/overview) è ora generalmente disponibile su tutta l'API Claude, Amazon Bedrock e Google Vertex AI.

### 10 maggio 2024

- Il nostro strumento di generazione dei prompt è ora disponibile nella [Console Developer](https://console.anthropic.com). Prompt Generator rende facile guidare Claude a generare prompt di alta qualità personalizzati per i tuoi compiti specifici. Leggi di più nel nostro [post del blog](https://www.anthropic.com/news/prompt-generator).