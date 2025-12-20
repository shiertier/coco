# Criador de perguntas de entrevista

Gere perguntas para entrevistas.

---

> Copie este prompt para nosso [Console](/dashboard) de desenvolvedor para experimentá-lo você mesmo!

|        | Conteúdo |
| --- | --- |
| Sistema | Sua tarefa é gerar uma série de perguntas reflexivas e abertas para uma entrevista baseada no contexto fornecido. As perguntas devem ser projetadas para extrair respostas perspicazes e detalhadas do entrevistado, permitindo que demonstrem seu conhecimento, experiência e habilidades de pensamento crítico. Evite perguntas de sim/não ou aquelas com respostas óbvias. Em vez disso, concentre-se em perguntas que encorajem reflexão, autoavaliação e o compartilhamento de exemplos específicos ou anedotas. |
| Usuário   | Você está entrevistando um candidato para uma posição de gerente de marketing em uma empresa de e-commerce bem estabelecida. O candidato ideal deve ter experiência no desenvolvimento e execução de campanhas de marketing multicanal, fortes habilidades analíticas e a capacidade de colaborar efetivamente com equipes multifuncionais. Eles também devem ser apaixonados por se manter atualizados com as últimas tendências e tecnologias de marketing. |

## Exemplo de Saída

1.  Você pode me explicar uma campanha de marketing multicanal bem-sucedida que você desenvolveu e executou em seu papel anterior? Quais foram os desafios que você enfrentou e como os superou?
2.  Como você mede o sucesso de suas campanhas de marketing e em quais métricas você se concentra? Você pode fornecer um exemplo de como usou dados e análises para otimizar o desempenho de uma campanha?
3.  Em sua experiência, quais são os elementos-chave de uma estratégia de marketing de e-commerce bem-sucedida? Como você garante que suas estratégias se alinhem com os objetivos gerais de negócios da empresa?
4.  Você pode compartilhar um exemplo de uma ocasião em que teve que colaborar com uma equipe multifuncional (por exemplo, produto, vendas ou atendimento ao cliente) para alcançar um objetivo de marketing? Quais desafios você encontrou e como promoveu uma colaboração efetiva?
5.  Como você se mantém informado sobre as últimas tendências e tecnologias de marketing, e como aplicou esse conhecimento para melhorar suas estratégias ou campanhas de marketing?
6.  Você pode descrever uma ocasião em que uma campanha de marketing da qual você participou não teve o desempenho esperado? O que você aprendeu com a experiência e como aplicou essas lições a campanhas futuras?
7.  Nossa empresa valoriza a inovação e a melhoria contínua. Você pode compartilhar um exemplo de uma ocasião em que introduziu uma nova abordagem de marketing, ferramenta ou tecnologia que impactou significativamente o desempenho ou os resultados de sua equipe?

---

## Solicitação de API

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
  max_tokens=1000,
  temperature=0.5,
  system="Sua tarefa é gerar uma série de perguntas reflexivas e abertas para uma entrevista baseada no contexto fornecido. As perguntas devem ser projetadas para extrair respostas perspicazes e detalhadas do entrevistado, permitindo que demonstrem seu conhecimento, experiência e habilidades de pensamento crítico. Evite perguntas de sim/não ou aquelas com respostas óbvias. Em vez disso, concentre-se em perguntas que encorajem reflexão, autoavaliação e o compartilhamento de exemplos específicos ou anedotas.",
  messages=[
    {
      "role": "user",
      "content": [
        {
          "type": "text",
          "text": "Você está entrevistando um candidato para uma posição de gerente de marketing em uma empresa de e-commerce bem estabelecida. O candidato ideal deve ter experiência no desenvolvimento e execução de campanhas de marketing multicanal, fortes habilidades analíticas e a capacidade de colaborar efetivamente com equipes multifuncionais. Eles também devem ser apaixonados por se manter atualizados com as últimas tendências e tecnologias de marketing."
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
  max_tokens: 1000,
  temperature: 0.5,
  system: "Sua tarefa é gerar uma série de perguntas reflexivas e abertas para uma entrevista baseada no contexto fornecido. As perguntas devem ser projetadas para extrair respostas perspicazes e detalhadas do entrevistado, permitindo que demonstrem seu conhecimento, experiência e habilidades de pensamento crítico. Evite perguntas de sim/não ou aquelas com respostas óbvias. Em vez disso, concentre-se em perguntas que encorajem reflexão, autoavaliação e o compartilhamento de exemplos específicos ou anedotas.",
  messages: [
    {
      "role": "user",
      "content": [
        {
          "type": "text",
          "text": "Você está entrevistando um candidato para uma posição de gerente de marketing em uma empresa de e-commerce bem estabelecida. O candidato ideal deve ter experiência no desenvolvimento e execução de campanhas de marketing multicanal, fortes habilidades analíticas e a capacidade de colaborar efetivamente com equipes multifuncionais. Eles também devem ser apaixonados por se manter atualizados com as últimas tendências e tecnologias de marketing."
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
    max_tokens=1000,
    temperature=0.5,
    system="Sua tarefa é gerar uma série de perguntas reflexivas e abertas para uma entrevista baseada no contexto fornecido. As perguntas devem ser projetadas para extrair respostas perspicazes e detalhadas do entrevistado, permitindo que demonstrem seu conhecimento, experiência e habilidades de pensamento crítico. Evite perguntas de sim/não ou aquelas com respostas óbvias. Em vez disso, concentre-se em perguntas que encorajem reflexão, autoavaliação e o compartilhamento de exemplos específicos ou anedotas.",
    messages=[
        {
            "role": "user",
            "content": [
                {
                    "type": "text",
                    "text": "Você está entrevistando um candidato para uma posição de gerente de marketing em uma empresa de e-commerce bem estabelecida. O candidato ideal deve ter experiência no desenvolvimento e execução de campanhas de marketing multicanal, fortes habilidades analíticas e a capacidade de colaborar efetivamente com equipes multifuncionais. Eles também devem ser apaixonados por se manter atualizados com as últimas tendências e tecnologias de marketing."
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
import AnthropicBedrock from "@anthropic-ai/bedrock-sdk";

// See https://docs.claude.com/claude/reference/claude-on-amazon-bedrock
// for authentication options
const client = new AnthropicBedrock();

const msg = await client.messages.create({
model: "anthropic.claude-sonnet-4-5-20250929-v1:0",
max_tokens: 1000,
temperature: 0.5,
system: "Sua tarefa é gerar uma série de perguntas reflexivas e abertas para uma entrevista baseada no contexto fornecido. As perguntas devem ser projetadas para extrair respostas perspicazes e detalhadas do entrevistado, permitindo que demonstrem seu conhecimento, experiência e habilidades de pensamento crítico. Evite perguntas de sim/não ou aquelas com respostas óbvias. Em vez disso, concentre-se em perguntas que encorajem reflexão, autoavaliação e o compartilhamento de exemplos específicos ou anedotas.",
messages: [
{
"role": "user",
"content": [
{
"type": "text",
"text": "Você está entrevistando um candidato para uma posição de gerente de marketing em uma empresa de e-commerce bem estabelecida. O candidato ideal deve ter experiência no desenvolvimento e execução de campanhas de marketing multicanal, fortes habilidades analíticas e a capacidade de colaborar efetivamente com equipes multifuncionais. Eles também devem ser apaixonados por se manter atualizados com as últimas tendências e tecnologias de marketing."
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
    max_tokens=1000,
    temperature=0.5,
    system="Sua tarefa é gerar uma série de perguntas reflexivas e abertas para uma entrevista baseada no contexto fornecido. As perguntas devem ser projetadas para extrair respostas perspicazes e detalhadas do entrevistado, permitindo que demonstrem seu conhecimento, experiência e habilidades de pensamento crítico. Evite perguntas de sim/não ou aquelas com respostas óbvias. Em vez disso, concentre-se em perguntas que encorajem reflexão, autoavaliação e o compartilhamento de exemplos específicos ou anedotas.",
    messages=[
        {
            "role": "user",
            "content": [
                {
                    "type": "text",
                    "text": "Você está entrevistando um candidato para uma posição de gerente de marketing em uma empresa de e-commerce bem estabelecida. O candidato ideal deve ter experiência no desenvolvimento e execução de campanhas de marketing multicanal, fortes habilidades analíticas e a capacidade de colaborar efetivamente com equipes multifuncionais. Eles também devem ser apaixonados por se manter atualizados com as últimas tendências e tecnologias de marketing."
                }
            ]
        }
    ]
)
print(message.content)

````

</Tab>
<Tab title="Vertex AI TypeScript">

```typescript
import { AnthropicVertex } from '@anthropic-ai/vertex-sdk';

// Reads from the `CLOUD_ML_REGION` & `ANTHROPIC_VERTEX_PROJECT_ID` environment variables.
// Additionally goes through the standard `google-auth-library` flow.
const client = new AnthropicVertex();

const msg = await client.messages.create({
  model: "claude-sonnet-4@20250514",
  max_tokens: 1000,
  temperature: 0.5,
  system: "Sua tarefa é gerar uma série de perguntas reflexivas e abertas para uma entrevista baseada no contexto fornecido. As perguntas devem ser projetadas para extrair respostas perspicazes e detalhadas do entrevistado, permitindo que demonstrem seu conhecimento, experiência e habilidades de pensamento crítico. Evite perguntas de sim/não ou aquelas com respostas óbvias. Em vez disso, concentre-se em perguntas que encorajem reflexão, autoavaliação e o compartilhamento de exemplos específicos ou anedotas.",
  messages: [
    {
      "role": "user",
      "content": [
        {
          "type": "text",
          "text": "Você está entrevistando um candidato para uma posição de gerente de marketing em uma empresa de e-commerce bem estabelecida. O candidato ideal deve ter experiência no desenvolvimento e execução de campanhas de marketing multicanal, fortes habilidades analíticas e a capacidade de colaborar efetivamente com equipes multifuncionais. Eles também devem ser apaixonados por se manter atualizados com as últimas tendências e tecnologias de marketing."
        }
      ]
    }
  ]
});
console.log(msg);

```

</Tab>
</Tabs>