# 程式碼顧問

建議改進以優化 Python 程式碼效能。

---

> 將此提示複製到我們的開發者 [Console](/dashboard) 來親自試試看！

|        | 內容 |
| --- | --- |
| System | 您的任務是分析提供的 Python 程式碼片段，並建議改進以優化其效能。識別程式碼可以變得更高效、更快速或更少資源密集的區域。提供具體的優化建議，並解釋這些變更如何增強程式碼的效能。優化後的程式碼應保持與原始程式碼相同的功能，同時展現改進的效率。 |
| User   | def fibonacci(n): <br/> if n \<= 0: <br/> return [] <br/> elif n \== 1: <br/> return [0] <br/> elif n \== 2: <br/> return [0, 1] <br/> else: <br/> fib = [0, 1] <br/> for i in range(2, n): <br/> fib.append(fib[i-1] + fib[i-2]) <br/> return fib |

## 範例輸出

```python Python
def fibonacci(n):
    a, b = 0, 1
    for _ in range(n):
        yield a
        a, b = b, a + b

```

解釋：使用帶有 yield 的生成器函數對於生成費波那契數列更具記憶體效率。它避免了在列表中創建和儲存整個數列。相反，它根據需要即時生成每個數字。

---

## API 請求

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
    system="您的任務是分析提供的 Python 程式碼片段，並建議改進以優化其效能。識別程式碼可以變得更高效、更快速或更少資源密集的區域。提供具體的優化建議，並解釋這些變更如何增強程式碼的效能。優化後的程式碼應保持與原始程式碼相同的功能，同時展現改進的效率。",
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
  system: "您的任務是分析提供的 Python 程式碼片段，並建議改進以優化其效能。識別程式碼可以變得更高效、更快速或更少資源密集的區域。提供具體的優化建議，並解釋這些變更如何增強程式碼的效能。優化後的程式碼應保持與原始程式碼相同的功能，同時展現改進的效率。",
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
    system="您的任務是分析提供的 Python 程式碼片段，並建議改進以優化其效能。識別程式碼可以變得更高效、更快速或更少資源密集的區域。提供具體的優化建議，並解釋這些變更如何增強程式碼的效能。優化後的程式碼應保持與原始程式碼相同的功能，同時展現改進的效率。",
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
  system: "您的任務是分析提供的 Python 程式碼片段，並建議改進以優化其效能。識別程式碼可以變得更高效、更快速或更少資源密集的區域。提供具體的優化建議，並解釋這些變更如何增強程式碼的效能。優化後的程式碼應保持與原始程式碼相同的功能，同時展現改進的效率。",
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
    system="您的任務是分析提供的 Python 程式碼片段，並建議改進以優化其效能。識別程式碼可以變得更高效、更快速或更少資源密集的區域。提供具體的優化建議，並解釋這些變更如何增強程式碼的效能。優化後的程式碼應保持與原始程式碼相同的功能，同時展現改進的效率。",
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
system: "您的任務是分析提供的 Python 程式碼片段，並建議改進以優化其效能。識別程式碼可以變得更高效、更快速或更少資源密集的區域。提供具體的優化建議，並解釋這些變更如何增強程式碼的效能。優化後的程式碼應保持與原始程式碼相同的功能，同時展現改進的效率。",
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