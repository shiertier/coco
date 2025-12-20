# 备忘录大师

基于关键要点撰写全面的公司备忘录。

---

> 将此提示复制到我们的开发者[控制台](/dashboard)中亲自试用！

|        | 内容 |
| --- | --- |
| System | 您的任务是基于提供的关键要点撰写一份全面的公司备忘录。备忘录应以专业的语调撰写，以清晰简洁的方式涵盖所有相关信息。使用适当的格式，如标题、副标题和项目符号，有效地组织内容。确保备忘录结构良好、连贯且易于目标受众理解。 |
| User   | - 宣布新的员工健康计划："Fit4Success" <br/> - 计划目标：促进身心健康，减少压力，提高生产力 <br/> - 组成部分：现场健身课程、健康餐食选择、心理健康资源和健康研讨会 <br/> - 与当地健身房和健康中心合作提供会员折扣 <br/> - 参与激励措施：健身追踪器、健康积分和抽奖活动 <br/> - 计划启动日期：2023年6月1日 <br/> - 注册流程：通过公司内网在线注册 <br/> - 计划协调员：人力资源和员工参与团队 |

## 示例输出

主题：介绍Fit4Success - 我们的新员工健康计划

大家好，

我很高兴宣布推出我们的新员工健康计划Fit4Success，该计划旨在促进团队成员的身心健康，减少压力，提高生产力。该计划将于2023年6月1日正式启动。

计划目标：

- 鼓励定期体育活动和健康的生活方式选择
- 为心理健康和压力管理提供资源和支持
- 营造积极参与的工作环境，优先考虑员工福祉
- 提高整体生产力和工作满意度

计划组成部分：

- 现场健身课程：瑜伽、普拉提和高强度间歇训练(HIIT)课程
- 健康餐食选择：公司食堂提供营养零食和餐食
- 心理健康资源：获得咨询服务、冥想应用程序和减压研讨会
- 健康研讨会：关于营养、睡眠卫生和工作生活平衡等主题的讲座

合作伙伴关系和折扣：  
 为了进一步支持您的健康之旅，我们与当地健身房和健康中心合作，专门为我们的员工提供会员折扣。利用这些特殊价格，您可以使用各种健身设施和课程。

参与激励措施：  
 为了鼓励积极参与Fit4Success计划，我们将提供以下激励措施：

- 为所有参与者提供健身追踪器，以监控他们的进度并设定个人目标
- 通过计划参与获得健康积分，可兑换额外奖励
- 为达到特定健康里程碑的参与者举行月度抽奖活动

注册流程：  
 要注册Fit4Success计划，请访问我们的公司内网并填写在线注册表。注册截止日期为2023年5月25日。如果您对注册流程有任何问题或疑虑，请联系我们的人力资源团队。

计划协调：  
 Fit4Success计划将由我们的人力资源和员工参与团队共同协调。他们将在您的健康之旅中提供指导、支持和资源。

我们坚信，投资于员工的福祉对我们组织的成功和发展至关重要。我们鼓励您利用这个令人兴奋的机会，与我们一起创造一个更健康、更快乐、更高效的工作场所。

让我们拥抱Fit4Success计划，共同努力迈向更健康的未来！

此致敬礼，  
 \[您的姓名\]  
 \[您的职位\]

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
    max_tokens=1000,
    temperature=1,
    system="您的任务是基于提供的关键要点撰写一份全面的公司备忘录。备忘录应以专业的语调撰写，以清晰简洁的方式涵盖所有相关信息。使用适当的格式，如标题、副标题和项目符号，有效地组织内容。确保备忘录结构良好、连贯且易于目标受众理解。",
    messages=[
        {
            "role": "user",
            "content": [
                {
                    "type": "text",
                    "text": "- 宣布新的员工健康计划：\"Fit4Success\"  \n- 计划目标：促进身心健康，减少压力，提高生产力  \n- 组成部分：现场健身课程、健康餐食选择、心理健康资源和健康研讨会  \n- 与当地健身房和健康中心合作提供会员折扣  \n- 参与激励措施：健身追踪器、健康积分和抽奖活动  \n- 计划启动日期：2023年6月1日  \n- 注册流程：通过公司内网在线注册  \n- 计划协调员：人力资源和员工参与团队"
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
  system: "您的任务是基于提供的关键要点撰写一份全面的公司备忘录。备忘录应以专业的语调撰写，以清晰简洁的方式涵盖所有相关信息。使用适当的格式，如标题、副标题和项目符号，有效地组织内容。确保备忘录结构良好、连贯且易于目标受众理解。",
  messages: [
    {
      "role": "user",
      "content": [
        {
          "type": "text",
          "text": "- 宣布新的员工健康计划：\"Fit4Success\"  \n- 计划目标：促进身心健康，减少压力，提高生产力  \n- 组成部分：现场健身课程、健康餐食选择、心理健康资源和健康研讨会  \n- 与当地健身房和健康中心合作提供会员折扣  \n- 参与激励措施：健身追踪器、健康积分和抽奖活动  \n- 计划启动日期：2023年6月1日  \n- 注册流程：通过公司内网在线注册  \n- 计划协调员：人力资源和员工参与团队"
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
system="您的任务是基于提供的关键要点撰写一份全面的公司备忘录。备忘录应以专业的语调撰写，以清晰简洁的方式涵盖所有相关信息。使用适当的格式，如标题、副标题和项目符号，有效地组织内容。确保备忘录结构良好、连贯且易于目标受众理解。",
messages=[
{
"role": "user",
"content": [
{
"type": "text",
"text": "- 宣布新的员工健康计划：\"Fit4Success\" \n- 计划目标：促进身心健康，减少压力，提高生产力 \n- 组成部分：现场健身课程、健康餐食选择、心理健康资源和健康研讨会 \n- 与当地健身房和健康中心合作提供会员折扣 \n- 参与激励措施：健身追踪器、健康积分和抽奖活动 \n- 计划启动日期：2023年6月1日 \n- 注册流程：通过公司内网在线注册 \n- 计划协调员：人力资源和员工参与团队"
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
  system: "您的任务是基于提供的关键要点撰写一份全面的公司备忘录。备忘录应以专业的语调撰写，以清晰简洁的方式涵盖所有相关信息。使用适当的格式，如标题、副标题和项目符号，有效地组织内容。确保备忘录结构良好、连贯且易于目标受众理解。",
  messages: [
    {
      "role": "user",
      "content": [
        {
          "type": "text",
          "text": "- 宣布新的员工健康计划：\"Fit4Success\"  \n- 计划目标：促进身心健康，减少压力，提高生产力  \n- 组成部分：现场健身课程、健康餐食选择、心理健康资源和健康研讨会  \n- 与当地健身房和健康中心合作提供会员折扣  \n- 参与激励措施：健身追踪器、健康积分和抽奖活动  \n- 计划启动日期：2023年6月1日  \n- 注册流程：通过公司内网在线注册  \n- 计划协调员：人力资源和员工参与团队"
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
    system="您的任务是基于提供的关键要点撰写一份全面的公司备忘录。备忘录应以专业的语调撰写，以清晰简洁的方式涵盖所有相关信息。使用适当的格式，如标题、副标题和项目符号，有效地组织内容。确保备忘录结构良好、连贯且易于目标受众理解。",
    messages=[
        {
            "role": "user",
            "content": [
                {
                    "type": "text",
                    "text": "- 宣布新的员工健康计划：\"Fit4Success\"  \n- 计划目标：促进身心健康，减少压力，提高生产力  \n- 组成部分：现场健身课程、健康餐食选择、心理健康资源和健康研讨会  \n- 与当地健身房和健康中心合作提供会员折扣  \n- 参与激励措施：健身追踪器、健康积分和抽奖活动  \n- 计划启动日期：2023年6月1日  \n- 注册流程：通过公司内网在线注册  \n- 计划协调员：人力资源和员工参与团队"
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
  system: "您的任务是基于提供的关键要点撰写一份全面的公司备忘录。备忘录应以专业的语调撰写，以清晰简洁的方式涵盖所有相关信息。使用适当的格式，如标题、副标题和项目符号，有效地组织内容。确保备忘录结构良好、连贯且易于目标受众理解。",
  messages: [
    {
      "role": "user",
      "content": [
        {
          "type": "text",
          "text": "- 宣布新的员工健康计划：\"Fit4Success\"  \n- 计划目标：促进身心健康，减少压力，提高生产力  \n- 组成部分：现场健身课程、健康餐食选择、心理健康资源和健康研讨会  \n- 与当地健身房和健康中心合作提供会员折扣  \n- 参与激励措施：健身追踪器、健康积分和抽奖活动  \n- 计划启动日期：2023年6月1日  \n- 注册流程：通过公司内网在线注册  \n- 计划协调员：人力资源和员工参与团队"
        }
      ]
    }
  ]
});
console.log(msg);

```

</Tab>
</Tabs>