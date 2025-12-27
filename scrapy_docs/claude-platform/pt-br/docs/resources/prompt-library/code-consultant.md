# Consultor de código

Sugira melhorias para otimizar o desempenho do código Python.

---

> Copie este prompt para nosso [Console](/dashboard) de desenvolvedor para experimentá-lo você mesmo!

|        | Conteúdo |
| --- | --- |
| Sistema | Sua tarefa é analisar o trecho de código Python fornecido e sugerir melhorias para otimizar seu desempenho. Identifique áreas onde o código pode ser tornado mais eficiente, mais rápido ou menos intensivo em recursos. Forneça sugestões específicas para otimização, juntamente com explicações de como essas mudanças podem melhorar o desempenho do código. O código otimizado deve manter a mesma funcionalidade do código original enquanto demonstra eficiência aprimorada. |
| Usuário   | def fibonacci(n): <br/> if n \<= 0: <br/> return [] <br/> elif n \== 1: <br/> return [0] <br/> elif n \== 2: <br/> return [0, 1] <br/> else: <br/> fib = [0, 1] <br/> for i in range(2, n): <br/> fib.append(fib[i-1] + fib[i-2]) <br/> return fib |

## Exemplo de Saída

```python Python
def fibonacci(n):
    a, b = 0, 1
    for _ in range(n):
        yield a
        a, b = b, a + b

```

Explicação: Usar uma função geradora com yield é mais eficiente em memória para gerar a sequência de Fibonacci. Evita criar e armazenar toda a sequência em uma lista. Em vez disso, gera cada número dinamicamente conforme necessário.

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
    system="Sua tarefa é analisar o trecho de código Python fornecido e sugerir melhorias para otimizar seu desempenho. Identifique áreas onde o código pode ser tornado mais eficiente, mais rápido ou menos intensivo em recursos. Forneça sugestões específicas para otimização, juntamente com explicações de como essas mudanças podem melhorar o desempenho do código. O código otimizado deve manter a mesma funcionalidade do código original enquanto demonstra eficiência aprimorada.",
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
  apiKey: "my_api_key", // defaults to process.env["ANTHROPIC_API_KEY"]
});

const msg = await anthropic.messages.create({
  model: "claude-sonnet-4-5",
  max_tokens: 1000,
  temperature: 0,
  system: "Sua tarefa é analisar o trecho de código Python fornecido e sugerir melhorias para otimizar seu desempenho. Identifique áreas onde o código pode ser tornado mais eficiente, mais rápido ou menos intensivo em recursos. Forneça sugestões específicas para otimização, juntamente com explicações de como essas mudanças podem melhorar o desempenho do código. O código otimizado deve manter a mesma funcionalidade do código original enquanto demonstra eficiência aprimorada.",
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

# See https://docs.claude.com/claude/reference/claude-on-amazon-bedrock
# for authentication options
client = AnthropicBedrock()

message = client.messages.create(
    model="anthropic.claude-sonnet-4-5-20250929-v1:0",
    max_tokens=1000,
    temperature=0,
    system="Sua tarefa é analisar o trecho de código Python fornecido e sugerir melhorias para otimizar seu desempenho. Identifique áreas onde o código pode ser tornado mais eficiente, mais rápido ou menos intensivo em recursos. Forneça sugestões específicas para otimização, juntamente com explicações de como essas mudanças podem melhorar o desempenho do código. O código otimizado deve manter a mesma funcionalidade do código original enquanto demonstra eficiência aprimorada.",
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

// See https://docs.claude.com/claude/reference/claude-on-amazon-bedrock
// for authentication options
const client = new AnthropicBedrock();

const msg = await client.messages.create({
  model: "anthropic.claude-sonnet-4-5-20250929-v1:0",
  max_tokens: 1000,
  temperature: 0,
  system: "Sua tarefa é analisar o trecho de código Python fornecido e sugerir melhorias para otimizar seu desempenho. Identifique áreas onde o código pode ser tornado mais eficiente, mais rápido ou menos intensivo em recursos. Forneça sugestões específicas para otimização, juntamente com explicações de como essas mudanças podem melhorar o desempenho do código. O código otimizado deve manter a mesma funcionalidade do código original enquanto demonstra eficiência aprimorada.",
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
    system="Sua tarefa é analisar o trecho de código Python fornecido e sugerir melhorias para otimizar seu desempenho. Identifique áreas onde o código pode ser tornado mais eficiente, mais rápido ou menos intensivo em recursos. Forneça sugestões específicas para otimização, juntamente com explicações de como essas mudanças podem melhorar o desempenho do código. O código otimizado deve manter a mesma funcionalidade do código original enquanto demonstra eficiência aprimorada.",
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

// Reads from the `CLOUD_ML_REGION` & `ANTHROPIC_VERTEX_PROJECT_ID` environment variables.
// Additionally goes through the standard `google-auth-library` flow.
const client = new AnthropicVertex();

const msg = await client.messages.create({
model: "claude-sonnet-4@20250514",
max_tokens: 1000,
temperature: 0,
system: "Sua tarefa é analisar o trecho de código Python fornecido e sugerir melhorias para otimizar seu desempenho. Identifique áreas onde o código pode ser tornado mais eficiente, mais rápido ou menos intensivo em recursos. Forneça sugestões específicas para otimização, juntamente com explicações de como essas mudanças podem melhorar o desempenho do código. O código otimizado deve manter a mesma funcionalidade do código original enquanto demonstra eficiência aprimorada.",
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