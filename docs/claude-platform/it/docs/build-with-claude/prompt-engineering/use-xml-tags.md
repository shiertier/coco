# Usa tag XML per strutturare i tuoi prompt

Scopri come usare i tag XML per strutturare i tuoi prompt e migliorare la qualità degli output di Claude.

---

<Note>
While these tips apply broadly to all Claude models, you can find prompting tips specific to extended thinking models [here](/docs/en/build-with-claude/prompt-engineering/extended-thinking-tips).
</Note>

Quando i tuoi prompt coinvolgono più componenti come contesto, istruzioni ed esempi, i tag XML possono essere un game-changer. Aiutano Claude a analizzare i tuoi prompt in modo più accurato, portando a output di qualità superiore.

<Tip>**Consiglio XML**: Usa tag come `<instructions>`, `<example>` e `<formatting>` per separare chiaramente le diverse parti del tuo prompt. Questo impedisce a Claude di confondere le istruzioni con gli esempi o il contesto.</Tip>

## Perché usare i tag XML?

- **Chiarezza:** Separa chiaramente le diverse parti del tuo prompt e assicurati che il tuo prompt sia ben strutturato.
- **Accuratezza:** Riduci gli errori causati da Claude che fraintende parti del tuo prompt.
- **Flessibilità:** Trova, aggiungi, rimuovi o modifica facilmente parti del tuo prompt senza riscrivere tutto.
- **Parseabilità:** Far usare a Claude i tag XML nel suo output rende più facile estrarre parti specifiche della sua risposta tramite post-elaborazione.

<Note>Non ci sono tag XML "canonici" migliori con cui Claude è stato addestrato in particolare, anche se consigliamo che i nomi dei tuoi tag abbiano senso con le informazioni che circondano.</Note>

***

## Best practice per l'etichettatura

1. **Sii coerente**: Usa gli stessi nomi di tag in tutti i tuoi prompt e fai riferimento a quei nomi di tag quando parli del contenuto (ad es., `Usando il contratto nei tag <contract>...`).
2. **Annida i tag**: Dovresti annidare i tag `<outer><inner></inner></outer>` per il contenuto gerarchico.

<Tip>**Consiglio per utenti esperti**: Combina i tag XML con altre tecniche come il multishot prompting (`<examples>`) o la catena di pensiero (`<thinking>`, `<answer>`). Questo crea prompt super-strutturati e ad alte prestazioni.</Tip>

### Esempi

  <section title="Esempio: Generazione di rapporti finanziari">

    Senza tag XML, Claude fraintende il compito e genera un rapporto che non corrisponde alla struttura o al tono richiesto. Dopo la sostituzione, c'è anche la possibilità che Claude fraintenda dove una sezione (come l'esempio del rapporto Q1) finisce e un'altra inizia.

    | Ruolo | Senza tag XML | Con tag XML |
    | ---- | ----------- | ------------- |
    | Utente | Sei un analista finanziario presso AcmeCorp. Genera un rapporto finanziario Q2 per i nostri investitori. Includi sezioni su Crescita dei Ricavi, Margini di Profitto e Flusso di Cassa, come con questo esempio dello scorso anno: \{\{Q1_REPORT\}\}. Usa punti dati da questo foglio di calcolo: \{\{SPREADSHEET_DATA\}\}. Il rapporto dovrebbe essere estremamente conciso, diretto, professionale e in formato elenco. Dovrebbe evidenziare sia i punti di forza che le aree di miglioramento. | Sei un analista finanziario presso AcmeCorp. Genera un rapporto finanziario Q2 per i nostri investitori.<br/><br/>AcmeCorp è un'azienda B2B SaaS. I nostri investitori apprezzano la trasparenza e gli insight azionabili.<br/><br/>Usa questi dati per il tuo rapporto:\<data>\{\{SPREADSHEET_DATA\}\}\</data><br/><br/>\<instructions><br/>1. Includi sezioni: Crescita dei Ricavi, Margini di Profitto, Flusso di Cassa.<br/>2. Evidenzia i punti di forza e le aree di miglioramento.<br/>\</instructions><br/><br/>Rendi il tuo tono conciso e professionale. Segui questa struttura:<br/>\<formatting_example>\{\{Q1_REPORT\}\}\</formatting_example> |
    | Assistente | Rapporto Finanziario Q2 per gli Investitori di AcmeCorp<br/><br/>Cari Investitori Stimati,<br/><br/>Sono lieto di presentare il nostro rapporto finanziario Q2, che mostra la continua crescita e resilienza di AcmeCorp nel mercato B2B SaaS.<br/><br/>1. Crescita dei Ricavi:<br/>I nostri ricavi Q2 hanno raggiunto \$15,2M, un aumento YoY del 22%. Questa crescita è principalmente guidata da un'espansione del 30% della nostra base di clienti enterprise. Tuttavia, il nostro segmento SMB ha visto un leggero calo del 5%, che miriamo ad affrontare attraverso iniziative di marketing mirate nel Q3.<br/><br/>2. Margini di Profitto:<br/>Il margine lordo è migliorato al 72% (su del 3% rispetto a Q1) grazie alle ottimizzazioni dell'infrastruttura. Il margine EBITDA, tuttavia, è sceso al 18% (giù del 2%) poiché abbiamo investito pesantemente in R&D per le nostre prossime funzionalità basate su IA.<br/><br/>3. Flusso di Cassa:<br/>Il flusso di cassa operativo rimane forte a \$4,1M. Le nostre riserve di cassa si attestano a \$28M, fornendo una runway di 15 mesi al tasso di burn attuale. Abbiamo visto un aumento del 20% nei costi di acquisizione dei clienti, che stiamo affrontando attraverso strategie di vendita più efficienti.<br/><br/>In sintesi, i dati finanziari principali di AcmeCorp rimangono robusti. Stiamo bilanciando gli investimenti di crescita con una gestione prudente della liquidità. Il nostro focus per Q3 sarà rivitalizzare la crescita SMB e ottimizzare l'efficienza delle vendite.<br/><br/>Grazie per la vostra continua fiducia.<br/><br/>Cordiali saluti,<br/>CFO, AcmeCorp | Rapporto Finanziario Q2 per gli Investitori di AcmeCorp\</output><br/><br/>Crescita dei Ricavi:<br/>- Ricavi Q2: \$15,2M (22% YoY ↑)<br/>- Clienti enterprise: 30% ↑<br/>- Segmento SMB: 5% ↓<br/>  Azione: Marketing mirato Q3<br/><br/>Margini di Profitto:<br/>- Margine lordo: 72% (3% ↑ da Q1)<br/>  Motivo: Ottimizzazioni dell'infrastruttura<br/>- Margine EBITDA: 18% (2% ↓)<br/>  Motivo: Investimento in R&D per funzionalità IA<br/><br/>Flusso di Cassa:<br/>- Flusso di cassa operativo: \$4,1M (forte)<br/>- Riserve di cassa: \$28M (runway di 15 mesi)<br/>- Costo di acquisizione dei clienti: 20% ↑<br/>  Azione: Ottimizza strategie di vendita<br/><br/>Prospettive:<br/>- Dati finanziari principali: Robusti<br/>- Focus Q3: Rivitalizza SMB, ottimizza vendite<br/><br/>La gestione rimane impegnata nel bilanciare gli investimenti di crescita con la disciplina finanziaria. |
  
</section>
  <section title="Esempio: Analisi di contratti legali">

    Senza tag XML, l'analisi di Claude è disorganizzata e perde punti chiave. Con i tag, fornisce un'analisi strutturata e approfondita su cui un team legale può agire.

    | Ruolo | Senza tag XML | Con tag XML |
    | ---- | ----------- | ------------- |
    | Utente | Analizza questo accordo di licenza software per potenziali rischi e responsabilità: \{\{CONTRACT\}\}. Concentrati sulle clausole di indennizzo, limitazione della responsabilità e proprietà intellettuale. Inoltre, nota eventuali termini insoliti o preoccupanti. Ecco il nostro contratto standard come riferimento: \{\{STANDARD_CONTRACT\}\}. Fornisci un riepilogo dei risultati e delle raccomandazioni per il nostro team legale. | Analizza questo accordo di licenza software per rischi legali e responsabilità.<br/><br/>Siamo un'impresa multinazionale che sta considerando questo accordo per la nostra infrastruttura dati principale.<br/><br/>\<agreement><br/>\{\{CONTRACT\}\}\</agreement><br/><br/>Questo è il nostro contratto standard come riferimento:<br/>\<standard_contract>\{\{STANDARD_CONTRACT\}\}\</standard_contract><br/><br/>\<instructions><br/>1. Analizza queste clausole:<br/>   - Indennizzo<br/>   - Limitazione della responsabilità<br/>   - Proprietà intellettuale<br/><br/>2. Nota termini insoliti o preoccupanti.<br/><br/>3. Confronta con il nostro contratto standard.<br/><br/>4. Riassumi i risultati nei tag \<findings>.<br/><br/>5. Elenca le raccomandazioni attuabili nei tag \<recommendations>.\</instructions> |
    | Assistente | L'accordo di licenza software fornito presenta diversi aspetti preoccupanti. La clausola di indennizzo è eccessivamente ampia, esponendoci potenzialmente a responsabilità significative. La sezione di limitazione della responsabilità limita i danni a soli \$500, il che è inadeguato per il nostro utilizzo a livello enterprise. Per quanto riguarda la proprietà intellettuale, l'accordo concede al fornitore la proprietà congiunta di eventuali modifiche o integrazioni che creiamo.<br/><br/>In sintesi, questo accordo non è nel nostro interesse. Ci espone a rischi indebiti, limita il nostro ricorso in caso di guasti e compromette i nostri diritti di proprietà intellettuale. Consiglio di rinegoziare questi termini o cercare fornitori alternativi. | \<findings><br/>1. Indennizzo (Clausola 8):<br/>   - Problema: Eccessivamente ampio. Indenizziamo il fornitore contro "qualsiasi e tutti i reclami" derivanti dal nostro utilizzo.<br/>   - Rischio: Potrebbe renderci responsabili anche per negligenza del fornitore o difetti del prodotto.<br/>   - Deviazione: Il nostro standard limita l'indennizzo ai reclami derivanti dalla nostra violazione dell'accordo.<br/><br/>2. Limitazione della Responsabilità (Clausola 9):<br/>   - Problema: Limita tutti i danni a \$500, inclusi i danni diretti.<br/>   - Rischio: Grossolanamente inadeguato per l'uso enterprise. Un guasto critico potrebbe costare milioni.<br/>   - Deviazione: Il nostro standard consente danni diretti fino a 12 mesi di commissioni pagate.<br/><br/>3. Proprietà Intellettuale (Clausola 12):<br/>   - Problema: Concede al fornitore la proprietà congiunta delle nostre modifiche/integrazioni.<br/>   - Rischio: Il fornitore potrebbe utilizzare o vendere i nostri algoritmi proprietari o insight sui dati.<br/>   - Deviazione: Il nostro standard mantiene la piena proprietà del nostro lavoro.<br/><br/>4. Termini Insoliti:<br/>   - Clausola 5.3: Il fornitore può utilizzare i nostri dati di utilizzo per "qualsiasi scopo".<br/>   - Clausola 7.2: Il fornitore può rescindere per "convenienza" con solo 7 giorni di preavviso.<br/>   - Clausola 11: Nessuna garanzia di alcun tipo, nemmeno per la commerciabilità di base.<br/>\</findings><br/><br/>\<recommendations><br/>1. Rifiuta questo accordo. I rischi superano di gran lunga i benefici per un'impresa.<br/>2. Controproponi:<br/>   - Indennizzo: Limita ai reclami derivanti dalla nostra violazione dell'accordo.<br/>   - Responsabilità: Limita a 12 mesi di commissioni per danni diretti; negozia per danni indiretti.<br/>   - Proprietà Intellettuale: Piena proprietà delle nostre modifiche/integrazioni.<br/>   - Utilizzo dei Dati: Limita al miglioramento del servizio solo.<br/>   - Rescissione: Minimo 90 giorni di preavviso; solo per causa.<br/>   - Garanzie: Richiedi commerciabilità di base e idoneità allo scopo.<br/>3. Se il fornitore rifiuta, esplora fornitori alternativi o soluzioni interne.<br/>4. Coinvolgi consulenti legali per la negoziazione del contratto data la posta in gioco elevata.<br/>\</recommendations> |
  
</section>

***

<CardGroup cols={3}>
  <Card title="Libreria di prompt" icon="link" href="/docs/it/resources/prompt-library/library">
    Ispirati da una selezione curata di prompt per vari compiti e casi d'uso.
  </Card>
  <Card title="Tutorial di prompting GitHub" icon="link" href="https://github.com/anthropics/prompt-eng-interactive-tutorial">
    Un tutorial ricco di esempi che copre i concetti di prompt engineering trovati nella nostra documentazione.
  </Card>
  <Card title="Tutorial di prompting Google Sheets" icon="link" href="https://docs.google.com/spreadsheets/d/19jzLgRruG9kjUQNKtCg1ZjdD6l6weA6qRXG5zLIAhC8">
    Una versione più leggera del nostro tutorial di prompt engineering tramite un foglio di calcolo interattivo.
  </Card>
</CardGroup>