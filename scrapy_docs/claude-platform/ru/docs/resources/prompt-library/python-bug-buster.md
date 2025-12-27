# Исправитель ошибок Python

Обнаружение и исправление ошибок в коде Python.

---

> Скопируйте этот промпт в нашу [Консоль](/dashboard) разработчика, чтобы попробовать его самостоятельно!

|        | Содержание |
| --- | --- |
| System | Ваша задача - проанализировать предоставленный фрагмент кода Python, выявить любые ошибки или баги, и предоставить исправленную версию кода, которая решает эти проблемы. Объясните проблемы, которые вы нашли в исходном коде, и как ваши исправления их решают. Исправленный код должен быть функциональным, эффективным и соответствовать лучшим практикам программирования на Python. |
| User   | def calculate_average(nums): <br/> sum = 0 <br/> for num in nums: <br/> sum += num <br/> average = sum / len(nums) <br/> return average <br/><br/> numbers = [10, 20, 30, 40, 50] <br/> result = calculate_average(numbers) <br/> print("The average is:", results) |

## Пример вывода

Исправленный код Python:

```python Python
def calculate_average(nums):
    total = 0
    for num in nums:
        total += num
    average = total / len(nums)
    return average

numbers = [10, 20, 30, 40, 50]
result = calculate_average(numbers)
print("The average is:", result)
```

Объяснение:

    1. Изменил имя переменной "sum" на "total", чтобы избежать использования встроенной функции Python "sum()".
    2. Исправил опечатку в операторе print, изменив "results" на "result" для корректной ссылки на переменную.

## API запрос

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
  system="Ваша задача - проанализировать предоставленный фрагмент кода Python, выявить любые ошибки или баги, и предоставить исправленную версию кода, которая решает эти проблемы. Объясните проблемы, которые вы нашли в исходном коде, и как ваши исправления их решают. Исправленный код должен быть функциональным, эффективным и соответствовать лучшим практикам программирования на Python.",
  messages=[
    {
    "role": "user",
    "content": [
        {
          "type": "text",
          "text": "def calculate_average(nums):\n sum = 0\n for num in nums:\n sum += num\n average = sum / len(nums)\n return average\n\nnumbers = [10, 20, 30, 40, 50]\nresult = calculate_average(numbers)\nprint(\"The average is:\", results)"
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
  system: "Ваша задача - проанализировать предоставленный фрагмент кода Python, выявить любые ошибки или баги, и предоставить исправленную версию кода, которая решает эти проблемы. Объясните проблемы, которые вы нашли в исходном коде, и как ваши исправления их решают. Исправленный код должен быть функциональным, эффективным и соответствовать лучшим практикам программирования на Python.",
  messages: [
    {
      "role": "user",
      "content": [
        {
          "type": "text",
          "text": "def calculate_average(nums):\n   sum = 0\n   for num in nums:\n      sum += num\n   average = sum / len(nums)\n   return average\n\nnumbers = [10, 20, 30, 40, 50]\nresult = calculate_average(numbers)\nprint(\"The average is:\", results)"
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
system="Ваша задача - проанализировать предоставленный фрагмент кода Python, выявить любые ошибки или баги, и предоставить исправленную версию кода, которая решает эти проблемы. Объясните проблемы, которые вы нашли в исходном коде, и как ваши исправления их решают. Исправленный код должен быть функциональным, эффективным и соответствовать лучшим практикам программирования на Python.",
messages=[
{
"role": "user",
"content": [
{
"type": "text",
"text": "def calculate_average(nums):\n sum = 0\n for num in nums:\n sum += num\n average = sum / len(nums)\n return average\n\nnumbers = [10, 20, 30, 40, 50]\nresult = calculate_average(numbers)\nprint(\"The average is:\", results)"
}
]
}
]
)
print(message.content)

````
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
  system: "Ваша задача - проанализировать предоставленный фрагмент кода Python, выявить любые ошибки или баги, и предоставить исправленную версию кода, которая решает эти проблемы. Объясните проблемы, которые вы нашли в исходном коде, и как ваши исправления их решают. Исправленный код должен быть функциональным, эффективным и соответствовать лучшим практикам программирования на Python.",
  messages: [
    {
      "role": "user",
      "content": [
        {
          "type": "text",
          "text": "def calculate_average(nums):\n   sum = 0\n   for num in nums:\n      sum += num\n   average = sum / len(nums)\n   return average\n\nnumbers = [10, 20, 30, 40, 50]\nresult = calculate_average(numbers)\nprint(\"The average is:\", results)"
        }
      ]
    }
  ]
});
console.log(msg);
````

  </Tab>
    <Tab title="Vertex AI Python">
```python
from anthropic import AnthropicVertex

client = AnthropicVertex()

message = client.messages.create(
model="claude-sonnet-4@20250514",
max_tokens=1000,
temperature=0,
system="Ваша задача - проанализировать предоставленный фрагмент кода Python, выявить любые ошибки или баги, и предоставить исправленную версию кода, которая решает эти проблемы. Объясните проблемы, которые вы нашли в исходном коде, и как ваши исправления их решают. Исправленный код должен быть функциональным, эффективным и соответствовать лучшим практикам программирования на Python.",
messages=[
{
"role": "user",
"content": [
{
"type": "text",
"text": "def calculate_average(nums):\n sum = 0\n for num in nums:\n sum += num\n average = sum / len(nums)\n return average\n\nnumbers = [10, 20, 30, 40, 50]\nresult = calculate_average(numbers)\nprint(\"The average is:\", results)"
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
  system: "Ваша задача - проанализировать предоставленный фрагмент кода Python, выявить любые ошибки или баги, и предоставить исправленную версию кода, которая решает эти проблемы. Объясните проблемы, которые вы нашли в исходном коде, и как ваши исправления их решают. Исправленный код должен быть функциональным, эффективным и соответствовать лучшим практикам программирования на Python.",
  messages: [
    {
      "role": "user",
      "content": [
        {
          "type": "text",
          "text": "def calculate_average(nums):\n   sum = 0\n   for num in nums:\n      sum += num\n   average = sum / len(nums)\n   return average\n\nnumbers = [10, 20, 30, 40, 50]\nresult = calculate_average(numbers)\nprint(\"The average is:\", results)"
        }
      ]
    }
  ]
});
console.log(msg);
````

  </Tab>
</Tabs>