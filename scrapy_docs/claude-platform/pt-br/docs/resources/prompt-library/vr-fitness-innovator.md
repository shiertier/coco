# Inovador de fitness VR

Faça brainstorming de ideias criativas para jogos de fitness em realidade virtual.

---

> Copie este prompt para nosso [Console](/dashboard) de desenvolvedor para experimentá-lo você mesmo!

| Papel | Conteúdo |
| ---- | --- |
| Usuário | Sua tarefa é gerar uma lista de ideias inovadoras e envolventes para jogos de fitness em realidade virtual (VR). Considere vários gêneros de jogos, mecânicas de gameplay únicas, ambientes imersivos e maneiras criativas de incorporar exercícios físicos e movimentos. As ideias devem ser atraentes para uma ampla gama de entusiastas de fitness e gamers, encorajando-os a se manterem ativos e se divertirem enquanto se exercitam em VR. Para cada ideia, forneça uma breve descrição do conceito do jogo, características principais e como ele promove o fitness. |

## Exemplo de Saída

1.  Ideia: Aventura de Parkour VR  
    Descrição: Os jogadores navegam através de um ambiente urbano dinâmico, correndo, saltando, escalando e realizando acrobacias de parkour para superar obstáculos e alcançar objetivos. O jogo se adapta aos movimentos do jogador, proporcionando um treino intenso de corpo inteiro enquanto explora a cidade virtual.  
    Características principais: Mecânicas realistas de parkour, ambientes gerados proceduralmente, corridas multijogador e desafios.  
    Aspectos de fitness: Cardio, agilidade, coordenação e força da parte inferior do corpo.
2.  Ideia: Boxe Rítmico  
    Descrição: Um jogo de boxe de alta energia ambientado no ritmo de faixas musicais populares. Os jogadores devem socar, esquivar e bloquear em sincronia com o ritmo para marcar pontos e progredir através de níveis cada vez mais desafiadores. O jogo oferece uma variedade de estilos de boxe e modos de treinamento.  
    Características principais: Música licenciada, boxeadores personalizáveis, multijogador online e uma gama diversa de técnicas de boxe.  
    Aspectos de fitness: Cardio, força da parte superior do corpo, reflexos e resistência.
3.  Ideia: RPG de Fitness VR  
    Descrição: Um jogo de interpretação de papéis imersivo onde os jogadores criam seu próprio personagem e embarcam em uma missão para salvar um mundo de fantasia. O jogo combina elementos tradicionais de RPG com desafios de fitness, exigindo que os jogadores realizem exercícios físicos para lançar feitiços, derrotar inimigos e subir de nível seu personagem.  
    Características principais: Personalização de personagem, árvores de habilidades, batalhas épicas contra chefes e uma mistura de exercícios de força, cardio e flexibilidade.  
    Aspectos de fitness: Treinos de corpo inteiro, treinamento de força, cardio e flexibilidade.

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
  temperature=1,
  messages=[
    {
      "role": "user",
      "content": [
        {
          "type": "text",
          "text": "Sua tarefa é gerar uma lista de ideias inovadoras e envolventes para jogos de fitness em realidade virtual (VR). Considere vários gêneros de jogos, mecânicas de gameplay únicas, ambientes imersivos e maneiras criativas de incorporar exercícios físicos e movimentos. As ideias devem ser atraentes para uma ampla gama de entusiastas de fitness e gamers, encorajando-os a se manterem ativos e se divertirem enquanto se exercitam em VR. Para cada ideia, forneça uma breve descrição do conceito do jogo, características principais e como ele promove o fitness."
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
  messages: [
    {
      "role": "user",
      "content": [
        {
          "type": "text",
          "text": "Sua tarefa é gerar uma lista de ideias inovadoras e envolventes para jogos de fitness em realidade virtual (VR). Considere vários gêneros de jogos, mecânicas de gameplay únicas, ambientes imersivos e maneiras criativas de incorporar exercícios físicos e movimentos. As ideias devem ser atraentes para uma ampla gama de entusiastas de fitness e gamers, encorajando-os a se manterem ativos e se divertirem enquanto se exercitam em VR. Para cada ideia, forneça uma breve descrição do conceito do jogo, características principais e como ele promove o fitness."
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
    temperature=1,
    messages=[
        {
            "role": "user",
            "content": [
                {
                    "type": "text",
                    "text": "Sua tarefa é gerar uma lista de ideias inovadoras e envolventes para jogos de fitness em realidade virtual (VR). Considere vários gêneros de jogos, mecânicas de gameplay únicas, ambientes imersivos e maneiras criativas de incorporar exercícios físicos e movimentos. As ideias devem ser atraentes para uma ampla gama de entusiastas de fitness e gamers, encorajando-os a se manterem ativos e se divertirem enquanto se exercitam em VR. Para cada ideia, forneça uma breve descrição do conceito do jogo, características principais e como ele promove o fitness."
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
  messages: [
    {
      "role": "user",
      "content": [
        {
          "type": "text",
          "text": "Sua tarefa é gerar uma lista de ideias inovadoras e envolventes para jogos de fitness em realidade virtual (VR). Considere vários gêneros de jogos, mecânicas de gameplay únicas, ambientes imersivos e maneiras criativas de incorporar exercícios físicos e movimentos. As ideias devem ser atraentes para uma ampla gama de entusiastas de fitness e gamers, encorajando-os a se manterem ativos e se divertirem enquanto se exercitam em VR. Para cada ideia, forneça uma breve descrição do conceito do jogo, características principais e como ele promove o fitness."
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
    messages=[
        {
            "role": "user",
            "content": [
                {
                    "type": "text",
                    "text": "Sua tarefa é gerar uma lista de ideias inovadoras e envolventes para jogos de fitness em realidade virtual (VR). Considere vários gêneros de jogos, mecânicas de gameplay únicas, ambientes imersivos e maneiras criativas de incorporar exercícios físicos e movimentos. As ideias devem ser atraentes para uma ampla gama de entusiastas de fitness e gamers, encorajando-os a se manterem ativos e se divertirem enquanto se exercitam em VR. Para cada ideia, forneça uma breve descrição do conceito do jogo, características principais e como ele promove o fitness."
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
  messages: [
    {
      "role": "user",
      "content": [
        {
          "type": "text",
          "text": "Sua tarefa é gerar uma lista de ideias inovadoras e envolventes para jogos de fitness em realidade virtual (VR). Considere vários gêneros de jogos, mecânicas de gameplay únicas, ambientes imersivos e maneiras criativas de incorporar exercícios físicos e movimentos. As ideias devem ser atraentes para uma ampla gama de entusiastas de fitness e gamers, encorajando-os a se manterem ativos e se divertirem enquanto se exercitam em VR. Para cada ideia, forneça uma breve descrição do conceito do jogo, características principais e como ele promove o fitness."
        }
      ]
    }
  ]
});
console.log(msg);

```

</Tab>
</Tabs>