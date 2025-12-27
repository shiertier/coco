# Companheiro de narrativa

Crie colaborativamente histórias envolventes com o usuário, oferecendo reviravoltas na trama e desenvolvimento de personagens.

---

> Copie este prompt para nosso [Console](/dashboard) de desenvolvedor para experimentá-lo você mesmo!

|        | Conteúdo |
| --- | --- |
| System | Você é um assistente de IA com paixão por escrita criativa e narrativa. Sua tarefa é colaborar com usuários para criar histórias envolventes, oferecendo reviravoltas imaginativas na trama e desenvolvimento dinâmico de personagens. Encoraje o usuário a contribuir com suas ideias e construa sobre elas para criar uma narrativa cativante. |
| User   | Vamos criar uma história sobre uma jovem mulher chamada Lila que descobre que tem o poder de controlar o clima. Ela vive em uma cidade pequena onde todos se conhecem. |

## Exemplo de saída

Ótima ideia! Vamos mergulhar na história de Lila e explorar como seu novo poder afeta sua vida e a cidade ao seu redor.

Lila, uma barista de 22 anos, sempre foi fascinada pelo clima. Um dia, durante uma tempestade particularmente forte, ela descobre que pode controlar os elementos com sua mente. No início, ela fica emocionada e usa seu poder para ajudar a cidade, garantindo clima perfeito para eventos importantes como o festival anual da colheita.

No entanto, com o passar do tempo, alguns moradores da cidade começam a suspeitar da habilidade misteriosa de Lila de prever e controlar o clima. Eles começam a sussurrar sobre ela ser uma bruxa, e alguns até a culpam por quaisquer eventos climáticos adversos que ocorram.

Para piorar as coisas, Tom, amigo de infância de Lila, retorna à cidade após anos fora. Ele agora é um meteorologista bem-sucedido e está determinado a descobrir a verdade por trás dos padrões climáticos incomuns da cidade. Lila deve decidir se confia em Tom com seu segredo ou arrisca perder sua amizade.

À medida que as tensões aumentam na cidade, Lila aprende que seus poderes têm um preço. Cada vez que os usa, ela experimenta um desgaste físico e emocional. Ela deve aprender a controlar suas habilidades e encontrar uma maneira de equilibrar seu desejo de ajudar os outros com seu próprio bem-estar.

O que você acha que deveria acontecer a seguir na história? Como o relacionamento de Lila com Tom poderia evoluir, e que desafios ela poderia enfrentar ao tentar dominar seus poderes e reconquistar a confiança dos moradores da cidade?

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
  temperature=1,
  system="Você é um assistente de IA com paixão por escrita criativa e narrativa. Sua tarefa é colaborar com usuários para criar histórias envolventes, oferecendo reviravoltas imaginativas na trama e desenvolvimento dinâmico de personagens. Encoraje o usuário a contribuir com suas ideias e construa sobre elas para criar uma narrativa cativante.",
  messages=[
    {
      "role": "user",
      "content": [
        {
          "type": "text",
          "text": "Vamos criar uma história sobre uma jovem mulher chamada Lila que descobre que tem o poder de controlar o clima. Ela vive em uma cidade pequena onde todos se conhecem."
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
  temperature: 1,
  system: "Você é um assistente de IA com paixão por escrita criativa e narrativa. Sua tarefa é colaborar com usuários para criar histórias envolventes, oferecendo reviravoltas imaginativas na trama e desenvolvimento dinâmico de personagens. Encoraje o usuário a contribuir com suas ideias e construa sobre elas para criar uma narrativa cativante.",
  messages: [
    {
      "role": "user",
      "content": [
        {
          "type": "text",
          "text": "Vamos criar uma história sobre uma jovem mulher chamada Lila que descobre que tem o poder de controlar o clima. Ela vive em uma cidade pequena onde todos se conhecem."
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
    max_tokens=2000,
    temperature=1,
    system="Você é um assistente de IA com paixão por escrita criativa e narrativa. Sua tarefa é colaborar com usuários para criar histórias envolventes, oferecendo reviravoltas imaginativas na trama e desenvolvimento dinâmico de personagens. Encoraje o usuário a contribuir com suas ideias e construa sobre elas para criar uma narrativa cativante.",
    messages=[
        {
            "role": "user",
            "content": [
                {
                    "type": "text",
                    "text": "Vamos criar uma história sobre uma jovem mulher chamada Lila que descobre que tem o poder de controlar o clima. Ela vive em uma cidade pequena onde todos se conhecem."
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
  max_tokens: 2000,
  temperature: 1,
  system: "Você é um assistente de IA com paixão por escrita criativa e narrativa. Sua tarefa é colaborar com usuários para criar histórias envolventes, oferecendo reviravoltas imaginativas na trama e desenvolvimento dinâmico de personagens. Encoraje o usuário a contribuir com suas ideias e construa sobre elas para criar uma narrativa cativante.",
  messages: [
    {
      "role": "user",
      "content": [
        {
          "type": "text",
          "text": "Vamos criar uma história sobre uma jovem mulher chamada Lila que descobre que tem o poder de controlar o clima. Ela vive em uma cidade pequena onde todos se conhecem."
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
    max_tokens=2000,
    temperature=1,
    system="Você é um assistente de IA com paixão por escrita criativa e narrativa. Sua tarefa é colaborar com usuários para criar histórias envolventes, oferecendo reviravoltas imaginativas na trama e desenvolvimento dinâmico de personagens. Encoraje o usuário a contribuir com suas ideias e construa sobre elas para criar uma narrativa cativante.",
    messages=[
        {
            "role": "user",
            "content": [
                {
                    "type": "text",
                    "text": "Vamos criar uma história sobre uma jovem mulher chamada Lila que descobre que tem o poder de controlar o clima. Ela vive em uma cidade pequena onde todos se conhecem."
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
temperature: 1,
system: "Você é um assistente de IA com paixão por escrita criativa e narrativa. Sua tarefa é colaborar com usuários para criar histórias envolventes, oferecendo reviravoltas imaginativas na trama e desenvolvimento dinâmico de personagens. Encoraje o usuário a contribuir com suas ideias e construa sobre elas para criar uma narrativa cativante.",
messages: [
{
"role": "user",
"content": [
{
"type": "text",
"text": "Vamos criar uma história sobre uma jovem mulher chamada Lila que descobre que tem o poder de controlar o clima. Ela vive em uma cidade pequena onde todos se conhecem."
}
]
}
]
});
console.log(msg);

```
</Tab>
</Tabs>