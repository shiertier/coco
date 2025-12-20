# 面試問題製作器

為面試生成問題。

---

> 將此提示複製到我們的開發者[控制台](/dashboard)中親自試用！

|        | 內容 |
| --- | --- |
| System | 您的任務是根據給定的背景為面試生成一系列深思熟慮的開放式問題。這些問題應該設計來引出受訪者有見地和詳細的回應，讓他們能夠展示自己的知識、經驗和批判性思維技能。避免是/否問題或那些有明顯答案的問題。相反，專注於鼓勵反思、自我評估和分享具體例子或軼事的問題。 |
| User   | 您正在為一家成熟的電子商務公司的行銷經理職位面試候選人。理想的候選人應該具有開發和執行多渠道行銷活動的經驗、強大的分析技能，以及與跨職能團隊有效協作的能力。他們還應該對保持最新行銷趨勢和技術的了解充滿熱情。 |

## 範例輸出

1.  您能帶我了解一下您在之前的職位中開發和執行的一個成功的多渠道行銷活動嗎？您面臨了哪些挑戰，以及您是如何克服它們的？
2.  您如何衡量行銷活動的成功，以及您專注於哪些指標？您能提供一個您如何使用數據和分析來優化活動表現的例子嗎？
3.  根據您的經驗，成功的電子商務行銷策略的關鍵要素是什麼？您如何確保您的策略與公司的整體業務目標保持一致？
4.  您能分享一個您必須與跨職能團隊（例如產品、銷售或客戶服務）協作以實現行銷目標的例子嗎？您遇到了哪些挑戰，以及您是如何促進有效協作的？
5.  您如何了解最新的行銷趨勢和技術，以及您如何應用這些知識來改善您的行銷策略或活動？
6.  您能描述一次您參與的行銷活動沒有達到預期表現的情況嗎？您從這次經驗中學到了什麼，以及您如何將這些教訓應用到未來的活動中？
7.  我們公司重視創新和持續改進。您能分享一個您引入新的行銷方法、工具或技術，對您團隊的表現或結果產生重大影響的例子嗎？

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
  temperature=0.5,
  system="您的任務是根據給定的背景為面試生成一系列深思熟慮的開放式問題。這些問題應該設計來引出受訪者有見地和詳細的回應，讓他們能夠展示自己的知識、經驗和批判性思維技能。避免是/否問題或那些有明顯答案的問題。相反，專注於鼓勵反思、自我評估和分享具體例子或軼事的問題。",
  messages=[
    {
      "role": "user",
      "content": [
        {
          "type": "text",
          "text": "您正在為一家成熟的電子商務公司的行銷經理職位面試候選人。理想的候選人應該具有開發和執行多渠道行銷活動的經驗、強大的分析技能，以及與跨職能團隊有效協作的能力。他們還應該對保持最新行銷趨勢和技術的了解充滿熱情。"
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
  temperature: 0.5,
  system: "您的任務是根據給定的背景為面試生成一系列深思熟慮的開放式問題。這些問題應該設計來引出受訪者有見地和詳細的回應，讓他們能夠展示自己的知識、經驗和批判性思維技能。避免是/否問題或那些有明顯答案的問題。相反，專注於鼓勵反思、自我評估和分享具體例子或軼事的問題。",
  messages: [
    {
      "role": "user",
      "content": [
        {
          "type": "text",
          "text": "您正在為一家成熟的電子商務公司的行銷經理職位面試候選人。理想的候選人應該具有開發和執行多渠道行銷活動的經驗、強大的分析技能，以及與跨職能團隊有效協作的能力。他們還應該對保持最新行銷趨勢和技術的了解充滿熱情。"
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
    temperature=0.5,
    system="您的任務是根據給定的背景為面試生成一系列深思熟慮的開放式問題。這些問題應該設計來引出受訪者有見地和詳細的回應，讓他們能夠展示自己的知識、經驗和批判性思維技能。避免是/否問題或那些有明顯答案的問題。相反，專注於鼓勵反思、自我評估和分享具體例子或軼事的問題。",
    messages=[
        {
            "role": "user",
            "content": [
                {
                    "type": "text",
                    "text": "您正在為一家成熟的電子商務公司的行銷經理職位面試候選人。理想的候選人應該具有開發和執行多渠道行銷活動的經驗、強大的分析技能，以及與跨職能團隊有效協作的能力。他們還應該對保持最新行銷趨勢和技術的了解充滿熱情。"
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
temperature: 0.5,
system: "您的任務是根據給定的背景為面試生成一系列深思熟慮的開放式問題。這些問題應該設計來引出受訪者有見地和詳細的回應，讓他們能夠展示自己的知識、經驗和批判性思維技能。避免是/否問題或那些有明顯答案的問題。相反，專注於鼓勵反思、自我評估和分享具體例子或軼事的問題。",
messages: [
{
"role": "user",
"content": [
{
"type": "text",
"text": "您正在為一家成熟的電子商務公司的行銷經理職位面試候選人。理想的候選人應該具有開發和執行多渠道行銷活動的經驗、強大的分析技能，以及與跨職能團隊有效協作的能力。他們還應該對保持最新行銷趨勢和技術的了解充滿熱情。"
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
    temperature=0.5,
    system="您的任務是根據給定的背景為面試生成一系列深思熟慮的開放式問題。這些問題應該設計來引出受訪者有見地和詳細的回應，讓他們能夠展示自己的知識、經驗和批判性思維技能。避免是/否問題或那些有明顯答案的問題。相反，專注於鼓勵反思、自我評估和分享具體例子或軼事的問題。",
    messages=[
        {
            "role": "user",
            "content": [
                {
                    "type": "text",
                    "text": "您正在為一家成熟的電子商務公司的行銷經理職位面試候選人。理想的候選人應該具有開發和執行多渠道行銷活動的經驗、強大的分析技能，以及與跨職能團隊有效協作的能力。他們還應該對保持最新行銷趨勢和技術的了解充滿熱情。"
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
  temperature: 0.5,
  system: "您的任務是根據給定的背景為面試生成一系列深思熟慮的開放式問題。這些問題應該設計來引出受訪者有見地和詳細的回應，讓他們能夠展示自己的知識、經驗和批判性思維技能。避免是/否問題或那些有明顯答案的問題。相反，專注於鼓勵反思、自我評估和分享具體例子或軼事的問題。",
  messages: [
    {
      "role": "user",
      "content": [
        {
          "type": "text",
          "text": "您正在為一家成熟的電子商務公司的行銷經理職位面試候選人。理想的候選人應該具有開發和執行多渠道行銷活動的經驗、強大的分析技能，以及與跨職能團隊有效協作的能力。他們還應該對保持最新行銷趨勢和技術的了解充滿熱情。"
        }
      ]
    }
  ]
});
console.log(msg);

```

</Tab>
</Tabs>