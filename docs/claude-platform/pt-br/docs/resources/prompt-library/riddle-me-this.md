# Decifre-me isso

Gere charadas e guie o usuário às soluções.

---

> Copie este prompt para nosso [Console](/dashboard) de desenvolvedor para experimentá-lo você mesmo!

| Papel | Conteúdo |
| ---- | --- |
| Usuário | Gere uma charada inteligente e forneça um guia passo a passo para ajudar o usuário a chegar às soluções corretas. A charada deve ser desafiadora, mas solucionável com pensamento lógico e atenção aos detalhes. Após apresentar cada charada, ofereça um conjunto de dicas ou perguntas que progressivamente levem o usuário em direção à resposta. Certifique-se de que as dicas não sejam muito óbvias, mas ainda forneçam informações suficientes para guiar o processo de pensamento do usuário. Finalmente, revele a solução e forneça uma breve explicação de como a charada pode ser resolvida usando as dicas fornecidas. |

## Exemplo de Saída

Eu não estou vivo, mas cresço; não tenho pulmões, mas preciso de ar; não tenho boca, mas a água me mata. O que sou eu?

Dica 1: Considere os diferentes estados da matéria e como eles podem mudar.  
Dica 2: Pense sobre a relação entre os três elementos mencionados: ar, água e o objeto misterioso.  
Dica 3: A resposta está relacionada a um fenômeno comum que ocorre na vida cotidiana.

Solução: Fogo.  
Explicação: O fogo não está vivo, mas cresce e se espalha conforme consome combustível. Ele não tem pulmões, mas requer oxigênio (ar) para se sustentar. Embora não tenha boca, a água é frequentemente usada para extinguir incêndios, efetivamente "matando-os".

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
  temperature=0,
  messages=[
    {
      "role": "user",
      "content": [
        {
          "type": "text",
          "text": "Gere uma charada inteligente e forneça um guia passo a passo para ajudar o usuário a chegar às soluções corretas. A charada deve ser desafiadora, mas solucionável com pensamento lógico e atenção aos detalhes. Após apresentar cada charada, ofereça um conjunto de dicas ou perguntas que progressivamente levem o usuário em direção à resposta. Certifique-se de que as dicas não sejam muito óbvias, mas ainda forneçam informações suficientes para guiar o processo de pensamento do usuário. Finalmente, revele a solução e forneça uma breve explicação de como a charada pode ser resolvida usando as dicas fornecidas."
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
  temperature: 0,
  messages: [
    {
      "role": "user",
      "content": [
        {
          "type": "text",
          "text": "Gere uma charada inteligente e forneça um guia passo a passo para ajudar o usuário a chegar às soluções corretas. A charada deve ser desafiadora, mas solucionável com pensamento lógico e atenção aos detalhes. Após apresentar cada charada, ofereça um conjunto de dicas ou perguntas que progressivamente levem o usuário em direção à resposta. Certifique-se de que as dicas não sejam muito óbvias, mas ainda forneçam informações suficientes para guiar o processo de pensamento do usuário. Finalmente, revele a solução e forneça uma breve explicação de como a charada pode ser resolvida usando as dicas fornecidas."
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
    temperature=0,
    messages=[
        {
            "role": "user",
            "content": [
                {
                    "type": "text",
                    "text": "Gere uma charada inteligente e forneça um guia passo a passo para ajudar o usuário a chegar às soluções corretas. A charada deve ser desafiadora, mas solucionável com pensamento lógico e atenção aos detalhes. Após apresentar cada charada, ofereça um conjunto de dicas ou perguntas que progressivamente levem o usuário em direção à resposta. Certifique-se de que as dicas não sejam muito óbvias, mas ainda forneçam informações suficientes para guiar o processo de pensamento do usuário. Finalmente, revele a solução e forneça uma breve explicação de como a charada pode ser resolvida usando as dicas fornecidas."
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
  temperature: 0,
  messages: [
    {
      "role": "user",
      "content": [
        {
          "type": "text",
          "text": "Gere uma charada inteligente e forneça um guia passo a passo para ajudar o usuário a chegar às soluções corretas. A charada deve ser desafiadora, mas solucionável com pensamento lógico e atenção aos detalhes. Após apresentar cada charada, ofereça um conjunto de dicas ou perguntas que progressivamente levem o usuário em direção à resposta. Certifique-se de que as dicas não sejam muito óbvias, mas ainda forneçam informações suficientes para guiar o processo de pensamento do usuário. Finalmente, revele a solução e forneça uma breve explicação de como a charada pode ser resolvida usando as dicas fornecidas."
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
    temperature=0,
    messages=[
        {
            "role": "user",
            "content": [
                {
                    "type": "text",
                    "text": "Gere uma charada inteligente e forneça um guia passo a passo para ajudar o usuário a chegar às soluções corretas. A charada deve ser desafiadora, mas solucionável com pensamento lógico e atenção aos detalhes. Após apresentar cada charada, ofereça um conjunto de dicas ou perguntas que progressivamente levem o usuário em direção à resposta. Certifique-se de que as dicas não sejam muito óbvias, mas ainda forneçam informações suficientes para guiar o processo de pensamento do usuário. Finalmente, revele a solução e forneça uma breve explicação de como a charada pode ser resolvida usando as dicas fornecidas."
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
  temperature: 0,
  messages: [
    {
      "role": "user",
      "content": [
        {
          "type": "text",
          "text": "Gere uma charada inteligente e forneça um guia passo a passo para ajudar o usuário a chegar às soluções corretas. A charada deve ser desafiadora, mas solucionável com pensamento lógico e atenção aos detalhes. Após apresentar cada charada, ofereça um conjunto de dicas ou perguntas que progressivamente levem o usuário em direção à resposta. Certifique-se de que as dicas não sejam muito óbvias, mas ainda forneçam informações suficientes para guiar o processo de pensamento do usuário. Finalmente, revele a solução e forneça uma breve explicação de como a charada pode ser resolvida usando as dicas fornecidas."
        }
      ]
    }
  ]
});
console.log(msg);

```

</Tab>
</Tabs>