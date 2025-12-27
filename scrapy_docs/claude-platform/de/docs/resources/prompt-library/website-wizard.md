# Website-Assistent

Erstelle einseitige Websites basierend auf Benutzerspezifikationen.

---

> Kopiere diesen Prompt in unsere Entwickler-[Konsole](/dashboard), um es selbst auszuprobieren!

|        | Inhalt |
| --- | --- |
| System | Deine Aufgabe ist es, eine einseitige Website basierend auf den gegebenen Spezifikationen zu erstellen, die als HTML-Datei mit eingebettetem JavaScript und CSS geliefert wird. Die Website sollte eine Vielzahl von ansprechenden und interaktiven Designfunktionen enthalten, wie Drop-Down-Menüs, dynamischen Text und Inhalt, anklickbare Schaltflächen und mehr. Stelle sicher, dass das Design visuell ansprechend, responsiv und benutzerfreundlich ist. Der HTML-, CSS- und JavaScript-Code sollte gut strukturiert, effizient organisiert und ordnungsgemäß kommentiert sein für Lesbarkeit und Wartbarkeit. |
| User   | Erstelle eine einseitige Website für eine Online-Lernplattform namens "EduQuest" mit den folgenden Funktionen und Abschnitten:<br/><br/>1. Eine feste Navigationsleiste mit Links zu Kurskategorien (Mathematik, Naturwissenschaften, Sprachen, Kunst) und einer Suchleiste.<br/>2. Ein Hero-Bereich mit einem Video-Hintergrund, der Studenten beim Online-Lernen zeigt, einem dynamischen Slogan, der alle 3 Sekunden zwischen "Lerne in deinem eigenen Tempo", "Entdecke neue Leidenschaften" und "Erweitere deine Horizonte" wechselt, und einer "Loslegen"-Schaltfläche, die zu einem Kurskatalog führt.<br/>3. Ein Bereich für empfohlene Kurse, der Kurskarten mit Platzhaltern für Kursbilder, Titel, Dozenten und Beschreibungen anzeigt.<br/>4. Ein interaktiver "Lernpfade"-Bereich mit einem kurzen Quiz zur Bestimmung von Lernstilen und Interessen und einer Schaltfläche zum Starten des Quiz.<br/>5. Ein "Erfolgsgeschichten"-Bereich mit Testimonials von zufriedenen Studenten, mit Platzhaltern für den Testimonial-Text und Studentennamen.<br/>6. Eine Fußzeile mit Links zum Blog der Plattform, FAQ, Datenschutzrichtlinie und einer "Kontaktiere uns"-Schaltfläche, die ein modales Fenster mit einem Kontaktformular und Kundensupport-Informationen öffnet.<br/><br/>Füge Fülltext-Platzhalter für den Video-Hintergrund, Kurskarten und Testimonials hinzu. Bette die CSS-Stile innerhalb des `<style>`-Tags im `<head>`-Bereich ein und platziere den JavaScript-Code innerhalb des `<script>`-Tags am Ende des `<body>`-Bereichs.<br/><br/>Der JavaScript-Code sollte den dynamischen Slogan im Hero-Bereich handhaben und alle 3 Sekunden durch die verschiedenen Slogans rotieren. |

## Beispielausgabe

```html
<!DOCTYPE html>
<html lang="en">
<head>
    <meta charset="UTF-8">
    <meta name="viewport" content="width=device-width, initial-scale=1.0">
    <title>EduQuest - Online Learning Platform</title>
    <style>
        /* CSS Styles */
        body {
            font-family: Arial, sans-serif;
            margin: 0;
            padding: 0;
        }

        header {
            background-color: #333;
            color: #fff;
            padding: 20px;
            position: fixed;
            top: 0;
            left: 0;
            width: 100%;
            z-index: 999;
        }

        nav ul {
            list-style: none;
            margin: 0;
            padding: 0;
            display: flex;
            justify-content: space-between;
        }

        nav ul li a {
            color: #fff;
            text-decoration: none;
            padding: 10px;
        }

        .hero {
            height: 100vh;
            display: flex;
            flex-direction: column;
            justify-content: center;
            align-items: center;
            text-align: center;
            position: relative;
            overflow: hidden;
        }

        .hero video {
            position: absolute;
            top: 0;
            left: 0;
            width: 100%;
            height: 100%;
            object-fit: cover;
            z-index: -1;
        }

        .hero h1 {
            font-size: 48px;
            margin-bottom: 20px;
        }

        .hero p {
            font-size: 24px;
            margin-bottom: 40px;
        }

        .hero button {
            padding: 10px 20px;
            font-size: 18px;
            background-color: #333;
            color: #fff;
            border: none;
            cursor: pointer;
        }

        .featured-courses, .success-stories {
            padding: 40px;
        }

        .course-card {
            background-color: #f5f5f5;
            padding: 20px;
            margin-bottom: 20px;
        }

        .testimonial {
            background-color: #f5f5f5;
            padding: 20px;
            margin-bottom: 20px;
        }

        footer {
            background-color: #333;
            color: #fff;
            padding: 20px;
            text-align: center;
        }

        footer a {
            color: #fff;
            text-decoration: none;
            margin: 0 10px;
        }

        footer button {
            padding: 10px 20px;
            font-size: 18px;
            background-color: #fff;
            color: #333;
            border: none;
            cursor: pointer;
            margin-top: 20px;
        }
    </style>
</head>
<body>
    <header>
        <nav>
            <ul>
                <li><a href="#">Math</a></li>
                <li><a href="#">Science</a></li>
                <li><a href="#">Languages</a></li>
                <li><a href="#">Arts</a></li>
                <li><input type="text" placeholder="Search"></li>
            </ul>
        </nav>
    </header>

    <section class="hero">
        <video src="placeholder-video.mp4" autoplay loop muted></video>
        <h1>Welcome to EduQuest</h1>
        <p id="tagline">Learn at your own pace</p>
        <button>Get Started</button>
    </section>

    <section class="featured-courses">
        <h2>Featured Courses</h2>
        <div class="course-card">
            <img src="placeholder-course-image.jpg" alt="Course Image">
            <h3>Course Title</h3>
            <p>Instructor: John Doe</p>
            <p>Course description goes here.</p>
        </div>
        <div class="course-card">
            <img src="placeholder-course-image.jpg" alt="Course Image">
            <h3>Course Title</h3>
            <p>Instructor: Jane Smith</p>
            <p>Course description goes here.</p>
        </div>
    </section>

    <section class="success-stories">
        <h2>Success Stories</h2>
        <div class="testimonial">
            <p>"EduQuest helped me learn a new skill and advance my career. Highly recommended!"</p>
            <p>- John Doe</p>
        </div>
        <div class="testimonial">
            <p>"I never thought I could learn a new language online, but EduQuest made it possible. Thank you!"</p>
            <p>- Jane Smith</p>
        </div>
    </section>

    <footer>
        <a href="#">Blog</a>
        <a href="#">FAQ</a>
        <a href="#">Privacy Policy</a>
        <button>Contact Us</button>
    </footer>

    <script>
        // JavaScript Code
        const taglines = [
            "Learn at your own pace",
            "Discover new passions",
            "Expand your horizons"
        ];

        let index = 0;
        const taglineElement = document.getElementById("tagline");

        function rotateTagline() {
            taglineElement.textContent = taglines[index];
            index = (index + 1) % taglines.length;
        }

        setInterval(rotateTagline, 3000);
    </script>
</body>
</html>
```

> Dieser Code erstellt eine einseitige Website für "EduQuest" mit den folgenden Funktionen und Abschnitten:
> 
> Eine feste Navigationsleiste mit Links zu Kurskategorien (Mathematik, Naturwissenschaften, Sprachen, Kunst) und einer Suchleiste.
> 
> Ein Hero-Bereich mit einem Platzhalter-Video-Hintergrund, einem dynamischen Slogan, der alle 3 Sekunden zwischen "Lerne in deinem eigenen Tempo", "Entdecke neue Leidenschaften" und "Erweitere deine Horizonte" wechselt, und einer "Loslegen"-Schaltfläche.
> 
> Ein Bereich für empfohlene Kurse, der Kurskarten mit Platzhaltern für Kursbilder, Titel, Dozenten und Beschreibungen anzeigt.
> 
> Ein "Erfolgsgeschichten"-Bereich mit Testimonials von zufriedenen Studenten, mit Platzhaltern für den Testimonial-Text und Studentennamen.
> 
> Eine Fußzeile mit Links zum Blog der Plattform, FAQ, Datenschutzrichtlinie und einer "Kontaktiere uns"-Schaltfläche.
> 
> Die CSS-Stile sind innerhalb des `<style>`-Tags im `<head>`-Bereich eingebettet, und der JavaScript-Code ist innerhalb des `<script>`-Tags am Ende des `<body>`-Bereichs platziert.
> 
> Der JavaScript-Code handhabt den dynamischen Slogan im Hero-Bereich und rotiert alle 3 Sekunden durch die verschiedenen Slogans mit der setInterval-Funktion.
> 
> Bitte beachte, dass der Video-Hintergrund, die Kursbilder und die Testimonials Platzhalter sind und durch tatsächlichen Inhalt ersetzt werden sollten.

## API-Anfrage

<Tabs>
<Tab title="Python">

```python
import anthropic

client = anthropic.Anthropic( # defaults to os.environ.get("ANTHROPIC_API_KEY")
api_key="my_api_key",
)
message = client.messages.create(
  model="claude-sonnet-4-5",
  max_tokens=4000,
  temperature=0,
  system="Deine Aufgabe ist es, eine einseitige Website basierend auf den gegebenen Spezifikationen zu erstellen, die als HTML-Datei mit eingebettetem JavaScript und CSS geliefert wird. Die Website sollte eine Vielzahl von ansprechenden und interaktiven Designfunktionen enthalten, wie Drop-Down-Menüs, dynamischen Text und Inhalt, anklickbare Schaltflächen und mehr. Stelle sicher, dass das Design visuell ansprechend, responsiv und benutzerfreundlich ist. Der HTML-, CSS- und JavaScript-Code sollte gut strukturiert, effizient organisiert und ordnungsgemäß kommentiert sein für Lesbarkeit und Wartbarkeit.",
  messages=[
    {
      "role": "user",
      "content": [
        {
          "type": "text",
          "text": "Erstelle eine einseitige Website für eine Online-Lernplattform namens \"EduQuest\" mit den folgenden Funktionen und Abschnitten: \n \n1. Eine feste Navigationsleiste mit Links zu Kurskategorien (Mathematik, Naturwissenschaften, Sprachen, Kunst) und einer Suchleiste. \n \n2. Ein Hero-Bereich mit einem Video-Hintergrund, der Studenten beim Online-Lernen zeigt, einem dynamischen Slogan, der alle 3 Sekunden zwischen \"Lerne in deinem eigenen Tempo\", \"Entdecke neue Leidenschaften\" und \"Erweitere deine Horizonte\" wechselt, und einer \"Loslegen\"-Schaltfläche, die zu einem Kurskatalog führt. \n \n3. Ein Bereich für empfohlene Kurse, der Kurskarten mit Platzhaltern für Kursbilder, Titel, Dozenten und Beschreibungen anzeigt. \n \n4. Ein interaktiver \"Lernpfade\"-Bereich mit einem kurzen Quiz zur Bestimmung von Lernstilen und Interessen und einer Schaltfläche zum Starten des Quiz. \n \n5. Ein \"Erfolgsgeschichten\"-Bereich mit Testimonials von zufriedenen Studenten, mit Platzhaltern für den Testimonial-Text und Studentennamen. \n \n6. Eine Fußzeile mit Links zum Blog der Plattform, FAQ, Datenschutzrichtlinie und einer \"Kontaktiere uns\"-Schaltfläche, die ein modales Fenster mit einem Kontaktformular und Kundensupport-Informationen öffnet. \n \nFüge Fülltext-Platzhalter für den Video-Hintergrund, Kurskarten und Testimonials hinzu. Bette die CSS-Stile innerhalb des `<style>`-Tags im `<head>`-Bereich ein und platziere den JavaScript-Code innerhalb des `<script>`-Tags am Ende des `<body>`-Bereichs. \n \nDer JavaScript-Code sollte den dynamischen Slogan im Hero-Bereich handhaben und alle 3 Sekunden durch die verschiedenen Slogans rotieren."
        }
      ]
    }
  ]
)
print(message.content)

```

</Tab>

<Tab title="TypeScript">
```typescript
import Anthropic from "@anthropic-ai/sdk";

const anthropic = new Anthropic({
  apiKey: "my_api_key", // defaults to process.env["ANTHROPIC_API_KEY"]
});

const msg = await anthropic.messages.create({
  model: "claude-sonnet-4-5",
  max_tokens: 4000,
  temperature: 0,
  system: "Deine Aufgabe ist es, eine einseitige Website basierend auf den gegebenen Spezifikationen zu erstellen, die als HTML-Datei mit eingebettetem JavaScript und CSS geliefert wird. Die Website sollte eine Vielzahl von ansprechenden und interaktiven Designfunktionen enthalten, wie Drop-Down-Menüs, dynamischen Text und Inhalt, anklickbare Schaltflächen und mehr. Stelle sicher, dass das Design visuell ansprechend, responsiv und benutzerfreundlich ist. Der HTML-, CSS- und JavaScript-Code sollte gut strukturiert, effizient organisiert und ordnungsgemäß kommentiert sein für Lesbarkeit und Wartbarkeit.",
  messages: [
    {
      "role": "user",
      "content": [
        {
          "type": "text",
          "text": "Erstelle eine einseitige Website für eine Online-Lernplattform namens \"EduQuest\" mit den folgenden Funktionen und Abschnitten:  \n  \n1. Eine feste Navigationsleiste mit Links zu Kurskategorien (Mathematik, Naturwissenschaften, Sprachen, Kunst) und einer Suchleiste.  \n  \n2. Ein Hero-Bereich mit einem Video-Hintergrund, der Studenten beim Online-Lernen zeigt, einem dynamischen Slogan, der alle 3 Sekunden zwischen \"Lerne in deinem eigenen Tempo\", \"Entdecke neue Leidenschaften\" und \"Erweitere deine Horizonte\" wechselt, und einer \"Loslegen\"-Schaltfläche, die zu einem Kurskatalog führt.  \n  \n3. Ein Bereich für empfohlene Kurse, der Kurskarten mit Platzhaltern für Kursbilder, Titel, Dozenten und Beschreibungen anzeigt.  \n  \n4. Ein interaktiver \"Lernpfade\"-Bereich mit einem kurzen Quiz zur Bestimmung von Lernstilen und Interessen und einer Schaltfläche zum Starten des Quiz.  \n  \n5. Ein \"Erfolgsgeschichten\"-Bereich mit Testimonials von zufriedenen Studenten, mit Platzhaltern für den Testimonial-Text und Studentennamen.  \n  \n6. Eine Fußzeile mit Links zum Blog der Plattform, FAQ, Datenschutzrichtlinie und einer \"Kontaktiere uns\"-Schaltfläche, die ein modales Fenster mit einem Kontaktformular und Kundensupport-Informationen öffnet.  \n  \nFüge Fülltext-Platzhalter für den Video-Hintergrund, Kurskarten und Testimonials hinzu. Bette die CSS-Stile innerhalb des `<style>`-Tags im `<head>`-Bereich ein und platziere den JavaScript-Code innerhalb des `<script>`-Tags am Ende des `<body>`-Bereichs.  \n  \nDer JavaScript-Code sollte den dynamischen Slogan im Hero-Bereich handhaben und alle 3 Sekunden durch die verschiedenen Slogans rotieren."
        }
      ]
    }
  ]
});
console.log(msg);

```

</Tab>
<Tab title="AWS Bedrock Python">

```python
from anthropic import AnthropicBedrock

# See https://docs.claude.com/claude/reference/claude-on-amazon-bedrock

# for authentication options

client = AnthropicBedrock()

message = client.messages.create(
  model="anthropic.claude-sonnet-4-5-20250929-v1:0",
  max_tokens=4000,
  temperature=0,
  system="Deine Aufgabe ist es, eine einseitige Website basierend auf den gegebenen Spezifikationen zu erstellen, die als HTML-Datei mit eingebettetem JavaScript und CSS geliefert wird. Die Website sollte eine Vielzahl von ansprechenden und interaktiven Designfunktionen enthalten, wie Drop-Down-Menüs, dynamischen Text und Inhalt, anklickbare Schaltflächen und mehr. Stelle sicher, dass das Design visuell ansprechend, responsiv und benutzerfreundlich ist. Der HTML-, CSS- und JavaScript-Code sollte gut strukturiert, effizient organisiert und ordnungsgemäß kommentiert sein für Lesbarkeit und Wartbarkeit.",
  messages=[
    {
      "role": "user",
      "content": [
        {
          "type": "text",
          "text": "Erstelle eine einseitige Website für eine Online-Lernplattform namens \"EduQuest\" mit den folgenden Funktionen und Abschnitten: \n \n1. Eine feste Navigationsleiste mit Links zu Kurskategorien (Mathematik, Naturwissenschaften, Sprachen, Kunst) und einer Suchleiste. \n \n2. Ein Hero-Bereich mit einem Video-Hintergrund, der Studenten beim Online-Lernen zeigt, einem dynamischen Slogan, der alle 3 Sekunden zwischen \"Lerne in deinem eigenen Tempo\", \"Entdecke neue Leidenschaften\" und \"Erweitere deine Horizonte\" wechselt, und einer \"Loslegen\"-Schaltfläche, die zu einem Kurskatalog führt. \n \n3. Ein Bereich für empfohlene Kurse, der Kurskarten mit Platzhaltern für Kursbilder, Titel, Dozenten und Beschreibungen anzeigt. \n \n4. Ein interaktiver \"Lernpfade\"-Bereich mit einem kurzen Quiz zur Bestimmung von Lernstilen und Interessen und einer Schaltfläche zum Starten des Quiz. \n \n5. Ein \"Erfolgsgeschichten\"-Bereich mit Testimonials von zufriedenen Studenten, mit Platzhaltern für den Testimonial-Text und Studentennamen. \n \n6. Eine Fußzeile mit Links zum Blog der Plattform, FAQ, Datenschutzrichtlinie und einer \"Kontaktiere uns\"-Schaltfläche, die ein modales Fenster mit einem Kontaktformular und Kundensupport-Informationen öffnet. \n \nFüge Fülltext-Platzhalter für den Video-Hintergrund, Kurskarten und Testimonials hinzu. Bette die CSS-Stile innerhalb des `<style>`-Tags im `<head>`-Bereich ein und platziere den JavaScript-Code innerhalb des `<script>`-Tags am Ende des `<body>`-Bereichs. \n \nDer JavaScript-Code sollte den dynamischen Slogan im Hero-Bereich handhaben und alle 3 Sekunden durch die verschiedenen Slogans rotieren."
        }
      ]
    }
  ]
)
print(message.content)

```

</Tab>
<Tab title="AWS Bedrock TypeScript">

```typescript

import AnthropicBedrock from '@anthropic-ai/bedrock-sdk';

// See https://docs.claude.com/claude/reference/claude-on-amazon-bedrock
// for authentication options
const client = new AnthropicBedrock();

const msg = await client.messages.create({
  model: "anthropic.claude-sonnet-4-5-20250929-v1:0",
  max_tokens: 4000,
  temperature: 0,
  system: "Deine Aufgabe ist es, eine einseitige Website basierend auf den gegebenen Spezifikationen zu erstellen, die als HTML-Datei mit eingebettetem JavaScript und CSS geliefert wird. Die Website sollte eine Vielzahl von ansprechenden und interaktiven Designfunktionen enthalten, wie Drop-Down-Menüs, dynamischen Text und Inhalt, anklickbare Schaltflächen und mehr. Stelle sicher, dass das Design visuell ansprechend, responsiv und benutzerfreundlich ist. Der HTML-, CSS- und JavaScript-Code sollte gut strukturiert, effizient organisiert und ordnungsgemäß kommentiert sein für Lesbarkeit und Wartbarkeit.",
  messages: [
    {
      "role": "user",
      
      "content": [
        {
          "type": "text",
          "text": "Erstelle eine einseitige Website für eine Online-Lernplattform namens \"EduQuest\" mit den folgenden Funktionen und Abschnitten: \n \n1. Eine feste Navigationsleiste mit Links zu Kurskategorien (Mathematik, Naturwissenschaften, Sprachen, Kunst) und einer Suchleiste. \n \n2. Ein Hero-Bereich mit einem Video-Hintergrund, der Studenten beim Online-Lernen zeigt, einem dynamischen Slogan, der alle 3 Sekunden zwischen \"Lerne in deinem eigenen Tempo\", \"Entdecke neue Leidenschaften\" und \"Erweitere deine Horizonte\" wechselt, und einer \"Loslegen\"-Schaltfläche, die zu einem Kurskatalog führt. \n \n3. Ein Bereich für empfohlene Kurse, der Kurskarten mit Platzhaltern für Kursbilder, Titel, Dozenten und Beschreibungen anzeigt. \n \n4. Ein interaktiver \"Lernpfade\"-Bereich mit einem kurzen Quiz zur Bestimmung von Lernstilen und Interessen und einer Schaltfläche zum Starten des Quiz. \n \n5. Ein \"Erfolgsgeschichten\"-Bereich mit Testimonials von zufriedenen Studenten, mit Platzhaltern für den Testimonial-Text und Studentennamen. \n \n6. Eine Fußzeile mit Links zum Blog der Plattform, FAQ, Datenschutzrichtlinie und einer \"Kontaktiere uns\"-Schaltfläche, die ein modales Fenster mit einem Kontaktformular und Kundensupport-Informationen öffnet. \n \nFüge Fülltext-Platzhalter für den Video-Hintergrund, Kurskarten und Testimonials hinzu. Bette die CSS-Stile innerhalb des `<style>`-Tags im `<head>`-Bereich ein und platziere den JavaScript-Code innerhalb des `<script>`-Tags am Ende des `<body>`-Bereichs. \n \nDer JavaScript-Code sollte den dynamischen Slogan im Hero-Bereich handhaben und alle 3 Sekunden durch die verschiedenen Slogans rotieren."
        }
      ]
    }
  ]
});
console.log(msg);

```
</Tab>

<Tab title="Vertex AI Python">

```python
from anthropic import AnthropicVertex

client = AnthropicVertex()

message = client.messages.create(
  model="claude-sonnet-4@20250514",
  max_tokens=4000,
  temperature=0,
  system="Deine Aufgabe ist es, eine einseitige Website basierend auf den gegebenen Spezifikationen zu erstellen, die als HTML-Datei mit eingebettetem JavaScript und CSS geliefert wird. Die Website sollte eine Vielzahl von ansprechenden und interaktiven Designfunktionen enthalten, wie Drop-Down-Menüs, dynamischen Text und Inhalt, anklickbare Schaltflächen und mehr. Stelle sicher, dass das Design visuell ansprechend, responsiv und benutzerfreundlich ist. Der HTML-, CSS- und JavaScript-Code sollte gut strukturiert, effizient organisiert und ordnungsgemäß kommentiert sein für Lesbarkeit und Wartbarkeit.",
  messages=[
    {
      "role": "user",
      "content": [
        {
          "type": "text",
          "text": "Erstelle eine einseitige Website für eine Online-Lernplattform namens \"EduQuest\" mit den folgenden Funktionen und Abschnitten: \n \n1. Eine feste Navigationsleiste mit Links zu Kurskategorien (Mathematik, Naturwissenschaften, Sprachen, Kunst) und einer Suchleiste. \n \n2. Ein Hero-Bereich mit einem Video-Hintergrund, der Studenten beim Online-Lernen zeigt, einem dynamischen Slogan, der alle 3 Sekunden zwischen \"Lerne in deinem eigenen Tempo\", \"Entdecke neue Leidenschaften\" und \"Erweitere deine Horizonte\" wechselt, und einer \"Loslegen\"-Schaltfläche, die zu einem Kurskatalog führt. \n \n3. Ein Bereich für empfohlene Kurse, der Kurskarten mit Platzhaltern für Kursbilder, Titel, Dozenten und Beschreibungen anzeigt. \n \n4. Ein interaktiver \"Lernpfade\"-Bereich mit einem kurzen Quiz zur Bestimmung von Lernstilen und Interessen und einer Schaltfläche zum Starten des Quiz. \n \n5. Ein \"Erfolgsgeschichten\"-Bereich mit Testimonials von zufriedenen Studenten, mit Platzhaltern für den Testimonial-Text und Studentennamen. \n \n6. Eine Fußzeile mit Links zum Blog der Plattform, FAQ, Datenschutzrichtlinie und einer \"Kontaktiere uns\"-Schaltfläche, die ein modales Fenster mit einem Kontaktformular und Kundensupport-Informationen öffnet. \n \nFüge Fülltext-Platzhalter für den Video-Hintergrund, Kurskarten und Testimonials hinzu. Bette die CSS-Stile innerhalb des `<style>`-Tags im `<head>`-Bereich ein und platziere den JavaScript-Code innerhalb des `<script>`-Tags am Ende des `<body>`-Bereichs. \n \nDer JavaScript-Code sollte den dynamischen Slogan im Hero-Bereich handhaben und alle 3 Sekunden durch die verschiedenen Slogans rotieren."
        }
      ]
    }
  ]
)
print(message.content)

```
</Tab>
<Tab title="Vertex AI TypeScript">

```typescript 
import { AnthropicVertex } from '@anthropic-ai/vertex-sdk';

// Reads from the `CLOUD_ML_REGION` & `ANTHROPIC_VERTEX_PROJECT_ID` environment variables.
// Additionally goes through the standard `google-auth-library` flow.
const client = new AnthropicVertex();

const msg = await client.messages.create({
model: "claude-sonnet-4@20250514",
max_tokens: 4000,
temperature: 0,
system: "Deine Aufgabe ist es, eine einseitige Website basierend auf den gegebenen Spezifikationen zu erstellen, die als HTML-Datei mit eingebettetem JavaScript und CSS geliefert wird. Die Website sollte eine Vielzahl von ansprechenden und interaktiven Designfunktionen enthalten, wie Drop-Down-Menüs, dynamischen Text und Inhalt, anklickbare Schaltflächen und mehr. Stelle sicher, dass das Design visuell ansprechend, responsiv und benutzerfreundlich ist. Der HTML-, CSS- und JavaScript-Code sollte gut strukturiert, effizient organisiert und ordnungsgemäß kommentiert sein für Lesbarkeit und Wartbarkeit.",
messages: [
{
"role": "user",
"content": [
{
"type": "text",
"text": "Erstelle eine einseitige Website für eine Online-Lernplattform namens \"EduQuest\" mit den folgenden Funktionen und Abschnitten: \n \n1. Eine feste Navigationsleiste mit Links zu Kurskategorien (Mathematik, Naturwissenschaften, Sprachen, Kunst) und einer Suchleiste. \n \n2. Ein Hero-Bereich mit einem Video-Hintergrund, der Studenten beim Online-Lernen zeigt, einem dynamischen Slogan, der alle 3 Sekunden zwischen \"Lerne in deinem eigenen Tempo\", \"Entdecke neue Leidenschaften\" und \"Erweitere deine Horizonte\" wechselt, und einer \"Loslegen\"-Schaltfläche, die zu einem Kurskatalog führt. \n \n3. Ein Bereich für empfohlene Kurse, der Kurskarten mit Platzhaltern für Kursbilder, Titel, Dozenten und Beschreibungen anzeigt. \n \n4. Ein interaktiver \"Lernpfade\"-Bereich mit einem kurzen Quiz zur Bestimmung von Lernstilen und Interessen und einer Schaltfläche zum Starten des Quiz. \n \n5. Ein \"Erfolgsgeschichten\"-Bereich mit Testimonials von zufriedenen Studenten, mit Platzhaltern für den Testimonial-Text und Studentennamen. \n \n6. Eine Fußzeile mit Links zum Blog der Plattform, FAQ, Datenschutzrichtlinie und einer \"Kontaktiere uns\"-Schaltfläche, die ein modales Fenster mit einem Kontaktformular und Kundensupport-Informationen öffnet. \n \nFüge Fülltext-Platzhalter für den Video-Hintergrund, Kurskarten und Testimonials hinzu. Bette die CSS-Stile innerhalb des `<style>`-Tags im `<head>`-Bereich ein und platziere den JavaScript-Code innerhalb des `<script>`-Tags am Ende des `<body>`-Bereichs. \n \nDer JavaScript-Code sollte den dynamischen Slogan im Hero-Bereich handhaben und alle 3 Sekunden durch die verschiedenen Slogans rotieren."
}
]
}
]
});
console.log(msg);

```

</Tab>
</Tabs>