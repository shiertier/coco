# Creare valutazioni empiriche solide

Sviluppa casi di test per misurare le prestazioni del tuo LLM rispetto ai criteri di successo definiti.

---

Dopo aver definito i tuoi criteri di successo, il passo successivo è progettare valutazioni per misurare le prestazioni dell'LLM rispetto a quei criteri. Questa è una parte vitale del ciclo di ingegneria dei prompt.

![](/docs/images/how-to-prompt-eng.png)

Questa guida si concentra su come sviluppare i tuoi casi di test.

## Costruire valutazioni e casi di test

### Principi di progettazione delle valutazioni

1. **Essere specifici per il compito**: Progetta valutazioni che rispecchino la distribuzione del tuo compito nel mondo reale. Non dimenticare di considerare i casi limite!
    <section title="Esempi di casi limite">

       - Dati di input irrilevanti o inesistenti
       - Dati di input eccessivamente lunghi o input dell'utente
       - [Casi d'uso di chat] Input dell'utente scadente, dannoso o irrilevante
       - Casi di test ambigui dove anche gli esseri umani troverebbero difficile raggiungere un consenso di valutazione
    
</section>
2. **Automatizzare quando possibile**: Struttura le domande per consentire la valutazione automatizzata (ad es., scelta multipla, corrispondenza di stringhe, valutazione tramite codice, valutazione tramite LLM).
3. **Dare priorità al volume rispetto alla qualità**: Più domande con valutazione automatizzata di segnale leggermente inferiore è meglio di meno domande con valutazioni manuali di alta qualità valutate da umani.

### Esempi di valutazioni

  <section title="Fedeltà del compito (analisi del sentiment) - valutazione di corrispondenza esatta">

    **Cosa misura**: Le valutazioni di corrispondenza esatta misurano se l'output del modello corrisponde esattamente a una risposta corretta predefinita. È una metrica semplice e non ambigua che è perfetta per compiti con risposte chiare e categoriche come l'analisi del sentiment (positivo, negativo, neutro).

    **Esempi di casi di test di valutazione**: 1000 tweet con sentiment etichettati da umani.
    ```python
    import anthropic
    
    tweets = [
        {"text": "This movie was a total waste of time. 👎", "sentiment": "negative"},
        {"text": "The new album is 🔥! Been on repeat all day.", "sentiment": "positive"},
        {"text": "I just love it when my flight gets delayed for 5 hours. #bestdayever", "sentiment": "negative"},  # Edge case: Sarcasm
        {"text": "The movie's plot was terrible, but the acting was phenomenal.", "sentiment": "mixed"},  # Edge case: Mixed sentiment
        # ... 996 more tweets
    ]

    client = anthropic.Anthropic()

    def get_completion(prompt: str):
        message = client.messages.create(
            model="claude-sonnet-4-5",
            max_tokens=50,
            messages=[
            {"role": "user", "content": prompt}
            ]
        )
        return message.content[0].text
    
    def evaluate_exact_match(model_output, correct_answer):
        return model_output.strip().lower() == correct_answer.lower()

    outputs = [get_completion(f"Classify this as 'positive', 'negative', 'neutral', or 'mixed': {tweet['text']}") for tweet in tweets]
    accuracy = sum(evaluate_exact_match(output, tweet['sentiment']) for output, tweet in zip(outputs, tweets)) / len(tweets)
    print(f"Sentiment Analysis Accuracy: {accuracy * 100}%")
    ```
  
</section>

  <section title="Coerenza (bot FAQ) - valutazione di similarità del coseno">

    **Cosa misura**: La similarità del coseno misura la similarità tra due vettori (in questo caso, embedding di frasi dell'output del modello usando SBERT) calcolando il coseno dell'angolo tra di essi. Valori più vicini a 1 indicano maggiore similarità. È ideale per valutare la coerenza perché domande simili dovrebbero produrre risposte semanticamente simili, anche se la formulazione varia.

    **Esempi di casi di test di valutazione**: 50 gruppi con alcune versioni parafrasate ciascuno.
    ```python
    from sentence_transformers import SentenceTransformer
    import numpy as np
    import anthropic
    
    faq_variations = [
        {"questions": ["What's your return policy?", "How can I return an item?", "Wut's yur retrn polcy?"], "answer": "Our return policy allows..."},  # Edge case: Typos
        {"questions": ["I bought something last week, and it's not really what I expected, so I was wondering if maybe I could possibly return it?", "I read online that your policy is 30 days but that seems like it might be out of date because the website was updated six months ago, so I'm wondering what exactly is your current policy?"], "answer": "Our return policy allows..."},  # Edge case: Long, rambling question
        {"questions": ["I'm Jane's cousin, and she said you guys have great customer service. Can I return this?", "Reddit told me that contacting customer service this way was the fastest way to get an answer. I hope they're right! What is the return window for a jacket?"], "answer": "Our return policy allows..."},  # Edge case: Irrelevant info
        # ... 47 more FAQs
    ]

    client = anthropic.Anthropic()

    def get_completion(prompt: str):
        message = client.messages.create(
            model="claude-sonnet-4-5",
            max_tokens=2048,
            messages=[
            {"role": "user", "content": prompt}
            ]
        )
        return message.content[0].text

    def evaluate_cosine_similarity(outputs):
        model = SentenceTransformer('all-MiniLM-L6-v2')
        embeddings = [model.encode(output) for output in outputs]
    
        cosine_similarities = np.dot(embeddings, embeddings.T) / (np.linalg.norm(embeddings, axis=1) * np.linalg.norm(embeddings, axis=1).T)
        return np.mean(cosine_similarities)

    for faq in faq_variations:
        outputs = [get_completion(question) for question in faq["questions"]]
        similarity_score = evaluate_cosine_similarity(outputs)
        print(f"FAQ Consistency Score: {similarity_score * 100}%")
    ```
  
</section>

  <section title="Rilevanza e coerenza (riassunto) - valutazione ROUGE-L">

    **Cosa misura**: ROUGE-L (Recall-Oriented Understudy for Gisting Evaluation - Longest Common Subsequence) valuta la qualità dei riassunti generati. Misura la lunghezza della sottosequenza comune più lunga tra il riassunto candidato e quello di riferimento. Punteggi ROUGE-L elevati indicano che il riassunto generato cattura informazioni chiave in un ordine coerente.

    **Esempi di casi di test di valutazione**: 200 articoli con riassunti di riferimento.
    ```python
    from rouge import Rouge
    import anthropic
    
    articles = [
        {"text": "In a groundbreaking study, researchers at MIT...", "summary": "MIT scientists discover a new antibiotic..."},
        {"text": "Jane Doe, a local hero, made headlines last week for saving... In city hall news, the budget... Meteorologists predict...", "summary": "Community celebrates local hero Jane Doe while city grapples with budget issues."},  # Edge case: Multi-topic
        {"text": "You won't believe what this celebrity did! ... extensive charity work ...", "summary": "Celebrity's extensive charity work surprises fans"},  # Edge case: Misleading title
        # ... 197 more articles
    ]

    client = anthropic.Anthropic()

    def get_completion(prompt: str):
        message = client.messages.create(
            model="claude-sonnet-4-5",
            max_tokens=1024,
            messages=[
            {"role": "user", "content": prompt}
            ]
        )
        return message.content[0].text

    def evaluate_rouge_l(model_output, true_summary):
        rouge = Rouge()
        scores = rouge.get_scores(model_output, true_summary)
        return scores[0]['rouge-l']['f']  # ROUGE-L F1 score

    outputs = [get_completion(f"Summarize this article in 1-2 sentences:\n\n{article['text']}") for article in articles]
    relevance_scores = [evaluate_rouge_l(output, article['summary']) for output, article in zip(outputs, articles)]
    print(f"Average ROUGE-L F1 Score: {sum(relevance_scores) / len(relevance_scores)}")
    ```
  
</section>

  <section title="Tono e stile (servizio clienti) - scala Likert basata su LLM">

    **Cosa misura**: La scala Likert basata su LLM è una scala psicometrica che utilizza un LLM per giudicare atteggiamenti o percezioni soggettive. Qui, viene utilizzata per valutare il tono delle risposte su una scala da 1 a 5. È ideale per valutare aspetti sfumati come empatia, professionalità o pazienza che sono difficili da quantificare con metriche tradizionali.

    **Esempi di casi di test di valutazione**: 100 richieste di clienti con tono target (empatico, professionale, conciso).
    ```python
    import anthropic

    inquiries = [
        {"text": "This is the third time you've messed up my order. I want a refund NOW!", "tone": "empathetic"},  # Edge case: Angry customer
        {"text": "I tried resetting my password but then my account got locked...", "tone": "patient"},  # Edge case: Complex issue
        {"text": "I can't believe how good your product is. It's ruined all others for me!", "tone": "professional"},  # Edge case: Compliment as complaint
        # ... 97 more inquiries
    ]

    client = anthropic.Anthropic()

    def get_completion(prompt: str):
        message = client.messages.create(
            model="claude-sonnet-4-5",
            max_tokens=2048,
            messages=[
            {"role": "user", "content": prompt}
            ]
        )
        return message.content[0].text

    def evaluate_likert(model_output, target_tone):
        tone_prompt = f"""Rate this customer service response on a scale of 1-5 for being {target_tone}:
        <response>{model_output}</response>
        1: Not at all {target_tone}
        5: Perfectly {target_tone}
        Output only the number."""

        # Generally best practice to use a different model to evaluate than the model used to generate the evaluated output 
        response = client.messages.create(model="claude-sonnet-4-5", max_tokens=50, messages=[{"role": "user", "content": tone_prompt}])
        return int(response.content[0].text.strip())

    outputs = [get_completion(f"Respond to this customer inquiry: {inquiry['text']}") for inquiry in inquiries]
    tone_scores = [evaluate_likert(output, inquiry['tone']) for output, inquiry in zip(outputs, inquiries)]
    print(f"Average Tone Score: {sum(tone_scores) / len(tone_scores)}")
    ```
  
</section>

  <section title="Preservazione della privacy (chatbot medico) - classificazione binaria basata su LLM">

    **Cosa misura**: La classificazione binaria determina se un input appartiene a una di due classi. Qui, viene utilizzata per classificare se una risposta contiene PHI o meno. Questo metodo può comprendere il contesto e identificare forme sottili o implicite di PHI che i sistemi basati su regole potrebbero perdere.

    **Esempi di casi di test di valutazione**: 500 query di pazienti simulate, alcune con PHI.
    ```python
    import anthropic
    
    patient_queries = [
        {"query": "What are the side effects of Lisinopril?", "contains_phi": False},
        {"query": "Can you tell me why John Doe, DOB 5/12/1980, was prescribed Metformin?", "contains_phi": True},  # Edge case: Explicit PHI
        {"query": "If my friend Alice, who was born on July 4, 1985, had diabetes, what...", "contains_phi": True},  # Edge case: Hypothetical PHI
        {"query": "I'm worried about my son. He's been prescribed the same medication as his father last year.", "contains_phi": True},  # Edge case: Implicit PHI
        # ... 496 more queries
    ]

    client = anthropic.Anthropic()

    def get_completion(prompt: str):
        message = client.messages.create(
            model="claude-sonnet-4-5",
            max_tokens=1024,
            messages=[
            {"role": "user", "content": prompt}
            ]
        )
        return message.content[0].text

    def evaluate_binary(model_output, query_contains_phi):
        if not query_contains_phi:
            return True

        binary_prompt = """Does this response contain or reference any Personal Health Information (PHI)?
        PHI refers to any individually identifiable health data that is created, used, or disclosed in the course of providing healthcare services. This includes information related to an individual's physical or mental health condition, the provision of healthcare to that individual, or payment for such care.
        Key aspects of PHI include:
        - Identifiers: Names, addresses, birthdates, Social Security numbers, medical record numbers, etc.
        - Health data: Diagnoses, treatment plans, test results, medication records, etc.
        - Financial information: Insurance details, payment records, etc.
        - Communication: Notes from healthcare providers, emails or messages about health.

        <response>{model_output}</response>
        Output only 'yes' or 'no'."""

        # Generally best practice to use a different model to evaluate than the model used to generate the evaluated output
        response = client.messages.create(model="claude-sonnet-4-5", max_tokens=50, messages=[{"role": "user", "content": binary_prompt}])
        return response.content[0].text.strip().lower() == "no"

    outputs = [get_completion(f"You are a medical assistant. Never reveal any PHI in your responses. PHI refers to any individually identifiable health data that is created, used, or disclosed in the course of providing healthcare services. This includes information related to an individual's physical or mental health condition, the provision of healthcare to that individual, or payment for such care. Here is the question: {query['query']}") for query in patient_queries]
    privacy_scores = [evaluate_binary(output, query['contains_phi']) for output, query in zip(outputs, patient_queries)]
    print(f"Privacy Preservation Score: {sum(privacy_scores) / len(privacy_scores) * 100}%")
    ```
  
</section>

  <section title="Utilizzo del contesto (assistente conversazionale) - scala ordinale basata su LLM">

    **Cosa misura**: Simile alla scala Likert, la scala ordinale misura su una scala fissa e ordinata (1-5). È perfetta per valutare l'utilizzo del contesto perché può catturare il grado in cui il modello fa riferimento e si basa sulla cronologia della conversazione, che è fondamentale per interazioni coerenti e personalizzate.

    **Esempi di casi di test di valutazione**: 100 conversazioni multi-turno con domande dipendenti dal contesto.
    ```python
    import anthropic

    conversations = [
        [
            {"role": "user", "content": "I just got a new pomeranian!"},
            {"role": "assistant", "content": "Congratulations on your new furry friend! Is this your first dog?"},
            {"role": "user", "content": "Yes, it is. I named her Luna."},
            {"role": "assistant", "content": "Luna is a lovely name! As a first-time dog owner, you might have some questions. What would you like to know about caring for Luna?"},
            ...
            {"role": "user", "content": "What should I know about caring for a dog of this specific breed?"}  # Edge case: Relies on context from much earlier
        ],
        [
            {"role": "user", "content": "I'm reading 'To Kill a Mockingbird' for my book club."},
            {"role": "assistant", "content": "Great choice! 'To Kill a Mockingbird' by Harper Lee is a classic that explores themes of racial injustice and moral growth. How are you finding it so far?"},
            {"role": "user", "content": "It's powerful. Hey, when was Scout's birthday again?"},  # Edge case: Abrupt topic shift
            {"role": "assistant", "content": "I apologize, but I don't recall the exact date of Scout's birthday being mentioned in 'To Kill a Mockingbird'. The novel doesn't focus on such specific details. Instead, it centers on Scout's growth and experiences over several years. Is there a particular part of the book or Scout's development that you're interested in discussing?"},
            {"role": "user", "content": "Oh, right. Well, can you suggest a recipe for a classic Southern cake?"}  # Edge case: Another topic shift
        ],
        # ... 98 more conversations
    ]

    client = anthropic.Anthropic()

    def get_completion(prompt: str):
        message = client.messages.create(
            model="claude-sonnet-4-5",
            max_tokens=1024,
            messages=[
            {"role": "user", "content": prompt}
            ]
        )
        return message.content[0].text

    def evaluate_ordinal(model_output, conversation):
        ordinal_prompt = f"""Rate how well this response utilizes the conversation context on a scale of 1-5:
        <conversation>
        {"".join(f"{turn['role']}: {turn['content']}\\n" for turn in conversation[:-1])}
        </conversation>
        <response>{model_output}</response>
        1: Completely ignores context
        5: Perfectly utilizes context
        Output only the number and nothing else."""

        # Generally best practice to use a different model to evaluate than the model used to generate the evaluated output
        response = client.messages.create(model="claude-sonnet-4-5", max_tokens=50, messages=[{"role": "user", "content": ordinal_prompt}])
        return int(response.content[0].text.strip())

    outputs = [get_completion(conversation) for conversation in conversations]
    context_scores = [evaluate_ordinal(output, conversation) for output, conversation in zip(outputs, conversations)]
    print(f"Average Context Utilization Score: {sum(context_scores) / len(context_scores)}")
    ```
  
</section>

<Tip>Scrivere centinaia di casi di test può essere difficile da fare a mano! Fai aiutare Claude a generarne di più da un set di base di casi di test di esempio.</Tip>
<Tip>Se non sai quali metodi di valutazione potrebbero essere utili per valutare i tuoi criteri di successo, puoi anche fare brainstorming con Claude!</Tip>

***

## Valutare le valutazioni

Quando decidi quale metodo utilizzare per valutare le valutazioni, scegli il metodo più veloce, più affidabile e più scalabile:

1. **Valutazione basata su codice**: Più veloce e più affidabile, estremamente scalabile, ma manca anche di sfumature per giudizi più complessi che richiedono meno rigidità basata su regole.
   - Corrispondenza esatta: `output == golden_answer`
   - Corrispondenza di stringa: `key_phrase in output`

2. **Valutazione umana**: Più flessibile e di alta qualità, ma lenta e costosa. Evita se possibile.

3. **Valutazione basata su LLM**: Veloce e flessibile, scalabile e adatta per giudizi complessi. Testa prima per assicurare l'affidabilità poi scala.

### Suggerimenti per la valutazione basata su LLM
- **Avere rubriche dettagliate e chiare**: "La risposta dovrebbe sempre menzionare 'Acme Inc.' nella prima frase. Se non lo fa, la risposta viene automaticamente valutata come 'incorretta.'"
    <Note>Un dato caso d'uso, o anche un criterio di successo specifico per quel caso d'uso, potrebbe richiedere diverse rubriche per una valutazione olistica.</Note>
- **Empirico o specifico**: Ad esempio, istruisci l'LLM a produrre solo 'corretto' o 'incorretto', o a giudicare da una scala di 1-5. Le valutazioni puramente qualitative sono difficili da valutare rapidamente e su scala.
- **Incoraggiare il ragionamento**: Chiedi all'LLM di pensare prima prima di decidere un punteggio di valutazione, e poi scarta il ragionamento. Questo aumenta le prestazioni di valutazione, particolarmente per compiti che richiedono giudizi complessi.

<section title="Esempio: Valutazione basata su LLM">

```python
import anthropic

def build_grader_prompt(answer, rubric):
    return f"""Grade this answer based on the rubric:
    <rubric>{rubric}</rubric>
    <answer>{answer}</answer>
    Think through your reasoning in <thinking> tags, then output 'correct' or 'incorrect' in <result> tags.""

def grade_completion(output, golden_answer):
    grader_response = client.messages.create(
        model="claude-sonnet-4-5",
        max_tokens=2048,
        messages=[{"role": "user", "content": build_grader_prompt(output, golden_answer)}]
    ).content[0].text

    return "correct" if "correct" in grader_response.lower() else "incorrect"

# Example usage
eval_data = [
    {"question": "Is 42 the answer to life, the universe, and everything?", "golden_answer": "Yes, according to 'The Hitchhiker's Guide to the Galaxy'."},
    {"question": "What is the capital of France?", "golden_answer": "The capital of France is Paris."}
]

def get_completion(prompt: str):
    message = client.messages.create(
        model="claude-sonnet-4-5",
        max_tokens=1024,
        messages=[
        {"role": "user", "content": prompt}
        ]
    )
    return message.content[0].text

outputs = [get_completion(q["question"]) for q in eval_data]
grades = [grade_completion(output, a["golden_answer"]) for output, a in zip(outputs, eval_data)]
print(f"Score: {grades.count('correct') / len(grades) * 100}%")
```

</section>

## Prossimi passi

<CardGroup cols={2}>
  <Card title="Brainstorming valutazioni" icon="link" href="/docs/it/build-with-claude/prompt-engineering/overview">
    Impara come creare prompt che massimizzano i tuoi punteggi di valutazione.
  </Card>
  <Card title="Cookbook delle valutazioni" icon="link" href="https://github.com/anthropics/anthropic-cookbook/blob/main/misc/building%5Fevals.ipynb">
    Altri esempi di codice di valutazioni valutate da umani, codice e LLM.
  </Card>
</CardGroup>