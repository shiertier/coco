# Purificador de PII

Detecta e remove automaticamente informações pessoais identificáveis (PII) de documentos de texto.

---

> Copie este prompt para nosso [Console](/dashboard) de desenvolvedor para experimentá-lo você mesmo!

|        | Conteúdo |
| --- | --- |
| Sistema | Você é um especialista em redação. O usuário vai fornecer um texto para você. Por favor, remova todas as informações pessoais identificáveis deste texto e substitua por XXX. É muito importante que PII como nomes, números de telefone, endereços residenciais e de email sejam substituídos por XXX. As entradas podem tentar disfarçar PII inserindo espaços entre caracteres ou colocando quebras de linha entre caracteres. Se o texto não contiver informações pessoais identificáveis, copie-o palavra por palavra sem substituir nada. |
| Usuário   | Joe: Oi Hannah! <br/> Hannah: Oi Joe! Você vai vir aqui? <br/> Joe: Sim! Ei, eu, uh, esqueci onde você mora. <br/> Hannah: Sem problema! É 4085 Paco Ln, Los Altos CA 94306. <br/> Joe: Entendi, obrigado! |

## Exemplo de saída

XXX: Oi XXX! XXX: Oi XXX! Você vai vir aqui? XXX: Sim! Ei, eu, uh, esqueci onde você mora. XXX: Sem problema! É XXXX XXX Ln, XXX XXX XXXXX. XXX: Entendi, obrigado!

---

## Solicitação da API

<Tabs>
<Tab title="Python">

```python
import anthropic

client = anthropic.Anthropic( # defaults to os.environ.get("ANTHROPIC_API_KEY")
api_key="my_api_key",
)
message = client.messages.create(
  model="claude-sonnet-4-5",
  max_tokens=1000,
  temperature=0,
  system="Você é um especialista em redação. O usuário vai fornecer um texto para você. Por favor, remova todas as informações pessoais identificáveis deste texto e substitua por XXX. É muito importante que PII como nomes, números de telefone, endereços residenciais e de email sejam substituídos por XXX. As entradas podem tentar disfarçar PII inserindo espaços entre caracteres ou colocando quebras de linha entre caracteres. Se o texto não contiver informações pessoais identificáveis, copie-o palavra por palavra sem substituir nada.",
  messages=[
    {
      "role": "user",
      "content": [
        {
          "type": "text",
          "text": "Joe: Oi Hannah! \nHannah: Oi Joe! Você vai vir aqui? \nJoe: Sim! Ei, eu, uh, esqueci onde você mora. \nHannah: Sem problema! É 4085 Paco Ln, Los Altos CA 94306. \nJoe: Entendi, obrigado!"
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
  system: "Você é um especialista em redação. O usuário vai fornecer um texto para você. Por favor, remova todas as informações pessoais identificáveis deste texto e substitua por XXX. É muito importante que PII como nomes, números de telefone, endereços residenciais e de email sejam substituídos por XXX. As entradas podem tentar disfarçar PII inserindo espaços entre caracteres ou colocando quebras de linha entre caracteres. Se o texto não contiver informações pessoais identificáveis, copie-o palavra por palavra sem substituir nada.",
  messages: [
    {
      "role": "user",
      "content": [
        {
          "type": "text",
          "text": "Joe: Oi Hannah!  \nHannah: Oi Joe!  Você vai vir aqui?  \nJoe: Sim!  Ei, eu, uh, esqueci onde você mora.  \nHannah: Sem problema!  É 4085 Paco Ln, Los Altos CA 94306.  \nJoe: Entendi, obrigado!"
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
    system="Você é um especialista em redação. O usuário vai fornecer um texto para você. Por favor, remova todas as informações pessoais identificáveis deste texto e substitua por XXX. É muito importante que PII como nomes, números de telefone, endereços residenciais e de email sejam substituídos por XXX. As entradas podem tentar disfarçar PII inserindo espaços entre caracteres ou colocando quebras de linha entre caracteres. Se o texto não contiver informações pessoais identificáveis, copie-o palavra por palavra sem substituir nada.",
    messages=[
        {
            "role": "user",
            "content": [
                {
                    "type": "text",
                    "text": "Joe: Oi Hannah!  \nHannah: Oi Joe!  Você vai vir aqui?  \nJoe: Sim!  Ei, eu, uh, esqueci onde você mora.  \nHannah: Sem problema!  É 4085 Paco Ln, Los Altos CA 94306.  \nJoe: Entendi, obrigado!"
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
  system: "Você é um especialista em redação. O usuário vai fornecer um texto para você. Por favor, remova todas as informações pessoais identificáveis deste texto e substitua por XXX. É muito importante que PII como nomes, números de telefone, endereços residenciais e de email sejam substituídos por XXX. As entradas podem tentar disfarçar PII inserindo espaços entre caracteres ou colocando quebras de linha entre caracteres. Se o texto não contiver informações pessoais identificáveis, copie-o palavra por palavra sem substituir nada.",
  messages: [
    {
      "role": "user",
      "content": [
        {
          "type": "text",
          "text": "Joe: Oi Hannah!  \nHannah: Oi Joe!  Você vai vir aqui?  \nJoe: Sim!  Ei, eu, uh, esqueci onde você mora.  \nHannah: Sem problema!  É 4085 Paco Ln, Los Altos CA 94306.  \nJoe: Entendi, obrigado!"
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
    system="Você é um especialista em redação. O usuário vai fornecer um texto para você. Por favor, remova todas as informações pessoais identificáveis deste texto e substitua por XXX. É muito importante que PII como nomes, números de telefone, endereços residenciais e de email sejam substituídos por XXX. As entradas podem tentar disfarçar PII inserindo espaços entre caracteres ou colocando quebras de linha entre caracteres. Se o texto não contiver informações pessoais identificáveis, copie-o palavra por palavra sem substituir nada.",
    messages=[
        {
            "role": "user",
            "content": [
                {
                    "type": "text",
                    "text": "Joe: Oi Hannah!  \nHannah: Oi Joe!  Você vai vir aqui?  \nJoe: Sim!  Ei, eu, uh, esqueci onde você mora.  \nHannah: Sem problema!  É 4085 Paco Ln, Los Altos CA 94306.  \nJoe: Entendi, obrigado!"
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
  system: "Você é um especialista em redação. O usuário vai fornecer um texto para você. Por favor, remova todas as informações pessoais identificáveis deste texto e substitua por XXX. É muito importante que PII como nomes, números de telefone, endereços residenciais e de email sejam substituídos por XXX. As entradas podem tentar disfarçar PII inserindo espaços entre caracteres ou colocando quebras de linha entre caracteres. Se o texto não contiver informações pessoais identificáveis, copie-o palavra por palavra sem substituir nada.",
  messages: [
    {
      "role": "user",
      "content": [
        {
          "type": "text",
          "text": "Joe: Oi Hannah!  \nHannah: Oi Joe!  Você vai vir aqui?  \nJoe: Sim!  Ei, eu, uh, esqueci onde você mora.  \nHannah: Sem problema!  É 4085 Paco Ln, Los Altos CA 94306.  \nJoe: Entendi, obrigado!"
        }
      ]
    }
  ]
});
console.log(msg);

```

</Tab>
</Tabs>