# Instradamento dei ticket

Questa guida illustra come sfruttare le capacità avanzate di comprensione del linguaggio naturale di Claude per classificare i ticket di supporto clienti su larga scala in base all'intento del cliente, all'urgenza, alla prioritizzazione, al profilo del cliente e altro ancora.

---

## Definire se utilizzare Claude per l'instradamento dei ticket

Ecco alcuni indicatori chiave che suggeriscono di utilizzare un LLM come Claude invece di approcci ML tradizionali per il tuo compito di classificazione:

    <section title="Hai a disposizione dati di addestramento etichettati limitati">

        I processi ML tradizionali richiedono enormi set di dati etichettati. Il modello pre-addestrato di Claude può classificare efficacemente i ticket con solo poche decine di esempi etichettati, riducendo significativamente il tempo e i costi di preparazione dei dati.
    
</section>
    <section title="Le tue categorie di classificazione potrebbero cambiare o evolversi nel tempo">

        Una volta stabilito un approccio ML tradizionale, modificarlo è un'impresa laboriosa e ad alta intensità di dati. D'altra parte, man mano che il tuo prodotto o le esigenze dei clienti evolvono, Claude può adattarsi facilmente ai cambiamenti nelle definizioni delle classi o alle nuove classi senza un'estesa rietichettatura dei dati di addestramento.
    
</section>
    <section title="Hai bisogno di gestire input di testo complessi e non strutturati">

        I modelli ML tradizionali spesso faticano con dati non strutturati e richiedono un'estesa ingegneria delle caratteristiche. La comprensione avanzata del linguaggio di Claude consente una classificazione accurata basata su contenuto e contesto, piuttosto che affidarsi a strutture ontologiche rigide.
    
</section>
    <section title="Le tue regole di classificazione si basano sulla comprensione semantica">

        Gli approcci ML tradizionali spesso si basano su modelli bag-of-words o semplici pattern matching. Claude eccelle nel comprendere e applicare regole sottostanti quando le classi sono definite da condizioni piuttosto che da esempi.
    
</section>
    <section title="Hai bisogno di un ragionamento interpretabile per le decisioni di classificazione">

        Molti modelli ML tradizionali forniscono poca visibilità nel loro processo decisionale. Claude può fornire spiegazioni leggibili dall'uomo per le sue decisioni di classificazione, costruendo fiducia nel sistema di automazione e facilitando un facile adattamento se necessario.
    
</section>
    <section title="Vuoi gestire i casi limite e i ticket ambigui in modo più efficace">

        I sistemi ML tradizionali spesso faticano con outlier e input ambigui, classificandoli frequentemente in modo errato o ricorrendo a una categoria catch-all. Le capacità di elaborazione del linguaggio naturale di Claude gli consentono di interpretare meglio il contesto e le sfumature nei ticket di supporto, riducendo potenzialmente il numero di ticket instradati erroneamente o non classificati che richiedono intervento manuale.
    
</section>
    <section title="Hai bisogno di supporto multilingue senza mantenere modelli separati">

        Gli approcci ML tradizionali in genere richiedono modelli separati o processi di traduzione estesi per ogni lingua supportata. Le capacità multilingue di Claude gli consentono di classificare i ticket in varie lingue senza la necessità di modelli separati o processi di traduzione estesi, semplificando il supporto per basi di clienti globali.
    
</section>

***

##  Costruire e distribuire il tuo flusso di lavoro di supporto LLM

### Comprendere il tuo approccio di supporto attuale
Prima di tuffarti nell'automazione, è fondamentale comprendere il tuo sistema di ticketing esistente. Inizia investigando come il tuo team di supporto gestisce attualmente l'instradamento dei ticket.

Considera domande come:
* Quali criteri vengono utilizzati per determinare quale SLA/offerta di servizio viene applicata?
* L'instradamento dei ticket viene utilizzato per determinare a quale livello di supporto o specialista di prodotto va il ticket?
* Ci sono regole o flussi di lavoro automatizzati già in atto? In quali casi falliscono?
* Come vengono gestiti i casi limite o i ticket ambigui?
* Come il team prioritizza i ticket?

Più conosci come gli umani gestiscono determinati casi, meglio sarai in grado di lavorare con Claude per svolgere il compito.

### Definire le categorie di intento dell'utente
Un elenco ben definito di categorie di intento dell'utente è fondamentale per una classificazione accurata dei ticket di supporto con Claude. La capacità di Claude di instradare i ticket efficacemente all'interno del tuo sistema è direttamente proporzionale a quanto bene sono definite le categorie del tuo sistema.

Ecco alcuni esempi di categorie e sottocategorie di intento dell'utente.

    <section title="Problema tecnico">

        * Problema hardware
        * Bug del software
        * Problema di compatibilità
        * Problema di prestazioni
    
</section>
    <section title="Gestione dell'account">

        * Ripristino della password
        * Problemi di accesso all'account
        * Richieste di fatturazione
        * Modifiche dell'abbonamento
    
</section>
    <section title="Informazioni sul prodotto">

        * Richieste di funzionalità
        * Domande sulla compatibilità del prodotto
        * Informazioni sui prezzi
        * Richieste di disponibilità
    
</section>
    <section title="Guida dell'utente">

        * Domande come fare
        * Assistenza nell'utilizzo delle funzionalità
        * Consigli sulle best practice
        * Guida alla risoluzione dei problemi
    
</section>
    <section title="Feedback">

        * Segnalazioni di bug
        * Richieste di funzionalità
        * Feedback generale o suggerimenti
        * Reclami
    
</section>
    <section title="Correlato agli ordini">

        * Richieste di stato dell'ordine
        * Informazioni sulla spedizione
        * Resi e scambi
        * Modifiche dell'ordine
    
</section>
    <section title="Richiesta di servizio">

        * Assistenza all'installazione
        * Richieste di aggiornamento
        * Pianificazione della manutenzione
        * Cancellazione del servizio
    
</section>
    <section title="Problemi di sicurezza">

        * Richieste di privacy dei dati
        * Segnalazioni di attività sospette
        * Assistenza per le funzionalità di sicurezza
    
</section>
    <section title="Conformità e questioni legali">

        * Domande sulla conformità normativa
        * Richieste di termini di servizio
        * Richieste di documentazione legale
    
</section>
    <section title="Supporto di emergenza">

        * Guasti critici del sistema
        * Problemi di sicurezza urgenti
        * Problemi sensibili al fattore tempo
    
</section>
    <section title="Formazione ed educazione">

        * Richieste di formazione sui prodotti
        * Richieste di documentazione
        * Informazioni su webinar o workshop
    
</section>
    <section title="Integrazione e API">

        * Assistenza all'integrazione
        * Domande sull'utilizzo dell'API
        * Richieste di compatibilità con terze parti
    
</section>

Oltre all'intento, l'instradamento e la prioritizzazione dei ticket possono anche essere influenzati da altri fattori come urgenza, tipo di cliente, SLA o lingua. Assicurati di considerare altri criteri di instradamento quando costruisci il tuo sistema di instradamento automatizzato.

### Stabilire i criteri di successo

Lavora con il tuo team di supporto per [definire chiari criteri di successo](/docs/it/test-and-evaluate/define-success) con benchmark, soglie e obiettivi misurabili.

Ecco alcuni criteri e benchmark standard quando si utilizzano LLM per l'instradamento dei ticket di supporto:

    <section title="Coerenza della classificazione">

        Questa metrica valuta quanto coerentemente Claude classifica ticket simili nel tempo. È fondamentale per mantenere l'affidabilità dell'instradamento. Misuralo testando periodicamente il modello con un set di input standardizzati e mirando a un tasso di coerenza del 95% o superiore.
    
</section>
    <section title="Velocità di adattamento">

        Questo misura quanto velocemente Claude può adattarsi a nuove categorie o a cambiamenti nei pattern dei ticket. Testalo introducendo nuovi tipi di ticket e misurando il tempo necessario al modello per raggiungere un'accuratezza soddisfacente (ad es. >90%) su queste nuove categorie. Mira all'adattamento entro 50-100 ticket di esempio.
    
</section>
    <section title="Gestione multilingue">

        Questo valuta la capacità di Claude di instradare accuratamente i ticket in più lingue. Misura l'accuratezza dell'instradamento in diverse lingue, mirando a non più di una riduzione di accuratezza del 5-10% per le lingue non primarie.
    
</section>
    <section title="Gestione dei casi limite">

        Questo valuta le prestazioni di Claude su ticket insoliti o complessi. Crea un set di test di casi limite e misura l'accuratezza dell'instradamento, mirando ad almeno l'80% di accuratezza su questi input impegnativi.
    
</section>
    <section title="Mitigazione dei pregiudizi">

        Questo misura l'equità di Claude nell'instradamento tra diversi dati demografici dei clienti. Controlla regolarmente le decisioni di instradamento per potenziali pregiudizi, mirando a un'accuratezza di instradamento coerente (entro il 2-3%) in tutti i gruppi di clienti.
    
</section>
    <section title="Efficienza del prompt">

        In situazioni in cui minimizzare il conteggio dei token è cruciale, questo criterio valuta quanto bene Claude si comporta con contesto minimo. Misura l'accuratezza dell'instradamento con quantità variabili di contesto fornito, mirando a un'accuratezza del 90%+ con solo il titolo del ticket e una breve descrizione.
    
</section>
    <section title="Punteggio di spiegabilità">

        Questo valuta la qualità e la rilevanza delle spiegazioni di Claude per le sue decisioni di instradamento. I valutatori umani possono valutare le spiegazioni su una scala (ad es. 1-5), con l'obiettivo di raggiungere un punteggio medio di 4 o superiore.
    
</section>

Ecco alcuni criteri di successo comuni che potrebbero essere utili indipendentemente dal fatto che venga utilizzato un LLM:

    <section title="Accuratezza dell'instradamento">

        L'accuratezza dell'instradamento misura quanto spesso i ticket vengono assegnati correttamente al team o all'individuo appropriato al primo tentativo. Questo è in genere misurato come percentuale di ticket instradati correttamente su ticket totali. I benchmark del settore spesso mirano a un'accuratezza del 90-95%, anche se questo può variare in base alla complessità della struttura di supporto.
    
</section>
    <section title="Tempo di assegnazione">

        Questa metrica traccia quanto velocemente i ticket vengono assegnati dopo essere stati inviati. I tempi di assegnazione più veloci generalmente portano a risoluzioni più rapide e a una maggiore soddisfazione dei clienti. I sistemi best-in-class spesso raggiungono tempi di assegnazione medi inferiori a 5 minuti, con molti che mirano all'instradamento quasi istantaneo (che è possibile con implementazioni LLM).
    
</section>
    <section title="Tasso di reinoltro">

        Il tasso di reinoltro indica quanto spesso i ticket devono essere riassegnati dopo l'instradamento iniziale. Un tasso più basso suggerisce un instradamento iniziale più accurato. Mira a un tasso di reinoltro inferiore al 10%, con i sistemi con le migliori prestazioni che raggiungono tassi del 5% o inferiori.
    
</section>
    <section title="Tasso di risoluzione al primo contatto">

        Questo misura la percentuale di ticket risolti durante la prima interazione con il cliente. Tassi più elevati indicano un instradamento efficiente e team di supporto ben preparati. I benchmark del settore in genere vanno dal 70-75%, con i migliori performer che raggiungono tassi dell'80% o superiori.
    
</section>
    <section title="Tempo medio di gestione">

        Il tempo medio di gestione misura quanto tempo ci vuole per risolvere un ticket dall'inizio alla fine. Un instradamento efficiente può ridurre significativamente questo tempo. I benchmark variano ampiamente per industria e complessità, ma molte organizzazioni mirano a mantenere il tempo medio di gestione sotto le 24 ore per i problemi non critici.
    
</section>
    <section title="Punteggi di soddisfazione dei clienti">

        Spesso misurati attraverso sondaggi post-interazione, questi punteggi riflettono la felicità generale dei clienti con il processo di supporto. Un instradamento efficace contribuisce a una maggiore soddisfazione. Mira a punteggi CSAT del 90% o superiori, con i migliori performer che spesso raggiungono tassi di soddisfazione del 95%+.
    
</section>
    <section title="Tasso di escalation">

        Questo misura quanto spesso i ticket devono essere escalati a livelli di supporto superiori. Tassi di escalation più bassi spesso indicano un instradamento iniziale più accurato. Sforzati di ottenere un tasso di escalation inferiore al 20%, con i sistemi best-in-class che raggiungono tassi del 10% o inferiori.
    
</section>
    <section title="Produttività dell'agente">

        Questa metrica esamina quanti ticket gli agenti possono gestire efficacemente dopo aver implementato la soluzione di instradamento. Un instradamento migliorato dovrebbe aumentare la produttività. Misuralo tracciando i ticket risolti per agente al giorno o all'ora, mirando a un miglioramento del 10-20% dopo aver implementato un nuovo sistema di instradamento.
    
</section>
    <section title="Tasso di deflazione self-service">

        Questo misura la percentuale di potenziali ticket risolti attraverso opzioni self-service prima di entrare nel sistema di instradamento. Tassi più elevati indicano un triage pre-instradamento efficace. Mira a un tasso di deflazione del 20-30%, con i migliori performer che raggiungono tassi del 40% o superiori.
    
</section>
    <section title="Costo per ticket">

        Questa metrica calcola il costo medio per risolvere ogni ticket di supporto. Un instradamento efficiente dovrebbe aiutare a ridurre questo costo nel tempo. Mentre i benchmark variano ampiamente, molte organizzazioni mirano a ridurre il costo per ticket del 10-15% dopo aver implementato un sistema di instradamento migliorato.
    
</section>

### Scegliere il modello Claude giusto

La scelta del modello dipende dai compromessi tra costo, accuratezza e tempo di risposta.

Molti clienti hanno trovato `claude-haiku-4-5-20251001` un modello ideale per l'instradamento dei ticket, poiché è il modello più veloce e conveniente della famiglia Claude 4 pur fornendo risultati eccellenti. Se il tuo problema di classificazione richiede una profonda esperienza in materia o un grande volume di categorie di intento con ragionamento complesso, potresti optare per il [modello Sonnet più grande](/docs/it/about-claude/models).

### Costruire un prompt forte

L'instradamento dei ticket è un tipo di compito di classificazione. Claude analizza il contenuto di un ticket di supporto e lo classifica in categorie predefinite in base al tipo di problema, all'urgenza, all'expertise richiesta o ad altri fattori rilevanti.

Scriviamo un prompt di classificazione dei ticket. Il nostro prompt iniziale dovrebbe contenere il contenuto della richiesta dell'utente e restituire sia il ragionamento che l'intento.

<Tip>
Prova il [generatore di prompt](/docs/it/prompt-generator) sulla [Claude Console](/login) per far scrivere a Claude una prima bozza per te.
</Tip>

Ecco un esempio di prompt di classificazione dell'instradamento dei ticket:
```python
def classify_support_request(ticket_contents):
    # Define the prompt for the classification task
    classification_prompt = f"""You will be acting as a customer support ticket classification system. Your task is to analyze customer support requests and output the appropriate classification intent for each request, along with your reasoning. 

        Here is the customer support request you need to classify:

        <request>{ticket_contents}</request>

        Please carefully analyze the above request to determine the customer's core intent and needs. Consider what the customer is asking for has concerns about.

        First, write out your reasoning and analysis of how to classify this request inside <reasoning> tags.

        Then, output the appropriate classification label for the request inside a <intent> tag. The valid intents are:
        <intents>
        <intent>Support, Feedback, Complaint</intent>
        <intent>Order Tracking</intent>
        <intent>Refund/Exchange</intent>
        </intents>

        A request may have ONLY ONE applicable intent. Only include the intent that is most applicable to the request.

        As an example, consider the following request:
        <request>Hello! I had high-speed fiber internet installed on Saturday and my installer, Kevin, was absolutely fantastic! Where can I send my positive review? Thanks for your help!</request>

        Here is an example of how your output should be formatted (for the above example request):
        <reasoning>The user seeks information in order to leave positive feedback.</reasoning>
        <intent>Support, Feedback, Complaint</intent>

        Here are a few more examples:
        <examples>
        <example 2>
        Example 2 Input:
        <request>I wanted to write and personally thank you for the compassion you showed towards my family during my father's funeral this past weekend. Your staff was so considerate and helpful throughout this whole process; it really took a load off our shoulders. The visitation brochures were beautiful. We'll never forget the kindness you showed us and we are so appreciative of how smoothly the proceedings went. Thank you, again, Amarantha Hill on behalf of the Hill Family.</request>

        Example 2 Output:
        <reasoning>User leaves a positive review of their experience.</reasoning>
        <intent>Support, Feedback, Complaint</intent>
        </example 2>
        <example 3>

        ...

        </example 8>
        <example 9>
        Example 9 Input:
        <request>Your website keeps sending ad-popups that block the entire screen. It took me twenty minutes just to finally find the phone number to call and complain. How can I possibly access my account information with all of these popups? Can you access my account for me, since your website is broken? I need to know what the address is on file.</request>

        Example 9 Output:
        <reasoning>The user requests help accessing their web account information.</reasoning>
        <intent>Support, Feedback, Complaint</intent>
        </example 9>

        Remember to always include your classification reasoning before your actual intent output. The reasoning should be enclosed in <reasoning> tags and the intent in <intent> tags. Return only the reasoning and the intent.
        """
```

Analizziamo i componenti chiave di questo prompt:
* Utilizziamo le f-string di Python per creare il modello di prompt, consentendo l'inserimento di `ticket_contents` nei tag `<request>`.
* Diamo a Claude un ruolo chiaramente definito come sistema di classificazione che analizza attentamente il contenuto del ticket per determinare l'intento e le esigenze fondamentali del cliente.
* Istruiamo Claude sulla corretta formattazione dell'output, in questo caso per fornire il suo ragionamento e analisi all'interno dei tag `<reasoning>`, seguito dall'etichetta di classificazione appropriata all'interno dei tag `<intent>`.
* Specifichiamo le categorie di intento valide: "Support, Feedback, Complaint", "Order Tracking" e "Refund/Exchange".
* Includiamo alcuni esempi (a.k.a. few-shot prompting) per illustrare come dovrebbe essere formattato l'output, il che migliora l'accuratezza e la coerenza.

Il motivo per cui vogliamo che Claude divida la sua risposta in varie sezioni di tag XML è in modo che possiamo utilizzare espressioni regolari per estrarre separatamente il ragionamento e l'intento dall'output. Questo ci consente di creare passaggi successivi mirati nel flusso di lavoro di instradamento dei ticket, come utilizzare solo l'intento per decidere a quale persona instradare il ticket.

### Distribuire il tuo prompt

È difficile sapere quanto bene funziona il tuo prompt senza distribuirlo in un'impostazione di produzione di test ed [eseguire valutazioni](/docs/it/test-and-evaluate/develop-tests).

Costruiamo la struttura di distribuzione. Inizia definendo la firma del metodo per avvolgere la nostra chiamata a Claude. Prenderemo il metodo che abbiamo già iniziato a scrivere, che ha `ticket_contents` come input, e ora restituiremo una tupla di `reasoning` e `intent` come output. Se hai un'automazione esistente utilizzando ML tradizionale, vorrai seguire quella firma del metodo.

```python
import anthropic
import re

# Create an instance of the Claude API client
client = anthropic.Anthropic()

# Set the default model
DEFAULT_MODEL="claude-haiku-4-5-20251001"

def classify_support_request(ticket_contents):
    # Define the prompt for the classification task
    classification_prompt = f"""You will be acting as a customer support ticket classification system. 
        ...
        ... The reasoning should be enclosed in <reasoning> tags and the intent in <intent> tags. Return only the reasoning and the intent.
        """
    # Send the prompt to the API to classify the support request.
    message = client.messages.create(
        model=DEFAULT_MODEL,
        max_tokens=500,
        temperature=0,
        messages=[{"role": "user", "content": classification_prompt}],
        stream=False,
    )
    reasoning_and_intent = message.content[0].text

    # Use Python's regular expressions library to extract `reasoning`.
    reasoning_match = re.search(
        r"<reasoning>(.*?)</reasoning>", reasoning_and_intent, re.DOTALL
    )
    reasoning = reasoning_match.group(1).strip() if reasoning_match else ""

    # Similarly, also extract the `intent`.
    intent_match = re.search(r"<intent>(.*?)</intent>", reasoning_and_intent, re.DOTALL)
    intent = intent_match.group(1).strip() if intent_match else ""

    return reasoning, intent
```

Questo codice:
* Importa la libreria Anthropic e crea un'istanza del client utilizzando la tua chiave API.
* Definisce una funzione `classify_support_request` che accetta una stringa `ticket_contents`.
* Invia `ticket_contents` a Claude per la classificazione utilizzando `classification_prompt`
* Restituisce il `reasoning` e l'`intent` del modello estratti dalla risposta.

Poiché abbiamo bisogno di attendere che l'intero testo di ragionamento e intento sia generato prima di analizzarlo, impostiamo `stream=False` (il valore predefinito).

***

## Valutare il tuo prompt

Il prompting spesso richiede test e ottimizzazione per essere pronto per la produzione. Per determinare la prontezza della tua soluzione, valuta le prestazioni in base ai criteri di successo e alle soglie che hai stabilito in precedenza.

Per eseguire la tua valutazione, avrai bisogno di casi di test su cui eseguirla. Il resto di questa guida presuppone che tu abbia già [sviluppato i tuoi casi di test](/docs/it/test-and-evaluate/develop-tests).

### Costruire una funzione di valutazione

La nostra valutazione di esempio per questa guida misura le prestazioni di Claude lungo tre metriche chiave:
* Accuratezza
* Costo per classificazione

Potrebbe essere necessario valutare Claude su altri assi a seconda di quali fattori sono importanti per te.

Per valutare questo, dobbiamo prima modificare lo script che abbiamo scritto e aggiungere una funzione per confrontare l'intento previsto con l'intento effettivo e calcolare la percentuale di previsioni corrette. Dobbiamo anche aggiungere funzionalità di calcolo dei costi e misurazione del tempo.

```python
import anthropic
import re

# Create an instance of the Claude API client
client = anthropic.Anthropic()

# Set the default model
DEFAULT_MODEL="claude-haiku-4-5-20251001"

def classify_support_request(request, actual_intent):
    # Define the prompt for the classification task
    classification_prompt = f"""You will be acting as a customer support ticket classification system. 
        ...
        ...The reasoning should be enclosed in <reasoning> tags and the intent in <intent> tags. Return only the reasoning and the intent.
        """

    message = client.messages.create(
        model=DEFAULT_MODEL,
        max_tokens=500,
        temperature=0,
        messages=[{"role": "user", "content": classification_prompt}],
    )
    usage = message.usage  # Get the usage statistics for the API call for how many input and output tokens were used.
    reasoning_and_intent = message.content[0].text

    # Use Python's regular expressions library to extract `reasoning`.
    reasoning_match = re.search(
        r"<reasoning>(.*?)</reasoning>", reasoning_and_intent, re.DOTALL
    )
    reasoning = reasoning_match.group(1).strip() if reasoning_match else ""

    # Similarly, also extract the `intent`.
    intent_match = re.search(r"<intent>(.*?)</intent>", reasoning_and_intent, re.DOTALL)
    intent = intent_match.group(1).strip() if intent_match else ""

      # Check if the model's prediction is correct.
    correct = actual_intent.strip() == intent.strip()

    # Return the reasoning, intent, correct, and usage.
    return reasoning, intent, correct, usage
```

Analizziamo le modifiche che abbiamo apportato:
* Abbiamo aggiunto l'`actual_intent` dai nostri casi di test nel metodo `classify_support_request` e impostato un confronto per valutare se la classificazione dell'intento di Claude corrisponde alla nostra classificazione dell'intento dorato.
* Abbiamo estratto le statistiche di utilizzo per la chiamata API per calcolare il costo in base ai token di input e output utilizzati

### Eseguire la tua valutazione

Una valutazione corretta richiede soglie e benchmark chiari per determinare quale sia un buon risultato. Lo script sopra ci darà i valori di runtime per accuratezza, tempo di risposta e costo per classificazione, ma avremmo comunque bisogno di soglie chiaramente stabilite. Ad esempio:
* **Accuratezza:** 95% (su 100 test)
* **Costo per classificazione:** riduzione del 50% in media (su 100 test) rispetto al metodo di instradamento attuale

Avere queste soglie ti consente di dire rapidamente e facilmente su larga scala, e con empirismo imparziale, quale metodo è migliore per te e quali cambiamenti potrebbero essere necessari per adattarsi meglio ai tuoi requisiti.

***

## Migliorare le prestazioni

In scenari complessi, potrebbe essere utile considerare strategie aggiuntive per migliorare le prestazioni oltre le [tecniche standard di prompt engineering](/docs/it/build-with-claude/prompt-engineering/overview) e le [strategie di implementazione dei guardrail](/docs/it/test-and-evaluate/strengthen-guardrails/reduce-hallucinations). Ecco alcuni scenari comuni:

### Utilizzare una gerarchia tassonomica per i casi con 20+ categorie di intento

Man mano che il numero di classi cresce, cresce anche il numero di esempi richiesti, rendendo potenzialmente il prompt ingombrante. In alternativa, puoi considerare l'implementazione di un sistema di classificazione gerarchico utilizzando una miscela di classificatori.
1. Organizza i tuoi intenti in una struttura di albero tassonomico.
2. Crea una serie di classificatori a ogni livello dell'albero, abilitando un approccio di instradamento a cascata.

Ad esempio, potresti avere un classificatore di primo livello che categorizza ampiamente i ticket in "Problemi tecnici", "Domande sulla fatturazione" e "Richieste generali". Ognuna di queste categorie può quindi avere il suo proprio sotto-classificatore per affinare ulteriormente la classificazione.

![](/docs/images/ticket-hierarchy.png)

* **Pro - maggiore sfumatura e accuratezza:** Puoi creare prompt diversi per ogni percorso genitore, consentendo una classificazione più mirata e specifica del contesto. Questo può portare a una maggiore accuratezza e a una gestione più sfumata delle richieste dei clienti.

* **Contro - latenza aumentata:** Tieni presente che più classificatori possono portare a una latenza aumentata, e consigliamo di implementare questo approccio con il nostro modello più veloce, Haiku.

### Utilizzare database vettoriali e ricerca di somiglianza per gestire ticket altamente variabili

Nonostante la fornitura di esempi sia il modo più efficace per migliorare le prestazioni, se le richieste di supporto sono altamente variabili, può essere difficile includere abbastanza esempi in un singolo prompt.

In questo scenario, potresti utilizzare un database vettoriale per eseguire ricerche di somiglianza da un set di dati di esempi e recuperare gli esempi più rilevanti per una determinata query.

Questo approccio, descritto in dettaglio nella nostra [ricetta di classificazione](https://github.com/anthropics/anthropic-cookbook/blob/82675c124e1344639b2a875aa9d3ae854709cd83/skills/classification/guide.ipynb), ha dimostrato di migliorare le prestazioni dal 71% di accuratezza al 93% di accuratezza.

### Tenere conto specificamente dei casi limite previsti

Ecco alcuni scenari in cui Claude potrebbe classificare erroneamente i ticket (potrebbero essercene altri unici per la tua situazione). In questi scenari, considera di fornire istruzioni esplicite o esempi nel prompt di come Claude dovrebbe gestire il caso limite:

    <section title="I clienti fanno richieste implicite">

        I clienti spesso esprimono le esigenze indirettamente. Ad esempio, "Sto aspettando il mio pacco da più di due settimane" potrebbe essere una richiesta indiretta di stato dell'ordine.
        * **Soluzione:** Fornisci a Claude alcuni esempi reali di clienti di questi tipi di richieste, insieme a quale sia l'intento sottostante. Puoi ottenere risultati ancora migliori se includi una logica di classificazione per intenti di ticket particolarmente sfumati, in modo che Claude possa generalizzare meglio la logica ad altri ticket.
    
</section>
    <section title="Claude prioritizza l'emozione rispetto all'intento">

        Quando i clienti esprimono insoddisfazione, Claude potrebbe prioritizzare l'affrontare l'emozione rispetto alla risoluzione del problema sottostante.
        * **Soluzione:** Fornisci a Claude indicazioni su quando prioritizzare il sentimento del cliente o meno. Può essere semplice come "Ignora tutte le emozioni dei clienti. Concentrati solo sull'analisi dell'intento della richiesta del cliente e su quali informazioni il cliente potrebbe chiedere."
    
</section>
    <section title="Più problemi causano confusione nella prioritizzazione dei problemi">

        Quando i clienti presentano più problemi in una singola interazione, Claude potrebbe avere difficoltà a identificare la preoccupazione principale.
        * **Soluzione:** Chiarisci la prioritizzazione degli intenti in modo che Claude possa classificare meglio gli intenti estratti e identificare la preoccupazione principale.
    
</section>

***

## Integrare Claude nel tuo flusso di lavoro di supporto più ampio

L'integrazione corretta richiede che tu prenda alcune decisioni riguardanti il modo in cui il tuo script di instradamento dei ticket basato su Claude si adatta all'architettura del tuo sistema di instradamento dei ticket più ampio. Ci sono due modi in cui potresti farlo:
* **Basato su push:** Il sistema di ticket di supporto che stai utilizzando (ad es. Zendesk) attiva il tuo codice inviando un evento webhook al tuo servizio di instradamento, che quindi classifica l'intento e lo instrada.
    * Questo approccio è più scalabile sul web, ma richiede di esporre un endpoint pubblico.
* **Basato su pull:** Il tuo codice estrae i ticket più recenti in base a una pianificazione data e li instrada al momento del pull.
    * Questo approccio è più facile da implementare ma potrebbe fare chiamate non necessarie al sistema di ticket di supporto quando la frequenza di pull è troppo alta o potrebbe essere eccessivamente lento quando la frequenza di pull è troppo bassa.

Per uno di questi approcci, dovrai avvolgere il tuo script in un servizio. La scelta dell'approccio dipende da quali API fornisce il tuo sistema di ticketing di supporto.

***

<CardGroup cols={2}>
    <Card title="Classification cookbook" icon="link" href="https://github.com/anthropics/anthropic-cookbook/tree/main/capabilities/classification">
        Visita il nostro classification cookbook per ulteriori codici di esempio e una guida di valutazione dettagliata.
    </Card>
    <Card title="Claude Console" icon="link" href="/dashboard">
        Inizia a costruire e valutare il tuo flusso di lavoro sulla Claude Console.
    </Card>
</CardGroup>