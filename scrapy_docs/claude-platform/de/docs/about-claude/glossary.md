# Glossar

Diese Konzepte sind nicht einzigartig für Anthropics Sprachmodelle, aber wir präsentieren unten eine kurze Zusammenfassung der wichtigsten Begriffe.

---

## Kontextfenster

Das "Kontextfenster" bezieht sich auf die Textmenge, auf die ein Sprachmodell zurückblicken und referenzieren kann, wenn es neuen Text generiert. Dies unterscheidet sich von dem großen Datenkorpus, auf dem das Sprachmodell trainiert wurde, und stellt stattdessen ein "Arbeitsgedächtnis" für das Modell dar. Ein größeres Kontextfenster ermöglicht es dem Modell, komplexere und längere Prompts zu verstehen und darauf zu antworten, während ein kleineres Kontextfenster die Fähigkeit des Modells einschränken kann, längere Prompts zu handhaben oder die Kohärenz über längere Gespräche aufrechtzuerhalten.

Siehe unseren [Leitfaden zum Verständnis von Kontextfenstern](/docs/de/build-with-claude/context-windows), um mehr zu erfahren.

## Fine-Tuning

Fine-Tuning ist der Prozess der weiteren Schulung eines vortrainierten Sprachmodells mit zusätzlichen Daten. Dies führt dazu, dass das Modell beginnt, die Muster und Eigenschaften des Fine-Tuning-Datensatzes zu repräsentieren und nachzuahmen. Claude ist kein reines Sprachmodell; es wurde bereits feinabgestimmt, um ein hilfreicher Assistent zu sein. Unsere API bietet derzeit kein Fine-Tuning an, aber bitte fragen Sie Ihren Anthropic-Kontakt, wenn Sie daran interessiert sind, diese Option zu erkunden. Fine-Tuning kann nützlich sein, um ein Sprachmodell an eine bestimmte Domäne, Aufgabe oder einen Schreibstil anzupassen, erfordert aber eine sorgfältige Betrachtung der Fine-Tuning-Daten und der potenziellen Auswirkungen auf die Leistung und Verzerrungen des Modells.

## HHH

Diese drei H's repräsentieren Anthropics Ziele bei der Sicherstellung, dass Claude für die Gesellschaft von Nutzen ist:

- Eine **hilfreiche** KI wird versuchen, die gestellte Aufgabe auszuführen oder die Frage nach besten Kräften zu beantworten und relevante und nützliche Informationen bereitzustellen.
- Eine **ehrliche** KI wird genaue Informationen geben und nicht halluzinieren oder konfabulieren. Sie wird ihre Grenzen und Unsicherheiten anerkennen, wenn es angebracht ist.
- Eine **harmlose** KI wird nicht beleidigend oder diskriminierend sein, und wenn sie gebeten wird, bei einer gefährlichen oder unethischen Handlung zu helfen, sollte die KI höflich ablehnen und erklären, warum sie nicht einwilligen kann.

## Latenz

Latenz bezieht sich im Kontext von generativer KI und großen Sprachmodellen auf die Zeit, die das Modell benötigt, um auf einen gegebenen Prompt zu antworten. Es ist die Verzögerung zwischen dem Einreichen eines Prompts und dem Erhalt der generierten Ausgabe. Niedrigere Latenz zeigt schnellere Antwortzeiten an, was für Echtzeitanwendungen, Chatbots und interaktive Erfahrungen entscheidend ist. Faktoren, die die Latenz beeinflussen können, umfassen Modellgröße, Hardware-Fähigkeiten, Netzwerkbedingungen und die Komplexität des Prompts und der generierten Antwort.

## LLM

Große Sprachmodelle (LLMs) sind KI-Sprachmodelle mit vielen Parametern, die in der Lage sind, eine Vielzahl überraschend nützlicher Aufgaben auszuführen. Diese Modelle werden auf riesigen Mengen von Textdaten trainiert und können menschenähnlichen Text generieren, Fragen beantworten, Informationen zusammenfassen und mehr. Claude ist ein konversationeller Assistent, der auf einem großen Sprachmodell basiert, das feinabgestimmt und mit RLHF trainiert wurde, um hilfreicher, ehrlicher und harmloser zu sein.

## MCP (Model Context Protocol)

Model Context Protocol (MCP) ist ein offenes Protokoll, das standardisiert, wie Anwendungen Kontext für LLMs bereitstellen. Wie ein USB-C-Anschluss für KI-Anwendungen bietet MCP eine einheitliche Möglichkeit, KI-Modelle mit verschiedenen Datenquellen und Tools zu verbinden. MCP ermöglicht es KI-Systemen, konsistenten Kontext über Interaktionen hinweg aufrechtzuerhalten und auf externe Ressourcen in standardisierter Weise zuzugreifen. Siehe unsere [MCP-Dokumentation](/docs/de/mcp), um mehr zu erfahren.

## MCP-Connector

Der MCP-Connector ist eine Funktion, die es API-Benutzern ermöglicht, sich direkt von der Messages API aus mit MCP-Servern zu verbinden, ohne einen MCP-Client zu erstellen. Dies ermöglicht eine nahtlose Integration mit MCP-kompatiblen Tools und Diensten über die Claude API. Der MCP-Connector unterstützt Funktionen wie Tool-Aufrufe und ist in der öffentlichen Beta verfügbar. Siehe unsere [MCP-Connector-Dokumentation](/docs/de/agents-and-tools/mcp-connector), um mehr zu erfahren.

## Pretraining

Pretraining ist der anfängliche Prozess der Schulung von Sprachmodellen auf einem großen ungelabelten Textkorpus. Im Fall von Claude werden autoregressive Sprachmodelle (wie Claudes zugrunde liegendes Modell) vortrainiert, um das nächste Wort vorherzusagen, gegeben den vorherigen Kontext des Textes im Dokument. Diese vortrainierten Modelle sind nicht von Natur aus gut darin, Fragen zu beantworten oder Anweisungen zu befolgen, und erfordern oft tiefe Fähigkeiten im Prompt Engineering, um gewünschte Verhaltensweisen hervorzurufen. Fine-Tuning und RLHF werden verwendet, um diese vortrainierten Modelle zu verfeinern und sie für eine breite Palette von Aufgaben nützlicher zu machen.

## RAG (Retrieval augmented generation)

Retrieval augmented generation (RAG) ist eine Technik, die Informationsabruf mit Sprachmodellgenerierung kombiniert, um die Genauigkeit und Relevanz des generierten Textes zu verbessern und die Antwort des Modells besser in Beweisen zu verankern. Bei RAG wird ein Sprachmodell mit einer externen Wissensbasis oder einem Satz von Dokumenten erweitert, die in das Kontextfenster übergeben werden. Die Daten werden zur Laufzeit abgerufen, wenn eine Anfrage an das Modell gesendet wird, obwohl das Modell selbst die Daten nicht unbedingt abruft (kann es aber mit [Tool-Verwendung](/docs/de/agents-and-tools/tool-use/overview) und einer Abruffunktion). Bei der Textgenerierung müssen relevante Informationen zuerst aus der Wissensbasis basierend auf dem Eingabeprompt abgerufen und dann zusammen mit der ursprünglichen Anfrage an das Modell weitergegeben werden. Das Modell verwendet diese Informationen, um die Ausgabe zu leiten, die es generiert. Dies ermöglicht es dem Modell, auf Informationen jenseits seiner Trainingsdaten zuzugreifen und sie zu nutzen, wodurch die Abhängigkeit von Auswendiglernen reduziert und die faktische Genauigkeit des generierten Textes verbessert wird. RAG kann besonders nützlich für Aufgaben sein, die aktuelle Informationen, domänenspezifisches Wissen oder explizite Zitation von Quellen erfordern. Die Wirksamkeit von RAG hängt jedoch von der Qualität und Relevanz der externen Wissensbasis und dem Wissen ab, das zur Laufzeit abgerufen wird.

## RLHF

Reinforcement Learning from Human Feedback (RLHF) ist eine Technik, die verwendet wird, um ein vortrainiertes Sprachmodell zu trainieren, sich auf Weise zu verhalten, die mit menschlichen Präferenzen übereinstimmt. Dies kann beinhalten, dem Modell zu helfen, Anweisungen effektiver zu befolgen oder mehr wie ein Chatbot zu agieren. Menschliches Feedback besteht aus der Bewertung einer Reihe von zwei oder mehr Beispieltexten, und der Verstärkungslernprozess ermutigt das Modell, Ausgaben zu bevorzugen, die den höher bewerteten ähnlich sind. Claude wurde mit RLHF trainiert, um ein hilfreicherer Assistent zu sein. Für weitere Details können Sie [Anthropics Papier zu diesem Thema](https://arxiv.org/abs/2204.05862) lesen.

## Temperature

Temperature ist ein Parameter, der die Zufälligkeit der Vorhersagen eines Modells während der Textgenerierung steuert. Höhere Temperaturen führen zu kreativeren und vielfältigeren Ausgaben, die mehrere Variationen in der Formulierung und, im Fall von Fiktion, Variation in Antworten ermöglichen. Niedrigere Temperaturen resultieren in konservativeren und deterministischeren Ausgaben, die bei der wahrscheinlichsten Formulierung und den wahrscheinlichsten Antworten bleiben. Die Anpassung der Temperature ermöglicht es Benutzern, ein Sprachmodell zu ermutigen, seltene, ungewöhnliche oder überraschende Wortauswahlen und Sequenzen zu erkunden, anstatt nur die wahrscheinlichsten Vorhersagen auszuwählen.

## TTFT (Time to first token)

Time to First Token (TTFT) ist eine Leistungsmetrik, die die Zeit misst, die ein Sprachmodell benötigt, um das erste Token seiner Ausgabe zu generieren, nachdem es einen Prompt erhalten hat. Es ist ein wichtiger Indikator für die Reaktionsfähigkeit des Modells und ist besonders relevant für interaktive Anwendungen, Chatbots und Echtzeitsysteme, wo Benutzer schnelles anfängliches Feedback erwarten. Ein niedrigerer TTFT zeigt an, dass das Modell schneller mit der Generierung einer Antwort beginnen kann, was eine nahtlosere und ansprechendere Benutzererfahrung bietet. Faktoren, die TTFT beeinflussen können, umfassen Modellgröße, Hardware-Fähigkeiten, Netzwerkbedingungen und die Komplexität des Prompts.

## Tokens

Tokens sind die kleinsten individuellen Einheiten eines Sprachmodells und können Wörtern, Teilwörtern, Zeichen oder sogar Bytes (im Fall von Unicode) entsprechen. Für Claude repräsentiert ein Token ungefähr 3,5 englische Zeichen, obwohl die genaue Anzahl je nach verwendeter Sprache variieren kann. Tokens sind typischerweise verborgen, wenn man mit Sprachmodellen auf der "Text"-Ebene interagiert, werden aber relevant, wenn man die genauen Eingaben und Ausgaben eines Sprachmodells untersucht. Wenn Claude Text zur Bewertung bereitgestellt wird, wird der Text (bestehend aus einer Reihe von Zeichen) in eine Reihe von Tokens für das Modell zur Verarbeitung kodiert. Größere Tokens ermöglichen Dateneffizienz während der Inferenz und des Pretrainings (und werden wenn möglich genutzt), während kleinere Tokens es einem Modell ermöglichen, ungewöhnliche oder nie zuvor gesehene Wörter zu handhaben. Die Wahl der Tokenisierungsmethode kann die Leistung des Modells, die Vokabulargröße und die Fähigkeit, mit Wörtern außerhalb des Vokabulars umzugehen, beeinflussen.