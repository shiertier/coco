# Assistente de websites

Crie websites de uma página baseados nas especificações do usuário.

---

> Copie este prompt para nosso [Console](/dashboard) de desenvolvedor para experimentá-lo você mesmo!

|        | Conteúdo |
| --- | --- |
| Sistema | Sua tarefa é criar um website de uma página baseado nas especificações fornecidas, entregue como um arquivo HTML com JavaScript e CSS incorporados. O website deve incorporar uma variedade de recursos de design envolventes e interativos, como menus suspensos, texto e conteúdo dinâmicos, botões clicáveis e muito mais. Certifique-se de que o design seja visualmente atraente, responsivo e amigável ao usuário. O código HTML, CSS e JavaScript deve ser bem estruturado, eficientemente organizado e adequadamente comentado para legibilidade e manutenibilidade. |
| Usuário   | Crie um website de uma página para uma plataforma de aprendizado online chamada "EduQuest" com os seguintes recursos e seções:<br/><br/>1. Uma barra de navegação fixa com links para categorias de cursos (Matemática, Ciências, Idiomas, Artes) e uma barra de pesquisa.<br/>2. Uma seção hero com um fundo de vídeo mostrando estudantes aprendendo online, um slogan dinâmico que alterna entre "Aprenda no seu próprio ritmo", "Descubra novas paixões" e "Expanda seus horizontes" a cada 3 segundos, e um botão "Começar" levando a um catálogo de cursos.<br/>3. Uma seção de cursos em destaque exibindo cartões de cursos com espaços reservados para imagens de cursos, títulos, instrutores e descrições.<br/>4. Uma seção interativa "Caminhos de Aprendizado" com um quiz curto para determinar estilos de aprendizado e interesses, e um botão para iniciar o quiz.<br/>5. Uma seção "Histórias de Sucesso" apresentando depoimentos de estudantes satisfeitos, com espaços reservados para o texto do depoimento e nomes dos estudantes.<br/>6. Um rodapé com links para o blog da plataforma, FAQ, política de privacidade e um botão "Entre em Contato" que abre uma janela modal com um formulário de contato e informações de suporte ao cliente.<br/><br/>Inclua conteúdo de preenchimento para o fundo de vídeo, cartões de cursos e depoimentos. Incorpore os estilos CSS dentro da tag `<style>` na seção `<head>` e coloque o código JavaScript dentro da tag `<script>` no final da seção `<body>`.<br/><br/>O código JavaScript deve lidar com o slogan dinâmico na seção hero, alternando entre os diferentes slogans a cada 3 segundos. |

## Exemplo de saída

```html
<!DOCTYPE html>
<html lang="en">
<head>
    <meta charset="UTF-8">
    <meta name="viewport" content="width=device-width, initial-scale=1.0">
    <title>EduQuest - Plataforma de Aprendizado Online</title>
    <style>
        /* Estilos CSS */
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
                <li><a href="#">Matemática</a></li>
                <li><a href="#">Ciências</a></li>
                <li><a href="#">Idiomas</a></li>
                <li><a href="#">Artes</a></li>
                <li><input type="text" placeholder="Pesquisar"></li>
            </ul>
        </nav>
    </header>

    <section class="hero">
        <video src="placeholder-video.mp4" autoplay loop muted></video>
        <h1>Bem-vindo ao EduQuest</h1>
        <p id="tagline">Aprenda no seu próprio ritmo</p>
        <button>Começar</button>
    </section>

    <section class="featured-courses">
        <h2>Cursos em Destaque</h2>
        <div class="course-card">
            <img src="placeholder-course-image.jpg" alt="Imagem do Curso">
            <h3>Título do Curso</h3>
            <p>Instrutor: João Silva</p>
            <p>Descrição do curso vai aqui.</p>
        </div>
        <div class="course-card">
            <img src="placeholder-course-image.jpg" alt="Imagem do Curso">
            <h3>Título do Curso</h3>
            <p>Instrutor: Maria Santos</p>
            <p>Descrição do curso vai aqui.</p>
        </div>
    </section>

    <section class="success-stories">
        <h2>Histórias de Sucesso</h2>
        <div class="testimonial">
            <p>"O EduQuest me ajudou a aprender uma nova habilidade e avançar na minha carreira. Altamente recomendado!"</p>
            <p>- João Silva</p>
        </div>
        <div class="testimonial">
            <p>"Nunca pensei que poderia aprender um novo idioma online, mas o EduQuest tornou isso possível. Obrigado!"</p>
            <p>- Maria Santos</p>
        </div>
    </section>

    <footer>
        <a href="#">Blog</a>
        <a href="#">FAQ</a>
        <a href="#">Política de Privacidade</a>
        <button>Entre em Contato</button>
    </footer>

    <script>
        // Código JavaScript
        const taglines = [
            "Aprenda no seu próprio ritmo",
            "Descubra novas paixões",
            "Expanda seus horizontes"
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

> Este código cria um website de uma página para "EduQuest" com os seguintes recursos e seções:
> 
> Uma barra de navegação fixa com links para categorias de cursos (Matemática, Ciências, Idiomas, Artes) e uma barra de pesquisa.
> 
> Uma seção hero com um fundo de vídeo de espaço reservado, um slogan dinâmico que alterna entre "Aprenda no seu próprio ritmo", "Descubra novas paixões" e "Expanda seus horizontes" a cada 3 segundos, e um botão "Começar".
> 
> Uma seção de cursos em destaque exibindo cartões de cursos com espaços reservados para imagens de cursos, títulos, instrutores e descrições.
> 
> Uma seção "Histórias de Sucesso" apresentando depoimentos de estudantes satisfeitos, com espaços reservados para o texto do depoimento e nomes dos estudantes.
> 
> Um rodapé com links para o blog da plataforma, FAQ, política de privacidade e um botão "Entre em Contato".
> 
> Os estilos CSS estão incorporados dentro da tag `<style>` na seção `<head>`, e o código JavaScript está colocado dentro da tag `<script>` no final da seção `<body>`.
> 
> O código JavaScript lida com o slogan dinâmico na seção hero, alternando entre os diferentes slogans a cada 3 segundos usando a função setInterval.
> 
> Por favor, note que o fundo de vídeo, imagens de cursos e depoimentos são espaços reservados e devem ser substituídos por conteúdo real.

## Solicitação de API

<Tabs>
<Tab title="Python">

```python
import anthropic

client = anthropic.Anthropic( # padrão para os.environ.get("ANTHROPIC_API_KEY")
api_key="my_api_key",
)
message = client.messages.create(
  model="claude-sonnet-4-5",
  max_tokens=4000,
  temperature=0,
  system="Sua tarefa é criar um website de uma página baseado nas especificações fornecidas, entregue como um arquivo HTML com JavaScript e CSS incorporados. O website deve incorporar uma variedade de recursos de design envolventes e interativos, como menus suspensos, texto e conteúdo dinâmicos, botões clicáveis e muito mais. Certifique-se de que o design seja visualmente atraente, responsivo e amigável ao usuário. O código HTML, CSS e JavaScript deve ser bem estruturado, eficientemente organizado e adequadamente comentado para legibilidade e manutenibilidade.",
  messages=[
    {
      "role": "user",
      "content": [
        {
          "type": "text",
          "text": "Crie um website de uma página para uma plataforma de aprendizado online chamada \"EduQuest\" com os seguintes recursos e seções: \n \n1. Uma barra de navegação fixa com links para categorias de cursos (Matemática, Ciências, Idiomas, Artes) e uma barra de pesquisa. \n \n2. Uma seção hero com um fundo de vídeo mostrando estudantes aprendendo online, um slogan dinâmico que alterna entre \"Aprenda no seu próprio ritmo\", \"Descubra novas paixões\" e \"Expanda seus horizontes\" a cada 3 segundos, e um botão \"Começar\" levando a um catálogo de cursos. \n \n3. Uma seção de cursos em destaque exibindo cartões de cursos com espaços reservados para imagens de cursos, títulos, instrutores e descrições. \n \n4. Uma seção interativa \"Caminhos de Aprendizado\" com um quiz curto para determinar estilos de aprendizado e interesses, e um botão para iniciar o quiz. \n \n5. Uma seção \"Histórias de Sucesso\" apresentando depoimentos de estudantes satisfeitos, com espaços reservados para o texto do depoimento e nomes dos estudantes. \n \n6. Um rodapé com links para o blog da plataforma, FAQ, política de privacidade e um botão \"Entre em Contato\" que abre uma janela modal com um formulário de contato e informações de suporte ao cliente. \n \nInclua conteúdo de preenchimento para o fundo de vídeo, cartões de cursos e depoimentos. Incorpore os estilos CSS dentro da tag `<style>` na seção `<head>` e coloque o código JavaScript dentro da tag `<script>` no final da seção `<body>`. \n \nO código JavaScript deve lidar com o slogan dinâmico na seção hero, alternando entre os diferentes slogans a cada 3 segundos."
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
  apiKey: "my_api_key", // padrão para process.env["ANTHROPIC_API_KEY"]
});

const msg = await anthropic.messages.create({
  model: "claude-sonnet-4-5",
  max_tokens: 4000,
  temperature: 0,
  system: "Sua tarefa é criar um website de uma página baseado nas especificações fornecidas, entregue como um arquivo HTML com JavaScript e CSS incorporados. O website deve incorporar uma variedade de recursos de design envolventes e interativos, como menus suspensos, texto e conteúdo dinâmicos, botões clicáveis e muito mais. Certifique-se de que o design seja visualmente atraente, responsivo e amigável ao usuário. O código HTML, CSS e JavaScript deve ser bem estruturado, eficientemente organizado e adequadamente comentado para legibilidade e manutenibilidade.",
  messages: [
    {
      "role": "user",
      "content": [
        {
          "type": "text",
          "text": "Crie um website de uma página para uma plataforma de aprendizado online chamada \"EduQuest\" com os seguintes recursos e seções:  \n  \n1. Uma barra de navegação fixa com links para categorias de cursos (Matemática, Ciências, Idiomas, Artes) e uma barra de pesquisa.  \n  \n2. Uma seção hero com um fundo de vídeo mostrando estudantes aprendendo online, um slogan dinâmico que alterna entre \"Aprenda no seu próprio ritmo\", \"Descubra novas paixões\" e \"Expanda seus horizontes\" a cada 3 segundos, e um botão \"Começar\" levando a um catálogo de cursos.  \n  \n3. Uma seção de cursos em destaque exibindo cartões de cursos com espaços reservados para imagens de cursos, títulos, instrutores e descrições.  \n  \n4. Uma seção interativa \"Caminhos de Aprendizado\" com um quiz curto para determinar estilos de aprendizado e interesses, e um botão para iniciar o quiz.  \n  \n5. Uma seção \"Histórias de Sucesso\" apresentando depoimentos de estudantes satisfeitos, com espaços reservados para o texto do depoimento e nomes dos estudantes.  \n  \n6. Um rodapé com links para o blog da plataforma, FAQ, política de privacidade e um botão \"Entre em Contato\" que abre uma janela modal com um formulário de contato e informações de suporte ao cliente.  \n  \nInclua conteúdo de preenchimento para o fundo de vídeo, cartões de cursos e depoimentos. Incorpore os estilos CSS dentro da tag `<style>` na seção `<head>` e coloque o código JavaScript dentro da tag `<script>` no final da seção `<body>`.  \n  \nO código JavaScript deve lidar com o slogan dinâmico na seção hero, alternando entre os diferentes slogans a cada 3 segundos."
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

# Veja https://docs.claude.com/claude/reference/claude-on-amazon-bedrock

# para opções de autenticação

client = AnthropicBedrock()

message = client.messages.create(
  model="anthropic.claude-sonnet-4-5-20250929-v1:0",
  max_tokens=4000,
  temperature=0,
  system="Sua tarefa é criar um website de uma página baseado nas especificações fornecidas, entregue como um arquivo HTML com JavaScript e CSS incorporados. O website deve incorporar uma variedade de recursos de design envolventes e interativos, como menus suspensos, texto e conteúdo dinâmicos, botões clicáveis e muito mais. Certifique-se de que o design seja visualmente atraente, responsivo e amigável ao usuário. O código HTML, CSS e JavaScript deve ser bem estruturado, eficientemente organizado e adequadamente comentado para legibilidade e manutenibilidade.",
  messages=[
    {
      "role": "user",
      "content": [
        {
          "type": "text",
          "text": "Crie um website de uma página para uma plataforma de aprendizado online chamada \"EduQuest\" com os seguintes recursos e seções: \n \n1. Uma barra de navegação fixa com links para categorias de cursos (Matemática, Ciências, Idiomas, Artes) e uma barra de pesquisa. \n \n2. Uma seção hero com um fundo de vídeo mostrando estudantes aprendendo online, um slogan dinâmico que alterna entre \"Aprenda no seu próprio ritmo\", \"Descubra novas paixões\" e \"Expanda seus horizontes\" a cada 3 segundos, e um botão \"Começar\" levando a um catálogo de cursos. \n \n3. Uma seção de cursos em destaque exibindo cartões de cursos com espaços reservados para imagens de cursos, títulos, instrutores e descrições. \n \n4. Uma seção interativa \"Caminhos de Aprendizado\" com um quiz curto para determinar estilos de aprendizado e interesses, e um botão para iniciar o quiz. \n \n5. Uma seção \"Histórias de Sucesso\" apresentando depoimentos de estudantes satisfeitos, com espaços reservados para o texto do depoimento e nomes dos estudantes. \n \n6. Um rodapé com links para o blog da plataforma, FAQ, política de privacidade e um botão \"Entre em Contato\" que abre uma janela modal com um formulário de contato e informações de suporte ao cliente. \n \nInclua conteúdo de preenchimento para o fundo de vídeo, cartões de cursos e depoimentos. Incorpore os estilos CSS dentro da tag `<style>` na seção `<head>` e coloque o código JavaScript dentro da tag `<script>` no final da seção `<body>`. \n \nO código JavaScript deve lidar com o slogan dinâmico na seção hero, alternando entre os diferentes slogans a cada 3 segundos."
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

// Veja https://docs.claude.com/claude/reference/claude-on-amazon-bedrock
// para opções de autenticação
const client = new AnthropicBedrock();

const msg = await client.messages.create({
  model: "anthropic.claude-sonnet-4-5-20250929-v1:0",
  max_tokens: 4000,
  temperature: 0,
  system: "Sua tarefa é criar um website de uma página baseado nas especificações fornecidas, entregue como um arquivo HTML com JavaScript e CSS incorporados. O website deve incorporar uma variedade de recursos de design envolventes e interativos, como menus suspensos, texto e conteúdo dinâmicos, botões clicáveis e muito mais. Certifique-se de que o design seja visualmente atraente, responsivo e amigável ao usuário. O código HTML, CSS e JavaScript deve ser bem estruturado, eficientemente organizado e adequadamente comentado para legibilidade e manutenibilidade.",
  messages: [
    {
      "role": "user",
      "content": [
        {
          "type": "text",
          "text": "Crie um website de uma página para uma plataforma de aprendizado online chamada \"EduQuest\" com os seguintes recursos e seções: \n \n1. Uma barra de navegação fixa com links para categorias de cursos (Matemática, Ciências, Idiomas, Artes) e uma barra de pesquisa. \n \n2. Uma seção hero com um fundo de vídeo mostrando estudantes aprendendo online, um slogan dinâmico que alterna entre \"Aprenda no seu próprio ritmo\", \"Descubra novas paixões\" e \"Expanda seus horizontes\" a cada 3 segundos, e um botão \"Começar\" levando a um catálogo de cursos. \n \n3. Uma seção de cursos em destaque exibindo cartões de cursos com espaços reservados para imagens de cursos, títulos, instrutores e descrições. \n \n4. Uma seção interativa \"Caminhos de Aprendizado\" com um quiz curto para determinar estilos de aprendizado e interesses, e um botão para iniciar o quiz. \n \n5. Uma seção \"Histórias de Sucesso\" apresentando depoimentos de estudantes satisfeitos, com espaços reservados para o texto do depoimento e nomes dos estudantes. \n \n6. Um rodapé com links para o blog da plataforma, FAQ, política de privacidade e um botão \"Entre em Contato\" que abre uma janela modal com um formulário de contato e informações de suporte ao cliente. \n \nInclua conteúdo de preenchimento para o f undo de vídeo, cartões de cursos e depoimentos. Incorpore os estilos CSS dentro da tag `<style>` na seção `<head>` e coloque o código JavaScript dentro da tag `<script>` no final da seção `<body>`. \n \nO código JavaScript deve lidar com o slogan dinâmico na seção hero, alternando entre os diferentes slogans a cada 3 segundos."
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
  system="Sua tarefa é criar um website de uma página baseado nas especificações fornecidas, entregue como um arquivo HTML com JavaScript e CSS incorporados. O website deve incorporar uma variedade de recursos de design envolventes e interativos, como menus suspensos, texto e conteúdo dinâmicos, botões clicáveis e muito mais. Certifique-se de que o design seja visualmente atraente, responsivo e amigável ao usuário. O código HTML, CSS e JavaScript deve ser bem estruturado, eficientemente organizado e adequadamente comentado para legibilidade e manutenibilidade.",
  messages=[
    {
      "role": "user",
      "content": [
        {
          "type": "text",
          "text": "Crie um website de uma página para uma plataforma de aprendizado online chamada \"EduQuest\" com os seguintes recursos e seções: \n \n1. Uma barra de navegação fixa com links para categorias de cursos (Matemática, Ciências, Idiomas, Artes) e uma barra de pesquisa. \n \n2. Uma seção hero com um fundo de vídeo mostrando estudantes aprendendo online, um slogan dinâmico que alterna entre \"Aprenda no seu próprio ritmo\", \"Descubra novas paixões\" e \"Expanda seus horizontes\" a cada 3 segundos, e um botão \"Começar\" levando a um catálogo de cursos. \n \n3. Uma seção de cursos em destaque exibindo cartões de cursos com espaços reservados para imagens de cursos, títulos, instrutores e descrições. \n \n4. Uma seção interativa \"Caminhos de Aprendizado\" com um quiz curto para determinar estilos de aprendizado e interesses, e um botão para iniciar o quiz. \n \n5. Uma seção \"Histórias de Sucesso\" apresentando depoimentos de estudantes satisfeitos, com espaços reservados para o texto do depoimento e nomes dos estudantes. \n \n6. Um rodapé com links para o blog da plataforma, FAQ, política de privacidade e um botão \"Entre em Contato\" que abre uma janela modal com um formulário de contato e informações de suporte ao cliente. \n \nInclua conteúdo de preenchimento para o fundo de vídeo, cartões de cursos e depoimentos. Incorpore os estilos CSS dentro da tag `<style>` na seção `<head>` e coloque o código JavaScript dentro da tag `<script>` no final da seção `<body>`. \n \nO código JavaScript deve lidar com o slogan dinâmico na seção hero, alternando entre os diferentes slogans a cada 3 segundos."
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

// Lê das variáveis de ambiente `CLOUD_ML_REGION` & `ANTHROPIC_VERTEX_PROJECT_ID`.
// Adicionalmente passa pelo fluxo padrão da `google-auth-library`.
const client = new AnthropicVertex();

const msg = await client.messages.create({
model: "claude-sonnet-4@20250514",
max_tokens: 4000,
temperature: 0,
system: "Sua tarefa é criar um website de uma página baseado nas especificações fornecidas, entregue como um arquivo HTML com JavaScript e CSS incorporados. O website deve incorporar uma variedade de recursos de design envolventes e interativos, como menus suspensos, texto e conteúdo dinâmicos, botões clicáveis e muito mais. Certifique-se de que o design seja visualmente atraente, responsivo e amigável ao usuário. O código HTML, CSS e JavaScript deve ser bem estruturado, eficientemente organizado e adequadamente comentado para legibilidade e manutenibilidade.",
messages: [
{
"role": "user",
"content": [
{
"type": "text",
"text": "Crie um website de uma página para uma plataforma de aprendizado online chamada \"EduQuest\" com os seguintes recursos e seções: \n \n1. Uma barra de navegação fixa com links para categorias de cursos (Matemática, Ciências, Idiomas, Artes) e uma barra de pesquisa. \n \n2. Uma seção hero com um fundo de vídeo mostrando estudantes aprendendo online, um slogan dinâmico que alterna entre \"Aprenda no seu próprio ritmo\", \"Descubra novas paixões\" e \"Expanda seus horizontes\" a cada 3 segundos, e um botão \"Começar\" levando a um catálogo de cursos. \n \n3. Uma seção de cursos em destaque exibindo cartões de cursos com espaços reservados para imagens de cursos, títulos, instrutores e descrições. \n \n4. Uma seção interativa \"Caminhos de Aprendizado\" com um quiz curto para determinar estilos de aprendizado e interesses, e um botão para iniciar o quiz. \n \n5. Uma seção \"Histórias de Sucesso\" apresentando depoimentos de estudantes satisfeitos, com espaços reservados para o texto do depoimento e nomes dos estudantes. \n \n6. Um rodapé com links para o blog da plataforma, FAQ, política de privacidade e um botão \"Entre em Contato\" que abre uma janela modal com um formulário de contato e informações de suporte ao cliente. \n \nInclua conteúdo de preenchimento para o fundo de vídeo, cartões de cursos e depoimentos. Incorpore os estilos CSS dentro da tag `<style>` na seção `<head>` e coloque o código JavaScript dentro da tag `<script>` no final da seção `<body>`. \n \nO código JavaScript deve lidar com o slogan dinâmico na seção hero, alternando entre os diferentes slogans a cada 3 segundos."
}
]
}
]
});
console.log(msg);

```

</Tab>
</Tabs>