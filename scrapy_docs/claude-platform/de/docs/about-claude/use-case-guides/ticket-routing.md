# Ticket-Routing

Diese Anleitung zeigt, wie Sie Claudes fortgeschrittene Fähigkeiten zum Verständnis natürlicher Sprache nutzen können, um Kundenservice-Tickets in großem Maßstab basierend auf Kundenabsicht, Dringlichkeit, Priorisierung, Kundenprofil und mehr zu klassifizieren.

---

## Entscheiden Sie, ob Sie Claude für Ticket-Routing verwenden sollten

Hier sind einige wichtige Indikatoren, dass Sie ein LLM wie Claude anstelle von traditionellen ML-Ansätzen für Ihre Klassifizierungsaufgabe verwenden sollten:

    <section title="Sie haben begrenzte gekennzeichnete Trainingsdaten verfügbar">

        Traditionelle ML-Prozesse erfordern massive gekennzeichnete Datensätze. Claudes vortrainiertes Modell kann Tickets effektiv mit nur wenigen Dutzend gekennzeichneten Beispielen klassifizieren und reduziert damit die Datenvorbereitung und Kosten erheblich.
    
</section>
    <section title="Ihre Klassifizierungskategorien werden sich wahrscheinlich im Laufe der Zeit ändern oder weiterentwickeln">

        Sobald ein traditioneller ML-Ansatz etabliert ist, ist eine Änderung ein mühsames und datenintensives Unterfangen. Andererseits kann Claude sich leicht an Änderungen in Klassendefinitionen oder neue Klassen anpassen, wenn sich Ihr Produkt oder Ihre Kundenbedürfnisse entwickeln, ohne umfangreiche Neuetikettierung von Trainingsdaten.
    
</section>
    <section title="Sie müssen komplexe, unstrukturierte Texteingaben verarbeiten">

        Traditionelle ML-Modelle haben oft Schwierigkeiten mit unstrukturierten Daten und erfordern umfangreiche Feature-Engineering. Claudes fortgeschrittenes Sprachverständnis ermöglicht eine genaue Klassifizierung basierend auf Inhalt und Kontext, anstatt sich auf strikte ontologische Strukturen zu verlassen.
    
</section>
    <section title="Ihre Klassifizierungsregeln basieren auf semantischem Verständnis">

        Traditionelle ML-Ansätze verlassen sich oft auf Bag-of-Words-Modelle oder einfaches Pattern Matching. Claude zeichnet sich darin aus, zugrunde liegende Regeln zu verstehen und anzuwenden, wenn Klassen durch Bedingungen statt durch Beispiele definiert werden.
    
</section>
    <section title="Sie benötigen interpretierbare Begründung für Klassifizierungsentscheidungen">

        Viele traditionelle ML-Modelle bieten wenig Einblick in ihren Entscheidungsprozess. Claude kann menschenlesbare Erklärungen für seine Klassifizierungsentscheidungen liefern, was Vertrauen in das Automatisierungssystem aufbaut und eine einfache Anpassung bei Bedarf ermöglicht.
    
</section>
    <section title="Sie möchten Grenzfälle und mehrdeutige Tickets effektiver handhaben">

        Traditionelle ML-Systeme haben oft Schwierigkeiten mit Ausreißern und mehrdeutigen Eingaben und klassifizieren sie häufig falsch oder ordnen sie einer Sammelkategorie zu. Claudes Fähigkeiten zur Verarbeitung natürlicher Sprache ermöglichen es, Kontext und Nuancen in Support-Tickets besser zu interpretieren, was möglicherweise die Anzahl der falsch gerouteten oder nicht klassifizierten Tickets reduziert, die manuelle Intervention erfordern.
    
</section>
    <section title="Sie benötigen mehrsprachige Unterstützung ohne separate Modelle zu verwalten">

        Traditionelle ML-Ansätze erfordern typischerweise separate Modelle oder umfangreiche Übersetzungsprozesse für jede unterstützte Sprache. Claudes mehrsprachige Fähigkeiten ermöglichen es, Tickets in verschiedenen Sprachen zu klassifizieren, ohne separate Modelle oder umfangreiche Übersetzungsprozesse zu benötigen, was den Support für globale Kundenstämme vereinfacht.
    
</section>

***

##  Erstellen und bereitstellen Sie Ihren LLM-Support-Workflow

### Verstehen Sie Ihren aktuellen Support-Ansatz
Bevor Sie sich in die Automatisierung stürzen, ist es entscheidend, Ihr bestehendes Ticketing-System zu verstehen. Beginnen Sie damit, zu untersuchen, wie Ihr Support-Team derzeit Ticket-Routing handhabt.

Berücksichtigen Sie Fragen wie:
* Welche Kriterien werden verwendet, um zu bestimmen, welche SLA/welches Service-Angebot angewendet wird?
* Wird Ticket-Routing verwendet, um zu bestimmen, welche Support-Stufe oder welcher Produktspezialist ein Ticket erhält?
* Gibt es bereits automatisierte Regeln oder Workflows? In welchen Fällen schlagen sie fehl?
* Wie werden Grenzfälle oder mehrdeutige Tickets behandelt?
* Wie priorisiert das Team Tickets?

Je mehr Sie darüber wissen, wie Menschen bestimmte Fälle handhaben, desto besser können Sie mit Claude zusammenarbeiten, um die Aufgabe zu erfüllen.

### Definieren Sie Kategorien der Benutzerabsicht
Eine gut definierte Liste von Benutzerabsicht-Kategorien ist entscheidend für eine genaue Support-Ticket-Klassifizierung mit Claude. Claudes Fähigkeit, Tickets effektiv in Ihrem System zu routen, ist direkt proportional zu der Qualität der Definition Ihrer Systemkategorien.

Hier sind einige Beispiel-Benutzerabsicht-Kategorien und Unterkategorien.

    <section title="Technisches Problem">

        * Hardwareproblem
        * Softwarefehler
        * Kompatibilitätsproblem
        * Leistungsproblem
    
</section>
    <section title="Kontoverwaltung">

        * Passwort zurücksetzen
        * Probleme mit dem Kontenzugriff
        * Abrechnungsanfragen
        * Abonnementänderungen
    
</section>
    <section title="Produktinformation">

        * Funktionsanfragen
        * Fragen zur Produktkompatibilität
        * Preisinformationen
        * Verfügbarkeitsanfragen
    
</section>
    <section title="Benutzerführung">

        * Wie-zu-Fragen
        * Unterstützung bei der Funktionsnutzung
        * Ratschläge zu Best Practices
        * Fehlerbehebungsanleitung
    
</section>
    <section title="Feedback">

        * Fehlerberichte
        * Feature-Anfragen
        * Allgemeines Feedback oder Vorschläge
        * Beschwerden
    
</section>
    <section title="Bestellbezogen">

        * Bestellstatus-Anfragen
        * Versandinformationen
        * Rückgaben und Umtausch
        * Bestelländerungen
    
</section>
    <section title="Serviceanfrage">

        * Installationsunterstützung
        * Upgrade-Anfragen
        * Wartungsplanung
        * Servicekündigung
    
</section>
    <section title="Sicherheitsbedenken">

        * Datenschutzanfragen
        * Berichte über verdächtige Aktivitäten
        * Unterstützung bei Sicherheitsfunktionen
    
</section>
    <section title="Compliance und Recht">

        * Fragen zur behördlichen Compliance
        * Anfragen zu Nutzungsbedingungen
        * Anfragen für rechtliche Dokumentation
    
</section>
    <section title="Notfall-Support">

        * Kritische Systemausfälle
        * Dringende Sicherheitsprobleme
        * Zeitkritische Probleme
    
</section>
    <section title="Schulung und Bildung">

        * Produktschulungsanfragen
        * Dokumentationsanfragen
        * Informationen zu Webinaren oder Workshops
    
</section>
    <section title="Integration und API">

        * Integrationshilfe
        * Fragen zur API-Nutzung
        * Anfragen zur Kompatibilität mit Drittanbietern
    
</section>

Zusätzlich zur Absicht kann Ticket-Routing und Priorisierung auch durch andere Faktoren wie Dringlichkeit, Kundentyp, SLAs oder Sprache beeinflusst werden. Berücksichtigen Sie unbedingt andere Routing-Kriterien beim Aufbau Ihres automatisierten Routing-Systems.

### Etablieren Sie Erfolgskriterien

Arbeiten Sie mit Ihrem Support-Team zusammen, um [klare Erfolgskriterien zu definieren](/docs/de/test-and-evaluate/define-success) mit messbaren Benchmarks, Schwellwerten und Zielen.

Hier sind einige Standard-Kriterien und Benchmarks bei der Verwendung von LLMs für Support-Ticket-Routing:

    <section title="Klassifizierungskonsistenz">

        Diese Metrik bewertet, wie konsistent Claude ähnliche Tickets im Laufe der Zeit klassifiziert. Sie ist entscheidend für die Aufrechterhaltung der Routing-Zuverlässigkeit. Messen Sie dies, indem Sie das Modell regelmäßig mit einem Satz standardisierter Eingaben testen und eine Konsistenzrate von 95% oder höher anstreben.
    
</section>
    <section title="Anpassungsgeschwindigkeit">

        Dies misst, wie schnell Claude sich an neue Kategorien oder sich ändernde Ticket-Muster anpassen kann. Testen Sie dies, indem Sie neue Ticket-Typen einführen und die Zeit messen, die das Modell benötigt, um eine zufriedenstellende Genauigkeit (z.B. >90%) bei diesen neuen Kategorien zu erreichen. Streben Sie eine Anpassung innerhalb von 50-100 Beispiel-Tickets an.
    
</section>
    <section title="Mehrsprachige Handhabung">

        Dies bewertet Claudes Fähigkeit, Tickets in mehreren Sprachen genau zu routen. Messen Sie die Routing-Genauigkeit über verschiedene Sprachen hinweg und streben Sie nicht mehr als einen Rückgang von 5-10% bei nicht-primären Sprachen an.
    
</section>
    <section title="Handhabung von Grenzfällen">

        Dies bewertet Claudes Leistung bei ungewöhnlichen oder komplexen Tickets. Erstellen Sie einen Testsatz von Grenzfällen und messen Sie die Routing-Genauigkeit, wobei Sie mindestens 80% Genauigkeit bei diesen schwierigen Eingaben anstreben.
    
</section>
    <section title="Bias-Minderung">

        Dies misst Claudes Fairness beim Routing über verschiedene Kundendemografien hinweg. Überprüfen Sie regelmäßig Routing-Entscheidungen auf mögliche Verzerrungen und streben Sie eine konsistente Routing-Genauigkeit (innerhalb von 2-3%) über alle Kundengruppen an.
    
</section>
    <section title="Prompt-Effizienz">

        In Situationen, in denen die Minimierung der Token-Anzahl entscheidend ist, bewertet dieses Kriterium, wie gut Claude mit minimalem Kontext funktioniert. Messen Sie die Routing-Genauigkeit mit unterschiedlichen Mengen an bereitgestelltem Kontext und streben Sie 90%+ Genauigkeit mit nur dem Ticket-Titel und einer kurzen Beschreibung an.
    
</section>
    <section title="Erklärbarkeits-Score">

        Dies bewertet die Qualität und Relevanz von Claudes Erklärungen für seine Routing-Entscheidungen. Menschliche Bewerter können Erklärungen auf einer Skala bewerten (z.B. 1-5), mit dem Ziel, einen Durchschnittswert von 4 oder höher zu erreichen.
    
</section>

Hier sind einige allgemeine Erfolgskriterien, die unabhängig davon nützlich sein können, ob ein LLM verwendet wird:

    <section title="Routing-Genauigkeit">

        Routing-Genauigkeit misst, wie oft Tickets beim ersten Versuch dem richtigen Team oder der richtigen Person zugewiesen werden. Dies wird typischerweise als Prozentsatz der korrekt gerouteten Tickets aus der Gesamtzahl der Tickets gemessen. Branchenbenchmarks streben oft 90-95% Genauigkeit an, obwohl dies je nach Komplexität der Support-Struktur variieren kann.
    
</section>
    <section title="Zeit bis zur Zuweisung">

        Diese Metrik verfolgt, wie schnell Tickets nach der Einreichung zugewiesen werden. Schnellere Zuweisungszeiten führen im Allgemeinen zu schnelleren Lösungen und verbesserter Kundenzufriedenheit. Best-in-Class-Systeme erreichen oft durchschnittliche Zuweisungszeiten von unter 5 Minuten, wobei viele eine nahezu sofortige Weiterleitung anstreben (was mit LLM-Implementierungen möglich ist).
    
</section>
    <section title="Umleitung-Rate">

        Die Umleitung-Rate zeigt an, wie oft Tickets nach dem initialen Routing neu zugewiesen werden müssen. Eine niedrigere Rate deutet auf genaueres initiales Routing hin. Streben Sie eine Umleitung-Rate unter 10% an, wobei Top-Performer Raten von 5% oder weniger erreichen.
    
</section>
    <section title="First-Contact-Resolution-Rate">

        Dies misst den Prozentsatz der Tickets, die während der ersten Interaktion mit dem Kunden gelöst werden. Höhere Raten deuten auf effizientes Routing und gut vorbereitete Support-Teams hin. Branchenbenchmarks liegen typischerweise zwischen 70-75%, wobei Top-Performer Raten von 80% oder höher erreichen.
    
</section>
    <section title="Durchschnittliche Bearbeitungszeit">

        Die durchschnittliche Bearbeitungszeit misst, wie lange es dauert, ein Ticket von Anfang bis Ende zu lösen. Effizientes Routing kann diese Zeit erheblich reduzieren. Benchmarks variieren stark je nach Branche und Komplexität, aber viele Organisationen streben an, die durchschnittliche Bearbeitungszeit für nicht-kritische Probleme unter 24 Stunden zu halten.
    
</section>
    <section title="Kundenzufriedenheitswerte">

        Oft gemessen durch Umfragen nach der Interaktion, spiegeln diese Werte die allgemeine Kundenzufriedenheit mit dem Support-Prozess wider. Effektives Routing trägt zu höherer Zufriedenheit bei. Streben Sie CSAT-Werte von 90% oder höher an, wobei Top-Performer oft 95%+ Zufriedenheitsraten erreichen.
    
</section>
    <section title="Eskalations-Rate">

        Dies misst, wie oft Tickets an höhere Support-Stufen eskaliert werden müssen. Niedrigere Eskalationsraten deuten oft auf genaueres initiales Routing hin. Streben Sie eine Eskalations-Rate unter 20% an, wobei Best-in-Class-Systeme Raten von 10% oder weniger erreichen.
    
</section>
    <section title="Agent-Produktivität">

        Diese Metrik betrachtet, wie viele Tickets Agenten effektiv nach der Implementierung der Routing-Lösung handhaben können. Verbessertes Routing sollte die Produktivität erhöhen. Messen Sie dies, indem Sie die pro Agent pro Tag oder Stunde gelösten Tickets verfolgen und eine Verbesserung von 10-20% nach der Implementierung eines neuen Routing-Systems anstreben.
    
</section>
    <section title="Self-Service-Deflection-Rate">

        Dies misst den Prozentsatz potenzieller Tickets, die durch Self-Service-Optionen gelöst werden, bevor sie das Routing-System betreten. Höhere Raten deuten auf effektive Pre-Routing-Triage hin. Streben Sie eine Deflection-Rate von 20-30% an, wobei Top-Performer Raten von 40% oder höher erreichen.
    
</section>
    <section title="Kosten pro Ticket">

        Diese Metrik berechnet die durchschnittlichen Kosten zur Lösung jedes Support-Tickets. Effizientes Routing sollte diese Kosten im Laufe der Zeit reduzieren. Während Benchmarks stark variieren, streben viele Organisationen an, die Kosten pro Ticket um 10-15% nach der Implementierung eines verbesserten Routing-Systems zu reduzieren.
    
</section>

### Wählen Sie das richtige Claude-Modell

Die Wahl des Modells hängt von den Kompromissen zwischen Kosten, Genauigkeit und Antwortzeit ab.

Viele Kunden haben `claude-haiku-4-5-20251001` als ideales Modell für Ticket-Routing befunden, da es das schnellste und kostengünstigste Modell in der Claude 4-Familie ist und gleichzeitig hervorragende Ergebnisse liefert. Wenn Ihr Klassifizierungsproblem tiefes Fachwissen oder eine große Anzahl von Absicht-Kategorien mit komplexem Reasoning erfordert, können Sie sich für das [größere Sonnet-Modell](/docs/de/about-claude/models) entscheiden.

### Erstellen Sie einen starken Prompt

Ticket-Routing ist eine Art Klassifizierungsaufgabe. Claude analysiert den Inhalt eines Support-Tickets und klassifiziert es in vordefinierte Kategorien basierend auf dem Problemtyp, der Dringlichkeit, erforderlichem Fachwissen oder anderen relevanten Faktoren.

Schreiben wir einen Ticket-Klassifizierungs-Prompt. Unser anfänglicher Prompt sollte den Inhalt der Benutzeranfrage enthalten und sowohl die Begründung als auch die Absicht zurückgeben.

<Tip>
Versuchen Sie den [Prompt-Generator](/docs/de/prompt-generator) auf der [Claude Console](/login), um Claude einen ersten Entwurf schreiben zu lassen.
</Tip>

Hier ist ein Beispiel für einen Ticket-Routing-Klassifizierungs-Prompt:
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

Lassen Sie uns die Schlüsselkomponenten dieses Prompts aufschlüsseln:
* Wir verwenden Python f-Strings, um die Prompt-Vorlage zu erstellen, die es ermöglicht, dass `ticket_contents` in die `<request>`-Tags eingefügt wird.
* Wir geben Claude eine klar definierte Rolle als Klassifizierungssystem, das den Ticket-Inhalt sorgfältig analysiert, um die Kernabsicht und Bedürfnisse des Kunden zu bestimmen.
* Wir weisen Claude die richtige Ausgabeformatierung an, in diesem Fall seine Begründung und Analyse in `<reasoning>`-Tags bereitzustellen, gefolgt von der entsprechenden Klassifizierungsbezeichnung in `<intent>`-Tags.
* Wir geben die gültigen Absicht-Kategorien an: "Support, Feedback, Complaint", "Order Tracking" und "Refund/Exchange".
* Wir fügen einige Beispiele (auch bekannt als Few-Shot-Prompting) ein, um zu zeigen, wie die Ausgabe formatiert werden sollte, was Genauigkeit und Konsistenz verbessert.

Der Grund, warum wir möchten, dass Claude seine Antwort in verschiedene XML-Tag-Abschnitte aufteilt, ist, dass wir reguläre Ausdrücke verwenden können, um die Begründung und Absicht separat aus der Ausgabe zu extrahieren. Dies ermöglicht es uns, gezielte nächste Schritte im Ticket-Routing-Workflow zu erstellen, wie z.B. die Verwendung nur der Absicht, um zu entscheiden, an wen das Ticket weitergeleitet werden soll.

### Stellen Sie Ihren Prompt bereit

Es ist schwierig zu wissen, wie gut Ihr Prompt funktioniert, ohne ihn in einer Test-Produktionsumgebung bereitzustellen und [Evaluierungen durchzuführen](/docs/de/test-and-evaluate/develop-tests).

Lassen Sie uns die Bereitstellungsstruktur aufbauen. Beginnen Sie damit, die Methodensignatur für den Wrapper unseres Aufrufs an Claude zu definieren. Wir nehmen die Methode, die wir bereits begonnen haben zu schreiben, die `ticket_contents` als Eingabe hat, und geben nun ein Tupel von `reasoning` und `intent` als Ausgabe zurück. Wenn Sie eine bestehende Automatisierung mit traditionellem ML haben, sollten Sie stattdessen dieser Methodensignatur folgen.

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

Dieser Code:
* Importiert die Anthropic-Bibliothek und erstellt eine Client-Instanz mit Ihrem API-Schlüssel.
* Definiert eine `classify_support_request`-Funktion, die einen `ticket_contents`-String annimmt.
* Sendet die `ticket_contents` an Claude zur Klassifizierung mit dem `classification_prompt`
* Gibt das `reasoning` und die `intent` des Modells zurück, die aus der Antwort extrahiert werden.

Da wir warten müssen, bis der gesamte Reasoning- und Intent-Text generiert ist, bevor wir ihn analysieren, setzen wir `stream=False` (die Standardeinstellung).

***

## Evaluieren Sie Ihren Prompt

Prompting erfordert oft Tests und Optimierung, um produktionsreif zu sein. Um die Bereitschaft Ihrer Lösung zu bestimmen, evaluieren Sie die Leistung basierend auf den Erfolgskriterien und Schwellwerten, die Sie zuvor etabliert haben.

Um Ihre Evaluierung durchzuführen, benötigen Sie Testfälle, um sie auszuführen. Der Rest dieser Anleitung geht davon aus, dass Sie bereits [Ihre Testfälle entwickelt haben](/docs/de/test-and-evaluate/develop-tests).

### Erstellen Sie eine Evaluierungsfunktion

Unsere Beispiel-Evaluierung für diese Anleitung misst Claudes Leistung entlang von drei Schlüsselmetriken:
* Genauigkeit
* Kosten pro Klassifizierung

Sie müssen Claude möglicherweise auf anderen Achsen bewerten, je nachdem, welche Faktoren für Sie wichtig sind.

Um dies zu bewerten, müssen wir zunächst das Skript, das wir geschrieben haben, modifizieren und eine Funktion hinzufügen, um die vorhergesagte Absicht mit der tatsächlichen Absicht zu vergleichen und den Prozentsatz der korrekten Vorhersagen zu berechnen. Wir müssen auch Kostenberechnungs- und Zeitmessfunktionalität hinzufügen.

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

Lassen Sie uns die Änderungen aufschlüsseln, die wir vorgenommen haben:
* Wir haben die `actual_intent` aus unseren Testfällen in die `classify_support_request`-Methode hinzugefügt und einen Vergleich eingerichtet, um zu bewerten, ob Claudes Intent-Klassifizierung unserer Golden-Intent-Klassifizierung entspricht.
* Wir haben Nutzungsstatistiken für den API-Aufruf extrahiert, um die Kosten basierend auf verwendeten Input- und Output-Tokens zu berechnen

### Führen Sie Ihre Evaluierung durch

Eine ordnungsgemäße Evaluierung erfordert klare Schwellwerte und Benchmarks, um zu bestimmen, was ein gutes Ergebnis ist. Das obige Skript gibt uns die Laufzeitwerte für Genauigkeit, Antwortzeit und Kosten pro Klassifizierung, aber wir müssen immer noch klar etablierte Schwellwerte haben. Zum Beispiel:
* **Genauigkeit:** 95% (aus 100 Tests)
* **Kosten pro Klassifizierung:** 50% Reduktion im Durchschnitt (über 100 Tests) gegenüber der aktuellen Routing-Methode

Mit diesen Schwellwerten können Sie schnell und einfach im großen Maßstab und mit unparteiischem Empirismus feststellen, welche Methode für Sie am besten ist und welche Änderungen möglicherweise vorgenommen werden müssen, um Ihre Anforderungen besser zu erfüllen.

***

## Verbessern Sie die Leistung

In komplexen Szenarien kann es hilfreich sein, zusätzliche Strategien zu erwägen, um die Leistung über Standard-[Prompt-Engineering-Techniken](/docs/de/build-with-claude/prompt-engineering/overview) & [Guardrail-Implementierungsstrategien](/docs/de/test-and-evaluate/strengthen-guardrails/reduce-hallucinations) hinaus zu verbessern. Hier sind einige häufige Szenarien:

### Verwenden Sie eine taxonomische Hierarchie für Fälle mit 20+ Intent-Kategorien

Mit zunehmender Anzahl von Klassen wächst auch die Anzahl der erforderlichen Beispiele, was den Prompt möglicherweise unhandlich macht. Alternativ können Sie ein hierarchisches Klassifizierungssystem mit einer Mischung von Klassifizierern implementieren.
1. Organisieren Sie Ihre Intents in einer taxonomischen Baumstruktur.
2. Erstellen Sie eine Reihe von Klassifizierern auf jeder Ebene des Baums, um einen kaskadierten Routing-Ansatz zu ermöglichen.

Zum Beispiel könnten Sie einen Top-Level-Klassifizierer haben, der Tickets grob in "Technische Probleme", "Abrechnungsfragen" und "Allgemeine Anfragen" kategorisiert. Jede dieser Kategorien kann dann ihren eigenen Sub-Klassifizierer haben, um die Klassifizierung weiter zu verfeinern.

![](/docs/images/ticket-hierarchy.png)

* **Vorteile - größere Nuance und Genauigkeit:** Sie können unterschiedliche Prompts für jeden übergeordneten Pfad erstellen, was eine gezieltere und kontextspezifischere Klassifizierung ermöglicht. Dies kann zu verbesserter Genauigkeit und nuancierteren Umgang mit Kundenanfragen führen.

* **Nachteile - erhöhte Latenz:** Seien Sie gewarnt, dass mehrere Klassifizierer zu erhöhter Latenz führen können, und wir empfehlen, diesen Ansatz mit unserem schnellsten Modell, Haiku, zu implementieren.

### Verwenden Sie Vektor-Datenbanken und Ähnlichkeitssuche-Abruf, um hochvariable Tickets zu handhaben

Obwohl das Bereitstellen von Beispielen die effektivste Methode zur Verbesserung der Leistung ist, kann es schwierig sein, genug Beispiele in einen einzelnen Prompt einzubeziehen, wenn Support-Anfragen hochvariabel sind.

In diesem Szenario könnten Sie eine Vektor-Datenbank verwenden, um Ähnlichkeitssuchen aus einem Datensatz von Beispielen durchzuführen und die relevantesten Beispiele für eine bestimmte Abfrage abzurufen.

Dieser Ansatz, der ausführlich in unserem [Klassifizierungs-Rezept](https://github.com/anthropics/anthropic-cookbook/blob/82675c124e1344639b2a875aa9d3ae854709cd83/skills/classification/guide.ipynb) beschrieben ist, hat sich als Verbesserung der Leistung von 71% Genauigkeit auf 93% Genauigkeit erwiesen.

### Berücksichtigen Sie speziell erwartete Grenzfälle

Hier sind einige Szenarien, in denen Claude Tickets möglicherweise falsch klassifiziert (es kann andere geben, die für Ihre Situation einzigartig sind). In diesen Szenarien sollten Sie explizite Anweisungen oder Beispiele im Prompt bereitstellen, wie Claude den Grenzfall handhaben sollte:

    <section title="Kunden machen implizite Anfragen">

        Kunden drücken Bedürfnisse oft indirekt aus. Zum Beispiel könnte "Ich warte schon über zwei Wochen auf mein Paket" eine indirekte Anfrage zum Bestellstatus sein.
        * **Lösung:** Geben Sie Claude einige echte Kundenbeispiele dieser Art von Anfragen zusammen mit der zugrunde liegenden Absicht. Sie können noch bessere Ergebnisse erzielen, wenn Sie eine Klassifizierungsbegründung für besonders nuancierte Ticket-Intents einbeziehen, damit Claude die Logik besser auf andere Tickets verallgemeinern kann.
    
</section>
    <section title="Claude priorisiert Emotion gegenüber Absicht">

        Wenn Kunden Unzufriedenheit ausdrücken, kann Claude die Emotion gegenüber der Lösung des zugrunde liegenden Problems priorisieren.
        * **Lösung:** Geben Sie Claude Anweisungen, wann Kundengefühle zu priorisieren sind oder nicht. Es kann so einfach sein wie "Ignorieren Sie alle Kundengefühle. Konzentrieren Sie sich nur auf die Analyse der Absicht der Kundenanfrage und welche Informationen der Kunde möglicherweise anfordert."
    
</section>
    <section title="Mehrere Probleme verursachen Verwirrung bei der Problempriorisierung">

        Wenn Kunden mehrere Probleme in einer einzelnen Interaktion präsentieren, kann Claude Schwierigkeiten haben, das primäre Anliegen zu identifizieren.
        * **Lösung:** Klären Sie die Priorisierung von Intents, damit Claude die extrahierten Intents besser einordnen und das primäre Anliegen identifizieren kann.
    
</section>

***

## Integrieren Sie Claude in Ihren größeren Support-Workflow

Eine ordnungsgemäße Integration erfordert, dass Sie einige Entscheidungen treffen, wie Ihr Claude-basiertes Ticket-Routing-Skript in die Architektur Ihres größeren Ticket-Routing-Systems passt. Es gibt zwei Möglichkeiten, wie Sie dies tun könnten:
* **Push-basiert:** Das Support-Ticket-System, das Sie verwenden (z.B. Zendesk), löst Ihren Code aus, indem es ein Webhook-Ereignis an Ihren Routing-Service sendet, der dann die Absicht klassifiziert und es weiterleitet.
    * Dieser Ansatz ist webskalierbarer, erfordert aber, dass Sie einen öffentlichen Endpunkt verfügbar machen.
* **Pull-basiert:** Ihr Code ruft die neuesten Tickets basierend auf einem bestimmten Zeitplan ab und leitet sie zum Zeitpunkt des Abrufs weiter.
    * Dieser Ansatz ist einfacher zu implementieren, könnte aber unnötige Aufrufe an das Support-Ticket-System machen, wenn die Abruffrequenz zu hoch ist, oder könnte übermäßig langsam sein, wenn die Abruffrequenz zu niedrig ist.

Für beide Ansätze müssen Sie Ihr Skript in einen Service einwickeln. Die Wahl des Ansatzes hängt davon ab, welche APIs Ihr Support-Ticketing-System bereitstellt.

***

<CardGroup cols={2}>
    <Card title="Classification cookbook" icon="link" href="https://github.com/anthropics/anthropic-cookbook/tree/main/capabilities/classification">
        Besuchen Sie unser Classification Cookbook für weitere Beispielcodes und detaillierte Eval-Anleitung.
    </Card>
    <Card title="Claude Console" icon="link" href="/dashboard">
        Beginnen Sie mit dem Aufbau und der Evaluierung Ihres Workflows auf der Claude Console.
    </Card>
</CardGroup>