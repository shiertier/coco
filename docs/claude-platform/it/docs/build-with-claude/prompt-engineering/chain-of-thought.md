# Lascia che Claude pensi (prompting con catena di pensiero) per aumentare le prestazioni

Scopri come utilizzare il prompting con catena di pensiero per migliorare le prestazioni di Claude su compiti complessi come ricerca, analisi e risoluzione dei problemi.

---

<Note>
While these tips apply broadly to all Claude models, you can find prompting tips specific to extended thinking models [here](/docs/en/build-with-claude/prompt-engineering/extended-thinking-tips).
</Note>

Quando affronti compiti complessi come ricerca, analisi o risoluzione dei problemi, dare a Claude lo spazio per pensare può migliorare drasticamente le sue prestazioni. Questa tecnica, nota come prompting con catena di pensiero (CoT), incoraggia Claude a scomporre i problemi passo dopo passo, portando a output più accurati e sfumati.

## Prima di implementare CoT

### Perché lasciare che Claude pensi?
- **Accuratezza:** Affrontare i problemi passo dopo passo riduce gli errori, soprattutto in matematica, logica, analisi o compiti generalmente complessi.
- **Coerenza:** Il pensiero strutturato porta a risposte più coese e ben organizzate.
- **Debug:** Vedere il processo di pensiero di Claude ti aiuta a individuare dove i prompt potrebbero essere poco chiari.

### Perché non lasciare che Claude pensi?
- L'aumento della lunghezza dell'output può influire sulla latenza.
- Non tutti i compiti richiedono un pensiero approfondito. Usa CoT con giudizio per garantire il giusto equilibrio tra prestazioni e latenza.

<Tip>Usa CoT per compiti che un umano dovrebbe affrontare pensandoci, come matematica complessa, analisi multi-step, scrittura di documenti complessi o decisioni con molti fattori.</Tip>

***

## Come richiedere il pensiero

Le tecniche di catena di pensiero di seguito sono **ordinate da meno a più complesse**. I metodi meno complessi occupano meno spazio nella finestra di contesto, ma sono generalmente anche meno potenti.

<Tip>**Suggerimento CoT**: Fai sempre in modo che Claude produca il suo pensiero. Senza produrre il suo processo di pensiero, non si verifica alcun pensiero!</Tip>

- **Prompt di base**: Includi "Pensa passo dopo passo" nel tuo prompt.
    - Manca la guida su *come* pensare (il che è particolarmente non ideale se un compito è molto specifico della tua app, caso d'uso o organizzazione)
    <section title="Esempio: Scrittura di email ai donatori (CoT di base)">

        | Ruolo | Contenuto |
        | ---- | ------- |
        | Utente | Redigi email personalizzate ai donatori chiedendo contributi al programma Care for Kids di quest'anno.<br/><br/>Informazioni sul programma:<br/>\\{\{PROGRAM_DETAILS}}<br/>\</program><br/><br/>Informazioni sul donatore:<br/>\<donor>\{\{DONOR_DETAILS}}<br/>\</donor><br/><br/>Pensa passo dopo passo prima di scrivere l'email. |
    
</section>
- **Prompt guidato**: Delinea i passaggi specifici che Claude deve seguire nel suo processo di pensiero.
    - Manca la strutturazione per rendere facile estrarre e separare la risposta dal pensiero.
    <section title="Esempio: Scrittura di email ai donatori (CoT guidato)">

        | Ruolo | Contenuto |
        | ---- | ------- |
        | Utente | Redigi email personalizzate ai donatori chiedendo contributi al programma Care for Kids di quest'anno.<br/><br/>Informazioni sul programma:<br/>\\{\{PROGRAM_DETAILS}}<br/>\</program><br/><br/>Informazioni sul donatore:<br/>\<donor>\{\{DONOR_DETAILS}}<br/>\</donor><br/><br/>Pensa prima di scrivere l'email. Per prima cosa, rifletti su quale messaggio potrebbe attirare questo donatore dato la sua storia di donazioni e quali campagne ha supportato in passato. Poi, rifletti su quali aspetti del programma Care for Kids potrebbero attirarlo, data la sua storia. Infine, scrivi l'email personalizzata al donatore utilizzando la tua analisi. |
    
</section>
- **Prompt strutturato**: Usa tag XML come `<thinking>` e `<answer>` per separare il ragionamento dalla risposta finale.
     <section title="Esempio: Scrittura di email ai donatori (CoT guidato strutturato)">

        | Ruolo | Contenuto |
        | ---- | ------- |
        | Utente | Redigi email personalizzate ai donatori chiedendo contributi al programma Care for Kids di quest'anno.<br/><br/>Informazioni sul programma:<br/>\\{\{PROGRAM_DETAILS}}<br/>\</program><br/><br/>Informazioni sul donatore:<br/>\<donor>\{\{DONOR_DETAILS}}<br/>\</donor><br/><br/>Pensa prima di scrivere l'email nei tag \<thinking>. Per prima cosa, rifletti su quale messaggio potrebbe attirare questo donatore dato la sua storia di donazioni e quali campagne ha supportato in passato. Poi, rifletti su quali aspetti del programma Care for Kids potrebbero attirarlo, data la sua storia. Infine, scrivi l'email personalizzata al donatore nei tag \<email>, utilizzando la tua analisi. |
    
</section>
    
### Esempi

  <section title="Esempio: Analisi finanziaria senza pensiero">

    Senza pensiero passo dopo passo, Claude fornisce una raccomandazione che sembra ragionevole ma manca di profondità. Non quantifica i risultati potenziali né considera la performance storica del mercato, che sono cruciali per prendere una decisione consapevole.
    | Ruolo | Contenuto |
    | ---- | ------- |
    | Utente | Sei un consulente finanziario. Un cliente vuole investire \$10.000. Può scegliere tra due opzioni: A) Un'azione che storicamente rende il 12% annuo ma è volatile, oppure B) Un'obbligazione che garantisce il 6% annuo. Il cliente ha bisogno del denaro tra 5 anni per un acconto su una casa. Quale opzione consigli? |
    | Assistente | In base alle informazioni fornite, consiglierei l'opzione B, l'obbligazione che garantisce un rendimento annuo del 6%. Ecco perché:<br/><br/>1. Certezza: L'obbligazione offre un rendimento annuo garantito del 6%, che fornisce certezza nella pianificazione finanziaria. Questo è cruciale quando il cliente ha un obiettivo finanziario specifico (acconto) con una timeline fissa (5 anni).<br/>2. Avversione al rischio: Poiché il cliente ha bisogno di questo denaro per un evento importante della vita (acquistare una casa), probabilmente ha una bassa tolleranza al rischio. La volatilità del mercato azionario potrebbe compromettere i suoi piani se c'è una flessione vicino al momento in cui ha bisogno dei fondi.<br/>3. Orizzonte temporale: Con soli 5 anni, c'è meno tempo per recuperare da potenziali flessioni del mercato azionario. Orizzonti di investimento più lunghi (10+ anni) sono generalmente più adatti per affrontare la volatilità del mercato azionario.<br/><br/>Mentre il rendimento storico del 12% dell'azione è attraente, la performance passata non garantisce risultati futuri. Il 6% garantito dall'obbligazione assicura che il cliente avrà i fondi di cui ha bisogno quando è il momento di fare l'acconto. |
  
</section>
  <section title="Esempio: Analisi finanziaria con pensiero">

    Con pensiero passo dopo passo, il consulente fornisce un'analisi molto più ricca. Calcola cifre esatte per entrambi gli scenari, considera la volatilità storica del mercato e analizza profondamente la tolleranza al rischio del cliente. Questo ragionamento approfondito porta a una raccomandazione più sicura e giustificabile.
    | Ruolo | Contenuto |
    | ---- | ------- |
    | Utente | Sei un consulente finanziario. Un cliente vuole investire \$10.000. Può scegliere tra due opzioni: A) Un'azione che storicamente rende il 12% annuo ma è volatile, oppure B) Un'obbligazione che garantisce il 6% annuo. Il cliente ha bisogno del denaro tra 5 anni per un acconto su una casa. Quale opzione consigli? Pensa passo dopo passo. |
    | Assistente | Per determinare l'opzione di investimento migliore per il nostro cliente, considererò diversi fattori passo dopo passo:<br/><br/>\<thinking><br/>1. Comprendi l'obiettivo e la timeline del cliente:<br/>   - Obiettivo: Acconto per una casa<br/>   - Timeline: 5 anni<br/>   - Implicazione: Questo è un obiettivo finanziario a breve termine e ad alta priorità. Il cliente probabilmente non può permettersi rischi significativi o volatilità.<br/><br/>2. Analizza l'Opzione A (Azione):<br/>   - Pro: Rendimento storico più elevato (12% annuo)<br/>   - Contro: Volatile (implicando potenziale per guadagni o perdite significativi)<br/>   - Calcola il rendimento potenziale di 5 anni:<br/>     - Caso migliore (12% annuo): \$10.000 * (1.12)^5 ≈ \$17.623,42<br/>     - Caso peggiore (crollo del mercato): Potrebbe perdere una porzione sostanziale, ad es. crollo del 50% = \$5.000<br/><br/>3. Analizza l'Opzione B (Obbligazione):<br/>   - Pro: Rendimento garantito (6% annuo)<br/>   - Contro: Rendimento inferiore rispetto alla performance storica delle azioni<br/>   - Calcola il rendimento di 5 anni:<br/>     - Garantito: \$10.000 * (1.06)^5 ≈ \$13.382,25<br/><br/>4. Considera la tolleranza al rischio:<br/>   - L'obiettivo è un acconto per una casa (essenziale, non discrezionale)<br/>   - Timeline breve (5 anni) limita la capacità di recuperare da flessioni del mercato<br/>   - Conclusione: Il cliente probabilmente ha una bassa tolleranza al rischio<br/><br/>5. Prospettiva storica:<br/>   - Il 12% dell'azione è storico, non garantito<br/>   - In periodi di 5 anni, il mercato azionario può variare selvaggiamente:<br/>     - 2003-2007 (pre-crisi finanziaria): S&P 500 in rialzo ~82%<br/>     - 2007-2011 (durante la crisi): S&P 500 in ribasso ~2%<br/>   - Conclusione: La performance passata non garantisce risultati futuri, soprattutto a breve termine<br/><br/>6. Analisi finale:<br/>   - Opzione A (Azione): Rendimento potenziale più elevato ma rischio inaccettabile di non raggiungere l'obiettivo dell'acconto<br/>   - Opzione B (Obbligazione): Garantito di raggiungere ~80% dello scenario migliore delle azioni, 100% di certezza di avere i fondi<br/>\</thinking><br/><br/>\<answer><br/>Consiglio l'Opzione B, l'obbligazione con un rendimento annuo garantito del 6%. Mentre il rendimento storico del 12% dell'azione è allettante, l'obbligazione assicura che avrai \$13.382,25 tra 5 anni per l'acconto della tua casa. Data l'importanza e la timeline breve del tuo obiettivo, la volatilità dell'azione pone un rischio inaccettabile. L'obbligazione fornisce certezza, che è inestimabile per un traguardo finanziario così cruciale.<br/>\</answer> |
  
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