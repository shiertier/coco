# 企業先知

從冗長的企業報告中提取洞察、識別風險，並將關鍵資訊濃縮成單一備忘錄。

---

> 將此提示複製到我們的開發者[控制台](/dashboard)中親自試用！

|      | 內容 |
| ---- | --- |
| User | 您的任務是分析以下報告：<br /> <br /> <report> [Matterport SEC 10-K 2023年申報文件的完整文本](https://investors.matterport.com/node/9501/html)，為簡潔起見此處未貼出] </report> <br /> <br /> 以簡潔明確的方式總結這份年度報告，並識別關鍵市場趨勢和要點。將您的發現輸出為我可以發送給團隊的簡短備忘錄。備忘錄的目標是確保我的團隊了解金融機構的最新狀況，並定性預測和識別下一季度是否有任何營運和收入風險。請確保在您的總結和分析中包含所有相關細節。 |

## 範例輸出

> 收件人：投資團隊  
> 寄件人：\[您的姓名\]  
> 日期：2024年3月4日  
> 主旨：Matterport 2022年10-K重點和財務展望
>
> 團隊您好，
>
> 我從Matterport的2022年年度報告中識別出一些關鍵要點：
>
> 財務表現：
>
> - 2022年收入增長22%至1.361億美元，由訂閱和服務收入增長推動
> - 然而，由於產品收入增加的成本和供應鏈挑戰，毛利率從54%下降至38%
> - 由於公司大力投資研發和銷售與行銷以推動增長，營運虧損擴大至2.755億美元
> - 2022年底擁有4.769億美元現金和投資以資助營運
>
> 業務亮點：
>
> - 訂戶基數增長39%，2022年底超過701,000人。管理空間達到920萬個
> - 透過推出Matterport Axis電動支架、Pro3相機和雲端軟體的重大更新來擴展平台
> - 完成對Enview（3D空間數據AI）和VHT Studios（房地產行銷）的收購以增強產品
> - 與Burns & McDonnell和CGS簽署重要合作夥伴關係，向其企業客戶提供Matterport的解決方案
>
> 展望：
>
> - Matterport看到數位化世界建築和空間的巨大市場機會（估計2400億美元以上的總可尋址市場）
> - 專注於推動房地產、建築工程施工、設施管理等關鍵垂直領域的大型企業客戶採用
> - 將繼續投資研發和國際擴張以滲透基本未開發的全球建築市場
> - 近期增長和利潤率可能受到宏觀經濟逆風和供應鏈問題的影響
> - 然而，健康的流動性狀況和相對穩定的客戶基礎提供下行保護
>
> 總結而言，Matterport在2022年實現強勁的營收增長，在滲透巨大全球市場方面取得進展。雖然由於公司大力投資增長而盈利能力仍面臨挑戰，但其資產負債表看起來穩固。除了一些持續的供應鏈問題外，基於關鍵營運指標，基本業務動能似乎保持完整。除非出現嚴重經濟衰退，Matterport似乎在新興的建築數位化領域中繼續獲得市場份額的位置良好。

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
    temperature=0,
    messages=[
        {
            "role": "user",
            "content": [
                {
                    "type": "text",
                    "text": "您的任務是分析以下報告：  \n<report>  \n[Matterport SEC 10-K 2023年申報文件的完整文本](https://investors.matterport.com/node/9501/html)，為簡潔起見此處未貼出]  \n</report>  \n  \n以簡潔明確的方式總結這份年度報告，並識別關鍵市場趨勢和要點。將您的發現輸出為我可以發送給團隊的簡短備忘錄。備忘錄的目標是確保我的團隊了解金融機構的最新狀況，並定性預測和識別下一季度是否有任何營運和收入風險。請確保在您的總結和分析中包含所有相關細節。"
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
  temperature: 0,
  messages: [
    {
      "role": "user",
      "content": [
        {
          "type": "text",
          "text": "您的任務是分析以下報告：  \n<report>  \n[Matterport SEC 10-K 2023年申報文件的完整文本](https://investors.matterport.com/node/9501/html)，為簡潔起見此處未貼出]  \n</report>  \n  \n以簡潔明確的方式總結這份年度報告，並識別關鍵市場趨勢和要點。將您的發現輸出為我可以發送給團隊的簡短備忘錄。備忘錄的目標是確保我的團隊了解金融機構的最新狀況，並定性預測和識別下一季度是否有任何營運和收入風險。請確保在您的總結和分析中包含所有相關細節。"
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
messages=[
{
"role": "user",
"content": [
{
"type": "text",
"text": "您的任務是分析以下報告： \n<report> \n[Matterport SEC 10-K 2023年申報文件的完整文本](https://investors.matterport.com/node/9501/html)，為簡潔起見此處未貼出] \n</report> \n \n以簡潔明確的方式總結這份年度報告，並識別關鍵市場趨勢和要點。將您的發現輸出為我可以發送給團隊的簡短備忘錄。備忘錄的目標是確保我的團隊了解金融機構的最新狀況，並定性預測和識別下一季度是否有任何營運和收入風險。請確保在您的總結和分析中包含所有相關細節。"
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
  temperature: 0,
  messages: [
    {
      "role": "user",
      "content": [
        {
          "type": "text",
          "text": "您的任務是分析以下報告：  \n<report>  \n[Matterport SEC 10-K 2023年申報文件的完整文本](https://investors.matterport.com/node/9501/html)，為簡潔起見此處未貼出]  \n</report>  \n  \n以簡潔明確的方式總結這份年度報告，並識別關鍵市場趨勢和要點。將您的發現輸出為我可以發送給團隊的簡短備忘錄。備忘錄的目標是確保我的團隊了解金融機構的最新狀況，並定性預測和識別下一季度是否有任何營運和收入風險。請確保在您的總結和分析中包含所有相關細節。"
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
    temperature=0,
    messages=[
        {
            "role": "user",
            "content": [
                {
                    "type": "text",
                    "text": "您的任務是分析以下報告：  \n<report>  \n[Matterport SEC 10-K 2023年申報文件的完整文本](https://investors.matterport.com/node/9501/html)，為簡潔起見此處未貼出]  \n</report>  \n  \n以簡潔明確的方式總結這份年度報告，並識別關鍵市場趨勢和要點。將您的發現輸出為我可以發送給團隊的簡短備忘錄。備忘錄的目標是確保我的團隊了解金融機構的最新狀況，並定性預測和識別下一季度是否有任何營運和收入風險。請確保在您的總結和分析中包含所有相關細節。"
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
  temperature: 0,
  messages: [
    {
      "role": "user",
      "content": [
        {
          "type": "text",
          "text": "您的任務是分析以下報告：  \n<report>  \n[Matterport SEC 10-K 2023年申報文件的完整文本](https://investors.matterport.com/node/9501/html)，為簡潔起見此處未貼出]  \n</report>  \n  \n以簡潔明確的方式總結這份年度報告，並識別關鍵市場趨勢和要點。將您的發現輸出為我可以發送給團隊的簡短備忘錄。備忘錄的目標是確保我的團隊了解金融機構的最新狀況，並定性預測和識別下一季度是否有任何營運和收入風險。請確保在您的總結和分析中包含所有相關細節。"
        }
      ]
    }
  ]
});
console.log(msg);

```

  </Tab>
</Tabs>