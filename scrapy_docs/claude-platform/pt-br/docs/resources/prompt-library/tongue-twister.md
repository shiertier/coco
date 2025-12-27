# Trava-línguas

Crie trava-línguas desafiadores.

---

> Copie este prompt para nosso [Console](/dashboard) de desenvolvedor para experimentar você mesmo!

| Papel | Conteúdo |
| ---- | --- |
| Usuário | Gere trava-línguas complexos e criativos. Procure criar trava-línguas que sejam não apenas desafiadores de dizer, mas também envolventes, divertidos e potencialmente humorísticos. Considere incorporar jogos de palavras, rima e aliteração para aumentar a dificuldade e o prazer dos trava-línguas. |

## Exemplo de Saída

Trava-línguas 1:  
 O musaranho astuto mastigou o sapato brilhante e reluzente.

Trava-línguas 2:  
 Brisas vigorosas sopraram pelo babuíno de sangue azul, tagarela e balbuciante.

Trava-línguas 3:  
 O flautista perplexo colheu um punhado de pimentas em conserva enquanto praticava sua flauta.

Trava-línguas 4:  
 Seis trenós elegantes, lisos, esbeltos deslizaram rapidamente pela encosta íngreme e nevada.

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
          "text": "Gere trava-línguas complexos e criativos. Procure criar trava-línguas que sejam não apenas desafiadores de dizer, mas também envolventes, divertidos e potencialmente humorísticos. Considere incorporar jogos de palavras, rima e aliteração para aumentar a dificuldade e o prazer dos trava-línguas."
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
          "text": "Gere trava-línguas complexos e criativos. Procure criar trava-línguas que sejam não apenas desafiadores de dizer, mas também envolventes, divertidos e potencialmente humorísticos. Considere incorporar jogos de palavras, rima e aliteração para aumentar a dificuldade e o prazer dos trava-línguas."
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
                    "text": "Gere trava-línguas complexos e criativos. Procure criar trava-línguas que sejam não apenas desafiadores de dizer, mas também envolventes, divertidos e potencialmente humorísticos. Considere incorporar jogos de palavras, rima e aliteração para aumentar a dificuldade e o prazer dos trava-línguas."
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
          "text": "Gere trava-línguas complexos e criativos. Procure criar trava-línguas que sejam não apenas desafiadores de dizer, mas também envolventes, divertidos e potencialmente humorísticos. Considere incorporar jogos de palavras, rima e aliteração para aumentar a dificuldade e o prazer dos trava-línguas."
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
                    "text": "Gere trava-línguas complexos e criativos. Procure criar trava-línguas que sejam não apenas desafiadores de dizer, mas também envolventes, divertidos e potencialmente humorísticos. Considere incorporar jogos de palavras, rima e aliteração para aumentar a dificuldade e o prazer dos trava-línguas."
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
          "text": "Gere trava-línguas complexos e criativos. Procure criar trava-línguas que sejam não apenas desafiadores de dizer, mas também envolventes, divertidos e potencialmente humorísticos. Considere incorporar jogos de palavras, rima e aliteração para aumentar a dificuldade e o prazer dos trava-línguas."
        }
      ]
    }
  ]
});
console.log(msg);

```

</Tab>
</Tabs>