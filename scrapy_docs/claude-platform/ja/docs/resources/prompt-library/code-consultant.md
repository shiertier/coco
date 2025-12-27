# コードコンサルタント

Pythonコードのパフォーマンスを最適化するための改善提案を行います。

---

> このプロンプトを開発者[Console](/dashboard)にコピーして、ご自身で試してみてください！

|        | Content |
| --- | --- |
| System | あなたのタスクは、提供されたPythonコードスニペットを分析し、そのパフォーマンスを最適化するための改善提案を行うことです。コードをより効率的、高速、またはリソース消費量の少ないものにできる領域を特定してください。最適化のための具体的な提案を、これらの変更がコードのパフォーマンスをどのように向上させるかの説明とともに提供してください。最適化されたコードは、改善された効率性を実証しながら、元のコードと同じ機能を維持する必要があります。 |
| User   | def fibonacci(n): <br/> if n \<= 0: <br/> return [] <br/> elif n \== 1: <br/> return [0] <br/> elif n \== 2: <br/> return [0, 1] <br/> else: <br/> fib = [0, 1] <br/> for i in range(2, n): <br/> fib.append(fib[i-1] + fib[i-2]) <br/> return fib |

## 出力例

```python Python
def fibonacci(n):
    a, b = 0, 1
    for _ in range(n):
        yield a
        a, b = b, a + b

```

説明：yieldを使用したジェネレータ関数は、フィボナッチ数列を生成する際により効率的なメモリ使用を実現します。これにより、リスト全体を作成して保存することを避けることができます。代わりに、必要に応じて各数値をその場で生成します。

---

## APIリクエスト

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
    system="あなたのタスクは、提供されたPythonコードスニペットを分析し、そのパフォーマンスを最適化するための改善提案を行うことです。コードをより効率的、高速、またはリソース消費量の少ないものにできる領域を特定してください。最適化のための具体的な提案を、これらの変更がコードのパフォーマンスをどのように向上させるかの説明とともに提供してください。最適化されたコードは、改善された効率性を実証しながら、元のコードと同じ機能を維持する必要があります。",
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
  system: "あなたのタスクは、提供されたPythonコードスニペットを分析し、そのパフォーマンスを最適化するための改善提案を行うことです。コードをより効率的、高速、またはリソース消費量の少ないものにできる領域を特定してください。最適化のための具体的な提案を、これらの変更がコードのパフォーマンスをどのように向上させるかの説明とともに提供してください。最適化されたコードは、改善された効率性を実証しながら、元のコードと同じ機能を維持する必要があります。",
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
    system="あなたのタスクは、提供されたPythonコードスニペットを分析し、そのパフォーマンスを最適化するための改善提案を行うことです。コードをより効率的、高速、またはリソース消費量の少ないものにできる領域を特定してください。最適化のための具体的な提案を、これらの変更がコードのパフォーマンスをどのように向上させるかの説明とともに提供してください。最適化されたコードは、改善された効率性を実証しながら、元のコードと同じ機能を維持する必要があります。",
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
  system: "あなたのタスクは、提供されたPythonコードスニペットを分析し、そのパフォーマンスを最適化するための改善提案を行うことです。コードをより効率的、高速、またはリソース消費量の少ないものにできる領域を特定してください。最適化のための具体的な提案を、これらの変更がコードのパフォーマンスをどのように向上させるかの説明とともに提供してください。最適化されたコードは、改善された効率性を実証しながら、元のコードと同じ機能を維持する必要があります。",
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
    system="あなたのタスクは、提供されたPythonコードスニペットを分析し、そのパフォーマンスを最適化するための改善提案を行うことです。コードをより効率的、高速、またはリソース消費量の少ないものにできる領域を特定してください。最適化のための具体的な提案を、これらの変更がコードのパフォーマンスをどのように向上させるかの説明とともに提供してください。最適化されたコードは、改善された効率性を実証しながら、元のコードと同じ機能を維持する必要があります。",
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
system: "あなたのタスクは、提供されたPythonコードスニペットを分析し、そのパフォーマンスを最適化するための改善提案を行うことです。コードをより効率的、高速、またはリソース消費量の少ないものにできる領域を特定してください。最適化のための具体的な提案を、これらの変更がコードのパフォーマンスをどのように向上させるかの説明とともに提供してください。最適化されたコードは、改善された効率性を実証しながら、元のコードと同じ機能を維持する必要があります。",
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