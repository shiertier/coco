# Precompila la risposta di Claude per un controllo maggiore dell'output

Scopri come precompilare le risposte di Claude per guidare il suo comportamento, saltare i preamboli, applicare formati specifici e mantenere la coerenza dei personaggi.

---

<Note>
While these tips apply broadly to all Claude models, you can find prompting tips specific to extended thinking models [here](/docs/en/build-with-claude/prompt-engineering/extended-thinking-tips).
</Note>

<Note>
La precompilazione è disponibile solo per le modalità non di extended thinking. Attualmente non è supportata con extended thinking.
</Note>

Quando utilizzi Claude, hai l'abilità unica di guidare le sue risposte precompilando il messaggio `Assistant`. Questa tecnica potente ti consente di dirigere le azioni di Claude, saltare i preamboli, applicare formati specifici come JSON o XML, e persino aiutare Claude a mantenere la coerenza dei personaggi negli scenari di role-play.

In alcuni casi in cui Claude non sta funzionando come previsto, poche frasi precompilate possono migliorare notevolmente le prestazioni di Claude. Un po' di precompilazione va molto lontano!

## Come precompilare la risposta di Claude

Per precompilare, includi il testo iniziale desiderato nel messaggio `Assistant` (la risposta di Claude continuerà da dove il messaggio `Assistant` si interrompe):

```python
import anthropic

client = anthropic.Anthropic()
response = client.messages.create(
    model="claude-sonnet-4-5",
    max_tokens=1024,
    messages=[
        {"role": "user", "content": "What is your favorite color?"},
        {"role": "assistant", "content": "As an AI assistant, I don't have a favorite color, But if I had to pick, it would be green because"}  # Prefill here
    ]
)
```

<Warning>
  Il contenuto della precompilazione non può terminare con spazi finali. Una precompilazione come `"As an AI assistant, I "` (con uno spazio alla fine) risulterà in un errore.
</Warning>

### Esempi

#### Esempio 1: Controllare la formattazione dell'output e saltare il preambolo

<Tip>**Suggerimento per utenti esperti**: Precompilare `{` forza Claude a saltare il preambolo e a generare direttamente l'oggetto JSON. Questo è più pulito, più conciso e più facile da analizzare per i programmi senza elaborazione aggiuntiva.
Per un output JSON garantito che si conformi a uno schema specifico, considera di utilizzare [Structured Outputs](/docs/it/build-with-claude/structured-outputs) invece della precompilazione. Gli output strutturati garantiscono che la risposta di Claude corrisponda sempre allo schema JSON definito, rendendolo ideale per le applicazioni di produzione che richiedono la conformità rigorosa al formato.</Tip>

    <section title="Esempio: Estrazione di dati strutturati senza precompilazione">

    | Ruolo | Contenuto |
    | ---- | ------- |
    | Utente | Estrai il nome, la dimensione, il prezzo e il colore da questa descrizione del prodotto come oggetto JSON:<br/><br/>\<description><br/>Lo SmartHome Mini è un assistente per la casa intelligente compatto disponibile in nero o bianco per soli \$49.99. Con una larghezza di soli 5 pollici, ti consente di controllare luci, termostati e altri dispositivi connessi tramite voce o app, non importa dove lo posizioni nella tua casa. Questo hub conveniente e conveniente porta un controllo pratico senza mani ai tuoi dispositivi intelligenti.<br/>\</description> |
    | Assistente | Ecco le informazioni estratte in formato JSON:<br/><br/>\`\`\`json<br/>\{<br/>  "name": "SmartHome Mini",<br/>  "size": "5 inches wide",<br/>  "price": "\$49.99",<br/>  "colors": ["black", "white"]<br/>\}<br/>\`\`\`<br/><br/>Ho estratto i seguenti dettagli dalla descrizione del prodotto:<br/>- **Nome**: SmartHome Mini<br/>- **Dimensione**: 5 pollici di larghezza<br/>- **Prezzo**: \$49.99<br/>- **Colori**: Disponibile in nero e bianco<br/><br/>L'oggetto JSON fornisce un formato strutturato che è facile da analizzare e utilizzare per i programmi. Fammi sapere se hai bisogno di modifiche o se hai altre domande! |
    
</section>

    <section title="Esempio: Estrazione di dati strutturati con precompilazione">

    | Ruolo | Contenuto |
    | ---- | ------- |
    | Utente | Estrai il nome, la dimensione, il prezzo e il colore da questa descrizione del prodotto come oggetto JSON:<br/><br/>\<description><br/>Lo SmartHome Mini è un assistente per la casa intelligente compatto disponibile in nero o bianco per soli \$49.99. Con una larghezza di soli 5 pollici, ti consente di controllare luci, termostati e altri dispositivi connessi tramite voce o app, non importa dove lo posizioni nella tua casa. Questo hub conveniente e conveniente porta un controllo pratico senza mani ai tuoi dispositivi intelligenti.<br/>\</description> |
    | Assistente (precompilazione) | \{ |
    | Assistente (risposta di Claude) | "name": "SmartHome Mini",<br/>"size": "5 inches wide",<br/>"price": "\$49.99",<br/>"colors": [<br/>  "black",<br/>  "white"<br/>]<br/>\} |
    
</section>

#### Esempio 2: Mantenere il personaggio negli scenari di role-play

<Tip>**Suggerimento per il role-play**: Precompilare un `[ROLE_NAME]` tra parentesi quadre può ricordare a Claude di rimanere in carattere, anche per conversazioni più lunghe e complesse. Questo è particolarmente potente se combinato con il role prompting nel parametro `system`.</Tip>

    <section title="Esempio: Mantenere il personaggio senza role prompting">

    | Ruolo | Contenuto |
    | ---- | ------- |
    | Utente (50 turni di conversazione con Claude "Sherlock Holmes") | Cosa deduci del proprietario di questa scarpa? \<image>muddy_shoe.jpg\</image> |
    | Assistente | Come assistente AI, sarei felice di analizzare l'immagine di una scarpa fangosa come se l'avessi fornita. Ecco la mia deduzione... |
    
</section>

    <section title="Esempio: Mantenere il personaggio con role prompting">

    | Ruolo | Contenuto |
    | ---- | ------- |
    | Utente (50 turni di conversazione con Claude "Sherlock Holmes") | Cosa deduci del proprietario di questa scarpa? <br/>\<image>muddy_shoe.jpg\</image> |
    | Assistente (precompilazione) | [Sherlock Holmes] |
    | Assistente (risposta di Claude) | Ah, cosa abbiamo qui? Un esemplare molto curioso, Watson! Esaminiamo questa scarpa con un occhio critico. Il fango, vedi, non è la comune argilla di Londra, ma piuttosto la tonalità rossastra distintiva del suolo ricco di ferro che si trova solo nei dintorni di Hampstead Heath. Eppure, l'usura sulla suola suggerisce un'andatura più adatta ai ciottoli di Whitechapel che ai dolci pendii della Heath.<br/><br/>Ora, osserva i lacci! Sono legati con un nodo particolare preferito da coloro che esercitano la professione medica, senza dubbio un'abitudine acquisita durante anni di legatura di camici chirurgici. Ma i segni di sfregamento sulla punta della scarpa raccontano una storia diversa: sono coerenti con il movimento di calcio che si potrebbe impiegare nelle arti marziali, specificamente il Baritsu.<br/><br/>Possiamo concludere, quindi, che il nostro proprietario di scarpe assente è un medico, possibilmente un chirurgo, che risiede a Whitechapel ma frequenta Hampstead Heath, forse per una pratica segreta di Baritsu, lontano da occhi indiscreti. Un caso molto intrigante, Watson! Dobbiamo affrettarci; il gioco è iniziato! |
    
</section>

---

<CardGroup cols={3}>
  <Card title="Lavorare con i messaggi" icon="messages" href="/docs/it/build-with-claude/working-with-messages">
    Vedi altri esempi di precompilazione e altri pattern dell'API Messages.
  </Card>
  <Card title="Libreria di prompt" icon="link" href="/docs/it/resources/prompt-library/library">
    Ispirati da una selezione curata di prompt per vari compiti e casi d'uso.
  </Card>
  <Card title="Tutorial di prompting su GitHub" icon="link" href="https://github.com/anthropics/prompt-eng-interactive-tutorial">
    Un tutorial ricco di esempi che copre i concetti di prompt engineering trovati nella nostra documentazione.
  </Card>
  <Card title="Tutorial di prompting su Google Sheets" icon="link" href="https://docs.google.com/spreadsheets/d/19jzLgRruG9kjUQNKtCg1ZjdD6l6weA6qRXG5zLIAhC8">
    Una versione più leggera del nostro tutorial di prompt engineering tramite un foglio di calcolo interattivo.
  </Card>
</CardGroup>