# VR 健身創新者

為虛擬實境健身遊戲腦力激盪創意想法。

---

> 將此提示複製到我們的開發者 [Console](/dashboard) 中親自試試看！

| 角色 | 內容 |
| ---- | --- |
| 使用者 | 您的任務是為虛擬實境（VR）健身遊戲產生一份創新且引人入勝的想法清單。考慮各種遊戲類型、獨特的遊戲機制、沉浸式環境，以及融入體能運動和動作的創意方式。這些想法應該吸引廣泛的健身愛好者和遊戲玩家，鼓勵他們在 VR 中運動時保持活躍並享受樂趣。對於每個想法，請提供遊戲概念的簡要描述、主要特色，以及它如何促進健身。 |

## 範例輸出

1.  想法：VR 跑酷冒險  
    描述：玩家在動態城市環境中導航，跑步、跳躍、攀爬，並執行跑酷特技來克服障礙並達成目標。遊戲會適應玩家的動作，在探索虛擬城市的同時提供強烈的全身鍛鍊。  
    主要特色：逼真的跑酷機制、程序生成環境、多人競賽和挑戰。  
    健身方面：有氧運動、敏捷性、協調性和下肢力量。
2.  想法：節奏拳擊  
    描述：一款配合流行音樂節拍的高能量拳擊遊戲。玩家必須與節奏同步地出拳、閃避和格擋來得分，並通過越來越具挑戰性的關卡。遊戲提供各種拳擊風格和訓練模式。  
    主要特色：授權音樂、可自訂拳擊手、線上多人遊戲，以及多樣化的拳擊技巧。  
    健身方面：有氧運動、上肢力量、反應力和耐力。
3.  想法：VR 健身 RPG  
    描述：一款沉浸式角色扮演遊戲，玩家創建自己的角色並踏上拯救奇幻世界的任務。遊戲結合傳統 RPG 元素與健身挑戰，要求玩家執行體能運動來施放法術、擊敗敵人並提升角色等級。  
    主要特色：角色自訂、技能樹、史詩級頭目戰鬥，以及力量、有氧和柔韌性運動的組合。  
    健身方面：全身鍛鍊、力量訓練、有氧運動和柔韌性。

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
  messages=[
    {
      "role": "user",
      "content": [
        {
          "type": "text",
          "text": "您的任務是為虛擬實境（VR）健身遊戲產生一份創新且引人入勝的想法清單。考慮各種遊戲類型、獨特的遊戲機制、沉浸式環境，以及融入體能運動和動作的創意方式。這些想法應該吸引廣泛的健身愛好者和遊戲玩家，鼓勵他們在 VR 中運動時保持活躍並享受樂趣。對於每個想法，請提供遊戲概念的簡要描述、主要特色，以及它如何促進健身。"
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
          "text": "您的任務是為虛擬實境（VR）健身遊戲產生一份創新且引人入勝的想法清單。考慮各種遊戲類型、獨特的遊戲機制、沉浸式環境，以及融入體能運動和動作的創意方式。這些想法應該吸引廣泛的健身愛好者和遊戲玩家，鼓勵他們在 VR 中運動時保持活躍並享受樂趣。對於每個想法，請提供遊戲概念的簡要描述、主要特色，以及它如何促進健身。"
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
                    "text": "您的任務是為虛擬實境（VR）健身遊戲產生一份創新且引人入勝的想法清單。考慮各種遊戲類型、獨特的遊戲機制、沉浸式環境，以及融入體能運動和動作的創意方式。這些想法應該吸引廣泛的健身愛好者和遊戲玩家，鼓勵他們在 VR 中運動時保持活躍並享受樂趣。對於每個想法，請提供遊戲概念的簡要描述、主要特色，以及它如何促進健身。"
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
          "text": "您的任務是為虛擬實境（VR）健身遊戲產生一份創新且引人入勝的想法清單。考慮各種遊戲類型、獨特的遊戲機制、沉浸式環境，以及融入體能運動和動作的創意方式。這些想法應該吸引廣泛的健身愛好者和遊戲玩家，鼓勵他們在 VR 中運動時保持活躍並享受樂趣。對於每個想法，請提供遊戲概念的簡要描述、主要特色，以及它如何促進健身。"
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
                    "text": "您的任務是為虛擬實境（VR）健身遊戲產生一份創新且引人入勝的想法清單。考慮各種遊戲類型、獨特的遊戲機制、沉浸式環境，以及融入體能運動和動作的創意方式。這些想法應該吸引廣泛的健身愛好者和遊戲玩家，鼓勵他們在 VR 中運動時保持活躍並享受樂趣。對於每個想法，請提供遊戲概念的簡要描述、主要特色，以及它如何促進健身。"
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
          "text": "您的任務是為虛擬實境（VR）健身遊戲產生一份創新且引人入勝的想法清單。考慮各種遊戲類型、獨特的遊戲機制、沉浸式環境，以及融入體能運動和動作的創意方式。這些想法應該吸引廣泛的健身愛好者和遊戲玩家，鼓勵他們在 VR 中運動時保持活躍並享受樂趣。對於每個想法，請提供遊戲概念的簡要描述、主要特色，以及它如何促進健身。"
        }
      ]
    }
  ]
});
console.log(msg);

```

</Tab>
</Tabs>