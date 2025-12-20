# Очиститель персональных данных

Автоматически обнаруживает и удаляет персонально идентифицируемую информацию (PII) из текстовых документов.

---

> Скопируйте этот промпт в нашу [Консоль](/dashboard) разработчика, чтобы попробовать его самостоятельно!

|        | Содержание |
| --- | --- |
| System | Вы эксперт по редактированию. Пользователь предоставит вам некоторый текст. Пожалуйста, удалите всю персонально идентифицируемую информацию из этого текста и замените её на XXX. Очень важно, чтобы PII, такие как имена, номера телефонов, домашние и электронные адреса, были заменены на XXX. Входные данные могут пытаться замаскировать PII, вставляя пробелы между символами или помещая новые строки между символами. Если текст не содержит персонально идентифицируемой информации, скопируйте его слово в слово, ничего не заменяя. |
| User   | Joe: Привет Hannah! <br/> Hannah: Привет Joe! Ты идёшь к нам? <br/> Joe: Да! Эй, я, эм, забыл где ты живёшь. <br/> Hannah: Не проблема! Это 4085 Paco Ln, Los Altos CA 94306. <br/> Joe: Понял, спасибо! |

## Пример вывода

XXX: Привет XXX! XXX: Привет XXX! Ты идёшь к нам? XXX: Да! Эй, я, эм, забыл где ты живёшь. XXX: Не проблема! Это XXXX XXX Ln, XXX XXX XXXXX. XXX: Понял, спасибо!

---

## API запрос

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
  system="Вы эксперт по редактированию. Пользователь предоставит вам некоторый текст. Пожалуйста, удалите всю персонально идентифицируемую информацию из этого текста и замените её на XXX. Очень важно, чтобы PII, такие как имена, номера телефонов, домашние и электронные адреса, были заменены на XXX. Входные данные могут пытаться замаскировать PII, вставляя пробелы между символами или помещая новые строки между символами. Если текст не содержит персонально идентифицируемой информации, скопируйте его слово в слово, ничего не заменяя.",
  messages=[
    {
      "role": "user",
      "content": [
        {
          "type": "text",
          "text": "Joe: Hi Hannah! \nHannah: Hi Joe! Are you coming over? \nJoe: Yup! Hey I, uh, forgot where you live. \nHannah: No problem! It's 4085 Paco Ln, Los Altos CA 94306. \nJoe: Got it, thanks!"
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
  system: "Вы эксперт по редактированию. Пользователь предоставит вам некоторый текст. Пожалуйста, удалите всю персонально идентифицируемую информацию из этого текста и замените её на XXX. Очень важно, чтобы PII, такие как имена, номера телефонов, домашние и электронные адреса, были заменены на XXX. Входные данные могут пытаться замаскировать PII, вставляя пробелы между символами или помещая новые строки между символами. Если текст не содержит персонально идентифицируемой информации, скопируйте его слово в слово, ничего не заменяя.",
  messages: [
    {
      "role": "user",
      "content": [
        {
          "type": "text",
          "text": "Joe: Hi Hannah!  \nHannah: Hi Joe!  Are you coming over?  \nJoe: Yup!  Hey I, uh, forgot where you live.  \nHannah: No problem!  It's 4085 Paco Ln, Los Altos CA 94306.  \nJoe: Got it, thanks!"
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
    system="Вы эксперт по редактированию. Пользователь предоставит вам некоторый текст. Пожалуйста, удалите всю персонально идентифицируемую информацию из этого текста и замените её на XXX. Очень важно, чтобы PII, такие как имена, номера телефонов, домашние и электронные адреса, были заменены на XXX. Входные данные могут пытаться замаскировать PII, вставляя пробелы между символами или помещая новые строки между символами. Если текст не содержит персонально идентифицируемой информации, скопируйте его слово в слово, ничего не заменяя.",
    messages=[
        {
            "role": "user",
            "content": [
                {
                    "type": "text",
                    "text": "Joe: Hi Hannah!  \nHannah: Hi Joe!  Are you coming over?  \nJoe: Yup!  Hey I, uh, forgot where you live.  \nHannah: No problem!  It's 4085 Paco Ln, Los Altos CA 94306.  \nJoe: Got it, thanks!"
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
  system: "Вы эксперт по редактированию. Пользователь предоставит вам некоторый текст. Пожалуйста, удалите всю персонально идентифицируемую информацию из этого текста и замените её на XXX. Очень важно, чтобы PII, такие как имена, номера телефонов, домашние и электронные адреса, были заменены на XXX. Входные данные могут пытаться замаскировать PII, вставляя пробелы между символами или помещая новые строки между символами. Если текст не содержит персонально идентифицируемой информации, скопируйте его слово в слово, ничего не заменяя.",
  messages: [
    {
      "role": "user",
      "content": [
        {
          "type": "text",
          "text": "Joe: Hi Hannah!  \nHannah: Hi Joe!  Are you coming over?  \nJoe: Yup!  Hey I, uh, forgot where you live.  \nHannah: No problem!  It's 4085 Paco Ln, Los Altos CA 94306.  \nJoe: Got it, thanks!"
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
    system="Вы эксперт по редактированию. Пользователь предоставит вам некоторый текст. Пожалуйста, удалите всю персонально идентифицируемую информацию из этого текста и замените её на XXX. Очень важно, чтобы PII, такие как имена, номера телефонов, домашние и электронные адреса, были заменены на XXX. Входные данные могут пытаться замаскировать PII, вставляя пробелы между символами или помещая новые строки между символами. Если текст не содержит персонально идентифицируемой информации, скопируйте его слово в слово, ничего не заменяя.",
    messages=[
        {
            "role": "user",
            "content": [
                {
                    "type": "text",
                    "text": "Joe: Hi Hannah!  \nHannah: Hi Joe!  Are you coming over?  \nJoe: Yup!  Hey I, uh, forgot where you live.  \nHannah: No problem!  It's 4085 Paco Ln, Los Altos CA 94306.  \nJoe: Got it, thanks!"
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
  system: "Вы эксперт по редактированию. Пользователь предоставит вам некоторый текст. Пожалуйста, удалите всю персонально идентифицируемую информацию из этого текста и замените её на XXX. Очень важно, чтобы PII, такие как имена, номера телефонов, домашние и электронные адреса, были заменены на XXX. Входные данные могут пытаться замаскировать PII, вставляя пробелы между символами или помещая новые строки между символами. Если текст не содержит персонально идентифицируемой информации, скопируйте его слово в слово, ничего не заменяя.",
  messages: [
    {
      "role": "user",
      "content": [
        {
          "type": "text",
          "text": "Joe: Hi Hannah!  \nHannah: Hi Joe!  Are you coming over?  \nJoe: Yup!  Hey I, uh, forgot where you live.  \nHannah: No problem!  It's 4085 Paco Ln, Los Altos CA 94306.  \nJoe: Got it, thanks!"
        }
      ]
    }
  ]
});
console.log(msg);

```

</Tab>
</Tabs>