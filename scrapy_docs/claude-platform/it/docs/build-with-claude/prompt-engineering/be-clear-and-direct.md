# Sii chiaro, diretto e dettagliato

Come comunicare in modo efficace con Claude fornendo istruzioni chiare, contestuali e specifiche

---

<Note>
While these tips apply broadly to all Claude models, you can find prompting tips specific to extended thinking models [here](/docs/en/build-with-claude/prompt-engineering/extended-thinking-tips).
</Note>

Quando interagisci con Claude, pensalo come un dipendente brillante ma molto nuovo (con amnesia) che ha bisogno di istruzioni esplicite. Come qualsiasi nuovo dipendente, Claude non ha contesto sulle tue norme, stili, linee guida o modi preferiti di lavorare.
Più precisamente spieghi quello che vuoi, migliore sarà la risposta di Claude.

<Tip>**La regola d'oro della comunicazione chiara**<br/>Mostra il tuo prompt a un collega, idealmente qualcuno che ha un contesto minimo sul compito, e chiedigli di seguire le istruzioni. Se sono confusi, probabilmente lo sarà anche Claude.</Tip>

## Come essere chiaro, contestuale e specifico

- **Fornisci a Claude informazioni contestuali:** Proprio come potresti eseguire meglio un compito se conoscessi più contesto, Claude eseguirà meglio se ha più informazioni contestuali. Alcuni esempi di informazioni contestuali:
    - A cosa serviranno i risultati del compito
    - Quale pubblico è destinato l'output
    - Di quale flusso di lavoro fa parte il compito e dove si colloca in quel flusso di lavoro
    - L'obiettivo finale del compito, o come appare il completamento riuscito del compito
- **Sii specifico su quello che vuoi che Claude faccia:** Ad esempio, se vuoi che Claude produca solo codice e nient'altro, dillo.
- **Fornisci le istruzioni come passaggi sequenziali:** Usa elenchi numerati o punti elenco per assicurarti meglio che Claude esegua il compito esattamente come lo desideri.

### Esempi

    <section title="Esempio: Anonimizzazione del feedback dei clienti">

    Nota che Claude commette ancora errori nell'esempio di prompt poco chiaro, come lasciare il nome di un cliente.

    | Ruolo | Prompt poco chiaro | Prompt chiaro |
    | --- | --- | --- |
    | Utente | Per favore rimuovi tutte le informazioni di identificazione personale da questi messaggi di feedback dei clienti: \{\{FEEDBACK_DATA\}\} | Il tuo compito è anonimizzare il feedback dei clienti per la nostra revisione trimestrale.<br/><br/>Istruzioni:<br/>1. Sostituisci tutti i nomi dei clienti con "CUSTOMER_[ID]" (ad es. "Jane Doe" → "CUSTOMER_001").<br/>2. Sostituisci gli indirizzi email con "EMAIL_[ID]@example.com".<br/>3. Oscura i numeri di telefono come "PHONE_[ID]".<br/>4. Se un messaggio menziona un prodotto specifico (ad es. "AcmeCloud"), lascialo intatto.<br/>5. Se non viene trovato alcun PII, copia il messaggio testualmente.<br/>6. Produci solo i messaggi elaborati, separati da "---".<br/><br/>Dati da elaborare: \{\{FEEDBACK_DATA\}\} |
    | Assistente (risposta di Claude) | Ecco i messaggi di feedback dei clienti anonimizzati:<br/>1. Sto usando il vostro software da anni, ed è stato fantastico. Ma ultimamente, AcmeCloud si è bloccato molto. Risolvilo!<br/>2. XXX, il vostro rappresentante del servizio clienti, è stato incredibilmente utile. Mi ha guidato attraverso l'intero processo. La mia email è XXX se vuoi inviare un sondaggio.<br/>3. Sono John Smith, e non sono felice. Sono rimasto in attesa per 2 ore cercando di raggiungere qualcuno a XXX. Questo è inaccettabile! | CUSTOMER_001: Sto usando il vostro software da anni, ed è stato fantastico. Ma ultimamente, AcmeCloud si è bloccato molto. Risolvilo!<br/>---<br/>CUSTOMER_002: CUSTOMER_SERVICE_001, il vostro rappresentante del servizio clienti, è stato incredibilmente utile. Mi ha guidato attraverso l'intero processo. La mia email è EMAIL_002@example.com se vuoi inviare un sondaggio.<br/>---<br/>CUSTOMER_003: Sono CUSTOMER_003, e non sono felice. Sono rimasto in attesa per 2 ore cercando di raggiungere qualcuno a PHONE_001. Questo è inaccettabile! |
    
</section>
    <section title="Esempio: Creazione di una campagna di email marketing">

    Nota che Claude inventa dettagli per colmare le lacune dove manca contesto con il prompt vago.

    | Ruolo | Prompt vago | Prompt specifico |
    | --- | --- | --- |
    | Utente | Scrivi un'email di marketing per le nostre nuove funzionalità di AcmeCloud. | Il tuo compito è creare un'email di marketing mirata per il rilascio delle funzionalità Q3 di AcmeCloud.<br/><br/>Istruzioni:<br/>1. Scrivi per questo pubblico target: Aziende tecnologiche di medie dimensioni (100-500 dipendenti) che passano da on-prem a cloud.<br/>2. Evidenzia 3 funzionalità chiave nuove: crittografia dati avanzata, sincronizzazione multipiattaforma e collaborazione in tempo reale.<br/>3. Tono: Professionale ma accessibile. Enfatizza sicurezza, efficienza e lavoro di squadra.<br/>4. Includi un CTA chiaro: Prova gratuita di 30 giorni con onboarding prioritario.<br/>5. Riga dell'oggetto: Meno di 50 caratteri, menziona "sicurezza" e "collaborazione".<br/>6. Personalizzazione: Usa le variabili \{\{COMPANY_NAME\}\} e \{\{CONTACT_NAME\}\}.<br/><br/>Struttura:<br/>1. Riga dell'oggetto<br/>2. Corpo dell'email (150-200 parole)<br/>3. Testo del pulsante CTA |
    | Assistente (risposta di Claude) | Ecco una bozza di email di marketing per le nostre nuove funzionalità:<br/><br/>Oggetto: Potenzia la tua attività con le ultime funzionalità di AcmeCloud<br/><br/>Caro cliente stimato,<br/><br/>Siamo entusiasti di introdurre tre potenti funzionalità in AcmeCloud che rivoluzionaranno il tuo flusso di lavoro:<br/><br/>1. Collaborazione in tempo reale: Lavora sui documenti contemporaneamente con il tuo team. Niente più problemi di controllo delle versioni!<br/>2. Insights alimentati da IA: Prendi decisioni basate sui dati con IA che analizza le tendenze e automatizza le attività.<br/>3. Suite di sicurezza avanzata: La crittografia end-to-end, l'autenticazione a più fattori e il rilevamento delle minacce mantengono i tuoi dati al sicuro.<br/><br/>Offerta limitata: Aggiorna ora per uno sconto del 20% sul primo anno. Inoltre, consiglia un amico per un ulteriore 10% di sconto!<br/><br/>Cordiali saluti,<br/>Il team di AcmeCloud | Riga dell'oggetto: AcmeCloud: La sicurezza aziendale incontra il lavoro di squadra in tempo reale<br/><br/>Corpo dell'email:<br/>Caro \{\{CONTACT_NAME\}\},<br/><br/>In \{\{COMPANY_NAME\}\}, stai crescendo velocemente e stai considerando il cloud. Ma con la crescita arriva la sfida di mantenere i dati sicuri mentre promuovi la collaborazione. È qui che entra in gioco il rilascio Q3 di AcmeCloud, realizzato su misura per innovatori tecnologici come te.<br/><br/>1. Fort Knox per i tuoi file: La crittografia di grado militare ora salvaguarda i tuoi dati, sia in movimento che a riposo. La tua proprietà intellettuale rimane tua, punto.<br/>2. Sincronizzazione senza confini: Windows, Mac o Linux—AcmeCloud parla tutte le lingue. Il controllo delle versioni unificato significa niente più "final_v2_REALLY_FINAL.doc".<br/>3. Collabora a velocità della luce: Dimentica il ping-pong degli allegati. Ora i tuoi sviluppatori, designer e data scientist co-creano in tempo reale, riducendo drasticamente il time-to-market.<br/><br/>Non prendere solo la nostra parola. Sperimenta la differenza di AcmeCloud con la nostra prova di 30 giorni. E poiché sappiamo che il tempo è il tuo bene più prezioso, stiamo aggiungendo l'onboarding prioritario.<br/><br/>Eleva il tuo team ora - Prova gratuita di 30 giorni |
    
</section>
    <section title="Esempio: Risposta agli incidenti">

    Nota che Claude produce testo superfluo e formattazione diversa con il prompt vago.
    | Ruolo | Prompt vago | Prompt dettagliato |
    | --- | --- | --- |
    | Utente | Analizza questo rapporto di interruzione di AcmeCloud e riassumi i punti chiave.<br/><br/>\{\{REPORT\}\} | Analizza questo rapporto di interruzione di AcmeCloud. Salta il preambolo. Mantieni la tua risposta concisa e scrivi solo le informazioni essenziali. Elenca solo:<br/>1) Causa<br/>2) Durata<br/>3) Servizi interessati<br/>4) Numero di utenti interessati<br/>5) Perdita di entrate stimata. <br/><br/>Ecco il rapporto: \{\{REPORT\}\}|
    | Assistente (risposta di Claude) | Ecco un riassunto dei punti chiave dal rapporto di interruzione di AcmeCloud:<br/><br/>Il 15 maggio, un aggiornamento del bilanciatore di carico mal configurato ha causato problemi significativi di instradamento del traffico, causando un'interruzione del servizio dalle 09:15 alle 13:52 UTC. L'interruzione di 4 ore e 37 minuti ha interessato il nostro servizio di sincronizzazione file principale e il dashboard di amministrazione, rendendoli inaccessibili. Inoltre, l'app mobile ha subito una perdita di funzionalità parziale. Approssimativamente il 68% della nostra base di utenti, per un totale di 1,3 milioni di utenti, è stato interessato. In base ai nostri Accordi sui livelli di servizio (SLA) e al ricavo medio per utente, stimiamo un impatto finanziario di $ 420.000 in crediti di servizio e potenziale abbandono dei clienti. | 1) Causa: Aggiornamento del bilanciatore di carico mal configurato<br/>2) Durata: 4h 37m (09:15-13:52 UTC, 15 maggio)<br/>3) Interessati: Sincronizzazione principale, dashboard di amministrazione (inattivi); app mobile (parziale)<br/>4) Utenti interessati: 1,3M (68% della base)<br/>5) Perdita di entrate stimata: $ 420.000 |
    
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