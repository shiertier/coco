# Asistente de sitios web

Crea sitios web de una página basados en las especificaciones del usuario.

---

> ¡Copia este prompt en nuestra [Consola](/dashboard) de desarrollador para probarlo tú mismo!

|        | Contenido |
| --- | --- |
| Sistema | Tu tarea es crear un sitio web de una página basado en las especificaciones dadas, entregado como un archivo HTML con JavaScript y CSS integrados. El sitio web debe incorporar una variedad de características de diseño atractivas e interactivas, como menús desplegables, texto y contenido dinámico, botones clicables, y más. Asegúrate de que el diseño sea visualmente atractivo, responsivo y fácil de usar. El código HTML, CSS y JavaScript debe estar bien estructurado, organizado de manera eficiente y comentado adecuadamente para legibilidad y mantenibilidad. |
| Usuario   | Crea un sitio web de una página para una plataforma de aprendizaje en línea llamada "EduQuest" con las siguientes características y secciones:<br/><br/>1. Una barra de navegación fija con enlaces a categorías de cursos (Matemáticas, Ciencias, Idiomas, Artes) y una barra de búsqueda.<br/>2. Una sección hero con un fondo de video mostrando estudiantes aprendiendo en línea, un eslogan dinámico que rote entre "Aprende a tu propio ritmo," "Descubre nuevas pasiones," y "Expande tus horizontes" cada 3 segundos, y un botón "Comenzar" que lleve a un catálogo de cursos.<br/>3. Una sección de cursos destacados mostrando tarjetas de cursos con marcadores de posición para imágenes de cursos, títulos, instructores y descripciones.<br/>4. Una sección interactiva de "Rutas de Aprendizaje" con un cuestionario corto para determinar estilos de aprendizaje e intereses, y un botón para comenzar el cuestionario.<br/>5. Una sección de "Historias de Éxito" con testimonios de estudiantes satisfechos, con marcadores de posición para el texto del testimonio y nombres de estudiantes.<br/>6. Un pie de página con enlaces al blog de la plataforma, FAQ, política de privacidad, y un botón "Contáctanos" que abra una ventana modal con un formulario de contacto e información de soporte al cliente.<br/><br/>Incluye contenido de marcador de posición de relleno para el fondo de video, tarjetas de cursos y testimonios. Integra los estilos CSS dentro de la etiqueta `<style>` en la sección `<head>` y coloca el código JavaScript dentro de la etiqueta `<script>` al final de la sección `<body>`.<br/><br/>El código JavaScript debe manejar el eslogan dinámico en la sección hero, rotando a través de los diferentes eslóganes cada 3 segundos. |

## Ejemplo de salida

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

> Este código crea un sitio web de una página para "EduQuest" con las siguientes características y secciones:
> 
> Una barra de navegación fija con enlaces a categorías de cursos (Matemáticas, Ciencias, Idiomas, Artes) y una barra de búsqueda.
> 
> Una sección hero con un fondo de video de marcador de posición, un eslogan dinámico que rota entre "Aprende a tu propio ritmo," "Descubre nuevas pasiones," y "Expande tus horizontes" cada 3 segundos, y un botón "Comenzar".
> 
> Una sección de cursos destacados mostrando tarjetas de cursos con marcadores de posición para imágenes de cursos, títulos, instructores y descripciones.
> 
> Una sección de "Historias de Éxito" con testimonios de estudiantes satisfechos, con marcadores de posición para el texto del testimonio y nombres de estudiantes.
> 
> Un pie de página con enlaces al blog de la plataforma, FAQ, política de privacidad, y un botón "Contáctanos".
> 
> Los estilos CSS están integrados dentro de la etiqueta `<style>` en la sección `<head>`, y el código JavaScript está colocado dentro de la etiqueta `<script>` al final de la sección `<body>`.
> 
> El código JavaScript maneja el eslogan dinámico en la sección hero, rotando a través de los diferentes eslóganes cada 3 segundos usando la función setInterval.
> 
> Ten en cuenta que el fondo de video, las imágenes de cursos y los testimonios son marcadores de posición y deben ser reemplazados con contenido real.

## Solicitud de API

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
  system="Tu tarea es crear un sitio web de una página basado en las especificaciones dadas, entregado como un archivo HTML con JavaScript y CSS integrados. El sitio web debe incorporar una variedad de características de diseño atractivas e interactivas, como menús desplegables, texto y contenido dinámico, botones clicables, y más. Asegúrate de que el diseño sea visualmente atractivo, responsivo y fácil de usar. El código HTML, CSS y JavaScript debe estar bien estructurado, organizado de manera eficiente y comentado adecuadamente para legibilidad y mantenibilidad.",
  messages=[
    {
      "role": "user",
      "content": [
        {
          "type": "text",
          "text": "Crea un sitio web de una página para una plataforma de aprendizaje en línea llamada \"EduQuest\" con las siguientes características y secciones: \n \n1. Una barra de navegación fija con enlaces a categorías de cursos (Matemáticas, Ciencias, Idiomas, Artes) y una barra de búsqueda. \n \n2. Una sección hero con un fondo de video mostrando estudiantes aprendiendo en línea, un eslogan dinámico que rote entre \"Aprende a tu propio ritmo,\" \"Descubre nuevas pasiones,\" y \"Expande tus horizontes\" cada 3 segundos, y un botón \"Comenzar\" que lleve a un catálogo de cursos. \n \n3. Una sección de cursos destacados mostrando tarjetas de cursos con marcadores de posición para imágenes de cursos, títulos, instructores y descripciones. \n \n4. Una sección interactiva de \"Rutas de Aprendizaje\" con un cuestionario corto para determinar estilos de aprendizaje e intereses, y un botón para comenzar el cuestionario. \n \n5. Una sección de \"Historias de Éxito\" con testimonios de estudiantes satisfechos, con marcadores de posición para el texto del testimonio y nombres de estudiantes. \n \n6. Un pie de página con enlaces al blog de la plataforma, FAQ, política de privacidad, y un botón \"Contáctanos\" que abra una ventana modal con un formulario de contacto e información de soporte al cliente. \n \nIncluye contenido de marcador de posición de relleno para el fondo de video, tarjetas de cursos y testimonios. Integra los estilos CSS dentro de la etiqueta `<style>` en la sección `<head>` y coloca el código JavaScript dentro de la etiqueta `<script>` al final de la sección `<body>`. \n \nEl código JavaScript debe manejar el eslogan dinámico en la sección hero, rotando a través de los diferentes eslóganes cada 3 segundos."
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
  system: "Tu tarea es crear un sitio web de una página basado en las especificaciones dadas, entregado como un archivo HTML con JavaScript y CSS integrados. El sitio web debe incorporar una variedad de características de diseño atractivas e interactivas, como menús desplegables, texto y contenido dinámico, botones clicables, y más. Asegúrate de que el diseño sea visualmente atractivo, responsivo y fácil de usar. El código HTML, CSS y JavaScript debe estar bien estructurado, organizado de manera eficiente y comentado adecuadamente para legibilidad y mantenibilidad.",
  messages: [
    {
      "role": "user",
      "content": [
        {
          "type": "text",
          "text": "Crea un sitio web de una página para una plataforma de aprendizaje en línea llamada \"EduQuest\" con las siguientes características y secciones:  \n  \n1. Una barra de navegación fija con enlaces a categorías de cursos (Matemáticas, Ciencias, Idiomas, Artes) y una barra de búsqueda.  \n  \n2. Una sección hero con un fondo de video mostrando estudiantes aprendiendo en línea, un eslogan dinámico que rote entre \"Aprende a tu propio ritmo,\" \"Descubre nuevas pasiones,\" y \"Expande tus horizontes\" cada 3 segundos, y un botón \"Comenzar\" que lleve a un catálogo de cursos.  \n  \n3. Una sección de cursos destacados mostrando tarjetas de cursos con marcadores de posición para imágenes de cursos, títulos, instructores y descripciones.  \n  \n4. Una sección interactiva de \"Rutas de Aprendizaje\" con un cuestionario corto para determinar estilos de aprendizaje e intereses, y un botón para comenzar el cuestionario.  \n  \n5. Una sección de \"Historias de Éxito\" con testimonios de estudiantes satisfechos, con marcadores de posición para el texto del testimonio y nombres de estudiantes.  \n  \n6. Un pie de página con enlaces al blog de la plataforma, FAQ, política de privacidad, y un botón \"Contáctanos\" que abra una ventana modal con un formulario de contacto e información de soporte al cliente.  \n  \nIncluye contenido de marcador de posición de relleno para el fondo de video, tarjetas de cursos y testimonios. Integra los estilos CSS dentro de la etiqueta `<style>` en la sección `<head>` y coloca el código JavaScript dentro de la etiqueta `<script>` al final de la sección `<body>`.  \n  \nEl código JavaScript debe manejar el eslogan dinámico en la sección hero, rotando a través de los diferentes eslóganes cada 3 segundos."
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
  system="Tu tarea es crear un sitio web de una página basado en las especificaciones dadas, entregado como un archivo HTML con JavaScript y CSS integrados. El sitio web debe incorporar una variedad de características de diseño atractivas e interactivas, como menús desplegables, texto y contenido dinámico, botones clicables, y más. Asegúrate de que el diseño sea visualmente atractivo, responsivo y fácil de usar. El código HTML, CSS y JavaScript debe estar bien estructurado, organizado de manera eficiente y comentado adecuadamente para legibilidad y mantenibilidad.",
  messages=[
    {
      "role": "user",
      "content": [
        {
          "type": "text",
          "text": "Crea un sitio web de una página para una plataforma de aprendizaje en línea llamada \"EduQuest\" con las siguientes características y secciones: \n \n1. Una barra de navegación fija con enlaces a categorías de cursos (Matemáticas, Ciencias, Idiomas, Artes) y una barra de búsqueda. \n \n2. Una sección hero con un fondo de video mostrando estudiantes aprendiendo en línea, un eslogan dinámico que rote entre \"Aprende a tu propio ritmo,\" \"Descubre nuevas pasiones,\" y \"Expande tus horizontes\" cada 3 segundos, y un botón \"Comenzar\" que lleve a un catálogo de cursos. \n \n3. Una sección de cursos destacados mostrando tarjetas de cursos con marcadores de posición para imágenes de cursos, títulos, instructores y descripciones. \n \n4. Una sección interactiva de \"Rutas de Aprendizaje\" con un cuestionario corto para determinar estilos de aprendizaje e intereses, y un botón para comenzar el cuestionario. \n \n5. Una sección de \"Historias de Éxito\" con testimonios de estudiantes satisfechos, con marcadores de posición para el texto del testimonio y nombres de estudiantes. \n \n6. Un pie de página con enlaces al blog de la plataforma, FAQ, política de privacidad, y un botón \"Contáctanos\" que abra una ventana modal con un formulario de contacto e información de soporte al cliente. \n \nIncluye contenido de marcador de posición de relleno para el fondo de video, tarjetas de cursos y testimonios. Integra los estilos CSS dentro de la etiqueta `<style>` en la sección `<head>` y coloca el código JavaScript dentro de la etiqueta `<script>` al final de la sección `<body>`. \n \nEl código JavaScript debe manejar el eslogan dinámico en la sección hero, rotando a través de los diferentes eslóganes cada 3 segundos."
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
  system: "Tu tarea es crear un sitio web de una página basado en las especificaciones dadas, entregado como un archivo HTML con JavaScript y CSS integrados. El sitio web debe incorporar una variedad de características de diseño atractivas e interactivas, como menús desplegables, texto y contenido dinámico, botones clicables, y más. Asegúrate de que el diseño sea visualmente atractivo, responsivo y fácil de usar. El código HTML, CSS y JavaScript debe estar bien estructurado, organizado de manera eficiente y comentado adecuadamente para legibilidad y mantenibilidad.",
  messages: [
    {
      "role": "user",
      "content": [
        {
          "type": "text",
          "text": "Crea un sitio web de una página para una plataforma de aprendizaje en línea llamada \"EduQuest\" con las siguientes características y secciones: \n \n1. Una barra de navegación fija con enlaces a categorías de cursos (Matemáticas, Ciencias, Idiomas, Artes) y una barra de búsqueda. \n \n2. Una sección hero con un fondo de video mostrando estudiantes aprendiendo en línea, un eslogan dinámico que rote entre \"Aprende a tu propio ritmo,\" \"Descubre nuevas pasiones,\" y \"Expande tus horizontes\" cada 3 segundos, y un botón \"Comenzar\" que lleve a un catálogo de cursos. \n \n3. Una sección de cursos destacados mostrando tarjetas de cursos con marcadores de posición para imágenes de cursos, títulos, instructores y descripciones. \n \n4. Una sección interactiva de \"Rutas de Aprendizaje\" con un cuestionario corto para determinar estilos de aprendizaje e intereses, y un botón para comenzar el cuestionario. \n \n5. Una sección de \"Historias de Éxito\" con testimonios de estudiantes satisfechos, con marcadores de posición para el texto del testimonio y nombres de estudiantes. \n \n6. Un pie de página con enlaces al blog de la plataforma, FAQ, política de privacidad, y un botón \"Contáctanos\" que abra una ventana modal con un formulario de contacto e información de soporte al cliente. \n \nIncluye contenido de marcador de posición de relleno para el fondo de video, tarjetas de cursos y testimonios. Integra los estilos CSS dentro de la etiqueta `<style>` en la sección `<head>` y coloca el código JavaScript dentro de la etiqueta `<script>` al final de la sección `<body>`. \n \nEl código JavaScript debe manejar el eslogan dinámico en la sección hero, rotando a través de los diferentes eslóganes cada 3 segundos."
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
  system="Tu tarea es crear un sitio web de una página basado en las especificaciones dadas, entregado como un archivo HTML con JavaScript y CSS integrados. El sitio web debe incorporar una variedad de características de diseño atractivas e interactivas, como menús desplegables, texto y contenido dinámico, botones clicables, y más. Asegúrate de que el diseño sea visualmente atractivo, responsivo y fácil de usar. El código HTML, CSS y JavaScript debe estar bien estructurado, organizado de manera eficiente y comentado adecuadamente para legibilidad y mantenibilidad.",
  messages=[
    {
      "role": "user",
      "content": [
        {
          "type": "text",
          "text": "Crea un sitio web de una página para una plataforma de aprendizaje en línea llamada \"EduQuest\" con las siguientes características y secciones: \n \n1. Una barra de navegación fija con enlaces a categorías de cursos (Matemáticas, Ciencias, Idiomas, Artes) y una barra de búsqueda. \n \n2. Una sección hero con un fondo de video mostrando estudiantes aprendiendo en línea, un eslogan dinámico que rote entre \"Aprende a tu propio ritmo,\" \"Descubre nuevas pasiones,\" y \"Expande tus horizontes\" cada 3 segundos, y un botón \"Comenzar\" que lleve a un catálogo de cursos. \n \n3. Una sección de cursos destacados mostrando tarjetas de cursos con marcadores de posición para imágenes de cursos, títulos, instructores y descripciones. \n \n4. Una sección interactiva de \"Rutas de Aprendizaje\" con un cuestionario corto para determinar estilos de aprendizaje e intereses, y un botón para comenzar el cuestionario. \n \n5. Una sección de \"Historias de Éxito\" con testimonios de estudiantes satisfechos, con marcadores de posición para el texto del testimonio y nombres de estudiantes. \n \n6. Un pie de página con enlaces al blog de la plataforma, FAQ, política de privacidad, y un botón \"Contáctanos\" que abra una ventana modal con un formulario de contacto e información de soporte al cliente. \n \nIncluye contenido de marcador de posición de relleno para el fondo de video, tarjetas de cursos y testimonios. Integra los estilos CSS dentro de la etiqueta `<style>` en la sección `<head>` y coloca el código JavaScript dentro de la etiqueta `<script>` al final de la sección `<body>`. \n \nEl código JavaScript debe manejar el eslogan dinámico en la sección hero, rotando a través de los diferentes eslóganes cada 3 segundos."
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
system: "Tu tarea es crear un sitio web de una página basado en las especificaciones dadas, entregado como un archivo HTML con JavaScript y CSS integrados. El sitio web debe incorporar una variedad de características de diseño atractivas e interactivas, como menús desplegables, texto y contenido dinámico, botones clicables, y más. Asegúrate de que el diseño sea visualmente atractivo, responsivo y fácil de usar. El código HTML, CSS y JavaScript debe estar bien estructurado, organizado de manera eficiente y comentado adecuadamente para legibilidad y mantenibilidad.",
messages: [
{
"role": "user",
"content": [
{
"type": "text",
"text": "Crea un sitio web de una página para una plataforma de aprendizaje en línea llamada \"EduQuest\" con las siguientes características y secciones: \n \n1. Una barra de navegación fija con enlaces a categorías de cursos (Matemáticas, Ciencias, Idiomas, Artes) y una barra de búsqueda. \n \n2. Una sección hero con un fondo de video mostrando estudiantes aprendiendo en línea, un eslogan dinámico que rote entre \"Aprende a tu propio ritmo,\" \"Descubre nuevas pasiones,\" y \"Expande tus horizontes\" cada 3 segundos, y un botón \"Comenzar\" que lleve a un catálogo de cursos. \n \n3. Una sección de cursos destacados mostrando tarjetas de cursos con marcadores de posición para imágenes de cursos, títulos, instructores y descripciones. \n \n4. Una sección interactiva de \"Rutas de Aprendizaje\" con un cuestionario corto para determinar estilos de aprendizaje e intereses, y un botón para comenzar el cuestionario. \n \n5. Una sección de \"Historias de Éxito\" con testimonios de estudiantes satisfechos, con marcadores de posición para el texto del testimonio y nombres de estudiantes. \n \n6. Un pie de página con enlaces al blog de la plataforma, FAQ, política de privacidad, y un botón \"Contáctanos\" que abra una ventana modal con un formulario de contacto e información de soporte al cliente. \n \nIncluye contenido de marcador de posición de relleno para el fondo de video, tarjetas de cursos y testimonios. Integra los estilos CSS dentro de la etiqueta `<style>` en la sección `<head>` y coloca el código JavaScript dentro de la etiqueta `<script>` al final de la sección `<body>`. \n \nEl código JavaScript debe manejar el eslogan dinámico en la sección hero, rotando a través de los diferentes eslóganes cada 3 segundos."
}
]
}
]
});
console.log(msg);

```

</Tab>
</Tabs>