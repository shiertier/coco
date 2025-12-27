# Agente di supporto clienti

Questa guida illustra come sfruttare le avanzate capacità conversazionali di Claude per gestire le richieste dei clienti in tempo reale, fornendo supporto 24/7, riducendo i tempi di attesa e gestendo volumi di supporto elevati con risposte accurate e interazioni positive.

---

## Prima di costruire con Claude

### Decidi se utilizzare Claude per il supporto chat

Ecco alcuni indicatori chiave che suggeriscono di utilizzare un LLM come Claude per automatizzare parti del tuo processo di supporto clienti:

  <section title="Volume elevato di query ripetitive">

    Claude eccelle nel gestire un gran numero di domande simili in modo efficiente, liberando gli agenti umani per problemi più complessi.
  
</section>
  <section title="Necessità di sintesi rapida delle informazioni">

    Claude può recuperare, elaborare e combinare rapidamente informazioni da vaste basi di conoscenza, mentre gli agenti umani potrebbero aver bisogno di tempo per ricercare o consultare più fonti.
  
</section>
  <section title="Requisito di disponibilità 24/7">

    Claude può fornire supporto ininterrotto senza affaticamento, mentre il personale di agenti umani per una copertura continua può essere costoso e impegnativo.
  
</section>
  <section title="Scalabilità rapida durante i periodi di picco">

    Claude può gestire improvvisi aumenti nel volume di query senza la necessità di assumere e formare personale aggiuntivo.
  
</section>
  <section title="Voce del marchio coerente">

    Puoi istruire Claude a rappresentare coerentemente il tono e i valori del tuo marchio, mentre gli agenti umani potrebbero variare negli stili di comunicazione.
  
</section>

Alcune considerazioni per scegliere Claude rispetto ad altri LLM:

- Dai priorità a conversazioni naturali e sfumate: la sofisticata comprensione del linguaggio di Claude consente conversazioni più naturali e consapevoli del contesto che si sentono più umane rispetto alle chat con altri LLM.
- Ricevi spesso query complesse e aperte: Claude può gestire un'ampia gamma di argomenti e richieste senza generare risposte preconfezionate o richiedere una programmazione estensiva di permutazioni di enunciati utente.
- Hai bisogno di supporto multilingue scalabile: le capacità multilingue di Claude gli permettono di impegnarsi in conversazioni in oltre 200 lingue senza la necessità di chatbot separati o processi di traduzione estensivi per ogni lingua supportata.

### Definisci la tua interazione chat ideale

Delinea un'interazione cliente ideale per definire come e quando ti aspetti che il cliente interagisca con Claude. Questo schema ti aiuterà a determinare i requisiti tecnici della tua soluzione.

Ecco un esempio di interazione chat per il supporto clienti di assicurazioni auto:

* **Cliente**: Avvia l'esperienza di chat di supporto
   * **Claude**: Saluta calorosamente il cliente e avvia la conversazione
* **Cliente**: Chiede informazioni sull'assicurazione per la sua nuova auto elettrica
   * **Claude**: Fornisce informazioni rilevanti sulla copertura dei veicoli elettrici
* **Cliente**: Pone domande relative a esigenze uniche per le assicurazioni dei veicoli elettrici
   * **Claude**: Risponde con risposte accurate e informative e fornisce link alle fonti
* **Cliente**: Pone domande fuori tema non correlate all'assicurazione o alle auto
   * **Claude**: Chiarisce che non discute argomenti non correlati e riporta l'utente all'assicurazione auto
* **Cliente**: Esprime interesse per un preventivo assicurativo
   * **Claude**: Pone una serie di domande per determinare il preventivo appropriato, adattandosi alle loro risposte
   * **Claude**: Invia una richiesta per utilizzare lo strumento API di generazione preventivi insieme alle informazioni necessarie raccolte dall'utente
   * **Claude**: Riceve le informazioni di risposta dallo strumento API, sintetizza le informazioni in una risposta naturale e presenta il preventivo fornito all'utente
* **Cliente**: Pone domande di follow-up
   * **Claude**: Risponde alle domande di follow-up secondo necessità
   * **Claude**: Guida il cliente ai prossimi passi nel processo assicurativo e chiude la conversazione

<Tip>Nell'esempio reale che scrivi per il tuo caso d'uso, potresti trovare utile scrivere le parole effettive in questa interazione in modo da avere anche un senso del tono ideale, della lunghezza della risposta e del livello di dettaglio che desideri che Claude abbia.</Tip>

### Suddividi l'interazione in compiti unici

Il chat di supporto clienti è una raccolta di più compiti diversi, dalla risposta alle domande al recupero di informazioni all'azione su richieste, racchiusi in una singola interazione cliente. Prima di iniziare a costruire, suddividi la tua interazione cliente ideale in ogni compito che desideri che Claude sia in grado di eseguire. Ciò garantisce che tu possa richiedere e valutare Claude per ogni compito e ti dà un buon senso della gamma di interazioni che devi considerare quando scrivi i casi di test.

<Tip>I clienti a volte trovano utile visualizzarlo come un diagramma di flusso di interazione dei possibili punti di inflessione della conversazione a seconda delle richieste dell'utente.</Tip>

Ecco i compiti chiave associati all'esempio di interazione assicurativa sopra:

1. Saluto e guida generale
   - Saluta calorosamente il cliente e avvia la conversazione
   - Fornisci informazioni generali sull'azienda e sull'interazione

2. Informazioni sul prodotto
   - Fornisci informazioni sulla copertura dei veicoli elettrici
   <Note>Ciò richiederà che Claude abbia le informazioni necessarie nel suo contesto e potrebbe implicare che sia necessaria un'[integrazione RAG](https://github.com/anthropics/anthropic-cookbook/blob/main/skills/retrieval_augmented_generation/guide.ipynb).</Note>
   - Rispondi a domande relative a esigenze assicurative uniche per veicoli elettrici
   - Rispondi a domande di follow-up sul preventivo o sui dettagli dell'assicurazione
   - Offri link alle fonti quando appropriato

3. Gestione della conversazione
   - Rimani in tema (assicurazione auto)
   - Reindirizza le domande fuori tema ai soggetti rilevanti

4. Generazione di preventivi
   - Poni domande appropriate per determinare l'idoneità del preventivo
   - Adatta le domande in base alle risposte del cliente
   - Invia le informazioni raccolte all'API di generazione preventivi
   - Presenta il preventivo fornito al cliente

### Stabilisci criteri di successo

Lavora con il tuo team di supporto per [definire chiari criteri di successo](/docs/it/test-and-evaluate/define-success) e scrivi [valutazioni dettagliate](/docs/it/test-and-evaluate/develop-tests) con benchmark e obiettivi misurabili.

Ecco criteri e benchmark che possono essere utilizzati per valutare quanto bene Claude esegue i compiti definiti:

  <section title="Precisione della comprensione delle query">

    Questa metrica valuta quanto accuratamente Claude comprende le richieste dei clienti su vari argomenti. Misuralo esaminando un campione di conversazioni e valutando se Claude ha la corretta interpretazione dell'intento del cliente, dei prossimi passi critici, di come appare una risoluzione di successo e altro ancora. Punta a una precisione di comprensione del 95% o superiore.
  
</section>
  <section title="Rilevanza della risposta">

    Questo valuta quanto bene la risposta di Claude affronta la domanda o il problema specifico del cliente. Valuta un insieme di conversazioni e valuta la rilevanza di ogni risposta (utilizzando la valutazione basata su LLM per la scala). Punta a un punteggio di rilevanza del 90% o superiore.
  
</section>
  <section title="Precisione della risposta">

    Valuta la correttezza delle informazioni generali sull'azienda e sul prodotto fornite all'utente, in base alle informazioni fornite a Claude nel contesto. Punta al 100% di precisione in queste informazioni introduttive.
  
</section>
  <section title="Rilevanza della fornitura di citazioni">

    Traccia la frequenza e la rilevanza dei link o delle fonti offerte. Punta a fornire fonti rilevanti nel 80% delle interazioni in cui informazioni aggiuntive potrebbero essere utili.
  
</section>
  <section title="Aderenza al tema">

    Misura quanto bene Claude rimane in tema, come il tema dell'assicurazione auto nel nostro esempio di implementazione. Punta al 95% delle risposte direttamente correlate all'assicurazione auto o alla query specifica del cliente.
  
</section>
  <section title="Efficacia della generazione di contenuti">

    Misura quanto bene Claude riesce a determinare quando generare contenuti informativi e quanto è rilevante quel contenuto. Ad esempio, nella nostra implementazione, staremmo determinando quanto bene Claude comprende quando generare un preventivo e quanto è accurato quel preventivo. Punta al 100% di precisione, poiché si tratta di informazioni vitali per un'interazione cliente di successo.
  
</section>
  <section title="Efficienza dell'escalation">

    Questo misura la capacità di Claude di riconoscere quando una query necessita di intervento umano e di escalare in modo appropriato. Traccia la percentuale di conversazioni correttamente escalate rispetto a quelle che avrebbero dovuto essere escalate ma non lo sono state. Punta a una precisione di escalation del 95% o superiore.
  
</section>

Ecco criteri e benchmark che possono essere utilizzati per valutare l'impatto aziendale dell'utilizzo di Claude per il supporto:

  <section title="Mantenimento del sentimento">

    Questo valuta la capacità di Claude di mantenere o migliorare il sentimento del cliente durante la conversazione. Utilizza strumenti di analisi del sentimento per misurare il sentimento all'inizio e alla fine di ogni conversazione. Punta al mantenimento o al miglioramento del sentimento nel 90% delle interazioni.
  
</section>
  <section title="Tasso di deflazione">

    La percentuale di richieste di clienti gestite con successo dal chatbot senza intervento umano. In genere punta a un tasso di deflazione del 70-80%, a seconda della complessità delle richieste.
  
</section>
  <section title="Punteggio di soddisfazione del cliente">

    Una misura di quanto i clienti sono soddisfatti della loro interazione con il chatbot. Di solito fatto attraverso sondaggi post-interazione. Punta a un punteggio CSAT di 4 su 5 o superiore.
  
</section>
  <section title="Tempo medio di gestione">

    Il tempo medio necessario al chatbot per risolvere una richiesta. Questo varia ampiamente in base alla complessità dei problemi, ma in generale, punta a un AHT inferiore rispetto agli agenti umani.
  
</section>

## Come implementare Claude come agente di servizio clienti

### Scegli il modello Claude giusto

La scelta del modello dipende dai compromessi tra costo, precisione e tempo di risposta.

Per il chat di supporto clienti, Claude Sonnet 4.5 è ben adatto per bilanciare intelligenza, latenza e costo. Tuttavia, per i casi in cui hai un flusso di conversazione con più prompt inclusi RAG, utilizzo di strumenti e/o prompt di contesto lungo, Claude Haiku 4.5 potrebbe essere più adatto per ottimizzare la latenza.

### Costruisci un prompt forte

L'utilizzo di Claude per il supporto clienti richiede che Claude abbia una direzione e un contesto sufficienti per rispondere in modo appropriato, pur avendo abbastanza flessibilità per gestire un'ampia gamma di richieste di clienti.

Iniziamo scrivendo gli elementi di un prompt forte, iniziando con un prompt di sistema:

```python
IDENTITY = """You are Eva, a friendly and knowledgeable AI assistant for Acme Insurance 
Company. Your role is to warmly welcome customers and provide information on 
Acme's insurance offerings, which include car insurance and electric car 
insurance. You can also help customers get quotes for their insurance needs."""
```

<Tip>Anche se potresti essere tentato di mettere tutte le tue informazioni all'interno di un prompt di sistema come un modo per separare le istruzioni dalla conversazione dell'utente, Claude in realtà funziona meglio con la maggior parte del contenuto del prompt scritto all'interno del primo turno `User` (con l'unica eccezione del prompt di ruolo). Leggi di più su [Dare a Claude un ruolo con un prompt di sistema](/docs/it/build-with-claude/prompt-engineering/system-prompts).</Tip>

È meglio suddividere i prompt complessi in sottosezioni e scrivere una parte alla volta. Per ogni compito, potresti trovare maggior successo seguendo un processo passo dopo passo per definire le parti del prompt di cui Claude avrebbe bisogno per fare bene il compito. Per questo esempio di supporto clienti di assicurazione auto, scriveremo a pezzi tutte le parti di un prompt iniziando con il compito "Saluto e guida generale". Questo rende anche il debug del tuo prompt più facile poiché puoi regolare più rapidamente le singole parti del prompt complessivo.

Metteremo tutti questi pezzi in un file chiamato `config.py`.

```python
STATIC_GREETINGS_AND_GENERAL = """
<static_context>
Acme Auto Insurance: Your Trusted Companion on the Road

About:
At Acme Insurance, we understand that your vehicle is more than just a mode of transportation—it's your ticket to life's adventures. 
Since 1985, we've been crafting auto insurance policies that give drivers the confidence to explore, commute, and travel with peace of mind.
Whether you're navigating city streets or embarking on cross-country road trips, Acme is there to protect you and your vehicle. 
Our innovative auto insurance policies are designed to adapt to your unique needs, covering everything from fender benders to major collisions.
With Acme's award-winning customer service and swift claim resolution, you can focus on the joy of driving while we handle the rest. 
We're not just an insurance provider—we're your co-pilot in life's journeys.
Choose Acme Auto Insurance and experience the assurance that comes with superior coverage and genuine care. Because at Acme, we don't just 
insure your car—we fuel your adventures on the open road.

Note: We also offer specialized coverage for electric vehicles, ensuring that drivers of all car types can benefit from our protection.

Acme Insurance offers the following products:
- Car insurance
- Electric car insurance
- Two-wheeler insurance

Business hours: Monday-Friday, 9 AM - 5 PM EST
Customer service number: 1-800-123-4567
</static_context>
"""
```

Faremo lo stesso per le nostre informazioni su assicurazione auto e assicurazione auto elettrica.

```python
STATIC_CAR_INSURANCE="""
<static_context>
Car Insurance Coverage:
Acme's car insurance policies typically cover:
1. Liability coverage: Pays for bodily injury and property damage you cause to others.
2. Collision coverage: Pays for damage to your car in an accident.
3. Comprehensive coverage: Pays for damage to your car from non-collision incidents.
4. Medical payments coverage: Pays for medical expenses after an accident.
5. Uninsured/underinsured motorist coverage: Protects you if you're hit by a driver with insufficient insurance.

Optional coverages include:
- Rental car reimbursement
- Roadside assistance
- New car replacement
</static_context>
"""

STATIC_ELECTRIC_CAR_INSURANCE="""
<static_context>
Electric Car Insurance:
Our specialized electric car insurance goes beyond traditional auto coverage, 
offering tailored protection for your high-tech investment. In addition to standard coverages, we offer:
- Safeguarding expensive battery systems
- Covering home charging equipment
- Adapting to the rapidly evolving world of electric vehicles

Whether you're driving a Rivian, Nissan Leaf, Chevrolet Bolt, 
or any other electric vehicle, our knowledgeable agents are ready to craft a policy 
that gives you the confidence to enjoy every electron-powered mile.
</static_context>
"""
```

Ora che abbiamo il nostro contenuto statico, aggiungiamo almeno 4-5 interazioni di esempio "buone" per guidare le risposte di Claude. Questi esempi dovrebbero essere rappresentativi della tua interazione cliente ideale e possono includere guardrail, chiamate di strumenti, ecc.

```python
EXAMPLES="""
Here are a few examples of how you can interact with customers:

<example 1>
H: Hi, do you offer commercial insurance for small businesses?

A: Ah! Unfortunately, we don't offer commercial insurance at this time. 
However, we do provide a range of personal insurance products including car insurance and electric car insurance. 
Would you like to know more about these offerings?
</example 1>

<example 2>
H: Hi, what are your business hours?

A: Our business hours are Monday to Friday, 9 AM to 5 PM EST. 
If you have any questions or need assistance, feel free to reach out during those times. 
How else can I assist you today?
</example 2>

<example 3>
H: I'm shopping for car insurance. What does Acme offer?

A: Great choice! Acme's car insurance policies typically include:
- Liability coverage: Protects you if you're at fault in an accident
- Collision coverage: Pays for damage to your car in an accident
- Comprehensive coverage: Covers non-collision incidents like theft or vandalism
- Medical payments coverage: Pays for medical expenses after an accident
- Uninsured/underinsured motorist coverage: Protects you from drivers with insufficient insurance

We also offer optional coverages like rental car reimbursement, roadside assistance, and new car replacement. 
Would you like more details on any of these coverages, or do you have specific questions about insuring your vehicle?
</example 3>

<example 4>
H: I'd like to hear about your electric car insurance.

A: Certainly! Our electric car insurance is specially designed to meet the unique needs of electric vehicle owners. 
In addition to all the standard coverages we offer for regular cars, our electric car policies include:

- Battery coverage: Protection against damage or degradation of your vehicle's expensive battery system
- Tech package: Covers repairs or replacement of advanced electronics and onboard computer systems
- Charging station coverage: Protects both home and portable charging equipment
- Cybersecurity protection: Coverage against hacking or digital threats
- Specialized roadside assistance: Includes flatbed towing to protect the vehicle's undercarriage

We cover a wide range of electric vehicles, including popular models from Tesla, Rivian, Nissan, Chevrolet, and more. 
Is there a specific electric vehicle you're looking to insure, or would you like more details on any of these coverages?
</example 4>

<example 5>
H: I'd like to get a quote for my car insurance.

A: Certainly! I'd be happy to help you get a quote for your car insurance. 
To provide you with an accurate quote, I'll need to collect some information about your vehicle and the primary driver. 
Let's start with the basics:

1. What is the make and model of your vehicle?
2. What year was it manufactured?
3. Approximately how many miles have you driven?
4. What is the age of the primary driver?

Once you provide this information, I'll use our quoting tool to generate a personalized insurance quote for you.
</example 5>
"""
```

Vorrai anche includere istruzioni importanti che delineano i Do e i Don't per come Claude dovrebbe interagire con il cliente. 
Questo potrebbe derivare da guardrail del marchio o politiche di supporto.

```python
ADDITIONAL_GUARDRAILS = """Please adhere to the following guardrails:
1. Only provide information about insurance types listed in our offerings.
2. If asked about an insurance type we don't offer, politely state 
that we don't provide that service.
3. Do not speculate about future product offerings or company plans.
4. Don't make promises or enter into agreements it's not authorized to make.
You only provide information and guidance.
5. Do not mention any competitor's products or services.
"""
```

Ora combiniamo tutte queste sezioni in una singola stringa da utilizzare come nostro prompt.

```python
TASK_SPECIFIC_INSTRUCTIONS = ' '.join([
   STATIC_GREETINGS_AND_GENERAL,
   STATIC_CAR_INSURANCE,
   STATIC_ELECTRIC_CAR_INSURANCE,
   EXAMPLES,
   ADDITIONAL_GUARDRAILS,
])
```

### Aggiungi capacità dinamiche e agentive con l'utilizzo di strumenti

Claude è in grado di intraprendere azioni e recuperare informazioni dinamicamente utilizzando la funzionalità di utilizzo di strumenti lato client. Inizia elencando eventuali strumenti esterni o API che il prompt dovrebbe utilizzare.

Per questo esempio, inizieremo con uno strumento per il calcolo del preventivo.

<Tip>Come promemoria, questo strumento non eseguirà il calcolo effettivo, segnala semplicemente all'applicazione che uno strumento dovrebbe essere utilizzato con gli argomenti specificati.</Tip>

Calcolatore di preventivi assicurativi di esempio:

```python
TOOLS = [{
  "name": "get_quote",
  "description": "Calculate the insurance quote based on user input. Returned value is per month premium.",
  "input_schema": {
    "type": "object",
    "properties": {
      "make": {"type": "string", "description": "The make of the vehicle."},
      "model": {"type": "string", "description": "The model of the vehicle."},
      "year": {"type": "integer", "description": "The year the vehicle was manufactured."},
      "mileage": {"type": "integer", "description": "The mileage on the vehicle."},
      "driver_age": {"type": "integer", "description": "The age of the primary driver."}
    },
    "required": ["make", "model", "year", "mileage", "driver_age"]
  }
}]

def get_quote(make, model, year, mileage, driver_age):
    """Returns the premium per month in USD"""
    # You can call an http endpoint or a database to get the quote.
    # Here, we simulate a delay of 1 seconds and return a fixed quote of 100.
    time.sleep(1)
    return 100
```

### Distribuisci i tuoi prompt

È difficile sapere quanto bene funziona il tuo prompt senza distribuirlo in un'impostazione di produzione di test ed [eseguire valutazioni](/docs/it/test-and-evaluate/develop-tests) quindi costruiamo una piccola applicazione utilizzando il nostro prompt, l'SDK Anthropic e streamlit per un'interfaccia utente.

In un file chiamato `chatbot.py`, inizia configurando la classe ChatBot, che incapsulerà le interazioni con l'SDK Anthropic.

La classe dovrebbe avere due metodi principali: `generate_message` e `process_user_input`.

```python
from anthropic import Anthropic
from config import IDENTITY, TOOLS, MODEL, get_quote
from dotenv import load_dotenv

load_dotenv()

class ChatBot:
   def __init__(self, session_state):
       self.anthropic = Anthropic()
       self.session_state = session_state

   def generate_message(
       self,
       messages,
       max_tokens,
   ):
       try:
           response = self.anthropic.messages.create(
               model=MODEL,
               system=IDENTITY,
               max_tokens=max_tokens,
               messages=messages,
               tools=TOOLS,
           )
           return response
       except Exception as e:
           return {"error": str(e)}

   def process_user_input(self, user_input):
       self.session_state.messages.append({"role": "user", "content": user_input})

       response_message = self.generate_message(
           messages=self.session_state.messages,
           max_tokens=2048,
       )

       if "error" in response_message:
           return f"An error occurred: {response_message['error']}"

       if response_message.content[-1].type == "tool_use":
           tool_use = response_message.content[-1]
           func_name = tool_use.name
           func_params = tool_use.input
           tool_use_id = tool_use.id

           result = self.handle_tool_use(func_name, func_params)
           self.session_state.messages.append(
               {"role": "assistant", "content": response_message.content}
           )
           self.session_state.messages.append({
               "role": "user",
               "content": [{
                   "type": "tool_result",
                   "tool_use_id": tool_use_id,
                   "content": f"{result}",
               }],
           })

           follow_up_response = self.generate_message(
               messages=self.session_state.messages,
               max_tokens=2048,
           )

           if "error" in follow_up_response:
               return f"An error occurred: {follow_up_response['error']}"

           response_text = follow_up_response.content[0].text
           self.session_state.messages.append(
               {"role": "assistant", "content": response_text}
           )
           return response_text
      
       elif response_message.content[0].type == "text":
           response_text = response_message.content[0].text
           self.session_state.messages.append(
               {"role": "assistant", "content": response_text}
           )
           return response_text
      
       else:
           raise Exception("An error occurred: Unexpected response type")

   def handle_tool_use(self, func_name, func_params):
       if func_name == "get_quote":
           premium = get_quote(**func_params)
           return f"Quote generated: ${premium:.2f} per month"
      
       raise Exception("An unexpected tool was used")
```

### Costruisci la tua interfaccia utente

Testa la distribuzione di questo codice con Streamlit utilizzando un metodo principale. Questa funzione `main()` configura un'interfaccia chat basata su Streamlit.

Lo faremo in un file chiamato `app.py`

```python
import streamlit as st
from chatbot import ChatBot
from config import TASK_SPECIFIC_INSTRUCTIONS

def main():
   st.title("Chat with Eva, Acme Insurance Company's Assistant🤖")

   if "messages" not in st.session_state:
       st.session_state.messages = [
           {'role': "user", "content": TASK_SPECIFIC_INSTRUCTIONS},
           {'role': "assistant", "content": "Understood"},
       ]

   chatbot = ChatBot(st.session_state)

   # Display user and assistant messages skipping the first two
   for message in st.session_state.messages[2:]:
       # ignore tool use blocks
       if isinstance(message["content"], str):
           with st.chat_message(message["role"]):
               st.markdown(message["content"])

   if user_msg := st.chat_input("Type your message here..."):
       st.chat_message("user").markdown(user_msg)

       with st.chat_message("assistant"):
           with st.spinner("Eva is thinking..."):
               response_placeholder = st.empty()
               full_response = chatbot.process_user_input(user_msg)
               response_placeholder.markdown(full_response)

if __name__ == "__main__":
   main()
```

Esegui il programma con:

```
streamlit run app.py
```

### Valuta i tuoi prompt

Il prompting spesso richiede test e ottimizzazione per essere pronto per la produzione. Per determinare la disponibilità della tua soluzione, valuta le prestazioni del chatbot utilizzando un processo sistematico che combina metodi quantitativi e qualitativi. La creazione di una [forte valutazione empirica](/docs/it/test-and-evaluate/develop-tests#building-evals-and-test-cases) basata sui tuoi criteri di successo definiti ti permetterà di ottimizzare i tuoi prompt.

<Tip>La [Console Claude](/dashboard) ora presenta uno strumento di valutazione che ti consente di testare i tuoi prompt in vari scenari.</Tip>

### Migliora le prestazioni

In scenari complessi, potrebbe essere utile considerare strategie aggiuntive per migliorare le prestazioni oltre le [tecniche standard di prompt engineering](/docs/it/build-with-claude/prompt-engineering/overview) e le [strategie di implementazione dei guardrail](/docs/it/test-and-evaluate/strengthen-guardrails/reduce-hallucinations). Ecco alcuni scenari comuni:

#### Riduci la latenza del contesto lungo con RAG

Quando si ha a che fare con grandi quantità di contesto statico e dinamico, includere tutte le informazioni nel prompt può portare a costi elevati, tempi di risposta più lenti e raggiungere i limiti della finestra di contesto. In questo scenario, l'implementazione di tecniche di Retrieval Augmented Generation (RAG) può migliorare significativamente le prestazioni e l'efficienza.

Utilizzando [modelli di embedding come Voyage](/docs/it/build-with-claude/embeddings) per convertire le informazioni in rappresentazioni vettoriali, puoi creare un sistema più scalabile e reattivo. Questo approccio consente il recupero dinamico di informazioni rilevanti in base alla query corrente, piuttosto che includere tutto il contesto possibile in ogni prompt.

L'implementazione di RAG per i casi d'uso di supporto [ricetta RAG](https://github.com/anthropics/anthropic-cookbook/blob/82675c124e1344639b2a875aa9d3ae854709cd83/skills/classification/guide.ipynb) ha dimostrato di aumentare l'accuratezza, ridurre i tempi di risposta e ridurre i costi dell'API nei sistemi con ampi requisiti di contesto.

#### Integra dati in tempo reale con l'utilizzo di strumenti

Quando si ha a che fare con query che richiedono informazioni in tempo reale, come saldi di account o dettagli di polizza, gli approcci RAG basati su embedding non sono sufficienti. Invece, puoi sfruttare l'utilizzo di strumenti per migliorare significativamente la capacità del tuo chatbot di fornire risposte accurate e in tempo reale. Ad esempio, puoi utilizzare l'utilizzo di strumenti per cercare informazioni sui clienti, recuperare dettagli degli ordini e annullare ordini per conto del cliente.

Questo approccio, [delineato nella nostra ricetta tool use: customer service agent](https://github.com/anthropics/anthropic-cookbook/blob/main/tool_use/customer_service_agent.ipynb), ti consente di integrare perfettamente dati live nelle risposte di Claude e fornire un'esperienza cliente più personalizzata ed efficiente.

#### Rafforza i guardrail di input e output

Quando distribuisci un chatbot, specialmente in scenari di servizio clienti, è fondamentale prevenire i rischi associati all'uso improprio, alle query fuori ambito e alle risposte inappropriate. Sebbene Claude sia intrinsecamente resiliente a tali scenari, ecco ulteriori passaggi per rafforzare i guardrail del tuo chatbot:

- [Riduci le allucinazioni](/docs/it/test-and-evaluate/strengthen-guardrails/reduce-hallucinations): Implementa meccanismi di verifica dei fatti e [citazioni](https://github.com/anthropics/anthropic-cookbook/blob/main/misc/using_citations.ipynb) per ancorare le risposte alle informazioni fornite.
- Verifica incrociata delle informazioni: Verifica che le risposte dell'agente siano allineate con le politiche della tua azienda e i fatti noti.
- Evita impegni contrattuali: Assicurati che l'agente non faccia promesse o entri in accordi che non è autorizzato a fare.
- [Attenua i jailbreak](/docs/it/test-and-evaluate/strengthen-guardrails/mitigate-jailbreaks): Utilizza metodi come schermi di innocuità e convalida dell'input per prevenire agli utenti di sfruttare le vulnerabilità del modello, mirando a generare contenuti inappropriati.
- Evita di menzionare i concorrenti: Implementa un filtro di menzione dei concorrenti per mantenere il focus del marchio e non menzionare i prodotti o servizi di alcun concorrente.
- [Mantieni Claude nel personaggio](/docs/it/test-and-evaluate/strengthen-guardrails/keep-claude-in-character): Previeni che Claude cambi il suo stile di contesto, anche durante interazioni lunghe e complesse.
- Rimuovi le informazioni di identificazione personale (PII): A meno che non sia esplicitamente richiesto e autorizzato, estrai qualsiasi PII dalle risposte.

#### Riduci il tempo di risposta percepito con lo streaming

Quando si ha a che fare con risposte potenzialmente lunghe, l'implementazione dello streaming può migliorare significativamente il coinvolgimento e la soddisfazione dell'utente. In questo scenario, gli utenti ricevono la risposta progressivamente invece di aspettare che l'intera risposta sia generata.

Ecco come implementare lo streaming:
1. Utilizza l'[API di streaming Anthropic](/docs/it/build-with-claude/streaming) per supportare le risposte di streaming.
2. Configura il tuo frontend per gestire i chunk di testo in arrivo.
3. Visualizza ogni chunk mentre arriva, simulando la digitazione in tempo reale.
4. Implementa un meccanismo per salvare la risposta completa, consentendo agli utenti di visualizzarla se si allontanano e ritornano.

In alcuni casi, lo streaming consente l'uso di modelli più avanzati con latenze di base più elevate, poiché la visualizzazione progressiva mitiga l'impatto dei tempi di elaborazione più lunghi.

#### Scala il tuo Chatbot

Man mano che la complessità del tuo Chatbot cresce, l'architettura della tua applicazione può evolversi di conseguenza. Prima di aggiungere ulteriori livelli alla tua architettura, considera le seguenti opzioni meno esaustive:

- Assicurati di sfruttare al massimo i tuoi prompt e di ottimizzare attraverso il prompt engineering. Utilizza le nostre [guide di prompt engineering](/docs/it/build-with-claude/prompt-engineering/overview) per scrivere i prompt più efficaci.
- Aggiungi ulteriori [strumenti](/docs/it/build-with-claude/tool-use) al prompt (che possono includere [catene di prompt](/docs/it/build-with-claude/prompt-engineering/chain-prompts)) e vedi se puoi ottenere la funzionalità richiesta.

Se il tuo Chatbot gestisce compiti incredibilmente vari, potresti voler considerare l'aggiunta di un [classificatore di intenti separato](https://github.com/anthropics/anthropic-cookbook/blob/main/skills/classification/guide.ipynb) per instradare la query iniziale del cliente. Per l'applicazione esistente, ciò comporterebbe la creazione di un albero decisionale che instrada le query dei clienti attraverso il classificatore e quindi a conversazioni specializzate (con il loro proprio set di strumenti e prompt di sistema). Nota, questo metodo richiede una chiamata aggiuntiva a Claude che può aumentare la latenza.

### Integra Claude nel tuo flusso di lavoro di supporto

Mentre i nostri esempi si sono concentrati su funzioni Python richiamabili all'interno di un ambiente Streamlit, la distribuzione di Claude per un chatbot di supporto in tempo reale richiede un servizio API.

Ecco come puoi affrontare questo:

1. Crea un wrapper API: Sviluppa un semplice wrapper API attorno alla tua funzione di classificazione. Ad esempio, puoi utilizzare Flask API o Fast API per avvolgere il tuo codice in un servizio HTTP. Il tuo servizio HTTP potrebbe accettare l'input dell'utente e restituire la risposta dell'Assistente nella sua interezza. Pertanto, il tuo servizio potrebbe avere le seguenti caratteristiche:
   - Server-Sent Events (SSE): SSE consente lo streaming in tempo reale delle risposte dal server al client. Questo è cruciale per fornire un'esperienza fluida e interattiva quando si lavora con gli LLM.
   - Caching: L'implementazione del caching può migliorare significativamente i tempi di risposta e ridurre le chiamate API non necessarie.
   - Conservazione del contesto: Mantenere il contesto quando un utente si allontana e ritorna è importante per la continuità nelle conversazioni.

2. Costruisci un'interfaccia web: Implementa un'interfaccia utente web intuitiva per interagire con l'agente alimentato da Claude.

<CardGroup cols={2}>
  <Card title="Retrieval Augmented Generation (RAG) cookbook" icon="link" href="https://github.com/anthropics/anthropic-cookbook/blob/main/skills/retrieval_augmented_generation/guide.ipynb">
    Visita la nostra ricetta RAG cookbook per ulteriori codici di esempio e indicazioni dettagliate.
  </Card>
  <Card title="Citations cookbook" icon="link" href="https://github.com/anthropics/anthropic-cookbook/blob/main/misc/using_citations.ipynb">
    Esplora la nostra ricetta Citations cookbook per come garantire l'accuratezza e la spiegabilità delle informazioni.
  </Card>
</CardGroup>