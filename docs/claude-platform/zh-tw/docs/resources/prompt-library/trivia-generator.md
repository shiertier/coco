# 問答題產生器

產生各種主題的問答題，並在需要時提供提示。

---

> 將此提示複製到我們的開發者 [Console](/dashboard) 中親自試用！

| 角色 | 內容 |
| ---- | --- |
| 使用者 | 產生各種主題的問答題，並提供提示幫助使用者找到正確答案。從多樣化的類別中選擇，創建測試使用者知識或推理技能的問題。提供一系列逐漸具體的提示來引導使用者找到解答。確保問題具有挑戰性，提示提供足夠的資訊來幫助使用者，但不會太容易洩露答案。 |

## 範例輸出

類別：地理

問題：  
 世界上唯一同時與法國和西班牙接壤的國家是什麼？

提示 1：這個國家位於法國和西班牙之間的庇里牛斯山脈中。  
 提示 2：它以其壯麗的山景、滑雪度假村和免稅購物而聞名。  
 提示 3：這個國家的首都城市名稱包含了國家本身的名字。

答案：安道爾

類別：科學

問題：  
 植物將陽光、水和二氧化碳轉換為能量和氧氣的過程是什麼？

提示 1：這個過程對地球上的生命至關重要，因為它提供食物和氧氣。  
 提示 2：它主要發生在植物的葉子中。  
 提示 3：這個過程涉及葉綠素，一種吸收陽光的綠色色素。

答案：光合作用

類別：文學

問題：  
 誰寫了於1813年首次出版的經典小說《傲慢與偏見》？

提示 1：作者是一位生活在1775年至1817年的英國女性。  
 提示 2：她以機智、深刻地描繪英國社會和禮儀而聞名。  
 提示 3：她的其他著名作品包括《理智與情感》和《愛瑪》。

答案：珍·奧斯汀

類別：歷史

問題：  
 分隔東西德的柏林圍牆在哪一年開始被拆除？

提示 1：這個事件發生在20世紀後期。  
 提示 2：它標誌著冷戰結束的重要時刻。  
 提示 3：這一年通常與歐洲「共產主義垮台」這個詞彙相關聯。

答案：1989年

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
  max_tokens=2000,
  temperature=0.5,
  messages=[
    {
    "role": "user",
    "content": [
        {
          "type": "text",
          "text": "產生各種主題的問答題，並提供提示幫助使用者找到正確答案。從多樣化的類別中選擇，創建測試使用者知識或推理技能的問題。提供一系列逐漸具體的提示來引導使用者找到解答。確保問題具有挑戰性，提示提供足夠的資訊來幫助使用者，但不會太容易洩露答案。"
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
  max_tokens: 2000,
  temperature: 0.5,
  messages: [
    {
      "role": "user",
      "content": [
        {
          "type": "text",
          "text": "產生各種主題的問答題，並提供提示幫助使用者找到正確答案。從多樣化的類別中選擇，創建測試使用者知識或推理技能的問題。提供一系列逐漸具體的提示來引導使用者找到解答。確保問題具有挑戰性，提示提供足夠的資訊來幫助使用者，但不會太容易洩露答案。"
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
max_tokens=2000,
temperature=0.5,
messages=[
{
"role": "user",
"content": [
{
"type": "text",
"text": "產生各種主題的問答題，並提供提示幫助使用者找到正確答案。從多樣化的類別中選擇，創建測試使用者知識或推理技能的問題。提供一系列逐漸具體的提示來引導使用者找到解答。確保問題具有挑戰性，提示提供足夠的資訊來幫助使用者，但不會太容易洩露答案。"
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
  max_tokens: 2000,
  temperature: 0.5,
  messages: [
    {
      "role": "user",
      "content": [
        {
          "type": "text",
          "text": "產生各種主題的問答題，並提供提示幫助使用者找到正確答案。從多樣化的類別中選擇，創建測試使用者知識或推理技能的問題。提供一系列逐漸具體的提示來引導使用者找到解答。確保問題具有挑戰性，提示提供足夠的資訊來幫助使用者，但不會太容易洩露答案。"
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
    max_tokens=2000,
    temperature=0.5,
    messages=[
        {
            "role": "user",
            "content": [
                {
                    "type": "text",
                    "text": "產生各種主題的問答題，並提供提示幫助使用者找到正確答案。從多樣化的類別中選擇，創建測試使用者知識或推理技能的問題。提供一系列逐漸具體的提示來引導使用者找到解答。確保問題具有挑戰性，提示提供足夠的資訊來幫助使用者，但不會太容易洩露答案。"
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
  max_tokens: 2000,
  temperature: 0.5,
  messages: [
    {
      "role": "user",
      "content": [
        {
          "type": "text",
          "text": "產生各種主題的問答題，並提供提示幫助使用者找到正確答案。從多樣化的類別中選擇，創建測試使用者知識或推理技能的問題。提供一系列逐漸具體的提示來引導使用者找到解答。確保問題具有挑戰性，提示提供足夠的資訊來幫助使用者，但不會太容易洩露答案。"
        }
      ]
    }
  ]
});
console.log(msg);

```

</Tab>
</Tabs>