# Simplificador de segunda série

Torne textos complexos fáceis de entender para jovens aprendizes.

---

> Copie este prompt para nosso [Console](/dashboard) de desenvolvedor para experimentá-lo você mesmo!

|        | Conteúdo |
| --- | --- |
| System | Sua tarefa é pegar o texto fornecido e reescrevê-lo de uma forma que seja fácil para jovens aprendizes da 3ª à 5ª série lerem e entenderem. Simplifique vocabulário avançado, divida frases longas, explique conceitos difíceis em linguagem simples e apresente as informações de forma clara e envolvente. O texto reescrito curto deve transmitir as ideias centrais do texto original de maneira apropriada para a idade. |
| User   | As mitocôndrias, frequentemente referidas como as usinas de energia da célula, são organelas essenciais que desempenham um papel crucial na produção de trifosfato de adenosina (ATP) através do processo de respiração celular. O ATP é a principal moeda energética da célula, permitindo que várias funções celulares e reações bioquímicas ocorram. |

## Exemplo de Saída

As mitocôndrias são partes minúsculas dentro das células que funcionam como pequenas estações de energia. Elas fazem uma molécula especial chamada ATP, que é como o dinheiro de energia da célula. O ATP ajuda a célula a fazer trabalhos importantes e a mantê-la funcionando suavemente, assim como o dinheiro ajuda as pessoas a comprar coisas que precisam. As mitocôndrias são muito importantes porque sem elas, as células não teriam energia para funcionar adequadamente.

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
  system="Sua tarefa é pegar o texto fornecido e reescrevê-lo de uma forma que seja fácil para jovens aprendizes da 3ª à 5ª série lerem e entenderem. Simplifique vocabulário avançado, divida frases longas, explique conceitos difíceis em linguagem simples e apresente as informações de forma clara e envolvente. O texto reescrito curto deve transmitir as ideias centrais do texto original de maneira apropriada para a idade.",
  messages=[
    {
      "role": "user",
      "content": [
        {
          "type": "text",
          "text": "As mitocôndrias, frequentemente referidas como as usinas de energia da célula, são organelas essenciais que desempenham um papel crucial na produção de trifosfato de adenosina (ATP) através do processo de respiração celular. O ATP é a principal moeda energética da célula, permitindo que várias funções celulares e reações bioquímicas ocorram."
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
  system: "Sua tarefa é pegar o texto fornecido e reescrevê-lo de uma forma que seja fácil para jovens aprendizes da 3ª à 5ª série lerem e entenderem. Simplifique vocabulário avançado, divida frases longas, explique conceitos difíceis em linguagem simples e apresente as informações de forma clara e envolvente. O texto reescrito curto deve transmitir as ideias centrais do texto original de maneira apropriada para a idade.",
  messages: [
    {
      "role": "user",
      "content": [
        {
          "type": "text",
          "text": "As mitocôndrias, frequentemente referidas como as usinas de energia da célula, são organelas essenciais que desempenham um papel crucial na produção de trifosfato de adenosina (ATP) através do processo de respiração celular. O ATP é a principal moeda energética da célula, permitindo que várias funções celulares e reações bioquímicas ocorram."
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
    system="Sua tarefa é pegar o texto fornecido e reescrevê-lo de uma forma que seja fácil para jovens aprendizes da 3ª à 5ª série lerem e entenderem. Simplifique vocabulário avançado, divida frases longas, explique conceitos difíceis em linguagem simples e apresente as informações de forma clara e envolvente. O texto reescrito curto deve transmitir as ideias centrais do texto original de maneira apropriada para a idade.",
    messages=[
        {
            "role": "user",
            "content": [
                {
                    "type": "text",
                    "text": "As mitocôndrias, frequentemente referidas como as usinas de energia da célula, são organelas essenciais que desempenham um papel crucial na produção de trifosfato de adenosina (ATP) através do processo de respiração celular. O ATP é a principal moeda energética da célula, permitindo que várias funções celulares e reações bioquímicas ocorram."
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
  system: "Sua tarefa é pegar o texto fornecido e reescrevê-lo de uma forma que seja fácil para jovens aprendizes da 3ª à 5ª série lerem e entenderem. Simplifique vocabulário avançado, divida frases longas, explique conceitos difíceis em linguagem simples e apresente as informações de forma clara e envolvente. O texto reescrito curto deve transmitir as ideias centrais do texto original de maneira apropriada para a idade.",
  messages: [
    {
      "role": "user",
      "content": [
        {
          "type": "text",
          "text": "As mitocôndrias, frequentemente referidas como as usinas de energia da célula, são organelas essenciais que desempenham um papel crucial na produção de trifosfato de adenosina (ATP) através do processo de respiração celular. O ATP é a principal moeda energética da célula, permitindo que várias funções celulares e reações bioquímicas ocorram."
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
system="Sua tarefa é pegar o texto fornecido e reescrevê-lo de uma forma que seja fácil para jovens aprendizes da 3ª à 5ª série lerem e entenderem. Simplifique vocabulário avançado, divida frases longas, explique conceitos difíceis em linguagem simples e apresente as informações de forma clara e envolvente. O texto reescrito curto deve transmitir as ideias centrais do texto original de maneira apropriada para a idade.",
messages=[
{
"role": "user",
"content": [
{
"type": "text",
"text": "As mitocôndrias, frequentemente referidas como as usinas de energia da célula, são organelas essenciais que desempenham um papel crucial na produção de trifosfato de adenosina (ATP) através do processo de respiração celular. O ATP é a principal moeda energética da célula, permitindo que várias funções celulares e reações bioquímicas ocorram."
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
  temperature: 0,
  system: "Sua tarefa é pegar o texto fornecido e reescrevê-lo de uma forma que seja fácil para jovens aprendizes da 3ª à 5ª série lerem e entenderem. Simplifique vocabulário avançado, divida frases longas, explique conceitos difíceis em linguagem simples e apresente as informações de forma clara e envolvente. O texto reescrito curto deve transmitir as ideias centrais do texto original de maneira apropriada para a idade.",
  messages: [
    {
      "role": "user",
      "content": [
        {
          "type": "text",
          "text": "As mitocôndrias, frequentemente referidas como as usinas de energia da célula, são organelas essenciais que desempenham um papel crucial na produção de trifosfato de adenosina (ATP) através do processo de respiração celular. O ATP é a principal moeda energética da célula, permitindo que várias funções celulares e reações bioquímicas ocorram."
        }
      ]
    }
  ]
});
console.log(msg);

````

</Tab>
</Tabs>