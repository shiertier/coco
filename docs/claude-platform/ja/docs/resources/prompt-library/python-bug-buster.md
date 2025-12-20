# Pythonバグバスター

Pythonコードのバグを検出して修正します。

---

> このプロンプトを開発者[Console](/dashboard)にコピーして、ご自身で試してみてください！

|        | Content |
| --- | --- |
| System | あなたのタスクは、提供されたPythonコードスニペットを分析し、存在するバグやエラーを特定し、これらの問題を解決する修正版のコードを提供することです。元のコードで見つけた問題と、あなたの修正がそれらをどのように解決するかを説明してください。修正されたコードは機能的で効率的であり、Pythonプログラミングのベストプラクティスに従っている必要があります。 |
| User   | def calculate_average(nums): <br/> sum = 0 <br/> for num in nums: <br/> sum += num <br/> average = sum / len(nums) <br/> return average <br/><br/> numbers = [10, 20, 30, 40, 50] <br/> result = calculate_average(numbers) <br/> print("The average is:", results) |

## 出力例

修正されたPythonコード：

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

説明：

    1. 変数名「sum」を「total」に変更して、Pythonの組み込み関数「sum()」の使用を避けました。
    2. print文のタイポを修正し、「results」を「result」に変更して変数を正しく参照するようにしました。

## APIリクエスト

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
  system="あなたのタスクは、提供されたPythonコードスニペットを分析し、存在するバグやエラーを特定し、これらの問題を解決する修正版のコードを提供することです。元のコードで見つけた問題と、あなたの修正がそれらをどのように解決するかを説明してください。修正されたコードは機能的で効率的であり、Pythonプログラミングのベストプラクティスに従っている必要があります。",
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
  system: "あなたのタスクは、提供されたPythonコードスニペットを分析し、存在するバグやエラーを特定し、これらの問題を解決する修正版のコードを提供することです。元のコードで見つけた問題と、あなたの修正がそれらをどのように解決するかを説明してください。修正されたコードは機能的で効率的であり、Pythonプログラミングのベストプラクティスに従っている必要があります。",
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
system="あなたのタスクは、提供されたPythonコードスニペットを分析し、存在するバグやエラーを特定し、これらの問題を解決する修正版のコードを提供することです。元のコードで見つけた問題と、あなたの修正がそれらをどのように解決するかを説明してください。修正されたコードは機能的で効率的であり、Pythonプログラミングのベストプラクティスに従っている必要があります。",
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
  system: "あなたのタスクは、提供されたPythonコードスニペットを分析し、存在するバグやエラーを特定し、これらの問題を解決する修正版のコードを提供することです。元のコードで見つけた問題と、あなたの修正がそれらをどのように解決するかを説明してください。修正されたコードは機能的で効率的であり、Pythonプログラミングのベストプラクティスに従っている必要があります。",
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
system="あなたのタスクは、提供されたPythonコードスニペットを分析し、存在するバグやエラーを特定し、これらの問題を解決する修正版のコードを提供することです。元のコードで見つけた問題と、あなたの修正がそれらをどのように解決するかを説明してください。修正されたコードは機能的で効率的であり、Pythonプログラミングのベストプラクティスに従っている必要があります。",
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
  system: "あなたのタスクは、提供されたPythonコードスニペットを分析し、存在するバグやエラーを特定し、これらの問題を解決する修正版のコードを提供することです。元のコードで見つけた問題と、あなたの修正がそれらをどのように解決するかを説明してください。修正されたコードは機能的で効率的であり、Pythonプログラミングのベストプラクティスに従っている必要があります。",
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