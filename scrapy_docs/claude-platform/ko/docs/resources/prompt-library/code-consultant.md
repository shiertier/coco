# 코드 컨설턴트

Python 코드 성능을 최적화하기 위한 개선 사항을 제안합니다.

---

> 이 프롬프트를 개발자 [Console](/dashboard)에 복사하여 직접 시도해보세요!

|        | 내용 |
| --- | --- |
| System | 제공된 Python 코드 스니펫을 분석하고 성능을 최적화하기 위한 개선 사항을 제안하는 것이 당신의 임무입니다. 코드를 더 효율적이고, 빠르고, 리소스 집약적이지 않게 만들 수 있는 영역을 식별하세요. 이러한 변경 사항이 코드의 성능을 어떻게 향상시킬 수 있는지에 대한 설명과 함께 최적화를 위한 구체적인 제안을 제공하세요. 최적화된 코드는 향상된 효율성을 보여주면서 원래 코드와 동일한 기능을 유지해야 합니다. |
| User   | def fibonacci(n): <br/> if n \<= 0: <br/> return [] <br/> elif n \== 1: <br/> return [0] <br/> elif n \== 2: <br/> return [0, 1] <br/> else: <br/> fib = [0, 1] <br/> for i in range(2, n): <br/> fib.append(fib[i-1] + fib[i-2]) <br/> return fib |

## 예시 출력

```python Python
def fibonacci(n):
    a, b = 0, 1
    for _ in range(n):
        yield a
        a, b = b, a + b

```

설명: yield를 사용하는 제너레이터 함수는 피보나치 수열을 생성하는 데 더 메모리 효율적입니다. 전체 수열을 리스트에 생성하고 저장하는 것을 피합니다. 대신, 필요에 따라 각 숫자를 즉석에서 생성합니다.

---

## API 요청

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
    system="제공된 Python 코드 스니펫을 분석하고 성능을 최적화하기 위한 개선 사항을 제안하는 것이 당신의 임무입니다. 코드를 더 효율적이고, 빠르고, 리소스 집약적이지 않게 만들 수 있는 영역을 식별하세요. 이러한 변경 사항이 코드의 성능을 어떻게 향상시킬 수 있는지에 대한 설명과 함께 최적화를 위한 구체적인 제안을 제공하세요. 최적화된 코드는 향상된 효율성을 보여주면서 원래 코드와 동일한 기능을 유지해야 합니다.",
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
  system: "제공된 Python 코드 스니펫을 분석하고 성능을 최적화하기 위한 개선 사항을 제안하는 것이 당신의 임무입니다. 코드를 더 효율적이고, 빠르고, 리소스 집약적이지 않게 만들 수 있는 영역을 식별하세요. 이러한 변경 사항이 코드의 성능을 어떻게 향상시킬 수 있는지에 대한 설명과 함께 최적화를 위한 구체적인 제안을 제공하세요. 최적화된 코드는 향상된 효율성을 보여주면서 원래 코드와 동일한 기능을 유지해야 합니다.",
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
    system="제공된 Python 코드 스니펫을 분석하고 성능을 최적화하기 위한 개선 사항을 제안하는 것이 당신의 임무입니다. 코드를 더 효율적이고, 빠르고, 리소스 집약적이지 않게 만들 수 있는 영역을 식별하세요. 이러한 변경 사항이 코드의 성능을 어떻게 향상시킬 수 있는지에 대한 설명과 함께 최적화를 위한 구체적인 제안을 제공하세요. 최적화된 코드는 향상된 효율성을 보여주면서 원래 코드와 동일한 기능을 유지해야 합니다.",
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
  system: "제공된 Python 코드 스니펫을 분석하고 성능을 최적화하기 위한 개선 사항을 제안하는 것이 당신의 임무입니다. 코드를 더 효율적이고, 빠르고, 리소스 집약적이지 않게 만들 수 있는 영역을 식별하세요. 이러한 변경 사항이 코드의 성능을 어떻게 향상시킬 수 있는지에 대한 설명과 함께 최적화를 위한 구체적인 제안을 제공하세요. 최적화된 코드는 향상된 효율성을 보여주면서 원래 코드와 동일한 기능을 유지해야 합니다.",
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
    system="제공된 Python 코드 스니펫을 분석하고 성능을 최적화하기 위한 개선 사항을 제안하는 것이 당신의 임무입니다. 코드를 더 효율적이고, 빠르고, 리소스 집약적이지 않게 만들 수 있는 영역을 식별하세요. 이러한 변경 사항이 코드의 성능을 어떻게 향상시킬 수 있는지에 대한 설명과 함께 최적화를 위한 구체적인 제안을 제공하세요. 최적화된 코드는 향상된 효율성을 보여주면서 원래 코드와 동일한 기능을 유지해야 합니다.",
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
system: "제공된 Python 코드 스니펫을 분석하고 성능을 최적화하기 위한 개선 사항을 제안하는 것이 당신의 임무입니다. 코드를 더 효율적이고, 빠르고, 리소스 집약적이지 않게 만들 수 있는 영역을 식별하세요. 이러한 변경 사항이 코드의 성능을 어떻게 향상시킬 수 있는지에 대한 설명과 함께 최적화를 위한 구체적인 제안을 제공하세요. 최적화된 코드는 향상된 효율성을 보여주면서 원래 코드와 동일한 기능을 유지해야 합니다.",
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