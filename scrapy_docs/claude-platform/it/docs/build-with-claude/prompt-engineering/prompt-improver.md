# Usa il nostro miglioratore di prompt per ottimizzare i tuoi prompt

Usa il nostro miglioratore di prompt per ottimizzare i tuoi prompt

---

<Note>
Il nostro miglioratore di prompt è compatibile con tutti i modelli Claude, inclusi quelli con capacità di pensiero esteso. Per suggerimenti di prompting specifici per i modelli di pensiero esteso, vedi [qui](/docs/it/build-with-claude/extended-thinking).
</Note>

Il miglioratore di prompt ti aiuta a iterare rapidamente e migliorare i tuoi prompt attraverso analisi e miglioramento automatizzati. Eccelle nel rendere i prompt più robusti per compiti complessi che richiedono alta precisione.

<Frame>
  ![Image](/docs/images/prompt_improver.png)
</Frame>

## Prima di iniziare

Avrai bisogno di:
- Un [template di prompt](/docs/it/build-with-claude/prompt-engineering/prompt-templates-and-variables) da migliorare
- Feedback sui problemi attuali con gli output di Claude (opzionale ma raccomandato)
- Input di esempio e output ideali (opzionale ma raccomandato)

## Come funziona il miglioratore di prompt

Il miglioratore di prompt migliora i tuoi prompt in 4 passaggi:

1. **Identificazione degli esempi**: Localizza ed estrae esempi dal tuo template di prompt
2. **Bozza iniziale**: Crea un template strutturato con sezioni chiare e tag XML
3. **Raffinamento della catena di pensiero**: Aggiunge e raffina istruzioni di ragionamento dettagliate
4. **Miglioramento degli esempi**: Aggiorna gli esempi per dimostrare il nuovo processo di ragionamento

Puoi guardare questi passaggi accadere in tempo reale nel modal di miglioramento.

## Cosa ottieni

Il miglioratore di prompt genera template con:
- Istruzioni dettagliate di catena di pensiero che guidano il processo di ragionamento di Claude e tipicamente migliorano le sue prestazioni
- Organizzazione chiara usando tag XML per separare diversi componenti
- Formattazione standardizzata degli esempi che dimostra ragionamento passo-passo dall'input all'output
- Prefill strategici che guidano le risposte iniziali di Claude

<Note>
Mentre gli esempi appaiono separatamente nell'interfaccia utente del Workbench, sono inclusi all'inizio del primo messaggio utente nella chiamata API effettiva. Visualizza il formato grezzo cliccando "**\<\/\> Get Code**" o inserisci esempi come testo grezzo tramite la casella Examples.
</Note>

## Come usare il miglioratore di prompt

1. Invia il tuo template di prompt
2. Aggiungi qualsiasi feedback sui problemi con gli output attuali di Claude (ad esempio, "i riassunti sono troppo basilari per pubblici esperti")
3. Includi input di esempio e output ideali
4. Rivedi il prompt migliorato

## Genera esempi di test

Non hai ancora esempi? Usa il nostro [Generatore di Casi di Test](/docs/it/test-and-evaluate/eval-tool#creating-test-cases) per:
1. Generare input di esempio
2. Ottenere le risposte di Claude
3. Modificare le risposte per corrispondere ai tuoi output ideali
4. Aggiungere gli esempi perfezionati al tuo prompt

## Quando usare il miglioratore di prompt

Il miglioratore di prompt funziona meglio per:
- Compiti complessi che richiedono ragionamento dettagliato
- Situazioni dove la precisione è più importante della velocità
- Problemi dove gli output attuali di Claude necessitano di miglioramento significativo

<Note>
Per applicazioni sensibili alla latenza o ai costi, considera l'uso di prompt più semplici. Il miglioratore di prompt crea template che producono risposte più lunghe, più approfondite, ma più lente.
</Note>

## Esempio di miglioramento

Ecco come il miglioratore di prompt migliora un prompt di classificazione basilare:

<section title="Prompt originale">

```
Dalla seguente lista di titoli di articoli Wikipedia, identifica da quale articolo proviene questa frase.
Rispondi solo con il titolo dell'articolo e nient'altro.

Titoli degli articoli:
{{titles}}

Frase da classificare:
{{sentence}}
```

</section>

<section title="Prompt migliorato">

```
Sei un sistema intelligente di classificazione del testo specializzato nell'abbinare frasi a titoli di articoli Wikipedia. Il tuo compito è identificare a quale articolo Wikipedia appartiene più probabilmente una data frase, basandoti su una lista fornita di titoli di articoli.

Prima, rivedi la seguente lista di titoli di articoli Wikipedia:
<article_titles>
{{titles}}
</article_titles>

Ora, considera questa frase che deve essere classificata:
<sentence_to_classify>
{{sentence}}
</sentence_to_classify>

Il tuo obiettivo è determinare quale titolo di articolo dalla lista fornita corrisponde meglio alla frase data. Segui questi passaggi:

1. Elenca i concetti chiave dalla frase
2. Confronta ogni concetto chiave con i titoli degli articoli
3. Classifica i primi 3 titoli più rilevanti e spiega perché sono rilevanti
4. Seleziona il titolo di articolo più appropriato che meglio comprende o si relaziona al contenuto della frase

Avvolgi la tua analisi in tag <analysis>. Includi quanto segue:
- Lista dei concetti chiave dalla frase
- Confronto di ogni concetto chiave con i titoli degli articoli
- Classifica dei primi 3 titoli più rilevanti con spiegazioni
- La tua scelta finale e ragionamento

Dopo la tua analisi, fornisci la tua risposta finale: il singolo titolo di articolo Wikipedia più appropriato dalla lista.

Fornisci solo il titolo dell'articolo scelto, senza alcun testo aggiuntivo o spiegazione.
```

</section>

Nota come il prompt migliorato:
- Aggiunge istruzioni di ragionamento chiare passo-passo
- Usa tag XML per organizzare il contenuto
- Fornisce requisiti espliciti di formattazione dell'output
- Guida Claude attraverso il processo di analisi

## Risoluzione dei problemi

Problemi comuni e soluzioni:

- **Gli esempi non appaiono nell'output**: Controlla che gli esempi siano formattati correttamente con tag XML e appaiano all'inizio del primo messaggio utente
- **Catena di pensiero troppo verbosa**: Aggiungi istruzioni specifiche sulla lunghezza desiderata dell'output e livello di dettaglio
- **I passaggi di ragionamento non corrispondono alle tue esigenze**: Modifica la sezione dei passaggi per corrispondere al tuo caso d'uso specifico

***

## Prossimi passi

<CardGroup cols={3}>
  <Card title="Libreria di prompt" icon="link" href="/docs/it/resources/prompt-library/library">
    Lasciati ispirare da prompt di esempio per vari compiti.
  </Card>
  <Card title="Tutorial di prompting GitHub" icon="link" href="https://github.com/anthropics/prompt-eng-interactive-tutorial">
    Impara le migliori pratiche di prompting con il nostro tutorial interattivo.
  </Card>
  <Card title="Testa i tuoi prompt" icon="link" href="/docs/it/test-and-evaluate/eval-tool">
    Usa il nostro strumento di valutazione per testare i tuoi prompt migliorati.
  </Card>
</CardGroup>