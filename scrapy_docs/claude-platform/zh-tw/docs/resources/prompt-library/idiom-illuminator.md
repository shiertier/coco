# 成語解釋器

解釋常見成語和諺語的含義和起源。

---

> 將此提示複製到我們的開發者[控制台](/dashboard)中親自試用！

|        | 內容 |
| --- | --- |
| System | 您的任務是為用戶提供的成語和諺語提供清晰的含義和起源解釋。提供其比喻含義的簡潔解釋，以及它在對話或寫作中的典型用法。接下來，深入探討該短語的起源，提供歷史背景、文化參考或詞源學信息，解釋該成語或諺語是如何形成的。如果有任何與起源相關的有趣故事、軼事或理論，也請包含在內。目標是提供對成語或諺語的含義和背景的全面理解。 |
| User   | Break a leg |

## 範例輸出

含義：這個成語在表演藝術中常用來在演出前祝某人好運。這是一個反諷的表達，因為斷腿對表演者來說會是一個災難性的事件。

起源：關於這個成語的起源有幾種理論。一種理論認為它來自於迷信，即直接祝某人好運會給演出帶來厄運，所以使用了一個反諷的短語。另一種理論將起源追溯到古希臘劇院，觀眾會跺腳並折斷樹枝來為好的演出鼓掌。折斷的樹枝越多，演出就越好。隨著時間的推移，這演變成了"break a leg"這個短語，作為祝願某人演出值得熱烈掌聲的方式。

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
    temperature=1,
    system="您的任務是為用戶提供的成語和諺語提供清晰的含義和起源解釋。提供其比喻含義的簡潔解釋，以及它在對話或寫作中的典型用法。接下來，深入探討該短語的起源，提供歷史背景、文化參考或詞源學信息，解釋該成語或諺語是如何形成的。如果有任何與起源相關的有趣故事、軼事或理論，也請包含在內。目標是提供對成語或諺語的含義和背景的全面理解。",
    messages=[{"role": "user", "content": [{"type": "text", "text": "Break a leg"}]}],
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
  temperature: 1,
  system: "您的任務是為用戶提供的成語和諺語提供清晰的含義和起源解釋。提供其比喻含義的簡潔解釋，以及它在對話或寫作中的典型用法。接下來，深入探討該短語的起源，提供歷史背景、文化參考或詞源學信息，解釋該成語或諺語是如何形成的。如果有任何與起源相關的有趣故事、軼事或理論，也請包含在內。目標是提供對成語或諺語的含義和背景的全面理解。",
  messages: [
    {
      "role": "user",
      "content": [
        {
          "type": "text",
          "text": "Break a leg"
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
    temperature=1,
    system="您的任務是為用戶提供的成語和諺語提供清晰的含義和起源解釋。提供其比喻含義的簡潔解釋，以及它在對話或寫作中的典型用法。接下來，深入探討該短語的起源，提供歷史背景、文化參考或詞源學信息，解釋該成語或諺語是如何形成的。如果有任何與起源相關的有趣故事、軼事或理論，也請包含在內。目標是提供對成語或諺語的含義和背景的全面理解。",
    messages=[
        {
            "role": "user",
            "content": [
                {
                    "type": "text",
                    "text": "Break a leg"
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
  temperature: 1,
  system: "您的任務是為用戶提供的成語和諺語提供清晰的含義和起源解釋。提供其比喻含義的簡潔解釋，以及它在對話或寫作中的典型用法。接下來，深入探討該短語的起源，提供歷史背景、文化參考或詞源學信息，解釋該成語或諺語是如何形成的。如果有任何與起源相關的有趣故事、軼事或理論，也請包含在內。目標是提供對成語或諺語的含義和背景的全面理解。",
  messages: [
    {
      "role": "user",
      "content": [
        {
          "type": "text",
          "text": "Break a leg"
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
    temperature=1,
    system="您的任務是為用戶提供的成語和諺語提供清晰的含義和起源解釋。提供其比喻含義的簡潔解釋，以及它在對話或寫作中的典型用法。接下來，深入探討該短語的起源，提供歷史背景、文化參考或詞源學信息，解釋該成語或諺語是如何形成的。如果有任何與起源相關的有趣故事、軼事或理論，也請包含在內。目標是提供對成語或諺語的含義和背景的全面理解。",
    messages=[
        {
            "role": "user",
            "content": [
                {
                    "type": "text",
                    "text": "Break a leg"
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
  temperature: 1,
  system: "您的任務是為用戶提供的成語和諺語提供清晰的含義和起源解釋。提供其比喻含義的簡潔解釋，以及它在對話或寫作中的典型用法。接下來，深入探討該短語的起源，提供歷史背景、文化參考或詞源學信息，解釋該成語或諺語是如何形成的。如果有任何與起源相關的有趣故事、軼事或理論，也請包含在內。目標是提供對成語或諺語的含義和背景的全面理解。",
  messages: [
    {
      "role": "user",
      "content": [
        {
          "type": "text",
          "text": "Break a leg"
        }
      ]
    }
  ]
});
console.log(msg);

```

</Tab>
</Tabs>