# Python 버그 해결사

Python 코드의 버그를 감지하고 수정합니다.

---

> 이 프롬프트를 개발자 [Console](/dashboard)에 복사하여 직접 시도해보세요!

|        | 내용 |
| --- | --- |
| System | 제공된 Python 코드 스니펫을 분석하고, 존재하는 버그나 오류를 식별하며, 이러한 문제를 해결하는 수정된 버전의 코드를 제공하는 것이 당신의 임무입니다. 원본 코드에서 발견한 문제점과 수정 사항이 이를 어떻게 해결하는지 설명하세요. 수정된 코드는 기능적이고 효율적이며 Python 프로그래밍의 모범 사례를 준수해야 합니다. |
| User   | def calculate_average(nums): <br/> sum = 0 <br/> for num in nums: <br/> sum += num <br/> average = sum / len(nums) <br/> return average <br/><br/> numbers = [10, 20, 30, 40, 50] <br/> result = calculate_average(numbers) <br/> print("The average is:", results) |

## 예시 출력

수정된 Python 코드:

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

설명:

    1. 내장 Python 함수 "sum()"을 사용하지 않도록 변수명 "sum"을 "total"로 변경했습니다.
    2. print 문의 오타를 수정하여 "results"를 "result"로 변경하여 변수를 올바르게 참조하도록 했습니다.

## API 요청

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
  system="제공된 Python 코드 스니펫을 분석하고, 존재하는 버그나 오류를 식별하며, 이러한 문제를 해결하는 수정된 버전의 코드를 제공하는 것이 당신의 임무입니다. 원본 코드에서 발견한 문제점과 수정 사항이 이를 어떻게 해결하는지 설명하세요. 수정된 코드는 기능적이고 효율적이며 Python 프로그래밍의 모범 사례를 준수해야 합니다.",
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
  system: "제공된 Python 코드 스니펫을 분석하고, 존재하는 버그나 오류를 식별하며, 이러한 문제를 해결하는 수정된 버전의 코드를 제공하는 것이 당신의 임무입니다. 원본 코드에서 발견한 문제점과 수정 사항이 이를 어떻게 해결하는지 설명하세요. 수정된 코드는 기능적이고 효율적이며 Python 프로그래밍의 모범 사례를 준수해야 합니다.",
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
system="제공된 Python 코드 스니펫을 분석하고, 존재하는 버그나 오류를 식별하며, 이러한 문제를 해결하는 수정된 버전의 코드를 제공하는 것이 당신의 임무입니다. 원본 코드에서 발견한 문제점과 수정 사항이 이를 어떻게 해결하는지 설명하세요. 수정된 코드는 기능적이고 효율적이며 Python 프로그래밍의 모범 사례를 준수해야 합니다.",
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
  system: "제공된 Python 코드 스니펫을 분석하고, 존재하는 버그나 오류를 식별하며, 이러한 문제를 해결하는 수정된 버전의 코드를 제공하는 것이 당신의 임무입니다. 원본 코드에서 발견한 문제점과 수정 사항이 이를 어떻게 해결하는지 설명하세요. 수정된 코드는 기능적이고 효율적이며 Python 프로그래밍의 모범 사례를 준수해야 합니다.",
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
system="제공된 Python 코드 스니펫을 분석하고, 존재하는 버그나 오류를 식별하며, 이러한 문제를 해결하는 수정된 버전의 코드를 제공하는 것이 당신의 임무입니다. 원본 코드에서 발견한 문제점과 수정 사항이 이를 어떻게 해결하는지 설명하세요. 수정된 코드는 기능적이고 효율적이며 Python 프로그래밍의 모범 사례를 준수해야 합니다.",
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
  system: "제공된 Python 코드 스니펫을 분석하고, 존재하는 버그나 오류를 식별하며, 이러한 문제를 해결하는 수정된 버전의 코드를 제공하는 것이 당신의 임무입니다. 원본 코드에서 발견한 문제점과 수정 사항이 이를 어떻게 해결하는지 설명하세요. 수정된 코드는 기능적이고 효율적이며 Python 프로그래밍의 모범 사례를 준수해야 합니다.",
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