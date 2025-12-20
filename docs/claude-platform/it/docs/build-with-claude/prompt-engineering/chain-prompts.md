# Concatenare prompt complessi per prestazioni più forti

Scopri come suddividere attività complesse in sottotask gestibili utilizzando la concatenazione di prompt per migliorare l'accuratezza e la chiarezza.

---

<Note>
While these tips apply broadly to all Claude models, you can find prompting tips specific to extended thinking models [here](/docs/en/build-with-claude/prompt-engineering/extended-thinking-tips).
</Note>

Quando si lavora con attività complesse, Claude a volte può non riuscire se si tenta di gestire tutto in un singolo prompt. La concatenazione di pensiero (CoT) è ottima, ma cosa succede se il tuo compito ha più fasi distinte che richiedono ciascuna un pensiero approfondito?

Entra in gioco la concatenazione di prompt: suddividere attività complesse in sottotask più piccoli e gestibili.

## Perché concatenare i prompt?

1. **Accuratezza**: Ogni sottotask riceve l'attenzione completa di Claude, riducendo gli errori.
2. **Chiarezza**: I sottotask più semplici significano istruzioni e output più chiari.
3. **Tracciabilità**: Individua e correggi facilmente i problemi nella tua catena di prompt.

***

## Quando concatenare i prompt

Utilizza la concatenazione di prompt per attività multi-step come la sintesi della ricerca, l'analisi dei documenti o la creazione di contenuti iterativa. Quando un'attività comporta più trasformazioni, citazioni o istruzioni, la concatenazione impedisce a Claude di perdere o gestire male i passaggi.

**Ricorda:** Ogni collegamento nella catena riceve l'attenzione completa di Claude!

<Tip>**Suggerimento per il debug**: Se Claude perde un passaggio o ha prestazioni scarse, isola quel passaggio nel suo prompt. Questo ti consente di ottimizzare i passaggi problematici senza ripetere l'intera attività.</Tip>

***

## Come concatenare i prompt

1. **Identifica i sottotask**: Suddividi il tuo compito in passaggi distinti e sequenziali.
2. **Struttura con XML per passaggi chiari**: Utilizza tag XML per passare gli output tra i prompt.
3. **Avere un obiettivo a singolo compito**: Ogni sottotask dovrebbe avere un singolo obiettivo chiaro.
4. **Itera**: Affina i sottotask in base alle prestazioni di Claude.

### Flussi di lavoro concatenati di esempio:
- **Analisi multi-step**: Vedi gli esempi legali e commerciali di seguito.
- **Pipeline di creazione di contenuti**: Ricerca → Struttura → Bozza → Modifica → Formattazione.
- **Elaborazione dei dati**: Estrai → Trasforma → Analizza → Visualizza.
- **Processo decisionale**: Raccogli informazioni → Elenca opzioni → Analizza ciascuna → Consiglia.
- **Cicli di verifica**: Genera contenuto → Rivedi → Affina → Rivedi di nuovo.

<Tip>**Suggerimento di ottimizzazione**: Per attività con sottotask indipendenti (come l'analisi di più documenti), crea prompt separati ed eseguili in parallelo per velocità.</Tip>

### Avanzate: Catene di auto-correzione

Puoi concatenare i prompt per far rivedere a Claude il suo stesso lavoro! Questo cattura gli errori e affina gli output, soprattutto per attività ad alto rischio.

  <section title="Esempio: Riepilogo di ricerca auto-correttivo">

    #### Prompt 1
    | Ruolo | Contenuto |
    | ---- | ------- |
    | Utente | Riassumi questo articolo di ricerca medica.<br/><br/>\\{\{RESEARCH_PAPER}}\</paper><br/><br/>Concentrati su metodologia, risultati e implicazioni cliniche. |
    | Assistente<br/>\{\{SUMMARY}} | Metodologia: RCT in doppio cieco con 500 pazienti con diabete di tipo 2. Il gruppo A (n=250) ha ricevuto un nuovo agonista duale GLP-1/GIP; il gruppo B (n=250) ha ricevuto un agonista GLP-1 standard. Studio di 24 settimane, endpoint primario: riduzione dell'HbA1c.<br/><br/>Risultati:<br/>- Gruppo A: riduzione media dell'HbA1c dell'1,8%<br/>- Gruppo B: riduzione media dell'HbA1c dell'1,1%<br/>- 68% del gruppo A vs. 52% del gruppo B ha raggiunto HbA1c \<7%<br/>- Gruppo A: perdita di peso media di 5,2 kg<br/>- Gruppo B: perdita di peso media di 3,1 kg<br/><br/>Sicurezza: Profili di eventi avversi simili, nessun evento grave attribuito al trattamento.<br/><br/>Implicazioni cliniche: L'agonista duale mostra un controllo glicemico superiore e una perdita di peso rispetto alla terapia standard. Potrebbe essere un game-changer, soprattutto per i diabetici obesi. |
    
    #### Prompt 2
    | Ruolo | Contenuto |
    | ---- | ------- |
    | Utente | Il tuo compito è fornire feedback su un riepilogo di un articolo di ricerca. Ecco un riepilogo di un articolo di ricerca medica:<br/>\<summary><br/>\{\{SUMMARY\}\}<br/>\</summary><br/><br/>Ecco l'articolo di ricerca:<br/>\<br/>\{\{RESEARCH_PAPER\}\}<br/>\</paper><br/><br/>Rivedi questo riepilogo per accuratezza, chiarezza e completezza su una scala di voti da A a F. |
    | Assistente<br/>\{\{FEEDBACK}} | 1. Accuratezza: A<br/>   - Ha riportato correttamente le statistiche chiave: riduzioni dell'HbA1c, perdita di peso.<br/>   - Ha notato accuratamente il profilo di sicurezza e l'assenza di eventi avversi gravi.<br/>   - L'affermazione di "game-changer" è giustificata da risultati significativamente migliori.<br/><br/>2. Chiarezza: A-<br/>   - Conciso e ben strutturato con sezioni chiare.<br/>   - Buon uso del linguaggio comparativo (ad es., "su del 3% rispetto al Q1") per il contesto.<br/>   - Problema minore: Gli acronimi come "RCT", "GLP-1" e "GIP" non sono definiti. Considera di scrivere per esteso al primo uso.<br/><br/>3. Completezza: B+<br/>   - Copre tutte le aree richieste: metodologia, risultati, implicazioni cliniche.<br/>   - Mancante: Caratteristiche di base dei gruppi (erano ben abbinati?).<br/>   - Mancante: Qualsiasi endpoint secondario (ad es., pressione sanguigna, lipidi).<br/>   - Considera: Implicazioni a lungo termine o necessità di ulteriori ricerche.<br/><br/>Nel complesso, il riepilogo è accurato e chiaro, fornendo informazioni chiave per una rapida comprensione dello studio. Per elevare ad A+, definisci gli acronimi, menziona le statistiche di base del gruppo, nota eventuali risultati secondari e tocca brevemente la necessità di studi a lungo termine. |
    
     #### Prompt 3
    | Ruolo | Contenuto |
    | ---- | ------- |
    | Utente | Il tuo compito è migliorare un riepilogo di un articolo data il feedback. Ecco la prima bozza di un articolo di ricerca medica:<br/>\<summary><br/>\{\{SUMMARY\}\}<br/>\</summary><br/><br/>Ecco l'articolo di ricerca:<br/>\<br/>\{\{RESEARCH_PAPER\}\}<br/>\</paper><br/><br/>Ecco il feedback:<br/>\<feedback><br/>\{\{FEEDBACK\}\}<br/>\</feedback><br/><br/>Aggiorna il riepilogo in base al feedback. |
    | Assistente | Metodologia: Studio randomizzato controllato in doppio cieco (RCT) con 500 pazienti con diabete di tipo 2. Il gruppo A (n=250) ha ricevuto un nuovo agonista duale glucagon-like peptide-1/polipeptide insulinotropico dipendente dal glucosio (GLP-1/GIP); il gruppo B (n=250) ha ricevuto un agonista GLP-1 standard. L'endpoint primario dello studio di 24 settimane era la riduzione dell'HbA1c. I gruppi erano ben abbinati per età, BMI e HbA1c di base.<br/><br/>Risultati:<br/>- Gruppo A: riduzione media dell'HbA1c dell'1,8%<br/>- Gruppo B: riduzione media dell'HbA1c dell'1,1%<br/>- 68% del gruppo A vs. 52% del gruppo B ha raggiunto HbA1c \<7%<br/>- Gruppo A: perdita di peso media di 5,2 kg<br/>- Gruppo B: perdita di peso media di 3,1 kg<br/><br/>Gli endpoint secondari hanno mostrato miglioramenti nella pressione sanguigna e nei profili lipidici per entrambi i gruppi, con il gruppo A che ha mostrato risultati leggermente migliori.<br/><br/>Sicurezza: Profili di eventi avversi simili, senza eventi gravi attribuiti a nessuno dei due trattamenti.<br/><br/>Implicazioni cliniche: L'agonista duale GLP-1/GIP dimostra un controllo glicemico superiore e una perdita di peso rispetto alla terapia GLP-1 standard. Questi risultati suggeriscono che potrebbe essere un avanzamento significativo, in particolare per i diabetici obesi. Tuttavia, sono necessari studi a più lungo termine per confermare l'efficacia e la sicurezza sostenute, nonché per indagare i potenziali benefici sui risultati cardiovascolari. |
  
</section>

***

## Esempi

  <section title="Esempio: Analisi di un contratto legale (senza concatenazione)">

    Nota che Claude perde l'istruzione di fornire le modifiche proposte nella sua bozza di email.

    | Ruolo | Contenuto |
    | ---- | ------- |
    | Utente | Sei il nostro Chief Legal Officer. Rivedi questo contratto SaaS per i rischi, concentrandoti sulla privacy dei dati, sugli SLA e sui limiti di responsabilità.<br/><br/>\<contract><br/>\{\{CONTRACT}}<br/>\</contract><br/><br/>Quindi redigi un'email al fornitore con le tue preoccupazioni e le modifiche proposte. |
    | Assistente | Dopo aver rivisto il contratto SaaS, ho identificato diverse preoccupazioni:<br/><br/>1. Privacy dei dati (Sezione 7):<br/>   - Problema: Il fornitore ha ampi diritti di utilizzare i dati dei clienti per il "miglioramento del servizio".<br/>   - Rischio: Potrebbe portare a condivisione non autorizzata dei dati o monetizzazione.<br/><br/>2. SLA (Sezione 5):<br/>   - Problema: La garanzia di uptime del 99,5% è al di sotto dello standard del settore.<br/>   - Rischio: Un tempo di inattività eccessivo potrebbe influire sulle nostre operazioni.<br/><br/>3. Responsabilità (Sezione 9):<br/>   - Problema: Il limite è fissato a 3 mesi di commissioni.<br/>   - Rischio: Inadeguato per potenziali violazioni dei dati o guasti del servizio.<br/><br/>Bozza di email:<br/><br/>Caro \{\{VENDOR}},<br/><br/>Dopo aver rivisto l'accordo SaaS, abbiamo preoccupazioni:<br/><br/>1. Privacy dei dati: Richiediamo che i nostri dati vengano utilizzati solo per la fornitura diretta del servizio, non per il "miglioramento del servizio" ampio. Si prega di modificare la Sezione 7.<br/><br/>2. SLA: L'uptime del 99,5% è insufficiente. Abbiamo bisogno del 99,99% con penalità chiaramente definite. Si prega di rivedere la Sezione 5.<br/><br/>3. Responsabilità: L'attuale limite di 3 mesi di commissioni è inadeguato. Proponiamo un limite di 12 mesi di commissioni, senza limite per le violazioni dei dati o la cattiva condotta intenzionale.<br/><br/>Apprezziamo la nostra potenziale partnership e crediamo che questi cambiamenti ci prepareranno al successo. Si prega di inviare un accordo rivisto.<br/><br/>Cordiali saluti,<br/>\{\{CLO_NAME}} |
  
</section>

  <section title="Esempio: Analisi di un contratto legale (con concatenazione)">

    #### Prompt 1    
    | Ruolo | Contenuto |
    | ---- | ------- |
    | Utente | Sei il nostro Chief Legal Officer. Rivedi questo contratto SaaS per i rischi, concentrandoti sulla privacy dei dati, sugli SLA e sui limiti di responsabilità.<br/><br/>\<contract><br/>\{\{CONTRACT}}<br/>\</contract><br/><br/>Fornisci i tuoi risultati nei tag \<risks>. |
    | Assistente<br/>\{\{CONCERNS}} | 1. Privacy dei dati (Sezione 7):<br/>   - Problema: Il fornitore ha ampi diritti di utilizzare i dati dei clienti per il "miglioramento del servizio".<br/>   - Rischio: Potrebbe portare a condivisione non autorizzata dei dati o monetizzazione dei dati dei clienti.<br/>   - Impatto: Potenziale violazione della fiducia dei clienti, sanzioni normative (GDPR, CCPA).<br/><br/>2. SLA (Sezione 5):<br/>   - Problema: La garanzia di uptime del 99,5% è al di sotto dello standard del settore (tipicamente 99,99%).<br/>   - Rischio: Fino a 43,8 ore di inattività all'anno rispetto a 52,6 minuti al 99,99%.<br/>   - Impatto: Interruzioni operative, perdita di entrate, insoddisfazione dei clienti.<br/><br/>3. Responsabilità (Sezione 9):<br/>   - Problema: Il limite di responsabilità è fissato a 3 mesi di commissioni.<br/>   - Rischio: Inadeguato per potenziali violazioni dei dati, furto di proprietà intellettuale o guasti del servizio prolungati.<br/>   - Impatto: Compenso insufficiente per i danni che potrebbero superare di gran lunga il limite, come cause legali collettive o sanzioni normative. |
    
    #### Prompt 2    
    | Ruolo | Contenuto |
    | ---- | ------- |
    | Utente | Redigi un'email a un fornitore di prodotti SaaS delineando le seguenti preoccupazioni e proponendo modifiche. Ecco le preoccupazioni:<br/>\<concerns><br/>\{\{CONCERNS\}\}<br/>\</concerns><br/><br/>|
    | Assistente<br/>\{\{EMAIL}} | Caro \{\{VENDOR}},<br/><br/>Dopo una revisione approfondita dell'accordo SaaS proposto, il nostro team legale ha identificato diverse preoccupazioni ad alta priorità che devono essere affrontate prima di poter procedere:<br/><br/>1. Privacy dei dati (Sezione 7):<br/>   - Attuale: Ampi diritti di utilizzare i dati dei clienti per il "miglioramento del servizio".<br/>   - Preoccupazione: Potenziale condivisione non autorizzata dei dati o monetizzazione.<br/>   - Modifica proposta: Limitare l'uso dei dati alla fornitura diretta del servizio. Qualsiasi uso per miglioramento, analisi o marketing deve essere opt-in, revocabile e anonimizzato.<br/><br/>2. SLA (Sezione 5):<br/>   - Attuale: Garanzia di uptime del 99,5%.<br/>   - Preoccupazione: Fino a 43,8 ore di inattività all'anno, rischiando interruzioni operative.<br/>   - Modifica proposta: Uptime del 99,99% con crediti di servizio a livelli. Ad es., credito del 10% per \<99,99%, 25% per \<99,9%, 50% per \<99,5%.<br/><br/>3. Responsabilità (Sezione 9):<br/>   - Attuale: Limite a 3 mesi di commissioni.<br/>   - Preoccupazione: Grossolanamente inadeguato per violazioni dei dati, furto di proprietà intellettuale o interruzioni prolungate.<br/>   - Modifica proposta: 12 mesi di commissioni per responsabilità generale. Senza limite per violazioni dei dati, cattiva condotta intenzionale, violazione della proprietà intellettuale.<br/><br/>Questi cambiamenti sono non negoziabili per noi. Si allineano con gli standard del settore e il nostro impegno verso la sicurezza dei dati, l'eccellenza operativa e la responsabilità fiduciaria. Apprezziamo il potenziale della nostra partnership, e affrontare questi problemi stabilirà una base solida.<br/><br/>Si prega di far rivedere al tuo team legale e incorporare questi cambiamenti. Siamo felici di discutere ulteriormente per garantire una comprensione e un accordo reciproci.<br/><br/>Cordiali saluti,<br/>\{\{CLO_NAME}} |
    
    #### Prompt 3    
    | Ruolo | Contenuto |
    | ---- | ------- |
    | Utente | Il tuo compito è rivedere un'email e fornire feedback. Ecco l'email:<br/>\<email><br/>\{\{EMAIL\}\}<br/>\</email><br/><br/>Fornisci feedback su tono, chiarezza e professionalità. |
    | Assistente | 1. Tono:<br/>   - Professionale e assertivo senza essere confrontazionale.<br/>   - L'uso di "non negoziabile" è forte ma giustificato data la criticità dei problemi.<br/>   - Il linguaggio collaborativo ("apprezziamo", "felici di discutere") bilancia la fermezza, mantenendo un tono costruttivo.<br/><br/>2. Chiarezza:<br/>   - Strutturazione eccellente: stato attuale, preoccupazione e modifica proposta per ogni problema.<br/>   - I termini tecnici (ad es., "EBITDA", "crediti di servizio a livelli") sono appropriati per un contesto B2B SaaS.<br/>   - Gli elementi di azione sono chiari: il team legale del fornitore deve rivedere e incorporare le modifiche.<br/><br/>3. Professionalità:<br/>   - Si concentra sul beneficio reciproco e gli standard del settore, non solo sulle nostre richieste.<br/>   - Menziona responsabilità più ampie (sicurezza dei dati, eccellenza operativa, dovere fiduciario), elevando la discussione.<br/>   - Offre ulteriori discussioni, mostrando impegno nel comprendere e risolvere i problemi.<br/><br/>Nel complesso, l'email raggiunge il giusto equilibrio tra assertività e collaborazione. Articola chiaramente le nostre preoccupazioni, propone soluzioni specifiche e mantiene un tono professionale e orientato alla partnership. Nessuna revisione necessaria; procedi con l'invio. |
  
</section>
  <section title="Esempio: Revisione della strategia multi-tenancy">

    #### Prompt 1    
    | Ruolo | Contenuto |
    | ---- | ------- |
    | Utente | Come architetto di soluzioni senior, rivedi e analizza questa strategia multi-tenancy per il nostro nuovo prodotto SaaS aziendale.<br/><br/>\<strategy><br/>\{\{STRATEGY}}<br/>\</strategy><br/><br/>Concentrati su scalabilità, sicurezza e convenienza. |
    | Assistente<br/>\{\{ANALYSIS}} | 1. Scalabilità:<br/>   + Lo sharding del database per tenant riduce la contesa, supporta la crescita elevata.<br/>   + I microservizi consentono il ridimensionamento indipendente dei componenti.<br/>   - I server app condivisi potrebbero creare un collo di bottiglia sotto carico estremo.<br/>   - Nessuna menzione di auto-scaling o bilanciamento del carico per il livello app.<br/><br/>2. Sicurezza:<br/>   + L'isolamento dei dati del tenant tramite sharding mitiga le perdite di dati.<br/>   + JWT con scadenza breve e token di aggiornamento è la migliore pratica del settore.<br/>   - Nessuna menzione di crittografia (a riposo, in transito) o gestione delle chiavi.<br/>   - La strategia IAM manca di dettagli su ruoli, autorizzazioni o RBAC.<br/><br/>3. Convenienza:<br/>   + I server app condivisi riducono i costi delle risorse inattive.<br/>   + Le funzioni serverless per attività infrequenti ottimizzano i costi.<br/>   - Il database per tenant potrebbe aumentare i costi in scala rispetto al database multi-tenant ottimizzato.<br/>   - Nessuna discussione su istanze riservate o istanze spot per il risparmio sui costi. |
    
    #### Prompt 2    
    | Ruolo | Contenuto |
    | ---- | ------- |
    | Utente | Redigi un documento di revisione della strategia per la leadership dell'ingegneria basato su questa analisi di una strategia multi-tenancy.<br/><br/>\<strategy><br/>\{\{STRATEGY}}<br/>\</strategy><br/><br/>\<analysis><br/>\{\{ANALYSIS}}<br/>\</analysis><br/><br/>Includi un riepilogo esecutivo, analisi dettagliata e raccomandazioni. |
    | Assistente<br/>\{\{STRATEGY_DOC}} | # Revisione della strategia multi-tenancy: Prodotto SaaS aziendale<br/><br/>## Riepilogo esecutivo<br/>La strategia proposta dimostra una base solida nei principi di scalabilità e sicurezza ma ha lacune nei dettagli di implementazione. Eccelle nell'isolamento dei dati e nel ridimensionamento a livello di componente ma manca di robusto ridimensionamento del livello app e misure di sicurezza avanzate. Le ottimizzazioni dei costi sono presenti ma potrebbero fallire in scala elevata. Con miglioramenti mirati, questa strategia può supportare efficacemente le nostre ambizioni SaaS aziendali.<br/><br/>## Analisi dettagliata<br/><br/>### 1. Scalabilità: B+<br/><br/>Punti di forza:<br/>- Lo sharding del database per tenant è eccellente. Minimizza la latenza delle query, supporta la crescita elevata dei dati e facilita il ridimensionamento senza soluzione di continuità per tenant.<br/>- L'architettura dei microservizi consente il ridimensionamento dei singoli componenti in base alla domanda, cruciale per gestire diversi carichi di lavoro aziendali.<br/><br/>Punti deboli:<br/>- I server applicativi condivisi rischiano di diventare un collo di bottiglia. Con l'aumento del numero di tenant e del traffico, questi potrebbero degradare le prestazioni su tutti i tenant.<br/>- L'assenza di auto-scaling e bilanciamento del carico per il livello app è una svista critica. Questi sono essenziali per gestire i picchi di traffico senza over-provisioning.<br/><br/>### 2. Sicurezza: B-<br/><br/>Punti di forza:<br/>- I database shardati forniscono un forte isolamento dei dati del tenant, mitigando i rischi di perdite di dati o accesso non autorizzato tra i tenant.<br/>- JWT con scadenza breve e token di aggiornamento si allineano con le migliori pratiche OWASP, riducendo le finestre di vulnerabilità dei token.<br/><br/>Punti deboli:<br/>- Nessuna menzione di crittografia dei dati a riposo o in transito (TLS 1.3). Questo è non negoziabile per i dati aziendali.<br/>- La strategia di gestione delle chiavi è mancante. Come vengono gestite e ruotate le chiavi di crittografia, i segreti JWT e le credenziali del database?<br/>- IAM manca di profondità. Abbiamo bisogno di RBAC con ruoli ben definiti, controllo di accesso basato su attributi (ABAC) per autorizzazioni granulari e integrazione con provider di identità aziendali (SAML, OIDC).<br/><br/>### 3. Convenienza: B<br/><br/>Punti di forza:<br/>- I server app condivisi riducono i costi della capacità inattiva, buono per il traffico in stato stazionario.<br/>- Serverless per attività infrequenti (ad es., reporting) è intelligente; paghi solo quando queste risorse vengono utilizzate.<br/><br/>Punti deboli:<br/>- I database per tenant potrebbero non essere convenienti in scala rispetto a un database multi-tenant ben progettato con partizionamento robusto dei dati.<br/>- Nessuna menzione di misure di risparmio sui costi come istanze riservate per carichi di lavoro prevedibili o istanze spot per attività tolleranti ai guasti.<br/>- La mancanza di auto-scaling del livello app potrebbe portare a over-provisioning, aumentando i costi.<br/><br/>## Raccomandazioni<br/><br/>1. **Resilienza del livello app**: Implementa auto-scaling con Kubernetes o servizi cloud-native (EKS, GKE). Distribuisci tra zone di disponibilità con una CDN per asset statici.<br/><br/>2. **Indurimento della sicurezza**:<br/>   a. Crittografa i dati a riposo (AES-256) e in transito (TLS 1.3).<br/>   b. Utilizza un KMS gestito (AWS KMS, GCP KMS) per la gestione delle chiavi.<br/>   c. Migliora IAM con RBAC, ABAC e SAML/OIDC per SSO aziendale.<br/><br/>3. **Ottimizzazione dei costi**:<br/>   a. Valuta il database multi-tenant colonnare (ad es., Snowflake) rispetto allo sharding in scala.<br/>   b. Utilizza istanze riservate per i server app, istanze spot per carichi di lavoro non critici.<br/>   c. Implementa il monitoraggio proattivo e il right-sizing per evitare l'over-provisioning.<br/><br/>4. **Proof of Concept (PoC)**: Prima dell'implementazione completa, conduci un PoC con una simulazione ad alta scala:<br/>   a. Testa il database shardato rispetto al database multi-tenant ottimizzato per costi e prestazioni.<br/>   b. Stress-test del livello app con traffico simulato a livello aziendale.<br/>   c. Conduci un audit di sicurezza di terze parti, concentrandosi sull'isolamento dei dati e sui controlli di accesso.<br/><br/>Con questi miglioramenti, la nostra strategia multi-tenancy sarà di livello aziendale: scalabile, sicura e conveniente. Il PoC convaliderà le nostre scelte di progettazione e instillerà fiducia nei nostri clienti aziendali. |
    
    #### Prompt 3    
    | Ruolo | Contenuto |
    | ---- | ------- |
    | Utente | Valuta questo documento di revisione della strategia per chiarezza, azionabilità e allineamento con le priorità aziendali. <br/><br/>\<br/>\{\{PRIORITIES}}<br/>\</priorities><br/><br/>\<strategy_doc><br/>\{\{STRATEGY_DOC}}<br/>\</strategy_doc> |
    | Assistente | 1. Chiarezza: A<br/>   + Struttura chiara con Riepilogo esecutivo, Analisi dettagliata e Raccomandazioni.<br/>   + L'analisi utilizza un sistema di valutazione (B+, B-) per una rapida comprensione dei punti di forza/debolezza.<br/>   + I termini tecnici (RBAC, ABAC, TLS) sono spiegati o contestualizzati, aiutando la leadership non tecnica.<br/>   + Gli aiuti visivi (punti elenco, rientri) migliorano la leggibilità.<br/><br/>2. Azionabilità: A-<br/>   + Le raccomandazioni sono specifiche e tecniche (ad es., "Implementa auto-scaling con Kubernetes").<br/>   + Il piano PoC con obiettivi chiari fornisce una roadmap.<br/>   - Mancante: Timeline, requisiti di risorse o assegnazioni di team per ogni raccomandazione.<br/>   - Considera: Aggiungi una sezione "Passaggi successivi" con azioni immediate, ruoli e una timeline di alto livello.<br/><br/>3. Allineamento aziendale: A+<br/>   + Affronta direttamente le priorità aziendali:<br/>     - Scalabilità: Discute la gestione dei picchi di traffico a livello aziendale.<br/>     - Sicurezza: Si concentra sull'isolamento dei dati, la crittografia e l'IAM aziendale.<br/>     - Costi: Le strategie si allineano con FinOps aziendale (istanze riservate, right-sizing).<br/>   + PoC con "simulazione ad alta scala" e audit di sicurezza mostra impegno verso la qualità di livello aziendale.<br/>   + Il linguaggio risuona con le preoccupazioni aziendali: "rischio inaccettabile", "livello aziendale", "instillare fiducia".<br/><br/>Nel complesso, il documento eccelle in chiarezza e allineamento aziendale. L'azionabilità è forte ma potrebbe essere elevata ad A+ con una sezione "Passaggi successivi". La profondità tecnica, accoppiata con spiegazioni chiare, rende questo altamente efficace sia per la leadership tecnica che non tecnica. Ottimo lavoro! |
  
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