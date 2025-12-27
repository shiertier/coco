# Python 錯誤修復器

檢測並修復 Python 程式碼中的錯誤。

---

> 將此提示複製到我們的開發者 [Console](/dashboard) 中親自試用！

|        | 內容 |
| --- | --- |
| System | 您的任務是分析提供的 Python 程式碼片段，識別存在的任何錯誤或問題，並提供解決這些問題的修正版本程式碼。解釋您在原始程式碼中發現的問題以及您的修復如何解決這些問題。修正後的程式碼應該是功能性的、高效的，並遵循 Python 程式設計的最佳實踐。 |
| User   | def calculate_average(nums): <br/> sum = 0 <br/> for num in nums: <br/> sum += num <br/> average = sum / len(nums) <br/> return average <br/><br/> numbers = [10, 20, 30, 40, 50] <br/> result = calculate_average(numbers) <br/> print("The average is:", results) |

## 範例輸出

修正後的 Python 程式碼：

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

說明：

    1. 將變數名稱 "sum" 改為 "total"，以避免使用 Python 內建函數 "sum()"。
    2. 修正 print 語句中的拼寫錯誤，將 "results" 改為 "result" 以正確引用變數。

## API 請求

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
  system="您的任務是分析提供的 Python 程式碼片段，識別存在的任何錯誤或問題，並提供解決這些問題的修正版本程式碼。解釋您在原始程式碼中發現的問題以及您的修復如何解決這些問題。修正後的程式碼應該是功能性的、高效的，並遵循 Python 程式設計的最佳實踐。",
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
  system: "您的任務是分析提供的 Python 程式碼片段，識別存在的任何錯誤或問題，並提供解決這些問題的修正版本程式碼。解釋您在原始程式碼中發現的問題以及您的修復如何解決這些問題。修正後的程式碼應該是功能性的、高效的，並遵循 Python 程式設計的最佳實踐。",
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
system="您的任務是分析提供的 Python 程式碼片段，識別存在的任何錯誤或問題，並提供解決這些問題的修正版本程式碼。解釋您在原始程式碼中發現的問題以及您的修復如何解決這些問題。修正後的程式碼應該是功能性的、高效的，並遵循 Python 程式設計的最佳實踐。",
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
  system: "您的任務是分析提供的 Python 程式碼片段，識別存在的任何錯誤或問題，並提供解決這些問題的修正版本程式碼。解釋您在原始程式碼中發現的問題以及您的修復如何解決這些問題。修正後的程式碼應該是功能性的、高效的，並遵循 Python 程式設計的最佳實踐。",
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
system="您的任務是分析提供的 Python 程式碼片段，識別存在的任何錯誤或問題，並提供解決這些問題的修正版本程式碼。解釋您在原始程式碼中發現的問題以及您的修復如何解決這些問題。修正後的程式碼應該是功能性的、高效的，並遵循 Python 程式設計的最佳實踐。",
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
  system: "您的任務是分析提供的 Python 程式碼片段，識別存在的任何錯誤或問題，並提供解決這些問題的修正版本程式碼。解釋您在原始程式碼中發現的問題以及您的修復如何解決這些問題。修正後的程式碼應該是功能性的、高效的，並遵循 Python 程式設計的最佳實踐。",
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