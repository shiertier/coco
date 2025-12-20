# Kundenservice-Agent

Diese Anleitung zeigt, wie Sie Claudes fortgeschrittene Konversationsfähigkeiten nutzen können, um Kundenanfragen in Echtzeit zu bearbeiten, 24/7-Support bereitzustellen, Wartezeiten zu reduzieren und hohe Support-Volumen mit genauen Antworten und positiven Interaktionen zu bewältigen.

---

## Vor dem Aufbau mit Claude

### Entscheiden Sie, ob Sie Claude für Support-Chat verwenden möchten

Hier sind einige wichtige Indikatoren dafür, dass Sie ein LLM wie Claude einsetzen sollten, um Teile Ihres Kundenservice-Prozesses zu automatisieren:

  <section title="Hohes Volumen wiederholter Anfragen">

    Claude zeichnet sich durch die effiziente Bearbeitung einer großen Anzahl ähnlicher Fragen aus und gibt menschlichen Agenten mehr Zeit für komplexere Probleme.
  
</section>
  <section title="Bedarf an schneller Informationssynthese">

    Claude kann schnell Informationen aus umfangreichen Wissensdatenbanken abrufen, verarbeiten und kombinieren, während menschliche Agenten möglicherweise Zeit für Recherchen oder die Konsultation mehrerer Quellen benötigen.
  
</section>
  <section title="Anforderung für 24/7-Verfügbarkeit">

    Claude kann rund um die Uhr Support bieten, ohne zu ermüden, während die Besetzung menschlicher Agenten für kontinuierliche Abdeckung kostspielig und herausfordernd sein kann.
  
</section>
  <section title="Schnelle Skalierung während Spitzenzeiten">

    Claude kann plötzliche Anstiege im Anfragevolumen bewältigen, ohne dass neue Mitarbeiter eingestellt und geschult werden müssen.
  
</section>
  <section title="Konsistente Markenstimme">

    Sie können Claude anweisen, die Tonalität und Werte Ihrer Marke konsistent zu vertreten, während menschliche Agenten in ihren Kommunikationsstilen variieren können.
  
</section>

Einige Überlegungen zur Wahl von Claude gegenüber anderen LLMs:

- Sie priorisieren natürliche, nuancierte Konversation: Claudes ausgefeiltes Sprachverständnis ermöglicht natürlichere, kontextbewusste Gespräche, die sich menschlicher anfühlen als Chats mit anderen LLMs.
- Sie erhalten häufig komplexe und offene Anfragen: Claude kann eine breite Palette von Themen und Anfragen bearbeiten, ohne vorgefertigte Antworten zu generieren oder umfangreiche Programmierung von Variationen von Benutzeräußerungen zu erfordern.
- Sie benötigen skalierbare mehrsprachige Unterstützung: Claudes mehrsprachige Fähigkeiten ermöglichen es, Gespräche in über 200 Sprachen zu führen, ohne dass separate Chatbots oder umfangreiche Übersetzungsprozesse für jede unterstützte Sprache erforderlich sind.

### Definieren Sie Ihre ideale Chat-Interaktion

Skizzieren Sie eine ideale Kundeninteraktion, um zu definieren, wie und wann Sie erwarten, dass der Kunde mit Claude interagiert. Diese Skizze hilft bei der Bestimmung der technischen Anforderungen Ihrer Lösung.

Hier ist ein Beispiel für eine Chat-Interaktion für Kundenservice bei Autoversicherungen:

* **Kunde**: Initiiert Support-Chat-Erlebnis
   * **Claude**: Begrüßt Kunden herzlich und leitet Gespräch ein
* **Kunde**: Fragt nach Versicherung für sein neues Elektroauto
   * **Claude**: Bietet relevante Informationen zur Abdeckung von Elektrofahrzeugen
* **Kunde**: Stellt Fragen zu besonderen Anforderungen für Elektroauto-Versicherungen
   * **Claude**: Antwortet mit genauen und informativen Antworten und stellt Links zu den Quellen bereit
* **Kunde**: Stellt themenfremde Fragen, die nicht mit Versicherung oder Autos zu tun haben
   * **Claude**: Klärt auf, dass es keine themenfremden Themen diskutiert, und lenkt den Benutzer zurück zur Autoversicherung
* **Kunde**: Zeigt Interesse an einem Versicherungsangebot
   * **Claude**: Stellt eine Reihe von Fragen, um das passende Angebot zu bestimmen, und passt sich an die Antworten an
   * **Claude**: Sendet eine Anfrage zur Verwendung des Angebotsgenerator-API-Tools zusammen mit den vom Benutzer gesammelten notwendigen Informationen
   * **Claude**: Empfängt die Antworteninformationen vom API-Tool-Einsatz, synthetisiert die Informationen in eine natürliche Antwort und präsentiert das bereitgestellte Angebot dem Benutzer
* **Kunde**: Stellt Anschlussfragen
   * **Claude**: Beantwortet Anschlussfragen nach Bedarf
   * **Claude**: Leitet den Kunden zu den nächsten Schritten im Versicherungsprozess und beendet das Gespräch

<Tip>Im echten Beispiel, das Sie für Ihren eigenen Anwendungsfall schreiben, könnte es hilfreich sein, die tatsächlichen Wörter in dieser Interaktion aufzuschreiben, damit Sie auch ein Gefühl für den idealen Ton, die Antwortlänge und das Detaillierungsniveau bekommen, das Claude haben soll.</Tip>

### Unterteilen Sie die Interaktion in eindeutige Aufgaben

Kundenservice-Chat ist eine Sammlung mehrerer verschiedener Aufgaben, von Beantwortung von Fragen bis zur Informationsbeschaffung bis zur Durchführung von Anfragen, alles in einer einzigen Kundeninteraktion. Bevor Sie mit dem Aufbau beginnen, unterteilen Sie Ihre ideale Kundeninteraktion in jede Aufgabe, die Claude ausführen können soll. Dies stellt sicher, dass Sie Claude für jede Aufgabe auffordern und bewerten können, und gibt Ihnen ein gutes Verständnis für die Bandbreite der Interaktionen, die Sie bei der Erstellung von Testfällen berücksichtigen müssen.

<Tip>Kunden finden es manchmal hilfreich, dies als Interaktions-Flussdiagramm möglicher Gesprächswendepunkte je nach Benutzeranfragen zu visualisieren.</Tip>

Hier sind die wichtigsten Aufgaben, die mit der obigen Beispiel-Versicherungsinteraktion verbunden sind:

1. Begrüßung und allgemeine Anleitung
   - Begrüßen Sie den Kunden herzlich und leiten Sie das Gespräch ein
   - Bieten Sie allgemeine Informationen über das Unternehmen und die Interaktion

2. Produktinformationen
   - Bieten Sie Informationen zur Abdeckung von Elektrofahrzeugen
   <Note>Dies erfordert, dass Claude die notwendigen Informationen in seinem Kontext hat, und könnte bedeuten, dass eine [RAG-Integration](https://github.com/anthropics/anthropic-cookbook/blob/main/skills/retrieval_augmented_generation/guide.ipynb) erforderlich ist.</Note>
   - Beantworten Sie Fragen zu besonderen Anforderungen der Elektroauto-Versicherung
   - Beantworten Sie Anschlussfragen zum Angebot oder zu Versicherungsdetails
   - Bieten Sie Links zu Quellen an, wenn angemessen

3. Gesprächsverwaltung
   - Bleiben Sie beim Thema (Autoversicherung)
   - Leiten Sie themenfremde Fragen zurück zu relevanten Themen

4. Angebotsgenerierung
   - Stellen Sie angemessene Fragen, um die Angebotsberechtigung zu bestimmen
   - Passen Sie Fragen basierend auf Kundenantworten an
   - Übermitteln Sie gesammelte Informationen an das Angebotsgenerator-API
   - Präsentieren Sie das bereitgestellte Angebot dem Kunden

### Etablieren Sie Erfolgskriterien

Arbeiten Sie mit Ihrem Support-Team zusammen, um [klare Erfolgskriterien zu definieren](/docs/de/test-and-evaluate/define-success) und schreiben Sie [detaillierte Bewertungen](/docs/de/test-and-evaluate/develop-tests) mit messbaren Benchmarks und Zielen.

Hier sind Kriterien und Benchmarks, die verwendet werden können, um zu bewerten, wie erfolgreich Claude die definierten Aufgaben erfüllt:

  <section title="Genauigkeit des Abfrageverständnisses">

    Diese Metrik bewertet, wie genau Claude Kundenanfragen zu verschiedenen Themen versteht. Messen Sie dies, indem Sie eine Stichprobe von Gesprächen überprüfen und bewerten, ob Claude die richtige Interpretation der Kundenabsicht, kritische nächste Schritte, wie erfolgreiche Lösung aussieht, und mehr hat. Streben Sie eine Verständnisgenauigkeit von 95% oder höher an.
  
</section>
  <section title="Relevanz der Antwort">

    Dies bewertet, wie gut Claudes Antwort die spezifische Frage oder das Problem des Kunden beantwortet. Bewerten Sie eine Reihe von Gesprächen und bewerten Sie die Relevanz jeder Antwort (mit LLM-basierter Bewertung für Skalierbarkeit). Streben Sie eine Relevanzbewertung von 90% oder höher an.
  
</section>
  <section title="Genauigkeit der Antwort">

    Bewerten Sie die Richtigkeit allgemeiner Unternehmens- und Produktinformationen, die dem Benutzer bereitgestellt werden, basierend auf den Informationen, die Claude im Kontext bereitgestellt wurden. Streben Sie 100% Genauigkeit in diesen einleitenden Informationen an.
  
</section>
  <section title="Relevanz der Zitierbereitstellung">

    Verfolgen Sie die Häufigkeit und Relevanz von angebotenen Links oder Quellen. Streben Sie an, relevante Quellen in 80% der Interaktionen bereitzustellen, bei denen zusätzliche Informationen hilfreich sein könnten.
  
</section>
  <section title="Themeneinhaltung">

    Messen Sie, wie gut Claude beim Thema bleibt, wie z.B. das Thema Autoversicherung in unserer Beispielimplementierung. Streben Sie an, dass 95% der Antworten direkt mit Autoversicherung oder der spezifischen Anfrage des Kunden zusammenhängen.
  
</section>
  <section title="Effektivität der Inhaltsgenerierung">

    Messen Sie, wie erfolgreich Claude darin ist, zu bestimmen, wann Informationsinhalte generiert werden sollen, und wie relevant dieser Inhalt ist. Zum Beispiel würden wir in unserer Implementierung bestimmen, wie gut Claude versteht, wann ein Angebot generiert werden soll, und wie genau dieses Angebot ist. Streben Sie 100% Genauigkeit an, da dies für eine erfolgreiche Kundeninteraktion wichtig ist.
  
</section>
  <section title="Eskalationseffizienz">

    Dies misst Claudes Fähigkeit, zu erkennen, wann eine Anfrage menschliche Intervention benötigt, und angemessen zu eskalieren. Verfolgen Sie den Prozentsatz korrekt eskalierter Gespräche gegenüber denen, die hätten eskaliert werden sollen, aber nicht. Streben Sie eine Eskalationsgenauigkeit von 95% oder höher an.
  
</section>

Hier sind Kriterien und Benchmarks, die verwendet werden können, um die geschäftliche Auswirkung des Einsatzes von Claude für Support zu bewerten:

  <section title="Stimmungserhaltung">

    Dies bewertet Claudes Fähigkeit, die Kundenstimmung während des Gesprächs zu erhalten oder zu verbessern. Verwenden Sie Stimmungsanalysewerkzeuge, um die Stimmung am Anfang und Ende jedes Gesprächs zu messen. Streben Sie an, die Stimmung in 90% der Interaktionen zu erhalten oder zu verbessern.
  
</section>
  <section title="Deflektionsrate">

    Der Prozentsatz von Kundenanfragen, die erfolgreich vom Chatbot ohne menschliche Intervention bearbeitet werden. Normalerweise sollte eine Deflektionsrate von 70-80% angestrebt werden, je nach Komplexität der Anfragen.
  
</section>
  <section title="Kundenzufriedenheitsscore">

    Ein Maß dafür, wie zufrieden Kunden mit ihrer Chatbot-Interaktion sind. Normalerweise durch Umfragen nach der Interaktion durchgeführt. Streben Sie einen CSAT-Score von 4 von 5 oder höher an.
  
</section>
  <section title="Durchschnittliche Bearbeitungszeit">

    Die durchschnittliche Zeit, die der Chatbot benötigt, um eine Anfrage zu lösen. Dies variiert stark je nach Komplexität der Probleme, aber im Allgemeinen sollte eine niedrigere AHT im Vergleich zu menschlichen Agenten angestrebt werden.
  
</section>

## So implementieren Sie Claude als Kundenservice-Agent

### Wählen Sie das richtige Claude-Modell

Die Wahl des Modells hängt von den Kompromissen zwischen Kosten, Genauigkeit und Antwortzeit ab.

Für Kundenservice-Chat ist Claude Sonnet 4.5 gut geeignet, um Intelligenz, Latenz und Kosten auszugleichen. Für Fälle, in denen Sie einen Gesprächsfluss mit mehreren Prompts haben, einschließlich RAG, Tool-Einsatz und/oder Long-Context-Prompts, kann Claude Haiku 4.5 besser geeignet sein, um die Latenz zu optimieren.

### Erstellen Sie einen starken Prompt

Die Verwendung von Claude für Kundenservice erfordert, dass Claude genug Anleitung und Kontext hat, um angemessen zu reagieren, während er genug Flexibilität hat, um eine breite Palette von Kundenanfragen zu bearbeiten.

Beginnen wir mit dem Schreiben der Elemente eines starken Prompts, beginnend mit einem System-Prompt:

```python
IDENTITY = """You are Eva, a friendly and knowledgeable AI assistant for Acme Insurance 
Company. Your role is to warmly welcome customers and provide information on 
Acme's insurance offerings, which include car insurance and electric car 
insurance. You can also help customers get quotes for their insurance needs."""
```

<Tip>Während Sie möglicherweise versucht sind, alle Ihre Informationen in einen System-Prompt zu packen, um Anweisungen von der Benutzerkonversation zu trennen, funktioniert Claude tatsächlich am besten, wenn der Großteil des Prompt-Inhalts im ersten `User`-Turn geschrieben wird (mit der einzigen Ausnahme von Rollen-Prompting). Lesen Sie mehr unter [Giving Claude a role with a system prompt](/docs/de/build-with-claude/prompt-engineering/system-prompts).</Tip>

Es ist am besten, komplexe Prompts in Unterabschnitte zu unterteilen und jeweils einen Teil zu schreiben. Für jede Aufgabe könnten Sie größeren Erfolg haben, wenn Sie einem Schritt-für-Schritt-Prozess folgen, um die Teile des Prompts zu definieren, die Claude benötigt, um die Aufgabe gut zu erfüllen. Für dieses Beispiel des Kundenservice bei Autoversicherungen schreiben wir schrittweise alle Teile eines Prompts, beginnend mit der Aufgabe "Begrüßung und allgemeine Anleitung". Dies macht auch das Debuggen Ihres Prompts einfacher, da Sie einzelne Teile des Gesamtprompts schneller anpassen können.

Wir werden alle diese Teile in eine Datei namens `config.py` einfügen.

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

Wir werden dann dasselbe für unsere Informationen zur Autoversicherung und Elektroauto-Versicherung tun.

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

Jetzt, da wir unseren statischen Inhalt haben, fügen wir mindestens 4-5 Beispiele für "gute" Interaktionen hinzu, um Claudes Antworten zu leiten. Diese Beispiele sollten repräsentativ für Ihre ideale Kundeninteraktion sein und können Schutzmaßnahmen, Tool-Aufrufe usw. enthalten.

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

Sie werden auch wichtige Anweisungen einbeziehen möchten, die die Dos und Don'ts für die Interaktion von Claude mit dem Kunden beschreiben. Dies kann sich aus Marken-Schutzmaßnahmen oder Support-Richtlinien ergeben.

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

Jetzt kombinieren wir alle diese Abschnitte in einen einzelnen String, um ihn als unseren Prompt zu verwenden.

```python
TASK_SPECIFIC_INSTRUCTIONS = ' '.join([
   STATIC_GREETINGS_AND_GENERAL,
   STATIC_CAR_INSURANCE,
   STATIC_ELECTRIC_CAR_INSURANCE,
   EXAMPLES,
   ADDITIONAL_GUARDRAILS,
])
```

### Fügen Sie dynamische und agentengesteuerte Fähigkeiten mit Tool-Einsatz hinzu

Claude ist in der Lage, Maßnahmen zu ergreifen und Informationen dynamisch mithilfe der Client-seitigen Tool-Use-Funktionalität abzurufen. Beginnen Sie damit, alle externen Tools oder APIs aufzulisten, die der Prompt verwenden sollte.

Für dieses Beispiel beginnen wir mit einem Tool zur Berechnung des Angebots.

<Tip>Zur Erinnerung: Dieses Tool führt nicht die tatsächliche Berechnung durch, es signalisiert nur der Anwendung, dass ein Tool mit den angegebenen Argumenten verwendet werden sollte.</Tip>

Beispiel-Versicherungsangebots-Rechner:

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

### Stellen Sie Ihre Prompts bereit

Es ist schwierig zu wissen, wie gut Ihr Prompt funktioniert, ohne ihn in einer Test-Produktionsumgebung bereitzustellen und [Bewertungen durchzuführen](/docs/de/test-and-evaluate/develop-tests). Lassen Sie uns eine kleine Anwendung mit unserem Prompt, dem Anthropic SDK und Streamlit für eine Benutzeroberfläche erstellen.

In einer Datei namens `chatbot.py` beginnen Sie mit der Einrichtung der ChatBot-Klasse, die die Interaktionen mit dem Anthropic SDK kapselt.

Die Klasse sollte zwei Hauptmethoden haben: `generate_message` und `process_user_input`.

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

### Erstellen Sie Ihre Benutzeroberfläche

Testen Sie die Bereitstellung dieses Codes mit Streamlit mit einer Hauptmethode. Diese `main()`-Funktion richtet eine Streamlit-basierte Chat-Schnittstelle ein.

Wir werden dies in einer Datei namens `app.py` tun

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

Führen Sie das Programm aus mit:

```
streamlit run app.py
```

### Bewerten Sie Ihre Prompts

Prompting erfordert oft Tests und Optimierung, um produktionsreif zu sein. Um die Bereitschaft Ihrer Lösung zu bestimmen, bewerten Sie die Chatbot-Leistung mit einem systematischen Prozess, der quantitative und qualitative Methoden kombiniert. Das Erstellen einer [starken empirischen Bewertung](/docs/de/test-and-evaluate/develop-tests#building-evals-and-test-cases) basierend auf Ihren definierten Erfolgskriterien ermöglicht es Ihnen, Ihre Prompts zu optimieren.

<Tip>Die [Claude Console](/dashboard) verfügt jetzt über ein Evaluierungstool, mit dem Sie Ihre Prompts unter verschiedenen Szenarien testen können.</Tip>

### Verbessern Sie die Leistung

In komplexen Szenarien kann es hilfreich sein, zusätzliche Strategien zur Verbesserung der Leistung über Standard-[Prompt-Engineering-Techniken](/docs/de/build-with-claude/prompt-engineering/overview) und [Schutzmaßnahmen-Implementierungsstrategien](/docs/de/test-and-evaluate/strengthen-guardrails/reduce-hallucinations) hinaus zu erwägen. Hier sind einige häufige Szenarien:

#### Reduzieren Sie die Latenz des langen Kontexts mit RAG

Bei großen Mengen an statischem und dynamischem Kontext kann das Einbeziehen aller Informationen in den Prompt zu hohen Kosten, langsameren Antwortzeiten und dem Erreichen von Kontextfenster-Grenzen führen. In diesem Szenario können die Implementierung von Retrieval Augmented Generation (RAG)-Techniken die Leistung und Effizienz erheblich verbessern.

Durch die Verwendung von [Embedding-Modellen wie Voyage](/docs/de/build-with-claude/embeddings), um Informationen in Vektordarstellungen umzuwandeln, können Sie ein skalierbares und reaktionsschnelleres System erstellen. Dieser Ansatz ermöglicht das dynamische Abrufen relevanter Informationen basierend auf der aktuellen Abfrage, anstatt alle möglichen Kontexte in jeden Prompt einzubeziehen.

Die Implementierung von RAG für Support-Anwendungsfälle [RAG-Rezept](https://github.com/anthropics/anthropic-cookbook/blob/82675c124e1344639b2a875aa9d3ae854709cd83/skills/classification/guide.ipynb) hat sich gezeigt, dass sie die Genauigkeit erhöht, die Antwortzeiten reduziert und die API-Kosten in Systemen mit umfangreichen Kontextanforderungen senkt.

#### Integrieren Sie Echtzeitdaten mit Tool-Einsatz

Bei Abfragen, die Echtzeitinformationen erfordern, wie z.B. Kontostände oder Richtliniendetails, sind Embedding-basierte RAG-Ansätze nicht ausreichend. Stattdessen können Sie Tool-Einsatz nutzen, um die Fähigkeit Ihres Chatbots, genaue Echtzeitantworten bereitzustellen, erheblich zu verbessern. Zum Beispiel können Sie Tool-Einsatz verwenden, um Kundeninformationen nachzuschlagen, Bestelldetails abzurufen und Bestellungen im Namen des Kunden zu stornieren.

Dieser Ansatz, [dargelegt in unserem Tool-Einsatz: Kundenservice-Agent-Rezept](https://github.com/anthropics/anthropic-cookbook/blob/main/tool_use/customer_service_agent.ipynb), ermöglicht es Ihnen, Live-Daten nahtlos in Claudes Antworten zu integrieren und ein persönlicheres und effizienteres Kundenerlebnis bereitzustellen.

#### Stärken Sie Ein- und Ausgabeschutzmaßnahmen

Bei der Bereitstellung eines Chatbots, besonders in Kundenservice-Szenarien, ist es entscheidend, Risiken im Zusammenhang mit Missbrauch, Anfragen außerhalb des Geltungsbereichs und unangemessenen Antworten zu verhindern. Während Claude von Natur aus widerstandsfähig gegen solche Szenarien ist, finden Sie hier zusätzliche Schritte zur Stärkung Ihrer Chatbot-Schutzmaßnahmen:

- [Reduzieren Sie Halluzinationen](/docs/de/test-and-evaluate/strengthen-guardrails/reduce-hallucinations): Implementieren Sie Faktenprüfungsmechanismen und [Zitate](https://github.com/anthropics/anthropic-cookbook/blob/main/misc/using_citations.ipynb), um Antworten in bereitgestellten Informationen zu verankern.
- Überprüfen Sie Informationen: Überprüfen Sie, dass die Antworten des Agenten mit den Richtlinien und bekannten Fakten Ihres Unternehmens übereinstimmen.
- Vermeiden Sie vertragliche Verpflichtungen: Stellen Sie sicher, dass der Agent keine Versprechungen macht oder Vereinbarungen eingeht, zu denen er nicht berechtigt ist.
- [Mindern Sie Jailbreaks](/docs/de/test-and-evaluate/strengthen-guardrails/mitigate-jailbreaks): Verwenden Sie Methoden wie Harmlosigkeitsscreens und Eingabevalidierung, um zu verhindern, dass Benutzer Modellschwachstellen ausnutzen und unangemessene Inhalte generieren.
- Vermeiden Sie die Erwähnung von Konkurrenten: Implementieren Sie einen Konkurrenten-Erwähnungsfilter, um den Markenfokus zu bewahren und keine Produkte oder Dienstleistungen von Konkurrenten zu erwähnen.
- [Halten Sie Claude in Charakter](/docs/de/test-and-evaluate/strengthen-guardrails/keep-claude-in-character): Verhindern Sie, dass Claude seinen Stil oder Kontext ändert, auch während langer, komplexer Interaktionen.
- Entfernen Sie persönlich identifizierbare Informationen (PII): Entfernen Sie sofern nicht ausdrücklich erforderlich und autorisiert, alle PII aus Antworten.

#### Reduzieren Sie die wahrgenommene Antwortzeit mit Streaming

Bei potenziell langen Antworten kann die Implementierung von Streaming die Benutzereinbindung und Zufriedenheit erheblich verbessern. In diesem Szenario erhalten Benutzer die Antwort progressiv, anstatt auf die gesamte Antwortgenerierung zu warten.

So implementieren Sie Streaming:
1. Verwenden Sie die [Anthropic Streaming API](/docs/de/build-with-claude/streaming), um Streaming-Antworten zu unterstützen.
2. Richten Sie Ihr Frontend ein, um eingehende Textblöcke zu verarbeiten.
3. Zeigen Sie jeden Block an, wenn er ankommt, um echtzeitiges Tippen zu simulieren.
4. Implementieren Sie einen Mechanismus, um die vollständige Antwort zu speichern, damit Benutzer sie anzeigen können, wenn sie navigieren und zurückkehren.

In einigen Fällen ermöglicht Streaming die Verwendung von fortgeschritteneren Modellen mit höheren Basis-Latenzen, da die progressive Anzeige die Auswirkungen längerer Verarbeitungszeiten mindert.

#### Skalieren Sie Ihren Chatbot

Mit zunehmender Komplexität Ihres Chatbots kann sich Ihre Anwendungsarchitektur entsprechend entwickeln. Bevor Sie weitere Ebenen zu Ihrer Architektur hinzufügen, beachten Sie die folgenden weniger umfassenden Optionen:

- Stellen Sie sicher, dass Sie das Beste aus Ihren Prompts herausholen und durch Prompt-Engineering optimieren. Verwenden Sie unsere [Prompt-Engineering-Leitfäden](/docs/de/build-with-claude/prompt-engineering/overview), um die effektivsten Prompts zu schreiben.
- Fügen Sie zusätzliche [Tools](/docs/de/build-with-claude/tool-use) zum Prompt hinzu (die [Prompt-Ketten](/docs/de/build-with-claude/prompt-engineering/chain-prompts) enthalten können) und sehen Sie, ob Sie die erforderliche Funktionalität erreichen können.

Wenn Ihr Chatbot unglaublich vielfältige Aufgaben bearbeitet, möchten Sie möglicherweise einen [separaten Intent-Klassifizierer](https://github.com/anthropics/anthropic-cookbook/blob/main/skills/classification/guide.ipynb) hinzufügen, um die anfängliche Kundenanfrage weiterzuleiten. Für die bestehende Anwendung würde dies das Erstellen eines Entscheidungsbaums beinhalten, der Kundenanfragen durch den Klassifizierer leitet und dann zu spezialisierten Gesprächen (mit ihren eigenen Tools und System-Prompts). Beachten Sie, dass diese Methode einen zusätzlichen Aufruf an Claude erfordert, der die Latenz erhöhen kann.

### Integrieren Sie Claude in Ihren Support-Workflow

Während sich unsere Beispiele auf Python-Funktionen konzentriert haben, die in einer Streamlit-Umgebung aufrufbar sind, erfordert die Bereitstellung von Claude für einen echtzeitigen Support-Chatbot einen API-Service.

So können Sie vorgehen:

1. Erstellen Sie einen API-Wrapper: Entwickeln Sie einen einfachen API-Wrapper um Ihre Klassifizierungsfunktion. Zum Beispiel können Sie Flask API oder Fast API verwenden, um Ihren Code in einen HTTP-Service zu verpacken. Ihr HTTP-Service könnte die Benutzereingabe akzeptieren und die Assistenten-Antwort vollständig zurückgeben. Daher könnte Ihr Service die folgenden Merkmale haben:
   - Server-Sent Events (SSE): SSE ermöglicht das Streaming von Antworten vom Server zum Client in Echtzeit. Dies ist entscheidend für die Bereitstellung eines reibungslosen, interaktiven Erlebnisses bei der Arbeit mit LLMs.
   - Caching: Die Implementierung von Caching kann die Antwortzeiten erheblich verbessern und unnötige API-Aufrufe reduzieren.
   - Kontextbeibehaltung: Die Aufrechterhaltung des Kontexts, wenn ein Benutzer navigiert und zurückkehrt, ist wichtig für die Kontinuität in Gesprächen.

2. Erstellen Sie eine Web-Schnittstelle: Implementieren Sie eine benutzerfreundliche Web-UI für die Interaktion mit dem Claude-gesteuerten Agent.

<CardGroup cols={2}>
  <Card title="Retrieval Augmented Generation (RAG) cookbook" icon="link" href="https://github.com/anthropics/anthropic-cookbook/blob/main/skills/retrieval_augmented_generation/guide.ipynb">
    Besuchen Sie unser RAG-Cookbook-Rezept für weitere Beispielcodes und detaillierte Anleitung.
  </Card>
  <Card title="Citations cookbook" icon="link" href="https://github.com/anthropics/anthropic-cookbook/blob/main/misc/using_citations.ipynb">
    Erkunden Sie unser Citations-Cookbook-Rezept, um zu erfahren, wie Sie die Genauigkeit und Erklärbarkeit von Informationen gewährleisten.
  </Card>
</CardGroup>