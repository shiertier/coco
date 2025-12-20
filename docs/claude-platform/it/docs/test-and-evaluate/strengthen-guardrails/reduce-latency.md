# Ridurre la latenza

Scopri come ridurre la latenza nei modelli Claude attraverso la scelta del modello giusto, l'ottimizzazione dei prompt e l'uso dello streaming.

---

La latenza si riferisce al tempo necessario al modello per elaborare un prompt e generare un output. La latenza può essere influenzata da vari fattori, come la dimensione del modello, la complessità del prompt e l'infrastruttura sottostante che supporta il modello e il punto di interazione.

<Note>
È sempre meglio prima progettare un prompt che funzioni bene senza vincoli del modello o del prompt, e poi provare le strategie di riduzione della latenza successivamente. Cercare di ridurre la latenza prematuramente potrebbe impedirti di scoprire come appare la massima performance.
</Note>

---

## Come misurare la latenza

Quando si discute di latenza, potresti incontrare diversi termini e misurazioni:

- **Latenza di base**: Questo è il tempo impiegato dal modello per elaborare il prompt e generare la risposta, senza considerare i token di input e output per secondo. Fornisce un'idea generale della velocità del modello.
- **Tempo al primo token (TTFT)**: Questa metrica misura il tempo necessario al modello per generare il primo token della risposta, da quando il prompt è stato inviato. È particolarmente rilevante quando stai usando lo streaming (ne parleremo più avanti) e vuoi fornire un'esperienza reattiva ai tuoi utenti.

Per una comprensione più approfondita di questi termini, consulta il nostro [glossario](/docs/it/about-claude/glossary).

---

## Come ridurre la latenza

### 1. Scegli il modello giusto

Uno dei modi più diretti per ridurre la latenza è selezionare il modello appropriato per il tuo caso d'uso. Anthropic offre una [gamma di modelli](/docs/it/about-claude/models/overview) con diverse capacità e caratteristiche di performance. Considera i tuoi requisiti specifici e scegli il modello che meglio si adatta alle tue esigenze in termini di velocità e qualità dell'output.

Per applicazioni critiche in termini di velocità, **Claude Haiku 4.5** offre i tempi di risposta più veloci mantenendo un'alta intelligenza:

```python
import anthropic

client = anthropic.Anthropic()

# Per applicazioni sensibili al tempo, usa Claude Haiku 4.5
message = client.messages.create(
    model="claude-haiku-4-5",
    max_tokens=100,
    messages=[{
        "role": "user",
        "content": "Riassumi questo feedback del cliente in 2 frasi: [testo del feedback]"
    }]
)
```

Per maggiori dettagli sulle metriche dei modelli, consulta la nostra pagina [panoramica dei modelli](/docs/it/about-claude/models/overview).

### 2. Ottimizza la lunghezza del prompt e dell'output

Minimizza il numero di token sia nel tuo prompt di input che nell'output atteso, mantenendo comunque alte prestazioni. Meno token il modello deve elaborare e generare, più veloce sarà la risposta.

Ecco alcuni suggerimenti per aiutarti a ottimizzare i tuoi prompt e output:

- **Sii chiaro ma conciso**: Mira a trasmettere la tua intenzione chiaramente e concisamente nel prompt. Evita dettagli non necessari o informazioni ridondanti, tenendo presente che [claude manca di contesto](/docs/it/build-with-claude/prompt-engineering/be-clear-and-direct) sul tuo caso d'uso e potrebbe non fare i salti logici previsti se le istruzioni non sono chiare.
- **Chiedi risposte più brevi**: Chiedi direttamente a Claude di essere conciso. La famiglia di modelli Claude 3 ha una migliore dirigibilità rispetto alle generazioni precedenti. Se Claude sta producendo output di lunghezza indesiderata, chiedi a Claude di [frenare la sua loquacità](/docs/it/build-with-claude/prompt-engineering/be-clear-and-direct).
  <Tip> A causa di come gli LLM contano i [token](/docs/it/about-claude/glossary#tokens) invece delle parole, chiedere un conteggio esatto di parole o un limite di conteggio di parole non è una strategia efficace quanto chiedere limiti di conteggio di paragrafi o frasi.</Tip>
- **Imposta limiti di output appropriati**: Usa il parametro `max_tokens` per impostare un limite rigido sulla lunghezza massima della risposta generata. Questo impedisce a Claude di generare output eccessivamente lunghi.
  > **Nota**: Quando la risposta raggiunge `max_tokens` token, la risposta verrà tagliata, forse a metà frase o a metà parola, quindi questa è una tecnica grossolana che potrebbe richiedere post-elaborazione ed è solitamente più appropriata per risposte a scelta multipla o risposte brevi dove la risposta arriva proprio all'inizio.
- **Sperimenta con la temperatura**: Il [parametro](/docs/it/api/messages) `temperature` controlla la casualità dell'output. Valori più bassi (ad esempio, 0.2) possono talvolta portare a risposte più focalizzate e più brevi, mentre valori più alti (ad esempio, 0.8) possono risultare in output più diversi ma potenzialmente più lunghi.

Trovare il giusto equilibrio tra chiarezza del prompt, qualità dell'output e conteggio dei token potrebbe richiedere qualche sperimentazione.

### 3. Sfrutta lo streaming

Lo streaming è una funzionalità che consente al modello di iniziare a inviare indietro la sua risposta prima che l'output completo sia terminato. Questo può migliorare significativamente la reattività percepita della tua applicazione, poiché gli utenti possono vedere l'output del modello in tempo reale.

Con lo streaming abilitato, puoi elaborare l'output del modello mentre arriva, aggiornando la tua interfaccia utente o eseguendo altre attività in parallelo. Questo può migliorare notevolmente l'esperienza utente e far sentire la tua applicazione più interattiva e reattiva.

Visita [streaming Messages](/docs/it/build-with-claude/streaming) per imparare come puoi implementare lo streaming per il tuo caso d'uso.