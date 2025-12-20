# 企业透视师

从冗长的企业报告中提取洞察、识别风险，并将关键信息提炼成单一备忘录。

---

> 将此提示复制到我们的开发者[控制台](/dashboard)中亲自试用！

|      | 内容 |
| ---- | --- |
| 用户 | 您的任务是分析以下报告：<br /> <br /> <report> [Matterport SEC 10-K 2023年度报告全文](https://investors.matterport.com/node/9501/html)，为简洁起见此处未粘贴] </report> <br /> <br /> 以简洁明了的方式总结这份年度报告，并识别关键市场趋势和要点。将您的发现输出为我可以发送给团队的简短备忘录。备忘录的目标是确保我的团队了解金融机构的表现情况，并定性预测和识别下一季度是否存在任何运营和收入风险。确保在您的总结和分析中包含所有相关细节。 |

## 示例输出

> 收件人：投资团队  
> 发件人：\[您的姓名\]  
> 日期：2024年3月4日  
> 主题：Matterport 2022年10-K要点和财务展望
>
> 团队您好，
>
> 我从Matterport的2022年年度报告中识别出一些关键要点：
>
> 财务表现：
>
> - 2022年收入增长22%至1.361亿美元，由订阅和服务收入增长推动
> - 然而，由于产品收入增加的成本和供应链挑战，毛利率从54%下降至38%
> - 由于公司大力投资研发和销售营销以推动增长，运营亏损扩大至2.755亿美元
> - 2022年底拥有4.769亿美元现金和投资用于运营资金
>
> 业务亮点：
>
> - 订户基数增长39%，2022年底超过70.1万。管理空间达到920万个
> - 通过推出Matterport Axis电动支架、Pro3相机和云软件的重大更新来扩展平台
> - 完成对Enview（3D空间数据AI）和VHT Studios（房地产营销）的收购以增强产品
> - 与Burns & McDonnell和CGS签署重要合作伙伴关系，向其企业客户提供Matterport的解决方案
>
> 展望：
>
> - Matterport看到数字化世界建筑和空间的巨大市场机会（估计TAM超过2400亿美元）
> - 专注于推动房地产、AEC、设施管理等关键垂直领域大型企业客户的采用
> - 将继续投资研发和国际扩张以渗透基本未开发的全球建筑市场
> - 近期增长和利润率可能受到宏观经济逆风和供应链问题的影响
> - 然而，健康的流动性状况和相对粘性的客户基础提供了下行保护
>
> 总之，Matterport在2022年实现了强劲的营收增长，在渗透巨大的全球市场方面取得了进展。虽然由于公司大力投资增长，盈利能力仍然面临挑战，但其资产负债表看起来稳固。除了一些持续的供应链问题外，基于关键运营指标，潜在的业务势头似乎保持完好。除非出现严重的经济衰退，Matterport似乎处于有利地位，可以在新兴的建筑数字化领域继续获得份额。

---

## API请求

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
                    "text": "您的任务是分析以下报告：  \n<report>  \n[Matterport SEC 10-K 2023年度报告全文](https://investors.matterport.com/node/9501/html)，为简洁起见此处未粘贴]  \n</report>  \n  \n以简洁明了的方式总结这份年度报告，并识别关键市场趋势和要点。将您的发现输出为我可以发送给团队的简短备忘录。备忘录的目标是确保我的团队了解金融机构的表现情况，并定性预测和识别下一季度是否存在任何运营和收入风险。确保在您的总结和分析中包含所有相关细节。"
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
          "text": "您的任务是分析以下报告：  \n<report>  \n[Matterport SEC 10-K 2023年度报告全文](https://investors.matterport.com/node/9501/html)，为简洁起见此处未粘贴]  \n</report>  \n  \n以简洁明了的方式总结这份年度报告，并识别关键市场趋势和要点。将您的发现输出为我可以发送给团队的简短备忘录。备忘录的目标是确保我的团队了解金融机构的表现情况，并定性预测和识别下一季度是否存在任何运营和收入风险。确保在您的总结和分析中包含所有相关细节。"
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
"text": "您的任务是分析以下报告： \n<report> \n[Matterport SEC 10-K 2023年度报告全文](https://investors.matterport.com/node/9501/html)，为简洁起见此处未粘贴] \n</report> \n \n以简洁明了的方式总结这份年度报告，并识别关键市场趋势和要点。将您的发现输出为我可以发送给团队的简短备忘录。备忘录的目标是确保我的团队了解金融机构的表现情况，并定性预测和识别下一季度是否存在任何运营和收入风险。确保在您的总结和分析中包含所有相关细节。"
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
          "text": "您的任务是分析以下报告：  \n<report>  \n[Matterport SEC 10-K 2023年度报告全文](https://investors.matterport.com/node/9501/html)，为简洁起见此处未粘贴]  \n</report>  \n  \n以简洁明了的方式总结这份年度报告，并识别关键市场趋势和要点。将您的发现输出为我可以发送给团队的简短备忘录。备忘录的目标是确保我的团队了解金融机构的表现情况，并定性预测和识别下一季度是否存在任何运营和收入风险。确保在您的总结和分析中包含所有相关细节。"
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
                    "text": "您的任务是分析以下报告：  \n<report>  \n[Matterport SEC 10-K 2023年度报告全文](https://investors.matterport.com/node/9501/html)，为简洁起见此处未粘贴]  \n</report>  \n  \n以简洁明了的方式总结这份年度报告，并识别关键市场趋势和要点。将您的发现输出为我可以发送给团队的简短备忘录。备忘录的目标是确保我的团队了解金融机构的表现情况，并定性预测和识别下一季度是否存在任何运营和收入风险。确保在您的总结和分析中包含所有相关细节。"
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
          "text": "您的任务是分析以下报告：  \n<report>  \n[Matterport SEC 10-K 2023年度报告全文](https://investors.matterport.com/node/9501/html)，为简洁起见此处未粘贴]  \n</report>  \n  \n以简洁明了的方式总结这份年度报告，并识别关键市场趋势和要点。将您的发现输出为我可以发送给团队的简短备忘录。备忘录的目标是确保我的团队了解金融机构的表现情况，并定性预测和识别下一季度是否存在任何运营和收入风险。确保在您的总结和分析中包含所有相关细节。"
        }
      ]
    }
  ]
});
console.log(msg);

```

  </Tab>
</Tabs>