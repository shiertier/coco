# Definisci i tuoi criteri di successo

---

Costruire un'applicazione di successo basata su LLM inizia con la chiara definizione dei tuoi criteri di successo. Come saprai quando la tua applicazione è abbastanza buona per essere pubblicata?

Avere criteri di successo chiari garantisce che i tuoi sforzi di prompt engineering e ottimizzazione siano focalizzati sul raggiungimento di obiettivi specifici e misurabili.

***

## Costruire criteri solidi

I buoni criteri di successo sono:
- **Specifici**: Definisci chiaramente ciò che vuoi ottenere. Invece di "buone prestazioni", specifica "classificazione accurata del sentiment".
- **Misurabili**: Utilizza metriche quantitative o scale qualitative ben definite. I numeri forniscono chiarezza e scalabilità, ma le misure qualitative possono essere preziose se applicate in modo coerente *insieme* alle misure quantitative.
    - Anche argomenti "nebulosi" come etica e sicurezza possono essere quantificati:
        |      | Criteri di sicurezza                |
        | ---- | --- |
        | Male  | Output sicuri                   |
        | Bene | Meno dello 0,1% degli output su 10.000 prove segnalate per tossicità dal nostro filtro di contenuti. | 
    <section title="Esempi di metriche e metodi di misurazione">

        **Metriche quantitative**:
            - Specifiche per attività: punteggio F1, punteggio BLEU, perplessità
            - Generiche: Accuratezza, precisione, richiamo
            - Operative: Tempo di risposta (ms), tempo di attività (%)

        **Metodi quantitativi**:
            - Test A/B: Confronta le prestazioni rispetto a un modello di riferimento o una versione precedente.
            - Feedback degli utenti: Misure implicite come i tassi di completamento delle attività.
            - Analisi dei casi limite: Percentuale di casi limite gestiti senza errori.

        **Scale qualitative**:
            - Scale Likert: "Valuta la coerenza da 1 (insensato) a 5 (perfettamente logico)"
            - Rubriche di esperti: Linguisti che valutano la qualità della traduzione su criteri definiti        
    
</section>
- **Raggiungibili**: Basa i tuoi obiettivi su benchmark di settore, esperimenti precedenti, ricerca sull'IA o conoscenze di esperti. I tuoi parametri di successo non dovrebbero essere irrealistici rispetto alle attuali capacità dei modelli all'avanguardia.
- **Rilevanti**: Allinea i tuoi criteri con lo scopo dell'applicazione e le esigenze degli utenti. Una forte accuratezza nelle citazioni potrebbe essere fondamentale per app mediche ma meno importante per chatbot casuali.

<section title="Esempio di criteri di fedeltà del compito per l'analisi del sentiment">

    |      | Criteri |
    | ---- | --- |
    | Male  | Il modello dovrebbe classificare bene i sentiment                    |
    | Bene | Il nostro modello di analisi del sentiment dovrebbe raggiungere un punteggio F1 di almeno 0,85 (Misurabile, Specifico) su un set di test separato* di 10.000 diversi post di Twitter (Rilevante), che rappresenta un miglioramento del 5% rispetto alla nostra baseline attuale (Raggiungibile). |

    **Maggiori informazioni sui set di test separati nella prossima sezione*

</section>

***

## Criteri di successo comuni da considerare

Ecco alcuni criteri che potrebbero essere importanti per il tuo caso d'uso. Questo elenco non è esaustivo.

  <section title="Fedeltà del compito">

    Quanto bene deve performare il modello sul compito? Potresti anche dover considerare la gestione dei casi limite, come quanto bene il modello deve performare su input rari o impegnativi.
  
</section>
  <section title="Coerenza">

    Quanto simili devono essere le risposte del modello per tipi simili di input? Se un utente pone la stessa domanda due volte, quanto è importante che riceva risposte semanticamente simili?
  
</section>
  <section title="Rilevanza e coerenza">

    Quanto bene il modello affronta direttamente le domande o le istruzioni dell'utente? Quanto è importante che le informazioni siano presentate in modo logico e facile da seguire?
  
</section>
  <section title="Tono e stile">

    Quanto bene lo stile di output del modello corrisponde alle aspettative? Quanto è appropriato il suo linguaggio per il pubblico target?
  
</section>
  <section title="Preservazione della privacy">

    Qual è una metrica di successo per come il modello gestisce le informazioni personali o sensibili? Può seguire le istruzioni di non utilizzare o condividere determinati dettagli?
  
</section>
  <section title="Utilizzo del contesto">

    Quanto efficacemente il modello utilizza il contesto fornito? Quanto bene fa riferimento e si basa sulle informazioni fornite nella sua cronologia?
  
</section>
  <section title="Latenza">

    Qual è il tempo di risposta accettabile per il modello? Questo dipenderà dai requisiti in tempo reale della tua applicazione e dalle aspettative degli utenti.
  
</section>
  <section title="Prezzo">

    Qual è il tuo budget per l'esecuzione del modello? Considera fattori come il costo per chiamata API, la dimensione del modello e la frequenza di utilizzo.
  
</section>

La maggior parte dei casi d'uso richiederà una valutazione multidimensionale lungo diversi criteri di successo.

<section title="Esempio di criteri multidimensionali per l'analisi del sentiment">

    |      | Criteri |
    | ---- | --- |
    | Male  | Il modello dovrebbe classificare bene i sentiment                    |
    | Bene | Su un set di test separato di 10.000 diversi post di Twitter, il nostro modello di analisi del sentiment dovrebbe raggiungere:<br/>- un punteggio F1 di almeno 0,85<br/>- il 99,5% degli output non è tossico<br/>- il 90% degli errori causerebbe inconvenienti, non errori gravi*<br/>- il 95% dei tempi di risposta < 200ms |

    **In realtà, definiremmo anche cosa significano "inconvenienti" ed "errori gravi".*

</section>

***

## Prossimi passi

<CardGroup cols={2}>
  <Card title="Brainstorm dei criteri" icon="link" href="https://claude.ai/">
    Fai un brainstorming dei criteri di successo per il tuo caso d'uso con Claude su claude.ai.<br/><br/>**Suggerimento**: Inserisci questa pagina nella chat come guida per Claude!
  </Card>
  <Card title="Progetta valutazioni" icon="link" href="/docs/it/be-clear-direct">
    Impara a costruire solidi set di test per valutare le prestazioni di Claude rispetto ai tuoi criteri.
  </Card>
</CardGroup>