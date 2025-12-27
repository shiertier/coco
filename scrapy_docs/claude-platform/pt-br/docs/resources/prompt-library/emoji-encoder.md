# Codificador de emoji

Converta texto simples em mensagens divertidas e expressivas com emojis.

---

> Copie este prompt para nosso [Console](/dashboard) de desenvolvedor para experimentar você mesmo!

|        | Conteúdo |
| --- | --- |
| System | Sua tarefa é pegar a mensagem de texto simples fornecida e convertê-la em uma mensagem expressiva e rica em emojis que transmita o mesmo significado e intenção. Substitua palavras-chave e frases por emojis relevantes quando apropriado para adicionar interesse visual e emoção. Use emojis de forma criativa, mas certifique-se de que a mensagem permaneça clara e fácil de entender. Não altere a mensagem principal nem adicione novas informações. |
| User   | Todo o mundo é um palco, e todos os homens e mulheres são apenas atores. Eles têm suas saídas e suas entradas; E um homem em seu tempo representa muitos papéis. |

## Exemplo de saída

Todo o 🌍 é um 🎭, e todos os 👨 e 👩 são apenas 🎭🎬. Eles têm suas 🚪🚶‍♂️ e suas 🚶‍♀️🚪; E um 👨 em seu ⌛ representa muitos 🎭.

---

## Solicitação da API

<Tabs>
<Tab title="Python">
```python
import anthropic

client = anthropic.Anthropic(  # defaults to os.environ.get("ANTHROPIC_API_KEY")
    api_key="my_api_key",
)
message = client.messages.create(
    model="claude-sonnet-4-5",
    max_tokens=1000,
    temperature=0,
    system="Sua tarefa é pegar a mensagem de texto simples fornecida e convertê-la em uma mensagem expressiva e rica em emojis que transmita o mesmo significado e intenção. Substitua palavras-chave e frases por emojis relevantes quando apropriado para adicionar interesse visual e emoção. Use emojis de forma criativa, mas certifique-se de que a mensagem permaneça clara e fácil de entender. Não altere a mensagem principal nem adicione novas informações.",
    messages=[
        {
            "role": "user",
            "content": [
                {
                    "type": "text",
                    "text": "Todo o mundo é um palco, e todos os homens e mulheres são apenas atores. Eles têm suas saídas e suas entradas; E um homem em seu tempo representa muitos papéis.",
                }
            ],
        }
    ],
)
print(message.content)


````
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
  system: "Sua tarefa é pegar a mensagem de texto simples fornecida e convertê-la em uma mensagem expressiva e rica em emojis que transmita o mesmo significado e intenção. Substitua palavras-chave e frases por emojis relevantes quando apropriado para adicionar interesse visual e emoção. Use emojis de forma criativa, mas certifique-se de que a mensagem permaneça clara e fácil de entender. Não altere a mensagem principal nem adicione novas informações.",
  messages: [
    {
      "role": "user",
      "content": [
        {
          "type": "text",
          "text": "Todo o mundo é um palco, e todos os homens e mulheres são apenas atores. Eles têm suas saídas e suas entradas; E um homem em seu tempo representa muitos papéis."
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
    system="Sua tarefa é pegar a mensagem de texto simples fornecida e convertê-la em uma mensagem expressiva e rica em emojis que transmita o mesmo significado e intenção. Substitua palavras-chave e frases por emojis relevantes quando apropriado para adicionar interesse visual e emoção. Use emojis de forma criativa, mas certifique-se de que a mensagem permaneça clara e fácil de entender. Não altere a mensagem principal nem adicione novas informações.",
    messages=[
        {
            "role": "user",
            "content": [
                {
                    "type": "text",
                    "text": "Todo o mundo é um palco, e todos os homens e mulheres são apenas atores. Eles têm suas saídas e suas entradas; E um homem em seu tempo representa muitos papéis."
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
  system: "Sua tarefa é pegar a mensagem de texto simples fornecida e convertê-la em uma mensagem expressiva e rica em emojis que transmita o mesmo significado e intenção. Substitua palavras-chave e frases por emojis relevantes quando apropriado para adicionar interesse visual e emoção. Use emojis de forma criativa, mas certifique-se de que a mensagem permaneça clara e fácil de entender. Não altere a mensagem principal nem adicione novas informações.",
  messages: [
    {
      "role": "user",
      "content": [
        {
          "type": "text",
          "text": "Todo o mundo é um palco, e todos os homens e mulheres são apenas atores. Eles têm suas saídas e suas entradas; E um homem em seu tempo representa muitos papéis."
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
    system="Sua tarefa é pegar a mensagem de texto simples fornecida e convertê-la em uma mensagem expressiva e rica em emojis que transmita o mesmo significado e intenção. Substitua palavras-chave e frases por emojis relevantes quando apropriado para adicionar interesse visual e emoção. Use emojis de forma criativa, mas certifique-se de que a mensagem permaneça clara e fácil de entender. Não altere a mensagem principal nem adicione novas informações.",
    messages=[
        {
            "role": "user",
            "content": [
                {
                    "type": "text",
                    "text": "Todo o mundo é um palco, e todos os homens e mulheres são apenas atores. Eles têm suas saídas e suas entradas; E um homem em seu tempo representa muitos papéis."
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
  system: "Sua tarefa é pegar a mensagem de texto simples fornecida e convertê-la em uma mensagem expressiva e rica em emojis que transmita o mesmo significado e intenção. Substitua palavras-chave e frases por emojis relevantes quando apropriado para adicionar interesse visual e emoção. Use emojis de forma criativa, mas certifique-se de que a mensagem permaneça clara e fácil de entender. Não altere a mensagem principal nem adicione novas informações.",
  messages: [
    {
      "role": "user",
      "content": [
        {
          "type": "text",
          "text": "Todo o mundo é um palco, e todos os homens e mulheres são apenas atores. Eles têm suas saídas e suas entradas; E um homem em seu tempo representa muitos papéis."
        }
      ]
    }
  ]
});
console.log(msg);

```

</Tab>
</Tabs>