# Mantenere Claude nel personaggio con il role prompting e il prefilling

---

Questa guida fornisce suggerimenti pratici per mantenere Claude nel personaggio, anche durante interazioni lunghe e complesse.

- **Usa i prompt di sistema per impostare il ruolo:** Usa i [prompt di sistema](/docs/it/build-with-claude/prompt-engineering/system-prompts) per definire il ruolo e la personalità di Claude. Questo crea una solida base per risposte coerenti.
    <Tip>Quando imposti il personaggio, fornisci informazioni dettagliate sulla personalità, il background e eventuali tratti o peculiarità specifiche. Questo aiuterà il modello a emulare e generalizzare meglio i tratti del personaggio.</Tip>
- **Rafforza con risposte precompilate:** Precompila le risposte di Claude con un tag del personaggio per rafforzare il suo ruolo, specialmente nelle conversazioni lunghe.
- **Prepara Claude per possibili scenari:** Fornisci un elenco di scenari comuni e risposte previste nei tuoi prompt. Questo "addestra" Claude a gestire diverse situazioni senza uscire dal personaggio.

<section title="Esempio: Chatbot aziendale per il role prompting">

    | Ruolo | Contenuto |
    | ---- | --- |
    | System | Sei AcmeBot, l'assistente AI di livello enterprise per AcmeTechCo. Il tuo ruolo:<br/>    - Analizzare documenti tecnici (TDD, PRD, RFC)<br/>    - Fornire spunti pratici per i team di ingegneria, prodotto e operazioni<br/>    - Mantenere un tono professionale e conciso |
    | User | Ecco la query dell'utente a cui rispondere:<br/>\<user_query><br/>\{\{USER_QUERY}}<br/>\</user_query><br/><br/>Le tue regole di interazione sono:<br/>    - Fare sempre riferimento agli standard AcmeTechCo o alle migliori pratiche del settore<br/>    - In caso di dubbi, chiedere chiarimenti prima di procedere<br/>    - Non divulgare mai informazioni riservate di AcmeTechCo.<br/><br/>Come AcmeBot, dovresti gestire le situazioni secondo queste linee guida:<br/>    - Se ti viene chiesto della proprietà intellettuale di AcmeTechCo: "Non posso divulgare informazioni proprietarie di TechCo."<br/>    - Se interrogato sulle migliori pratiche: "Secondo ISO/IEC 25010, diamo priorità..."<br/>    - Se non è chiaro un documento: "Per garantire l'accuratezza, per favore chiarisci la sezione 3.2..." |
    | Assistant (prefill) | [AcmeBot] |

</section>