# Migliori pratiche di prompt engineering

Tecniche specifiche di prompt engineering per i modelli Claude 4.x, con indicazioni specifiche per Sonnet 4.5, Haiku 4.5 e Opus 4.5.

---

Questa guida fornisce tecniche specifiche di prompt engineering per i modelli Claude 4.x, con indicazioni specifiche per Sonnet 4.5, Haiku 4.5 e Opus 4.5. Questi modelli sono stati addestrati per seguire le istruzioni in modo più preciso rispetto alle generazioni precedenti dei modelli Claude.
<Tip>
  Per una panoramica delle nuove capacità di Claude 4.5, consulta [Novità in Claude 4.5](/docs/it/about-claude/models/whats-new-claude-4-5). Per indicazioni sulla migrazione dai modelli precedenti, consulta [Migrazione a Claude 4.5](/docs/it/about-claude/models/migrating-to-claude-4).
</Tip>

## Principi generali

### Sii esplicito con le tue istruzioni

I modelli Claude 4.x rispondono bene a istruzioni chiare ed esplicite. Essere specifico riguardo all'output desiderato può aiutare a migliorare i risultati. I clienti che desiderano il comportamento "oltre le aspettative" dai modelli Claude precedenti potrebbero aver bisogno di richiedere più esplicitamente questi comportamenti con i modelli più recenti.

<section title="Esempio: Creazione di una dashboard di analisi">

**Meno efficace:**
```text
Crea una dashboard di analisi
```

**Più efficace:**
```text
Crea una dashboard di analisi. Includi il maggior numero possibile di funzionalità e interazioni rilevanti. Vai oltre le basi per creare un'implementazione completamente funzionale.
```

</section>

### Aggiungi contesto per migliorare le prestazioni

Fornire contesto o motivazione dietro le tue istruzioni, come spiegare a Claude perché tale comportamento è importante, può aiutare i modelli Claude 4.x a comprendere meglio i tuoi obiettivi e fornire risposte più mirate.

<section title="Esempio: Preferenze di formattazione">

**Meno efficace:**
```text
NON usare mai i puntini di sospensione
```

**Più efficace:**
```text
La tua risposta verrà letta ad alta voce da un motore di sintesi vocale, quindi non usare mai i puntini di sospensione poiché il motore di sintesi vocale non saprà come pronunciarli.
```

</section>

Claude è abbastanza intelligente da generalizzare dalla spiegazione.

### Sii vigile con gli esempi e i dettagli

I modelli Claude 4.x prestano molta attenzione ai dettagli e agli esempi come parte delle loro capacità precise di seguire le istruzioni. Assicurati che i tuoi esempi si allineino con i comportamenti che desideri incoraggiare e minimizzare i comportamenti che desideri evitare.

### Ragionamento a lungo termine e tracciamento dello stato

I modelli Claude 4.5 eccellono nei compiti di ragionamento a lungo termine con eccezionali capacità di tracciamento dello stato. Mantiene l'orientamento durante sessioni estese concentrandosi sul progresso incrementale, facendo progressi costanti su poche cose alla volta piuttosto che tentare tutto contemporaneamente. Questa capacità emerge soprattutto su più finestre di contesto o iterazioni di compiti, dove Claude può lavorare su un compito complesso, salvare lo stato e continuare con una finestra di contesto nuova.

#### Consapevolezza del contesto e flussi di lavoro multi-finestra

I modelli Claude 4.5 dispongono di [consapevolezza del contesto](/docs/it/build-with-claude/context-windows#context-awareness-in-claude-sonnet-4-5), consentendo al modello di tracciare la sua finestra di contesto rimanente (cioè il "budget di token") durante una conversazione. Ciò consente a Claude di eseguire compiti e gestire il contesto in modo più efficace comprendendo quanto spazio ha a disposizione.

**Gestione dei limiti di contesto:**

Se stai utilizzando Claude in un harness di agente che compatta il contesto o consente di salvare il contesto in file esterni (come in Claude Code), ti suggeriamo di aggiungere queste informazioni al tuo prompt in modo che Claude possa comportarsi di conseguenza. Altrimenti, Claude potrebbe a volte cercare naturalmente di concludere il lavoro mentre si avvicina al limite di contesto. Di seguito è riportato un prompt di esempio:

```text Prompt di esempio
La tua finestra di contesto verrà compattata automaticamente quando si avvicina al suo limite, permettendoti di continuare a lavorare indefinitamente da dove hai lasciato. Pertanto, non interrompere i compiti in anticipo a causa di preoccupazioni relative al budget di token. Man mano che ti avvicini al limite del tuo budget di token, salva il tuo progresso e stato attuali in memoria prima che la finestra di contesto si aggiorni. Sii sempre il più persistente e autonomo possibile e completa i compiti completamente, anche se la fine del tuo budget si sta avvicinando. Non interrompere mai artificialmente alcun compito in anticipo indipendentemente dal contesto rimanente.
```

Lo [strumento di memoria](/docs/it/agents-and-tools/tool-use/memory-tool) si abbina naturalmente alla consapevolezza del contesto per transizioni di contesto senza interruzioni.

#### Flussi di lavoro multi-finestra di contesto

Per compiti che si estendono su più finestre di contesto:

1. **Usa un prompt diverso per la primissima finestra di contesto**: Usa la prima finestra di contesto per impostare un framework (scrivi test, crea script di configurazione), quindi usa le future finestre di contesto per iterare su una lista di cose da fare.

2. **Fai scrivere al modello i test in un formato strutturato**: Chiedi a Claude di creare test prima di iniziare il lavoro e tieni traccia di essi in un formato strutturato (ad es. `tests.json`). Questo porta a una migliore capacità a lungo termine di iterare. Ricorda a Claude l'importanza dei test: "È inaccettabile rimuovere o modificare i test perché ciò potrebbe portare a funzionalità mancanti o difettose."

3. **Configura strumenti per la qualità della vita**: Incoraggia Claude a creare script di configurazione (ad es. `init.sh`) per avviare gracefully i server, eseguire suite di test e linter. Ciò previene il lavoro ripetuto quando si continua da una finestra di contesto nuova.

4. **Iniziare da zero vs compattare**: Quando una finestra di contesto viene cancellata, considera di iniziare con una finestra di contesto completamente nuova piuttosto che utilizzare la compattazione. I modelli Claude 4.5 sono estremamente efficaci nel scoprire lo stato dal file system locale. In alcuni casi, potresti voler sfruttare questo rispetto alla compattazione. Sii prescrittivo su come dovrebbe iniziare:
   - "Chiama pwd; puoi solo leggere e scrivere file in questa directory."
   - "Rivedi progress.txt, tests.json e i log git."
   - "Esegui manualmente un test di integrazione fondamentale prima di passare all'implementazione di nuove funzionalità."

5. **Fornisci strumenti di verifica**: Man mano che la lunghezza dei compiti autonomi cresce, Claude ha bisogno di verificare la correttezza senza feedback umano continuo. Strumenti come il server Playwright MCP o le capacità di utilizzo del computer per testare le interfacce utente sono utili.

6. **Incoraggia l'utilizzo completo del contesto**: Chiedi a Claude di completare efficientemente i componenti prima di passare oltre:

```text Prompt di esempio
Questo è un compito molto lungo, quindi potrebbe essere vantaggioso pianificare chiaramente il tuo lavoro. È incoraggiato spendere l'intero contesto di output lavorando sul compito - assicurati solo di non esaurire il contesto con lavoro significativo non sottoposto a commit. Continua a lavorare sistematicamente fino a quando non avrai completato questo compito.
```

#### Migliori pratiche di gestione dello stato

- **Usa formati strutturati per i dati di stato**: Quando traccia informazioni strutturate (come risultati di test o stato del compito), usa JSON o altri formati strutturati per aiutare Claude a comprendere i requisiti dello schema
- **Usa testo non strutturato per le note di progresso**: Le note di progresso in formato libero funzionano bene per tracciare il progresso generale e il contesto
- **Usa git per il tracciamento dello stato**: Git fornisce un registro di ciò che è stato fatto e checkpoint che possono essere ripristinati. I modelli Claude 4.5 si comportano particolarmente bene nell'usare git per tracciare lo stato su più sessioni.
- **Sottolinea il progresso incrementale**: Chiedi esplicitamente a Claude di tenere traccia del suo progresso e concentrarsi sul lavoro incrementale

<section title="Esempio: Tracciamento dello stato">

```json
// File di stato strutturato (tests.json)
{
  "tests": [
    {"id": 1, "name": "authentication_flow", "status": "passing"},
    {"id": 2, "name": "user_management", "status": "failing"},
    {"id": 3, "name": "api_endpoints", "status": "not_started"}
  ],
  "total": 200,
  "passing": 150,
  "failing": 25,
  "not_started": 25
}
```

```text
// Note di progresso (progress.txt)
Progresso della sessione 3:
- Corretta la convalida del token di autenticazione
- Aggiornato il modello utente per gestire i casi limite
- Prossimo: investigare i fallimenti del test user_management (test #2)
- Nota: Non rimuovere i test poiché ciò potrebbe portare a funzionalità mancanti
```

</section>

### Stile di comunicazione

I modelli Claude 4.5 hanno uno stile di comunicazione più conciso e naturale rispetto ai modelli precedenti:

- **Più diretto e radicato**: Fornisce rapporti di progresso basati su fatti piuttosto che aggiornamenti auto-celebrativi
- **Più conversazionale**: Leggermente più fluido e colloquiale, meno simile a una macchina
- **Meno verboso**: Potrebbe saltare riassunti dettagliati per efficienza a meno che non richiesto diversamente

Questo stile di comunicazione riflette accuratamente ciò che è stato realizzato senza elaborazione non necessaria.

## Indicazioni per situazioni specifiche

### Bilancia la verbosità

I modelli Claude 4.5 tendono verso l'efficienza e potrebbero saltare riassunti verbali dopo le chiamate di strumenti, passando direttamente all'azione successiva. Sebbene ciò crei un flusso di lavoro snellito, potresti preferire più visibilità nel suo processo di ragionamento.

Se desideri che Claude fornisca aggiornamenti mentre lavora:

```text Prompt di esempio
Dopo aver completato un compito che comporta l'uso di strumenti, fornisci un breve riassunto del lavoro che hai svolto.
```

### Modelli di utilizzo degli strumenti

I modelli Claude 4.5 sono addestrati per il seguire le istruzioni in modo preciso e beneficiano di indicazioni esplicite per utilizzare strumenti specifici. Se dici "puoi suggerire alcuni cambiamenti", a volte fornirà suggerimenti piuttosto che implementarli, anche se apportare modifiche potrebbe essere quello che intendevi.

Affinché Claude agisca, sii più esplicito:

<section title="Esempio: Istruzioni esplicite">

**Meno efficace (Claude suggerirà solo):**
```text
Puoi suggerire alcuni cambiamenti per migliorare questa funzione?
```

**Più efficace (Claude apporterà i cambiamenti):**
```text
Modifica questa funzione per migliorarne le prestazioni.
```

O:
```text
Apporta questi modifiche al flusso di autenticazione.
```

</section>

Per rendere Claude più proattivo nel prendere azioni per impostazione predefinita, puoi aggiungere questo al tuo prompt di sistema:

```text Prompt di esempio per azione proattiva
<default_to_action>
Per impostazione predefinita, implementa i cambiamenti piuttosto che solo suggerirli. Se l'intento dell'utente non è chiaro, deduci l'azione più utile probabile e procedi, utilizzando gli strumenti per scoprire eventuali dettagli mancanti invece di indovinare. Prova a dedurre l'intento dell'utente su se una chiamata di strumento (ad es. modifica o lettura di file) è intesa o meno, e agisci di conseguenza.
</default_to_action>
```

D'altra parte, se desideri che il modello sia più esitante per impostazione predefinita, meno propenso a saltare direttamente alle implementazioni, e agisca solo se richiesto, puoi guidare questo comportamento con un prompt come il seguente:

```text Prompt di esempio per azione conservativa
<do_not_act_before_instructions>
Non saltare nell'implementazione o modificare file a meno che non sia chiaramente istruito di apportare modifiche. Quando l'intento dell'utente è ambiguo, predefinito per fornire informazioni, fare ricerche e fornire raccomandazioni piuttosto che agire. Procedi con modifiche, alterazioni o implementazioni solo quando l'utente le richiede esplicitamente.
</do_not_act_before_instructions>
```

### Utilizzo degli strumenti e attivazione

Claude Opus 4.5 è più reattivo al prompt di sistema rispetto ai modelli precedenti. Se i tuoi prompt erano progettati per ridurre il sottoutilizzo degli strumenti o delle competenze, Claude Opus 4.5 potrebbe ora sovraattivare. La soluzione è attenuare qualsiasi linguaggio aggressivo. Dove avresti potuto dire "CRITICO: DEVI usare questo strumento quando...", puoi usare prompt più normali come "Usa questo strumento quando...".

### Controlla il formato delle risposte

Abbiamo trovato alcuni modi particolarmente efficaci per guidare la formattazione dell'output nei modelli Claude 4.x:

1. **Dì a Claude cosa fare invece di cosa non fare**

   - Invece di: "Non usare markdown nella tua risposta"
   - Prova: "La tua risposta dovrebbe essere composta da paragrafi di prosa che scorrono dolcemente."

2. **Usa indicatori di formato XML**

   - Prova: "Scrivi le sezioni di prosa della tua risposta in tag \<smoothly_flowing_prose_paragraphs\>."

3. **Abbina lo stile del tuo prompt all'output desiderato**

   Lo stile di formattazione utilizzato nel tuo prompt potrebbe influenzare lo stile di risposta di Claude. Se stai ancora riscontrando problemi di steerability con la formattazione dell'output, ti consigliamo di abbinare il più possibile lo stile del tuo prompt allo stile di output desiderato. Ad esempio, rimuovere markdown dal tuo prompt può ridurre il volume di markdown nell'output.

4. **Usa prompt dettagliati per preferenze di formattazione specifiche**

   Per un maggiore controllo sull'uso di markdown e formattazione, fornisci indicazioni esplicite:

```text Prompt di esempio per minimizzare markdown
<avoid_excessive_markdown_and_bullet_points>
Quando scrivi rapporti, documenti, spiegazioni tecniche, analisi o qualsiasi contenuto di lunga forma, scrivi in prosa chiara e fluida usando paragrafi e frasi complete. Usa interruzioni di paragrafo standard per l'organizzazione e riserva markdown principalmente per `codice inline`, blocchi di codice (```...```), e intestazioni semplici (###, e ###). Evita di usare **grassetto** e *corsivo*.

NON usare elenchi ordinati (1. ...) o elenchi non ordinati (*) a meno che: a) stai presentando elementi veramente discreti dove un formato di elenco è la migliore opzione, o b) l'utente richiede esplicitamente un elenco o una classificazione

Invece di elencare elementi con punti elenco o numeri, incorporali naturalmente nelle frasi. Questa indicazione si applica soprattutto alla scrittura tecnica. Usare prosa invece di formattazione eccessiva migliorerà la soddisfazione dell'utente. NON produrre mai una serie di punti elenco eccessivamente brevi.

Il tuo obiettivo è testo leggibile e fluido che guida il lettore naturalmente attraverso le idee piuttosto che frammentare le informazioni in punti isolati.
</avoid_excessive_markdown_and_bullet_points>
```

### Ricerca e raccolta di informazioni

I modelli Claude 4.5 dimostrano capacità di ricerca agentica eccezionali e possono trovare e sintetizzare informazioni da più fonti in modo efficace. Per risultati di ricerca ottimali:

1. **Fornisci criteri di successo chiari**: Definisci cosa costituisce una risposta di successo alla tua domanda di ricerca

2. **Incoraggia la verifica delle fonti**: Chiedi a Claude di verificare le informazioni su più fonti

3. **Per compiti di ricerca complessi, usa un approccio strutturato**:

```text Prompt di esempio per ricerca complessa
Cerca queste informazioni in modo strutturato. Man mano che raccogli dati, sviluppa diverse ipotesi concorrenti. Traccia i tuoi livelli di confidenza nelle note di progresso per migliorare la calibrazione. Critica regolarmente il tuo approccio e pianifica. Aggiorna un file di albero di ipotesi o note di ricerca per persistere le informazioni e fornire trasparenza. Suddividi sistematicamente questo compito di ricerca complesso.
```

Questo approccio strutturato consente a Claude di trovare e sintetizzare praticamente qualsiasi informazione e criticare iterativamente i suoi risultati, indipendentemente dalla dimensione del corpus.

### Orchestrazione di subagenti

I modelli Claude 4.5 dimostrano capacità di orchestrazione di subagenti nativi significativamente migliorate. Questi modelli possono riconoscere quando i compiti beneficerebbero dalla delega del lavoro a subagenti specializzati e lo fanno proattivamente senza richiedere istruzioni esplicite.

Per sfruttare questo comportamento:

1. **Assicura strumenti di subagenti ben definiti**: Avere strumenti di subagenti disponibili e descritti nelle definizioni di strumenti
2. **Lascia che Claude orchestri naturalmente**: Claude delegherà in modo appropriato senza istruzioni esplicite
3. **Regola la conservatività se necessario**:

```text Prompt di esempio per utilizzo conservativo di subagenti
Delega ai subagenti solo quando il compito chiaramente beneficia da un agente separato con una finestra di contesto nuova.
```

### Autoconoscenza del modello

Se desideri che Claude si identifichi correttamente nella tua applicazione o usi stringhe API specifiche:

```text Prompt di esempio per identità del modello
L'assistente è Claude, creato da Anthropic. Il modello attuale è Claude Sonnet 4.5.
```

Per app basate su LLM che devono specificare stringhe di modello:

```text Prompt di esempio per stringa di modello
Quando è necessario un LLM, per favore predefinito a Claude Sonnet 4.5 a meno che l'utente non richieda diversamente. La stringa di modello esatta per Claude Sonnet 4.5 è claude-sonnet-4-5-20250929.
```

### Sensibilità del pensiero

Quando il pensiero esteso è disabilitato, Claude Opus 4.5 è particolarmente sensibile alla parola "think" e alle sue varianti. Ti consigliamo di sostituire "think" con parole alternative che trasmettono significati simili, come "consider," "believe," e "evaluate."

### Sfrutta le capacità di pensiero e pensiero interlacciato

I modelli Claude 4.x offrono capacità di pensiero che possono essere particolarmente utili per compiti che coinvolgono la riflessione dopo l'uso di strumenti o ragionamento multi-step complesso. Puoi guidare il suo pensiero iniziale o interlacciato per risultati migliori.

```text Prompt di esempio
Dopo aver ricevuto i risultati degli strumenti, rifletti attentamente sulla loro qualità e determina i passaggi successivi ottimali prima di procedere. Usa il tuo pensiero per pianificare e iterare in base a queste nuove informazioni, e poi intraprendi la migliore azione successiva.
```

<Info>
  Per ulteriori informazioni sulle capacità di pensiero, consulta [Pensiero esteso](/docs/it/build-with-claude/extended-thinking).
</Info>

### Creazione di documenti

I modelli Claude 4.5 eccellono nella creazione di presentazioni, animazioni e documenti visivi. Questi modelli corrispondono o superano Claude Opus 4.1 in questo dominio, con un'impressionante creatività e un seguire le istruzioni più forte. I modelli producono output lucido e utilizzabile al primo tentativo nella maggior parte dei casi.

Per i migliori risultati con la creazione di documenti:

```text Prompt di esempio
Crea una presentazione professionale su [topic]. Includi elementi di design ponderati, gerarchia visiva e animazioni coinvolgenti dove appropriato.
```

### Capacità di visione migliorate

Claude Opus 4.5 ha capacità di visione migliorate rispetto ai modelli Claude precedenti. Si comporta meglio nei compiti di elaborazione delle immagini e estrazione dei dati, in particolare quando ci sono più immagini presenti nel contesto. Questi miglioramenti si estendono all'utilizzo del computer, dove il modello può interpretare più affidabilmente gli screenshot e gli elementi dell'interfaccia utente. Puoi anche usare Claude Opus 4.5 per analizzare i video suddividendoli in fotogrammi.

Una tecnica che abbiamo trovato efficace per aumentare ulteriormente le prestazioni è dare a Claude Opus 4.5 uno strumento di ritaglio o una [competenza](/docs/it/agents-and-tools/agent-skills/overview). Abbiamo visto un miglioramento coerente nelle valutazioni delle immagini quando Claude è in grado di "ingrandire" le regioni rilevanti di un'immagine. Abbiamo messo insieme un cookbook per lo strumento di ritaglio [qui](https://github.com/anthropics/claude-cookbooks/blob/main/multimodal/crop_tool.ipynb).

### Ottimizza le chiamate di strumenti parallele

I modelli Claude 4.x eccellono nell'esecuzione di strumenti paralleli, con Sonnet 4.5 che è particolarmente aggressivo nel lanciare più operazioni simultaneamente. I modelli Claude 4.x:

- Eseguono più ricerche speculative durante la ricerca
- Leggono diversi file contemporaneamente per costruire il contesto più velocemente
- Eseguono comandi bash in parallelo (che possono anche creare un collo di bottiglia nelle prestazioni del sistema)

Questo comportamento è facilmente steerable. Sebbene il modello abbia un alto tasso di successo nelle chiamate di strumenti paralleli senza prompt, puoi aumentare questo a ~100% o regolare il livello di aggressività:

```text Prompt di esempio per massima efficienza parallela
<use_parallel_tool_calls>
Se intendi chiamare più strumenti e non ci sono dipendenze tra le chiamate di strumenti, fai tutte le chiamate di strumenti indipendenti in parallelo. Dai priorità alle chiamate di strumenti simultanee ogni volta che le azioni possono essere eseguite in parallelo piuttosto che sequenzialmente. Ad esempio, quando leggi 3 file, esegui 3 chiamate di strumenti in parallelo per leggere tutti e 3 i file nel contesto contemporaneamente. Massimizza l'uso di chiamate di strumenti parallele dove possibile per aumentare la velocità e l'efficienza. Tuttavia, se alcune chiamate di strumenti dipendono da chiamate precedenti per informare valori dipendenti come i parametri, NON chiamare questi strumenti in parallelo e invece chiamarli sequenzialmente. Non usare mai segnaposti o indovinare parametri mancanti nelle chiamate di strumenti.
</use_parallel_tool_calls>
```

```text Prompt di esempio per ridurre l'esecuzione parallela
Esegui le operazioni sequenzialmente con brevi pause tra ogni passaggio per garantire la stabilità.
```

### Riduci la creazione di file nella codifica agentica

I modelli Claude 4.x a volte potrebbero creare nuovi file per scopi di test e iterazione, in particolare quando si lavora con il codice. Questo approccio consente a Claude di usare file, in particolare script Python, come un "blocco note temporaneo" prima di salvare l'output finale. L'uso di file temporanei può migliorare i risultati in particolare per i casi di utilizzo della codifica agentica.

Se preferisci minimizzare la creazione netta di nuovi file, puoi istruire Claude a pulire dopo se stesso:

```text Prompt di esempio
Se crei file temporanei nuovi, script o file helper per l'iterazione, pulisci questi file rimuovendoli alla fine del compito.
```

### Eccessivo entusiasmo e creazione di file

Claude Opus 4.5 ha una tendenza a sovraingegnerizzare creando file extra, aggiungendo astrazioni non necessarie, o costruendo flessibilità che non era stata richiesta. Se stai vedendo questo comportamento indesiderato, aggiungi prompt esplicito per mantenere le soluzioni minime.

Ad esempio:

```text Prompt di esempio per minimizzare l'overengineering
Evita l'overengineering. Apporta solo i cambiamenti che sono direttamente richiesti o chiaramente necessari. Mantieni le soluzioni semplici e focalizzate.

Non aggiungere funzionalità, refactorizzare il codice, o fare "miglioramenti" oltre quello che è stato chiesto. Una correzione di bug non ha bisogno di codice circostante pulito. Una funzionalità semplice non ha bisogno di configurabilità extra.

Non aggiungere gestione degli errori, fallback, o validazione per scenari che non possono accadere. Fidati delle garanzie del codice interno e del framework. Valida solo ai confini del sistema (input dell'utente, API esterne). Non usare shim di compatibilità all'indietro quando puoi semplicemente cambiare il codice.

Non creare helper, utilità, o astrazioni per operazioni una tantum. Non progettare per requisiti futuri ipotetici. La giusta quantità di complessità è il minimo necessario per il compito attuale. Riutilizza le astrazioni esistenti dove possibile e segui il principio DRY.
```

### Design del frontend

I modelli Claude 4.x, in particolare Opus 4.5, eccellono nella costruzione di applicazioni web complesse e reali con un forte design del frontend. Tuttavia, senza indicazioni, i modelli possono predefinire modelli generici che creano quello che gli utenti chiamano l'estetica "AI slop". Per creare frontend distintivi e creativi che sorprendono e deliziano:

<Tip>
Per una guida dettagliata sul miglioramento del design del frontend, consulta il nostro post del blog su [miglioramento del design del frontend attraverso le competenze](https://www.claude.com/blog/improving-frontend-design-through-skills).
</Tip>

Ecco uno snippet di prompt di sistema che puoi usare per incoraggiare un migliore design del frontend:

```text Prompt di esempio per estetica del frontend
<frontend_aesthetics>
Tendi a convergere verso output generici e "on distribution". Nel design del frontend, questo crea quello che gli utenti chiamano l'estetica "AI slop". Evita questo: crea frontend creativi e distintivi che sorprendono e deliziano.

Concentrati su:
- Tipografia: Scegli font che sono belli, unici e interessanti. Evita font generici come Arial e Inter; opta invece per scelte distintive che elevino l'estetica del frontend.
- Colore e tema: Impegnati in un'estetica coerente. Usa variabili CSS per la coerenza. I colori dominanti con accenti nitidi superano le tavolozze timide e uniformemente distribuite. Trai ispirazione dai temi IDE e dalle estetiche culturali.
- Movimento: Usa le animazioni per effetti e micro-interazioni. Dai priorità alle soluzioni solo CSS per HTML. Usa la libreria Motion per React quando disponibile. Concentrati su momenti ad alto impatto: un caricamento di pagina ben orchestrato con rivela sfalsate (animation-delay) crea più delizia di micro-interazioni sparse.
- Sfondi: Crea atmosfera e profondità piuttosto che predefinire colori solidi. Stratifica gradienti CSS, usa modelli geometrici, o aggiungi effetti contestuali che corrispondono all'estetica generale.

Evita estetiche generiche generate da IA:
- Famiglie di font overused (Inter, Roboto, Arial, font di sistema)
- Schemi di colore clichéd (in particolare gradienti viola su sfondi bianchi)
- Layout e modelli di componenti prevedibili
- Design generico che manca di carattere specifico del contesto

Interpreta creativamente e fai scelte inaspettate che si sentono genuinamente progettate per il contesto. Varia tra temi chiari e scuri, diversi font, diverse estetiche. Tendi ancora a convergere su scelte comuni (Space Grotesk, ad esempio) tra le generazioni. Evita questo: è critico che tu pensi fuori dagli schemi!
</frontend_aesthetics>
```

Puoi anche fare riferimento alla competenza completa [qui](https://github.com/anthropics/claude-code/blob/main/plugins/frontend-design/skills/frontend-design/SKILL.md).

### Evita di concentrarsi sul passaggio dei test e sulla codifica fissa

I modelli Claude 4.x a volte possono concentrarsi troppo pesantemente sul far passare i test a spese di soluzioni più generali, o possono usare workaround come script helper per il refactoring complesso invece di usare strumenti standard direttamente. Per prevenire questo comportamento e garantire soluzioni robuste e generalizzabili:

```text Prompt di esempio
Per favore scrivi una soluzione di alta qualità e di uso generale utilizzando gli strumenti standard disponibili. Non creare script helper o workaround per completare il compito in modo più efficiente. Implementa una soluzione che funziona correttamente per tutti gli input validi, non solo i casi di test. Non codificare valori o creare soluzioni che funzionano solo per input di test specifici. Invece, implementa la logica effettiva che risolve il problema in generale.

Concentrati sulla comprensione dei requisiti del problema e sull'implementazione dell'algoritmo corretto. I test sono lì per verificare la correttezza, non per definire la soluzione. Fornisci un'implementazione principiata che segue le migliori pratiche e i principi di progettazione del software.

Se il compito è irragionevole o infattibile, o se uno qualsiasi dei test è scorretto, per favore informami piuttosto che aggirare. La soluzione dovrebbe essere robusta, mantenibile ed estensibile.
```

### Incoraggiare l'esplorazione del codice

Claude Opus 4.5 è altamente capace ma può essere eccessivamente conservatore quando esplora il codice. Se noti che il modello propone soluzioni senza guardare il codice o fa supposizioni sul codice che non ha letto, la migliore soluzione è aggiungere istruzioni esplicite al prompt. Claude Opus 4.5 è il nostro modello più steerable fino ad oggi e risponde in modo affidabile alla guida diretta.

Ad esempio:

```text Prompt di esempio per esplorazione del codice
SEMPRE leggi e comprendi i file rilevanti prima di proporre modifiche al codice. Non speculare sul codice che non hai ispezionato. Se l'utente fa riferimento a un file/percorso specifico, DEVI aprire e ispezionarlo prima di spiegare o proporre correzioni. Sii rigoroso e persistente nella ricerca del codice per fatti chiave. Rivedi a fondo lo stile, le convenzioni e le astrazioni della base di codice prima di implementare nuove funzionalità o astrazioni.
```

### Minimizzare le allucinazioni nella codifica agentica

I modelli Claude 4.x sono meno soggetti a allucinazioni e forniscono risposte più accurate, radicate e intelligenti basate sul codice. Per incoraggiare ancora di più questo comportamento e minimizzare le allucinazioni:

```text Prompt di esempio
<investigate_before_answering>
Non speculare mai sul codice che non hai aperto. Se l'utente fa riferimento a un file specifico, DEVI leggere il file prima di rispondere. Assicurati di investigare e leggere i file rilevanti PRIMA di rispondere alle domande sulla base di codice. Non fare mai affermazioni sul codice prima di investigare a meno che tu non sia certo della risposta corretta - fornisci risposte radicate e libere da allucinazioni.
</investigate_before_answering>
```

## Considerazioni sulla migrazione

Quando si esegue la migrazione ai modelli Claude 4.5:

1. **Sii specifico sul comportamento desiderato**: Considera di descrivere esattamente cosa vorresti vedere nell'output.

2. **Inquadra le tue istruzioni con modificatori**: Aggiungere modificatori che incoraggiano Claude ad aumentare la qualità e il dettaglio del suo output può aiutare a modellare meglio le prestazioni di Claude. Ad esempio, invece di "Crea una dashboard di analisi", usa "Crea una dashboard di analisi. Includi il maggior numero possibile di funzionalità e interazioni rilevanti. Vai oltre le basi per creare un'implementazione completamente funzionale."

3. **Richiedi funzionalità specifiche esplicitamente**: Le animazioni e gli elementi interattivi dovrebbero essere richiesti esplicitamente quando desiderati.