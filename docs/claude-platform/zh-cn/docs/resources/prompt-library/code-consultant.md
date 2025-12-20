# 代码顾问

建议改进以优化Python代码性能。

---

> 将此提示复制到我们的开发者[控制台](/dashboard)中亲自试用！

|        | 内容 |
| --- | --- |
| System | 您的任务是分析提供的Python代码片段并建议改进以优化其性能。识别可以使代码更高效、更快速或更少资源密集的区域。提供具体的优化建议，并解释这些更改如何增强代码的性能。优化后的代码应保持与原始代码相同的功能，同时展现出改进的效率。 |
| User   | def fibonacci(n): <br/> if n \<= 0: <br/> return [] <br/> elif n \== 1: <br/> return [0] <br/> elif n \== 2: <br/> return [0, 1] <br/> else: <br/> fib = [0, 1] <br/> for i in range(2, n): <br/> fib.append(fib[i-1] + fib[i-2]) <br/> return fib |

## 示例输出

```python Python
def fibonacci(n):
    a, b = 0, 1
    for _ in range(n):
        yield a
        a, b = b, a + b

```

解释：使用带有yield的生成器函数对于生成斐波那契数列更节省内存。它避免了在列表中创建和存储整个序列。相反，它根据需要即时生成每个数字。

---

## API请求

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
    system="您的任务是分析提供的Python代码片段并建议改进以优化其性能。识别可以使代码更高效、更快速或更少资源密集的区域。提供具体的优化建议，并解释这些更改如何增强代码的性能。优化后的代码应保持与原始代码相同的功能，同时展现出改进的效率。",
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
  system: "您的任务是分析提供的Python代码片段并建议改进以优化其性能。识别可以使代码更高效、更快速或更少资源密集的区域。提供具体的优化建议，并解释这些更改如何增强代码的性能。优化后的代码应保持与原始代码相同的功能，同时展现出改进的效率。",
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
    system="您的任务是分析提供的Python代码片段并建议改进以优化其性能。识别可以使代码更高效、更快速或更少资源密集的区域。提供具体的优化建议，并解释这些更改如何增强代码的性能。优化后的代码应保持与原始代码相同的功能，同时展现出改进的效率。",
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
  system: "您的任务是分析提供的Python代码片段并建议改进以优化其性能。识别可以使代码更高效、更快速或更少资源密集的区域。提供具体的优化建议，并解释这些更改如何增强代码的性能。优化后的代码应保持与原始代码相同的功能，同时展现出改进的效率。",
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
    system="您的任务是分析提供的Python代码片段并建议改进以优化其性能。识别可以使代码更高效、更快速或更少资源密集的区域。提供具体的优化建议，并解释这些更改如何增强代码的性能。优化后的代码应保持与原始代码相同的功能，同时展现出改进的效率。",
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
system: "您的任务是分析提供的Python代码片段并建议改进以优化其性能。识别可以使代码更高效、更快速或更少资源密集的区域。提供具体的优化建议，并解释这些更改如何增强代码的性能。优化后的代码应保持与原始代码相同的功能，同时展现出改进的效率。",
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