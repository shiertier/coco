# Git gud

Генерируйте подходящие команды Git на основе описанных пользователем действий контроля версий.

---

> Скопируйте этот промпт в нашу [Консоль](/dashboard) разработчика, чтобы попробовать его самостоятельно!

| Роль | Содержание |
| ---- | --- |
| Пользователь | Я внес некоторые изменения в свои локальные файлы и хочу сохранить их в своем локальном Git репозитории. Какую команду Git мне следует использовать? |

## Пример вывода

Чтобы сохранить изменения, которые вы внесли в свои локальные файлы в вашем локальном Git репозитории, вам следует использовать следующие команды Git:

1.  `git add .` или `git add <файл>`

    - Эта команда подготавливает внесенные вами изменения, готовя их к коммиту. Точка `.` добавляет все изменения в текущем каталоге и его подкаталогах. Если вы хотите подготовить только определенные файлы, замените `.` на имя файла или путь.

2.  `git commit -m "Ваше сообщение коммита"`

    - Эта команда фиксирует подготовленные изменения в вашем локальном Git репозитории. Флаг `-m` позволяет вам предоставить сообщение коммита, описывающее внесенные вами изменения. Замените `"Ваше сообщение коммита"` на краткое, описательное резюме ваших изменений.

Например, если вы внесли изменения в файл с именем `example.txt`, вы бы выполнили:

```
git add example.txt
git commit -m "Обновить example.txt с новым содержимым"

```

Эти команды сохранят ваши изменения в вашем локальном Git репозитории, создав новый коммит с предоставленным сообщением. Позже вы можете отправить эти изменения в удаленный репозиторий или продолжить работу над вашей локальной веткой.

Помните, что нужно выполнять эти команды из каталога вашего Git репозитория в вашем терминале или командной строке.

---

## API запрос

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
    messages=[
        {
            "role": "user",
            "content": [
                {
                    "type": "text",
                    "text": "I have made some changes to my local files and I want to save them in my local Git repository. What Git command should I use?",
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
  messages: [
    {
      "role": "user",
      "content": [
        {
          "type": "text",
          "text": "I have made some changes to my local files and I want to save them in my local Git repository. What Git command should I use?"
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
                    "text": "I have made some changes to my local files and I want to save them in my local Git repository. What Git command should I use?"
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
          "text": "I have made some changes to my local files and I want to save them in my local Git repository. What Git command should I use?"
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
                    "text": "I have made some changes to my local files and I want to save them in my local Git repository. What Git command should I use?"
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
          "text": "I have made some changes to my local files and I want to save them in my local Git repository. What Git command should I use?"
        }
      ]
    }
  ]
});
console.log(msg);

```

</Tab>
</Tabs>