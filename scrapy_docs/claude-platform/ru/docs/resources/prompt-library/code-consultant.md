# Консультант по коду

Предложите улучшения для оптимизации производительности Python кода.

---

> Скопируйте этот промпт в нашу [Консоль](/dashboard) разработчика, чтобы попробовать его самостоятельно!

|        | Содержание |
| --- | --- |
| System | Ваша задача - проанализировать предоставленный фрагмент Python кода и предложить улучшения для оптимизации его производительности. Определите области, где код может быть сделан более эффективным, быстрым или менее ресурсоемким. Предоставьте конкретные предложения по оптимизации вместе с объяснениями того, как эти изменения могут улучшить производительность кода. Оптимизированный код должен сохранять ту же функциональность, что и исходный код, демонстрируя при этом улучшенную эффективность. |
| User   | def fibonacci(n): <br/> if n \<= 0: <br/> return [] <br/> elif n \== 1: <br/> return [0] <br/> elif n \== 2: <br/> return [0, 1] <br/> else: <br/> fib = [0, 1] <br/> for i in range(2, n): <br/> fib.append(fib[i-1] + fib[i-2]) <br/> return fib |

## Пример вывода

```python Python
def fibonacci(n):
    a, b = 0, 1
    for _ in range(n):
        yield a
        a, b = b, a + b

```

Объяснение: Использование функции-генератора с yield более эффективно по памяти для генерации последовательности Фибоначчи. Это позволяет избежать создания и хранения всей последовательности в списке. Вместо этого она генерирует каждое число на лету по мере необходимости.

---

## API Запрос

<Tabs>
<Tab title="Python">
```python
import anthropic

client = anthropic.Anthropic(  # по умолчанию os.environ.get("ANTHROPIC_API_KEY")
    api_key="my_api_key",
)
message = client.messages.create(
    model="claude-sonnet-4-5",
    max_tokens=1000,
    temperature=0,
    system="Ваша задача - проанализировать предоставленный фрагмент Python кода и предложить улучшения для оптимизации его производительности. Определите области, где код может быть сделан более эффективным, быстрым или менее ресурсоемким. Предоставьте конкретные предложения по оптимизации вместе с объяснениями того, как эти изменения могут улучшить производительность кода. Оптимизированный код должен сохранять ту же функциональность, что и исходный код, демонстрируя при этом улучшенную эффективность.",
    messages=[
        {
            "role": "user",
            "content": [
                {
                    "type": "text",
                    "text": "def fibonacci(n):\n if n <= 0:\n return []\n elif n == 1:\n return [0]\n elif n == 2:\n return [0, 1]\n else:\n fib = [0, 1]\n for i in range(2, n):\n fib.append(fib[i-1] + fib[i-2])\n return fib",
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
  apiKey: "my_api_key", // по умолчанию process.env["ANTHROPIC_API_KEY"]
});

const msg = await anthropic.messages.create({
  model: "claude-sonnet-4-5",
  max_tokens: 1000,
  temperature: 0,
  system: "Ваша задача - проанализировать предоставленный фрагмент Python кода и предложить улучшения для оптимизации его производительности. Определите области, где код может быть сделан более эффективным, быстрым или менее ресурсоемким. Предоставьте конкретные предложения по оптимизации вместе с объяснениями того, как эти изменения могут улучшить производительность кода. Оптимизированный код должен сохранять ту же функциональность, что и исходный код, демонстрируя при этом улучшенную эффективность.",
  messages: [
    {
      "role": "user",
      "content": [
        {
          "type": "text",
          "text": "def fibonacci(n):\n    if n <= 0:\n        return []\n    elif n == 1:\n        return [0]\n    elif n == 2:\n        return [0, 1]\n    else:\n        fib = [0, 1]\n        for i in range(2, n):\n            fib.append(fib[i-1] + fib[i-2])\n    return fib"
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

# См. https://docs.claude.com/claude/reference/claude-on-amazon-bedrock
# для вариантов аутентификации
client = AnthropicBedrock()

message = client.messages.create(
    model="anthropic.claude-sonnet-4-5-20250929-v1:0",
    max_tokens=1000,
    temperature=0,
    system="Ваша задача - проанализировать предоставленный фрагмент Python кода и предложить улучшения для оптимизации его производительности. Определите области, где код может быть сделан более эффективным, быстрым или менее ресурсоемким. Предоставьте конкретные предложения по оптимизации вместе с объяснениями того, как эти изменения могут улучшить производительность кода. Оптимизированный код должен сохранять ту же функциональность, что и исходный код, демонстрируя при этом улучшенную эффективность.",
    messages=[
        {
            "role": "user",
            "content": [
                {
                    "type": "text",
                    "text": "def fibonacci(n):\n    if n <= 0:\n        return []\n    elif n == 1:\n        return [0]\n    elif n == 2:\n        return [0, 1]\n    else:\n        fib = [0, 1]\n        for i in range(2, n):\n            fib.append(fib[i-1] + fib[i-2])\n    return fib"
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

// См. https://docs.claude.com/claude/reference/claude-on-amazon-bedrock
// для вариантов аутентификации
const client = new AnthropicBedrock();

const msg = await client.messages.create({
  model: "anthropic.claude-sonnet-4-5-20250929-v1:0",
  max_tokens: 1000,
  temperature: 0,
  system: "Ваша задача - проанализировать предоставленный фрагмент Python кода и предложить улучшения для оптимизации его производительности. Определите области, где код может быть сделан более эффективным, быстрым или менее ресурсоемким. Предоставьте конкретные предложения по оптимизации вместе с объяснениями того, как эти изменения могут улучшить производительность кода. Оптимизированный код должен сохранять ту же функциональность, что и исходный код, демонстрируя при этом улучшенную эффективность.",
  messages: [
    {
      "role": "user",
      "content": [
        {
          "type": "text",
          "text": "def fibonacci(n):\n    if n <= 0:\n        return []\n    elif n == 1:\n        return [0]\n    elif n == 2:\n        return [0, 1]\n    else:\n        fib = [0, 1]\n        for i in range(2, n):\n            fib.append(fib[i-1] + fib[i-2])\n    return fib"
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
    system="Ваша задача - проанализировать предоставленный фрагмент Python кода и предложить улучшения для оптимизации его производительности. Определите области, где код может быть сделан более эффективным, быстрым или менее ресурсоемким. Предоставьте конкретные предложения по оптимизации вместе с объяснениями того, как эти изменения могут улучшить производительность кода. Оптимизированный код должен сохранять ту же функциональность, что и исходный код, демонстрируя при этом улучшенную эффективность.",
    messages=[
        {
            "role": "user",
            "content": [
                {
                    "type": "text",
                    "text": "def fibonacci(n):\n    if n <= 0:\n        return []\n    elif n == 1:\n        return [0]\n    elif n == 2:\n        return [0, 1]\n    else:\n        fib = [0, 1]\n        for i in range(2, n):\n            fib.append(fib[i-1] + fib[i-2])\n    return fib"
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

// Читает из переменных окружения `CLOUD_ML_REGION` и `ANTHROPIC_VERTEX_PROJECT_ID`.
// Дополнительно проходит через стандартный поток `google-auth-library`.
const client = new AnthropicVertex();

const msg = await client.messages.create({
model: "claude-sonnet-4@20250514",
max_tokens: 1000,
temperature: 0,
system: "Ваша задача - проанализировать предоставленный фрагмент Python кода и предложить улучшения для оптимизации его производительности. Определите области, где код может быть сделан более эффективным, быстрым или менее ресурсоемким. Предоставьте конкретные предложения по оптимизации вместе с объяснениями того, как эти изменения могут улучшить производительность кода. Оптимизированный код должен сохранять ту же функциональность, что и исходный код, демонстрируя при этом улучшенную эффективность.",
messages: [
{
"role": "user",
"content": [
{
"type": "text",
"text": "def fibonacci(n):\n if n <= 0:\n return []\n elif n == 1:\n return [0]\n elif n == 2:\n return [0, 1]\n else:\n fib = [0, 1]\n for i in range(2, n):\n fib.append(fib[i-1] + fib[i-2])\n return fib"
}
]
}
]
});
console.log(msg);

```
</Tab>
</Tabs>
```