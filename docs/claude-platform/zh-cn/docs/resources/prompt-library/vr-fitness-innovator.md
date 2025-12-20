# VR健身创新者

为虚拟现实健身游戏集思广益创意想法。

---

> 将此提示复制到我们的开发者[控制台](/dashboard)中亲自试用！

| 角色 | 内容 |
| ---- | --- |
| 用户 | 您的任务是为虚拟现实（VR）健身游戏生成一系列创新且引人入胜的想法。考虑各种游戏类型、独特的游戏机制、沉浸式环境，以及将体育锻炼和动作融入其中的创意方式。这些想法应该对广泛的健身爱好者和游戏玩家具有吸引力，鼓励他们在VR中锻炼时保持活跃并享受乐趣。对于每个想法，请提供游戏概念的简要描述、关键特性以及它如何促进健身。 |

## 示例输出

1.  想法：VR跑酷冒险  
    描述：玩家在动态城市环境中导航，跑步、跳跃、攀爬并执行跑酷特技来克服障碍并达到目标。游戏适应玩家的动作，在探索虚拟城市的同时提供强烈的全身锻炼。  
    关键特性：逼真的跑酷机制、程序生成的环境、多人竞赛和挑战。  
    健身方面：有氧运动、敏捷性、协调性和下肢力量。
2.  想法：节奏拳击  
    描述：一款配合流行音乐节拍的高能量拳击游戏。玩家必须与节奏同步地出拳、躲避和格挡来得分并通过越来越具有挑战性的关卡。游戏提供各种拳击风格和训练模式。  
    关键特性：授权音乐、可定制拳击手、在线多人游戏和多样化的拳击技巧。  
    健身方面：有氧运动、上肢力量、反应能力和耐力。
3.  想法：VR健身RPG  
    描述：一款沉浸式角色扮演游戏，玩家创建自己的角色并踏上拯救奇幻世界的任务。游戏将传统RPG元素与健身挑战相结合，要求玩家执行体育锻炼来施放法术、击败敌人并提升角色等级。  
    关键特性：角色定制、技能树、史诗级boss战斗，以及力量、有氧和柔韧性锻炼的混合。  
    健身方面：全身锻炼、力量训练、有氧运动和柔韧性。

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
  messages=[
    {
      "role": "user",
      "content": [
        {
          "type": "text",
          "text": "您的任务是为虚拟现实（VR）健身游戏生成一系列创新且引人入胜的想法。考虑各种游戏类型、独特的游戏机制、沉浸式环境，以及将体育锻炼和动作融入其中的创意方式。这些想法应该对广泛的健身爱好者和游戏玩家具有吸引力，鼓励他们在VR中锻炼时保持活跃并享受乐趣。对于每个想法，请提供游戏概念的简要描述、关键特性以及它如何促进健身。"
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
  messages: [
    {
      "role": "user",
      "content": [
        {
          "type": "text",
          "text": "您的任务是为虚拟现实（VR）健身游戏生成一系列创新且引人入胜的想法。考虑各种游戏类型、独特的游戏机制、沉浸式环境，以及将体育锻炼和动作融入其中的创意方式。这些想法应该对广泛的健身爱好者和游戏玩家具有吸引力，鼓励他们在VR中锻炼时保持活跃并享受乐趣。对于每个想法，请提供游戏概念的简要描述、关键特性以及它如何促进健身。"
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
    messages=[
        {
            "role": "user",
            "content": [
                {
                    "type": "text",
                    "text": "您的任务是为虚拟现实（VR）健身游戏生成一系列创新且引人入胜的想法。考虑各种游戏类型、独特的游戏机制、沉浸式环境，以及将体育锻炼和动作融入其中的创意方式。这些想法应该对广泛的健身爱好者和游戏玩家具有吸引力，鼓励他们在VR中锻炼时保持活跃并享受乐趣。对于每个想法，请提供游戏概念的简要描述、关键特性以及它如何促进健身。"
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
  messages: [
    {
      "role": "user",
      "content": [
        {
          "type": "text",
          "text": "您的任务是为虚拟现实（VR）健身游戏生成一系列创新且引人入胜的想法。考虑各种游戏类型、独特的游戏机制、沉浸式环境，以及将体育锻炼和动作融入其中的创意方式。这些想法应该对广泛的健身爱好者和游戏玩家具有吸引力，鼓励他们在VR中锻炼时保持活跃并享受乐趣。对于每个想法，请提供游戏概念的简要描述、关键特性以及它如何促进健身。"
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
    messages=[
        {
            "role": "user",
            "content": [
                {
                    "type": "text",
                    "text": "您的任务是为虚拟现实（VR）健身游戏生成一系列创新且引人入胜的想法。考虑各种游戏类型、独特的游戏机制、沉浸式环境，以及将体育锻炼和动作融入其中的创意方式。这些想法应该对广泛的健身爱好者和游戏玩家具有吸引力，鼓励他们在VR中锻炼时保持活跃并享受乐趣。对于每个想法，请提供游戏概念的简要描述、关键特性以及它如何促进健身。"
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
  messages: [
    {
      "role": "user",
      "content": [
        {
          "type": "text",
          "text": "您的任务是为虚拟现实（VR）健身游戏生成一系列创新且引人入胜的想法。考虑各种游戏类型、独特的游戏机制、沉浸式环境，以及将体育锻炼和动作融入其中的创意方式。这些想法应该对广泛的健身爱好者和游戏玩家具有吸引力，鼓励他们在VR中锻炼时保持活跃并享受乐趣。对于每个想法，请提供游戏概念的简要描述、关键特性以及它如何促进健身。"
        }
      ]
    }
  ]
});
console.log(msg);

```

</Tab>
</Tabs>