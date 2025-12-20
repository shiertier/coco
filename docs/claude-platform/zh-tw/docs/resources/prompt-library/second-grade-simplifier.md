# 二年級簡化器

讓複雜文本變得容易讓年輕學習者理解。

---

> 將此提示複製到我們的開發者 [Console](/dashboard) 中親自試用！

|        | 內容 |
| --- | --- |
| System | 您的任務是將提供的文本重寫，使其易於3-5年級的年輕學習者閱讀和理解。簡化高級詞彙，分解長句，用簡單的語言解釋困難的概念，並以清晰、引人入勝的方式呈現信息。簡短的重寫文本應以適合年齡的方式傳達原文的核心思想。 |
| User   | 粒線體，通常被稱為細胞的發電廠，是必需的胞器，在通過細胞呼吸過程產生三磷酸腺苷（ATP）方面發揮著關鍵作用。ATP是細胞的主要能量貨幣，使各種細胞功能和生化反應得以發生。 |

## 範例輸出

粒線體是細胞內的微小部分，就像小發電站一樣工作。它們製造一種叫做ATP的特殊分子，這就像細胞的能量貨幣。ATP幫助細胞做重要的工作並保持其順利運行，就像金錢幫助人們購買他們需要的東西一樣。粒線體非常重要，因為沒有它們，細胞就沒有能量正常運作。

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
  max_tokens=1000,
  temperature=0,
  system="您的任務是將提供的文本重寫，使其易於3-5年級的年輕學習者閱讀和理解。簡化高級詞彙，分解長句，用簡單的語言解釋困難的概念，並以清晰、引人入勝的方式呈現信息。簡短的重寫文本應以適合年齡的方式傳達原文的核心思想。",
  messages=[
    {
      "role": "user",
      "content": [
        {
          "type": "text",
          "text": "粒線體，通常被稱為細胞的發電廠，是必需的胞器，在通過細胞呼吸過程產生三磷酸腺苷（ATP）方面發揮著關鍵作用。ATP是細胞的主要能量貨幣，使各種細胞功能和生化反應得以發生。"
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
  system: "您的任務是將提供的文本重寫，使其易於3-5年級的年輕學習者閱讀和理解。簡化高級詞彙，分解長句，用簡單的語言解釋困難的概念，並以清晰、引人入勝的方式呈現信息。簡短的重寫文本應以適合年齡的方式傳達原文的核心思想。",
  messages: [
    {
      "role": "user",
      "content": [
        {
          "type": "text",
          "text": "粒線體，通常被稱為細胞的發電廠，是必需的胞器，在通過細胞呼吸過程產生三磷酸腺苷（ATP）方面發揮著關鍵作用。ATP是細胞的主要能量貨幣，使各種細胞功能和生化反應得以發生。"
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
    system="您的任務是將提供的文本重寫，使其易於3-5年級的年輕學習者閱讀和理解。簡化高級詞彙，分解長句，用簡單的語言解釋困難的概念，並以清晰、引人入勝的方式呈現信息。簡短的重寫文本應以適合年齡的方式傳達原文的核心思想。",
    messages=[
        {
            "role": "user",
            "content": [
                {
                    "type": "text",
                    "text": "粒線體，通常被稱為細胞的發電廠，是必需的胞器，在通過細胞呼吸過程產生三磷酸腺苷（ATP）方面發揮著關鍵作用。ATP是細胞的主要能量貨幣，使各種細胞功能和生化反應得以發生。"
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
  system: "您的任務是將提供的文本重寫，使其易於3-5年級的年輕學習者閱讀和理解。簡化高級詞彙，分解長句，用簡單的語言解釋困難的概念，並以清晰、引人入勝的方式呈現信息。簡短的重寫文本應以適合年齡的方式傳達原文的核心思想。",
  messages: [
    {
      "role": "user",
      "content": [
        {
          "type": "text",
          "text": "粒線體，通常被稱為細胞的發電廠，是必需的胞器，在通過細胞呼吸過程產生三磷酸腺苷（ATP）方面發揮著關鍵作用。ATP是細胞的主要能量貨幣，使各種細胞功能和生化反應得以發生。"
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
system="您的任務是將提供的文本重寫，使其易於3-5年級的年輕學習者閱讀和理解。簡化高級詞彙，分解長句，用簡單的語言解釋困難的概念，並以清晰、引人入勝的方式呈現信息。簡短的重寫文本應以適合年齡的方式傳達原文的核心思想。",
messages=[
{
"role": "user",
"content": [
{
"type": "text",
"text": "粒線體，通常被稱為細胞的發電廠，是必需的胞器，在通過細胞呼吸過程產生三磷酸腺苷（ATP）方面發揮著關鍵作用。ATP是細胞的主要能量貨幣，使各種細胞功能和生化反應得以發生。"
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
  system: "您的任務是將提供的文本重寫，使其易於3-5年級的年輕學習者閱讀和理解。簡化高級詞彙，分解長句，用簡單的語言解釋困難的概念，並以清晰、引人入勝的方式呈現信息。簡短的重寫文本應以適合年齡的方式傳達原文的核心思想。",
  messages: [
    {
      "role": "user",
      "content": [
        {
          "type": "text",
          "text": "粒線體，通常被稱為細胞的發電廠，是必需的胞器，在通過細胞呼吸過程產生三磷酸腺苷（ATP）方面發揮著關鍵作用。ATP是細胞的主要能量貨幣，使各種細胞功能和生化反應得以發生。"
        }
      ]
    }
  ]
});
console.log(msg);

````

</Tab>
</Tabs>