# 情緒色彩化器

將情緒的文字描述轉換為對應的十六進制色碼。

---

> 將此提示複製到我們的開發者[控制台](/dashboard)中親自試用！

|        | 內容 |
| --- | --- |
| System | 您的任務是接收提供的情緒或情感文字描述，並生成一個視覺上代表該情緒的十六進制色碼。使用色彩心理學原理和常見關聯來確定給定情緒最合適的顏色。如果文字描述不清楚、模糊或沒有提供足夠的資訊來確定合適的顏色，請回應「無法為給定情緒確定十六進制色碼。」 |
| User   | 一種充滿激情、強烈且火熱的情感，充滿愛意和渴望。 |

## 範例輸出

`# FF1744`

這個十六進制色碼代表一種鮮豔、大膽的紅色色調。紅色通常與激情、強烈、愛情和渴望相關聯。這裡選擇的特定色調是一種火熱、充滿活力的紅色，捕捉了所描述情感的熱度和熱情。紅色以刺激興奮和激情的感覺而聞名，使其成為代表這種情緒的合適選擇。

---

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
  max_tokens=500,
  temperature=0.5,
  system="您的任務是接收提供的情緒或情感文字描述，並生成一個視覺上代表該情緒的十六進制色碼。使用色彩心理學原理和常見關聯來確定給定情緒最合適的顏色。如果文字描述不清楚、模糊或沒有提供足夠的資訊來確定合適的顏色，請回應「無法為給定情緒確定十六進制色碼。」",
  messages=[
    {
    "role": "user",
    "content": [
        {
          "type": "text",
          "text": "一種充滿激情、強烈且火熱的情感，充滿愛意和渴望。"
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
  max_tokens: 500,
  temperature: 0.5,
  system: "您的任務是接收提供的情緒或情感文字描述，並生成一個視覺上代表該情緒的十六進制色碼。使用色彩心理學原理和常見關聯來確定給定情緒最合適的顏色。如果文字描述不清楚、模糊或沒有提供足夠的資訊來確定合適的顏色，請回應「無法為給定情緒確定十六進制色碼。」",
  messages: [
    {
      "role": "user",
      "content": [
        {
          "type": "text",
          "text": "一種充滿激情、強烈且火熱的情感，充滿愛意和渴望。"
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
    max_tokens=500,
    temperature=0.5,
    system="您的任務是接收提供的情緒或情感文字描述，並生成一個視覺上代表該情緒的十六進制色碼。使用色彩心理學原理和常見關聯來確定給定情緒最合適的顏色。如果文字描述不清楚、模糊或沒有提供足夠的資訊來確定合適的顏色，請回應「無法為給定情緒確定十六進制色碼。」",
    messages=[
        {
            "role": "user",
            "content": [
                {
                    "type": "text",
                    "text": "一種充滿激情、強烈且火熱的情感，充滿愛意和渴望。"
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
  max_tokens: 500,
  temperature: 0.5,
  system: "您的任務是接收提供的情緒或情感文字描述，並生成一個視覺上代表該情緒的十六進制色碼。使用色彩心理學原理和常見關聯來確定給定情緒最合適的顏色。如果文字描述不清楚、模糊或沒有提供足夠的資訊來確定合適的顏色，請回應「無法為給定情緒確定十六進制色碼。」",
  messages: [
    {
      "role": "user",
      "content": [
        {
          "type": "text",
          "text": "一種充滿激情、強烈且火熱的情感，充滿愛意和渴望。"
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
    max_tokens=500,
    temperature=0.5,
    system="您的任務是接收提供的情緒或情感文字描述，並生成一個視覺上代表該情緒的十六進制色碼。使用色彩心理學原理和常見關聯來確定給定情緒最合適的顏色。如果文字描述不清楚、模糊或沒有提供足夠的資訊來確定合適的顏色，請回應「無法為給定情緒確定十六進制色碼。」",
    messages=[
        {
            "role": "user",
            "content": [
                {
                    "type": "text",
                    "text": "一種充滿激情、強烈且火熱的情感，充滿愛意和渴望。"
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
  max_tokens: 500,
  temperature: 0.5,
  system: "您的任務是接收提供的情緒或情感文字描述，並生成一個視覺上代表該情緒的十六進制色碼。使用色彩心理學原理和常見關聯來確定給定情緒最合適的顏色。如果文字描述不清楚、模糊或沒有提供足夠的資訊來確定合適的顏色，請回應「無法為給定情緒確定十六進制色碼。」",
  messages: [
    {
      "role": "user",
      "content": [
        {
          "type": "text",
          "text": "一種充滿激情、強烈且火熱的情感，充滿愛意和渴望。"
        }
      ]
    }
  ]
});
console.log(msg);

```

</Tab>
</Tabs>