# Sintesi di documenti legali

Questa guida illustra come sfruttare le capacità avanzate di elaborazione del linguaggio naturale di Claude per riassumere in modo efficiente i documenti legali, estraendo informazioni chiave e accelerando la ricerca legale. Con Claude, puoi semplificare la revisione di contratti, la preparazione del contenzioso e il lavoro normativo, risparmiando tempo e garantendo accuratezza nei tuoi processi legali.

---

> Visita il nostro [manuale sulla sintesi](https://github.com/anthropics/anthropic-cookbook/blob/main/skills/summarization/guide.ipynb) per vedere un'implementazione di esempio di sintesi legale utilizzando Claude.

## Prima di iniziare a costruire con Claude

### Decidi se utilizzare Claude per la sintesi di documenti legali

Ecco alcuni indicatori chiave che suggeriscono di utilizzare un LLM come Claude per riassumere documenti legali:

<section title="Desideri rivedere un volume elevato di documenti in modo efficiente e conveniente">
La revisione di documenti su larga scala può essere dispendiosa in termini di tempo e costi quando eseguita manualmente. Claude può elaborare e riassumere rapidamente grandi quantità di documenti legali, riducendo significativamente il tempo e i costi associati alla revisione dei documenti. Questa capacità è particolarmente preziosa per attività come la due diligence, l'analisi dei contratti o la scoperta nel contenzioso, dove l'efficienza è cruciale.
</section>
<section title="Hai bisogno dell'estrazione automatizzata di metadati chiave">
Claude può estrarre e categorizzare in modo efficiente i metadati importanti dai documenti legali, come le parti coinvolte, le date, i termini del contratto o clausole specifiche. Questa estrazione automatizzata può aiutare a organizzare le informazioni, rendendo più facile cercare, analizzare e gestire grandi insiemi di documenti. È particolarmente utile per la gestione dei contratti, i controlli di conformità o la creazione di database ricercabili di informazioni legali.
</section>
<section title="Desideri generare riassunti chiari, concisi e standardizzati">
Claude può generare riassunti strutturati che seguono formati predeterminati, rendendo più facile per i professionisti legali comprendere rapidamente i punti chiave di vari documenti. Questi riassunti standardizzati possono migliorare la leggibilità, facilitare il confronto tra documenti e migliorare la comprensione complessiva, soprattutto quando si ha a che fare con linguaggio legale complesso o gergo tecnico.
</section>
<section title="Hai bisogno di citazioni precise per i tuoi riassunti">
Quando si creano riassunti legali, l'attribuzione corretta e la citazione sono cruciali per garantire credibilità e conformità agli standard legali. Claude può essere istruito per includere citazioni accurate per tutti i punti legali referenziati, rendendo più facile per i professionisti legali rivedere e verificare le informazioni riassunte.
</section>
<section title="Desideri semplificare e accelerare il tuo processo di ricerca legale">
Claude può assistere nella ricerca legale analizzando rapidamente grandi volumi di giurisprudenza, statuti e commenti legali. Può identificare i precedenti rilevanti, estrarre i principi legali chiave e riassumere gli argomenti legali complessi. Questa capacità può accelerare significativamente il processo di ricerca, consentendo ai professionisti legali di concentrarsi su analisi di livello superiore e sviluppo della strategia.
</section>

### Determina i dettagli che desideri che la sintesi estragga
Non esiste un unico riassunto corretto per un determinato documento. Senza una direzione chiara, può essere difficile per Claude determinare quali dettagli includere. Per ottenere risultati ottimali, identifica le informazioni specifiche che desideri includere nel riassunto.

Ad esempio, quando si riassume un accordo di subaffitto, potresti desiderare di estrarre i seguenti punti chiave:

```python
details_to_extract = [
    'Parties involved (sublessor, sublessee, and original lessor)',
    'Property details (address, description, and permitted use)', 
    'Term and rent (start date, end date, monthly rent, and security deposit)',
    'Responsibilities (utilities, maintenance, and repairs)',
    'Consent and notices (landlord\'s consent, and notice requirements)',
    'Special provisions (furniture, parking, and subletting restrictions)'
]
```

### Stabilisci i criteri di successo

La valutazione della qualità dei riassunti è notoriamente un compito impegnativo. A differenza di molti altri compiti di elaborazione del linguaggio naturale, la valutazione dei riassunti spesso manca di metriche chiare e obiettive. Il processo può essere altamente soggettivo, con diversi lettori che valorizzano aspetti diversi di un riassunto. Ecco i criteri che potresti desiderare di considerare quando valuti quanto bene Claude esegue la sintesi legale.

<section title="Correttezza fattuale">
Il riassunto dovrebbe rappresentare accuratamente i fatti, i concetti legali e i punti chiave nel documento.
</section>
<section title="Precisione legale">
La terminologia e i riferimenti a statuti, giurisprudenza o normative devono essere corretti e allineati con gli standard legali.
</section>
<section title="Concisione">
Il riassunto dovrebbe condensare il documento legale ai suoi punti essenziali senza perdere dettagli importanti.
</section>
<section title="Coerenza">
Se si riassumono più documenti, l'LLM dovrebbe mantenere una struttura e un approccio coerenti a ogni riassunto.
</section>
<section title="Leggibilità">
Il testo dovrebbe essere chiaro e facile da comprendere. Se il pubblico non è composto da esperti legali, il riassunto non dovrebbe includere gergo legale che potrebbe confondere il pubblico.
</section>
<section title="Bias e equità">
Il riassunto dovrebbe presentare una rappresentazione imparziale ed equa degli argomenti e delle posizioni legali.
</section>

Consulta la nostra guida su [come stabilire i criteri di successo](/docs/it/test-and-evaluate/define-success) per ulteriori informazioni.

---

## Come riassumere documenti legali utilizzando Claude

### Seleziona il modello Claude giusto

L'accuratezza del modello è estremamente importante quando si riassumono documenti legali. Claude Sonnet 4.5 è un'eccellente scelta per casi d'uso come questo dove è richiesta un'elevata accuratezza. Se la dimensione e la quantità dei tuoi documenti è grande al punto che i costi iniziano a diventare una preoccupazione, puoi anche provare a utilizzare un modello più piccolo come Claude Haiku 4.5.

Per aiutare a stimare questi costi, di seguito è riportato un confronto del costo per riassumere 1.000 accordi di subaffitto utilizzando sia Sonnet che Haiku:

* **Dimensione del contenuto**
    * Numero di accordi: 1.000
    * Caratteri per accordo: 300.000
    * Caratteri totali: 300M

* **Token stimati**
    * Token di input: 86M (assumendo 1 token ogni 3,5 caratteri)
    * Token di output per riassunto: 350
    * Token di output totali: 350.000
 
* **Costo stimato di Claude Sonnet 4.5**
    * Costo dei token di input: 86 MTok * \$3,00/MTok = \$258
    * Costo dei token di output: 0,35 MTok * \$15,00/MTok = \$5,25
    * Costo totale: \$258,00 + \$5,25 = \$263,25

* **Costo stimato di Claude Haiku 3**
    * Costo dei token di input: 86 MTok * \$0,25/MTok = \$21,50
    * Costo dei token di output: 0,35 MTok * \$1,25/MTok = \$0,44
    * Costo totale: \$21,50 + \$0,44 = \$21,96

<Tip>I costi effettivi possono differire da queste stime. Queste stime si basano sull'esempio evidenziato nella sezione su [prompting](#build-a-strong-prompt).</Tip>

### Trasforma i documenti in un formato che Claude può elaborare

Prima di iniziare a riassumere i documenti, devi preparare i tuoi dati. Questo comporta l'estrazione del testo dai PDF, la pulizia del testo e l'assicurazione che sia pronto per essere elaborato da Claude.

Ecco una dimostrazione di questo processo su un PDF di esempio:

```python
from io import BytesIO
import re

import pypdf
import requests

def get_llm_text(pdf_file):
    reader = pypdf.PdfReader(pdf_file)
    text = "\n".join([page.extract_text() for page in reader.pages])

    # Remove extra whitespace
    text = re.sub(r'\s+', ' ', text) 

    # Remove page numbers
    text = re.sub(r'\n\s*\d+\s*\n', '\n', text) 

    return text


# Create the full URL from the GitHub repository
url = "https://raw.githubusercontent.com/anthropics/anthropic-cookbook/main/skills/summarization/data/Sample Sublease Agreement.pdf"
url = url.replace(" ", "%20")

# Download the PDF file into memory
response = requests.get(url)

# Load the PDF from memory
pdf_file = BytesIO(response.content)

document_text = get_llm_text(pdf_file) 
print(document_text[:50000]) 
```

In questo esempio, scarichiamo prima un PDF di un accordo di subaffitto di esempio utilizzato nel [manuale sulla sintesi](https://github.com/anthropics/anthropic-cookbook/blob/main/skills/summarization/data/Sample%20Sublease%20Agreement.pdf). Questo accordo è stato ottenuto da un accordo di subaffitto disponibile pubblicamente dal [sito web sec.gov](https://www.sec.gov/Archives/edgar/data/1045425/000119312507044370/dex1032.htm).

Utilizziamo la libreria pypdf per estrarre il contenuto del PDF e convertirlo in testo. I dati di testo vengono quindi puliti rimuovendo gli spazi bianchi extra e i numeri di pagina.

### Costruisci un prompt forte

Claude può adattarsi a vari stili di sintesi. Puoi modificare i dettagli del prompt per guidare Claude a essere più o meno verboso, includere più o meno terminologia tecnica, o fornire un riassunto di livello superiore o inferiore del contesto in questione.

Ecco un esempio di come creare un prompt che garantisce che i riassunti generati seguano una struttura coerente quando si analizzano accordi di subaffitto:

```python
import anthropic

# Initialize the Anthropic client
client = anthropic.Anthropic()

def summarize_document(text, details_to_extract, model="claude-sonnet-4-5", max_tokens=1000):

    # Format the details to extract to be placed within the prompt's context
    details_to_extract_str = '\n'.join(details_to_extract)
    
    # Prompt the model to summarize the sublease agreement
    prompt = f"""Summarize the following sublease agreement. Focus on these key aspects:

    {details_to_extract_str}

    Provide the summary in bullet points nested within the XML header for each section. For example:

    <parties involved>
    - Sublessor: [Name]
    // Add more details as needed
    </parties involved>
    
    If any information is not explicitly stated in the document, note it as "Not specified". Do not preamble.

    Sublease agreement text:
    {text}
    """

    response = client.messages.create(
        model=model,
        max_tokens=max_tokens,
        system="You are a legal analyst specializing in real estate law, known for highly accurate and detailed summaries of sublease agreements.",
        messages=[
            {"role": "user", "content": prompt},
            {"role": "assistant", "content": "Here is the summary of the sublease agreement: <summary>"}
        ],
        stop_sequences=["</summary>"]
    )

    return response.content[0].text

sublease_summary = summarize_document(document_text, details_to_extract)
print(sublease_summary)
```

Questo codice implementa una funzione `summarize_document` che utilizza Claude per riassumere il contenuto di un accordo di subaffitto. La funzione accetta una stringa di testo e un elenco di dettagli da estrarre come input. In questo esempio, chiamiamo la funzione con le variabili `document_text` e `details_to_extract` che sono state definite negli snippet di codice precedenti.

All'interno della funzione, viene generato un prompt per Claude, includendo il documento da riassumere, i dettagli da estrarre e istruzioni specifiche per riassumere il documento. Il prompt istruisce Claude a rispondere con un riassunto di ogni dettaglio da estrarre annidato all'interno di intestazioni XML.

Poiché abbiamo deciso di emettere ogni sezione del riassunto all'interno di tag, ogni sezione può essere facilmente analizzata come fase di post-elaborazione. Questo approccio consente riassunti strutturati che possono essere adattati al tuo caso d'uso, in modo che ogni riassunto segua lo stesso modello.

### Valuta il tuo prompt

Il prompting spesso richiede test e ottimizzazione per essere pronto per la produzione. Per determinare la disponibilità della tua soluzione, valuta la qualità dei tuoi riassunti utilizzando un processo sistematico che combina metodi quantitativi e qualitativi. La creazione di una [valutazione empirica forte](/docs/it/test-and-evaluate/develop-tests#building-evals-and-test-cases) basata sui tuoi criteri di successo definiti ti permetterà di ottimizzare i tuoi prompt. Ecco alcune metriche che potresti desiderare di includere all'interno della tua valutazione empirica:

<section title="Punteggi ROUGE">
Questo misura la sovrapposizione tra il riassunto generato e un riassunto di riferimento creato da un esperto. Questa metrica si concentra principalmente sul recall ed è utile per valutare la copertura dei contenuti.
</section>
<section title="Punteggi BLEU">
Sebbene originariamente sviluppato per la traduzione automatica, questa metrica può essere adattata per compiti di sintesi. I punteggi BLEU misurano la precisione delle corrispondenze n-gram tra il riassunto generato e i riassunti di riferimento. Un punteggio più alto indica che il riassunto generato contiene frasi e terminologia simili al riassunto di riferimento.
</section>
<section title="Somiglianza dell'incorporamento contestuale">
Questa metrica comporta la creazione di rappresentazioni vettoriali (incorporamenti) sia del riassunto generato che di quello di riferimento. La somiglianza tra questi incorporamenti viene quindi calcolata, spesso utilizzando la somiglianza del coseno. Punteggi di somiglianza più elevati indicano che il riassunto generato cattura il significato semantico e il contesto del riassunto di riferimento, anche se la formulazione esatta differisce.
</section>
<section title="Valutazione basata su LLM">
Questo metodo comporta l'utilizzo di un LLM come Claude per valutare la qualità dei riassunti generati rispetto a una rubrica di valutazione. La rubrica può essere personalizzata in base alle tue esigenze specifiche, valutando fattori chiave come accuratezza, completezza e coerenza. Per indicazioni sull'implementazione della valutazione basata su LLM, consulta questi [suggerimenti](/docs/it/test-and-evaluate/develop-tests#tips-for-llm-based-grading).
</section>
<section title="Valutazione umana">
Oltre a creare i riassunti di riferimento, gli esperti legali possono anche valutare la qualità dei riassunti generati. Sebbene questo sia costoso e dispendioso in termini di tempo su larga scala, questo viene spesso fatto su alcuni riassunti come controllo di sanità mentale prima della distribuzione in produzione.
</section>

### Distribuisci il tuo prompt

Ecco alcune considerazioni aggiuntive da tenere a mente mentre distribuisci la tua soluzione in produzione.

1. **Assicura l'assenza di responsabilità:** Comprendi le implicazioni legali degli errori nei riassunti, che potrebbero portare a responsabilità legale per la tua organizzazione o i tuoi clienti. Fornisci disclaimer o avvisi legali che chiariscono che i riassunti sono generati da AI e dovrebbero essere rivisti da professionisti legali.

2. **Gestisci diversi tipi di documenti:** In questa guida, abbiamo discusso come estrarre il testo dai PDF. Nel mondo reale, i documenti possono arrivare in una varietà di formati (PDF, documenti Word, file di testo, ecc.). Assicurati che la tua pipeline di estrazione dei dati possa convertire tutti i formati di file che ti aspetti di ricevere.

3. **Parallelizza le chiamate API a Claude:** I documenti lunghi con un gran numero di token possono richiedere fino a un minuto per Claude per generare un riassunto. Per grandi raccolte di documenti, potresti desiderare di inviare chiamate API a Claude in parallelo in modo che i riassunti possano essere completati in un lasso di tempo ragionevole. Fai riferimento ai [limiti di velocità](/docs/it/api/rate-limits#rate-limits) di Anthropic per determinare il numero massimo di chiamate API che possono essere eseguite in parallelo.

---

## Migliora le prestazioni

In scenari complessi, potrebbe essere utile considerare strategie aggiuntive per migliorare le prestazioni oltre alle [tecniche standard di prompt engineering](/docs/it/build-with-claude/prompt-engineering/overview). Ecco alcune strategie avanzate:

### Esegui meta-sintesi per riassumere documenti lunghi

La sintesi legale spesso comporta la gestione di documenti lunghi o di molti documenti correlati contemporaneamente, in modo tale da superare la finestra di contesto di Claude. Puoi utilizzare un metodo di chunking noto come meta-sintesi per gestire questo caso d'uso. Questa tecnica comporta la suddivisione dei documenti in chunk più piccoli e gestibili e quindi l'elaborazione di ogni chunk separatamente. Puoi quindi combinare i riassunti di ogni chunk per creare una meta-sintesi dell'intero documento.

Ecco un esempio di come eseguire la meta-sintesi:

```python
import anthropic

# Initialize the Anthropic client
client = anthropic.Anthropic()

def chunk_text(text, chunk_size=20000):
    return [text[i:i+chunk_size] for i in range(0, len(text), chunk_size)]

def summarize_long_document(text, details_to_extract, model="claude-sonnet-4-5", max_tokens=1000):

    # Format the details to extract to be placed within the prompt's context
    details_to_extract_str = '\n'.join(details_to_extract)

    # Iterate over chunks and summarize each one
    chunk_summaries = [summarize_document(chunk, details_to_extract, model=model, max_tokens=max_tokens) for chunk in chunk_text(text)]
    
    final_summary_prompt = f"""
    
    You are looking at the chunked summaries of multiple documents that are all related. 
    Combine the following summaries of the document from different truthful sources into a coherent overall summary:

    <chunked_summaries>
    {"".join(chunk_summaries)}
    </chunked_summaries>

    Focus on these key aspects:
    {details_to_extract_str})

    Provide the summary in bullet points nested within the XML header for each section. For example:

    <parties involved>
    - Sublessor: [Name]
    // Add more details as needed
    </parties involved>
    
    If any information is not explicitly stated in the document, note it as "Not specified". Do not preamble.
    """

    response = client.messages.create(
        model=model,
        max_tokens=max_tokens,
        system="You are a legal expert that summarizes notes on one document.",
        messages=[
            {"role": "user",  "content": final_summary_prompt},
            {"role": "assistant", "content": "Here is the summary of the sublease agreement: <summary>"}

        ],
        stop_sequences=["</summary>"]
    )
    
    return response.content[0].text

long_summary = summarize_long_document(document_text, details_to_extract)
print(long_summary)
```

La funzione `summarize_long_document` si basa sulla precedente funzione `summarize_document` dividendo il documento in chunk più piccoli e riassumendo ogni chunk individualmente.

Il codice realizza questo applicando la funzione `summarize_document` a ogni chunk di 20.000 caratteri all'interno del documento originale. I riassunti individuali vengono quindi combinati e viene creato un riassunto finale da questi riassunti di chunk.

Nota che la funzione `summarize_long_document` non è strettamente necessaria per il nostro PDF di esempio, poiché l'intero documento rientra nella finestra di contesto di Claude. Tuttavia, diventa essenziale per documenti che superano la finestra di contesto di Claude o quando si riassumono più documenti correlati insieme. Indipendentemente da ciò, questa tecnica di meta-sintesi spesso cattura dettagli importanti aggiuntivi nel riassunto finale che sono stati persi nell'approccio di sintesi singola precedente.

### Utilizza documenti indicizzati per sintesi per esplorare una grande raccolta di documenti

La ricerca di una raccolta di documenti con un LLM di solito comporta la generazione aumentata da recupero (RAG). Tuttavia, in scenari che coinvolgono documenti lunghi o quando il recupero di informazioni precise è cruciale, un approccio RAG di base potrebbe essere insufficiente. I documenti indicizzati per sintesi è un approccio RAG avanzato che fornisce un modo più efficiente di classificare i documenti per il recupero, utilizzando meno contesto rispetto ai metodi RAG tradizionali. In questo approccio, utilizzi prima Claude per generare un riassunto conciso per ogni documento nel tuo corpus, e poi utilizzi Claude per classificare la rilevanza di ogni riassunto rispetto alla query posta. Per ulteriori dettagli su questo approccio, incluso un esempio basato su codice, consulta la sezione dei documenti indicizzati per sintesi nel [manuale sulla sintesi](https://github.com/anthropics/anthropic-cookbook/blob/main/skills/summarization/guide.ipynb).

### Ottimizza Claude per imparare dal tuo dataset

Un'altra tecnica avanzata per migliorare la capacità di Claude di generare riassunti è l'ottimizzazione. L'ottimizzazione comporta l'addestramento di Claude su un dataset personalizzato che si allinea specificamente alle tue esigenze di sintesi legale, assicurando che Claude si adatti al tuo caso d'uso. Ecco una panoramica di come eseguire l'ottimizzazione:

1. **Identifica gli errori:** Inizia raccogliendo istanze in cui i riassunti di Claude sono carenti - questo potrebbe includere dettagli legali critici mancanti, malintesi del contesto o utilizzo di terminologia legale inappropriata.

2. **Cura un dataset:** Una volta identificati questi problemi, compila un dataset di questi esempi problematici. Questo dataset dovrebbe includere i documenti legali originali insieme ai tuoi riassunti corretti, assicurando che Claude impari il comportamento desiderato.

3. **Esegui l'ottimizzazione:** L'ottimizzazione comporta l'addestramento del modello sul tuo dataset curato per regolare i suoi pesi e parametri. Questo addestramento aiuta Claude a comprendere meglio i requisiti specifici del tuo dominio legale, migliorando la sua capacità di riassumere i documenti secondo i tuoi standard.

4. **Miglioramento iterativo:** L'ottimizzazione non è un processo una tantum. Man mano che Claude continua a generare riassunti, puoi aggiungere iterativamente nuovi esempi in cui ha avuto prestazioni inferiori, affinando ulteriormente le sue capacità. Nel tempo, questo ciclo di feedback continuo risulterà in un modello altamente specializzato per i tuoi compiti di sintesi legale.

<Tip>L'ottimizzazione è attualmente disponibile solo tramite Amazon Bedrock. Ulteriori dettagli sono disponibili nel [blog di lancio AWS](https://aws.amazon.com/blogs/machine-learning/fine-tune-anthropics-claude-3-haiku-in-amazon-bedrock-to-boost-model-accuracy-and-quality/).</Tip>

<CardGroup cols={2}> 
  <Card title="Manuale sulla sintesi" icon="link" href="https://github.com/anthropics/anthropic-cookbook/blob/main/skills/summarization/guide.ipynb">
    Visualizza un esempio di codice completamente implementato di come utilizzare Claude per riassumere i contratti.
  </Card>
  <Card title="Manuale sulle citazioni" icon="link" href="https://github.com/anthropics/anthropic-cookbook/blob/main/misc/using_citations.ipynb">
    Esplora il nostro manuale sulle citazioni per indicazioni su come garantire accuratezza e spiegabilità delle informazioni.
  </Card>
</CardGroup>