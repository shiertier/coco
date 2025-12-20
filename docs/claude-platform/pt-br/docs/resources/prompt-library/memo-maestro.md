# Mestre de memorandos

Componha memorandos abrangentes da empresa com base em pontos-chave.

---

> Copie este prompt para nosso [Console](/dashboard) de desenvolvedor para experimentá-lo você mesmo!

|        | Conteúdo |
| --- | --- |
| System | Sua tarefa é compor um memorando abrangente da empresa com base nos pontos-chave fornecidos. O memorando deve ser escrito em um tom profissional, abordando todas as informações relevantes de maneira clara e concisa. Use formatação apropriada, como títulos, subtítulos e marcadores, para organizar o conteúdo de forma eficaz. Certifique-se de que o memorando seja bem estruturado, coerente e fácil de entender para o público-alvo. |
| User   | - Anúncio de um novo programa de bem-estar dos funcionários: "Fit4Success" <br/> - Objetivos do programa: promover bem-estar físico e mental, reduzir o estresse e aumentar a produtividade <br/> - Componentes: aulas de fitness no local, opções de refeições saudáveis, recursos de saúde mental e workshops de bem-estar <br/> - Parceria com academias locais e centros de bem-estar para associações com desconto <br/> - Incentivos para participação: rastreadores de fitness, pontos de bem-estar e sorteios de prêmios <br/> - Data de lançamento do programa: 1º de junho de 2023 <br/> - Processo de inscrição: registro online através da intranet da empresa <br/> - Coordenadores do programa: equipes de Recursos Humanos e Engajamento de Funcionários |

## Exemplo de Saída

Assunto: Apresentando o Fit4Success - Nosso Novo Programa de Bem-Estar dos Funcionários

Olá pessoal,

Estou empolgado em anunciar o lançamento do nosso novo programa de bem-estar dos funcionários, Fit4Success, projetado para promover bem-estar físico e mental, reduzir o estresse e aumentar a produtividade entre os membros da nossa equipe. O programa começará oficialmente em 1º de junho de 2023.

Objetivos do Programa:

- Encorajar atividade física regular e escolhas de estilo de vida saudável
- Fornecer recursos e apoio para saúde mental e gerenciamento de estresse
- Promover um ambiente de trabalho positivo e envolvente que priorize o bem-estar dos funcionários
- Melhorar a produtividade geral e a satisfação no trabalho

Componentes do Programa:

- Aulas de fitness no local: sessões de yoga, Pilates e treinamento intervalado de alta intensidade (HIIT)
- Opções de refeições saudáveis: lanches nutritivos e refeições disponíveis no refeitório da empresa
- Recursos de saúde mental: acesso a serviços de aconselhamento, aplicativos de meditação e workshops de alívio do estresse
- Workshops de bem-estar: seminários sobre tópicos como nutrição, higiene do sono e equilíbrio trabalho-vida

Parcerias e Descontos:  
 Para apoiar ainda mais sua jornada de bem-estar, fizemos parceria com academias locais e centros de bem-estar para oferecer associações com desconto exclusivamente para nossos funcionários. Aproveite essas tarifas especiais para acessar uma ampla gama de instalações de fitness e aulas.

Incentivos para Participação:  
 Para encorajar a participação ativa no programa Fit4Success, ofereceremos os seguintes incentivos:

- Rastreadores de fitness para todos os participantes monitorarem seu progresso e estabelecerem metas pessoais
- Pontos de bem-estar ganhos através da participação no programa, resgatáveis por recompensas adicionais
- Sorteios mensais de prêmios para participantes que atingirem marcos específicos de bem-estar

Processo de Inscrição:  
 Para se inscrever no programa Fit4Success, visite nossa intranet da empresa e complete o formulário de registro online. O prazo para inscrição é 25 de maio de 2023. Se você tiver alguma dúvida ou preocupação sobre o processo de inscrição, entre em contato com nossa equipe de Recursos Humanos.

Coordenação do Programa:  
 O programa Fit4Success será coordenado em conjunto por nossas equipes de Recursos Humanos e Engajamento de Funcionários. Eles estarão disponíveis para fornecer orientação, apoio e recursos ao longo de sua jornada de bem-estar.

Acreditamos firmemente que investir no bem-estar de nossos funcionários é crucial para o sucesso e crescimento de nossa organização. Encorajamos você a aproveitar esta oportunidade empolgante e se juntar a nós na criação de um local de trabalho mais saudável, feliz e produtivo.

Vamos abraçar o programa Fit4Success e trabalhar juntos em direção a um futuro mais saudável!

Atenciosamente,  
 \[Seu Nome\]  
 \[Seu Cargo\]

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
    max_tokens=1000,
    temperature=1,
    system="Sua tarefa é compor um memorando abrangente da empresa com base nos pontos-chave fornecidos. O memorando deve ser escrito em um tom profissional, abordando todas as informações relevantes de maneira clara e concisa. Use formatação apropriada, como títulos, subtítulos e marcadores, para organizar o conteúdo de forma eficaz. Certifique-se de que o memorando seja bem estruturado, coerente e fácil de entender para o público-alvo.",
    messages=[
        {
            "role": "user",
            "content": [
                {
                    "type": "text",
                    "text": "- Anúncio de um novo programa de bem-estar dos funcionários: \"Fit4Success\"  \n- Objetivos do programa: promover bem-estar físico e mental, reduzir o estresse e aumentar a produtividade  \n- Componentes: aulas de fitness no local, opções de refeições saudáveis, recursos de saúde mental e workshops de bem-estar  \n- Parceria com academias locais e centros de bem-estar para associações com desconto  \n- Incentivos para participação: rastreadores de fitness, pontos de bem-estar e sorteios de prêmios  \n- Data de lançamento do programa: 1º de junho de 2023  \n- Processo de inscrição: registro online através da intranet da empresa  \n- Coordenadores do programa: equipes de Recursos Humanos e Engajamento de Funcionários"
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
  temperature: 1,
  system: "Sua tarefa é compor um memorando abrangente da empresa com base nos pontos-chave fornecidos. O memorando deve ser escrito em um tom profissional, abordando todas as informações relevantes de maneira clara e concisa. Use formatação apropriada, como títulos, subtítulos e marcadores, para organizar o conteúdo de forma eficaz. Certifique-se de que o memorando seja bem estruturado, coerente e fácil de entender para o público-alvo.",
  messages: [
    {
      "role": "user",
      "content": [
        {
          "type": "text",
          "text": "- Anúncio de um novo programa de bem-estar dos funcionários: \"Fit4Success\"  \n- Objetivos do programa: promover bem-estar físico e mental, reduzir o estresse e aumentar a produtividade  \n- Componentes: aulas de fitness no local, opções de refeições saudáveis, recursos de saúde mental e workshops de bem-estar  \n- Parceria com academias locais e centros de bem-estar para associações com desconto  \n- Incentivos para participação: rastreadores de fitness, pontos de bem-estar e sorteios de prêmios  \n- Data de lançamento do programa: 1º de junho de 2023  \n- Processo de inscrição: registro online através da intranet da empresa  \n- Coordenadores do programa: equipes de Recursos Humanos e Engajamento de Funcionários"
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
max_tokens=1000,
temperature=1,
system="Sua tarefa é compor um memorando abrangente da empresa com base nos pontos-chave fornecidos. O memorando deve ser escrito em um tom profissional, abordando todas as informações relevantes de maneira clara e concisa. Use formatação apropriada, como títulos, subtítulos e marcadores, para organizar o conteúdo de forma eficaz. Certifique-se de que o memorando seja bem estruturado, coerente e fácil de entender para o público-alvo.",
messages=[
{
"role": "user",
"content": [
{
"type": "text",
"text": "- Anúncio de um novo programa de bem-estar dos funcionários: \"Fit4Success\" \n- Objetivos do programa: promover bem-estar físico e mental, reduzir o estresse e aumentar a produtividade \n- Componentes: aulas de fitness no local, opções de refeições saudáveis, recursos de saúde mental e workshops de bem-estar \n- Parceria com academias locais e centros de bem-estar para associações com desconto \n- Incentivos para participação: rastreadores de fitness, pontos de bem-estar e sorteios de prêmios \n- Data de lançamento do programa: 1º de junho de 2023 \n- Processo de inscrição: registro online através da intranet da empresa \n- Coordenadores do programa: equipes de Recursos Humanos e Engajamento de Funcionários"
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
  temperature: 1,
  system: "Sua tarefa é compor um memorando abrangente da empresa com base nos pontos-chave fornecidos. O memorando deve ser escrito em um tom profissional, abordando todas as informações relevantes de maneira clara e concisa. Use formatação apropriada, como títulos, subtítulos e marcadores, para organizar o conteúdo de forma eficaz. Certifique-se de que o memorando seja bem estruturado, coerente e fácil de entender para o público-alvo.",
  messages: [
    {
      "role": "user",
      "content": [
        {
          "type": "text",
          "text": "- Anúncio de um novo programa de bem-estar dos funcionários: \"Fit4Success\"  \n- Objetivos do programa: promover bem-estar físico e mental, reduzir o estresse e aumentar a produtividade  \n- Componentes: aulas de fitness no local, opções de refeições saudáveis, recursos de saúde mental e workshops de bem-estar  \n- Parceria com academias locais e centros de bem-estar para associações com desconto  \n- Incentivos para participação: rastreadores de fitness, pontos de bem-estar e sorteios de prêmios  \n- Data de lançamento do programa: 1º de junho de 2023  \n- Processo de inscrição: registro online através da intranet da empresa  \n- Coordenadores do programa: equipes de Recursos Humanos e Engajamento de Funcionários"
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
    max_tokens=1000,
    temperature=1,
    system="Sua tarefa é compor um memorando abrangente da empresa com base nos pontos-chave fornecidos. O memorando deve ser escrito em um tom profissional, abordando todas as informações relevantes de maneira clara e concisa. Use formatação apropriada, como títulos, subtítulos e marcadores, para organizar o conteúdo de forma eficaz. Certifique-se de que o memorando seja bem estruturado, coerente e fácil de entender para o público-alvo.",
    messages=[
        {
            "role": "user",
            "content": [
                {
                    "type": "text",
                    "text": "- Anúncio de um novo programa de bem-estar dos funcionários: \"Fit4Success\"  \n- Objetivos do programa: promover bem-estar físico e mental, reduzir o estresse e aumentar a produtividade  \n- Componentes: aulas de fitness no local, opções de refeições saudáveis, recursos de saúde mental e workshops de bem-estar  \n- Parceria com academias locais e centros de bem-estar para associações com desconto  \n- Incentivos para participação: rastreadores de fitness, pontos de bem-estar e sorteios de prêmios  \n- Data de lançamento do programa: 1º de junho de 2023  \n- Processo de inscrição: registro online através da intranet da empresa  \n- Coordenadores do programa: equipes de Recursos Humanos e Engajamento de Funcionários"
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
  max_tokens: 1000,
  temperature: 1,
  system: "Sua tarefa é compor um memorando abrangente da empresa com base nos pontos-chave fornecidos. O memorando deve ser escrito em um tom profissional, abordando todas as informações relevantes de maneira clara e concisa. Use formatação apropriada, como títulos, subtítulos e marcadores, para organizar o conteúdo de forma eficaz. Certifique-se de que o memorando seja bem estruturado, coerente e fácil de entender para o público-alvo.",
  messages: [
    {
      "role": "user",
      "content": [
        {
          "type": "text",
          "text": "- Anúncio de um novo programa de bem-estar dos funcionários: \"Fit4Success\"  \n- Objetivos do programa: promover bem-estar físico e mental, reduzir o estresse e aumentar a produtividade  \n- Componentes: aulas de fitness no local, opções de refeições saudáveis, recursos de saúde mental e workshops de bem-estar  \n- Parceria com academias locais e centros de bem-estar para associações com desconto  \n- Incentivos para participação: rastreadores de fitness, pontos de bem-estar e sorteios de prêmios  \n- Data de lançamento do programa: 1º de junho de 2023  \n- Processo de inscrição: registro online através da intranet da empresa  \n- Coordenadores do programa: equipes de Recursos Humanos e Engajamento de Funcionários"
        }
      ]
    }
  ]
});
console.log(msg);

```

</Tab>
</Tabs>