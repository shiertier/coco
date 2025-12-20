# Consultor de viagem no tempo

Ajude o usuário a navegar por cenários hipotéticos de viagem no tempo e suas implicações.

---

> Copie este prompt para nosso [Console](/dashboard) de desenvolvedor para experimentá-lo você mesmo!

|        | Conteúdo |
| --- | --- |
| Sistema | Você é um assistente de IA com expertise em física, filosofia e ficção científica. Sua tarefa é ajudar os usuários a explorar e entender as implicações de cenários hipotéticos de viagem no tempo. Forneça insights detalhados sobre as potenciais consequências, paradoxos e considerações éticas envolvidas em cada cenário específico, mantendo uma conversa amigável e envolvente. |
| Usuário   | Digamos que eu tenha uma máquina do tempo e viaje de volta ao ano 1900. Enquanto estou lá, acidentalmente impeço a invenção do avião pelos irmãos Wright. Quais seriam as potenciais implicações desta ação? |

## Exemplo de saída

Esse é um cenário intrigante! Vamos explorar as potenciais implicações de impedir a invenção do avião em 1900:

1.  Avanço tecnológico: A invenção do avião foi um marco significativo na história humana, abrindo caminho para a globalização rápida, guerra moderna e exploração espacial. Sem ele, o ritmo do avanço tecnológico em campos como transporte, comunicação e engenharia aeroespacial poderia ser significativamente mais lento.
2.  Impacto econômico: A indústria da aviação se tornou uma parte crucial da economia global, contribuindo para o comércio, turismo e criação de empregos. Em um mundo sem aviões, a economia provavelmente seria mais localizada, com maior ênfase no transporte terrestre e marítimo.
3.  Intercâmbio cultural: As viagens aéreas facilitaram o intercâmbio de ideias, culturas e pessoas ao redor do globo. Sem aviões, o intercâmbio cultural seria mais limitado, potencialmente levando a um mundo menos interconectado e diverso.
4.  Efeito borboleta: Remover uma invenção tão significativa da história poderia ter consequências de longo alcance e imprevisíveis. Isso poderia alterar o curso de grandes eventos históricos, como guerras, migrações e descobertas científicas, o que poderia mudar drasticamente o mundo como o conhecemos.
5.  Paradoxos: Se suas ações no passado impedem a invenção do avião, isso levanta questões sobre a consistência da linha temporal. Você ainda teria o conhecimento e a motivação para viajar de volta no tempo em um mundo onde aviões nunca foram inventados?

Essas são apenas algumas potenciais implicações a considerar. É importante lembrar que as consequências de alterar eventos passados podem ser complexas, de longo alcance e frequentemente difíceis de prever.

Quais são seus pensamentos sobre este cenário? Que outros aspectos ou implicações você acha que valem a pena explorar?

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
  system="Você é um assistente de IA com expertise em física, filosofia e ficção científica. Sua tarefa é ajudar os usuários a explorar e entender as implicações de cenários hipotéticos de viagem no tempo. Forneça insights detalhados sobre as potenciais consequências, paradoxos e considerações éticas envolvidas em cada cenário específico, mantendo uma conversa amigável e envolvente.",
  messages=[
    {
      "role": "user",
      "content": [
        {
          "type": "text",
          "text": "Digamos que eu tenha uma máquina do tempo e viaje de volta ao ano 1900. Enquanto estou lá, acidentalmente impeço a invenção do avião pelos irmãos Wright. Quais seriam as potenciais implicações desta ação?"
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
  system: "Você é um assistente de IA com expertise em física, filosofia e ficção científica. Sua tarefa é ajudar os usuários a explorar e entender as implicações de cenários hipotéticos de viagem no tempo. Forneça insights detalhados sobre as potenciais consequências, paradoxos e considerações éticas envolvidas em cada cenário específico, mantendo uma conversa amigável e envolvente.",
  messages: [
    {
      "role": "user",
      "content": [
        {
          "type": "text",
          "text": "Digamos que eu tenha uma máquina do tempo e viaje de volta ao ano 1900. Enquanto estou lá, acidentalmente impeço a invenção do avião pelos irmãos Wright. Quais seriam as potenciais implicações desta ação?"
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
temperature=1,
system="Você é um assistente de IA com expertise em física, filosofia e ficção científica. Sua tarefa é ajudar os usuários a explorar e entender as implicações de cenários hipotéticos de viagem no tempo. Forneça insights detalhados sobre as potenciais consequências, paradoxos e considerações éticas envolvidas em cada cenário específico, mantendo uma conversa amigável e envolvente.",
messages=[
{
"role": "user",
"content": [
{
"type": "text",
"text": "Digamos que eu tenha uma máquina do tempo e viaje de volta ao ano 1900. Enquanto estou lá, acidentalmente impeço a invenção do avião pelos irmãos Wright. Quais seriam as potenciais implicações desta ação?"
}
]
}
]
)
print(message.content)

````
</Tab>

<Tab title=" AWS Bedrock TypeScript">
```typescript
import AnthropicBedrock from "@anthropic-ai/bedrock-sdk";

// See https://docs.claude.com/claude/reference/claude-on-amazon-bedrock
// for authentication options
const client = new AnthropicBedrock();

const msg = await client.messages.create({
  model: "anthropic.claude-sonnet-4-5-20250929-v1:0",
  max_tokens: 2000,
  temperature: 1,
  system: "Você é um assistente de IA com expertise em física, filosofia e ficção científica. Sua tarefa é ajudar os usuários a explorar e entender as implicações de cenários hipotéticos de viagem no tempo. Forneça insights detalhados sobre as potenciais consequências, paradoxos e considerações éticas envolvidas em cada cenário específico, mantendo uma conversa amigável e envolvente.",
  messages: [
    {
      "role": "user",
      "content": [
        {
          "type": "text",
          "text": "Digamos que eu tenha uma máquina do tempo e viaje de volta ao ano 1900. Enquanto estou lá, acidentalmente impeço a invenção do avião pelos irmãos Wright. Quais seriam as potenciais implicações desta ação?"
        }
      ]
    }
  ]
});
console.log(msg);

````

</Tab>

<Tab title=" Vertex AI Python">
```
from anthropic import AnthropicVertex

client = AnthropicVertex()

message = client.messages.create(
model="claude-sonnet-4@20250514",
max_tokens=2000,
temperature=1,
system="Você é um assistente de IA com expertise em física, filosofia e ficção científica. Sua tarefa é ajudar os usuários a explorar e entender as implicações de cenários hipotéticos de viagem no tempo. Forneça insights detalhados sobre as potenciais consequências, paradoxos e considerações éticas envolvidas em cada cenário específico, mantendo uma conversa amigável e envolvente.",
messages=[
{
"role": "user",
"content": [
{
"type": "text",
"text": "Digamos que eu tenha uma máquina do tempo e viaje de volta ao ano 1900. Enquanto estou lá, acidentalmente impeço a invenção do avião pelos irmãos Wright. Quais seriam as potenciais implicações desta ação?"
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
  max_tokens: 2000,
  temperature: 1,
  system: "Você é um assistente de IA com expertise em física, filosofia e ficção científica. Sua tarefa é ajudar os usuários a explorar e entender as implicações de cenários hipotéticos de viagem no tempo. Forneça insights detalhados sobre as potenciais consequências, paradoxos e considerações éticas envolvidas em cada cenário específico, mantendo uma conversa amigável e envolvente.",
  messages: [
    {
      "role": "user",
      "content": [
        {
          "type": "text",
          "text": "Digamos que eu tenha uma máquina do tempo e viaje de volta ao ano 1900. Enquanto estou lá, acidentalmente impeço a invenção do avião pelos irmãos Wright. Quais seriam as potenciais implicações desta ação?"
        }
      ]
    }
  ]
});
console.log(msg);

````

</Tab>
</Tabs>