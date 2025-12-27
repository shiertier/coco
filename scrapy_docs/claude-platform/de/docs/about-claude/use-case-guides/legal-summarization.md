# Zusammenfassung von Rechtsdokumenten

Diese Anleitung zeigt, wie Sie die fortschrittlichen Funktionen zur Verarbeitung natürlicher Sprache von Claude nutzen können, um Rechtsdokumente effizient zusammenzufassen, wichtige Informationen zu extrahieren und die Rechtsrecherche zu beschleunigen. Mit Claude können Sie die Überprüfung von Verträgen, die Vorbereitung von Rechtsstreitigkeiten und behördliche Arbeiten rationalisieren, Zeit sparen und Genauigkeit in Ihren Rechtsprozessen gewährleisten.

---

> Besuchen Sie unser [Zusammenfassungs-Cookbook](https://github.com/anthropics/anthropic-cookbook/blob/main/skills/summarization/guide.ipynb), um ein Beispiel einer Implementierung der Zusammenfassung von Rechtsdokumenten mit Claude zu sehen.

## Vor dem Aufbau mit Claude

### Entscheiden Sie, ob Sie Claude für die Zusammenfassung von Rechtsdokumenten verwenden möchten

Hier sind einige wichtige Indikatoren dafür, dass Sie ein LLM wie Claude zur Zusammenfassung von Rechtsdokumenten einsetzen sollten:

<section title="Sie möchten eine große Menge von Dokumenten effizient und kostengünstig überprüfen">
Die manuelle Überprüfung großer Dokumentmengen kann zeitaufwändig und teuer sein. Claude kann große Mengen an Rechtsdokumenten schnell verarbeiten und zusammenfassen, wodurch die Zeit und Kosten für die Dokumentenüberprüfung erheblich reduziert werden. Diese Fähigkeit ist besonders wertvoll für Aufgaben wie Due Diligence, Vertragsanalyse oder Rechtsstreit-Discovery, wo Effizienz entscheidend ist.
</section>
<section title="Sie benötigen die automatisierte Extraktion wichtiger Metadaten">
Claude kann wichtige Metadaten aus Rechtsdokumenten effizient extrahieren und kategorisieren, wie beteiligte Parteien, Daten, Vertragsbedingungen oder spezifische Klauseln. Diese automatisierte Extraktion kann dabei helfen, Informationen zu organisieren, wodurch es einfacher wird, große Dokumentmengen zu durchsuchen, zu analysieren und zu verwalten. Dies ist besonders nützlich für Vertragsverwaltung, Compliance-Prüfungen oder die Erstellung durchsuchbarer Datenbanken mit Rechtsinformationen.
</section>
<section title="Sie möchten klare, prägnante und standardisierte Zusammenfassungen erstellen">
Claude kann strukturierte Zusammenfassungen erstellen, die vorgegebene Formate befolgen, wodurch es für Rechtsanwälte einfacher wird, die wichtigsten Punkte verschiedener Dokumente schnell zu erfassen. Diese standardisierten Zusammenfassungen können die Lesbarkeit verbessern, den Vergleich zwischen Dokumenten erleichtern und das Gesamtverständnis verbessern, besonders wenn es um komplexe Rechtssprache oder technische Fachbegriffe geht.
</section>
<section title="Sie benötigen präzise Zitate für Ihre Zusammenfassungen">
Bei der Erstellung von Rechtszusammenfassungen sind ordnungsgemäße Zuschreibung und Zitierung entscheidend, um Glaubwürdigkeit und Einhaltung von Rechtsstandards zu gewährleisten. Claude kann so angewiesen werden, dass er genaue Zitate für alle referenzierten Rechtspunkte einfügt, wodurch es für Rechtsanwälte einfacher wird, die zusammengefassten Informationen zu überprüfen und zu verifizieren.
</section>
<section title="Sie möchten Ihren Rechtsrechercheprozess rationalisieren und beschleunigen">
Claude kann bei der Rechtsrecherche helfen, indem es schnell große Mengen an Rechtsprechung, Gesetzen und Rechtskommentaren analysiert. Es kann relevante Präzedenzfälle identifizieren, wichtige Rechtsprinzipien extrahieren und komplexe Rechtsargumente zusammenfassen. Diese Fähigkeit kann den Forschungsprozess erheblich beschleunigen und es Rechtsanwälten ermöglichen, sich auf höherwertige Analysen und Strategieentwicklung zu konzentrieren.
</section>

### Bestimmen Sie die Details, die die Zusammenfassung extrahieren soll
Es gibt keine einzige richtige Zusammenfassung für ein gegebenes Dokument. Ohne klare Anleitung kann es für Claude schwierig sein, zu bestimmen, welche Details einzubeziehen sind. Um optimale Ergebnisse zu erzielen, identifizieren Sie die spezifischen Informationen, die Sie in die Zusammenfassung aufnehmen möchten.

Wenn Sie beispielsweise eine Untermietvereinbarung zusammenfassen, möchten Sie möglicherweise die folgenden wichtigsten Punkte extrahieren:

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

### Etablieren Sie Erfolgskriterien

Die Bewertung der Qualität von Zusammenfassungen ist bekanntermaßen eine schwierige Aufgabe. Im Gegensatz zu vielen anderen Aufgaben der Verarbeitung natürlicher Sprache fehlen bei der Bewertung von Zusammenfassungen oft klare, objektive Metriken. Der Prozess kann sehr subjektiv sein, wobei verschiedene Leser verschiedene Aspekte einer Zusammenfassung schätzen. Hier sind Kriterien, die Sie möglicherweise berücksichtigen möchten, wenn Sie bewerten, wie gut Claude die Zusammenfassung von Rechtsdokumenten durchführt.

<section title="Sachliche Korrektheit">
Die Zusammenfassung sollte die Fakten, Rechtskonzepte und wichtigsten Punkte im Dokument genau darstellen.
</section>
<section title="Rechtliche Präzision">
Terminologie und Verweise auf Gesetze, Rechtsprechung oder Vorschriften müssen korrekt sein und mit Rechtsstandards übereinstimmen.
</section>
<section title="Prägnanz">
Die Zusammenfassung sollte das Rechtsdokument auf seine wesentlichen Punkte verdichten, ohne wichtige Details zu verlieren.
</section>
<section title="Konsistenz">
Bei der Zusammenfassung mehrerer Dokumente sollte das LLM eine konsistente Struktur und einen konsistenten Ansatz für jede Zusammenfassung beibehalten.
</section>
<section title="Lesbarkeit">
Der Text sollte klar und leicht verständlich sein. Wenn die Zielgruppe keine Rechtsexperten sind, sollte die Zusammenfassung keine Rechtsfachbegriffe enthalten, die die Zielgruppe verwirren könnten.
</section>
<section title="Voreingenommenheit und Fairness">
Die Zusammenfassung sollte eine unvoreingenommene und faire Darstellung der Rechtsargumente und Positionen bieten.
</section>

Weitere Informationen finden Sie in unserem Leitfaden zum [Festlegen von Erfolgskriterien](/docs/de/test-and-evaluate/define-success).

---

## Zusammenfassung von Rechtsdokumenten mit Claude

### Wählen Sie das richtige Claude-Modell

Die Modellgenauigkeit ist äußerst wichtig bei der Zusammenfassung von Rechtsdokumenten. Claude Sonnet 4.5 ist eine ausgezeichnete Wahl für Anwendungsfälle wie diesen, bei denen hohe Genauigkeit erforderlich ist. Wenn die Größe und Menge Ihrer Dokumente so groß ist, dass die Kosten zu einem Problem werden, können Sie auch ein kleineres Modell wie Claude Haiku 4.5 verwenden.

Um diese Kosten zu schätzen, finden Sie hier einen Vergleich der Kosten für die Zusammenfassung von 1.000 Untermietvereinbarungen mit Sonnet und Haiku:

* **Inhaltsgröße**
    * Anzahl der Vereinbarungen: 1.000
    * Zeichen pro Vereinbarung: 300.000
    * Gesamtzeichen: 300M

* **Geschätzte Token**
    * Input-Token: 86M (angenommen 1 Token pro 3,5 Zeichen)
    * Output-Token pro Zusammenfassung: 350
    * Gesamtausgabe-Token: 350.000
 
* **Claude Sonnet 4.5 geschätzte Kosten**
    * Input-Token-Kosten: 86 MTok * \$3,00/MTok = \$258
    * Output-Token-Kosten: 0,35 MTok * \$15,00/MTok = \$5,25
    * Gesamtkosten: \$258,00 + \$5,25 = \$263,25

* **Claude Haiku 3 geschätzte Kosten**
    * Input-Token-Kosten: 86 MTok * \$0,25/MTok = \$21,50
    * Output-Token-Kosten: 0,35 MTok * \$1,25/MTok = \$0,44
    * Gesamtkosten: \$21,50 + \$0,44 = \$21,96

<Tip>Die tatsächlichen Kosten können von diesen Schätzungen abweichen. Diese Schätzungen basieren auf dem Beispiel, das im Abschnitt zum [Prompting](#build-a-strong-prompt) hervorgehoben wird.</Tip>

### Transformieren Sie Dokumente in ein Format, das Claude verarbeiten kann

Bevor Sie mit der Zusammenfassung von Dokumenten beginnen, müssen Sie Ihre Daten vorbereiten. Dies umfasst das Extrahieren von Text aus PDFs, das Bereinigen des Textes und das Sicherstellen, dass er von Claude verarbeitet werden kann.

Hier ist eine Demonstration dieses Prozesses auf einer Beispiel-PDF:

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

In diesem Beispiel laden wir zunächst eine PDF einer Beispiel-Untermietvereinbarung herunter, die im [Zusammenfassungs-Cookbook](https://github.com/anthropics/anthropic-cookbook/blob/main/skills/summarization/data/Sample%20Sublease%20Agreement.pdf) verwendet wird. Diese Vereinbarung stammt aus einer öffentlich verfügbaren Untermietvereinbarung von der [sec.gov-Website](https://www.sec.gov/Archives/edgar/data/1045425/000119312507044370/dex1032.htm).

Wir verwenden die pypdf-Bibliothek, um den Inhalt der PDF zu extrahieren und in Text umzuwandeln. Die Textdaten werden dann bereinigt, indem zusätzliche Leerzeichen und Seitenzahlen entfernt werden.

### Erstellen Sie einen starken Prompt

Claude kann sich an verschiedene Zusammenfassungsstile anpassen. Sie können die Details des Prompts ändern, um Claude anzuleiten, mehr oder weniger ausführlich zu sein, mehr oder weniger technische Terminologie einzubeziehen oder eine höhere oder niedrigere Zusammenfassung des Kontexts bereitzustellen.

Hier ist ein Beispiel, wie Sie einen Prompt erstellen, der sicherstellt, dass die generierten Zusammenfassungen eine konsistente Struktur bei der Analyse von Untermietvereinbarungen befolgen:

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

Dieser Code implementiert eine `summarize_document`-Funktion, die Claude verwendet, um den Inhalt einer Untermietvereinbarung zusammenzufassen. Die Funktion akzeptiert eine Textzeichenkette und eine Liste von zu extrahierenden Details als Eingaben. In diesem Beispiel rufen wir die Funktion mit den Variablen `document_text` und `details_to_extract` auf, die in den vorherigen Code-Snippets definiert wurden.

Innerhalb der Funktion wird ein Prompt für Claude generiert, der das zusammenzufassende Dokument, die zu extrahierenden Details und spezifische Anweisungen zur Zusammenfassung des Dokuments enthält. Der Prompt weist Claude an, mit einer Zusammenfassung jedes zu extrahierenden Details zu antworten, das in XML-Headern verschachtelt ist.

Da wir uns entschieden haben, jeden Abschnitt der Zusammenfassung in Tags auszugeben, kann jeder Abschnitt leicht als Nachbearbeitungsschritt analysiert werden. Dieser Ansatz ermöglicht strukturierte Zusammenfassungen, die für Ihren Anwendungsfall angepasst werden können, sodass jede Zusammenfassung dem gleichen Muster folgt.

### Bewerten Sie Ihren Prompt

Das Prompting erfordert oft Tests und Optimierungen, um produktionsreif zu sein. Um die Bereitschaft Ihrer Lösung zu bestimmen, bewerten Sie die Qualität Ihrer Zusammenfassungen mit einem systematischen Prozess, der quantitative und qualitative Methoden kombiniert. Die Erstellung einer [starken empirischen Bewertung](/docs/de/test-and-evaluate/develop-tests#building-evals-and-test-cases) basierend auf Ihren definierten Erfolgskriterien ermöglicht es Ihnen, Ihre Prompts zu optimieren. Hier sind einige Metriken, die Sie möglicherweise in Ihre empirische Bewertung einbeziehen möchten:

<section title="ROUGE-Scores">
Dies misst die Überlappung zwischen der generierten Zusammenfassung und einer von Experten erstellten Referenzzusammenfassung. Diese Metrik konzentriert sich hauptsächlich auf Recall und ist nützlich zur Bewertung der Inhaltsabdeckung.
</section>
<section title="BLEU-Scores">
Obwohl ursprünglich für maschinelle Übersetzung entwickelt, kann diese Metrik für Zusammenfassungsaufgaben angepasst werden. BLEU-Scores messen die Präzision von N-Gramm-Übereinstimmungen zwischen der generierten Zusammenfassung und Referenzzusammenfassungen. Ein höherer Score zeigt an, dass die generierte Zusammenfassung ähnliche Phrasen und Terminologie wie die Referenzzusammenfassung enthält.
</section>
<section title="Ähnlichkeit von kontextuellem Embedding">
Diese Metrik beinhaltet die Erstellung von Vektordarstellungen (Embeddings) sowohl der generierten als auch der Referenzzusammenfassung. Die Ähnlichkeit zwischen diesen Embeddings wird dann berechnet, oft unter Verwendung von Kosinus-Ähnlichkeit. Höhere Ähnlichkeitswerte zeigen an, dass die generierte Zusammenfassung die semantische Bedeutung und den Kontext der Referenzzusammenfassung erfasst, auch wenn die genaue Formulierung unterschiedlich ist.
</section>
<section title="LLM-basierte Bewertung">
Diese Methode beinhaltet die Verwendung eines LLM wie Claude, um die Qualität generierter Zusammenfassungen gegen eine Bewertungsrubrik zu bewerten. Die Rubrik kann auf Ihre spezifischen Anforderungen zugeschnitten werden und bewertet Schlüsselfaktoren wie Genauigkeit, Vollständigkeit und Kohärenz. Anleitungen zur Implementierung von LLM-basierter Bewertung finden Sie in diesen [Tipps](/docs/de/test-and-evaluate/develop-tests#tips-for-llm-based-grading).
</section>
<section title="Menschliche Bewertung">
Zusätzlich zur Erstellung der Referenzzusammenfassungen können Rechtsexperten auch die Qualität der generierten Zusammenfassungen bewerten. Obwohl dies im großen Maßstab teuer und zeitaufwändig ist, wird dies oft bei einigen wenigen Zusammenfassungen als Plausibilitätsprüfung vor der Bereitstellung in der Produktion durchgeführt.
</section>

### Stellen Sie Ihren Prompt bereit

Hier sind einige zusätzliche Überlegungen, die Sie bei der Bereitstellung Ihrer Lösung in der Produktion beachten sollten.

1. **Stellen Sie sicher, dass keine Haftung besteht:** Verstehen Sie die rechtlichen Auswirkungen von Fehlern in den Zusammenfassungen, die zu rechtlicher Haftung für Ihre Organisation oder Ihre Kunden führen könnten. Geben Sie Haftungsausschlüsse oder rechtliche Hinweise ab, die klarstellen, dass die Zusammenfassungen von KI generiert werden und von Rechtsanwälten überprüft werden sollten.

2. **Behandeln Sie verschiedene Dokumenttypen:** In diesem Leitfaden haben wir besprochen, wie man Text aus PDFs extrahiert. In der Praxis können Dokumente in verschiedenen Formaten vorliegen (PDFs, Word-Dokumente, Textdateien usw.). Stellen Sie sicher, dass Ihre Datenextraktions-Pipeline alle Dateiformate konvertieren kann, die Sie erwarten zu erhalten.

3. **Parallelisieren Sie API-Aufrufe an Claude:** Lange Dokumente mit einer großen Anzahl von Token können bis zu eine Minute dauern, bis Claude eine Zusammenfassung generiert. Für große Dokumentsammlungen möchten Sie möglicherweise API-Aufrufe an Claude parallel senden, damit die Zusammenfassungen in einem angemessenen Zeitrahmen abgeschlossen werden können. Beachten Sie die [Rate Limits](/docs/de/api/rate-limits#rate-limits) von Anthropic, um die maximale Anzahl von API-Aufrufen zu bestimmen, die parallel durchgeführt werden können.

---

## Verbessern Sie die Leistung

In komplexen Szenarien kann es hilfreich sein, zusätzliche Strategien zu berücksichtigen, um die Leistung über standardmäßige [Prompt-Engineering-Techniken](/docs/de/build-with-claude/prompt-engineering/overview) hinaus zu verbessern. Hier sind einige fortgeschrittene Strategien:

### Führen Sie Meta-Zusammenfassung durch, um lange Dokumente zusammenzufassen

Die Zusammenfassung von Rechtsdokumenten beinhaltet oft die Behandlung langer Dokumente oder vieler verwandter Dokumente gleichzeitig, sodass Sie das Kontextfenster von Claude überschreiten. Sie können eine Chunking-Methode namens Meta-Zusammenfassung verwenden, um diesen Anwendungsfall zu behandeln. Diese Technik beinhaltet die Aufteilung von Dokumenten in kleinere, verwaltbare Chunks und die separate Verarbeitung jedes Chunks. Sie können dann die Zusammenfassungen jedes Chunks kombinieren, um eine Meta-Zusammenfassung des gesamten Dokuments zu erstellen.

Hier ist ein Beispiel, wie Sie Meta-Zusammenfassung durchführen:

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

Die `summarize_long_document`-Funktion baut auf der früheren `summarize_document`-Funktion auf, indem sie das Dokument in kleinere Chunks aufteilt und jeden Chunk einzeln zusammenfasst.

Der Code erreicht dies, indem die `summarize_document`-Funktion auf jeden Chunk von 20.000 Zeichen im ursprünglichen Dokument angewendet wird. Die einzelnen Zusammenfassungen werden dann kombiniert, und eine endgültige Zusammenfassung wird aus diesen Chunk-Zusammenfassungen erstellt.

Beachten Sie, dass die `summarize_long_document`-Funktion für unsere Beispiel-PDF nicht unbedingt erforderlich ist, da das gesamte Dokument in Claudes Kontextfenster passt. Sie wird jedoch für Dokumente, die Claudes Kontextfenster überschreiten, oder bei der Zusammenfassung mehrerer verwandter Dokumente zusammen unerlässlich. Unabhängig davon erfasst diese Meta-Zusammenfassungstechnik oft zusätzliche wichtige Details in der endgültigen Zusammenfassung, die beim früheren Single-Summary-Ansatz übersehen wurden.

### Verwenden Sie zusammengefasste indizierte Dokumente, um eine große Dokumentsammlung zu durchsuchen

Das Durchsuchen einer Dokumentsammlung mit einem LLM beinhaltet normalerweise Retrieval-Augmented Generation (RAG). In Szenarien mit großen Dokumenten oder wenn eine genaue Informationsbeschaffung entscheidend ist, kann ein grundlegender RAG-Ansatz jedoch unzureichend sein. Summary Indexed Documents ist ein fortgeschrittener RAG-Ansatz, der eine effizientere Möglichkeit bietet, Dokumente für den Abruf zu bewerten, wobei weniger Kontext als bei traditionellen RAG-Methoden verwendet wird. Bei diesem Ansatz generieren Sie zunächst mit Claude eine prägnante Zusammenfassung für jedes Dokument in Ihrem Corpus und verwenden dann Claude, um die Relevanz jeder Zusammenfassung für die gestellte Frage zu bewerten. Weitere Details zu diesem Ansatz, einschließlich eines Code-basierten Beispiels, finden Sie im Abschnitt Summary Indexed Documents im [Zusammenfassungs-Cookbook](https://github.com/anthropics/anthropic-cookbook/blob/main/skills/summarization/guide.ipynb).

### Fine-Tunen Sie Claude, um von Ihrem Datensatz zu lernen

Eine weitere fortgeschrittene Technik zur Verbesserung der Fähigkeit von Claude, Zusammenfassungen zu generieren, ist Fine-Tuning. Fine-Tuning beinhaltet das Training von Claude auf einem benutzerdefinierten Datensatz, der speziell auf Ihre Anforderungen zur Zusammenfassung von Rechtsdokumenten abgestimmt ist, um sicherzustellen, dass Claude sich an Ihren Anwendungsfall anpasst. Hier ist ein Überblick über die Durchführung von Fine-Tuning:

1. **Identifizieren Sie Fehler:** Beginnen Sie damit, Instanzen zu sammeln, in denen Claudes Zusammenfassungen zu kurz kommen – dies könnte das Übersehen kritischer Rechtsdetails, das Missverständnis von Kontext oder die Verwendung unangemessener Rechtsterminologie umfassen.

2. **Kuratieren Sie einen Datensatz:** Nachdem Sie diese Probleme identifiziert haben, stellen Sie einen Datensatz dieser problematischen Beispiele zusammen. Dieser Datensatz sollte die ursprünglichen Rechtsdokumente zusammen mit Ihren korrigierten Zusammenfassungen enthalten, um sicherzustellen, dass Claude das gewünschte Verhalten erlernt.

3. **Führen Sie Fine-Tuning durch:** Fine-Tuning beinhaltet das erneute Training des Modells auf Ihrem kuratierten Datensatz, um seine Gewichte und Parameter anzupassen. Dieses erneute Training hilft Claude, die spezifischen Anforderungen Ihrer Rechtsdomäne besser zu verstehen und verbessert seine Fähigkeit, Dokumente nach Ihren Standards zusammenzufassen.

4. **Iterative Verbesserung:** Fine-Tuning ist kein einmaliger Prozess. Während Claude weiterhin Zusammenfassungen generiert, können Sie iterativ neue Beispiele hinzufügen, bei denen es unterperformt hat, und seine Fähigkeiten weiter verfeinern. Im Laufe der Zeit wird diese kontinuierliche Feedback-Schleife zu einem Modell führen, das hochgradig spezialisiert auf Ihre Aufgaben zur Zusammenfassung von Rechtsdokumenten ist.

<Tip>Fine-Tuning ist derzeit nur über Amazon Bedrock verfügbar. Weitere Details finden Sie im [AWS-Startblog](https://aws.amazon.com/blogs/machine-learning/fine-tune-anthropics-claude-3-haiku-in-amazon-bedrock-to-boost-model-accuracy-and-quality/).</Tip>

<CardGroup cols={2}> 
  <Card title="Zusammenfassungs-Cookbook" icon="link" href="https://github.com/anthropics/anthropic-cookbook/blob/main/skills/summarization/guide.ipynb">
    Sehen Sie sich ein vollständig implementiertes Code-basiertes Beispiel an, wie Sie Claude zur Zusammenfassung von Verträgen verwenden.
  </Card>
  <Card title="Zitate-Cookbook" icon="link" href="https://github.com/anthropics/anthropic-cookbook/blob/main/misc/using_citations.ipynb">
    Erkunden Sie unser Zitate-Cookbook-Rezept für Anleitungen zur Gewährleistung von Genauigkeit und Erklärbarkeit von Informationen.
  </Card>
</CardGroup>