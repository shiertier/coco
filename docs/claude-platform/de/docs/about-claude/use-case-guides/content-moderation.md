# Inhaltsmoderation

Inhaltsmoderation ist ein kritischer Aspekt für die Aufrechterhaltung einer sicheren, respektvollen und produktiven Umgebung in digitalen Anwendungen. In diesem Leitfaden besprechen wir, wie Claude zur Moderation von Inhalten in Ihrer digitalen Anwendung verwendet werden kann.

---

> Besuchen Sie unser [Inhaltsmoderation-Kochbuch](https://github.com/anthropics/anthropic-cookbook/blob/main/misc/building%5Fmoderation%5Ffilter.ipynb), um ein Beispiel für eine Inhaltsmoderation-Implementierung mit Claude zu sehen.

<Tip>Dieser Leitfaden konzentriert sich auf die Moderation von benutzergenerierten Inhalten in Ihrer Anwendung. Wenn Sie nach Anleitungen zur Moderation von Interaktionen mit Claude suchen, lesen Sie bitte unseren [Leitfaden zu Schutzmaßnahmen](/docs/de/test-and-evaluate/strengthen-guardrails/reduce-hallucinations).</Tip>

## Vor der Entwicklung mit Claude

### Entscheiden Sie, ob Sie Claude für die Inhaltsmoderation verwenden möchten

Hier sind einige wichtige Indikatoren dafür, dass Sie ein LLM wie Claude anstelle eines traditionellen ML- oder regelbasierten Ansatzes für die Inhaltsmoderation verwenden sollten:

<section title="Sie möchten eine kosteneffektive und schnelle Implementierung">
Traditionelle ML-Methoden erfordern erhebliche Ingenieurressourcen, ML-Expertise und Infrastrukturkosten. Menschliche Moderationssysteme verursachen sogar noch höhere Kosten. Mit Claude können Sie ein ausgeklügeltes Moderationssystem in einem Bruchteil der Zeit für einen Bruchteil des Preises zum Laufen bringen.
</section>
<section title="Sie wünschen sowohl semantisches Verständnis als auch schnelle Entscheidungen">
Traditionelle ML-Ansätze, wie Bag-of-Words-Modelle oder einfache Musterabgleichung, haben oft Schwierigkeiten, den Ton, die Absicht und den Kontext des Inhalts zu verstehen. Während menschliche Moderationssysteme beim Verstehen semantischer Bedeutung hervorragend sind, benötigen sie Zeit für die Überprüfung von Inhalten. Claude überbrückt diese Lücke, indem es semantisches Verständnis mit der Fähigkeit kombiniert, Moderationsentscheidungen schnell zu treffen.
</section>
<section title="Sie benötigen konsistente Richtlinienentscheidungen">
Durch die Nutzung seiner fortgeschrittenen Argumentationsfähigkeiten kann Claude komplexe Moderationsrichtlinien einheitlich interpretieren und anwenden. Diese Konsistenz hilft dabei, eine faire Behandlung aller Inhalte zu gewährleisten und das Risiko inkonsistenter oder voreingenommener Moderationsentscheidungen zu reduzieren, die das Vertrauen der Benutzer untergraben können.
</section>
<section title="Ihre Moderationsrichtlinien werden sich wahrscheinlich ändern oder im Laufe der Zeit weiterentwickeln">
Sobald ein traditioneller ML-Ansatz etabliert wurde, ist eine Änderung ein mühsames und datenintensives Unterfangen. Andererseits kann sich Claude, wenn sich Ihr Produkt oder die Bedürfnisse Ihrer Kunden entwickeln, leicht an Änderungen oder Ergänzungen der Moderationsrichtlinien anpassen, ohne umfangreiche Neuetikettierung von Trainingsdaten.
</section>
<section title="Sie benötigen interpretierbare Begründungen für Ihre Moderationsentscheidungen">
Wenn Sie Benutzern oder Regulierungsbehörden klare Erklärungen für Moderationsentscheidungen liefern möchten, kann Claude detaillierte und kohärente Begründungen generieren. Diese Transparenz ist wichtig für den Aufbau von Vertrauen und die Gewährleistung von Rechenschaftspflicht in Inhaltsmoderation-Praktiken.
</section>
<section title="Sie benötigen mehrsprachige Unterstützung ohne separate Modelle zu pflegen">
Traditionelle ML-Ansätze erfordern typischerweise separate Modelle oder umfangreiche Übersetzungsprozesse für jede unterstützte Sprache. Menschliche Moderation erfordert die Einstellung einer Belegschaft, die in jeder unterstützten Sprache fließend ist. Claudes mehrsprachige Fähigkeiten ermöglichen es, Tickets in verschiedenen Sprachen zu klassifizieren, ohne separate Modelle oder umfangreiche Übersetzungsprozesse zu benötigen, was die Moderation für globale Kundenstämme vereinfacht.
</section>
<section title="Sie benötigen multimodale Unterstützung">
Claudes multimodale Fähigkeiten ermöglichen es, Inhalte sowohl in Text als auch in Bildern zu analysieren und zu interpretieren. Dies macht es zu einem vielseitigen Werkzeug für umfassende Inhaltsmoderation in Umgebungen, in denen verschiedene Medientypen zusammen bewertet werden müssen.
</section>

<Note>Anthropic hat alle Claude-Modelle darauf trainiert, ehrlich, hilfreich und harmlos zu sein. Dies kann dazu führen, dass Claude Inhalte moderiert, die als besonders gefährlich eingestuft werden (im Einklang mit unserer [Richtlinie für akzeptable Nutzung](https://www.anthropic.com/legal/aup)), unabhängig von der verwendeten Eingabeaufforderung. Zum Beispiel könnte eine Website für Erwachsene, die Benutzern erlauben möchte, explizite sexuelle Inhalte zu posten, feststellen, dass Claude explizite Inhalte immer noch als moderationsbedürftig kennzeichnet, auch wenn sie in ihrer Eingabeaufforderung angeben, explizite sexuelle Inhalte nicht zu moderieren. Wir empfehlen, unsere AUP im Voraus der Entwicklung einer Moderationslösung zu überprüfen.</Note>

### Generieren Sie Beispiele für zu moderierende Inhalte
Bevor Sie eine Inhaltsmoderation-Lösung entwickeln, erstellen Sie zunächst Beispiele für Inhalte, die gekennzeichnet werden sollten, und Inhalte, die nicht gekennzeichnet werden sollten. Stellen Sie sicher, dass Sie Grenzfälle und herausfordernde Szenarien einschließen, die für ein Inhaltsmoderation-System schwer effektiv zu handhaben sein könnten. Überprüfen Sie anschließend Ihre Beispiele, um eine gut definierte Liste von Moderationskategorien zu erstellen.
Zum Beispiel könnten die von einer Social-Media-Plattform generierten Beispiele folgende umfassen:

```python
allowed_user_comments = [
    'This movie was great, I really enjoyed it. The main actor really killed it!',
    'I hate Mondays.',
    'It is a great time to invest in gold!'
]

disallowed_user_comments = [
    'Delete this post now or you better hide. I am coming after you and your family.',
    'Stay away from the 5G cellphones!! They are using 5G to control you.',
    'Congratulations! You have won a $1,000 gift card. Click here to claim your prize!'
]

# Sample user comments to test the content moderation
user_comments = allowed_user_comments + disallowed_user_comments

# List of categories considered unsafe for content moderation
unsafe_categories = [
    'Child Exploitation',
    'Conspiracy Theories',
    'Hate',
    'Indiscriminate Weapons', 
    'Intellectual Property',
    'Non-Violent Crimes', 
    'Privacy',
    'Self-Harm',
    'Sex Crimes',
    'Sexual Content',
    'Specialized Advice',
    'Violent Crimes'
]
```

Die effektive Moderation dieser Beispiele erfordert ein nuanciertes Verständnis der Sprache. In dem Kommentar `This movie was great, I really enjoyed it. The main actor really killed it!` muss das Inhaltsmoderation-System erkennen, dass "killed it" eine Metapher ist, nicht ein Hinweis auf tatsächliche Gewalt. Umgekehrt sollte der Kommentar `Delete this post now or you better hide. I am coming after you and your family.` trotz des Fehlens expliziter Gewalterwähnungen vom Inhaltsmoderation-System gekennzeichnet werden.

Die `unsafe_categories`-Liste kann an Ihre spezifischen Bedürfnisse angepasst werden. Wenn Sie beispielsweise verhindern möchten, dass Minderjährige Inhalte auf Ihrer Website erstellen, könnten Sie "Underage Posting" zur Liste hinzufügen.

___

## Wie man Inhalte mit Claude moderiert

### Wählen Sie das richtige Claude-Modell
Bei der Auswahl eines Modells ist es wichtig, die Größe Ihrer Daten zu berücksichtigen. Wenn Kosten ein Anliegen sind, ist ein kleineres Modell wie Claude Haiku 3 aufgrund seiner Kosteneffizienz eine ausgezeichnete Wahl. Unten ist eine Schätzung der Kosten zur Moderation von Text für eine Social-Media-Plattform, die eine Milliarde Posts pro Monat erhält:

* **Inhaltsgröße**
    * Posts pro Monat: 1 Mrd.
    * Zeichen pro Post: 100
    * Gesamtzeichen: 100 Mrd.

* **Geschätzte Token**
    * Input-Token: 28,6 Mrd. (angenommen 1 Token pro 3,5 Zeichen)
    * Prozentsatz der gekennzeichneten Nachrichten: 3%
    * Output-Token pro gekennzeichneter Nachricht: 50
    * Gesamte Output-Token: 1,5 Mrd.

* **Claude Haiku 3 geschätzte Kosten**
    * Input-Token-Kosten: 2.860 MTok * \$0,25/MTok = \$715
    * Output-Token-Kosten: 1.500 MTok * \$1,25/MTok = \$1.875
    * Monatliche Kosten: \$715 + \$1.875 = \$2.590

* **Claude Sonnet 4.5 geschätzte Kosten**
    * Input-Token-Kosten: 2.860 MTok * \$3,00/MTok = \$8.580
    * Output-Token-Kosten: 1.500 MTok * \$15,00/MTok = \$22.500
    * Monatliche Kosten: \$8.580 + \$22.500 = \$31.080

<Tip>Die tatsächlichen Kosten können von diesen Schätzungen abweichen. Diese Schätzungen basieren auf der Eingabeaufforderung, die im Abschnitt über [Stapelverarbeitung](#consider-batch-processing) hervorgehoben wird. Output-Token können noch weiter reduziert werden, indem das `explanation`-Feld aus der Antwort entfernt wird.</Tip>  

### Erstellen Sie eine starke Eingabeaufforderung

Um Claude für die Inhaltsmoderation zu verwenden, muss Claude die Moderationsanforderungen Ihrer Anwendung verstehen. Beginnen wir mit dem Schreiben einer Eingabeaufforderung, die es Ihnen ermöglicht, Ihre Moderationsbedürfnisse zu definieren:

```python
import anthropic
import json

# Initialize the Anthropic client
client = anthropic.Anthropic()

def moderate_message(message, unsafe_categories):
    # Convert the list of unsafe categories into a string, with each category on a new line
    unsafe_category_str = '\n'.join(unsafe_categories)
    
    # Construct the prompt for Claude, including the message and unsafe categories
    assessment_prompt = f"""
    Determine whether the following message warrants moderation, 
    based on the unsafe categories outlined below.
    
    Message:
    <message>{message}</message>

    Unsafe Categories:
    <categories>
    {unsafe_category_str}
    </categories>

    Respond with ONLY a JSON object, using the format below:
    {{
    "violation": <Boolean field denoting whether the message should be moderated>,
    "categories": [Comma-separated list of violated categories],
    "explanation": [Optional. Only include if there is a violation.]
    }}"""

    # Send the request to Claude for content moderation
    response = client.messages.create(
        model="claude-3-haiku-20240307",  # Using the Haiku model for lower costs
        max_tokens=200,
        temperature=0,   # Use 0 temperature for increased consistency
        messages=[
            {"role": "user", "content": assessment_prompt}
        ]
    )
    
    # Parse the JSON response from Claude
    assessment = json.loads(response.content[0].text)
    
    # Extract the violation status from the assessment
    contains_violation = assessment['violation']
    
    # If there's a violation, get the categories and explanation; otherwise, use empty defaults
    violated_categories = assessment.get('categories', []) if contains_violation else []
    explanation = assessment.get('explanation') if contains_violation else None
    
    return contains_violation, violated_categories, explanation

# Process each comment and print the results
for comment in user_comments:
    print(f"\nComment: {comment}")
    violation, violated_categories, explanation = moderate_message(comment, unsafe_categories)
    
    if violation:
        print(f"Violated Categories: {', '.join(violated_categories)}")
        print(f"Explanation: {explanation}")
    else:
        print("No issues detected.")
```

In diesem Beispiel enthält die `moderate_message`-Funktion eine Bewertungsaufforderung, die die unsicheren Inhaltskategorien und die Nachricht, die wir bewerten möchten, einschließt. Die Eingabeaufforderung bittet Claude zu bewerten, ob die Nachricht moderiert werden sollte, basierend auf den unsicheren Kategorien, die wir definiert haben.

Die Bewertung des Modells wird dann geparst, um zu bestimmen, ob es eine Verletzung gibt. Wenn es eine Verletzung gibt, gibt Claude auch eine Liste der verletzten Kategorien sowie eine Erklärung zurück, warum die Nachricht unsicher ist.

### Bewerten Sie Ihre Eingabeaufforderung

Inhaltsmoderation ist ein Klassifikationsproblem. Daher können Sie dieselben Techniken verwenden, die in unserem [Klassifikations-Kochbuch](https://github.com/anthropics/anthropic-cookbook/blob/main/skills/classification/guide.ipynb) beschrieben sind, um die Genauigkeit Ihres Inhaltsmoderation-Systems zu bestimmen.

Eine zusätzliche Überlegung ist, dass Sie anstatt die Inhaltsmoderation als binäres Klassifikationsproblem zu behandeln, stattdessen mehrere Kategorien erstellen können, um verschiedene Risikoebenen darzustellen. Das Erstellen mehrerer Risikoebenen ermöglicht es Ihnen, die Aggressivität Ihrer Moderation anzupassen. Zum Beispiel möchten Sie möglicherweise Benutzeranfragen, die als hohes Risiko eingestuft werden, automatisch blockieren, während Benutzer mit vielen mittleren Risikoanfragen für menschliche Überprüfung gekennzeichnet werden.

```python
import anthropic
import json

# Initialize the Anthropic client
client = anthropic.Anthropic()

def assess_risk_level(message, unsafe_categories):
    # Convert the list of unsafe categories into a string, with each category on a new line
    unsafe_category_str = '\n'.join(unsafe_categories)
    
    # Construct the prompt for Claude, including the message, unsafe categories, and risk level definitions
    assessment_prompt = f"""
    Assess the risk level of the following message warrants moderation, 
    based on the unsafe categories listed below.
    
Message:
<message>{message}</message>

Unsafe Categories:
<categories>
{unsafe_category_str}
</categories>

Assign a risk level based on your confidence that the user's message should be moderated 
based on the defined unsafe categories:

0 - No risk
1 - Low risk
2 - Medium risk
3 - High risk

Respond with ONLY a JSON object, using the format below:
{{
  "risk_level": <Numerical field denoting the risk level>,
  "categories": [Comma-separated list of violated categories],
  "explanation": <Optional. Only include if risk level is greater than 0>
}}"""

    # Send the request to Claude for risk assessment
    response = client.messages.create(
        model="claude-3-haiku-20240307",  # Using the Haiku model for lower costs
        max_tokens=200,
        temperature=0,   # Use 0 temperature for increased consistency
        messages=[
            {"role": "user", "content": assessment_prompt}
        ]
    )
    
    # Parse the JSON response from Claude
    assessment = json.loads(response.content[0].text)
    
    # Extract the risk level, violated categories, and explanation from the assessment
    risk_level = assessment["risk_level"]
    violated_categories = assessment["categories"]
    explanation = assessment.get("explanation")
    
    return risk_level, violated_categories, explanation

# Process each comment and print the results
for comment in user_comments:
    print(f"\nComment: {comment}")
    risk_level, violated_categories, explanation = assess_risk_level(comment, unsafe_categories)
    
    print(f"Risk Level: {risk_level}")
    if violated_categories:
        print(f"Violated Categories: {', '.join(violated_categories)}")
    if explanation:
        print(f"Explanation: {explanation}")
```

Dieser Code implementiert eine `assess_risk_level`-Funktion, die Claude verwendet, um das Risikoniveau einer Nachricht zu bewerten. Die Funktion akzeptiert eine Nachricht und eine Liste unsicherer Kategorien als Eingaben.

Innerhalb der Funktion wird eine Eingabeaufforderung für Claude generiert, die die zu bewertende Nachricht, die unsicheren Kategorien und spezifische Anweisungen zur Bewertung des Risikoniveaus einschließt. Die Eingabeaufforderung weist Claude an, mit einem JSON-Objekt zu antworten, das das Risikoniveau, die verletzten Kategorien und eine optionale Erklärung einschließt.

Dieser Ansatz ermöglicht flexible Inhaltsmoderation durch die Zuweisung von Risikoebenen. Er kann nahtlos in ein größeres System integriert werden, um die Inhaltsfilterung zu automatisieren oder Kommentare basierend auf ihrem bewerteten Risikoniveau für menschliche Überprüfung zu kennzeichnen. Zum Beispiel wird bei der Ausführung dieses Codes der Kommentar `Delete this post now or you better hide. I am coming after you and your family.` als hohes Risiko identifiziert aufgrund seiner gefährlichen Bedrohung. Umgekehrt wird der Kommentar `Stay away from the 5G cellphones!! They are using 5G to control you.` als mittleres Risiko kategorisiert.

### Stellen Sie Ihre Eingabeaufforderung bereit

Sobald Sie von der Qualität Ihrer Lösung überzeugt sind, ist es Zeit, sie in der Produktion bereitzustellen. Hier sind einige bewährte Praktiken, die Sie befolgen sollten, wenn Sie Inhaltsmoderation in der Produktion verwenden:

1. **Geben Sie Benutzern klares Feedback:** Wenn Benutzereingaben blockiert werden oder eine Antwort aufgrund von Inhaltsmoderation gekennzeichnet wird, geben Sie informatives und konstruktives Feedback, um Benutzern zu helfen zu verstehen, warum ihre Nachricht gekennzeichnet wurde und wie sie sie angemessen umformulieren können. In den obigen Codierungsbeispielen geschieht dies durch das `explanation`-Tag in der Claude-Antwort.

2. **Analysieren Sie moderierte Inhalte:** Verfolgen Sie die Arten von Inhalten, die von Ihrem Moderationssystem gekennzeichnet werden, um Trends und potenzielle Verbesserungsbereiche zu identifizieren.

3. **Kontinuierlich bewerten und verbessern:** Bewerten Sie regelmäßig die Leistung Ihres Inhaltsmoderation-Systems mit Metriken wie Präzisions- und Recall-Tracking. Verwenden Sie diese Daten, um Ihre Moderationsaufforderungen, Schlüsselwörter und Bewertungskriterien iterativ zu verfeinern.

___

## Leistung verbessern

In komplexen Szenarien kann es hilfreich sein, zusätzliche Strategien zu berücksichtigen, um die Leistung über standardmäßige [Eingabeaufforderungs-Engineering-Techniken](/docs/de/build-with-claude/prompt-engineering/overview) hinaus zu verbessern. Hier sind einige fortgeschrittene Strategien:

### Definieren Sie Themen und geben Sie Beispiele

Zusätzlich zur Auflistung der unsicheren Kategorien in der Eingabeaufforderung können weitere Verbesserungen vorgenommen werden, indem Definitionen und Phrasen zu jeder Kategorie bereitgestellt werden.

```python
import anthropic
import json

# Initialize the Anthropic client
client = anthropic.Anthropic()

# Dictionary of categories considered unsafe for content moderation, with their definitions
unsafe_category_definitions = {
    'Child Exploitation': 'Content that depicts child nudity or that enables, encourages, excuses, or depicts the sexual abuse of children.',
    'Conspiracy Theories': """Content that promotes or endorses unfounded, false, or misleading theories about events, situations, or individuals, 
        which can incite harm or undermine public trust in institutions or facts.""",
    'Hate': """Content that is hateful toward people on the basis of their protected characteristics 
        (race, color, ethnicity, national origin, disability, religious affiliation, caste, sexual orientation, sex, gender identity and serious disease), 
        or content that perpetuates negative stereotypes.""",
    'Indiscriminate Weapons': 'Content that enables the creation of weapons of mass destruction.', 
    'Intellectual Property': 'Content that violates the intellectual property rights of third parties.',
    'Non-Violent Crimes': 'Content that enables, encourages, or excuses the commission of non-violent crimes.', 
    '': 'Content that contains sensitive, personal information about private individuals.',
    'Self-Harm': 'Content that enables, encourages, or excuses acts of intentional self-harm.',
    'Sex Crimes': 'Content that enables, encourages, or excuses the commission of sex crimes.',
    'Sexual Content': 'Content that contains erotic descriptions or explicit depictions of sex acts.',
    'Specialized Advice': """Content that contains financial, medical, or legal advice. 
        Financial advice includes guidance on investments, stocks, bonds, or any financial planning.""",
    'Violent Crimes': 'Content that enables, encourages, or excuses the commission of violent crimes.', 
}


def moderate_message_with_definitions(message, unsafe_category_definitions):
    # Format unsafe categories string, with each category and its definition on a new line
    unsafe_category_str = '\n'.join(f'{category}: {definition}' 
        for category, definition in unsafe_category_definitions.items()
    )
    
    # Construct the prompt for Claude, including the message and unsafe categories
    assessment_prompt = f"""Determine whether the following message warrants moderation, based on the unsafe categories outlined below.

Message:
<message>{message}</message>

Unsafe Categories and Their Definitions:
<categories>
{unsafe_category_str}
</categories>

It's important that you remember all unsafe categories and their definitions.

Respond with ONLY a JSON object, using the format below:
{{
  "violation": <Boolean field denoting whether the message should be moderated>,
  "categories": [Comma-separated list of violated categories],
  "explanation": [Optional. Only include if there is a violation.]
}}"""

    # Send the request to Claude for content moderation
    response = client.messages.create(
        model="claude-3-haiku-20240307",  # Using the Haiku model for lower costs
        max_tokens=200,
        temperature=0,   # Use 0 temperature for increased consistency
        messages=[
            {"role": "user", "content": assessment_prompt}
        ]
    )
    
    # Parse the JSON response from Claude
    assessment = json.loads(response.content[0].text)
    
    # Extract the violation status from the assessment
    contains_violation = assessment['violation']
    
    # If there's a violation, get the categories and explanation; otherwise, use empty defaults
    violated_categories = assessment.get('categories', []) if contains_violation else []
    explanation = assessment.get('explanation') if contains_violation else None
    
    return contains_violation, violated_categories, explanation


# Process each comment and print the results
for comment in user_comments:
    print(f"\nComment: {comment}")
    violation, violated_categories, explanation = moderate_message_with_definitions(comment, unsafe_category_definitions)
    
    if violation:
        print(f"Violated Categories: {', '.join(violated_categories)}")
        print(f"Explanation: {explanation}")
    else:
        print("No issues detected.")
```

Die `moderate_message_with_definitions`-Funktion erweitert die frühere `moderate_message`-Funktion, indem sie es ermöglicht, jede unsichere Kategorie mit einer detaillierten Definition zu paaren. Dies geschieht im Code durch das Ersetzen der `unsafe_categories`-Liste aus der ursprünglichen Funktion durch ein `unsafe_category_definitions`-Wörterbuch. Dieses Wörterbuch ordnet jede unsichere Kategorie ihrer entsprechenden Definition zu. Sowohl die Kategorienamen als auch ihre Definitionen sind in der Eingabeaufforderung enthalten.

Bemerkenswert ist, dass die Definition für die `Specialized Advice`-Kategorie nun die Arten von Finanzberatung spezifiziert, die verboten werden sollten. Infolgedessen löst der Kommentar `It's a great time to invest in gold!`, der zuvor die `moderate_message`-Bewertung bestanden hat, nun eine Verletzung aus.

### Erwägen Sie Stapelverarbeitung

Um Kosten in Situationen zu reduzieren, in denen Echtzeitmoderation nicht notwendig ist, erwägen Sie die Moderation von Nachrichten in Stapeln. Schließen Sie mehrere Nachrichten in den Kontext der Eingabeaufforderung ein und bitten Sie Claude zu bewerten, welche Nachrichten moderiert werden sollten.

```python
import anthropic
import json

# Initialize the Anthropic client
client = anthropic.Anthropic()

def batch_moderate_messages(messages, unsafe_categories):
    # Convert the list of unsafe categories into a string, with each category on a new line
    unsafe_category_str = '\n'.join(unsafe_categories)
    
    # Format messages string, with each message wrapped in XML-like tags and given an ID
    messages_str = '\n'.join([f'<message id={idx}>{msg}</message>' for idx, msg in enumerate(messages)])
    
    # Construct the prompt for Claude, including the messages and unsafe categories
    assessment_prompt = f"""Determine the messages to moderate, based on the unsafe categories outlined below.

Messages:
<messages>
{messages_str}
</messages>

Unsafe categories and their definitions:
<categories>
{unsafe_category_str}
</categories>

Respond with ONLY a JSON object, using the format below:
{{
  "violations": [
    {{
      "id": <message id>,
      "categories": [list of violated categories],
      "explanation": <Explanation of why there's a violation>
    }},
    ...
  ]
}}

Important Notes:
- Remember to analyze every message for a violation.
- Select any number of violations that reasonably apply."""

    # Send the request to Claude for content moderation
    response = client.messages.create(
        model="claude-3-haiku-20240307",  # Using the Haiku model for lower costs
        max_tokens=2048,  # Increased max token count to handle batches
        temperature=0,    # Use 0 temperature for increased consistency
        messages=[
            {"role": "user", "content": assessment_prompt}
        ]
    )
    
    # Parse the JSON response from Claude
    assessment = json.loads(response.content[0].text)
    return assessment


# Process the batch of comments and get the response
response_obj = batch_moderate_messages(user_comments, unsafe_categories)

# Print the results for each detected violation
for violation in response_obj['violations']:
    print(f"""Comment: {user_comments[violation['id']]}
Violated Categories: {', '.join(violation['categories'])}
Explanation: {violation['explanation']}
""")
```
In diesem Beispiel behandelt die `batch_moderate_messages`-Funktion die Moderation eines ganzen Stapels von Nachrichten mit einem einzigen Claude-API-Aufruf.
Innerhalb der Funktion wird eine Eingabeaufforderung erstellt, die die Liste der zu bewertenden Nachrichten, die definierten unsicheren Inhaltskategorien und ihre Beschreibungen einschließt. Die Eingabeaufforderung weist Claude an, ein JSON-Objekt zurückzugeben, das alle Nachrichten auflistet, die Verletzungen enthalten. Jede Nachricht in der Antwort wird durch ihre ID identifiziert, die der Position der Nachricht in der Eingabeliste entspricht.
Beachten Sie, dass das Finden der optimalen Stapelgröße für Ihre spezifischen Bedürfnisse möglicherweise etwas Experimentieren erfordert. Während größere Stapelgrößen die Kosten senken können, könnten sie auch zu einem leichten Qualitätsverlust führen. Zusätzlich müssen Sie möglicherweise den `max_tokens`-Parameter im Claude-API-Aufruf erhöhen, um längere Antworten zu berücksichtigen. Für Details zur maximalen Anzahl von Token, die Ihr gewähltes Modell ausgeben kann, siehe die [Modellvergleichsseite](/docs/de/about-claude/models#model-comparison-table).

<CardGroup cols={2}> 
  <Card title="Inhaltsmoderation-Kochbuch" icon="link" href="https://github.com/anthropics/anthropic-cookbook/blob/main/misc/building%5Fmoderation%5Ffilter.ipynb">
    Sehen Sie sich ein vollständig implementiertes codebasiertes Beispiel an, wie Claude für die Inhaltsmoderation verwendet wird.
  </Card>
  <Card title="Leitfaden zu Schutzmaßnahmen" icon="link" href="/docs/de/test-and-evaluate/strengthen-guardrails/reduce-hallucinations">
    Erkunden Sie unseren Leitfaden zu Schutzmaßnahmen für Techniken zur Moderation von Interaktionen mit Claude.
  </Card>
</CardGroup>