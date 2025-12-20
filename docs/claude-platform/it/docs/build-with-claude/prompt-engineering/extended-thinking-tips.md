# Suggerimenti per il pensiero esteso

---

Questa guida fornisce strategie e tecniche avanzate per ottenere il massimo dalle funzionalità di pensiero esteso di Claude. Il pensiero esteso consente a Claude di lavorare attraverso problemi complessi passo dopo passo, migliorando le prestazioni su compiti difficili.

Vedi [Modelli di pensiero esteso](/docs/it/about-claude/models/extended-thinking-models) per una guida su quando decidere di utilizzare il pensiero esteso.

## Prima di iniziare

Questa guida presuppone che tu abbia già deciso di utilizzare la modalità di pensiero esteso e che tu abbia esaminato i nostri passaggi di base su [come iniziare con il pensiero esteso](/docs/it/about-claude/models/extended-thinking-models#getting-started-with-extended-thinking-models) così come la nostra [guida all'implementazione del pensiero esteso](/docs/it/build-with-claude/extended-thinking).

### Considerazioni tecniche per il pensiero esteso

- I token di pensiero hanno un budget minimo di 1024 token. Ti consigliamo di iniziare con il budget di pensiero minimo e di aumentare gradualmente per adattarti in base alle tue esigenze e alla complessità del compito.
- Per carichi di lavoro dove il budget di pensiero ottimale è superiore a 32K, ti consigliamo di utilizzare [l'elaborazione batch](/docs/it/build-with-claude/batch-processing) per evitare problemi di rete. Le richieste che spingono il modello a pensare oltre 32K token causano richieste di lunga durata che potrebbero scontrarsi con timeout di sistema e limiti di connessione aperta.
- Il pensiero esteso funziona meglio in inglese, anche se gli output finali possono essere in [qualsiasi lingua supportata da Claude](/docs/it/build-with-claude/multilingual-support).
- Se hai bisogno di pensiero al di sotto del budget minimo, ti consigliamo di utilizzare la modalità standard, con il pensiero disattivato, con il prompting tradizionale chain-of-thought con tag XML (come `<thinking>`). Vedi [prompting chain of thought](/docs/it/build-with-claude/prompt-engineering/chain-of-thought).

## Tecniche di prompting per il pensiero esteso

### Usa prima istruzioni generali, poi risolvi i problemi con istruzioni più dettagliate passo dopo passo

Claude spesso funziona meglio con istruzioni di alto livello per pensare profondamente a un compito piuttosto che con una guida prescrittiva passo dopo passo. La creatività del modello nell'affrontare i problemi può superare la capacità di un umano di prescrivere il processo di pensiero ottimale.

Ad esempio, invece di:

<CodeGroup>
```text User
Pensa a questo problema di matematica passo dopo passo:
1. Prima, identifica le variabili
2. Poi, imposta l'equazione
3. Successivamente, risolvi per x
...
```
</CodeGroup>

Considera:

<CodeGroup>
```text User
Per favore pensa a questo problema di matematica in modo approfondito e molto dettagliato.
Considera approcci multipli e mostra il tuo ragionamento completo.
Prova metodi diversi se il tuo primo approccio non funziona.
```
/>
</CodeGroup>

<TryInConsoleButton
  userPrompt={
    `Per favore pensa a questo problema di matematica in modo approfondito e molto dettagliato.
Considera approcci multipli e mostra il tuo ragionamento completo.
Prova metodi diversi se il tuo primo approccio non funziona.`
  }
  thinkingBudgetTokens={16000}
>
  Prova nella Console
</TryInConsoleButton>

Detto questo, Claude può ancora seguire efficacemente passaggi di esecuzione strutturati complessi quando necessario. Il modello può gestire anche liste più lunghe con istruzioni più complesse rispetto alle versioni precedenti. Ti consigliamo di iniziare con istruzioni più generalizzate, poi leggere l'output di pensiero di Claude e iterare per fornire istruzioni più specifiche per guidare il suo pensiero da lì.

### Prompting multishot con pensiero esteso

Il [prompting multishot](/docs/it/build-with-claude/prompt-engineering/multishot-prompting) funziona bene con il pensiero esteso. Quando fornisci a Claude esempi di come pensare attraverso i problemi, seguirà modelli di ragionamento simili all'interno dei suoi blocchi di pensiero esteso.

Puoi includere esempi few-shot nel tuo prompt in scenari di pensiero esteso utilizzando tag XML come `<thinking>` o `<scratchpad>` per indicare modelli canonici di pensiero esteso in quegli esempi.

Claude generalizzerà il modello al processo formale di pensiero esteso. Tuttavia, è possibile che otterrai risultati migliori dando a Claude libertà di pensare nel modo che ritiene migliore.

Esempio:

<CodeGroup>
```text User
Ti mostrerò come risolvere un problema di matematica, poi voglio che tu ne risolva uno simile.

Problema 1: Quanto è il 15% di 80?

<thinking>
Per trovare il 15% di 80:
1. Converti il 15% in decimale: 15% = 0,15
2. Moltiplica: 0,15 × 80 = 12
</thinking>

La risposta è 12.

Ora risolvi questo:
Problema 2: Quanto è il 35% di 240?
```
/>
</CodeGroup>

<TryInConsoleButton
  userPrompt={
    `Ti mostrerò come risolvere un problema di matematica, poi voglio che tu ne risolva uno simile.

Problema 1: Quanto è il 15% di 80?

<thinking>
Per trovare il 15% di 80:
1. Converti il 15% in decimale: 15% = 0,15
2. Moltiplica: 0,15 × 80 = 12
</thinking>

La risposta è 12.

Ora risolvi questo:
Problema 2: Quanto è il 35% di 240?`
  }
  thinkingBudgetTokens={16000} 
>
  Prova nella Console
</TryInConsoleButton>

### Massimizzare il seguimento delle istruzioni con il pensiero esteso
Claude mostra un seguimento delle istruzioni significativamente migliorato quando il pensiero esteso è abilitato. Il modello tipicamente:
1. Ragiona sulle istruzioni all'interno del blocco di pensiero esteso
2. Esegue quelle istruzioni nella risposta

Per massimizzare il seguimento delle istruzioni:
- Sii chiaro e specifico su quello che vuoi
- Per istruzioni complesse, considera di suddividerle in passaggi numerati che Claude dovrebbe seguire metodicamente
- Consenti a Claude budget sufficiente per elaborare completamente le istruzioni nel suo pensiero esteso

### Utilizzare il pensiero esteso per debuggare e guidare il comportamento di Claude
Puoi utilizzare l'output di pensiero di Claude per debuggare la logica di Claude, anche se questo metodo non è sempre perfettamente affidabile.

Per fare il miglior uso di questa metodologia, raccomandiamo i seguenti suggerimenti:
- Non raccomandiamo di passare il pensiero esteso di Claude nel blocco di testo utente, poiché questo non migliora le prestazioni e può effettivamente degradare i risultati.
- Il prefilling del pensiero esteso è esplicitamente non consentito, e modificare manualmente il testo di output del modello che segue il suo blocco di pensiero probabilmente degraderà i risultati a causa della confusione del modello.

Quando il pensiero esteso è disattivato, il [prefill](/docs/it/build-with-claude/prompt-engineering/prefill-claudes-response) del testo di risposta standard `assistant` è ancora consentito.

<Note>
A volte Claude può ripetere il suo pensiero esteso nel testo di output dell'assistant. Se vuoi una risposta pulita, istruisci Claude a non ripetere il suo pensiero esteso e a produrre solo la risposta.
</Note>

### Sfruttare al meglio output lunghi e pensiero a lungo termine

Per casi d'uso di generazione di dataset, prova prompt come "Per favore crea una tabella estremamente dettagliata di..." per generare dataset completi.

Per casi d'uso come la generazione di contenuti dettagliati dove potresti voler generare blocchi di pensiero esteso più lunghi e risposte più dettagliate, prova questi suggerimenti:
- Aumenta sia la lunghezza massima del pensiero esteso E chiedi esplicitamente output più lunghi
- Per output molto lunghi (20.000+ parole), richiedi uno schema dettagliato con conteggi di parole fino al livello di paragrafo. Poi chiedi a Claude di indicizzare i suoi paragrafi allo schema e mantenere i conteggi di parole specificati

<Warning>
Non raccomandiamo di spingere Claude a produrre più token per il gusto di produrre token. Piuttosto, ti incoraggiamo a iniziare con un piccolo budget di pensiero e aumentare secondo necessità per trovare le impostazioni ottimali per il tuo caso d'uso.
</Warning>

Ecco esempi di casi d'uso dove Claude eccelle grazie al pensiero esteso più lungo:

  <section title="Problemi STEM complessi">

    I problemi STEM complessi richiedono a Claude di costruire modelli mentali, applicare conoscenze specializzate e lavorare attraverso passaggi logici sequenziali—processi che beneficiano di un tempo di ragionamento più lungo.
    
    <Tabs>
      <Tab title="Prompt standard">
        <CodeGroup>
        ```text User
        Scrivi uno script python per una palla gialla che rimbalza all'interno di un quadrato,
        assicurati di gestire correttamente il rilevamento delle collisioni.
        Fai ruotare lentamente il quadrato.
        ```
        />
        </CodeGroup>
        
        <TryInConsoleButton
          userPrompt={
            `Scrivi uno script python per una palla gialla che rimbalza all'interno di un quadrato,
assicurati di gestire correttamente il rilevamento delle collisioni.
Fai ruotare lentamente il quadrato.`
          }
          thinkingBudgetTokens={16000}
        >
          Prova nella Console
        </TryInConsoleButton>
        <Note>
        Questo compito più semplice tipicamente risulta in solo circa alcuni secondi di tempo di pensiero.
        </Note>
      </Tab>
      <Tab title="Prompt migliorato">
        <CodeGroup>
        ```text User
        Scrivi uno script Python per una palla gialla che rimbalza all'interno di un tesseratto,
        assicurandoti di gestire correttamente il rilevamento delle collisioni.
        Fai ruotare lentamente il tesseratto.
        Assicurati che la palla rimanga all'interno del tesseratto.
        ```
        />
        </CodeGroup>
        
        <TryInConsoleButton
          userPrompt={
            `Scrivi uno script Python per una palla gialla che rimbalza all'interno di un tesseratto,
assicurandoti di gestire correttamente il rilevamento delle collisioni.
Fai ruotare lentamente il tesseratto.
Assicurati che la palla rimanga all'interno del tesseratto.`
          }
          thinkingBudgetTokens={16000}
        >
          Prova nella Console
        </TryInConsoleButton>
        <Note>
        Questa sfida complessa di visualizzazione 4D sfrutta al meglio il tempo di pensiero esteso lungo mentre Claude lavora attraverso la complessità matematica e di programmazione.
        </Note>
      </Tab>
    </Tabs>
  
</section>
  
  <section title="Problemi di ottimizzazione con vincoli">

    L'ottimizzazione con vincoli sfida Claude a soddisfare simultaneamente requisiti multipli in competizione, cosa che si realizza meglio quando si consente un tempo di pensiero esteso lungo così che il modello possa affrontare metodicamente ogni vincolo.
    
    <Tabs>
      <Tab title="Prompt standard">
        <CodeGroup>
        ```text User
        Pianifica una vacanza di una settimana in Giappone.
        ```
        />
        </CodeGroup>
        
        <TryInConsoleButton
          userPrompt="Pianifica una vacanza di una settimana in Giappone."
          thinkingBudgetTokens={16000}
        >
          Prova nella Console
        </TryInConsoleButton>
        <Note>
        Questa richiesta aperta tipicamente risulta in solo circa alcuni secondi di tempo di pensiero.
        </Note>
      </Tab>
      <Tab title="Prompt migliorato">
        <CodeGroup>
        ```text User
        Pianifica un viaggio di 7 giorni in Giappone con i seguenti vincoli:
        - Budget di $2.500
        - Deve includere Tokyo e Kyoto
        - Necessità di accomodare una dieta vegetariana
        - Preferenza per esperienze culturali rispetto allo shopping
        - Deve includere un giorno di escursionismo
        - Non più di 2 ore di viaggio tra le località per giorno
        - Necessità di tempo libero ogni pomeriggio per chiamate verso casa
        - Deve evitare le folle dove possibile
        ```
        />
        </CodeGroup>
        
        <TryInConsoleButton
          userPrompt={
            `Pianifica un viaggio di 7 giorni in Giappone con i seguenti vincoli:
- Budget di $2.500
- Deve includere Tokyo e Kyoto
- Necessità di accomodare una dieta vegetariana
- Preferenza per esperienze culturali rispetto allo shopping
- Deve includere un giorno di escursionismo
- Non più di 2 ore di viaggio tra le località per giorno
- Necessità di tempo libero ogni pomeriggio per chiamate verso casa
- Deve evitare le folle dove possibile`
          }
          thinkingBudgetTokens={16000}
        >
          Prova nella Console
        </TryInConsoleButton>
        <Note>
        Con vincoli multipli da bilanciare, Claude naturalmente funzionerà meglio quando gli viene dato più spazio per pensare a come soddisfare ottimalmente tutti i requisiti.
        </Note>
      </Tab>
    </Tabs>
  
</section>
  
  <section title="Framework di pensiero">

    I framework di pensiero strutturati danno a Claude una metodologia esplicita da seguire, che può funzionare meglio quando a Claude viene dato spazio di pensiero esteso lungo per seguire ogni passaggio.
    
    <Tabs>
      <Tab title="Prompt standard">
        <CodeGroup>
        ```text User
        Sviluppa una strategia completa per Microsoft
        per entrare nel mercato della medicina personalizzata entro il 2027.
        ```
        />
        </CodeGroup>
        
        <TryInConsoleButton
          userPrompt={
            `Sviluppa una strategia completa per Microsoft
per entrare nel mercato della medicina personalizzata entro il 2027.`
          }
          thinkingBudgetTokens={16000}
        >
          Prova nella Console
        </TryInConsoleButton>
        <Note>
        Questa domanda strategica ampia tipicamente risulta in solo circa alcuni secondi di tempo di pensiero.
        </Note>
      </Tab>
      <Tab title="Prompt migliorato">
        <CodeGroup>
        ```text User
        Sviluppa una strategia completa per Microsoft per entrare
        nel mercato della medicina personalizzata entro il 2027.
        
        Inizia con:
        1. Un canvas di Strategia Blue Ocean
        2. Applica le Cinque Forze di Porter per identificare le pressioni competitive
        
        Successivamente, conduci un esercizio di pianificazione di scenari con quattro
        futuri distinti basati su variabili regolatorie e tecnologiche.
        
        Per ogni scenario:
        - Sviluppa risposte strategiche utilizzando la Matrice di Ansoff
        
        Infine, applica il framework dei Tre Orizzonti per:
        - Mappare il percorso di transizione
        - Identificare potenziali innovazioni dirompenti in ogni fase
        ```
        />
        </CodeGroup>
        
        <TryInConsoleButton
          userPrompt={
            `Sviluppa una strategia completa per Microsoft per entrare
nel mercato della medicina personalizzata entro il 2027.

Inizia con:
1. Un canvas di Strategia Blue Ocean
2. Applica le Cinque Forze di Porter per identificare le pressioni competitive

Successivamente, conduci un esercizio di pianificazione di scenari con quattro
futuri distinti basati su variabili regolatorie e tecnologiche.

Per ogni scenario:
- Sviluppa risposte strategiche utilizzando la Matrice di Ansoff

Infine, applica il framework dei Tre Orizzonti per:
- Mappare il percorso di transizione
- Identificare potenziali innovazioni dirompenti in ogni fase`
          }
          thinkingBudgetTokens={16000}
        >
          Prova nella Console
        </TryInConsoleButton>
        <Note>
        Specificando framework analitici multipli che devono essere applicati sequenzialmente, il tempo di pensiero aumenta naturalmente mentre Claude lavora attraverso ogni framework metodicamente.
        </Note>
      </Tab>
    </Tabs>
  
</section>

### Fai riflettere Claude e controllare il suo lavoro per migliorare la coerenza e la gestione degli errori
Puoi utilizzare il prompting in linguaggio naturale semplice per migliorare la coerenza e ridurre gli errori:
1. Chiedi a Claude di verificare il suo lavoro con un test semplice prima di dichiarare un compito completo
2. Istruisci il modello ad analizzare se il suo passaggio precedente ha raggiunto il risultato atteso
3. Per compiti di codifica, chiedi a Claude di eseguire casi di test nel suo pensiero esteso

Esempio:

<CodeGroup>
```text User
Scrivi una funzione per calcolare il fattoriale di un numero.
Prima di finire, per favore verifica la tua soluzione con casi di test per:
- n=0
- n=1
- n=5
- n=10
E correggi eventuali problemi che trovi.
```
/>
</CodeGroup>

<TryInConsoleButton
  userPrompt={
    `Scrivi una funzione per calcolare il fattoriale di un numero.
Prima di finire, per favore verifica la tua soluzione con casi di test per:
- n=0
- n=1
- n=5
- n=10
E correggi eventuali problemi che trovi.`
  }
  thinkingBudgetTokens={16000}
>
  Prova nella Console
</TryInConsoleButton>

## Prossimi passi

<CardGroup>
  <Card title="Cookbook del pensiero esteso" icon="book" href="https://github.com/anthropics/anthropic-cookbook/tree/main/extended_thinking">
    Esplora esempi pratici di pensiero esteso nel nostro cookbook.
  </Card>
  <Card title="Guida al pensiero esteso" icon="code" href="/docs/it/build-with-claude/extended-thinking">
    Vedi la documentazione tecnica completa per implementare il pensiero esteso.
  </Card>
</CardGroup>