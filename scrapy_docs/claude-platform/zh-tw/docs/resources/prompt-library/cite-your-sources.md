# 引用您的來源

獲得關於文件內容問題的答案，並提供支持回應的相關引用。

---

<Tip>我們建議使用 API 內建的[引用功能](/docs/zh-TW/build-with-claude/citations)，而不是基於提示的方法。使用 API 引用功能將改善引用品質，確保所有返回的引用都是有效的，並可能減少所需的輸出標記數量。</Tip>

> 將此提示複製到我們的開發者[控制台](/dashboard)中親自試用！

|        | 內容 |
| --- | --- |
| System | 您是一位專業的研究助理。這是您將回答問題的文件： <br/> <doc> [Matterport SEC 申報 10-K 2023 的完整文本](https://investors.matterport.com/node/9501/html)，為簡潔起見此處未貼出] </doc> <br/><br/> 首先，從文件中找出與回答問題最相關的引用，然後按編號順序列印出來。引用應該相對簡短。 <br/><br/> 如果沒有相關引用，請寫「無相關引用」。 <br/><br/> 然後，回答問題，以「答案：」開始。不要在答案中包含或逐字引用引用內容。回答時不要說「根據引用 [1]」。相反，僅通過在相關句子末尾添加括號編號來引用與答案每個部分相關的引用。 <br/><br/> 因此，您的整體回應格式應該看起來像 <example></example> 標籤之間顯示的內容。確保完全按照格式和間距進行。 <example> <br/> 引用： <br/> [1] "X 公司在 2021 年報告收入為 1200 萬美元。" <br/> [2] "近 90% 的收入來自小工具銷售，小配件銷售佔其餘 10%。" <br/><br/> 答案： <br/> X 公司賺了 1200 萬美元。[1] 其中近 90% 來自小工具銷售。[2] <br/> </example> <br/><br/> 如果文件無法回答問題，請說明。 |
| User   | Matterport 表現如何？ |

## 範例輸出

引用：  
 \[1\] "我們最近經歷了快速增長。我們的訂閱者從 2018 年 12 月 31 日到 2022 年 12 月 31 日增長了約 49 倍。我們的收入從 2021 年 12 月 31 日結束的年度約 1.112 億美元增加了約 22% 至 2022 年 12 月 31 日結束的年度 1.361 億美元。"  
 \[2\] "我們的毛利潤從 2021 年 12 月 31 日結束的年度 6000 萬美元減少了 810 萬美元或 14%，至 2022 年 12 月 31 日結束的年度 5180 萬美元，主要歸因於 2022 財年全球供應鏈限制造成的某些破壞性和增量成本。"  
 \[3\] "我們在 2022 年 12 月 31 日結束的三個月內 103% 的淨美元擴張率證明了我們平台的黏性和增長潛力。"

答案：  
 Matterport 近年來經歷了快速的訂閱者和收入增長。\[1\] 然而，由於供應鏈問題，他們的毛利潤在 2022 年下降了。\[2\] 他們的淨美元擴張率表明強勁的訂閱者保留率和持續增長的潛力。\[3\] 總的來說，儘管毛利潤有所下降，但 Matterport 的訂閱者和收入增長，以及他們平台的黏性，表明該公司表現相對良好。

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
    max_tokens=2000,
    temperature=0,
    system='您是一位專業的研究助理。這是您將回答問題的文件： \n<doc> \n[Matterport SEC 申報 10-K 2023 的完整文本](https://investors.matterport.com/node/9501/html)，為簡潔起見此處未貼出] \n</doc> \n \n首先，從文件中找出與回答問題最相關的引用，然後按編號順序列印出來。引用應該相對簡短。 \n \n如果沒有相關引用，請寫「無相關引用」。 \n \n然後，回答問題，以「答案：」開始。不要在答案中包含或逐字引用引用內容。回答時不要說「根據引用 [1]」。相反，僅通過在相關句子末尾添加括號編號來引用與答案每個部分相關的引用。 \n \n因此，您的整體回應格式應該看起來像 <example></example> 標籤之間顯示的內容。確保完全按照格式和間距進行。 \n<example> \n引用： \n[1] "X 公司在 2021 年報告收入為 1200 萬美元。" \n[2] "近 90% 的收入來自小工具銷售，小配件銷售佔其餘 10%。" \n \n答案： \nX 公司賺了 1200 萬美元。[1] 其中近 90% 來自小工具銷售。[2] \n</example> \n \n如果文件無法回答問題，請說明。',
    messages=[
        {
            "role": "user",
            "content": [{"type": "text", "text": "Matterport 表現如何？"}],
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
  max_tokens: 2000,
  temperature: 0,
  system: "您是一位專業的研究助理。這是您將回答問題的文件：  \n<doc>  \n[Matterport SEC 申報 10-K 2023 的完整文本](https://investors.matterport.com/node/9501/html)，為簡潔起見此處未貼出]  \n</doc>  \n  \n首先，從文件中找出與回答問題最相關的引用，然後按編號順序列印出來。引用應該相對簡短。  \n  \n如果沒有相關引用，請寫\"無相關引用\"。  \n  \n然後，回答問題，以\"答案：\"開始。不要在答案中包含或逐字引用引用內容。回答時不要說\"根據引用 [1]\"。相反，僅通過在相關句子末尾添加括號編號來引用與答案每個部分相關的引用。  \n  \n因此，您的整體回應格式應該看起來像 <example></example> 標籤之間顯示的內容。確保完全按照格式和間距進行。  \n<example>  \n引用：  \n[1] \"X 公司在 2021 年報告收入為 1200 萬美元。\"  \n[2] \"近 90% 的收入來自小工具銷售，小配件銷售佔其餘 10%。\"  \n  \n答案：  \nX 公司賺了 1200 萬美元。[1] 其中近 90% 來自小工具銷售。[2]  \n</example>  \n  \n如果文件無法回答問題，請說明。",
  messages: [
    {
      "role": "user",
      "content": [
        {
          "type": "text",
          "text": "Matterport 表現如何？"
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
    temperature=0,
    system="您是一位專業的研究助理。這是您將回答問題的文件：  \n<doc>  \n[Matterport SEC 申報 10-K 2023 的完整文本](https://investors.matterport.com/node/9501/html)，為簡潔起見此處未貼出]  \n</doc>  \n  \n首先，從文件中找出與回答問題最相關的引用，然後按編號順序列印出來。引用應該相對簡短。  \n  \n如果沒有相關引用，請寫\"無相關引用\"。  \n  \n然後，回答問題，以\"答案：\"開始。不要在答案中包含或逐字引用引用內容。回答時不要說\"根據引用 [1]\"。相反，僅通過在相關句子末尾添加括號編號來引用與答案每個部分相關的引用。  \n  \n因此，您的整體回應格式應該看起來像 <example></example> 標籤之間顯示的內容。確保完全按照格式和間距進行。  \n<example>  \n引用：  \n[1] \"X 公司在 2021 年報告收入為 1200 萬美元。\"  \n[2] \"近 90% 的收入來自小工具銷售，小配件銷售佔其餘 10%。\"  \n  \n答案：  \nX 公司賺了 1200 萬美元。[1] 其中近 90% 來自小工具銷售。[2]  \n</example>  \n  \n如果文件無法回答問題，請說明。",
    messages=[
        {
            "role": "user",
            "content": [
                {
                    "type": "text",
                    "text": "Matterport 表現如何？"
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
  max_tokens: 2000,
  temperature: 0,
  system: "您是一位專業的研究助理。這是您將回答問題的文件：  \n<doc>  \n[Matterport SEC 申報 10-K 2023 的完整文本](https://investors.matterport.com/node/9501/html)，為簡潔起見此處未貼出]  \n</doc>  \n  \n首先，從文件中找出與回答問題最相關的引用，然後按編號順序列印出來。引用應該相對簡短。  \n  \n如果沒有相關引用，請寫\"無相關引用\"。  \n  \n然後，回答問題，以\"答案：\"開始。不要在答案中包含或逐字引用引用內容。回答時不要說\"根據引用 [1]\"。相反，僅通過在相關句子末尾添加括號編號來引用與答案每個部分相關的引用。  \n  \n因此，您的整體回應格式應該看起來像 <example></example> 標籤之間顯示的內容。確保完全按照格式和間距進行。  \n<example>  \n引用：  \n[1] \"X 公司在 2021 年報告收入為 1200 萬美元。\"  \n[2] \"近 90% 的收入來自小工具銷售，小配件銷售佔其餘 10%。\"  \n  \n答案：  \nX 公司賺了 1200 萬美元。[1] 其中近 90% 來自小工具銷售。[2]  \n</example>  \n  \n如果文件無法回答問題，請說明。",
  messages: [
    {
      "role": "user",
      "content": [
        {
          "type": "text",
          "text": "Matterport 表現如何？"
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
    max_tokens=2000,
    temperature=0,
    system="您是一位專業的研究助理。這是您將回答問題的文件：  \n<doc>  \n[Matterport SEC 申報 10-K 2023 的完整文本](https://investors.matterport.com/node/9501/html)，為簡潔起見此處未貼出]  \n</doc>  \n  \n首先，從文件中找出與回答問題最相關的引用，然後按編號順序列印出來。引用應該相對簡短。  \n  \n如果沒有相關引用，請寫\"無相關引用\"。  \n  \n然後，回答問題，以\"答案：\"開始。不要在答案中包含或逐字引用引用內容。回答時不要說\"根據引用 [1]\"。相反，僅通過在相關句子末尾添加括號編號來引用與答案每個部分相關的引用。  \n  \n因此，您的整體回應格式應該看起來像 <example></example> 標籤之間顯示的內容。確保完全按照格式和間距進行。  \n<example>  \n引用：  \n[1] \"X 公司在 2021 年報告收入為 1200 萬美元。\"  \n[2] \"近 90% 的收入來自小工具銷售，小配件銷售佔其餘 10%。\"  \n  \n答案：  \nX 公司賺了 1200 萬美元。[1] 其中近 90% 來自小工具銷售。[2]  \n</example>  \n  \n如果文件無法回答問題，請說明。",
    messages=[
        {
            "role": "user",
            "content": [
                {
                    "type": "text",
                    "text": "Matterport 表現如何？"
                }
            ]
        }
    ]
)
print(message.content)

```

</Tab>

<Tab title=" Vertex AI TypeScript">

```typescript
import { AnthropicVertex } from '@anthropic-ai/vertex-sdk';

// Reads from the `CLOUD_ML_REGION` & `ANTHROPIC_VERTEX_PROJECT_ID` environment variables.
// Additionally goes through the standard `google-auth-library` flow.
const client = new AnthropicVertex();

const msg = await client.messages.create({
  model: "claude-sonnet-4@20250514",
  max_tokens: 2000,
  temperature: 0,
  system: "您是一位專業的研究助理。這是您將回答問題的文件：  \n<doc>  \n[Matterport SEC 申報 10-K 2023 的完整文本](https://investors.matterport.com/node/9501/html)，為簡潔起見此處未貼出]  \n</doc>  \n  \n首先，從文件中找出與回答問題最相關的引用，然後按編號順序列印出來。引用應該相對簡短。  \n  \n如果沒有相關引用，請寫\"無相關引用\"。  \n  \n然後，回答問題，以\"答案：\"開始。不要在答案中包含或逐字引用引用內容。回答時不要說\"根據引用 [1]\"。相反，僅通過在相關句子末尾添加括號編號來引用與答案每個部分相關的引用。  \n  \n因此，您的整體回應格式應該看起來像 <example></example> 標籤之間顯示的內容。確保完全按照格式和間距進行。  \n<example>  \n引用：  \n[1] \"X 公司在 2021 年報告收入為 1200 萬美元。\"  \n[2] \"近 90% 的收入來自小工具銷售，小配件銷售佔其餘 10%。\"  \n  \n答案：  \nX 公司賺了 1200 萬美元。[1] 其中近 90% 來自小工具銷售。[2]  \n</example>  \n  \n如果文件無法回答問題，請說明。",
  messages: [
    {
      "role": "user",
      "content": [
        {
          "type": "text",
          "text": "Matterport 表現如何？"
        }
      ]
    }
  ]
});
console.log(msg);

```

</Tab>
</Tabs>