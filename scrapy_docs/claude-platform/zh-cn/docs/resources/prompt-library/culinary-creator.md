# 烹饪创作者

根据用户的现有食材和饮食偏好建议食谱创意。

---

> 将此提示复制到我们的开发者[控制台](/dashboard)中亲自试用！

|        | 内容 |
| --- | --- |
| System | 您的任务是根据用户输入的现有食材和饮食偏好生成个性化的食谱创意。使用这些信息建议各种创意美味的食谱，这些食谱可以使用给定的食材制作，同时满足用户的饮食需求（如果有提及的话）。对于每个食谱，提供简要描述、所需食材清单和简单的制作说明。确保食谱易于遵循、营养丰富，并且可以用最少的额外食材或设备制作。 |
| User   | 现有食材：鸡胸肉、西兰花、胡萝卜、洋葱、大蒜、橄榄油、米饭、无麸质生抽、蜂蜜 <br/> 饮食偏好：无麸质 |

## 示例输出

建议食谱：

蜂蜜蒜香鸡肉炒菜

描述：一道快速美味的炒菜，配有嫩滑的鸡胸肉、爽脆的蔬菜和甜咸的蜂蜜蒜香酱汁。  
食材：

- 2块鸡胸肉，切成一口大小的块
- 2杯西兰花小花
- 1杯切片胡萝卜
- 1个洋葱，切丁
- 3瓣大蒜，切碎
- 2汤匙橄榄油
- 1/4杯生抽（无麸质）
- 2汤匙蜂蜜

制作说明：

1.  在大平底锅或炒锅中，用中高火加热橄榄油。
2.  加入切丁的洋葱和切碎的大蒜，炒至香味四溢，约2分钟。
3.  加入鸡肉块，炒至不再呈粉红色，约5-7分钟。
4.  加入西兰花和胡萝卜，炒至蔬菜呈嫩脆状，约5分钟。
5.  在小碗中，将生抽和蜂蜜搅拌均匀。
6.  将酱汁倒在鸡肉和蔬菜上，搅拌均匀包裹。
7.  继续烹饪2-3分钟，或直到酱汁稍微变稠。
8.  趁热配蒸米饭享用。

鸡肉蔬菜米饭碗

描述：一道丰盛营养的米饭碗，配有调味鸡肉、烤蔬菜和生抽淋汁。

食材：

- 2块鸡胸肉，切成一口大小的块
- 2杯西兰花小花
- 1杯切片胡萝卜
- 1个洋葱，切丁
- 2瓣大蒜，切碎
- 2汤匙橄榄油
- 1/4杯生抽（无麸质）
- 2杯煮熟的米饭

制作说明：

1.  将烤箱预热至400°F（200°C）。
2.  在大碗中，将鸡肉块、西兰花、胡萝卜、洋葱和大蒜混合。
3.  淋上橄榄油和一半的生抽，搅拌均匀包裹。
4.  将鸡肉和蔬菜摊在烤盘上，在预热的烤箱中烤20-25分钟，或直到鸡肉完全熟透，蔬菜变软。
5.  将煮熟的米饭分装到各个碗中。
6.  在每个碗上放上烤鸡肉和蔬菜。
7.  在每个碗的顶部淋上剩余的生抽。
8.  趁热享用！

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
  temperature=0.5,
  system="您的任务是根据用户输入的现有食材和饮食偏好生成个性化的食谱创意。使用这些信息建议各种创意美味的食谱，这些食谱可以使用给定的食材制作，同时满足用户的饮食需求（如果有提及的话）。对于每个食谱，提供简要描述、所需食材清单和简单的制作说明。确保食谱易于遵循、营养丰富，并且可以用最少的额外食材或设备制作。",
  messages=[
    {
    "role": "user",
    "content": [
        {
          "type": "text",
          "text": "现有食材：鸡胸肉、西兰花、胡萝卜、洋葱、大蒜、橄榄油、米饭、无麸质生抽、蜂蜜 \n饮食偏好：无麸质"
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
  system: "您的任务是根据用户输入的现有食材和饮食偏好生成个性化的食谱创意。使用这些信息建议各种创意美味的食谱，这些食谱可以使用给定的食材制作，同时满足用户的饮食需求（如果有提及的话）。对于每个食谱，提供简要描述、所需食材清单和简单的制作说明。确保食谱易于遵循、营养丰富，并且可以用最少的额外食材或设备制作。",
  messages: [
    {
      "role": "user",
      "content": [
        {
          "type": "text",
          "text": "现有食材：鸡胸肉、西兰花、胡萝卜、洋葱、大蒜、橄榄油、米饭、无麸质生抽、蜂蜜  \n饮食偏好：无麸质"
        }
      ]
    }
  ]
});
console.log(msg);

````

</Tab>
<Tab title="AWS Bedrock Python">

```
from anthropic import AnthropicBedrock

# See https://docs.claude.com/claude/reference/claude-on-amazon-bedrock
# for authentication options
client = AnthropicBedrock()

message = client.messages.create(
    model="anthropic.claude-sonnet-4-5-20250929-v1:0",
    max_tokens=2000,
    temperature=0.5,
    system="您的任务是根据用户输入的现有食材和饮食偏好生成个性化的食谱创意。使用这些信息建议各种创意美味的食谱，这些食谱可以使用给定的食材制作，同时满足用户的饮食需求（如果有提及的话）。对于每个食谱，提供简要描述、所需食材清单和简单的制作说明。确保食谱易于遵循、营养丰富，并且可以用最少的额外食材或设备制作。",
    messages=[
        {
            "role": "user",
            "content": [
                {
                    "type": "text",
                    "text": "现有食材：鸡胸肉、西兰花、胡萝卜、洋葱、大蒜、橄榄油、米饭、无麸质生抽、蜂蜜  \n饮食偏好：无麸质"
                }
            ]
        }
    ]
)
print(message.content)

```

</Tab>
<Tab title="AWS Bedrock TypeScript">

```
import AnthropicBedrock from "@anthropic-ai/bedrock-sdk";

// See https://docs.claude.com/claude/reference/claude-on-amazon-bedrock
// for authentication options
const client = new AnthropicBedrock();

const msg = await client.messages.create({
  model: "anthropic.claude-sonnet-4-5-20250929-v1:0",
  max_tokens: 2000,
  temperature: 0.5,
  system: "您的任务是根据用户输入的现有食材和饮食偏好生成个性化的食谱创意。使用这些信息建议各种创意美味的食谱，这些食谱可以使用给定的食材制作，同时满足用户的饮食需求（如果有提及的话）。对于每个食谱，提供简要描述、所需食材清单和简单的制作说明。确保食谱易于遵循、营养丰富，并且可以用最少的额外食材或设备制作。",
  messages: [
    {
      "role": "user",
      "content": [
        {
          "type": "text",
          "text": "现有食材：鸡胸肉、西兰花、胡萝卜、洋葱、大蒜、橄榄油、米饭、无麸质生抽、蜂蜜  \n饮食偏好：无麸质"
        }
      ]
    }
  ]
});
console.log(msg);

```

</Tab>
<Tab title="Vertex AI Python">

```
from anthropic import AnthropicVertex

client = AnthropicVertex()

message = client.messages.create(
    model="claude-sonnet-4@20250514",
    max_tokens=2000,
    temperature=0.5,
    system="您的任务是根据用户输入的现有食材和饮食偏好生成个性化的食谱创意。使用这些信息建议各种创意美味的食谱，这些食谱可以使用给定的食材制作，同时满足用户的饮食需求（如果有提及的话）。对于每个食谱，提供简要描述、所需食材清单和简单的制作说明。确保食谱易于遵循、营养丰富，并且可以用最少的额外食材或设备制作。",
    messages=[
        {
            "role": "user",
            "content": [
                {
                    "type": "text",
                    "text": "现有食材：鸡胸肉、西兰花、胡萝卜、洋葱、大蒜、橄榄油、米饭、无麸质生抽、蜂蜜  \n饮食偏好：无麸质"
                }
            ]
        }
    ]
)
print(message.content)

```

</Tab>
<Tab title="Vertex AI TypeScript">

```
import { AnthropicVertex } from '@anthropic-ai/vertex-sdk';

// Reads from the `CLOUD_ML_REGION` & `ANTHROPIC_VERTEX_PROJECT_ID` environment variables.
// Additionally goes through the standard `google-auth-library` flow.
const client = new AnthropicVertex();

const msg = await client.messages.create({
  model: "claude-sonnet-4@20250514",
  max_tokens: 2000,
  temperature: 0.5,
  system: "您的任务是根据用户输入的现有食材和饮食偏好生成个性化的食谱创意。使用这些信息建议各种创意美味的食谱，这些食谱可以使用给定的食材制作，同时满足用户的饮食需求（如果有提及的话）。对于每个食谱，提供简要描述、所需食材清单和简单的制作说明。确保食谱易于遵循、营养丰富，并且可以用最少的额外食材或设备制作。",
  messages: [
    {
      "role": "user",
      "content": [
        {
          "type": "text",
          "text": "现有食材：鸡胸肉、西兰花、胡萝卜、洋葱、大蒜、橄榄油、米饭、无麸质生抽、蜂蜜  \n饮食偏好：无麸质"
        }
      ]
    }
  ]
});
console.log(msg);

```

</Tab>
</Tabs>