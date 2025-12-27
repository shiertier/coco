# Mitigare i jailbreak e le iniezioni di prompt

---

I jailbreak e le iniezioni di prompt si verificano quando gli utenti creano prompt per sfruttare le vulnerabilità del modello, con l'obiettivo di generare contenuti inappropriati. Mentre Claude è intrinsecamente resiliente a tali attacchi, ecco ulteriori passaggi per rafforzare le tue protezioni, in particolare contro usi che violano i nostri [Termini di Servizio](https://www.anthropic.com/legal/commercial-terms) o la [Politica di Utilizzo](https://www.anthropic.com/legal/aup).

<Tip>Claude è molto più resistente ai jailbreak rispetto ad altri importanti LLM, grazie a metodi di addestramento avanzati come l'Intelligenza Artificiale Costituzionale.</Tip>

- **Filtri di innocuità**: Utilizza un modello leggero come Claude Haiku 3 per pre-esaminare gli input degli utenti.

    <section title="Esempio: Filtro di innocuità per la moderazione dei contenuti">

        | Ruolo | Contenuto |
        | ---- | --- |
        | Utente | Un utente ha inviato questo contenuto:<br/>\<content><br/>\{\{CONTENT}\}<br/>\</content><br/><br/>Rispondi con (Y) se fa riferimento ad attività dannose, illegali o esplicite. Rispondi con (N) se è sicuro. |
        | Assistente (prefill) | \( |
        | Assistente | N) |
    
</section>

- **Convalida degli input**: Filtra i prompt per individuare schemi di jailbreaking. Puoi anche utilizzare un LLM per creare un filtro di convalida generalizzato fornendo esempi di linguaggio noto per il jailbreaking.

- **Ingegneria dei prompt**: Crea prompt che enfatizzano i confini etici e legali.

    <section title="Esempio: Prompt di sistema etico per un chatbot aziendale">

        | Ruolo | Contenuto |
        | ---- | --- |
        | Sistema | Sei l'assistente AI etico di AcmeCorp. Le tue risposte devono allinearsi con i nostri valori:<br/>\<values><br/>- Integrità: Non ingannare mai o aiutare nell'inganno.<br/>- Conformità: Rifiuta qualsiasi richiesta che violi leggi o le nostre politiche.<br/>- Privacy: Proteggi tutti i dati personali e aziendali.<br/>Rispetto per la proprietà intellettuale: I tuoi output non dovrebbero violare i diritti di proprietà intellettuale altrui.<br/>\</values><br/><br/>Se una richiesta è in conflitto con questi valori, rispondi: "Non posso eseguire questa azione poiché va contro i valori di AcmeCorp." |
    
</section>

Adatta le risposte e considera di limitare o bannare gli utenti che ripetutamente si impegnano in comportamenti abusivi cercando di aggirare le protezioni di Claude. Ad esempio, se un particolare utente attiva ripetutamente lo stesso tipo di rifiuto (es. "output bloccato dalla politica di filtraggio dei contenuti"), informa l'utente che le sue azioni violano le politiche di utilizzo pertinenti e agisci di conseguenza.

- **Monitoraggio continuo**: Analizza regolarmente gli output per individuare segni di jailbreaking.
Utilizza questo monitoraggio per perfezionare iterativamente i tuoi prompt e le strategie di convalida.

## Avanzato: Protezioni a catena
Combina strategie per una protezione robusta. Ecco un esempio di livello enterprise con l'uso di strumenti:

<section title="Esempio: Protezione multi-livello per un chatbot consulente finanziario">

  ### Prompt di sistema del bot
  | Ruolo | Contenuto |
  | ---- | --- |
  | Sistema | Sei AcmeFinBot, un consulente finanziario per AcmeTrade Inc. La tua direttiva principale è proteggere gli interessi dei clienti e mantenere la conformità normativa.<br/><br/>\<directives><br/>1. Convalida tutte le richieste rispetto alle linee guida SEC e FINRA.<br/>2. Rifiuta qualsiasi azione che potrebbe essere interpretata come insider trading o manipolazione del mercato.<br/>3. Proteggi la privacy del cliente; non divulgare mai dati personali o finanziari.<br/>\</directives><br/><br/>Istruzioni passo per passo:<br/>\<instructions><br/>1. Esamina la query dell'utente per la conformità (usa lo strumento 'harmlessness_screen').<br/>2. Se conforme, elabora la query.<br/>3. Se non conforme, rispondi: "Non posso elaborare questa richiesta in quanto viola le normative finanziarie o la privacy del cliente."<br/>\</instructions> |
  
  ### Prompt all'interno dello strumento `harmlessness_screen`
  | Ruolo | Contenuto |
  | --- | --- |
  | Utente | \<user_query><br/>\{\{USER_QUERY}}<br/>\</user_query><br/><br/>Valuta se questa query viola le regole SEC, le linee guida FINRA o la privacy del cliente. Rispondi (Y) se lo fa, (N) se non lo fa. |
  | Assistente (prefill) | \( |

</section>

Stratificando queste strategie, crei una difesa robusta contro i jailbreak e le iniezioni di prompt, garantendo che le tue applicazioni basate su Claude mantengano i più alti standard di sicurezza e conformità.