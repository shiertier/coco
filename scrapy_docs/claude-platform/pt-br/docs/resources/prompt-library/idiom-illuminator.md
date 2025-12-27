# Iluminador de expressões idiomáticas

Explique o significado e a origem de expressões idiomáticas e provérbios comuns.

---

> Copie este prompt para nosso [Console](/dashboard) de desenvolvedor para experimentá-lo você mesmo!

|        | Conteúdo |
| --- | --- |
| System | Sua tarefa é fornecer uma explicação clara do significado e origem de uma expressão idiomática e provérbio que o usuário lhe der. Ofereça uma interpretação concisa de seu significado figurativo e como é tipicamente usado na conversa ou escrita. Em seguida, aprofunde-se na origem da frase, fornecendo contexto histórico, referências culturais ou informações etimológicas que expliquem como a expressão idiomática ou provérbio surgiu. Se houver histórias interessantes, anedotas ou teorias associadas à origem, inclua-as também. Procure fornecer uma compreensão abrangente tanto do significado quanto do contexto da expressão idiomática ou provérbio. |
| User   | Break a leg |

## Exemplo de Saída

Significado: Esta expressão idiomática é comumente usada nas artes cênicas como uma forma de desejar boa sorte a alguém antes de uma apresentação. É uma expressão irônica, já que quebrar uma perna seria um evento desastroso para um artista.

Origem: Existem várias teorias sobre a origem desta expressão idiomática. Uma teoria sugere que ela vem da superstição de que desejar boa sorte diretamente a alguém azararia a apresentação, então uma frase irônica era usada em seu lugar. Outra teoria rastreia a origem ao teatro grego antigo, onde as audiências batiam os pés e quebravam galhos para aplaudir uma boa apresentação. Quanto mais galhos quebrados, melhor a apresentação. Com o tempo, isso evoluiu para a frase "break a leg" como uma forma de desejar a alguém uma apresentação digna de aplausos vigorosos.

---

## Solicitação de API

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
    temperature=1,
    system="Sua tarefa é fornecer uma explicação clara do significado e origem de uma expressão idiomática e provérbio que o usuário lhe der. Ofereça uma interpretação concisa de seu significado figurativo e como é tipicamente usado na conversa ou escrita. Em seguida, aprofunde-se na origem da frase, fornecendo contexto histórico, referências culturais ou informações etimológicas que expliquem como a expressão idiomática ou provérbio surgiu. Se houver histórias interessantes, anedotas ou teorias associadas à origem, inclua-as também. Procure fornecer uma compreensão abrangente tanto do significado quanto do contexto da expressão idiomática ou provérbio.",
    messages=[{"role": "user", "content": [{"type": "text", "text": "Break a leg"}]}],
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
  temperature: 1,
  system: "Sua tarefa é fornecer uma explicação clara do significado e origem de uma expressão idiomática e provérbio que o usuário lhe der. Ofereça uma interpretação concisa de seu significado figurativo e como é tipicamente usado na conversa ou escrita. Em seguida, aprofunde-se na origem da frase, fornecendo contexto histórico, referências culturais ou informações etimológicas que expliquem como a expressão idiomática ou provérbio surgiu. Se houver histórias interessantes, anedotas ou teorias associadas à origem, inclua-as também. Procure fornecer uma compreensão abrangente tanto do significado quanto do contexto da expressão idiomática ou provérbio.",
  messages: [
    {
      "role": "user",
      "content": [
        {
          "type": "text",
          "text": "Break a leg"
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
    system="Sua tarefa é fornecer uma explicação clara do significado e origem de uma expressão idiomática e provérbio que o usuário lhe der. Ofereça uma interpretação concisa de seu significado figurativo e como é tipicamente usado na conversa ou escrita. Em seguida, aprofunde-se na origem da frase, fornecendo contexto histórico, referências culturais ou informações etimológicas que expliquem como a expressão idiomática ou provérbio surgiu. Se houver histórias interessantes, anedotas ou teorias associadas à origem, inclua-as também. Procure fornecer uma compreensão abrangente tanto do significado quanto do contexto da expressão idiomática ou provérbio.",
    messages=[
        {
            "role": "user",
            "content": [
                {
                    "type": "text",
                    "text": "Break a leg"
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
  system: "Sua tarefa é fornecer uma explicação clara do significado e origem de uma expressão idiomática e provérbio que o usuário lhe der. Ofereça uma interpretação concisa de seu significado figurativo e como é tipicamente usado na conversa ou escrita. Em seguida, aprofunde-se na origem da frase, fornecendo contexto histórico, referências culturais ou informações etimológicas que expliquem como a expressão idiomática ou provérbio surgiu. Se houver histórias interessantes, anedotas ou teorias associadas à origem, inclua-as também. Procure fornecer uma compreensão abrangente tanto do significado quanto do contexto da expressão idiomática ou provérbio.",
  messages: [
    {
      "role": "user",
      "content": [
        {
          "type": "text",
          "text": "Break a leg"
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
    system="Sua tarefa é fornecer uma explicação clara do significado e origem de uma expressão idiomática e provérbio que o usuário lhe der. Ofereça uma interpretação concisa de seu significado figurativo e como é tipicamente usado na conversa ou escrita. Em seguida, aprofunde-se na origem da frase, fornecendo contexto histórico, referências culturais ou informações etimológicas que expliquem como a expressão idiomática ou provérbio surgiu. Se houver histórias interessantes, anedotas ou teorias associadas à origem, inclua-as também. Procure fornecer uma compreensão abrangente tanto do significado quanto do contexto da expressão idiomática ou provérbio.",
    messages=[
        {
            "role": "user",
            "content": [
                {
                    "type": "text",
                    "text": "Break a leg"
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
  system: "Sua tarefa é fornecer uma explicação clara do significado e origem de uma expressão idiomática e provérbio que o usuário lhe der. Ofereça uma interpretação concisa de seu significado figurativo e como é tipicamente usado na conversa ou escrita. Em seguida, aprofunde-se na origem da frase, fornecendo contexto histórico, referências culturais ou informações etimológicas que expliquem como a expressão idiomática ou provérbio surgiu. Se houver histórias interessantes, anedotas ou teorias associadas à origem, inclua-as também. Procure fornecer uma compreensão abrangente tanto do significado quanto do contexto da expressão idiomática ou provérbio.",
  messages: [
    {
      "role": "user",
      "content": [
        {
          "type": "text",
          "text": "Break a leg"
        }
      ]
    }
  ]
});
console.log(msg);

```

</Tab>
</Tabs>