# Procedura guidata per siti web

Crea siti web di una sola pagina basati sulle specifiche dell'utente.

---

> Copia questo prompt nella nostra [Console](/dashboard) per sviluppatori per provarlo tu stesso!

|        | Contenuto |
| --- | --- |
| System | Il tuo compito è creare un sito web di una sola pagina basato sulle specifiche fornite, consegnato come file HTML con JavaScript e CSS incorporati. Il sito web dovrebbe incorporare una varietà di funzionalità di design coinvolgenti e interattive, come menu a discesa, testo e contenuto dinamici, pulsanti cliccabili e altro ancora. Assicurati che il design sia visivamente accattivante, reattivo e user-friendly. Il codice HTML, CSS e JavaScript dovrebbe essere ben strutturato, organizzato in modo efficiente e adeguatamente commentato per leggibilità e manutenibilità. |
| User   | Crea un sito web di una sola pagina per una piattaforma di apprendimento online chiamata "EduQuest" con le seguenti funzionalità e sezioni:<br/><br/>1. Una barra di navigazione fissa con collegamenti alle categorie di corsi (Matematica, Scienze, Lingue, Arti) e una barra di ricerca.<br/>2. Una sezione hero con uno sfondo video che mostra studenti che imparano online, un tagline dinamico che ruota tra "Impara al tuo ritmo," "Scopri nuove passioni," e "Espandi i tuoi orizzonti" ogni 3 secondi, e un pulsante "Inizia" che porta a un catalogo dei corsi.<br/>3. Una sezione corsi in evidenza che mostra schede dei corsi con segnaposto per immagini dei corsi, titoli, istruttori e descrizioni.<br/>4. Una sezione interattiva "Percorsi di Apprendimento" con un breve quiz per determinare stili di apprendimento e interessi, e un pulsante per iniziare il quiz.<br/>5. Una sezione "Storie di Successo" con testimonianze di studenti soddisfatti, con segnaposto per il testo delle testimonianze e i nomi degli studenti.<br/>6. Un footer con collegamenti al blog della piattaforma, FAQ, informativa sulla privacy, e un pulsante "Contattaci" che apre una finestra modale con un modulo di contatto e informazioni di supporto clienti.<br/><br/>Includi contenuto segnaposto di riempimento per lo sfondo video, le schede dei corsi e le testimonianze. Incorpora gli stili CSS all'interno del tag `<style>` nella sezione `<head>` e posiziona il codice JavaScript all'interno del tag `<script>` alla fine della sezione `<body>`.<br/><br/>Il codice JavaScript dovrebbe gestire il tagline dinamico nella sezione hero, ruotando attraverso i diversi tagline ogni 3 secondi. |

## Output di esempio

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

> Questo codice crea un sito web di una sola pagina per "EduQuest" con le seguenti funzionalità e sezioni:
> 
> Una barra di navigazione fissa con collegamenti alle categorie di corsi (Matematica, Scienze, Lingue, Arti) e una barra di ricerca.
> 
> Una sezione hero con uno sfondo video segnaposto, un tagline dinamico che ruota tra "Impara al tuo ritmo," "Scopri nuove passioni," e "Espandi i tuoi orizzonti" ogni 3 secondi, e un pulsante "Inizia".
> 
> Una sezione corsi in evidenza che mostra schede dei corsi con segnaposto per immagini dei corsi, titoli, istruttori e descrizioni.
> 
> Una sezione "Storie di Successo" con testimonianze di studenti soddisfatti, con segnaposto per il testo delle testimonianze e i nomi degli studenti.
> 
> Un footer con collegamenti al blog della piattaforma, FAQ, informativa sulla privacy, e un pulsante "Contattaci".
> 
> Gli stili CSS sono incorporati all'interno del tag `<style>` nella sezione `<head>`, e il codice JavaScript è posizionato all'interno del tag `<script>` alla fine della sezione `<body>`.
> 
> Il codice JavaScript gestisce il tagline dinamico nella sezione hero, ruotando attraverso i diversi tagline ogni 3 secondi utilizzando la funzione setInterval.
> 
> Si prega di notare che lo sfondo video, le immagini dei corsi e le testimonianze sono segnaposto e dovrebbero essere sostituiti con contenuto reale.

## Richiesta API

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
  system="Il tuo compito è creare un sito web di una sola pagina basato sulle specifiche fornite, consegnato come file HTML con JavaScript e CSS incorporati. Il sito web dovrebbe incorporare una varietà di funzionalità di design coinvolgenti e interattive, come menu a discesa, testo e contenuto dinamici, pulsanti cliccabili e altro ancora. Assicurati che il design sia visivamente accattivante, reattivo e user-friendly. Il codice HTML, CSS e JavaScript dovrebbe essere ben strutturato, organizzato in modo efficiente e adeguatamente commentato per leggibilità e manutenibilità.",
  messages=[
    {
      "role": "user",
      "content": [
        {
          "type": "text",
          "text": "Crea un sito web di una sola pagina per una piattaforma di apprendimento online chiamata \"EduQuest\" con le seguenti funzionalità e sezioni: \n \n1. Una barra di navigazione fissa con collegamenti alle categorie di corsi (Matematica, Scienze, Lingue, Arti) e una barra di ricerca. \n \n2. Una sezione hero con uno sfondo video che mostra studenti che imparano online, un tagline dinamico che ruota tra \"Impara al tuo ritmo,\" \"Scopri nuove passioni,\" e \"Espandi i tuoi orizzonti\" ogni 3 secondi, e un pulsante \"Inizia\" che porta a un catalogo dei corsi. \n \n3. Una sezione corsi in evidenza che mostra schede dei corsi con segnaposto per immagini dei corsi, titoli, istruttori e descrizioni. \n \n4. Una sezione interattiva \"Percorsi di Apprendimento\" con un breve quiz per determinare stili di apprendimento e interessi, e un pulsante per iniziare il quiz. \n \n5. Una sezione \"Storie di Successo\" con testimonianze di studenti soddisfatti, con segnaposto per il testo delle testimonianze e i nomi degli studenti. \n \n6. Un footer con collegamenti al blog della piattaforma, FAQ, informativa sulla privacy, e un pulsante \"Contattaci\" che apre una finestra modale con un modulo di contatto e informazioni di supporto clienti. \n \nIncludi contenuto segnaposto di riempimento per lo sfondo video, le schede dei corsi e le testimonianze. Incorpora gli stili CSS all'interno del tag `<style>` nella sezione `<head>` e posiziona il codice JavaScript all'interno del tag `<script>` alla fine della sezione `<body>`. \n \nIl codice JavaScript dovrebbe gestire il tagline dinamico nella sezione hero, ruotando attraverso i diversi tagline ogni 3 secondi."
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
  system: "Il tuo compito è creare un sito web di una sola pagina basato sulle specifiche fornite, consegnato come file HTML con JavaScript e CSS incorporati. Il sito web dovrebbe incorporare una varietà di funzionalità di design coinvolgenti e interattive, come menu a discesa, testo e contenuto dinamici, pulsanti cliccabili e altro ancora. Assicurati che il design sia visivamente accattivante, reattivo e user-friendly. Il codice HTML, CSS e JavaScript dovrebbe essere ben strutturato, organizzato in modo efficiente e adeguatamente commentato per leggibilità e manutenibilità.",
  messages: [
    {
      "role": "user",
      "content": [
        {
          "type": "text",
          "text": "Crea un sito web di una sola pagina per una piattaforma di apprendimento online chiamata \"EduQuest\" con le seguenti funzionalità e sezioni:  \n  \n1. Una barra di navigazione fissa con collegamenti alle categorie di corsi (Matematica, Scienze, Lingue, Arti) e una barra di ricerca.  \n  \n2. Una sezione hero con uno sfondo video che mostra studenti che imparano online, un tagline dinamico che ruota tra \"Impara al tuo ritmo,\" \"Scopri nuove passioni,\" e \"Espandi i tuoi orizzonti\" ogni 3 secondi, e un pulsante \"Inizia\" che porta a un catalogo dei corsi.  \n  \n3. Una sezione corsi in evidenza che mostra schede dei corsi con segnaposto per immagini dei corsi, titoli, istruttori e descrizioni.  \n  \n4. Una sezione interattiva \"Percorsi di Apprendimento\" con un breve quiz per determinare stili di apprendimento e interessi, e un pulsante per iniziare il quiz.  \n  \n5. Una sezione \"Storie di Successo\" con testimonianze di studenti soddisfatti, con segnaposto per il testo delle testimonianze e i nomi degli studenti.  \n  \n6. Un footer con collegamenti al blog della piattaforma, FAQ, informativa sulla privacy, e un pulsante \"Contattaci\" che apre una finestra modale con un modulo di contatto e informazioni di supporto clienti.  \n  \nIncludi contenuto segnaposto di riempimento per lo sfondo video, le schede dei corsi e le testimonianze. Incorpora gli stili CSS all'interno del tag `<style>` nella sezione `<head>` e posiziona il codice JavaScript all'interno del tag `<script>` alla fine della sezione `<body>`.  \n  \nIl codice JavaScript dovrebbe gestire il tagline dinamico nella sezione hero, ruotando attraverso i diversi tagline ogni 3 secondi."
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
  system="Il tuo compito è creare un sito web di una sola pagina basato sulle specifiche fornite, consegnato come file HTML con JavaScript e CSS incorporati. Il sito web dovrebbe incorporare una varietà di funzionalità di design coinvolgenti e interattive, come menu a discesa, testo e contenuto dinamici, pulsanti cliccabili e altro ancora. Assicurati che il design sia visivamente accattivante, reattivo e user-friendly. Il codice HTML, CSS e JavaScript dovrebbe essere ben strutturato, organizzato in modo efficiente e adeguatamente commentato per leggibilità e manutenibilità.",
  messages=[
    {
      "role": "user",
      "content": [
        {
          "type": "text",
          "text": "Crea un sito web di una sola pagina per una piattaforma di apprendimento online chiamata \"EduQuest\" con le seguenti funzionalità e sezioni: \n \n1. Una barra di navigazione fissa con collegamenti alle categorie di corsi (Matematica, Scienze, Lingue, Arti) e una barra di ricerca. \n \n2. Una sezione hero con uno sfondo video che mostra studenti che imparano online, un tagline dinamico che ruota tra \"Impara al tuo ritmo,\" \"Scopri nuove passioni,\" e \"Espandi i tuoi orizzonti\" ogni 3 secondi, e un pulsante \"Inizia\" che porta a un catalogo dei corsi. \n \n3. Una sezione corsi in evidenza che mostra schede dei corsi con segnaposto per immagini dei corsi, titoli, istruttori e descrizioni. \n \n4. Una sezione interattiva \"Percorsi di Apprendimento\" con un breve quiz per determinare stili di apprendimento e interessi, e un pulsante per iniziare il quiz. \n \n5. Una sezione \"Storie di Successo\" con testimonianze di studenti soddisfatti, con segnaposto per il testo delle testimonianze e i nomi degli studenti. \n \n6. Un footer con collegamenti al blog della piattaforma, FAQ, informativa sulla privacy, e un pulsante \"Contattaci\" che apre una finestra modale con un modulo di contatto e informazioni di supporto clienti. \n \nIncludi contenuto segnaposto di riempimento per lo sfondo video, le schede dei corsi e le testimonianze. Incorpora gli stili CSS all'interno del tag `<style>` nella sezione `<head>` e posiziona il codice JavaScript all'interno del tag `<script>` alla fine della sezione `<body>`. \n \nIl codice JavaScript dovrebbe gestire il tagline dinamico nella sezione hero, ruotando attraverso i diversi tagline ogni 3 secondi."
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
  system: "Il tuo compito è creare un sito web di una sola pagina basato sulle specifiche fornite, consegnato come file HTML con JavaScript e CSS incorporati. Il sito web dovrebbe incorporare una varietà di funzionalità di design coinvolgenti e interattive, come menu a discesa, testo e contenuto dinamici, pulsanti cliccabili e altro ancora. Assicurati che il design sia visivamente accattivante, reattivo e user-friendly. Il codice HTML, CSS e JavaScript dovrebbe essere ben strutturato, organizzato in modo efficiente e adeguatamente commentato per leggibilità e manutenibilità.",
  messages: [
    {
      "role": "user",
      "content": [
        {
          "type": "text",
          "text": "Crea un sito web di una sola pagina per una piattaforma di apprendimento online chiamata \"EduQuest\" con le seguenti funzionalità e sezioni: \n \n1. Una barra di navigazione fissa con collegamenti alle categorie di corsi (Matematica, Scienze, Lingue, Arti) e una barra di ricerca. \n \n2. Una sezione hero con uno sfondo video che mostra studenti che imparano online, un tagline dinamico che ruota tra \"Impara al tuo ritmo,\" \"Scopri nuove passioni,\" e \"Espandi i tuoi orizzonti\" ogni 3 secondi, e un pulsante \"Inizia\" che porta a un catalogo dei corsi. \n \n3. Una sezione corsi in evidenza che mostra schede dei corsi con segnaposto per immagini dei corsi, titoli, istruttori e descrizioni. \n \n4. Una sezione interattiva \"Percorsi di Apprendimento\" con un breve quiz per determinare stili di apprendimento e interessi, e un pulsante per iniziare il quiz. \n \n5. Una sezione \"Storie di Successo\" con testimonianze di studenti soddisfatti, con segnaposto per il testo delle testimonianze e i nomi degli studenti. \n \n6. Un footer con collegamenti al blog della piattaforma, FAQ, informativa sulla privacy, e un pulsante \"Contattaci\" che apre una finestra modale con un modulo di contatto e informazioni di supporto clienti. \n \nIncludi contenuto segnaposto di riempimento per lo sf ondo video, le schede dei corsi e le testimonianze. Incorpora gli stili CSS all'interno del tag `<style>` nella sezione `<head>` e posiziona il codice JavaScript all'interno del tag `<script>` alla fine della sezione `<body>`. \n \nIl codice JavaScript dovrebbe gestire il tagline dinamico nella sezione hero, ruotando attraverso i diversi tagline ogni 3 secondi."
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
  system="Il tuo compito è creare un sito web di una sola pagina basato sulle specifiche fornite, consegnato come file HTML con JavaScript e CSS incorporati. Il sito web dovrebbe incorporare una varietà di funzionalità di design coinvolgenti e interattive, come menu a discesa, testo e contenuto dinamici, pulsanti cliccabili e altro ancora. Assicurati che il design sia visivamente accattivante, reattivo e user-friendly. Il codice HTML, CSS e JavaScript dovrebbe essere ben strutturato, organizzato in modo efficiente e adeguatamente commentato per leggibilità e manutenibilità.",
  messages=[
    {
      "role": "user",
      "content": [
        {
          "type": "text",
          "text": "Crea un sito web di una sola pagina per una piattaforma di apprendimento online chiamata \"EduQuest\" con le seguenti funzionalità e sezioni: \n \n1. Una barra di navigazione fissa con collegamenti alle categorie di corsi (Matematica, Scienze, Lingue, Arti) e una barra di ricerca. \n \n2. Una sezione hero con uno sfondo video che mostra studenti che imparano online, un tagline dinamico che ruota tra \"Impara al tuo ritmo,\" \"Scopri nuove passioni,\" e \"Espandi i tuoi orizzonti\" ogni 3 secondi, e un pulsante \"Inizia\" che porta a un catalogo dei corsi. \n \n3. Una sezione corsi in evidenza che mostra schede dei corsi con segnaposto per immagini dei corsi, titoli, istruttori e descrizioni. \n \n4. Una sezione interattiva \"Percorsi di Apprendimento\" con un breve quiz per determinare stili di apprendimento e interessi, e un pulsante per iniziare il quiz. \n \n5. Una sezione \"Storie di Successo\" con testimonianze di studenti soddisfatti, con segnaposto per il testo delle testimonianze e i nomi degli studenti. \n \n6. Un footer con collegamenti al blog della piattaforma, FAQ, informativa sulla privacy, e un pulsante \"Contattaci\" che apre una finestra modale con un modulo di contatto e informazioni di supporto clienti. \n \nIncludi contenuto segnaposto di riempimento per lo sfondo video, le schede dei corsi e le testimonianze. Incorpora gli stili CSS all'interno del tag `<style>` nella sezione `<head>` e posiziona il codice JavaScript all'interno del tag `<script>` alla fine della sezione `<body>`. \n \nIl codice JavaScript dovrebbe gestire il tagline dinamico nella sezione hero, ruotando attraverso i diversi tagline ogni 3 secondi."
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
system: "Il tuo compito è creare un sito web di una sola pagina basato sulle specifiche fornite, consegnato come file HTML con JavaScript e CSS incorporati. Il sito web dovrebbe incorporare una varietà di funzionalità di design coinvolgenti e interattive, come menu a discesa, testo e contenuto dinamici, pulsanti cliccabili e altro ancora. Assicurati che il design sia visivamente accattivante, reattivo e user-friendly. Il codice HTML, CSS e JavaScript dovrebbe essere ben strutturato, organizzato in modo efficiente e adeguatamente commentato per leggibilità e manutenibilità.",
messages: [
{
"role": "user",
"content": [
{
"type": "text",
"text": "Crea un sito web di una sola pagina per una piattaforma di apprendimento online chiamata \"EduQuest\" con le seguenti funzionalità e sezioni: \n \n1. Una barra di navigazione fissa con collegamenti alle categorie di corsi (Matematica, Scienze, Lingue, Arti) e una barra di ricerca. \n \n2. Una sezione hero con uno sfondo video che mostra studenti che imparano online, un tagline dinamico che ruota tra \"Impara al tuo ritmo,\" \"Scopri nuove passioni,\" e \"Espandi i tuoi orizzonti\" ogni 3 secondi, e un pulsante \"Inizia\" che porta a un catalogo dei corsi. \n \n3. Una sezione corsi in evidenza che mostra schede dei corsi con segnaposto per immagini dei corsi, titoli, istruttori e descrizioni. \n \n4. Una sezione interattiva \"Percorsi di Apprendimento\" con un breve quiz per determinare stili di apprendimento e interessi, e un pulsante per iniziare il quiz. \n \n5. Una sezione \"Storie di Successo\" con testimonianze di studenti soddisfatti, con segnaposto per il testo delle testimonianze e i nomi degli studenti. \n \n6. Un footer con collegamenti al blog della piattaforma, FAQ, informativa sulla privacy, e un pulsante \"Contattaci\" che apre una finestra modale con un modulo di contatto e informazioni di supporto clienti. \n \nIncludi contenuto segnaposto di riempimento per lo sfondo video, le schede dei corsi e le testimonianze. Incorpora gli stili CSS all'interno del tag `<style>` nella sezione `<head>` e posiziona il codice JavaScript all'interno del tag `<script>` alla fine della sezione `<body>`. \n \nIl codice JavaScript dovrebbe gestire il tagline dinamico nella sezione hero, ruotando attraverso i diversi tagline ogni 3 secondi."
}
]
}
]
});
console.log(msg);

```

</Tab>
</Tabs>