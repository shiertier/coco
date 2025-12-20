# Utilizzo dello Strumento di Valutazione

La [Claude Console](/dashboard) presenta uno **strumento di Valutazione** che ti permette di testare i tuoi prompt in vari scenari.

---

## Accesso alla Funzione di Valutazione

Per iniziare con lo strumento di Valutazione:

1. Apri la Claude Console e naviga verso l'editor di prompt.
2. Dopo aver composto il tuo prompt, cerca la scheda 'Evaluate' nella parte superiore dello schermo.

![Accesso alla Funzione di Valutazione](/docs/images/access_evaluate.png)

<Tip>
Assicurati che il tuo prompt includa almeno 1-2 variabili dinamiche utilizzando la sintassi delle doppie parentesi graffe: \{\{variable\}\}. Questo è necessario per creare set di test di valutazione.
</Tip>

## Generazione di Prompt

La Console offre un [generatore di prompt](/docs/it/build-with-claude/prompt-engineering/prompt-generator) integrato alimentato da Claude Opus 4.1:

<Steps>
  <Step title="Clicca 'Generate Prompt'">
    Cliccando sullo strumento helper 'Generate Prompt' si aprirà una finestra modale che ti permette di inserire le informazioni del tuo compito.
  </Step>
  <Step title="Descrivi il tuo compito">
    Descrivi il compito desiderato (ad esempio, "Smistare le richieste di supporto clienti in entrata") con tutti i dettagli che desideri o con pochi dettagli. Più contesto includi, più Claude può adattare il prompt generato alle tue esigenze specifiche.
  </Step>
  <Step title="Genera il tuo prompt">
    Cliccando il pulsante arancione 'Generate Prompt' in basso, Claude genererà un prompt di alta qualità per te. Puoi poi migliorare ulteriormente quei prompt utilizzando la schermata di Valutazione nella Console.
  </Step>
</Steps>

Questa funzione rende più facile creare prompt con la sintassi delle variabili appropriata per la valutazione.

![Generatore di Prompt](/docs/images/promptgenerator.png)

## Creazione di Casi di Test

Quando accedi alla schermata di Valutazione, hai diverse opzioni per creare casi di test:

1. Clicca il pulsante '+ Add Row' in basso a sinistra per aggiungere manualmente un caso.
2. Usa la funzione 'Generate Test Case' per far generare automaticamente casi di test a Claude.
3. Importa casi di test da un file CSV.

Per utilizzare la funzione 'Generate Test Case':

<Steps>
  <Step title="Clicca su 'Generate Test Case'">
    Claude genererà casi di test per te, una riga alla volta per ogni volta che clicchi il pulsante.
  </Step>
  <Step title="Modifica la logica di generazione (opzionale)">
    Puoi anche modificare la logica di generazione dei casi di test cliccando sul menu a discesa con la freccia a destra del pulsante 'Generate Test Case', poi su 'Show generation logic' nella parte superiore della finestra Variables che appare. Potresti dover cliccare `Generate' in alto a destra di questa finestra per popolare la logica di generazione iniziale.
    
    Modificare questo ti permette di personalizzare e affinare i casi di test che Claude genera con maggiore precisione e specificità.
  </Step>
</Steps>

Ecco un esempio di una schermata di Valutazione popolata con diversi casi di test:

![Schermata di Valutazione Popolata](/docs/images/eval_populated.png)

<Note>
Se aggiorni il testo del tuo prompt originale, puoi rieseguire l'intera suite di valutazione contro il nuovo prompt per vedere come i cambiamenti influenzano le prestazioni su tutti i casi di test.
</Note>

## Consigli per una Valutazione Efficace

<section title="Struttura del Prompt per la Valutazione">

Per sfruttare al meglio lo strumento di Valutazione, struttura i tuoi prompt con formati di input e output chiari. Per esempio:

```
In questo compito, genererai una storia carina di una frase che incorpora due elementi: un colore e un suono.
Il colore da includere nella storia è:
<color>
{{COLOR}}
</color>
Il suono da includere nella storia è:
<sound>
{{SOUND}}
</sound>
Ecco i passaggi per generare la storia:
1. Pensa a un oggetto, animale o scena che è comunemente associato al colore fornito. Per esempio, se il colore è "blu", potresti pensare al cielo, all'oceano o a un uccello azzurro.
2. Immagina un'azione semplice, evento o scena che coinvolge l'oggetto/animale/scena colorato che hai identificato e il suono fornito. Per esempio, se il colore è "blu" e il suono è "fischio", potresti immaginare un uccello azzurro che fischia una melodia.
3. Descrivi l'azione, evento o scena che hai immaginato in una singola frase concisa. Concentrati nel rendere la frase carina, evocativa e fantasiosa. Per esempio: "Un allegro uccello azzurro fischiava una melodia gioiosa mentre volava attraverso il cielo azzurro."
Per favore mantieni la tua storia a una sola frase. Mira a rendere quella frase il più affascinante e coinvolgente possibile incorporando naturalmente il colore e il suono dati.
Scrivi la tua storia completa di una frase dentro i tag <story>.

```

Questa struttura rende facile variare gli input (\{\{COLOR\}\} e \{\{SOUND\}\}) e valutare gli output in modo coerente.

</section>

<Tip>
Usa lo strumento helper 'Generate a prompt' nella Console per creare rapidamente prompt con la sintassi delle variabili appropriata per la valutazione.
</Tip>

## Comprensione e confronto dei risultati

Lo strumento di Valutazione offre diverse funzioni per aiutarti a raffinare i tuoi prompt:

1. **Confronto affiancato**: Confronta gli output di due o più prompt per vedere rapidamente l'impatto dei tuoi cambiamenti.
2. **Valutazione della qualità**: Valuta la qualità delle risposte su una scala di 5 punti per tracciare i miglioramenti nella qualità delle risposte per prompt.
3. **Versionamento dei prompt**: Crea nuove versioni del tuo prompt e riesegui la suite di test per iterare rapidamente e migliorare i risultati.

Rivedendo i risultati attraverso i casi di test e confrontando diverse versioni di prompt, puoi individuare pattern e fare aggiustamenti informati al tuo prompt in modo più efficiente.

Inizia a valutare i tuoi prompt oggi per costruire applicazioni AI più robuste con Claude!