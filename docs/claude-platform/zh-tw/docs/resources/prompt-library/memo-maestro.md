# 備忘錄大師

根據關鍵要點撰寫全面的公司備忘錄。

---

> 將此提示複製到我們的開發者[控制台](/dashboard)中親自試用！

|        | 內容 |
| --- | --- |
| System | 您的任務是根據提供的關鍵要點撰寫一份全面的公司備忘錄。備忘錄應以專業語調撰寫，以清晰簡潔的方式涵蓋所有相關資訊。使用適當的格式，如標題、副標題和項目符號，有效地組織內容。確保備忘錄結構良好、連貫且易於目標受眾理解。 |
| User   | - 宣布新的員工健康計劃：「Fit4Success」 <br/> - 計劃目標：促進身心健康、減少壓力並提高生產力 <br/> - 組成部分：現場健身課程、健康餐飲選擇、心理健康資源和健康研習會 <br/> - 與當地健身房和健康中心合作提供會員折扣 <br/> - 參與獎勵：健身追蹤器、健康積分和抽獎活動 <br/> - 計劃啟動日期：2023年6月1日 <br/> - 註冊流程：透過公司內網線上註冊 <br/> - 計劃協調員：人力資源和員工參與團隊 |

## 範例輸出

主旨：介紹Fit4Success - 我們的新員工健康計劃

大家好，

我很高興宣布推出我們的新員工健康計劃Fit4Success，該計劃旨在促進團隊成員的身心健康、減少壓力並提高生產力。該計劃將於2023年6月1日正式開始。

計劃目標：

- 鼓勵定期體育活動和健康的生活方式選擇
- 為心理健康和壓力管理提供資源和支持
- 營造積極且引人入勝的工作環境，優先考慮員工福祉
- 提升整體生產力和工作滿意度

計劃組成部分：

- 現場健身課程：瑜伽、普拉提和高強度間歇訓練（HIIT）課程
- 健康餐飲選擇：公司餐廳提供營養零食和餐點
- 心理健康資源：獲得諮詢服務、冥想應用程式和壓力緩解研習會
- 健康研習會：關於營養、睡眠衛生和工作生活平衡等主題的研討會

合作夥伴關係和折扣：  
 為了進一步支持您的健康之旅，我們與當地健身房和健康中心合作，專門為我們的員工提供會員折扣。利用這些特殊優惠來使用各種健身設施和課程。

參與獎勵：  
 為了鼓勵積極參與Fit4Success計劃，我們將提供以下獎勵：

- 為所有參與者提供健身追蹤器，以監控他們的進度並設定個人目標
- 透過計劃參與獲得健康積分，可兌換額外獎勵
- 為達到特定健康里程碑的參與者舉辦每月抽獎活動

註冊流程：  
 要註冊Fit4Success計劃，請造訪我們的公司內網並完成線上註冊表格。註冊截止日期為2023年5月25日。如果您對註冊流程有任何疑問或顧慮，請聯繫我們的人力資源團隊。

計劃協調：  
 Fit4Success計劃將由我們的人力資源和員工參與團隊共同協調。他們將在您的健康之旅中提供指導、支持和資源。

我們堅信，投資於員工的福祉對我們組織的成功和成長至關重要。我們鼓勵您利用這個令人興奮的機會，與我們一起創造一個更健康、更快樂、更有生產力的工作場所。

讓我們擁抱Fit4Success計劃，共同努力邁向更健康的未來！

此致敬禮，  
 \[您的姓名\]  
 \[您的職稱\]

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
    temperature=1,
    system="您的任務是根據提供的關鍵要點撰寫一份全面的公司備忘錄。備忘錄應以專業語調撰寫，以清晰簡潔的方式涵蓋所有相關資訊。使用適當的格式，如標題、副標題和項目符號，有效地組織內容。確保備忘錄結構良好、連貫且易於目標受眾理解。",
    messages=[
        {
            "role": "user",
            "content": [
                {
                    "type": "text",
                    "text": "- 宣布新的員工健康計劃：\"Fit4Success\"  \n- 計劃目標：促進身心健康、減少壓力並提高生產力  \n- 組成部分：現場健身課程、健康餐飲選擇、心理健康資源和健康研習會  \n- 與當地健身房和健康中心合作提供會員折扣  \n- 參與獎勵：健身追蹤器、健康積分和抽獎活動  \n- 計劃啟動日期：2023年6月1日  \n- 註冊流程：透過公司內網線上註冊  \n- 計劃協調員：人力資源和員工參與團隊"
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
  temperature: 1,
  system: "您的任務是根據提供的關鍵要點撰寫一份全面的公司備忘錄。備忘錄應以專業語調撰寫，以清晰簡潔的方式涵蓋所有相關資訊。使用適當的格式，如標題、副標題和項目符號，有效地組織內容。確保備忘錄結構良好、連貫且易於目標受眾理解。",
  messages: [
    {
      "role": "user",
      "content": [
        {
          "type": "text",
          "text": "- 宣布新的員工健康計劃：\"Fit4Success\"  \n- 計劃目標：促進身心健康、減少壓力並提高生產力  \n- 組成部分：現場健身課程、健康餐飲選擇、心理健康資源和健康研習會  \n- 與當地健身房和健康中心合作提供會員折扣  \n- 參與獎勵：健身追蹤器、健康積分和抽獎活動  \n- 計劃啟動日期：2023年6月1日  \n- 註冊流程：透過公司內網線上註冊  \n- 計劃協調員：人力資源和員工參與團隊"
        }
      ]
    }
  ]
});
console.log(msg);

```

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
system="您的任務是根據提供的關鍵要點撰寫一份全面的公司備忘錄。備忘錄應以專業語調撰寫，以清晰簡潔的方式涵蓋所有相關資訊。使用適當的格式，如標題、副標題和項目符號，有效地組織內容。確保備忘錄結構良好、連貫且易於目標受眾理解。",
messages=[
{
"role": "user",
"content": [
{
"type": "text",
"text": "- 宣布新的員工健康計劃：\"Fit4Success\" \n- 計劃目標：促進身心健康、減少壓力並提高生產力 \n- 組成部分：現場健身課程、健康餐飲選擇、心理健康資源和健康研習會 \n- 與當地健身房和健康中心合作提供會員折扣 \n- 參與獎勵：健身追蹤器、健康積分和抽獎活動 \n- 計劃啟動日期：2023年6月1日 \n- 註冊流程：透過公司內網線上註冊 \n- 計劃協調員：人力資源和員工參與團隊"
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
  temperature: 1,
  system: "您的任務是根據提供的關鍵要點撰寫一份全面的公司備忘錄。備忘錄應以專業語調撰寫，以清晰簡潔的方式涵蓋所有相關資訊。使用適當的格式，如標題、副標題和項目符號，有效地組織內容。確保備忘錄結構良好、連貫且易於目標受眾理解。",
  messages: [
    {
      "role": "user",
      "content": [
        {
          "type": "text",
          "text": "- 宣布新的員工健康計劃：\"Fit4Success\"  \n- 計劃目標：促進身心健康、減少壓力並提高生產力  \n- 組成部分：現場健身課程、健康餐飲選擇、心理健康資源和健康研習會  \n- 與當地健身房和健康中心合作提供會員折扣  \n- 參與獎勵：健身追蹤器、健康積分和抽獎活動  \n- 計劃啟動日期：2023年6月1日  \n- 註冊流程：透過公司內網線上註冊  \n- 計劃協調員：人力資源和員工參與團隊"
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
    temperature=1,
    system="您的任務是根據提供的關鍵要點撰寫一份全面的公司備忘錄。備忘錄應以專業語調撰寫，以清晰簡潔的方式涵蓋所有相關資訊。使用適當的格式，如標題、副標題和項目符號，有效地組織內容。確保備忘錄結構良好、連貫且易於目標受眾理解。",
    messages=[
        {
            "role": "user",
            "content": [
                {
                    "type": "text",
                    "text": "- 宣布新的員工健康計劃：\"Fit4Success\"  \n- 計劃目標：促進身心健康、減少壓力並提高生產力  \n- 組成部分：現場健身課程、健康餐飲選擇、心理健康資源和健康研習會  \n- 與當地健身房和健康中心合作提供會員折扣  \n- 參與獎勵：健身追蹤器、健康積分和抽獎活動  \n- 計劃啟動日期：2023年6月1日  \n- 註冊流程：透過公司內網線上註冊  \n- 計劃協調員：人力資源和員工參與團隊"
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
  system: "您的任務是根據提供的關鍵要點撰寫一份全面的公司備忘錄。備忘錄應以專業語調撰寫，以清晰簡潔的方式涵蓋所有相關資訊。使用適當的格式，如標題、副標題和項目符號，有效地組織內容。確保備忘錄結構良好、連貫且易於目標受眾理解。",
  messages: [
    {
      "role": "user",
      "content": [
        {
          "type": "text",
          "text": "- 宣布新的員工健康計劃：\"Fit4Success\"  \n- 計劃目標：促進身心健康、減少壓力並提高生產力  \n- 組成部分：現場健身課程、健康餐飲選擇、心理健康資源和健康研習會  \n- 與當地健身房和健康中心合作提供會員折扣  \n- 參與獎勵：健身追蹤器、健康積分和抽獎活動  \n- 計劃啟動日期：2023年6月1日  \n- 註冊流程：透過公司內網線上註冊  \n- 計劃協調員：人力資源和員工參與團隊"
        }
      ]
    }
  ]
});
console.log(msg);

```

</Tab>
</Tabs>