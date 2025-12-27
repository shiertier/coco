# Usa template di prompt e variabili

---

Quando distribuisci un'applicazione basata su LLM con Claude, le tue chiamate API consisteranno tipicamente di due tipi di contenuto:
- **Contenuto fisso:** Istruzioni statiche o contesto che rimangono costanti attraverso multiple interazioni
- **Contenuto variabile:** Elementi dinamici che cambiano con ogni richiesta o conversazione, come:
    - Input dell'utente
    - Contenuto recuperato per la Generazione Aumentata da Recupero (RAG)
    - Contesto di conversazione come la cronologia dell'account utente
    - Dati generati dal sistema come risultati di utilizzo di strumenti alimentati da altre chiamate indipendenti a Claude

Un **template di prompt** combina queste parti fisse e variabili, utilizzando segnaposto per il contenuto dinamico. Nella [Console Claude](/), questi segnaposto sono indicati con **\{\{doppie parentesi graffe\}\}**, rendendoli facilmente identificabili e permettendo test rapidi di valori diversi.

---

# Quando usare template di prompt e variabili
Dovresti sempre usare template di prompt e variabili quando ti aspetti che qualsiasi parte del tuo prompt venga ripetuta in un'altra chiamata a Claude (solo tramite l'API o la [Console Claude](/). [claude.ai](https://claude.ai/) attualmente non supporta template di prompt o variabili).

I template di prompt offrono diversi benefici:
- **Coerenza:** Assicurano una struttura coerente per i tuoi prompt attraverso multiple interazioni
- **Efficienza:** Scambia facilmente il contenuto variabile senza riscrivere l'intero prompt
- **Testabilità:** Testa rapidamente input diversi e casi limite cambiando solo la porzione variabile
- **Scalabilità:** Semplifica la gestione dei prompt mentre la tua applicazione cresce in complessità
- **Controllo versione:** Traccia facilmente i cambiamenti alla struttura del tuo prompt nel tempo tenendo traccia solo della parte centrale del tuo prompt, separata dagli input dinamici

La [Console Claude](/) utilizza pesantemente template di prompt e variabili per supportare funzionalità e strumenti per tutto quanto sopra, come con:
- **[Generatore di prompt](/docs/it/build-with-claude/prompt-engineering/prompt-generator):** Decide quali variabili il tuo prompt necessita e le include nel template che produce
- **[Miglioratore di prompt](/docs/it/build-with-claude/prompt-engineering/prompt-improver):** Prende il tuo template esistente, incluse tutte le variabili, e le mantiene nel template migliorato che produce
- **[Strumento di valutazione](/docs/it/test-and-evaluate/eval-tool):** Ti permette di testare, scalare e tracciare facilmente le versioni dei tuoi prompt separando le porzioni variabili e fisse del tuo template di prompt

---

# Esempio di template di prompt

Consideriamo una semplice applicazione che traduce testo inglese in spagnolo. Il testo tradotto sarebbe variabile poiché ti aspetteresti che questo testo cambi tra utenti o chiamate a Claude. Questo testo tradotto potrebbe essere recuperato dinamicamente da database o dall'input dell'utente.

Quindi, per la tua app di traduzione, potresti usare questo semplice template di prompt:
```
Traduci questo testo dall'inglese allo spagnolo: {{text}}
```

---

## Prossimi passi

<CardGroup cols={2}>
  <Card title="Genera un prompt" icon="link" href="/docs/it/build-with-claude/prompt-engineering/prompt-generator">
    Scopri il generatore di prompt nella Console Claude e prova a far generare un prompt a Claude per te.
  </Card>
  <Card title="Applica tag XML" icon="link" href="/docs/it/build-with-claude/prompt-engineering/use-xml-tags">
    Se vuoi migliorare il tuo gioco con le variabili di prompt, avvolgile in tag XML.
  </Card>
  <Card title="Console Claude" icon="link" href="/">
    Scopri la miriade di strumenti di sviluppo prompt disponibili nella Console Claude.
  </Card>
</CardGroup>