# Usa esempi (multishot prompting) per guidare il comportamento di Claude

Scopri come utilizzare esempi e il multishot prompting per migliorare l'accuratezza e la coerenza degli output di Claude.

---

<Note>
While these tips apply broadly to all Claude models, you can find prompting tips specific to extended thinking models [here](/docs/en/build-with-claude/prompt-engineering/extended-thinking-tips).
</Note>

Gli esempi sono la tua arma segreta per ottenere esattamente quello che hai bisogno da Claude. Fornendo alcuni esempi ben realizzati nel tuo prompt, puoi migliorare drasticamente l'accuratezza, la coerenza e la qualità degli output di Claude.
Questa tecnica, nota come few-shot o multishot prompting, è particolarmente efficace per compiti che richiedono output strutturati o l'aderenza a formati specifici.

<Tip>**Potenzia i tuoi prompt**: Includi 3-5 esempi diversi e rilevanti per mostrare a Claude esattamente quello che vuoi. Più esempi = migliori prestazioni, soprattutto per compiti complessi.</Tip>

## Perché usare gli esempi?

- **Accuratezza**: Gli esempi riducono le interpretazioni errate delle istruzioni.
- **Coerenza**: Gli esempi garantiscono una struttura e uno stile uniformi.
- **Prestazioni**: Esempi ben scelti aumentano la capacità di Claude di gestire compiti complessi.

## Creazione di esempi efficaci

Per la massima efficacia, assicurati che i tuoi esempi siano:
- **Rilevanti**: I tuoi esempi rispecchiano il tuo caso d'uso effettivo.
- **Diversi**: I tuoi esempi coprono casi limite e sfide potenziali, e variano abbastanza affinché Claude non colga involontariamente schemi non intenzionali.
- **Chiari**: I tuoi esempi sono racchiusi in tag `<example>` (se multipli, annidati all'interno di tag `<examples>`) per la struttura.

<Tip>Chiedi a Claude di valutare i tuoi esempi per rilevanza, diversità o chiarezza. Oppure fai generare a Claude più esempi in base al tuo set iniziale.</Tip>
<section title="Esempio: Analisi del feedback dei clienti">

    Nota che nella versione "senza esempi", Claude non elenca più opzioni per ogni categoria, nonostante la sua analisi scritta indichi che ci sono più categorie per determinati feedback. Include anche una spiegazione lunga quando potremmo non volerla.

    | Ruolo | Senza esempi | Con esempi |
    | --- | --- | --- |
    | Utente | Analizza questo feedback dei clienti e categorizza i problemi. Usa queste categorie: UI/UX, Performance, Feature Request, Integration, Pricing, e Other. Valuta anche il sentiment (Positive/Neutral/Negative) e la priorità (High/Medium/Low).<br/><br/>Ecco il feedback: \{\{FEEDBACK\}\} | Il nostro team CS è sopraffatto dal feedback non strutturato. Il tuo compito è analizzare il feedback e categorizzare i problemi per i nostri team di prodotto e ingegneria. Usa queste categorie: UI/UX, Performance, Feature Request, Integration, Pricing, e Other. Valuta anche il sentiment (Positive/Neutral/Negative) e la priorità (High/Medium/Low). Ecco un esempio:<br/><br/>\<example><br/>Input: Il nuovo dashboard è un disastro! Impiega un'eternità a caricarsi, e non riesco a trovare il pulsante di esportazione. Risolvilo ASAP!<br/>Category: UI/UX, Performance<br/>Sentiment: Negative<br/>Priority: High\</example><br/><br/>Ora, analizza questo feedback: \{\{FEEDBACK\}\} |
    | Assistente (risposta di Claude) | Ecco l'analisi del feedback dei clienti:<br/><br/>Issue: UI/UX<br/>Sentiment: Negative<br/>Priority: High<br/>Il cliente è frustrato dal nuovo design del dashboard, che trova disordinato e lento a caricarsi. Evidenzia specificamente il pulsante "esportazione" mancante, che è probabilmente una funzione critica. Questo problema deve essere affrontato come priorità alta.<br/><br/>Issue: Integration<br/>Sentiment: Positive<br/>Priority: Medium<br/>Il cliente è soddisfatto dell'integrazione Salesforce esistente ma vorrebbe vedere il supporto per Hubspot. Questa è una richiesta di funzione che potrebbe essere considerata per un aggiornamento futuro del prodotto.<br/><br/>Issue: Pricing<br/>Sentiment: Negative<br/>Priority: High<br/>Il cliente trova il prezzo irragionevole, soprattutto per le funzioni di base. Sta minacciando di passare a un concorrente, quindi questo è un problema ad alta priorità che deve essere rivisto.<br/><br/>Issue: Performance<br/>Sentiment Negative<br/>Priority: High<br/>Il cliente sta riscontrando problemi critici con l'app mobile, dove i caricamenti di file di grandi dimensioni causano costantemente arresti anomali. Questo è un problema significativo che influisce sulla capacità del cliente di utilizzare il prodotto in modo efficace e deve essere affrontato come priorità alta.<br/><br/>Issue: UI/UX<br/>Sentiment: Positive<br/>Priority: Low<br/>Il cliente è complessivamente soddisfatto del prodotto, evidenziando specificamente l'interfaccia utente intuitiva e l'eccellente supporto clienti. Questo è un feedback positivo che non richiede alcuna azione immediata. | Ecco l'analisi del feedback dei clienti:<br/><br/>1. Il nuovo dashboard è un disastro! Impiega un'eternità a caricarsi, e non riesco a trovare il pulsante di esportazione. Risolvilo ASAP!<br/>Category: UI/UX, Performance<br/>Sentiment: Negative<br/>Priority: High<br/><br/>2. Amo l'integrazione Salesforce! Ma sarebbe fantastico se potessi aggiungere anche Hubspot.<br/>Category: Integration, Feature Request<br/>Sentiment: Positive<br/>Priority: Medium<br/><br/>3. Il vostro prezzo è ridicolo! $99/mo per le funzioni di base? Sto passando a Competitor X.<br/>Category: Pricing<br/>Sentiment: Negative<br/>Priority: High<br/><br/>4. L'app mobile continua a bloccarsi quando provo a caricare file di grandi dimensioni (>50MB). Questo è critico per i nostri agenti sul campo.<br/>Category: Performance<br/>Sentiment: Negative<br/>Priority: High<br/><br/>5. Nel complesso, sono soddisfatto del prodotto. L'interfaccia utente è intuitiva e il supporto clienti è di prim'ordine. Continuate così!<br/>Category: UI/UX, Other (Customer Support)<br/>Sentiment: Positive<br/>Priority: Low |

</section>

***

<CardGroup cols={3}>
  <Card title="Libreria di prompt" icon="link" href="/docs/it/resources/prompt-library/library">
    Ispirati da una selezione curata di prompt per vari compiti e casi d'uso.
  </Card>
  <Card title="Tutorial GitHub prompting" icon="link" href="https://github.com/anthropics/prompt-eng-interactive-tutorial">
    Un tutorial ricco di esempi che copre i concetti di prompt engineering trovati nella nostra documentazione.
  </Card>
  <Card title="Tutorial Google Sheets prompting" icon="link" href="https://docs.google.com/spreadsheets/d/19jzLgRruG9kjUQNKtCg1ZjdD6l6weA6qRXG5zLIAhC8">
    Una versione più leggera del nostro tutorial di prompt engineering tramite un foglio di calcolo interattivo.
  </Card>
</CardGroup>