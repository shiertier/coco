# Assistant de site web

Créer des sites web d'une page basés sur les spécifications de l'utilisateur.

---

> Copiez cette invite dans notre [Console](/dashboard) développeur pour l'essayer vous-même !

|        | Contenu |
| --- | --- |
| System | Votre tâche est de créer un site web d'une page basé sur les spécifications données, livré sous forme de fichier HTML avec JavaScript et CSS intégrés. Le site web devrait incorporer une variété de fonctionnalités de conception engageantes et interactives, telles que des menus déroulants, du texte et du contenu dynamiques, des boutons cliquables, et plus encore. Assurez-vous que la conception soit visuellement attrayante, responsive et conviviale. Le code HTML, CSS et JavaScript devrait être bien structuré, organisé efficacement et correctement commenté pour la lisibilité et la maintenabilité. |
| User   | Créez un site web d'une page pour une plateforme d'apprentissage en ligne appelée "EduQuest" avec les fonctionnalités et sections suivantes :<br/><br/>1. Une barre de navigation fixe avec des liens vers les catégories de cours (Mathématiques, Sciences, Langues, Arts) et une barre de recherche.<br/>2. Une section héros avec un arrière-plan vidéo montrant des étudiants apprenant en ligne, un slogan dynamique qui alterne entre "Apprenez à votre rythme", "Découvrez de nouvelles passions" et "Élargissez vos horizons" toutes les 3 secondes, et un bouton "Commencer" menant à un catalogue de cours.<br/>3. Une section de cours en vedette affichant des cartes de cours avec des espaces réservés pour les images de cours, les titres, les instructeurs et les descriptions.<br/>4. Une section interactive "Parcours d'apprentissage" avec un court quiz pour déterminer les styles d'apprentissage et les intérêts, et un bouton pour commencer le quiz.<br/>5. Une section "Histoires de réussite" présentant des témoignages d'étudiants satisfaits, avec des espaces réservés pour le texte du témoignage et les noms des étudiants.<br/>6. Un pied de page avec des liens vers le blog de la plateforme, FAQ, politique de confidentialité, et un bouton "Nous contacter" qui ouvre une fenêtre modale avec un formulaire de contact et des informations de support client.<br/><br/>Incluez du contenu de remplissage pour l'arrière-plan vidéo, les cartes de cours et les témoignages. Intégrez les styles CSS dans la balise `<style>` dans la section `<head>` et placez le code JavaScript dans la balise `<script>` à la fin de la section `<body>`.<br/><br/>Le code JavaScript devrait gérer le slogan dynamique dans la section héros, alternant entre les différents slogans toutes les 3 secondes. |

## Exemple de sortie

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

> Ce code crée un site web d'une page pour "EduQuest" avec les fonctionnalités et sections suivantes :
> 
> Une barre de navigation fixe avec des liens vers les catégories de cours (Mathématiques, Sciences, Langues, Arts) et une barre de recherche.
> 
> Une section héros avec un arrière-plan vidéo de substitution, un slogan dynamique qui alterne entre "Apprenez à votre rythme", "Découvrez de nouvelles passions" et "Élargissez vos horizons" toutes les 3 secondes, et un bouton "Commencer".
> 
> Une section de cours en vedette affichant des cartes de cours avec des espaces réservés pour les images de cours, les titres, les instructeurs et les descriptions.
> 
> Une section "Histoires de réussite" présentant des témoignages d'étudiants satisfaits, avec des espaces réservés pour le texte du témoignage et les noms des étudiants.
> 
> Un pied de page avec des liens vers le blog de la plateforme, FAQ, politique de confidentialité, et un bouton "Nous contacter".
> 
> Les styles CSS sont intégrés dans la balise `<style>` dans la section `<head>`, et le code JavaScript est placé dans la balise `<script>` à la fin de la section `<body>`.
> 
> Le code JavaScript gère le slogan dynamique dans la section héros, alternant entre les différents slogans toutes les 3 secondes en utilisant la fonction setInterval.
> 
> Veuillez noter que l'arrière-plan vidéo, les images de cours et les témoignages sont des espaces réservés et devraient être remplacés par du contenu réel.

## Requête API

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
  system="Votre tâche est de créer un site web d'une page basé sur les spécifications données, livré sous forme de fichier HTML avec JavaScript et CSS intégrés. Le site web devrait incorporer une variété de fonctionnalités de conception engageantes et interactives, telles que des menus déroulants, du texte et du contenu dynamiques, des boutons cliquables, et plus encore. Assurez-vous que la conception soit visuellement attrayante, responsive et conviviale. Le code HTML, CSS et JavaScript devrait être bien structuré, organisé efficacement et correctement commenté pour la lisibilité et la maintenabilité.",
  messages=[
    {
      "role": "user",
      "content": [
        {
          "type": "text",
          "text": "Créez un site web d'une page pour une plateforme d'apprentissage en ligne appelée \"EduQuest\" avec les fonctionnalités et sections suivantes : \n \n1. Une barre de navigation fixe avec des liens vers les catégories de cours (Mathématiques, Sciences, Langues, Arts) et une barre de recherche. \n \n2. Une section héros avec un arrière-plan vidéo montrant des étudiants apprenant en ligne, un slogan dynamique qui alterne entre \"Apprenez à votre rythme\", \"Découvrez de nouvelles passions\" et \"Élargissez vos horizons\" toutes les 3 secondes, et un bouton \"Commencer\" menant à un catalogue de cours. \n \n3. Une section de cours en vedette affichant des cartes de cours avec des espaces réservés pour les images de cours, les titres, les instructeurs et les descriptions. \n \n4. Une section interactive \"Parcours d'apprentissage\" avec un court quiz pour déterminer les styles d'apprentissage et les intérêts, et un bouton pour commencer le quiz. \n \n5. Une section \"Histoires de réussite\" présentant des témoignages d'étudiants satisfaits, avec des espaces réservés pour le texte du témoignage et les noms des étudiants. \n \n6. Un pied de page avec des liens vers le blog de la plateforme, FAQ, politique de confidentialité, et un bouton \"Nous contacter\" qui ouvre une fenêtre modale avec un formulaire de contact et des informations de support client. \n \nIncluez du contenu de remplissage pour l'arrière-plan vidéo, les cartes de cours et les témoignages. Intégrez les styles CSS dans la balise `<style>` dans la section `<head>` et placez le code JavaScript dans la balise `<script>` à la fin de la section `<body>`. \n \nLe code JavaScript devrait gérer le slogan dynamique dans la section héros, alternant entre les différents slogans toutes les 3 secondes."
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
  system: "Votre tâche est de créer un site web d'une page basé sur les spécifications données, livré sous forme de fichier HTML avec JavaScript et CSS intégrés. Le site web devrait incorporer une variété de fonctionnalités de conception engageantes et interactives, telles que des menus déroulants, du texte et du contenu dynamiques, des boutons cliquables, et plus encore. Assurez-vous que la conception soit visuellement attrayante, responsive et conviviale. Le code HTML, CSS et JavaScript devrait être bien structuré, organisé efficacement et correctement commenté pour la lisibilité et la maintenabilité.",
  messages: [
    {
      "role": "user",
      "content": [
        {
          "type": "text",
          "text": "Créez un site web d'une page pour une plateforme d'apprentissage en ligne appelée \"EduQuest\" avec les fonctionnalités et sections suivantes :  \n  \n1. Une barre de navigation fixe avec des liens vers les catégories de cours (Mathématiques, Sciences, Langues, Arts) et une barre de recherche.  \n  \n2. Une section héros avec un arrière-plan vidéo montrant des étudiants apprenant en ligne, un slogan dynamique qui alterne entre \"Apprenez à votre rythme\", \"Découvrez de nouvelles passions\" et \"Élargissez vos horizons\" toutes les 3 secondes, et un bouton \"Commencer\" menant à un catalogue de cours.  \n  \n3. Une section de cours en vedette affichant des cartes de cours avec des espaces réservés pour les images de cours, les titres, les instructeurs et les descriptions.  \n  \n4. Une section interactive \"Parcours d'apprentissage\" avec un court quiz pour déterminer les styles d'apprentissage et les intérêts, et un bouton pour commencer le quiz.  \n  \n5. Une section \"Histoires de réussite\" présentant des témoignages d'étudiants satisfaits, avec des espaces réservés pour le texte du témoignage et les noms des étudiants.  \n  \n6. Un pied de page avec des liens vers le blog de la plateforme, FAQ, politique de confidentialité, et un bouton \"Nous contacter\" qui ouvre une fenêtre modale avec un formulaire de contact et des informations de support client.  \n  \nIncluez du contenu de remplissage pour l'arrière-plan vidéo, les cartes de cours et les témoignages. Intégrez les styles CSS dans la balise `<style>` dans la section `<head>` et placez le code JavaScript dans la balise `<script>` à la fin de la section `<body>`.  \n  \nLe code JavaScript devrait gérer le slogan dynamique dans la section héros, alternant entre les différents slogans toutes les 3 secondes."
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
  system="Votre tâche est de créer un site web d'une page basé sur les spécifications données, livré sous forme de fichier HTML avec JavaScript et CSS intégrés. Le site web devrait incorporer une variété de fonctionnalités de conception engageantes et interactives, telles que des menus déroulants, du texte et du contenu dynamiques, des boutons cliquables, et plus encore. Assurez-vous que la conception soit visuellement attrayante, responsive et conviviale. Le code HTML, CSS et JavaScript devrait être bien structuré, organisé efficacement et correctement commenté pour la lisibilité et la maintenabilité.",
  messages=[
    {
      "role": "user",
      "content": [
        {
          "type": "text",
          "text": "Créez un site web d'une page pour une plateforme d'apprentissage en ligne appelée \"EduQuest\" avec les fonctionnalités et sections suivantes : \n \n1. Une barre de navigation fixe avec des liens vers les catégories de cours (Mathématiques, Sciences, Langues, Arts) et une barre de recherche. \n \n2. Une section héros avec un arrière-plan vidéo montrant des étudiants apprenant en ligne, un slogan dynamique qui alterne entre \"Apprenez à votre rythme\", \"Découvrez de nouvelles passions\" et \"Élargissez vos horizons\" toutes les 3 secondes, et un bouton \"Commencer\" menant à un catalogue de cours. \n \n3. Une section de cours en vedette affichant des cartes de cours avec des espaces réservés pour les images de cours, les titres, les instructeurs et les descriptions. \n \n4. Une section interactive \"Parcours d'apprentissage\" avec un court quiz pour déterminer les styles d'apprentissage et les intérêts, et un bouton pour commencer le quiz. \n \n5. Une section \"Histoires de réussite\" présentant des témoignages d'étudiants satisfaits, avec des espaces réservés pour le texte du témoignage et les noms des étudiants. \n \n6. Un pied de page avec des liens vers le blog de la plateforme, FAQ, politique de confidentialité, et un bouton \"Nous contacter\" qui ouvre une fenêtre modale avec un formulaire de contact et des informations de support client. \n \nIncluez du contenu de remplissage pour l'arrière-plan vidéo, les cartes de cours et les témoignages. Intégrez les styles CSS dans la balise `<style>` dans la section `<head>` et placez le code JavaScript dans la balise `<script>` à la fin de la section `<body>`. \n \nLe code JavaScript devrait gérer le slogan dynamique dans la section héros, alternant entre les différents slogans toutes les 3 secondes."
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
  system: "Votre tâche est de créer un site web d'une page basé sur les spécifications données, livré sous forme de fichier HTML avec JavaScript et CSS intégrés. Le site web devrait incorporer une variété de fonctionnalités de conception engageantes et interactives, telles que des menus déroulants, du texte et du contenu dynamiques, des boutons cliquables, et plus encore. Assurez-vous que la conception soit visuellement attrayante, responsive et conviviale. Le code HTML, CSS et JavaScript devrait être bien structuré, organisé efficacement et correctement commenté pour la lisibilité et la maintenabilité.",
  messages: [
    {
      "role": "user",
      "content": [
        {
          "type": "text",
          "text": "Créez un site web d'une page pour une plateforme d'apprentissage en ligne appelée \"EduQuest\" avec les fonctionnalités et sections suivantes : \n \n1. Une barre de navigation fixe avec des liens vers les catégories de cours (Mathématiques, Sciences, Langues, Arts) et une barre de recherche. \n \n2. Une section héros avec un arrière-plan vidéo montrant des étudiants apprenant en ligne, un slogan dynamique qui alterne entre \"Apprenez à votre rythme\", \"Découvrez de nouvelles passions\" et \"Élargissez vos horizons\" toutes les 3 secondes, et un bouton \"Commencer\" menant à un catalogue de cours. \n \n3. Une section de cours en vedette affichant des cartes de cours avec des espaces réservés pour les images de cours, les titres, les instructeurs et les descriptions. \n \n4. Une section interactive \"Parcours d'apprentissage\" avec un court quiz pour déterminer les styles d'apprentissage et les intérêts, et un bouton pour commencer le quiz. \n \n5. Une section \"Histoires de réussite\" présentant des témoignages d'étudiants satisf aits, avec des espaces réservés pour le texte du témoignage et les noms des étudiants. \n \n6. Un pied de page avec des liens vers le blog de la plateforme, FAQ, politique de confidentialité, et un bouton \"Nous contacter\" qui ouvre une fenêtre modale avec un formulaire de contact et des informations de support client. \n \nIncluez du contenu de remplissage pour l'arrière-plan vidéo, les cartes de cours et les témoignages. Intégrez les styles CSS dans la balise `<style>` dans la section `<head>` et placez le code JavaScript dans la balise `<script>` à la fin de la section `<body>`. \n \nLe code JavaScript devrait gérer le slogan dynamique dans la section héros, alternant entre les différents slogans toutes les 3 secondes."
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
  system="Votre tâche est de créer un site web d'une page basé sur les spécifications données, livré sous forme de fichier HTML avec JavaScript et CSS intégrés. Le site web devrait incorporer une variété de fonctionnalités de conception engageantes et interactives, telles que des menus déroulants, du texte et du contenu dynamiques, des boutons cliquables, et plus encore. Assurez-vous que la conception soit visuellement attrayante, responsive et conviviale. Le code HTML, CSS et JavaScript devrait être bien structuré, organisé efficacement et correctement commenté pour la lisibilité et la maintenabilité.",
  messages=[
    {
      "role": "user",
      "content": [
        {
          "type": "text",
          "text": "Créez un site web d'une page pour une plateforme d'apprentissage en ligne appelée \"EduQuest\" avec les fonctionnalités et sections suivantes : \n \n1. Une barre de navigation fixe avec des liens vers les catégories de cours (Mathématiques, Sciences, Langues, Arts) et une barre de recherche. \n \n2. Une section héros avec un arrière-plan vidéo montrant des étudiants apprenant en ligne, un slogan dynamique qui alterne entre \"Apprenez à votre rythme\", \"Découvrez de nouvelles passions\" et \"Élargissez vos horizons\" toutes les 3 secondes, et un bouton \"Commencer\" menant à un catalogue de cours. \n \n3. Une section de cours en vedette affichant des cartes de cours avec des espaces réservés pour les images de cours, les titres, les instructeurs et les descriptions. \n \n4. Une section interactive \"Parcours d'apprentissage\" avec un court quiz pour déterminer les styles d'apprentissage et les intérêts, et un bouton pour commencer le quiz. \n \n5. Une section \"Histoires de réussite\" présentant des témoignages d'étudiants satisfaits, avec des espaces réservés pour le texte du témoignage et les noms des étudiants. \n \n6. Un pied de page avec des liens vers le blog de la plateforme, FAQ, politique de confidentialité, et un bouton \"Nous contacter\" qui ouvre une fenêtre modale avec un formulaire de contact et des informations de support client. \n \nIncluez du contenu de remplissage pour l'arrière-plan vidéo, les cartes de cours et les témoignages. Intégrez les styles CSS dans la balise `<style>` dans la section `<head>` et placez le code JavaScript dans la balise `<script>` à la fin de la section `<body>`. \n \nLe code JavaScript devrait gérer le slogan dynamique dans la section héros, alternant entre les différents slogans toutes les 3 secondes."
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
system: "Votre tâche est de créer un site web d'une page basé sur les spécifications données, livré sous forme de fichier HTML avec JavaScript et CSS intégrés. Le site web devrait incorporer une variété de fonctionnalités de conception engageantes et interactives, telles que des menus déroulants, du texte et du contenu dynamiques, des boutons cliquables, et plus encore. Assurez-vous que la conception soit visuellement attrayante, responsive et conviviale. Le code HTML, CSS et JavaScript devrait être bien structuré, organisé efficacement et correctement commenté pour la lisibilité et la maintenabilité.",
messages: [
{
"role": "user",
"content": [
{
"type": "text",
"text": "Créez un site web d'une page pour une plateforme d'apprentissage en ligne appelée \"EduQuest\" avec les fonctionnalités et sections suivantes : \n \n1. Une barre de navigation fixe avec des liens vers les catégories de cours (Mathématiques, Sciences, Langues, Arts) et une barre de recherche. \n \n2. Une section héros avec un arrière-plan vidéo montrant des étudiants apprenant en ligne, un slogan dynamique qui alterne entre \"Apprenez à votre rythme\", \"Découvrez de nouvelles passions\" et \"Élargissez vos horizons\" toutes les 3 secondes, et un bouton \"Commencer\" menant à un catalogue de cours. \n \n3. Une section de cours en vedette affichant des cartes de cours avec des espaces réservés pour les images de cours, les titres, les instructeurs et les descriptions. \n \n4. Une section interactive \"Parcours d'apprentissage\" avec un court quiz pour déterminer les styles d'apprentissage et les intérêts, et un bouton pour commencer le quiz. \n \n5. Une section \"Histoires de réussite\" présentant des témoignages d'étudiants satisfaits, avec des espaces réservés pour le texte du témoignage et les noms des étudiants. \n \n6. Un pied de page avec des liens vers le blog de la plateforme, FAQ, politique de confidentialité, et un bouton \"Nous contacter\" qui ouvre une fenêtre modale avec un formulaire de contact et des informations de support client. \n \nIncluez du contenu de remplissage pour l'arrière-plan vidéo, les cartes de cours et les témoignages. Intégrez les styles CSS dans la balise `<style>` dans la section `<head>` et placez le code JavaScript dans la balise `<script>` à la fin de la section `<body>`. \n \nLe code JavaScript devrait gérer le slogan dynamique dans la section héros, alternant entre les différents slogans toutes les 3 secondes."
}
]
}
]
});
console.log(msg);

```

</Tab>
</Tabs>