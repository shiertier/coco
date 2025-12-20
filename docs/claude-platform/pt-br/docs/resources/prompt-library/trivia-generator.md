# Gerador de perguntas de curiosidades

Gere perguntas de curiosidades sobre uma ampla gama de tópicos e forneça dicas quando necessário.

---

> Copie este prompt para nosso [Console](/dashboard) de desenvolvedor para experimentá-lo você mesmo!

| Papel | Conteúdo |
| ---- | --- |
| Usuário | Gere perguntas de curiosidades sobre vários tópicos e forneça dicas para ajudar os usuários a chegar à resposta correta. Selecione de um conjunto diverso de categorias e crie perguntas que testem o conhecimento ou habilidades de raciocínio do usuário. Ofereça uma série de dicas cada vez mais específicas para guiar os usuários em direção à solução. Certifique-se de que as perguntas sejam desafiadoras e que as dicas forneçam informações suficientes para ajudar o usuário sem revelar a resposta muito facilmente. |

## Exemplo de Saída

Categoria: Geografia

Pergunta:  
 Qual é o único país do mundo que faz fronteira tanto com a França quanto com a Espanha?

Dica 1: Este país está localizado nas montanhas dos Pirineus entre a França e a Espanha.  
 Dica 2: É conhecido por suas paisagens montanhosas deslumbrantes, estações de esqui e compras livres de impostos.  
 Dica 3: A capital do país apresenta o nome do próprio país.

Resposta: Andorra

Categoria: Ciência

Pergunta:  
 Qual é o processo pelo qual as plantas convertem luz solar, água e dióxido de carbono em energia e oxigênio?

Dica 1: Este processo é essencial para a vida na Terra, pois fornece tanto alimento quanto oxigênio.  
 Dica 2: Ocorre principalmente nas folhas das plantas.  
 Dica 3: O processo envolve clorofila, um pigmento verde que absorve a luz solar.

Resposta: Fotossíntese

Categoria: Literatura

Pergunta:  
 Quem escreveu o romance clássico "Orgulho e Preconceito", que foi publicado pela primeira vez em 1813?

Dica 1: A autora é uma mulher britânica que viveu de 1775 a 1817.  
 Dica 2: Ela é conhecida por seus retratos espirituosos e perspicazes da sociedade e costumes ingleses.  
 Dica 3: Suas outras obras famosas incluem "Razão e Sensibilidade" e "Emma".

Resposta: Jane Austen

Categoria: História

Pergunta:  
 Em que ano o Muro de Berlim, que dividia a Alemanha Oriental e Ocidental, começou a ser derrubado?

Dica 1: O evento ocorreu no final do século XX.  
 Dica 2: Marcou um momento significativo no fim da Guerra Fria.  
 Dica 3: O ano é frequentemente associado à frase "queda do comunismo" na Europa.

Resposta: 1989

---

## Solicitação da API

<Tabs>
<Tab title="Python">

```python
import anthropic

client = anthropic.Anthropic(
  # defaults to os.environ.get("ANTHROPIC_API_KEY")
  api_key="my_api_key",
)
message = client.messages.create(
  model="claude-sonnet-4-5",
  max_tokens=2000,
  temperature=0.5,
  messages=[
    {
    "role": "user",
    "content": [
        {
          "type": "text",
          "text": "Gere perguntas de curiosidades sobre vários tópicos e forneça dicas para ajudar os usuários a chegar à resposta correta. Selecione de um conjunto diverso de categorias e crie perguntas que testem o conhecimento ou habilidades de raciocínio do usuário. Ofereça uma série de dicas cada vez mais específicas para guiar os usuários em direção à solução. Certifique-se de que as perguntas sejam desafiadoras e que as dicas forneçam informações suficientes para ajudar o usuário sem revelar a resposta muito facilmente."
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
  max_tokens: 2000,
  temperature: 0.5,
  messages: [
    {
      "role": "user",
      "content": [
        {
          "type": "text",
          "text": "Gere perguntas de curiosidades sobre vários tópicos e forneça dicas para ajudar os usuários a chegar à resposta correta. Selecione de um conjunto diverso de categorias e crie perguntas que testem o conhecimento ou habilidades de raciocínio do usuário. Ofereça uma série de dicas cada vez mais específicas para guiar os usuários em direção à solução. Certifique-se de que as perguntas sejam desafiadoras e que as dicas forneçam informações suficientes para ajudar o usuário sem revelar a resposta muito facilmente."
        }
      ]
    }
  ]
});
console.log(msg);

````

</Tab>
<Tab title="AWS Bedrock Python">
```python
from anthropic import AnthropicBedrock

# See https://docs.claude.com/claude/reference/claude-on-amazon-bedrock

# for authentication options

client = AnthropicBedrock()

message = client.messages.create(
model="anthropic.claude-sonnet-4-5-20250929-v1:0",
max_tokens=2000,
temperature=0.5,
messages=[
{
"role": "user",
"content": [
{
"type": "text",
"text": "Gere perguntas de curiosidades sobre vários tópicos e forneça dicas para ajudar os usuários a chegar à resposta correta. Selecione de um conjunto diverso de categorias e crie perguntas que testem o conhecimento ou habilidades de raciocínio do usuário. Ofereça uma série de dicas cada vez mais específicas para guiar os usuários em direção à solução. Certifique-se de que as perguntas sejam desafiadoras e que as dicas forneçam informações suficientes para ajudar o usuário sem revelar a resposta muito facilmente."
}
]
}
]
)
print(message.content)

````
</Tab>
<Tab title="AWS Bedrock TypeScript">

```typescript
import AnthropicBedrock from "@anthropic-ai/bedrock-sdk";

// See https://docs.claude.com/claude/reference/claude-on-amazon-bedrock
// for authentication options
const client = new AnthropicBedrock();

const msg = await client.messages.create({
  model: "anthropic.claude-sonnet-4-5-20250929-v1:0",
  max_tokens: 2000,
  temperature: 0.5,
  messages: [
    {
      "role": "user",
      "content": [
        {
          "type": "text",
          "text": "Gere perguntas de curiosidades sobre vários tópicos e forneça dicas para ajudar os usuários a chegar à resposta correta. Selecione de um conjunto diverso de categorias e crie perguntas que testem o conhecimento ou habilidades de raciocínio do usuário. Ofereça uma série de dicas cada vez mais específicas para guiar os usuários em direção à solução. Certifique-se de que as perguntas sejam desafiadoras e que as dicas forneçam informações suficientes para ajudar o usuário sem revelar a resposta muito facilmente."
        }
      ]
    }
  ]
});
console.log(msg);

````

</Tab>
<Tab title="Vertex AI Python">

```python
from anthropic import AnthropicVertex

client = AnthropicVertex()

message = client.messages.create(
    model="claude-sonnet-4@20250514",
    max_tokens=2000,
    temperature=0.5,
    messages=[
        {
            "role": "user",
            "content": [
                {
                    "type": "text",
                    "text": "Gere perguntas de curiosidades sobre vários tópicos e forneça dicas para ajudar os usuários a chegar à resposta correta. Selecione de um conjunto diverso de categorias e crie perguntas que testem o conhecimento ou habilidades de raciocínio do usuário. Ofereça uma série de dicas cada vez mais específicas para guiar os usuários em direção à solução. Certifique-se de que as perguntas sejam desafiadoras e que as dicas forneçam informações suficientes para ajudar o usuário sem revelar a resposta muito facilmente."
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
  max_tokens: 2000,
  temperature: 0.5,
  messages: [
    {
      "role": "user",
      "content": [
        {
          "type": "text",
          "text": "Gere perguntas de curiosidades sobre vários tópicos e forneça dicas para ajudar os usuários a chegar à resposta correta. Selecione de um conjunto diverso de categorias e crie perguntas que testem o conhecimento ou habilidades de raciocínio do usuário. Ofereça uma série de dicas cada vez mais específicas para guiar os usuários em direção à solução. Certifique-se de que as perguntas sejam desafiadoras e que as dicas forneçam informações suficientes para ajudar o usuário sem revelar a resposta muito facilmente."
        }
      ]
    }
  ]
});
console.log(msg);

```

</Tab>
</Tabs>